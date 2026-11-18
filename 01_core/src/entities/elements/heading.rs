//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/entities/elements/heading.md
//! @prompt-hash 30641b81
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
    /// **P829** — campos explicitamente assentes no constructor (máscara
    /// `HEADING_SET_*`). O vanilla distingue campo assente de default
    /// materializado: `heading[H].has("level")` → false (o nível vem da
    /// chain), `heading(level: 2)[H].has("level")` → true; heading de markup
    /// (`= H`) assenta `depth`, não `level` (medido em P829, fixtures b2/b7).
    /// O field access `it.level` (show rules) NÃO consulta esta máscara —
    /// continua a devolver o valor baked.
    pub set_fields: u8,
}

/// **P829** — `level:` assente explicitamente (`#heading(2, [H])` ou
/// `#heading(level: 2)[H]`).
pub const HEADING_SET_LEVEL: u8 = 1;
/// **P829** — heading veio do markup (`= H`): o vanilla assenta `depth`
/// (não `level`); `depth` exposto = `level` (o `offset` do vanilla é
/// scope-out — não modelado).
pub const HEADING_SET_DEPTH: u8 = 2;
/// **P829** — `outlined:` assente explicitamente (named).
pub const HEADING_SET_OUTLINED: u8 = 4;

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
        Self {
            level: level.clamp(1, 6),
            body,
            outlined: true,
            bookmarked: None,
            // Caminho do markup (`= H`) — paridade vanilla: assenta `depth`.
            set_fields: HEADING_SET_DEPTH,
        }
    }

    // Propagate the actual traversal's absence of replacement; equality is irrelevant.
    pub(in crate::entities) fn map_content_if_changed<F>(
        &self,
        transform: &mut F,
    ) -> SourceResult<Option<Content>>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(self
            .body
            .map_content_if_changed(transform)?
            .map(|body| Content::Heading(Arc::new(self.with_body(body)))))
    }

    /// P493 — construtor com controlo de `outlined`.
    pub fn new_with_outlined(level: u8, body: Content, outlined: bool) -> Self {
        Self {
            level: level.clamp(1, 6),
            body,
            outlined,
            bookmarked: None,
            set_fields: HEADING_SET_DEPTH,
        }
    }

    /// **P606** — construtor completo com controlo separado de `outlined` e `bookmarked`.
    ///
    /// `set_fields` começa a zero — o chamador (native `#heading`, P829)
    /// assente os bits `HEADING_SET_*` conforme os argumentos recebidos.
    pub fn new_with_outlined_and_bookmarked(
        level: u8,
        body: Content,
        outlined: bool,
        bookmarked: Option<bool>,
    ) -> Self {
        Self {
            level: level.clamp(1, 6),
            body,
            outlined,
            bookmarked,
            set_fields: 0,
        }
    }

    /// **P606** — valor efectivo de `bookmarked`: explicitamente definido ou segue `outlined`.
    pub fn is_bookmarked(&self) -> bool {
        self.bookmarked.unwrap_or(self.outlined)
    }

    /// **P829** — reconstrói com novo corpo preservando todos os campos,
    /// incluindo `set_fields` (usado nas reconstruções recursivas —
    /// `map_content`/`map_text`/materialização de tempo — para os métodos
    /// `has`/`at`/`fields` continuarem a ver os campos assentes originais).
    pub fn with_body(&self, body: Content) -> Self {
        Self {
            level: self.level,
            body,
            outlined: self.outlined,
            bookmarked: self.bookmarked,
            set_fields: self.set_fields,
        }
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
        Ok(Content::Heading(Arc::new(self.with_body(self.body.map_content(transform)?))))
    }

    fn map_text<F>(&self, transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::Heading(Arc::new(self.with_body(self.body.map_text(transform))))
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
            counter_update: CounterUpdate::step(),
            // P788: placeholder — o valor real é baked no walk a partir da
            // chain (`heading.numbering`), ver `introspect.rs`.
            numbering_active: false,
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

#[cfg(test)]
mod stable_origin_mapping_tests {
    use super::*;
    #[test]
    fn no_replacement_retains_exact_heading_origin() {
        let source =
            Content::Heading(Arc::new(HeadingElem::new_with_outlined_and_bookmarked(
                3,
                Content::strong(Content::emph(Content::text("A"))),
                false,
                Some(true),
            )));
        let Content::Heading(original) = &source else { unreachable!() };
        let mut out = source.clone();
        for _ in 0..3 {
            out = out.map_content(&mut |_| Ok(None)).unwrap();
        }
        let Content::Heading(mapped) = out else { unreachable!() };
        assert!(
            Arc::ptr_eq(original, &mapped),
            "no replacement must preserve source heading origin"
        );
        let other = Content::heading(3, original.body.clone())
            .map_content(&mut |_| Ok(None))
            .unwrap();
        let Content::Heading(other) = other else { unreachable!() };
        assert!(!Arc::ptr_eq(original, &other));
    }
    #[test]
    fn equal_replacement_rebuilds_preserving_fields_and_callback_order() {
        let mut elem = HeadingElem::new_with_outlined_and_bookmarked(
            3,
            Content::sequence(vec![Content::text("A"), Content::text("B")]),
            false,
            Some(true),
        );
        elem.set_fields = 7;
        let owner = Arc::new(elem);
        let source = Content::Heading(owner.clone());
        let mut visits = Vec::new();
        let out = source
            .map_content(&mut |node| {
                if let Content::Text(t) = node {
                    visits.push(t.to_string());
                    if t.as_str() == "A" {
                        return Ok(Some(Content::text("A")));
                    }
                } else if matches!(node, Content::Heading(_)) {
                    visits.push("H".into());
                }
                Ok(None)
            })
            .unwrap();
        let Content::Heading(mapped) = out else { unreachable!() };
        assert!(!Arc::ptr_eq(&owner, &mapped));
        assert_eq!(visits, vec!["A", "B", "H"]);
        assert_eq!(
            (mapped.level, mapped.outlined, mapped.bookmarked, mapped.set_fields),
            (3, false, Some(true), 7)
        );
        assert_eq!(mapped.body.plain_text(), "AB");
        assert_eq!(owner.body.plain_text(), "AB");
    }
    #[test]
    fn first_error_preserves_source_and_stops_later_callbacks() {
        let source = Content::heading(
            2,
            Content::sequence(vec![Content::text("A"), Content::text("B")]),
        );
        let mut visits = Vec::new();
        let out = source.map_content(&mut |node| {
            if let Content::Text(t) = node {
                visits.push(t.to_string());
                return Err(Vec::new());
            }
            Ok(None)
        });
        assert!(out.is_err());
        assert_eq!(visits, vec!["A"]);
        assert_eq!(source.plain_text(), "AB");
    }
}
