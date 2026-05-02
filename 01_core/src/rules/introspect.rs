//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/introspect.md
//! @prompt-hash 281ed270
//! @layer L1
//! @updated 2026-04-30
//!
//! P162 sub-passos .E + .F: walk passa a aceitar `&mut Locator` e
//! `&mut Vec<Tag>`; emite `Tag::Start` antes da mutação de estado e
//! `Tag::End` depois da recursão para variantes locatable
//! (Heading/Figure/Cite). Tags descartadas em M1 — consumidor real
//! virá em M2/M3. API pública preservada (`introspect()` retorna
//! `CounterStateLegacy` igual a antes).

pub mod convergence;
pub mod extract_payload;
pub mod fixpoint;
pub mod from_tags;
pub mod locatable;

use crate::entities::{
    content::Content,
    content_hash::hash_content,
    counter_state_legacy::{CounterAction, CounterStateLegacy},
    element_info::ElementInfo,
    label::Label,
    locator::Locator,
    tag::Tag,
};

use crate::rules::introspect::extract_payload::extract_payload as do_extract_payload;

/// Pré-passagem analítica sobre `Content` — entry point legado.
///
/// Percorre a árvore completa uma vez, avançando contadores e populando
/// `resolved_labels`, sem realizar nenhum cálculo visual.
///
/// O `CounterStateLegacy` retornado é injectado no Layouter como estado inicial
/// (apenas o campo `resolved_labels`), garantindo que todas as referências —
/// incluindo para a frente — estão resolvidas antes do primeiro `FrameItem`
/// ser gerado.
///
/// **P166 (M4)**: esta função é agora wrapper sobre
/// `introspect_with_introspector` — descarta o `TagIntrospector`. Consumers
/// que precisem do introspector devem chamar `introspect_with_introspector`
/// directamente. M5 migra primeiro consumer real; M6 elimina este wrapper
/// + `CounterStateLegacy`.
///
/// **P173**: passa `None, None` para `introspect_with_introspector` —
/// Funcs em `state.update(key, fn)` são silenciosamente ignoradas neste
/// path legacy (sem Engine disponível). Comportamento defensivo coerente
/// com P171 ("update sem init é ignorado").
pub fn introspect(content: &Content) -> CounterStateLegacy {
    let (state, _introspector) = introspect_with_introspector(content, None, None);
    state
}

