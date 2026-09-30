//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/content/rust-hash.md
//! @prompt-hash 82603586
//! @layer L1
//! @updated 2026-09-26

use super::*;

// P204B (M8): impl Hash via hash_content (existing Debug-based hash
// function from P162). Necessária para `#[comemo::track]` no trait
// `Introspector` per ADR-0073 — métodos como `headings_for_toc` que
// retornam `&[(Label, Option<String>, Content, usize)]` exigem `Content: Hash`.
// Estratégia: delega ao hash_content u128 que já existe, hashing-o
// como tuple no hasher genérico.
impl std::hash::Hash for Content {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            // `Content` has a deliberately compact `Debug` implementation,
            // therefore the historical Debug-based fallback cannot observe
            // the causal fields of this carrier.
            Self::MathCases(elem) => {
                std::mem::discriminant(self).hash(state);
                elem.hash(state);
            }
            Self::MathMatrix(elem) => {
                std::mem::discriminant(self).hash(state);
                elem.hash(state);
            }
            _ => crate::entities::content_hash::hash_content(self).hash(state),
        }
    }
}

#[cfg(test)]
mod rust_hash_tests {
    use super::Content;
    use std::hash::{DefaultHasher, Hash, Hasher};

    #[test]
    fn equal_space_variants_have_equal_hashes() {
        fn hash(content: &Content) -> u64 {
            let mut state = DefaultHasher::new();
            content.hash(&mut state);
            state.finish()
        }
        assert_eq!(hash(&Content::Space), hash(&Content::SpaceFromNewline));
    }
}
