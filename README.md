# Snake Game on WebAssembly written in Rust

![Screenshot](/screenshot.png)


This project has been inspired from [wasm-tetris](https://github.com/xuu/wasm-tetris/).

Is is a copy the structure of the game and write another one with it.

## Development setup

Necessary compiler target `wasm32-unknown-unknown` (Rust nightly only)

```
$ rustup target add wasm32-unknown-unknown
$ cargo +nightly install cargo-web
```

Start a web dev server

```
$ cargo +nightly web start
```

## Build

```
$ cargo +nightly web build
```

## Reference

* https://github.com/xuu/wasm-tetris/
* https://github.com/koute/cargo-web
* https://github.com/koute/stdweb
* https://developer.mozilla.org/en-US/docs/WebAssembly

## License

MIT
