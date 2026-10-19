//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/pipeline/paged.md
//! @prompt-hash fca49f00
//! @layer L3
//! @updated 2026-10-09
//!
//! Orquestra a compilação paginada, introspecção runtime, realização de outlines, callbacks math/par.line, PageStore e binding dos números lógicos. Devolve somente candidatos completos, conserva os ciclos e os orçamentos vigentes e passa ao shaping a geração final validada.

use super::*;

/// Positive proof for the narrow page-independent subset. Unknown carriers
/// retain the complete numbering fixpoint; no visual equality is involved.
fn page_independent_style(key: &str, value: &Value, captured: bool) -> bool {
    match key {
        "text.font" => match value {
            Value::Str(_) => true,
            Value::Array(families) => {
                families.iter().all(|family| matches!(family, Value::Str(_)))
            }
            _ => false,
        },
        "eval.flow.scope" => matches!(value, Value::Bool(_)),
        "text.size" | "par.linebreaks" | "page.width" | "page.height"
        | "page.margin-left" | "page.margin-right" | "page.margin-top"
        | "page.margin-bottom" => matches!(
            value,
            Value::None
                | Value::Auto
                | Value::Bool(_)
                | Value::Int(_)
                | Value::Float(_)
                | Value::Str(_)
                | Value::Length(_)
                | Value::Color(_)
        ),
        "cell.fill" => matches!(value, Value::None | Value::Color(_)),
        "eval.table.local" => matches!(value, Value::Bool(true)),
        "eval.table.local.recipe" if !captured => {
            let Value::Content(Content::ShowRecipe(recipe)) = value else { return false };
            !recipe.textual
                && recipe.rules.is_empty()
                && recipe.guards.is_empty()
                && initial_layout_independent(&recipe.body)
                && recipe
                    .styles
                    .collapse()
                    .custom
                    .iter()
                    .all(|(key, value)| page_independent_style(key.as_str(), value, true))
        }
        _ => false,
    }
}

pub(super) fn initial_layout_independent(content: &Content) -> bool {
    let optional =
        |body: &Option<Content>| body.as_ref().is_none_or(initial_layout_independent);
    match content {
        Content::Empty
        | Content::Text(_)
        | Content::Space
        | Content::SpaceFromNewline
        | Content::Parbreak
        | Content::Raw(_)
        | Content::HSpace(_)
        | Content::VSpace(_)
        | Content::Pagebreak(_)
        | Content::Colbreak(_)
        | Content::Linebreak(_) => true,
        Content::Sequence(parts) => parts.iter().all(initial_layout_independent),
        Content::Par { body } => initial_layout_independent(body),
        Content::Strong(elem) => initial_layout_independent(&elem.body),
        Content::Emph(elem) => initial_layout_independent(&elem.body),
        Content::Source(elem) => initial_layout_independent(&elem.body),
        Content::Located(elem) => initial_layout_independent(&elem.body),
        Content::Realized(elem) => {
            initial_layout_independent(&elem.original)
                && initial_layout_independent(&elem.visual)
        }
        Content::Styled(elem) => {
            let scalar_styles =
                elem.styles.delta().custom.iter().all(|(key, value)| {
                    page_independent_style(key.as_str(), value, false)
                });
            scalar_styles && initial_layout_independent(&elem.body)
        }
        Content::Table(elem) => {
            elem.resolved
                .as_ref()
                .map_or(&elem.children, |r| &r.cells)
                .iter()
                .all(initial_layout_independent)
                && (elem.resolved.is_some()
                    || (optional(&elem.header) && optional(&elem.footer)))
                && optional(&elem.caption)
        }
        Content::Grid(elem) => {
            elem.cells.iter().all(initial_layout_independent)
                && optional(&elem.header)
                && optional(&elem.footer)
        }
        Content::TableCell(elem) => initial_layout_independent(
            elem.resolved.as_ref().map_or(&elem.body, |r| &r.visual),
        ),
        Content::GridCell(elem) => initial_layout_independent(&elem.body),
        Content::SetPage(delta) => {
            let marginal = |value: &Option<
                typst_core::entities::page_running::PageMarginal,
            >| {
                matches!(
                    value,
                    None | Some(typst_core::entities::page_running::PageMarginal::Auto)
                        | Some(typst_core::entities::page_running::PageMarginal::None)
                )
            };
            delta.numbering.is_none()
                && delta.supplement.is_none()
                && marginal(&delta.header)
                && marginal(&delta.footer)
                && delta.background.as_ref().is_none_or(Option::is_none)
                && delta.foreground.as_ref().is_none_or(Option::is_none)
        }
        _ => false, // Deliberate fail-closed boundary: not an assumption of purity.
    }
}

/// Executa os pós-processadores de introspecção que precisam de Engine sobre
/// o conteúdo final, já com ContextBlocks expandidos e Locations estáveis.
pub(super) fn introspect_with_runtime_for_pipeline(
    content: &Content,
    world: &dyn World,
    source: &Source,
) -> (
    SourceResult<typst_core::entities::introspector::TagIntrospector>,
    Vec<SourceDiagnostic>,
) {
    introspect_with_runtime_for_target(
        content,
        world,
        source,
        EvalTarget::Paged,
        typst_core::entities::compiler_features::Features::default(),
    )
}

