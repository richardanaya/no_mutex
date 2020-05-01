# no_mutex

A `no_std` mutex (does not even require `alloc`) for strict single threaded environments.

This mutex is an extraction of the [Rust web assembly mutex](https://raw.githubusercontent.com/rust-lang/rust/master/src/libstd/sys/wasm/mutex.rs) ( made for an environment without threads ). This library exists to help a user avoid including the standard library.  License ownership is of the Rust team.