//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/physical_memo.md
//! @prompt-hash d632f69e
//! @layer L1
//! @updated 2026-11-13

pub(super) mod introspection;
mod providers;
use super::*;
use crate::entities::{introspector::TagIntrospector, page_store::PageStore};
use std::{num::NonZeroUsize, sync::Mutex};
const LIMIT: usize = 32 * 1024 * 1024;
const JOURNAL_MAX_CHECKS: usize = 16_384;
type MetricCheck = Box<dyn Fn(&dyn FontMetrics) -> bool + Send + Sync>;
type IndexCheck = Box<dyn Fn(&TagIntrospector) -> bool + Send + Sync>;
const MAX_OBSERVED_PAGE_NUMBERS: usize = 128;
const PAGE_NUMBER_CHARGE: usize = std::mem::size_of::<(NonZeroUsize, usize)>();

fn effective_page_number(store: &PageStore, page: NonZeroUsize) -> usize {
    store.logical_number_for_page(page).unwrap_or(page.get())
}

#[derive(Default)]
struct PageReads {
    first_iteration_completed: bool,
    exact_snapshot: bool,
    logical: Vec<(NonZeroUsize, usize)>,
}

impl PageReads {
    fn record(&mut self, page: NonZeroUsize, number: usize) {
        if self.exact_snapshot || self.logical.contains(&(page, number)) {
            return;
        }
        if self.logical.len() == MAX_OBSERVED_PAGE_NUMBERS
            || self.logical.iter().any(|(previous, _)| *previous == page)
        {
            self.require_exact_snapshot();
            return;
        }
        self.logical.push((page, number));
    }

    fn require_exact_snapshot(&mut self) {
        self.exact_snapshot = true;
        self.logical.clear();
    }

    fn validates(&self, current: &PageStore) -> bool {
        !self.exact_snapshot
            && self.logical.iter().all(|&(page, expected)| {
                effective_page_number(current, page) == expected
            })
    }
}

impl<M: FontMetrics, S: ImageSizer> Layouter<'_, M, S> {
    pub(super) fn effective_runtime_page_number(&self, page: NonZeroUsize) -> usize {
        let number = effective_page_number(&self.runtime.known_page_store, page);
        if let Some(journal) = self.physical_page_reads {
            journal.record_page_number(page, number);
        }
        number
    }

    pub(super) fn exact_runtime_page_store(&self) -> &PageStore {
        if let Some(journal) = self.physical_page_reads {
            journal.require_exact_page_snapshot();
        }
        &self.runtime.known_page_store
    }
}

enum Check {
    Metric(MetricCheck),
    Index(IndexCheck),
}
#[derive(Default)]
struct JournalState {
    page_reads: PageReads,
    checks: Vec<Check>,
    charged: usize,
    refusals: Vec<&'static str>,
    fonts: Vec<(u8, usize)>,
}
impl JournalState {
    fn has_room_for(&self, charge: usize) -> bool {
        let page_checks = self.page_reads.logical.len();
        self.checks.len() + page_checks < JOURNAL_MAX_CHECKS
            && self.charged.checked_add(charge).is_some_and(|bytes| {
                bytes <= LIMIT - page_checks * PAGE_NUMBER_CHARGE
            })
    }
}
#[derive(Clone, Default)]
pub(super) struct Journal(Arc<Mutex<JournalState>>);
impl Journal {
    fn decline(&self, reason: &'static str) {
        let mut state = self.0.lock().unwrap();
        if !state.refusals.contains(&reason) {
            state.refusals.push(reason);
        }
    }
    fn add(&self, charge: usize, check: Check) {
        let mut s = self.0.lock().unwrap();
        if !s.refusals.is_empty() {
            return;
        }
        if !s.has_room_for(charge) {
            s.refusals.push("journal budget");
            return;
        }
        s.charged += charge;
        s.checks.push(check);
    }
    fn metric(
        &self,
        charge: usize,
        check: impl Fn(&dyn FontMetrics) -> bool + Send + Sync + 'static,
    ) {
        self.add(charge, Check::Metric(Box::new(check)));
    }
    fn index(
        &self,
        charge: usize,
        check: impl Fn(&TagIntrospector) -> bool + Send + Sync + 'static,
    ) {
        self.add(charge, Check::Index(Box::new(check)));
    }
    fn font_charge(&self, owner: (u8, usize), bytes: usize) -> Option<usize> {
        if bytes > 16 * 1024 * 1024 {
            self.decline("font budget");
            return None;
        }
        let mut s = self.0.lock().unwrap();
        if s.fonts.contains(&owner) {
            Some(0)
        } else {
            s.fonts.push(owner);
            Some(bytes)
        }
    }

    fn record_page_number(&self, page: NonZeroUsize, number: usize) {
        let mut state = self.0.lock().unwrap();
        if state.has_room_for(PAGE_NUMBER_CHARGE) {
            state.page_reads.record(page, number);
        } else {
            state.page_reads.require_exact_snapshot();
        }
    }

