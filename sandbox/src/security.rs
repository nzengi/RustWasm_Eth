pub struct Security {
    gas_limit: u64,
    gas_used: u64,
}

impl Security {
    /// Creates a new `Security` instance with a specified gas limit.
    ///
    /// # Parameters
    /// - `gas_limit`: The maximum amount of gas allowed during execution.
    ///
    /// # Returns
    /// A new `Security` instance.
    pub fn new(gas_limit: u64) -> Self {
        Security {
            gas_limit,
            gas_used: 0, // Initialize gas usage to zero
        }
    }

    /// Performs security checks to ensure gas limit is not exceeded.
    ///
    /// # Returns
    /// - `Ok(())`: If the gas usage is within limits.
    /// - `Err(String)`: If the gas usage exceeds the limit.
    pub fn perform_checks(&mut self) -> Result<(), String> {
        if self.gas_used >= self.gas_limit {
            Err("Gas limit exceeded during execution.".into())
        } else {
            println!("Security checks passed.");
            Ok(())
        }
    }

    /// Consumes a specified amount of gas during execution.
    ///
    /// # Parameters
    /// - `amount`: The amount of gas to consume.
    ///
    /// # Returns
    /// - `Ok(())`: If the gas consumption is successful.
    /// - `Err(String)`: If consuming more gas would exceed the limit.
    pub fn consume_gas(&mut self, amount: u64) -> Result<(), String> {
        if self.gas_used + amount > self.gas_limit {
            Err("Gas limit exceeded.".into())
        } else {
            self.gas_used += amount;
            Ok(())
        }
    }

    /// Resets the gas usage to zero, allowing a fresh execution cycle.
    pub fn reset_gas(&mut self) {
        self.gas_used = 0; // Reset gas usage back to zero
    }
}
