#!/usr/bin/env bash

# $`(source script.sh && <function_name>)`

run() {
    source ./script/example.sh
    source ./script/parallel_read.sh
    source ./script/wasm_multithread.sh

    cargo +nightly run -v --release --features plot --bin plot -- ./result/parallel_write_parquet_[30M,native].out
    cargo +nightly run -v --release --features plot --bin plot -- ./result/parallel_write_parquet_[10M,native].out ./result/parallel_write_parquet_[10M.wasix].out
    cargo +nightly run -v --release --features plot --bin plot -- ./result/parallel_write_parquet_[10M,native]_2.out ./result/parallel_write_parquet_[10M.wasix]_2.out
    cargo +nightly run -v --release --features plot --bin plot -- ./result/parallel_write_parquet_[10M,native,cpu_util].out ./result/parallel_write_parquet_[10M,wasix,cpu_util].out

    cargo +nightly build -v --features script --release --bin scaling_analysis

    perf stat -- ./target/release/parallel_write_parquet 100 16
    perf stat -- wasmer run -v --enable-all --mapdir ./resource/:./resource/ ./target/wasm32-wasmer-wasi/release/parallel_write_parquet.wasm 10000000 16

    perf record -o ./tmp/perf-example.data -- ./target/release/parallel_write_parquet 100 16
    perf report -n -i ./tmp/perf-example.data --stdio

    perf record -F 997 --all-cpus --call-graph dwarf,16384 -g -o ./result/perf_record/perf-record-[laptop,native].data -- ./target/release/parallel_write_parquet 10000000 8
    perf record -F 997 --all-cpus --call-graph dwarf,16384 -g -o ./result/perf_record/perf-record-[laptop,wasix].data -- wasmer run --enable-all --mapdir ./resource/:./resource/ ./target/wasm32-wasmer-wasi/release/parallel_write_parquet.wasm 10000000 8

    flamegraph -o tmp/flamegraph-parallel-write-[10M,native].svg -- ./target/release/parallel_write_parquet 10000000 16
    flamegraph -o tmp/flamegraph-parallel-write-[10M,wasix].svg -- ./target/wasm32-wasmer-wasi/release/parallel_write_parquet.wasm 10000000 16

    flamegraph --perfdata ./result/perf_record/perf-record-[laptop,native].data -o ./result/perf_record/perf-record-[laptop,native].svg
    flamegraph --perfdata ./result/perf_record/perf-record-[laptop,wasix].data -o ./result/perf_record/perf-record-[laptop,wasix].svg
}

tool() {
    ./dependency/wabt/bin/wasm-objdump -x export.wasm
}

benchmark_env() {
    # show infomation on Euler nodes
    scontrol show node euler01
    lscpu

    # transfer files
    export USER=username
    scp -r ./*.toml ./script ./src ${USER}@euler.wacc.wisc.edu:~/code/wasm
    # compile wasm locally and push to remote testbed
    scp -r ./target/wasm32-wasmer-wasi/release/*.wasm ${USER}@euler.wacc.wisc.edu:~/code/wasm/target/wasm32-wasmer-wasi/release/
    mkdir -p resource

    sbatch ./script/slurm.sh && watch -n 2 --differences=cumulative "squeue -u ${USER}"

    scp -r ${USER}@euler.wacc.wisc.edu:~/code/wasm/slurm-* ./tmp/
    scp -r ${USER}@euler.wacc.wisc.edu:~/code/wasm/perf-* ./tmp/
    scp -r ${USER}@euler.wacc.wisc.edu:~/code/wasm/perf-graph-* ./tmp/
}

setup() {
    # repo
    cargo +nightly update
    cargo +nightly doc --all-featuresd
    rustup upgrade && rustup update

    # LLVM, Clang https://apt.llvm.org/
    {
        sudo su -
        apt update && apt upgrade
        apt install lsb-release wget software-properties-common gnupg clang-format clang-tidy clang-tools clang clangd libc++-dev libc++1 libc++abi-dev libc++abi1 libclang-dev libclang1 liblldb-dev libllvm-ocaml-dev libomp-dev libomp5 lld lldb llvm-dev llvm-runtime llvm python3-clang
        bash -c "$(wget -O - https://apt.llvm.org/llvm.sh)"
        # version 16
        wget https://apt.llvm.org/llvm.sh
        chmod +x llvm.sh
        sudo ./llvm.sh 16 all
        sudo apt-get install clang-16 lldb-16 lld-16
        sudo ./llvm.sh 17 all
        sudo ./llvm.sh 18 all

    }

    # Install 32-bit headers and libraries
    {
        sudo apt-get install gcc-multilib g++-multilib
        # libc6-dev-i386 lib32gcc-10-dev

    }

    # wasi-sdk https://github.com/WebAssembly/wasi-sdk
    {
        export WASI_VERSION=20
        export WASI_VERSION_FULL=${WASI_VERSION}.0
        wget https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-${WASI_VERSION}/wasi-sdk-${WASI_VERSION_FULL}-linux.tar.gz
        tar xvf wasi-sdk-${WASI_VERSION_FULL}-linux.tar.gz
    }

    # wasm targets & toolchain
    {
        rustup +nightly toolchain install nightly-x86_64-unknown-linux-gnu
        cargo install cargo-wasix && cargo wasix --version && rustup toolchain list | grep wasix
        rustup target add wasm32-wasmer-wasi
        rustup target add wasm32-wasi
        rustup target add wasm32-unknown-unknown
        # rustup target add wasm64-unknown-unknown
        # https://doc.rust-lang.org/rustc/platform-support/wasm32-wasi-preview1-threads.html#wasm32-wasi-preview1-threads
        rustup target add wasm32-wasi-preview1-threads --toolchain nightly
    }

    # Wasm runtimes
    {
        curl https://wasmtime.dev/install.sh -sSf | bash
        curl https://get.wasmer.io -sSfL | sh
    }

    # wabt tool
    {
        sudo apt install -y build-essential cmake ninja-build
        (cd ./dependency/wabt && git submodule update --init && make)
    }

    # general tools
    {
        sudo apt install linux-perf
    }

    # download datasets
    {
        # https://www.tablab.app/datasets/sample/
        # https://platform.opentargets.org/downloads
        echo ""
    }
}

version() {
    # record version
    version_info_filename=version-info.txt
    echo "___" >$version_info_filename
    wasmer --version >>$version_info_filename
    cargo wasix --version >>$version_info_filename
    wasmtime --version >>$version_info_filename
    rustc --version >>$version_info_filename
    cargo --version >>$version_info_filename
    gcc --version >>$version_info_filename
    clang --version >>$version_info_filename

}
