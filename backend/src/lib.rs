pub mod executor;
pub mod wasm_runtime;
pub mod gas_metering;

pub use executor::Executor;
pub use gas_metering::GasMetering;

// WASM yürütme ve gaz ölçme işlemlerini test eder
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_executor() {
        let wasm_code: &[u8] = &[0x00, 0x61, 0x62, 0x63]; // Dummy WASM bytecode
        let mut executor = Executor::new(1024);
        assert!(executor.execute(wasm_code).is_ok());
    }

    #[test]
    fn test_gas_metering() {
        let gas_limit = 10000;
        let mut gas_meter = GasMetering::new(gas_limit);

        // Gaz kullanımını test et
        assert!(gas_meter.track_gas_usage(3000).is_ok());
        assert_eq!(gas_meter.remaining_gas(), 7000);

        // Gaz limitini aşmayı test et
        assert!(gas_meter.track_gas_usage(8000).is_err());

        // Gaz kullanımını sıfırlama
        gas_meter.reset();
        assert_eq!(gas_meter.remaining_gas(), gas_limit);
    }
}
