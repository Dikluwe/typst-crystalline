//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/outline.md
//! @prompt-hash 28a5cdc1
//! @layer L1
//! @updated 2026-06-11
//!
//! `OutlineElem` — Lote 8 P323 (por largura). `outline()` — índice do documento.
//! P457: expande para `{ title, depth, indent }` (campos settable do vanilla).
//! **Locatável** (queryable desde P178) — absorve `extract_payload`.

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::element_kind::ElementKind;
use crate::entities::element_payload::ElementPayload;
use crate::entities::elements::Element;
use crate::entities::func::Func;
use crate::entities::layout_types::Length;
use crate::entities::source_result::SourceResult;

/// Alvo do índice: headings (TOC), figures (LoF) ou tables (LoT).
///
/// **P472** — `OutlineTarget` distingue os três tipos de lista gerados
/// por `outline()`, `lof()` e `lot()`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum OutlineTarget {
    #[default]
    Headings,
    Figures,
    Tables,
}

/// **P502** — Valor aceite por `outline(indent:)`.
/// Vanilla 0.14.2/0.15.0: `length | function | auto`.
/// Cristalino mantém `bool` para compatibilidade reversa.
#[derive(Debug, Clone, PartialEq)]
pub enum OutlineIndent {
    Auto,
    Bool(bool),
    Length(Length),
    Function(Func),
}

impl std::hash::Hash for OutlineIndent {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Function e Length não implementam Hash; usa representação Debug
        // (paridade content_hash; OutlineIndent nunca é chave de mapa).
        format!("{self:?}").hash(state);
    }
}

impl OutlineIndent {
    /// Indica se a indentação está activa no layout actual.
    /// `Length`/`Function` são aceites mas tratados como `Auto` até
    /// a renderização específica ser implementada (scope-out P502).
    pub fn is_active(&self) -> bool {
        match self {
            OutlineIndent::Auto => true,
            OutlineIndent::Bool(b) => *b,
            OutlineIndent::Length(_) | OutlineIndent::Function(_) => true,
        }
    }
}

/// Marcador de índice (table of contents / list of figures / list of tables).
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct OutlineElem {
    pub title: Option<Content>,
    pub depth: usize,
    pub indent: OutlineIndent,
    /// **P472** — alvo do índice. Default `Headings` (TOC normal).
    pub target: OutlineTarget,
}

impl OutlineElem {
    /// Defaults do vanilla: sem título customizado, profundidade 3, indentação auto.
    pub fn new(title: Option<Content>, depth: usize, indent: OutlineIndent) -> Self {
        Self {
            title,
            depth,
            indent,
            target: OutlineTarget::Headings,
        }
    }

    /// **P472** — constrói com `target` explícito.
    pub fn with_target(
        title: Option<Content>,
        depth: usize,
        indent: OutlineIndent,
        target: OutlineTarget,
    ) -> Self {
        Self { title, depth, indent, target }
    }
}

impl Element for OutlineElem {
    fn plain_text(&self) -> String {
        String::new()
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::Outline(Arc::new(OutlineElem {
            title: self.title.as_ref().map(|c| c.map_content(transform)).transpose()?,
            depth: self.depth,
            indent: self.indent.clone(),
            target: self.target,
        })))
    }

    fn map_text<F>(&self, transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::Outline(Arc::new(OutlineElem {
            title: self.title.as_ref().map(|c| c.map_text(transform)),
            depth: self.depth,
            indent: self.indent.clone(),
            target: self.target,
        }))
    }

    fn element_kind(&self) -> Option<ElementKind> {
        Some(ElementKind::Outline)
    }

    fn to_payload(&self) -> Option<ElementPayload> {
        Some(ElementPayload::Outline)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_text_vazio() {
        assert_eq!(OutlineElem::new(None, 3, OutlineIndent::Auto).plain_text(), "");
    }

    #[test]
    fn is_empty_default_false() {
        assert!(!OutlineElem::new(None, 3, OutlineIndent::Auto).is_empty());
    }

    #[test]
    fn igualdade_singleton() {
        assert_eq!(
            OutlineElem::new(Some(Content::text("A")), 2, OutlineIndent::Bool(true)),
            OutlineElem::new(Some(Content::text("A")), 2, OutlineIndent::Bool(true))
        );
        assert_ne!(
            OutlineElem::new(None, 3, OutlineIndent::Auto),
            OutlineElem::new(None, 2, OutlineIndent::Auto)
        );
    }

    #[test]
    fn map_content_terminal_preserva_campos() {
        let mut f = |_c: &Content| -> SourceResult<Option<Content>> { Ok(None) };
        let elem =
            OutlineElem::new(Some(Content::text("T")), 3, OutlineIndent::Bool(true));
        match elem.map_content(&mut f).unwrap() {
            Content::Outline(e) => {
                assert_eq!(
                    e.title.as_ref().map(|c| c.plain_text()),
                    Some("T".to_string())
                );
                assert_eq!(e.depth, 3);
                assert_eq!(e.indent, OutlineIndent::Bool(true));
            }
            _ => panic!("esperado Outline"),
        }
    }

    #[test]
    fn locatavel_kind_e_payload() {
        assert_eq!(
            OutlineElem::new(None, 3, OutlineIndent::Auto).element_kind(),
            Some(ElementKind::Outline)
        );
        assert_eq!(
            OutlineElem::new(None, 3, OutlineIndent::Auto).to_payload(),
            Some(ElementPayload::Outline)
        );
    }
}
