# Rust-based mini postcode lookup

A Learning rust experiment.

Trying to replicate this postcode lookup https://github.com/mysociety/mini-postcode-lookup/ in rust, and use wasm.

See that repo for details on the data licencing. 

To run this application:

```
cargo run "SW1W 0NY,PO16 7GZ"
```


Notes:

This approach optimises for a small rust binary. See `script/build` for the wasm approach. 

RUSTFLAGS="-Zlocation-detail=none" cargo +nightly build -Z build-std=std,panic_abort   -Z build-std-features="panic_immediate_abort,optimize_for_size"   --target x86_64-unknown-linux-gnu --release
