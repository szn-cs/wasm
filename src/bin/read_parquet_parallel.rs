fn main() {
    #[cfg(feature = "wasm_multithread")]
    {
        use wasm_assessment::parquet;
        parquet::read();
    }

    println!("done");
}
