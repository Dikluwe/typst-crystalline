//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/page_running_requests.md
//! @prompt-hash 5d6f085e
//! @layer L1
//! @updated 2026-10-21

use std::num::NonZeroUsize;
use std::sync::Arc;

use super::{LayoutCallbackRequests, SealedParLineCallbacks};
use crate::compiler::introspect::page_running::{
    PacketId, PlanPhase, SourcePlan, StyleEnvironmentId,
};
use crate::compiler::math::layout::callbacks::SealedMathCallbacks;
use crate::entities::content::Content;
use crate::entities::introspector::Introspector;
use crate::entities::layout_types::{PagedDocument, Rect};
use crate::entities::sealed_positions::SealedPositions;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::style_chain::StyleChain;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PageRunningSlot {
    Background,
    Header,
    Footer,
    Foreground,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PageRunningKey {
    pub page: NonZeroUsize,
    pub slot: PageRunningSlot,
}

#[derive(Debug, Clone)]
pub struct PageRunningBody {
    pub(in crate::compiler) content: Content,
    pub(in crate::compiler) plan: Arc<SourcePlan>,
}

impl PageRunningBody {
    pub fn content(&self) -> &Content {
        &self.content
    }
}

/// A stored source path, not equality or identity of a temporary style chain.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::compiler) struct EnvironmentPath {
    pub(in crate::compiler) base: StyleEnvironmentId,
    pub(in crate::compiler) overlays: Vec<PacketId>,
}

#[derive(Debug, Clone)]
pub(in crate::compiler) struct RequestAuthority {
    pub(in crate::compiler) source: Arc<SourcePlan>,
    pub(in crate::compiler) template: PacketId,
    pub(in crate::compiler) key: PageRunningKey,
    pub(in crate::compiler) environment: EnvironmentPath,
    pub(in crate::compiler) chain: StyleChain,
    pub(in crate::compiler) region: Rect,
    pub(in crate::compiler) span: Span,
}

#[derive(Debug, Clone)]
pub struct PageRunningRequest {
    pub(in crate::compiler) authority: Arc<RequestAuthority>,
    pub(in crate::compiler) reusable: Option<Arc<PageRunningResolution>>,
}

impl PageRunningRequest {
    pub fn key(&self) -> PageRunningKey {
        self.authority.key
    }
    pub fn span(&self) -> Span {
        self.authority.span
    }

    pub(in crate::compiler) fn new(authority: RequestAuthority) -> Self {
        Self { authority: Arc::new(authority), reusable: None }
    }

    pub(in crate::compiler) fn matches(&self, other: &Self) -> bool {
        let a = &self.authority;
        let b = &other.authority;
        Arc::ptr_eq(&a.source, &b.source)
            && a.template == b.template
            && a.key == b.key
            && a.environment == b.environment
            && a.region == b.region
            && a.span == b.span
    }
}

#[derive(Debug, Clone)]
pub struct PageRunningResolution {
    pub(in crate::compiler) request: PageRunningRequest,
    pub(in crate::compiler) body: PageRunningBody,
    owner: Arc<RequestAuthority>,
    contribution: Arc<SourcePlan>,
    unexpanded: PageRunningBody,
    pub(in crate::compiler) realization_diagnostics: Arc<[SourceDiagnostic]>,
}

impl PageRunningResolution {
    pub fn key(&self) -> PageRunningKey {
        self.request.key()
    }
    pub fn content(&self) -> &Content {
        self.body.content()
    }

    pub(in crate::compiler) fn new(
        request: &PageRunningRequest,
        body: PageRunningBody,
    ) -> Self {
        Self {
            request: request.clone(),
            owner: request.authority.clone(),
            contribution: body.plan.clone(),
            unexpanded: body.clone(),
            realization_diagnostics: Arc::from([]),
            body,
        }
    }

    pub(in crate::compiler) fn unbound(&self) -> SourceResult<Self> {
        self.validate_owner()?;
        let mut result = self.clone();
        result.body = self.unexpanded.clone();
        result.contribution = result.body.plan.clone();
        Ok(result)
    }

    pub(in crate::compiler) fn with_bound_body(
        &self,
        body: PageRunningBody,
    ) -> SourceResult<Self> {
        self.validate_owner()?;
        let mut result = Self::new(&self.request, body);
        result.unexpanded = self.unexpanded.clone();
        result.realization_diagnostics = self.realization_diagnostics.clone();
        Ok(result)
    }

