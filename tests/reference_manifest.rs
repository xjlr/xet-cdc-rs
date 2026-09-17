use xet_cdc::{ReferenceChunk, parse_reference_manifest, parse_xet_hash};

const HASH0: &str = "b10aa1dc71c61661de92280c41a188aabc47981739b785724a099945d8dc5ce4";
const HASH1: &str = "26255591fa803b6baf25d88c315b8a6f5153d5bcfdf18ec5ef526264e0ccc907";
const HASH2: &str = "099cb228194fe640e36a6c7d274ee5ed3a714ccd557a0951d9b6b43a7292b5d1";

fn chunk(hash: &str, size: u32) -> ReferenceChunk {
    ReferenceChunk {
        hash: parse_xet_hash(hash).expect("test hash must be valid"),
        size,
    }
}

#[test]
fn parses_single_entry() {
    let parsed = parse_reference_manifest(&format!("{HASH0} 131072\n")).unwrap();
    assert_eq!(parsed, vec![chunk(HASH0, 131072)]);
}

#[test]
fn preserves_manifest_order() {
    let text = format!("{HASH0} 131072\n{HASH1} 106099\n{HASH2} 61389\n");

    assert_eq!(
        parse_reference_manifest(&text).unwrap(),
        vec![
            chunk(HASH0, 131072),
            chunk(HASH1, 106099),
            chunk(HASH2, 61389),
        ]
    );
}

#[test]
fn accepts_empty_manifest_and_blank_lines() {
    assert!(parse_reference_manifest("").unwrap().is_empty());

    let text = format!("\n   \n{HASH0} 131072\n\t\n{HASH1}\t106099\n\n");
    assert_eq!(
        parse_reference_manifest(&text).unwrap(),
        vec![chunk(HASH0, 131072), chunk(HASH1, 106099)]
    );
}

#[test]
fn accepts_windows_line_endings_and_no_final_newline() {
    let text = format!("{HASH0} 131072\r\n{HASH1} 106099");
    assert_eq!(
        parse_reference_manifest(&text).unwrap(),
        vec![chunk(HASH0, 131072), chunk(HASH1, 106099)]
    );
}

#[test]
fn rejects_malformed_hash() {
    let error = parse_reference_manifest("not-a-hash 131072\n").unwrap_err();
    assert!(error.contains("line 1"));
}

#[test]
fn rejects_malformed_size() {
    let error = parse_reference_manifest(&format!("{HASH0} twelve\n")).unwrap_err();
    assert!(error.contains("line 1"));
}

#[test]
fn rejects_size_that_does_not_fit_u32() {
    let error = parse_reference_manifest(&format!("{HASH0} 4294967296\n")).unwrap_err();
    assert!(error.contains("line 1"));
}

#[test]
fn rejects_missing_or_extra_fields() {
    assert!(parse_reference_manifest("131072\n").is_err());
    assert!(parse_reference_manifest(&format!("{HASH0}\n")).is_err());
    assert!(parse_reference_manifest(&format!("{HASH0} 131072 extra\n")).is_err());
}

#[test]
fn error_reports_failing_line_number() {
    let text = format!("{HASH0} 131072\n{HASH1} 106099\n{HASH2} oops\n");

    let error = parse_reference_manifest(&text).unwrap_err();
    assert!(error.contains("line 3"), "unexpected error: {error}");
}
