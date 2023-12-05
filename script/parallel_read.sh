#!/usr/bin/env bash

run() {

    { # native ✅
        CC=clang-17 cargo +nightly build -v --features parallel_read &&
            RUST_BACKTRACE=1 ./target/debug/parallel_read
    }

    { # wasm-wasi → ✅
        CC=clang-17 cargo +nightly build -v --target wasm32-wasi --release --features parallel_read
        wasmer run --mapdir ./resource/:./resource/ ./target/wasm32-wasi/release/parallel_read.wasm
        wasmtime ./target/wasm32-wasi/release/parallel_read.wasm
    }

    { # wasm32-wasi-preview1-threads → ❌ runtime  execution failure (program correctness influenced, probably because of thread implementation issues)
        CC=clang-17 cargo +nightly build -v --target wasm32-wasi-preview1-threads --release --features parallel_read
        RUST_BACKTRACE=1 wasmer run --mapdir ./resource/:./resource/ ./target/wasm32-wasi-preview1-threads/release/parallel_read.wasm
    }

    { # wasm32-wasmer-wasi → ✅
        CC=clang-17 cargo +nightly wasix build -v --release --features parallel_read >compilation.log 2>&1
        wasmer run --mapdir ./resource/:./resource/ ./target/wasm32-wasmer-wasi/release/parallel_read.wasm
    }

}
