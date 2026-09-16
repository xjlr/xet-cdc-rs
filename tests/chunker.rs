use xet_cdc::{ChunkBoundary, Chunker, MAX_CHUNK_SIZE, MIN_CHUNK_SIZE};

fn patterned_data(size: usize) -> Vec<u8> {
    (0..size).map(|i| (i & 0xff) as u8).collect()
}

#[test]
fn does_not_emit_before_minimum_chunk_size() {
    let mut chunker = Chunker::new();
    let data = vec![0x42; MIN_CHUNK_SIZE - 1];

    let boundaries = chunker.feed(&data);

    assert!(boundaries.is_empty());
}

#[test]
fn emits_only_valid_chunk_sizes() {
    let mut chunker = Chunker::new();
    let data = patterned_data(20 * MAX_CHUNK_SIZE);

    let boundaries = chunker.feed(&data);

    assert!(!boundaries.is_empty());
    for boundary in boundaries {
        assert!(boundary.size as usize >= MIN_CHUNK_SIZE);
        assert!(boundary.size as usize <= MAX_CHUNK_SIZE);
    }
}

#[test]
fn emits_contiguous_boundaries() {
    let mut chunker = Chunker::new();
    let data = patterned_data(20 * MAX_CHUNK_SIZE);

    let boundaries = chunker.feed(&data);

    assert!(!boundaries.is_empty());
    let mut expected_offset = 0u64;

    for boundary in boundaries {
        assert_eq!(boundary.offset, expected_offset);
        assert_eq!(boundary.end_offset(), expected_offset + u64::from(boundary.size));
        expected_offset = boundary.end_offset();
    }
}

#[test]
fn identical_input_produces_identical_boundaries() {
    let data = patterned_data(20 * MAX_CHUNK_SIZE);
    let mut first = Chunker::new();
    let mut second = Chunker::new();

    assert_eq!(first.feed(&data), second.feed(&data));
}

#[test]
fn boundaries_do_not_depend_on_feed_split() {
    let data = patterned_data(20 * MAX_CHUNK_SIZE);

    let mut whole_chunker = Chunker::new();
    let whole_boundaries = whole_chunker.feed(&data);

    let mut split_chunker = Chunker::new();
    let mut split_boundaries = Vec::new();

    for block in data.chunks(4096) {
        split_boundaries.extend(split_chunker.feed(block));
    }

    assert_eq!(split_boundaries, whole_boundaries);
}

#[test]
fn finish_returns_none_for_empty_input() {
    let mut chunker = Chunker::new();

    assert_eq!(chunker.finish(), None);
}

#[test]
fn finish_emits_final_chunk_smaller_than_minimum() {
    let mut chunker = Chunker::new();
    let data = vec![0x42; 100];

    assert!(chunker.feed(&data).is_empty());
    assert_eq!(
        chunker.finish(),
        Some(ChunkBoundary {
            offset: 0,
            size: 100,
        })
    );
}

#[test]
fn finish_emits_remaining_tail_with_correct_offset() {
    let mut chunker = Chunker::new();
    let mut emitted = None;

    for i in 0..(10 * MAX_CHUNK_SIZE) {
        let byte = [(i & 0xff) as u8];
        let boundaries = chunker.feed(&byte);
        if let Some(boundary) = boundaries.first().copied() {
            assert_eq!(boundaries.len(), 1);
            emitted = Some(boundary);
            break;
        }
    }

    let emitted = emitted.expect("expected at least one boundary");
    let tail = vec![0x42; 100];

    assert!(chunker.feed(&tail).is_empty());
    assert_eq!(
        chunker.finish(),
        Some(ChunkBoundary {
            offset: emitted.end_offset(),
            size: 100,
        })
    );
}

#[test]
fn finish_does_not_emit_final_chunk_twice() {
    let mut chunker = Chunker::new();
    let data = vec![0x42; 100];

    assert!(chunker.feed(&data).is_empty());
    assert_eq!(
        chunker.finish(),
        Some(ChunkBoundary {
            offset: 0,
            size: 100,
        })
    );
    assert_eq!(chunker.finish(), None);
}
