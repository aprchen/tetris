# tetris clone
rust game 01


```rust
# run with dev
 cargo run --features bevy/dynamic
```

lint
```rust
cargo clippy
```
build
```rust
# build wasm 
cargo build --release --target wasm32-unknown-unknown
# bind wasm web
wasm-bindgen --out-name tetris_wasm --out-dir docs/wasm --target web target/wasm32-unknown-unknown/release/tetris.wasm
```


- [bevy](https://github.com/bevyengine/bevy)
- [bevy wasm build tutorial](https://github.com/bevyengine/bevy/tree/main/examples#wasm)