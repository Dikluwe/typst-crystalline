//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/element_info.md
//! @prompt-hash 2ec956ae
//! @layer L1
//! @updated 2026-04-30
//!
//! `ElementInfo` — agrupa `ElementPayload` (kind-específico) com a
//! `Label` opcional atribuída por sintaxe `<label>`. P161 sub-passo .8.
//!
//! Separação payload/label expressa que label é ortogonal ao kind:
//! qualquer elemento indexado pode ter ou não ter label.

use crate::entities::element_payload::ElementPayload;
use crate::entities::label::Label;

/// Dados completos de um elemento indexado: payload + label opcional.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ElementInfo {
    pub payload: ElementPayload,
    pub label:   Option<Label>,
}

impl ElementInfo {
    /// Constrói sem label.
    pub fn new(payload: ElementPayload) -> Self {
        Self { payload, label: None }
    }

    /// Constrói com label.
    pub fn with_label(payload: ElementPayload, label: Label) -> Self {
        Self { payload, label: Some(label) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::counter_update::CounterUpdate;

    fn payload_heading() -> ElementPayload {
        ElementPayload::Heading {
            depth: 1,
            body_hash: 0,
            counter_update: CounterUpdate::Step,
        }
    }

    #[test]
    fn new_constroi_sem_label() {
        let info = ElementInfo::new(payload_heading());
        assert_eq!(info.label, None);
    }

    #[test]
    fn with_label_constroi_com_label() {
        let info = ElementInfo::with_label(
            payload_heading(),
            Label("intro".to_string()),
        );
        assert_eq!(info.label, Some(Label("intro".to_string())));
    }

    #[test]
    fn igualdade_compara_payload_e_label() {
        let a = ElementInfo::with_label(payload_heading(), Label("x".into()));
        let b = ElementInfo::with_label(payload_heading(), Label("x".into()));
        let c = ElementInfo::with_label(payload_heading(), Label("y".into()));
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn clone_preserva_campos() {
        let a = ElementInfo::with_label(payload_heading(), Label("z".into()));
        let b = a.clone();
        assert_eq!(a, b);
    }
}
