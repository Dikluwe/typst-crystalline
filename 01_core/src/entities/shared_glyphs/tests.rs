//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/entities/shared_glyphs/tests.md
//! @prompt-hash 5cbf8cc9
//! @layer L1
//! @updated 2026-11-07
use super::*;
use std::sync::Arc;
fn fixture() -> Vec<ShapedGlyph> {
    vec![
        ShapedGlyph {
            glyph_id: 37,
            x_advance: -321,
            x_offset: 17,
            y_offset: -29,
            cluster: 7,
            char_code: 'Ж',
        },
        ShapedGlyph {
            glyph_id: 0,
            x_advance: 920,
            x_offset: -7,
            y_offset: 8,
            cluster: 0,
            char_code: '\u{301}',
        },
    ]
}
#[test]
fn freeze_preserves_every_field_order_and_debug() {
    let mut retained = fixture();
    let expected = retained.clone();
    let shared = SharedGlyphs::from(retained.clone());
    assert_eq!(shared.as_ref(), expected.as_slice());
    assert_eq!(&*shared, expected.as_slice());
    assert_eq!(format!("{shared:?}"), format!("{expected:?}"));
    retained[0].glyph_id = 99;
    retained.clear();
    assert_eq!(shared.as_ref(), expected.as_slice());
    assert_eq!((&shared).into_iter().cloned().collect::<Vec<_>>(), expected);
}
#[test]
fn clone_is_one_payload_and_equality_is_by_contents() {
    let a = SharedGlyphs::from(fixture());
    let b = a.clone();
    let equal = SharedGlyphs::from(fixture());
    assert_eq!(a.as_ptr(), b.as_ptr());
    assert_eq!(Arc::strong_count(&a.0), 2);
    assert_ne!(a.as_ptr(), equal.as_ptr());
    assert_eq!(a, equal);
    let mut other = fixture();
    other[1].cluster = 8;
    assert_ne!(a, SharedGlyphs::from(other));
}
#[test]
fn empty_payload_is_a_successful_owned_slice() {
    let a = SharedGlyphs::default();
    let b = SharedGlyphs::from(Vec::new());
    assert!(a.is_empty());
    assert_eq!(a, b);
    assert_eq!((&a).into_iter().count(), 0);
    let c = a.clone();
    assert_eq!(a.as_ptr(), c.as_ptr());
}
#[test]
fn last_owner_releases_and_retained_clone_crosses_threads() {
    let a = SharedGlyphs::from(fixture());
    let weak = Arc::downgrade(&a.0);
    let b = a.clone();
    drop(a);
    assert_eq!(weak.strong_count(), 1);
    assert!(weak.upgrade().is_some());
    let returned = std::thread::spawn(move || {
        assert_eq!(b.as_ref(), fixture().as_slice());
        b
    })
    .join()
    .unwrap();
    assert_eq!(returned.as_ref(), fixture().as_slice());
    drop(returned);
    assert!(weak.upgrade().is_none());
}
