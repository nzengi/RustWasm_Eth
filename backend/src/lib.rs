pub mod executor;
pub mod wasm_runtime;
pub mod gas_metering;

pub use executor::Executor;

// backend/src/lib.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_executor() {
        let wasm_code: &[u8] = &[0x00, 0x61, 0x62, 0x63]; // Dummy WASM bytecode
        let mut executor = Executor::new(1024);
        assert!(executor.execute(wasm_code).is_ok());
    }
}