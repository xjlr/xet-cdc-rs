use xet_cdc::ChunkBoundary;

#[test]
fn stores_offset_and_size() {
    let boundary = ChunkBoundary {
        offset: 1024,
        size: 4096,
    };

    assert_eq!(boundary.offset, 1024);
    assert_eq!(boundary.size, 4096);
}

#[test]
fn calculates_end_offset() {
    let boundary = ChunkBoundary {
        offset: 1024,
        size: 4096,
    };

    assert_eq!(boundary.end_offset(), 5120);
}

#[test]
fn boundaries_with_same_values_are_equal() {
    let lhs = ChunkBoundary {
        offset: 123,
        size: 456,
    };

    let rhs = ChunkBoundary {
        offset: 123,
        size: 456,
    };

    assert_eq!(lhs, rhs);
}

#[test]
fn boundaries_with_different_values_are_not_equal() {
    let lhs = ChunkBoundary {
        offset: 123,
        size: 456,
    };

    let rhs = ChunkBoundary {
        offset: 123,
        size: 457,
    };

    assert_ne!(lhs, rhs);
}
