//! # Backoff
//!
//! Backoff provides the base components for implementing backoff and retry operations.
//!
//! ```rust
//! use backoff_rs::ExponentialBackoffBuilder;
//! use std::time::Duration;
//!
//! fn main() {
//!     let bo = ExponentialBackoffBuilder::default()
//!         .factor(1.75)
//!         .interval(Duration::from_millis(500))
//!         .jitter(Duration::from_millis(150))
//!         .max(Duration::from_secs(5))
//!         .build();
//!
//!     for attempt in 0..=5 {
//!         println!("{:?}", bo.duration(attempt));
//!     }
//! }
//! ```

use rand::Rng;
use std::time::Duration;

/// Configures an ExponentialBackoff instance for use.
pub struct ExponentialBackoffBuilder {
    factor: f64,
    interval: Duration,
    jitter: Duration,
    max: Option<Duration>,
}

impl Default for ExponentialBackoffBuilder {
    #[inline]
    fn default() -> Self {
        Self {
            factor: 1.75,
            interval: Duration::from_millis(500),
            jitter: Duration::from_millis(150),
            max: None,
        }
    }
}

impl ExponentialBackoffBuilder {
    /// Factor sets a backoff factor for the backoff algorithm.
    #[inline]
    pub const fn factor(mut self, factor: f64) -> Self {
        self.factor = factor;
        self
    }

    /// Interval sets base wait interval for the backoff algorithm.
    pub const fn interval(mut self, interval: Duration) -> Self {
        self.interval = interval;
        self
    }

    /// Jitter sets the maximum jitter for the backoff algorithm.
    #[inline]
    pub const fn jitter(mut self, jitter: Duration) -> Self {
        self.jitter = jitter;
        self
    }

    /// Max sets the maximum timeout despite the number of attempts. none/zero is the default.
    #[inline]
    pub const fn max(mut self, max: Duration) -> Self {
        self.max = Some(max);
        self
    }

    /// finalizes the configuration and returns a usable [Exponential] instance.
    #[inline]
    pub const fn build(self) -> Exponential {
        Exponential {
            factor: self.factor,
            interval: self.interval.as_nanos() as f64,
            jitter: self.jitter.as_nanos() as f64,
            max: match self.max {
                Some(d) => Some(d.as_nanos() as u64),
                None => None,
            },
        }
    }
}

/// An Exponential Backoff instance for calculating backoff durations.
pub struct Exponential {
    factor: f64,
    interval: f64,
    jitter: f64,
    max: Option<u64>,
}

impl Exponential {
    /// returns the calculated backoff duration for backoff and retries based on the attempt.
    pub fn duration(&self, attempt: usize) -> Duration {
        let nanoseconds = (self.factor.powi(attempt as i32) * self.interval
            + rand::thread_rng().gen_range(0.0..=self.jitter)) as u64;
        match self.max {
            Some(max) if nanoseconds > max => Duration::from_nanos(max),
            _ => Duration::from_nanos(nanoseconds),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn no_jitter() {
        let bo = ExponentialBackoffBuilder::default()
            .jitter(Duration::default())
            .max(Duration::from_secs(5))
            .build();

        assert_eq!(bo.duration(0), Duration::from_millis(500));
        assert_eq!(bo.duration(1), Duration::from_millis(875));
        assert_eq!(bo.duration(2), Duration::from_nanos(1531250000));
        assert_eq!(bo.duration(3), Duration::from_nanos(2679687500));
        assert_eq!(bo.duration(4), Duration::from_nanos(4689453125));
        assert_eq!(bo.duration(5), Duration::from_secs(5));
    }

    #[test]
    fn with_jitter() {
        let bo = ExponentialBackoffBuilder::default()
            .jitter(Duration::default())
            .max(Duration::from_secs(5))
            .build();

        assert!(bo.duration(0) <= Duration::from_millis(500));
        assert!(bo.duration(1) <= Duration::from_millis(875));
        assert!(bo.duration(2) <= Duration::from_nanos(1531250000));
        assert!(bo.duration(3) <= Duration::from_nanos(2679687500));
        assert!(bo.duration(4) <= Duration::from_nanos(4689453125));
        assert!(bo.duration(5) <= Duration::from_secs(5));
    }
}
