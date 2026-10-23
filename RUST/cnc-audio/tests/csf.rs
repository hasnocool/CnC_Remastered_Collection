//! CSF (string table) loader tests.
//!
//! `fixtures/synthetic.csf` is a hand-built, fully synthetic file: every
//! label and value was authored for this repository, and it contains no
//! retail asset data.

use cnc_audio::csf::{CsfError, CsfFile, CsfLanguage, CSF_HEADER_LEN};

const FIXTURE: &[u8] = include_bytes!("fixtures/synthetic.csf");

fn fixture() -> CsfFile {
    CsfFile::parse(FIXTURE).expect("synthetic fixture must parse")
}

#[test]
fn csf_fixture_parses_labels_and_values() {
    let csf = fixture();
    assert_eq!(csf.version, 3);
    assert_eq!(csf.language, CsfLanguage::Us);
    // Fourteen label blocks, one of which repeats a label name.
    assert_eq!(csf.label_count(), 13);
    assert_eq!(
        csf.text("TXT_EVA_CONSTRUCTION_COMPLETE"),
        Some("Construction complete.")
    );
    assert_eq!(csf.text("TXT_EVA_UNIT_READY"), Some("Unit ready."));
    assert_eq!(
        csf.text("TXT_EVA_BASE_ATTACK"),
        Some("Our base is under attack.")
    );
    assert_eq!(csf.text("TXT_UNIT_MAMMOTH"), Some("Mammoth Tank"));
}

#[test]
fn csf_lookup_is_case_insensitive_and_trims_names() {
    let csf = fixture();
    assert_eq!(csf.text("txt_eva_unit_ready"), Some("Unit ready."));
    assert_eq!(
        csf.text("TXT_SPACED"),
        Some("surrounding blanks are ignored")
    );
    assert!(csf.text("TXT_ABSENT").is_none());
}

#[test]
fn csf_duplicate_label_keeps_the_last_definition() {
    let csf = fixture();
    assert_eq!(csf.text("TXT_DUPLICATE"), Some("last definition"));
}

#[test]
fn csf_decodes_non_ascii_values() {
    let csf = fixture();
    assert_eq!(csf.text("TXT_GERMAN_CONSTRUCTION"), Some("Gebäude fertig."));
}

#[test]
fn csf_reads_extra_value_blocks() {
    let csf = fixture();
    let label = csf.label("TXT_TOOLTIP").expect("tooltip label");
    assert_eq!(label.values.len(), 2);
    assert_eq!(label.values[0].text, "Tooltip");
    assert_eq!(label.values[0].extra.as_deref(), Some("TXT_UNIT_MAMMOTH"));
    assert_eq!(label.values[1].text, "Second value");
    assert_eq!(label.values[1].extra, None);
}

#[test]
fn csf_resolves_string_references() {
    let csf = fixture();
    assert_eq!(
        csf.resolve("TXT_STATUS_READY").as_deref(),
        Some("Mammoth Tank ready.")
    );
}

#[test]
fn csf_leaves_unknown_references_verbatim() {
    let csf = fixture();
    assert_eq!(
        csf.text("TXT_UNKNOWN_REF"),
        Some("%TXT_DOES_NOT_EXIST% here")
    );
    assert_eq!(
        csf.resolve("TXT_UNKNOWN_REF").as_deref(),
        Some("%TXT_DOES_NOT_EXIST% here")
    );
}

#[test]
fn csf_reference_cycles_do_not_recurse_forever() {
    let csf = fixture();
    let resolved = csf.resolve("TXT_REF_CYCLE_A").expect("cycle resolves");
    assert!(resolved.contains("TXT_REF_CYCLE_A") || resolved.contains("TXT_REF_CYCLE_B"));
}

#[test]
fn csf_round_trip_through_encode() {
    let csf = fixture();
    let encoded = csf.encode();
    let reparsed = CsfFile::parse(&encoded).expect("encoded table parses");
    assert_eq!(reparsed, csf);
}

#[test]
fn csf_reports_bad_magic() {
    let data = b" ARG"
        .iter()
        .copied()
        .chain(FIXTURE[4..].iter().copied())
        .collect::<Vec<_>>();
    assert!(matches!(CsfFile::parse(&data), Err(CsfError::BadMagic(_))));
}

