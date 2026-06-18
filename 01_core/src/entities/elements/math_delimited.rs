//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/math_delimited.md
//! @prompt-hash 3b840d8f
//! @layer L1
//! @updated 2026-06-11
//!
//! `MathDelimitedElem` — Lote 2 P317 (família math). Expressão delimitada.
//! Comportamento idêntico ao braço anterior do hub (content-preserving).

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::source_result::SourceResult;

/// Expressão entre delimitadores (`(...)`, `[...]`, `{...}`). `open`/`close`
/// são os caracteres delimitadores; variante própria para selecção de variantes
/// de tamanho no layout.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MathDelimitedElem {
    pub open:  char,
    pub body:  Content,
    pub close: char,
}

impl Element for MathDelimitedElem {
    fn plain_text(&self) -> String {
        format!("{}{}{}", self.open, self.body.plain_text(), self.close)
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::MathDelimited(Arc::new(MathDelimitedElem {
            open:  self.open,
            body:  self.body.map_content(transform)?,
            close: self.close,
        })))
    }

    fn map_text<F>(&self, _transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        // Terminal (math structural; não desce).
        Content::MathDelimited(Arc::new(self.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ex() -> MathDelimitedElem {
        MathDelimitedElem { open: '(', body: Content::text("x"), close: ')' }
    }

    #[test]
    fn plain_text_envolve() {
        assert_eq!(ex().plain_text(), "(x)");
    }

    #[test]
    fn igualdade_estrutural() {
        assert_eq!(ex(), ex());
        let mut other = ex();
        other.close = ']';
        assert_ne!(ex(), other);
    }

    #[test]
    fn map_content_recurse_body_preserva_delim() {
        let mut f = |c: &Content| -> SourceResult<Option<Content>> {
            match c {
                Content::Text(s) if s.as_str() == "x" => Ok(Some(Content::text("Z"))),
                _ => Ok(None),
            }
        };
        let r = ex().map_content(&mut f).unwrap();
        match r {
            Content::MathDelimited(e) => {
                assert_eq!(e.plain_text(), "(Z)");
                assert_eq!((e.open, e.close), ('(', ')'));
            }
            _ => panic!("esperado MathDelimited"),
        }
    }
}
