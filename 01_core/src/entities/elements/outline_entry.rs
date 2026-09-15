//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/outline_entry.md
//! @prompt-hash ecdf19ae
//! @layer L1
//! @updated 2026-09-14

use std::sync::Arc;

use super::Element;
use crate::entities::content::Content;
use crate::entities::element_kind::ElementKind;
use crate::entities::location::Location;
use crate::entities::source_result::SourceResult;
use crate::entities::value::Value;

/// Entrada realizada por um `outline`.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct OutlineEntryElem {
    pub level: usize,
    pub element: Content,
    pub fill: Option<Content>,
    pub fill_set: bool,
    prefix: Option<Content>,
    element_location: Option<Location>,
}

impl OutlineEntryElem {
    pub fn new(
        level: usize,
        element: Content,
        fill: Option<Content>,
        fill_set: bool,
    ) -> Self {
        Self {
            level: level.max(1),
            element,
            fill,
            fill_set,
            prefix: None,
            element_location: None,
        }
    }

    pub fn realized(
        level: usize,
        element: Content,
        fill: Option<Content>,
        fill_set: bool,
        prefix: Option<Content>,
        element_location: Location,
    ) -> Self {
        Self {
            level: level.max(1),
            element,
            fill,
            fill_set,
            prefix,
            element_location: Some(element_location),
        }
    }

    pub fn body(&self) -> Result<Content, String> {
        match &self.element {
            Content::Heading(heading) => Ok(heading.body.clone()),
            other => Err(format!("cannot outline {}", other.elem_name())),
        }
    }

    pub fn prefix(&self) -> Option<Content> {
        self.prefix.clone()
    }

    pub fn element_location(&self) -> Option<Location> {
        self.element_location
    }

    pub(crate) fn remapped(
        &self,
        element: Content,
        fill: Option<Content>,
        prefix: Option<Content>,
    ) -> Self {
        Self {
            level: self.level,
            element,
            fill,
            fill_set: self.fill_set,
            prefix,
            element_location: self.element_location,
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
            prefix: self.prefix.as_ref().map(|c| c.map_content(transform)).transpose()?,
            element_location: self.element_location,
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
            prefix: self.prefix.as_ref().map(|c| c.map_text(transform)),
            element_location: self.element_location,
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

    #[test]
    fn p1403_entry_realizada_preserva_prefixo_e_location() {
        let location = Location::from_raw(7);
        let entry = OutlineEntryElem::realized(
            2,
            Content::heading(2, Content::text("H")),
            None,
            true,
            Some(Content::text("1.2.")),
            location,
        );
        assert_eq!(entry.prefix(), Some(Content::text("1.2.")));
        assert_eq!(entry.element_location(), Some(location));
    }

    #[test]
    fn p1403_entry_direta_nao_inventa_contexto() {
        let entry =
            OutlineEntryElem::new(1, Content::heading(1, Content::text("H")), None, true);
        assert_eq!(entry.prefix(), None);
        assert_eq!(entry.element_location(), None);
    }
}
