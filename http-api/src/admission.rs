//! Admission to rendering: at most `ZPL_RENDER_CONCURRENCY` renders run at once (default:
//! the CPUs available to the process), and overload is turned away early instead of
//! queueing without bound.
//!
//! A request that cannot start rendering right away waits in line, unless
//! `ZPL_RENDER_QUEUE` requests (default 4 per render slot) are already waiting, or it
//! has waited `ZPL_RENDER_MAX_WAIT_MS` (default 5000): then `/render` answers 503 with
//! `Retry-After: 1`. The queue is checked before the body is read as well, so a shed
//! request costs next to nothing. Shed requests are not billed.
//!
//! Keeping renders to the CPU count leaves the async workers free, so `/health` and the
//! body reads of other requests stay fast however busy rendering is.
//!
//! The memory budget ([`RENDER_PERMITS`], one per started MiB of input) is taken in the
//! same wait, so a large batch waiting for memory counts against the same limits.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

use tokio::sync::{OwnedSemaphorePermit, Semaphore};

use crate::log_every::LogEvery;

pub const DEFAULT_MAX_WAIT: Duration = Duration::from_secs(5);
/// Default queue length per render slot.
pub const QUEUE_PER_SLOT: usize = 4;
/// Rendering needs roughly 8x its input in memory (a 22 MB batch of bitmap labels peaks
/// near 190 MB), so renders share a budget of one maximum-size body per process: a large
/// request waits for its share instead of running the pod out of memory, while small ones
/// (1 permit each) still run side by side.
pub const RENDER_PERMITS: usize = crate::MAX_BODY_BYTES >> 20;
const LOG_INTERVAL: Duration = Duration::from_secs(10);

pub struct Admission {
    concurrency: usize,
    max_queue: usize,
    max_wait: Duration,
    slots: Arc<Semaphore>,
    budget: Arc<Semaphore>,
    /// Requests waiting for a slot or for memory.
    waiting: AtomicUsize,
    /// Shed because the queue was full, and because the wait ran out.
    shed: LogEvery<2>,
}

/// Why a request was turned away.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shed {
    QueueFull,
    WaitedTooLong,
}

/// Leave to render; dropping it frees the slot and the memory.
pub struct Ticket {
    _slot: OwnedSemaphorePermit,
    _budget: OwnedSemaphorePermit,
}

impl Admission {
    /// `concurrency` renders at once (at least 1), `max_queue` waiting, each for at most
    /// `max_wait`.
    pub fn new(concurrency: usize, max_queue: usize, max_wait: Duration) -> Admission {
        let concurrency = concurrency.max(1);
        Admission {
            concurrency,
            max_queue,
            max_wait,
            slots: Arc::new(Semaphore::new(concurrency)),
            budget: Arc::new(Semaphore::new(RENDER_PERMITS)),
            waiting: AtomicUsize::new(0),
            shed: LogEvery::new(LOG_INTERVAL),
        }
    }

    /// The defaults: a render per available CPU.
    pub fn with_defaults() -> Admission {
        let concurrency = default_concurrency();
        Admission::new(concurrency, concurrency * QUEUE_PER_SLOT, DEFAULT_MAX_WAIT)
    }

    /// From `ZPL_RENDER_CONCURRENCY`, `ZPL_RENDER_QUEUE` and `ZPL_RENDER_MAX_WAIT_MS`,
    /// defaults when unset.
    pub fn from_env() -> Result<Admission, String> {
        let var = |name| std::env::var(name).ok();
        from_vars(
            var("ZPL_RENDER_CONCURRENCY").as_deref(),
            var("ZPL_RENDER_QUEUE").as_deref(),
            var("ZPL_RENDER_MAX_WAIT_MS").as_deref(),
        )
    }

    pub fn concurrency(&self) -> usize {
        self.concurrency
    }

    pub fn max_queue(&self) -> usize {
        self.max_queue
    }

    pub fn max_wait(&self) -> Duration {
        self.max_wait
    }

    /// Whether a new request would be turned away now; checked before reading its body.
    pub fn check(&self) -> Result<(), Shed> {
        if self.slots.available_permits() == 0 && self.waiting.load(Ordering::Relaxed) >= self.max_queue {
            self.note(Shed::QueueFull);
            return Err(Shed::QueueFull);
        }
        Ok(())
    }

    /// Waits for a render slot and `permits` of the memory budget (capped at all of it).
    pub async fn admit(&self, permits: u32) -> Result<Ticket, Shed> {
        let permits = permits.min(RENDER_PERMITS as u32);
        // Free capacity: no queueing (the semaphores are fair, so this never jumps the line).
        if let Ok(budget) = self.budget.clone().try_acquire_many_owned(permits)
            && let Ok(slot) = self.slots.clone().try_acquire_owned()
        {
            return Ok(Ticket { _slot: slot, _budget: budget });
        }

        let waiting = Waiting::enter(&self.waiting);
        if waiting.ahead >= self.max_queue {
            self.note(Shed::QueueFull);
            return Err(Shed::QueueFull);
        }
        let acquire = async {
            let budget = self.budget.clone().acquire_many_owned(permits).await.expect("never closed");
            let slot = self.slots.clone().acquire_owned().await.expect("never closed");
            Ticket { _slot: slot, _budget: budget }
        };
        match tokio::time::timeout(self.max_wait, acquire).await {
            Ok(ticket) => Ok(ticket),
            Err(_) => {
                self.note(Shed::WaitedTooLong);
                Err(Shed::WaitedTooLong)
            }
        }
    }

