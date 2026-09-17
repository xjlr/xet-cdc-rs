use std::fmt::Write;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChunkHash {
    pub bytes: [u8; 32],
}

const DATA_KEY: [u8; 32] = [
    0x66, 0x97, 0xf5, 0x77, 0x5b, 0x95, 0x50, 0xde,
    0x31, 0x35, 0xcb, 0xac, 0xa5, 0x97, 0x18, 0x1c,
    0x9d, 0xe4, 0x21, 0x10, 0x9b, 0xeb, 0x2b, 0x58,
    0xb4, 0xd0, 0xb0, 0x4b, 0x93, 0xad, 0xf2, 0x29,
];

pub fn hash_chunk(data: &[u8]) -> ChunkHash {
    ChunkHash {
        bytes: *blake3::keyed_hash(&DATA_KEY, data).as_bytes(),
    }
}

pub fn to_xet_hex(hash: &ChunkHash) -> String {
    let mut result = String::with_capacity(64);
    for block in hash.bytes.chunks(8) {
        for byte in block.iter().rev() {
            write!(&mut result, "{byte:02x}")
                .expect("writing to String cannot fail");
        }
    }
    result
}

pub fn parse_xet_hash(text: &str) -> Result<ChunkHash, String> {
    if text.len() != 64 {
        return Err(format!("Invalid hash length: expected 64 characters, got {}", text.len()));
    }

    const HEX_CHARS: &str = "0123456789abcdef";
    if !text.chars().all(|c| HEX_CHARS.contains(c)) {
        return Err("Invalid hash format: expected lowercase hexadecimal characters".to_string());
    }

    let mut hash  = ChunkHash { bytes: [0u8; 32] };

    for i in 0..32 {
        let dst = 8 * (i / 8) + (7 - i % 8);
        let byte_str = &text[i * 2..i * 2 + 2];
        hash.bytes[dst] = u8::from_str_radix(byte_str, 16).map_err(|e| format!("Failed to parse byte {}: {}", i, e))?;
    }

    Ok(hash)
}
