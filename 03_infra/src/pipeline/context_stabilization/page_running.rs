//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/pipeline/context_stabilization/page_running.md
//! @prompt-hash 75d52aeb
//! @layer L3
//! @updated 2026-10-21

use super::*;
use std::num::NonZeroUsize;
use typst_core::compiler::introspect::page_running::{
    introspect_with_page_running, prepare_page_running_body, PageRunningIndexInput,
};
use typst_core::compiler::layout::{
    layout_with_page_running, PageRunningBody, PageRunningKey, PageRunningLayoutInputs,
    PageRunningRequest, PageRunningResolution, PageRunningTrace, SealedPageRunning,
};
use typst_core::compiler::realize::page_running::{
    bind_page_running_content, realize_page_running,
};

struct Contribution {
    key: PageRunningKey,
    generation: u64,
    original: PageRunningResolution,
    bound: PageRunningResolution,
    roots: Vec<usize>,
    binding_versions: Option<Vec<(usize, u64)>>,
    warnings: Vec<SourceDiagnostic>,
}

struct AllocationFrames {
    realize: ContextOccurrenceFrame,
    bind: ContextOccurrenceFrame,
}

#[derive(Default)]
struct LayoutCallbacks {
    math: Option<typst_core::compiler::math::layout::callbacks::SealedMathCallbacks>,
    par_line: Option<typst_core::compiler::layout::SealedParLineCallbacks>,
    passes: usize,
    closed: bool,
    warnings: Vec<SourceDiagnostic>,
}

struct PhysicalStep {
    body: PageRunningBody,
    target: EvalTarget,
    running: Option<(PageRunningTrace, SealedPageRunning)>,
    document: Option<PagedDocument>,
    warnings: Vec<SourceDiagnostic>,
}

pub(super) struct PhysicalCandidate {
    pub(super) index: TagIntrospector,
    pub(super) document: Option<PagedDocument>,
    pub(super) complete: bool,
    pub(super) warnings: Vec<SourceDiagnostic>,
}

struct OutlineExecution {
    ctx: EvalContext,
    input: Arc<TagIntrospector>,
    result: SourceResult<Content>,
    warnings: Vec<SourceDiagnostic>,
}

#[derive(Default)]
struct OutlineState {
    first_frame: Option<ContextOccurrenceFrame>,
    focal_frame: Option<ContextOccurrenceFrame>,
    executions: Vec<OutlineExecution>,
    retired: Vec<OutlineExecution>,
    product: Option<Content>,
    roots: Vec<usize>,
    generation: u64,
}

#[derive(Default)]
pub(super) struct RunningState {
    generation: u64,
    document_index: Option<usize>,
    body: Option<PageRunningBody>,
    body_frame: Option<ContextOccurrenceFrame>,
    index_frame: Option<ContextOccurrenceFrame>,
    versions: Vec<(usize, u64)>,
    outline: OutlineState,
    planned_outline_generation: u64,
    contributions: Vec<Contribution>,
    allocation_frames: HashMap<PageRunningKey, AllocationFrames>,
    failed_warnings: Vec<SourceDiagnostic>,
    bundle: Vec<RunningState>,
    numbering_partition: Option<Vec<usize>>,
}

/// Counts come only from these actual document steps, never request/paint inference.
fn document_page_counts(steps: &[PhysicalStep], targets: &[bool]) -> Option<Vec<usize>> {
    if steps.len() != targets.len() {
        return None;
    }
    steps
        .iter()
        .zip(targets)
        .map(|(step, &html)| {
            if html {
                (step.target == EvalTarget::Html && step.document.is_none()).then_some(0)
            } else if step.target == EvalTarget::Paged {
                let count = step.document.as_ref()?.pages.len();
                (count > 0).then_some(count)
            } else {
                None
            }
        })
        .collect()
}

fn document_numbering_ranges(
    input: &TagIntrospector,
    counts: Option<&[usize]>,
    targets: &[bool],
) -> Option<Vec<Option<(NonZeroUsize, NonZeroUsize)>>> {
    let counts = counts?;
    let total = input.page_store.total_pages()?.get();
    if counts.len() != targets.len() {
        return None;
    }
    let mut offset = 0usize;
    let mut ranges = Vec::with_capacity(counts.len());
    for (&count, &html) in counts.iter().zip(targets) {
        if html {
            if count != 0 {
                return None;
            }
            ranges.push(None);
        } else {
            let size = NonZeroUsize::new(count)?;
            let first = NonZeroUsize::new(offset.checked_add(1)?)?;
            offset = offset.checked_add(count)?;
            if offset > total {
                return None;
            }
            ranges.push(Some((first, size)));
        }
    }
    (offset == total).then_some(ranges)
}

/// Only the exact immutable outer capture proves unchanged execution inputs.
/// Producer-object continuity separately permits allocation-frame retention.
fn same_running_capture(left: &Content, right: &Content) -> bool {
    match left {
        Content::Source(a) => {
            if let Content::Source(b) = right {
                Arc::ptr_eq(a, b)
            } else {
                false
            }
        }
        Content::ShowRecipe(a) => {
            if let Content::ShowRecipe(b) = right {
                Arc::ptr_eq(a, b)
            } else {
                false
            }
        }
        Content::Realized(a) => {
            if let Content::Realized(b) = right {
                Arc::ptr_eq(a, b)
            } else {
                false
            }
        }
        Content::Located(a) => {
            if let Content::Located(b) = right {
                Arc::ptr_eq(a, b)
            } else {
                false
            }
        }
        Content::Styled(a) => {
            if let Content::Styled(b) = right {
                Arc::ptr_eq(a, b)
            } else {
                false
            }
        }
        Content::Sequence(a) => {
            if let Content::Sequence(b) = right {
                Arc::ptr_eq(a, b)
            } else {
                false
            }
        }
        Content::SetPage(..)
        | Content::PageRun(..)
        | Content::Empty
        | Content::Text(..)
        | Content::Space
        | Content::SpaceFromNewline
        | Content::Parbreak
        | Content::HtmlElem(..)
        | Content::HtmlFrame(..)
        | Content::Par { .. }
        | Content::Heading(..)
        | Content::Title(..)
        | Content::Strong(..)
        | Content::Emph(..)
        | Content::Raw(..)
        | Content::ListItem(..)
        | Content::EnumItem(..)
        | Content::Link(..)
        | Content::Equation(..)
        | Content::MathSequence(..)
        | Content::Symbol(..)
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
        | Content::Label(..)
        | Content::Ref(..)
        | Content::CounterDisplay(..)
        | Content::CounterUpdate(..)
        | Content::Outline(..)
        | Content::OutlineEntry(..)
        | Content::Figure(..)
        | Content::FigureCaption(..)
        | Content::Image(..)
        | Content::Shape(..)
        | Content::Curve(..)
        | Content::Transform(..)
        | Content::Grid(..)
        | Content::GridHeader(..)
        | Content::GridFooter(..)
        | Content::GridCell(..)
        | Content::Align(..)
        | Content::Place(..)
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
        | Content::SmallCaps { .. }
        | Content::Pad(..)
        | Content::Hide(..)
        | Content::HSpace(..)
        | Content::VSpace(..)
        | Content::Pagebreak(..)
        | Content::Colbreak(..)
        | Content::Stack(..)
        | Content::Boxed(..)
        | Content::Block(..)
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
        | Content::Repeat(..)
        | Content::Columns(..)
        | Content::Metadata(..)
        | Content::State(..)
        | Content::StateUpdate(..)
        | Content::StateDisplay(..)
        | Content::CounterDisplayCallback(..)
        | Content::ContextBlock(..)
        | Content::Dynamic(..) => false,
    }
}

