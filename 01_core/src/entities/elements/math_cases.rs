//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/math_cases.md
//! @prompt-hash 37c365d9
//! @layer L1
//! @updated 2026-06-11
//!
//! `MathCasesElem` — Lote 2 P317 (família math). Função por ramos `cases(...)`.
//! Comportamento idêntico ao braço anterior do hub (content-preserving).

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::source_result::SourceResult;

/// Função definida por ramos (`cases(...)`). `rows`: lista de ramos; cada ramo
/// é um array de células (separadas por `&`). Delimitador esquerdo `{`; sem
/// delimitador direito (no layout, inalterado).
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MathCasesElem {
    pub rows: Vec<Vec<Content>>,
}

impl Element for MathCasesElem {
    fn plain_text(&self) -> String {
        self.rows.iter().map(|row| {
            row.iter().map(|c| c.plain_text()).collect::<Vec<_>>().join(" & ")
        }).collect::<Vec<_>>().join(", ")
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        let rows: SourceResult<Vec<Vec<Content>>> = self.rows.iter()
            .map(|row| row.iter().map(|c| c.map_content(transform)).collect())
            .collect();
        Ok(Content::MathCases(Arc::new(MathCasesElem { rows: rows? })))
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
