pub mod env;
pub mod error;
pub mod env;
pub mod error;
pub mod memory;
pub mod state;

pub use env::Env;
pub use error::CoreError;
pub use memory::Memory;
pub use state::State;

// Central library for the core module


pub use env::Externalities;
pub use error::ContractError;
pub use memory::Memory;
pub use state::State;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_initialization() {
        let state = State::new(1024);
        assert_eq!(state.memory.size(), 1024);
    }

    #[test]
    fn test_memory_reset() {
        let mut state = State::new(1024);
        state.memory.write(0, &[1, 2, 3]);
        assert_eq!(state.memory.read(0, 3), &[1, 2, 3]);

        state.reset();
        assert_eq!(state.memory.read(0, 3), &[0, 0, 0]);
    }
}
