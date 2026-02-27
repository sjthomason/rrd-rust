//! Rust API for [`librrd`](https://oss.oetiker.ch/rrdtool/index.en.html).
//!
//! The Rust wrappers for supported `librrd` functions are in [`ops`], e.g. [`ops::create`].
//!
//! See the `/examples` directory or `tests/tutorial.rs` for detailed examples. The latter is a
//! recreation of <https://oss.oetiker.ch/rrdtool/tut/rrdtutorial.en.html>, which uses the CLI
//! tools, with this library.
//!
//! # Logging
//!
//! If unexpected behavior is observed, it can be helpful to see exactly what paramters are being
//! provided to the underlying `librrd` functions. For operations that do any level of mapping of
//! their input into `librrd` input, the [`log`](https://crates.io/crates/log) crate is used at
//! `debug` level, so log output can be enabled with `RUST_LOG=rrd=debug` (if using `env_logger`)
//! or other means of configuring `log`.

#![deny(missing_docs)]

// TODO get confirmation from upstream about librrd thread safety
pub mod data;
pub mod error;
pub mod ops;
pub mod util;

// `chrono::DateTime` and `chrono::Utc` are used for timestamps, so this is provided to allow
// easy access without a separate `chrono` dependency.
#[cfg(feature = "chrono")]
pub use chrono;

/// The point in time associated with a data point.
#[cfg(feature = "chrono")]
pub type Timestamp = chrono::DateTime<chrono::Utc>;

/// The point in time associated with a data point.
#[cfg(not(feature = "chrono"))]
pub type Timestamp = std::time::SystemTime;

/// Internal extensions for [`Timestamp`]
pub(crate) trait TimestampExt {
    /// Returns the timestamp as seconds since epoch.
    fn as_time_t(&self) -> rrd_sys::time_t;

    /// Converts seconds since epoch into [`Timestamp`]
    fn from_time_t(secs: rrd_sys::time_t) -> Self;
}

#[cfg(feature = "chrono")]
impl TimestampExt for Timestamp {
    fn as_time_t(&self) -> rrd_sys::time_t {
        self.timestamp()
    }

    fn from_time_t(secs: rrd_sys::time_t) -> Self {
        Timestamp::from_timestamp(secs, 0).expect("invalid timestamp")
    }
}

#[cfg(not(feature = "chrono"))]
impl TimestampExt for Timestamp {
    fn as_time_t(&self) -> rrd_sys::time_t {
        self.duration_since(std::time::SystemTime::UNIX_EPOCH)
            .expect("system clock before unix epoch")
            .as_secs() as rrd_sys::time_t
    }

    fn from_time_t(secs: rrd_sys::time_t) -> Self {
        std::time::SystemTime::UNIX_EPOCH
            + std::time::Duration::from_secs(secs.try_into().expect("invalid timestamp"))
    }
}

/// How to aggregate primary data points in a RRA.
///
/// See [`ops::create::Archive`] and [`ops::graph::elements::Def`].
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConsolidationFn {
    Avg,
    Min,
    Max,
    Last,
}

impl ConsolidationFn {
    pub(crate) fn as_arg_str(&self) -> &str {
        match self {
            ConsolidationFn::Avg => "AVERAGE",
            ConsolidationFn::Min => "MIN",
            ConsolidationFn::Max => "MAX",
            ConsolidationFn::Last => "LAST",
        }
    }
}
