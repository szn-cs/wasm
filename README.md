# wasm disaggregated-storage pushdown


# WASM Threading parallelisation of pushdown operations: 

- Architectures storage: 
    - Storage is underpowered (# of cores or speed is not very great); Low-end edge computing; Types of operations supported are limited; 
    - Base layer and an intermediary (with more mem & compute power but with higher latency) that is doing client handling (buffering between the two); 
	
- Scale-out: handling 100k or 1000s of requests per second. 
    - what is worthwhile and what budget there is to parallelize. Typically many of these operations in terms of pushdown are not parallelizable;
	- Many of the opreations are pushdown to a highlt optimized specialized services for a particular operation (e.g. parsing, aggregation);  

- CSV & Parquet
	- Parsing a CSV can be broken into chunks although the bottleneck is the disk; But parsing a Parquet file has more indexing built into it; Especially if doing aggregation can be split more easily and do parallelism on them; 

___

# Notes
- Software fault isolation
- Fassm project OSS
- SQL queries + UDF → custom user impl; WASM user defined applications;
- Wasmtime is used by 50% of respondents, up from 36% in 2022, followed by Wasmer (40%, up from 36%), Wasm3 (18%, up from 14%), WasmEdge (15%, up from 7%), and WebAssembly Micro Runtime (WAMR) (12%, up from 6%). Wazeroo is used by 12% and it was not asked about in previous surveys.
- Wrap wasm sandbox into another container VM; Firecracker; 
- Create a market place of WASM plugins that can manipulate raw data. e.g. implement table joins targeting specific tradeoffs; 

# Resouces
- https://github.com/dsrg-uoft/LangBench 
- https://docs.google.com/document/d/14pN91vSG15tKaK3jcQYcHUC_aMlZwV7OaV2HMtau5NI/edit#heading=h.zd3qu2ajtsci
- https://drive.google.com/drive/folders/1RMy9SZ-34iHI8cfmK6wWZ_RtFOJ25OfS
- https://webassembly.github.io/spec/
    - https://webassembly.github.io/spec/core/index.html
- WASI spec https://wasi.dev/
    - Wasmer extension WASIX https://wasix.org/docs 
    - https://doc.rust-lang.org/stable/nightly-rustc/rustc_target/spec/wasm32_wasi/index.html 
    - https://github.com/bytecodealliance/wasmtime/blob/main/docs/WASI-tutorial.md
- Threads draft spec & proposal
    - https://github.com/WebAssembly/threads
    - https://github.com/WebAssembly/wasi-threads
- Capability based security https://github.com/bytecodealliance/wasmtime/blob/main/docs/WASI-capabilities.md


- tools
    - Binary tools https://github.com/webassembly/wabt
    - https://github.com/WebAssembly/binaryen
    - wasm-opt
    - https://bytecodealliance.github.io/cargo-wasi/steps.html
    - Datasets https://www.tablab.app/datasets/sample/parquet
-   Rust wasm modules for npm https://rustwasm.github.io/docs/wasm-pack/
- https://github.com/rustwasm
- https://doc.rust-lang.org/rustc/ 
- https://developer.mozilla.org/en-US/docs/WebAssembly
- https://wasmbyexample.dev/home.en-us.html
- [✅ skim] Tutorial & Template for Rust-Wasm project targeting the web (wasm32-unknown-unknown): https://rustwasm.github.io/docs/book/
- [✅ read] Tutorial Rust-Wasm compilation targeting non-web env (wasm32-wasi target + WASI POSIX env): https://blog.cloudflare.com/announcing-wasi-on-workers/
- https://github.com/bytecodealliance/wasm-parallel-gzip
- https://github.com/WebAssembly/wasi-threads 
- ✅ https://medium.com/@giorgadzeluka1991/webassembly-for-the-backend-a-new-era-of-web-development-414218e8786c
    - https://hacks.mozilla.org/2019/03/standardizing-wasi-a-webassembly-system-interface/
- Wasm landscape https://landscape.cncf.io/wasm 
- ✅ https://hacks.mozilla.org/2019/03/standardizing-wasi-a-webassembly-system-interface/
- UDF functions https://www.pingcap.com/blog/how-webassembly-powers-databases-build-a-udf-engine-with-wasm/
- https://www.pingcap.com/blog/how-webassembly-powers-databases-build-a-udf-engine-with-wasm/
- Different types of parquet files: https://www.legendu.net/misc/blog/read-and-write-parquet-files-in-rust/
- Parquet in Rust https://jorgecarleitao.medium.com/parquet-with-rust-336a667a9426
- Parquet info & wasm https://github.com/kylebarron/parquet-wasm/tree/main
- Benchmark multithread parquet https://wesmckinney.com/blog/python-parquet-multithreading/
- Arrow thread https://arrow.apache.org/docs/cpp/threading.html
- Mutlithread parquet2 Rust https://rustrepo.com/repo/jorgecarleitao-parquet2 
- https://medium.com/@learnwithshobhit/web-assembly-feasibility-analysis-in-a-nutshell-wasi-wasm-762c231474ff

# GPU 
-   https://github.com/juj/wasm_webgpu

___

## TODO
- [x] WASI + WASIX
- [ ] Parquet2 parallel read
- [ ] performance under docker
- [ ] Intergrate benchmark implementations