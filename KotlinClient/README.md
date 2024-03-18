#  Kotlin/WASM

This is a Kotlin/WASM implementation of the benchmark.

This program renders Data Entries fetched from a corresponding API (KotlinDataStore).
It is used for performance testing Kotlin/WASM client-side + Kotlin/SpringBoot server-side E2E chain.


## Running the app

Kotlin/WASM has a dependency to Java.

In order to start the client:
1. Build it by executing `./gradlew clean build` from `<projectRoot>/KotlinClient`

2. Serve the client app via, for example, `npx http-server -p 8087` from `<projectRoot>/KotlinClient/build/dist/wasmJs/productionExecutable/`