/// Inspect reached static visual members through the existing public surface.
/// Operational templates and Realized originals are not reached body work.
/// Plain numbering declarations and heading Outlines join the approved channel.
pub(super) fn has_reached_physical_content(content: &Content) -> bool {
    let mut found = false;
    match content {
        Content::Source(e) => {
            found |= has_reached_physical_content(&e.body);
        }
        Content::ShowRecipe(e) => {
            found |= has_reached_physical_content(&e.body);
        }
        Content::Located(e) => {
            found |= has_reached_physical_content(&e.body);
        }
        Content::Realized(e) => {
            found |= has_reached_physical_content(&e.visual);
        }
        Content::Sequence(parts) | Content::MathSequence(parts) => {
            for child in parts.iter() {
                found |= has_reached_physical_content(child);
            }
        }
        Content::Par { body } | Content::SmallCaps { body } => {
            found |= has_reached_physical_content(body)
        }
        Content::HtmlElem(e) => {
            if let typst_core::entities::html::HtmlBody::Content(body) = &e.body {
                found |= has_reached_physical_content(body);
            }
        }
        Content::PageRun(e) => {
            found |= matches!(
                &e.numbering,
                Some(Some(typst_core::entities::numbering::Numbering::Pattern(_)))
            );
            found |= has_reached_physical_content(&e.body);
        }
        Content::SetPage(e) => {
            found |= matches!(
                &e.numbering,
                Some(Some(typst_core::entities::numbering::Numbering::Pattern(_)))
            );
        }
        Content::HtmlFrame(e) => found |= has_reached_physical_content(&e.body),
        Content::Heading(e) => found |= has_reached_physical_content(&e.body),
        Content::Title(e) => found |= has_reached_physical_content(&e.body),
        Content::Strong(e) => found |= has_reached_physical_content(&e.body),
        Content::Emph(e) => found |= has_reached_physical_content(&e.body),
        Content::ListItem(e) => found |= has_reached_physical_content(&e.body),
        Content::EnumItem(e) => found |= has_reached_physical_content(&e.body),
        Content::Link(e) => found |= has_reached_physical_content(&e.body),
        Content::Label(e) => found |= has_reached_physical_content(&e.body),
        Content::Equation(e) => found |= has_reached_physical_content(&e.body),
        Content::MathStretch(e) => found |= has_reached_physical_content(&e.body),
        Content::MathDelimited(e) => found |= has_reached_physical_content(&e.body),
        Content::MathLr(e) => found |= has_reached_physical_content(&e.body),
        Content::MathMid(e) => found |= has_reached_physical_content(&e.body),
        Content::MathCancel(e) => found |= has_reached_physical_content(&e.body),
        Content::MathUnderline(e) => found |= has_reached_physical_content(&e.body),
        Content::MathOverline(e) => found |= has_reached_physical_content(&e.body),
        Content::MathClassOverride(e) => found |= has_reached_physical_content(&e.body),
        Content::MathLimitsOverride(e) => found |= has_reached_physical_content(&e.body),
        Content::MathStyled(e) => found |= has_reached_physical_content(&e.body),
        Content::PdfArtifact(e) => found |= has_reached_physical_content(&e.body),
        Content::Underline(e) => found |= has_reached_physical_content(&e.body),
        Content::Strike(e) => found |= has_reached_physical_content(&e.body),
        Content::Overline(e) => found |= has_reached_physical_content(&e.body),
        Content::Pad(e) => found |= has_reached_physical_content(&e.body),
        Content::Hide(e) => found |= has_reached_physical_content(&e.body),
        Content::Block(e) => found |= has_reached_physical_content(&e.body),
        Content::Boxed(e) => found |= has_reached_physical_content(&e.body),
        Content::Repeat(_) => {
            found = true;
        }
        Content::Columns(e) => found |= has_reached_physical_content(&e.body),
        Content::Transform(e) => found |= has_reached_physical_content(&e.body),
        Content::GridHeader(e) => found |= has_reached_physical_content(&e.body),
        Content::GridFooter(e) => found |= has_reached_physical_content(&e.body),
        Content::GridCell(e) => found |= has_reached_physical_content(&e.body),
        Content::TableHeader(e) => found |= has_reached_physical_content(&e.body),
        Content::TableFooter(e) => found |= has_reached_physical_content(&e.body),
        Content::Footnote(e) => found |= has_reached_physical_content(&e.body),
        Content::Align(e) => found |= has_reached_physical_content(&e.body),
        Content::Place(e) => found |= has_reached_physical_content(&e.body),
        Content::Styled(e) => found |= has_reached_physical_content(&e.body),
        Content::Figure(e) => {
            found |= has_reached_physical_content(&e.body);
            if let Some(c) = &e.caption {
                found |= has_reached_physical_content(c);
            }
        }
        Content::FigureCaption(e) => {
            found |= has_reached_physical_content(&e.body);
            if let Some(c) = &e.separator {
                found |= has_reached_physical_content(c);
            }
        }
        Content::MathFrac(e) => {
            found |= has_reached_physical_content(&e.num);
            found |= has_reached_physical_content(&e.den);
        }
        Content::MathAttach(e) => {
            found |= has_reached_physical_content(&e.base);
            for slot in [&e.t, &e.b, &e.tl, &e.bl, &e.tr, &e.br] {
                if let typst_core::entities::elements::math_attach::MathAttachSlot::Present(
                    c,
                ) = slot
                {
                    found |= has_reached_physical_content(c);
                }
            }
        }
        Content::MathRoot(e) => {
            if let Some(c) = &e.index {
                found |= has_reached_physical_content(c);
            }
            found |= has_reached_physical_content(&e.radicand);
        }
        Content::MathOverbrace(e) => {
            found |= has_reached_physical_content(&e.body);
            if let Some(c) = &e.annotation {
                found |= has_reached_physical_content(c);
            }
        }
        Content::MathUnderbrace(e) => {
            found |= has_reached_physical_content(&e.body);
            if let Some(c) = &e.annotation {
                found |= has_reached_physical_content(c);
            }
        }
        Content::MathUnderbracket(e) => {
            found |= has_reached_physical_content(&e.body);
            if let Some(c) = &e.annotation {
                found |= has_reached_physical_content(c);
            }
        }
        Content::MathUnderparen(e) => {
            found |= has_reached_physical_content(&e.body);
            if let Some(c) = &e.annotation {
                found |= has_reached_physical_content(c);
            }
        }
        Content::MathUndershell(e) => {
            found |= has_reached_physical_content(&e.body);
            if let Some(c) = &e.annotation {
                found |= has_reached_physical_content(c);
            }
        }
        Content::MathOverbracket(e) => {
            found |= has_reached_physical_content(&e.body);
            if let Some(c) = &e.annotation {
                found |= has_reached_physical_content(c);
            }
        }
        Content::MathOverparen(e) => {
            found |= has_reached_physical_content(&e.body);
            if let Some(c) = &e.annotation {
                found |= has_reached_physical_content(c);
            }
        }
        Content::MathOvershell(e) => {
            found |= has_reached_physical_content(&e.body);
            if let Some(c) = &e.annotation {
                found |= has_reached_physical_content(c);
            }
        }
        Content::MathMatrix(e) => {
            for row in &e.rows {
                for c in row {
                    found |= has_reached_physical_content(c);
                }
            }
        }
        Content::MathCases(e) => {
            for c in &e.children {
                found |= has_reached_physical_content(c);
            }
        }
        Content::MathVec(e) => {
            for c in &e.children {
                found |= has_reached_physical_content(c);
            }
        }
        Content::MathAccent(e) => found |= has_reached_physical_content(&e.base),
        Content::MathUnderover(e) => {
            found |= has_reached_physical_content(&e.base);
            for c in [&e.under, &e.over].into_iter().flatten() {
                found |= has_reached_physical_content(c);
            }
        }
        Content::MathOp(e) => found |= has_reached_physical_content(&e.text),
        Content::Terms(e) => {
            for c in &e.items {
                found |= has_reached_physical_content(c);
            }
        }
        Content::TermItem(e) => {
            found |= has_reached_physical_content(&e.term);
            found |= has_reached_physical_content(&e.description);
        }
        Content::Quote(e) => {
            found |= has_reached_physical_content(&e.body);
            if let Some(c) = &e.attribution {
                found |= has_reached_physical_content(c);
            }
        }
        Content::Stack(e) => {
            for c in e.children.iter() {
                found |= has_reached_physical_content(c);
            }
        }
        Content::Outline(e) => {
            found |= e.target
                == typst_core::entities::elements::outline::OutlineTarget::Headings;
            if let Some(c) = &e.title {
                found |= has_reached_physical_content(c);
            }
            if let Some(c) = e.realized_body() {
                found |= has_reached_physical_content(c);
            }
        }
        Content::OutlineEntry(e) => {
            found |= has_reached_physical_content(&e.element);
            if let Some(c) = &e.fill {
                found |= has_reached_physical_content(c);
            }
            if let Some(c) = e.prefix() {
                found |= has_reached_physical_content(&c);
            }
        }
        Content::Grid(e) => {
            for c in &e.cells {
                found |= has_reached_physical_content(c);
            }
            for c in [&e.header, &e.footer].into_iter().flatten() {
                found |= has_reached_physical_content(c);
            }
        }
        Content::Table(e) => {
            if let Some(plan) = &e.resolved {
                for c in &plan.cells {
                    found |= has_reached_physical_content(c);
                }
            } else if let Some(items) = &e.items {
                for c in items {
                    found |= has_reached_physical_content(c);
                }
            } else {
                for c in &e.children {
                    found |= has_reached_physical_content(c);
                }
                for c in [&e.header, &e.footer].into_iter().flatten() {
                    found |= has_reached_physical_content(c);
                }
            }
            if let Some(c) = &e.caption {
                found |= has_reached_physical_content(c);
            }
        }
        Content::TableCell(e) => {
            if let Some(resolved) = &e.resolved {
                found |= has_reached_physical_content(&resolved.visual);
            } else if let Some(standalone) = &e.standalone {
                found |= has_reached_physical_content(&standalone.visual);
            } else {
                found |= has_reached_physical_content(&e.body);
            }
        }
        Content::Bibliography(e) => {
            if let Some(c) = &e.title {
                found |= has_reached_physical_content(c);
            }
        }
        Content::Cite(e) => {
            if let Some(c) = &e.supplement {
                found |= has_reached_physical_content(c);
            }
        }
        Content::FootnoteEntry(e) => {
            found |= has_reached_physical_content(&e.note);
            if let Some(c) = &e.separator {
                found |= has_reached_physical_content(c);
            }
        }
        Content::Document { body, title, .. } => {
            found |= has_reached_physical_content(body);
            if let Some(c) = title {
                found |= has_reached_physical_content(c);
            }
        }
        Content::Empty
        | Content::Text(..)
        | Content::Space
        | Content::SpaceFromNewline
        | Content::Parbreak
        | Content::Raw(..)
        | Content::Symbol(..)
        | Content::MathIdent(..)
        | Content::MathText(..)
        | Content::MathPrimes(..)
        | Content::MathAlignPoint(..)
        | Content::Linebreak(..)
        | Content::Ref(..)
        | Content::CounterDisplay(..)
        | Content::CounterUpdate(..)
        | Content::Image(..)
        | Content::Shape(..)
        | Content::Curve(..)
        | Content::Flush(..)
        | Content::Divider(..)
        | Content::Asset { .. }
        | Content::SmartQuote(..)
        | Content::PdfAttach(..)
        | Content::HSpace(..)
        | Content::VSpace(..)
        | Content::Pagebreak(..)
        | Content::Colbreak(..)
        | Content::GridHLine(..)
        | Content::GridVLine(..)
        | Content::TableHLine(..)
        | Content::TableVLine(..)
        | Content::Metadata(..)
        | Content::State(..)
        | Content::StateUpdate(..)
        | Content::StateDisplay(..)
        | Content::CounterDisplayCallback(..)
        | Content::ContextBlock(..)
        | Content::Dynamic(..) => {}
    }
    found
}

