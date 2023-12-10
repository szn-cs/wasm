#!/usr/bin/env bash

run() {

    { # native ✅
        CC=clang-17 cargo +nightly build -v --features parallel_read &&
            RUST_BACKTRACE=1 ./target/debug/parallel_read &&
            RUST_BACKTRACE=1 ./target/debug/generic_parallel_read &&
            RUST_BACKTRACE=1 ./target/debug/parallel_write_parquet
    }

    { # wasm-wasi → ❌ runtime error: parallel_write_parquet allocation of memory limit
        CC=clang-17 cargo +nightly build -v --target wasm32-wasi --release --features parallel_read
        wasmer run -v --mapdir ./resource/:./resource/ ./target/wasm32-wasi/release/parallel_read.wasm
        wasmer run -v --mapdir ./resource/:./resource/ ./target/wasm32-wasi/release/generic_parallel_read.wasm
        wasmer run -v --mapdir ./resource/:./resource/ ./target/wasm32-wasi/release/parallel_write_parquet.wasm >"./tmp/compilation_error [wasm32-wasi, parallel_read].log" 2>&1
        wasmtime ./target/wasm32-wasi/release/parallel_read.wasm
    }

    { # wasm32-wasi-preview1-threads → ❌ runtime  execution failure — threads hanging (program correctness influenced, probably because of thread implementation issues)
        CC=clang-17 cargo +nightly build -v --target wasm32-wasi-preview1-threads --release --features parallel_read
        RUST_BACKTRACE=1 wasmer run -v --mapdir ./resource/:./resource/ ./target/wasm32-wasi-preview1-threads/release/parallel_read.wasm
    }

    { # wasm32-wasmer-wasi → ✅
        CC=clang-17 cargo +nightly wasix build -v --release --features parallel_read
        # >./tmp/compilation.log 2>&1

        set RUST_BACKTRACE=full
        wasmer run -v --mapdir ./resource/:./resource/ ./target/wasm32-wasmer-wasi/release/parallel_read.wasm
        wasmer run -v --mapdir ./resource/:./resource/ ./target/wasm32-wasmer-wasi/release/generic_parallel_read.wasm
        wasmer run -v --enable-all --mapdir ./resource/:./resource/ ./target/wasm32-wasmer-wasi/release/parallel_write_parquet.wasm 50000000 8
        wasmer cache clean

        # Max size = ~536,865,000
        # calc := size * 4 byte entry * 2 stack/heap; i.e. 4G / 4 / 2 - stack used by binary (~50 KB) - parallel_write function implementation used stack & heap;

        perf mem -a wasmer run -v --mapdir ./resource/:./resource/ ./target/wasm32-wasmer-wasi/release/parallel_write_parquet.wasm 10000000

    }
}
