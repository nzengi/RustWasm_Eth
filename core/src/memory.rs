pub struct Memory {
    data: Vec<u8>,
}

impl Memory {
    // Yeni bir Memory yapısı oluştur
    pub fn new(size: usize) -> Self {
        Memory {
            data: vec![0; size],
        }
    }

    // Bellekten veri oku
    pub fn read(&self, offset: usize, length: usize) -> &[u8] {
        if offset + length <= self.data.len() {
            &self.data[offset..offset + length]
        } else {
            panic!("Memory read out of bounds");
        }
    }

    // Belleğe veri yaz
    pub fn write(&mut self, offset: usize, data: &[u8]) {
        if offset + data.len() <= self.data.len() {
            self.data[offset..offset + data.len()].copy_from_slice(data);
        } else {
            panic!("Memory write out of bounds");
        }
    }

    // Bellek boyutunu döndür
    pub fn size(&self) -> usize {
        self.data.len()
    }

    // Belleği sıfırla
    pub fn reset(&mut self) {
        for byte in &mut self.data {
            *byte = 0;
        }
    }
}
