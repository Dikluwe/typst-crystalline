//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/pipeline/context_stabilization.md
//! @prompt-hash d6ab9409
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
    selected_reference_supplement: bool,
    selected_html_position: bool,
    invalid: bool,
    active: bool,
    target: EvalTarget,
    reference_depth: usize,
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

fn publish_candidate_warnings(
    diagnostics: impl IntoIterator<Item = SourceDiagnostic>,
) -> Vec<SourceDiagnostic> {
    let mut publication = TypstSink::new();
    for diagnostic in diagnostics {
        publication.warn(diagnostic);
    }
    publication.into_diagnostics()
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
    let mut actual_children = Vec::new();
    let explicit_children = if let Some(child) = typst_core::compiler::realize::reference_supplements::prepared_reference_supplement(content) {
        actual_children.push(child);
        true
    } else {
        match content {
            Content::Figure(e) => {
                actual_children.push(&e.body);
                if let Some(supplement) = figure_context_supplement(e, chain) { actual_children.push(supplement); }
                if let Some(caption) = &e.caption { actual_children.push(caption); }
                true
            }
            Content::Realized(e) => { actual_children.push(&e.visual); true }
            Content::Source(e) => { actual_children.push(&e.body); true }
            Content::Located(e) => { actual_children.push(&e.body); true }
            Content::ShowRecipe(e) => return ordered_blocks(&e.body, &e.styles),
            Content::Styled(e) => return ordered_blocks(&e.body, &chain.push_styles(&e.styles)),
            Content::Sequence(parts) | Content::MathSequence(parts) => { actual_children.extend(parts.iter()); true }
            Content::Strong(e) => { actual_children.push(&e.body); true }
            Content::Emph(e) => { actual_children.push(&e.body); true }
            Content::Block(e) => { actual_children.push(&e.body); true }
            Content::Boxed(e) => { actual_children.push(&e.body); true }
            Content::Heading(e) => { actual_children.push(&e.body); true }
            Content::Title(e) => { actual_children.push(&e.body); true }
            Content::ListItem(e) => { actual_children.push(&e.body); true }
            Content::EnumItem(e) => { actual_children.push(&e.body); true }
            Content::Link(e) => { actual_children.push(&e.body); true }
            Content::Columns(e) => { actual_children.push(&e.body); true }
            Content::Align(e) => { actual_children.push(&e.body); true }
            Content::Place(e) => { actual_children.push(&e.body); true }
            Content::Repeat(e) => { actual_children.push(&e.body); true }
            Content::Transform(e) => { actual_children.push(&e.body); true }
            Content::Label(e) => { actual_children.push(&e.body); true }
            Content::Par { body } | Content::SmallCaps { body } => { actual_children.push(body); true }
            // These current variants delegate to the existing generic actual-
            // member traversal below; they are not classified as empty leaves.
            Content::Empty
            | Content::Text(..)
            | Content::Space
            | Content::SpaceFromNewline
            | Content::Parbreak
            | Content::HtmlElem(..)
            | Content::HtmlFrame(..)
            | Content::Raw(..)
            | Content::Equation(..)
            | Content::MathIdent(..)
            | Content::MathText(..)
            | Content::MathFrac(..)
            | Content::MathAttach(..)
            | Content::MathPrimes(..)
            | Content::MathRoot(..)
            | Content::MathStretch(..)
            | Content::MathDelimited(..)
            | Content::MathLr(..)
            | Content::MathMid(..)
            | Content::MathOverbrace(..)
            | Content::MathUnderbrace(..)
            | Content::MathUnderbracket(..)
            | Content::MathUnderparen(..)
            | Content::MathUndershell(..)
            | Content::MathOverbracket(..)
            | Content::MathOverparen(..)
            | Content::MathOvershell(..)
            | Content::MathAlignPoint(..)
            | Content::Linebreak(..)
            | Content::MathMatrix(..)
            | Content::MathCases(..)
            | Content::MathAccent(..)
            | Content::MathCancel(..)
            | Content::MathUnderline(..)
            | Content::MathOverline(..)
            | Content::MathVec(..)
            | Content::MathClassOverride(..)
            | Content::MathLimitsOverride(..)
            | Content::MathUnderover(..)
            | Content::MathOp(..)
            | Content::MathStyled(..)
            | Content::Ref(..)
            | Content::CounterDisplay(..)
            | Content::CounterUpdate(..)
            | Content::Outline(..)
            | Content::OutlineEntry(..)
            | Content::FigureCaption(..)
            | Content::Image(..)
            | Content::Shape(..)
            | Content::Curve(..)
            | Content::Grid(..)
            | Content::GridHeader(..)
            | Content::GridFooter(..)
            | Content::GridCell(..)
            | Content::SetPage(..)
            | Content::PageRun(..)
            | Content::Flush(..)
            | Content::Divider(..)
            | Content::Terms(..)
            | Content::TermItem(..)
            | Content::Quote(..)
            | Content::Document { .. }
            | Content::Asset { .. }
            | Content::SmartQuote(..)
            | Content::PdfAttach(..)
            | Content::PdfArtifact(..)
            | Content::Underline(..)
            | Content::Strike(..)
            | Content::Overline(..)
            | Content::Pad(..)
            | Content::Hide(..)
            | Content::HSpace(..)
            | Content::VSpace(..)
            | Content::Pagebreak(..)
            | Content::Colbreak(..)
            | Content::Stack(..)
            | Content::TableCell(..)
            | Content::Bibliography(..)
            | Content::Cite(..)
            | Content::Footnote(..)
            | Content::FootnoteEntry(..)
            | Content::TableHeader(..)
            | Content::TableFooter(..)
            | Content::GridHLine(..)
            | Content::GridVLine(..)
            | Content::TableHLine(..)
            | Content::TableVLine(..)
            | Content::Table(..)
            | Content::Metadata(..)
            | Content::State(..)
            | Content::StateUpdate(..)
            | Content::StateDisplay(..)
            | Content::CounterDisplayCallback(..)
            | Content::ContextBlock(..)
            | Content::Dynamic(..) => false,
        }
    };
    if explicit_children {
        let mut ordered = Vec::new();
        let mut seen = HashSet::new();
        for child in actual_children {
            for pair in ordered_blocks(child, chain)? {
                if !seen.insert(pair.0.id) {
                    return None;
                }
                ordered.push(pair);
            }
        }
        return Some(ordered);
    }
    let by_id = collect_context_blocks(content, chain);
    if by_id.is_empty() {
        return Some(Vec::new());
    }
    let mut seen = HashSet::new();
    let mut ordered = Vec::new();
    let mut unique = true;
    let _ = content.map_content(&mut |node| {
        if let Some(child) = typst_core::compiler::realize::reference_supplements::prepared_reference_supplement(node) {
            match ordered_blocks(child, chain) {
                Some(blocks) => {
                    for (elem, _) in blocks {
                        if let Some(pair) = by_id.get(&elem.id) {
                            if !seen.insert(elem.id) { unique = false; }
                            ordered.push(pair.clone());
                        }
                    }
                }
                None => unique = false,
            }
            return Ok(None);
        }
        let contextual = match node {
            Content::ContextBlock(elem) => Some(elem),
            Content::Located(e) => match &e.body {
                Content::ContextBlock(elem) => Some(elem),
                _ => None,
            },
            _ => None,
        };
        if let Some(elem) = contextual {
            if let Some(pair) = by_id.get(&elem.id) {
                if !seen.insert(elem.id) {
                    unique = false;
                }
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

    fn figure_with_context_template(caption: bool) -> Content {
        use typst_core::entities::elements::figure::{FigureElem, FigureKind};
        Content::Figure(Arc::new(FigureElem {
            body: block(1),
            caption: caption.then(|| block(3)),
            kind: None,
            resolved_kind: Some(FigureKind::Name("test".into())),
            supplement: None,
            resolved_supplement: Some(Some(block(2))),
        }))
    }

    #[test]
    fn figure_context_template_is_not_an_unrendered_producer_or_substitution() {
        let content = Content::strong(figure_with_context_template(false));
        let blocks = ordered_blocks(&content, &StyleChain::default_chain()).unwrap();
        assert_eq!(blocks.iter().map(|(e, _)| e.id).collect::<Vec<_>>(), vec![1]);
        let resolved =
            HashMap::from([(1, Content::text("BODY")), (2, Content::text("PHANTOM"))]);
        let substituted = substitute_context_blocks(content, &resolved);
        let Content::Strong(strong) = substituted else { panic!("strong preserved") };
        let Content::Figure(figure) = &strong.body else { panic!("Figure preserved") };
        assert_eq!(figure.body.plain_text(), "BODY");
        assert!(matches!(
            figure.resolved_supplement,
            Some(Some(Content::ContextBlock(_)))
        ));
    }

    #[test]
    fn figure_context_caption_uses_actual_supplement_order_and_duplicate_policy() {
        let blocks = ordered_blocks(
            &figure_with_context_template(true),
            &StyleChain::default_chain(),
        )
        .unwrap();
        assert_eq!(blocks.iter().map(|(e, _)| e.id).collect::<Vec<_>>(), vec![1, 2, 3]);
        let duplicate =
            Content::sequence(vec![figure_with_context_template(false), block(1)]);
        assert!(ordered_blocks(&duplicate, &StyleChain::default_chain()).is_none());
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
        let reference_depth = parent.map_or(0, |parent| self.nodes[parent].reference_depth)
            + usize::from(typst_core::compiler::realize::reference_supplements::is_reference_supplement_producer(&elem));
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
            selected_reference_supplement: false,
            selected_html_position: false,
            invalid: true,
            active: true,
            target,
            reference_depth,
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
        let selected_reference_supplement = typst_core::compiler::realize::reference_supplements::is_reference_supplement_producer(&node.elem);
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
        let base_route = Route::root().with_id(self.source.id());
        let mut reference_route = Route::extend(base_route.track());
        for _ in 1..node.reference_depth {
            reference_route.increase();
        }
        let route = if node.reference_depth == 0 {
            base_route.track()
        } else {
            reference_route.track()
        };
        let exceeded_reference_depth =
            selected_reference_supplement && !route.within(Route::MAX_CALL_DEPTH);
        let result = {
            let mut tracked = sink.track_mut();
            let mut local = TrackedMut::reborrow_mut(&mut tracked);
            let mut engine = Engine {
                world: self.world,
                font_metrics: &self.metrics,
                route,
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
        if exceeded_reference_depth {
            self.impossible = true;
        }
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
        node.selected |= selected_filtered_counter
            || selected_reference_supplement
            || selected_html_position;
        node.selected_filtered_counter |= selected_filtered_counter;
        node.selected_reference_supplement |= selected_reference_supplement;
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
            if self.impossible {
                return;
            }
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
        let base_route = Route::root().with_id(self.source.id());
        let mut reference_route = Route::extend(base_route.track());
        for _ in 1..node.reference_depth {
            reference_route.increase();
        }
        let route = if node.reference_depth == 0 {
            base_route.track()
        } else {
            reference_route.track()
        };
        let mut tracked = sink.track_mut();
        let mut local = TrackedMut::reborrow_mut(&mut tracked);
        let mut engine = Engine {
            world: self.world,
            font_metrics: &self.metrics,
            route,
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
            let content = self.content();
            let (runtime, mut phase_warnings) =
                if let Some(html_targets) = self.bundle_html_targets.as_deref() {
                    let Content::Sequence(parts) = &content else {
                        return (terminal(self.source, Vec::new()), self.warnings());
                    };
                    let (result, warnings) = introspect_bundle_with_runtime_for_pipeline(
                        parts.as_ref(),
                        html_targets,
                        self.features,
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
                Err(error) => {
                    return (
                        self.fail(error),
                        publish_candidate_warnings(
                            self.warnings().into_iter().chain(phase_warnings),
                        ),
                    );
                }
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
                        return (self.fail(errors), self.warnings());
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
                // Each retained execution owns its effects. Publish this one
                // candidate through the existing Sink policy after validation.
                let warnings = publish_candidate_warnings(
                    warnings.into_iter().chain(phase_warnings),
                );
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
    if session.impossible {
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
    let selected = selected_filtered_counter || selected_html_position || page_selected;
    let (prepared_content, mut warnings) = if selected {
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
    if !selected {
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
    let (runtime, runtime_warnings) = introspect_bundle_with_runtime_for_pipeline(
        &raw_bodies,
        html_targets,
        features,
        world,
        source,
    );
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

#[cfg(test)]
mod figure_pipeline_transaction_tests {
    use super::*;
    use std::num::NonZeroU16;
    use typst_core::entities::{
        file_id::FileId,
        font_book::FontBook,
        world_types::{Bytes, Datetime, FileError, FileResult, Font, Library},
    };

    struct FixtureWorld {
        library: Library,
        book: FontBook,
        source: Source,
    }
    impl World for FixtureWorld {
        fn library(&self) -> &Library {
            &self.library
        }
        fn book(&self) -> &FontBook {
            &self.book
        }
        fn main(&self) -> FileId {
            self.source.id()
        }
        fn source(&self, id: FileId) -> FileResult<Source> {
            if id == self.source.id() {
                Ok(self.source.clone())
            } else {
                Err(FileError::NotFound)
            }
        }
        fn file(&self, _: FileId) -> FileResult<Bytes> {
            Err(FileError::NotFound)
        }
        fn font(&self, _: usize) -> Option<Font> {
            None
        }
        fn today(
            &self,
            _: Option<typst_core::entities::duration::Duration>,
        ) -> Option<Datetime> {
            None
        }
    }
    fn world(source: &str) -> FixtureWorld {
        FixtureWorld {
            library: Library::new(),
            book: FontBook::new(),
            source: Source::new(
                FileId::from_raw(NonZeroU16::new(1).unwrap()),
                source.to_owned(),
            ),
        }
    }
    fn paged_warnings(source: &str) -> Vec<SourceDiagnostic> {
        let world = world(source);
        let (result, warnings) =
            super::super::paged::compile_to_paged_document_full_error(
                &world,
                &world.source,
                false,
                &mut Timings::default(),
            );
        assert!(result.is_ok(), "{result:?}");
        warnings
    }
    const UPDATE: &str = r#"#counter(figure.where(kind:table)).update(n=>{let ignored=numbering("א",0);n})
"#;
    const WARNING: &str = "the numeral system `hebrew` cannot represent zero";

    #[test]
    fn figure_pipeline_retains_only_accepted_implicit_warning_generation() {
        let source = format!("{UPDATE}#figure([A],kind:table,caption:[CAP])");
        for _ in 0..2 {
            let warnings = paged_warnings(&source);
            assert_eq!(
                warnings
                    .iter()
                    .map(|warning| warning.message.as_str())
                    .collect::<Vec<_>>(),
                vec![WARNING]
            );
            assert!(!warnings[0].span.is_detached());
        }
        assert!(paged_warnings(&format!("{UPDATE}#figure([A],kind:table)")).is_empty());
    }

    #[test]
    fn figure_pipeline_retains_explicit_and_implicit_effects_in_one_candidate_sink() {
        let source = format!(
            "{UPDATE}#figure([A],kind:table,caption:[CAP])\n#context repr(counter(figure.where(kind:table)).final())"
        );
        let warnings = paged_warnings(&source);
        assert_eq!(
            warnings
                .iter()
                .map(|warning| warning.message.as_str())
                .collect::<Vec<_>>(),
            vec![WARNING]
        );
        assert!(!warnings[0].span.is_detached());
    }

    #[test]
    fn figure_pipeline_retains_same_warning_at_each_distinct_source_origin() {
        let source = format!(
            "{UPDATE}{UPDATE}#figure([A],kind:table,caption:[CAP])\n#context repr(counter(figure.where(kind:table)).final())"
        );
        let warnings = paged_warnings(&source);
        assert_eq!(
            warnings
                .iter()
                .map(|warning| warning.message.as_str())
                .collect::<Vec<_>>(),
            vec![WARNING, WARNING]
        );
        assert!(!warnings[0].span.is_detached());
        assert!(!warnings[1].span.is_detached());
        assert_ne!(warnings[0].span, warnings[1].span);
    }

    #[test]
    fn figure_pipeline_terminal_failed_demand_retains_prior_sourced_warning() {
        let prefix = "#counter(figure.where(kind:table)).update(n=>{let ignored=numbering(\"א\",0);panic(\"AFTER_WARNING\")})\n#figure([A],kind:table,caption:[CAP])\n";
        for tail in [
            "",
            "#context repr(counter(heading.where(level:1)).final())",
            "#context repr(counter(figure.where(kind:table)).final())",
        ] {
            let world = world(&format!("{prefix}{tail}"));
            let (result, warnings) =
                super::super::paged::compile_to_paged_document_full_error(
                    &world,
                    &world.source,
                    false,
                    &mut Timings::default(),
                );
            let errors = result.expect_err("the original demanded panic must propagate");
            assert!(
                errors.iter().any(|error| error.message.contains("AFTER_WARNING")),
                "{errors:?}"
            );
            assert!(errors.iter().all(|error| !error.span.is_detached()));
            assert_eq!(
                warnings
                    .iter()
                    .map(|warning| warning.message.as_str())
                    .collect::<Vec<_>>(),
                vec![WARNING],
                "{tail}"
            );
            assert!(!warnings[0].span.is_detached());
        }
    }

    fn reference_preparation_error(source: &str) {
        let world = world(source);
        let (result, _) = super::super::paged::compile_to_paged_document_full_error(
            &world,
            &world.source,
            false,
            &mut Timings::default(),
        );
        let errors = result.expect_err(
            "an actually emitted reference must prepare its selected nested Figure",
        );
        assert!(
            errors.iter().any(|error| error.message.contains("INNER_PREPARATION")),
            "{errors:?}"
        );
        assert!(errors.iter().all(|error| !error.span.is_detached()));
    }

    fn captionless_block_compilation(
        source: &str,
    ) -> super::super::paged::PagedCompilation {
        let fixture = world(source);
        let (compiled, warnings) = super::super::paged::compile_to_paged_document_full_error_and_features_with_introspector(
            &fixture, &fixture.source, false, Features::default(), &mut Timings::default(),
        );
        assert!(warnings.is_empty(), "{warnings:?}");
        compiled.unwrap_or_else(|errors| panic!("{errors:?}"))
    }

    fn captionless_count_metadata(intr: &TagIntrospector) -> &Value {
        use typst_core::entities::{introspector::Introspector, label::Label};
        let location = intr.query_by_label(&Label("count".into())).unwrap();
        let Some(Content::Metadata(metadata)) = intr.element_at(location) else {
            panic!("actual count Metadata must remain a language object");
        };
        metadata.value.as_ref()
    }

    #[test]
    fn figure_captionless_default_block_none_prunes_body_before_effects() {
        use typst_core::entities::{
            element_kind::ElementKind, introspector::Introspector, label::Label,
        };
        for body in [
            "#context panic(\"BODY_CONTEXT_PREPARATION\")",
            "#figure([INNER],kind:table,supplement:it=>panic(\"BODY_FIGURE_PREPARATION\"))",
        ] {
            let source = format!("#show block:none\n#figure([{body}],kind:image)<outer>\n#context [#metadata(counter(figure.where(kind:image)).final())<count>]");
            let compiled = captionless_block_compilation(&source);
            assert!(compiled.document.plain_text().trim().is_empty());
            let intr = &compiled.introspector;
            let figures = intr.query_by_kind(ElementKind::Figure);
            assert_eq!(figures.len(), 1, "the prepared outer Figure still exists");
            let location = intr.query_by_label(&Label("outer".into())).unwrap();
            assert_eq!(figures, vec![location]);
            let key = intr.counter_key_for_label(&Label("outer".into())).unwrap();
            assert_eq!(intr.counter_final_values(key), None, "no caption/reference demand publishes a history cache");
            assert_eq!(captionless_count_metadata(intr), &Value::Array(vec![Value::Int(1)]));
            assert_eq!(intr.query_by_kind(ElementKind::ContextBlock).len(), 1, "hidden body contexts are not language occurrences");
        }
    }

    #[test]
    fn figure_captionless_default_block_callback_sees_actual_body_morphology() {
        let compiled = captionless_block_compilation(
            r#"#show block: it => {
  assert.eq(it.body.children.len(), 2)
  assert.eq(it.body.children.at(0).func(), strong)
  assert.eq(it.body.children.at(1).func(), parbreak)
  [BLOCKBODYVALID]
}
#figure([*BODY*],kind:image)
"#,
        );
        assert_eq!(
            compiled
                .document
                .plain_text()
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" "),
            "BLOCKBODYVALID"
        );
    }

    #[test]
    fn figure_captionless_default_block_empty_sequence_and_strong_empty_morphology() {
        for (body, expected_children, middle_assertion) in [
            ("", 1, ""),
            ("#empty", 1, ""),
            ("A *B* C", 6, "assert.eq(it.body.children.at(2).func(), strong)"),
            ("#strong([])", 2, "assert.eq(it.body.children.at(0).func(), strong)"),
        ] {
            let source = format!("#let empty=[]\n#show block: it => {{\nassert.eq(it.body.children.len(), {expected_children})\n{middle_assertion}\nassert.eq(it.body.children.last().func(), parbreak)\n[MORPHOLOGYVALID]\n}}\n#figure([{body}],kind:image)");
            let compiled = captionless_block_compilation(&source);
            assert_eq!(
                compiled
                    .document
                    .plain_text()
                    .split_whitespace()
                    .collect::<Vec<_>>()
                    .join(" "),
                "MORPHOLOGYVALID",
                "{body}"
            );
        }
    }

    #[test]
    fn figure_captionless_default_block_preserves_original_location_reference_and_page() {
        use typst_core::entities::{
            element_kind::ElementKind, introspector::Introspector, label::Label,
        };
        let compiled = captionless_block_compilation("#figure([BODY],kind:image)<outer>\n@outer\n#context [#metadata(counter(figure.where(kind:image)).final())<count>]");
        let intr = &compiled.introspector;
        let location = intr.query_by_label(&Label("outer".into())).unwrap();
        assert_eq!(intr.query_by_kind(ElementKind::Figure), vec![location]);
        let key = intr.counter_key_for_label(&Label("outer".into())).unwrap();
        assert_eq!(intr.counter_final_values(key), Some(&[1][..]));
        assert_eq!(
            compiled
                .document
                .extracted_positions
                .position_of(location)
                .unwrap()
                .page
                .get(),
            1
        );
        assert!(
            compiled.document.extracted_figure_page_numbers.is_empty(),
            "captionless Figures do not acquire caption page entries"
        );
        let text = compiled.document.plain_text();
        assert!(text.contains("BODY"), "{text}");
        let words = text.split_whitespace().collect::<Vec<_>>();
        assert!(words.windows(2).any(|pair| pair == ["Figure", "1"]), "{text}");
    }

    #[test]
    fn figure_captionless_default_block_own_figure_replacement_has_no_artificial_block() {
        let compiled = captionless_block_compilation("#show block: it => [ARTIFICIALBLOCK]\n#show figure: it => [OWNPRODUCT]\n#figure([BODY],kind:image)");
        assert_eq!(
            compiled
                .document
                .plain_text()
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" "),
            "OWNPRODUCT"
        );
    }

    #[test]
    fn figure_captionless_default_block_reached_context_and_nested_figure_are_indexed() {
        use typst_core::entities::{
            element_kind::ElementKind, introspector::Introspector, label::Label,
        };
        let compiled = captionless_block_compilation("#figure([OUTER #context [CTX] #figure([INNER],kind:table,supplement:[Inner])<inner>],kind:image)<outer>\n#context [#metadata((counter(figure.where(kind:image)).final(),counter(figure.where(kind:table)).final()))<count>]");
        let intr = &compiled.introspector;
        assert_eq!(intr.query_by_kind(ElementKind::Figure).len(), 2);
        assert_eq!(intr.query_by_kind(ElementKind::ContextBlock).len(), 2);
        for label in ["outer", "inner"] {
            let label = Label(label.into());
            let location = intr.query_by_label(&label).unwrap();
            let key = intr.counter_key_for_label(&label).unwrap();
            assert_eq!(
                intr.counter_final_values(key),
                None,
                "metadata-only filtered reads do not publish a history cache"
            );
            assert_eq!(
                compiled
                    .document
                    .extracted_positions
                    .position_of(location)
                    .unwrap()
                    .page
                    .get(),
                1
            );
        }
        assert_eq!(
            captionless_count_metadata(intr),
            &Value::Array(vec![
                Value::Array(vec![Value::Int(1)]),
                Value::Array(vec![Value::Int(1)])
            ])
        );
        let text = compiled.document.plain_text();
        assert!(text.contains("CTX"), "{text}");
        assert!(text.contains("INNER"), "{text}");
    }

    #[test]
    fn figure_captionless_default_block_original_root_preserves_actual_heading_membership(
    ) {
        use typst_core::entities::{
            element_kind::ElementKind, introspector::Introspector,
        };
        for (rule, suffix, expected) in
            [("none", "", 0), ("it=>it", "", 1), ("it=>it", "<block-label>", 1)]
        {
            let source = format!("#show block:{rule}\n#block[#heading([INNER])<inside>]{suffix}\n#context [#metadata(query(<inside>).len())<count>]");
            let compiled = captionless_block_compilation(&source);
            assert_eq!(
                captionless_count_metadata(&compiled.introspector),
                &Value::Int(expected)
            );
            assert_eq!(
                compiled.introspector.query_by_kind(ElementKind::Heading).len(),
                expected as usize
            );
            // The separate nonlocatable Block-label membership gap remains open.
        }
    }

    #[test]
    fn figure_reference_layout_diagnostics_keep_exact_lexical_source() {
        for (source, message, column) in [
            ("#heading([H])<h>\n@h", "cannot reference heading without numbering", 0),
            (
                "#heading([H])<h>\n#ref(<h>,form:\"page\")",
                "cannot reference without page numbering",
                1,
            ),
        ] {
            let fixture = world(source);
            let (result, warnings) =
                super::super::paged::compile_to_paged_document_full_error(
                    &fixture,
                    &fixture.source,
                    false,
                    &mut Timings::default(),
                );
            assert!(warnings.is_empty(), "{warnings:?}");
            let errors = result.expect_err("the existing source error must remain");
            assert_eq!(errors.len(), 1);
            assert_eq!(errors[0].message, message);
            assert_eq!(
                fixture.source.span_to_line_col(errors[0].span),
                Some((2, column))
            );
            assert_eq!(errors[0].hints.len(), 1);
        }
    }

    #[test]
    fn figure_reference_phase_generic_grid_does_not_admit_dormant_template() {
        let world = world(
            r#"#grid(columns:1, figure([BODY],kind:table,supplement:[#context panic("DORMANT_TEMPLATE")]))
#context repr(counter(heading.where(level:1)).final())
"#,
        );
        let (result, warnings) =
            super::super::paged::compile_to_paged_document_full_error(
                &world,
                &world.source,
                false,
                &mut Timings::default(),
            );
        assert!(result.is_ok(), "{result:?}");
        assert!(warnings.is_empty(), "{warnings:?}");
    }

    #[test]
    fn figure_reference_phase_explicit_supplement_reaches_nested_preparation() {
        reference_preparation_error(
            r#"#figure([OUTER],supplement:figure([INNER], kind: table, caption:[INNER-CAP], supplement:it=>panic("INNER_PREPARATION"))) <outer>
@outer"#,
        );
    }

    #[test]
    fn figure_reference_phase_callback_supplement_reaches_nested_preparation() {
        reference_preparation_error(
            r#"#figure([OUTER],supplement:it=>figure([INNER], kind: table, caption:[INNER-CAP], supplement:it=>panic("INNER_PREPARATION"))) <outer>
@outer"#,
        );
    }

    #[test]
    fn figure_reference_phase_transport_keeps_source_label_and_context_membership() {
        use typst_core::entities::{
            content::Content, element_kind::ElementKind, introspector::Introspector,
            label::Label, selector::Selector,
        };
        let fixture = world(
            "#figure([OUTER],supplement:figure([INNER],kind:table,caption:[IC],supplement:[Inner]))<outer>\n@outer <reference-label>\n#context metadata(\"source-context\")",
        );
        let (module, _) = eval_to_module_with_sink_target_features(
            &fixture,
            &fixture.source,
            false,
            EvalTarget::Paged,
            Features::default(),
        );
        let module = module.expect("source occurrence preparation");
        let content = module.content().unwrap();
        let intr = module
            .realization_introspector()
            .cloned()
            .unwrap_or_else(|| introspect_with_introspector(content));
        let blocks = collect_context_blocks(content, &StyleChain::default_chain());
        let generated = blocks.values().filter(|(block, _)| typst_core::compiler::realize::reference_supplements::is_reference_supplement_producer(block)).collect::<Vec<_>>();
        assert_eq!(generated.len(), 1);
        let ordinary = intr.query(&Selector::Kind(ElementKind::ContextBlock));
        assert_eq!(ordinary.len(), 1, "only the actual source context is semantic");
        let location = intr
            .query_by_label(&Label("reference-label".into()))
            .expect("labeled source Ref occurrence");
        assert!(matches!(intr.element_at(location), Some(Content::Ref(_))));
        assert!(!ordinary.contains(&location));
        assert_eq!(intr.context_block_locations.get(&generated[0].0.id), Some(&location));
        assert!(!typst_core::compiler::introspect::locatable::is_locatable(
            &Content::ContextBlock(generated[0].0.clone())
        ));
        assert!(typst_core::compiler::introspect::extract_payload::extract_payload(
            &Content::ContextBlock(generated[0].0.clone())
        )
        .is_none());
        let (compiled, _) = super::super::paged::compile_to_paged_document_full_error_and_features_with_introspector(&fixture, &fixture.source, false, Features::default(), &mut Timings::default());
        let compiled = compiled
            .expect("labeled reference retains its source morphology through expansion");
        let location = compiled
            .introspector
            .query_by_label(&Label("reference-label".into()))
            .unwrap();
        assert!(matches!(
            compiled.introspector.element_at(location),
            Some(Content::Ref(_))
        ));
        assert_eq!(
            compiled
                .introspector
                .query(&Selector::Kind(ElementKind::ContextBlock))
                .len(),
            1
        );
        assert_eq!(
            compiled
                .introspector
                .query(&Selector::Kind(ElementKind::Figure))
                .len(),
            2
        );
    }

    #[test]
    fn figure_reference_phase_html_does_not_execute_implicit_selected_children() {
        let fixture = world(
            r#"#figure([OUTER],supplement:figure([INNER],kind:table,caption:[IC],supplement:it=>panic("INNER_PREPARATION")))<outer>
@outer"#,
        );
        let (result, warnings) = super::super::html::compile_to_html_string_with_features(
            &fixture,
            &fixture.source,
            Features::html(),
        );
        if let Err(errors) = result {
            assert!(
                errors
                    .iter()
                    .all(|error| !error.message.contains("INNER_PREPARATION")),
                "{errors:?}"
            );
        }
        assert!(warnings.is_empty(), "{warnings:?}");
    }

    #[test]
    fn figure_reference_phase_cycles_are_bounded_and_finite_chain_compiles() {
        for supplement in ["[@outer]", "[@outer @outer]"] {
            let fixture = world(&format!(
                "#figure([OUTER],kind:image,supplement:{supplement})<outer>\n@outer"
            ));
            let (result, _) = super::super::paged::compile_to_paged_document_full_error(
                &fixture,
                &fixture.source,
                false,
                &mut Timings::default(),
            );
            let errors = result.expect_err("recursive reference contributions must terminate with a sourced depth error");
            assert!(
                errors.iter().any(|error| error.message.contains("depth exceeded")),
                "{errors:?}"
            );
            assert!(errors.iter().all(|error| !error.span.is_detached()));
        }
        let fixture = world(
            "#figure([A],kind:image,supplement:[@b])<a>\n#figure([B],kind:image,supplement:[@c])<b>\n#figure([C],kind:image,supplement:[End])<c>\n@a",
        );
        let (result, _) = super::super::paged::compile_to_paged_document_full_error(
            &fixture,
            &fixture.source,
            false,
            &mut Timings::default(),
        );
        assert!(result.is_ok(), "{result:?}");
    }

    #[test]
    fn figure_pipeline_html_explicit_filtered_read_keeps_actual_target() {
        let fixture = world(
            "#show figure:none\n#figure([A],kind:table)\n#context repr(counter(figure.where(kind:table)).final())",
        );
        let (result, warnings) = super::super::html::compile_to_html_string_with_features(
            &fixture,
            &fixture.source,
            Features::html(),
        );
        let html = result.expect(
            "HTML explicit Figure read must stabilize without paged implicit demand",
        );
        assert!(html.contains("(1,)"), "{html}");
        assert!(warnings.is_empty(), "{warnings:?}");

        let ordinary = world(
            "#counter(heading.where(level:1)).update(7)\n#context repr(counter(heading.where(level:1)).final())",
        );
        let (result, warnings) = super::super::html::compile_to_html_string_with_features(
            &ordinary,
            &ordinary.source,
            Features::html(),
        );
        assert!(result.expect("shared non-Figure filtered read").contains("(7,)"));
        assert!(warnings.is_empty(), "{warnings:?}");

        let undemanded = world(
            "#counter(figure.where(kind:table)).update(n=>panic(\"unread caption fold\"))\n#show figure:none\n#figure([A],kind:table,caption:[CAP])",
        );
        let (result, warnings) = super::super::html::compile_to_html_string_with_features(
            &undemanded,
            &undemanded.source,
            Features::html(),
        );
        assert!(result.is_ok(), "{result:?}");
        assert!(warnings.is_empty(), "{warnings:?}");
    }
}
