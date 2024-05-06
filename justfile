async_std := '--cfg async_executor_impl=\"async-std\" --cfg async_channel_impl=\"async-std\"'
tokio := '--cfg async_executor_impl=\"tokio\" --cfg async_channel_impl=\"tokio\"'
async_std_flume := '--cfg async_executor_impl=\"async-std\" --cfg async_channel_impl=\"flume\"'
tokio_flume := '--cfg async_executor_impl=\"tokio\" --cfg async_channel_impl=\"flume\"'


matrix *args:
    {{args}}
    env RUSTFLAGS="{{async_std}}" RUSTDOCFLAGS="{{async_std}}" {{args}}
    env RUSTFLAGS="{{tokio}}" RUSTDOCFLAGS="{{tokio}}" {{args}}
    env RUSTFLAGS="{{async_std_flume}}" RUSTDOCFLAGS="{{async_std_flume}}" {{args}}
    env RUSTFLAGS="{{tokio_flume}}" RUSTDOCFLAGS="{{tokio_flume}}" {{args}}

test *args:
    cargo test {{args}}

test-all *args:
    just matrix just test {{args}}

test-all-logging-utils *args:
    just test-all --features logging-utils {{args}}

clippy:
    cargo clippy --all-targets --workspace --release --bins --tests --examples --features="logging-utils" -- -D warnings

clippy-all *args:
    just matrix just clippy
