//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/linebreak.md
//! @prompt-hash d7416bdb
//! @layer L1
//! @updated 2026-06-11
//!
//! `LinebreakElem` — quebra de linha (`\\`) com presença de `justify`.

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::source_result::SourceResult;
use crate::entities::value::Value;

/// Quebra de linha forçada (`\\`).
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct LinebreakElem {
    pub justify: bool,
    pub justify_explicit: bool,
}

impl Element for LinebreakElem {
    fn plain_text(&self) -> String {
        "\n".to_string()
    }

    fn get_field(&self, field: &str) -> Option<Value> {
        match field {
            "justify" if self.justify_explicit => Some(Value::Bool(self.justify)),
            _ => None,
        }
    }

    fn map_content<F>(&self, _transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::Linebreak(Arc::new(self.clone())))
    }

    fn map_text<F>(&self, _transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::Linebreak(Arc::new(self.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_text_newline() {
        assert_eq!(linebreak(false, false).plain_text(), "\n");
    }

    #[test]
    fn is_empty_default_false() {
        assert!(!linebreak(false, false).is_empty());
    }

    #[test]
    fn igualdade_inclui_valor_e_presenca() {
        assert_eq!(linebreak(false, false), linebreak(false, false));
        assert_ne!(linebreak(false, false), linebreak(false, true));
        assert_ne!(linebreak(false, true), linebreak(true, true));
    }

    fn linebreak(justify: bool, justify_explicit: bool) -> LinebreakElem {
        LinebreakElem { justify, justify_explicit }
    }
}