pub(super) fn has_potential(content: &Content) -> bool {
    let mut found = false;
    let _ = content.map_content(&mut |node| {
        if let Content::Located(located) = node {
            found |= has_potential(&located.body);
        }
        if let Content::SetPage(page) = node {
            found |= matches!(
                &page.header,
                Some(typst_core::entities::page_running::PageMarginal::Content(_))
            ) || matches!(
                &page.footer,
                Some(typst_core::entities::page_running::PageMarginal::Content(_))
            ) || matches!(&page.background, Some(Some(_)))
                || matches!(&page.foreground, Some(Some(_)));
        }
        if let Content::PageRun(page) = node {
            found |= matches!(
                &page.header,
                Some(typst_core::entities::page_running::PageMarginal::Content(_))
            ) || matches!(
                &page.footer,
                Some(typst_core::entities::page_running::PageMarginal::Content(_))
            ) || matches!(&page.background, Some(Some(_)))
                || matches!(&page.foreground, Some(Some(_)));
        }
        Ok(None)
    });
    found || has_reached_physical_content(content)
}

fn retain_input_numbering_views(index: &mut TagIntrospector, input: &TagIntrospector) {
    use typst_core::entities::page_store::PageStore;

    let current = &index.page_store;
    let Some(total) = current.total_pages() else { return };
    // Source construction already supplies the global physical order and current
    // document offsets. Only the previous layout's two realized views travel
    // through this boundary; all source-derived page facts stay current.
    let pages: Vec<_> = (1..=total.get())
        .map(|page| NonZeroUsize::new(page).expect("physical pages start at one"))
        .collect();
    let retained = PageStore::from_realized(
        total,
        pages
            .iter()
            .map(|&page| current.numbering_for_page(page).cloned())
            .collect(),
        pages
            .iter()
            .map(|&page| {
                current
                    .logical_number_for_page(page)
                    .expect("physical source index supplies each logical page number")
            })
            .collect(),
        pages
            .iter()
            .map(|&page| {
                current
                    .numbering_span_for_page(page)
                    .expect("physical source index supplies each numbering span")
            })
            .collect(),
        pages
            .iter()
            .map(|&page| input.page_store.visible_numbering_for_page(page).cloned())
            .collect(),
        pages
            .iter()
            .map(|&page| input.page_store.reference_numbering_for_page(page).cloned())
            .collect(),
        pages
            .iter()
            .map(|&page| {
                current
                    .supplement_for_page(page)
                    .expect("physical source index supplies each page supplement")
                    .clone()
            })
            .collect(),
    );
    index.inject_pages(retained);
}

