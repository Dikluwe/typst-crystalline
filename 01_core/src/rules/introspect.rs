//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/introspect.md
//! @prompt-hash bc989be4
//! @layer L1
//! @updated 2026-04-12

use crate::entities::{
    content::Content,
    counter_state::{CounterAction, CounterState},
};

/// Pré-passagem analítica sobre `Content`.
///
/// Percorre a árvore completa uma vez, avançando contadores e populando
/// `resolved_labels`, sem realizar nenhum cálculo visual.
///
/// O `CounterState` retornado é injectado no Layouter como estado inicial
/// (apenas o campo `resolved_labels`), garantindo que todas as referências —
/// incluindo para a frente — estão resolvidas antes do primeiro `FrameItem`
/// ser gerado.
pub fn introspect(content: &Content) -> CounterState {
    let mut state = CounterState::new();
    walk(content, &mut state);
    state
}

/// Percurso recursivo sem efeitos visuais.
///
/// Replica exactamente os side-effects de estado que o Layouter produz,
/// na mesma ordem de travessia, mas sem aceder a `FontMetrics` nem alocar
/// `Frame`/`FrameItem`.
fn walk(content: &Content, state: &mut CounterState) {
    match content {
        Content::Sequence(seq) => {
            for item in seq.iter() {
                walk(item, state);
            }
        }

        Content::Heading { level, body } => {
            state.step_hierarchical("heading", *level as usize);
            walk(body, state);
        }

        Content::Equation { block, body } => {
            if *block && state.is_numbering_active("equation") {
                state.step_flat("equation");
            }
            walk(body, state);
        }

        Content::Labelled { target, label } => {
            // Walk no target primeiro — garante que o contador já avançou.
            walk(target, state);

            let resolved_text = match &**target {
                Content::Heading { .. } => state
                    .format_hierarchical("heading")
                    .map(|n| format!("Secção {}", n)),
                Content::Equation { block, .. } if *block => {
                    let n = state.get_flat("equation");
                    if n > 0 { Some(format!("Equação ({})", n)) } else { None }
                }
                _ => None,
            };
            if let Some(text) = resolved_text {
                state.resolved_labels.insert(label.clone(), text);
            }
        }

        Content::SetHeadingNumbering { active } => {
            state.numbering_active.insert("heading".to_string(), *active);
        }

        Content::CounterUpdate { key, action } => match action {
            CounterAction::Step => {
                if key == "heading" {
                    state.step_hierarchical("heading", 1);
                } else {
                    state.step_flat(key);
                }
            }
            CounterAction::Update(val) => {
                state.update_flat(key, *val);
            }
        },

        // Nós com filhos que podem conter Labels — percorrer recursivamente.
        Content::Strong(body) | Content::Emph(body) => walk(body, state),

        // Terminais e nós sem efeito em contadores — cobertos explicitamente
        // para que o compilador detecte variantes em falta (sem wildcard silencioso).
        Content::Empty
        | Content::Text(_, _)
        | Content::Space
        | Content::Ref { .. }
        | Content::CounterDisplay { .. }
        | Content::Raw { .. }
        | Content::ListItem(_)
        | Content::EnumItem { .. }
        | Content::Link { .. }
        | Content::MathSequence(_)
        | Content::MathIdent(_)
        | Content::MathText(_)
        | Content::MathFrac { .. }
        | Content::MathAttach { .. }
        | Content::MathRoot { .. }
        | Content::MathDelimited { .. }
        | Content::MathMatrix { .. }
        | Content::MathCases { .. }
        | Content::MathAlignPoint
        | Content::Linebreak => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::{
        content::Content,
        counter_state::{CounterAction, CounterState},
        label::Label,
    };

    #[test]
    fn introspect_popula_label_forward() {
        // Ref antes do Labelled — forward reference
        let content = Content::Sequence(
            vec![
                Content::SetHeadingNumbering { active: true },
                Content::Ref { target: Label("conclusao".to_string()) },
                Content::Labelled {
                    label:  Label("conclusao".to_string()),
                    target: Box::new(Content::heading(1, Content::text("Conclusão"))),
                },
            ]
            .into(),
        );

        let state = introspect(&content);
        assert!(
            state.resolved_labels.contains_key(&Label("conclusao".to_string())),
            "introspect deve popular resolved_labels mesmo para forward refs"
        );
        assert_eq!(
            state
                .resolved_labels
                .get(&Label("conclusao".to_string()))
                .map(|s| s.as_str()),
            Some("Secção 1")
        );
    }

    #[test]
    fn introspect_counter_update_e_aplicado() {
        let content = Content::Sequence(
            vec![Content::CounterUpdate {
                key:    "equation".to_string(),
                action: CounterAction::Update(5),
            }]
            .into(),
        );

        let state = introspect(&content);
        assert_eq!(state.get_flat("equation"), 5);
    }

    #[test]
    fn introspect_dois_conteudos_independentes() {
        let content_a = Content::Labelled {
            label:  Label("a".to_string()),
            target: Box::new(Content::heading(1, Content::text("A"))),
        };
        let content_b = Content::Ref { target: Label("a".to_string()) };

        let state_a = introspect(&content_a);
        let state_b = introspect(&content_b);

        assert!(state_a.resolved_labels.contains_key(&Label("a".to_string())));
        assert!(
            !state_b.resolved_labels.contains_key(&Label("a".to_string())),
            "estados de introspecção devem ser independentes"
        );
    }

    #[test]
    fn introspect_set_heading_numbering_activa_flag() {
        let content = Content::SetHeadingNumbering { active: true };
        let state = introspect(&content);
        assert!(state.is_numbering_active("heading"));
    }

    #[test]
    fn introspect_backward_ref_tambem_funciona() {
        // Labelled antes de Ref — deve também popular o mapa
        let content = Content::Sequence(
            vec![
                Content::Labelled {
                    label:  Label("sec".to_string()),
                    target: Box::new(Content::heading(1, Content::text("Secção"))),
                },
                Content::Ref { target: Label("sec".to_string()) },
            ]
            .into(),
        );

        let state = introspect(&content);
        assert!(
            state.resolved_labels.contains_key(&Label("sec".to_string())),
            "backward ref deve também estar em resolved_labels"
        );
    }
}
