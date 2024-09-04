pub struct Isolation {
    level: u8,
}

impl Isolation {
    // Yeni bir Isolation yapısı oluştur
    pub fn new(level: u8) -> Self {
        Isolation { level }
    }

    // İzolasyon seviyesini kontrol et ve uygula
    pub fn enforce_isolation(&self) -> Result<(), String> {
        if self.level == 0 {
            Err("Isolation level is too low".into())
        } else {
            println!("Isolation level {} applied.", self.level);
            Ok(())
        }
    }
}
