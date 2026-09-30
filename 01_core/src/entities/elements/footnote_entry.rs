//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/footnote_entry.md
//! @prompt-hash 796b49b8
//! @layer L1
//! @updated 2026-09-21
//!
//! `FootnoteEntryElem` — **P1473**, escopo reduzido (construção sem
//! realização). `footnote.entry(note, separator?, clearance?, gap?,
//! indent?)` — dá forma pública ao constructor; não participa do layout de
//! notas de rodapé nesta fatia (`compiler/layout/footnote.rs` não muda). Ver
//! `compiler/stdlib/structural/footnote.md` §P1473 para a medição completa.

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::layout_types::Length;
use crate::entities::source_result::SourceResult;

/// `footnote.entry(note, separator?, clearance?, gap?, indent?)` —
/// construção sem realização.
///
/// `note` não é validado além de `Content` (deveria ser um `FootnoteElem`
/// real com `Location`, mas essa exigência pertence ao pipeline de
/// posicionamento, fora desta fatia). Os quatro últimos campos são aceites e
/// preservados sem efeito de layout — mesmo padrão já vigente em
/// `ListItemElem` para `marker_align`/`indent`/`tight`.
#[derive(Debug, Clone, PartialEq)]
pub struct FootnoteEntryElem {
    pub note: Content,
    /// Vanilla default: uma linha (`LineElem`) de 30% de largura, traço
    /// 0.05em. Layout real é scope-out; campo aceite e preservado sem efeito
    /// visual. `None` = default do vanilla (não replicado aqui).
    pub separator: Option<Content>,
    /// Vanilla default `1em`. Layout real é scope-out.
    pub clearance: Option<Length>,
    /// Vanilla default `0.5em`. Layout real é scope-out.
    pub gap: Option<Length>,
    /// Vanilla default `1em`. Layout real é scope-out.
    pub indent: Option<Length>,
}

// `Hash` manual via `Debug` — `Length` carrega `f64`, que não implementa
// `Hash` (mesma causa já registada em `entities/elements/list_item.rs` para
// `indent`/`body_indent`/`tight`). Mesmo padrão `Hash`-via-`Debug`.
impl std::hash::Hash for FootnoteEntryElem {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        format!("{self:?}").hash(state);
    }
}

impl Element for FootnoteEntryElem {
    fn plain_text(&self) -> String {
        self.note.plain_text()
    }

    fn is_empty(&self) -> bool {
        self.note.is_empty()
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::FootnoteEntry(Arc::new(FootnoteEntryElem {
            note: self.note.map_content(transform)?,
            separator: self
                .separator
                .as_ref()
                .map(|s| s.map_content(transform))
                .transpose()?,
            clearance: self.clearance,
            gap: self.gap,
            indent: self.indent,
        })))
    }

    fn map_text<F>(&self, transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::FootnoteEntry(Arc::new(FootnoteEntryElem {
            note: self.note.map_text(transform),
            separator: self.separator.as_ref().map(|s| s.map_text(transform)),
            clearance: self.clearance,
            gap: self.gap,
            indent: self.indent,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn minimal(note: Content) -> FootnoteEntryElem {
        FootnoteEntryElem {
            note,
            separator: None,
            clearance: None,
            gap: None,
            indent: None,
        }
    }

    #[test]
    fn plain_text_e_is_empty_delegam_a_note() {
        let fe = minimal(Content::text("nota"));
        assert_eq!(fe.plain_text(), "nota");
        assert!(!fe.is_empty());

        let vazia = minimal(Content::Empty);
        assert!(vazia.is_empty());
    }

    #[test]
    fn quatro_opcionais_sao_preservados_por_valor() {
        let fe = FootnoteEntryElem {
            note: Content::text("x"),
            separator: Some(Content::text("--")),
            clearance: Some(Length::em(1.0)),
            gap: Some(Length::em(0.5)),
            indent: Some(Length::em(1.0)),
        };
        assert_eq!(fe.separator, Some(Content::text("--")));
        assert_eq!(fe.clearance, Some(Length::em(1.0)));
        assert_eq!(fe.gap, Some(Length::em(0.5)));
        assert_eq!(fe.indent, Some(Length::em(1.0)));
    }

    #[test]
    fn map_text_transforma_note_e_separator_preserva_lengths() {
        let fe = FootnoteEntryElem {
            note: Content::text("ab"),
            separator: Some(Content::text("cd")),
            clearance: Some(Length::em(2.0)),
            gap: None,
            indent: None,
        };
        let mut up = |s: &str| s.to_uppercase();
        match fe.map_text(&mut up) {
            Content::FootnoteEntry(e) => {
                assert_eq!(e.plain_text(), "AB");
                assert_eq!(e.separator, Some(Content::text("CD")));
                assert_eq!(e.clearance, Some(Length::em(2.0)));
            }
            other => panic!("esperado FootnoteEntry, obteve {other:?}"),
        }
    }

    #[test]
    fn map_content_recursa_em_note_e_separator_preserva_lengths() {
        let fe = FootnoteEntryElem {
            note: Content::text("n"),
            separator: Some(Content::text("s")),
            clearance: Some(Length::em(1.0)),
            gap: Some(Length::em(0.5)),
            indent: Some(Length::em(1.0)),
        };
        let mut f = |_c: &Content| -> SourceResult<Option<Content>> { Ok(None) };
        match fe.map_content(&mut f).unwrap() {
            Content::FootnoteEntry(e) => {
                assert_eq!(e.note, Content::text("n"));
                assert_eq!(e.separator, Some(Content::text("s")));
                assert_eq!(e.clearance, Some(Length::em(1.0)));
                assert_eq!(e.gap, Some(Length::em(0.5)));
                assert_eq!(e.indent, Some(Length::em(1.0)));
            }
            other => panic!("esperado FootnoteEntry, obteve {other:?}"),
        }
    }

    #[test]
    fn eq_e_hash_discriminam_note() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let a = minimal(Content::text("x"));
        let b = minimal(Content::text("y"));
        assert_ne!(a, b);
        let hash_of = |v: &FootnoteEntryElem| {
            let mut h = DefaultHasher::new();
            v.hash(&mut h);
            h.finish()
        };
        assert_ne!(hash_of(&a), hash_of(&b));
    }
}
