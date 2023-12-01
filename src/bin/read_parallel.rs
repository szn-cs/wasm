#[cfg(feature = "wasm_multithread")]
fn main() {
    use wasm_assessment::example;
    example::threads();
}

fn main() {}
