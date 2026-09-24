//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/pipeline/context_stabilization.md
//! @prompt-hash 559788e9
//! @layer L3
//! @updated 2026-09-10
//!
//! A compilation-local owner of contextual contributions and their lifetimes.

use super::*;
use typst_core::entities::compiler_features::Features;
use typst_core::entities::introspector::TagIntrospector;
use typst_core::entities::location::Location;

pub(super) struct Prepared {
    pub content: Content,
    pub settled: Option<(TagIntrospector, PagedDocument)>,
}

struct Execution {
    ctx: EvalContext,
    input: TagIntrospector,
    result: SourceResult<Content>,
    sink: TypstSink,
    location: Location,
    final_styles: StyleChain,
}

struct Producer {
    elem: Arc<ContextBlockElem>,
    chain: StyleChain,
    parent: Option<usize>,
    children: Vec<usize>,
    current: Option<Box<Execution>>,
    selected: bool,
    selected_filtered_counter: bool,
    selected_html_position: bool,
    invalid: bool,
    active: bool,
    target: EvalTarget,
}

struct Session<'a> {
    world: &'a dyn World,
    source: &'a Source,
    metrics: FallbackFontMetrics<'a>,
    target: EvalTarget,
    features: Features,
    full_error: bool,
    origin: Content,
    original_intr: TagIntrospector,
    nodes: Vec<Producer>,
    roots: Vec<usize>,
    // Retain causal objects, not fingerprints of newly recreated closures.
    retired: Vec<Box<Execution>>,
    next_id: u64,
    impossible: bool,
    bundle_html_targets: Option<Vec<bool>>,
    bundle_candidate: Option<TagIntrospector>,
}

fn terminal<T>(source: &Source, errors: Vec<SourceDiagnostic>) -> SourceResult<T> {
    Err(if errors.is_empty() {
        vec![SourceDiagnostic::error(
            source.root().span(),
            "contextual stability could not be verified",
        )]
    } else {
        errors
    })
}

/// Collect in actual content order. The map supplies the existing exhaustive
/// style traversal, but never chooses diagnostic order or drops an occurrence.
fn ordered_blocks(
    content: &Content,
    chain: &StyleChain,
) -> Option<Vec<(Arc<ContextBlockElem>, StyleChain)>> {
    let by_id = collect_context_blocks(content, chain);
    let mut seen = HashSet::new();
    let mut ordered = Vec::new();
    let mut unique = true;
    let _ = content.map_content(&mut |node| {
        if let Content::ContextBlock(elem) = node {
            if !seen.insert(elem.id) {
                unique = false;
            }
            if let Some(pair) = by_id.get(&elem.id) {
                ordered.push(pair.clone());
            }
        }
        Ok(None)
    });
    unique.then_some(ordered)
}

impl<'a> Session<'a> {
    fn new(
        origin: Content,
        intr: &TagIntrospector,
        world: &'a dyn World,
        source: &'a Source,
        full_error: bool,
        features: Features,
        target: EvalTarget,
    ) -> Self {
        let blocks = ordered_blocks(&origin, &StyleChain::default_chain());
        let impossible = blocks.is_none();
        // Reused ids are legal on the legacy path. Keep its one evaluation per
        // id until effective selection establishes that a new session is due.
        let blocks = blocks.unwrap_or_else(|| {
            let by_id = collect_context_blocks(&origin, &StyleChain::default_chain());
            let mut seen = HashSet::new();
            let mut blocks = Vec::new();
            let _ = origin.map_content(&mut |node| {
                if let Content::ContextBlock(elem) = node {
                    if seen.insert(elem.id) {
                        if let Some(pair) = by_id.get(&elem.id) {
                            blocks.push(pair.clone());
                        }
                    }
                }
                Ok(None)
            });
            blocks
        });
        let next_id = blocks
            .iter()
            .map(|(e, _)| e.id)
            .max()
            .and_then(|id| id.checked_add(1))
            .unwrap_or(0);
        let mut session = Self {
            world,
            source,
            metrics: FallbackFontMetrics::new(world),
            target,
            features,
            full_error,
            origin,
            original_intr: intr.clone(),
            nodes: Vec::new(),
            roots: Vec::new(),
            retired: Vec::new(),
            next_id,
            impossible,
            bundle_html_targets: None,
            bundle_candidate: None,
        };
        for (elem, chain) in blocks {
            let index = session.add(elem, chain, None);
            session.roots.push(index);
        }
        session
    }

