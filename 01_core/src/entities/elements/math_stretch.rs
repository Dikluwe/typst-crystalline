//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/entities/elements/math_stretch.md
//! @prompt-hash 8060d88a
//! @layer L1

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::layout_types::Length;
use crate::entities::rel::Rel;
use crate::entities::source_result::SourceResult;
use crate::entities::value::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct MathStretchElem {
    pub body: Content,
    pub size: Rel<Length>,
    pub size_explicit: bool,
}

impl std::hash::Hash for MathStretchElem {
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

impl Element for MathStretchElem {
    fn plain_text(&self) -> String {
        self.body.plain_text()
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(self
            .map_content_if_changed(transform)?
            .unwrap_or_else(|| Content::MathStretch(Arc::new(self.clone()))))
    }

    fn map_text<F>(&self, _transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::MathStretch(Arc::new(self.clone()))
    }

    fn get_field(&self, field: &str) -> Option<Value> {
        match field {
            "body" => Some(Value::Content(self.body.clone())),
            "size" if self.size_explicit => Some(Value::Relative(self.size)),
            _ => None,
        }
    }
}

impl MathStretchElem {
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
        Ok(Some(Content::MathStretch(Arc::new(mapped))))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_content_preserva_tamanho_e_presenca() {
        let elem = MathStretchElem {
            body: Content::text("x"),
            size: Rel::from_percent(150.0),
            size_explicit: true,
        };
        let mut rewrite = |_: &Content| Ok(Some(Content::text("y")));
        let Content::MathStretch(mapped) = elem.map_content(&mut rewrite).unwrap() else {
            panic!("MathStretch")
        };
        assert_eq!(mapped.body, Content::text("y"));
        assert_eq!(mapped.size, elem.size);
        assert!(mapped.size_explicit);
    }
}
