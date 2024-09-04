/// Represents a memory structure that holds the data for WASM execution.
/// This structure allows for reading, writing, and resetting memory in a controlled way.
pub struct Memory {
    data: Vec<u8>, // Holds the memory data as a vector of bytes
}

impl Memory {
    /// Creates a new instance of `Memory` with a specified size.
    /// The memory is initialized with zeros.
    ///
    /// # Arguments
    ///
    /// * `size` - The size of the memory in bytes.
    pub fn new(size: usize) -> Self {
        Memory {
            data: vec![0; size], // Initialize memory with zeros
        }
    }

    /// Reads a slice of data from the memory starting at a specified offset.
    ///
    /// # Arguments
    ///
    /// * `offset` - The starting point in memory to read from.
    /// * `length` - The number of bytes to read.
    ///
    /// # Panics
    ///
    /// This function will panic if the read exceeds the memory bounds.
    pub fn read(&self, offset: usize, length: usize) -> &[u8] {
        if offset + length <= self.data.len() {
            &self.data[offset..offset + length] // Return a slice of memory
        } else {
            panic!("Memory read out of bounds"); // Panic if read exceeds memory size
        }
    }

    /// Writes data into memory at a specified offset.
    ///
    /// # Arguments
    ///
    /// * `offset` - The position in memory to start writing the data.
    /// * `data` - The data to be written into memory.
    ///
    /// # Panics
    ///
    /// This function will panic if the write exceeds the memory bounds.
    pub fn write(&mut self, offset: usize, data: &[u8]) {
        if offset + data.len() <= self.data.len() {
            self.data[offset..offset + data.len()].copy_from_slice(data); // Write data to memory
        } else {
            panic!("Memory write out of bounds"); // Panic if write exceeds memory size
        }
    }

    /// Returns the size of the memory in bytes.
    pub fn size(&self) -> usize {
        self.data.len()
    }

    /// Resets the memory by filling it with zeros.
    /// This function is useful for clearing memory between executions.
    pub fn reset(&mut self) {
        for byte in &mut self.data {
            *byte = 0; // Reset each byte to zero
        }
    }
}
