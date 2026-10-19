//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/entities/elements/math_delimited.md
//! @prompt-hash d4791efa
//! @layer L1
//! @updated 2026-10-10
//!
//! `MathDelimitedElem` — Lote 2 P317 (família math). Expressão delimitada.
//! Comportamento idêntico ao braço anterior do hub (content-preserving).

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::layout_types::Length;
use crate::entities::rel::Rel;
use crate::entities::source_result::SourceResult;
use crate::entities::value::Value;

/// Expressão entre delimitadores (`(...)`, `[...]`, `{...}`). `open`/`close`
/// são os caracteres delimitadores; variante própria para selecção de variantes
/// de tamanho no layout.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MathDelimitedElem {
    pub open: char,
    pub body: Content,
    pub close: char,
}

/// Elemento público `math.lr`, distinto do carrier mecânico delimitado.
#[derive(Debug, Clone, PartialEq)]
pub struct MathLrElem {
    pub body: Content,
    pub size: Rel<Length>,
    pub size_explicit: bool,
}

/// Elemento público `math.mid`, escalado pelo `math.lr` envolvente mais próximo.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MathMidElem {
    pub body: Content,
}

impl std::hash::Hash for MathLrElem {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        fn bits(value: f64) -> u64 {
            if value == 0.0 {
                0
            } else {
                value.to_bits()
            }
        }
        self.body.hash(state);
        bits(self.size.rel).hash(state);
        bits(self.size.abs.abs.0).hash(state);
        bits(self.size.abs.em).hash(state);
        self.size_explicit.hash(state);
    }
}

impl Element for MathDelimitedElem {
    fn plain_text(&self) -> String {
        format!("{}{}{}", self.open, self.body.plain_text(), self.close)
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(self
            .map_content_if_changed(transform)?
            .unwrap_or_else(|| Content::MathDelimited(Arc::new(self.clone()))))
    }

    fn map_text<F>(&self, _transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        // Terminal (math structural; não desce).
        Content::MathDelimited(Arc::new(self.clone()))
    }
}

impl Element for MathLrElem {
    fn plain_text(&self) -> String {
        self.body.plain_text()
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(self
            .map_content_if_changed(transform)?
            .unwrap_or_else(|| Content::MathLr(Arc::new(self.clone()))))
    }

    fn map_text<F>(&self, _transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::MathLr(Arc::new(self.clone()))
    }

    fn get_field(&self, field: &str) -> Option<Value> {
        match field {
            "body" => Some(Value::Content(self.body.clone())),
            "size" if self.size_explicit => Some(Value::Relative(self.size)),
            _ => None,
        }
    }
}

impl Element for MathMidElem {
    fn plain_text(&self) -> String {
        self.body.plain_text()
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(self
            .map_content_if_changed(transform)?
            .unwrap_or_else(|| Content::MathMid(Arc::new(self.clone()))))
    }

    fn map_text<F>(&self, _transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::MathMid(Arc::new(self.clone()))
    }
}

impl MathDelimitedElem {
    pub(crate) fn map_content_if_changed<F>(
        &self,
        transform: &mut F,
    ) -> SourceResult<Option<Content>>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        let body = self.body.map_content_if_changed(transform)?;
        if body.is_none() {
            return Ok(None);
        }
        let mut mapped = self.clone();
        if let Some(changed) = body {
            mapped.body = changed;
        }
        Ok(Some(Content::MathDelimited(Arc::new(mapped))))
    }
}

impl MathLrElem {
    pub(crate) fn map_content_if_changed<F>(
        &self,
        transform: &mut F,
    ) -> SourceResult<Option<Content>>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        let body = self.body.map_content_if_changed(transform)?;
        if body.is_none() {
            return Ok(None);
        }
        let mut mapped = self.clone();
        if let Some(changed) = body {
            mapped.body = changed;
        }
        Ok(Some(Content::MathLr(Arc::new(mapped))))
    }
}

impl MathMidElem {
    pub(crate) fn map_content_if_changed<F>(
        &self,
        transform: &mut F,
    ) -> SourceResult<Option<Content>>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        let body = self.body.map_content_if_changed(transform)?;
        if body.is_none() {
            return Ok(None);
        }
        let mut mapped = self.clone();
        if let Some(changed) = body {
            mapped.body = changed;
        }
        Ok(Some(Content::MathMid(Arc::new(mapped))))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ex() -> MathDelimitedElem {
        MathDelimitedElem { open: '(', body: Content::text("x"), close: ')' }
    }

    #[test]
    fn plain_text_envolve() {
        assert_eq!(ex().plain_text(), "(x)");
    }

    #[test]
    fn igualdade_estrutural() {
        assert_eq!(ex(), ex());
        let mut other = ex();
        other.close = ']';
        assert_ne!(ex(), other);
    }

    #[test]
    fn map_content_recurse_body_preserva_delim() {
        let mut f = |c: &Content| -> SourceResult<Option<Content>> {
            match c {
                Content::Text(s) if s.as_str() == "x" => Ok(Some(Content::text("Z"))),
                _ => Ok(None),
            }
        };
        let r = ex().map_content(&mut f).unwrap();
        match r {
            Content::MathDelimited(e) => {
                assert_eq!(e.plain_text(), "(Z)");
                assert_eq!((e.open, e.close), ('(', ')'));
            }
            _ => panic!("esperado MathDelimited"),
        }
    }
}