    pub(in crate::compiler) fn validate_owner(&self) -> SourceResult<()> {
        if !Arc::ptr_eq(&self.owner, &self.request.authority)
            || !Arc::ptr_eq(&self.contribution, &self.body.plan)
        {
            return Err(vec![SourceDiagnostic::error(
                self.request.span(),
                "invalid page running contribution ownership",
            )]);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::compiler) enum TraceSection {
    Background,
    Header,
    Body,
    Footer,
    Foreground,
}

impl TraceSection {
    fn order(self) -> usize {
        match self {
            Self::Background => 0,
            Self::Header => 1,
            Self::Body => 2,
            Self::Footer => 3,
            Self::Foreground => 4,
        }
    }
}

impl From<PageRunningSlot> for TraceSection {
    fn from(slot: PageRunningSlot) -> Self {
        match slot {
            PageRunningSlot::Background => Self::Background,
            PageRunningSlot::Header => Self::Header,
            PageRunningSlot::Footer => Self::Footer,
            PageRunningSlot::Foreground => Self::Foreground,
        }
    }
}

#[derive(Debug, Clone)]
pub(in crate::compiler) struct Admission {
    pub(in crate::compiler) plan: Arc<SourcePlan>,
    pub(in crate::compiler) packet: PacketId,
    pub(in crate::compiler) phase: PlanPhase,
    pub(in crate::compiler) page: NonZeroUsize,
    pub(in crate::compiler) section: TraceSection,
    causal_order: Vec<usize>,
    deferred_anchor: bool,
}

impl Admission {
    pub(super) fn project_page(&mut self, page: NonZeroUsize) {
        if !self.deferred_anchor {
            self.page = page;
        }
    }
}

#[derive(Debug, Clone)]
pub struct PageRunningTrace {
    pub(in crate::compiler) data: Box<PageRunningTraceData>,
}

#[derive(Debug, Clone)]
pub(in crate::compiler) struct PageRunningTraceData {
    pub(in crate::compiler) source: Arc<SourcePlan>,
    pub(in crate::compiler) admissions: Vec<Admission>,
    pub(in crate::compiler) requests: Vec<PageRunningRequest>,
    pub(in crate::compiler) positions: SealedPositions,
    pub(in crate::compiler) pages: crate::entities::page_store::PageStore,
    pub(in crate::compiler) complete: bool,
    inactive_repeat_bodies: Vec<InactiveRepeatBody>,
}

impl PageRunningTrace {
    pub(in crate::compiler) fn new(source: Arc<SourcePlan>) -> Self {
        Self {
            data: Box::new(PageRunningTraceData {
                source,
                admissions: vec![],
                requests: vec![],
                positions: SealedPositions::default(),
                pages: crate::entities::page_store::PageStore::default(),
                complete: false,
                inactive_repeat_bodies: vec![],
            }),
        }
    }

    pub(in crate::compiler) fn ordered(&self) -> Vec<&Admission> {
        let mut ordered: Vec<_> = self.data.admissions.iter().collect();
        ordered.sort_by(|a, b| {
            (a.page, a.section.order(), &a.causal_order).cmp(&(
                b.page,
                b.section.order(),
                &b.causal_order,
            ))
        });
        ordered
    }
}

pub struct PageRunningRequests {
    pub callbacks: LayoutCallbackRequests,
    pub running: Vec<PageRunningRequest>,
    pub trace: PageRunningTrace,
}

pub struct PageRunningDocument {
    pub document: PagedDocument,
    pub trace: PageRunningTrace,
}

enum PassResult {
    Pending(Box<PageRunningRequests>),
    Complete(Box<PageRunningDocument>),
}

pub struct PageRunningPassOutcome {
    state: PassResult,
}

impl PageRunningPassOutcome {
    pub fn into_result(self) -> Result<PageRunningDocument, PageRunningRequests> {
        match self.state {
            PassResult::Complete(document) => Ok(*document),
            PassResult::Pending(requests) => Err(*requests),
        }
    }

    pub(super) fn pending(requests: PageRunningRequests) -> Self {
        Self { state: PassResult::Pending(Box::new(requests)) }
    }

    pub(super) fn complete(document: PageRunningDocument) -> Self {
        Self { state: PassResult::Complete(Box::new(document)) }
    }
}

#[derive(Debug, Clone)]
pub struct SealedPageRunning {
    resolutions: Vec<PageRunningResolution>,
}

impl SealedPageRunning {
    pub fn new(resolutions: Vec<PageRunningResolution>) -> SourceResult<Self> {
        let mut keys = std::collections::HashSet::new();
        let source = resolutions.first().map(|r| &r.request.authority.source);
        for resolution in &resolutions {
            resolution.validate_owner()?;
            let request = &resolution.request;
            let region = request.authority.region;
            let valid_geometry = [region.x.0, region.y.0, region.w.0, region.h.0]
                .into_iter()
                .all(f64::is_finite)
                && region.w.0 >= 0.0
                && region.h.0 >= 0.0;
            let authority = &request.authority;
            let valid_frontier = authority.source.valid_request_frontier(
                authority.template,
                authority.environment.base,
                &authority.environment.overlays,
            );
            if !valid_geometry
                || !valid_frontier
                || !keys.insert(request.key())
                || source.is_some_and(|s| !Arc::ptr_eq(s, &request.authority.source))
            {
                return Err(vec![SourceDiagnostic::error(
                    request.span(),
                    "invalid or duplicate page running resolution",
                )]);
            }
        }
        Ok(Self { resolutions })
    }

    pub fn resolutions(&self) -> &[PageRunningResolution] {
        &self.resolutions
    }
}

pub struct PageRunningLayoutInputs<'a> {
    pub math: Option<&'a SealedMathCallbacks>,
    pub par_line: Option<&'a SealedParLineCallbacks>,
    pub running: Option<&'a SealedPageRunning>,
}

#[derive(Debug, Clone)]
pub(super) struct CausalAnchor {
    page: NonZeroUsize,
    order: Vec<usize>,
}

#[derive(Debug, Clone)]
pub(in crate::compiler) struct Cursor {
    pub(super) plan: Arc<SourcePlan>,
    pub(super) packet: PacketId,
    pub(super) section: TraceSection,
    pub(super) generated_edges: Vec<crate::compiler::introspect::page_running::PlanEdge>,
    pub(super) anchor: Option<CausalAnchor>,
}

/// Cloned atomically by causal checkpoints, including all consumed flags.
#[derive(Debug, Clone)]
pub(in crate::compiler) struct PlannedPass {
    pub(super) source: Arc<SourcePlan>,
    pub(super) cursor: Option<Cursor>,
    pub(super) trace: PageRunningTrace,
    pub(super) store: Option<SealedPageRunning>,
    pub(super) consumed: Vec<bool>,
    pub(super) invalid: bool,
    pub(super) slots: [Option<(Arc<SourcePlan>, PacketId)>; 4],
    pub(super) environment: Option<EnvironmentPath>,
    pub(super) chain: StyleChain,
    next_causal_order: usize,
}

impl PlannedPass {
    pub(super) fn new(body: &PageRunningBody, store: Option<&SealedPageRunning>) -> Self {
        Self {
            source: body.plan.clone(),
            cursor: None,
            trace: PageRunningTrace::new(body.plan.clone()),
            store: store.cloned(),
            consumed: store.map_or_else(Vec::new, |s| vec![false; s.resolutions.len()]),
            invalid: false,
            slots: [None, None, None, None],
            environment: None,
            chain: StyleChain::default_chain(),
            next_causal_order: 0,
        }
    }