    fn add(
        &mut self,
        elem: Arc<ContextBlockElem>,
        chain: StyleChain,
        parent: Option<usize>,
    ) -> usize {
        let index = self.nodes.len();
        let target =
            parent.map(|parent| self.nodes[parent].target).unwrap_or(self.target);
        let selected = self.bundle_html_targets.is_some();
        self.nodes.push(Producer {
            elem,
            chain,
            parent,
            children: Vec::new(),
            current: None,
            selected,
            selected_filtered_counter: false,
            selected_html_position: false,
            invalid: true,
            active: true,
            target,
        });
        index
    }

    fn execute(&mut self, index: usize, input: &TagIntrospector, location: Location) {
        let node = &self.nodes[index];
        let mut ctx = EvalContext::new();
        ctx.in_context = true;
        ctx.introspector = input.clone();
        ctx.current_location = Some(location);
        ctx.target = node.target;
        // Discovery and never-selected producers retain EvalContext::new's
        // legacy options. HTML is a distinct causal target: its closures must
        // retain the feature set selected by the compile invocation.
        if node.selected || node.target == EvalTarget::Html {
            ctx.features = self.features;
            ctx.full_error = self.full_error;
        }
        ctx.next_context_id = self.next_id;
        let mut scopes = Scopes::new(None);
        let mut styles = node.chain.clone();
        let mut show_rules: Arc<[ShowRule]> = Arc::from([]);
        let mut active_guards = Vec::new();
        let mut sink = TypstSink::new();
        let route = Route::root().with_id(self.source.id());
        let result = {
            let mut tracked = sink.track_mut();
            let mut local = TrackedMut::reborrow_mut(&mut tracked);
            let mut engine = Engine {
                world: self.world,
                font_metrics: &self.metrics,
                route: route.track(),
                styles: &mut styles,
                show_rules: &mut show_rules,
                active_guards: &mut active_guards,
                current_file: self.source.id(),
                sink: &mut local,
            };
            apply_func(
                node.elem.closure.clone(),
                Args::positional(vec![]),
                &mut scopes,
                &mut ctx,
                &mut engine,
            )
            .map(|v| value_to_content(&v))
        };
        self.next_id = ctx.next_context_id;
        let selected_filtered_counter = ctx.has_filtered_counter_reads();
        let selected_html_position =
            ctx.target == EvalTarget::Html && ctx.has_location_position_reads();
        let execution = Box::new(Execution {
            ctx,
            input: input.clone(),
            result,
            sink,
            location,
            final_styles: styles,
        });
        let node = &mut self.nodes[index];
        node.selected |= selected_filtered_counter || selected_html_position;
        node.selected_filtered_counter |= selected_filtered_counter;
        node.selected_html_position |= selected_html_position;
        node.invalid = false;
        if let Some(previous) = node.current.replace(execution) {
            self.retired.push(previous);
        }
    }

    fn discover(&mut self) -> bool {
        let input = self.original_intr.clone();
        for index in self.roots.clone() {
            if let Some(location) =
                input.context_block_locations.get(&self.nodes[index].elem.id).copied()
            {
                self.execute(index, &input, location);
                if self.nodes[index].target == EvalTarget::Html && !self.impossible {
                    self.realize(index, &input);
                }
            }
        }
        self.nodes.iter().any(|n| n.selected)
    }

    fn body(&self, index: usize) -> Content {
        let node = &self.nodes[index];
        let Some(execution) = &node.current else { return Content::Empty };
        let Ok(body) = &execution.result else { return Content::Empty };
        let resolved = node
            .children
            .iter()
            .filter(|&&i| self.nodes[i].active)
            .map(|&i| (self.nodes[i].elem.id, self.body(i)))
            .collect();
        substitute_context_blocks(body.clone(), &resolved)
    }

    fn content(&self) -> Content {
        let resolved = self
            .roots
            .iter()
            .map(|&i| (self.nodes[i].elem.id, self.body(i)))
            .collect();
        substitute_context_blocks(self.origin.clone(), &resolved)
    }

    fn order_into(&self, index: usize, order: &mut Vec<usize>) {
        if !self.nodes[index].active {
            return;
        }
        order.push(index);
        for &child in &self.nodes[index].children {
            self.order_into(child, order);
        }
    }

    fn order(&self) -> Vec<usize> {
        let mut order = Vec::new();
        for &root in &self.roots {
            self.order_into(root, &mut order);
        }
        order
    }

    fn errors(&self) -> Vec<SourceDiagnostic> {
        self.order()
            .into_iter()
            .filter_map(|i| self.nodes[i].current.as_ref())
            .filter_map(|e| e.result.as_ref().err())
            .flatten()
            .cloned()
            .collect()
    }

    fn warnings(&self) -> Vec<SourceDiagnostic> {
        self.order()
            .into_iter()
            .filter_map(|i| self.nodes[i].current.as_ref())
            .flat_map(|e| e.sink.clone().into_diagnostics())
            .collect()
    }

