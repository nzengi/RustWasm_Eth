pub mod sandbox;
pub mod security;
pub mod isolation;

pub use sandbox::Sandbox;

// sandbox/src/lib.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sandbox() {
        // This is a mock WASM bytecode used for testing
        let wasm_code: &[u8] = &[0x00, 0x61, 0x62, 0x63];
        
        // Create a new sandbox environment with defined memory and gas limits
        let mut sandbox = Sandbox::new(1024, 10000, 1);
        
        // Ensure that the WASM execution within the sandbox is successful
        assert!(sandbox.execute(wasm_code).is_ok());
    }
}
