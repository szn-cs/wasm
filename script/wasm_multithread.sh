#!/usr/bin/env bash

run() {

    { # native
        cargo +nightly build -v --features wasm_multithread
        RUST_BACKTRACE=1 ./target/debug/read_parquet_parallel
    }

    { # wasm-wasi - parquet2 → ❌ compilation error with parquet2
        cargo +nightly build -v --target wasm32-wasi --release --features wasm_multithread >./tmp/parquet2-compilation-error.txt 2>&1
        wasmer run ./target/wasm32-wasi/release/read_parquet_parallel.wasm
        wasmtime ./target/wasm32-wasi/release/read_parquet_parallel.wasm
    }

    { # wasm32-wasi-preview1-threads - parquet2 → ❌ error: linking with `rust-lld` failed: exit status: 1
        cargo +nightly build -v --target wasm32-wasi-preview1-threads --release --features wasm_multithread
        RUST_BACKTRACE=1 wasmer run --mapdir ./resource/:./resource/ ./target/wasm32-wasi-preview1-threads/release/read_parquet_parallel.wasm
    }

    { # wasm32-wasmer-wasi → ❌ error: linking with `rust-lld` failed: exit status: 1
        cargo +nightly wasix build -v --features wasm_multithread
        wasmer run --mapdir ./resource/:./resource/ ./target/wasm32-wasmer-wasi/release/read_parquet_parallel.wasm
    }

}