    fn causal_order(&mut self, anchor: Option<&CausalAnchor>) -> Vec<usize> {
        let mut order = anchor.map_or_else(Vec::new, |anchor| anchor.order.clone());
        order.push(self.next_causal_order);
        self.next_causal_order += 1;
        order
    }

    fn admit_cursor(
        &mut self,
        cursor: &Cursor,
        packet: PacketId,
        phase: PlanPhase,
        page: NonZeroUsize,
    ) {
        let causal_order = self.causal_order(cursor.anchor.as_ref());
        self.trace.data.admissions.push(Admission {
            plan: cursor.plan.clone(),
            packet,
            phase,
            page: cursor.anchor.as_ref().map_or(page, |anchor| anchor.page),
            section: cursor.section,
            causal_order,
            deferred_anchor: cursor.anchor.is_some(),
        });
    }

    pub(super) fn admit(&mut self, phase: PlanPhase, page: NonZeroUsize) {
        if let Some(cursor) = self.cursor.clone() {
            if !cursor.generated_edges.is_empty() {
                return;
            }
            self.admit_cursor(&cursor, cursor.packet, phase, page);
        }
    }

    pub(super) fn resolve(
        &mut self,
        mut request: PageRunningRequest,
    ) -> Option<PageRunningBody> {
        if self
            .trace
            .data
            .requests
            .iter()
            .any(|prior| prior.key() == request.key())
        {
            self.invalid = true;
        }
        let found = self.store.as_ref().and_then(|store| {
            store.resolutions.iter().position(|r| r.request.matches(&request))
        });
        let body = if let Some(index) = found {
            if !self.consumed[index] {
                self.consumed[index] = true;
                self.store.as_ref().map(|store| {
                    let resolution = &store.resolutions[index];
                    request.reusable = Some(Arc::new(resolution.clone()));
                    resolution.body.clone()
                })
            } else {
                None
            }
        } else {
            None
        };
        self.trace.data.requests.push(request);
        body
    }

    pub(super) fn pending(&self) -> Vec<PageRunningRequest> {
        let missing = self
            .trace
            .data
            .requests
            .iter()
            .any(|request| request.reusable.is_none());
        if missing || !self.consumed_all() {
            self.trace.data.requests.clone()
        } else {
            vec![]
        }
    }

