//! Async compatibility layer
//! meant to control abstractions between tokio and async_std
//! with a feature flag toggle
//! while exposing the same interface for general consumption between both

#[cfg(all(async_executor_impl = "async-std", async_executor_impl = "tokio"))]
std::compile_error!(
    "Both cfg options \"async-std\" and \"tokio\" must not be concurrently enabled for this crate."
);

/// abstraction over both `tokio` and `async-std`, making it possible to use either based on a feature flag
#[cfg(not(async_executor_impl = "tokio"))]
#[path = "art/async-std.rs"]
pub mod art;

/// abstraction over both `tokio` and `async-std`, making it possible to use either based on a feature flag
#[cfg(async_executor_impl = "tokio")]
#[path = "art/tokio.rs"]
pub mod art;

pub mod channel;

pub mod async_primitives;

#[cfg(feature = "logging-utils")]
pub mod logging;
