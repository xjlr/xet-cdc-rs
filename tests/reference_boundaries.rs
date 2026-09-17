use std::fs;
use std::path::Path;

use xet_cdc::{ChunkBoundary, Chunker};

const REFERENCE_FILE: &str = "reference-data/Electric_Vehicle_Population_Data_20250917.csv";
const REFERENCE_MANIFEST: &str =
    "reference-data/Electric_Vehicle_Population_Data_20250917.csv.chunks";

fn parse_reference_sizes(text: &str) -> Vec<u32> {
    let mut result = Vec::new();

    for line in text.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let mut parts = line.split_whitespace();

        let _hash = parts.next().expect("missing hash in manifest");
        let size = parts
            .next()
            .expect("missing size in manifest")
            .parse::<u32>()
            .expect("invalid size in manifest");

        assert!(
            parts.next().is_none(),
            "unexpected extra field in manifest line: {line}"
        );

        result.push(size);
    }

    result
}

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

#[test]
fn parses_reference_sizes_from_manifest_shape() {
    let manifest = concat!(
        "b10aa1dc71c61661de92280c41a188aabc47981739b785724a099945d8dc5ce4 131072\n",
        "26255591fa803b6baf25d88c315b8a6f5153d5bcfdf18ec5ef526264e0ccc907 106099\n",
        "099cb228194fe640e36a6c7d274ee5ed3a714ccd557a0951d9b6b43a7292b5d1 61389\n",
    );

    assert_eq!(parse_reference_sizes(manifest), vec![131072, 106099, 61389]);
}

#[test]
fn reproduces_all_official_xet_chunk_boundaries() {
    let file = Path::new(REFERENCE_FILE);
    let manifest = Path::new(REFERENCE_MANIFEST);

    if !file.exists() || !manifest.exists() {
        eprintln!("reference-data/ is not present; see README.md for download instructions");
        return;
    }

    let data = fs::read(file).expect("failed to read reference CSV");
    let manifest_text = fs::read_to_string(manifest).expect("failed to read reference manifest");
    let expected_sizes = parse_reference_sizes(&manifest_text);

    assert_eq!(expected_sizes.len(), 796);

    let actual = chunk_file(&data);
    assert_eq!(actual.len(), expected_sizes.len());

    let mut expected_offset = 0u64;

    for (index, (boundary, expected_size)) in actual.iter().zip(expected_sizes.iter()).enumerate() {
        assert_eq!(
            boundary.offset, expected_offset,
            "offset mismatch at chunk {index}"
        );
        assert_eq!(
            boundary.size, *expected_size,
            "size mismatch at chunk {index}"
        );

        expected_offset += u64::from(*expected_size);
    }

    assert_eq!(expected_offset, data.len() as u64);
}