pub(super) fn introspect_with_runtime_for_target(
    content: &Content,
    world: &dyn World,
    source: &Source,
    target: EvalTarget,
    features: typst_core::entities::compiler_features::Features,
) -> (
    SourceResult<typst_core::entities::introspector::TagIntrospector>,
    Vec<SourceDiagnostic>,
) {
    super::runtime::with_introspection_runtime(
        world,
        source,
        target,
        features,
        |engine, ctx| {
            typst_core::compiler::introspect::introspect_with_runtime(
                content, engine, ctx,
            )
        },
    )
}

/// P1406 — realiza outlines com Engine em L3, mantendo toda a decisão
/// semântica em L1. Um segundo passe focal só ocorre quando os carriers de
/// prefixo revelam máximos novos.
pub(super) fn realize_outlines_for_pipeline(
    content: &Content,
    introspector: &typst_core::entities::introspector::TagIntrospector,
    world: &dyn World,
    source: &Source,
) -> (
    SourceResult<(Content, typst_core::entities::introspector::TagIntrospector)>,
    Vec<SourceDiagnostic>,
) {
    let run_once = |input: &typst_core::entities::introspector::TagIntrospector| {
        let font_metrics = FallbackFontMetrics::new(world);
        let mut ctx = EvalContext::new();
        ctx.in_context = true;
        ctx.introspector = input.clone();
        let mut styles = StyleChain::default_chain();
        let mut show_rules: Arc<[ShowRule]> = Arc::from([]);
        let mut active_guards = Vec::new();
        let mut sink = TypstSink::new();
        let route = Route::root().with_id(source.id());
        let result = {
            let mut tracked_sink = sink.track_mut();
            let mut local_sink = TrackedMut::reborrow_mut(&mut tracked_sink);
            let mut engine = Engine {
                world,
                font_metrics: &font_metrics,
                route: route.track(),
                styles: &mut styles,
                show_rules: &mut show_rules,
                active_guards: &mut active_guards,
                current_file: source.id(),
                sink: &mut local_sink,
            };
            realize_outlines(content.clone(), input, &mut ctx, &mut engine)
        };
        (result, sink.into_diagnostics())
    };

    let (first, mut warnings) = run_once(introspector);
    let first = match first {
        Ok(content) => content,
        Err(errors) => return (Err(errors), warnings),
    };
    let same_carrier = match (content, &first) {
        (Content::Sequence(a), Content::Sequence(b)) => Arc::ptr_eq(a, b),
        (Content::Styled(a), Content::Styled(b)) => Arc::ptr_eq(a, b),
        (Content::Located(a), Content::Located(b)) => Arc::ptr_eq(a, b),
        (Content::Realized(a), Content::Realized(b)) => Arc::ptr_eq(a, b),
        _ => false,
    };
    if same_carrier {
        return (Ok((first, introspector.clone())), warnings);
    }
    let observed = introspect_with_introspector(&first).outline_prefix_widths;
    let mut stabilized = introspector.clone();
    if observed != stabilized.outline_prefix_widths {
        stabilized.outline_prefix_widths = observed;
        let (second, second_warnings) = run_once(&stabilized);
        warnings.extend(second_warnings);
        match second {
            Ok(content) => (Ok((content, stabilized)), warnings),
            Err(errors) => (Err(errors), warnings),
        }
    } else {
        (Ok((first, stabilized)), warnings)
    }
}

/// P1411 — expande blocos contextuais que só passam a existir durante a
/// realização pós-introspecção de outlines. A topologia realizada fornece
/// apenas as Locations desses produtores; páginas, posições e demais stores
/// continuam vindo do snapshot paginado causal recebido.
fn visit_context_blocks(
    content: &Content,
    visit_root: bool,
    visit: &mut impl FnMut(
        &Arc<typst_core::entities::elements::context_block::ContextBlockElem>,
    ),
) {
    match content {
        Content::ContextBlock(elem) => {
            if visit_root {
                visit(elem);
            }
        }
        Content::Source(e) => visit_context_blocks(&e.body, visit_root, visit),
        Content::Located(e) => visit_context_blocks(&e.body, false, visit),
        Content::Realized(e) => visit_context_blocks(&e.visual, true, visit),
        Content::ShowRecipe(e) => visit_context_blocks(&e.body, true, visit),
        Content::Styled(e) => visit_context_blocks(&e.body, true, visit),
        Content::Sequence(parts) => {
            for part in parts.iter() {
                visit_context_blocks(part, true, visit);
            }
        }
        Content::Table(e) => {
            if let Some(resolved) = &e.resolved {
                for cell in &resolved.cells {
                    visit_context_blocks(cell, true, visit);
                }
            } else {
                for child in &e.children {
                    visit_context_blocks(child, true, visit);
                }
                if let Some(header) = &e.header {
                    visit_context_blocks(header, true, visit);
                }
                if let Some(footer) = &e.footer {
                    visit_context_blocks(footer, true, visit);
                }
            }
            if let Some(caption) = &e.caption {
                visit_context_blocks(caption, true, visit);
            }
        }
        Content::TableCell(e) => {
            let visual = e
                .standalone
                .as_ref()
                .map(|r| &r.visual)
                .or_else(|| e.resolved.as_ref().map(|r| &r.visual))
                .unwrap_or(&e.body);
            visit_context_blocks(visual, true, visit);
        }
        other => {
            let _ = other.map_content(&mut |node| {
                if let Content::ContextBlock(elem) = node {
                    visit(elem);
                }
                Ok(None)
            });
        }
    }
}

