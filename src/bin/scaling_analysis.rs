use cmd_lib::*;

fn main() -> CmdResult {
    for t in 1..=40 {
        // println!("Executing {}", n);
        run_cmd!(
            ./target/debug/parallel_write_parquet 10000000 $t
        )?;
    }

    Ok(())
}
