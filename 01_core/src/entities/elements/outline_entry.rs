//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/outline_entry.md
//! @prompt-hash af09bd6f
//! @layer L1
//! @updated 2026-09-14

use std::sync::Arc;

use super::Element;
use crate::entities::content::Content;
use crate::entities::element_kind::ElementKind;
use crate::entities::source_result::SourceResult;
use crate::entities::value::Value;

/// Entrada realizada por um `outline`.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct OutlineEntryElem {
    pub level: usize,
    pub element: Content,
    pub fill: Option<Content>,
    pub fill_set: bool,
}

impl OutlineEntryElem {
    pub fn new(
        level: usize,
        element: Content,
        fill: Option<Content>,
        fill_set: bool,
    ) -> Self {
        Self { level: level.max(1), element, fill, fill_set }
    }

    pub fn body(&self) -> Result<Content, String> {
        match &self.element {
            Content::Heading(heading) => Ok(heading.body.clone()),
            other => Err(format!("cannot outline {}", other.elem_name())),
        }
    }
}

impl Element for OutlineEntryElem {
    fn element_kind(&self) -> Option<ElementKind> {
        Some(ElementKind::OutlineEntry)
    }

    fn to_payload(&self) -> Option<crate::entities::element_payload::ElementPayload> {
        Some(crate::entities::element_payload::ElementPayload::NativeElement)
    }

    fn plain_text(&self) -> String {
        self.body().map_or_else(|_| String::new(), |body| body.plain_text())
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::OutlineEntry(Arc::new(Self {
            level: self.level,
            element: self.element.map_content(transform)?,
            fill: self.fill.as_ref().map(|c| c.map_content(transform)).transpose()?,
            fill_set: self.fill_set,
        })))
    }

    fn map_text<F>(&self, transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::OutlineEntry(Arc::new(Self {
            level: self.level,
            element: self.element.map_text(transform),
            fill: self.fill.as_ref().map(|c| c.map_text(transform)),
            fill_set: self.fill_set,
        }))
    }

    fn get_field(&self, field: &str) -> Option<Value> {
        match field {
            "level" => Some(Value::Int(self.level as i64)),
            "element" => Some(Value::Content(self.element.clone())),
            "fill" => Some(self.fill.clone().map(Value::Content).unwrap_or(Value::None)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn body_preserva_content_do_heading() {
        let entry = OutlineEntryElem::new(
            1,
            Content::heading(1, Content::strong(Content::text("H"))),
            None,
            false,
        );
        assert_eq!(entry.body().unwrap(), Content::strong(Content::text("H")));
    }

    #[test]
    fn rejeita_elemento_nao_outlinable() {
        let entry =
            OutlineEntryElem::new(1, Content::strong(Content::text("H")), None, false);
        assert_eq!(entry.body().unwrap_err(), "cannot outline strong");
    }
}