fn expand_realized_outline_contexts(
    content: Content,
    input: &Content,
    introspector: &typst_core::entities::introspector::TagIntrospector,
    world: &dyn World,
    source: &Source,
) -> SourceResult<Content> {
    let mut existing = Vec::new();
    visit_context_blocks(input, true, &mut |elem| existing.push(elem.clone()));
    let mut has_new_context = false;
    visit_context_blocks(&content, true, &mut |elem| {
        has_new_context |= !existing.iter().any(|old| Arc::ptr_eq(old, elem));
    });
    // Sem produtores introduzidos pela realização de outlines, a passagem
    // seria um no-op semântico. Preservar os marcadores existentes é porém
    // essencial: o layout atribui-lhes a Position usada por `here()` na
    // iteração seguinte de `location.page-numbering()`.
    if !has_new_context {
        return Ok(content);
    }
    let content = content.map_content(&mut |node| {
        let Content::ContextBlock(elem) = node else { return Ok(None) };
        Ok(existing
            .iter()
            .any(|old| Arc::ptr_eq(old, elem))
            .then_some(Content::Empty))
    })?;
    let realized_topology = introspect_with_introspector(&content);
    let mut contextual_intr = introspector.clone();
    contextual_intr.context_block_locations = realized_topology.context_block_locations;
    context_stabilization::expand_legacy(content, &contextual_intr, world, source)
}

/// Adaptador da suíte com features default; delega ao pipeline produtivo.
#[cfg(test)]
pub(super) fn compile_to_paged_document_full_error(
    world: &dyn World,
    source: &Source,
    full_error: bool,
    timings: &mut Timings,
) -> (Result<PagedDocument, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>) {
    compile_to_paged_document_full_error_and_features(
        world,
        source,
        full_error,
        typst_core::entities::compiler_features::Features::default(),
        timings,
    )
}

pub(super) fn compile_to_paged_document_full_error_and_features(
    world: &dyn World,
    source: &Source,
    full_error: bool,
    features: typst_core::entities::compiler_features::Features,
    timings: &mut Timings,
) -> (Result<PagedDocument, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>) {
    let (result, warnings) =
        compile_to_paged_document_full_error_and_features_with_introspector(
            world, source, full_error, features, timings,
        );
    (result.map(|compiled| compiled.document), warnings)
}

pub(super) struct PagedCompilation {
    pub(super) document: PagedDocument,
    pub(super) introspector: typst_core::entities::introspector::TagIntrospector,
}

