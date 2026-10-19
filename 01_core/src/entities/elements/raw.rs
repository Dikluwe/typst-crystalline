//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/entities/elements/raw.md
//! @prompt-hash b58d7e5f
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

/// Presença lexical de `block`, inclusive quando falso.
pub const RAW_SET_BLOCK: u8 = 1;
/// Presença lexical de `lang`, inclusive quando `none`.
pub const RAW_SET_LANG: u8 = 2;

/// Código raw. `lang` opcional (syntax highlighting); `block` = bloco vs inline.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct RawElem {
    pub text: EcoString,
    pub lang: Option<EcoString>,
    pub block: bool,
    /// Máscara dos campos opcionais conhecidos; demais bits são reservados.
    pub set_fields: u8,
}

impl Element for RawElem {
    fn get_field(&self, field: &str) -> Option<Value> {
        match field {
            "text" => Some(Value::Str(self.text.clone())),
            "block" if self.set_fields & RAW_SET_BLOCK != 0 => {
                Some(Value::Bool(self.block))
            }
            "lang" if self.set_fields & RAW_SET_LANG != 0 => {
                Some(self.lang.clone().map(Value::Str).unwrap_or(Value::None))
            }
            _ => None,
        }
    }

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
            set_fields: RAW_SET_BLOCK | RAW_SET_LANG,
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

    #[test]
    fn raw_presence_maps_preserve_known_none_and_false() {
        for set_fields in [0, RAW_SET_BLOCK, RAW_SET_LANG, RAW_SET_BLOCK | RAW_SET_LANG] {
            let raw = RawElem {
                text: "x".into(),
                lang: None,
                block: false,
                set_fields,
            };
            let original = Content::Raw(Arc::new(raw.clone()));
            for mapped in [
                original.clone(),
                original.map_content(&mut |_| Ok(None)).unwrap(),
                original.map_text(&mut |s| s.to_owned()),
            ] {
                assert_eq!(mapped, original);
                assert_eq!(mapped.get_field("text"), Some(Value::Str("x".into())));
                assert_eq!(
                    mapped.get_field("block"),
                    (set_fields & RAW_SET_BLOCK != 0).then_some(Value::Bool(false))
                );
                assert_eq!(
                    mapped.get_field("lang"),
                    (set_fields & RAW_SET_LANG != 0).then_some(Value::None)
                );
            }
        }
    }

    #[test]
    fn raw_presence_required_constructor_block_and_optional_language() {
        let inline = Content::raw("x", None, false);
        assert_eq!(inline.get_field("block"), Some(Value::Bool(false)));
        assert_eq!(inline.get_field("lang"), None);
        let typed = Content::raw("x", Some("typc".into()), false);
        assert_eq!(typed.get_field("lang"), Some(Value::Str("typc".into())));
    }
}