    pub(super) fn consumed_all(&self) -> bool {
        !self.invalid && self.consumed.iter().all(|consumed| *consumed)
    }
}

impl<'a, M: super::FontMetrics, S: crate::entities::image_sizer::ImageSizer>
    super::Layouter<'a, M, S>
{
    pub(super) fn admit_planned_phase(&mut self, phase: PlanPhase, page: NonZeroUsize) {
        let event = self.page_running_pass.as_mut().and_then(|pass| {
            if !pass.cursor.as_ref()?.generated_edges.is_empty() {
                return None;
            }
            let event = pass.trace.data.admissions.len();
            pass.admit(phase, page);
            Some(event)
        });
        if let Some(event) = event {
            self.record_packet_position(
                event,
                phase == PlanPhase::PostChildren,
                crate::entities::layout_types::Point {
                    x: self.regions.current.cursor_x,
                    y: self.regions.current.cursor_y,
                },
            );
        }
    }

    pub(super) fn layout_planned_root(
        &mut self,
        body: &PageRunningBody,
        section: TraceSection,
    ) {
        let cursor = Cursor {
            plan: body.plan.clone(),
            packet: body.plan.root,
            section,
            generated_edges: vec![],
            anchor: None,
        };
        self.layout_planned_cursor(cursor, body.content(), false);
    }

    pub(super) fn layout_planned_cursor(
        &mut self,
        cursor: Cursor,
        content: &Content,
        body_only: bool,
    ) {
        self.admit_footnote_block_frontier(content);
        let note_atomic = self.begin_footnote_atomic(content);
        let previous = self
            .page_running_pass
            .as_mut()
            .and_then(|pass| pass.cursor.replace(cursor));
        self.begin_flow_suffix_if_pending();
        let generation = self.page_running_generation;
        self.observe_page_environment();
        let page = NonZeroUsize::new(self.pages.len() + 1).unwrap();
        self.admit_planned_phase(PlanPhase::Root, page);
        if body_only {
            self.layout_content_body(content);
        } else {
            self.layout_content_retaining_plan(content);
        }
        let page = NonZeroUsize::new(self.pages.len() + 1).unwrap();
        if generation == self.page_running_generation {
            self.admit_planned_phase(PlanPhase::PostChildren, page);
        }
        self.record_footnote_spacing(content);
        self.end_footnote_atomic(note_atomic);
        if let Some(pass) = &mut self.page_running_pass {
            pass.cursor = previous;
        }
    }

    pub(super) fn layout_planned_child(
        &mut self,
        edge: crate::compiler::introspect::page_running::PlanEdge,
        content: &Content,
    ) {
        self.layout_planned_child_mode(edge, content, false);
    }

    pub(super) fn layout_planned_child_body(
        &mut self,
        edge: crate::compiler::introspect::page_running::PlanEdge,
        content: &Content,
    ) {
        self.layout_planned_child_mode(edge, content, true);
    }

    fn layout_planned_child_mode(
        &mut self,
        edge: crate::compiler::introspect::page_running::PlanEdge,
        content: &Content,
        body_only: bool,
    ) {
        let had_parent = self
            .page_running_pass
            .as_ref()
            .is_some_and(|pass| pass.cursor.is_some());
        if let Some(cursor) = self.planned_child_cursor(edge) {
            self.layout_planned_cursor(cursor, content, body_only);
            return;
        }
        if had_parent {
            if let Some(pass) = &mut self.page_running_pass {
                pass.invalid = true;
            }
        }

        if body_only {
            self.layout_content_body(content);
        } else {
            self.layout_content(content);
        }
    }

    pub(super) fn planned_location(&self) -> Option<crate::entities::location::Location> {
        let cursor = self.page_running_pass.as_ref()?.cursor.as_ref()?;
        if !cursor.generated_edges.is_empty() {
            return None;
        }
        cursor.plan.node(cursor.packet)?.location
    }
}

impl<'a, M: super::FontMetrics, S: crate::entities::image_sizer::ImageSizer>
    super::Layouter<'a, M, S>
{
    pub(super) fn defer_planned_cursor(
        &mut self,
        cursor: Option<Cursor>,
    ) -> Option<Cursor> {
        let mut cursor = cursor?;
        let pass = self.page_running_pass.as_mut()?;
        let page = cursor.anchor.as_ref().map_or_else(
            || NonZeroUsize::new(self.pages.len() + 1).unwrap(),
            |anchor| anchor.page,
        );
        let order = pass.causal_order(cursor.anchor.as_ref());
        cursor.anchor = Some(CausalAnchor { page, order });
        Some(cursor)
    }

    pub(super) fn planned_equation_body_cursor(&mut self) -> Option<Cursor> {
        use crate::compiler::introspect::page_running::{PlanEdge, PlanPhase};
        let cursor = self.planned_child_cursor(PlanEdge::EquationBody)?;
        let mut pending = vec![cursor.packet];
        while let Some(packet) = pending.pop() {
            if cursor.plan.required(packet, PlanPhase::Root)
                || cursor.plan.required(packet, PlanPhase::PostChildren)
            {
                return Some(cursor);
            }
            if let Some(node) = cursor.plan.node(packet) {
                pending.extend(node.children.iter().map(|(_, child)| *child));
            }
        }
        None
    }

    pub(super) fn planned_child_cursor(
        &mut self,
        edge: crate::compiler::introspect::page_running::PlanEdge,
    ) -> Option<Cursor> {
        let mut parent = self.page_running_pass.as_ref()?.cursor.as_ref()?.clone();
        if let Some(expected) = parent.generated_edges.first() {
            if *expected != edge {
                self.layout_errors.push(SourceDiagnostic::error(
                    self.current_source_span,
                    "invalid generated wrapper source edge",
                ));
                if let Some(pass) = &mut self.page_running_pass {
                    pass.invalid = true;
                }
                return None;
            }
            parent.generated_edges.remove(0);
            return Some(parent);
        }
        let Some(packet) = parent.plan.child(parent.packet, edge) else {
            self.layout_errors.push(SourceDiagnostic::error(
                self.current_source_span,
                "missing planned source child",
            ));
            return None;
        };
        Some(Cursor { packet, ..parent })
    }
}

impl PageRunningSlot {
    fn index(self) -> usize {
        match self {
            Self::Background => 0,
            Self::Header => 1,
            Self::Footer => 2,
            Self::Foreground => 3,
        }
    }
}

impl<'a, M: super::FontMetrics, S: crate::entities::image_sizer::ImageSizer>
    super::Layouter<'a, M, S>
{
    pub(super) fn bind_running_slot(
        &mut self,
        slot: PageRunningSlot,
        edge: crate::compiler::introspect::page_running::PlanEdge,
        selected: bool,
    ) {
        let Some(pass) = &mut self.page_running_pass else {
            return;
        };
        pass.slots[slot.index()] = if selected {
            pass.cursor.as_ref().and_then(|cursor| {
                cursor
                    .plan
                    .child(cursor.packet, edge)
                    .map(|packet| (cursor.plan.clone(), packet))
            })
        } else {
            None
        };
    }

    pub(super) fn running_content(
        &mut self,
        slot: PageRunningSlot,
        region: Rect,
    ) -> Option<PageRunningBody> {
        let page = NonZeroUsize::new(self.pages.len() + 1).unwrap();
        let pass = self.page_running_pass.as_mut()?;
        let Some((source, template)) = pass.slots[slot.index()].clone() else {
            pass.invalid = true;
            self.layout_errors.push(SourceDiagnostic::error(
                self.current_source_span,
                "missing physical page template binding",
            ));
            return None;
        };
        let span = source.node(template)?.span;
        let environment = pass.environment.clone().unwrap_or(EnvironmentPath {
            base: source.node(source.root)?.environment,
            overlays: vec![],
        });
        let request = PageRunningRequest::new(RequestAuthority {
            source,
            template,
            key: PageRunningKey { page, slot },
            environment,
            chain: pass.chain.clone(),
            region,
            span,
        });
        pass.resolve(request)
    }

    pub(super) fn observe_page_environment(&mut self) {
        if self.is_sub_frame {
            return;
        }
        let Some(pass) = &mut self.page_running_pass else {
            return;
        };
        let Some(cursor) = &pass.cursor else {
            return;
        };
        if cursor.section != TraceSection::Body
            || !cursor.generated_edges.is_empty()
            || !cursor.plan.page_frontier(cursor.packet)
        {
            return;
        }
        let Some(node) = cursor.plan.node(cursor.packet) else {
            return;
        };
        pass.environment =
            Some(EnvironmentPath { base: node.environment, overlays: vec![] });
        if let Some(environment) = cursor.plan.environment(node.environment) {
            pass.chain = environment.chain.clone();
        }
    }
}

impl<'a, M: super::FontMetrics, S: crate::entities::image_sizer::ImageSizer>
    super::Layouter<'a, M, S>
{
    pub(super) fn layout_running_sub_frame(
        &mut self,
        body: &PageRunningBody,
        slot: PageRunningSlot,
        region: super::sub_frame::SubLayoutRegion,
    ) -> Vec<crate::entities::layout_types::FrameItem> {
        let previous_chain = self.chain.clone();
        let previous_style = self.style.clone();
        if let Some(pass) = &self.page_running_pass {
            self.chain = pass.chain.clone();
            self.style = crate::entities::layout_types::TextStyle::from(&self.chain);
        }
        let cursor = Cursor {
            plan: body.plan.clone(),
            packet: body.plan.root,
            section: slot.into(),
            generated_edges: vec![],
            anchor: None,
        };
        let (_, items, _, _, _) =
            self.layout_sub_frame_planned(body.content(), region, false, Some(cursor));
        self.chain = previous_chain;
        self.style = previous_style;
        items
    }
}

impl PlannedPass {
    pub(super) fn coverage_complete(&self) -> bool {
        let mut plans = vec![self.source.clone()];
        for event in &self.trace.data.admissions {
            if !plans.iter().any(|plan| Arc::ptr_eq(plan, &event.plan)) {
                plans.push(event.plan.clone());
            }
        }
        plans.iter().all(|plan| {
            plan.nodes.iter().enumerate().all(|(index, _)| {
                let packet = PacketId(index);
                [PlanPhase::Root, PlanPhase::PostChildren].into_iter().all(|phase| {
                    !plan.required(packet, phase)
                        || self.trace.data.admissions.iter().any(|event| {
                            Arc::ptr_eq(&event.plan, plan)
                                && event.packet == packet
                                && event.phase == phase
                        })
                })
            })
        })
    }
}

impl<'a, M: super::FontMetrics, S: crate::entities::image_sizer::ImageSizer>
    super::Layouter<'a, M, S>
{
    pub(super) fn register_planned_original(&mut self) -> bool {
        self.register_planned_child(
            crate::compiler::introspect::page_running::PlanEdge::RealizedOriginal,
        )
    }

    pub(super) fn register_planned_child(
        &mut self,
        edge: crate::compiler::introspect::page_running::PlanEdge,
    ) -> bool {
        let Some(cursor) = self.planned_child_cursor(edge) else {
            return false;
        };
        let page = NonZeroUsize::new(self.pages.len() + 1).unwrap();
        let point = crate::entities::layout_types::Point {
            x: self.regions.current.cursor_x,
            y: self.regions.current.cursor_y,
        };
        fn visit<M: super::FontMetrics, S: crate::entities::image_sizer::ImageSizer>(
            layouter: &mut super::Layouter<'_, M, S>,
            cursor: &Cursor,
            packet: PacketId,
            page: NonZeroUsize,
            point: crate::entities::layout_types::Point,
        ) {
            use crate::compiler::introspect::page_running::PlanRole;
            let Some(node) = cursor.plan.node(packet) else {
                return;
            };
            if matches!(
                node.role,
                PlanRole::OperationalTemplate | PlanRole::PrunedOriginal
            ) {
                return;
            }
            let reference_selected = |content: &Content, original: bool| match content {
                Content::Ref(reference) => Some(
                    !original
                        && layouter
                            .introspector
                            .bib_entry_for_key(reference.name.as_str())
                            .is_some(),
                ),
                Content::Source(..)
                | Content::ShowRecipe(..)
                | Content::Realized(..)
                | Content::Located(..)
                | Content::Empty
                | Content::Text(..)
                | Content::Space
                | Content::SpaceFromNewline
                | Content::Parbreak
                | Content::Sequence(..)
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
                | Content::SetPage(..)
                | Content::PageRun(..)
                | Content::Align(..)
                | Content::Place(..)
                | Content::Flush(..)
                | Content::Styled(..)
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
                | Content::Dynamic(..) => None,
            };
            if let Some(owner) = node.conditional_ref {
                let selected = cursor
                    .plan
                    .node(owner)
                    .and_then(|owner| reference_selected(&owner.content, owner.original))
                    .unwrap_or(false);
                if !selected {
                    return;
                }
            }
            let citation_selected =
                reference_selected(&node.content, node.original).unwrap_or(true);
            for location in node
                .location
                .iter()
                .chain(node.auxiliary_locations.iter().filter(|_| citation_selected))
                .copied()
            {
                layouter.locator.observe(location);
                layouter.record_location_position(
                    location,
                    crate::entities::position::Position { page, point },
                    true,
                );
            }
            if let Some(pass) = &mut layouter.page_running_pass {
                pass.admit_cursor(cursor, packet, PlanPhase::Root, page);
            }
            for (_, child) in &node.children {
                visit(layouter, cursor, *child, page, point);
            }
            if let Some(pass) = &mut layouter.page_running_pass {
                pass.admit_cursor(cursor, packet, PlanPhase::PostChildren, page);
            }
        }
        visit(self, &cursor, cursor.packet, page, point);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::introspect::page_running::{
        EnvironmentOrigin, PlanEnvironment, PlanNode, PlanRole,
    };
    use crate::entities::layout_types::Pt;
    use crate::entities::locator::Locator;

    fn body() -> PageRunningBody {
        let content =
            Content::SetPage(Arc::new(crate::entities::content::SetPageDelta {
                header: Some(crate::entities::page_running::PageMarginal::Content(
                    Arc::new(Content::Empty),
                )),
                ..Default::default()
            }));
        let chain = StyleChain::default_chain();
        let template = PlanNode {
            content: Content::Empty,
            children: vec![],
            location: None,
            source_parent: Some(PacketId(0)),
            environment: StyleEnvironmentId(0),
            lang: None,
            label: None,
            span: Span::detached(),
            role: PlanRole::OperationalTemplate,
            auxiliary_locations: vec![],
            conditional_ref: None,
            original: false,
        };
        let root = PlanNode {
            content: content.clone(),
            children: vec![(
                crate::compiler::introspect::page_running::PlanEdge::SetPageHeader,
                PacketId(1),
            )],
            location: None,
            source_parent: None,
            environment: StyleEnvironmentId(0),
            lang: None,
            label: None,
            span: Span::detached(),
            role: PlanRole::Canonical,
            auxiliary_locations: vec![],
            conditional_ref: None,
            original: false,
        };
        let plan = Arc::new(SourcePlan {
            root: PacketId(0),
            allocation_start: Locator::new().checkpoint(),
            allocation_owner: None,
            document_boundary: None,
            environments: vec![PlanEnvironment {
                parent: None,
                origin: EnvironmentOrigin::Root,
                chain,
            }],
            nodes: vec![root, template],
        });
        PageRunningBody { content, plan }
    }

    fn request(body: &PageRunningBody) -> PageRunningRequest {
        PageRunningRequest::new(RequestAuthority {
            source: body.plan.clone(),
            template: PacketId(1),
            key: PageRunningKey {
                page: NonZeroUsize::new(1).unwrap(),
                slot: PageRunningSlot::Header,
            },
            environment: EnvironmentPath {
                base: StyleEnvironmentId(0),
                overlays: vec![],
            },
            chain: StyleChain::default_chain(),
            region: Rect { x: Pt(0.0), y: Pt(0.0), w: Pt(100.0), h: Pt(20.0) },
            span: Span::detached(),
        })
    }

    #[test]
    fn physical_page_request_requires_exact_source_environment_and_geometry() {
        let source = body();
        let original = request(&source);
        assert!(original.matches(&original.clone()));
        assert!(!original.matches(&request(&body())));
        let mut changed = (*original.authority).clone();
        changed.environment.overlays.push(PacketId(0));
        assert!(!original.matches(&PageRunningRequest::new(changed)));
        let mut changed = (*original.authority).clone();
        changed.region.h = Pt(21.0);
        assert!(!original.matches(&PageRunningRequest::new(changed)));
        let mut changed = (*original.authority).clone();
        changed.key.slot = PageRunningSlot::Footer;
        assert!(!original.matches(&PageRunningRequest::new(changed)));
    }

    #[test]
    fn physical_page_seal_rejects_duplicates_nonfinite_and_replaced_contribution() {
        let source = body();
        let original = request(&source);
        let resolved = PageRunningResolution::new(&original, body());
        assert!(SealedPageRunning::new(vec![resolved.clone()]).is_ok());
        assert!(SealedPageRunning::new(vec![resolved.clone(), resolved.clone()]).is_err());
        let mut invalid = resolved.clone();
        invalid.body = body();
        assert!(SealedPageRunning::new(vec![invalid]).is_err());
        let mut invalid = (*original.authority).clone();
        invalid.region.w = Pt(f64::INFINITY);
        assert!(SealedPageRunning::new(vec![PageRunningResolution::new(
            &PageRunningRequest::new(invalid),
            body()
        )])
        .is_err());
        let mut other = request(&body());
        Arc::make_mut(&mut other.authority).key.slot = PageRunningSlot::Footer;
        assert!(SealedPageRunning::new(vec![
            resolved,
            PageRunningResolution::new(&other, body())
        ])
        .is_err());
    }

    #[test]
    fn physical_page_deferred_order_survives_late_geometry_and_nested_reservation() {
        let source = body();
        let mut pass = PlannedPass::new(&source, None);
        let p1 = NonZeroUsize::new(1).unwrap();
        let p2 = NonZeroUsize::new(2).unwrap();
        let ordinary = Cursor {
            plan: source.plan.clone(),
            packet: PacketId(0),
            section: TraceSection::Body,
            generated_edges: vec![],
            anchor: None,
        };
        pass.admit_cursor(&ordinary, PacketId(0), PlanPhase::Root, p1);
        let checkpoint = pass.clone();
        let order = pass.causal_order(None);
        let deferred = Cursor {
            packet: PacketId(1),
            anchor: Some(CausalAnchor { page: p1, order: order.clone() }),
            ..ordinary.clone()
        };
        let mut restored = checkpoint;
        assert_eq!(restored.causal_order(None), order);
        pass.admit_cursor(&ordinary, PacketId(0), PlanPhase::PostChildren, p1);
        pass.admit_cursor(&deferred, PacketId(1), PlanPhase::Root, p2);
        let nested_order = pass.causal_order(deferred.anchor.as_ref());
        let nested = Cursor {
            anchor: Some(CausalAnchor { page: p1, order: nested_order }),
            ..deferred.clone()
        };
        pass.admit_cursor(&deferred, PacketId(1), PlanPhase::PostChildren, p2);
        pass.admit_cursor(&nested, PacketId(0), PlanPhase::Root, p2);
        let header = Cursor { section: TraceSection::Header, ..ordinary.clone() };
        pass.admit_cursor(&header, PacketId(0), PlanPhase::Root, p2);
        pass.trace.data.admissions[2].project_page(p2);
        assert_eq!(pass.trace.data.admissions[2].page, p1);
        let ordered = pass.trace.ordered();
        assert_eq!(
            ordered
                .iter()
                .map(|e| (e.packet, e.phase, e.page, e.section))
                .collect::<Vec<_>>(),
            vec![
                (PacketId(0), PlanPhase::Root, p1, TraceSection::Body),
                (PacketId(1), PlanPhase::Root, p1, TraceSection::Body),
                (PacketId(0), PlanPhase::Root, p1, TraceSection::Body),
                (PacketId(1), PlanPhase::PostChildren, p1, TraceSection::Body),
                (PacketId(0), PlanPhase::PostChildren, p1, TraceSection::Body),
                (PacketId(0), PlanPhase::Root, p2, TraceSection::Header),
            ]
        );
    }

    #[test]
    fn physical_page_rollback_restores_consumption_and_reuse_is_not_complete() {
        let source = body();
        let original = request(&source);
        let store =
            SealedPageRunning::new(vec![PageRunningResolution::new(&original, body())])
                .unwrap();
        let mut pass = PlannedPass::new(&source, Some(&store));
        let checkpoint = pass.clone();
        assert!(pass.resolve(original.clone()).is_some());
        assert!(pass.consumed_all());
        pass = checkpoint;
        assert!(pass.resolve(original.clone()).is_some());
        assert!(pass.resolve(original).is_none());
        assert!(!pass.consumed_all());
        assert!(!pass.pending().is_empty());
    }
}

#[derive(Debug)]
pub(super) struct PacketScope {
    previous: Option<Cursor>,
    generation: u64,
}

impl<'a, M: super::FontMetrics, S: crate::entities::image_sizer::ImageSizer>
    super::Layouter<'a, M, S>
{
    pub(super) fn begin_planned_packet(&mut self, cursor: Cursor) -> PacketScope {
        let previous = self
            .page_running_pass
            .as_mut()
            .and_then(|pass| pass.cursor.replace(cursor));
        self.begin_flow_suffix_if_pending();
        self.observe_page_environment();
        self.admit_planned_phase(
            PlanPhase::Root,
            NonZeroUsize::new(self.pages.len() + 1).unwrap(),
        );
        PacketScope { previous, generation: self.page_running_generation }
    }

    pub(super) fn end_planned_packet(&mut self, scope: PacketScope) {
        if self.page_running_generation == scope.generation {
            self.admit_planned_phase(
                PlanPhase::PostChildren,
                NonZeroUsize::new(self.pages.len() + 1).unwrap(),
            );
        }
        if let Some(pass) = &mut self.page_running_pass {
            pass.cursor = scope.previous;
        }
    }
}

impl Cursor {
    pub(super) fn through_generated(
        mut self,
        edges: &[crate::compiler::introspect::page_running::PlanEdge],
    ) -> Self {
        let mut path = edges.to_vec();
        path.append(&mut self.generated_edges);
        self.generated_edges = path;
        self
    }
}

impl<'a, M: super::FontMetrics, S: crate::entities::image_sizer::ImageSizer>
    super::Layouter<'a, M, S>
{
    pub(super) fn generated_source_pending(&self) -> bool {
        self.page_running_pass
            .as_ref()
            .and_then(|pass| pass.cursor.as_ref())
            .is_some_and(|cursor| !cursor.generated_edges.is_empty())
    }
}

/// Mechanical carrier recognition; never a language-content predicate.
pub(in crate::compiler) fn is_position_marker(
    item: &crate::entities::layout_types::FrameItem,
) -> bool {
    super::position_marker::is_marker(item)
}

#[derive(Debug, Clone)]
struct InactiveRepeatBody {
    plan: Arc<SourcePlan>,
    repeat: PacketId,
    body: PacketId,
    root_event: usize,
    post_event: usize,
}

impl PageRunningTrace {
    fn unique_phase_index(
        &self,
        plan: &Arc<SourcePlan>,
        packet: PacketId,
        phase: PlanPhase,
    ) -> Option<usize> {
        let mut found = self.data.admissions.iter().enumerate().filter(|(_, event)| {
            Arc::ptr_eq(&event.plan, plan)
                && event.packet == packet
                && event.phase == phase
        });
        let (index, _) = found.next()?;
        found.next().is_none().then_some(index)
    }

    fn completed_repeat_body(
        &self,
        plan: &Arc<SourcePlan>,
        repeat: PacketId,
        body: PacketId,
    ) -> Option<InactiveRepeatBody> {
        use crate::compiler::introspect::page_running::PlanEdge;
        if !matches!(&plan.node(repeat)?.content, Content::Repeat(_))
            || plan.child(repeat, PlanEdge::RepeatBody) != Some(body)
            || body == repeat
        {
            return None;
        }
        let parent_root = self.unique_phase_index(plan, repeat, PlanPhase::Root)?;
        let root_event = self.unique_phase_index(plan, body, PlanPhase::Root)?;
        let post_event = self.unique_phase_index(plan, body, PlanPhase::PostChildren)?;
        if parent_root >= root_event || root_event >= post_event {
            return None;
        }
        Some(InactiveRepeatBody {
            plan: plan.clone(),
            repeat,
            body,
            root_event,
            post_event,
        })
    }

    pub(in crate::compiler) fn repeat_activity_valid(&self) -> bool {
        self.data
            .inactive_repeat_bodies
            .iter()
            .enumerate()
            .all(|(index, proof)| {
                let Some(completed) =
                    self.completed_repeat_body(&proof.plan, proof.repeat, proof.body)
                else {
                    return false;
                };
                let Some(parent_post) = self.unique_phase_index(
                    &proof.plan,
                    proof.repeat,
                    PlanPhase::PostChildren,
                ) else {
                    return false;
                };
                completed.root_event == proof.root_event
                    && completed.post_event == proof.post_event
                    && proof.post_event < parent_post
                    && !self.data.inactive_repeat_bodies[..index].iter().any(|previous| {
                        Arc::ptr_eq(&previous.plan, &proof.plan)
                            && previous.repeat == proof.repeat
                            && previous.body == proof.body
                    })
            })
    }

    pub(in crate::compiler) fn source_contribution_active(
        &self,
        plan: &Arc<SourcePlan>,
        packet: PacketId,
    ) -> bool {
        fn contains(plan: &SourcePlan, root: PacketId, target: PacketId) -> bool {
            let mut pending = vec![root];
            let mut seen = std::collections::HashSet::new();
            while let Some(current) = pending.pop() {
                if !seen.insert(current) {
                    continue;
                }
                if current == target {
                    return true;
                }
                if let Some(node) = plan.node(current) {
                    pending.extend(node.children.iter().map(|(_, child)| *child));
                }
            }
            false
        }
        !self.data.inactive_repeat_bodies.iter().any(|proof| {
            Arc::ptr_eq(&proof.plan, plan) && contains(plan, proof.body, packet)
        })
    }
}

impl<'a, M: super::FontMetrics, S: crate::entities::image_sizer::ImageSizer>
    super::Layouter<'a, M, S>
{
    pub(super) fn discard_empty_repeat_body(&mut self, child: Option<&Cursor>) {
        let Some(child) = child else { return };
        let Some(pass) = &mut self.page_running_pass else { return };
        let Some(parent) = &pass.cursor else { return };
        if !parent.generated_edges.is_empty() || !child.generated_edges.is_empty() {
            return;
        }
        if !Arc::ptr_eq(&parent.plan, &child.plan) || parent.section != child.section {
            pass.invalid = true;
            return;
        }
        let Some(proof) =
            pass.trace
                .completed_repeat_body(&parent.plan, parent.packet, child.packet)
        else {
            pass.invalid = true;
            return;
        };
        if pass.trace.data.inactive_repeat_bodies.iter().any(|previous| {
            Arc::ptr_eq(&previous.plan, &proof.plan)
                && previous.repeat == proof.repeat
                && previous.body == proof.body
        }) {
            pass.invalid = true;
            return;
        }
        pass.trace.data.inactive_repeat_bodies.push(proof);
    }
}

#[cfg(test)]
mod repeat_activity_tests {
    use super::*;
    use crate::compiler::eval::EvalContext;
    use crate::compiler::introspect::page_running::{
        prepare_page_running_body, PlanEdge,
    };
    use crate::compiler::realize::context_occurrences::ContextOccurrenceArena;
    use crate::entities::value::Value;

    fn actual_source_trace() -> (PageRunningTrace, PacketId, PacketId, PacketId) {
        let origin = Content::sequence(vec![
            Content::repeat(
                Content::sequence(vec![
                    Content::text("X"),
                    Content::metadata(Value::Str("inside".into())),
                ]),
                None,
                false,
            ),
            Content::metadata(Value::Str("outside".into())),
        ]);
        let mut arena = ContextOccurrenceArena::new(&origin);
        let mut frame = arena.frame();
        let mut ctx = EvalContext::new();
        let body = arena
            .evaluate(&mut frame, &mut ctx, |ctx| prepare_page_running_body(&origin, ctx))
            .unwrap();
        let repeat = body
            .plan
            .nodes
            .iter()
            .position(|node| matches!(&node.content, Content::Repeat(_)))
            .map(PacketId)
            .unwrap();
        let child = body.plan.child(repeat, PlanEdge::RepeatBody).unwrap();
        let outside = body.plan.nodes.iter().position(|node|
            matches!(&node.content, Content::Metadata(value)
                if matches!(value.value.as_ref(), Value::Str(text) if text.as_str()=="outside")))
            .map(PacketId).unwrap();
        let mut pass = PlannedPass::new(&body, None);
        let cursor = Cursor {
            plan: body.plan.clone(),
            packet: body.plan.root,
            section: TraceSection::Body,
            generated_edges: vec![],
            anchor: None,
        };
        fn visit(pass: &mut PlannedPass, cursor: &Cursor, packet: PacketId) {
            let page = NonZeroUsize::new(1).unwrap();
            pass.admit_cursor(cursor, packet, PlanPhase::Root, page);
            for (_, child) in &cursor.plan.node(packet).unwrap().children {
                visit(pass, cursor, *child);
            }
            pass.admit_cursor(cursor, packet, PlanPhase::PostChildren, page);
        }
        visit(&mut pass, &cursor, body.plan.root);
        (pass.trace, repeat, child, outside)
    }

    #[test]
    fn zero_copy_proof_keeps_all_raw_events_and_only_excludes_its_typed_body() {
        let (mut trace, repeat, child, outside) = actual_source_trace();
        let plan = trace.data.source.clone();
        let before = trace.ordered().len();
        let proof = trace.completed_repeat_body(&plan, repeat, child).unwrap();
        trace.data.inactive_repeat_bodies.push(proof);
        assert!(trace.repeat_activity_valid());
        assert_eq!(trace.ordered().len(), before);
        assert_eq!(trace.data.admissions.len(), before);
        assert!(trace.source_contribution_active(&plan, repeat));
        assert!(trace.source_contribution_active(&plan, outside));
        assert!(!trace.source_contribution_active(&plan, child));
    }

    #[test]
    fn zero_copy_proof_rejects_wrong_edge_foreign_stale_missing_and_duplicate_claims() {
        let (trace, repeat, child, outside) = actual_source_trace();
        let proof = trace
            .completed_repeat_body(&trace.data.source, repeat, child)
            .unwrap();
        for mode in [
            "wrong-parent",
            "wrong-child",
            "foreign",
            "stale",
            "missing-post",
            "duplicate-phase",
            "duplicate-proof",
        ] {
            let mut invalid = trace.clone();
            let mut forged = proof.clone();
            match mode {
                "wrong-parent" => forged.repeat = child,
                "wrong-child" => forged.body = outside,
                "foreign" => forged.plan = Arc::new((*forged.plan).clone()),
                "stale" => forged.root_event = forged.post_event,
                "missing-post" => {
                    invalid.data.admissions.remove(forged.post_event);
                }
                "duplicate-phase" => {
                    invalid
                        .data
                        .admissions
                        .push(invalid.data.admissions[forged.root_event].clone());
                }
                "duplicate-proof" => {
                    invalid.data.inactive_repeat_bodies.push(forged.clone())
                }
                _ => unreachable!(),
            }
            invalid.data.inactive_repeat_bodies.push(forged);
            assert!(!invalid.repeat_activity_valid(), "forged {mode} must reject");
        }
    }
}