    fn require_exact_page_snapshot(&self) {
        self.0.lock().unwrap().page_reads.require_exact_snapshot();
    }

    pub(super) fn certify_unseeded_first_iteration(&self) {
        self.0.lock().unwrap().page_reads.first_iteration_completed = true;
    }

    fn page_inputs_match(&self, original: &PageStore, current: &PageStore) -> bool {
        if original.same_scalar_snapshot(current) {
            return true;
        }
        if current.is_empty() {
            return false;
        }
        let state = self.0.lock().unwrap();
        (!original.is_empty() || state.page_reads.first_iteration_completed)
            && state.page_reads.validates(current)
    }

    fn validates(&self, metrics: &dyn FontMetrics, index: &TagIntrospector) -> bool {
        let state = self.0.lock().unwrap();
        state.refusals.is_empty()
            && state.checks.iter().all(|check| match check {
                Check::Metric(f) => f(metrics),
                Check::Index(f) => f(index),
            })
    }
}
struct Entry {
    epoch: Option<MetricEpoch>,
    source: Arc<crate::compiler::introspect::page_running::SourcePlan>,
    pages: PageStore,
    font_size: u64,
    running: bool,
    journal: Journal,
    product: Arc<PageRunningDocument>,
}
#[derive(Default)]
struct Memo {
    entry: Option<Entry>,
    hits: usize,
    builds: usize,
    refused: usize,
    reasons: Vec<&'static str>,
}
/// Bounded complete physical results local to an explicit compilation session.
#[derive(Default)]
pub struct PhysicalLayoutSession(
    Mutex<Memo>,
    #[cfg(test)] std::sync::atomic::AtomicUsize,
);
impl PhysicalLayoutSession {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn stats(&self) -> (usize, usize, usize) {
        let m = self.0.lock().unwrap();
        (m.hits, m.builds, m.refused)
    }
    pub fn refusals(&self) -> Vec<&'static str> {
        self.0.lock().unwrap().reasons.clone()
    }
}
fn clone_document(product: &PageRunningDocument, _session: &PhysicalLayoutSession) -> PagedDocument {
    #[cfg(test)]
    _session.1.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    product.document.clone()
}

fn cloned(product: &PageRunningDocument, session: &PhysicalLayoutSession) -> PageRunningDocument {
    PageRunningDocument {
        document: clone_document(product, session),
        trace: product.trace.clone(),
    }
}
fn same_epoch(expected: &Option<MetricEpoch>, current: &Option<MetricEpoch>) -> bool {
    match (expected, current) {
        (None, None) => true,
        (Some(a), Some(b)) => a.same(b),
        _ => false,
    }
}
/// Produces the authoritative discovery trace and retains eligible layout for replay.
pub fn discover<M: FontMetrics + Clone, S: ImageSizer + Clone>(
    body: &PageRunningBody,
    input: TagIntrospector,
    metrics: M,
    sizer: S,
    font_size: f64,
    inputs: PageRunningLayoutInputs<'_>,
    session: &PhysicalLayoutSession,
) -> Result<super::PageRunningTrace, super::PageRunningRequests> {
    layout_with(
        body, input, metrics, sizer, font_size, inputs, session,
        |product| product.trace.clone(),
        |product, retain| {
            if retain {
                (product.trace.clone(), Some(Arc::new(product)))
            } else {
                (product.trace, None)
            }
        },
    )
}

/// Returns a shared physical result for read-only stabilization.
pub fn replay_shared<M: FontMetrics + Clone, S: ImageSizer + Clone>(
    body: &PageRunningBody,
    input: TagIntrospector,
    metrics: M,
    sizer: S,
    font_size: f64,
    inputs: PageRunningLayoutInputs<'_>,
    session: &PhysicalLayoutSession,
) -> Result<Arc<PageRunningDocument>, super::PageRunningRequests> {
    layout_with(
        body, input, metrics, sizer, font_size, inputs, session,
        Arc::clone,
        |product, retain| {
            let product = Arc::new(product);
            let retained = retain.then(|| Arc::clone(&product));
            (product, retained)
        },
    )
}

/// Transfers the selected document at terminal publication.
///
/// Releases the session's retained owner when it refers to this exact result.
/// Other readers keep an immutable snapshot if the document needs to be copied.
pub fn finish_replay(
    product: Arc<PageRunningDocument>,
    session: &PhysicalLayoutSession,
) -> PagedDocument {
    let retained = {
        let mut memo = session.0.lock().unwrap();
        memo.entry
            .take_if(|entry| Arc::ptr_eq(&entry.product, &product))
    };
    drop(retained);
    match Arc::try_unwrap(product) {
        Ok(product) => product.document,
        Err(product) => clone_document(&product, session),
    }
}