/// Entry point novo (M4 / P166): produz `CounterStateLegacy` E
/// `TagIntrospector` num único walk. Consumers que precisem do
/// introspector usam este entry point; consumers legacy continuam a
/// usar `introspect()`.
///
/// **Walk único subjacente**: state + introspector vêm da mesma
/// passagem — não há duplicação. `introspect()` é wrapper que descarta
/// o introspector.
///
/// **P173 (M9 sub-passo 5)**: aceita `Engine + EvalContext` opcionais.
/// Quando ambos `Some`, `from_tags` avalia `StateUpdate::Func` via
/// `apply_func`. Walk em si **NÃO modificado** — continua puro
/// (P163 invariante preservado). Engine só intervém em `from_tags`
/// (eval localizada).
///
/// Padrão de migração M5+: caller que actualmente faz
/// `let state = introspect(&c)` e quer queries via Introspector pode
/// adoptar `let (state, intr) = introspect_with_introspector(&c, None, None)`
/// sem custo adicional. Caller que precisa de eval real de Funcs em
/// state.update passa `Some(&mut engine), Some(&mut ctx)`.
pub fn introspect_with_introspector(
    content: &Content,
    engine:  Option<&mut crate::entities::engine::Engine<'_>>,
    ctx:     Option<&mut crate::rules::eval::EvalContext>,
) -> (CounterStateLegacy, crate::entities::introspector::TagIntrospector) {
    let mut state = CounterStateLegacy::new();
    let mut locator = Locator::new();
    let mut tags: Vec<Tag> = Vec::new();
    walk(content, &mut state, &mut locator, &mut tags, None);
    let introspector = self::from_tags::from_tags(&tags, engine, ctx);
    (state, introspector)
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
fn materialize_time(content: &Content, state: &CounterStateLegacy) -> Content {
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
        | Content::Shape { .. }
        // P169 (M9): Metadata é terminal — clonar directamente.
        | Content::Metadata { .. }
        // P171 (M9): State e StateUpdate são terminais.
        | Content::State { .. }
        | Content::StateUpdate { .. } => content.clone(),
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
        // Passo 159A — par acoplado Bibliography + Cite. Recurse em
        // title (Bibliography) ou supplement (Cite); preserva
        // entries/key.
        Content::Bibliography { entries, title } => Content::Bibliography {
            entries: entries.clone(),
            title:   title.as_ref().map(|t| Box::new(materialize_time(t, state))),
        },
        Content::Cite { key, supplement, form } => Content::Cite {
            key:        key.clone(),
            supplement: supplement.as_ref().map(|s| Box::new(materialize_time(s, state))),
            form:       *form,
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
///
/// **P162 .E**: emite `Tag::Start`/`Tag::End` em paralelo para os 3 kinds
/// locatable (Heading/Figure/Cite). `label_from_parent` é `Some(label)`
/// quando este nó é descendente directo de um `Content::Labelled` wrapper;
/// `None` caso contrário. Tags acumuladas em `tags` são descartadas no
/// fim do walk em M1 — consumo real virá em M2/M3.
fn walk(
    content:           &Content,
    state:             &mut CounterStateLegacy,
    locator:           &mut Locator,
    tags:              &mut Vec<Tag>,
    label_from_parent: Option<&Label>,
) {
    // P162 .E: emissão Tag::Start em paralelo, antes da mutação de estado.
    let emitted_loc = if let Some(payload) = do_extract_payload(content) {
        let loc = locator.next();
        let info = ElementInfo {
            payload,
            label: label_from_parent.cloned(),
        };
        tags.push(Tag::Start(loc, info));
        Some(loc)
    } else {
        None
    };

    match content {
        Content::Sequence(seq) => {
            for item in seq.iter() {
                walk(item, state, locator, tags, None);
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

            walk(body, state, locator, tags, None);
        }

        Content::Equation { block, body } => {
            if *block && state.is_numbering_active("equation") {
                state.step_flat("equation");
            }
            walk(body, state, locator, tags, None);
        }

        Content::Figure { body, caption, kind, numbering } => {
            // Avançar o contador apenas se a figura tiver numeração activa e legenda —
            // figuras sem caption não consomem número (evita "Figura 1", [gap], "Figura 3").
            if numbering.is_some() && caption.is_some() {
                // P158C: kind é Option<String>; resolver default "image"
                // em uso (não em construção).
                let kind_key = kind.as_deref().unwrap_or("image").to_string();
                let counter = state.local_figure_counters
                    .entry(kind_key.clone())
                    .or_insert(0);
                *counter += 1;
                let figure_number = *counter;
                state.figure_numbers
                    .entry(kind_key)
                    .or_default()
                    .push(figure_number);
            }
            walk(body, state, locator, tags, None);
            if let Some(cap) = caption {
                walk(cap, state, locator, tags, None);
            }
        }

        Content::Labelled { target, label } => {
            // Walk no target primeiro — garante que o contador já avançou.
            // P162 .E: passa `Some(label)` para que o tag emitido pelo
            // walk recursivo (ex. Heading) inclua a label do wrapper.
            walk(target, state, locator, tags, Some(label));

            let resolved_text = match &**target {
                Content::Heading { .. } => state
                    .format_hierarchical("heading")
                    .map(|n| format!("Secção {}", n)),
                Content::Equation { block, .. } if *block => {
                    let n = state.get_flat("equation");
                    if n > 0 { Some(format!("Equação ({})", n)) } else { None }
                }
                Content::Figure { kind, numbering, caption, .. } => {
                    // P158C: kind é Option<String>; resolver default "image"
                    // em uso (paridade walk arm acima).
                    let kind_key = kind.as_deref().unwrap_or("image");
                    let n = if numbering.is_some() && caption.is_some() {
                        state.figure_numbers
                            .get(kind_key)
                            .and_then(|v| v.last())
                            .copied()
                            .unwrap_or(0)
                    } else {
                        0
                    };
                    if n > 0 {
                        state.figure_label_numbers.insert(label.clone(), n);
                        // Passo 158B: supplement localizado por lang.
                        // Lang vem de state.lang (None → fallback PT
                        // per diagnóstico P158B §8.2 backwards compat).
                        let supplement = crate::rules::lang::figure_supplement::figure_supplement_for_lang(
                            kind_key,
                            state.lang.as_ref(),
                        );
                        Some(format!("{} {}", supplement, n))
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
        | Content::Shape { .. }
        // P169 (M9): Metadata é terminal — sem efeito em counters.
        // Tag::Start/End já é emitido no topo de walk via extract_payload
        // (que produz `Some(ElementPayload::Metadata)`).
        | Content::Metadata { .. }
        // P171 (M9): State e StateUpdate são terminais. Tag emitido
        // no topo via extract_payload.
        | Content::State { .. }
        | Content::StateUpdate { .. } => {}

        // Passo 154B — Terms / TermItem: descem em items para que filhos
        // com contadores ou labels sejam processados.
        Content::Terms { items } => {
            for item in items { walk(item, state, locator, tags, None); }
        }
        Content::TermItem { term, description } => {
            walk(term, state, locator, tags, None);
            walk(description, state, locator, tags, None);
        }

        // Passo 155 — Quote: walk em body + attribution.
        Content::Quote { body, attribution, .. } => {
            walk(body, state, locator, tags, None);
            if let Some(a) = attribution {
                walk(a, state, locator, tags, None);
            }
        }

        Content::Transform { body, .. } => walk(body, state, locator, tags, None),

        Content::Grid { cells, .. } => {
            for cell in cells { walk(cell, state, locator, tags, None); }
        }

        // Passo 157A — Table (paridade Grid).
        Content::Table { children, .. } => {
            for c in children { walk(c, state, locator, tags, None); }
        }

        // Passo 157B — TableCell (recurse no body).
        Content::TableCell { body, .. } => walk(body, state, locator, tags, None),

        // Passo 157C — par simétrico TableHeader/TableFooter
        // (recurse no body).
        Content::TableHeader { body, .. } => walk(body, state, locator, tags, None),
        Content::TableFooter { body, .. } => walk(body, state, locator, tags, None),

        // P181H: walk arm puro (P163 invariante restaurada para bib).
        // Pré-P181H (P159C/F): walk mutava `state.bib_entries.extend(...)`
        // e `state.bib_numbers.entry(key).or_insert(...)` directamente.
        // Pós-P181H: tag emitida no topo via `extract_payload` (P181D);
        // BibStore populado por `from_tags` arm Bibliography (P181E).
        // Apenas descida no `title` permanece — `entries` são dados
        // opacos consumidos pelo `extract_payload` que constrói
        // `ElementPayload::Bibliography { entries }`. Cite walk em
        // supplement; sem validação cross-reference (ADR-0017
        // Introspection runtime adiada).
        Content::Bibliography { title, .. } => {
            if let Some(t) = title {
                walk(t, state, locator, tags, None);
            }
        }
        Content::Cite { supplement, .. } => {
            if let Some(s) = supplement { walk(s, state, locator, tags, None); }
        }

        Content::Align { body, .. } => walk(body, state, locator, tags, None),

        Content::Place { body, .. } => walk(body, state, locator, tags, None),

        // Passo 156C (ADR-0061 Fase 1) — pad / hide são containers
        // estruturais; descer no body para que counters/labels dentro sejam
        // processados. `Hide` mesmo "ocultando visualmente" mantém a
        // semântica de presence (label/ref dentro de hide ainda resolvem).
        Content::Pad  { body, .. } => walk(body, state, locator, tags, None),
        Content::Hide { body }     => walk(body, state, locator, tags, None),

        // Passo 156G (ADR-0061 Fase 2) — block container; descer no body.
        Content::Block { body, .. } => walk(body, state, locator, tags, None),

        // Passo 156H (ADR-0061 Fase 2 sub-passo 2) — box inline container.
        Content::Boxed { body, .. } => walk(body, state, locator, tags, None),

        // Passo 156I (ADR-0061 Fase 2 sub-passo 3) — stack compositivo.
        // Walk em cada child em ordem (counters/labels resolvem).
        Content::Stack { children, .. } => {
            for c in children.iter() {
                walk(c, state, locator, tags, None);
            }
        },

        // Passo 156J (ADR-0061 Fase 3 sub-passo 1) — repeat container.
        // Walk no body uma vez (counters/labels dentro de body
        // resolvem; semântica de repetição é runtime-only e não
        // multiplica state — vanilla repeat também só conta uma vez).
        Content::Repeat { body, .. } => walk(body, state, locator, tags, None),

        // Passo 99 (ADR-0038): `Styled` é transparente — desce no body.
        Content::Styled(body, _) => walk(body, state, locator, tags, None),

        Content::Outline => {
            state.has_outline = true;
            // Outline não altera contadores — apenas sinaliza que o fixpoint é necessário.
        }
    }

    // P162 .E: emissão Tag::End após recursão. Usa o mesmo Location
    // que o Tag::Start emitido no topo, e o hash determinístico do
    // conteúdo via hash_content.
    if let Some(loc) = emitted_loc {
        tags.push(Tag::End(loc, hash_content(content)));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::{
        content::Content,
        counter_state_legacy::CounterAction,
        element_payload::ElementPayload,
        label::Label,
        location::Location,
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
                    kind:      Some("image".to_string()),
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
                        kind:      Some("image".to_string()),
                        numbering: Some("1".to_string()),
                    }),
                },
                Content::Labelled {
                    label:  Label("f2".to_string()),
                    target: Box::new(Content::Figure {
                        body:      Box::new(Content::text("B")),
                        caption:   Some(Box::new(Content::text("Legenda B"))),
                        kind:      Some("image".to_string()),
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
                    kind:      Some("image".to_string()),
                    numbering: Some("1".to_string()),
                },
                Content::Labelled {
                    label:  Label("f2".to_string()),
                    target: Box::new(Content::Figure {
                        body:      Box::new(Content::text("B")),
                        caption:   Some(Box::new(Content::text("Legenda"))),
                        kind:      Some("image".to_string()),
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
        use crate::entities::counter_state_legacy::CounterStateLegacy;

        let mut state = CounterStateLegacy::new();
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
        use crate::entities::counter_state_legacy::CounterStateLegacy;

        let state = CounterStateLegacy::new();

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
            kind:      Some("image".to_string()),
            numbering: Some("1".to_string()),
        };
        if let Content::Figure { kind, numbering, .. } = fig {
            assert_eq!(kind.as_deref(), Some("image"));
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
                kind:      Some("image".to_string()),
                numbering: Some("1".to_string()),
            },
            Content::Figure {
                body:      Box::new(Content::text("tab1")),
                caption:   Some(Box::new(Content::text("cap2"))),
                kind:      Some("table".to_string()),
                numbering: Some("1".to_string()),
            },
            Content::Figure {
                body:      Box::new(Content::text("img2")),
                caption:   Some(Box::new(Content::text("cap3"))),
                kind:      Some("image".to_string()),
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

    // ── Passo 158B — Supplement automático por lang em figure ────────────

    /// Helper para construir state com lang explícito.
    fn introspect_with_lang(content: &Content, lang_code: &str) -> CounterStateLegacy {
        use std::str::FromStr;
        use crate::entities::lang::Lang;
        let mut state = CounterStateLegacy::new();
        state.lang = Some(Lang::from_str(lang_code).unwrap());
        let mut locator = Locator::new();
        let mut tags: Vec<Tag> = Vec::new();
        walk(content, &mut state, &mut locator, &mut tags, None);
        state
    }

    #[test]
    fn figure_label_default_no_lang_set_devolve_pt() {
        // P158B §8.2: lang None → fallback PT (backwards compat).
        use crate::entities::label::Label;
        let label = Label("fig1".to_string());
        let figure = Content::Figure {
            body: Box::new(Content::text("body")),
            caption: Some(Box::new(Content::text("caption"))),
            kind: Some("image".to_string()),
            numbering: Some("1".to_string()),
        };
        let labelled = Content::Labelled {
            target: Box::new(figure),
            label: label.clone(),
        };
        let state = introspect(&labelled);
        assert_eq!(
            state.resolved_labels.get(&label).map(String::as_str),
            Some("Figura 1"),
            "Default sem lang → PT 'Figura'"
        );
    }

    #[test]
    fn figure_label_lang_pt_image_devolve_figura() {
        use crate::entities::label::Label;
        let label = Label("fig1".to_string());
        let figure = Content::Figure {
            body: Box::new(Content::text("body")),
            caption: Some(Box::new(Content::text("caption"))),
            kind: Some("image".to_string()),
            numbering: Some("1".to_string()),
        };
        let labelled = Content::Labelled {
            target: Box::new(figure),
            label: label.clone(),
        };
        let state = introspect_with_lang(&labelled, "pt");
        assert_eq!(
            state.resolved_labels.get(&label).map(String::as_str),
            Some("Figura 1"),
        );
    }

    #[test]
    fn figure_label_lang_en_table_devolve_table() {
        use crate::entities::label::Label;
        let label = Label("tab1".to_string());
        let figure = Content::Figure {
            body: Box::new(Content::text("body")),
            caption: Some(Box::new(Content::text("caption"))),
            kind: Some("table".to_string()),
            numbering: Some("1".to_string()),
        };
        let labelled = Content::Labelled {
            target: Box::new(figure),
            label: label.clone(),
        };
        let state = introspect_with_lang(&labelled, "en");
        assert_eq!(
            state.resolved_labels.get(&label).map(String::as_str),
            Some("Table 1"),
        );
    }

    #[test]
    fn figure_label_lang_de_raw_devolve_listing() {
        use crate::entities::label::Label;
        let label = Label("lst1".to_string());
        let figure = Content::Figure {
            body: Box::new(Content::text("body")),
            caption: Some(Box::new(Content::text("caption"))),
            kind: Some("raw".to_string()),
            numbering: Some("1".to_string()),
        };
        let labelled = Content::Labelled {
            target: Box::new(figure),
            label: label.clone(),
        };
        let state = introspect_with_lang(&labelled, "de");
        assert_eq!(
            state.resolved_labels.get(&label).map(String::as_str),
            Some("Listing 1"),
        );
    }

    #[test]
    fn figure_label_lang_unknown_fallback_pt() {
        // P158B §8.2: lang desconhecido (zh) → fallback PT.
        use crate::entities::label::Label;
        let label = Label("fig1".to_string());
        let figure = Content::Figure {
            body: Box::new(Content::text("body")),
            caption: Some(Box::new(Content::text("caption"))),
            kind: Some("image".to_string()),
            numbering: Some("1".to_string()),
        };
        let labelled = Content::Labelled {
            target: Box::new(figure),
            label: label.clone(),
        };
        let state = introspect_with_lang(&labelled, "zh");
        assert_eq!(
            state.resolved_labels.get(&label).map(String::as_str),
            Some("Figura 1"),
            "Lang desconhecido cai no fallback PT"
        );
    }

    #[test]
    fn figure_label_kind_custom_devolve_capitalizado() {
        // P158B §6: kind desconhecido devolve string capitalizada.
        use crate::entities::label::Label;
        let label = Label("custom1".to_string());
        let figure = Content::Figure {
            body: Box::new(Content::text("body")),
            caption: Some(Box::new(Content::text("caption"))),
            kind: Some("custom".to_string()),
            numbering: Some("1".to_string()),
        };
        let labelled = Content::Labelled {
            target: Box::new(figure),
            label: label.clone(),
        };
        let state = introspect_with_lang(&labelled, "en");
        assert_eq!(
            state.resolved_labels.get(&label).map(String::as_str),
            Some("Custom 1"),
            "Kind desconhecido capitalizado"
        );
    }

    #[test]
    fn figure_counters_independentes_por_kind_continuam_a_funcionar_apos_p158b() {
        // Regression P157A/P158A: counters por kind continuam
        // independentes; supplement P158B não interfere.
        use std::sync::Arc;
        let content = Content::Sequence(Arc::from(vec![
            Content::Figure {
                body: Box::new(Content::text("body1")),
                caption: Some(Box::new(Content::text("c1"))),
                kind: Some("image".to_string()),
                numbering: Some("1".to_string()),
            },
            Content::Figure {
                body: Box::new(Content::text("body2")),
                caption: Some(Box::new(Content::text("c2"))),
                kind: Some("table".to_string()),
                numbering: Some("1".to_string()),
            },
            Content::Figure {
                body: Box::new(Content::text("body3")),
                caption: Some(Box::new(Content::text("c3"))),
                kind: Some("image".to_string()),
                numbering: Some("1".to_string()),
            },
        ]));
        let state = introspect(&content);
        assert_eq!(state.figure_numbers.get("image").cloned().unwrap_or_default(),
            vec![1, 2], "image counter independente");
        assert_eq!(state.figure_numbers.get("table").cloned().unwrap_or_default(),
            vec![1], "table counter independente");
    }

    // ── Passo 158C: Figure.kind = None resolve a "image" em uso ─────────

    #[test]
    fn introspect_figure_kind_none_resolve_para_image_no_counter() {
        // P158C: kind=None deve cair no default "image" via fallback
        // em counter (paridade backwards compat com tests pré-existentes
        // que usam Some("image".to_string())).
        let content = Content::Sequence(
            vec![
                Content::Labelled {
                    label:  Label("f_none".to_string()),
                    target: Box::new(Content::Figure {
                        body:      Box::new(Content::text("body sem kind explícito")),
                        caption:   Some(Box::new(Content::text("legenda"))),
                        kind:      None,  // P158C: novo caso
                        numbering: Some("1".to_string()),
                    }),
                },
            ]
            .into(),
        );
        let state = introspect(&content);
        // Counter "image" deve avançar via fallback default.
        assert_eq!(state.figure_numbers.get("image").cloned().unwrap_or_default(),
            vec![1],
            "kind=None deve cair no default 'image' no counter");
        // Label resolve para "Figura 1" via fallback (PT default em
        // figure_supplement_for_lang).
        assert_eq!(
            state.resolved_labels.get(&Label("f_none".to_string())).map(|s| s.as_str()),
            Some("Figura 1"),
            "label de figura kind=None deve resolver via fallback 'image' default"
        );
    }

    // ── P162 .G — Tests do walk com tags em paralelo ─────────────────────

    /// Helper de teste para correr walk e devolver state + tags.
    fn introspect_with_tags(content: &Content) -> (CounterStateLegacy, Vec<Tag>) {
        let mut state = CounterStateLegacy::new();
        let mut locator = Locator::new();
        let mut tags: Vec<Tag> = Vec::new();
        walk(content, &mut state, &mut locator, &mut tags, None);
        (state, tags)
    }

    #[test]
    fn walk_emite_start_e_end_para_heading() {
        let h = Content::heading(1, Content::text("title"));
        let (_, tags) = introspect_with_tags(&h);
        assert_eq!(tags.len(), 2, "heading deve emitir Tag::Start + Tag::End");
        let (start_loc, end_loc) = match (&tags[0], &tags[1]) {
            (Tag::Start(loc_s, _), Tag::End(loc_e, _)) => (*loc_s, *loc_e),
            _ => panic!("ordem esperada: Start, End; obtido {tags:?}"),
        };
        assert_eq!(start_loc, end_loc, "Location do Start e End devem coincidir");
    }

    #[test]
    fn walk_nao_emite_para_text_simples() {
        let t = Content::text("plain");
        let (_, tags) = introspect_with_tags(&t);
        assert!(tags.is_empty(), "text simples não deve emitir tags");
    }

    #[test]
    fn walk_aninha_start_end_para_heading_contendo_figure() {
        // Heading com Figure aninhada no body.
        let figure = Content::Figure {
            body:      Box::new(Content::Empty),
            caption:   Some(Box::new(Content::text("cap"))),
            kind:      Some("image".into()),
            numbering: Some("1".into()),
        };
        let h = Content::heading(1, figure);
        let (_, tags) = introspect_with_tags(&h);
        // Esperado: Start(Heading), Start(Figure), End(Figure), End(Heading) = 4 tags.
        assert_eq!(tags.len(), 4, "heading-com-figura deve emitir 4 tags, obtido {tags:?}");
        match (&tags[0], &tags[1], &tags[2], &tags[3]) {
            (Tag::Start(_, _), Tag::Start(_, _), Tag::End(_, _), Tag::End(_, _)) => {}
            other => panic!("ordem esperada: Start, Start, End, End; obtido {other:?}"),
        }
    }

    #[test]
    fn walk_emite_tags_em_paralelo_com_state() {
        // Verifica que tanto state quanto tags são populados.
        let content = Content::Sequence(
            vec![
                Content::SetHeadingNumbering { active: true },
                Content::heading(1, Content::text("um")),
                Content::heading(1, Content::text("dois")),
            ]
            .into(),
        );
        let (state, tags) = introspect_with_tags(&content);
        // State: contador heading deve estar em "2" após dois headings nivel 1.
        assert_eq!(state.format_hierarchical("heading").as_deref(), Some("2"),
            "state deve ter contador heading=2 após dois headings nível 1");
        // P182C: SetHeadingNumbering passou a ser locatable (emite
        // ElementPayload::StateUpdate sob chave numbering_active:heading).
        // Tags: 1 SetHeadingNumbering × 2 + 2 headings × 2 = 6.
        assert_eq!(tags.len(), 6, "deve haver Start+End para SetHeadingNumbering e cada heading; obtido {tags:?}");
    }

    #[test]
    fn walk_label_de_wrapper_chega_ao_payload() {
        // Content::Labelled { target: Heading } → tag Heading recebe Some(label).
        let content = Content::Labelled {
            label:  Label("intro".to_string()),
            target: Box::new(Content::heading(1, Content::text("Introdução"))),
        };
        let (_, tags) = introspect_with_tags(&content);
        // Esperado: Start(Heading) com label="intro", End(Heading).
        match &tags[0] {
            Tag::Start(_, info) => {
                assert_eq!(
                    info.label.as_ref().map(|l| l.0.as_str()),
                    Some("intro"),
                    "label do wrapper Labelled deve aparecer no ElementInfo do Heading"
                );
            }
            other => panic!("esperado Tag::Start, obtido {other:?}"),
        }
    }

    // ── P163 .C — Tests E2E de bracketing ────────────────────────────────

    /// Helper local: constrói Content com headings, figures, citations
    /// aninhados em diversas profundidades para tests E2E.
    fn make_content_complexo() -> Content {
        Content::Sequence(
            vec![
                Content::SetHeadingNumbering { active: true },
                Content::heading(1, Content::text("Capítulo")),
                Content::Figure {
                    body:      Box::new(Content::Empty),
                    caption:   Some(Box::new(Content::text("legenda"))),
                    kind:      Some("image".into()),
                    numbering: Some("1".into()),
                },
                Content::heading(2, Content::text("Secção")),
                Content::cite("smith2024", None, None),
            ]
            .into(),
        )
    }

    #[test]
    fn walk_e_deterministico() {
        // P163 .C.1: walk duas vezes sobre o mesmo Content produz
        // Vec<Tag> idêntico (mesma ordem, mesmas Locations, mesmos hashes).
        let content = make_content_complexo();
        let (_, tags1) = introspect_with_tags(&content);
        let (_, tags2) = introspect_with_tags(&content);
        assert_eq!(
            tags1, tags2,
            "walk não-determinístico: tags1.len={}, tags2.len={}",
            tags1.len(), tags2.len()
        );
    }

    #[test]
    fn bracketing_valido_em_aninhamento_complexo() {
        // P163 .C.2: heading ⊃ figure-com-caption-com-text ⊃ heading.
        // Verificar que cada Start tem o seu End correspondente, sem
        // overlapping. Headings emitem tags; figures aninhadas no
        // caption também.
        let inner_h = Content::heading(2, Content::text("inner"));
        let figure_with_h = Content::Figure {
            body:      Box::new(inner_h),
            caption:   Some(Box::new(Content::text("cap"))),
            kind:      Some("image".into()),
            numbering: Some("1".into()),
        };
        let outer_h = Content::heading(1, figure_with_h);
        let (_, tags) = introspect_with_tags(&outer_h);

        let mut stack: Vec<Location> = Vec::new();
        for tag in &tags {
            match tag {
                Tag::Start(loc, _) => stack.push(*loc),
                Tag::End(loc, _) => {
                    let top = stack.pop().expect("End sem Start correspondente");
                    assert_eq!(top, *loc, "End com Location diferente do último Start");
                }
            }
        }
        assert!(stack.is_empty(), "Start sem End correspondente; stack={stack:?}");
    }

    #[test]
    fn bracketing_valido_em_sequencia_plana() {
        // Caso adicional .C.2: múltiplos headings ao mesmo nível
        // (não aninhados) — bracketing também válido.
        let content = Content::Sequence(
            vec![
                Content::heading(1, Content::text("um")),
                Content::heading(1, Content::text("dois")),
                Content::heading(1, Content::text("três")),
            ]
            .into(),
        );
        let (_, tags) = introspect_with_tags(&content);

        let mut stack: Vec<Location> = Vec::new();
        for tag in &tags {
            match tag {
                Tag::Start(loc, _) => stack.push(*loc),
                Tag::End(loc, _) => {
                    let top = stack.pop().expect("End sem Start correspondente");
                    assert_eq!(top, *loc);
                }
            }
        }
        assert!(stack.is_empty());
        assert_eq!(tags.len(), 6, "3 headings × 2 tags = 6");
    }

    #[test]
    fn end_hash_distingue_conteudo() {
        // P163 .C.3: dois headings com bodies diferentes produzem
        // Tag::End com u128 distintos.
        let a = Content::heading(1, Content::text("Título A"));
        let b = Content::heading(1, Content::text("Título B"));
        let (_, tags_a) = introspect_with_tags(&a);
        let (_, tags_b) = introspect_with_tags(&b);

        let end_a = tags_a.iter().find_map(|t| match t {
            Tag::End(_, h) => Some(*h),
            _ => None,
        }).expect("nenhum Tag::End emitido para a");
        let end_b = tags_b.iter().find_map(|t| match t {
            Tag::End(_, h) => Some(*h),
            _ => None,
        }).expect("nenhum Tag::End emitido para b");

        assert_ne!(
            end_a, end_b,
            "Contents distintos produziram mesmo content_hash em Tag::End"
        );
    }

    // ── P163 .D — Tests de consistência por kind ─────────────────────────

    #[test]
    fn headings_capturados_em_paralelo() {
        // P163 .D.1: walk sobre Content com N headings em níveis
        // variados. Verificar:
        //  - número de Tag::Start(_, Heading{..}) bate com input;
        //  - depth de cada Heading bate com level esperado;
        //  - state.format_hierarchical("heading") fica no valor
        //    esperado após todos os headings (verificação cruzada).
        let levels = vec![1u8, 2, 2, 3];
        let content = Content::Sequence(
            std::iter::once(Content::SetHeadingNumbering { active: true })
                .chain(levels.iter().map(|&l| Content::heading(l, Content::text("h"))))
                .collect::<Vec<_>>()
                .into(),
        );
        let (state, tags) = introspect_with_tags(&content);

        let captured_levels: Vec<u8> = tags.iter()
            .filter_map(|t| match t {
                Tag::Start(_, info) => match &info.payload {
                    ElementPayload::Heading { depth, .. } => Some(*depth),
                    _ => None,
                },
                _ => None,
            })
            .collect();

        assert_eq!(
            captured_levels, levels,
            "depth dos heading payloads em tags difere dos levels do input"
        );

        // Verificação cruzada: state.format_hierarchical("heading")
        // depois de [1, 2, 2, 3] deve ser "1.1.1.1" — após
        // step ao nível 1, depois nível 2 (= [1,1]), nível 2 outra
        // vez (= [1,2]), depois nível 3 (= [1,2,1]).
        // Na verdade: walk usa step_hierarchical que avança o último
        // segmento ou empurra um novo nível. Com [1,2,2,3]:
        //  []     +1 → [1]
        //  [1]    +2 → [1, 1]
        //  [1,1]  +2 → [1, 2]
        //  [1,2]  +3 → [1, 2, 1]
        // → format = "1.2.1"
        assert_eq!(
            state.format_hierarchical("heading").as_deref(),
            Some("1.2.1"),
            "format_hierarchical não bate com sequência [1,2,2,3]"
        );
    }

    #[test]
    fn figures_capturadas_em_paralelo() {
        // P163 .D.2: walk sobre Content com 3 figures (kind variados).
        let content = Content::Sequence(
            vec![
                Content::Figure {
                    body:      Box::new(Content::Empty),
                    caption:   Some(Box::new(Content::text("c1"))),
                    kind:      Some("image".into()),
                    numbering: Some("1".into()),
                },
                Content::Figure {
                    body:      Box::new(Content::Empty),
                    caption:   Some(Box::new(Content::text("c2"))),
                    kind:      Some("table".into()),
                    numbering: Some("1".into()),
                },
                Content::Figure {
                    body:      Box::new(Content::Empty),
                    caption:   Some(Box::new(Content::text("c3"))),
                    kind:      None,
                    numbering: Some("1".into()),
                },
            ]
            .into(),
        );
        let (state, tags) = introspect_with_tags(&content);

        let captured_kinds: Vec<Option<String>> = tags.iter()
            .filter_map(|t| match t {
                Tag::Start(_, info) => match &info.payload {
                    ElementPayload::Figure { kind, .. } => Some(kind.clone()),
                    _ => None,
                },
                _ => None,
            })
            .collect();

        assert_eq!(
            captured_kinds,
            vec![Some("image".to_string()), Some("table".to_string()), None],
            "kinds das figures em tags não batem com input"
        );

        // Verificação cruzada: state.figure_numbers populado por kind.
        // Walk arm Figure resolve kind=None para "image" (default
        // P158C). Logo: 2 figures contam para "image" (kind=Some
        // e kind=None) e 1 para "table".
        // Tags, em contraste, preservam kind=None literal — divergência
        // documentada (ver .E lacunas-captura).
        assert_eq!(
            state.figure_numbers.get("image").cloned().unwrap_or_default().len(),
            2,
            "image figure_numbers count: kind=Some(image) + kind=None"
        );
        assert_eq!(
            state.figure_numbers.get("table").cloned().unwrap_or_default().len(),
            1,
            "table figure_numbers count"
        );
    }

    #[test]
    fn citations_capturadas_em_paralelo() {
        // P163 .D.3: walk sobre Content com 3 citations distintas.
        let content = Content::Sequence(
            vec![
                Content::cite("smith2024", None, None),
                Content::cite("jones2023", None, None),
                Content::cite("smith2024", None, None),  // repetida
            ]
            .into(),
        );
        let (_, tags) = introspect_with_tags(&content);

        let captured_keys: Vec<String> = tags.iter()
            .filter_map(|t| match t {
                Tag::Start(_, info) => match &info.payload {
                    ElementPayload::Citation { key } => Some(key.clone()),
                    _ => None,
                },
                _ => None,
            })
            .collect();

        assert_eq!(
            captured_keys,
            vec!["smith2024".to_string(), "jones2023".to_string(), "smith2024".to_string()],
            "keys das citations em tags não batem com input (incluindo repetição)"
        );

        // Verificação: 3 citations × 2 tags cada = 6 (Start + End por citation).
        let citation_tags: Vec<&Tag> = tags.iter()
            .filter(|t| match t {
                Tag::Start(_, info) => matches!(info.payload, ElementPayload::Citation { .. }),
                Tag::End(_, _) => true, // End não tem payload mas todos os Ends correspondem a Citations aqui
            })
            .collect();
        assert_eq!(citation_tags.len(), 6, "3 citations × 2 tags = 6, obtido {}", citation_tags.len());
    }

    // ── P165 .G — Tests E2E paralelo CounterStateLegacy + Introspector ───

    use crate::entities::element_kind::ElementKind;
    use crate::entities::introspector::{Introspector, TagIntrospector};
    use crate::rules::introspect::from_tags::from_tags;

    /// Helper de teste para correr walk + construir Introspector em paralelo.
    /// **P173**: passa `None, None` — testes locais não exercitam Funcs.
    fn introspect_with_introspector(content: &Content) -> (CounterStateLegacy, TagIntrospector) {
        let mut state = CounterStateLegacy::new();
        let mut locator = Locator::new();
        let mut tags: Vec<Tag> = Vec::new();
        walk(content, &mut state, &mut locator, &mut tags, None);
        let intr = from_tags(&tags, None, None);
        (state, intr)
    }

    #[test]
    fn introspector_consistencia_heading() {
        // P165 .G.1: walk com headings em níveis [1, 2, 2, 3] →
        // Introspector.kind_index[Heading] tem 4 locations.
        // CounterStateLegacy.format_hierarchical("heading") confirma
        // mesma contagem.
        //
        // **P170 (M9 sub-passo 2)**: paridade extendida — Introspector
        // agora também produz "1.2.1" via formatted_counter (resolve
        // lacuna #5).
        let levels = vec![1u8, 2, 2, 3];
        let content = Content::Sequence(
            std::iter::once(Content::SetHeadingNumbering { active: true })
                .chain(levels.iter().map(|&l| Content::heading(l, Content::text("h"))))
                .collect::<Vec<_>>()
                .into(),
        );
        let (state, intr) = introspect_with_introspector(&content);
        // Introspector tem 4 headings indexados.
        assert_eq!(intr.query_by_kind(ElementKind::Heading).len(), 4);
        // CounterStateLegacy tem hierarchical "1.2.1" após [1,2,2,3]
        // (verificado em P163 .D.1).
        assert_eq!(
            state.format_hierarchical("heading").as_deref(),
            Some("1.2.1")
        );
        // P170: Introspector tem mesma string via formatted_counter.
        assert_eq!(
            intr.formatted_counter("heading").as_deref(),
            Some("1.2.1"),
            "P170: paridade entre legacy.format_hierarchical e \
             introspector.formatted_counter"
        );
    }

    #[test]
    fn introspector_consistencia_figure() {
        // P165 .G.2: 3 figures (kind variados) → introspector indexa 3.
        let content = Content::Sequence(
            vec![
                Content::Figure {
                    body: Box::new(Content::Empty), caption: Some(Box::new(Content::text("c1"))),
                    kind: Some("image".into()), numbering: Some("1".into()),
                },
                Content::Figure {
                    body: Box::new(Content::Empty), caption: Some(Box::new(Content::text("c2"))),
                    kind: Some("table".into()), numbering: Some("1".into()),
                },
                Content::Figure {
                    body: Box::new(Content::Empty), caption: Some(Box::new(Content::text("c3"))),
                    kind: None, numbering: Some("1".into()),
                },
            ]
            .into(),
        );
        let (state, intr) = introspect_with_introspector(&content);
        assert_eq!(intr.query_by_kind(ElementKind::Figure).len(), 3);
        // CounterStateLegacy resolve kind=None para "image" → image=2, table=1.
        // Introspector preserva kind literal mas conta as 3 sob ElementKind::Figure.
        // Divergência conhecida (m1-lacunas-captura.md #1).
        assert_eq!(state.figure_numbers.get("image").map(|v| v.len()).unwrap_or(0), 2);
        assert_eq!(state.figure_numbers.get("table").map(|v| v.len()).unwrap_or(0), 1);
    }

    #[test]
    fn introspector_consistencia_citation() {
        // P165 .G.3: 3 citations distintas com keys → introspector indexa 3.
        let content = Content::Sequence(
            vec![
                Content::cite("smith2024", None, None),
                Content::cite("jones2023", None, None),
                Content::cite("brown2022", None, None),
            ]
            .into(),
        );
        let (_, intr) = introspect_with_introspector(&content);
        let locs = intr.query_by_kind(ElementKind::Citation);
        assert_eq!(locs.len(), 3);
    }

    // ── P182C — pipeline E2E SetHeadingNumbering → StateRegistry ─────────

    #[test]
    fn introspector_set_heading_numbering_active_true_popula_state_registry() {
        // P182C: walk emite tag para Content::SetHeadingNumbering;
        // extract_payload produz ElementPayload::StateUpdate;
        // from_tags arm StateUpdate popula StateRegistry; trait method
        // is_numbering_active retorna true para chave canónica.
        let content = Content::SetHeadingNumbering { active: true };
        let (_, intr) = introspect_with_introspector(&content);
        assert!(
            intr.is_numbering_active("numbering_active:heading"),
            "P182C: pipeline deve popular StateRegistry com Bool(true)"
        );
    }

    #[test]
    fn introspector_set_heading_numbering_active_false_em_state_registry() {
        // Caso simétrico — Bool(false) é registado e propagado;
        // is_numbering_active devolve false (não por estar ausente,
        // mas porque o valor explícito é Bool(false)).
        let content = Content::SetHeadingNumbering { active: false };
        let (_, intr) = introspect_with_introspector(&content);
        assert!(!intr.is_numbering_active("numbering_active:heading"));
    }

    #[test]
    fn introspector_query_by_label() {
        // P165 .G.4: walk com Heading labelled → query_by_label retorna location;
        // mesma location aparece em query_by_kind(Heading).
        let content = Content::Labelled {
            label:  Label("intro".to_string()),
            target: Box::new(Content::heading(1, Content::text("Introdução"))),
        };
        let (_, intr) = introspect_with_introspector(&content);

        let by_label = intr.query_by_label(&Label("intro".to_string()));
        assert!(by_label.is_some(), "label intro deveria ter sido indexada");
        let by_kind = intr.query_by_kind(ElementKind::Heading);
        assert_eq!(by_kind.len(), 1);
        assert_eq!(by_kind.first().copied(), by_label, "location deve coincidir");
    }

    #[test]
    fn introspector_query_first_e_query_unique() {
        // P165 .G.5: walk com 1 Figure → query_first e query_unique retornam Some.
        // walk com 2 Figures → query_first retorna Some(loc1), query_unique None.
        let single_figure = Content::Figure {
            body: Box::new(Content::Empty), caption: Some(Box::new(Content::text("c"))),
            kind: Some("image".into()), numbering: Some("1".into()),
        };
        let (_, intr1) = introspect_with_introspector(&single_figure);
        assert!(intr1.query_first(ElementKind::Figure).is_some());
        assert!(intr1.query_unique(ElementKind::Figure).is_some());
        assert_eq!(
            intr1.query_first(ElementKind::Figure),
            intr1.query_unique(ElementKind::Figure)
        );

        let two_figures = Content::Sequence(
            vec![
                Content::Figure {
                    body: Box::new(Content::Empty), caption: Some(Box::new(Content::text("c1"))),
                    kind: Some("image".into()), numbering: Some("1".into()),
                },
                Content::Figure {
                    body: Box::new(Content::Empty), caption: Some(Box::new(Content::text("c2"))),
                    kind: Some("table".into()), numbering: Some("1".into()),
                },
            ]
            .into(),
        );
        let (_, intr2) = introspect_with_introspector(&two_figures);
        let first = intr2.query_first(ElementKind::Figure);
        assert!(first.is_some(), "query_first deve devolver primeira location");
        assert_eq!(intr2.query_unique(ElementKind::Figure), None,
            "query_unique deve devolver None com 2 figures");
    }

    // ── P166 .C — Tests da exposição pública do TagIntrospector ──────────

    #[test]
    fn introspect_with_introspector_devolve_par() {
        // P166 .C.1: novo entry point retorna tuple (state, introspector).
        // Verificar que introspector tem 1 heading indexado.
        let content = Content::heading(1, Content::text("título"));
        let (_, intr) = introspect_with_introspector(&content);
        assert!(
            intr.query_first(ElementKind::Heading).is_some(),
            "introspector exposto deve ter o heading indexado"
        );
    }

    #[test]
    fn introspect_legacy_continua_a_funcionar() {
        // P166 .C.3 (backward compat): call-site antigo
        // `let state = introspect(&c)` continua a compilar e funcionar.
        // Esta é a invariante crítica de M4b — wrapper preserva API.
        let content = Content::Sequence(
            vec![
                Content::SetHeadingNumbering { active: true },
                Content::heading(1, Content::text("um")),
                Content::heading(1, Content::text("dois")),
            ]
            .into(),
        );
        let state = introspect(&content);
        assert_eq!(
            state.format_hierarchical("heading").as_deref(),
            Some("2"),
            "wrapper introspect() deve produzir mesmo state que antes de M4"
        );
    }

    #[test]
    fn introspect_e_introspect_with_introspector_produzem_mesmo_state() {
        // P166 .C.2: state retornado pelo wrapper e pelo entry point novo
        // deve ser idêntico (walk único subjacente; wrapper só descarta
        // o introspector).
        let content = Content::Sequence(
            vec![
                Content::heading(1, Content::text("a")),
                Content::heading(2, Content::text("b")),
                Content::Figure {
                    body:      Box::new(Content::Empty),
                    caption:   Some(Box::new(Content::text("cap"))),
                    kind:      Some("image".into()),
                    numbering: Some("1".into()),
                },
            ]
            .into(),
        );
        let state_legacy = introspect(&content);
        let (state_new, _) = introspect_with_introspector(&content);

        // Comparação por campos relevantes (CounterStateLegacy não
        // implementa PartialEq globalmente; comparar via API pública).
        assert_eq!(
            state_legacy.format_hierarchical("heading").as_deref(),
            state_new.format_hierarchical("heading").as_deref(),
        );
        assert_eq!(
            state_legacy.figure_numbers.get("image"),
            state_new.figure_numbers.get("image"),
        );
        assert_eq!(
            state_legacy.resolved_labels.len(),
            state_new.resolved_labels.len(),
        );
    }

    // ── P173 (M9 sub-passo 5) — E2E cascade Engine através da API pública ─

    use crate::contracts::world::World as _;
    use crate::entities::args::Args;
    use crate::entities::engine::Engine;
    use crate::entities::file_id::FileId;
    use crate::entities::font_book::FontBook;
    use crate::entities::func::Func;
    use crate::entities::show::{RuleId, ShowRule};
    use crate::entities::sink::Sink;
    use crate::entities::state_update::StateUpdate;
    use crate::entities::style_chain::StyleChain;
    use crate::entities::value::Value;
    use crate::entities::world_types::{
        Bytes, Datetime, FileError, FileResult, Font, Library, Route,
    };
    use crate::rules::eval::EvalContext;
    use std::num::NonZeroU16;
    use std::sync::Arc;

    struct E2EWorld {
        library: Library,
        book:    FontBook,
        main_id: FileId,
    }
    impl crate::contracts::world::World for E2EWorld {
        fn library(&self) -> &Library { &self.library }
        fn book(&self)    -> &FontBook { &self.book }
        fn main(&self)    -> FileId { self.main_id }
        fn source(&self, _: FileId) -> FileResult<crate::entities::source::Source>
        { Err(FileError::NotFound) }
        fn file(&self, _: FileId) -> FileResult<Bytes>
        { Err(FileError::NotFound) }
        fn font(&self, _: usize) -> Option<Font> { None }
        fn today(&self, _: Option<i64>) -> Option<Datetime> { None }
    }
    fn make_e2e_world() -> E2EWorld {
        E2EWorld {
            library: Library::new(),
            book:    FontBook::new(),
            main_id: FileId::from_raw(NonZeroU16::new(1).unwrap()),
        }
    }
    fn add_one_native(
        _ctx: &mut EvalContext,
        args: &Args,
        _world: &dyn crate::contracts::world::World,
        _current_file: FileId,
        _figure_numbering: Option<&str>,
    ) -> crate::entities::source_result::SourceResult<Value> {
        match args.items.first() {
            Some(Value::Int(n)) => Ok(Value::Int(n + 1)),
            _ => Ok(Value::None),
        }
    }

    macro_rules! with_engine {
        ($world:expr, |$engine:ident, $ctx:ident| $body:block) => {{
            use comemo::Track;
            let world: &dyn crate::contracts::world::World = $world;
            let mut $ctx = EvalContext::new();
            let route = Route::root().with_id(world.main());
            let mut styles = StyleChain::default_chain();
            let mut show_rules: Arc<[ShowRule]> = Arc::from([]);
            let mut active_guards: Vec<RuleId> = Vec::new();
            let current_file = world.main();
            let mut figure_numbering: Option<String> = None;
            let mut sink_local = Sink::new();
            let mut sink = sink_local.track_mut();
            let mut $engine = Engine {
                world,
                route: route.track(),
                styles: &mut styles,
                show_rules: &mut show_rules,
                active_guards: &mut active_guards,
                current_file,
                figure_numbering: &mut figure_numbering,
                sink: &mut sink,
            };
            $body
        }};
    }

    #[test]
    fn p173_cascade_engine_via_api_publica() {
        // E2E: estado de count via API pública. Cascade Engine →
        // from_tags → apply_func executa Func real.
        let f = Func::native("add_one", add_one_native);
        let content = Content::Sequence(
            vec![
                Content::State {
                    key:  "c".to_string(),
                    init: Box::new(Value::Int(0)),
                },
                Content::StateUpdate {
                    key:    "c".to_string(),
                    update: StateUpdate::Func(f),
                },
            ]
            .into(),
        );
        let world = make_e2e_world();
        let intr = with_engine!(&world, |engine, ctx| {
            let (_, intr) = super::introspect_with_introspector(
                &content, Some(&mut engine), Some(&mut ctx),
            );
            intr
        });
        assert_eq!(intr.state_final_value("c"), Some(&Value::Int(1)));
    }

    #[test]
    fn p173_introspect_legacy_ignora_func() {
        // E2E: API legacy `introspect()` (sem Engine) continua a funcionar.
        // Funcs em state.update são silenciosamente ignoradas.
        let f = Func::native("add_one", add_one_native);
        let content = Content::Sequence(
            vec![
                Content::State {
                    key:  "c".to_string(),
                    init: Box::new(Value::Int(0)),
                },
                Content::StateUpdate {
                    key:    "c".to_string(),
                    update: StateUpdate::Func(f),
                },
            ]
            .into(),
        );
        // Path legacy: walk + from_tags(_, None, None).
        let (_, intr) = super::introspect_with_introspector(&content, None, None);
        // Func ignorada → final value continua em init.
        assert_eq!(intr.state_final_value("c"), Some(&Value::Int(0)));
    }

    #[test]
    fn p173_determinismo_func_eval() {
        // E2E: dois chamadas com Engine produzem mesmo resultado.
        let f1 = Func::native("add_one", add_one_native);
        let f2 = Func::native("add_one", add_one_native);
        let content_a = Content::Sequence(
            vec![
                Content::State {
                    key:  "c".to_string(),
                    init: Box::new(Value::Int(5)),
                },
                Content::StateUpdate {
                    key:    "c".to_string(),
                    update: StateUpdate::Func(f1),
                },
            ]
            .into(),
        );
        let content_b = Content::Sequence(
            vec![
                Content::State {
                    key:  "c".to_string(),
                    init: Box::new(Value::Int(5)),
                },
                Content::StateUpdate {
                    key:    "c".to_string(),
                    update: StateUpdate::Func(f2),
                },
            ]
            .into(),
        );
        let world = make_e2e_world();
        let v_a = with_engine!(&world, |engine, ctx| {
            let (_, intr) = super::introspect_with_introspector(
                &content_a, Some(&mut engine), Some(&mut ctx),
            );
            intr.state_final_value("c").cloned()
        });
        let v_b = with_engine!(&world, |engine, ctx| {
            let (_, intr) = super::introspect_with_introspector(
                &content_b, Some(&mut engine), Some(&mut ctx),
            );
            intr.state_final_value("c").cloned()
        });
        assert_eq!(v_a, Some(Value::Int(6)));
        assert_eq!(v_a, v_b);
    }

    // ── P181H — Walk arm Bibliography puro (P163 invariante restaurada) ─

    #[test]
    fn walk_arm_bibliography_nao_muta_state_bib_legacy() {
        // P181H: walk arm `Content::Bibliography` não muta directamente
        // `state.bib_entries` ou `state.bib_numbers`. Tag emitida via
        // extract_payload do topo de walk (P181D); BibStore populado
        // por from_tags arm (P181E). Walk puro restaurado para bib —
        // invariante P163 preservada.
        use crate::entities::bib_entry::BibEntry;
        use crate::entities::content::Content;

        let content = Content::Bibliography {
            entries: vec![
                BibEntry::new("a", "Author A", "Title A", 2024),
                BibEntry::new("b", "Author B", "Title B", 2025),
            ],
            title: None,
        };

        let mut state = CounterStateLegacy::new();
        let mut locator = Locator::new();
        let mut tags: Vec<Tag> = Vec::new();
        walk(&content, &mut state, &mut locator, &mut tags, None);

        // Walk arm puro: state.bib_* não populado.
        assert!(state.bib_entries.is_empty(),
            "P181H walk puro: state.bib_entries deve ficar vazio (era populado por walk arm pré-P181H)");
        assert!(state.bib_numbers.is_empty(),
            "P181H walk puro: state.bib_numbers deve ficar vazio (era populado por walk arm pré-P181H)");

        // Tag emitida pelo topo via extract_payload (P181D): existe
        // exactamente uma Tag::Start de Bibliography.
        use crate::entities::element_payload::ElementPayload;
        let bib_tags: Vec<_> = tags.iter().filter(|t| matches!(
            t,
            Tag::Start(_, info) if matches!(info.payload, ElementPayload::Bibliography { .. })
        )).collect();
        assert_eq!(bib_tags.len(), 1,
            "tag Bibliography deve ser emitida via extract_payload mesmo com walk puro");
    }

    #[test]
    fn walk_arm_bibliography_desce_em_title() {
        // P181H: walk arm puro continua a descer no `title` (preserva
        // comportamento legacy para que children dentro de title
        // sejam visíveis a outros consumers).
        use crate::entities::bib_entry::BibEntry;
        use crate::entities::content::Content;
        use crate::entities::label::Label;

        let titulo = Content::Labelled {
            target: Box::new(Content::Heading {
                level: 1,
                body:  Box::new(Content::Empty),
            }),
            label: Label("bib-title".to_string()),
        };

        let content = Content::Bibliography {
            entries: vec![BibEntry::new("a", "A", "T", 2024)],
            title:   Some(Box::new(titulo)),
        };

        let mut state = CounterStateLegacy::new();
        let mut locator = Locator::new();
        let mut tags: Vec<Tag> = Vec::new();
        walk(&content, &mut state, &mut locator, &mut tags, None);

        // Heading dentro de title produz Tag de Heading.
        use crate::entities::element_payload::ElementPayload;
        let heading_tags: Vec<_> = tags.iter().filter(|t| matches!(
            t,
            Tag::Start(_, info) if matches!(info.payload, ElementPayload::Heading { .. })
        )).collect();
        assert_eq!(heading_tags.len(), 1,
            "walk deve descer em Bibliography.title — Heading interno deve produzir Tag");
    }
}
