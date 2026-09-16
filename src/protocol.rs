pub const KIB: usize = 1024;

pub const TARGET_CHUNK_SIZE: usize = 64 * KIB;
pub const MIN_CHUNK_SIZE: usize = 8 * KIB;
pub const MAX_CHUNK_SIZE: usize = 128 * KIB;

pub const BOUNDARY_MASK: u64 = 0xFFFF_0000_0000_0000;
