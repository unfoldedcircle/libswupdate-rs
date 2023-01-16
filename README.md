# SWUpdate client library wrapper

`libswupdate` is a Rust wrapper for the [SWUpdate C client library](https://sbabic.github.io/swupdate/swupdate-ipc-interface.html#client-library).

At the moment only the progress API in [progress_ipc.h](https://github.com/sbabic/swupdate/blob/master/include/progress_ipc.h)
is supported! The network IPC functions are only available as low-level bindings (see [libswupdate-sys](./libswupdate-sys)).

## Usage

This crate is not yet published on [crates.io](https://crates.io/).  
It will be published once we have an initial release after some more testing. 

Add the following to your `Cargo.toml`:

```toml
[dependencies]
libswupdate = { git = "https://github.com/unfoldedcircle/libswupdate-rs.git", rev = "$COMMIT" }
```

Replace `$COMMIT` with the desired Git commit hash.

Also see [examples](./examples) directory. 

## License

This project is licensed under either of

* [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)
  ([LICENSE-APACHE](https://github.com/unfoldedcircle/libswupdate-rs/blob/main/LICENSE-APACHE))

* [MIT License](https://opensource.org/licenses/MIT)
  ([LICENSE-MIT](https://github.com/unfoldedcircle/libswupdate-rs/blob/main/LICENSE-MIT))

at your option.