pub(super) fn compile_to_paged_document_full_error_and_features_with_introspector(
    world: &dyn World,
    source: &Source,
    full_error: bool,
    features: typst_core::entities::compiler_features::Features,
    timings: &mut Timings,
) -> (SourceResult<PagedCompilation>, Vec<SourceDiagnostic>) {
    let t0 = Instant::now();
    let (eval_result, mut warnings) = eval_to_module_with_sink_target_features(
        world,
        source,
        full_error,
        EvalTarget::Paged,
        features,
    );
    let t1 = Instant::now();
    timings.eval_ms = duration_ms(t1.duration_since(t0));

    let module = match eval_result {
        Ok(m) => m,
        Err(errors) => {
            timings.total_ms = timings.eval_ms;
            return (Err(errors), warnings);
        }
    };
    let content = match module.content() {
        Some(c) => c,
        None => {
            timings.total_ms = timings.eval_ms;
            return (
                Ok(PagedCompilation {
                    document: PagedDocument::new(vec![]),
                    introspector:
                        typst_core::entities::introspector::TagIntrospector::empty(),
                }),
                warnings,
            );
        }
    };
    let intr_content = module.introspection_content().unwrap_or(content).clone();
    let intr_content =
        typst_core::compiler::introspect::convert_bib_refs_to_cites(intr_content);
    let content =
        typst_core::compiler::introspect::convert_bib_refs_to_cites(content.clone());
    // Preservado para reexpandir `context` depois de o PageStore existir.
    let contextual_content = content.clone();

    let mut intr = module
        .realization_introspector()
        .cloned()
        .unwrap_or_else(|| introspect_with_introspector(&intr_content));
    let (supplements, supplement_warnings) = super::runtime::prepare_heading_supplements(
        &mut intr,
        world,
        source,
        EvalTarget::Paged,
        features,
    );
    warnings.extend(supplement_warnings);
    if let Err(errors) = supplements {
        return (Err(errors), warnings);
    }
    for (key, style) in module.bibliography_styles() {
        intr.bib_store.add_style(*key, style.clone());
    }
    let t2 = Instant::now();
    timings.introspect_ms = duration_ms(t2.duration_since(t1));

    let (prepared, contextual_warnings) = context_stabilization::prepare(
        content.clone(),
        &intr,
        world,
        source,
        full_error,
        features,
        Some(&module),
    );
    warnings.extend(contextual_warnings);
    let prepared = match prepared {
        Ok(prepared) => prepared,
        Err(errors) => {
            timings.expand_context_ms = duration_ms(Instant::now().duration_since(t2));
            timings.total_ms =
                timings.eval_ms + timings.introspect_ms + timings.expand_context_ms;
            return (Err(errors), warnings);
        }
    };
    let mut content = prepared.content;
    let t3 = Instant::now();
    timings.expand_context_ms = duration_ms(t3.duration_since(t2));
    let mut format_content = content.clone();
    let mut ordinary_layout_metrics = None;
    let mut selected_layout_metrics = None;
    let (mut doc, mut intr_for_positions, extracted_headings) =
        if let Some((intr, doc, metrics)) = prepared.settled {
            let headings = intr.headings_for_bookmarks().to_vec();
            selected_layout_metrics = Some(metrics);
            (doc, intr, headings)
        } else {
            let (runtime_intr, runtime_warnings) =
                introspect_with_runtime_for_pipeline(&content, world, source);
            warnings.extend(runtime_warnings);
            let mut intr = match runtime_intr {
                Ok(intr) => intr,
                Err(errors) => {
                    timings.expand_context_ms =
                        duration_ms(Instant::now().duration_since(t2));
                    timings.total_ms = timings.eval_ms
                        + timings.introspect_ms
                        + timings.expand_context_ms;
                    return (Err(errors), warnings);
                }
            };
            for (key, style) in module.bibliography_styles() {
                intr.bib_store.add_style(*key, style.clone());
            }
            let extracted_headings = intr.headings_for_bookmarks().to_vec();
            let mut intr_for_positions = intr.clone();
            let mut layout_intr = intr;
            let (realized, realization_warnings) =
                realize_outlines_for_pipeline(&content, &layout_intr, world, source);
            warnings.extend(realization_warnings);
            let (realized_content, realized_intr) = match realized {
                Ok(pair) => pair,
                Err(errors) => return (Err(errors), warnings),
            };
            layout_intr = realized_intr;
            let realized_content = match expand_realized_outline_contexts(
                realized_content,
                &content,
                &layout_intr,
                world,
                source,
            ) {
                Ok(content) => content,
                Err(errors) => return (Err(errors), warnings),
            };
            let mut final_realized_content = realized_content.clone();
            // One World and one compilation: let the second numbering candidate
            // reuse font/shaping caches warmed by the first layout.
            let layout_metrics = FallbackFontMetrics::new(world);
            let (mut doc, callback_warnings) =
                match layout_with_realized_math_callbacks_with_metrics(
                    &realized_content,
                    layout_intr.clone(),
                    world,
                    source,
                    &layout_metrics,
                ) {
                    Ok(pair) => pair,
                    Err(errors) => return (Err(errors), warnings),
                };
            warnings.extend(callback_warnings);
            let reuse_initial_layout = initial_layout_independent(&contextual_content)
                && initial_layout_independent(&realized_content)
                && doc.pages.iter().all(|page| page.numbering.is_none());
            // P1159: callbacks são executados somente aqui, onde existe Engine. O
            // layouter recebe apenas vistas Content seladas da passagem anterior.
            for _ in 0..5 {
                let previous_pages = doc.pages.len();
                let previous_values = layout_intr.page_counter_values.clone();
                let previous_finals = layout_intr.page_counter_final_values.clone();
                layout_intr.inject_positions(doc.extracted_positions.clone());
                let (snapshot, snapshot_warnings) =
                    context_stabilization::realize_page_snapshot_for_pipeline(
                        &[(Vec::new(), std::num::NonZeroUsize::new(doc.pages.len()))],
                        &mut layout_intr,
                        world,
                        source,
                    );
                if let Err(errors) = snapshot {
                    return (Err(errors), warnings);
                }
                warnings.extend(snapshot_warnings);
                let page_numbers = (1..=doc.pages.len())
                    .map(|p| {
                        layout_intr
                            .page_store
                            .logical_number_for_page(
                                std::num::NonZeroUsize::new(p).unwrap(),
                            )
                            .unwrap_or(p)
                    })
                    .collect::<Vec<_>>();
                let final_number = layout_intr
                    .page_counter_final_values
                    .values()
                    .next()
                    .and_then(|v| v.first())
                    .copied()
                    .unwrap_or_else(|| page_numbers.last().copied().unwrap_or(0));
                let snapshot_same = previous_values == layout_intr.page_counter_values
                    && previous_finals == layout_intr.page_counter_final_values;
                let reference_pages = page_reference_pages(&content, &doc);
                let (store, numbering_warnings) = match realize_page_numberings(
                    world,
                    source,
                    &doc,
                    &page_numbers,
                    final_number,
                    &reference_pages,
                ) {
                    Ok(pair) => pair,
                    Err(errors) => return (Err(errors), warnings),
                };
                warnings.extend(numbering_warnings);
                layout_intr.inject_positions(doc.extracted_positions.clone());
                layout_intr.inject_pages(store.clone());
                if reuse_initial_layout {
                    break;
                }
                // `location.page-numbering()` vive em `context`; a primeira expansão
                // ocorreu legitimamente sem páginas conhecidas. Reexecutá-la com o
                // snapshot anterior é parte do fixpoint, não um default falso.
                let expanded = match context_stabilization::expand_legacy(
                    contextual_content.clone(),
                    &layout_intr,
                    world,
                    source,
                ) {
                    Ok(expanded) => expanded,
                    Err(errors) => return (Err(errors), warnings),
                };
                let (runtime_expanded, runtime_expanded_warnings) =
                    introspect_with_runtime_for_pipeline(&expanded, world, source);
                warnings.extend(runtime_expanded_warnings);
                let mut expanded_intr = match runtime_expanded {
                    Ok(intr) => intr,
                    Err(errors) => return (Err(errors), warnings),
                };
                for (key, style) in module.bibliography_styles() {
                    expanded_intr.bib_store.add_style(*key, style.clone());
                }
                expanded_intr.inject_positions(doc.extracted_positions.clone());
                expanded_intr.inject_pages(store);
                expanded_intr.page_counter_values =
                    layout_intr.page_counter_values.clone();
                expanded_intr.page_counter_final_values =
                    layout_intr.page_counter_final_values.clone();
                content = expanded;
                layout_intr = expanded_intr;
                let (realized, realization_warnings) =
                    realize_outlines_for_pipeline(&content, &layout_intr, world, source);
                warnings.extend(realization_warnings);
                let (realized_content, realized_intr) = match realized {
                    Ok(pair) => pair,
                    Err(errors) => return (Err(errors), warnings),
                };
                layout_intr = realized_intr;
                let realized_content = match expand_realized_outline_contexts(
                    realized_content,
                    &content,
                    &layout_intr,
                    world,
                    source,
                ) {
                    Ok(content) => content,
                    Err(errors) => return (Err(errors), warnings),
                };
                let (next, callback_warnings) =
                    match layout_with_realized_math_callbacks_with_metrics(
                        &realized_content,
                        layout_intr.clone(),
                        world,
                        source,
                        &layout_metrics,
                    ) {
                        Ok(pair) => pair,
                        Err(errors) => return (Err(errors), warnings),
                    };
                warnings.extend(callback_warnings);
                let converged = next.pages.len() == previous_pages
                    && (snapshot_same || layout_intr.context_block_locations.is_empty());
                final_realized_content = realized_content;
                doc = next;
                if converged {
                    break;
                }
            }
            intr_for_positions.inject_pages(layout_intr.page_store.clone());
            intr_for_positions.page_counter_values =
                layout_intr.page_counter_values.clone();
            intr_for_positions.page_counter_final_values =
                layout_intr.page_counter_final_values.clone();
            format_content = final_realized_content;
            ordinary_layout_metrics = Some(layout_metrics);
            (doc, intr_for_positions, extracted_headings)
        };

    let realized_total = intr_for_positions
        .page_store
        .total_pages()
        .map(|count| count.get())
        .unwrap_or(0);
    if realized_total != doc.pages.len() {
        return (Err(bundle_error(format!(
            "realized page numbers do not match final document (rendered {}, store {realized_total})",
            doc.pages.len(),
        ))), warnings);
    }
    if let Err(errors) =
        bind_logical_page_numbers(&mut doc, &intr_for_positions.page_store, 0)
    {
        return (Err(errors), warnings);
    }
    doc.format_options = match collect_document_format_options(&format_content) {
        Ok(options) => options,
        Err(errors) => {
            timings.layout_ms = duration_ms(Instant::now().duration_since(t3));
            timings.total_ms = timings.eval_ms
                + timings.introspect_ms
                + timings.expand_context_ms
                + timings.layout_ms;
            return (Err(errors), warnings);
        }
    };
    for warning in doc.layout_warnings.drain(..) {
        warnings.push(SourceDiagnostic::warning(Span::detached(), warning));
    }
    if !doc.layout_errors.is_empty() {
        let errors: Vec<SourceDiagnostic> = doc.layout_errors.drain(..).collect();
        timings.layout_ms = duration_ms(Instant::now().duration_since(t3));
        timings.total_ms = timings.eval_ms
            + timings.introspect_ms
            + timings.expand_context_ms
            + timings.layout_ms;
        return (Err(errors), warnings);
    }
    doc.extracted_headings = extracted_headings;

    let heading_locations = intr_for_positions.query_by_kind(ElementKind::Heading);
    for (idx, (label, _, _, _)) in doc.extracted_headings.iter().enumerate() {
        if let Some(loc) = heading_locations.get(idx) {
            if let Some(pos) = doc.extracted_positions.position_of(*loc) {
                doc.extracted_label_pages.insert(label.clone(), pos.page.get());
                doc.extracted_label_positions.insert(label.clone(), pos.point);
            }
        }
    }

    doc.document_info = module.document_info().clone();

    let t4 = Instant::now();
    timings.layout_ms = duration_ms(t4.duration_since(t3));

    let bidi_registry = crate::bidi_context::Registry::new(&doc);
    let bidi_resolved = bidi_registry.resolve();
    let doc = match ordinary_layout_metrics.as_ref().or(selected_layout_metrics.as_ref())
    {
        Some(metrics) => crate::layout_bidi::reorder_bidi_document_with_context(
            doc,
            metrics,
            &bidi_resolved,
        ),
        None => {
            #[cfg(test)]
            super::tests::record_uncached_bidi_metrics();
            crate::layout_bidi::reorder_bidi_document_with_context(
                doc,
                &FallbackFontMetrics::new(world),
                &bidi_resolved,
            )
        }
    };

    let doc = crate::shaper::shape_document_with_context(world, doc, &bidi_resolved);
    let doc = match ordinary_layout_metrics.as_ref().or(selected_layout_metrics.as_ref())
    {
        Some(metrics) => crate::shaper::fix_line_positions_with_metrics(doc, metrics),
        None => {
            #[cfg(test)]
            super::tests::record_uncached_reconciliation_metrics();
            crate::shaper::fix_line_positions(world, doc)
        }
    };
    let t5 = Instant::now();
    timings.shape_ms = duration_ms(t5.duration_since(t4));

    if let Err(msg) = crate::export::validate_document_images(&doc) {
        timings.total_ms = timings.eval_ms
            + timings.introspect_ms
            + timings.expand_context_ms
            + timings.layout_ms
            + timings.shape_ms;
        return (Err(vec![SourceDiagnostic::error(Span::detached(), msg)]), warnings);
    }

    intr_for_positions.inject_positions(doc.extracted_positions.clone());
    (Ok(PagedCompilation { document: doc, introspector: intr_for_positions }), warnings)
}

