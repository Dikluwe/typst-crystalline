//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/list_item.md
//! @prompt-hash 12d94a02
//! @layer L1
//! @updated 2026-06-29
//!
//! `ListItemElem` — Lote 3 P318 (família lista/termos). Item de lista (`- …`).
//! Campo `marker` adicionado em P470. Contentor de prosa: recurse em
//! map_content E map_text (precedente Heading).

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::layout_types::{Align2D, Length};
use crate::entities::list_marker::ListMarker;
use crate::entities::source_result::SourceResult;

/// Item de lista não ordenada (`- ...`).
#[derive(Debug, Clone, PartialEq)]
pub struct ListItemElem {
    pub body: Content,
    pub marker: Option<ListMarker>, // None = bullet padrão "•"
    /// **P504** — alinhamento do marker (Typst 0.15.0). Layout real é
    /// scope-out; o campo é aceite e preservado sem efeito visual.
    pub marker_align: Option<Align2D>,
    /// **P505** — indentação do marker em relação à margem esquerda.
    pub indent: Option<Length>,
    /// **P505** — indentação do corpo do item em relação ao marker.
    pub body_indent: Option<Length>,
    /// **P505** — `false` adiciona espaçamento de parágrafo entre itens.
    pub tight: Option<bool>,
}

// `Hash` manual via `Debug` (paridade `content_hash::hash_content`).
// CAUSA: `Length` carrega `f64` e não implementa `Hash`; o trait `Element`
// exige `Hash`, logo `#[derive(Hash)]` não compila.
// RESSALVA: hash-via-Debug pode violar Hash/Eq para `-0.0` vs `0.0`
// (PartialEq-iguais, Debug distinto). Aceitável: regime do `content_hash`
// pré-existente e os `…Elem` NÃO são chaves de mapa. Revisitar se vier a sê-lo.
impl std::hash::Hash for ListItemElem {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        format!("{self:?}").hash(state);
    }
}

impl Element for ListItemElem {
    fn plain_text(&self) -> String {
        let m = self.marker.as_ref().map(|m| m.render()).unwrap_or("•");
        format!("{} {}", m, self.body.plain_text())
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::ListItem(Arc::new(ListItemElem {
            body: self.body.map_content(transform)?,
            marker: self.marker.clone(),
            marker_align: self.marker_align,
            indent: self.indent.clone(),
            body_indent: self.body_indent.clone(),
            tight: self.tight,
        })))
    }

    fn map_text<F>(&self, transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::ListItem(Arc::new(ListItemElem {
            body: self.body.map_text(transform),
            marker: self.marker.clone(),
            marker_align: self.marker_align,
            indent: self.indent.clone(),
            body_indent: self.body_indent.clone(),
            tight: self.tight,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_text_com_bullet_default() {
        let li = ListItemElem {
            body: Content::text("um"),
            marker: None,
            marker_align: None,
            indent: None,
            body_indent: None,
            tight: None,
        };
        assert_eq!(li.plain_text(), "• um");
    }

    #[test]
    fn plain_text_com_marcador_custom() {
        let li = ListItemElem {
            body: Content::text("um"),
            marker: Some(ListMarker::Custom("→".into())),
            marker_align: None,
            indent: None,
            body_indent: None,
            tight: None,
        };
        assert_eq!(li.plain_text(), "→ um");
    }

    #[test]
    fn igualdade_estrutural() {
        let a = ListItemElem {
            body: Content::text("x"),
            marker: None,
            marker_align: None,
            indent: None,
            body_indent: None,
            tight: None,
        };
        assert_eq!(a.clone(), a.clone());
        assert_ne!(
            a,
            ListItemElem {
                body: Content::text("y"),
                marker: None,
                marker_align: None,
                indent: None,
                body_indent: None,
                tight: None
            }
        );
        assert_ne!(
            a,
            ListItemElem {
                body: Content::text("x"),
                marker: Some(ListMarker::Custom("→".into())),
                marker_align: None,
                indent: None,
                body_indent: None,
                tight: None,
            }
        );
    }

    #[test]
    fn map_text_recurse_body_preserva_marker_e_indentacao() {
        let li = ListItemElem {
            body: Content::text("ab"),
            marker: Some(ListMarker::Custom("*".into())),
            marker_align: None,
            indent: Some(Length::em(1.5)),
            body_indent: Some(Length::em(0.5)),
            tight: Some(false),
        };
        let mut up = |s: &str| s.to_uppercase();
        match li.map_text(&mut up) {
            Content::ListItem(e) => {
                assert_eq!(e.plain_text(), "* AB");
                assert_eq!(e.marker, Some(ListMarker::Custom("*".into())));
                assert_eq!(e.indent, Some(Length::em(1.5)));
                assert_eq!(e.body_indent, Some(Length::em(0.5)));
                assert_eq!(e.tight, Some(false));
            }
            _ => panic!("esperado ListItem"),
        }
    }

    #[test]
    fn map_content_preserva_marker_e_indentacao() {
        let li = ListItemElem {
            body: Content::text("x"),
            marker: Some(ListMarker::Default),
            marker_align: None,
            indent: Some(Length::em(1.0)),
            body_indent: Some(Length::em(0.5)),
            tight: Some(true),
        };
        let mut f = |_c: &Content| -> SourceResult<Option<Content>> { Ok(None) };
        match li.map_content(&mut f).unwrap() {
            Content::ListItem(e) => {
                assert_eq!(e.marker, Some(ListMarker::Default));
                assert_eq!(e.indent, Some(Length::em(1.0)));
                assert_eq!(e.body_indent, Some(Length::em(0.5)));
                assert_eq!(e.tight, Some(true));
            }
            _ => panic!("esperado ListItem"),
        }
    }
}
