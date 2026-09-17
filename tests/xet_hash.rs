use xet_cdc::{ChunkHash, parse_xet_hash, to_xet_hex};

const SPEC_EXAMPLE_HASH: ChunkHash = ChunkHash {
    bytes: [
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
        0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d,
        0x1e, 0x1f,
    ],
};

const SPEC_EXAMPLE_TEXT: &str = "07060504030201000f0e0d0c0b0a090817161514131211101f1e1d1c1b1a1918";

const FIRST_REFERENCE_HASH: ChunkHash = ChunkHash {
    bytes: [
        0x61, 0x16, 0xc6, 0x71, 0xdc, 0xa1, 0x0a, 0xb1, 0xaa, 0x88, 0xa1, 0x41, 0x0c, 0x28, 0x92,
        0xde, 0x72, 0x85, 0xb7, 0x39, 0x17, 0x98, 0x47, 0xbc, 0xe4, 0x5c, 0xdc, 0xd8, 0x45, 0x99,
        0x09, 0x4a,
    ],
};

const FIRST_REFERENCE_TEXT: &str =
    "b10aa1dc71c61661de92280c41a188aabc47981739b785724a099945d8dc5ce4";

const SECOND_REFERENCE_HASH: ChunkHash = ChunkHash {
    bytes: [
        0x6b, 0x3b, 0x80, 0xfa, 0x91, 0x55, 0x25, 0x26, 0x6f, 0x8a, 0x5b, 0x31, 0x8c, 0xd8, 0x25,
        0xaf, 0xc5, 0x8e, 0xf1, 0xfd, 0xbc, 0xd5, 0x53, 0x51, 0x07, 0xc9, 0xcc, 0xe0, 0x64, 0x62,
        0x52, 0xef,
    ],
};

const SECOND_REFERENCE_TEXT: &str =
    "26255591fa803b6baf25d88c315b8a6f5153d5bcfdf18ec5ef526264e0ccc907";

#[test]
fn encodes_the_specification_example() {
    assert_eq!(to_xet_hex(&SPEC_EXAMPLE_HASH), SPEC_EXAMPLE_TEXT);
}

#[test]
fn encodes_published_reference_hashes() {
    assert_eq!(to_xet_hex(&FIRST_REFERENCE_HASH), FIRST_REFERENCE_TEXT);
    assert_eq!(to_xet_hex(&SECOND_REFERENCE_HASH), SECOND_REFERENCE_TEXT);
}

#[test]
fn xet_encoding_is_not_a_plain_hex_dump() {
    assert_ne!(
        to_xet_hex(&SPEC_EXAMPLE_HASH),
        "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f"
    );
}

#[test]
fn parses_the_specification_example() {
    assert_eq!(
        parse_xet_hash(SPEC_EXAMPLE_TEXT).unwrap(),
        SPEC_EXAMPLE_HASH
    );
}

#[test]
fn parses_published_reference_hashes() {
    assert_eq!(
        parse_xet_hash(FIRST_REFERENCE_TEXT).unwrap(),
        FIRST_REFERENCE_HASH
    );
    assert_eq!(
        parse_xet_hash(SECOND_REFERENCE_TEXT).unwrap(),
        SECOND_REFERENCE_HASH
    );
}

#[test]
fn raw_hash_round_trips_through_xet_text() {
    for hash in [
        SPEC_EXAMPLE_HASH,
        FIRST_REFERENCE_HASH,
        SECOND_REFERENCE_HASH,
    ] {
        assert_eq!(parse_xet_hash(&to_xet_hex(&hash)).unwrap(), hash);
    }
}

#[test]
fn xet_text_round_trips_through_raw_hash() {
    for text in [
        SPEC_EXAMPLE_TEXT,
        FIRST_REFERENCE_TEXT,
        SECOND_REFERENCE_TEXT,
    ] {
        assert_eq!(to_xet_hex(&parse_xet_hash(text).unwrap()), text);
    }
}

#[test]
fn encoding_is_64_lowercase_hex_characters() {
    let text = to_xet_hex(&SPEC_EXAMPLE_HASH);
    assert_eq!(text.len(), 64);
    assert!(
        text.bytes()
            .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
    );
}

#[test]
fn parser_rejects_noncanonical_text() {
    assert!(parse_xet_hash("").is_err());
    assert!(parse_xet_hash(&FIRST_REFERENCE_TEXT[..63]).is_err());
    assert!(parse_xet_hash(&(FIRST_REFERENCE_TEXT.to_owned() + "0")).is_err());
    assert!(
        parse_xet_hash("g10aa1dc71c61661de92280c41a188aabc47981739b785724a099945d8dc5ce4").is_err()
    );
    assert!(
        parse_xet_hash("B10AA1DC71C61661DE92280C41A188AABC47981739B785724A099945D8DC5CE4").is_err()
    );
    assert!(parse_xet_hash(&(FIRST_REFERENCE_TEXT.to_owned() + "\n")).is_err());
}
