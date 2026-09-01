//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/math_vec.md
//! @prompt-hash 72f2ba66
//! @layer L1
//! @updated 2026-09-01
//!
//! `MathVecElem` — vetor matemático com identidade e named preservados.

use std::hash::{Hash, Hasher};
use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::layout_types::{HAlign, Length};
use crate::entities::rel::Rel;
use crate::entities::source_result::SourceResult;

/// Presença morfológica dos argumentos nomeados de `math.vec`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct MathVecExplicit {
    pub delim: bool,
    pub align: bool,
    pub gap: bool,
}

/// Vetor matemático: cada filho corresponde a uma linha de uma célula.
#[derive(Debug, Clone, PartialEq)]
pub struct MathVecElem {
    pub children: Vec<Content>,
    pub delim: (char, char),
    pub align: HAlign,
    pub gap: Rel<Length>,
    pub explicit: MathVecExplicit,
}

impl Hash for MathVecElem {
    fn hash<H: Hasher>(&self, state: &mut H) {
        fn bits(value: f64) -> u64 {
            if value == 0.0 {
                0
            } else {
                value.to_bits()
            }
        }
        self.children.hash(state);
        self.delim.hash(state);
        self.align.hash(state);
        bits(self.gap.rel).hash(state);
        bits(self.gap.abs.abs.0).hash(state);
        bits(self.gap.abs.em).hash(state);
        self.explicit.hash(state);
    }
}

impl Element for MathVecElem {
    fn plain_text(&self) -> String {
        self.children.iter().map(Content::plain_text).collect()
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        let children = self
            .children
            .iter()
            .map(|child| child.map_content(transform))
            .collect::<SourceResult<Vec<_>>>()?;
        Ok(Content::MathVec(Arc::new(Self {
            children,
            delim: self.delim,
            align: self.align,
            gap: self.gap,
            explicit: self.explicit,
        })))
    }

    fn map_text<F>(&self, _transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        Content::MathVec(Arc::new(self.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1292_c_math_vec_preserva_identidade_vazia_e_presenca() {
        let omitted = Content::math_vec(vec![]);
        let explicit = Content::math_vec_full(
            vec![],
            ('(', ')'),
            HAlign::Center,
            Rel { rel: 0.0, abs: Length::em(0.2) },
            MathVecExplicit { delim: true, align: true, gap: true },
        );
        assert!(matches!(omitted, Content::MathVec(_)));
        assert_ne!(omitted, explicit, "named default explícito participa da identidade");
        assert_eq!(omitted.plain_text(), "");
    }
}
