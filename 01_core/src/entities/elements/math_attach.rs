//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/math_attach.md
//! @prompt-hash b29224f0
//! @layer L1
//! @updated 2026-08-20
//!
//! `MathAttachElem` — Lote 2 P317 (família math). Base com scripts e limits (`attach`).

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::source_result::SourceResult;

/// Base com anexos de limites (t/b) e/ou scripts nos 4 cantos (tl/bl/tr/br).
#[derive(Debug, Clone, PartialEq, Hash)]
pub enum MathAttachSlot {
    Omitted,
    ExplicitNone,
    Present(Content),
}

impl MathAttachSlot {
    fn as_content(&self) -> Option<&Content> {
        match self {
            Self::Present(content) => Some(content),
            Self::Omitted | Self::ExplicitNone => None,
        }
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Self>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        match self {
            Self::Omitted => Ok(Self::Omitted),
            Self::ExplicitNone => Ok(Self::ExplicitNone),
            Self::Present(content) => content.map_content(transform).map(Self::Present),
        }
    }
}

/// Base com anexos de limites (t/b) e/ou scripts nos 4 cantos (tl/bl/tr/br).
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MathAttachElem {
    pub base: Content,
    pub t: MathAttachSlot,
    pub b: MathAttachSlot,
    pub tl: MathAttachSlot,
    pub bl: MathAttachSlot,
    pub tr: MathAttachSlot,
    pub br: MathAttachSlot,
}

impl MathAttachElem {
    /// Retrocompatibilidade: script superior direito (tr ou t)
    pub fn sup(&self) -> Option<&Content> {
        self.tr.as_content().or_else(|| self.t.as_content())
    }
    /// Retrocompatibilidade: script inferior direito (br ou b)
    pub fn sub(&self) -> Option<&Content> {
        self.br.as_content().or_else(|| self.b.as_content())
    }
}

impl Element for MathAttachElem {
    fn plain_text(&self) -> String {
        let mut s = String::new();
        if let Some(tl) = self.tl.as_content() {
            s.push_str(&format!("^{}", tl.plain_text()));
        }
        if let Some(bl) = self.bl.as_content() {
            s.push_str(&format!("_{}", bl.plain_text()));
        }
        s.push_str(&self.base.plain_text());
        if let Some(t) = self.t.as_content() {
            s.push_str(&format!("^{}", t.plain_text()));
        }
        if let Some(b) = self.b.as_content() {
            s.push_str(&format!("_{}", b.plain_text()));
        }
        if let Some(tr) = self.tr.as_content() {
            s.push_str(&format!("^{}", tr.plain_text()));
        }
        if let Some(br) = self.br.as_content() {
            s.push_str(&format!("_{}", br.plain_text()));
        }
        s
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::MathAttach(Arc::new(MathAttachElem {
            base: self.base.map_content(transform)?,
            t: self.t.map_content(transform)?,
            b: self.b.map_content(transform)?,
            tl: self.tl.map_content(transform)?,
            bl: self.bl.map_content(transform)?,
            tr: self.tr.map_content(transform)?,
            br: self.br.map_content(transform)?,
        })))
    }

    fn map_text<F>(&self, _transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        // Terminal (math structural; não desce).
        Content::MathAttach(Arc::new(self.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    fn ex() -> MathAttachElem {
        MathAttachElem {
            base: Content::text("x"),
            t: MathAttachSlot::Omitted,
            b: MathAttachSlot::Omitted,
            tl: MathAttachSlot::Omitted,
            bl: MathAttachSlot::Omitted,
            tr: MathAttachSlot::Present(Content::text("2")),
            br: MathAttachSlot::Present(Content::text("1")),
        }
    }

    #[test]
    fn plain_text_ordem() {
        assert_eq!(ex().plain_text(), "x^2_1");
    }

    #[test]
    fn igualdade_estrutural() {
        assert_eq!(ex(), ex());
        let mut other = ex();
        other.tr = MathAttachSlot::Omitted;
        assert_ne!(ex(), other);
    }

    #[test]
    fn p1293_carrier_distingue_omissao_none_e_markup_vazio_no_hash() {
        fn hash(slot: MathAttachSlot) -> u64 {
            let mut hasher = DefaultHasher::new();
            slot.hash(&mut hasher);
            hasher.finish()
        }

        let omitted = hash(MathAttachSlot::Omitted);
        let explicit_none = hash(MathAttachSlot::ExplicitNone);
        let empty_markup = hash(MathAttachSlot::Present(Content::Empty));
        assert_ne!(omitted, explicit_none);
        assert_ne!(omitted, empty_markup);
        assert_ne!(explicit_none, empty_markup);
    }

    #[test]
    fn p1293_sup_sub_projetam_apenas_present_com_precedencia() {
        let mut elem = ex();
        elem.t = MathAttachSlot::Present(Content::text("top"));
        elem.b = MathAttachSlot::Present(Content::text("bottom"));
        elem.tr = MathAttachSlot::ExplicitNone;
        elem.br = MathAttachSlot::Omitted;
        assert_eq!(elem.sup().map(Content::plain_text).as_deref(), Some("top"));
        assert_eq!(elem.sub().map(Content::plain_text).as_deref(), Some("bottom"));

        elem.tr = MathAttachSlot::Present(Content::text("right-top"));
        elem.br = MathAttachSlot::Present(Content::text("right-bottom"));
        assert_eq!(elem.sup().map(Content::plain_text).as_deref(), Some("right-top"));
        assert_eq!(elem.sub().map(Content::plain_text).as_deref(), Some("right-bottom"));
    }

    #[test]
    fn p1293_plain_text_e_traversal_preservam_carrier() {
        let elem = MathAttachElem {
            base: Content::text("x"),
            t: MathAttachSlot::ExplicitNone,
            b: MathAttachSlot::Present(Content::text("b")),
            tl: MathAttachSlot::Present(Content::text("tl")),
            bl: MathAttachSlot::Omitted,
            tr: MathAttachSlot::Present(Content::Empty),
            br: MathAttachSlot::Present(Content::text("br")),
        };
        assert_eq!(elem.plain_text(), "^tlx_b^_br");

        let mapped = elem
            .map_content(&mut |content| {
                Ok(matches!(content, Content::Text(text) if text == "b")
                    .then(|| Content::text("B")))
            })
            .unwrap();
        let Content::MathAttach(mapped) = mapped else {
            panic!("MathAttach traversal changed variant")
        };
        assert!(matches!(mapped.t, MathAttachSlot::ExplicitNone));
        assert!(matches!(mapped.bl, MathAttachSlot::Omitted));
        assert!(matches!(mapped.tr, MathAttachSlot::Present(Content::Empty)));
        assert!(
            matches!(mapped.b, MathAttachSlot::Present(Content::Text(ref text)) if text == "B")
        );
    }
}
