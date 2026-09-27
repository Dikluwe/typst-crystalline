//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/pipeline/paged.md
//! @prompt-hash 17d87d76
//! @layer L3
//! @updated 2026-09-26
//!
//! Orquestra a compilação paginada, introspecção runtime, realização de outlines, callbacks math/par.line, PageStore e binding dos números lógicos. Devolve somente candidatos completos, conserva os ciclos e os orçamentos vigentes e passa ao shaping a geração final validada.

use super::*;

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
    let font_metrics = FallbackFontMetrics::new(world);
    let mut ctx = EvalContext::new();
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
        typst_core::compiler::introspect::introspect_with_runtime(
            content,
            &mut engine,
            &mut ctx,
        )
    };
    (result, sink.into_diagnostics())
}

/// P1406 — realiza outlines com Engine em L3, mantendo toda a decisão
/// semântica em L1. Um segundo passe focal só ocorre quando os carriers de
/// prefixo revelam máximos novos.
fn realize_outlines_for_pipeline(
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
fn expand_realized_outline_contexts(
    content: Content,
    input: &Content,
    introspector: &typst_core::entities::introspector::TagIntrospector,
    world: &dyn World,
    source: &Source,
) -> SourceResult<Content> {
    let mut existing = Vec::new();
    let _ = input.map_content(&mut |node| {
        if let Content::ContextBlock(elem) = node {
            existing.push(elem.clone());
        }
        Ok(None)
    });
    let mut has_new_context = false;
    let _ = content.map_content(&mut |node| {
        if let Content::ContextBlock(elem) = node {
            has_new_context |= !existing.iter().any(|old| Arc::ptr_eq(old, elem));
        }
        Ok(None)
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

/// Compila `source` contra `world` até produzir um `PagedDocument` pronto
/// para exportação (eval → introspect → expand → layout → bidi → shape → validação de imagens).
///
/// Esta função é partilhada pelos backends PDF, PNG e SVG (P870).
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
    (result.map(|(document, _)| document), warnings)
}

pub(super) fn compile_to_paged_document_full_error_and_features_with_introspector(
    world: &dyn World,
    source: &Source,
    full_error: bool,
    features: typst_core::entities::compiler_features::Features,
    timings: &mut Timings,
) -> (
    Result<
        (PagedDocument, typst_core::entities::introspector::TagIntrospector),
        Vec<SourceDiagnostic>,
    >,
    Vec<SourceDiagnostic>,
) {
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
                Ok((
                    PagedDocument::new(vec![]),
                    typst_core::entities::introspector::TagIntrospector::empty(),
                )),
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

    let mut intr = introspect_with_introspector(&intr_content);
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
    let (mut doc, mut intr_for_positions, extracted_headings) = if let Some((intr, doc)) =
        prepared.settled
    {
        let headings = intr.headings_for_bookmarks().to_vec();
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
                timings.total_ms =
                    timings.eval_ms + timings.introspect_ms + timings.expand_context_ms;
                return (Err(errors), warnings);
            }
        };
        for (key, style) in module.bibliography_styles() {
            intr.bib_store.add_style(*key, style.clone());
        }
        let extracted_headings = intr.headings_for_bookmarks().to_vec();
        let logical_page_start = intr
            .counter_final_values(&typst_core::entities::counter::CounterKey::Page)
            .and_then(|values| values.first().copied())
            .unwrap_or(1);
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
        let (mut doc, callback_warnings) = match layout_with_realized_math_callbacks(
            &realized_content,
            layout_intr.clone(),
            world,
            source,
        ) {
            Ok(pair) => pair,
            Err(errors) => return (Err(errors), warnings),
        };
        warnings.extend(callback_warnings);
        // P1159: callbacks são executados somente aqui, onde existe Engine. O
        // layouter recebe apenas vistas Content seladas da passagem anterior.
        for _ in 0..5 {
            let previous_pages = doc.pages.len();
            let reference_pages = page_reference_pages(&content, &doc);
            let (store, numbering_warnings) = match realize_page_numberings(
                world,
                source,
                &doc,
                logical_page_start,
                &reference_pages,
            ) {
                Ok(pair) => pair,
                Err(errors) => return (Err(errors), warnings),
            };
            warnings.extend(numbering_warnings);
            layout_intr.inject_positions(doc.extracted_positions.clone());
            layout_intr.inject_pages(store.clone());
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
            let (next, callback_warnings) = match layout_with_realized_math_callbacks(
                &realized_content,
                layout_intr.clone(),
                world,
                source,
            ) {
                Ok(pair) => pair,
                Err(errors) => return (Err(errors), warnings),
            };
            warnings.extend(callback_warnings);
            let converged = next.pages.len() == previous_pages;
            final_realized_content = realized_content;
            doc = next;
            if converged {
                break;
            }
        }
        intr_for_positions.inject_pages(layout_intr.page_store.clone());
        format_content = final_realized_content;
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

    let doc =
        crate::layout_bidi::reorder_bidi_document(doc, &FallbackFontMetrics::new(world));

    let doc = crate::shaper::shape_document(world, doc);
    let doc = crate::shaper::fix_line_positions(world, doc);
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
    (Ok((doc, intr_for_positions)), warnings)
}

/// Runs the pure L1 transcript until a complete document is produced. Every
/// provisional document is consumed by `Pending` and therefore cannot reach
/// numbering, shaping, or export.
pub(super) fn layout_with_realized_math_callbacks(
    content: &Content,
    introspector: typst_core::entities::introspector::TagIntrospector,
    world: &dyn World,
    source: &Source,
) -> SourceResult<(PagedDocument, Vec<SourceDiagnostic>)> {
    layout_with_realized_math_callbacks_from(
        content,
        introspector,
        world,
        source,
        typst_core::entities::locator::Locator::new().checkpoint(),
    )
}

pub(super) fn layout_with_realized_math_callbacks_from(
    content: &Content,
    introspector: typst_core::entities::introspector::TagIntrospector,
    world: &dyn World,
    source: &Source,
    location_start: typst_core::entities::locator::LocatorSeed,
) -> SourceResult<(PagedDocument, Vec<SourceDiagnostic>)> {
    let mut math_store: Option<SealedMathCallbacks> = None;
    let mut par_line_store: Option<SealedParLineCallbacks> = None;
    let mut warnings = Vec::new();
    for _ in 0..8 {
        match layout_with_introspector_and_metrics_and_callbacks_from(
            content,
            introspector.clone(),
            FallbackFontMetrics::new(world),
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
    logical_start: usize,
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
            let current = logical_start + index;
            let logical_total = logical_start + total.get() - 1;
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
            (logical_start..logical_start + total.get()).collect(),
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
