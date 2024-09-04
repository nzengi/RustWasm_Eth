use crate::memory::Memory;

/// Represents the execution state, including memory and other related execution data.
pub struct State {
    pub memory: Memory,
}

impl State {
    /// Creates a new `State` instance with a specified memory size.
    ///
    /// # Arguments
    ///
    /// * `memory_size` - The size of the memory to allocate.
    pub fn new(memory_size: usize) -> Self {
        State {
            memory: Memory::new(memory_size),
        }
    }

    /// Resets the state, clearing the memory to ensure the state is ready for a new execution.
    pub fn reset(&mut self) {
        self.memory.reset();
    }
}
