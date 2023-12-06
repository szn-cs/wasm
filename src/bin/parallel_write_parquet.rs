fn main() {
    {
        use wasm_assessment::parallel_read::parallel_write_parquet;
        let _ = parallel_write_parquet::run();
    }

    println!("done");
}
