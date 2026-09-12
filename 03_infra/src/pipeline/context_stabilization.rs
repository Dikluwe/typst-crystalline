//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/pipeline/context_stabilization.md
//! @prompt-hash d948e9a3
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
    invalid: bool,
    active: bool,
}

struct Session<'a> {
    world: &'a dyn World,
    source: &'a Source,
    metrics: FallbackFontMetrics<'a>,
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
            features,
            full_error,
            origin,
            original_intr: intr.clone(),
            nodes: Vec::new(),
            roots: Vec::new(),
            retired: Vec::new(),
            next_id,
            impossible,
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
        self.nodes.push(Producer {
            elem,
            chain,
            parent,
            children: Vec::new(),
            current: None,
            selected: false,
            invalid: true,
            active: true,
        });
        index
    }

    fn execute(&mut self, index: usize, input: &TagIntrospector, location: Location) {
        let node = &self.nodes[index];
        let mut ctx = EvalContext::new();
        ctx.in_context = true;
        ctx.introspector = input.clone();
        ctx.current_location = Some(location);
        ctx.target = EvalTarget::Paged;
        // Discovery and never-selected producers retain EvalContext::new's
        // legacy options. Do not repair feature propagation before selection.
        if node.selected {
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
        let selected = ctx.has_filtered_counter_reads();
        let execution = Box::new(Execution {
            ctx,
            input: input.clone(),
            result,
            sink,
            location,
            final_styles: styles,
        });
        let node = &mut self.nodes[index];
        node.selected |= selected;
        node.invalid = false;
        if let Some(previous) = node.current.replace(execution) {
            self.retired.push(previous);
        }
    }

    fn discover(&mut self) -> bool {
        for index in self.roots.clone() {
            if let Some(location) = self
                .original_intr
                .context_block_locations
                .get(&self.nodes[index].elem.id)
                .copied()
            {
                self.execute(index, &self.original_intr.clone(), location);
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
        for index in self.roots.clone() {
            if self.nodes[index].selected {
                if let Some(discovery) = self.nodes[index].current.take() {
                    self.retired.push(discovery);
                }
                self.nodes[index].invalid = true;
            }
        }
        let mut history = vec![TagIntrospector::empty()];
        for attempt in 1..=5 {
            let input = history.last().unwrap();
            for root in self.roots.clone() {
                self.realize(root, input);
                if self.impossible {
                    return (terminal(self.source, self.errors()), self.warnings());
                }
            }
            let content = self.content();
            let (runtime, mut phase_warnings) =
                introspect_with_runtime_for_pipeline(&content, self.world, self.source);
            let mut candidate = match runtime {
                Ok(intr) => intr,
                Err(error) => return (self.fail(error), self.warnings()),
            };
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
            let (mut valid, mut closed) = if module.is_some() {
                pagination_stability(input, &candidate)
            } else {
                (true, true)
            };
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
    let mut session = Session::new(content, intr, world, source, full_error, features);
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
    let mut session =
        Session::new(content, intr, world, source, false, Features::default());
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
    let mut session = Session::new(content, &intr, world, source, true, features);
    session.discover();
    session
        .nodes
        .iter()
        .filter_map(|n| n.current.as_ref())
        .map(|e| (e.ctx.full_error, e.ctx.features))
        .collect()
}
