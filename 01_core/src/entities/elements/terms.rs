//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/entities/elements/terms.md
//! @prompt-hash f69ee4b5
//! @layer L1
//! @updated 2026-06-11
//!
//! `TermsElem` — Lote 3 P318 (família lista/termos). Lista de termos.
//! Contentor de prosa: recurse em map_content E map_text (precedente Heading).

use std::sync::Arc;

use ecow::EcoString;
use indexmap::IndexMap;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::source_result::SourceResult;
use crate::entities::value::Value;

/// Espécie dos itens de um conjunto.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub(crate) enum ListKind {
    #[default]
    Terms,
    List,
    Enum,
}

impl ListKind {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::Terms => "terms",
            Self::List => "list",
            Self::Enum => "enum",
        }
    }
}

/// Conjunto de itens: lista, enumeração ou lista de termos.
#[derive(Debug, Clone, PartialEq)]
pub struct TermsElem {
    /// Itens, seguidos dos rótulos já preparados.
    pub items: Vec<Content>,
    pub(crate) kind: ListKind,
    /// Argumentos lexicais do construtor.
    pub(crate) named: Option<Arc<IndexMap<EcoString, Value, rustc_hash::FxBuildHasher>>>,
    /// Quantos dos últimos `items` são rótulos preparados.
    pub(crate) labels: usize,
    /// Regras de exibição já aplicadas a este conjunto.
    pub(crate) guards: Vec<crate::entities::show::RuleId>,
}

// `Hash` manual via `Debug`: `Value` carrega `f64` (regime de `ShapeElem`).
impl std::hash::Hash for TermsElem {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        format!("{self:?}").hash(state);
    }
}

impl TermsElem {
    /// Reconstrói com outros itens, conservando o resto.
    pub fn with_items(&self, items: Vec<Content>) -> Self {
        Self { items, ..self.clone() }
    }

    /// Nome do elemento da linguagem.
    pub fn element_name(&self) -> &'static str {
        self.kind.name()
    }

    /// Os itens, sem os rótulos preparados.
    pub fn children(&self) -> &[Content] {
        &self.items[..self.items.len().saturating_sub(self.labels)]
    }

    /// O rótulo preparado do item `index`.
    pub(crate) fn label(&self, index: usize) -> Option<(usize, &Content)> {
        let first = self.children().len();
        (index < self.labels).then(|| (first + index, &self.items[first + index]))
    }

    pub(crate) fn lexical(&self, name: &str) -> Option<&Value> {
        self.named.as_ref()?.get(name)
    }
}

impl Element for TermsElem {
    fn plain_text(&self) -> String {
        self.children()
            .iter()
            .map(|t| t.plain_text())
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        let items: SourceResult<Vec<Content>> =
            self.items.iter().map(|c| c.map_content(transform)).collect();
        Ok(Content::Terms(Arc::new(self.with_items(items?))))
    }

    fn map_text<F>(&self, transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::Terms(Arc::new(
            self.with_items(self.items.iter().map(|c| c.map_text(transform)).collect()),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ex() -> TermsElem {
        TermsElem {
            items: vec![Content::text("a"), Content::text("b")],
 kind: ListKind::Terms, named: None, labels: 0, guards: vec![],
        }
    }

    #[test]
    fn plain_text_join_newline() {
        assert_eq!(ex().plain_text(), "a\nb");
    }

    #[test]
    fn is_empty_sem_items() {
        assert!(!ex().is_empty());
        assert!(TermsElem { items: vec![], kind: ListKind::Terms, named: None, labels: 0, guards: vec![] }.is_empty());
    }

    #[test]
    fn map_content_recurse_items() {
        let mut f = |c: &Content| -> SourceResult<Option<Content>> {
            match c {
                Content::Text(s) if s.as_str() == "a" => Ok(Some(Content::text("Z"))),
                _ => Ok(None),
            }
        };
        match ex().map_content(&mut f).unwrap() {
            Content::Terms(e) => assert_eq!(e.plain_text(), "Z\nb"),
            _ => panic!("esperado Terms"),
        }
    }
}
