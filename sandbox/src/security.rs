pub struct Security {
    gas_limit: u64,
    gas_used: u64,
}

impl Security {
    // Yeni bir Security yapısı oluştur
    pub fn new(gas_limit: u64) -> Self {
        Security {
            gas_limit,
            gas_used: 0,
        }
    }

    // Güvenlik kontrollerini gerçekleştir
    pub fn perform_checks(&mut self) -> Result<(), String> {
        if self.gas_used >= self.gas_limit {
            Err("Gas limit exceeded during execution.".into())
        } else {
            println!("Security checks passed.");
            Ok(())
        }
    }

    // Belirli bir miktar gaz kullan
    pub fn consume_gas(&mut self, amount: u64) -> Result<(), String> {
        if self.gas_used + amount > self.gas_limit {
            Err("Gas limit exceeded.".into())
        } else {
            self.gas_used += amount;
            Ok(())
        }
    }

    // Gaz kullanımını sıfırla
    pub fn reset_gas(&mut self) {
        self.gas_used = 0;
    }
}
