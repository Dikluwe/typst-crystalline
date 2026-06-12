//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/equation.md
//! @prompt-hash 5b4f50a9
//! @layer L1
//! @updated 2026-06-11
//!
//! `EquationElem` — Lote 10 P325 (por largura). `equation(body, block)`.
//! **Locatável** (P186B) — absorve `extract_payload`. Contentor **assimétrico**:
//! `map_content` recursa no body; `map_text` é **terminal** (math structural).

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::counter_update::CounterUpdate;
use crate::entities::element_kind::ElementKind;
use crate::entities::element_payload::ElementPayload;
use crate::entities::elements::Element;
use crate::entities::source_result::SourceResult;

/// Equação matemática. `block = true` → display (linha própria).
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct EquationElem {
    pub body:  Content,
    pub block: bool,
    /// **Numeração ativa (Lote F-2 S2, P335)** — assada na criação a partir do
    /// `engine.styles.custom("equation.numbering")` (escopo léxico via chain;
    /// fecha o canal global `SetEquationNumbering`/StateRegistry). Só "ativo";
    /// o **valor** do contador continua via Introspector.
    pub numbering_active: bool,
}

impl EquationElem {
    pub fn new(body: Content, block: bool) -> Self {
        Self { body, block, numbering_active: false }
    }
    pub fn new_numbered(body: Content, block: bool, numbering_active: bool) -> Self {
        Self { body, block, numbering_active }
    }
}

impl Element for EquationElem {
    fn plain_text(&self) -> String {
        // Paridade content.rs:1736: block → envolto em \n.
        if self.block {
            format!("\n{}\n", self.body.plain_text())
        } else {
            self.body.plain_text()
        }
    }

    fn map_content<F>(&self, transform: &mut F) -> SourceResult<Content>
    where
        F: FnMut(&Content) -> SourceResult<Option<Content>>,
    {
        Ok(Content::Equation(Arc::new(EquationElem {
            body:  self.body.map_content(transform)?,
            block: self.block,
            numbering_active: self.numbering_active,
        })))
    }

    fn map_text<F>(&self, _transform: &mut F) -> Content
    where
        F: FnMut(&str) -> String,
    {
        // **Terminal** (assimetria): math structural não desce em texto —
        // paridade hub (`Equation` no arm |-combinado de map_text, content.rs:2466).
        Content::Equation(Arc::new(self.clone()))
    }

    fn element_kind(&self) -> Option<ElementKind> {
        Some(ElementKind::Equation)
    }

    fn to_payload(&self) -> Option<ElementPayload> {
        Some(ElementPayload::Equation {
            block:          self.block,
            counter_update: CounterUpdate::Step,
            numbering_active: self.numbering_active,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;

    fn ex() -> EquationElem {
        EquationElem::new(Content::text("x"), false)
    }

    #[test]
    fn plain_text_inline_sem_quebras() {
        assert_eq!(ex().plain_text(), "x");
    }

    #[test]
    fn plain_text_block_com_quebras() {
        assert_eq!(EquationElem::new(Content::text("x"), true).plain_text(), "\nx\n");
    }

    #[test]
    fn is_empty_default_false_nao_delega() {
        let e = EquationElem::new(Content::Empty, false);
        assert!(!e.is_empty());
    }

    #[test]
    fn map_content_recurse_body_preserva_block() {
        let mut f = |c: &Content| -> SourceResult<Option<Content>> {
            match c {
                Content::Text(s, _) if s.as_str() == "x" => Ok(Some(Content::text("Z"))),
                _ => Ok(None),
            }
        };
        let src = EquationElem::new(Content::text("x"), true);
        match src.map_content(&mut f).unwrap() {
            Content::Equation(e) => {
                assert!(e.block);
                assert!(matches!(&e.body, Content::Text(s, _) if s.as_str() == "Z"));
            }
            _ => panic!("esperado Equation"),
        }
    }

    #[test]
    fn map_text_terminal_nao_desce() {
        // Assimetria: map_text NÃO recursa (body preservado intacto).
        let r = ex().map_text(&mut |s| s.to_uppercase());
        match r {
            Content::Equation(e) => assert!(matches!(&e.body, Content::Text(s, _) if s.as_str() == "x")),
            _ => panic!("esperado Equation"),
        }
    }

    #[test]
    fn locatavel_kind_e_payload() {
        assert_eq!(ex().element_kind(), Some(ElementKind::Equation));
        assert_eq!(ex().to_payload(), Some(ElementPayload::Equation {
            block: false, counter_update: CounterUpdate::Step, numbering_active: false,
        }));
    }

    fn h(e: &EquationElem) -> u64 {
        let mut s = DefaultHasher::new();
        e.hash(&mut s);
        s.finish()
    }

    #[test]
    fn payload_diferente_produz_hash_diferente() {
        assert_ne!(h(&ex()), h(&EquationElem::new(Content::text("x"), true)));
    }
}
