//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/math_matrix.md
//! @prompt-hash b8d47bbe
//! @layer L1
//! @updated 2026-06-11
//!
//! `MathMatrixElem` — Lote 2 P317 (família math). Matriz `mat(...)`.
//! Comportamento idêntico ao braço anterior do hub (content-preserving).

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::source_result::SourceResult;

/// Matriz matemática (`mat(...)`). `rows`: lista de linhas, cada linha é uma
/// lista de células. `delim`: par de delimitadores (`('(', ')')` por defeito).
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MathMatrixElem {
    pub rows:  Vec<Vec<Content>>,
    pub delim: (char, char),
}

impl Element for MathMatrixElem {
    fn plain_text(&self) -> String {
        self.rows.iter().map(|row| {
            row.iter().map(|c| c.plain_text()).collect::<Vec<_>>().join(", ")
        }).collect::<Vec<_>>().join("; ")
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        let rows: SourceResult<Vec<Vec<Content>>> = self.rows.iter()
            .map(|row| row.iter().map(|c| c.map_content(transform)).collect())
            .collect();
        Ok(Content::MathMatrix(Arc::new(MathMatrixElem { rows: rows?, delim: self.delim })))
    }

    fn map_text<F>(&self, _transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        // Terminal (math structural; não desce).
        Content::MathMatrix(Arc::new(self.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ex() -> MathMatrixElem {
        MathMatrixElem {
            rows: vec![
                vec![Content::text("a"), Content::text("b")],
                vec![Content::text("c"), Content::text("d")],
            ],
            delim: ('(', ')'),
        }
    }

    #[test]
    fn plain_text_celulas_e_linhas() {
        assert_eq!(ex().plain_text(), "a, b; c, d");
    }

    #[test]
    fn igualdade_preserva_delim() {
        assert_eq!(ex(), ex());
        let mut other = ex();
        other.delim = ('[', ']');
        assert_ne!(ex(), other);
    }

    #[test]
    fn map_content_recurse_e_preserva_delim() {
        let mut f = |c: &Content| -> SourceResult<Option<Content>> {
            match c {
                Content::Text(s, _) if s.as_str() == "d" => Ok(Some(Content::text("Z"))),
                _ => Ok(None),
            }
        };
        let r = ex().map_content(&mut f).unwrap();
        match r {
            Content::MathMatrix(e) => {
                assert_eq!(e.plain_text(), "a, b; c, Z");
                assert_eq!(e.delim, ('(', ')'));
            }
            _ => panic!("esperado MathMatrix"),
        }
    }
}