/// Runs the pure L1 transcript until a complete document is produced. Every
/// provisional document is consumed by `Pending` and therefore cannot reach
/// numbering, shaping, or export.
pub(super) fn layout_with_realized_math_callbacks_with_metrics(
    content: &Content,
    introspector: typst_core::entities::introspector::TagIntrospector,
    world: &dyn World,
    source: &Source,
    font_metrics: &FallbackFontMetrics<'_>,
) -> SourceResult<(PagedDocument, Vec<SourceDiagnostic>)> {
    layout_with_realized_math_callbacks_from_with_metrics(
        content,
        introspector,
        world,
        source,
        typst_core::entities::locator::Locator::new().checkpoint(),
        Some(font_metrics),
    )
}

pub(super) fn layout_with_realized_math_callbacks_from(
    content: &Content,
    introspector: typst_core::entities::introspector::TagIntrospector,
    world: &dyn World,
    source: &Source,
    location_start: typst_core::entities::locator::LocatorSeed,
) -> SourceResult<(PagedDocument, Vec<SourceDiagnostic>)> {
    layout_with_realized_math_callbacks_from_with_metrics(
        content,
        introspector,
        world,
        source,
        location_start,
        None,
    )
}

fn layout_with_realized_math_callbacks_from_with_metrics(
    content: &Content,
    introspector: typst_core::entities::introspector::TagIntrospector,
    world: &dyn World,
    source: &Source,
    location_start: typst_core::entities::locator::LocatorSeed,
    shared_metrics: Option<&FallbackFontMetrics<'_>>,
) -> SourceResult<(PagedDocument, Vec<SourceDiagnostic>)> {
    #[cfg(test)]
    super::tests::record_initial_layout_call();

    let mut math_store: Option<SealedMathCallbacks> = None;
    let mut par_line_store: Option<SealedParLineCallbacks> = None;
    let mut warnings = Vec::new();
    for _ in 0..8 {
        match layout_with_introspector_and_metrics_and_callbacks_from(
            content,
            introspector.clone(),
            shared_metrics.cloned().unwrap_or_else(|| {
                #[cfg(test)]
                super::tests::record_uncached_layout_metrics();
                FallbackFontMetrics::new(world)
            }),
            ImageSizeImageSizer,
            11.0,
            math_store.as_ref(),
            par_line_store.as_ref(),
            location_start,
        ) {
            LayoutPassOutcome::Complete(document) => return Ok((document, warnings)),
            LayoutPassOutcome::Pending(requests) => {
                if requests.math.is_empty() && requests.par_line.is_empty() {
                    return Err(vec![SourceDiagnostic::error(
                        Span::detached(),
                        "layout did not converge before callback realization",
                    )]);
                }
                if !requests.math.is_empty() {
                    let (realized, pass_warnings) =
                        realize_math_cancel_requests(world, source, requests.math)?;
                    warnings.extend(pass_warnings);
                    math_store = Some(realized);
                }
                if !requests.par_line.is_empty() {
                    let (realized, pass_warnings) =
                        realize_par_line_requests(world, source, requests.par_line)?;
                    warnings.extend(pass_warnings);
                    par_line_store = Some(realized);
                }
            }
        }
    }
    Err(vec![SourceDiagnostic::error(
        Span::detached(),
        "layout callbacks did not converge",
    )])
}

