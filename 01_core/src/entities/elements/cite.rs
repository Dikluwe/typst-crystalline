//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/cite.md
//! @prompt-hash 31513e36
//! @layer L1
//! @updated 2026-06-11
//!
//! `CiteElem` — Lote 9 P324 (por largura). `cite(key, supplement, form)`.
//! **Locatável** (M1, junto de Heading/Figure) — absorve `extract_payload`.
//! Contentor parcial: recurse no `supplement`.

use std::sync::Arc;

use crate::entities::citation_form::CitationForm;
use crate::entities::citation_style::CitationStyle;
use crate::entities::content::Content;
use crate::entities::element_kind::ElementKind;
use crate::entities::element_payload::ElementPayload;
use crate::entities::elements::Element;
use crate::entities::source_result::SourceResult;

/// Citação `@key` com `supplement`, `form` e `style` opcionais.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct CiteElem {
    pub key: String,
    pub supplement: Option<Content>,
    pub form: Option<CitationForm>,
    /// **P468** — estilo de citação (`numeric`, `author-date`, `alphabetic`).
    pub style: Option<CitationStyle>,
}

impl Element for CiteElem {
    fn plain_text(&self) -> String {
        let mut out = format!("[{}]", self.key);
        if let Some(s) = &self.supplement {
            out.push_str(&s.plain_text());
        }
        out
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::Cite(Arc::new(CiteElem {
            key: self.key.clone(),
            supplement: self
                .supplement
                .as_ref()
                .map(|s| s.map_content(transform))
                .transpose()?,
            form: self.form,
            style: self.style,
        })))
    }

    fn map_text<F>(&self, transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::Cite(Arc::new(CiteElem {
            key: self.key.clone(),
            supplement: self.supplement.as_ref().map(|s| s.map_text(transform)),
            form: self.form,
            style: self.style,
        }))
    }

    fn element_kind(&self) -> Option<ElementKind> {
        Some(ElementKind::Citation)
    }

    fn to_payload(&self) -> Option<ElementPayload> {
        Some(ElementPayload::Citation { key: self.key.clone() })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    fn ex() -> CiteElem {
        CiteElem {
            key: "smith2024".to_string(),
            supplement: None,
            form: None,
            style: None,
        }
    }

    #[test]
    fn plain_text_com_key() {
        assert_eq!(ex().plain_text(), "[smith2024]");
    }

    #[test]
    fn plain_text_com_supplement() {
        let c = CiteElem {
            key: "k".to_string(),
            supplement: Some(Content::text(" p.5")),
            form: None,
            style: None,
        };
        assert_eq!(c.plain_text(), "[k] p.5");
    }

    #[test]
    fn is_empty_default_false() {
        assert!(!ex().is_empty());
    }

    #[test]
    fn locatavel_kind_e_payload() {
        assert_eq!(ex().element_kind(), Some(ElementKind::Citation));
        assert_eq!(
            ex().to_payload(),
            Some(ElementPayload::Citation { key: "smith2024".to_string() })
        );
    }

    #[test]
    fn map_content_recurse_supplement_preserva_key() {
        let c = CiteElem {
            key: "k".to_string(),
            supplement: Some(Content::text("a")),
            form: Some(CitationForm::Prose),
            style: None,
        };
        let mut f = |x: &Content| -> SourceResult<Option<Content>> {
            match x {
                Content::Text(s) if s.as_str() == "a" => Ok(Some(Content::text("Z"))),
                _ => Ok(None),
            }
        };
        match c.map_content(&mut f).unwrap() {
            Content::Cite(e) => {
                assert_eq!(e.key, "k");
                assert_eq!(e.form, Some(CitationForm::Prose));
                assert!(
                    matches!(e.supplement.as_ref().unwrap(), Content::Text(s) if s.as_str() == "Z")
                );
            }
            _ => panic!("esperado Cite"),
        }
    }

    fn h(e: &CiteElem) -> u64 {
        let mut s = DefaultHasher::new();
        e.hash(&mut s);
        s.finish()
    }

    #[test]
    fn payload_diferente_produz_hash_diferente() {
        let outro = CiteElem {
            key: "other".to_string(),
            supplement: None,
            form: None,
            style: None,
        };
        assert_ne!(h(&ex()), h(&outro));
    }
}