#[test]
fn csf_reports_truncated_header() {
    assert!(CsfFile::parse(FIXTURE).is_ok());
    assert!(matches!(
        CsfFile::parse(&FIXTURE[..CSF_HEADER_LEN - 1]),
        Err(CsfError::Truncated { .. })
    ));
    assert!(matches!(
        CsfFile::parse(&[]),
        Err(CsfError::Truncated { .. })
    ));
}

#[test]
fn csf_reports_unsupported_version() {
    let mut data = FIXTURE.to_vec();
    data[4..8].copy_from_slice(&9u32.to_le_bytes());
    assert!(matches!(
        CsfFile::parse(&data),
        Err(CsfError::UnsupportedVersion(9))
    ));
}

#[test]
fn csf_reports_bad_label_tag() {
    let mut data = FIXTURE.to_vec();
    data[CSF_HEADER_LEN..CSF_HEADER_LEN + 4].copy_from_slice(b"XXXX");
    assert!(matches!(
        CsfFile::parse(&data),
        Err(CsfError::BadLabelTag { .. })
    ));
}

#[test]
fn csf_reports_bad_value_tag() {
    // First label: header starts at 0x18, its value block right after the
    // 0x0C-byte label header plus the 32-byte name.
    let mut data = FIXTURE.to_vec();
    let name_len = u32::from_le_bytes(
        data[CSF_HEADER_LEN + 8..CSF_HEADER_LEN + 12]
            .try_into()
            .expect("4-byte slice"),
    ) as usize;
    let value_offset = CSF_HEADER_LEN + 12 + name_len;
    data[value_offset..value_offset + 4].copy_from_slice(b"XXXX");
    assert!(matches!(
        CsfFile::parse(&data),
        Err(CsfError::BadValueTag { .. })
    ));
}

#[test]
fn csf_reports_truncated_value_payload() {
    let mut data = FIXTURE.to_vec();
    let name_len = u32::from_le_bytes(
        data[CSF_HEADER_LEN + 8..CSF_HEADER_LEN + 12]
            .try_into()
            .expect("4-byte slice"),
    ) as usize;
    let value_offset = CSF_HEADER_LEN + 12 + name_len;
    // Claim 999 characters for a value with far fewer bytes available.
    data[value_offset + 4..value_offset + 8].copy_from_slice(&999u32.to_le_bytes());
    assert!(matches!(
        CsfFile::parse(&data),
        Err(CsfError::ValueLengthMismatch { .. })
    ));
}

#[test]
fn csf_reports_value_count_mismatch() {
    let mut data = FIXTURE.to_vec();
    data[12..16].copy_from_slice(&2u32.to_le_bytes());
    assert!(matches!(
        CsfFile::parse(&data),
        Err(CsfError::ValueCountMismatch { .. })
    ));
}

#[test]
fn csf_reports_trailing_garbage() {
    let mut data = FIXTURE.to_vec();
    data.push(0x77);
    assert!(matches!(
        CsfFile::parse(&data),
        Err(CsfError::TrailingBytes { .. })
    ));
}

#[test]
fn csf_rejects_unpaired_surrogates() {
    // One label with one value whose only UTF-16 unit decodes (after the
    // format's bitwise inversion) to a lone high surrogate.
    let mut value_payload = Vec::new();
    value_payload.extend_from_slice(&(!0xD800u16).to_le_bytes());
    let mut data = Vec::new();
    data.extend_from_slice(b" FSC");
    data.extend_from_slice(&3u32.to_le_bytes());
    data.extend_from_slice(&1u32.to_le_bytes());
    data.extend_from_slice(&1u32.to_le_bytes());
    data.extend_from_slice(&0u32.to_le_bytes());
    data.extend_from_slice(&0u32.to_le_bytes());
    data.extend_from_slice(b" LBL");
    data.extend_from_slice(&1u32.to_le_bytes());
    data.extend_from_slice(&4u32.to_le_bytes());
    data.extend_from_slice(b"TXTX");
    data.extend_from_slice(b" RTS");
    data.extend_from_slice(&1u32.to_le_bytes());
    data.extend_from_slice(&value_payload);
    assert!(matches!(
        CsfFile::parse(&data),
        Err(CsfError::InvalidValueEncoding { .. })
    ));
}