fn realize_par_line_requests(
    world: &dyn World,
    source: &Source,
    requests: Vec<ParLineCallbackRequest>,
) -> SourceResult<(SealedParLineCallbacks, Vec<SourceDiagnostic>)> {
    let font_metrics = FallbackFontMetrics::new(world);
    let mut ctx = EvalContext::new();
    let mut scopes = Scopes::new(None);
    let mut styles = StyleChain::default_chain();
    let mut show_rules: Arc<[ShowRule]> = Arc::from([]);
    let mut active_guards = Vec::new();
    let mut sink = TypstSink::new();
    let route = Route::root().with_id(source.id());
    let mut resolutions = Vec::with_capacity(requests.len());
    {
        let mut tracked_sink = sink.track_mut();
        let mut local_sink = TrackedMut::reborrow_mut(&mut tracked_sink);
        let mut engine = Engine {
            world,
            font_metrics: &font_metrics,
            route: route.track(),
            styles: &mut styles,
            show_rules: &mut show_rules,
            active_guards: &mut active_guards,
            current_file: source.id(),
            sink: &mut local_sink,
        };
        for request in requests {
            *engine.styles = request.styles.clone();
            let numbering =
                typst_core::entities::numbering::Numbering::Func(request.func.clone());
            let content = realize_numbering(
                &numbering,
                &[request.number],
                request.span,
                &mut scopes,
                &mut ctx,
                &mut engine,
            )
            .map_err(|mut diagnostics| {
                for diagnostic in &mut diagnostics {
                    diagnostic.span = request.span;
                }
                diagnostics
            })?;
            resolutions.push(ParLineCallbackResolution { request, content });
        }
    }
    Ok((SealedParLineCallbacks::new(resolutions), sink.into_diagnostics()))
}

