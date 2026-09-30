//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/content/sequence.md
//! @prompt-hash fe4e3a95
//! @layer L1
//! @updated 2026-09-26

use super::*;

impl Content {
    pub fn sequence(parts: Vec<Content>) -> Self {
        match parts.len() {
            0 => Self::Empty,
            1 => parts.into_iter().next().unwrap(),
            _ => Self::Sequence(parts.into()), // Vec<Content> → Arc<[Content]>
        }
    }
}

#[cfg(test)]
mod sequence_tests {
    use super::Content;

    #[test]
    fn normalizes_empty_and_singleton_inputs() {
        assert_eq!(Content::sequence(vec![]), Content::Empty);
        assert_eq!(Content::sequence(vec![Content::Space]), Content::Space);
    }
}
