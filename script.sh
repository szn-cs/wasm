#!/usr/bin/env bash

# $`(source script.sh && <function_name>)`

run() {
    source ./script/example.sh

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
    cargo install cargo-wasix && cargo wasix --version && rustup toolchain list | grep wasix
    rustup target add wasm32-wasi
    rustup target add wasm32-unknown-unknown
    # rustup target add wasm64-unknown-unknown
    # https://doc.rust-lang.org/rustc/platform-support/wasm32-wasi-preview1-threads.html#wasm32-wasi-preview1-threads
    rustup target add wasm32-wasi-preview1-threads --toolchain nightly

    # Wasm runtimes
    curl https://wasmtime.dev/install.sh -sSf | bash
    curl https://get.wasmer.io -sSfL | sh

    # wabt tool
    sudo apt install -y build-essential cmake ninja-build
    (cd ./dependency/wabt && git submodule update --init && make)

    # repo
    cargo +nightly update
    cargo +nightly doc --all-features

}

version() {
    # record version
    version_info_filename=version-info.txt
    wasmer --version >>$version_info_filename
    cargo wasix --version >>$version_info_filename
    wasmtime --version >>$version_info_filename
    rustc --version >>$version_info_filename
    cargo --version >>$version_info_filename

}
