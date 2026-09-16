#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChunkBoundary {
    pub offset: u64,
    pub size: u32,
}

impl ChunkBoundary {
    pub fn end_offset(&self) -> u64 {
        self.offset + self.size as u64
    }
}