impl Session<'_> {
    fn retire_running_contribution(&mut self, contribution: Contribution) {
        for index in contribution.roots {
            self.discard_descendants(index);
            self.nodes[index].active = false;
            if let Some(execution) = self.nodes[index].current.take() {
                self.retired.push(*execution);
            }
        }
    }

    fn running_operation<T>(
        &mut self,
        frame: &mut ContextOccurrenceFrame,
        input: &TagIntrospector,
        operation: impl FnOnce(&mut EvalContext, &mut Engine<'_>) -> SourceResult<T>,
    ) -> (SourceResult<T>, Vec<SourceDiagnostic>) {
        let (result, _, warnings) = self.running_transaction(frame, input, operation);
        (result, warnings)
    }

    fn running_transaction<T>(
        &mut self,
        frame: &mut ContextOccurrenceFrame,
        input: &TagIntrospector,
        operation: impl FnOnce(&mut EvalContext, &mut Engine<'_>) -> SourceResult<T>,
    ) -> (SourceResult<T>, EvalContext, Vec<SourceDiagnostic>) {
        let mut ctx = EvalContext::new();
        ctx.target = EvalTarget::Paged;
        ctx.features = self.features;
        ctx.full_error = self.full_error;
        ctx.introspector = input.clone();
        ctx.next_context_id = self.next_id;
        let mut styles = StyleChain::default_chain();
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
            self.occurrences
                .as_mut()
                .expect("physical channel owns an arena")
                .evaluate(frame, &mut ctx, |ctx| operation(ctx, &mut engine))
        };
        self.next_id = self.next_id.max(ctx.next_context_id);
        (result, ctx, sink.into_diagnostics())
    }

    fn retire_outline_root(&mut self, root: usize) {
        self.discard_descendants(root);
        self.nodes[root].active = false;
        if let Some(execution) = self.nodes[root].current.take() {
            self.retired.push(*execution);
        }
    }

    fn outline_execution(
        &mut self,
        frame: &mut ContextOccurrenceFrame,
        content: &Content,
        input: &Arc<TagIntrospector>,
    ) -> OutlineExecution {
        let (result, mut ctx, warnings) =
            self.running_transaction(frame, input, |ctx, engine| {
                ctx.in_context = true;
                realize_outlines(content.clone(), input, ctx, engine)
            });
        ctx.introspector = TagIntrospector::empty();
        OutlineExecution { ctx, input: input.clone(), result, warnings }
    }

    fn prepare_outline_candidate(
        &mut self,
        content: &Content,
        input: &Arc<TagIntrospector>,
    ) -> SourceResult<()> {
        self.occurrences
            .get_or_insert_with(|| ContextOccurrenceArena::new(&self.origin));
        let mut state = std::mem::take(&mut self.running.outline);
        let had_product = state.product.take().is_some();
        let previous_roots = std::mem::take(&mut state.roots);
        state.retired.append(&mut state.executions);
        let mut first_frame = state
            .first_frame
            .take()
            .unwrap_or_else(|| self.occurrences.as_mut().unwrap().frame());
        let first = self.outline_execution(&mut first_frame, content, input);
        state.first_frame = Some(first_frame);
        let first_product = first.result.clone();
        state.executions.push(first);
        let first_product = match first_product {
            Ok(product) => product,
            Err(errors) => {
                for root in previous_roots {
                    self.retire_outline_root(root);
                }
                self.running.outline = state;
                return Err(errors);
            }
        };
        if same_running_capture(content, &first_product) {
            for root in previous_roots {
                self.retire_outline_root(root);
            }
            if had_product {
                state.generation += 1;
            }
            self.running.outline = state;
            return Ok(());
        }

        // This scratch view supplies only the established focal-prefix decision.
        // Published stores are built later by the one physical index facade.
        let observed = introspect_with_introspector(&first_product).outline_prefix_widths;
        let product = if observed != input.outline_prefix_widths {
            let mut focal_input = input.as_ref().clone();
            focal_input.outline_prefix_widths = observed;
            let focal_input = Arc::new(focal_input);
            let mut focal_frame = state
                .focal_frame
                .take()
                .unwrap_or_else(|| self.occurrences.as_mut().unwrap().frame());
            let focal = self.outline_execution(&mut focal_frame, content, &focal_input);
            state.focal_frame = Some(focal_frame);
            let product = focal.result.clone();
            state.executions.push(focal);
            match product {
                Ok(product) => product,
                Err(errors) => {
                    for root in previous_roots {
                        self.retire_outline_root(root);
                    }
                    self.running.outline = state;
                    return Err(errors);
                }
            }
        } else {
            first_product
        };
        state.generation += 1;
        state.product = Some(product.clone());
        self.running.outline = state;
        let Some(blocks) = ordered_blocks(&product, &StyleChain::default_chain()) else {
            for root in previous_roots {
                self.retire_outline_root(root);
            }
            return terminal(self.source, Vec::new());
        };
        for &root in &previous_roots {
            if !blocks
                .iter()
                .any(|(elem, _)| Arc::ptr_eq(&self.nodes[root].elem, elem))
            {
                self.retire_outline_root(root);
            }
        }
        for (elem, chain) in blocks {
            if let Some(&root) = previous_roots.iter().find(|&&root| {
                self.nodes[root].active && Arc::ptr_eq(&self.nodes[root].elem, &elem)
            }) {
                self.nodes[root].chain = chain;
                self.nodes[root].invalid = true;
                self.running.outline.roots.push(root);
                continue;
            }
            if self
                .nodes
                .iter()
                .any(|node| node.active && Arc::ptr_eq(&node.elem, &elem))
            {
                continue;
            }
            if self.nodes.iter().any(|node| node.active && node.elem.id == elem.id) {
                return terminal(self.source, Vec::new());
            }
            let root = self.add(elem, chain, None);
            self.nodes[root].selected = true;
            self.nodes[root].target = EvalTarget::Paged;
            self.running.outline.roots.push(root);
        }
        for root in self.running.outline.roots.clone() {
            self.realize(root, input);
            if self.impossible {
                return terminal(self.source, self.errors());
            }
        }
        Ok(())
    }

    fn outline_candidate_content(&self, content: &Content) -> Content {
        self.outline_state_content(&self.running, content)
    }

    fn outline_state_content(&self, state: &RunningState, content: &Content) -> Content {
        let Some(product) = &state.outline.product else { return content.clone() };
        let resolved = state
            .outline
            .roots
            .iter()
            .map(|&root| (self.nodes[root].elem.id, self.body(root)))
            .collect();
        substitute_context_blocks(product.clone(), &resolved)
    }

    fn with_outline_replay_engine<T>(
        &self,
        execution: &OutlineExecution,
        operation: impl FnOnce(&EvalContext, &mut Engine<'_>) -> SourceResult<T>,
    ) -> SourceResult<T> {
        // Individual replay reads install their captured styles/rules/guards/file.
        let mut styles = StyleChain::default_chain();
        let mut show_rules: Arc<[ShowRule]> = Arc::from([]);
        let mut guards = Vec::new();
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
            active_guards: &mut guards,
            current_file: self.source.id(),
            sink: &mut local,
        };
        operation(&execution.ctx, &mut engine)
    }

    pub(super) fn outline_reads_valid(
        &self,
        candidate: &TagIntrospector,
    ) -> SourceResult<(bool, bool)> {
        fn executions<'a>(
            state: &'a RunningState,
            output: &mut Vec<&'a OutlineExecution>,
        ) {
            output.extend(state.outline.executions.iter());
            for child in &state.bundle {
                executions(child, output);
            }
        }
        let mut active = Vec::new();
        executions(&self.running, &mut active);
        let mut valid = true;
        let mut closed = true;
        for execution in active {
            closed &= self.with_outline_replay_engine(execution, |ctx, engine| {
                ctx.context_reads_valid_for(&execution.input, engine)
            })?;
            valid &= self.with_outline_replay_engine(execution, |ctx, engine| {
                ctx.context_reads_valid_for(candidate, engine)
            })?;
        }
        Ok((valid && closed, closed))
    }

    pub(super) fn outline_nonconvergence_diagnostics(
        &self,
        history: &[TagIntrospector],
    ) -> SourceResult<Vec<SourceDiagnostic>> {
        fn executions<'a>(
            state: &'a RunningState,
            output: &mut Vec<&'a OutlineExecution>,
        ) {
            output.extend(state.outline.executions.iter());
            for child in &state.bundle {
                executions(child, output);
            }
        }
        let mut active = Vec::new();
        executions(&self.running, &mut active);
        let mut diagnostics = Vec::new();
        for execution in active {
            diagnostics.extend(
                self.with_outline_replay_engine(execution, |ctx, engine| {
                    ctx.context_nonconvergence_diagnostics(history, engine)
                })?,
            );
        }
        Ok(diagnostics)
    }

    fn prepare_running_body(&mut self, content: &Content) -> SourceResult<()> {
        let mut versions = Vec::new();
        for &root in &self.roots {
            let mut order = Vec::new();
            self.order_into(root, &mut order);
            for index in order {
                if let Some(execution) = &self.nodes[index].current {
                    versions.push((index, execution.version));
                }
            }
        }
        for &root in &self.running.outline.roots {
            let mut order = Vec::new();
            self.order_into(root, &mut order);
            for index in order {
                if let Some(execution) = &self.nodes[index].current {
                    versions.push((index, execution.version));
                }
            }
        }
        if self.running.body.is_some()
            && self.running.versions == versions
            && self.running.planned_outline_generation == self.running.outline.generation
        {
            return Ok(());
        }
        self.occurrences
            .get_or_insert_with(|| ContextOccurrenceArena::new(&self.origin));
        let mut frame = self
            .running
            .body_frame
            .take()
            .unwrap_or_else(|| self.occurrences.as_mut().unwrap().frame());
        let (body, warnings) =
            self.running_operation(&mut frame, &TagIntrospector::empty(), |ctx, _| {
                prepare_page_running_body(content, ctx)
            });
        self.running.body_frame = Some(frame);
        // Source-plan preparation has no language effects.
        if !warnings.is_empty() {
            return Err(vec![SourceDiagnostic::error(
                self.source.root().span(),
                "source plan preparation unexpectedly produced diagnostics",
            )]);
        }
        self.running.body = Some(body?);
        self.running.generation += 1;
        self.running.versions = versions;
        self.running.planned_outline_generation = self.running.outline.generation;
        Ok(())
    }

    fn take_running_frames(&mut self, key: PageRunningKey) -> AllocationFrames {
        self.running.allocation_frames.remove(&key).unwrap_or_else(|| {
            let arena = self
                .occurrences
                .as_mut()
                .expect("physical frames share the Session arena");
            AllocationFrames { realize: arena.frame(), bind: arena.frame() }
        })
    }

    fn admit_running_request(
        &mut self,
        request: &PageRunningRequest,
        input: &Arc<TagIntrospector>,
    ) -> SourceResult<()> {
        let mut frames = self.take_running_frames(request.key());
        let (resolution, warnings) =
            self.running_operation(&mut frames.realize, input, |ctx, engine| {
                realize_page_running(request, ctx, engine)
            });
        self.running.allocation_frames.insert(request.key(), frames);
        let resolution = match resolution {
            Ok(resolution) => resolution,
            Err(errors) => {
                self.running.failed_warnings.extend(warnings);
                return Err(errors);
            }
        };
        let Some(blocks) =
            ordered_blocks(resolution.content(), &StyleChain::default_chain())
        else {
            return terminal(self.source, Vec::new());
        };
        if let Some(index) = self
            .running
            .contributions
            .iter()
            .position(|entry| entry.key == request.key())
        {
            let retained = &self.running.contributions[index];
            let reuse_execution = retained.generation == self.running.generation
                && same_running_capture(
                    retained.original.content(),
                    resolution.content(),
                );
            if blocks.len() == retained.roots.len()
                && (!blocks.is_empty() || reuse_execution)
                && blocks
                    .iter()
                    .zip(&retained.roots)
                    .all(|((elem, _), &root)| Arc::ptr_eq(elem, &self.nodes[root].elem))
            {
                let roots = retained.roots.clone();
                if !reuse_execution {
                    self.running.contributions[index].binding_versions = None;
                    for ((_, chain), &root) in blocks.iter().zip(&roots) {
                        self.nodes[root].chain = chain.clone();
                        self.nodes[root].invalid = true;
                    }
                }
                self.running.contributions[index].original = resolution;
                self.running.contributions[index].generation = self.running.generation;
                self.running.contributions[index].warnings = warnings;
                if !reuse_execution {
                    for root in roots {
                        self.realize(root, input);
                    }
                    if self.impossible {
                        return terminal(self.source, self.errors());
                    }
                }
                return Ok(());
            }
            let obsolete = self.running.contributions.remove(index);
            self.retire_running_contribution(obsolete);
        }
        let mut roots = Vec::new();
        for (elem, chain) in blocks {
            if self.nodes.iter().any(|node| node.active && node.elem.id == elem.id) {
                return terminal(self.source, Vec::new());
            }
            let index = self.add(elem, chain, None);
            self.nodes[index].selected = true;
            roots.push(index);
        }
        self.running.contributions.push(Contribution {
            key: request.key(),
            generation: self.running.generation,
            original: resolution.clone(),
            bound: resolution,
            roots: roots.clone(),
            binding_versions: None,
            warnings,
        });
        for root in roots {
            self.realize(root, input);
        }
        if self.impossible {
            return terminal(self.source, self.errors());
        }
        Ok(())
    }

    fn running_binding_versions(&self, roots: &[usize]) -> Option<Vec<(usize, u64)>> {
        let mut versions = Vec::new();
        for &root in roots {
            if !self.nodes[root].active {
                return None;
            }
            let mut order = Vec::new();
            self.order_into(root, &mut order);
            for index in order {
                let node = &self.nodes[index];
                if node.invalid {
                    return None;
                }
                let execution = node.current.as_ref()?;
                if execution.result.is_err() {
                    return None;
                }
                versions.push((index, execution.version));
            }
        }
        Some(versions)
    }

    fn bind_running_contributions(
        &mut self,
        input: &Arc<TagIntrospector>,
    ) -> SourceResult<()> {
        let mut contributions = std::mem::take(&mut self.running.contributions);
        for contribution in &mut contributions {
            if contribution.generation != self.running.generation {
                continue;
            }
            let versions = self.running_binding_versions(&contribution.roots);
            if versions.is_some() && contribution.binding_versions == versions {
                continue;
            }
            contribution.binding_versions = None;
            let resolved = contribution
                .roots
                .iter()
                .map(|&index| (self.nodes[index].elem.id, self.body(index)))
                .collect();
            let expanded = substitute_context_blocks(
                contribution.original.content().clone(),
                &resolved,
            );
            let mut frames = self.take_running_frames(contribution.key);
            let (bound, warnings) =
                self.running_operation(&mut frames.bind, input, |ctx, _| {
                    bind_page_running_content(&contribution.original, expanded, ctx)
                });
            self.running.allocation_frames.insert(contribution.key, frames);
            if !warnings.is_empty() {
                self.running.contributions = contributions;
                return terminal(self.source, Vec::new());
            }
            match bound {
                Ok(bound) => {
                    contribution.bound = bound;
                    contribution.binding_versions = versions;
                }
                Err(errors) => {
                    self.running.contributions = contributions;
                    return Err(errors);
                }
            }
        }
        self.running.contributions = contributions;
        Ok(())
    }

    pub(super) fn running_topology(&self) -> Content {
        fn originals(state: &RunningState, content: &mut Vec<Content>) {
            content.extend(
                state
                    .contributions
                    .iter()
                    .filter(|entry| entry.generation == state.generation)
                    .map(|entry| entry.original.content().clone()),
            );
            for child in &state.bundle {
                originals(child, content);
            }
        }
        let source = self.content();
        let body = if self.bundle_html_targets.is_some() {
            if let Content::Sequence(parts) = &source {
                let mut parts = parts.to_vec();
                if !self.running.bundle.is_empty() {
                    assert_eq!(
                        parts.len(),
                        self.running.bundle.len(),
                        "physical Bundle states retain the validated document slice"
                    );
                    for (part, state) in parts.iter_mut().zip(&self.running.bundle) {
                        *part = self.outline_state_content(state, part);
                    }
                } else if let Some(index) = self.running.document_index {
                    let part = parts.get_mut(index)
                        .expect("selected physical document belongs to the validated Bundle slice");
                    *part = self.outline_candidate_content(part);
                }
                Content::Sequence(parts.into())
            } else {
                // Ordinary malformed-Bundle validation remains the owning boundary.
                source
            }
        } else {
            self.outline_candidate_content(&source)
        };
        let mut content = vec![body];
        originals(&self.running, &mut content);
        Content::sequence(content)
    }

    pub(super) fn running_roots(&self) -> Vec<usize> {
        fn roots(state: &RunningState, output: &mut Vec<usize>) {
            output.extend(state.outline.roots.iter().copied());
            output.extend(
                state
                    .contributions
                    .iter()
                    .filter(|entry| entry.generation == state.generation)
                    .flat_map(|entry| entry.roots.iter().copied()),
            );
            for child in &state.bundle {
                roots(child, output);
            }
        }
        let mut output = Vec::new();
        roots(&self.running, &mut output);
        output
    }

    pub(super) fn running_warnings(&self) -> Vec<SourceDiagnostic> {
        fn warnings(state: &RunningState, output: &mut Vec<SourceDiagnostic>) {
            output.extend(
                state
                    .outline
                    .executions
                    .iter()
                    .flat_map(|execution| execution.warnings.iter().cloned()),
            );
            output.extend(
                state
                    .contributions
                    .iter()
                    .filter(|entry| entry.generation == state.generation)
                    .flat_map(|entry| entry.warnings.iter().cloned()),
            );
            output.extend(state.failed_warnings.iter().cloned());
            for child in &state.bundle {
                warnings(child, output);
            }
        }
        let mut output = Vec::new();
        warnings(&self.running, &mut output);
        output
    }

    fn index_physical_steps(
        &mut self,
        steps: &[PhysicalStep],
        input: &TagIntrospector,
    ) -> SourceResult<(
        TagIntrospector,
        Vec<typst_core::entities::locator::LocatorSeed>,
        Vec<SourceDiagnostic>,
    )> {
        let inputs: Vec<_> = steps
            .iter()
            .map(|step| PageRunningIndexInput {
                body: &step.body,
                target: step.target,
                running: step.running.as_ref().map(|(trace, sealed)| (trace, sealed)),
            })
            .collect();
        let features = self.features;
        let occurrences = self
            .occurrences
            .as_mut()
            .expect("physical index shares the Session arena");
        let mut frame =
            self.running.index_frame.take().unwrap_or_else(|| occurrences.frame());
        let (result, warnings) = super::super::runtime::with_introspection_runtime(
            self.world,
            self.source,
            self.target,
            features,
            |engine, ctx| {
                occurrences.evaluate(&mut frame, ctx, |ctx| {
                    introspect_with_page_running(&inputs, features, ctx, engine)
                })
            },
        );
        self.running.index_frame = Some(frame);
        match result {
            Ok((mut index, starts)) => {
                retain_input_numbering_views(&mut index, input);
                Ok((index, starts, warnings))
            }
            Err(errors) => {
                self.running.failed_warnings.extend(warnings);
                self.running
                    .failed_warnings
                    .extend(steps.iter().flat_map(|step| step.warnings.iter().cloned()));
                Err(errors)
            }
        }
    }

    pub(super) fn physical_candidate(
        &mut self,
        content: &Content,
        input: &Arc<TagIntrospector>,
    ) -> SourceResult<PhysicalCandidate> {
        if let Some(targets) = self.bundle_html_targets.clone() {
            return self.physical_bundle_candidate(content, &targets, input);
        }
        self.prepare_outline_candidate(content, input)?;
        let mut callbacks = LayoutCallbacks::default();
        let discovery =
            self.physical_step(content, input, &mut callbacks, false, None)?;
        let (index, _, _) =
            self.index_physical_steps(std::slice::from_ref(&discovery), input)?;
        let replay_input = Arc::new(index);
        let step =
            self.physical_step(content, &replay_input, &mut callbacks, true, None)?;
        let (index, _, mut warnings) =
            self.index_physical_steps(std::slice::from_ref(&step), input)?;
        warnings.extend(step.warnings);
        Ok(PhysicalCandidate {
            index,
            complete: step.document.is_some(),
            document: step.document,
            warnings,
        })
    }

    fn physical_bundle_candidate(
        &mut self,
        content: &Content,
        targets: &[bool],
        input: &Arc<TagIntrospector>,
    ) -> SourceResult<PhysicalCandidate> {
        let Content::Sequence(parts) = content else {
            return terminal(self.source, Vec::new());
        };
        if parts.len() != targets.len() {
            return terminal(self.source, Vec::new());
        }
        let discovery_ranges = document_numbering_ranges(
            input,
            self.running.numbering_partition.as_deref(),
            targets,
        );
        let mut partition_qualified =
            input.page_store.is_empty() || discovery_ranges.is_some();
        let mut states = std::mem::take(&mut self.running.bundle);
        states.resize_with(parts.len(), RunningState::default);
        let mut callbacks: Vec<_> =
            (0..parts.len()).map(|_| LayoutCallbacks::default()).collect();
        let mut steps = Vec::with_capacity(parts.len());
        for (index, (body, &html)) in parts.iter().zip(targets).enumerate() {
            std::mem::swap(&mut self.running, &mut states[index]);
            self.running.document_index = Some(index);
            let step = if html {
                self.prepare_running_body(body).map(|()| PhysicalStep {
                    body: self.running.body.as_ref().unwrap().clone(),
                    target: EvalTarget::Html,
                    running: None,
                    document: None,
                    warnings: Vec::new(),
                })
            } else {
                self.prepare_outline_candidate(body, input).and_then(|()| {
                    self.physical_step(
                        body,
                        input,
                        &mut callbacks[index],
                        false,
                        discovery_ranges.as_ref().and_then(|ranges| ranges[index]),
                    )
                })
            };
            std::mem::swap(&mut self.running, &mut states[index]);
            match step {
                Ok(step) => steps.push(step),
                Err(errors) => {
                    self.running.bundle = states;
                    return Err(errors);
                }
            }
        }
        self.running.bundle = states;
        let (mut index, mut starts, mut warnings) =
            self.index_physical_steps(&steps, input)?;
        if steps.iter().any(|step| step.target == EvalTarget::Paged) {
            let replay_input = Arc::new(index);
            let replay_counts = document_page_counts(&steps, targets);
            let replay_ranges = document_numbering_ranges(
                &replay_input,
                replay_counts.as_deref(),
                targets,
            );
            partition_qualified &=
                replay_input.page_store.is_empty() || replay_ranges.is_some();
            let mut states = std::mem::take(&mut self.running.bundle);
            for (i, (body, &html)) in parts.iter().zip(targets).enumerate() {
                if html {
                    continue;
                }
                std::mem::swap(&mut self.running, &mut states[i]);
                let replay = self.physical_step(
                    body,
                    &replay_input,
                    &mut callbacks[i],
                    true,
                    replay_ranges.as_ref().and_then(|ranges| ranges[i]),
                );
                std::mem::swap(&mut self.running, &mut states[i]);
                match replay {
                    Ok(step) => steps[i] = step,
                    Err(errors) => {
                        self.running.bundle = states;
                        return Err(errors);
                    }
                }
            }
            self.running.bundle = states;
            let rebuilt = self.index_physical_steps(&steps, input)?;
            index = rebuilt.0;
            starts = rebuilt.1;
            warnings = rebuilt.2;
        }
        let final_counts = document_page_counts(&steps, targets);
        self.running.numbering_partition = final_counts.filter(|counts| {
            document_numbering_ranges(&index, Some(counts), targets).is_some()
        });
        let complete = partition_qualified
            && steps
                .iter()
                .all(|step| step.target == EvalTarget::Html || step.document.is_some());
        for step in &steps {
            warnings.extend(step.warnings.iter().cloned());
        }
        let documents: Vec<_> = steps.into_iter().map(|step| step.document).collect();
        if complete {
            warnings.extend(super::bundle_page_views(
                content,
                targets,
                &mut index,
                input,
                self.world,
                self.source,
                Some((&starts, &documents)),
            )?);
            self.bundle_physical_documents = Some(documents);
        } else {
            self.bundle_physical_documents = None;
        }
        self.bundle_physical_starts = Some(starts);
        Ok(PhysicalCandidate { index, document: None, complete, warnings })
    }

    fn current_running_store(&self) -> SourceResult<SealedPageRunning> {
        SealedPageRunning::new(
            self.running
                .contributions
                .iter()
                .filter(|entry| entry.generation == self.running.generation)
                .map(|entry| entry.bound.clone())
                .collect(),
        )
    }

    fn retire_stale_running(&mut self) {
        let previous = std::mem::take(&mut self.running.contributions);
        for contribution in previous {
            if contribution.generation == self.running.generation {
                self.running.contributions.push(contribution);
            } else {
                self.retire_running_contribution(contribution);
            }
        }
    }

    fn physical_step(
        &mut self,
        content: &Content,
        input: &Arc<TagIntrospector>,
        callbacks: &mut LayoutCallbacks,
        replay: bool,
        numbering_range: Option<(NonZeroUsize, NonZeroUsize)>,
    ) -> SourceResult<PhysicalStep> {
        self.running.failed_warnings.clear();
        let content = self.outline_candidate_content(content);
        self.prepare_running_body(&content)?;
        if !replay {
            for root in self.running_roots() {
                self.realize(root, input);
            }
        }
        self.bind_running_contributions(input)?;
        loop {
            if callbacks.passes == 8 && !callbacks.closed {
                return Err(vec![SourceDiagnostic::error(
                    self.source.root().span(),
                    "layout callbacks did not converge",
                )]);
            }
            let sealed = self.current_running_store()?;
            let body = self.running.body.as_ref().unwrap();
            let projected = numbering_range
                .map(|(first, total)| {
                    body.with_numbering_input(&input.page_store, first, total)
                })
                .transpose()?;
            let body = projected.as_ref().unwrap_or(body);
            let outcome = layout_with_page_running(
                body,
                input.as_ref().clone(),
                self.metrics.clone(),
                ImageSizeImageSizer,
                11.0,
                PageRunningLayoutInputs {
                    math: callbacks.math.as_ref(),
                    par_line: callbacks.par_line.as_ref(),
                    running: Some(&sealed),
                },
            )
            .into_result();
            match &outcome {
                Ok(_) => {
                    if !callbacks.closed {
                        callbacks.passes += 1;
                        callbacks.closed = true;
                    }
                }
                Err(pending) => {
                    if !pending.callbacks.math.is_empty()
                        || !pending.callbacks.par_line.is_empty()
                    {
                        if callbacks.passes == 8 {
                            return Err(vec![SourceDiagnostic::error(
                                self.source.root().span(),
                                "layout callbacks did not converge",
                            )]);
                        }
                        callbacks.passes += 1;
                        callbacks.closed = false;
                    }
                }
            }
            match outcome {
                Ok(complete) => {
                    let body = body.clone();
                    self.retire_stale_running();
                    return Ok(PhysicalStep {
                        body,
                        target: EvalTarget::Paged,
                        running: Some((complete.trace, sealed)),
                        document: Some(complete.document),
                        warnings: callbacks.warnings.clone(),
                    });
                }
                Err(pending) => {
                    let progress = !pending.running.is_empty()
                        || !pending.callbacks.math.is_empty()
                        || !pending.callbacks.par_line.is_empty();
                    if !progress {
                        return terminal(self.source, Vec::new());
                    }
                    if !pending.running.is_empty() {
                        // Actual L1 requests select existing read transactions.
                        // Selection alone never reexecutes a retained producer.
                        for node in &mut self.nodes {
                            if node.active {
                                node.selected = true;
                            }
                        }
                    }
                    if !pending.running.is_empty() {
                        let keys: HashSet<_> =
                            pending.running.iter().map(|request| request.key()).collect();
                        let previous = std::mem::take(&mut self.running.contributions);
                        for contribution in previous {
                            if keys.contains(&contribution.key) {
                                self.running.contributions.push(contribution);
                            } else {
                                self.retire_running_contribution(contribution);
                            }
                        }
                    }
                    for request in &pending.running {
                        self.admit_running_request(request, input)?;
                    }
                    self.bind_running_contributions(input)?;
                    if !pending.callbacks.math.is_empty()
                        || !pending.callbacks.par_line.is_empty()
                    {
                        if !pending.callbacks.math.is_empty() {
                            let (store, diagnostics) =
                                super::super::paged::realize_math_cancel_requests(
                                    self.world,
                                    self.source,
                                    pending.callbacks.math,
                                )?;
                            callbacks.math = Some(store);
                            callbacks.warnings.extend(diagnostics);
                        }
                        if !pending.callbacks.par_line.is_empty() {
                            let (store, diagnostics) =
                                super::super::paged::realize_par_line_requests(
                                    self.world,
                                    self.source,
                                    pending.callbacks.par_line,
                                )?;
                            callbacks.par_line = Some(store);
                            callbacks.warnings.extend(diagnostics);
                        }
                    }
                    if !pending.running.is_empty() {
                        let sealed = self.current_running_store()?;
                        return Ok(PhysicalStep {
                            body: self.running.body.as_ref().unwrap().clone(),
                            target: EvalTarget::Paged,
                            running: Some((pending.trace, sealed)),
                            document: None,
                            warnings: callbacks.warnings.clone(),
                        });
                    }
                }
            }
        }
    }
}
