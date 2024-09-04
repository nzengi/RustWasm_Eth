// Module for managing the execution of WASM runtimes, with an emphasis on
// sandboxed and gas-metered execution environments for smart contracts.

use crate::gas_metering::GasMeter;

/// Structure representing the WASM runtime environment.
pub struct WasmRuntime {
    memory: Vec<u8>,
    gas_meter: GasMeter,
}

impl WasmRuntime {
    /// Creates a new `WasmRuntime` with a specified memory size and gas limit.
    pub fn new(memory_size: usize, gas_limit: u64) -> Self {
        WasmRuntime {
            memory: vec![0; memory_size],
            gas_meter: GasMeter::new(gas_limit),
        }
    }

    /// Loads WASM bytecode into memory and prepares for execution.
    pub fn load_wasm(&mut self, wasm_code: &[u8]) -> Result<(), String> {
        if wasm_code.len() > self.memory.len() {
            return Err("WASM code is too large for the allocated memory".into());
        }

        // Copy the WASM code into memory.
        self.memory[..wasm_code.len()].copy_from_slice(wasm_code);
        Ok(())
    }

    /// Executes the loaded WASM code, consuming gas in the process.
    pub fn execute(&mut self) -> Result<(), String> {
        // Simulate gas consumption for execution steps.
        self.gas_meter.consume_gas(100)?;

        // Simulated execution logic.
        println!("Executing WASM code within the runtime...");

        // Continue execution and simulate more gas consumption.
        self.gas_meter.consume_gas(200)?;

        // Check if gas has been exhausted during execution.
        if self.gas_meter.is_exhausted() {
            return Err("Out of gas during execution".into());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test to ensure WASM runtime can load and execute bytecode with proper gas metering.
    #[test]
    fn test_wasm_runtime() {
        let wasm_code: &[u8] = &[0x00, 0x61, 0x62, 0x63]; // Dummy WASM bytecode
        let mut runtime = WasmRuntime::new(1024, 500);

        assert!(runtime.load_wasm(wasm_code).is_ok(), "WASM should load successfully");
        assert!(runtime.execute().is_ok(), "WASM execution should succeed with sufficient gas");

        let mut out_of_gas_runtime = WasmRuntime::new(1024, 100);
        assert!(out_of_gas_runtime.load_wasm(wasm_code).is_ok(), "WASM should load successfully");
        assert!(out_of_gas_runtime.execute().is_err(), "Execution should fail due to out of gas");
    }
}
