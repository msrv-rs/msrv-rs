Example .cargo/config usage:

```toml
# [resolver_hook]
# kind = "process"
# path = "/path/to/msrv-rs/cargo-resolver-hook"

# [resolver_hook]
# kind = "plugin"
# path = "/path/to/msrv-rs/target/release/libcargo_resolver_hook.so"
# params = { msrv = "1.40.0" }

[resolver_hook]
kind = "plugin"
path = "/path/to/msrv-rs/target/release/libcargo_resolver_hook.so"
params = { exclude = ["adler32 1.0.4", "adler32 1.0.3"] }
```
