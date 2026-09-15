pub struct GearHash {
    value: u64,
}

impl GearHash {
    pub fn new() -> Self {
        todo!("initialize GearHash")
    }

    pub fn update(&mut self, byte: u8) {
        let _ = byte;
        todo!("update GearHash state")
    }

    pub fn value(&self) -> u64 {
        todo!("return current GearHash state")
    }

    pub fn reset(&mut self) {
        todo!("reset GearHash state")
    }
}

impl Default for GearHash {
    fn default() -> Self {
        Self::new()
    }
}
