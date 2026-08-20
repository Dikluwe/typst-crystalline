//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/math_attach.md
//! @prompt-hash 02f69149
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
pub struct MathAttachElem {
    pub base: Content,
    pub t: Option<Content>,
    pub b: Option<Content>,
    pub tl: Option<Content>,
    pub bl: Option<Content>,
    pub tr: Option<Content>,
    pub br: Option<Content>,
}

impl MathAttachElem {
    /// Retrocompatibilidade: script superior direito (tr ou t)
    pub fn sup(&self) -> Option<&Content> {
        self.tr.as_ref().or(self.t.as_ref())
    }
    /// Retrocompatibilidade: script inferior direito (br ou b)
    pub fn sub(&self) -> Option<&Content> {
        self.br.as_ref().or(self.b.as_ref())
    }
}

impl Element for MathAttachElem {
    fn plain_text(&self) -> String {
        let mut s = String::new();
        if let Some(tl) = &self.tl {
            s.push_str(&format!("^{}", tl.plain_text()));
        }
        if let Some(bl) = &self.bl {
            s.push_str(&format!("_{}", bl.plain_text()));
        }
        s.push_str(&self.base.plain_text());
        if let Some(t) = &self.t {
            s.push_str(&format!("^{}", t.plain_text()));
        }
        if let Some(b) = &self.b {
            s.push_str(&format!("_{}", b.plain_text()));
        }
        if let Some(tr) = &self.tr {
            s.push_str(&format!("^{}", tr.plain_text()));
        }
        if let Some(br) = &self.br {
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
            t: self.t.as_ref().map(|c| c.map_content(transform)).transpose()?,
            b: self.b.as_ref().map(|c| c.map_content(transform)).transpose()?,
            tl: self.tl.as_ref().map(|c| c.map_content(transform)).transpose()?,
            bl: self.bl.as_ref().map(|c| c.map_content(transform)).transpose()?,
            tr: self.tr.as_ref().map(|c| c.map_content(transform)).transpose()?,
            br: self.br.as_ref().map(|c| c.map_content(transform)).transpose()?,
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

    fn ex() -> MathAttachElem {
        MathAttachElem {
            base: Content::text("x"),
            t: None,
            b: None,
            tl: None,
            bl: None,
            tr: Some(Content::text("2")),
            br: Some(Content::text("1")),
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
        other.tr = None;
        assert_ne!(ex(), other);
    }
}
