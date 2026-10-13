//! Per-API-key rate limit: a guard against runaway loops and abuse, not a product limit.
//!
//! A token bucket per key: `ZPL_RATE_LIMIT_PER_SECOND` requests a second sustained
//! (default 50) with bursts of `ZPL_RATE_LIMIT_BURST` (default 200). Either set to 0 turns
//! the limit off. Only applies with API keys enabled.
//!
//! Buckets live in memory per process, so with several replicas a key gets about that
//! limit on each: the limit is approximate. Keys idle for [`IDLE`] are forgotten (their
//! bucket has refilled long before) and at most [`MAX_KEYS`] are tracked.

use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use uuid::Uuid;

pub const DEFAULT_PER_SECOND: u32 = 50;
pub const DEFAULT_BURST: u32 = 200;
/// Buckets unused this long are dropped at the next sweep.
pub const IDLE: Duration = Duration::from_secs(5 * 60);
const SWEEP_INTERVAL: Duration = Duration::from_secs(60);
/// Most keys tracked at once; beyond it refilled buckets go first, then the least recent.
pub const MAX_KEYS: usize = 100_000;

pub struct RateLimiter {
    per_second: u32,
    burst: u32,
    max_keys: usize,
    state: Mutex<State>,
    clock: Box<dyn Fn() -> Instant + Send + Sync>,
}

struct State {
    buckets: HashMap<Uuid, Bucket>,
    swept: Instant,
}

struct Bucket {
    tokens: f64,
    at: Instant,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Decision {
    /// Go ahead; `remaining` more requests may follow right away.
    Allowed { remaining: u32 },
    /// Over the limit; a request succeeds again after `retry_after` seconds (at least 1).
    Limited { retry_after: u64 },
}

impl RateLimiter {
    /// `None` when either number is 0 (no limit).
    pub fn new(per_second: u32, burst: u32) -> Option<RateLimiter> {
        let state = Mutex::new(State { buckets: HashMap::new(), swept: Instant::now() });
        let clock = Box::new(Instant::now);
        (per_second > 0 && burst > 0).then_some(RateLimiter { per_second, burst, max_keys: MAX_KEYS, state, clock })
    }

    /// From `ZPL_RATE_LIMIT_PER_SECOND` and `ZPL_RATE_LIMIT_BURST`, defaults when unset.
    pub fn from_env() -> Result<Option<RateLimiter>, String> {
        let var = |name| std::env::var(name).ok();
        from_vars(var("ZPL_RATE_LIMIT_PER_SECOND").as_deref(), var("ZPL_RATE_LIMIT_BURST").as_deref())
    }

    pub fn per_second(&self) -> u32 {
        self.per_second
    }

    pub fn burst(&self) -> u32 {
        self.burst
    }

    /// Takes a token from `key`'s bucket, if it has one.
    pub fn check(&self, key: Uuid) -> Decision {
        self.check_at(key, (self.clock)())
    }

    /// [`check`](Self::check) at time `now`.
    pub fn check_at(&self, key: Uuid, now: Instant) -> Decision {
        let (rate, burst) = (f64::from(self.per_second), f64::from(self.burst));
        let mut state = self.state.lock().expect("rate limit lock");
        if now.saturating_duration_since(state.swept) >= SWEEP_INTERVAL {
            state.buckets.retain(|_, b| now.saturating_duration_since(b.at) < IDLE);
            state.swept = now;
        }
        if state.buckets.len() >= self.max_keys && !state.buckets.contains_key(&key) {
            state.make_room(now, Duration::from_secs_f64(burst / rate), self.max_keys);
        }
        let bucket = state.buckets.entry(key).or_insert(Bucket { tokens: burst, at: now });
        // Concurrent requests may take their `now` out of order; never move time back.
        let elapsed = now.saturating_duration_since(bucket.at).as_secs_f64();
        bucket.tokens = (bucket.tokens + elapsed * rate).min(burst);
        bucket.at = bucket.at.max(now);
        if bucket.tokens >= 1.0 {
            bucket.tokens -= 1.0;
            Decision::Allowed { remaining: bucket.tokens as u32 }
        } else {
            let retry_after = ((1.0 - bucket.tokens) / rate).ceil().max(1.0) as u64;
            Decision::Limited { retry_after }
        }
    }

