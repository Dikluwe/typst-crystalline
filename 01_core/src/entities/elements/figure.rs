//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/figure.md
//! @prompt-hash ae099801
//! @layer L1
//! @updated 2026-06-11
//!
//! `FigureElem` — Lote 13 P328 (o último element-shaped). `figure(body, …)`.
//! **Locatável** (M1) — absorve `extract_payload`. Contentor: recurse em
//! `body` E `caption` (simétrico em map_content/map_text).

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::counter_update::CounterUpdate;
use crate::entities::element_kind::ElementKind;
use crate::entities::element_payload::ElementPayload;
use crate::entities::elements::Element;
use crate::entities::source_result::SourceResult;

/// Figura com `body`, `caption` opcional, `kind`/`numbering` opcionais.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct FigureElem {
    pub body: Content,
    pub caption: Option<Content>,
    pub kind: Option<String>,
    // F-5a de-bake (P365, `f_fronteira_e1.md` §3a.9): o campo assado `numbering`
    // foi **removido** — o padrão de numeração vive **só na chain**
    // (`#set figure(numbering:)` → `custom("figure.numbering")`, transportado por
    // `Content::Styled`). O consumidor (layout/introspect) lê o gate da chain; o
    // **número** (`figure_number_at_index`) segue via Introspector.
}

impl Element for FigureElem {
    fn plain_text(&self) -> String {
        // Paridade content.rs:1706: body + caption com espaço; vazios omitidos.
        let body_text = self.body.plain_text();
        let cap_text = self.caption.as_ref().map(|c| c.plain_text()).unwrap_or_default();
        match (body_text.is_empty(), cap_text.is_empty()) {
            (false, false) => format!("{} {}", body_text, cap_text),
            (false, true) => body_text,
            (true, false) => cap_text,
            (true, true) => String::new(),
        }
    }

    fn is_empty(&self) -> bool {
        self.body.is_empty() && self.caption.as_ref().is_none_or(|c| c.is_empty())
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::Figure(Arc::new(FigureElem {
            body: self.body.map_content(transform)?,
            caption: self
                .caption
                .as_ref()
                .map(|c| c.map_content(transform))
                .transpose()?,
            kind: self.kind.clone(),
        })))
    }

    fn map_text<F>(&self, transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::Figure(Arc::new(FigureElem {
            body: self.body.map_text(transform),
            caption: self.caption.as_ref().map(|c| c.map_text(transform)),
            kind: self.kind.clone(),
        }))
    }

    fn element_kind(&self) -> Option<ElementKind> {
        Some(ElementKind::Figure)
    }

    fn to_payload(&self) -> Option<ElementPayload> {
        // F-5a de-bake (P365): `is_counted` é a conjunção do **gate** (padrão, vindo
        // da chain) com a presença de caption. O elemento não baka mais o padrão;
        // aqui dá-se a parte que conhece (caption), e o walk top (`introspect.rs`)
        // faz `is_counted &= chain.custom("figure.numbering").is_str()` na emissão.
        Some(ElementPayload::Figure {
            kind: self.kind.clone(),
            counter_update: CounterUpdate::Step,
            is_counted: self.caption.is_some(),
            caption_text: self.caption.as_ref().map(|c| c.plain_text()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    fn ex() -> FigureElem {
        FigureElem {
            body: Content::text("img"),
            caption: Some(Content::text("Fig 1")),
            kind: Some("image".to_string()),
        }
    }

    #[test]
    fn plain_text_body_e_caption() {
        assert_eq!(ex().plain_text(), "img Fig 1");
    }

    #[test]
    fn plain_text_sem_caption() {
        let f = FigureElem {
            body: Content::text("img"),
            caption: None,
            kind: None,
        };
        assert_eq!(f.plain_text(), "img");
    }

    #[test]
    fn is_empty_body_e_caption_vazios() {
        assert!(!ex().is_empty());
        let vazia = FigureElem { body: Content::Empty, caption: None, kind: None };
        assert!(vazia.is_empty());
    }

    #[test]
    fn map_content_recurse_body_e_caption_preserva_kind() {
        let mut f = |c: &Content| -> SourceResult<Option<Content>> {
            match c {
                Content::Text(s) if s.as_str() == "img" => Ok(Some(Content::text("Z"))),
                _ => Ok(None),
            }
        };
        match ex().map_content(&mut f).unwrap() {
            Content::Figure(e) => {
                assert_eq!(e.kind, Some("image".to_string()));
                assert!(matches!(&e.body, Content::Text(s) if s.as_str() == "Z"));
            }
            _ => panic!("esperado Figure"),
        }
    }

    #[test]
    fn locatavel_kind_e_payload() {
        assert_eq!(ex().element_kind(), Some(ElementKind::Figure));
        match ex().to_payload() {
            Some(ElementPayload::Figure { kind, counter_update, is_counted, .. }) => {
                assert_eq!(kind, Some("image".to_string()));
                assert_eq!(counter_update, CounterUpdate::Step);
                // F-5a de-bake (P365): `is_counted` no payload é o placeholder
                // = caption.is_some() (ex tem caption); o gate do padrão é ANDado
                // pela chain no walk top (testado no nível introspect).
                assert!(is_counted);
            }
            other => panic!("esperado Figure payload, obtido {other:?}"),
        }
    }

    #[test]
    fn is_counted_placeholder_falso_sem_caption() {
        // F-5a de-bake (P365): sem o campo `numbering`, o placeholder de
        // `is_counted` reflete só a caption; sem caption → false.
        let f = FigureElem {
            body: Content::text("x"),
            caption: None,
            kind: None,
        };
        match f.to_payload() {
            Some(ElementPayload::Figure { is_counted, .. }) => assert!(!is_counted),
            _ => panic!("esperado Figure payload"),
        }
    }

    fn h(e: &FigureElem) -> u64 {
        let mut s = DefaultHasher::new();
        e.hash(&mut s);
        s.finish()
    }

    #[test]
    fn payload_diferente_produz_hash_diferente() {
        let mut v = ex();
        v.kind = Some("table".to_string());
        assert_ne!(h(&ex()), h(&v));
    }
}
