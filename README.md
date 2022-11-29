# What is this?

This crate exports four things:

- A compatibility/abstraction layer for writing async-executor agnostic code. We support two async executors: async-std and tokio. Each may be toggled with a feature flag.
- A compatibility/abstraction layer for writing async channel agnostic code. We support three async channel implementations: async-std-channels. Each may be toggled with a feature flag.
- A library exporting a bunch of useful async primitives.
- A tracing configuration layer optionally supporting console and opentelemetry integration.




