#  Rust/WASM

This is a Rust/WASM implementation of the benchmark. It uses Actix framework.

This app contains an API with an /api/data/{count} endpoint which generates 'count' number of entries in the format:
```
[{
    id: 1,
    label: "First cool entry"
},
{
    id: 2,
    label: "Second cool entry"
}]
```
It is used for performance testing Leptos(Rust)/WASM client-side + Rust/Actix server-side E2E chain.


## Running the app

The project has a dependency to Rust.

In order to build & start the app for performance testing:
1. Build it by executing `cargo build --release` from `<projectRoot>/RustDataStore`

2. Run the app via, for example, `./rust-actix-web-rest-api` from `<projectRoot>/RustDataStore/target/release`
