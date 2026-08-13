//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/pagebreak.md
//! @prompt-hash d5935fbe
//! @layer L1
//! @updated 2026-06-11
//!
//! `PagebreakElem` — Lote 5 P320. Quebra de página (`pagebreak()`). Event leaf.
//! `Hash` por derive: `Parity` ganhou `Hash` no Lote 5 (dependência do lote;
//! enum `Copy+Eq` sem floats → canónico). Ver `pagebreak.md`.

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::parity::Parity;
use crate::entities::source_result::SourceResult;

/// Quebra de página. `weak` colapsável; `to` força paridade da próxima página.
/// Nunca vazio (event observable).
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct PagebreakElem {
    pub weak: bool,
    pub to: Option<Parity>,
}

impl Element for PagebreakElem {
    fn plain_text(&self) -> String {
        String::new()
    }

    fn is_empty(&self) -> bool {
        // Event com efeito mesmo sem body — nunca vazio (cf. Divider).
        false
    }

    fn map_content<F>(&self, _transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::Pagebreak(Arc::new(self.clone())))
    }

    fn map_text<F>(&self, _transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::Pagebreak(Arc::new(self.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_text_vazio() {
        assert_eq!(PagebreakElem { weak: false, to: None }.plain_text(), "");
    }

    #[test]
    fn is_empty_sempre_false() {
        assert!(!PagebreakElem { weak: true, to: None }.is_empty());
    }

    #[test]
    fn igualdade_por_weak_e_to() {
        let a = PagebreakElem { weak: false, to: Some(Parity::Even) };
        assert_eq!(a.clone(), a.clone());
        assert_ne!(a, PagebreakElem { weak: false, to: Some(Parity::Odd) });
        assert_ne!(
            PagebreakElem { weak: false, to: None },
            PagebreakElem { weak: true, to: None },
        );
    }
}
