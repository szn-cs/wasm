fn main() {
    {
        use wasm_assessment::parallel_read::run;
        let _ = run();
    }

    {
        use wasm_assessment::parallel_read::generic_parallel_read;
        let _ = generic_parallel_read::parallel_read();
    }

    println!("done");
}
