use crate::{ChunkBoundary, GearHash, protocol};

pub struct Chunker {
    hash: GearHash,
    chunk_offset: u64,
    chunk_size: u32,
}

impl Chunker {
    pub fn new() -> Self {
        Self {
            hash: GearHash::new(),
            chunk_offset: 0,
            chunk_size: 0,
        }
    }

    pub fn feed(&mut self, data: &[u8]) -> Vec<ChunkBoundary> {
        let mut boundaries = Vec::new();
        for &byte in data {
            if let Some(boundary) = self.update(byte) {
                boundaries.push(boundary);
            }
        }
        boundaries
    }

    pub fn finish(&mut self) -> Option<ChunkBoundary> {
        if self.chunk_size == 0 {
            return None;
        }

        let cb = ChunkBoundary {
            offset: self.chunk_offset,
            size: self.chunk_size,
        };

        self.chunk_offset += u64::from(self.chunk_size);
        self.chunk_size = 0;
        self.hash.reset();

        Some(cb)
    }

    fn update(&mut self, byte: u8) -> Option<ChunkBoundary> {
        self.hash.update(byte);
        self.chunk_size += 1;

        if self.chunk_size < protocol::MIN_CHUNK_SIZE as u32 {
            return None;
        }

        if self.chunk_size >= protocol::MAX_CHUNK_SIZE as u32 
            || (self.hash.value() & protocol::BOUNDARY_MASK) == 0 {
            let boundary = ChunkBoundary {
                offset: self.chunk_offset,
                size: self.chunk_size,
            };
            self.chunk_offset += u64::from(self.chunk_size);
            self.chunk_size = 0;
            self.hash.reset();

            return Some(boundary);
        }
        None
    }
}

impl Default for Chunker {
    fn default() -> Self {
        Self::new()
    }
}

