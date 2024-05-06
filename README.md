# What is this?

This crate exports four things:

- A compatibility/abstraction layer for writing async-executor agnostic code. We
  support two async executors: async-std and tokio. Each may be toggled with a
  configuration flag.
- A compatibility/abstraction layer for writing async channel agnostic code. We
  support three async channel implementations: async-std, tokio and flume. Each
  may be toggled with a configuration flag.
- A library exporting a bunch of useful async primitives.
- A tracing configuration layer optionally supporting console and opentelemetry integration.

# Example usage
By default the `async-std` executor and channels are used.

To use tokio:

```bash
RUSTFLAGS='--cfg async_executor_impl="tokio" --cfg async_channel_impl="tokio"' cargo build
```

`async_executor_impl` may be either `tokio` or `async-std`. `async_channel_impl`
may be either `tokio`, `async-std`, or `flume`.

Note that using `tokio` channels requires `tokio` to be the runtime.
