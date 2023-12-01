#!/usr/bin/env bash

# $`(source script.sh && <function_name>)`

run() {
    cargo +nightly run --bin read_parallel

    # TODO: fix feature definintion
    cargo build --target wasm32-unknown-unknown --release --features wasm_multithread
    cargo build --target wasm32-wasi --release
}

benchmark_env() {
    sbatch ./slurm.sh && watch -n 2 --differences=cumulative "squeue -u <user>"
}

setup() {
    rustup upgrade

    # LLVM, Clang https://apt.llvm.org/
    sudo su -
    apt update && apt upgrade
    apt install lsb-release wget software-properties-common gnupg clang-format clang-tidy clang-tools clang clangd libc++-dev libc++1 libc++abi-dev libc++abi1 libclang-dev libclang1 liblldb-dev libllvm-ocaml-dev libomp-dev libomp5 lld lldb llvm-dev llvm-runtime llvm python3-clang
    bash -c "$(wget -O - https://apt.llvm.org/llvm.sh)"

    # wasi-sdk https://github.com/WebAssembly/wasi-sdk
    export WASI_VERSION=20
    export WASI_VERSION_FULL=${WASI_VERSION}.0
    wget https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-${WASI_VERSION}/wasi-sdk-${WASI_VERSION_FULL}-linux.tar.gz
    tar xvf wasi-sdk-${WASI_VERSION_FULL}-linux.tar.gz

    # wasm targets
    rustup target add wasm32-wasi
    rustup target add wasm32-unknown-unknown

    # Wasm runtimes
    curl https://wasmtime.dev/install.sh -sSf | bash
    curl https://get.wasmer.io -sSfL | sh

}