    fn fail(&self, errors: Vec<SourceDiagnostic>) -> SourceResult<Prepared> {
        let originals = self.errors();
        Err(if originals.is_empty() { errors } else { originals })
    }

    fn discard_descendants(&mut self, index: usize) {
        let children = std::mem::take(&mut self.nodes[index].children);
        for child in children {
            self.discard_descendants(child);
            self.nodes[child].active = false;
            if let Some(old) = self.nodes[child].current.take() {
                self.retired.push(old);
            }
        }
    }

    fn realize(&mut self, index: usize, input: &TagIntrospector) {
        if self.nodes[index].invalid {
            self.discard_descendants(index);
            // Locations come from the real provisional tree, never the local id.
            let topology = introspect_with_introspector(&self.content());
            let Some(location) = topology
                .context_block_locations
                .get(&self.nodes[index].elem.id)
                .copied()
            else {
                self.impossible = true;
                return;
            };
            self.execute(index, input, location);
        }
        if self.nodes[index].children.is_empty() {
            let node = &self.nodes[index];
            let children = node
                .current
                .as_ref()
                .and_then(|e| e.result.as_ref().ok())
                .map(|body| ordered_blocks(body, &node.chain));
            match children {
                Some(Some(blocks)) => {
                    for (elem, chain) in blocks {
                        // The allocator is compilation-local across body evaluations.
                        // A retained producer is its actual object, not this integer.
                        if self.nodes.iter().any(|n| n.active && n.elem.id == elem.id) {
                            self.impossible = true;
                            return;
                        }
                        let child = self.add(elem, chain, Some(index));
                        self.nodes[index].children.push(child);
                    }
                }
                Some(None) => {
                    self.impossible = true;
                    return;
                }
                None => {}
            }
        }
        for child in self.nodes[index].children.clone() {
            self.realize(child, input);
            if self.impossible {
                return;
            }
        }
    }