pub fn layout<M: FontMetrics + Clone, S: ImageSizer + Clone>(
    body: &PageRunningBody,
    input: TagIntrospector,
    metrics: M,
    sizer: S,
    font_size: f64,
    inputs: PageRunningLayoutInputs<'_>,
    session: &PhysicalLayoutSession,
) -> PageRunningPassOutcome {
    let result = layout_with(
        body, input, metrics, sizer, font_size, inputs, session,
        |product| cloned(product, session),
        |product, retain| {
            let retained = retain.then(|| Arc::new(cloned(&product, session)));
            (product, retained)
        },
    );
    match result {
        Ok(product) => PageRunningPassOutcome::complete(product),
        Err(pending) => PageRunningPassOutcome::pending(pending),
    }
}

#[allow(clippy::too_many_arguments)]
fn layout_with<M: FontMetrics + Clone, S: ImageSizer + Clone, R>(
    body: &PageRunningBody,
    input: TagIntrospector,
    metrics: M,
    sizer: S,
    font_size: f64,
    inputs: PageRunningLayoutInputs<'_>,
    session: &PhysicalLayoutSession,
    on_hit: impl FnOnce(&Arc<PageRunningDocument>) -> R,
    on_production: impl FnOnce(PageRunningDocument, bool) -> (R, Option<Arc<PageRunningDocument>>),
) -> Result<R, super::PageRunningRequests> {
    let prior = session.0.lock().unwrap().entry.take();
    let running = inputs.running.is_some();
    let epoch = metrics.physical_epoch();
    let eligible = body.numbering_input.is_none()
        && inputs.math.is_none()
        && inputs.par_line.is_none()
        && inputs.running.is_none_or(|s| s.resolutions().is_empty())
        && font_size.is_finite()
        && font_size > 0.0
        && !super::convergence::has_bibliography(body.content(), &input)
        && input.page_store.same_scalar_snapshot(&input.page_store);
    if eligible {
        if let Some(entry) = prior {
            if same_epoch(&entry.epoch, &epoch)
                && Arc::ptr_eq(&entry.source, &body.plan)
                && entry.font_size == font_size.to_bits()
                && entry.running == running
                && entry.journal.page_inputs_match(&entry.pages, &input.page_store)
                && entry.journal.validates(&metrics, &input)
                && same_epoch(&entry.epoch, &metrics.physical_epoch())
            {
                let out = on_hit(&entry.product);
                let mut memo = session.0.lock().unwrap();
                memo.hits += 1;
                memo.entry = Some(entry);
                return Ok(out);
            }
        }
    }
    session.0.lock().unwrap().builds += 1;
    if !eligible {
        let mut m = session.0.lock().unwrap();
        m.refused += 1;
        if !m.reasons.contains(&"input gate") {
            m.reasons.push("input gate");
        }
        drop(m);
        return super::layout_with_page_running(
            body, input, metrics, sizer, font_size, inputs,
        )
        .into_result()
        .map(|product| on_production(product, false).0);
    }
    let journal = Journal::default();
    let recorded = providers::RecordingMetrics {
        inner: metrics,
        journal: journal.clone(),
        immutable: epoch.is_some(),
    };
    let images = providers::RecordingSizer { inner: sizer, journal: journal.clone() };
    let result = super::convergence::run_planned(
        body,
        input.clone(),
        recorded.clone(),
        images,
        font_size,
        inputs,
        Some(&journal),
    )
    .into_result();
    match result {
        Ok(product) => {
            let admitted = same_epoch(&epoch, &recorded.inner.physical_epoch())
                && document_bounded(&product.document)
                && journal.validates(&recorded.inner, &input)
                && same_epoch(&epoch, &recorded.inner.physical_epoch());
            let (out, retained) = on_production(product, admitted);
            let mut memo = session.0.lock().unwrap();
            if let Some(product) = retained {
                memo.entry = Some(Entry {
                    epoch,
                    source: body.plan.clone(),
                    pages: input.page_store.clone(),
                    font_size: font_size.to_bits(),
                    running,
                    journal,
                    product,
                });
            } else {
                memo.refused += 1;
                let state = journal.0.lock().unwrap();
                for reason in state
                    .refusals
                    .iter()
                    .copied()
                    .chain(std::iter::once("output or validation"))
                {
                    if !memo.reasons.contains(&reason) {
                        memo.reasons.push(reason);
                    }
                }
            }
            Ok(out)
        }
        Err(pending) => Err(pending),
    }
}
pub(super) fn style_charge(style: &TextStyle) -> Option<usize> {
    super::paragraph_session::style_payload(style, &mut Vec::new())
        .filter(|n| *n <= 64 * 1024)
}
pub(super) fn frame_charge(items: &[&FrameItem]) -> Result<usize, &'static str> {
    fn visit(
        item: &FrameItem,
        depth: usize,
        count: &mut usize,
        size: &mut usize,
    ) -> Result<(), &'static str> {
        if depth > 32 || *count >= 32768 {
            return Err("frame depth or item budget");
        }
        *count += 1;
        *size = size
            .checked_add(std::mem::size_of::<FrameItem>())
            .ok_or("frame size overflow")?;
        let (extra, children) = match item {
            FrameItem::Text { text, style, .. } => (
                text.len()
                    .checked_add(style_charge(style).ok_or("text style")?)
                    .ok_or("text size overflow")?,
                None,
            ),
            FrameItem::TextSeparator { style, .. } => {
                (style_charge(style).ok_or("separator style")?, None)
            }
            FrameItem::TextReplacement { text, items } => {
                (text.len(), Some(items.as_slice()))
            }
            FrameItem::Semantic { alt, items, .. } => {
                (alt.as_ref().map_or(0, |s| s.len()), Some(items.as_slice()))
            }
            FrameItem::Link { target, items, .. } => {
                let n = match target {
                    crate::entities::layout_types::LinkTarget::Url(s) => s.len(),
                    crate::entities::layout_types::LinkTarget::Destination(s) => {
                        s.0.len()
                    }
                };
                (n, Some(items.as_slice()))
            }
            FrameItem::Group { clip_mask, items, .. } => {
                if clip_mask.is_some() {
                    return Err("clipped group");
                }
                (0, Some(items.as_slice()))
            }
            FrameItem::Line { color, .. } => {
                match color {
                    Some(crate::entities::paint::Paint::Solid(
                        crate::entities::color::Color::Spot(_),
                    )) => return Err("spot line"),
                    Some(
                        crate::entities::paint::Paint::Gradient(_)
                        | crate::entities::paint::Paint::Tiling(_),
                    ) => return Err("gradient line"),
                    Some(crate::entities::paint::Paint::Solid(_)) | None => {}
                }
                (0, None)
            }
            FrameItem::TextShaped { .. } => return Err("shaped text"),
            FrameItem::Glyph { .. } => return Err("glyph"),
            FrameItem::Image { .. } => return Err("image frame"),
            FrameItem::Shape { kind, fill, stroke, .. } => {
                use crate::entities::{frame_paint::FramePaint, geometry::ShapeKind};
                let mut charge = match kind {
                    ShapeKind::Path(path) => {
                        if path.len() > 8192 {
                            return Err("shape path budget");
                        }
                        path.len()
                            .checked_mul(std::mem::size_of::<
                                crate::entities::geometry::PathItem,
                            >())
                            .ok_or("shape size overflow")?
                    }
                    ShapeKind::Rect
                    | ShapeKind::RoundedRect { .. }
                    | ShapeKind::Ellipse
                    | ShapeKind::Line { .. } => 0,
                };
                for paint in fill.iter().chain(stroke.iter().map(|s| &s.paint)) {
                    match paint {
                        FramePaint::Solid(color)
                            if !matches!(
                                color,
                                crate::entities::color::Color::Spot(_)
                            ) => {}
                        FramePaint::Solid(_)
                        | FramePaint::Gradient(_)
                        | FramePaint::Tiling(_) => return Err("complex shape paint"),
                    }
                }
                if let Some(dash) = stroke.as_ref().and_then(|s| s.dash.as_ref()) {
                    if dash.array.len() > 1024 {
                        return Err("shape dash budget");
                    }
                    charge = charge
                        .checked_add(
                            dash.array
                                .len()
                                .checked_mul(std::mem::size_of::<
                                    crate::entities::geometry::DashLength,
                                >())
                                .ok_or("shape size overflow")?,
                        )
                        .ok_or("shape size overflow")?;
                }
                (charge, None)
            }
        };
        *size = size.checked_add(extra).ok_or("frame size overflow")?;
        if *size > LIMIT {
            return Err("frame byte budget");
        }
        if let Some(children) = children {
            for child in children {
                visit(child, depth + 1, count, size)?;
            }
        }
        Ok(())
    }
    let mut count = 0;
    let mut size = 0;
    for item in items {
        visit(item, 0, &mut count, &mut size)?;
    }
    Ok(size)
}
fn document_bounded(doc: &PagedDocument) -> bool {
    if doc.pages.len() > 128
        || !doc.attachments.is_empty()
        || !doc.extracted_headings.is_empty()
        || !doc.layout_errors.is_empty()
        || !doc.layout_warnings.is_empty()
    {
        return false;
    }
    let items: Vec<_> = doc
        .pages
        .iter()
        .flat_map(|p| p.background.iter().chain(&p.items).chain(&p.foreground))
        .collect();
    frame_charge(&items).is_ok()
}

