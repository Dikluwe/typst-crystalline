//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/figure_caption.md
//! @prompt-hash 934dfb89
//! @layer L1
//! @updated 2026-09-21
//!
//! `FigureCaptionElem` — **P1473**, escopo reduzido (construção sem
//! síntese). `figure.caption(body, position?, separator?)` — dá forma
//! pública ao constructor; não participa da síntese (`kind`/`supplement`/
//! `numbering`/`counter`) nem do layout de figuras nesta fatia. Ver
//! `compiler/stdlib/figure_image.md` §P1473 para a medição completa.

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::source_result::SourceResult;

/// Posição da legenda relativa à figura (vanilla `OuterVAlignment`, restrito
/// aos dois valores válidos desta rota — não reutiliza o `Align2D` genérico
/// de layout, que carrega mais variantes que as duas aqui válidas).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CaptionPosition {
    Top,
    Bottom,
}

/// `figure.caption(body, position?, separator?)` — construção sem síntese.
///
/// `position`/`separator` são settable (aceites e preservados), mas sem
/// default materializado aqui: `None` = não definido explicitamente. Sem
/// campo `Length`/`f64`; `#[derive(Hash)]` directo é seguro (ao contrário de
/// `FootnoteEntryElem`, que carrega `Length`).
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct FigureCaptionElem {
    /// Vanilla default: `bottom`. Settable; sem efeito de layout nesta fatia.
    pub position: Option<CaptionPosition>,
    /// Vanilla default: Auto (idioma/região). `None` aqui = não definido
    /// explicitamente; o default por idioma não é resolvido nesta fatia.
    pub separator: Option<Content>,
    pub body: Content,
}

impl Element for FigureCaptionElem {
    fn plain_text(&self) -> String {
        self.body.plain_text()
    }

    fn is_empty(&self) -> bool {
        self.body.is_empty()
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::FigureCaption(Arc::new(FigureCaptionElem {
            position: self.position,
            separator: self
                .separator
                .as_ref()
                .map(|s| s.map_content(transform))
                .transpose()?,
            body: self.body.map_content(transform)?,
        })))
    }

    fn map_text<F>(&self, transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::FigureCaption(Arc::new(FigureCaptionElem {
            position: self.position,
            separator: self.separator.as_ref().map(|s| s.map_text(transform)),
            body: self.body.map_text(transform),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_text_e_is_empty_delegam_ao_body() {
        let fc = FigureCaptionElem {
            position: None,
            separator: None,
            body: Content::text("legenda"),
        };
        assert_eq!(fc.plain_text(), "legenda");
        assert!(!fc.is_empty());

        let vazia = FigureCaptionElem {
            position: None,
            separator: None,
            body: Content::Empty,
        };
        assert!(vazia.is_empty());
    }

    #[test]
    fn position_e_separator_sao_preservados_por_valor() {
        let fc = FigureCaptionElem {
            position: Some(CaptionPosition::Top),
            separator: Some(Content::text("--")),
            body: Content::text("x"),
        };
        assert_eq!(fc.position, Some(CaptionPosition::Top));
        assert_eq!(fc.separator, Some(Content::text("--")));
    }

    #[test]
    fn map_text_transforma_body_e_separator_preserva_position() {
        let fc = FigureCaptionElem {
            position: Some(CaptionPosition::Bottom),
            separator: Some(Content::text("ab")),
            body: Content::text("cd"),
        };
        let mut up = |s: &str| s.to_uppercase();
        match fc.map_text(&mut up) {
            Content::FigureCaption(e) => {
                assert_eq!(e.position, Some(CaptionPosition::Bottom));
                assert_eq!(e.plain_text(), "CD");
                assert_eq!(e.separator, Some(Content::text("AB")));
            }
            other => panic!("esperado FigureCaption, obteve {other:?}"),
        }
    }

    #[test]
    fn map_content_preserva_position_e_recursa_em_body_e_separator() {
        let fc = FigureCaptionElem {
            position: Some(CaptionPosition::Top),
            separator: Some(Content::text("s")),
            body: Content::text("b"),
        };
        let mut f = |_c: &Content| -> SourceResult<Option<Content>> { Ok(None) };
        match fc.map_content(&mut f).unwrap() {
            Content::FigureCaption(e) => {
                assert_eq!(e.position, Some(CaptionPosition::Top));
                assert_eq!(e.body, Content::text("b"));
                assert_eq!(e.separator, Some(Content::text("s")));
            }
            other => panic!("esperado FigureCaption, obteve {other:?}"),
        }
    }

    #[test]
    fn eq_e_hash_discriminam_position() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let a = FigureCaptionElem {
            position: Some(CaptionPosition::Top),
            separator: None,
            body: Content::text("x"),
        };
        let b = FigureCaptionElem {
            position: Some(CaptionPosition::Bottom),
            separator: None,
            body: Content::text("x"),
        };
        assert_ne!(a, b);
        let hash_of = |v: &FigureCaptionElem| {
            let mut h = DefaultHasher::new();
            v.hash(&mut h);
            h.finish()
        };
        assert_ne!(hash_of(&a), hash_of(&b));
    }
}
