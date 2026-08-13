//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/counter_display.md
//! @prompt-hash 7f6f9c33
//! @layer L1
//! @updated 2026-06-11
//!
//! `CounterDisplayElem` — Lote 6 P321 (família state/counter). Legacy
//! single-pass (`counter(key).display()`). **Não-locatável** (DEBT-10).

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::counter::CounterKey;
use crate::entities::elements::Element;
use crate::entities::source_result::SourceResult;

/// Exibição de contador resolvida no Layouter (single-pass). `kind` é a chave.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct CounterDisplayElem {
    pub kind: CounterKey,
}

impl Element for CounterDisplayElem {
    fn plain_text(&self) -> String {
        String::new()
    }

    fn map_content<F>(&self, _transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::CounterDisplay(Arc::new(self.clone())))
    }

    fn map_text<F>(&self, _transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::CounterDisplay(Arc::new(self.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_text_vazio() {
        assert_eq!(
            CounterDisplayElem {
                kind: CounterKey::Str("heading".into())
            }
            .plain_text(),
            ""
        );
    }

    #[test]
    fn nao_locatavel() {
        let e = CounterDisplayElem {
            kind: CounterKey::Str("figure".into()),
        };
        assert!(e.element_kind().is_none());
        assert!(e.to_payload().is_none());
    }

    #[test]
    fn igualdade_por_kind() {
        assert_eq!(
            CounterDisplayElem {
                kind: CounterKey::Str("h".into())
            },
            CounterDisplayElem {
                kind: CounterKey::Str("h".into())
            }
        );
        assert_ne!(
            CounterDisplayElem {
                kind: CounterKey::Str("h".into())
            },
            CounterDisplayElem {
                kind: CounterKey::Str("f".into())
            }
        );
    }
}