#[cfg(test)]
mod physical_retention_tests {
    use super::*;
    use crate::compiler::{
        eval::EvalContext, introspect::page_running::prepare_page_running_body,
        realize::context_occurrences::ContextOccurrenceArena,
    };
    use crate::entities::introspector::TagIntrospector;
    use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
    #[derive(Default)]
    struct State {
        clones: AtomicUsize,
        calls: AtomicUsize,
        epoch: Mutex<Option<super::super::MetricEpoch>>,
        revoke: std::sync::atomic::AtomicBool,
        horizontal: AtomicU64,
        vertical: AtomicU64,
    }
    struct Provider(Arc<State>);
    impl Clone for Provider {
        fn clone(&self) -> Self {
            self.0.clones.fetch_add(1, Ordering::Relaxed);
            Self(self.0.clone())
        }
    }
    impl Provider {
        fn new() -> Self {
            Self(Arc::new(State {
                horizontal: AtomicU64::new(1.0f64.to_bits()),
                vertical: AtomicU64::new(1.0f64.to_bits()),
                ..Default::default()
            }))
        }
    }
    impl FontMetrics for Provider {
        fn physical_epoch(&self) -> Option<super::super::MetricEpoch> {
            self.0.epoch.lock().unwrap().clone()
        }

        fn advance(&self, t: &str, s: Pt, st: &TextStyle) -> Pt {
            self.0.calls.fetch_add(1, Ordering::Relaxed);
            if self.0.revoke.load(Ordering::Relaxed) {
                *self.0.epoch.lock().unwrap() = None;
            }
            FixedMetrics.advance(t, s, st)
                * f64::from_bits(self.0.horizontal.load(Ordering::Relaxed))
        }
        fn vertical_metrics(&self, s: Pt, st: &TextStyle) -> (Pt, Pt) {
            self.0.calls.fetch_add(1, Ordering::Relaxed);
            if self.0.revoke.load(Ordering::Relaxed) {
                *self.0.epoch.lock().unwrap() = None;
            }
            let (a, b) = FixedMetrics.vertical_metrics(s, st);
            let k = f64::from_bits(self.0.vertical.load(Ordering::Relaxed));
            (a * k, b * k)
        }
        fn cap_height(&self, s: Pt, st: &TextStyle) -> Pt {
            self.0.calls.fetch_add(1, Ordering::Relaxed);
            if self.0.revoke.load(Ordering::Relaxed) {
                *self.0.epoch.lock().unwrap() = None;
            }
            FixedMetrics.cap_height(s, st)
                * f64::from_bits(self.0.vertical.load(Ordering::Relaxed))
        }
        fn text_edges(&self, s: Pt, st: &TextStyle) -> (Pt, Pt) {
            self.0.calls.fetch_add(1, Ordering::Relaxed);
            if self.0.revoke.load(Ordering::Relaxed) {
                *self.0.epoch.lock().unwrap() = None;
            }
            let (a, b) = FixedMetrics.text_edges(s, st);
            let k = f64::from_bits(self.0.vertical.load(Ordering::Relaxed));
            (a * k, b * k)
        }
    }
    fn body() -> PageRunningBody {
        let content = Content::sequence(vec![
            Content::text("Alpha beta gamma delta"),
            Content::Parbreak,
            Content::text("Another paragraph"),
        ]);
        let mut arena = ContextOccurrenceArena::new(&content);
        let mut frame = arena.frame();
        let mut ctx = EvalContext::new();
        arena
            .evaluate(&mut frame, &mut ctx, |ctx| {
                prepare_page_running_body(&content, ctx)
            })
            .unwrap()
    }
    fn run(
        body: &PageRunningBody,
        p: Provider,
        session: &PhysicalLayoutSession,
    ) -> PageRunningDocument {
        layout(
            body,
            TagIntrospector::empty(),
            p,
            NullImageSizer,
            11.0,
            PageRunningLayoutInputs { math: None, par_line: None, running: None },
            session,
        )
        .into_result()
        .ok()
        .expect("complete scalar body")
    }
    fn signature(d: &PagedDocument) -> String {
        format!("{:?}", d)
    }
    #[test]
    fn shared_replay_moves_the_selected_document_without_a_full_copy() {
        let session = PhysicalLayoutSession::new();
        let provider = Provider::new();
        let body = body();
        let replay = |session: &PhysicalLayoutSession| {
            replay_shared(
                &body,
                TagIntrospector::empty(),
                provider.clone(),
                NullImageSizer,
                11.0,
                PageRunningLayoutInputs { math: None, par_line: None, running: None },
                session,
            )
            .ok()
            .expect("complete replay")
        };
        let first = replay(&session);
        let selected = replay(&session);
        assert_eq!(
            session.1.load(Ordering::Relaxed),
            0,
            "replay copied the full document"
        );
        assert!(Arc::ptr_eq(&first, &selected));
        assert!(Arc::ptr_eq(&selected.trace.data.source, &body.plan));
        let expected = signature(&first.document);
        drop(first);
        let allocation = selected.document.pages.as_ptr();
        let output = finish_replay(selected, &session);
        assert_eq!(output.pages.as_ptr(), allocation);
        assert_eq!(signature(&output), expected);
        assert_eq!(session.1.load(Ordering::Relaxed), 0);

        let selected = replay(&session);
        let other_reader = selected.clone();
        let mut output = finish_replay(selected, &session);
        output.pages.clear();
        assert_eq!(signature(&other_reader.document), expected);
        assert_eq!(session.1.load(Ordering::Relaxed), 1);

        let obsolete = replay(&session);
        let current_body = self::body();
        let current = replay_shared(
            &current_body,
            TagIntrospector::empty(),
            provider.clone(),
            NullImageSizer,
            11.0,
            PageRunningLayoutInputs { math: None, par_line: None, running: None },
            &session,
        )
        .ok()
        .expect("complete replay for new plan");
        finish_replay(obsolete, &session);
        let again = replay_shared(
            &current_body,
            TagIntrospector::empty(),
            provider,
            NullImageSizer,
            11.0,
            PageRunningLayoutInputs { math: None, par_line: None, running: None },
            &session,
        )
        .ok()
        .expect("same current replay");
        assert!(Arc::ptr_eq(&current, &again), "publication evicted another product");
    }

