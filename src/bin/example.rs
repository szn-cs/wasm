fn main() {
    #[cfg(feature = "example")]
    {
        use wasm_assessment::example;
        example::threads();
    }

    println!("done");
}