fn realize_math_cancel_requests(
    world: &dyn World,
    source: &Source,
    requests: Vec<typst_core::compiler::math::layout::callbacks::MathCancelRequest>,
) -> SourceResult<(SealedMathCallbacks, Vec<SourceDiagnostic>)> {
    let font_metrics = FallbackFontMetrics::new(world);
    let mut ctx = EvalContext::new();
    let mut scopes = Scopes::new(None);
    let mut styles = StyleChain::default_chain();
    let mut show_rules: Arc<[ShowRule]> = Arc::from([]);
    let mut active_guards = Vec::new();
    let mut sink = TypstSink::new();
    let route = Route::root().with_id(source.id());
    let mut resolutions = Vec::with_capacity(requests.len());
    {
        let mut tracked_sink = sink.track_mut();
        let mut local_sink = TrackedMut::reborrow_mut(&mut tracked_sink);
        let mut engine = Engine {
            world,
            font_metrics: &font_metrics,
            route: route.track(),
            styles: &mut styles,
            show_rules: &mut show_rules,
            active_guards: &mut active_guards,
            current_file: source.id(),
            sink: &mut local_sink,
        };
        for request in requests {
            *engine.styles = request.styles.clone();
            let mut value = apply_func(
                request.func.clone(),
                Args::positional(vec![Value::Angle(request.default)]),
                &mut scopes,
                &mut ctx,
                &mut engine,
            )
            .map_err(|mut diagnostics| {
                for diagnostic in &mut diagnostics {
                    diagnostic.span = request.span;
                }
                diagnostics
            })?;
            // A contextual callback is represented by the existing delayed
            // ContextBlock carrier. At this L3 boundary its captured closure
            // can be evaluated against the request's sealed style snapshot.
            if let Value::Content(Content::ContextBlock(contextual)) = value {
                value = apply_func(
                    contextual.closure.clone(),
                    Args::positional(Vec::new()),
                    &mut scopes,
                    &mut ctx,
                    &mut engine,
                )
                .map_err(|mut diagnostics| {
                    for diagnostic in &mut diagnostics {
                        diagnostic.span = request.span;
                    }
                    diagnostics
                })?;
            }
            let Value::Angle(angle) = value else {
                let found = match value.type_name() {
                    "int" => "integer",
                    "str" => "string",
                    "bool" => "boolean",
                    other => other,
                };
                return Err(vec![SourceDiagnostic::error(
                    request.span,
                    format!("expected angle, found {found}"),
                )]);
            };
            resolutions.push(MathCancelResolution { request, angle });
        }
    }
    Ok((SealedMathCallbacks::new(resolutions), sink.into_diagnostics()))
}

pub(super) fn bind_logical_page_numbers(
    doc: &mut PagedDocument,
    store: &typst_core::entities::page_store::PageStore,
    offset: usize,
) -> SourceResult<()> {
    let total = store.total_pages().map(|count| count.get()).unwrap_or(0);
    let end = offset
        .checked_add(doc.pages.len())
        .ok_or_else(|| bundle_error("realized page numbers exceed page store"))?;
    if end > total {
        return Err(bundle_error(format!(
            "realized page numbers do not match final document (offset {offset}, rendered {}, store {total})",
            doc.pages.len(),
        )));
    }
    for (index, page) in doc.pages.iter_mut().enumerate() {
        let physical = std::num::NonZeroUsize::new(offset + index + 1)
            .expect("page indices start at one");
        page.logical_number = store
            .logical_number_for_page(physical)
            .ok_or_else(|| bundle_error("missing realized logical page number"))?;
    }
    Ok(())
}

