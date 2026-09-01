//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/flush.md
//! @prompt-hash 75967e09
//! @layer L1
//! @updated 2026-09-01
//!
//! `FlushElem` — sentinela zero-field que realiza floats pendentes no ponto
//! exacto do fluxo paginado.

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::source_result::SourceResult;

/// Marcador estrutural que drena os floats já pendentes.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct FlushElem;

impl Element for FlushElem {
    fn plain_text(&self) -> String {
        String::new()
    }

    fn map_content<F>(&self, _transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::Flush(Arc::new(self.clone())))
    }

    fn map_text<F>(&self, _transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::Flush(Arc::new(self.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1292_d_flush_is_a_non_empty_terminal_without_text() {
        let flush = FlushElem;
        assert_eq!(flush.plain_text(), "");
        assert!(!flush.is_empty());

        let mut visited = false;
        let mapped = flush
            .map_content(&mut |_content| {
                visited = true;
                Ok(None)
            })
            .unwrap();
        assert!(!visited);
        assert!(matches!(mapped, Content::Flush(_)));
    }
}
