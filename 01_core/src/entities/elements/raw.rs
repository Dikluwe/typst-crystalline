//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/raw.md
//! @prompt-hash 470ded04
//! @layer L1
//! @updated 2026-06-11
//!
//! `RawElem` — Lote 7 P322. Código raw inline/bloco (`` `...` ``). Folha.

use std::sync::Arc;

use ecow::EcoString;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;

/// Código raw. `lang` opcional (syntax highlighting); `block` = bloco vs inline.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct RawElem {
    pub text: EcoString,
    pub lang: Option<EcoString>,
    pub block: bool,
}

impl Element for RawElem {
    fn plain_text(&self) -> String {
        self.text.to_string()
    }

    fn map_content<F>(&self, _transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::Raw(Arc::new(self.clone())))
    }

    fn map_text<F>(&self, _transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::Raw(Arc::new(self.clone()))
    }
}

/// Linha sintetizada de um elemento raw, exposta como `raw.line`.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct RawLineElem {
    pub number: i64,
    pub count: i64,
    pub text: EcoString,
    pub body: Content,
}

impl RawLineElem {
    pub fn new(number: i64, count: i64, text: EcoString, body: Content) -> Self {
        Self { number, count, text, body }
    }

    pub fn construct(values: &[Value]) -> SourceResult<Content> {
        if values.len() != 4 {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "raw.line expects 4 arguments",
            )]);
        }
        let Value::Int(number) = values[0] else {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "raw.line number must be int",
            )]);
        };
        let Value::Int(count) = values[1] else {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "raw.line count must be int",
            )]);
        };
        let Value::Str(text) = &values[2] else {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "raw.line text must be string",
            )]);
        };
        let Value::Content(body) = &values[3] else {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "raw.line body must be content",
            )]);
        };
        Ok(Content::dynamic(Self::new(number, count, text.clone(), body.clone())))
    }
}

impl Element for RawLineElem {
    fn plain_text(&self) -> String {
        self.text.to_string()
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::dynamic(Self {
            number: self.number,
            count: self.count,
            text: self.text.clone(),
            body: self.body.map_content(transform)?,
        }))
    }

    fn map_text<F>(&self, transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::dynamic(Self {
            number: self.number,
            count: self.count,
            text: self.text.clone(),
            body: self.body.map_text(transform),
        })
    }

    fn get_field(&self, field: &str) -> Option<Value> {
        match field {
            "number" => Some(Value::Int(self.number)),
            "count" => Some(Value::Int(self.count)),
            "text" => Some(Value::Str(self.text.clone())),
            "body" => Some(Value::Content(self.body.clone())),
            _ => None,
        }
    }

    fn dyn_kind_name(&self) -> &'static str {
        "raw.line"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ex() -> RawElem {
        RawElem {
            text: "let x = 1".into(),
            lang: Some("rust".into()),
            block: true,
        }
    }

    #[test]
    fn plain_text_e_o_texto() {
        assert_eq!(ex().plain_text(), "let x = 1");
    }

    #[test]
    fn igualdade_estrutural() {
        assert_eq!(ex(), ex());
        let mut other = ex();
        other.block = false;
        assert_ne!(ex(), other);
    }
}