/// Realiza as vistas binária (margem) e unária (referência) de cada página.
pub(super) fn realize_page_numberings(
    world: &dyn World,
    source: &Source,
    doc: &PagedDocument,
    logical_numbers: &[usize],
    logical_total: usize,
    reference_pages: &HashSet<usize>,
) -> SourceResult<(typst_core::entities::page_store::PageStore, Vec<SourceDiagnostic>)> {
    let Some(total) = std::num::NonZeroUsize::new(doc.pages.len()) else {
        return Ok((typst_core::entities::page_store::PageStore::empty(), Vec::new()));
    };
    let font_metrics = FallbackFontMetrics::new(world);
    let mut ctx = EvalContext::new();
    let mut scopes = Scopes::new(None);
    let mut styles = StyleChain::default_chain();
    let mut show_rules: Arc<[ShowRule]> = Arc::from([]);
    let mut active_guards = Vec::new();
    let mut sink = TypstSink::new();
    let route = Route::root().with_id(source.id());
    let mut visible = Vec::with_capacity(total.get());
    let mut reference = Vec::with_capacity(total.get());
    {
        let mut tracked_sink = sink.track_mut();
        let mut local_sink = TrackedMut::reborrow_mut(&mut tracked_sink);
        let mut engine = Engine {
            world,
            font_metrics: &font_metrics,
            route: route.track(),
            styles: &mut styles,
            show_rules: &mut show_rules,
            active_guards: &mut active_guards,
            current_file: source.id(),
            sink: &mut local_sink,
        };
        for (index, page) in doc.pages.iter().enumerate() {
            let current = logical_numbers.get(index).copied().unwrap_or(index + 1);
            match &page.numbering {
                Some(numbering) => {
                    visible.push(Some(typst_core::compiler::stdlib::realize_numbering(
                        numbering,
                        &[current, logical_total],
                        Span::detached(),
                        &mut scopes,
                        &mut ctx,
                        &mut engine,
                    )?));
                    if reference_pages.contains(&(index + 1)) {
                        reference.push(Some(
                            typst_core::compiler::stdlib::realize_numbering(
                                numbering,
                                &[current],
                                Span::detached(),
                                &mut scopes,
                                &mut ctx,
                                &mut engine,
                            )?,
                        ));
                    } else {
                        reference.push(None);
                    }
                }
                None => {
                    visible.push(None);
                    reference.push(None);
                }
            }
        }
    }
    let warnings = sink.into_diagnostics();
    Ok((
        typst_core::entities::page_store::PageStore::from_realized(
            total,
            doc.pages.iter().map(|page| page.numbering.clone()).collect(),
            logical_numbers.to_vec(),
            vec![Span::detached(); total.get()],
            visible,
            reference,
            doc.pages.iter().map(|page| page.supplement.clone()).collect(),
        ),
        warnings,
    ))
}

pub(super) fn page_reference_pages(
    content: &Content,
    doc: &PagedDocument,
) -> HashSet<usize> {
    use typst_core::entities::label::Label;

    let mut pages = HashSet::new();
    if doc.extracted_label_pages.is_empty() {
        return pages;
    }
    let _ = content.clone().map_content(&mut |node| {
        if let Content::Ref(elem) = node {
            if elem.form == typst_core::entities::elements::r#ref::RefForm::Page {
                if let Some(page) =
                    doc.extracted_label_pages.get(&Label(elem.name.to_string()))
                {
                    pages.insert(*page);
                }
            }
        }
        Ok(None)
    });
    pages
}

#[cfg(test)]
mod context_discovery_tests {
    use super::*;
    use typst_core::entities::content::{LocatedContent, RealizedContent};
    use typst_core::entities::func::Func;
    use typst_core::entities::locator::Locator;
    use typst_core::entities::style::Styles;

    #[test]
    fn page_reference_pages_handles_empty_and_present_destinations() {
        use typst_core::entities::elements::r#ref::{RefElem, RefForm};
        use typst_core::entities::label::Label;

        let page_ref = |name: &str, form| {
            Content::Ref(Arc::new(RefElem { name: name.into(), supplement: None, form }))
        };
        let content = Content::sequence(vec![
            page_ref("hit", RefForm::Page),
            page_ref("missing", RefForm::Page),
            page_ref("normal", RefForm::Normal),
        ]);
        let mut doc = PagedDocument::new(vec![]);
        assert!(page_reference_pages(&content, &doc).is_empty());

        doc.extracted_label_pages.insert(Label("hit".into()), 3);
        doc.extracted_label_pages.insert(Label("normal".into()), 7);
        assert_eq!(page_reference_pages(&content, &doc), HashSet::from([3]));
    }

    #[test]
    fn borrowed_context_visit_matches_map_content_order_and_identity() {
        let block = |id| {
            Content::ContextBlock(Arc::new(ContextBlockElem {
                id,
                closure: Func::native("dummy", |_ctx, _args, _world, _file| {
                    Ok(Value::None)
                }),
            }))
        };
        let mut locator = Locator::new();
        let located = Content::Located(Arc::new(LocatedContent {
            location: locator.next(),
            body: block(2),
        }));
        let cell = Content::table_cell(block(3), None, None, None, None);
        let table =
            Content::table_with_caption(vec![], vec![], vec![cell], Some(block(4)));
        let realized = Content::Realized(Arc::new(RealizedContent {
            original: block(5),
            visual: block(6),
        }));
        let tree = Content::sequence(vec![
            block(1),
            located,
            table,
            realized,
            Content::styled(Box::new(block(7)), Styles::new()),
        ]);
        let mut previous = Vec::new();
        let _ = tree.map_content(&mut |node| {
            if let Content::ContextBlock(elem) = node {
                previous.push((elem.id, Arc::as_ptr(elem) as usize));
            }
            Ok(None)
        });
        let mut borrowed = Vec::new();
        visit_context_blocks(&tree, true, &mut |elem| {
            borrowed.push((elem.id, Arc::as_ptr(elem) as usize));
        });
        assert_eq!(borrowed, previous);
        assert_eq!(
            borrowed.iter().map(|(id, _)| *id).collect::<Vec<_>>(),
            vec![1, 3, 4, 6, 7]
        );
    }
}