    #[cfg(test)]
    pub(crate) fn tracked_keys(&self) -> usize {
        self.state.lock().expect("rate limit lock").buckets.len()
    }

    /// For tests: reads the time from `clock` instead of [`Instant::now`].
    #[cfg(test)]
    pub(crate) fn with_clock(mut self, clock: impl Fn() -> Instant + Send + Sync + 'static) -> Self {
        self.clock = Box::new(clock);
        self
    }

    #[cfg(test)]
    pub(crate) fn with_max_keys(mut self, max_keys: usize) -> Self {
        self.max_keys = max_keys;
        self
    }
}

impl State {
    /// Drops buckets that have refilled (forgetting them changes nothing), and if that is
    /// not enough, the least recently used one.
    fn make_room(&mut self, now: Instant, refill: Duration, max_keys: usize) {
        self.buckets.retain(|_, b| now.saturating_duration_since(b.at) < refill);
        if self.buckets.len() >= max_keys {
            let oldest = self.buckets.iter().min_by_key(|(_, b)| b.at).map(|(key, _)| *key);
            if let Some(oldest) = oldest {
                self.buckets.remove(&oldest);
            }
        }
    }
}

fn from_vars(per_second: Option<&str>, burst: Option<&str>) -> Result<Option<RateLimiter>, String> {
    let parse = |name: &str, value: Option<&str>, default: u32| match value.map(str::trim).filter(|v| !v.is_empty()) {
        None => Ok(default),
        Some(v) => v.parse::<u32>().map_err(|_| format!("{name}: expected a whole number, got {v:?}")),
    };
    Ok(RateLimiter::new(
        parse("ZPL_RATE_LIMIT_PER_SECOND", per_second, DEFAULT_PER_SECOND)?,
        parse("ZPL_RATE_LIMIT_BURST", burst, DEFAULT_BURST)?,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn configuration() {
        let limiter = from_vars(None, None).unwrap().unwrap();
        assert_eq!((limiter.per_second(), limiter.burst()), (DEFAULT_PER_SECOND, DEFAULT_BURST));
        let limiter = from_vars(Some(" 5 "), Some("")).unwrap().unwrap();
        assert_eq!((limiter.per_second(), limiter.burst()), (5, DEFAULT_BURST));
        assert!(from_vars(Some("0"), None).unwrap().is_none());
        assert!(from_vars(None, Some("0")).unwrap().is_none());
        assert!(from_vars(Some("fast"), None).err().unwrap().contains("ZPL_RATE_LIMIT_PER_SECOND"));
        assert!(from_vars(None, Some("-1")).is_err());
    }

    #[test]
    fn idle_keys_are_forgotten() {
        let limiter = RateLimiter::new(1, 1).unwrap();
        let start = Instant::now();
        for i in 0..10 {
            limiter.check_at(Uuid::from_u128(i), start);
        }
        assert_eq!(limiter.tracked_keys(), 10);
        // Just short of idle, a sweep keeps every key; a minute later (the sweep interval)
        // only key 0, used in between, and the newly seen key 1 are left.
        let almost = start + IDLE - Duration::from_secs(1);
        limiter.check_at(Uuid::from_u128(0), almost);
        assert_eq!(limiter.tracked_keys(), 10);
        limiter.check_at(Uuid::from_u128(1), almost + SWEEP_INTERVAL);
        assert_eq!(limiter.tracked_keys(), 2);
    }

    #[test]
    fn tracked_keys_are_capped() {
        let limiter = RateLimiter::new(1, 10).unwrap().with_max_keys(3);
        let start = Instant::now();
        for i in 0..3 {
            limiter.check_at(Uuid::from_u128(i), start + Duration::from_secs(i as u64));
        }
        // None has refilled (10 s), so the least recently used, key 0, makes room.
        limiter.check_at(Uuid::from_u128(3), start + Duration::from_secs(3));
        assert_eq!(limiter.tracked_keys(), 3);
        for i in 0..5 {
            assert_eq!(limiter.check_at(Uuid::from_u128(0), start + Duration::from_secs(3)), Decision::Allowed {
                remaining: 9 - i
            });
        }
    }
}
