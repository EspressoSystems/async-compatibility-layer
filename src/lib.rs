//! Async compatability layer
//! meant to control abstractions between tokio and async_std
//! with a feature flag toggle
//! while exposing the same interface for general consumption between both

#[cfg(all(feature = "async-std-executor", feature = "tokio-executor"))]
std::compile_error!("Both feature \"async-std-executor\" and feature \"tokio-executor\" must not be concurrently enabled for this crate.");

#[cfg(not(any(feature = "async-std-executor", feature = "tokio-executor")))]
compile_error! {"Either feature \"async-std-executor\" or feature \"tokio-executor\" must be enabled for this crate."}

/// abstraction over both `tokio` and `async-std`, making it possible to use either based on a feature flag
#[cfg(feature = "async-std-executor")]
#[path = "art/async-std.rs"]
pub mod art;

/// abstraction over both `tokio` and `async-std`, making it possible to use either based on a feature flag
#[cfg(feature = "tokio-executor")]
#[path = "art/tokio.rs"]
pub mod art;

pub mod channel;

pub mod async_primitives;

#[cfg(feature = "logging-utils")]
pub mod logging;