    #[test]
    fn discovery_avoids_full_document_clones() {
        let session = PhysicalLayoutSession::new();
        let provider = Provider::new();
        let body = body();
        for _ in 0..2 {
            let trace = discover(
                &body,
                TagIntrospector::empty(),
                provider.clone(),
                NullImageSizer,
                11.0,
                PageRunningLayoutInputs { math: None, par_line: None, running: None },
                &session,
            )
            .ok()
            .expect("complete discovery");
            assert!(trace.data.complete);
            assert!(Arc::ptr_eq(&trace.data.source, &body.plan));
        }
        assert_eq!(session.stats(), (1, 1, 0));
        assert_eq!(
            session.1.load(Ordering::Relaxed),
            0,
            "discovery cloned the full document"
        );
        let hot = run(&body, provider.clone(), &session);
        let cold = run(&body, provider, &PhysicalLayoutSession::new());
        assert_eq!(signature(&hot.document), signature(&cold.document));
        assert_eq!(session.1.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn physical_retention_same_plan_avoids_the_physical_coordinator() {
        let session = PhysicalLayoutSession::new();
        let p = Provider::new();
        let body = body();
        let first = run(&body, p.clone(), &session);
        let provider = p.clone();
        let before = p.0.clones.load(Ordering::Relaxed);
        let second = run(&body, provider, &session);
        assert_eq!(
            p.0.clones.load(Ordering::Relaxed),
            before,
            "unchanged complete pass must not instantiate another physical layouter"
        );
        assert_eq!(session.stats().0, 1);
        assert_eq!(signature(&first.document), signature(&second.document));
        let mut changed = second;
        changed.document.pages[0].items.clear();
        let third = run(&body, p.clone(), &session);
        assert_eq!(signature(&first.document), signature(&third.document));
    }
    #[test]
    fn physical_retention_changed_metric_answers_recompute_current_geometry() {
        let session = PhysicalLayoutSession::new();
        let p = Provider::new();
        let body = body();
        let first = run(&body, p.clone(), &session);
        for horizontal in [true, false] {
            if horizontal {
                p.0.horizontal.store(1.5f64.to_bits(), Ordering::Relaxed)
            } else {
                p.0.vertical.store(1.7f64.to_bits(), Ordering::Relaxed)
            }
            let provider = p.clone();
            let before = p.0.clones.load(Ordering::Relaxed);
            let changed = run(&body, provider, &session);
            assert!(p.0.clones.load(Ordering::Relaxed) > before);
            assert_eq!(session.stats().0, 0);
            let cold = run(&body, p.clone(), &PhysicalLayoutSession::new());
            assert_eq!(signature(&changed.document), signature(&cold.document));
            assert_ne!(signature(&first.document), signature(&changed.document));
        }
    }
    #[test]
    fn physical_retention_new_source_plan_never_reuses_old_trace() {
        let session = PhysicalLayoutSession::new();
        let p = Provider::new();
        let a = body();
        let b = body();
        run(&a, p.clone(), &session);
        let provider = p.clone();
        let before = p.0.clones.load(Ordering::Relaxed);
        run(&b, provider, &session);
        assert!(p.0.clones.load(Ordering::Relaxed) > before);
        assert_eq!(session.stats().0, 0);
    }
    #[test]
    fn physical_retention_page_snapshot_change_recomputes_numbering_total() {
        use std::num::NonZeroUsize;
        let session = PhysicalLayoutSession::new();
        let p = Provider::new();
        let content = Content::sequence(vec![
            Content::SetPage(Arc::new(crate::entities::content::SetPageDelta {
                numbering: Some(Some("1 / 1".into())),
                ..Default::default()
            })),
            Content::text("Body"),
        ]);
        let mut arena = ContextOccurrenceArena::new(&content);
        let mut frame = arena.frame();
        let mut ctx = EvalContext::new();
        let body = arena
            .evaluate(&mut frame, &mut ctx, |ctx| {
                prepare_page_running_body(&content, ctx)
            })
            .unwrap();
        run(&body, p.clone(), &session);
        let mut input = TagIntrospector::empty();
        input.page_store = PageStore::from_realized(
            NonZeroUsize::new(1).unwrap(),
            vec![None],
            vec![9],
            vec![crate::entities::span::Span::detached()],
            vec![None],
            vec![None],
            vec![Content::text("Page")],
        );
        let cold = super::super::layout_with_page_running(
            &body,
            input.clone(),
            p.clone(),
            NullImageSizer,
            11.0,
            PageRunningLayoutInputs { math: None, par_line: None, running: None },
        )
        .into_result()
        .ok()
        .unwrap();
        let footer = |document: &PagedDocument| {
            document.pages[0]
                .foreground
                .iter()
                .filter_map(|item| {
                    if let FrameItem::Text { text, .. } = item {
                        Some(text.to_string())
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        };
        assert!(
            footer(&cold.document).contains(&"9 / 9".to_string()),
            "cold facade must observe both current logical page and total: {:?}",
            footer(&cold.document)
        );
        let result = layout(
            &body,
            input,
            p.clone(),
            NullImageSizer,
            11.0,
            PageRunningLayoutInputs { math: None, par_line: None, running: None },
            &session,
        )
        .into_result()
        .ok()
        .unwrap();
        assert_eq!(footer(&result.document), footer(&cold.document));
        assert_eq!(session.stats().0, 0);
    }
    #[test]
    fn seeded_page_replay_keeps_only_equivalent_effective_numbering() {
        use std::num::NonZeroUsize;
        let p = Provider::new();
        let content = Content::sequence(vec![
            Content::SetPage(Arc::new(crate::entities::content::SetPageDelta {
                numbering: Some(Some("1 / 1".into())),
                ..Default::default()
            })),
            Content::text("Body"),
        ]);
        let mut arena = ContextOccurrenceArena::new(&content);
        let mut frame = arena.frame();
        let mut ctx = EvalContext::new();
        let body = arena
            .evaluate(&mut frame, &mut ctx, |ctx| {
                prepare_page_running_body(&content, ctx)
            })
            .unwrap();
        let make_store = |total: usize, number| PageStore::from_realized(
            NonZeroUsize::new(total).unwrap(),
            vec![None; total],
            vec![number; total],
            vec![crate::entities::span::Span::detached(); total],
            vec![None; total],
            vec![None; total],
            vec![Content::text("Page"); total],
        );
        let compile = |store, session: &PhysicalLayoutSession| {
            let mut index = TagIntrospector::empty();
            index.page_store = store;
            layout(
                &body, index, p.clone(), NullImageSizer, 11.0,
                PageRunningLayoutInputs { math: None, par_line: None, running: None },
                session,
            ).into_result().ok().expect("complete numbered body")
        };
        let session = PhysicalLayoutSession::new();
        let first = compile(make_store(1, 1), &session);
        let stable = compile(PageStore::from_total_pages(NonZeroUsize::new(2).unwrap()), &session);
        let cold = compile(PageStore::from_total_pages(NonZeroUsize::new(2).unwrap()), &PhysicalLayoutSession::new());
        assert_eq!(signature(&stable.document), signature(&cold.document));
        assert_eq!(signature(&first.document), signature(&stable.document));
        assert_eq!(session.stats().0, 1, "seeded replay repeated the physical traversal");

        let moved = compile(make_store(2, 9), &session);
        let cold = compile(make_store(2, 9), &PhysicalLayoutSession::new());
        assert_eq!(signature(&moved.document), signature(&cold.document));
        assert_ne!(signature(&stable.document), signature(&moved.document));
        assert_eq!(session.stats().0, 1);
        let unseeded = compile(PageStore::empty(), &session);
        let cold = compile(PageStore::empty(), &PhysicalLayoutSession::new());
        assert_eq!(signature(&unseeded.document), signature(&cold.document));
        assert_eq!(session.stats().0, 1);

        let session = PhysicalLayoutSession::new();
        let unseeded = compile(PageStore::empty(), &session);
        let seeded = compile(make_store(1, 1), &session);
        let cold = compile(make_store(1, 1), &PhysicalLayoutSession::new());
        assert_eq!(signature(&seeded.document), signature(&cold.document));
        assert_eq!(signature(&seeded.document), signature(&unseeded.document));
        assert_eq!(session.stats().0, 1, "first complete unseeded traversal was repeated");

        let session = PhysicalLayoutSession::new();
        compile(PageStore::empty(), &session);
        let seeded = compile(make_store(1, 9), &session);
        let cold = compile(make_store(1, 9), &PhysicalLayoutSession::new());
        assert_eq!(signature(&seeded.document), signature(&cold.document));
        assert_ne!(signature(&seeded.document), signature(&unseeded.document));
        assert_eq!(session.stats().0, 0);
    }

    #[test]
    fn physical_retention_font_size_and_running_presence_are_part_of_input() {
        let session = PhysicalLayoutSession::new();
        let p = Provider::new();
        let body = body();
        run(&body, p.clone(), &session);
        let changed = layout(
            &body,
            TagIntrospector::empty(),
            p.clone(),
            NullImageSizer,
            13.0,
            PageRunningLayoutInputs { math: None, par_line: None, running: None },
            &session,
        )
        .into_result()
        .ok()
        .unwrap();
        let cold = layout(
            &body,
            TagIntrospector::empty(),
            p.clone(),
            NullImageSizer,
            13.0,
            PageRunningLayoutInputs { math: None, par_line: None, running: None },
            &PhysicalLayoutSession::new(),
        )
        .into_result()
        .ok()
        .unwrap();
        assert_eq!(signature(&changed.document), signature(&cold.document));
        assert_eq!(session.stats().0, 0);
        let empty = SealedPageRunning::new(vec![]).unwrap();
        layout(
            &body,
            TagIntrospector::empty(),
            p.clone(),
            NullImageSizer,
            13.0,
            PageRunningLayoutInputs { math: None, par_line: None, running: Some(&empty) },
            &session,
        )
        .into_result()
        .ok()
        .unwrap();
        assert_eq!(session.stats().0, 0);
    }
    #[test]
    fn physical_retention_sessions_are_independent() {
        let session = PhysicalLayoutSession::new();
        let p = Provider::new();
        let body = body();
        run(&body, p.clone(), &session);
        let fresh = PhysicalLayoutSession::new();
        let provider = p.clone();
        let before = p.0.clones.load(Ordering::Relaxed);
        run(&body, provider, &fresh);
        assert!(p.0.clones.load(Ordering::Relaxed) > before);
        assert_eq!(fresh.stats().0, 0);
    }
    fn epoch() -> super::super::MetricEpoch {
        super::super::MetricEpoch::new(Arc::new(()), Arc::new(()))
    }
    #[test]
    fn metric_epoch_reuse_avoids_geometry_replay_and_preserves_output() {
        let p = Provider::new();
        *p.0.epoch.lock().unwrap() = Some(epoch());
        let body = body();
        let session = PhysicalLayoutSession::new();
        let first = run(&body, p.clone(), &session);
        let before = p.0.calls.load(Ordering::Relaxed);
        assert!(before > 0);
        let second = run(&body, p.clone(), &session);
        assert_eq!(
            p.0.calls.load(Ordering::Relaxed),
            before,
            "unchanged immutable epoch must avoid metric replay"
        );
        assert_eq!(signature(&first.document), signature(&second.document));
        assert_eq!(session.stats().0, 1);
    }
    #[test]
    fn metric_epoch_changed_owner_forces_original_layout() {
        let p = Provider::new();
        *p.0.epoch.lock().unwrap() = Some(epoch());
        let body = body();
        let session = PhysicalLayoutSession::new();
        run(&body, p.clone(), &session);
        *p.0.epoch.lock().unwrap() = Some(epoch());
        let provider = p.clone();
        let before = p.0.clones.load(Ordering::Relaxed);
        run(&body, provider, &session);
        assert!(
            p.0.clones.load(Ordering::Relaxed) > before,
            "changed epoch must execute the physical coordinator"
        );
        assert_eq!(session.stats().0, 0);
    }
    #[test]
    fn metric_epoch_revoked_during_pass_never_retains_incomplete_journal() {
        let p = Provider::new();
        *p.0.epoch.lock().unwrap() = Some(epoch());
        p.0.revoke.store(true, Ordering::Relaxed);
        let body = body();
        let session = PhysicalLayoutSession::new();
        run(&body, p.clone(), &session);
        let provider = p.clone();
        let before = p.0.clones.load(Ordering::Relaxed);
        run(&body, provider, &session);
        assert!(
            p.0.clones.load(Ordering::Relaxed) > before,
            "revoked capture must not retain an incomplete journal"
        );
        assert_eq!(session.stats().0, 0);
    }
    #[test]
    fn metric_epoch_changed_geometry_and_generation_matches_cold() {
        let p = Provider::new();
        *p.0.epoch.lock().unwrap() = Some(epoch());
        let body = body();
        let session = PhysicalLayoutSession::new();
        let first = run(&body, p.clone(), &session);
        p.0.horizontal.store(1.7f64.to_bits(), Ordering::Relaxed);
        *p.0.epoch.lock().unwrap() = Some(epoch());
        let changed = run(&body, p.clone(), &session);
        let cold = run(&body, p.clone(), &PhysicalLayoutSession::new());
        assert_eq!(signature(&changed.document), signature(&cold.document));
        assert_ne!(signature(&first.document), signature(&changed.document));
        assert_eq!(session.stats().0, 0);
    }
}
