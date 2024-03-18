#  Rust/WASM

This is a Rust/WASM implementation of the benchmark. It uses Leptos framework.

This app renders Data Entries fetched from a corresponding API (RustDataStore).
It is used for performance testing Leptos(Rust)/WASM client-side + Rust/Actix server-side E2E chain.


## Running the app

Leptos has a dependency to Rust.
It is suggested to use (Trunk tool)[https://trunkrs.dev/] for building the app.

In order to build & start the app for performance testing:
1. Build it by executing ` trunk build --release` from `<projectRoot>/rust-client`

2. Serve the client app via, for example, `npx http-server -p 8087` from `<projectRoot>/rust-client/dist`