    /// Counts a shed request; logs the totals at most every [`LOG_INTERVAL`].
    fn note(&self, shed: Shed) {
        self.shed.add(&[(shed as usize, 1)], |[full, late]| {
            eprintln!(
                "overloaded: answered {} render request(s) with 503 since the last report \
                 ({full} found {} already waiting, {late} waited {} ms); rendering {} at once",
                full + late,
                self.max_queue,
                self.max_wait.as_millis(),
                self.concurrency,
            );
        });
    }
}

/// Counts a request as waiting while it lives.
struct Waiting<'a> {
    count: &'a AtomicUsize,
    /// Requests that were waiting already.
    ahead: usize,
}

impl<'a> Waiting<'a> {
    fn enter(count: &'a AtomicUsize) -> Self {
        Waiting { count, ahead: count.fetch_add(1, Ordering::Relaxed) }
    }
}

impl Drop for Waiting<'_> {
    fn drop(&mut self) {
        self.count.fetch_sub(1, Ordering::Relaxed);
    }
}

fn default_concurrency() -> usize {
    // Honours cgroup CPU quotas on Linux, so a pod limited to 2 CPUs gets 2.
    std::thread::available_parallelism().map_or(1, usize::from)
}

fn from_vars(concurrency: Option<&str>, queue: Option<&str>, max_wait_ms: Option<&str>) -> Result<Admission, String> {
    let parse = |name: &str, value: Option<&str>| match value.map(str::trim).filter(|v| !v.is_empty()) {
        None => Ok(None),
        Some(v) => v.parse::<usize>().map(Some).map_err(|_| format!("{name}: expected a whole number, got {v:?}")),
    };
    let concurrency = match parse("ZPL_RENDER_CONCURRENCY", concurrency)? {
        Some(0) => return Err("ZPL_RENDER_CONCURRENCY: must be at least 1".into()),
        Some(n) => n,
        None => default_concurrency(),
    };
    let queue = parse("ZPL_RENDER_QUEUE", queue)?.unwrap_or(concurrency * QUEUE_PER_SLOT);
    let max_wait = match parse("ZPL_RENDER_MAX_WAIT_MS", max_wait_ms)? {
        Some(0) => return Err("ZPL_RENDER_MAX_WAIT_MS: must be at least 1".into()),
        Some(ms) => Duration::from_millis(ms as u64),
        None => DEFAULT_MAX_WAIT,
    };
    Ok(Admission::new(concurrency, queue, max_wait))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn configuration() {
        let admission = from_vars(None, None, None).unwrap();
        assert_eq!(admission.concurrency(), default_concurrency());
        assert_eq!(admission.max_queue(), default_concurrency() * QUEUE_PER_SLOT);
        assert_eq!(admission.max_wait(), DEFAULT_MAX_WAIT);
        let admission = from_vars(Some(" 3 "), None, Some("250")).unwrap();
        assert_eq!((admission.concurrency(), admission.max_queue()), (3, 12));
        assert_eq!(admission.max_wait(), Duration::from_millis(250));
        assert_eq!(from_vars(None, Some("0"), None).unwrap().max_queue(), 0);
        assert!(from_vars(Some("0"), None, None).err().unwrap().contains("ZPL_RENDER_CONCURRENCY"));
        assert!(from_vars(None, Some("-1"), None).err().unwrap().contains("ZPL_RENDER_QUEUE"));
        assert!(from_vars(None, None, Some("0")).err().unwrap().contains("ZPL_RENDER_MAX_WAIT_MS"));
    }

    #[tokio::test]
    async fn bounded_queue_and_wait() {
        let admission = Arc::new(Admission::new(1, 1, Duration::from_millis(100)));
        let ticket = admission.admit(1).await.unwrap();
        assert_eq!(admission.check(), Ok(()), "nobody is waiting yet");

        // The second waits in line; a third finds the line full, before and after reading.
        let second = tokio::spawn({
            let admission = admission.clone();
            async move { admission.admit(1).await.map(drop) }
        });
        while admission.waiting.load(Ordering::Relaxed) == 0 {
            tokio::task::yield_now().await;
        }
        assert_eq!(admission.check(), Err(Shed::QueueFull));
        assert_eq!(admission.admit(1).await.err(), Some(Shed::QueueFull));

        // The second gives up after the maximum wait.
        let start = Instant::now();
        assert_eq!(second.await.unwrap(), Err(Shed::WaitedTooLong));
        assert!(start.elapsed() < Duration::from_secs(2));
        assert_eq!(admission.waiting.load(Ordering::Relaxed), 0);

        // Once the slot is free, requests go straight through.
        drop(ticket);
        let ticket = admission.admit(1).await.unwrap();
        drop(ticket);
        admission.admit(1).await.unwrap();
    }

    #[tokio::test]
    async fn waiting_requests_get_the_slot() {
        let admission = Arc::new(Admission::new(1, 4, Duration::from_secs(5)));
        let ticket = admission.admit(1).await.unwrap();
        let next = tokio::spawn({
            let admission = admission.clone();
            async move { admission.admit(1).await.map(drop) }
        });
        tokio::task::yield_now().await;
        drop(ticket);
        assert_eq!(next.await.unwrap(), Ok(()));
    }

    #[tokio::test]
    async fn memory_budget_is_shared() {
        let admission = Admission::new(4, 0, Duration::from_millis(50));
        // A maximum-size request takes the whole budget; a small one must wait for it.
        let big = admission.admit(u32::MAX).await.unwrap();
        assert_eq!(admission.admit(1).await.err(), Some(Shed::QueueFull));
        drop(big);
        admission.admit(1).await.unwrap();
    }
}
