# Rargo resolver plugin to respect MSRVs

Currently still in development

## Usage:

First obtain rargo, e.g. by running `cargo install --git https://github.com/msrv-rs/rargo`.

Then, specify a resolver plugin. There are two types of plugins:

1. executable plugins. Rargo invokes a binary and talks with it via stdout/stdin.
2. library plugins. Rargo loads a dynamic library (.so/.dll/...) using the libloading crate and invokes its functions.

It is easier to build plugins with the first method, but the method has a communication overhead, while the dynamic plugin overhead is negligible.

This repo contains two example executable plugins, one written in bash (`cargo-resolver-hook` in the top directory), the other in Rust (`src/main.rs`). It also contains a library plugin, found at `src/lib.rs`.

In order for rargo to use the resolver plugin, you need to specify it in `.cargo/config`.

Executable plugins are specified like this:

```toml
[resolver_hook]
kind = "process"
path = "/path/to/msrv-rs/cargo-resolver-hook"
```

Library plugins like this:

```toml
[resolver_hook]
kind = "plugin"
path = "/path/to/msrv-rs/target/release/libcargo_resolver_hook.so"
params = { msrv = "1.40.0" }
```

The params array is an optional toml value that is passed to the plugin during initialization. Only library plugins support it.
