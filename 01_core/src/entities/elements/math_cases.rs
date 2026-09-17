//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/math_cases.md
//! @prompt-hash a468a807
//! @layer L1
//! @updated 2026-06-11
//!
//! `MathCasesElem` — Lote 2 P317 (família math). Função por ramos `cases(...)`.
//! Comportamento idêntico ao braço anterior do hub (content-preserving).

use std::hash::{Hash, Hasher};
use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::layout_types::Length;
use crate::entities::rel::Rel;
use crate::entities::source_result::SourceResult;

/// Presença morfológica dos argumentos nomeados de `math.cases`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct MathCasesExplicit {
    pub delim: bool,
    pub reverse: bool,
    pub gap: bool,
}

/// Função definida por ramos (`cases(...)`). Cada filho é um ramo e conserva
/// os seus pontos de alinhamento até ao consumer de layout.
#[derive(Debug, Clone, PartialEq)]
pub struct MathCasesElem {
    pub children: Vec<Content>,
    pub delim: (Option<char>, Option<char>),
    pub reverse: bool,
    pub gap: Rel<Length>,
    pub explicit: MathCasesExplicit,
}

impl Hash for MathCasesElem {
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
        self.reverse.hash(state);
        bits(self.gap.rel).hash(state);
        bits(self.gap.abs.abs.0).hash(state);
        bits(self.gap.abs.em).hash(state);
        self.explicit.hash(state);
    }
}

impl Element for MathCasesElem {
    fn plain_text(&self) -> String {
        self.children
            .iter()
            .map(Content::plain_text)
            .collect::<Vec<_>>()
            .join(", ")
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
        Ok(Content::MathCases(Arc::new(MathCasesElem {
            children,
            delim: self.delim,
            reverse: self.reverse,
            gap: self.gap,
            explicit: self.explicit,
        })))
    }

    fn map_text<F>(&self, _transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        // Terminal (math structural; não desce).
        Content::MathCases(Arc::new(self.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ex() -> MathCasesElem {
        MathCasesElem {
            children: vec![Content::text("ab"), Content::text("c")],
            delim: (Some('{'), Some('}')),
            reverse: false,
            gap: Rel { rel: 0.0, abs: Length::em(0.2) },
            explicit: MathCasesExplicit::default(),
        }
    }

    #[test]
    fn plain_text_celulas_e_ramos() {
        assert_eq!(ex().plain_text(), "ab, c");
    }

    #[test]
    fn igualdade_estrutural() {
        assert_eq!(ex(), ex());
        let mut other = ex();
        other.children.pop();
        assert_ne!(ex(), other);
    }

    #[test]
    fn map_content_recurse_grelha() {
        let mut f = |c: &Content| -> SourceResult<Option<Content>> {
            match c {
                Content::Text(s) if s.as_str() == "a" => Ok(Some(Content::text("Z"))),
                _ => Ok(None),
            }
        };
        let r = ex().map_content(&mut f).unwrap();
        match r {
            Content::MathCases(e) => assert_eq!(e.plain_text(), "ab, c"),
            _ => panic!("esperado MathCases"),
        }
    }
}
