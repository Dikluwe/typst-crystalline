//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/entities/elements/math_underline.md
//! @prompt-hash f2d8bf2d
//! @layer L1
//! @updated 2026-10-10
//!
//! `MathUnderlineElem` — underline matemático body-only, distinto da
//! decoração textual `UnderlineElem`.

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::source_result::SourceResult;

/// Underline matemático estrutural. A geometria pertence ao owner de layout.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MathUnderlineElem {
    pub body: Content,
}

/// Overline matemático estrutural, distinto da decoração textual homônima.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MathOverlineElem {
    pub body: Content,
}

impl Element for MathUnderlineElem {
    fn plain_text(&self) -> String {
        self.body.plain_text()
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(self
            .map_content_if_changed(transform)?
            .unwrap_or_else(|| Content::MathUnderline(Arc::new(self.clone()))))
    }

    fn map_text<F>(&self, _transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::MathUnderline(Arc::new(self.clone()))
    }
}

impl Element for MathOverlineElem {
    fn plain_text(&self) -> String {
        self.body.plain_text()
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(self
            .map_content_if_changed(transform)?
            .unwrap_or_else(|| Content::MathOverline(Arc::new(self.clone()))))
    }

    fn map_text<F>(&self, _transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::MathOverline(Arc::new(self.clone()))
    }
}

impl MathUnderlineElem {
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
        Ok(Some(Content::MathUnderline(Arc::new(mapped))))
    }
}

impl MathOverlineElem {
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
        Ok(Some(Content::MathOverline(Arc::new(mapped))))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1292_math_underline_owner_preserva_body_estrutural() {
        let elem = MathUnderlineElem { body: Content::MathIdent("x".into()) };
        assert_eq!(elem.plain_text(), "x");
        assert_eq!(
            elem.map_text(&mut |text| text.to_uppercase()),
            Content::MathUnderline(Arc::new(elem)),
            "math.underline continua body-only e distinto da decoração textual",
        );
    }
}