    fn with_replay_engine<T>(
        &self,
        index: usize,
        run: impl FnOnce(&EvalContext, &mut Engine<'_>) -> SourceResult<T>,
    ) -> SourceResult<T> {
        let node = &self.nodes[index];
        let execution = node.current.as_ref().expect("executed selected producer");
        let mut styles = node.chain.clone();
        let mut show_rules: Arc<[ShowRule]> = Arc::from([]);
        let mut active_guards = Vec::new();
        let mut sink = TypstSink::new();
        let route = Route::root().with_id(self.source.id());
        let mut tracked = sink.track_mut();
        let mut local = TrackedMut::reborrow_mut(&mut tracked);
        let mut engine = Engine {
            world: self.world,
            font_metrics: &self.metrics,
            route: route.track(),
            styles: &mut styles,
            show_rules: &mut show_rules,
            active_guards: &mut active_guards,
            current_file: self.source.id(),
            sink: &mut local,
        };
        // This sink, and the L1 replay transaction inside it, are never published.
        run(&execution.ctx, &mut engine)
    }

    fn stabilize(
        &mut self,
        module: Option<&Module>,
    ) -> (SourceResult<Prepared>, Vec<SourceDiagnostic>) {
        for index in self.order() {
            if self.nodes[index].selected {
                if let Some(discovery) = self.nodes[index].current.take() {
                    self.retired.push(discovery);
                }
                self.nodes[index].invalid = true;
            }
        }
        let mut seed =
            if self.target == EvalTarget::Html && self.bundle_html_targets.is_none() {
                self.original_intr.clone()
            } else {
                TagIntrospector::empty()
            };
        seed.inject_html_positions(HashMap::new());
        let mut history = vec![seed];
        for attempt in 1..=5 {
            let input = history.last().unwrap();
            for root in self.roots.clone() {
                self.realize(root, input);
                if self.impossible {
                    return (terminal(self.source, self.errors()), self.warnings());
                }
            }
            if self.target == EvalTarget::Html
                && self.bundle_html_targets.is_none()
                && self
                    .nodes
                    .iter()
                    .any(|node| node.active && node.selected_filtered_counter)
            {
                return (terminal(self.source, self.errors()), self.warnings());
            }
            let content = self.content();
            let (runtime, mut phase_warnings) = if self.bundle_html_targets.is_some() {
                let Content::Sequence(parts) = &content else {
                    return (terminal(self.source, Vec::new()), self.warnings());
                };
                let (result, warnings) = introspect_bundle_with_runtime_for_pipeline(
                    parts.as_ref(),
                    self.world,
                    self.source,
                );
                (result.map(|(intr, _starts)| intr), warnings)
            } else {
                introspect_with_runtime_for_pipeline(&content, self.world, self.source)
            };
            let mut candidate = match runtime {
                Ok(intr) => intr,
                Err(error) => return (self.fail(error), self.warnings()),
            };
            if let Some(html_targets) = &self.bundle_html_targets {
                let page_warnings = match bundle_page_views(
                    &content,
                    html_targets,
                    &mut candidate,
                    input,
                    self.world,
                    self.source,
                ) {
                    Ok(warnings) => warnings,
                    Err(errors) => return (self.fail(errors), self.warnings()),
                };
                phase_warnings.extend(page_warnings);
            }
            let html_positions_selected =
                self.nodes.iter().any(|node| node.selected_html_position)
                    || !input.html_positions.is_empty();
            if html_positions_selected {
                let positions = if let Some(html_targets) = &self.bundle_html_targets {
                    let Content::Sequence(parts) = &content else {
                        return (terminal(self.source, Vec::new()), self.warnings());
                    };
                    let html_content = Content::sequence(
                        parts
                            .iter()
                            .zip(html_targets)
                            .filter(|(_, html)| **html)
                            .map(|(body, _)| body.clone())
                            .collect(),
                    );
                    html_frame_positions(
                        &html_content,
                        &candidate,
                        self.metrics.clone(),
                        self.source,
                    )
                } else {
                    html_frame_positions(
                        &content,
                        &candidate,
                        self.metrics.clone(),
                        self.source,
                    )
                };
                match positions {
                    Ok(positions) => candidate.inject_html_positions(positions),
                    Err(errors) => return (self.fail(errors), self.warnings()),
                }
            }

            let mut document = None;
            if let Some(module) = module {
                for (key, style) in module.bibliography_styles() {
                    candidate.bib_store.add_style(*key, style.clone());
                }
                let logical_start = candidate
                    .counter_final_values(
                        &typst_core::entities::counter::CounterKey::Page,
                    )
                    .and_then(|v| v.first().copied())
                    .unwrap_or(1);
                candidate.inject_positions(input.positions.clone());
                candidate.inject_pages(input.page_store.clone());
                let (doc, callback_warnings) = match layout_with_realized_math_callbacks(
                    &content,
                    candidate.clone(),
                    self.world,
                    self.source,
                ) {
                    Ok(pair) => pair,
                    Err(errors) => return (self.fail(errors), self.warnings()),
                };
                phase_warnings.extend(callback_warnings);
                let references = page_reference_pages(&content, &doc);
                let (pages, numbering_warnings) = match realize_page_numberings(
                    self.world,
                    self.source,
                    &doc,
                    logical_start,
                    &references,
                ) {
                    Ok(pair) => pair,
                    Err(errors) => return (self.fail(errors), self.warnings()),
                };
                phase_warnings.extend(numbering_warnings);
                candidate.inject_positions(doc.extracted_positions.clone());
                candidate.inject_pages(pages);
                document = Some(doc);
            }
            // P1159 participates in this same budget even when selected reads
            // are page-independent. The first D cannot consume its own newly
            // realized numbering views: they belong to the next layout.
            let (mut valid, mut closed) =
                if module.is_some() || self.bundle_html_targets.is_some() {
                    pagination_stability(input, &candidate)
                } else {
                    (true, true)
                };
            if html_positions_selected {
                let html_same = input.html_positions == candidate.html_positions;
                valid &= html_same;
                closed &= html_same;
            }
            for index in self.order() {
                if !self.nodes[index].selected {
                    continue;
                }
                let execution = self.nodes[index].current.as_ref().unwrap();
                let same_location =
                    candidate.context_block_locations.get(&self.nodes[index].elem.id)
                        == Some(&execution.location);
                let reflexive = self.with_replay_engine(index, |ctx, engine| {
                    ctx.context_reads_valid_for(&execution.input, engine)
                });
                let checked = self.with_replay_engine(index, |ctx, engine| {
                    ctx.context_reads_valid_for(&candidate, engine)
                });
                match (reflexive, checked) {
                    (Ok(proven), Ok(same)) => {
                        closed &= proven;
                        self.nodes[index].invalid = !same || !same_location || !proven;
                        valid &= !self.nodes[index].invalid;
                    }
                    (Err(errors), _) | (_, Err(errors)) => {
                        return (self.fail(errors), self.warnings())
                    }
                }
            }
            history.push(candidate.clone());
            if valid || attempt == 5 {
                let mut warnings = self.warnings();
                let errors = self.errors();
                if !errors.is_empty() {
                    return (Err(errors), warnings);
                }
                if !valid && !closed {
                    return (terminal(self.source, errors), warnings);
                }
                if !valid {
                    let mut details = Vec::new();
                    for index in self.order() {
                        if self.nodes[index].selected {
                            match self.with_replay_engine(index, |ctx, engine| {
                                ctx.context_nonconvergence_diagnostics(&history, engine)
                            }) {
                                Ok(found) => details.extend(found),
                                Err(errors) => return (Err(errors), warnings),
                            }
                        }
                    }
                    if !details.is_empty() {
                        warnings.push(
                            SourceDiagnostic::warning(
                                Span::detached(),
                                "document did not converge within five attempts",
                            )
                            .with_hint(format!(
                                "see {} additional warnings for more details",
                                details.len()
                            ))
                            .with_hint("see https://typst.app/help/convergence for help"),
                        );
                        warnings.extend(details);
                    }
                }
                warnings.extend(phase_warnings);
                if self.bundle_html_targets.is_some() {
                    self.bundle_candidate = Some(candidate.clone());
                }
                return (
                    Ok(Prepared {
                        content,
                        settled: document.map(|doc| (candidate, doc)),
                    }),
                    warnings,
                );
            }
        }
        unreachable!("the final attempt returns its own result")
    }
}

pub(super) fn prepare(
    content: Content,
    intr: &TagIntrospector,
    world: &dyn World,
    source: &Source,
    full_error: bool,
    features: Features,
    module: Option<&Module>,
) -> (SourceResult<Prepared>, Vec<SourceDiagnostic>) {
    let mut session = Session::new(
        content,
        intr,
        world,
        source,
        full_error,
        features,
        EvalTarget::Paged,
    );
    if session.discover() {
        if session.impossible {
            (terminal(source, session.errors()), session.warnings())
        } else {
            session.stabilize(module)
        }
    } else {
        let errors = session.errors();
        // No new cycle and no new warning policy for the unselected legacy path.
        let result = if errors.is_empty() {
            Ok(Prepared { content: session.content(), settled: None })
        } else {
            Err(errors)
        };
        (result, Vec::new())
    }
}

/// Prepare contextual content for the semantic HTML target without entering
/// paginated stabilization. Markers retained for layout locations are removed
/// only after their corresponding producer has been executed successfully.
pub(super) fn prepare_html(
    content: Content,
    intr: &TagIntrospector,
    world: &dyn World,
    source: &Source,
    full_error: bool,
    features: Features,
) -> (SourceResult<Content>, Vec<SourceDiagnostic>) {
    let mut session = Session::new(
        content,
        intr,
        world,
        source,
        full_error,
        features,
        EvalTarget::Html,
    );
    session.discover();
    let selected_filtered_counter = session
        .nodes
        .iter()
        .any(|node| node.active && node.selected_filtered_counter);
    let selected_html_position = session
        .nodes
        .iter()
        .any(|node| node.active && node.selected_html_position);
    if session.impossible || selected_filtered_counter {
        return (terminal(source, session.errors()), session.warnings());
    }

    let (prepared_content, warnings) = if selected_html_position {
        let (prepared, warnings) = session.stabilize(None);
        let prepared = match prepared {
            Ok(prepared) => prepared,
            Err(errors) => return (Err(errors), warnings),
        };
        (prepared.content, warnings)
    } else {
        let errors = session.errors();
        let warnings = session.warnings();
        if !errors.is_empty() {
            return (Err(errors), warnings);
        }
        (session.content(), warnings)
    };

    let executed: Vec<_> = session
        .nodes
        .iter()
        .filter(|node| node.active && node.current.is_some())
        .map(|node| node.elem.clone())
        .collect();
    let prepared =
        strip_executed_html_markers(prepared_content, &executed, source, false);
    (prepared, warnings)
}

/// Prepara todos os bodies contra uma única geração causal antes de exportar.
pub(super) fn prepare_bundle(
    bodies: Vec<Content>,
    html_targets: &[bool],
    intr: &TagIntrospector,
    world: &dyn World,
    source: &Source,
    full_error: bool,
    features: Features,
) -> (
    SourceResult<(
        Vec<Content>,
        TagIntrospector,
        Vec<typst_core::entities::locator::LocatorSeed>,
    )>,
    Vec<SourceDiagnostic>,
) {
    if bodies.len() != html_targets.len() {
        return (terminal(source, Vec::new()), Vec::new());
    }
    let origin = Content::Sequence(bodies.clone().into());
    let mut session = Session::new(
        origin,
        intr,
        world,
        source,
        full_error,
        features,
        EvalTarget::Paged,
    );
    session.bundle_html_targets = Some(html_targets.to_vec());
    let mut targets = HashMap::new();
    for (body, &html) in bodies.iter().zip(html_targets) {
        let Some(blocks) = ordered_blocks(body, &StyleChain::default_chain()) else {
            session.impossible = true;
            break;
        };
        for (elem, _) in blocks {
            if targets
                .insert(elem.id, if html { EvalTarget::Html } else { EvalTarget::Paged })
                .is_some()
            {
                session.impossible = true;
            }
        }
    }
    for node in &mut session.nodes {
        if let Some(target) = targets.get(&node.elem.id) {
            node.target = *target;
        }
    }
    if session.impossible {
        return (terminal(source, session.errors()), session.warnings());
    }
    session.discover();
    if session.impossible {
        return (terminal(source, session.errors()), session.warnings());
    }
    // Um produtor posterior pode criar um elemento visto por query anterior,
    // mesmo sem filtered-counter read. No bundle, todo produtor participa
    // do mesmo fixpoint global.
    let selected = !session.roots.is_empty();
    if selected {
        for node in &mut session.nodes {
            node.selected = true;
        }
    }
    let (prepared, mut warnings) = if selected {
        session.stabilize(None)
    } else {
        let errors = session.errors();
        let warnings = session.warnings();
        let prepared = if errors.is_empty() {
            Ok(Prepared { content: session.content(), settled: None })
        } else {
            Err(errors)
        };
        (prepared, warnings)
    };
    let prepared = match prepared {
        Ok(prepared) => prepared,
        Err(errors) => return (Err(errors), warnings),
    };
    let raw_bodies = match prepared.content {
        Content::Sequence(parts) if parts.len() == bodies.len() => parts.to_vec(),
        _ => return (terminal(source, Vec::new()), warnings),
    };
    let (pure, starts) =
        typst_core::compiler::introspect::introspect_bundle_documents_with_starts(
            &raw_bodies,
        );
    let (runtime, runtime_warnings) =
        introspect_bundle_with_runtime_for_pipeline(&raw_bodies, world, source);
    if !selected {
        warnings.extend(runtime_warnings);
    }
    let (runtime, runtime_starts) = match runtime {
        Ok(pair) => pair,
        Err(errors) => return (Err(errors), warnings),
    };
    let same_locations = runtime_starts == starts
        && pure.elements.keys().collect::<HashSet<_>>()
            == runtime.elements.keys().collect::<HashSet<_>>()
        && pure.context_block_locations == runtime.context_block_locations
        && pure.kind_index == runtime.kind_index
        && pure.parent_locations == runtime.parent_locations;
    if !same_locations {
        return (terminal(source, Vec::new()), warnings);
    }
    let mut runtime = session.bundle_candidate.take().unwrap_or(runtime);
    if !selected {
        // P1486: sem produtores contextuais, a Session não entrou em
        // stabilize(). O PDF ainda precisa do PageStore final para os
        // rótulos, e os bodies HTML não devem consumir páginas.
        let content = Content::Sequence(raw_bodies.clone().into());
        match bundle_page_views(
            &content,
            html_targets,
            &mut runtime,
            &TagIntrospector::empty(),
            world,
            source,
        ) {
            Ok(page_warnings) => warnings.extend(page_warnings),
            Err(errors) => return (Err(errors), warnings),
        }
    }
    let executed_html: Vec<_> = session
        .nodes
        .iter()
        .filter(|node| {
            node.active && node.target == EvalTarget::Html && node.current.is_some()
        })
        .map(|node| node.elem.clone())
        .collect();
    let mut final_bodies = Vec::with_capacity(raw_bodies.len());
    for (body, &html) in raw_bodies.into_iter().zip(html_targets) {
        if html {
            match strip_executed_html_markers(body, &executed_html, source, true) {
                Ok(body) => final_bodies.push(body),
                Err(errors) => return (Err(errors), warnings),
            }
        } else {
            final_bodies.push(body);
        }
    }
    (Ok((final_bodies, runtime, starts)), warnings)
}

/// Compoe visoes paginadas dos bodies sem layout do target HTML.
fn bundle_page_views(
    content: &Content,
    html_targets: &[bool],
    candidate: &mut TagIntrospector,
    input: &TagIntrospector,
    world: &dyn World,
    source: &Source,
) -> SourceResult<Vec<SourceDiagnostic>> {
    use std::num::NonZeroUsize;
    use typst_core::entities::page_store::PageStore;
    use typst_core::entities::sealed_positions::SealedPositions;

    let Content::Sequence(parts) = content else {
        return terminal(source, Vec::new());
    };
    if parts.len() != html_targets.len() {
        return terminal(source, Vec::new());
    }
    let bodies = parts.to_vec();
    let (pure, starts) =
        typst_core::compiler::introspect::introspect_bundle_documents_with_starts(
            &bodies,
        );
    if pure.elements.keys().collect::<HashSet<_>>()
        != candidate.elements.keys().collect::<HashSet<_>>()
        || pure.context_block_locations != candidate.context_block_locations
        || pure.kind_index != candidate.kind_index
    {
        return terminal(source, Vec::new());
    }
    candidate.inject_positions(input.positions.clone());
    candidate.inject_pages(input.page_store.clone());

    let locations: HashSet<Location> = candidate
        .elements
        .keys()
        .copied()
        .chain(candidate.kind_index.values().flatten().copied())
        .chain(candidate.context_block_locations.values().copied())
        .chain(candidate.parent_locations.keys().copied())
        .chain(candidate.parent_locations.values().copied())
        .collect();
    let mut positions = HashMap::new();
    let mut numberings = Vec::new();
    let mut logical_numbers = Vec::new();
    let mut numbering_spans = Vec::new();
    let mut visible_numberings = Vec::new();
    let mut reference_numberings = Vec::new();
    let mut supplements = Vec::new();
    let mut warnings = Vec::new();
    let mut offset = 0usize;
    let starts_raw: Vec<_> = starts
        .iter()
        .map(|&seed| {
            typst_core::entities::locator::Locator::from_seed(seed)
                .next()
                .as_u128()
        })
        .collect();
    let counter_updates = candidate
        .query_by_kind(typst_core::entities::element_kind::ElementKind::CounterUpdate);
    for (index, ((body, &html), &start)) in
        bodies.iter().zip(html_targets).zip(&starts).enumerate()
    {
        if html {
            continue;
        }
        let (doc, callback_warnings) = layout_with_realized_math_callbacks_from(
            body,
            candidate.clone(),
            world,
            source,
            start,
        )?;
        warnings.extend(callback_warnings);
        let references = page_reference_pages(body, &doc);
        // O início lógico Paged é o valor Page após as ações deste body,
        // já resolvidas pelo único snapshot runtime. A última ocorrência de
        // CounterUpdate delimita uma leitura dentro da faixa [start, next).
        // Sem ocorrência, a fronteira lógica no próprio start fornece 1.
        let upper = starts_raw.get(index + 1).copied().unwrap_or(u128::MAX);
        let observation = counter_updates
            .iter()
            .filter(|loc| {
                let raw = loc.as_u128();
                raw >= starts_raw[index] && raw < upper
            })
            .max_by_key(|loc| loc.as_u128())
            .copied()
            .unwrap_or_else(|| {
                typst_core::entities::locator::Locator::from_seed(start).next()
            });
        let logical_start = candidate
            .counter_values_at(
                &typst_core::entities::counter::CounterKey::Page,
                observation,
            )
            .and_then(|values| values.first().copied())
            .unwrap_or(1);
        let (pages, numbering_warnings) =
            realize_page_numberings(world, source, &doc, logical_start, &references)?;
        warnings.extend(numbering_warnings);
        for &location in &locations {
            if let Some(position) = doc.extracted_positions.position_of(location) {
                let shifted = typst_core::entities::position::Position {
                    page: NonZeroUsize::new(position.page.get() + offset)
                        .expect("page number remains nonzero"),
                    point: position.point,
                };
                if positions.insert(location, shifted).is_some() {
                    return terminal(source, Vec::new());
                }
            }
        }
        for page in 1..=doc.pages.len() {
            let page = NonZeroUsize::new(page).unwrap();
            numberings.push(pages.numbering_for_page(page).cloned());
            logical_numbers
                .push(pages.logical_number_for_page(page).unwrap_or(page.get()));
            numbering_spans
                .push(pages.numbering_span_for_page(page).unwrap_or(Span::detached()));
            visible_numberings.push(pages.visible_numbering_for_page(page).cloned());
            reference_numberings.push(pages.reference_numbering_for_page(page).cloned());
            supplements
                .push(pages.supplement_for_page(page).cloned().unwrap_or(Content::Empty));
        }
        offset += doc.pages.len();
    }
    candidate.inject_positions(SealedPositions::from_runtime(positions));
    let store = if let Some(total) = NonZeroUsize::new(offset) {
        PageStore::from_realized(
            total,
            numberings,
            logical_numbers,
            numbering_spans,
            visible_numberings,
            reference_numberings,
            supplements,
        )
    } else {
        PageStore::empty()
    };
    candidate.inject_pages(store);
    Ok(warnings)
}

fn strip_executed_html_markers(
    content: Content,
    executed: &[Arc<ContextBlockElem>],
    source: &Source,
    strip_counter_updates: bool,
) -> SourceResult<Content> {
    content.map_content(&mut |node| {
        if let Content::ContextBlock(elem) = node {
            if executed.iter().any(|known| Arc::ptr_eq(known, elem)) {
                return Ok(Some(Content::Empty));
            }
            return Err(vec![SourceDiagnostic::error(
                source.root().span(),
                "HTML contextual expansion left unresolved content",
            )]);
        }
        // Update já participou do snapshot validado; não possui render HTML.
        if strip_counter_updates && matches!(node, Content::CounterUpdate(_)) {
            return Ok(Some(Content::Empty));
        }
        if let Content::Sequence(parts) = node {
            let retained: Vec<_> = parts
                .iter()
                .filter(|part| !matches!(part, Content::Empty))
                .cloned()
                .collect();
            if retained.iter().all(|part| matches!(part, Content::Space | Content::SpaceFromNewline)) {
                return Ok(Some(Content::Empty));
            }
            if retained.len() != parts.len() {
                return Ok(Some(Content::sequence(retained)));
            }
        }
        Ok(None)
    })
}

/// Conservative equality of the actual layout input views, not a language
/// equality for counter keys or a comparison of newly executed closures.
/// Coverage is checked so an unenumerated position cannot certify stability.
fn pagination_stability(
    before: &TagIntrospector,
    after: &TagIntrospector,
) -> (bool, bool) {
    fn locations(intr: &TagIntrospector) -> HashSet<Location> {
        intr.elements
            .keys()
            .copied()
            .chain(intr.kind_index.values().flatten().copied())
            .chain(intr.context_block_locations.values().copied())
            .chain(intr.parent_locations.keys().copied())
            .chain(intr.parent_locations.values().copied())
            .collect()
    }
    let mut keys = locations(before);
    keys.extend(locations(after));
    let covered = [before, after].into_iter().all(|intr| {
        keys.iter()
            .filter(|&&key| intr.positions.position_of(key).is_some())
            .count()
            == intr.positions.len()
    });
    let same = covered
        && before.page_store == after.page_store
        && before.positions.len() == after.positions.len()
        && keys.into_iter().all(|key| {
            before.positions.position_of(key) == after.positions.position_of(key)
        });
    (same, covered)
}

pub(super) fn expand(
    content: Content,
    intr: &TagIntrospector,
    world: &dyn World,
    source: &Source,
) -> SourceResult<Content> {
    prepare(content, intr, world, source, false, Features::default(), None)
        .0
        .map(|prepared| prepared.content)
}

/// Keep an unselected paginated compilation on its legacy path. Relayout
/// cannot start an independent selective attempt budget after this decision.
pub(super) fn expand_legacy(
    content: Content,
    intr: &TagIntrospector,
    world: &dyn World,
    source: &Source,
) -> SourceResult<Content> {
    let mut session = Session::new(
        content,
        intr,
        world,
        source,
        false,
        Features::default(),
        EvalTarget::Paged,
    );
    session.discover();
    let errors = session.errors();
    if errors.is_empty() {
        Ok(session.content())
    } else {
        Err(errors)
    }
}

#[cfg(test)]
pub(super) fn discovered_options_for_test(
    world: &dyn World,
    source: &Source,
    features: Features,
) -> Vec<(bool, Features)> {
    let module = eval_to_module_with_sink_target_features(
        world,
        source,
        true,
        EvalTarget::Paged,
        features,
    )
    .0
    .unwrap();
    let content = module.content().unwrap().clone();
    let intr = introspect_with_introspector(&content);
    let mut session =
        Session::new(content, &intr, world, source, true, features, EvalTarget::Paged);
    session.discover();
    session
        .nodes
        .iter()
        .filter_map(|n| n.current.as_ref())
        .map(|e| (e.ctx.full_error, e.ctx.features))
        .collect()
}

#[cfg(test)]
mod p1494_tests {
    use super::*;
    use std::num::NonZeroU16;
    use typst_core::entities::file_id::FileId;

    #[test]
    fn strip_terminal_colapsa_somente_sequencia_so_de_espacos() {
        let id = FileId::from_raw(NonZeroU16::new(1).unwrap());
        let source = Source::new(id, String::new());
        let spaces = Content::sequence(vec![
            Content::Space,
            Content::SpaceFromNewline,
            Content::Space,
        ]);
        let stripped = strip_executed_html_markers(spaces, &[], &source, false)
            .expect("sem markers pendentes");
        assert!(matches!(stripped, Content::Empty));

        let with_text = Content::sequence(vec![
            Content::text("A"),
            Content::SpaceFromNewline,
            Content::text("B"),
        ]);
        let retained = strip_executed_html_markers(with_text, &[], &source, false)
            .expect("conteúdo textual preservado");
        let Content::Sequence(items) = retained else {
            panic!("sequência com texto não deve colapsar");
        };
        assert!(
            items.iter().any(|item| matches!(item, Content::SpaceFromNewline)),
            "a origem deve sobreviver ao strip enquanto há texto"
        );
    }
}
