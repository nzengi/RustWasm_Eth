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
        let wasm_code: &[u8] = &[0x00, 0x61, 0x62, 0x63]; // Dummy WASM bytecode
        let mut sandbox = Sandbox::new(1024, 10000, 1);
        assert!(sandbox.execute(wasm_code).is_ok());
    }
}