//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/math_limits_override.md
//! @prompt-hash edf8101e
//! @layer L1
//! @updated 2026-08-10
//!
//! `MathLimitsOverrideElem` — Passo 992. `limits(body, inline:)`/
//! `scripts(body)`. Força o posicionamento de scripts do `MathAttach` pai
//! (empilhado vs lateral), sem afectar layout/classe do `body` em si.

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::source_result::SourceResult;

/// Override explícito de posicionamento de scripts — `limits(body, inline:)`
/// (`limits: true`) ou `scripts(body)` (`limits: false`). `inline` só é
/// significativo quando `limits: true` — irrelevante em `scripts()`.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MathLimitsOverrideElem {
    pub body: Content,
    pub limits: bool,
    pub inline: bool,
}

impl Element for MathLimitsOverrideElem {
    fn plain_text(&self) -> String {
        self.body.plain_text()
    }

    fn is_empty(&self) -> bool {
        self.body.is_empty()
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::MathLimitsOverride(Arc::new(MathLimitsOverrideElem {
            body: self.body.map_content(transform)?,
            limits: self.limits,
            inline: self.inline,
        })))
    }

    fn map_text<F>(&self, _transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        // Terminal (math structural; não desce).
        Content::MathLimitsOverride(Arc::new(self.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ex() -> MathLimitsOverrideElem {
        MathLimitsOverrideElem { body: Content::text("A"), limits: true, inline: true }
    }

    #[test]
    fn plain_text_delega_no_body() {
        assert_eq!(ex().plain_text(), "A");
    }

    #[test]
    fn igualdade_estrutural() {
        assert_eq!(ex(), ex());
        let mut other = ex();
        other.limits = false;
        assert_ne!(ex(), other);
        let mut other2 = ex();
        other2.inline = false;
        assert_ne!(ex(), other2);
    }

    #[test]
    fn map_content_recurse_preserva_limits_e_inline() {
        let mut f = |c: &Content| -> SourceResult<Option<Content>> {
            match c {
                Content::Text(s) if s.as_str() == "A" => Ok(Some(Content::text("Z"))),
                _ => Ok(None),
            }
        };
        match ex().map_content(&mut f).unwrap() {
            Content::MathLimitsOverride(e) => {
                assert_eq!(e.plain_text(), "Z");
                assert!(e.limits);
                assert!(e.inline);
            }
            _ => panic!("esperado MathLimitsOverride"),
        }
    }

    #[test]
    fn map_text_terminal_preserva_campos() {
        let mut f = |s: &str| s.to_uppercase();
        match ex().map_text(&mut f) {
            Content::MathLimitsOverride(e) => {
                assert_eq!(e.plain_text(), "A"); // não desce — texto inalterado
                assert!(e.limits);
            }
            _ => panic!("esperado MathLimitsOverride"),
        }
    }
}
