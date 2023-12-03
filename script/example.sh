#!/usr/bin/env bash

run() {
    { # native ✅
        cargo +nightly run --bin example --features example

        cargo +nightly build -v --features example
        ./target/debug/example
    }

    { # wasm32-unknown-unknown → ❌ missing _start function
        cargo +nightly build -v --target wasm32-unknown-unknown --features example
        wasmer run ./target/wasm32-unknown-unknown/debug/example.wasm
        mkdir -p tmp && ./dependency/wabt/bin/wasm2wat ./target/wasm32-unknown-unknown/debug/example.wasm >tmp/example-\[wasm32-unknown-unknown,debug\].wat
    }

    { # wasm-wasi - example ✅
        cargo +nightly build -v --target wasm32-wasi --release --features example
        wasmer run ./target/wasm32-wasi/release/example.wasm
        wasmtime ./target/wasm32-wasi/release/example.wasm
        mkdir -p tmp && ./dependency/wabt/bin/wasm2wat ./target/wasm32-wasi/release/example.wasm >tmp/example-\[wasm32-wasi,release\].wat

        cargo +nightly build -v --target wasm32-wasi --features example
        wasmer run ./target/wasm32-wasi/debug/example.wasm
        mkdir -p tmp && ./dependency/wabt/bin/wasm2wat ./target/wasm32-wasi/debug/example.wasm >tmp/example-\[wasm32-wasi,debug\].wat
        hexdump -C -n64 ./target/wasm32-wasi/debug/example.wasm | head
    }

    { # wasm32-wasi-preview1-threads - parquet2 ✅
        cargo +nightly build -v --target wasm32-wasi-preview1-threads --release --features example
        RUST_BACKTRACE=1 wasmer run ./target/wasm32-wasi-preview1-threads/release/example.wasm
    }

    { # wasm32-wasmer-wasi ✅
        cargo +nightly wasix run --bin example --release --features example

        cargo +nightly wasix build -v --features example
        wasmer run ./target/wasm32-wasmer-wasi/release/example.wasm
    }

}
