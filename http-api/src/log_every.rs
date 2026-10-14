//! Logging for events that can happen thousands of times a second under load: count them
//! and log the totals at most once per interval, instead of a line per event.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;
use std::time::{Duration, Instant};

/// Counters logged together at most once per `interval`; the first event logs at once.
pub struct LogEvery<const N: usize> {
    interval: Duration,
    counts: [AtomicU64; N],
    /// When the last line was written; `None` before the first.
    last: Mutex<Option<Instant>>,
}

impl<const N: usize> LogEvery<N> {
    pub const fn new(interval: Duration) -> Self {
        LogEvery { interval, counts: [const { AtomicU64::new(0) }; N], last: Mutex::new(None) }
    }

    /// Adds each `(counter, n)`. When a line is due, takes every counter (what was counted
    /// since the previous line) and calls `log` with them.
    pub fn add(&self, adds: &[(usize, u64)], log: impl FnOnce([u64; N])) {
        for &(index, n) in adds {
            self.counts[index].fetch_add(n, Ordering::Relaxed);
        }
        let now = Instant::now();
        let Ok(mut last) = self.last.try_lock() else { return };
        if last.is_some_and(|at| now.duration_since(at) < self.interval) {
            return;
        }
        *last = Some(now);
        drop(last);
        log(std::array::from_fn(|i| self.counts[i].swap(0, Ordering::Relaxed)));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn logs_first_then_at_most_once_per_interval() {
        let log = LogEvery::<2>::new(Duration::from_secs(3600));
        let mut lines = Vec::new();
        log.add(&[(0, 1)], |counts| lines.push(counts));
        for _ in 0..5 {
            log.add(&[(1, 2)], |counts| lines.push(counts));
        }
        assert_eq!(lines, vec![[1, 0]]);
        let log = LogEvery::<2>::new(Duration::ZERO);
        let mut line = [0; 2];
        log.add(&[(0, 1), (1, 7)], |counts| line = counts);
        assert_eq!(line, [1, 7]);
        // Nothing is lost: the next line carries what was counted in between.
        let log = LogEvery::<1>::new(Duration::ZERO);
        log.add(&[(0, 1)], |_| {});
        let mut total = 0;
        log.add(&[(0, 4)], |[n]| total = n);
        assert_eq!(total, 4);
    }
}
