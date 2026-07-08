//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/heading.md
//! @prompt-hash cd89a7bc
//! @layer L1
//! @updated 2026-06-10
//!
//! `HeadingElem` — teto do lote piloto D: locatável (prova a absorção de
//! `ElementKind`/`ElementPayload`).

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::content_hash::hash_content;
use crate::entities::counter_update::CounterUpdate;
use crate::entities::element_kind::ElementKind;
use crate::entities::element_payload::ElementPayload;
use crate::entities::elements::Element;
use crate::entities::source_result::SourceResult;
use crate::entities::value::Value;

/// Cabeçalho com nível 1–6 e corpo.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct HeadingElem {
    /// Nível clamped 1..=6.
    pub level: u8,
    /// Corpo do cabeçalho (era `Box<Content>`; agora `Content` dentro do `Arc`).
    pub body: Content,
    /// P493/P605 — visível no índice do documento (`#outline()`); default true.
    pub outlined: bool,
    /// **P606** — visível na árvore de bookmarks PDF (`/Outlines`);
    /// `None` significa "seguir `outlined`" (comportamento vanilla `auto`).
    pub bookmarked: Option<bool>,
}

impl HeadingElem {
    /// Construtor com clamp de paridade (content.rs:1267).
    ///
    /// **F-5a de-bake (P364, `f_fronteira_e1.md` §3a.9):** o campo assado
    /// `numbering_active` foi **removido** — o gate de numeração vive **só na
    /// chain** (`#set heading(numbering:)` → `custom("heading.numbering")`,
    /// transportado por `Content::Styled`). O consumidor (layout/introspect) lê o
    /// gate da chain; o **valor** do contador segue via Introspector
    /// (`formatted_counter_at("heading")`, P335 incondicional).
    pub fn new(level: u8, body: Content) -> Self {
        Self { level: level.clamp(1, 6), body, outlined: true, bookmarked: None }
    }

    /// P493 — construtor com controlo de `outlined`.
    pub fn new_with_outlined(level: u8, body: Content, outlined: bool) -> Self {
        Self { level: level.clamp(1, 6), body, outlined, bookmarked: None }
    }

    /// **P606** — construtor completo com controlo separado de `outlined` e `bookmarked`.
    pub fn new_with_outlined_and_bookmarked(
        level: u8,
        body: Content,
        outlined: bool,
        bookmarked: Option<bool>,
    ) -> Self {
        Self { level: level.clamp(1, 6), body, outlined, bookmarked }
    }

    /// **P606** — valor efectivo de `bookmarked`: explicitamente definido ou segue `outlined`.
    pub fn is_bookmarked(&self) -> bool {
        self.bookmarked.unwrap_or(self.outlined)
    }
}

impl Element for HeadingElem {
    fn plain_text(&self) -> String {
        self.body.plain_text()
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::Heading(Arc::new(HeadingElem {
            level: self.level,
            outlined: self.outlined,
            bookmarked: self.bookmarked,
            body: self.body.map_content(transform)?,
        })))
    }

    fn map_text<F>(&self, transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::Heading(Arc::new(HeadingElem {
            level: self.level,
            outlined: self.outlined,
            bookmarked: self.bookmarked,
            body: self.body.map_text(transform),
        }))
    }

    fn get_field(&self, field: &str) -> Option<Value> {
        match field {
            "body" => Some(Value::Content(self.body.clone())),
            "level" => Some(Value::Int(self.level as i64)),
            "outlined" => Some(Value::Bool(self.outlined)),
            "bookmarked" => self.bookmarked.map(Value::Bool),
            _ => None,
        }
    }

    fn element_kind(&self) -> Option<ElementKind> {
        Some(ElementKind::Heading)
    }

    fn to_payload(&self) -> Option<ElementPayload> {
        Some(ElementPayload::Heading {
            depth: self.level,
            body_hash: hash_content(&self.body),
            counter_update: CounterUpdate::Step,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn h() -> HeadingElem {
        HeadingElem::new(2, Content::text("Intro"))
    }

    #[test]
    fn clamp_nivel() {
        assert_eq!(HeadingElem::new(9, Content::empty()).level, 6);
        assert_eq!(HeadingElem::new(0, Content::empty()).level, 1);
    }

    #[test]
    fn plain_text_e_o_corpo() {
        assert_eq!(h().plain_text(), "Intro");
    }

    #[test]
    fn get_field_body_level() {
        assert!(matches!(h().get_field("body"), Some(Value::Content(_))));
        assert!(matches!(h().get_field("level"), Some(Value::Int(2))));
        assert!(h().get_field("inexistente").is_none());
    }

    #[test]
    fn locatavel_kind_e_payload() {
        assert_eq!(h().element_kind(), Some(ElementKind::Heading));
        assert!(matches!(
            h().to_payload(),
            Some(ElementPayload::Heading { depth: 2, .. })
        ));
    }
}
