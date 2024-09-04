/// Represents the different errors that can occur during contract execution.
#[derive(Debug, PartialEq)]
pub enum ContractError {
    /// Occurs when the contract runs out of gas during execution.
    OutOfGas,

    /// Indicates an invalid operation or invalid instruction encountered during execution.
    InvalidOperation,

    /// Occurs when memory access is out of bounds.
    MemoryAccessOutOfBounds,

    /// A generic error for any other issues.
    GenericError(String),
}

impl std::fmt::Display for ContractError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContractError::OutOfGas => write!(f, "Out of Gas"),
            ContractError::InvalidOperation => write!(f, "Invalid Operation"),
            ContractError::MemoryAccessOutOfBounds => write!(f, "Memory Access Out of Bounds"),
            ContractError::GenericError(msg) => write!(f, "Generic Error: {}", msg),
        }
    }
}

impl std::error::Error for ContractError {}
