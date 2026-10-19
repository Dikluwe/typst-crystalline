//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/entities/content/tests/morphology.md
//! @prompt-hash 5fba234f
//! @layer L1
//! @updated 2026-09-26

use super::super::*;

#[test]
fn p1494_space_from_newline_mantem_morfologia_e_hash_publicos() {
    use std::hash::{Hash, Hasher};

    let explicit = Content::Space;
    let inferred = Content::SpaceFromNewline;
    assert_eq!(inferred, explicit);
    assert_eq!(inferred.elem_name(), explicit.elem_name());
    assert_eq!(format!("{inferred:?}"), format!("{explicit:?}"));
    assert_eq!(inferred.plain_text(), explicit.plain_text());
    assert_eq!(
        crate::compiler::eval::repr::repr_content(&inferred),
        crate::compiler::eval::repr::repr_content(&explicit),
    );

    let mut inferred_hash = std::collections::hash_map::DefaultHasher::new();
    let mut explicit_hash = std::collections::hash_map::DefaultHasher::new();
    inferred.hash(&mut inferred_hash);
    explicit.hash(&mut explicit_hash);
    assert_eq!(inferred_hash.finish(), explicit_hash.finish());

    let nested =
        Content::sequence(vec![Content::text("水"), inferred, Content::text("果")]);
    let expected =
        Content::sequence(vec![Content::text("水"), explicit, Content::text("果")]);
    assert_eq!(nested, expected);
    assert_eq!(nested.morph_canon(), expected.morph_canon());
    assert!(!Content::SpaceFromNewline.is_empty());
}
