//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/math_accent.md
//! @prompt-hash 661ea0cb
//! @layer L1
//! @updated 2026-10-10
//!
//! `MathAccentElem` — Lote 2 P317 (família math). Acento matemático (P296).
//! Comportamento idêntico ao braço anterior do hub (content-preserving).

use std::hash::Hash;
use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::layout_types::Length;
use crate::entities::rel::Rel;
use crate::entities::source_result::SourceResult;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct MathAccentExplicit {
    pub size: bool,
    pub dotless: bool,
}

/// Acento matemático — vanilla `AccentElem` minimal (P296). Layouter posiciona
/// `accent` centrado horizontalmente acima da `base`.
#[derive(Debug, Clone)]
pub struct MathAccentElem {
    pub base: Content,
    pub accent: char,
    pub size: Rel<Length>,
    pub dotless: bool,
    pub explicit: MathAccentExplicit,
}

impl PartialEq for MathAccentElem {
    fn eq(&self, other: &Self) -> bool {
        self.base == other.base
            && self.accent == other.accent
            && self.size == other.size
            && self.dotless == other.dotless
            && self.explicit == other.explicit
    }
}

impl std::hash::Hash for MathAccentElem {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        fn bits(value: f64) -> u64 {
            if value == 0.0 {
                0
            } else {
                value.to_bits()
            }
        }
        self.base.hash(state);
        self.accent.hash(state);
        bits(self.size.rel).hash(state);
        bits(self.size.abs.abs.0).hash(state);
        bits(self.size.abs.em).hash(state);
        self.dotless.hash(state);
        self.explicit.hash(state);
    }
}

impl Element for MathAccentElem {
    fn plain_text(&self) -> String {
        format!("{}{}", self.base.plain_text(), self.accent)
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(self
            .map_content_if_changed(transform)?
            .unwrap_or_else(|| Content::MathAccent(Arc::new(self.clone()))))
    }

    fn map_text<F>(&self, _transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        // Terminal (math structural; não desce).
        Content::MathAccent(Arc::new(self.clone()))
    }
}

impl MathAccentElem {
    pub(crate) fn map_content_if_changed<F>(
        &self,
        transform: &mut F,
    ) -> SourceResult<Option<Content>>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        let base = self.base.map_content_if_changed(transform)?;
        if base.is_none() {
            return Ok(None);
        }
        let mut mapped = self.clone();
        if let Some(changed) = base {
            mapped.base = changed;
        }
        Ok(Some(Content::MathAccent(Arc::new(mapped))))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ex() -> MathAccentElem {
        MathAccentElem {
            base: Content::text("x"),
            accent: '^',
            size: Rel::from_percent(100.0),
            dotless: true,
            explicit: MathAccentExplicit::default(),
        }
    }

    #[test]
    fn plain_text_concat() {
        assert_eq!(ex().plain_text(), "x^");
    }

    #[test]
    fn igualdade_estrutural() {
        assert_eq!(ex(), ex());
        let mut other = ex();
        other.accent = '~';
        assert_ne!(ex(), other);
    }

    #[test]
    fn map_content_recurse_ambos() {
        let mut f = |c: &Content| -> SourceResult<Option<Content>> {
            match c {
                Content::Text(s) if s.as_str() == "x" => Ok(Some(Content::text("Z"))),
                _ => Ok(None),
            }
        };
        let r = ex().map_content(&mut f).unwrap();
        match r {
            Content::MathAccent(e) => assert_eq!(e.plain_text(), "Z^"),
            _ => panic!("esperado MathAccent"),
        }
    }
}
