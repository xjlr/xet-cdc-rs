use std::fs;
use std::path::Path;

use xet_cdc::{
    ChunkBoundary, Chunker, ReferenceChunk, hash_chunk, parse_reference_manifest,
};

const REFERENCE_FILE: &str =
    "reference-data/Electric_Vehicle_Population_Data_20250917.csv";
const REFERENCE_MANIFEST: &str =
    "reference-data/Electric_Vehicle_Population_Data_20250917.csv.chunks";

fn chunk_file(data: &[u8]) -> Vec<ChunkBoundary> {
    let mut chunker = Chunker::new();
    let mut boundaries = Vec::new();

    for block in data.chunks(4096) {
        boundaries.extend(chunker.feed(block));
    }

    if let Some(final_boundary) = chunker.finish() {
        boundaries.push(final_boundary);
    }

    boundaries
}

fn hash_chunks(data: &[u8], _boundaries: &[ChunkBoundary]) -> Vec<ReferenceChunk> {
    let mut chunks = Vec::new();
    for boundary in _boundaries {
        let start = boundary.offset as usize;
        let end = start + boundary.size as usize;
        let hash = hash_chunk(&data[start..end]);

        chunks.push(ReferenceChunk {
            hash,
            size: boundary.size,
        });
    }

    chunks
}

#[test]
fn hashes_boundary_defined_ranges() {
    let data = b"abcdef";
    let boundaries = [
        ChunkBoundary { offset: 0, size: 2 },
        ChunkBoundary { offset: 2, size: 4 },
    ];

    let actual = hash_chunks(data, &boundaries);

    assert_eq!(
        actual,
        vec![
            ReferenceChunk {
                hash: hash_chunk(b"ab"),
                size: 2,
            },
            ReferenceChunk {
                hash: hash_chunk(b"cdef"),
                size: 4,
            },
        ]
    );
}

#[test]
fn reproduces_the_official_reference_manifest() {
    let file = Path::new(REFERENCE_FILE);
    let manifest = Path::new(REFERENCE_MANIFEST);

    if !file.exists() || !manifest.exists() {
        eprintln!("reference-data/ is not present; see README.md for download instructions");
        return;
    }

    let data = fs::read(file).expect("failed to read reference CSV");
    let manifest_text = fs::read_to_string(manifest).expect("failed to read reference manifest");
    let expected = parse_reference_manifest(&manifest_text)
        .expect("failed to parse reference manifest");

    assert_eq!(expected.len(), 796);

    let boundaries = chunk_file(&data);
    assert_eq!(boundaries.len(), expected.len());

    let actual = hash_chunks(&data, &boundaries);
    assert_eq!(actual.len(), expected.len());

    for (index, (actual_chunk, expected_chunk)) in
        actual.iter().zip(expected.iter()).enumerate()
    {
        assert_eq!(
            actual_chunk.size, expected_chunk.size,
            "size mismatch at chunk {index}"
        );
        assert_eq!(
            actual_chunk.hash, expected_chunk.hash,
            "hash mismatch at chunk {index}"
        );
    }

    let total_size: u64 = actual.iter().map(|chunk| u64::from(chunk.size)).sum();
    assert_eq!(total_size, data.len() as u64);
}
