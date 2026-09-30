//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/content/morphology.md
//! @prompt-hash 0b6d01b5
//! @layer L1
//! @updated 2026-09-26

use super::*;

impl Content {
    pub fn morph_canon(&self) -> Content {
        let mut transform = |node: &Content| -> crate::entities::source_result::SourceResult<
            Option<Content>,
        > {
            Ok(match node {
                Content::Text(s) => Some(Content::Text(s.clone())),
                Content::SpaceFromNewline => Some(Content::Space),
                // F-5a de-bake (P364): heading/equation numbering deixaram de
                // viver em campo assado — viajam como `custom` num
                // `Content::Styled` semanticamente vazio, já tratado
                // transparente pelo arm acima (desce no body). Os arms
                // dedicados (que zeravam `numbering_active`) tornaram-se
                // redundantes e foram removidos: o mecanismo é único (o custom
                // é render, não morfologia — P345 N1).
                Content::Styled(styled) if styled.styles.is_semantically_empty() => {
                    Some(styled.body.clone())
                }
                _ => None, // neutro: N16[β] — variantes sem canonicalização especial retornam None em morph_canon
            })
        };
        // `transform` é total (nunca devolve `Err`) → `map_content` não falha.
        self.map_content(&mut transform)
            .expect("morph_canon: transform total nunca devolve Err")
    }
}

#[cfg(test)]
mod morphology_tests {
    use super::Content;

    #[test]
    fn empty_is_already_canonical() {
        assert_eq!(Content::Empty.morph_canon(), Content::Empty);
    }
}
