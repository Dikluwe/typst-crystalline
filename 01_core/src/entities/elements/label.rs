//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/label.md
//! @layer L1
//! @updated 2026-06-25
//!
//! `LabelElem` — P460. Destino nomeado para referências cruzadas.
//! Wrapper transparente: `map_*` recursam no `body`. Não-locatável no trait
//! (o destino é metadado posicional, sem presença visual própria).

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::source_result::SourceResult;
use ecow::EcoString;

/// Associa um nome de destino a um `body` (wrapper transparente de identidade).
///
/// **P464**: campo `auto` distingue labels gerados automaticamente (`<label>` em
/// headings/figures/equations) de labels criados explicitamente via `#label(...)`.
/// Ambos partilham o mesmo tipo semântico; a origem é metadado, não diferenciação
/// estrutural.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct LabelElem {
    pub name: EcoString,
    pub body: Content,
    pub auto: bool,
}

impl Element for LabelElem {
    fn plain_text(&self) -> String {
        self.body.plain_text()
    }

    fn is_empty(&self) -> bool {
        self.body.is_empty()
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::Label(Arc::new(LabelElem {
            body: self.body.map_content(transform)?,
            name: self.name.clone(),
            auto: self.auto,
        })))
    }

    fn map_text<F>(&self, transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::Label(Arc::new(LabelElem {
            body: self.body.map_text(transform),
            name: self.name.clone(),
            auto: self.auto,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    fn ex() -> LabelElem {
        LabelElem {
            body: Content::text("x"),
            name: "sec1".into(),
            auto: false,
        }
    }

    #[test]
    fn plain_text_delega_ao_body() {
        assert_eq!(ex().plain_text(), "x");
    }

    #[test]
    fn is_empty_delega_ao_body() {
        assert!(!ex().is_empty());
        assert!(LabelElem {
            body: Content::Empty,
            name: "l".into(),
            auto: false
        }
        .is_empty());
    }

    #[test]
    fn map_content_recurse_body_preserva_name() {
        let mut f = |c: &Content| -> SourceResult<Option<Content>> {
            match c {
                Content::Text(s) if s.as_str() == "x" => Ok(Some(Content::text("Z"))),
                _ => Ok(None),
            }
        };
        match ex().map_content(&mut f).unwrap() {
            Content::Label(e) => {
                assert_eq!(e.name.as_str(), "sec1");
                assert!(matches!(&e.body, Content::Text(s) if s.as_str() == "Z"));
            }
            _ => panic!("esperado Label"),
        }
    }

    fn h(e: &LabelElem) -> u64 {
        let mut s = DefaultHasher::new();
        e.hash(&mut s);
        s.finish()
    }

    #[test]
    fn nome_diferente_produz_hash_diferente() {
        assert_ne!(
            h(&ex()),
            h(&LabelElem {
                body: Content::text("x"),
                name: "outro".into(),
                auto: false
            })
        );
    }

    #[test]
    fn auto_true_e_auto_false_sao_ambos_content_label() {
        let user = Content::label("user", Content::text("x"));
        let auto = Content::label_auto("auto", Content::text("x"));
        assert!(matches!(user, Content::Label(_)), "user label deve ser Content::Label");
        assert!(matches!(auto, Content::Label(_)), "auto label deve ser Content::Label");
    }
}
