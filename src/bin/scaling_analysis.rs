use cmd_lib::*;

fn main() {
    let _ = parallel_write_parquet_cpu_utilization();
}

fn parallel_write_parquet() -> CmdResult {
    let num_iteration = 10_000_000;

    for t in 1..=40 {
        // println!("Executing {}", n);
        run_cmd!(
            ./target/release/parallel_write_parquet $num_iteration $t
        )?;
    }

    /*
    for t in 1..=40 {
        // println!("Executing {}", n);
        run_cmd!(
            wasmer run --enable-all --mapdir ./resource/:./resource/ ./target/wasm32-wasmer-wasi/release/parallel_write_parquet.wasm $num_iteration $t
        )?;
    }
    */

    Ok(())
}

fn parallel_write_parquet_cpu_utilization() -> CmdResult {
    let num_iteration = 10_000_000;

    /*
    for t in 1..=40 {
        // println!("Executing {}", n);
        run_cmd!(
            perf stat -- ./target/release/parallel_write_parquet $num_iteration $t
        )?;
    }
    */

    for t in 1..=40 {
        // println!("Executing {}", n);
        run_cmd!(
            perf stat -- wasmer run --enable-all --mapdir ./resource/:./resource/ ./target/wasm32-wasmer-wasi/release/parallel_write_parquet.wasm $num_iteration $t
        )?;
    }

    Ok(())
}

// NOTE: run sub-sections separately to clear caches
fn parallel_read_arquet() -> CmdResult {
    println!("Native execution of parallel read:");
    for t in 1..=40 {
        // println!("Executing {}", n);
        run_cmd!(
            ./target/release/parallel_read "./resource/House_Price.parquet" $t
        )?;
    }

    // ___

    // println!("Wasm execution of parallel read:");
    // for t in 1..=40 {
    //     // println!("Executing {}", n);
    //     run_cmd!(
    //         wasmer run --enable-all --mapdir ./resource/:./resource/ ./target/wasm32-wasmer-wasi/release/./target/release/parallel_read.wasm "./resource/House_Price.parquet" $t
    //     )?;
    // }

    Ok(())
}
