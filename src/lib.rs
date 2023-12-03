#[cfg(feature = "example")]
pub mod example;
#[cfg(feature = "wasm_multithread")]
pub mod parquet;

#[cfg(test)]
mod integration_tests {
    #[test]
    fn t() {}
}
