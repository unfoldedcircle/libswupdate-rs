# libswupdate-sys - Raw FFI bindings to SWUpdate client library

Bindings for SWUpdate client APIs:

- [progress_ipc.h](https://github.com/sbabic/swupdate/blob/master/include/progress_ipc.h)
- [network_ipc.h](https://github.com/sbabic/swupdate/blob/master/include/network_ipc.h)

Generated from Ubuntu `libswupdate-dev` package version: `2021.11-1`.

## Usage

Add the following to your `Cargo.toml`:

```toml
[dependencies]
libswupdate-sys = "0.1"
```

## Update bindings

We use [bindgen](https://crates.io/crates/bindgen) to generate the Rust declarations from SWUpdate's C header file.  
At the moment we don't run `bindgen` at build-time, but use pre-generated bindings with a few manual fixes
(see [src/bindings.rs](./src/bindings.rs)).

Bindgen command:
```shell
cargo install bindgen-cli
bindgen wrapper.h -o src/bindings.rs \
    --allowlist-function '(^.*ipc.*|^get_.*_socket|^swupdate.*)' \
    --allowlist-type 'msgtype.*' \
    --allowlist-var '(IPC_.*|SWUPDATE_.*|CMD_.*|SOCKET_PROGRESS_PATH)'
```

## License

This project is licensed under either of

* [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)
  ([LICENSE-APACHE](https://github.com/unfoldedcircle/libswupdate-rs/blob/main/LICENSE-APACHE))

* [MIT License](https://opensource.org/licenses/MIT)
  ([LICENSE-MIT](https://github.com/unfoldedcircle/libswupdate-rs/blob/main/LICENSE-MIT))

at your option.
