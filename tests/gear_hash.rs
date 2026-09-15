use xet_cdc::GearHash;

#[test]
fn starts_at_zero() {
    let hash = GearHash::new();

    assert_eq!(hash.value(), 0);
}

#[test]
fn can_be_reset() {
    let mut hash = GearHash::new();

    hash.update(0);
    assert_eq!(hash.value(), 0xb088_d3a9_e840_f559);

    hash.reset();
    assert_eq!(hash.value(), 0);

    hash.update(0);
    assert_eq!(hash.value(), 0xb088_d3a9_e840_f559);
}

#[test]
fn updates_state_incrementally() {
    let mut hash = GearHash::new();

    hash.update(0);
    assert_eq!(hash.value(), 0xb088_d3a9_e840_f559);

    hash.update(1);
    assert_eq!(hash.value(), 0xb764_6f4b_0a6f_0b88);
}

#[test]
fn matches_known_text_vector() {
    let mut hash = GearHash::new();

    for &byte in b"hello" {
        hash.update(byte);
    }

    assert_eq!(hash.value(), 0xc3db_fa67_038d_e097);
}

#[test]
fn handles_every_possible_byte_value() {
    let mut hash = GearHash::new();

    for byte in 0u8..=u8::MAX {
        hash.update(byte);
    }

    assert_eq!(hash.value(), 0xd5bc_3856_d8df_cdeb);
}

#[test]
fn default_is_equivalent_to_new() {
    let new_hash = GearHash::new();
    let default_hash = GearHash::default();

    assert_eq!(new_hash.value(), default_hash.value());
    assert_eq!(default_hash.value(), 0);
}
