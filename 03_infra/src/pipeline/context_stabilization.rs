//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/pipeline/context_stabilization.md
//! @prompt-hash e8bb9400
//! @layer L3
//! @updated 2026-10-09
//!
//! A compilation-local owner of contextual contributions and their lifetimes.

use super::*;
use typst_core::compiler::realize::context_occurrences::{
    ContextOccurrenceArena, ContextOccurrenceFrame,
};
use typst_core::entities::compiler_features::Features;
use typst_core::entities::introspector::TagIntrospector;
use typst_core::entities::location::Location;

pub(super) struct Prepared<'a> {
    pub content: Content,
    pub settled: Option<(TagIntrospector, PagedDocument, FallbackFontMetrics<'a>)>,
}

struct Execution {
    ctx: EvalContext,
    input: Arc<TagIntrospector>,
    result: SourceResult<Content>,
    sink: TypstSink,
    location: Location,
    // Retains style-owned values until this execution record is dropped.
    _retained_styles: StyleChain,
}

struct Producer {
    occurrences: ContextOccurrenceFrame,
    elem: Arc<ContextBlockElem>,
    chain: StyleChain,
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
    occurrences: Option<ContextOccurrenceArena>,
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
    retired: Vec<Execution>,
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
    if by_id.is_empty() {
        return Some(Vec::new());
    }
    let mut seen = HashSet::new();
    let mut ordered = Vec::new();
    let mut unique = true;
    let _ = content.map_content(&mut |node| {
        let contextual = match node {
            Content::ContextBlock(elem) => Some(elem),
            Content::Located(e) => match &e.body {
                Content::ContextBlock(elem) => Some(elem),
                _ => None,
            },
            _ => None,
        };
        if let Some(elem) = contextual {
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

#[cfg(test)]
mod ordered_blocks_tests {
    use super::*;
    use typst_core::entities::content::RealizedContent;
    use typst_core::entities::func::Func;
    use typst_core::entities::layout_types::TrackSizing;

    fn block(id: u64) -> Content {
        Content::ContextBlock(Arc::new(ContextBlockElem {
            id,
            closure: Func::native("dummy", |_ctx, _args, _world, _file| Ok(Value::None)),
        }))
    }

    #[test]
    fn empty_table_and_hidden_original_have_no_ordered_producers() {
        let table = Content::table(
            vec![TrackSizing::Auto],
            vec![],
            vec![Content::table_cell(Content::text("A"), None, None, None, None)],
        );
        let hidden = Content::Realized(Arc::new(RealizedContent {
            original: block(9),
            visual: Content::text("visible"),
        }));
        let content = Content::sequence(vec![table, hidden]);
        assert!(ordered_blocks(&content, &StyleChain::default_chain())
            .unwrap()
            .is_empty());
    }

    #[test]
    fn duplicate_ids_remain_impossible_and_distinct_ids_keep_order() {
        let chain = StyleChain::default_chain();
        let duplicate = Content::sequence(vec![block(7), block(7)]);
        assert!(ordered_blocks(&duplicate, &chain).is_none());
        let distinct = Content::sequence(vec![block(8), block(3)]);
        let ordered = ordered_blocks(&distinct, &chain).unwrap();
        assert_eq!(
            ordered.iter().map(|(elem, _)| elem.id).collect::<Vec<_>>(),
            vec![8, 3]
        );
    }
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
                let contextual = match node {
                    Content::ContextBlock(elem) => Some(elem),
                    Content::Located(e) => match &e.body {
                        Content::ContextBlock(elem) => Some(elem),
                        _ => None,
                    },
                    _ => None,
                };
                if let Some(elem) = contextual {
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
            occurrences: None,
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
            occurrences: self
                .occurrences
                .get_or_insert_with(|| ContextOccurrenceArena::new(&self.origin))
                .frame(),
            elem,
            chain,
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

    fn execute(
        &mut self,
        index: usize,
        input: &Arc<TagIntrospector>,
        location: Location,
    ) {
        let node = &mut self.nodes[index];
        let mut ctx = EvalContext::new();
        ctx.in_context = true;
        ctx.introspector = input.as_ref().clone();
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
            let closure = node.elem.closure.clone();
            self.occurrences.as_mut().expect("producer has an arena").evaluate(
                &mut node.occurrences,
                &mut ctx,
                |ctx| {
                    apply_func(
                        closure,
                        Args::positional(vec![]),
                        &mut scopes,
                        ctx,
                        &mut engine,
                    )
                    .and_then(|v| {
                        typst_core::compiler::realize::realize_math_insertions(
                            value_to_content(&v),
                            ctx,
                            &mut engine,
                        )
                    })
                    .and_then(|content| {
                        typst_core::compiler::realize::realize_table_insertions(
                            content,
                            ctx,
                            &mut engine,
                        )
                    })
                },
            )
        };
        self.next_id = ctx.next_context_id;
        let selected_filtered_counter = ctx.has_filtered_counter_reads();
        let selected_html_position =
            ctx.target == EvalTarget::Html && ctx.has_location_position_reads();
        // Replay uses the recorded reads and clones each candidate into a
        // fresh context. This evaluation-only view has no later consumer.
        ctx.introspector = TagIntrospector::empty();
        let execution = Box::new(Execution {
            ctx,
            input: Arc::clone(input),
            result,
            sink,
            location,
            _retained_styles: styles,
        });
        let node = &mut self.nodes[index];
        node.selected |= selected_filtered_counter || selected_html_position;
        node.selected_filtered_counter |= selected_filtered_counter;
        node.selected_html_position |= selected_html_position;
        node.invalid = false;
        if let Some(previous) = node.current.replace(execution) {
            self.retired.push(*previous);
        }
    }

    fn discover(&mut self) -> bool {
        let input = Arc::new(self.original_intr.clone());
        for index in self.roots.clone() {
            if let Some(location) =
                input.context_block_locations.get(&self.nodes[index].elem.id).copied()
            {
                self.execute(index, &input, location);
                if !self.impossible {
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
        if self.roots.is_empty() {
            return self.origin.clone();
        }
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

    fn fail(&self, errors: Vec<SourceDiagnostic>) -> SourceResult<Prepared<'a>> {
        let originals = self.errors();
        Err(if originals.is_empty() { errors } else { originals })
    }

    fn discard_descendants(&mut self, index: usize) {
        let children = std::mem::take(&mut self.nodes[index].children);
        for child in children {
            self.discard_descendants(child);
            self.nodes[child].active = false;
            if let Some(old) = self.nodes[child].current.take() {
                self.retired.push(*old);
            }
        }
    }

    fn realize(&mut self, index: usize, input: &Arc<TagIntrospector>) {
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
    ) -> (SourceResult<Prepared<'a>>, Vec<SourceDiagnostic>) {
        for index in self.order() {
            if self.nodes[index].selected {
                if let Some(discovery) = self.nodes[index].current.take() {
                    self.retired.push(*discovery);
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
            let snapshot = Arc::new(history.last().unwrap().clone());
            let input = &snapshot;
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
                super::paged::introspect_with_runtime_for_target(
                    &content,
                    self.world,
                    self.source,
                    self.target,
                    self.features,
                )
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
                candidate.inject_positions(input.positions.clone());
                candidate.inject_pages(input.page_store.clone());
                let (doc, callback_warnings) =
                    match super::paged::layout_with_realized_math_callbacks_with_metrics(
                        &content,
                        candidate.clone(),
                        self.world,
                        self.source,
                        &self.metrics,
                    ) {
                        Ok(pair) => pair,
                        Err(errors) => return (self.fail(errors), self.warnings()),
                    };
                phase_warnings.extend(callback_warnings);
                candidate.inject_positions(doc.extracted_positions.clone());
                let (snapshot, page_warnings) = realize_page_snapshot_for_pipeline(
                    &[(Vec::new(), std::num::NonZeroUsize::new(doc.pages.len()))],
                    &mut candidate,
                    self.world,
                    self.source,
                );
                if let Err(errors) = snapshot {
                    return (self.fail(errors), self.warnings());
                }
                phase_warnings.extend(page_warnings);
                let logical_numbers = (1..=doc.pages.len())
                    .map(|p| {
                        candidate
                            .page_store
                            .logical_number_for_page(
                                std::num::NonZeroUsize::new(p).unwrap(),
                            )
                            .unwrap_or(p)
                    })
                    .collect::<Vec<_>>();
                let logical_final = candidate
                    .page_counter_final_values
                    .values()
                    .next()
                    .and_then(|v| v.first())
                    .copied()
                    .unwrap_or(0);
                let references = page_reference_pages(&content, &doc);
                let (pages, numbering_warnings) = match realize_page_numberings(
                    self.world,
                    self.source,
                    &doc,
                    &logical_numbers,
                    logical_final,
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
            if self.target == EvalTarget::Html && self.bundle_html_targets.is_none() {
                let (snapshot, page_warnings) = realize_page_snapshot_for_pipeline(
                    &[(Vec::new(), None)],
                    &mut candidate,
                    self.world,
                    self.source,
                );
                if let Err(errors) = snapshot {
                    return (self.fail(errors), self.warnings());
                }
                phase_warnings.extend(page_warnings);
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
                        settled: document
                            .map(|doc| (candidate, doc, self.metrics.clone())),
                    }),
                    warnings,
                );
            }
        }
        unreachable!("the final attempt returns its own result")
    }
}

pub(super) fn prepare<'a>(
    content: Content,
    intr: &TagIntrospector,
    world: &'a dyn World,
    source: &'a Source,
    full_error: bool,
    features: Features,
    module: Option<&Module>,
) -> (SourceResult<Prepared<'a>>, Vec<SourceDiagnostic>) {
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
    let mut seed = intr.clone();
    let (snapshot, mut page_warnings) = realize_page_snapshot_for_pipeline(
        &[(Vec::new(), None)],
        &mut seed,
        world,
        source,
    );
    if let Err(errors) = snapshot {
        return (Err(errors), page_warnings);
    }
    let mut session = Session::new(
        content,
        &seed,
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

    let body = session.content();
    let mut page_probe = introspect_with_introspector(&body);
    let (probe, _) = realize_page_snapshot_for_pipeline(
        &[(Vec::new(), None)],
        &mut page_probe,
        world,
        source,
    );
    if let Err(errors) = probe {
        return (Err(errors), session.warnings());
    }
    let mut page_selected = false;
    for index in session.order() {
        let execution = session.nodes[index].current.as_ref().unwrap();
        let mut candidate = execution.input.as_ref().clone();
        candidate.page_counter_values = page_probe.page_counter_values.clone();
        candidate.page_counter_final_values =
            page_probe.page_counter_final_values.clone();
        match session.with_replay_engine(index, |ctx, engine| {
            ctx.context_reads_valid_for(&candidate, engine)
        }) {
            Ok(false) => {
                session.nodes[index].selected = true;
                session.nodes[index].invalid = true;
                page_selected = true;
            }
            Ok(true) => {}
            Err(errors) => return (Err(errors), session.warnings()),
        }
    }
    let (prepared_content, mut warnings) = if selected_html_position || page_selected {
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
    let prepared = strip_executed_html_markers(prepared_content, &executed, source, true);
    if !page_selected {
        page_warnings.append(&mut warnings);
        warnings = page_warnings;
    }
    (prepared, warnings)
}

/// Bodies e identidades pertencentes à mesma geração causal validada.
pub(super) struct PreparedBundle {
    pub(super) bodies: Vec<Content>,
    pub(super) introspector: TagIntrospector,
    pub(super) document_starts: Vec<typst_core::entities::locator::LocatorSeed>,
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
) -> (SourceResult<PreparedBundle>, Vec<SourceDiagnostic>) {
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
    (
        Ok(PreparedBundle {
            bodies: final_bodies,
            introspector: runtime,
            document_starts: starts,
        }),
        warnings,
    )
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
    use typst_core::entities::{
        page_store::PageStore, sealed_positions::SealedPositions,
    };
    let Content::Sequence(parts) = content else {
        return terminal(source, Vec::new());
    };
    if parts.len() != html_targets.len() {
        return terminal(source, Vec::new());
    }
    let (pure, starts) =
        typst_core::compiler::introspect::introspect_bundle_documents_with_starts(parts);
    if pure.elements.keys().collect::<HashSet<_>>()
        != candidate.elements.keys().collect::<HashSet<_>>()
        || pure.context_block_locations != candidate.context_block_locations
        || pure.kind_index != candidate.kind_index
    {
        return terminal(source, Vec::new());
    }
    candidate.inject_positions(input.positions.clone());
    candidate.inject_pages(input.page_store.clone());
    let locations = snapshot_locations(candidate);
    let mut positions = HashMap::new();
    let mut docs = Vec::new();
    let mut descriptors = Vec::new();
    let mut warnings = Vec::new();
    let mut offset = 0;
    for ((body, &html), &start) in parts.iter().zip(html_targets).zip(&starts) {
        let boundary = typst_core::entities::locator::Locator::from_seed(start).next();
        if html {
            docs.push(None);
            descriptors.push((vec![boundary], None));
            continue;
        }
        let (doc, found) = layout_with_realized_math_callbacks_from(
            body,
            candidate.clone(),
            world,
            source,
            start,
        )?;
        warnings.extend(found);
        for &loc in &locations {
            if let Some(p) = doc.extracted_positions.position_of(loc) {
                let p = typst_core::entities::position::Position {
                    page: NonZeroUsize::new(p.page.get() + offset).unwrap(),
                    point: p.point,
                };
                if positions.insert(loc, p).is_some() {
                    return terminal(source, Vec::new());
                }
            }
        }
        descriptors.push((vec![boundary], NonZeroUsize::new(doc.pages.len())));
        offset += doc.pages.len();
        docs.push(Some(doc));
    }
    candidate.inject_positions(SealedPositions::from_runtime(positions));
    let (snapshot, found) =
        realize_page_snapshot_for_pipeline(&descriptors, candidate, world, source);
    snapshot?;
    warnings.extend(found);
    let mut numberings = Vec::new();
    let mut numbers = Vec::new();
    let mut spans = Vec::new();
    let mut visible = Vec::new();
    let mut reference = Vec::new();
    let mut supplements = Vec::new();
    let mut offset = 0;
    for (body, doc) in parts.iter().zip(docs) {
        let Some(doc) = doc else { continue };
        let logical = (1..=doc.pages.len())
            .map(|p| {
                candidate
                    .page_store
                    .logical_number_for_page(NonZeroUsize::new(p + offset).unwrap())
                    .unwrap()
            })
            .collect::<Vec<_>>();
        let final_value = locations
            .iter()
            .find_map(|loc| {
                doc.extracted_positions
                    .position_of(*loc)
                    .and_then(|_| candidate.page_counter_final_values.get(loc))
                    .and_then(|v| v.first())
                    .copied()
            })
            .or_else(|| logical.last().copied())
            .unwrap_or(0);
        let refs = page_reference_pages(body, &doc);
        let (pages, found) =
            realize_page_numberings(world, source, &doc, &logical, final_value, &refs)?;
        warnings.extend(found);
        for p in 1..=doc.pages.len() {
            let p = NonZeroUsize::new(p).unwrap();
            numberings.push(pages.numbering_for_page(p).cloned());
            numbers.push(pages.logical_number_for_page(p).unwrap());
            spans.push(pages.numbering_span_for_page(p).unwrap_or(Span::detached()));
            visible.push(pages.visible_numbering_for_page(p).cloned());
            reference.push(pages.reference_numbering_for_page(p).cloned());
            supplements
                .push(pages.supplement_for_page(p).cloned().unwrap_or(Content::Empty));
        }
        offset += doc.pages.len();
    }
    let store = if let Some(total) = NonZeroUsize::new(offset) {
        PageStore::from_realized(
            total,
            numberings,
            numbers,
            spans,
            visible,
            reference,
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
        if let Content::Located(e) = node {
            if let Content::ContextBlock(elem) = &e.body {
                if executed.iter().any(|known| Arc::ptr_eq(known, elem)) {
                    return Ok(Some(Content::Empty));
                }
                return Err(vec![SourceDiagnostic::error(
                    source.root().span(),
                    "HTML contextual expansion left unresolved content",
                )]);
            }
            if strip_counter_updates && matches!(e.body, Content::CounterUpdate(_)) {
                return Ok(Some(Content::Empty));
            }
        }
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
            if retained
                .iter()
                .all(|part| matches!(part, Content::Space | Content::SpaceFromNewline))
            {
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
        && before.page_counter_values == after.page_counter_values
        && before.page_counter_final_values == after.page_counter_final_values
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

/// Pure Page fold with the Engine already owned by this compilation.
pub(super) fn realize_page_snapshot_for_pipeline(
    documents: &[(Vec<Location>, Option<std::num::NonZeroUsize>)],
    intr: &mut TagIntrospector,
    world: &dyn World,
    source: &Source,
) -> (SourceResult<()>, Vec<SourceDiagnostic>) {
    let metrics = FallbackFontMetrics::new(world);
    let ctx = EvalContext::new();
    let mut scopes_styles = StyleChain::default_chain();
    let mut rules: Arc<[ShowRule]> = Arc::from([]);
    let mut guards = Vec::new();
    let mut sink = TypstSink::new();
    let route = Route::root().with_id(source.id());
    let result = {
        let mut tracked = sink.track_mut();
        let mut local = TrackedMut::reborrow_mut(&mut tracked);
        let mut engine = Engine {
            world,
            font_metrics: &metrics,
            route: route.track(),
            styles: &mut scopes_styles,
            show_rules: &mut rules,
            active_guards: &mut guards,
            current_file: source.id(),
            sink: &mut local,
        };
        typst_core::compiler::introspect::from_tags::realize_page_counter_snapshot(
            &[],
            documents,
            intr,
            &mut engine,
            &ctx,
        )
    };
    (result, sink.into_diagnostics())
}

fn snapshot_locations(intr: &TagIntrospector) -> Vec<Location> {
    intr.elements
        .keys()
        .copied()
        .chain(intr.kind_index.values().flatten().copied())
        .chain(intr.context_block_locations.values().copied())
        .chain(intr.parent_locations.keys().copied())
        .chain(intr.parent_locations.values().copied())
        .collect::<HashSet<_>>()
        .into_iter()
        .collect()
}

#[cfg(test)]
mod memory_snapshot_tests {
    use super::*;

    #[test]
    fn execution_retains_style_values_through_retirement_and_vector_growth() {
        use typst_core::entities::{func::Func, locator::Locator, version::Version};

        let world = crate::world::SystemWorld::for_stdin(
            std::path::PathBuf::from("/tmp"),
            String::new(),
        );
        let source = world.source(world.main()).unwrap();
        let input = Arc::new(TagIntrospector::empty());
        let mut session = Session::new(
            Content::Empty,
            &input,
            &world,
            &source,
            false,
            Features::default(),
            EvalTarget::Paged,
        );
        let value = Arc::new(Version::new(1, 2, 3));
        let weak = Arc::downgrade(&value);
        let chain = StyleChain::default_chain()
            .push_custom("retention.witness", Value::Version(value));
        let elem = Arc::new(ContextBlockElem {
            id: 1,
            closure: Func::native("retention-witness", |_ctx, _args, _world, _file| {
                Ok(Value::None)
            }),
        });
        let index = session.add(elem, chain, None);
        let location = Locator::new().next();
        session.execute(index, &input, location);
        session.nodes[index].chain = StyleChain::default_chain();
        assert!(weak.upgrade().is_some(), "current execution retains final styles");

        session.execute(index, &input, location);
        assert!(weak.upgrade().is_some(), "retired execution retains final styles");
        let capacity = session.retired.capacity();
        for _ in 0..=capacity {
            session.execute(index, &input, location);
        }
        assert!(session.retired.capacity() > capacity);
        assert!(weak.upgrade().is_some(), "vector growth preserves retained values");
        assert!(Arc::ptr_eq(&session.retired[0].input, &input));
        drop(session);
        assert!(weak.upgrade().is_none(), "session drop releases retained styles");
    }

    #[test]
    fn same_generation_shares_snapshot_and_mutable_context_stays_independent() {
        let world = crate::world::SystemWorld::for_stdin(
            std::path::PathBuf::from("/tmp"),
            "#context [A]\n#context [B]".into(),
        );
        let source = world.source(world.main()).unwrap();
        let module = eval_to_module_with_sink_target_features(
            &world,
            &source,
            false,
            EvalTarget::Paged,
            Features::default(),
        )
        .0
        .unwrap();
        let content = module.content().unwrap().clone();
        let intr = introspect_with_introspector(&content);
        let mut session = Session::new(
            content,
            &intr,
            &world,
            &source,
            false,
            Features::default(),
            EvalTarget::Paged,
        );
        session.discover();
        assert_eq!(session.nodes.len(), 2);
        assert_eq!(session.body(0).plain_text(), "A");
        assert_eq!(session.body(1).plain_text(), "B");
        let first: &TagIntrospector = &session.nodes[0].current.as_ref().unwrap().input;
        let other: &TagIntrospector = &session.nodes[1].current.as_ref().unwrap().input;
        assert!(
            std::ptr::eq(first, other),
            "one immutable snapshot per concrete generation"
        );
        assert!(
            session.nodes[0]
                .current
                .as_ref()
                .unwrap()
                .ctx
                .introspector
                .context_block_locations
                .is_empty(),
            "completed execution must release its private introspector"
        );
        assert_eq!(
            session.nodes[0]
                .current
                .as_ref()
                .unwrap()
                .input
                .context_block_locations
                .len(),
            2
        );
        assert_eq!(
            session.nodes[1]
                .current
                .as_ref()
                .unwrap()
                .input
                .context_block_locations
                .len(),
            2
        );
        let next = Arc::new(session.original_intr.clone());
        let location = next.context_block_locations[&session.nodes[0].elem.id];
        session.execute(0, &next, location);
        assert_eq!(session.retired.len(), 1);
        let old: &TagIntrospector = &session.retired[0].input;
        let retained: &TagIntrospector =
            &session.nodes[1].current.as_ref().unwrap().input;
        let fresh: &TagIntrospector = &session.nodes[0].current.as_ref().unwrap().input;
        assert!(std::ptr::eq(old, retained));
        assert!(!std::ptr::eq(old, fresh));
        assert_eq!(old.context_block_locations, fresh.context_block_locations);
    }
}
