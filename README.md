# wasm disaggregated-storage pushdown


# WASM Threading parallelisation of pushdown operations: 

- Architectures storage: 
    - Storage is underpowered (# of cores or speed is not very great); Low-end edge computing; Types of operations supported are limited; 
    - Base layer and an intermediary (with more mem & compute power but with higher latency) that is doing client handling (buffering between the two); 
	
- Scale-out: handling 100k or 1000s of requests per second. 
    - what is worthwhile and what budget there is to parallelize. Typically many of these operations in terms of pushdown are not parallelizable;
	- Many of the opreations are pushdown to a highly optimized specialized services for a particular operation (e.g. parsing, aggregation, filters, etc.);  

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
    - support https://bytecodealliance.zulipchat.com/ 
- WASI spec https://wasi.dev/
    - Wasmer extension WASIX https://wasix.org/docs 
    - https://doc.rust-lang.org/stable/nightly-rustc/rustc_target/spec/wasm32_wasi/index.html 
    - https://github.com/bytecodealliance/wasmtime/blob/main/docs/WASI-tutorial.md 
    - https://github.com/WebAssembly/WASI/tree/main
        - https://github.com/WebAssembly/WASI/blob/main/Proposals.md
            - https://github.com/WebAssembly/wasi-threads
        - draft spec & proposal https://github.com/WebAssembly/threads
- Capability based security https://github.com/bytecodealliance/wasmtime/blob/main/docs/WASI-capabilities.md



- tools
    - Binary tools https://github.com/webassembly/wabt
    - https://github.com/WebAssembly/binaryen
    - wasm-opt
    - https://bytecodealliance.github.io/cargo-wasi/steps.html
    - Datasets https://www.tablab.app/datasets/sample/parquet 
    - https://github.com/bheisler/criterion.rs 
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
- Data formats:
    - Different types of parquet files: https://www.legendu.net/misc/blog/read-and-write-parquet-files-in-rust/
    - Parquet in Rust https://jorgecarleitao.medium.com/parquet-with-rust-336a667a9426
    - Parquet info & wasm https://github.com/kylebarron/parquet-wasm/tree/main
    - Benchmark multithread parquet https://wesmckinney.com/blog/python-parquet-multithreading/
    - Arrow thread https://arrow.apache.org/docs/cpp/threading.html
    - Mutlithread parquet2 Rust https://rustrepo.com/repo/jorgecarleitao-parquet2 
    - https://dkharazi.github.io/blog/parquet 
- Arrow2 implementation [API](https://jorgecarleitao.github.io/arrow2/main/docs/arrow2); [Guide](https://jorgecarleitao.github.io/arrow2/main/guide)
    - Arrow2 Arrays are always immutable (at least not growable), thus no append operation is possible;
- https://medium.com/@learnwithshobhit/web-assembly-feasibility-analysis-in-a-nutshell-wasi-wasm-762c231474ff
- performance measurement wasm ("Exploit Parallelism for...") https://youtu.be/g8JHCcMc79s
- WebAssembly Research Center https://www.cs.cmu.edu/wrc/
- SIMD https://blog.dkwr.de/development/wasm-simd-operations/
- wasm blog
    - https://blog.dkwr.de/development/wasm-memory/
    - https://blog.dkwr.de/development/wasm-control-flow/
    - https://blog.dkwr.de/development/wasm-technical-view/
    - https://blog.dkwr.de/development/wasi-rust/
- https://pengowray.github.io/wasm-ops/ 
- Performance analysis https://easyperf.net/blog/2019/10/05/Performance-Analysis-Of-MT-apps
- http://www.roylongbottom.org.uk/linux%20multithreading%20benchmarks.htm 
- https://www.anandtech.com/show/11425/launching-the-cpu-overload-project-testing-every-x86-desktop-processor-since-2010

# GPU 
-   https://github.com/juj/wasm_webgpu

___

## TODO
- [x] WASI + WASIX
- [x] Parquet2 parallel read (deserialization)
- [x] Parquet2 parallel write (deserialization + compression) ❌ runtime error: `memory allocation of 811043 bytes failed
error: RuntimeError: unreachable;` 
- [ ] wasm64-unknown-unknown target for larger space x64
- [ ] performance under docker
- [ ] Intergrate benchmark implementations
- [ ] scales not linearily with array size growth (marshelling and copying data); Individual processes on chucks of data; Show that there is a big gap between wasm and native execution; 

___

# memory
- 64KiB memory page
- 2^16 * 64KiB = 4GiB bytes   i.e. 32-bit address = 16-bits for pages & 16-bits dedicated for bytes
    - mem spec https://webassembly.github.io/spec/core/exec/runtime.html#memory-instances 
    - In the current version of WebAssembly, at most one memory is allowed in a module.
- Multiple-memory proposal: https://github.com/WebAssembly/multi-memory/blob/main/proposals/multi-memory/Overview.md
- shared-memory is imported to the module from the host: https://github.com/wasmerio/wasmer/issues/2284
- memories: in current version only a single memory can be defined/imported in a single module; 

