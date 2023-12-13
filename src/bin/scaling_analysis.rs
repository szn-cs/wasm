use cmd_lib::*;

fn main() -> CmdResult {
    parallel_write_parquet();

    Ok(())
}

fn parallel_write_parquet() {
    for t in 1..=40 {
        // println!("Executing {}", n);
        run_cmd!(
            ./target/release/parallel_write_parquet 10000000 $t
        )?;
    }

    /*
    for t in 1..=40 {
        // println!("Executing {}", n);
        run_cmd!(
            wasmer run --enable-all --mapdir ./resource/:./resource/ ./target/wasm32-wasmer-wasi/release/parallel_write_parquet.wasm 10000000 $t
        )?;
    }
    */
}

// NOTE: run sub-sections separately to clear caches
fn parallel_read_arquet() {
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
}
