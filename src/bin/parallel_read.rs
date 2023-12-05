fn main() {
    {
        use wasm_assessment::parallel_read::run;
        let _ = run();
    }

    println!("done");
}
