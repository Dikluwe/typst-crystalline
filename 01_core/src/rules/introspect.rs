//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/introspect.md
//! @prompt-hash bc989be4
//! @layer L1
//! @updated 2026-04-20

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

/// "Congela" o AST substituindo nós dependentes de contexto (como CounterDisplay)
/// pelos seus valores em texto estático no momento exacto da introspecção (Passo 66, DEBT-18).
///
/// Resolve DEBT-18: sem esta função, a TOC mostraria os valores dos contadores
/// no início do documento, não o valor que cada contador tinha quando o título ocorreu.
///
/// Dois braços explícitos — sem wildcard para manter verificação de exaustividade:
/// - Containers: propagam recursivamente.
/// - Terminais: clonados directamente.
fn materialize_time(content: &Content, state: &CounterState) -> Content {
    match content {
        // O caso crítico: substituir o nó dinâmico pelo valor actual do contador.
        Content::CounterDisplay { kind } => {
            Content::text(state.display_value(kind))
        }

        // ── Containers com filhos (propagação recursiva) ──────────────────
        Content::Sequence(seq) => {
            Content::Sequence(
                seq.iter().map(|c| materialize_time(c, state)).collect::<Vec<_>>().into()
            )
        }
        // Passo 101: `Content::Strong` e `Content::Emph` removidos.
        // O arm `Content::Styled` abaixo cobre ambos (propaga recursivamente
        // preservando os estilos).
        Content::Heading { level, body } => Content::Heading {
            level: *level,
            body:  Box::new(materialize_time(body, state)),
        },
        Content::ListItem(body) => Content::ListItem(Box::new(materialize_time(body, state))),
        Content::EnumItem { number, body } => Content::EnumItem {
            number: *number,
            body:   Box::new(materialize_time(body, state)),
        },
        Content::Link { url, body } => Content::Link {
            url:  url.clone(),
            body: Box::new(materialize_time(body, state)),
        },
        Content::Labelled { target, label } => Content::Labelled {
            target: Box::new(materialize_time(target, state)),
            label:  label.clone(),
        },
        Content::Figure { body, caption, kind, numbering } => Content::Figure {
            body:      Box::new(materialize_time(body, state)),
            caption:   caption.as_ref().map(|c| Box::new(materialize_time(c, state))),
            kind:      kind.clone(),
            numbering: numbering.clone(),
        },

        // Passo 154B: Terms recurse em items; TermItem recurse em par.
        Content::Terms { items } => Content::Terms {
            items: items.iter().map(|c| materialize_time(c, state)).collect(),
        },
        Content::TermItem { term, description } => Content::TermItem {
            term:        Box::new(materialize_time(term, state)),
            description: Box::new(materialize_time(description, state)),
        },

        // Passo 155: Quote — recurse em body e attribution.
        Content::Quote { body, attribution, block, quotes } => Content::Quote {
            body:        Box::new(materialize_time(body, state)),
            attribution: attribution.as_ref().map(|c| Box::new(materialize_time(c, state))),
            block:       *block,
            quotes:      *quotes,
        },

        // ── Terminais — clonar directamente ──────────────────────────────
        // Nós matemáticos (Equation e subtipos) não podem conter CounterDisplay
        // em markup válido — clonados em bloco sem recursão.
        Content::Empty
        | Content::Text(_, _)
        | Content::Space
        | Content::Raw { .. }
        | Content::Ref { .. }
        | Content::SetHeadingNumbering { .. }
        | Content::SetFigureNumbering { .. }
        | Content::SetPage { .. }
        | Content::CounterUpdate { .. }
        | Content::Outline
        | Content::Linebreak
        | Content::MathAlignPoint
        | Content::MathIdent(_)
        | Content::MathText(_)
        | Content::Equation { .. }
        | Content::MathSequence(_)
        | Content::MathFrac { .. }
        | Content::MathAttach { .. }
        | Content::MathRoot { .. }
        | Content::MathDelimited { .. }
        | Content::MathMatrix { .. }
        | Content::MathCases { .. }
        | Content::Image { .. }
        | Content::Divider
        // Passo 156D (ADR-0061 Fase 1 sub-passo 2) — h/v spacing leaves.
        | Content::HSpace { .. }
        | Content::VSpace { .. }
        // Passo 156E (ADR-0061 Fase 1 sub-passo 3) — pagebreak leaf.
        | Content::Pagebreak { .. }
        | Content::Shape { .. } => content.clone(),
        // Passo 156C (ADR-0061 Fase 1) — pad / hide containers.
        // Materialize_time desce no body para resolver counters dentro;
        // padding e o invariante "hide" preservam-se.
        Content::Pad { body, sides } => Content::Pad {
            body:  Box::new(materialize_time(body, state)),
            sides: *sides,
        },
        Content::Hide { body } => Content::Hide {
            body: Box::new(materialize_time(body, state)),
        },
        // Passo 156G (ADR-0061 Fase 2) — block container.
        // Análogo a Pad: descer no body; preservar atributos.
        Content::Block { body, width, height, inset, breakable } => Content::Block {
            body:      Box::new(materialize_time(body, state)),
            width:     *width,
            height:    *height,
            inset:     *inset,
            breakable: *breakable,
        },
        // Passo 156H (ADR-0061 Fase 2 sub-passo 2) — box inline container.
        // Análogo a Block; preservar atributos.
        Content::Boxed { body, width, height, inset, baseline } => Content::Boxed {
            body:     Box::new(materialize_time(body, state)),
            width:    *width,
            height:   *height,
            inset:    *inset,
            baseline: *baseline,
        },
        // Passo 156I (ADR-0061 Fase 2 sub-passo 3) — stack compositivo.
        // Materialize_time em cada child; preservar dir/spacing.
        Content::Stack { children, dir, spacing } => {
            let new_children: Vec<Content> = children.iter()
                .map(|c| materialize_time(c, state))
                .collect();
            Content::Stack {
                children: std::sync::Arc::from(new_children),
                dir:      *dir,
                spacing:  *spacing,
            }
        },
        // Passo 156J (ADR-0061 Fase 3 sub-passo 1) — repeat.
        // Análogo a Block: descer no body; preservar atributos.
        Content::Repeat { body, gap, justify } => Content::Repeat {
            body:    Box::new(materialize_time(body, state)),
            gap:     *gap,
            justify: *justify,
        },
        Content::Transform { matrix, body } => Content::Transform {
            matrix: *matrix,
            body:   Box::new(materialize_time(body, state)),
        },
        Content::Grid { columns, rows, cells } => Content::Grid {
            columns: columns.clone(),
            rows:    rows.clone(),
            cells:   cells.iter().map(|c| materialize_time(c, state)).collect(),
        },
        // Passo 157A (ADR-0060 Fase 2 sub-passo 1) — table.
        // Análogo a Grid: descer em cada child; preservar tracks.
        Content::Table { columns, rows, children } => Content::Table {
            columns:  columns.clone(),
            rows:     rows.clone(),
            children: children.iter().map(|c| materialize_time(c, state)).collect(),
        },
        // Passo 157B (ADR-0060 Fase 2 sub-passo 2) — table cell.
        // Recurse no body; preserva fields x/y/colspan/rowspan
        // (Copy primitivos).
        Content::TableCell { body, x, y, colspan, rowspan } => Content::TableCell {
            body:    Box::new(materialize_time(body, state)),
            x:       *x,
            y:       *y,
            colspan: *colspan,
            rowspan: *rowspan,
        },
        // Passo 157C (ADR-0060 Fase 2 sub-passo 3) — par simétrico
        // TableHeader/TableFooter. Recurse no body; preserva repeat.
        Content::TableHeader { body, repeat } => Content::TableHeader {
            body:   Box::new(materialize_time(body, state)),
            repeat: *repeat,
        },
        Content::TableFooter { body, repeat } => Content::TableFooter {
            body:   Box::new(materialize_time(body, state)),
            repeat: *repeat,
        },
        Content::Align { alignment, body } => Content::Align {
            alignment: *alignment,
            body:      Box::new(materialize_time(body, state)),
        },
        Content::Place { alignment, dx, dy, scope, body } => Content::Place {
            alignment: *alignment,
            dx:        *dx,
            dy:        *dy,
            scope:     *scope,
            body:      Box::new(materialize_time(body, state)),
        },

        // Passo 99 (ADR-0038): `Styled` é transparente para materialização de
        // contadores — o body é processado e os estilos preservados.
        Content::Styled(body, styles) => Content::Styled(
            Box::new(materialize_time(body, state)),
            styles.clone(),
        ),
    }
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

            // Gerar label automática única para que a TOC possa referenciar este título.
            state.auto_label_counter += 1;
            let auto_label = crate::entities::label::Label(
                format!("auto-toc-{}", state.auto_label_counter)
            );

            // Registar prefixo se a numeração estiver activa.
            // Se inactiva, inserir string vazia — o braço Ref resolverá para ""
            // em vez de usar o fallback "@auto-toc-N".
            let resolved_text = if state.is_numbering_active("heading") {
                state.format_hierarchical("heading")
                    .map(|prefix| format!("Secção {}", prefix))
                    .unwrap_or_default()
            } else {
                String::new()
            };
            state.resolved_labels.insert(auto_label.clone(), resolved_text);

            // Guardar para a TOC: congelar o AST substitui CounterDisplay
            // pelo valor actual (DEBT-18 resolvido via materialize_time).
            let frozen_body = materialize_time(body, state);
            state.headings_for_toc.push((auto_label, frozen_body, *level as usize));

            walk(body, state);
        }

        Content::Equation { block, body } => {
            if *block && state.is_numbering_active("equation") {
                state.step_flat("equation");
            }
            walk(body, state);
        }

        Content::Figure { body, caption, kind, numbering } => {
            // Avançar o contador apenas se a figura tiver numeração activa e legenda —
            // figuras sem caption não consomem número (evita "Figura 1", [gap], "Figura 3").
            if numbering.is_some() && caption.is_some() {
                let counter = state.local_figure_counters
                    .entry(kind.clone())
                    .or_insert(0);
                *counter += 1;
                let figure_number = *counter;
                state.figure_numbers
                    .entry(kind.clone())
                    .or_default()
                    .push(figure_number);
            }
            walk(body, state);
            if let Some(cap) = caption {
                walk(cap, state);
            }
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
                Content::Figure { kind, numbering, caption, .. } => {
                    let n = if numbering.is_some() && caption.is_some() {
                        state.figure_numbers
                            .get(kind.as_str())
                            .and_then(|v| v.last())
                            .copied()
                            .unwrap_or(0)
                    } else {
                        0
                    };
                    if n > 0 {
                        state.figure_label_numbers.insert(label.clone(), n);
                        Some(format!("Figura {}", n))
                    } else {
                        Some(String::new())
                    }
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

        // Passo 101: `Content::Strong`/`Content::Emph` removidos; o caso
        // equivalente de filhos com Labels/contadores dentro de um bloco
        // estilizado está agora coberto pelo arm `Content::Styled` abaixo.

        // Terminais e nós sem efeito em contadores — cobertos explicitamente
        // para que o compilador detecte variantes em falta (sem wildcard silencioso).
        Content::SetFigureNumbering { .. } => {
            // No-op: a numeração está baked-in em cada nó Figure (capturada em eval).
        }

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
        | Content::Linebreak
        | Content::Image { .. }
        | Content::SetPage { .. }
        | Content::Divider
        // Passo 156D — h/v spacing leaves; sem effect em counters.
        | Content::HSpace { .. }
        | Content::VSpace { .. }
        // Passo 156E — pagebreak leaf; sem effect em counters.
        | Content::Pagebreak { .. }
        | Content::Shape { .. } => {}

        // Passo 154B — Terms / TermItem: descem em items para que filhos
        // com contadores ou labels sejam processados.
        Content::Terms { items } => {
            for item in items { walk(item, state); }
        }
        Content::TermItem { term, description } => {
            walk(term, state);
            walk(description, state);
        }

        // Passo 155 — Quote: walk em body + attribution.
        Content::Quote { body, attribution, .. } => {
            walk(body, state);
            if let Some(a) = attribution {
                walk(a, state);
            }
        }

        Content::Transform { body, .. } => walk(body, state),

        Content::Grid { cells, .. } => {
            for cell in cells { walk(cell, state); }
        }

        // Passo 157A — Table (paridade Grid).
        Content::Table { children, .. } => {
            for c in children { walk(c, state); }
        }

        // Passo 157B — TableCell (recurse no body).
        Content::TableCell { body, .. } => walk(body, state),

        // Passo 157C — par simétrico TableHeader/TableFooter
        // (recurse no body).
        Content::TableHeader { body, .. } => walk(body, state),
        Content::TableFooter { body, .. } => walk(body, state),

        Content::Align { body, .. } => walk(body, state),

        Content::Place { body, .. } => walk(body, state),

        // Passo 156C (ADR-0061 Fase 1) — pad / hide são containers
        // estruturais; descer no body para que counters/labels dentro sejam
        // processados. `Hide` mesmo "ocultando visualmente" mantém a
        // semântica de presence (label/ref dentro de hide ainda resolvem).
        Content::Pad  { body, .. } => walk(body, state),
        Content::Hide { body }     => walk(body, state),

        // Passo 156G (ADR-0061 Fase 2) — block container; descer no body.
        Content::Block { body, .. } => walk(body, state),

        // Passo 156H (ADR-0061 Fase 2 sub-passo 2) — box inline container.
        Content::Boxed { body, .. } => walk(body, state),

        // Passo 156I (ADR-0061 Fase 2 sub-passo 3) — stack compositivo.
        // Walk em cada child em ordem (counters/labels resolvem).
        Content::Stack { children, .. } => {
            for c in children.iter() {
                walk(c, state);
            }
        },

        // Passo 156J (ADR-0061 Fase 3 sub-passo 1) — repeat container.
        // Walk no body uma vez (counters/labels dentro de body
        // resolvem; semântica de repetição é runtime-only e não
        // multiplica state — vanilla repeat também só conta uma vez).
        Content::Repeat { body, .. } => walk(body, state),

        // Passo 99 (ADR-0038): `Styled` é transparente — desce no body.
        Content::Styled(body, _) => walk(body, state),

        Content::Outline => {
            state.has_outline = true;
            // Outline não altera contadores — apenas sinaliza que o fixpoint é necessário.
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::{
        content::Content,
        counter_state::CounterAction,
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

    // ── Testes de Passo 61 — TOC ─────────────────────────────────────────

    #[test]
    fn introspect_cataloga_headings_para_toc() {
        let content = Content::Sequence(vec![
            Content::SetHeadingNumbering { active: true },
            Content::heading(1, Content::text("Introdução")),
            Content::heading(2, Content::text("Motivação")),
            Content::heading(1, Content::text("Conclusão")),
        ].into());

        let state = introspect(&content);
        assert_eq!(state.headings_for_toc.len(), 3);

        let (_, title_0, level_0) = &state.headings_for_toc[0];
        assert_eq!(title_0.plain_text(), "Introdução");
        assert_eq!(*level_0, 1);

        let (_, _, level_1) = &state.headings_for_toc[1];
        assert_eq!(*level_1, 2);
    }

    #[test]
    fn introspect_gera_labels_automaticas_unicas() {
        let content = Content::Sequence(vec![
            Content::heading(1, Content::text("A")),
            Content::heading(1, Content::text("B")),
        ].into());

        let state = introspect(&content);
        let label_a = &state.headings_for_toc[0].0;
        let label_b = &state.headings_for_toc[1].0;
        assert_ne!(label_a, label_b, "labels automáticas devem ser únicas");

        // As labels devem estar em resolved_labels
        assert!(state.resolved_labels.contains_key(label_a));
        assert!(state.resolved_labels.contains_key(label_b));
    }

    #[test]
    fn introspect_heading_sem_numbering_insere_string_vazia_em_resolved_labels() {
        // Sem numeração activa, resolved_labels deve conter "" (não "@auto-toc-N").
        let content = Content::heading(1, Content::text("Título"));
        let state = introspect(&content);
        assert_eq!(state.headings_for_toc.len(), 1);
        let (label, _, _) = &state.headings_for_toc[0];
        assert_eq!(
            state.resolved_labels.get(label).map(|s| s.as_str()),
            Some(""),
            "heading sem numeração deve ter string vazia em resolved_labels"
        );
    }

    // ── Testes de Passo 62 — Figuras ─────────────────────────────────────

    #[test]
    fn introspect_resolve_label_de_figura() {
        let content = Content::Sequence(
            vec![Content::Labelled {
                label:  Label("fig1".to_string()),
                target: Box::new(Content::Figure {
                    body:      Box::new(Content::text("Um gráfico")),
                    caption:   Some(Box::new(Content::text("Evolução"))),
                    kind:      "image".to_string(),
                    numbering: Some("1".to_string()),
                }),
            }]
            .into(),
        );

        let state = introspect(&content);
        assert_eq!(
            state.resolved_labels.get(&Label("fig1".to_string())).map(|s| s.as_str()),
            Some("Figura 1"),
            "label de figura deve resolver para 'Figura 1'"
        );
    }

    #[test]
    fn introspect_duas_figuras_contadores_independentes() {
        let content = Content::Sequence(
            vec![
                Content::Labelled {
                    label:  Label("f1".to_string()),
                    target: Box::new(Content::Figure {
                        body:      Box::new(Content::text("A")),
                        caption:   Some(Box::new(Content::text("Legenda A"))),
                        kind:      "image".to_string(),
                        numbering: Some("1".to_string()),
                    }),
                },
                Content::Labelled {
                    label:  Label("f2".to_string()),
                    target: Box::new(Content::Figure {
                        body:      Box::new(Content::text("B")),
                        caption:   Some(Box::new(Content::text("Legenda B"))),
                        kind:      "image".to_string(),
                        numbering: Some("1".to_string()),
                    }),
                },
            ]
            .into(),
        );

        let state = introspect(&content);
        assert_eq!(
            state.resolved_labels.get(&Label("f1".to_string())).map(|s| s.as_str()),
            Some("Figura 1")
        );
        assert_eq!(
            state.resolved_labels.get(&Label("f2".to_string())).map(|s| s.as_str()),
            Some("Figura 2")
        );
    }

    #[test]
    fn introspect_figura_sem_caption_nao_incrementa_contador() {
        let content = Content::Sequence(
            vec![
                Content::Figure {
                    body:      Box::new(Content::text("Diagrama")),
                    caption:   None,
                    kind:      "image".to_string(),
                    numbering: Some("1".to_string()),
                },
                Content::Labelled {
                    label:  Label("f2".to_string()),
                    target: Box::new(Content::Figure {
                        body:      Box::new(Content::text("B")),
                        caption:   Some(Box::new(Content::text("Legenda"))),
                        kind:      "image".to_string(),
                        numbering: Some("1".to_string()),
                    }),
                },
            ]
            .into(),
        );

        let state = introspect(&content);
        // Figura sem caption não consome contador — a segunda figura numerada é "Figura 1"
        assert_eq!(
            state.resolved_labels.get(&Label("f2".to_string())).map(|s| s.as_str()),
            Some("Figura 1"),
            "figura sem caption não deve consumir o contador"
        );
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

    // ── Testes de Passo 66 — Materialização temporal (DEBT-18) ───────────────

    #[test]
    fn materialize_time_substitui_counter_display() {
        use crate::entities::counter_state::CounterState;

        let mut state = CounterState::new();
        state.update_flat("fig", 42);

        let dynamic_ast = Content::Sequence(
            vec![
                Content::text("Figura "),
                Content::CounterDisplay { kind: "fig".to_string() },
            ]
            .into(),
        );

        let frozen = materialize_time(&dynamic_ast, &state);

        let expected = Content::Sequence(
            vec![
                Content::text("Figura "),
                Content::text("42"),
            ]
            .into(),
        );

        assert_eq!(frozen, expected,
            "CounterDisplay deve ser materializado em Text com o valor do contador");
    }

    #[test]
    fn materialize_time_preserva_terminais() {
        use crate::entities::counter_state::CounterState;

        let state = CounterState::new();

        // Nós terminais sem CounterDisplay devem ser clonados sem alteração.
        let content = Content::Sequence(
            vec![
                Content::text("Texto estático"),
                Content::strong(Content::text("Negrito")),
            ]
            .into(),
        );

        let frozen = materialize_time(&content, &state);
        assert_eq!(frozen, content, "Terminais sem CounterDisplay não devem ser alterados");
    }

    #[test]
    fn introspect_headings_for_toc_congelados() {
        // Simular: = Figura #counter("fig").display()
        // O CounterDisplay no título deve ser substituído pelo valor no momento
        // da introspecção — não pelo valor quando a TOC for renderizada.
        let content = Content::Sequence(
            vec![
                Content::SetHeadingNumbering { active: true },
                Content::CounterUpdate {
                    key:    "fig".to_string(),
                    action: CounterAction::Update(7),
                },
                Content::heading(1, Content::Sequence(
                    vec![
                        Content::text("Figura "),
                        Content::CounterDisplay { kind: "fig".to_string() },
                    ]
                    .into(),
                )),
            ]
            .into(),
        );

        let state = introspect(&content);
        assert_eq!(state.headings_for_toc.len(), 1);

        let (_, frozen_body, _) = &state.headings_for_toc[0];
        let text = frozen_body.plain_text();
        // O body congelado deve conter "7" (valor no momento da introspecção),
        // não "0" (valor no início do documento quando a TOC é renderizada).
        assert!(text.contains("7"),
            "CounterDisplay no título deve ser congelado com o valor correcto: {:?}", text);
    }

    // ── Testes de Passo 75 — figure_numbers por kind (DEBT-14/15) ───────────

    #[test]
    fn figure_tem_kind_e_numbering() {
        let fig = Content::Figure {
            body:      Box::new(Content::text("corpo")),
            caption:   Some(Box::new(Content::text("legenda"))),
            kind:      "image".to_string(),
            numbering: Some("1".to_string()),
        };
        if let Content::Figure { kind, numbering, .. } = fig {
            assert_eq!(kind, "image");
            assert_eq!(numbering, Some("1".to_string()));
        } else {
            panic!("Variante inesperada");
        }
    }

    #[test]
    fn figuras_kind_diferente_contadores_independentes() {
        let doc = Content::Sequence(vec![
            Content::Figure {
                body:      Box::new(Content::text("img1")),
                caption:   Some(Box::new(Content::text("cap1"))),
                kind:      "image".to_string(),
                numbering: Some("1".to_string()),
            },
            Content::Figure {
                body:      Box::new(Content::text("tab1")),
                caption:   Some(Box::new(Content::text("cap2"))),
                kind:      "table".to_string(),
                numbering: Some("1".to_string()),
            },
            Content::Figure {
                body:      Box::new(Content::text("img2")),
                caption:   Some(Box::new(Content::text("cap3"))),
                kind:      "image".to_string(),
                numbering: Some("1".to_string()),
            },
        ].into());

        let state = introspect(&doc);

        let image_nums = state.figure_numbers.get("image").cloned().unwrap_or_default();
        let table_nums = state.figure_numbers.get("table").cloned().unwrap_or_default();

        assert_eq!(image_nums, vec![1, 2],
            "Duas figuras de kind 'image' devem produzir [1, 2]");
        assert_eq!(table_nums, vec![1],
            "Uma figura de kind 'table' deve produzir [1] independentemente");
    }
}
