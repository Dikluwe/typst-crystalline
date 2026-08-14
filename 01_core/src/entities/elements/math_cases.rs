//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/math_cases.md
//! @prompt-hash b3d081a0
//! @layer L1
//! @updated 2026-06-11
//!
//! `MathCasesElem` — Lote 2 P317 (família math). Função por ramos `cases(...)`.
//! Comportamento idêntico ao braço anterior do hub (content-preserving).

use std::sync::Arc;
use std::hash::{Hash, Hasher};

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::layout_types::Length;
use crate::entities::source_result::SourceResult;

/// Função definida por ramos (`cases(...)`). `rows`: lista de ramos; cada ramo
/// é um array de células (separadas por `&`). `delim`: par de delimitadores (por
/// defeito `('{', '}')`). `reverse`: se o delimitador fica à direita. `gap`: espaçamento vertical.
#[derive(Debug, Clone, PartialEq)]
pub struct MathCasesElem {
    pub rows: Vec<Vec<Content>>,
    pub delim: (char, char),
    pub reverse: bool,
    pub gap: Option<Length>,
}

impl Hash for MathCasesElem {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.rows.hash(state);
        self.delim.hash(state);
        self.reverse.hash(state);
        if let Some(gap) = &self.gap {
            gap.abs.0.to_bits().hash(state);
            gap.em.to_bits().hash(state);
        }
    }
}

impl Element for MathCasesElem {
    fn plain_text(&self) -> String {
        self.rows
            .iter()
            .map(|row| row.iter().map(|c| c.plain_text()).collect::<Vec<_>>().join(" & "))
            .collect::<Vec<_>>()
            .join(", ")
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        let rows: SourceResult<Vec<Vec<Content>>> = self
            .rows
            .iter()
            .map(|row| row.iter().map(|c| c.map_content(transform)).collect())
            .collect();
        Ok(Content::MathCases(Arc::new(MathCasesElem {
            rows: rows?,
            delim: self.delim,
            reverse: self.reverse,
            gap: self.gap,
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
            rows: vec![
                vec![Content::text("a"), Content::text("b")],
                vec![Content::text("c")],
            ],
            delim: ('{', '}'),
            reverse: false,
            gap: None,
        }
    }

    #[test]
    fn plain_text_celulas_e_ramos() {
        assert_eq!(ex().plain_text(), "a & b, c");
    }

    #[test]
    fn igualdade_estrutural() {
        assert_eq!(ex(), ex());
        let mut other = ex();
        other.rows.pop();
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
            Content::MathCases(e) => assert_eq!(e.plain_text(), "Z & b, c"),
            _ => panic!("esperado MathCases"),
        }
    }
}
