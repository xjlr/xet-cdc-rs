use crate::{ChunkHash, parse_xet_hash};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReferenceChunk {
    pub hash: ChunkHash,
    pub size: u32,
}

pub fn parse_reference_manifest(text: &str) -> Result<Vec<ReferenceChunk>, String> {
    let mut chunks = Vec::new();

    for (index, line) in text.lines().enumerate() {
        let line_number = index + 1;
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        let mut parts = line.split_whitespace();

        let hash_str = parts
            .next()
            .ok_or_else(|| format!("line {line_number}: missing hash"))?;

        let size_str = parts
            .next()
            .ok_or_else(|| format!("line {line_number}: missing size"))?;

        if parts.next().is_some() {
            return Err(format!("line {line_number}: unexpected extra field"));
        }

        let hash = parse_xet_hash(hash_str)
            .map_err(|e| format!("line {line_number}: invalid hash: {e}"))?;

        let size = size_str
            .parse::<u32>()
            .map_err(|e| format!("line {line_number}: invalid size: {e}"))?;

        chunks.push(ReferenceChunk { hash, size });
    }

    Ok(chunks)
}
