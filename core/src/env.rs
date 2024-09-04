/// The `Externalities` trait defines the interface for interacting with external environment resources.
pub trait Externalities {
    /// Reads a value from external storage by a given key.
    ///
    /// # Arguments
    ///
    /// * `key` - A reference to the key for which to retrieve the value.
    ///
    /// # Returns
    ///
    /// A byte vector containing the value or `None` if the key does not exist.
    fn read_storage(&self, key: &[u8]) -> Option<Vec<u8>>;

    /// Writes a value to external storage for a given key.
    ///
    /// # Arguments
    ///
    /// * `key` - A reference to the key where the value will be stored.
    /// * `value` - The value to be stored in the external storage.
    fn write_storage(&mut self, key: &[u8], value: &[u8]);

    /// Removes a value from external storage for a given key.
    ///
    /// # Arguments
    ///
    /// * `key` - A reference to the key for which to remove the value.
    fn remove_storage(&mut self, key: &[u8]);
}
