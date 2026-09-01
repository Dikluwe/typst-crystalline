//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/math_underline.md
//! @prompt-hash 27b6cc39
//! @layer L1
//! @updated 2026-08-31
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

impl Element for MathUnderlineElem {
    fn plain_text(&self) -> String {
        self.body.plain_text()
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::MathUnderline(Arc::new(MathUnderlineElem {
            body: self.body.map_content(transform)?,
        })))
    }

    fn map_text<F>(&self, _transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::MathUnderline(Arc::new(self.clone()))
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
