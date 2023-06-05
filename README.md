## Run

`cargo build && wasm-bindgen target/wasm32-unknown-unknown/debug/test.wasm --out-dir web --no-typescript --target web`

Host the `web` folder with an HTTP server of your choosing.

Should fail on start with:

```
panicked at 'misaligned pointer dereference: address must be a multiple of 0x8 but is 0x4', src/main.rs:3:1
```
