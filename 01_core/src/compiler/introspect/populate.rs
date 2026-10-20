//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/introspect/populate.md
//! @prompt-hash f1ab504f
//! @layer L1
//! @updated 2026-09-30

use super::*;

/// **P191B (ADR-0071)** — populate `TagIntrospector` sub-stores a partir
/// de uma `Tag::Start` emitida pelo walk. Substitui o match exhaustivo
/// que vivia em `from_tags::from_tags` (eliminado em P191B). Walk arms
/// chamam este helper imediatamente antes de `tags.push(Tag::Start(...))`
/// para que sub-stores fiquem populated em ordem location-monotónica.
///
/// **`StateUpdate::Func` excepção**: Funcs requerem `Engine +
/// EvalContext` para `apply_func` — não disponíveis em walk. Defer
/// para `apply_state_funcs` (post-pass slim chamado em `fixpoint`).
pub(super) fn populate_intr_from_tag_start(
    intr: &mut TagIntrospector,
    info: &ElementInfo,
    loc: Location,
) {
    if let Some(label) = &info.label {
        intr.labels.add(label.clone(), loc);
    }
    let first_label = info
        .label
        .as_ref()
        .filter(|label| intr.labels.lookup(label) == Some(loc));
    // Guardar a ação efetiva, antes de materializar snapshots. Escritas
    // auxiliares (figure:kind) não são novas ocorrências automáticas.
    let automatic = match &info.payload {
        ElementPayload::NativeElement
        | ElementPayload::Footnote { .. }
        | ElementPayload::Citation { .. } => Some(CounterUpdate::step()),
        ElementPayload::Heading { depth, numbering_active: true, .. } => {
            Some(CounterUpdate::Step(
                std::num::NonZeroUsize::new((*depth as usize).max(1)).unwrap(),
            ))
        }
        ElementPayload::Figure { is_counted: true, counter_update, .. }
        | ElementPayload::Table { is_counted: true, counter_update, .. } => {
            Some(counter_update.clone())
        }
        ElementPayload::Equation { block: true, numbering_active: true, .. } => {
            Some(CounterUpdate::step())
        }
        ElementPayload::Metadata { .. }
        | ElementPayload::State { .. }
        | ElementPayload::StateUpdate { .. }
        | ElementPayload::StateDisplay { .. }
        | ElementPayload::CounterDisplay { .. }
        | ElementPayload::Outline
        | ElementPayload::Bibliography { .. }
        | ElementPayload::Labelled { .. }
        | ElementPayload::CounterUpdate { .. }
        | ElementPayload::HeadingForToc { .. }
        | ElementPayload::HeadingForBookmarks { .. }
        | ElementPayload::ContextBlock { .. }
        | ElementPayload::Heading { numbering_active: false, .. }
        | ElementPayload::Figure { is_counted: false, .. }
        | ElementPayload::Table { is_counted: false, .. }
        | ElementPayload::Equation { block: false, .. }
        | ElementPayload::Equation { block: true, numbering_active: false, .. } => None,
    };
    if let Some(action) = automatic {
        intr.counters.record_action(loc, None, action);
    }
    if let ElementPayload::CounterUpdate { key, action } = &info.payload {
        intr.counters.record_action(loc, Some(key.clone()), action.clone());
    }
    match &info.payload {
        ElementPayload::NativeElement => {}
        ElementPayload::Heading { depth, numbering_active, .. } => {
            intr.kind_index.entry(ElementKind::Heading).or_default().push(loc);
            // **P1019** — o counter de heading só avança quando `numbering_active`
            // (já assado na chain no walk top). O heading continua locatable/indexado
            // mesmo sem numeração; só o counter fica dormente.
            if *numbering_active {
                intr.counters.apply_hierarchical_at(
                    CounterKey::Selector(Selector::Kind(ElementKind::Heading)),
                    *depth as usize,
                    loc,
                );
            }
            // P788 — flag de numbering por Location (alimenta o erro
            // vanilla `cannot reference heading without numbering`).
            intr.heading_numbering.insert(loc, *numbering_active);
            if let Some(label) = first_label {
                intr.label_to_counter_key.insert(
                    label.clone(),
                    CounterKey::Selector(Selector::Kind(ElementKind::Heading)),
                );
            }
        }
        ElementPayload::Figure {
            kind, counter_update, is_counted, caption_text, ..
        } => {
            intr.kind_index.entry(ElementKind::Figure).or_default().push(loc);
            if *is_counted {
                let figure_key =
                    CounterKey::Selector(Selector::Kind(ElementKind::Figure));
                intr.counters
                    .apply_at(figure_key.clone(), counter_update.clone(), loc);
                if let Some(cap) = caption_text {
                    let num = intr
                        .counters
                        .value_at(&figure_key, loc)
                        .and_then(|v| v.last().copied())
                        .unwrap_or(0);
                    intr.figures_for_lof.push((num, cap.clone()));
                }
                if let Some(label) = first_label {
                    let resolved = kind.clone().unwrap_or_else(|| {
                        crate::entities::elements::figure::FigureKind::Element(
                            crate::entities::func::Func::native(
                                "image",
                                crate::compiler::stdlib::native_image,
                            ),
                        )
                    });
                    intr.label_to_counter_key.insert(
                        label.clone(),
                        super::figure_counters::figure_counter_key(&resolved),
                    );
                }
            }
        }
        ElementPayload::Citation { key } => {
            intr.kind_index.entry(ElementKind::Citation).or_default().push(loc);
            // **P468** — numeração de citações por ordem de aparição.
            intr.counters.apply_at(
                CounterKey::Str("citation".into()),
                crate::entities::counter_update::CounterUpdate::step(),
                loc,
            );
            intr.bib_store.record_citation(key.as_str().to_string());
        }
        ElementPayload::Metadata { value } => {
            intr.kind_index.entry(ElementKind::Metadata).or_default().push(loc);
            intr.metadata.add((**value).clone());
        }
        ElementPayload::State { key, init } => {
            intr.kind_index.entry(ElementKind::State).or_default().push(loc);
            intr.state.init(key.clone(), (**init).clone(), loc);
        }
        ElementPayload::Outline => {
            intr.kind_index.entry(ElementKind::Outline).or_default().push(loc);
        }
        ElementPayload::Bibliography { entries } => {
            intr.kind_index
                .entry(ElementKind::Bibliography)
                .or_default()
                .push(loc);
            let entries_owned = entries.clone();
            for entry in &entries_owned {
                let next_num = intr.bib_store.numbers_len() as u32 + 1;
                intr.bib_store.assign_number(entry.key.clone(), next_num);
            }
            intr.bib_store.add_bibliography(entries_owned);
        }
        ElementPayload::StateUpdate { key, update } => {
            intr.kind_index.entry(ElementKind::StateUpdate).or_default().push(loc);
            intr.state.record_action(key, update, loc);
        }
        ElementPayload::StateDisplay { .. } => {
            // P240 (M9d/M7+1): kind_index registo apenas em walk; valor
            // pre-rendered é produzido em `apply_state_displays` pós-walk
            // (paralelo `apply_state_funcs` — requer Engine+ctx).
            intr.kind_index
                .entry(ElementKind::StateDisplay)
                .or_default()
                .push(loc);
        }
        ElementPayload::CounterDisplay { .. } => {
            // P241 (M9d/M7+2): paralelo absoluto StateDisplay; valor
            // pre-rendered em `apply_counter_displays` pós-walk.
            intr.kind_index
                .entry(ElementKind::CounterDisplay)
                .or_default()
                .push(loc);
        }
        ElementPayload::Equation {
            block,
            counter_update,
            numbering_active,
            numbering_pattern,
            numbering_callback,
            supplement,
            supplement_lang,
            number_align,
            alt,
        } => {
            intr.kind_index.entry(ElementKind::Equation).or_default().push(loc);
            // P856 — flag de numbering por Location, análoga a heading_numbering.
            intr.equation_numbering.insert(loc, *numbering_active);
            let numbering = numbering_callback
                .clone()
                .map(Value::Func)
                .or_else(|| numbering_pattern.clone().map(Value::Str))
                .unwrap_or(Value::None);
            intr.equation_numberings.insert(loc, numbering);
            if let Some(pat) = numbering_pattern {
                intr.equation_numbering_pattern.insert(loc, pat.clone());
            }
            if let Some(callback) = numbering_callback {
                intr.equation_numbering_callbacks.insert(loc, callback.clone());
            }
            intr.equation_supplements.insert(loc, supplement.clone());
            intr.equation_supplement_langs.insert(loc, *supplement_lang);
            intr.equation_number_aligns.insert(loc, *number_align);
            intr.equation_alts.insert(
                loc,
                match alt {
                    Value::Str(text) => Some(text.clone()),
                    _ => None,
                },
            );
            // Lote F-2 S2 (P335): gate pelo `numbering_active` **assado** no
            // `EquationElem` (escopo léxico via chain) — não mais pelo
            // StateRegistry `numbering_active:equation` (canal global retirado).
            if *block && *numbering_active {
                intr.counters.apply_at(
                    CounterKey::Selector(Selector::Kind(ElementKind::Equation)),
                    counter_update.clone(),
                    loc,
                );
                if let Some(label) = first_label {
                    intr.label_to_counter_key.insert(
                        label.clone(),
                        CounterKey::Selector(Selector::Kind(ElementKind::Equation)),
                    );
                }
            }
            // P856 — equation com label mas sem numbering: regista como não
            // referenciável para emitir erro paridade vanilla.
            if *block && !*numbering_active {
                if let Some(label) = first_label {
                    intr.unreferencable_labels.insert(
                        label.clone(),
                        UnreferencableKind::EquationWithoutNumbering,
                    );
                }
            }
        }
        ElementPayload::Labelled { label, resolved_text, figure_number } => {
            if let Some(text) = resolved_text {
                intr.resolved_labels.insert(label.clone(), text.clone());
            }
            if let Some(n) = figure_number {
                intr.figure_label_numbers.insert(label.clone(), *n);
            }
            // P788: o `label_to_counter_key.remove(label)` que aqui existia
            // forçava o caminho legacy mesmo para elementos numerados —
            // removido; o caminho numérico tem precedência (o legacy fica
            // como fallback em `resolve_ref_text`).
        }
        ElementPayload::Table { counter_update, is_counted, caption_text } => {
            intr.kind_index.entry(ElementKind::Table).or_default().push(loc);
            // P461: counter "table" avança só quando caption + numbering.
            if *is_counted {
                let table_key = CounterKey::Selector(Selector::Kind(ElementKind::Table));
                intr.counters.apply_at(table_key.clone(), counter_update.clone(), loc);
                // **P472** — popular tables_for_lot com (número, caption).
                let num = intr
                    .counters
                    .value_at(&table_key, loc)
                    .and_then(|v| v.last().copied())
                    .unwrap_or(0);
                if let Some(cap) = caption_text {
                    intr.tables_for_lot.push((num, cap.clone()));
                }
                if let Some(label) = first_label {
                    intr.label_to_counter_key.insert(label.clone(), table_key);
                }
            }
        }
        ElementPayload::Footnote { counter_update } => {
            intr.kind_index.entry(ElementKind::Footnote).or_default().push(loc);
            // P1016: sem gate — toda a nota conta (o vanilla numera todas).
            let footnote_key =
                CounterKey::Selector(Selector::Kind(ElementKind::Footnote));
            intr.counters
                .apply_at(footnote_key.clone(), counter_update.clone(), loc);
            if let Some(label) = first_label {
                intr.label_to_counter_key.insert(label.clone(), footnote_key);
            }
        }
        ElementPayload::CounterUpdate { key, action } => {
            intr.kind_index
                .entry(ElementKind::CounterUpdate)
                .or_default()
                .push(loc);
            match action {
                CounterUpdate::Step(level) => {
                    intr.counters.apply_at(key.clone(), CounterUpdate::Step(*level), loc)
                }
                CounterUpdate::Set(values) => intr.counters.apply_at(
                    key.clone(),
                    CounterUpdate::Set(values.clone()),
                    loc,
                ),
                CounterUpdate::Func(_) => {
                    // Avaliada no post-pass que possui Engine + EvalContext.
                }
            }
        }
        ElementPayload::HeadingForToc { label, number, body, level } => {
            intr.headings_for_toc.push((
                label.clone(),
                number.clone(),
                body.clone(),
                *level,
            ));
        }
        ElementPayload::HeadingForBookmarks { label, number, body, level } => {
            intr.headings_for_bookmarks.push((
                label.clone(),
                number.clone(),
                body.clone(),
                *level,
            ));
        }
        ElementPayload::ContextBlock { id } => {
            // P506: ContextBlock só precisa de ser locatable (tag emitida);
            // a expansão pós-introspecção resolve o closure.
            intr.kind_index
                .entry(ElementKind::ContextBlock)
                .or_default()
                .push(loc);
            intr.context_block_locations.insert(*id, loc);
        }
    }
}
