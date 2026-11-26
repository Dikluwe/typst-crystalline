//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/introspect/page_running.md
//! @prompt-hash b46115d6
//! @layer L1
//! @updated 2026-10-21

use std::sync::Arc;

use rustc_hash::FxHashSet;

use crate::compiler::eval::{EvalContext, EvalTarget};
use crate::compiler::layout::{PageRunningBody, PageRunningTrace, SealedPageRunning};
use crate::entities::{
    compiler_features::Features, content::Content, engine::Engine,
    introspector::TagIntrospector, label::Label, lang::Lang, location::Location,
    locator::LocatorSeed, source_result::SourceResult, span::Span,
    style_chain::StyleChain,
};

/// One actual document's source plan and, for Paged, its physical transcript.
pub struct PageRunningIndexInput<'a> {
    pub body: &'a PageRunningBody,
    pub target: EvalTarget,
    pub running: Option<(&'a PageRunningTrace, &'a SealedPageRunning)>,
}

/// A construction address in one immutable source plan, not a language identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(in crate::compiler) struct PacketId(pub(in crate::compiler) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(in crate::compiler) struct StyleEnvironmentId(pub(in crate::compiler) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(in crate::compiler) enum PlanPhase {
    Root,
    PostChildren,
}

/// Source edges are chosen by feature owners, never inferred from a child value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(in crate::compiler) enum PlanEdge {
    SourceBody,
    LocatedBody,
    StyledBody,
    ShowRecipeBody,
    RealizedOriginal,
    RealizedVisual,
    SequencePart(usize),
    ParagraphBody,
    HeadingBody,
    TitleBody,
    StrongBody,
    EmphBody,
    SmallCapsBody,
    LabelBody,
    ListItemBody,
    EnumItemBody,
    EquationBody,
    FigureBody,
    FigureCaption,
    FigureSelectedSupplement,
    FigureCaptionBody,
    FigureCaptionSeparator,
    PreparedReferenceSupplement,
    CitationSupplement,
    GridCell(usize),
    GridHeader,
    GridFooter,
    GridCellBody,
    GridHeaderBody,
    GridFooterBody,
    TableResolvedCell(usize),
    TableRawChild(usize),
    TableHeader,
    TableFooter,
    TableCaption,
    CellSelectedVisual,
    TableHeaderBody,
    TableFooterBody,
    PageRunBody,
    PageRunBackground,
    PageRunHeader,
    PageRunFooter,
    PageRunForeground,
    SetPageBackground,
    SetPageHeader,
    SetPageFooter,
    SetPageForeground,
    HtmlFrameBody,
    TermsItem(usize),
    TermItemTerm,
    TermItemDescription,
    QuoteBody,
    QuoteAttribution,
    StackChild(usize),
    AlignBody,
    PlaceBody,
    TransformBody,
    PdfArtifactBody,
    UnderlineBody,
    StrikeBody,
    OverlineBody,
    PadBody,
    HideBody,
    BoxedBody,
    BlockBody,
    RepeatBody,
    ColumnsBody,
    BibliographyTitle,
    FootnoteBody,
    FootnoteEntryNote,
    FootnoteEntrySeparator,
    OutlineRealizedBody,
    OutlineEntryBody,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::compiler) enum PlanRole {
    Canonical,
    OriginalOnly,
    VisualOnly,
    OperationalTemplate,
    PreparedReference,
    PrunedOriginal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::compiler) enum EnvironmentOrigin {
    Root,
    Styled(PacketId),
    CapturedRecipe(PacketId),
}

#[derive(Debug, Clone)]
pub(in crate::compiler) struct PlanEnvironment {
    pub(in crate::compiler) parent: Option<StyleEnvironmentId>,
    pub(in crate::compiler) origin: EnvironmentOrigin,
    pub(in crate::compiler) chain: StyleChain,
}

#[derive(Debug, Clone)]
pub(in crate::compiler) struct PlanNode {
    pub(in crate::compiler) content: Content,
    pub(in crate::compiler) children: Vec<(PlanEdge, PacketId)>,
    pub(in crate::compiler) location: Option<Location>,
    pub(in crate::compiler) source_parent: Option<PacketId>,
    pub(in crate::compiler) environment: StyleEnvironmentId,
    pub(in crate::compiler) lang: Option<Lang>,
    pub(in crate::compiler) label: Option<Label>,
    pub(in crate::compiler) span: Span,
    pub(in crate::compiler) role: PlanRole,
    pub(in crate::compiler) auxiliary_locations: Vec<Location>,
    pub(in crate::compiler) conditional_ref: Option<PacketId>,
    pub(in crate::compiler) original: bool,
    pub(in crate::compiler) math: bool,
}

#[derive(Debug, Clone)]
pub(in crate::compiler) struct SourcePlan {
    pub(in crate::compiler) root: PacketId,
    pub(in crate::compiler) nodes: Vec<PlanNode>,
    pub(in crate::compiler) environments: Vec<PlanEnvironment>,
    pub(in crate::compiler) allocation_start: LocatorSeed,
    pub(in crate::compiler) allocation_owner: Option<Arc<()>>,
    pub(in crate::compiler) document_boundary: Option<(LocatorSeed, Location)>,
    // Derived only from this immutable plan; never source identity or authority.
    page_frontiers: Vec<bool>,
    child_ranges: Vec<(usize, usize)>,
    child_lookup: Vec<(PlanEdge, PacketId)>,
}

impl SourcePlan {
    pub(in crate::compiler) fn node(&self, id: PacketId) -> Option<&PlanNode> {
        self.nodes.get(id.0)
    }

    pub(in crate::compiler) fn child(
        &self,
        parent: PacketId,
        edge: PlanEdge,
    ) -> Option<PacketId> {
        self.child_query(parent, edge, || {})
    }

    fn child_query(
        &self,
        parent: PacketId,
        edge: PlanEdge,
        mut visit: impl FnMut(),
    ) -> Option<PacketId> {
        let &(start, end) = self.child_ranges.get(parent.0)?;
        let children = &self.child_lookup[start..end];
        let index = children.partition_point(|(found, _)| {
            visit();
            *found < edge
        });
        children.get(index).and_then(|(found, child)| {
            visit();
            (*found == edge).then_some(*child)
        })
    }

    pub(in crate::compiler) fn page_frontier(&self, packet: PacketId) -> bool {
        self.frontier_query(packet, || {})
    }

    fn frontier_query(&self, packet: PacketId, mut visit: impl FnMut()) -> bool {
        visit();
        self.page_frontiers.get(packet.0).copied().unwrap_or(false)
    }

    fn initialize_queries(&mut self) {
        self.child_ranges = Vec::with_capacity(self.nodes.len());
        self.child_lookup =
            Vec::with_capacity(self.nodes.iter().map(|node| node.children.len()).sum());
        for node in &self.nodes {
            let start = self.child_lookup.len();
            self.child_lookup.extend_from_slice(&node.children);
            // Stable order retains the first original match even for duplicates.
            self.child_lookup[start..].sort_by_key(|(edge, _)| *edge);
            self.child_ranges.push((start, self.child_lookup.len()));
        }
        self.page_frontiers = vec![false; self.nodes.len()];
        let mut pending = vec![self.root];
        while let Some(current) = pending.pop() {
            let Some(node) = self.node(current) else { continue };
            if self.page_frontiers[current.0]
                || node.original
                || node.conditional_ref.is_some()
                || matches!(
                    node.role,
                    PlanRole::OperationalTemplate | PlanRole::PrunedOriginal
                )
            {
                continue;
            }
            self.page_frontiers[current.0] = true;
            let node = &self.nodes[current.0];
            for (edge, child) in &node.children {
                if child.0 > current.0 && Self::continues_frontier(node, edge) {
                    pending.push(*child);
                }
            }
        }
    }

    fn continues_frontier(node: &PlanNode, edge: &PlanEdge) -> bool {
        match edge {
            PlanEdge::SourceBody
            | PlanEdge::LocatedBody
            | PlanEdge::ShowRecipeBody
            | PlanEdge::RealizedVisual
            | PlanEdge::SequencePart(_)
            | PlanEdge::PageRunBody => true,
            PlanEdge::StyledBody => {
                if let Content::Styled(styled) = &node.content {
                    styled.styles.delta().custom.iter().any(|(key, value)| {
                        key.as_str() == "eval.flow.scope"
                            && matches!(value, crate::entities::value::Value::Bool(true))
                    })
                } else {
                    false
                }
            }
            PlanEdge::ColumnsBody => {
                matches!(&node.content, Content::Columns(columns) if columns.page_columns)
            }
            PlanEdge::RealizedOriginal
            | PlanEdge::ParagraphBody
            | PlanEdge::HeadingBody
            | PlanEdge::TitleBody
            | PlanEdge::StrongBody
            | PlanEdge::EmphBody
            | PlanEdge::SmallCapsBody
            | PlanEdge::LabelBody
            | PlanEdge::ListItemBody
            | PlanEdge::EnumItemBody
            | PlanEdge::EquationBody
            | PlanEdge::FigureBody
            | PlanEdge::FigureCaption
            | PlanEdge::FigureSelectedSupplement
            | PlanEdge::FigureCaptionBody
            | PlanEdge::FigureCaptionSeparator
            | PlanEdge::PreparedReferenceSupplement
            | PlanEdge::CitationSupplement
            | PlanEdge::GridCell(_)
            | PlanEdge::GridHeader
            | PlanEdge::GridFooter
            | PlanEdge::GridCellBody
            | PlanEdge::GridHeaderBody
            | PlanEdge::GridFooterBody
            | PlanEdge::TableResolvedCell(_)
            | PlanEdge::TableRawChild(_)
            | PlanEdge::TableHeader
            | PlanEdge::TableFooter
            | PlanEdge::TableCaption
            | PlanEdge::CellSelectedVisual
            | PlanEdge::TableHeaderBody
            | PlanEdge::TableFooterBody
            | PlanEdge::PageRunBackground
            | PlanEdge::PageRunHeader
            | PlanEdge::PageRunFooter
            | PlanEdge::PageRunForeground
            | PlanEdge::SetPageBackground
            | PlanEdge::SetPageHeader
            | PlanEdge::SetPageFooter
            | PlanEdge::SetPageForeground
            | PlanEdge::HtmlFrameBody
            | PlanEdge::TermsItem(_)
            | PlanEdge::TermItemTerm
            | PlanEdge::TermItemDescription
            | PlanEdge::QuoteBody
            | PlanEdge::QuoteAttribution
            | PlanEdge::StackChild(_)
            | PlanEdge::AlignBody
            | PlanEdge::PlaceBody
            | PlanEdge::TransformBody
            | PlanEdge::PdfArtifactBody
            | PlanEdge::UnderlineBody
            | PlanEdge::StrikeBody
            | PlanEdge::OverlineBody
            | PlanEdge::PadBody
            | PlanEdge::HideBody
            | PlanEdge::BoxedBody
            | PlanEdge::BlockBody
            | PlanEdge::RepeatBody
            | PlanEdge::BibliographyTitle
            | PlanEdge::FootnoteBody
            | PlanEdge::FootnoteEntryNote
            | PlanEdge::FootnoteEntrySeparator
            | PlanEdge::OutlineRealizedBody
            | PlanEdge::OutlineEntryBody => false,
        }
    }

    #[cfg(test)]
    pub(in crate::compiler) fn test_source_plan(
        root: PacketId,
        nodes: Vec<PlanNode>,
        environments: Vec<PlanEnvironment>,
        allocation_start: LocatorSeed,
        allocation_owner: Option<Arc<()>>,
        document_boundary: Option<(LocatorSeed, Location)>,
    ) -> Self {
        let mut plan = Self {
            root,
            nodes,
            environments,
            allocation_start,
            allocation_owner,
            document_boundary,
            page_frontiers: Vec::new(),
            child_ranges: Vec::new(),
            child_lookup: Vec::new(),
        };
        plan.initialize_queries();
        plan
    }

    #[cfg(test)]
    /// Page-wide style frontiers follow declared source ancestry, never local render style.
    fn reference_frontier(&self, packet: PacketId) -> bool {
        fn reaches(plan: &SourcePlan, current: PacketId, target: PacketId) -> bool {
            let Some(node) = plan.node(current) else { return false };
            if node.original
                || node.conditional_ref.is_some()
                || matches!(
                    node.role,
                    PlanRole::OperationalTemplate | PlanRole::PrunedOriginal
                )
            {
                return false;
            }
            if current == target {
                return true;
            }
            node.children.iter().any(|(edge, child)| {
                if child.0 <= current.0 || child.0 > target.0 { return false; }
                let continues = match edge {
                    PlanEdge::SourceBody | PlanEdge::LocatedBody | PlanEdge::ShowRecipeBody
                    | PlanEdge::RealizedVisual | PlanEdge::SequencePart(_) | PlanEdge::PageRunBody => true,
                    PlanEdge::StyledBody => if let Content::Styled(styled) = &node.content {
                        styled.styles.delta().custom.iter().any(|(key, value)|
                            key.as_str() == "eval.flow.scope" && matches!(value, crate::entities::value::Value::Bool(true)))
                    } else { false },
                    PlanEdge::ColumnsBody => matches!(&node.content, Content::Columns(columns) if columns.page_columns),
                    PlanEdge::RealizedOriginal
                    | PlanEdge::ParagraphBody
                    | PlanEdge::HeadingBody
                    | PlanEdge::TitleBody
                    | PlanEdge::StrongBody
                    | PlanEdge::EmphBody
                    | PlanEdge::SmallCapsBody
                    | PlanEdge::LabelBody
                    | PlanEdge::ListItemBody
                    | PlanEdge::EnumItemBody
                    | PlanEdge::EquationBody
                    | PlanEdge::FigureBody
                    | PlanEdge::FigureCaption
                    | PlanEdge::FigureSelectedSupplement
                    | PlanEdge::FigureCaptionBody
                    | PlanEdge::FigureCaptionSeparator
                    | PlanEdge::PreparedReferenceSupplement
                    | PlanEdge::CitationSupplement
                    | PlanEdge::GridCell(_)
                    | PlanEdge::GridHeader
                    | PlanEdge::GridFooter
                    | PlanEdge::GridCellBody
                    | PlanEdge::GridHeaderBody
                    | PlanEdge::GridFooterBody
                    | PlanEdge::TableResolvedCell(_)
                    | PlanEdge::TableRawChild(_)
                    | PlanEdge::TableHeader
                    | PlanEdge::TableFooter
                    | PlanEdge::TableCaption
                    | PlanEdge::CellSelectedVisual
                    | PlanEdge::TableHeaderBody
                    | PlanEdge::TableFooterBody
                    | PlanEdge::PageRunBackground
                    | PlanEdge::PageRunHeader
                    | PlanEdge::PageRunFooter
                    | PlanEdge::PageRunForeground
                    | PlanEdge::SetPageBackground
                    | PlanEdge::SetPageHeader
                    | PlanEdge::SetPageFooter
                    | PlanEdge::SetPageForeground
                    | PlanEdge::HtmlFrameBody
                    | PlanEdge::TermsItem(_)
                    | PlanEdge::TermItemTerm
                    | PlanEdge::TermItemDescription
                    | PlanEdge::QuoteBody
                    | PlanEdge::QuoteAttribution
                    | PlanEdge::StackChild(_)
                    | PlanEdge::AlignBody
                    | PlanEdge::PlaceBody
                    | PlanEdge::TransformBody
                    | PlanEdge::PdfArtifactBody
                    | PlanEdge::UnderlineBody
                    | PlanEdge::StrikeBody
                    | PlanEdge::OverlineBody
                    | PlanEdge::PadBody
                    | PlanEdge::HideBody
                    | PlanEdge::BoxedBody
                    | PlanEdge::BlockBody
                    | PlanEdge::RepeatBody
                    | PlanEdge::BibliographyTitle
                    | PlanEdge::FootnoteBody
                    | PlanEdge::FootnoteEntryNote
                    | PlanEdge::FootnoteEntrySeparator
                    | PlanEdge::OutlineRealizedBody
                    | PlanEdge::OutlineEntryBody => false,
                };
                continues && reaches(plan, *child, target)
            })
        }
        self.node(packet).is_some() && reaches(self, self.root, packet)
    }
    pub(in crate::compiler) fn environment(
        &self,
        id: StyleEnvironmentId,
    ) -> Option<&PlanEnvironment> {
        self.environments.get(id.0)
    }
}

/// A plan carrier transports a child's identity, never owns its semantic payload.
fn plan_owns_payload(content: &Content) -> bool {
    !matches!(content, Content::Source(_) | Content::Located(_))
        && super::do_extract_payload(content).is_some()
}

fn invalid_plan(span: Span) -> Vec<crate::entities::source_result::SourceDiagnostic> {
    vec![crate::entities::source_result::SourceDiagnostic::error(
        span,
        "page running source plan could not be verified",
    )]
}

/// Reserve explicit identities, including dormant operational templates, without
/// adding semantic elements or invoking captured language functions.
fn reserve_source(content: &Content, ctx: &mut EvalContext) -> SourceResult<()> {
    let _ = content.map_content_pre(&mut |node| {
        match node {
            Content::Located(located) => {
                ctx.realization_locator.observe(located.location);
                reserve_source(&located.body, ctx)?;
                return Ok(Some(node.clone()));
            }
            Content::Realized(realized) => {
                reserve_source(&realized.original, ctx)?;
                reserve_source(&realized.visual, ctx)?;
                return Ok(Some(node.clone()));
            }
            Content::ContextBlock(block) => {
                ctx.next_context_id = ctx.next_context_id.max(block.id.saturating_add(1));
            }
            Content::ShowRecipe(recipe) => {
                ctx.next_context_id = ctx.next_context_id.max(recipe.context_id_floor);
            }
            Content::Ref(reference) => {
                if let Some(supplement) = &reference.supplement {
                    reserve_source(supplement, ctx)?;
                }
            }
            Content::SetPage(delta) => {
                for marginal in [&delta.header, &delta.footer] {
                    if let Some(crate::entities::page_running::PageMarginal::Content(
                        body,
                    )) = marginal
                    {
                        reserve_source(body, ctx)?;
                    }
                }
                for layer in [&delta.background, &delta.foreground] {
                    if let Some(Some(body)) = layer {
                        reserve_source(body, ctx)?;
                    }
                }
            }
            Content::Source(_)
            | Content::Empty
            | Content::Text(_)
            | Content::Space
            | Content::SpaceFromNewline
            | Content::Parbreak
            | Content::Sequence(_)
            | Content::HtmlElem(_)
            | Content::HtmlFrame(_)
            | Content::Par { .. }
            | Content::Heading(_)
            | Content::Title(_)
            | Content::Strong(_)
            | Content::Emph(_)
            | Content::Raw(_)
            | Content::ListItem(_)
            | Content::EnumItem(_)
            | Content::Link(_)
            | Content::Equation(_)
            | Content::MathSequence(_)
            | Content::Symbol(_)
            | Content::MathIdent(_)
            | Content::MathText(_)
            | Content::MathFrac(_)
            | Content::MathAttach(_)
            | Content::MathPrimes(_)
            | Content::MathRoot(_)
            | Content::MathStretch(_)
            | Content::MathDelimited(_)
            | Content::MathLr(_)
            | Content::MathMid(_)
            | Content::MathOverbrace(_)
            | Content::MathUnderbrace(_)
            | Content::MathUnderbracket(_)
            | Content::MathUnderparen(_)
            | Content::MathUndershell(_)
            | Content::MathOverbracket(_)
            | Content::MathOverparen(_)
            | Content::MathOvershell(_)
            | Content::MathAlignPoint(_)
            | Content::Linebreak(_)
            | Content::MathMatrix(_)
            | Content::MathCases(_)
            | Content::MathAccent(_)
            | Content::MathCancel(_)
            | Content::MathUnderline(_)
            | Content::MathOverline(_)
            | Content::MathVec(_)
            | Content::MathClassOverride(_)
            | Content::MathLimitsOverride(_)
            | Content::MathUnderover(_)
            | Content::MathOp(_)
            | Content::MathStyled(_)
            | Content::Label(_)
            | Content::CounterDisplay(_)
            | Content::CounterUpdate(_)
            | Content::Outline(_)
            | Content::OutlineEntry(_)
            | Content::Figure(_)
            | Content::FigureCaption(_)
            | Content::Image(_)
            | Content::Shape(_)
            | Content::Curve(_)
            | Content::Transform(_)
            | Content::Grid(_)
            | Content::GridHeader(_)
            | Content::GridFooter(_)
            | Content::GridCell(_)
            | Content::PageRun(_)
            | Content::Align(_)
            | Content::Place(_)
            | Content::Flush(_)
            | Content::Styled(_)
            | Content::Divider(_)
            | Content::Terms(_)
            | Content::TermItem(_)
            | Content::Quote(_)
            | Content::Document { .. }
            | Content::Asset { .. }
            | Content::SmartQuote(_)
            | Content::PdfAttach(_)
            | Content::PdfArtifact(_)
            | Content::Underline(_)
            | Content::Strike(_)
            | Content::Overline(_)
            | Content::SmallCaps { .. }
            | Content::Pad(_)
            | Content::Hide(_)
            | Content::HSpace(_)
            | Content::VSpace(_)
            | Content::Pagebreak(_)
            | Content::Colbreak(_)
            | Content::Stack(_)
            | Content::Boxed(_)
            | Content::Block(_)
            | Content::TableCell(_)
            | Content::Bibliography(_)
            | Content::Cite(_)
            | Content::Footnote(_)
            | Content::FootnoteEntry(_)
            | Content::TableHeader(_)
            | Content::TableFooter(_)
            | Content::GridHLine(_)
            | Content::GridVLine(_)
            | Content::TableHLine(_)
            | Content::TableVLine(_)
            | Content::Table(_)
            | Content::Repeat(_)
            | Content::Columns(_)
            | Content::Metadata(_)
            | Content::State(_)
            | Content::StateUpdate(_)
            | Content::StateDisplay(_)
            | Content::CounterDisplayCallback(_)
            | Content::Dynamic(_) => {}
        }
        Ok(None)
    })?;
    Ok(())
}

/// Inherited source provenance for one recursive construction boundary.
struct PlanBuildContext {
    environment: StyleEnvironmentId,
    source_parent: Option<PacketId>,
    label: Option<Label>,
    forced: Option<Location>,
    role: PlanRole,
    span: Span,
    math: bool,
}

/// One owned child selected by the closed source partition before descent.
struct PlanChild {
    edge: PlanEdge,
    content: Content,
    environment: StyleEnvironmentId,
    label: Option<Label>,
    forced: Option<Location>,
    role: PlanRole,
}

struct PlanBuilder<'a> {
    ctx: &'a mut EvalContext,
    allocate: bool,
    nodes: Vec<PlanNode>,
    environments: Vec<PlanEnvironment>,
    conditional_ref: Option<PacketId>,
    original: bool,
}

impl PlanBuilder<'_> {
    fn environment_id(
        &mut self,
        parent: Option<StyleEnvironmentId>,
        origin: EnvironmentOrigin,
        chain: StyleChain,
    ) -> StyleEnvironmentId {
        let id = StyleEnvironmentId(self.environments.len());
        self.environments.push(PlanEnvironment { parent, origin, chain });
        id
    }

    fn build(
        &mut self,
        content: &Content,
        context: PlanBuildContext,
    ) -> SourceResult<PacketId> {
        let PlanBuildContext {
            environment,
            source_parent,
            label,
            forced,
            role,
            span,
            math,
        } = context;
        let id = PacketId(self.nodes.len());
        let pruned = self.original
            && matches!(content, Content::Figure(figure) if figure.resolved_kind.is_none());
        let role = if pruned { PlanRole::PrunedOriginal } else { role };
        let chain = self.environments[environment.0].chain.clone();
        let own_span = content.source_span();
        let span = if own_span.is_detached() { span } else { own_span };
        let location = if pruned {
            None
        } else if let Content::Located(located) = content {
            Some(located.location)
        } else if plan_owns_payload(content)
            || matches!(content, Content::ContextBlock(block) if
                crate::compiler::realize::reference_supplements::is_reference_supplement_producer(block))
        {
            match forced {
                Some(location) => Some(location),
                None if role == PlanRole::OperationalTemplate => None,
                None => {
                    if !self.allocate {
                        return Err(invalid_plan(span));
                    }
                    let located =
                        crate::compiler::realize::locate(content.clone(), self.ctx);
                    let Content::Located(located) = located else {
                        unreachable!("locate returns Located")
                    };
                    Some(located.location)
                }
            }
        } else {
            None
        };
        self.nodes.push(PlanNode {
            content: content.clone(),
            children: Vec::new(),
            location,
            source_parent,
            environment,
            lang: chain.lang(),
            label: label.clone(),
            span,
            role,
            auxiliary_locations: Vec::new(),
            conditional_ref: self.conditional_ref,
            original: self.original,
            math,
        });
        if matches!(role, PlanRole::OperationalTemplate | PlanRole::PrunedOriginal) {
            return Ok(id);
        }
        if let Content::Ref(reference) = content {
            if self.allocate && !self.original {
                let location = if let Some(location) = forced {
                    location
                } else {
                    let located = crate::compiler::realize::locate(
                        Content::cite(
                            reference.name.to_string(),
                            reference.supplement.clone(),
                            None,
                        ),
                        self.ctx,
                    );
                    let Content::Located(located) = located else {
                        unreachable!("locate returns Located")
                    };
                    located.location
                };
                self.nodes[id.0].auxiliary_locations.push(location);
            }
        }
        if let Content::Outline(outline) = content {
            if self.allocate
                && outline.realized_body().is_none()
                && (!outline.title_was_set() || outline.title.is_some())
            {
                let title =
                    outline.title.clone().unwrap_or_else(|| Content::text("Contents"));
                let located = crate::compiler::realize::locate(
                    Content::heading(1, title),
                    self.ctx,
                );
                if let Content::Located(located) = located {
                    self.nodes[id.0].auxiliary_locations.push(located.location);
                }
            }
        }
        let nested_equation = math && matches!(content, Content::Equation(_));
        let parent = if (plan_owns_payload(content) && !nested_equation)
            || matches!(content, Content::Ref(_))
        {
            Some(id)
        } else {
            source_parent
        };
        let mut children: Vec<PlanChild> = Vec::new();
        macro_rules! child {
            ($edge:expr, $body:expr) => {
                children.push(PlanChild {
                    edge: $edge,
                    content: ($body).clone(),
                    environment,
                    label: None,
                    forced: None,
                    role: PlanRole::Canonical,
                })
            };
        }
        if crate::compiler::realize::reference_supplements::prepared_reference_supplement(
            content,
        )
        .is_some()
        {
            if let Content::Styled(carrier) = content {
                children.push(PlanChild {
                    edge: PlanEdge::StyledBody,
                    content: carrier.body.clone(),
                    environment,
                    label,
                    forced,
                    role: PlanRole::PreparedReference,
                });
            }
        } else {
            match content {
                Content::Source(source) => children.push(PlanChild {
                    edge: PlanEdge::SourceBody,
                    content: source.body.clone(),
                    environment,
                    label,
                    forced,
                    role,
                }),
                Content::Located(located) => children.push(PlanChild {
                    edge: PlanEdge::LocatedBody,
                    content: located.body.clone(),
                    environment,
                    label,
                    forced: Some(located.location),
                    role,
                }),
                Content::ShowRecipe(recipe) => {
                    // Captured recipe is already realized or an operational template;
                    // ordinary semantic traversal keeps its declared source body.
                    let captured = self.environment_id(
                        Some(environment),
                        EnvironmentOrigin::CapturedRecipe(id),
                        chain.clone(),
                    );
                    children.push(PlanChild {
                        edge: PlanEdge::ShowRecipeBody,
                        content: recipe.body.clone(),
                        environment: captured,
                        label,
                        forced,
                        role,
                    });
                }
                Content::Styled(styled) => {
                    let pushed = chain.push_styles(&styled.styles);
                    let env = self.environment_id(
                        Some(environment),
                        EnvironmentOrigin::Styled(id),
                        pushed,
                    );
                    children.push(PlanChild {
                        edge: PlanEdge::StyledBody,
                        content: styled.body.clone(),
                        environment: env,
                        label,
                        forced,
                        role,
                    });
                }
                Content::Realized(realized) => {
                    children.push(PlanChild {
                        edge: PlanEdge::RealizedOriginal,
                        content: realized.original.clone(),
                        environment,
                        label: label.clone(),
                        forced: None,
                        role: PlanRole::OriginalOnly,
                    });
                    children.push(PlanChild {
                        edge: PlanEdge::RealizedVisual,
                        content: realized.visual.clone(),
                        environment,
                        label,
                        forced: None,
                        role: PlanRole::VisualOnly,
                    });
                }
                Content::Sequence(parts) => {
                    for (index, part) in parts.iter().enumerate() {
                        child!(PlanEdge::SequencePart(index), part);
                    }
                }
                Content::Par { body } => child!(PlanEdge::ParagraphBody, &**body),
                Content::SmallCaps { body } => child!(PlanEdge::SmallCapsBody, &**body),
                Content::Heading(e) => child!(PlanEdge::HeadingBody, &e.body),
                Content::Title(e) => child!(PlanEdge::TitleBody, &e.body),
                Content::Strong(e) => child!(PlanEdge::StrongBody, &e.body),
                Content::Emph(e) => child!(PlanEdge::EmphBody, &e.body),
                Content::ListItem(e) => child!(PlanEdge::ListItemBody, &e.body),
                Content::EnumItem(e) => child!(PlanEdge::EnumItemBody, &e.body),
                Content::Equation(_) if role == PlanRole::OriginalOnly => {}
                Content::Equation(e) => child!(PlanEdge::EquationBody, &e.body),
                Content::Label(e) => children.push(PlanChild {
                    edge: PlanEdge::LabelBody,
                    content: e.body.clone(),
                    environment,
                    label: Some(Label(e.name.to_string())),
                    forced: None,
                    role: PlanRole::Canonical,
                }),
                Content::Figure(e) if role == PlanRole::OriginalOnly => {
                    let _ = e;
                }
                Content::Figure(e) => {
                    child!(PlanEdge::FigureBody, &e.body);
                    if let Some(caption) = &e.caption {
                        if e.resolved_kind.is_some()
                            && matches!(
                                chain.custom("figure.numbering"),
                                None | Some(crate::entities::value::Value::Str(_))
                            )
                        {
                            if let Some(Some(supplement)) = &e.resolved_supplement {
                                child!(PlanEdge::FigureSelectedSupplement, supplement);
                            }
                        }
                        child!(PlanEdge::FigureCaption, caption);
                    }
                }
                Content::FigureCaption(e) => {
                    child!(PlanEdge::FigureCaptionBody, &e.body);
                    if let Some(separator) = &e.separator {
                        child!(PlanEdge::FigureCaptionSeparator, separator);
                    }
                }
                Content::Table(e) if role == PlanRole::OriginalOnly => {
                    let _ = e;
                }
                Content::Table(e) => {
                    if let Some(caption) = &e.caption {
                        child!(PlanEdge::TableCaption, caption);
                    }
                    if let Some(resolved) = &e.resolved {
                        let mut cells: Vec<_> =
                            resolved.cells.iter().enumerate().collect();
                        cells.sort_by_key(|(_, cell)| match cell {
                            Content::TableCell(cell) => cell
                                .resolved
                                .as_ref()
                                .map(|r| (r.y + r.rowspan - 1, r.x))
                                .unwrap_or((0, 0)),
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
                            | Content::Dynamic(..) => (0, 0),
                        });
                        for (index, cell) in cells {
                            child!(PlanEdge::TableResolvedCell(index), cell);
                        }
                    } else if let Some(items) = &e.items {
                        for (index, item) in items.iter().enumerate() {
                            child!(PlanEdge::TableRawChild(index), item);
                        }
                    } else {
                        if let Some(header) = &e.header {
                            child!(PlanEdge::TableHeader, header);
                        }
                        for (index, cell) in e.children.iter().enumerate() {
                            child!(PlanEdge::TableRawChild(index), cell);
                        }
                        if let Some(footer) = &e.footer {
                            child!(PlanEdge::TableFooter, footer);
                        }
                    }
                }
                Content::TableCell(e) => child!(PlanEdge::CellSelectedVisual, e.visual()),
                Content::Grid(e) => {
                    for (index, cell) in e.cells.iter().enumerate() {
                        child!(PlanEdge::GridCell(index), cell);
                    }
                    if let Some(header) = &e.header {
                        child!(PlanEdge::GridHeader, header);
                    }
                    if let Some(footer) = &e.footer {
                        child!(PlanEdge::GridFooter, footer);
                    }
                }
                Content::GridCell(e) => child!(PlanEdge::GridCellBody, &e.body),
                Content::GridHeader(e) => child!(PlanEdge::GridHeaderBody, &e.body),
                Content::GridFooter(e) => child!(PlanEdge::GridFooterBody, &e.body),
                Content::TableHeader(e) => child!(PlanEdge::TableHeaderBody, &e.body),
                Content::TableFooter(e) => child!(PlanEdge::TableFooterBody, &e.body),
                Content::PageRun(e) => {
                    child!(PlanEdge::PageRunBody, &e.body);
                    for (edge, marginal) in [
                        (PlanEdge::PageRunHeader, &e.header),
                        (PlanEdge::PageRunFooter, &e.footer),
                    ] {
                        if let Some(
                            crate::entities::page_running::PageMarginal::Content(body),
                        ) = marginal
                        {
                            children.push(PlanChild {
                                edge,
                                content: (**body).clone(),
                                environment,
                                label: None,
                                forced: None,
                                role: PlanRole::OperationalTemplate,
                            });
                        }
                    }
                    for (edge, layer) in [
                        (PlanEdge::PageRunBackground, &e.background),
                        (PlanEdge::PageRunForeground, &e.foreground),
                    ] {
                        if let Some(Some(body)) = layer {
                            children.push(PlanChild {
                                edge,
                                content: body.clone(),
                                environment,
                                label: None,
                                forced: None,
                                role: PlanRole::OperationalTemplate,
                            });
                        }
                    }
                }
                Content::SetPage(delta) => {
                    for (edge, marginal) in [
                        (PlanEdge::SetPageHeader, &delta.header),
                        (PlanEdge::SetPageFooter, &delta.footer),
                    ] {
                        if let Some(
                            crate::entities::page_running::PageMarginal::Content(body),
                        ) = marginal
                        {
                            children.push(PlanChild {
                                edge,
                                content: (**body).clone(),
                                environment,
                                label: None,
                                forced: None,
                                role: PlanRole::OperationalTemplate,
                            });
                        }
                    }
                    for (edge, layer) in [
                        (PlanEdge::SetPageBackground, &delta.background),
                        (PlanEdge::SetPageForeground, &delta.foreground),
                    ] {
                        if let Some(Some(body)) = layer {
                            children.push(PlanChild {
                                edge,
                                content: (**body).clone(),
                                environment,
                                label: None,
                                forced: None,
                                role: PlanRole::OperationalTemplate,
                            });
                        }
                    }
                }
                Content::HtmlFrame(e) => child!(PlanEdge::HtmlFrameBody, &e.body),
                Content::Terms(e) => {
                    for (index, item) in e.items.iter().enumerate() {
                        child!(PlanEdge::TermsItem(index), item);
                    }
                }
                Content::TermItem(e) => {
                    child!(PlanEdge::TermItemTerm, &e.term);
                    child!(PlanEdge::TermItemDescription, &e.description);
                }
                Content::Quote(e) => {
                    child!(PlanEdge::QuoteBody, &e.body);
                    if let Some(a) = &e.attribution {
                        child!(PlanEdge::QuoteAttribution, a);
                    }
                }
                Content::Stack(e) => {
                    for (index, item) in e.children.iter().enumerate() {
                        child!(PlanEdge::StackChild(index), item);
                    }
                }
                Content::Align(e) => child!(PlanEdge::AlignBody, &e.body),
                Content::Place(e) => child!(PlanEdge::PlaceBody, &e.body),
                Content::Transform(e) => child!(PlanEdge::TransformBody, &e.body),
                Content::PdfArtifact(e) => child!(PlanEdge::PdfArtifactBody, &e.body),
                Content::Underline(e) => child!(PlanEdge::UnderlineBody, &e.body),
                Content::Strike(e) => child!(PlanEdge::StrikeBody, &e.body),
                Content::Overline(e) => child!(PlanEdge::OverlineBody, &e.body),
                Content::Pad(e) => child!(PlanEdge::PadBody, &e.body),
                Content::Hide(e) => child!(PlanEdge::HideBody, &e.body),
                Content::Boxed(e) => child!(PlanEdge::BoxedBody, &e.body),
                Content::Block(_) if role == PlanRole::OriginalOnly => {}
                Content::Block(e) => child!(PlanEdge::BlockBody, &e.body),
                Content::Repeat(e) => child!(PlanEdge::RepeatBody, &e.body),
                Content::Columns(e) => child!(PlanEdge::ColumnsBody, &e.body),
                Content::Bibliography(e) => {
                    if let Some(title) = &e.title {
                        child!(PlanEdge::BibliographyTitle, title);
                    }
                }
                Content::Cite(e) => {
                    if let Some(supplement) = &e.supplement {
                        child!(PlanEdge::CitationSupplement, supplement);
                    }
                }
                Content::Footnote(e) => child!(PlanEdge::FootnoteBody, &e.body),
                Content::FootnoteEntry(e) => {
                    child!(PlanEdge::FootnoteEntryNote, &e.note);
                    if let Some(separator) = &e.separator {
                        child!(PlanEdge::FootnoteEntrySeparator, separator);
                    }
                }
                Content::Outline(e) => {
                    if let Some(body) = e.realized_body() {
                        child!(PlanEdge::OutlineRealizedBody, body);
                    }
                }
                Content::OutlineEntry(_) if role == PlanRole::OriginalOnly => {}
                Content::OutlineEntry(e) => {
                    if let Ok(body) = e.body() {
                        child!(PlanEdge::OutlineEntryBody, &body);
                    }
                }
                Content::Ref(reference) => {
                    if !self.original {
                        if let Some(supplement) = &reference.supplement {
                            child!(PlanEdge::CitationSupplement, supplement);
                        }
                    }
                    if role == PlanRole::PreparedReference {
                        if let Some(body) = &reference.supplement {
                            child!(PlanEdge::PreparedReferenceSupplement, body);
                        }
                    }
                }
                Content::Empty
                | Content::Text(_)
                | Content::Space
                | Content::SpaceFromNewline
                | Content::Parbreak
                | Content::HtmlElem(_)
                | Content::Raw(_)
                | Content::Link(_)
                | Content::MathSequence(_)
                | Content::Symbol(_)
                | Content::MathIdent(_)
                | Content::MathText(_)
                | Content::MathFrac(_)
                | Content::MathAttach(_)
                | Content::MathPrimes(_)
                | Content::MathRoot(_)
                | Content::MathStretch(_)
                | Content::MathDelimited(_)
                | Content::MathLr(_)
                | Content::MathMid(_)
                | Content::MathOverbrace(_)
                | Content::MathUnderbrace(_)
                | Content::MathUnderbracket(_)
                | Content::MathUnderparen(_)
                | Content::MathUndershell(_)
                | Content::MathOverbracket(_)
                | Content::MathOverparen(_)
                | Content::MathOvershell(_)
                | Content::MathAlignPoint(_)
                | Content::Linebreak(_)
                | Content::MathMatrix(_)
                | Content::MathCases(_)
                | Content::MathAccent(_)
                | Content::MathCancel(_)
                | Content::MathUnderline(_)
                | Content::MathOverline(_)
                | Content::MathVec(_)
                | Content::MathClassOverride(_)
                | Content::MathLimitsOverride(_)
                | Content::MathUnderover(_)
                | Content::MathOp(_)
                | Content::MathStyled(_)
                | Content::CounterDisplay(_)
                | Content::CounterUpdate(_)
                | Content::Image(_)
                | Content::Shape(_)
                | Content::Curve(_)
                | Content::Flush(_)
                | Content::Divider(_)
                | Content::Document { .. }
                | Content::Asset { .. }
                | Content::SmartQuote(_)
                | Content::PdfAttach(_)
                | Content::HSpace(_)
                | Content::VSpace(_)
                | Content::Pagebreak(_)
                | Content::Colbreak(_)
                | Content::GridHLine(_)
                | Content::GridVLine(_)
                | Content::TableHLine(_)
                | Content::TableVLine(_)
                | Content::Metadata(_)
                | Content::State(_)
                | Content::StateUpdate(_)
                | Content::StateDisplay(_)
                | Content::CounterDisplayCallback(_)
                | Content::ContextBlock(_)
                | Content::Dynamic(_) => {}
            }
        }
        for PlanChild {
            edge,
            content: child,
            environment,
            label,
            forced,
            role,
        } in children
        {
            let previous_conditional = self.conditional_ref;
            let previous_original = self.original;
            self.original |= edge == PlanEdge::RealizedOriginal;
            if edge == PlanEdge::CitationSupplement && matches!(content, Content::Ref(_))
            {
                self.conditional_ref = Some(id);
            }
            let packet = self.build(
                &child,
                PlanBuildContext {
                    environment,
                    source_parent: parent,
                    label,
                    forced,
                    role,
                    span,
                    math: super::walk::child_math_scope(content, math),
                },
            )?;
            self.conditional_ref = previous_conditional;
            self.original = previous_original;
            self.nodes[id.0].children.push((edge, packet));
        }
        Ok(id)
    }
}

/// Prepare immutable source addressing without publishing semantic stores.
pub fn prepare_page_running_body(
    content: &Content,
    ctx: &mut EvalContext,
) -> SourceResult<PageRunningBody> {
    if ctx.context_occurrence_slots.is_none() {
        return Err(invalid_plan(content.source_span()));
    }
    build_body(content, ctx, &StyleChain::default_chain(), true, true)
}

pub(in crate::compiler) fn prepare_with_root_chain(
    content: &Content,
    ctx: &mut EvalContext,
    chain: &StyleChain,
) -> SourceResult<PageRunningBody> {
    if ctx.context_occurrence_slots.is_none() {
        return Err(invalid_plan(content.source_span()));
    }
    build_body(content, ctx, chain, true, false)
}

fn build_body(
    content: &Content,
    ctx: &mut EvalContext,
    chain: &StyleChain,
    allocate: bool,
    document: bool,
) -> SourceResult<PageRunningBody> {
    let allocation_owner =
        crate::compiler::realize::context_occurrences::allocation_owner(ctx);
    if allocate && allocation_owner.is_none() {
        return Err(invalid_plan(content.source_span()));
    }
    reserve_source(content, ctx)?;
    let document_boundary = if document {
        Some(
            crate::compiler::realize::context_occurrences::next_with_seed(ctx)
                .ok_or_else(|| invalid_plan(content.source_span()))?,
        )
    } else {
        None
    };
    let allocation_start = document_boundary
        .map(|(seed, _)| seed)
        .unwrap_or_else(|| ctx.realization_locator.checkpoint());
    let mut builder = PlanBuilder {
        ctx,
        allocate,
        nodes: Vec::new(),
        environments: Vec::new(),
        conditional_ref: None,
        original: false,
    };
    let environment =
        builder.environment_id(None, EnvironmentOrigin::Root, chain.clone());
    let root = builder.build(
        content,
        PlanBuildContext {
            environment,
            source_parent: None,
            label: None,
            forced: None,
            role: PlanRole::Canonical,
            span: Span::detached(),
            math: false,
        },
    )?;
    let mut plan = SourcePlan {
        root,
        nodes: builder.nodes,
        environments: builder.environments,
        allocation_start,
        allocation_owner,
        document_boundary,
        page_frontiers: Vec::new(),
        child_ranges: Vec::new(),
        child_lookup: Vec::new(),
    };
    plan.initialize_queries();
    let plan = Arc::new(plan);
    Ok(PageRunningBody {
        content: content.clone(),
        plan,
        numbering_input: None,
    })
}

impl SourcePlan {
    /// Optimized paint may omit proven no-fact nodes, never source effects.
    pub(in crate::compiler) fn required(
        &self,
        packet: PacketId,
        phase: PlanPhase,
    ) -> bool {
        let Some(node) = self.node(packet) else { return true };
        if node.conditional_ref.is_some() {
            return false;
        }
        self.required_effect(packet, phase)
    }

    fn required_effect(&self, packet: PacketId, phase: PlanPhase) -> bool {
        let Some(node) = self.node(packet) else { return true };
        if matches!(node.role, PlanRole::OperationalTemplate | PlanRole::PrunedOriginal) {
            return false;
        }
        if phase == PlanPhase::Root && plan_owns_payload(&node.content) {
            return true;
        }
        match &node.content {
            Content::Heading(_) | Content::Outline(_) | Content::OutlineEntry(_) => phase == PlanPhase::PostChildren,
            Content::Label(_) => true,
            Content::Ref(_) => phase == PlanPhase::Root && node.role != PlanRole::OriginalOnly,
            Content::Styled(styled) => phase == PlanPhase::Root && styled.styles.delta().custom.iter().any(|(key, _)| key.as_str() == "outline.prefix-info"),
            Content::ContextBlock(block) => phase == PlanPhase::Root && crate::compiler::realize::reference_supplements::is_reference_supplement_producer(block),
            Content::Source(_)
            | Content::ShowRecipe(_)
            | Content::Realized(_)
            | Content::Located(_)
            | Content::Empty
            | Content::Text(_)
            | Content::Space
            | Content::SpaceFromNewline
            | Content::Parbreak
            | Content::Sequence(_)
            | Content::HtmlElem(_)
            | Content::HtmlFrame(_)
            | Content::Par { .. }
            | Content::Title(_)
            | Content::Strong(_)
            | Content::Emph(_)
            | Content::Raw(_)
            | Content::ListItem(_)
            | Content::EnumItem(_)
            | Content::Link(_)
            | Content::Equation(_)
            | Content::MathSequence(_)
            | Content::Symbol(_) | Content::MathIdent(_)
            | Content::MathText(_)
            | Content::MathFrac(_)
            | Content::MathAttach(_)
            | Content::MathPrimes(_)
            | Content::MathRoot(_)
            | Content::MathStretch(_)
            | Content::MathDelimited(_)
            | Content::MathLr(_)
            | Content::MathMid(_)
            | Content::MathOverbrace(_)
            | Content::MathUnderbrace(_)
            | Content::MathUnderbracket(_)
            | Content::MathUnderparen(_)
            | Content::MathUndershell(_)
            | Content::MathOverbracket(_)
            | Content::MathOverparen(_)
            | Content::MathOvershell(_)
            | Content::MathAlignPoint(_)
            | Content::Linebreak(_)
            | Content::MathMatrix(_)
            | Content::MathCases(_)
            | Content::MathAccent(_)
            | Content::MathCancel(_)
            | Content::MathUnderline(_)
            | Content::MathOverline(_)
            | Content::MathVec(_)
            | Content::MathClassOverride(_)
            | Content::MathLimitsOverride(_)
            | Content::MathUnderover(_)
            | Content::MathOp(_)
            | Content::MathStyled(_)
            | Content::CounterDisplay(_)
            | Content::CounterUpdate(_)
            | Content::Figure(_)
            | Content::FigureCaption(_)
            | Content::Image(_)
            | Content::Shape(_)
            | Content::Curve(_)
            | Content::Transform(_)
            | Content::Grid(_)
            | Content::GridHeader(_)
            | Content::GridFooter(_)
            | Content::GridCell(_)
            | Content::SetPage(_)
            | Content::PageRun(_)
            | Content::Align(_)
            | Content::Place(_)
            | Content::Flush(_)
            | Content::Divider(_)
            | Content::Terms(_)
            | Content::TermItem(_)
            | Content::Quote(_)
            | Content::Document { .. }
            | Content::Asset { .. }
            | Content::SmartQuote(_)
            | Content::PdfAttach(_)
            | Content::PdfArtifact(_)
            | Content::Underline(_)
            | Content::Strike(_)
            | Content::Overline(_)
            | Content::SmallCaps { .. }
            | Content::Pad(_)
            | Content::Hide(_)
            | Content::HSpace(_)
            | Content::VSpace(_)
            | Content::Pagebreak(_)
            | Content::Colbreak(_)
            | Content::Stack(_)
            | Content::Boxed(_)
            | Content::Block(_)
            | Content::TableCell(_)
            | Content::Bibliography(_)
            | Content::Cite(_)
            | Content::Footnote(_)
            | Content::FootnoteEntry(_)
            | Content::TableHeader(_)
            | Content::TableFooter(_)
            | Content::GridHLine(_)
            | Content::GridVLine(_)
            | Content::TableHLine(_)
            | Content::TableVLine(_)
            | Content::Table(_)
            | Content::Repeat(_)
            | Content::Columns(_)
            | Content::Metadata(_)
            | Content::State(_)
            | Content::StateUpdate(_)
            | Content::StateDisplay(_)
            | Content::CounterDisplayCallback(_)
            | Content::Dynamic(_) => false,
        }
    }
}

#[derive(Clone)]
struct PlannedEvent {
    plan: Arc<SourcePlan>,
    packet: PacketId,
    phase: PlanPhase,
}

fn source_events(
    plan: &Arc<SourcePlan>,
    packet: PacketId,
    events: &mut Vec<PlannedEvent>,
) -> SourceResult<()> {
    let node = plan.node(packet).ok_or_else(|| invalid_plan(Span::detached()))?;
    if matches!(node.role, PlanRole::OperationalTemplate | PlanRole::PrunedOriginal)
        || node.conditional_ref.is_some()
    {
        return Ok(());
    }
    events.push(PlannedEvent { plan: plan.clone(), packet, phase: PlanPhase::Root });
    for (_, child) in &node.children {
        source_events(plan, *child, events)?;
    }
    events.push(PlannedEvent {
        plan: plan.clone(),
        packet,
        phase: PlanPhase::PostChildren,
    });
    Ok(())
}

#[cfg(test)]
fn first_root_location(plan: &SourcePlan, packet: PacketId) -> Option<Location> {
    let node = plan.node(packet)?;
    if matches!(node.role, PlanRole::OperationalTemplate | PlanRole::PrunedOriginal) {
        return None;
    }
    if plan_owns_payload(&node.content) {
        return node.location;
    }
    node.children
        .iter()
        .find_map(|(_, child)| first_root_location(plan, *child))
}

fn conditional_active(
    plan: &SourcePlan,
    packet: PacketId,
    keys: &std::collections::HashSet<String>,
) -> bool {
    let Some(node) = plan.node(packet) else { return false };
    let Some(owner) = node.conditional_ref else { return true };
    if owner.0 >= packet.0 {
        return false;
    }
    let Some(reference) = plan.node(owner) else { return false };
    conditional_active(plan, owner, keys)
        && super::bibliography::project_reference_root(&reference.content, keys).is_some()
}

fn selected_location(
    node: &PlanNode,
    keys: &std::collections::HashSet<String>,
) -> Option<Location> {
    if !node.original
        && super::bibliography::project_reference_root(&node.content, keys).is_some()
    {
        node.auxiliary_locations.first().copied()
    } else {
        node.location
    }
}

fn first_semantic_location(
    plan: &Arc<SourcePlan>,
    packet: PacketId,
    keys: &std::collections::HashSet<String>,
    active_roots: &[ObservedSourceRoots],
) -> Option<Location> {
    let node = plan.node(packet)?;
    if matches!(node.role, PlanRole::OperationalTemplate | PlanRole::PrunedOriginal)
        || !conditional_active(plan, packet, keys)
    {
        return None;
    }
    if (plan_owns_payload(&node.content)
        && !(node.math && matches!(node.content, Content::Equation(_))))
        || (!node.original
            && super::bibliography::project_reference_root(&node.content, keys).is_some())
    {
        return active_roots
            .iter()
            .any(|observed| {
                Arc::ptr_eq(&observed.plan, plan) && observed.packets.contains(&packet)
            })
            .then(|| selected_location(node, keys))
            .flatten();
    }
    node.children
        .iter()
        .find_map(|(_, child)| first_semantic_location(plan, *child, keys, active_roots))
}

fn active_source_events(
    plan: &Arc<SourcePlan>,
    packet: PacketId,
    keys: &std::collections::HashSet<String>,
    events: &mut Vec<PlannedEvent>,
) -> SourceResult<()> {
    let node = plan.node(packet).ok_or_else(|| invalid_plan(Span::detached()))?;
    if matches!(node.role, PlanRole::OperationalTemplate | PlanRole::PrunedOriginal)
        || !conditional_active(plan, packet, keys)
    {
        return Ok(());
    }
    events.push(PlannedEvent { plan: plan.clone(), packet, phase: PlanPhase::Root });
    for (_, child) in &node.children {
        active_source_events(plan, *child, keys, events)?;
    }
    events.push(PlannedEvent {
        plan: plan.clone(),
        packet,
        phase: PlanPhase::PostChildren,
    });
    Ok(())
}

fn active_bibliography_keys(
    schedules: &[Vec<PlannedEvent>],
) -> std::collections::HashSet<String> {
    let mut keys = std::collections::HashSet::new();
    loop {
        let before = keys.len();
        for event in schedules.iter().flatten() {
            if event.phase != PlanPhase::Root
                || !conditional_active(&event.plan, event.packet, &keys)
            {
                continue;
            }
            if let Some(PlanNode {
                content: Content::Bibliography(bibliography),
                original: false,
                ..
            }) = event.plan.node(event.packet)
            {
                keys.extend(bibliography.entries.iter().map(|entry| entry.key.clone()));
            }
        }
        if before == keys.len() {
            return keys;
        }
    }
}

fn input_contribution_active(
    input: &PageRunningIndexInput<'_>,
    plan: &Arc<SourcePlan>,
    packet: PacketId,
) -> bool {
    input
        .running
        .is_none_or(|(trace, _)| trace.source_contribution_active(plan, packet))
}

fn complete_conditional_sources(
    input: &PageRunningIndexInput<'_>,
    events: &mut Vec<PlannedEvent>,
    keys: &std::collections::HashSet<String>,
) -> SourceResult<()> {
    let original = events.clone();
    let mut expanded = Vec::new();
    for event in &original {
        if !conditional_active(&event.plan, event.packet, keys) {
            return Err(invalid_plan(
                event
                    .plan
                    .node(event.packet)
                    .map_or(Span::detached(), |node| node.span),
            ));
        }
        expanded.push(event.clone());
        if event.phase != PlanPhase::Root {
            continue;
        }
        let node = event
            .plan
            .node(event.packet)
            .ok_or_else(|| invalid_plan(Span::detached()))?;
        if node.original
            || super::bibliography::project_reference_root(&node.content, keys).is_none()
        {
            continue;
        }
        let Some(child) = event.plan.child(event.packet, PlanEdge::CitationSupplement)
        else {
            continue;
        };
        let already_painted = original.iter().any(|candidate| {
            Arc::ptr_eq(&candidate.plan, &event.plan)
                && candidate
                    .plan
                    .node(candidate.packet)
                    .is_some_and(|node| node.conditional_ref == Some(event.packet))
        });
        if !already_painted {
            if input.running.is_some_and(|(trace, _)| trace.data.complete) {
                let mut expected = Vec::new();
                active_source_events(&event.plan, child, keys, &mut expected)?;
                expected.retain(|candidate| {
                    input_contribution_active(input, &candidate.plan, candidate.packet)
                });
                if expected.iter().any(|candidate| {
                    candidate.plan.required_effect(candidate.packet, candidate.phase)
                }) {
                    return Err(invalid_plan(node.span));
                }
                continue;
            }
            active_source_events(&event.plan, child, keys, &mut expanded)?;
        }
    }
    expanded.retain(|event| input_contribution_active(input, &event.plan, event.packet));
    let mut plans: Vec<Arc<SourcePlan>> = Vec::new();
    for event in &expanded {
        if !plans.iter().any(|plan| Arc::ptr_eq(plan, &event.plan)) {
            plans.push(event.plan.clone());
        }
    }
    for plan in plans {
        for (index, node) in plan.nodes.iter().enumerate() {
            if !input_contribution_active(input, &plan, PacketId(index))
                || node.conditional_ref.is_none()
                || !conditional_active(&plan, PacketId(index), keys)
            {
                continue;
            }
            for phase in [PlanPhase::Root, PlanPhase::PostChildren] {
                if plan.required_effect(PacketId(index), phase)
                    && !expanded.iter().any(|event| {
                        Arc::ptr_eq(&event.plan, &plan)
                            && event.packet == PacketId(index)
                            && event.phase == phase
                    })
                {
                    return Err(invalid_plan(node.span));
                }
            }
        }
    }
    *events = expanded;
    Ok(())
}

struct ObservedSourceRoots {
    plan: Arc<SourcePlan>,
    packets: std::collections::HashSet<PacketId>,
}

struct Construction {
    intr: TagIntrospector,
    tags: Vec<crate::entities::tag::Tag>,
    auto_label: usize,
    headings: Vec<(Arc<SourcePlan>, PacketId, super::heading::PlannedHeading)>,
    bibliography_keys: std::collections::HashSet<String>,
    observed_roots: Vec<ObservedSourceRoots>,
    entry_memberships: std::collections::HashSet<Location>,
}

impl Construction {
    fn new() -> Self {
        Self {
            intr: TagIntrospector::empty(),
            tags: Vec::new(),
            auto_label: 0,
            headings: Vec::new(),
            bibliography_keys: std::collections::HashSet::new(),
            observed_roots: Vec::new(),
            entry_memberships: std::collections::HashSet::new(),
        }
    }

    fn observe(&mut self, events: &[PlannedEvent]) -> SourceResult<()> {
        for event in events {
            let node = event
                .plan
                .node(event.packet)
                .ok_or_else(|| invalid_plan(Span::detached()))?;
            if matches!(
                node.role,
                PlanRole::OperationalTemplate | PlanRole::PrunedOriginal
            ) {
                return Err(invalid_plan(node.span));
            }
            if event.phase == PlanPhase::Root {
                if let Some(observed) = self
                    .observed_roots
                    .iter_mut()
                    .find(|observed| Arc::ptr_eq(&observed.plan, &event.plan))
                {
                    observed.packets.insert(event.packet);
                } else {
                    self.observed_roots.push(ObservedSourceRoots {
                        plan: event.plan.clone(),
                        packets: std::collections::HashSet::from([event.packet]),
                    });
                }
                if let Some(location) = selected_location(node, &self.bibliography_keys) {
                    self.intr.counters.observe_location(location);
                    self.intr.state.observe_location(location);
                }
            }
        }
        Ok(())
    }

    fn apply(&mut self, event: &PlannedEvent) -> SourceResult<()> {
        use crate::entities::{
            element_info::ElementInfo,
            element_kind::ElementKind,
            element_payload::ElementPayload,
            tag::Tag,
            value::{IntrospectedContent, Value},
        };
        let node = event
            .plan
            .node(event.packet)
            .ok_or_else(|| invalid_plan(Span::detached()))?;
        let environment = event
            .plan
            .environment(node.environment)
            .ok_or_else(|| invalid_plan(node.span))?;
        let chain = &environment.chain;
        let projected = if node.original {
            None
        } else {
            super::bibliography::project_reference_root(
                &node.content,
                &self.bibliography_keys,
            )
        };
        let semantic = projected.as_ref().unwrap_or(&node.content);
        if event.phase == PlanPhase::Root {
            let internal_context = matches!(&node.content, Content::ContextBlock(block) if
                crate::compiler::realize::reference_supplements::is_reference_supplement_producer(block));
            if (plan_owns_payload(semantic)
                && !(node.math && matches!(semantic, Content::Equation(_))))
                || internal_context
            {
                let location = selected_location(node, &self.bibliography_keys)
                    .ok_or_else(|| invalid_plan(node.span))?;
                let already = self.intr.elements.contains_key(&location);
                let unprepared_original = node.role == PlanRole::OriginalOnly
                    && matches!(&node.content, Content::Figure(figure) if figure.resolved_kind.is_none());
                if !unprepared_original && (!already || internal_context) {
                    super::walk::planned_root(
                        semantic,
                        Some(location),
                        chain,
                        node.label.as_ref(),
                        &mut self.intr,
                        &mut self.tags,
                    );
                    if let Content::Heading(heading) = &node.content {
                        let projection = super::heading::begin_planned(
                            heading,
                            location,
                            chain,
                            &self.intr,
                            &mut self.auto_label,
                        );
                        self.headings.push((
                            event.plan.clone(),
                            event.packet,
                            projection,
                        ));
                    }
                    let mut parent = node.source_parent;
                    while let Some(packet) = parent {
                        let source = event
                            .plan
                            .node(packet)
                            .ok_or_else(|| invalid_plan(node.span))?;
                        if let Some(parent_location) =
                            selected_location(source, &self.bibliography_keys)
                        {
                            if parent_location != location {
                                self.intr
                                    .parent_locations
                                    .entry(location)
                                    .or_insert(parent_location);
                            }
                            break;
                        }
                        parent = source.source_parent;
                    }
                }
            }
            if let Content::Label(label) = &node.content {
                if let Some((reference, location)) =
                    super::labelled::located_reference_original(&label.body)
                {
                    let label = Label(label.name.to_string());
                    let first = self.intr.labels.lookup(&label).is_none();
                    self.intr.labels.add(label.clone(), location);
                    self.intr.elements.entry(location).or_insert_with(|| {
                        IntrospectedContent::new(reference.clone(), None)
                    });
                    self.intr.counters.observe_location(location);
                    self.intr.state.observe_location(location);
                    if first {
                        self.intr.unreferencable_labels.insert(
                            label,
                            crate::entities::label_kind::UnreferencableKind::Other,
                        );
                    }
                }
            }
            if let Content::Styled(styled) = &node.content {
                for (key, value) in &styled.styles.delta().custom {
                    if key.as_str() != "outline.prefix-info" {
                        continue;
                    }
                    if let Value::Array(parts) = value {
                        if let [Value::Int(parent), Value::Int(level), Value::Length(inset)] =
                            parts.as_slice()
                        {
                            if let Ok(level) = usize::try_from(*level) {
                                self.intr.record_outline_prefix_width(
                                    *parent,
                                    level,
                                    inset.resolve_pt(0.0),
                                );
                            }
                        }
                    }
                }
            }
        } else {
            match &node.content {
                Content::Heading(heading) => {
                    if let Some(index) =
                        self.headings.iter().position(|(plan, packet, _)| {
                            Arc::ptr_eq(plan, &event.plan) && *packet == event.packet
                        })
                    {
                        let (_, _, projection) = self.headings.remove(index);
                        super::heading::finish_planned(
                            heading,
                            projection,
                            &mut self.intr,
                            &mut self.tags,
                        );
                    }
                }
                Content::Label(label) => {
                    if node.math && super::walk::is_equation_root(&label.body) {
                        return Ok(());
                    }
                    if super::labelled::located_reference_original(&label.body).is_none()
                    {
                        let target = event
                            .plan
                            .child(event.packet, PlanEdge::LabelBody)
                            .and_then(|child| {
                                first_semantic_location(
                                    &event.plan,
                                    child,
                                    &self.bibliography_keys,
                                    &self.observed_roots,
                                )
                            });
                        let name = Label(label.name.to_string());
                        if label.auto {
                            if let Some(location) = target {
                                let (resolved_text, figure_number) =
                                    super::labelled::compute_labelled(
                                        &self.intr,
                                        location,
                                        &label.body,
                                        node.lang.as_ref(),
                                    );
                                if resolved_text.is_some() || figure_number.is_some() {
                                    let info =
                                        ElementInfo::new(ElementPayload::Labelled {
                                            label: name.clone(),
                                            resolved_text,
                                            figure_number,
                                        });
                                    super::populate_intr_from_tag_start(
                                        &mut self.intr,
                                        &info,
                                        location,
                                    );
                                    self.tags.push(Tag::Start(location, Box::new(info)));
                                    self.tags.push(Tag::End(location, 0));
                                }
                            }
                        }
                        if target.is_none() && self.intr.labels.lookup(&name).is_none() {
                            self.intr.unreferencable_labels.insert(
                                name,
                                super::classify_unreferencable_body(&label.body),
                            );
                        }
                    }
                }
                Content::Outline(outline) => {
                    for location in &node.auxiliary_locations {
                        self.intr
                            .kind_index
                            .entry(ElementKind::Heading)
                            .or_default()
                            .push(*location);
                    }
                    self.intr.outline_elements.push((**outline).clone());
                }
                Content::OutlineEntry(_) => {
                    if let Some(location) = node.location {
                        if self.entry_memberships.insert(location) {
                            self.intr
                                .kind_index
                                .entry(ElementKind::OutlineEntry)
                                .or_default()
                                .push(location);
                        }
                    }
                }
                Content::Source(_)
                | Content::ShowRecipe(_)
                | Content::Realized(_)
                | Content::Located(_)
                | Content::Empty
                | Content::Text(_)
                | Content::Space
                | Content::SpaceFromNewline
                | Content::Parbreak
                | Content::Sequence(_)
                | Content::HtmlElem(_)
                | Content::HtmlFrame(_)
                | Content::Par { .. }
                | Content::Title(_)
                | Content::Strong(_)
                | Content::Emph(_)
                | Content::Raw(_)
                | Content::ListItem(_)
                | Content::EnumItem(_)
                | Content::Link(_)
                | Content::Equation(_)
                | Content::MathSequence(_)
                | Content::Symbol(_)
                | Content::MathIdent(_)
                | Content::MathText(_)
                | Content::MathFrac(_)
                | Content::MathAttach(_)
                | Content::MathPrimes(_)
                | Content::MathRoot(_)
                | Content::MathStretch(_)
                | Content::MathDelimited(_)
                | Content::MathLr(_)
                | Content::MathMid(_)
                | Content::MathOverbrace(_)
                | Content::MathUnderbrace(_)
                | Content::MathUnderbracket(_)
                | Content::MathUnderparen(_)
                | Content::MathUndershell(_)
                | Content::MathOverbracket(_)
                | Content::MathOverparen(_)
                | Content::MathOvershell(_)
                | Content::MathAlignPoint(_)
                | Content::Linebreak(_)
                | Content::MathMatrix(_)
                | Content::MathCases(_)
                | Content::MathAccent(_)
                | Content::MathCancel(_)
                | Content::MathUnderline(_)
                | Content::MathOverline(_)
                | Content::MathVec(_)
                | Content::MathClassOverride(_)
                | Content::MathLimitsOverride(_)
                | Content::MathUnderover(_)
                | Content::MathOp(_)
                | Content::MathStyled(_)
                | Content::Ref(_)
                | Content::CounterDisplay(_)
                | Content::CounterUpdate(_)
                | Content::Figure(_)
                | Content::FigureCaption(_)
                | Content::Image(_)
                | Content::Shape(_)
                | Content::Curve(_)
                | Content::Transform(_)
                | Content::Grid(_)
                | Content::GridHeader(_)
                | Content::GridFooter(_)
                | Content::GridCell(_)
                | Content::SetPage(_)
                | Content::PageRun(_)
                | Content::Align(_)
                | Content::Place(_)
                | Content::Flush(_)
                | Content::Styled(_)
                | Content::Divider(_)
                | Content::Terms(_)
                | Content::TermItem(_)
                | Content::Quote(_)
                | Content::Document { .. }
                | Content::Asset { .. }
                | Content::SmartQuote(_)
                | Content::PdfAttach(_)
                | Content::PdfArtifact(_)
                | Content::Underline(_)
                | Content::Strike(_)
                | Content::Overline(_)
                | Content::SmallCaps { .. }
                | Content::Pad(_)
                | Content::Hide(_)
                | Content::HSpace(_)
                | Content::VSpace(_)
                | Content::Pagebreak(_)
                | Content::Colbreak(_)
                | Content::Stack(_)
                | Content::Boxed(_)
                | Content::Block(_)
                | Content::TableCell(_)
                | Content::Bibliography(_)
                | Content::Cite(_)
                | Content::Footnote(_)
                | Content::FootnoteEntry(_)
                | Content::TableHeader(_)
                | Content::TableFooter(_)
                | Content::GridHLine(_)
                | Content::GridVLine(_)
                | Content::TableHLine(_)
                | Content::TableVLine(_)
                | Content::Table(_)
                | Content::Repeat(_)
                | Content::Columns(_)
                | Content::Metadata(_)
                | Content::State(_)
                | Content::StateUpdate(_)
                | Content::StateDisplay(_)
                | Content::CounterDisplayCallback(_)
                | Content::ContextBlock(_)
                | Content::Dynamic(_) => {}
            }
        }
        Ok(())
    }
}

/// Existing-only bootstrap after ordinary realization, never a physical result.
pub(in crate::compiler) fn provisional_body_index(
    content: &Content,
    ctx: &mut EvalContext,
) -> SourceResult<TagIntrospector> {
    let body = build_body(content, ctx, &StyleChain::default_chain(), false, false)?;
    let mut events = Vec::new();
    source_events(&body.plan, body.plan.root, &mut events)?;
    let mut construction = Construction::new();
    construction.observe(&events)?;
    for event in &events {
        construction.apply(event)?;
    }
    Ok(construction.intr)
}

impl SourcePlan {
    pub(in crate::compiler) fn valid_request_frontier(
        &self,
        template: PacketId,
        base: StyleEnvironmentId,
        overlays: &[PacketId],
    ) -> bool {
        if self
            .node(template)
            .is_none_or(|n| n.role != PlanRole::OperationalTemplate)
            || self.environment(base).is_none()
        {
            return false;
        }
        let declared = self.nodes.iter().any(|node| {
            node.children
                .iter()
                .any(|(edge, packet)| *packet == template && edge.is_template())
        });
        if !declared {
            return false;
        }
        let mut current = base;
        for packet in overlays {
            let Some(node) = self.node(*packet) else { return false };
            let origin = match node.content {
                Content::Styled(_) => EnvironmentOrigin::Styled(*packet),
                Content::ShowRecipe(_) => EnvironmentOrigin::CapturedRecipe(*packet),
                Content::Source(_)
                | Content::Realized(_)
                | Content::Located(_)
                | Content::Empty
                | Content::Text(_)
                | Content::Space
                | Content::SpaceFromNewline
                | Content::Parbreak
                | Content::Sequence(_)
                | Content::HtmlElem(_)
                | Content::HtmlFrame(_)
                | Content::Par { .. }
                | Content::Heading(_)
                | Content::Title(_)
                | Content::Strong(_)
                | Content::Emph(_)
                | Content::Raw(_)
                | Content::ListItem(_)
                | Content::EnumItem(_)
                | Content::Link(_)
                | Content::Equation(_)
                | Content::MathSequence(_)
                | Content::Symbol(_)
                | Content::MathIdent(_)
                | Content::MathText(_)
                | Content::MathFrac(_)
                | Content::MathAttach(_)
                | Content::MathPrimes(_)
                | Content::MathRoot(_)
                | Content::MathStretch(_)
                | Content::MathDelimited(_)
                | Content::MathLr(_)
                | Content::MathMid(_)
                | Content::MathOverbrace(_)
                | Content::MathUnderbrace(_)
                | Content::MathUnderbracket(_)
                | Content::MathUnderparen(_)
                | Content::MathUndershell(_)
                | Content::MathOverbracket(_)
                | Content::MathOverparen(_)
                | Content::MathOvershell(_)
                | Content::MathAlignPoint(_)
                | Content::Linebreak(_)
                | Content::MathMatrix(_)
                | Content::MathCases(_)
                | Content::MathAccent(_)
                | Content::MathCancel(_)
                | Content::MathUnderline(_)
                | Content::MathOverline(_)
                | Content::MathVec(_)
                | Content::MathClassOverride(_)
                | Content::MathLimitsOverride(_)
                | Content::MathUnderover(_)
                | Content::MathOp(_)
                | Content::MathStyled(_)
                | Content::Label(_)
                | Content::Ref(_)
                | Content::CounterDisplay(_)
                | Content::CounterUpdate(_)
                | Content::Outline(_)
                | Content::OutlineEntry(_)
                | Content::Figure(_)
                | Content::FigureCaption(_)
                | Content::Image(_)
                | Content::Shape(_)
                | Content::Curve(_)
                | Content::Transform(_)
                | Content::Grid(_)
                | Content::GridHeader(_)
                | Content::GridFooter(_)
                | Content::GridCell(_)
                | Content::SetPage(_)
                | Content::PageRun(_)
                | Content::Align(_)
                | Content::Place(_)
                | Content::Flush(_)
                | Content::Divider(_)
                | Content::Terms(_)
                | Content::TermItem(_)
                | Content::Quote(_)
                | Content::Document { .. }
                | Content::Asset { .. }
                | Content::SmartQuote(_)
                | Content::PdfAttach(_)
                | Content::PdfArtifact(_)
                | Content::Underline(_)
                | Content::Strike(_)
                | Content::Overline(_)
                | Content::SmallCaps { .. }
                | Content::Pad(_)
                | Content::Hide(_)
                | Content::HSpace(_)
                | Content::VSpace(_)
                | Content::Pagebreak(_)
                | Content::Colbreak(_)
                | Content::Stack(_)
                | Content::Boxed(_)
                | Content::Block(_)
                | Content::TableCell(_)
                | Content::Bibliography(_)
                | Content::Cite(_)
                | Content::Footnote(_)
                | Content::FootnoteEntry(_)
                | Content::TableHeader(_)
                | Content::TableFooter(_)
                | Content::GridHLine(_)
                | Content::GridVLine(_)
                | Content::TableHLine(_)
                | Content::TableVLine(_)
                | Content::Table(_)
                | Content::Repeat(_)
                | Content::Columns(_)
                | Content::Metadata(_)
                | Content::State(_)
                | Content::StateUpdate(_)
                | Content::StateDisplay(_)
                | Content::CounterDisplayCallback(_)
                | Content::ContextBlock(_)
                | Content::Dynamic(_) => return false,
            };
            let Some((index, _)) =
                self.environments.iter().enumerate().find(|(_, environment)| {
                    environment.parent == Some(current) && environment.origin == origin
                })
            else {
                return false;
            };
            current = StyleEnvironmentId(index);
        }
        true
    }
}

impl PlanEdge {
    fn is_template(self) -> bool {
        match self {
            Self::PageRunBackground
            | Self::PageRunHeader
            | Self::PageRunFooter
            | Self::PageRunForeground
            | Self::SetPageBackground
            | Self::SetPageHeader
            | Self::SetPageFooter
            | Self::SetPageForeground => true,
            Self::SourceBody
            | Self::LocatedBody
            | Self::StyledBody
            | Self::ShowRecipeBody
            | Self::RealizedOriginal
            | Self::RealizedVisual
            | Self::SequencePart(_)
            | Self::ParagraphBody
            | Self::HeadingBody
            | Self::TitleBody
            | Self::StrongBody
            | Self::EmphBody
            | Self::SmallCapsBody
            | Self::LabelBody
            | Self::ListItemBody
            | Self::EnumItemBody
            | Self::EquationBody
            | Self::FigureBody
            | Self::FigureCaption
            | Self::FigureSelectedSupplement
            | Self::FigureCaptionBody
            | Self::FigureCaptionSeparator
            | Self::PreparedReferenceSupplement
            | Self::CitationSupplement
            | Self::GridCell(_)
            | Self::GridHeader
            | Self::GridFooter
            | Self::GridCellBody
            | Self::GridHeaderBody
            | Self::GridFooterBody
            | Self::TableResolvedCell(_)
            | Self::TableRawChild(_)
            | Self::TableHeader
            | Self::TableFooter
            | Self::TableCaption
            | Self::CellSelectedVisual
            | Self::TableHeaderBody
            | Self::TableFooterBody
            | Self::PageRunBody
            | Self::HtmlFrameBody
            | Self::TermsItem(_)
            | Self::TermItemTerm
            | Self::TermItemDescription
            | Self::QuoteBody
            | Self::QuoteAttribution
            | Self::StackChild(_)
            | Self::AlignBody
            | Self::PlaceBody
            | Self::TransformBody
            | Self::PdfArtifactBody
            | Self::UnderlineBody
            | Self::StrikeBody
            | Self::OverlineBody
            | Self::PadBody
            | Self::HideBody
            | Self::BoxedBody
            | Self::BlockBody
            | Self::RepeatBody
            | Self::ColumnsBody
            | Self::BibliographyTitle
            | Self::FootnoteBody
            | Self::FootnoteEntryNote
            | Self::FootnoteEntrySeparator
            | Self::OutlineRealizedBody
            | Self::OutlineEntryBody => false,
        }
    }
}

// All event Arcs remain alive throughout this invocation. These addresses are
// exact allocation identity only; they are never dereferenced or retained.
fn planned_events_are_unique(events: &[PlannedEvent]) -> bool {
    event_keys_are_unique(
        events
            .iter()
            .map(|event| (Arc::as_ptr(&event.plan), event.packet, event.phase)),
    )
}

fn event_keys_are_unique<K: Eq + std::hash::Hash>(
    mut keys: impl Iterator<Item = K>,
) -> bool {
    let mut seen = FxHashSet::with_capacity_and_hasher(
        keys.size_hint().0,
        Default::default(),
    );
    keys.all(|key| seen.insert(key))
}

fn scheduled_document(
    input: &PageRunningIndexInput<'_>,
) -> SourceResult<Vec<PlannedEvent>> {
    use crate::compiler::layout::page_running_requests::TraceSection;
    match (input.target, input.running) {
        (EvalTarget::Html, None) => {
            let mut events = Vec::new();
            source_events(&input.body.plan, input.body.plan.root, &mut events)?;
            Ok(events)
        }
        (EvalTarget::Paged, Some((trace, running))) => {
            if !Arc::ptr_eq(&trace.data.source, &input.body.plan) {
                return Err(invalid_plan(Span::detached()));
            }
            let mut plans = vec![input.body.plan.clone()];
            for resolution in running.resolutions() {
                resolution.validate_owner()?;
                let body_owner = input
                    .body
                    .plan
                    .allocation_owner
                    .as_ref()
                    .ok_or_else(|| invalid_plan(resolution.request.span()))?;
                let contribution_owner =
                    resolution
                        .body
                        .plan
                        .allocation_owner
                        .as_ref()
                        .ok_or_else(|| invalid_plan(resolution.request.span()))?;
                if !Arc::ptr_eq(body_owner, contribution_owner) {
                    return Err(invalid_plan(resolution.request.span()));
                }
                if !Arc::ptr_eq(&resolution.request.authority.source, &input.body.plan)
                    || !trace
                        .data
                        .requests
                        .iter()
                        .any(|request| request.matches(&resolution.request))
                {
                    return Err(invalid_plan(resolution.request.span()));
                }
                plans.push(resolution.body.plan.clone());
            }
            for request in &trace.data.requests {
                if !running
                    .resolutions()
                    .iter()
                    .any(|resolution| request.matches(&resolution.request))
                {
                    return Err(invalid_plan(request.span()));
                }
            }
            let mut ordered: Vec<(usize, u8, usize, PlannedEvent)> = Vec::new();
            for (index, admission) in trace.ordered().into_iter().enumerate() {
                if !plans.iter().any(|plan| Arc::ptr_eq(plan, &admission.plan))
                    || admission.plan.node(admission.packet).is_none()
                {
                    return Err(invalid_plan(Span::detached()));
                }
                let section = match admission.section {
                    TraceSection::Background => 0,
                    TraceSection::Header => 1,
                    TraceSection::Body => 2,
                    TraceSection::Footer => 3,
                    TraceSection::Foreground => 4,
                };
                ordered.push((
                    admission.page.get(),
                    section,
                    index,
                    PlannedEvent {
                        plan: admission.plan.clone(),
                        packet: admission.packet,
                        phase: admission.phase,
                    },
                ));
            }
            // A Pending discovery transcript already proves the selected physical
            // slot. Its newly bound source contribution can construct the next
            // semantic candidate before it has ever been painted. Geometry is
            // still taken exclusively from the later actual sealed projection.
            for resolution in running.resolutions() {
                let plan = &resolution.body.plan;
                if !ordered.iter().any(|(_, _, _, event)| Arc::ptr_eq(&event.plan, plan))
                {
                    if trace.data.complete {
                        return Err(invalid_plan(resolution.request.span()));
                    }
                    let mut events = Vec::new();
                    source_events(plan, plan.root, &mut events)?;
                    let section = match resolution.key().slot {
                        crate::compiler::layout::PageRunningSlot::Background => 0,
                        crate::compiler::layout::PageRunningSlot::Header => 1,
                        crate::compiler::layout::PageRunningSlot::Footer => 3,
                        crate::compiler::layout::PageRunningSlot::Foreground => 4,
                    };
                    for (index, event) in events.into_iter().enumerate() {
                        ordered.push((
                            resolution.key().page.get(),
                            section,
                            index,
                            event,
                        ));
                    }
                }
            }
            ordered
                .sort_by_key(|(page, section, ordinal, _)| (*page, *section, *ordinal));
            let events: Vec<_> =
                ordered.into_iter().map(|(_, _, _, event)| event).collect();
            for plan in plans {
                for (index, _) in plan.nodes.iter().enumerate() {
                    for phase in [PlanPhase::Root, PlanPhase::PostChildren] {
                        let packet = PacketId(index);
                        if plan.required(packet, phase)
                            && !events.iter().any(|event| {
                                Arc::ptr_eq(&event.plan, &plan)
                                    && event.packet == packet
                                    && event.phase == phase
                            })
                        {
                            return Err(invalid_plan(plan.nodes[index].span));
                        }
                    }
                }
            }
            if !planned_events_are_unique(&events) {
                return Err(invalid_plan(Span::detached()));
            }
            if !trace.repeat_activity_valid() {
                return Err(invalid_plan(Span::detached()));
            }
            Ok(events
                .into_iter()
                .filter(|event| {
                    trace.source_contribution_active(&event.plan, event.packet)
                })
                .collect())
        }
        (EvalTarget::Html, Some(_))
        | (EvalTarget::Paged, None)
        | (EvalTarget::Bundle, None | Some(_)) => Err(invalid_plan(Span::detached())),
    }
}

/// Build one source-derived semantic candidate for all actual document targets.
pub fn introspect_with_page_running(
    documents: &[PageRunningIndexInput<'_>],
    features: Features,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<(TagIntrospector, Vec<LocatorSeed>)> {
    use crate::entities::{
        counter::CounterKey, counter_update::CounterUpdate, page_store::PageStore,
        sealed_positions::SealedPositions,
    };
    use std::collections::{HashMap, HashSet};
    use std::num::NonZeroUsize;
    // All target/authority metadata is checked before any runtime callback.
    let mut schedules = documents
        .iter()
        .map(scheduled_document)
        .collect::<SourceResult<Vec<_>>>()?;
    let keys = active_bibliography_keys(&schedules);
    for (input, events) in documents.iter().zip(&mut schedules) {
        complete_conditional_sources(input, events, &keys)?;
    }
    let mut construction = Construction::new();
    construction.bibliography_keys = keys;
    let owner = crate::compiler::realize::context_occurrences::allocation_owner(ctx)
        .ok_or_else(|| invalid_plan(Span::detached()))?;
    for (input, events) in documents.iter().zip(&schedules) {
        let plan_owner = input
            .body
            .plan
            .allocation_owner
            .as_ref()
            .ok_or_else(|| invalid_plan(Span::detached()))?;
        if !Arc::ptr_eq(plan_owner, &owner) {
            return Err(invalid_plan(Span::detached()));
        }
        let (start, boundary) = input
            .body
            .plan
            .document_boundary
            .ok_or_else(|| invalid_plan(Span::detached()))?;
        if input.body.plan.allocation_start != start
            || crate::entities::locator::Locator::from_seed(start).next() != boundary
        {
            return Err(invalid_plan(Span::detached()));
        }
        construction.intr.counters.observe_location(boundary);
        construction.intr.state.observe_location(boundary);
        construction.observe(events)?;
    }
    let mut starts = Vec::with_capacity(documents.len());
    let mut body_tags = Vec::new();
    let mut page_documents = Vec::new();
    let mut positions = HashMap::new();
    let mut physical_offset = 0;
    let mut numberings = Vec::new();
    let mut logical_numbers = Vec::new();
    let mut numbering_spans = Vec::new();
    let mut visible_numberings = Vec::new();
    let mut reference_numberings = Vec::new();
    let mut supplements = Vec::new();
    for (input, events) in documents.iter().zip(&schedules) {
        let (start, boundary) = input
            .body
            .plan
            .document_boundary
            .ok_or_else(|| invalid_plan(Span::detached()))?;
        starts.push(start);
        construction.intr.counters.apply_at(
            CounterKey::Page,
            CounterUpdate::Set(vec![1]),
            boundary,
        );
        let first_tag = construction.tags.len();
        let mut locations = vec![boundary];
        let mut seen = HashSet::from([boundary]);
        for event in events {
            let node = event
                .plan
                .node(event.packet)
                .ok_or_else(|| invalid_plan(Span::detached()))?;
            if event.phase == PlanPhase::Root {
                if let Some(location) =
                    selected_location(node, &construction.bibliography_keys)
                {
                    if seen.insert(location) {
                        locations.push(location);
                    }
                }
            }
            construction.apply(event)?;
        }
        construction
            .intr
            .counters
            .record_document_locations(locations.clone());
        body_tags.push((boundary, first_tag..construction.tags.len()));
        let total = if let Some((trace, _)) = input.running {
            let total = trace
                .data
                .pages
                .total_pages()
                .ok_or_else(|| invalid_plan(Span::detached()))?;
            for event in events {
                if let Some(location) = event.plan.node(event.packet).and_then(|node| {
                    selected_location(node, &construction.bibliography_keys)
                }) {
                    if let Some(mut position) = trace.data.positions.position_of(location)
                    {
                        position.page =
                            NonZeroUsize::new(position.page.get() + physical_offset)
                                .ok_or_else(|| invalid_plan(Span::detached()))?;
                        positions.entry(location).or_insert(position);
                    }
                }
            }
            for page in 1..=total.get() {
                let page = NonZeroUsize::new(page).expect("physical page is nonzero");
                numberings.push(trace.data.pages.numbering_for_page(page).cloned());
                logical_numbers.push(
                    trace.data.pages.logical_number_for_page(page).unwrap_or(page.get()),
                );
                numbering_spans.push(
                    trace
                        .data
                        .pages
                        .numbering_span_for_page(page)
                        .unwrap_or(Span::detached()),
                );
                visible_numberings
                    .push(trace.data.pages.visible_numbering_for_page(page).cloned());
                reference_numberings
                    .push(trace.data.pages.reference_numbering_for_page(page).cloned());
                supplements.push(
                    trace
                        .data
                        .pages
                        .supplement_for_page(page)
                        .cloned()
                        .unwrap_or(Content::Empty),
                );
            }
            physical_offset += total.get();
            Some(total)
        } else {
            None
        };
        page_documents.push((locations, total));
    }
    construction
        .intr
        .inject_positions(SealedPositions::from_runtime(positions));
    if let Some(total) = NonZeroUsize::new(physical_offset) {
        construction.intr.inject_pages(PageStore::from_realized(
            total,
            numberings,
            logical_numbers,
            numbering_spans,
            visible_numberings,
            reference_numberings,
            supplements,
        ));
    }
    super::outline_entries::materialize_outline_entries_with(
        &mut construction.intr,
        || {
            crate::compiler::realize::context_occurrences::next_with_seed(ctx)
                .expect("validated shared retained index frame remains installed")
                .1
        },
    );
    super::from_tags::apply_bundle_counter_funcs(
        &construction.tags,
        &body_tags,
        &mut construction.intr,
        engine,
        ctx,
    )?;
    super::from_tags::apply_state_displays_checked(
        &construction.tags,
        &mut construction.intr,
        engine,
        ctx,
    )?;
    super::from_tags::apply_counter_displays(
        &construction.tags,
        &mut construction.intr,
        engine,
        ctx,
    );
    super::from_tags::apply_equation_numberings(
        &construction.tags,
        &mut construction.intr,
        engine,
        ctx,
    )?;
    super::from_tags::apply_equation_supplements(
        &construction.tags,
        &mut construction.intr,
        engine,
        ctx,
    )?;
    super::from_tags::apply_heading_supplements(&mut construction.intr, engine, ctx)?;
    super::from_tags::realize_equation_elements(
        &construction.tags,
        &mut construction.intr,
    );
    let mut demands = Vec::new();
    for ((input, events), _) in documents.iter().zip(&schedules).zip(&page_documents) {
        if input.target != EvalTarget::Paged {
            continue;
        }
        for event in events {
            if event.phase != PlanPhase::Root {
                continue;
            }
            let node = event
                .plan
                .node(event.packet)
                .ok_or_else(|| invalid_plan(Span::detached()))?;
            if node.original
                || super::bibliography::project_reference_root(
                    &node.content,
                    &construction.bibliography_keys,
                )
                .is_some()
            {
                continue;
            }
            let chain = &event
                .plan
                .environment(node.environment)
                .ok_or_else(|| invalid_plan(node.span))?
                .chain;
            super::figure_demands::collect_planned_root(
                &node.content,
                &construction.intr,
                chain,
                node.location,
                node.span,
                &mut demands,
            )?;
        }
    }
    let mut observer = EvalContext::new();
    observer.target = EvalTarget::Paged;
    observer.features = features;
    super::figure_counters::finish_demands(
        demands,
        &mut construction.intr,
        engine,
        &observer,
    )?;
    super::from_tags::realize_page_counter_snapshot(
        &construction.tags,
        &page_documents,
        &mut construction.intr,
        engine,
        &observer,
    )?;
    Ok((construction.intr, starts))
}

#[cfg(test)]
mod source_plan_tests {
    use super::*;
    use crate::compiler::realize::context_occurrences::ContextOccurrenceArena;
    use crate::entities::locator::Locator;

    #[test]
    fn page_running_document_boundary_retains_seed_across_retry_and_is_not_first_element()
    {
        let origin = Content::heading(1, Content::text("body"));
        let mut arena = ContextOccurrenceArena::new(&origin);
        let mut frame = arena.frame();
        let mut ctx = EvalContext::new();
        let first = arena
            .evaluate(&mut frame, &mut ctx, |ctx| prepare_page_running_body(&origin, ctx))
            .unwrap();
        let (start, boundary) = first.plan.document_boundary.expect("document boundary");
        assert_eq!(Locator::from_seed(start).next(), boundary);
        assert_ne!(first_root_location(&first.plan, first.plan.root), Some(boundary));
        let retry = arena
            .evaluate(&mut frame, &mut ctx, |ctx| prepare_page_running_body(&origin, ctx))
            .unwrap();
        assert_eq!(retry.plan.document_boundary, first.plan.document_boundary);
        assert_eq!(retry.content(), &origin);
        assert!(Arc::ptr_eq(
            first.plan.allocation_owner.as_ref().unwrap(),
            retry.plan.allocation_owner.as_ref().unwrap()
        ));
        assert_eq!(
            first_root_location(&retry.plan, retry.plan.root),
            first_root_location(&first.plan, first.plan.root)
        );
    }

    #[test]
    fn page_running_contribution_has_no_document_boundary_and_foreign_arena_has_distinct_authority(
    ) {
        let origin = Content::text("source");
        let mut ctx = EvalContext::new();
        let mut arena = ContextOccurrenceArena::new(&origin);
        let mut frame = arena.frame();
        let contribution = arena
            .evaluate(&mut frame, &mut ctx, |ctx| {
                prepare_with_root_chain(&origin, ctx, &StyleChain::default_chain())
            })
            .unwrap();
        assert!(contribution.plan.document_boundary.is_none());
        let mut foreign = ContextOccurrenceArena::new(&origin);
        let mut foreign_frame = foreign.frame();
        let other = foreign
            .evaluate(&mut foreign_frame, &mut ctx, |ctx| {
                prepare_page_running_body(&origin, ctx)
            })
            .unwrap();
        assert!(!Arc::ptr_eq(
            contribution.plan.allocation_owner.as_ref().unwrap(),
            other.plan.allocation_owner.as_ref().unwrap()
        ));
        assert!(prepare_page_running_body(&origin, &mut ctx).is_err());
    }

    #[test]
    fn page_running_frontier_distinguishes_global_singleton_from_local_styles_and_block()
    {
        use crate::entities::{layout_types::Length, sides::Sides, value::Value};
        let global = crate::entities::style::Styles::new()
            .push_custom("eval.flow.scope", Value::Bool(true));
        let local = crate::entities::style::Styles::new()
            .push_custom("text.size", Value::Length(Length::pt(10.0)));
        let origin = Content::sequence(vec![
            Content::styled(Box::new(Content::text("GLOBAL SINGLETON")), global.clone()),
            Content::styled(Box::new(Content::text("LOCAL INLINE")), local),
            Content::block(
                Content::styled(Box::new(Content::text("LOCAL SET")), global),
                None,
                None,
                Sides::uniform(Length::ZERO),
                true,
            ),
        ]);
        let mut arena = ContextOccurrenceArena::new(&origin);
        let mut frame = arena.frame();
        let mut ctx = EvalContext::new();
        let body = arena
            .evaluate(&mut frame, &mut ctx, |ctx| prepare_page_running_body(&origin, ctx))
            .unwrap();
        let root = body.plan.root;
        let global = body.plan.child(root, PlanEdge::SequencePart(0)).unwrap();
        let local = body.plan.child(root, PlanEdge::SequencePart(1)).unwrap();
        let block = body.plan.child(root, PlanEdge::SequencePart(2)).unwrap();
        assert!(body
            .plan
            .page_frontier(body.plan.child(global, PlanEdge::StyledBody).unwrap()));
        assert!(!body
            .plan
            .page_frontier(body.plan.child(local, PlanEdge::StyledBody).unwrap()));
        let local_set = body.plan.child(block, PlanEdge::BlockBody).unwrap();
        assert!(!body.plan.page_frontier(local_set));
        assert!(!body
            .plan
            .page_frontier(body.plan.child(local_set, PlanEdge::StyledBody).unwrap()));
    }

    #[test]
    fn page_running_grid_table_actual_paint_has_complete_unique_source_phases() {
        use crate::compiler::layout::{
            layout_with_page_running, FixedMetrics, PageRunningLayoutInputs,
        };
        use crate::entities::image_sizer::NullImageSizer;
        use crate::entities::{
            elements::grid::GridElem,
            layout_types::{Length, TrackSizing},
            sides::Sides,
        };
        let cells = vec![
            Content::heading(1, Content::text("FIRST")),
            Content::heading(1, Content::text("SECOND")),
        ];
        let grid = Content::Grid(Arc::new(GridElem { spec: None,
            source_order: None,
            columns: vec![TrackSizing::Auto],
            rows: vec![],
            cells: cells.clone(),
            hlines: vec![],
            vlines: vec![],
            gutter: None,
            align: None,
            inset: Sides::uniform(Length::ZERO),
            header: None,
            footer: None,
            stroke: None,
            fill: None,
        }));
        let table = Content::table(vec![TrackSizing::Auto], vec![], cells);
        for (name, origin) in [("Grid", grid), ("Table", table)] {
            let mut arena = ContextOccurrenceArena::new(&origin);
            let mut frame = arena.frame();
            let mut ctx = EvalContext::new();
            let body = arena
                .evaluate(&mut frame, &mut ctx, |ctx| {
                    prepare_page_running_body(&origin, ctx)
                })
                .unwrap();
            let empty = SealedPageRunning::new(vec![]).unwrap();
            let outcome = layout_with_page_running(
                &body,
                source_only_index(&body),
                FixedMetrics,
                NullImageSizer,
                11.0,
                PageRunningLayoutInputs {
                    math: None,
                    par_line: None,
                    running: Some(&empty),
                },
            )
            .into_result();
            let trace = match outcome {
                Ok(document) => document.trace,
                Err(pending) => pending.trace,
            };
            let mut missing = Vec::new();
            for (index, node) in body.plan.nodes.iter().enumerate() {
                for phase in [PlanPhase::Root, PlanPhase::PostChildren] {
                    if body.plan.required(PacketId(index), phase)
                        && !trace.data.admissions.iter().any(|event| {
                            event.packet == PacketId(index) && event.phase == phase
                        })
                    {
                        missing.push(format!(
                            "packet {index} {phase:?}: {:?}",
                            node.content
                        ));
                    }
                }
            }
            let mut duplicate = Vec::new();
            for (index, event) in trace.data.admissions.iter().enumerate() {
                if trace.data.admissions[..index].iter().any(|prior| {
                    prior.packet == event.packet
                        && prior.phase == event.phase
                        && Arc::ptr_eq(&prior.plan, &event.plan)
                }) {
                    duplicate.push(format!("{:?} {:?}", event.packet, event.phase));
                }
            }
            assert!(missing.is_empty(), "{name}: missing source phases {missing:?}");
            assert!(
                duplicate.is_empty(),
                "{name}: duplicate source phases {duplicate:?}"
            );
            assert!(
                trace.data.complete,
                "{name}: exact selected source routes must close the actual pass"
            );
            assert!(
                scheduled_document(&PageRunningIndexInput {
                    body: &body,
                    target: EvalTarget::Paged,
                    running: Some((&trace, &empty))
                })
                .is_ok(),
                "{name}: actual transcript must construct an incremental index"
            );
        }
    }

    struct PlanTestWorld {
        source: crate::entities::source::Source,
        library: crate::entities::world_types::Library,
        book: crate::entities::font_book::FontBook,
    }
    impl crate::contracts::world::World for PlanTestWorld {
        fn library(&self) -> &crate::entities::world_types::Library {
            &self.library
        }
        fn book(&self) -> &crate::entities::font_book::FontBook {
            &self.book
        }
        fn main(&self) -> crate::entities::file_id::FileId {
            self.source.id()
        }
        fn source(
            &self,
            _: crate::entities::file_id::FileId,
        ) -> crate::entities::world_types::FileResult<crate::entities::source::Source>
        {
            Ok(self.source.clone())
        }
        fn file(
            &self,
            _: crate::entities::file_id::FileId,
        ) -> crate::entities::world_types::FileResult<crate::entities::world_types::Bytes>
        {
            Err(crate::entities::world_types::FileError::NotFound)
        }
        fn font(&self, _: usize) -> Option<crate::entities::world_types::Font> {
            None
        }
        fn today(
            &self,
            _: Option<crate::entities::duration::Duration>,
        ) -> Option<crate::entities::world_types::Datetime> {
            None
        }
    }

    struct TableTrace {
        body: PageRunningBody,
        trace: PageRunningTrace,
        store: SealedPageRunning,
        document: Option<crate::entities::layout_types::PagedDocument>,
    }

    fn prepared_table_trace(source: &str) -> TableTrace {
        use crate::compiler::layout::{
            layout_with_page_running, FixedMetrics, PageRunningLayoutInputs,
        };
        use crate::contracts::world::World;
        use crate::entities::{
            image_sizer::NullImageSizer,
            world_types::{Library, Route, Routines, Sink, Traced},
        };
        use comemo::Track;
        let source = crate::entities::source::Source::detached(source)
            .expect("valid Source test fixture");
        let world = PlanTestWorld {
            source,
            library: Library::new(),
            book: crate::entities::font_book::FontBook::new(),
        };
        let route = Route::root();
        let mut sink = Sink::new();
        let traced = Traced::default();
        let registry = crate::entities::element_registry::ElementRegistry::new();
        let module = crate::compiler::eval::eval(
            &Routines::new(),
            &world,
            traced.track(),
            sink.track_mut(),
            route.track(),
            &world.source,
            &registry,
        )
        .unwrap();
        let origin = module.content().unwrap().clone();
        let mut ctx = EvalContext::new();
        let mut styles = StyleChain::default_chain();
        let mut rules = Arc::from([]);
        let mut guards = Vec::new();
        let mut tracked_sink = sink.track_mut();
        let mut engine = Engine {
            world: &world,
            font_metrics: &FixedMetrics,
            route: route.track(),
            styles: &mut styles,
            show_rules: &mut rules,
            active_guards: &mut guards,
            current_file: world.main(),
            sink: &mut tracked_sink,
        };
        let intr = super::super::introspect_with_runtime(&origin, &mut engine, &mut ctx)
            .unwrap();
        let mut arena = ContextOccurrenceArena::new(&origin);
        let mut frame = arena.frame();
        let body = arena
            .evaluate(&mut frame, &mut ctx, |ctx| prepare_page_running_body(&origin, ctx))
            .unwrap();
        let empty = SealedPageRunning::new(vec![]).unwrap();
        let outcome = layout_with_page_running(
            &body,
            intr,
            FixedMetrics,
            NullImageSizer,
            11.0,
            PageRunningLayoutInputs { math: None, par_line: None, running: Some(&empty) },
        )
        .into_result();
        let (trace, document) = match outcome {
            Ok(document) => (document.trace, Some(document.document)),
            Err(pending) => (pending.trace, None),
        };
        TableTrace { body, trace, store: empty, document }
    }

    #[test]
    fn page_running_prepared_table_figure_captioned_paint_has_unique_source_phases() {
        let TableTrace { body, trace, store: empty, .. } = prepared_table_trace(
            "#table(columns:1,[#figure([FIRST],caption:[C1]) <first>],[#figure([SECOND],caption:[C2]) <second>])"
        );
        let mut missing = Vec::new();
        for (index, node) in body.plan.nodes.iter().enumerate() {
            for phase in [PlanPhase::Root, PlanPhase::PostChildren] {
                if body.plan.required(PacketId(index), phase)
                    && !trace.data.admissions.iter().any(|event| {
                        event.packet == PacketId(index) && event.phase == phase
                    })
                {
                    missing.push(format!(
                        "packet {index} {phase:?} role {:?}: {:?}",
                        node.role, node.content
                    ));
                }
            }
        }
        let mut duplicate = Vec::new();
        for (index, event) in trace.data.admissions.iter().enumerate() {
            if trace.data.admissions[..index].iter().any(|prior| {
                prior.packet == event.packet
                    && prior.phase == event.phase
                    && Arc::ptr_eq(&prior.plan, &event.plan)
            }) {
                duplicate.push(format!(
                    "{:?} {:?}: {:?}",
                    event.packet,
                    event.phase,
                    body.plan.node(event.packet).unwrap().content
                ));
            }
        }
        assert!(missing.is_empty(), "prepared Table missing source phases {missing:?}");
        assert!(
            duplicate.is_empty(),
            "prepared Table duplicate source phases {duplicate:?}"
        );
        assert!(trace.data.complete, "prepared Table must finish source coverage");
        assert!(
            scheduled_document(&PageRunningIndexInput {
                body: &body,
                target: EvalTarget::Paged,
                running: Some((&trace, &empty))
            })
            .is_ok(),
            "prepared Table transcript must construct the incremental index"
        );
    }

    #[test]
    fn issue_8935_repeated_table_header_has_one_source_dispatch() {
        let TableTrace { body, trace, store, document } = prepared_table_trace(
            "#set page(width:200pt,height:150pt,margin:10pt)\n#set text(size:10pt)\n#table(columns:1,rows:20pt,inset:2pt,table.header([H#footnote[NOTE]<note-foo>]),..range(10).map(i=>[body #i]))"
        );
        let document = document.expect("the fixed-height fixture must paint");
        assert!(document.pages.len() > 1);
        assert!(
            document.plain_text().matches('H').count() > 1,
            "the header must actually repeat"
        );
        let mut duplicate = Vec::new();
        for (index, event) in trace.data.admissions.iter().enumerate() {
            if trace.data.admissions[..index].iter().any(|prior| {
                Arc::ptr_eq(&prior.plan, &event.plan)
                    && prior.packet == event.packet
                    && prior.phase == event.phase
            }) {
                duplicate.push(format!("{:?} {:?}", event.packet, event.phase));
            }
        }
        assert!(
            duplicate.is_empty(),
            "physical header copies readmitted source: {duplicate:?}"
        );
        assert!(trace.data.complete, "required raw source coverage must stay complete");
        assert!(
            scheduled_document(&PageRunningIndexInput {
                body: &body,
                target: EvalTarget::Paged,
                running: Some((&trace, &store)),
            })
            .is_ok(),
            "strict source construction must accept the actual transcript"
        );
    }

    #[test]
    fn page_running_captured_table_first_pending_keeps_every_body_phase() {
        use crate::compiler::layout::{
            layout_with_page_running, FixedMetrics, PageRunningLayoutInputs,
        };
        use crate::contracts::world::World;
        use crate::entities::{
            image_sizer::NullImageSizer,
            world_types::{Library, Route, Routines, Sink, Traced},
        };
        use comemo::Track;
        let source = crate::entities::source::Source::detached(
            r#"#set page(width:280pt,height:320pt,margin:60pt,header:[#figure([HEADER],caption:[HC]) <header>])
#table(columns:1,[#figure([FIRST],caption:[C1]) <first>],[#figure([SECOND],caption:[C2]) <second>])
#context {
  assert.eq(query(figure).len(), 3)
  assert.eq(query(figure).map(it=>it.location()).dedup().len(), 3)
  assert.eq(query(<header>).map(it=>counter(figure).at(it.location())), ((1,),))
  assert.eq(query(<first>).map(it=>counter(figure).at(it.location())), ((2,),))
  assert.eq(query(<second>).map(it=>counter(figure).at(it.location())), ((3,),))
  assert.eq(counter(figure).final(), (3,))
}"#,
        ).expect("valid Source test fixture");
        let world = PlanTestWorld {
            source,
            library: Library::new(),
            book: crate::entities::font_book::FontBook::new(),
        };
        let route = Route::root();
        let mut sink = Sink::new();
        let traced = Traced::default();
        let registry = crate::entities::element_registry::ElementRegistry::new();
        let module = crate::compiler::eval::eval(
            &Routines::new(),
            &world,
            traced.track(),
            sink.track_mut(),
            route.track(),
            &world.source,
            &registry,
        )
        .unwrap();
        let origin = module.content().unwrap().clone();
        assert!(crate::compiler::realize::page_running::needs_capture(&origin));
        let mut ctx = EvalContext::new();
        let mut styles = StyleChain::default_chain();
        let mut rules = Arc::from([]);
        let mut guards = Vec::new();
        let mut tracked_sink = sink.track_mut();
        let mut engine = Engine {
            world: &world,
            font_metrics: &FixedMetrics,
            route: route.track(),
            styles: &mut styles,
            show_rules: &mut rules,
            active_guards: &mut guards,
            current_file: world.main(),
            sink: &mut tracked_sink,
        };
        let intr = super::super::introspect_with_runtime(&origin, &mut engine, &mut ctx)
            .unwrap();
        let mut arena = ContextOccurrenceArena::new(&origin);
        let mut frame = arena.frame();
        let body = arena
            .evaluate(&mut frame, &mut ctx, |ctx| prepare_page_running_body(&origin, ctx))
            .unwrap();
        let outcome = layout_with_page_running(
            &body,
            intr,
            FixedMetrics,
            NullImageSizer,
            11.0,
            PageRunningLayoutInputs { math: None, par_line: None, running: None },
        )
        .into_result();
        let pending = match outcome {
            Ok(_) => {
                panic!("the first physical pass must discover the uncaptured header")
            }
            Err(pending) => pending,
        };
        assert_eq!(pending.running.len(), 1);
        let trace = pending.trace;
        let mut missing = Vec::new();
        for (index, node) in body.plan.nodes.iter().enumerate() {
            for phase in [PlanPhase::Root, PlanPhase::PostChildren] {
                if body.plan.required(PacketId(index), phase)
                    && !trace.data.admissions.iter().any(|event| {
                        event.packet == PacketId(index) && event.phase == phase
                    })
                {
                    missing.push(format!(
                        "packet {index} {phase:?} role {:?}: {:?}",
                        node.role, node.content
                    ));
                }
            }
        }
        let mut duplicate = Vec::new();
        for (index, event) in trace.data.admissions.iter().enumerate() {
            if trace.data.admissions[..index].iter().any(|prior| {
                prior.packet == event.packet
                    && prior.phase == event.phase
                    && Arc::ptr_eq(&prior.plan, &event.plan)
            }) {
                duplicate.push(format!(
                    "{:?} {:?}: {:?}",
                    event.packet,
                    event.phase,
                    body.plan.node(event.packet).unwrap().content
                ));
            }
        }
        assert!(
            missing.is_empty(),
            "captured full-source Table missing body phases {missing:?}"
        );
        assert!(
            duplicate.is_empty(),
            "captured full-source Table duplicate body phases {duplicate:?}"
        );
    }

    #[test]
    fn page_running_requests_retain_actual_twopage_effective_style_chain() {
        use crate::compiler::layout::{
            layout_with_page_running, FixedMetrics, PageRunningLayoutInputs,
            PageRunningSlot,
        };
        use crate::entities::{
            image_sizer::NullImageSizer,
            world_types::{Library, Route, Routines, Sink, Traced},
        };
        use comemo::Track;
        let source = crate::entities::source::Source::detached(
            r#"#set text(10pt)
#set page(width:280pt,height:320pt,margin:60pt,header:context metadata(repr(text.size)))
#set text(20pt)
FIRST
#pagebreak()
#set text(30pt)
SECOND
"#,
        )
        .expect("valid Source test fixture");
        let world = PlanTestWorld {
            source,
            library: Library::new(),
            book: crate::entities::font_book::FontBook::new(),
        };
        let route = Route::root();
        let mut sink = Sink::new();
        let traced = Traced::default();
        let registry = crate::entities::element_registry::ElementRegistry::new();
        let module = crate::compiler::eval::eval(
            &Routines::new(),
            &world,
            traced.track(),
            sink.track_mut(),
            route.track(),
            &world.source,
            &registry,
        )
        .unwrap();
        let origin = module.content().unwrap().clone();
        let mut arena = ContextOccurrenceArena::new(&origin);
        let mut frame = arena.frame();
        let mut ctx = EvalContext::new();
        let body = arena
            .evaluate(&mut frame, &mut ctx, |ctx| prepare_page_running_body(&origin, ctx))
            .unwrap();
        let outcome = layout_with_page_running(
            &body,
            source_only_index(&body),
            FixedMetrics,
            NullImageSizer,
            11.0,
            PageRunningLayoutInputs { math: None, par_line: None, running: None },
        )
        .into_result();
        let pending = match outcome {
            Ok(_) => panic!("uncaptured contextual headers require discovery"),
            Err(pending) => pending,
        };
        let actual = pending
            .running
            .iter()
            .filter(|request| request.key().slot == PageRunningSlot::Header)
            .map(|request| (request.key().page.get(), request.authority.chain.size()))
            .collect::<Vec<_>>();
        assert_eq!(
            actual,
            vec![(1, 20.0), (2, 30.0)],
            "physical request authority must carry each actual page-wide lexical frontier, not creation10/default11 or partial render styles"
        );
    }

    #[test]
    fn page_running_transform_retains_descendant_membership_and_actual_source_paint() {
        use crate::compiler::layout::{
            layout_with_page_running, FixedMetrics, PageRunningLayoutInputs,
        };
        use crate::entities::{
            element_kind::ElementKind, image_sizer::NullImageSizer,
            introspector::Introspector, layout_types::TransformMatrix,
            selector::Selector,
        };
        let origin = Content::transform(
            TransformMatrix::identity(),
            Content::heading(1, Content::text("TRANSFORMED HEADING")),
        );
        let mut arena = ContextOccurrenceArena::new(&origin);
        let mut frame = arena.frame();
        let mut ctx = EvalContext::new();
        let body = arena
            .evaluate(&mut frame, &mut ctx, |ctx| prepare_page_running_body(&origin, ctx))
            .unwrap();
        let intr = source_only_index(&body);
        let headings = intr.query(&Selector::Kind(ElementKind::Heading));
        assert_eq!(
            headings.len(),
            1,
            "an identity transform must retain its actual descendant source membership"
        );
        assert_eq!(
            intr.elements[&headings[0]].content().plain_text(),
            "TRANSFORMED HEADING"
        );
        let empty = SealedPageRunning::new(vec![]).unwrap();
        let outcome = layout_with_page_running(
            &body,
            intr,
            FixedMetrics,
            NullImageSizer,
            11.0,
            PageRunningLayoutInputs { math: None, par_line: None, running: Some(&empty) },
        )
        .into_result();
        let trace = match outcome {
            Ok(document) => document.trace,
            Err(pending) => pending.trace,
        };
        assert!(
            trace.data.complete,
            "actual transformed source paint must retain required descendant phases"
        );
        assert!(scheduled_document(&PageRunningIndexInput {
            body: &body,
            target: EvalTarget::Paged,
            running: Some((&trace, &empty))
        })
        .is_ok());
    }

    #[test]
    fn page_running_original_outline_entry_keeps_raw_template_without_phantom_child_allocation(
    ) {
        use crate::entities::{
            content::RealizedContent,
            elements::{metadata::MetadataElem, outline_entry::OutlineEntryElem},
            introspector::Introspector,
            value::Value,
        };
        let template = Content::Metadata(Arc::new(MetadataElem {
            value: Box::new(Value::Str("ACTUAL ONLY".into())),
        }));
        let entry = Content::OutlineEntry(Arc::new(OutlineEntryElem::new(
            1,
            Content::heading(1, template.clone()),
            None,
            true,
        )));
        let mut arena = ContextOccurrenceArena::new(&Content::Empty);
        let mut frame = arena.frame();
        let mut ctx = EvalContext::new();
        let (scene, index) = arena
            .evaluate(&mut frame, &mut ctx, |ctx| {
                let original = crate::compiler::realize::locate(entry.clone(), ctx);
                let visual = crate::compiler::realize::locate(template.clone(), ctx);
                let scene =
                    Content::Realized(Arc::new(RealizedContent { original, visual }));
                let index = provisional_body_index(&scene, ctx)?;
                Ok::<_, Vec<crate::entities::source_result::SourceDiagnostic>>((
                    scene, index,
                ))
            })
            .expect("an original-only template cannot require a new child identity");
        assert_eq!(index.query_metadata(), vec![Value::Str("ACTUAL ONLY".into())]);
        let Content::Realized(done) = scene else { panic!("realized own-show scene") };
        let Content::Located(original) = &done.original else {
            panic!("actual original root identity")
        };
        let Content::OutlineEntry(entry) = &original.body else {
            panic!("original entry")
        };
        let Content::Heading(heading) = &entry.element else {
            panic!("retained raw element")
        };
        assert!(
            matches!(heading.body, Content::Metadata(_)),
            "the reflected raw template must remain raw and exact"
        );
        let Content::Located(visual) = &done.visual else {
            panic!("actual replacement identity")
        };
        assert_ne!(original.location, visual.location);
        let Content::OutlineEntry(observed) =
            index.elements[&original.location].content()
        else {
            panic!("reflected original entry")
        };
        assert_eq!(observed.level, 1);
        assert!(observed.fill.is_none() && observed.fill_set);
        let Content::Heading(observed_heading) = &observed.element else {
            panic!("reflected element morphology")
        };
        let Content::Metadata(stored_template) = &observed_heading.body else {
            panic!("reflected raw template")
        };
        assert_eq!(stored_template.value.as_ref(), &Value::Str("ACTUAL ONLY".into()));
        let Content::Metadata(actual) = index.elements[&visual.location].content() else {
            panic!("actual replacement morphology")
        };
        assert_eq!(actual.value.as_ref(), &Value::Str("ACTUAL ONLY".into()));
    }

    #[test]
    fn page_running_realized_outline_title_has_one_actual_snapshot_without_auxiliary() {
        use crate::entities::{
            element_kind::ElementKind, elements::heading::HeadingElem,
            introspector::Introspector,
        };
        let Content::Outline(outline) = Content::outline() else { panic!("outline") };
        let title = Content::Heading(Arc::new(HeadingElem::new_with_outlined(
            1,
            Content::text("Contents"),
            false,
        )));
        let realized = Content::Outline(Arc::new(outline.with_realized_body(title)));
        let origin = Content::sequence(vec![
            realized,
            Content::heading(1, Content::text("source")),
        ]);
        let mut arena = ContextOccurrenceArena::new(&origin);
        let mut frame = arena.frame();
        let mut ctx = EvalContext::new();
        let body = arena
            .evaluate(&mut frame, &mut ctx, |ctx| prepare_page_running_body(&origin, ctx))
            .unwrap();
        let index = source_only_index(&body);
        let headings = index.query_by_kind(ElementKind::Heading);
        assert_eq!(
            headings.len(),
            2,
            "actual title and source heading are the two occurrences"
        );
        assert!(
            headings.iter().all(|location| index.elements.contains_key(location)),
            "no unreflectable compatibility title may accompany the actual title"
        );
        assert_eq!(
            index.outlined_heading_locations.len(),
            1,
            "the actual title is excluded from its own outline"
        );
    }

    #[test]
    fn page_running_realized_outline_entry_has_no_compatibility_duplicate() {
        use crate::compiler::layout::FixedMetrics;
        use crate::contracts::world::World;
        use crate::entities::{
            element_kind::ElementKind,
            elements::outline::{OutlineElem, OutlineIndent, OutlineTarget},
            introspector::Introspector,
            world_types::{Library, Route, Sink},
        };
        use comemo::Track;
        let heading = Content::heading(1, Content::text("source"));
        let entry = Content::outline_entry(1, heading.clone(), None);
        let outline = OutlineElem::with_target_and_title_presence(
            None,
            true,
            3,
            OutlineIndent::Auto,
            OutlineTarget::Headings,
        )
        .with_realized_body(entry);
        let origin =
            Content::sequence(vec![Content::Outline(Arc::new(outline)), heading]);
        let world = PlanTestWorld {
            source: crate::entities::source::Source::detached("")
                .expect("valid Source test fixture"),
            library: Library::new(),
            book: crate::entities::font_book::FontBook::new(),
        };
        let mut arena = ContextOccurrenceArena::new(&origin);
        let mut body_frame = arena.frame();
        let mut index_frame = arena.frame();
        let mut ctx = EvalContext::new();
        let body = arena
            .evaluate(&mut body_frame, &mut ctx, |ctx| {
                prepare_page_running_body(&origin, ctx)
            })
            .unwrap();
        let route = Route::root();
        let mut styles = StyleChain::default_chain();
        let mut rules = Arc::from([]);
        let mut guards = Vec::new();
        let mut sink = Sink::new();
        let mut tracked_sink = sink.track_mut();
        let mut engine = Engine {
            world: &world,
            font_metrics: &FixedMetrics,
            route: route.track(),
            styles: &mut styles,
            show_rules: &mut rules,
            active_guards: &mut guards,
            current_file: world.main(),
            sink: &mut tracked_sink,
        };
        let (index, _) = arena
            .evaluate(&mut index_frame, &mut ctx, |ctx| {
                introspect_with_page_running(
                    &[PageRunningIndexInput {
                        body: &body,
                        target: EvalTarget::Html,
                        running: None,
                    }],
                    Features::default(),
                    ctx,
                    &mut engine,
                )
            })
            .unwrap();
        let entries = index.query_by_kind(ElementKind::OutlineEntry);
        assert_eq!(
            entries.len(),
            1,
            "one actual entry must not receive a compatibility duplicate"
        );
        assert!(matches!(
            index.elements[&entries[0]].content(),
            Content::OutlineEntry(_)
        ));
    }

    #[test]
    fn page_running_source_located_carriers_preserve_actual_payload_and_parent() {
        use crate::entities::{
            bib_entry::BibEntry, element_kind::ElementKind, introspector::Introspector,
            value::Value,
        };
        let source = crate::entities::source::Source::detached("actual source carriers")
            .expect("valid Source test fixture");
        let span = source.root().span();
        let mut arena = ContextOccurrenceArena::new(&Content::Empty);
        let mut frame = arena.frame();
        let mut ctx = EvalContext::new();
        arena
            .evaluate(&mut frame, &mut ctx, |ctx| {
                let metadata = crate::compiler::realize::locate(
                    Content::metadata(Value::Str("child".into())),
                    ctx,
                );
                let Content::Located(metadata_location) = &metadata else {
                    panic!("actual metadata")
                };
                let metadata_location = metadata_location.location;
                let heading = crate::compiler::realize::locate(
                    Content::heading(
                        1,
                        Content::sequence(vec![
                            Content::text("HEADING"),
                            metadata.with_source_span(span),
                        ]),
                    ),
                    ctx,
                );
                let Content::Located(heading_location) = &heading else {
                    panic!("actual heading")
                };
                let heading_location = heading_location.location;
                let sourced = heading.with_source_span(span);
                let index = provisional_body_index(&sourced, ctx)?;
                assert_eq!(index.query_by_kind(ElementKind::Heading), [heading_location]);
                assert_eq!(
                    index.query_by_kind(ElementKind::Metadata),
                    [metadata_location]
                );
                assert_eq!(index.elements.len(), 2, "carriers add no semantic snapshot");
                assert_eq!(index.outlined_heading_locations, [heading_location]);
                assert_eq!(index.headings_for_toc.len(), 1);
                assert_eq!(index.headings_for_toc[0].2.plain_text(), "HEADING");
                assert_eq!(
                    index.parent_locations.get(&metadata_location),
                    Some(&heading_location)
                );
                assert!(matches!(
                    index.elements[&heading_location].content(),
                    Content::Heading(_)
                ));
                assert!(matches!(
                    index.elements[&metadata_location].content(),
                    Content::Metadata(_)
                ));
                for raw in [
                    Content::heading(1, Content::text("RAW")),
                    Content::metadata(Value::None),
                ] {
                    assert!(
                        provisional_body_index(&raw.with_source_span(span), ctx).is_err(),
                        "the actual unlocated child must still reject"
                    );
                }
                let reference = crate::entities::elements::r#ref::RefElem {
                    name: "known".into(),
                    supplement: Some(Content::heading(1, Content::text("supplement"))),
                    form: crate::entities::elements::r#ref::RefForm::Normal,
                };
                let alias = crate::compiler::realize::locate(
                    Content::Ref(Arc::new(reference)),
                    ctx,
                )
                .with_source_span(span);
                let scene = Content::sequence(vec![
                    alias,
                    Content::bibliography(
                        vec![BibEntry::new("known", "A", "T", 2026)],
                        None,
                    ),
                ]);
                let body = prepare_page_running_body(&scene, ctx)?;
                let selected = source_only_index(&body);
                assert_eq!(selected.citation_order(), ["known"]);
                assert_eq!(selected.query_by_kind(ElementKind::Heading).len(), 1);
                Ok::<_, Vec<crate::entities::source_result::SourceDiagnostic>>(())
            })
            .expect("Source and Located carry actual roots without owning their payload");
    }

    fn membership_body(origin: &Content) -> PageRunningBody {
        let mut arena = ContextOccurrenceArena::new(origin);
        let mut frame = arena.frame();
        let mut ctx = EvalContext::new();
        arena
            .evaluate(&mut frame, &mut ctx, |ctx| prepare_page_running_body(origin, ctx))
            .unwrap()
    }
    #[test]
    fn readiness_equation_membership_planned_nested_counter_and_label() {
        use crate::entities::{
            counter::CounterKey, element_kind::ElementKind, introspector::Introspector,
            label::Label, selector::Selector, style::Styles, value::Value,
        };
        let nested =
            Content::label_auto("inner", Content::equation(Content::text("x"), true));
        let origin = Content::styled(
            Content::equation(nested, true),
            Styles::new().push_custom("equation.numbering", Value::Str("(1)".into())),
        );
        let body = membership_body(&origin);
        let intr = source_only_index(&body);
        assert_eq!(intr.query_by_kind(ElementKind::Equation).len(), 1);
        assert_eq!(
            intr.counter_final_values(&CounterKey::Selector(Selector::Kind(
                ElementKind::Equation
            ))),
            Some([1].as_slice())
        );
        assert!(intr.labels.lookup(&Label("inner".into())).is_none());
        assert!(!intr.unreferencable_labels.contains_key(&Label("inner".into())));
    }
    #[test]
    fn readiness_equation_membership_planned_hidden_original_and_actual_visual() {
        use crate::entities::{
            content::{LocatedContent, RealizedContent},
            element_kind::ElementKind,
            introspector::Introspector,
            locator::Locator,
            value::Value,
        };
        let mut locator = Locator::new();
        let root = locator.next();
        let actual = locator.next();
        let metadata = Content::metadata(Value::Str("M".into()));
        let original = Content::Located(Arc::new(LocatedContent {
            location: root,
            body: Content::equation(metadata.clone(), true),
        }));
        let hidden = Content::Realized(Arc::new(RealizedContent {
            original: original.clone(),
            visual: Content::Empty,
        }));
        let intr = source_only_index(&membership_body(&hidden));
        assert_eq!(intr.query_by_kind(ElementKind::Equation).len(), 1);
        assert!(intr.query_by_kind(ElementKind::Metadata).is_empty());
        let Content::Equation(snapshot) = intr.elements[&root].content().source_body()
        else {
            panic!("snapshot")
        };
        assert!(matches!(snapshot.body, Content::Metadata(_)));
        let visual = Content::Located(Arc::new(LocatedContent {
            location: root,
            body: Content::equation(
                Content::Located(Arc::new(LocatedContent {
                    location: actual,
                    body: metadata,
                })),
                true,
            ),
        }));
        let shown = Content::Realized(Arc::new(RealizedContent { original, visual }));
        assert_eq!(
            source_only_index(&membership_body(&shown))
                .query_by_kind(ElementKind::Metadata),
            vec![actual]
        );
    }
    #[test]
    fn readiness_equation_membership_planned_raw_inner_phase_is_still_required() {
        use crate::compiler::layout::{
            layout_with_page_running, FixedMetrics, PageRunningLayoutInputs,
        };
        use crate::entities::{image_sizer::NullImageSizer, value::Value};
        let origin = Content::equation(
            Content::equation(Content::metadata(Value::Str("M".into())), false),
            true,
        );
        let body = membership_body(&origin);
        let empty = SealedPageRunning::new(vec![]).unwrap();
        let outcome = layout_with_page_running(
            &body,
            source_only_index(&body),
            FixedMetrics,
            NullImageSizer,
            11.0,
            PageRunningLayoutInputs { math: None, par_line: None, running: Some(&empty) },
        )
        .into_result();
        let trace = match outcome {
            Ok(doc) => doc.trace,
            Err(pending) => pending.trace,
        };
        assert!(trace.data.complete);
        let inside = trace.data.admissions.iter().position(|event| event.phase == PlanPhase::Root && matches!(&event.plan.node(event.packet).unwrap().content, Content::Equation(eq) if !eq.block)).unwrap();
        let mut invalid = trace.clone();
        invalid.data.admissions.remove(inside);
        assert!(scheduled_document(&PageRunningIndexInput {
            body: &body,
            target: EvalTarget::Paged,
            running: Some((&invalid, &empty))
        })
        .is_err());
    }

    fn source_only_index(body: &PageRunningBody) -> TagIntrospector {
        let mut events = Vec::new();
        source_events(&body.plan, body.plan.root, &mut events).unwrap();
        let keys = active_bibliography_keys(&[events.clone()]);
        complete_conditional_sources(
            &PageRunningIndexInput { body, target: EvalTarget::Html, running: None },
            &mut events,
            &keys,
        )
        .unwrap();
        let mut construction = Construction::new();
        construction.bibliography_keys = keys;
        construction.observe(&events).unwrap();
        for event in &events {
            construction.apply(event).unwrap();
        }
        construction.intr
    }

    #[test]
    fn page_running_known_bibliography_refs_preserve_order_aliases_and_selected_rich_supplement(
    ) {
        use crate::entities::{
            bib_entry::BibEntry, element_kind::ElementKind, introspector::Introspector,
        };
        let mut ctx = EvalContext::new();
        let alias = crate::compiler::realize::locate(Content::reference("b"), &mut ctx);
        let mut reference = crate::entities::elements::r#ref::RefElem {
            name: "a".into(),
            supplement: None,
            form: crate::entities::elements::r#ref::RefForm::Normal,
        };
        reference.supplement =
            Some(Content::heading(1, Content::text("selected supplement")));
        let origin = Content::sequence(vec![
            alias.clone(),
            alias,
            Content::Ref(Arc::new(reference)),
            Content::bibliography(
                vec![
                    BibEntry::new("a", "A", "A", 2026),
                    BibEntry::new("b", "B", "B", 2026),
                ],
                None,
            ),
        ]);
        let mut arena = ContextOccurrenceArena::new(&origin);
        let mut frame = arena.frame();
        let body = arena
            .evaluate(&mut frame, &mut ctx, |ctx| prepare_page_running_body(&origin, ctx))
            .unwrap();
        let intr = source_only_index(&body);
        assert_eq!(intr.citation_order(), ["b", "a"]);
        assert_eq!(intr.kind_index[&ElementKind::Citation].len(), 2);
        let headings =
            intr.query(&crate::entities::selector::Selector::Kind(ElementKind::Heading));
        assert_eq!(headings.len(), 1);
        assert_eq!(
            intr.elements[&headings[0]].content().plain_text(),
            "selected supplement"
        );
        assert_eq!(body.content(), &origin);
    }

    #[test]
    fn page_running_unknown_reference_supplement_and_located_set_page_remain_dormant_but_reserved(
    ) {
        use crate::entities::{
            content::{SetPageDelta, ShowRecipeContent},
            file_id::FileId,
            page_running::PageMarginal,
        };
        let mut allocator = Locator::new();
        let reserved = (0..43).map(|_| allocator.next()).last().unwrap();
        let template = Content::ShowRecipe(Arc::new(ShowRecipeContent {
            body: Content::Located(Arc::new(crate::entities::content::LocatedContent {
                body: Content::heading(1, Content::text("DORMANT")),
                location: reserved,
            })),
            textual: false,
            context_id_floor: 77,
            rules: Arc::from([]),
            styles: StyleChain::default_chain(),
            guards: vec![],
            file: FileId::from_raw(std::num::NonZeroU16::new(1).unwrap()),
        }));
        let page = Content::SetPage(Arc::new(SetPageDelta {
            header: Some(PageMarginal::Content(Arc::new(template))),
            ..SetPageDelta::default()
        }));
        let located =
            Content::Located(Arc::new(crate::entities::content::LocatedContent {
                body: page,
                location: Locator::new().next(),
            }));
        let mut reference = crate::entities::elements::r#ref::RefElem {
            name: "missing".into(),
            supplement: None,
            form: crate::entities::elements::r#ref::RefForm::Normal,
        };
        reference.supplement = Some(Content::heading(1, Content::text("INACTIVE")));
        let origin = Content::sequence(vec![located, Content::Ref(Arc::new(reference))]);
        let mut arena = ContextOccurrenceArena::new(&Content::Empty);
        let mut frame = arena.frame();
        let mut ctx = EvalContext::new();
        let body = arena
            .evaluate(&mut frame, &mut ctx, |ctx| prepare_page_running_body(&origin, ctx))
            .unwrap();
        let (_, boundary) = body.plan.document_boundary.unwrap();
        let mut after_reserved = Locator::new();
        after_reserved.observe(reserved);
        assert_eq!(boundary, after_reserved.next());
        assert_eq!(ctx.next_context_id, 77);
        let intr = source_only_index(&body);
        assert!(intr.elements.is_empty());
        assert!(intr
            .kind_index
            .get(&crate::entities::element_kind::ElementKind::Heading)
            .is_none_or(Vec::is_empty));
    }
    fn zero_copy_metadata(value: &str) -> Content {
        Content::metadata(crate::entities::value::Value::Str(value.into()))
    }

    fn zero_copy_box(body: Content, width: f64) -> Content {
        use crate::entities::{layout_types::Length, sides::Sides};
        Content::boxed(
            body,
            Some(Length::pt(width)),
            Some(Length::pt(16.0)),
            Sides::uniform(Length::ZERO),
            Length::ZERO,
        )
    }

    fn zero_copy_trace(
        origin: Content,
    ) -> (PageRunningBody, PageRunningTrace, SealedPageRunning) {
        use crate::compiler::layout::{
            layout_with_page_running, FixedMetrics, PageRunningLayoutInputs,
        };
        use crate::entities::image_sizer::NullImageSizer;
        let mut arena = ContextOccurrenceArena::new(&origin);
        let mut frame = arena.frame();
        let mut ctx = EvalContext::new();
        let body = arena
            .evaluate(&mut frame, &mut ctx, |ctx| prepare_page_running_body(&origin, ctx))
            .unwrap();
        let empty = SealedPageRunning::new(vec![]).unwrap();
        let outcome = layout_with_page_running(
            &body,
            source_only_index(&body),
            FixedMetrics,
            NullImageSizer,
            11.0,
            PageRunningLayoutInputs { math: None, par_line: None, running: Some(&empty) },
        )
        .into_result();
        let trace = match outcome {
            Ok(document) => document.trace,
            Err(pending) => pending.trace,
        };
        (body, trace, empty)
    }

    fn zero_copy_metadata_values(events: &[PlannedEvent]) -> Vec<String> {
        events
            .iter()
            .filter(|event| event.phase == PlanPhase::Root)
            .filter_map(|event| {
                let Content::Metadata(value) = &event.plan.node(event.packet)?.content
                else {
                    return None;
                };
                let crate::entities::value::Value::Str(value) = value.value.as_ref()
                else {
                    return None;
                };
                Some(value.to_string())
            })
            .collect()
    }

    fn zero_copy_scene(gap: f64) -> Content {
        use crate::entities::layout_types::Length;
        Content::sequence(vec![
            zero_copy_metadata("before"),
            zero_copy_box(
                Content::repeat(
                    Content::sequence(vec![
                        Content::text("X"),
                        zero_copy_metadata("inside"),
                    ]),
                    Some(Length::pt(gap)),
                    false,
                ),
                80.0,
            ),
            zero_copy_metadata("after"),
        ])
    }

    #[test]
    fn zero_copy_repeat_keeps_raw_coverage_without_semantic_body_membership() {
        for (gap, expected) in
            [(2.0, vec!["before", "inside", "after"]), (-100.0, vec!["before", "after"])]
        {
            let (body, trace, empty) = zero_copy_trace(zero_copy_scene(gap));
            assert!(trace.data.complete, "dispatch coverage must remain complete");
            let inside = body
                .plan
                .nodes
                .iter()
                .position(|node| matches!(&node.content, Content::Metadata(value)
                    if matches!(value.value.as_ref(), crate::entities::value::Value::Str(text) if text.as_str()=="inside")))
                .map(PacketId)
                .expect("the prepared source must retain the body packet");
            for phase in [PlanPhase::Root, PlanPhase::PostChildren] {
                assert_eq!(
                    trace
                        .ordered()
                        .iter()
                        .filter(|event| Arc::ptr_eq(&event.plan, &body.plan)
                            && event.packet == inside
                            && event.phase == phase)
                        .count(),
                    1,
                    "the actual body dispatch is covered once even when no copy paints"
                );
            }
            let events = scheduled_document(&PageRunningIndexInput {
                body: &body,
                target: EvalTarget::Paged,
                running: Some((&trace, &empty)),
            })
            .unwrap();
            assert_eq!(zero_copy_metadata_values(&events), expected);
        }
    }

    #[test]
    fn zero_copy_repeat_activity_stays_with_its_exact_nested_source_subtree() {
        use crate::entities::layout_types::Length;
        for (outer_gap, inner_gap, expected) in [
            (2.0, -100.0, vec!["before", "outer", "after"]),
            (-100.0, 2.0, vec!["before", "after"]),
        ] {
            let inner = zero_copy_box(
                Content::repeat(
                    Content::sequence(vec![
                        Content::text("X"),
                        zero_copy_metadata("inner"),
                    ]),
                    Some(Length::pt(inner_gap)),
                    false,
                ),
                20.0,
            );
            let outer = zero_copy_box(
                Content::repeat(
                    Content::sequence(vec![zero_copy_metadata("outer"), inner]),
                    Some(Length::pt(outer_gap)),
                    false,
                ),
                80.0,
            );
            let (body, trace, empty) = zero_copy_trace(Content::sequence(vec![
                zero_copy_metadata("before"),
                outer,
                zero_copy_metadata("after"),
            ]));
            assert!(trace.data.complete, "nested dispatch coverage must stay complete");
            let events = scheduled_document(&PageRunningIndexInput {
                body: &body,
                target: EvalTarget::Paged,
                running: Some((&trace, &empty)),
            })
            .unwrap();
            assert_eq!(zero_copy_metadata_values(&events), expected);
        }
    }

    #[test]
    fn zero_copy_repeat_cannot_hide_missing_duplicate_or_foreign_source_phases() {
        let (body, trace, empty) = zero_copy_trace(zero_copy_scene(-100.0));
        assert!(trace.data.complete);
        let inside = trace.data.admissions.iter().position(|event|
            event.phase == PlanPhase::Root
                && matches!(&event.plan.node(event.packet).unwrap().content,
                    Content::Metadata(value) if matches!(value.value.as_ref(),
                        crate::entities::value::Value::Str(text) if text.as_str()=="inside")))
            .expect("inside source admission");
        for mode in ["missing-root", "missing-post", "duplicate", "foreign"] {
            let mut invalid = trace.clone();
            match mode {
                "missing-root" => {
                    invalid.data.admissions.remove(inside);
                }
                "missing-post" => {
                    let repeat = body
                        .plan
                        .nodes
                        .iter()
                        .position(|node| matches!(node.content, Content::Repeat(_)))
                        .map(PacketId)
                        .unwrap();
                    let child = body.plan.child(repeat, PlanEdge::RepeatBody).unwrap();
                    let post = invalid
                        .data
                        .admissions
                        .iter()
                        .position(|candidate| {
                            candidate.phase == PlanPhase::PostChildren
                                && candidate.packet == child
                                && Arc::ptr_eq(&candidate.plan, &body.plan)
                        })
                        .unwrap();
                    invalid.data.admissions.remove(post);
                }
                "duplicate" => {
                    invalid.data.admissions.push(trace.data.admissions[inside].clone());
                }
                "foreign" => {
                    invalid.data.admissions[inside].plan = Arc::new((*body.plan).clone());
                }
                _ => unreachable!(),
            }
            assert!(
                scheduled_document(&PageRunningIndexInput {
                    body: &body,
                    target: EvalTarget::Paged,
                    running: Some((&invalid, &empty)),
                })
                .is_err(),
                "inactive contribution cannot conceal {mode} corruption"
            );
        }
    }
}


#[cfg(test)]
mod source_plan_query_tests {
    use super::*;
    use crate::compiler::realize::context_occurrences::ContextOccurrenceArena;
    use crate::entities::{elements::columns::ColumnsElem, style::Styles, value::Value};

    fn prepared(content: Content) -> SourcePlan {
        let mut arena = ContextOccurrenceArena::new(&content);
        let mut frame = arena.frame();
        let mut ctx = EvalContext::new();
        let body = arena
            .evaluate(&mut frame, &mut ctx, |ctx| {
                prepare_page_running_body(&content, ctx)
            })
            .unwrap();
        (*body.plan).clone()
    }

    #[test]
    fn frontier_queries_have_constant_observed_work() {
        let plan = prepared(Content::sequence(
            (0..4096).map(|i| Content::text(format!("item {i}"))).collect(),
        ));
        let target = PacketId(plan.nodes.len() - 1);
        let mut work = 0;
        for _ in 0..32 {
            assert!(plan.frontier_query(target, || work += 1));
            assert!(plan.page_frontier(target));
        }
        assert!(
            work <= 32,
            "frontier query must not traverse the root per packet: {work}"
        );
    }

    #[test]
    fn child_queries_have_logarithmic_observed_work() {
        let plan = prepared(Content::sequence(
            (0..4096).map(|i| Content::text(format!("item {i}"))).collect(),
        ));
        let expected = plan.nodes[plan.root.0]
            .children
            .iter()
            .find_map(|(e, p)| (*e == PlanEdge::SequencePart(4095)).then_some(*p))
            .unwrap();
        let mut work = 0;
        assert_eq!(
            plan.child_query(plan.root, PlanEdge::SequencePart(4095), || work += 1),
            Some(expected)
        );
        assert_eq!(plan.child(plan.root, PlanEdge::SequencePart(4095)), Some(expected));
        assert!(work <= 16, "child query must not scan the whole sibling list: {work}");
    }

    #[test]
    fn every_typed_edge_and_frontier_exclusion_matches_the_original_predicate() {
        let edges = vec![
            PlanEdge::SourceBody,
            PlanEdge::LocatedBody,
            PlanEdge::StyledBody,
            PlanEdge::ShowRecipeBody,
            PlanEdge::RealizedOriginal,
            PlanEdge::RealizedVisual,
            PlanEdge::SequencePart(1),
            PlanEdge::ParagraphBody,
            PlanEdge::HeadingBody,
            PlanEdge::TitleBody,
            PlanEdge::StrongBody,
            PlanEdge::EmphBody,
            PlanEdge::SmallCapsBody,
            PlanEdge::LabelBody,
            PlanEdge::ListItemBody,
            PlanEdge::EnumItemBody,
            PlanEdge::EquationBody,
            PlanEdge::FigureBody,
            PlanEdge::FigureCaption,
            PlanEdge::FigureSelectedSupplement,
            PlanEdge::FigureCaptionBody,
            PlanEdge::FigureCaptionSeparator,
            PlanEdge::PreparedReferenceSupplement,
            PlanEdge::CitationSupplement,
            PlanEdge::GridCell(1),
            PlanEdge::GridHeader,
            PlanEdge::GridFooter,
            PlanEdge::GridCellBody,
            PlanEdge::GridHeaderBody,
            PlanEdge::GridFooterBody,
            PlanEdge::TableResolvedCell(1),
            PlanEdge::TableRawChild(1),
            PlanEdge::TableHeader,
            PlanEdge::TableFooter,
            PlanEdge::TableCaption,
            PlanEdge::CellSelectedVisual,
            PlanEdge::TableHeaderBody,
            PlanEdge::TableFooterBody,
            PlanEdge::PageRunBody,
            PlanEdge::PageRunBackground,
            PlanEdge::PageRunHeader,
            PlanEdge::PageRunFooter,
            PlanEdge::PageRunForeground,
            PlanEdge::SetPageBackground,
            PlanEdge::SetPageHeader,
            PlanEdge::SetPageFooter,
            PlanEdge::SetPageForeground,
            PlanEdge::HtmlFrameBody,
            PlanEdge::TermsItem(1),
            PlanEdge::TermItemTerm,
            PlanEdge::TermItemDescription,
            PlanEdge::QuoteBody,
            PlanEdge::QuoteAttribution,
            PlanEdge::StackChild(1),
            PlanEdge::AlignBody,
            PlanEdge::PlaceBody,
            PlanEdge::TransformBody,
            PlanEdge::PdfArtifactBody,
            PlanEdge::UnderlineBody,
            PlanEdge::StrikeBody,
            PlanEdge::OverlineBody,
            PlanEdge::PadBody,
            PlanEdge::HideBody,
            PlanEdge::BoxedBody,
            PlanEdge::BlockBody,
            PlanEdge::RepeatBody,
            PlanEdge::ColumnsBody,
            PlanEdge::BibliographyTitle,
            PlanEdge::FootnoteBody,
            PlanEdge::FootnoteEntryNote,
            PlanEdge::FootnoteEntrySeparator,
            PlanEdge::OutlineRealizedBody,
            PlanEdge::OutlineEntryBody,
        ];
        let contents = vec![
            Content::text("root"),
            Content::styled(
                Box::new(Content::Empty),
                Styles::new().push_custom("eval.flow.scope", Value::Bool(true)),
            ),
            Content::styled(
                Box::new(Content::Empty),
                Styles::new().push_custom("eval.flow.scope", Value::Bool(false)),
            ),
            Content::Columns(Arc::new(ColumnsElem {
                count: 2,
                gutter: None,
                body: Content::Empty,
                page_columns: true,
            })),
            Content::Columns(Arc::new(ColumnsElem {
                count: 2,
                gutter: None,
                body: Content::Empty,
                page_columns: false,
            })),
        ];
        let base =
            prepared(Content::sequence(vec![Content::text("a"), Content::text("b")]));
        let roles = [
            PlanRole::Canonical,
            PlanRole::OriginalOnly,
            PlanRole::VisualOnly,
            PlanRole::OperationalTemplate,
            PlanRole::PreparedReference,
            PlanRole::PrunedOriginal,
        ];
        for edge in edges {
            for content in &contents {
                for role in roles {
                    for original in [false, true] {
                        for conditional in [None, Some(PacketId(0))] {
                            for at in [0, 1] {
                                let mut plan = base.clone();
                                plan.root = PacketId(0);
                                for n in &mut plan.nodes {
                                    n.children.clear();
                                    n.original = false;
                                    n.conditional_ref = None;
                                    n.role = PlanRole::Canonical;
                                }
                                plan.nodes[0].content = content.clone();
                                plan.nodes[0].children = vec![(edge, PacketId(1))];
                                plan.nodes[at].role = role;
                                plan.nodes[at].original = original;
                                plan.nodes[at].conditional_ref = conditional;
                                let before = plan
                                    .nodes
                                    .iter()
                                    .map(|n| n.children.clone())
                                    .collect::<Vec<_>>();
                                plan.initialize_queries();
                                assert_eq!(
                                    before,
                                    plan.nodes
                                        .iter()
                                        .map(|n| n.children.clone())
                                        .collect::<Vec<_>>()
                                );
                                for id in 0..plan.nodes.len() + 2 {
                                    assert_eq!(plan.page_frontier(PacketId(id)),plan.reference_frontier(PacketId(id)),"edge {edge:?} role {role:?} original {original} conditional {conditional:?} at {at} id {id}");
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn child_first_match_missing_edges_and_forward_constraints_are_preserved() {
        let mut plan =
            prepared(Content::sequence(vec![Content::text("a"), Content::text("b")]));
        plan.root = PacketId(0);
        plan.nodes[0].children = vec![
            (PlanEdge::SequencePart(2), PacketId(2)),
            (PlanEdge::SequencePart(1), PacketId(2)),
            (PlanEdge::SequencePart(1), PacketId(1)),
            (PlanEdge::SourceBody, PacketId(9999)),
        ];
        plan.nodes[1].children = vec![(PlanEdge::SourceBody, PacketId(0))];
        plan.initialize_queries();
        for parent in 0..plan.nodes.len() + 2 {
            for edge in [
                PlanEdge::SequencePart(1),
                PlanEdge::SequencePart(2),
                PlanEdge::SequencePart(3),
                PlanEdge::SourceBody,
                PlanEdge::ParagraphBody,
            ] {
                let expected = plan.node(PacketId(parent)).and_then(|n| {
                    n.children.iter().find_map(|(e, p)| (*e == edge).then_some(*p))
                });
                assert_eq!(plan.child(PacketId(parent), edge), expected);
            }
        }
        assert_eq!(plan.child(PacketId(0), PlanEdge::SequencePart(1)), Some(PacketId(2)));
        assert_eq!(plan.child(PacketId(0), PlanEdge::SourceBody), Some(PacketId(9999)));
        for id in 0..plan.nodes.len() + 2 {
            assert_eq!(
                plan.page_frontier(PacketId(id)),
                plan.reference_frontier(PacketId(id))
            );
        }
        plan.root = PacketId(2);
        plan.nodes[2].children = vec![(PlanEdge::SourceBody, PacketId(1))];
        plan.initialize_queries();
        assert!(!plan.page_frontier(PacketId(1)));
        plan.root = PacketId(99999);
        plan.initialize_queries();
        assert!(!plan.page_frontier(PacketId(0)));
    }

    #[test]
    fn different_plans_and_clones_do_not_share_frontier_answers() {
        let make = |v| {
            prepared(Content::styled(
                Box::new(Content::text("child")),
                Styles::new().push_custom("eval.flow.scope", Value::Bool(v)),
            ))
        };
        let a = make(true);
        let b = make(false);
        let ac = a.child(a.root, PlanEdge::StyledBody).unwrap();
        let bc = b.child(b.root, PlanEdge::StyledBody).unwrap();
        assert_eq!(ac, bc);
        for _ in 0..4 {
            assert!(a.page_frontier(ac));
            assert!(!b.page_frontier(bc));
            assert!(a.clone().page_frontier(ac));
        }
        assert!(!Arc::ptr_eq(
            a.allocation_owner.as_ref().unwrap(),
            b.allocation_owner.as_ref().unwrap()
        ));
    }
    #[test]
    fn deep_forward_frontiers_do_not_need_a_root_walk_per_query() {
        let mut plan = prepared(Content::text("node"));
        let seed = plan.nodes[0].clone();
        plan.nodes = (0..8192)
            .map(|i| {
                let mut n = seed.clone();
                n.children = if i + 1 < 8192 {
                    vec![(PlanEdge::SourceBody, PacketId(i + 1))]
                } else {
                    vec![]
                };
                n.original = false;
                n.conditional_ref = None;
                n.role = PlanRole::Canonical;
                n
            })
            .collect();
        plan.root = PacketId(0);
        plan.initialize_queries();
        let mut work = 0;
        assert!(plan.frontier_query(PacketId(8191), || work += 1));
        assert!(work <= 1, "deep query must use precomputed provenance: {work}");
    }

    #[inline(never)]
    fn query_storage_bytes(plan: &SourcePlan) -> usize {
        std::mem::size_of_val(&plan.page_frontiers)
            + std::mem::size_of_val(&plan.child_ranges)
            + std::mem::size_of_val(&plan.child_lookup)
            + plan.page_frontiers.capacity() * std::mem::size_of::<bool>()
            + plan.child_ranges.capacity() * std::mem::size_of::<(usize, usize)>()
            + plan.child_lookup.capacity() * std::mem::size_of::<(PlanEdge, PacketId)>()
    }

    #[test]
    fn private_query_storage_has_a_linear_budget() {
        for width in [125, 250, 500, 1000, 4096] {
            let plan = prepared(Content::sequence(
                (0..width).map(|i| Content::text(format!("item {i}"))).collect(),
            ));
            let edges = plan.nodes.iter().map(|n| n.children.len()).sum::<usize>();
            let bytes = query_storage_bytes(&plan);
            assert!(
                bytes <= 64 * (plan.nodes.len() + edges) + 72,
                "private tables exceed linear storage budget: {bytes}"
            );
        }
    }
}

#[cfg(test)]
mod event_membership_tests {
    use super::*;
    use crate::compiler::realize::context_occurrences::ContextOccurrenceArena;
    use crate::entities::locator::Locator;
    use std::cell::Cell;
    use std::hash::{Hash, Hasher};

    struct Counted<'a> {
        value: usize,
        comparisons: &'a Cell<usize>,
        collide: bool,
    }
    impl PartialEq for Counted<'_> {
        fn eq(&self, other: &Self) -> bool {
            self.comparisons.set(self.comparisons.get() + 1);
            self.value == other.value && self.collide == other.collide
        }
    }
    impl Eq for Counted<'_> {}
    impl Hash for Counted<'_> {
        fn hash<H: Hasher>(&self, state: &mut H) {
            if self.collide {
                0usize.hash(state)
            } else {
                self.value.hash(state)
            }
        }
    }

    fn plan() -> Arc<SourcePlan> {
        let content =
            Content::sequence(vec![Content::text("same"), Content::text("same")]);
        let mut arena = ContextOccurrenceArena::new(&content);
        let mut frame = arena.frame();
        let mut ctx = EvalContext::new();
        arena
            .evaluate(&mut frame, &mut ctx, |ctx| {
                prepare_page_running_body(&content, ctx)
            })
            .unwrap()
            .plan
    }

    fn old_unique(events: &[PlannedEvent]) -> bool {
        let mut seen: Vec<(Arc<SourcePlan>, PacketId, PlanPhase)> = Vec::new();
        for event in events {
            if seen.iter().any(|(plan, packet, phase)| {
                Arc::ptr_eq(plan, &event.plan)
                    && *packet == event.packet
                    && *phase == event.phase
            }) {
                return false;
            }
            seen.push((event.plan.clone(), event.packet, event.phase));
        }
        true
    }

    #[test]
    fn unique_membership_has_linear_observed_equality_work() {
        for size in [256, 1024, 4096] {
            let comparisons = Cell::new(0);
            assert!(event_keys_are_unique((0..size).map(|value| Counted {
                value,
                comparisons: &comparisons,
                collide: false
            })));
            assert!(comparisons.get() <= size * 4,
                "membership must avoid scanning all prior unique events: size={size}, actual Eq calls={}", comparisons.get());
        }
    }

    #[test]
    fn collisions_compare_full_keys_and_reject_duplicates() {
        let comparisons = Cell::new(0);
        assert!(event_keys_are_unique((0..64).map(|value| Counted {
            value,
            comparisons: &comparisons,
            collide: true
        })));
        assert!(!event_keys_are_unique((0..64).chain([31]).map(|value| Counted {
            value,
            comparisons: &comparisons,
            collide: true
        })));
        assert!(comparisons.get() > 64); // Collision control is not a constant-work claim.
    }

    #[test]
    fn exact_plan_packet_and_phase_identity_matches_old_validation() {
        let a = plan();
        let b = Arc::new((*a).clone());
        assert!(!Arc::ptr_eq(&a, &b));
        let event = |plan: &Arc<SourcePlan>, packet, phase| PlannedEvent {
            plan: plan.clone(),
            packet: PacketId(packet),
            phase,
        };
        assert!(planned_events_are_unique(&[]));
        let base = vec![
            event(&a, 0, PlanPhase::Root),
            event(&a, 0, PlanPhase::PostChildren),
            event(&a, 1, PlanPhase::Root),
            event(&b, 0, PlanPhase::Root),
        ];
        assert!(planned_events_are_unique(&base));
        for index in 0..base.len() {
            for position in 0..=base.len() {
                let mut duplicate = base.clone();
                duplicate.insert(position, base[index].clone());
                assert!(!planned_events_are_unique(&duplicate));
                assert_eq!(planned_events_are_unique(&duplicate), old_unique(&duplicate));
            }
        }
        for generation in 0..16 {
            let fresh = plan();
            let mut events = Vec::new();
            for i in 0..128 {
                let packet = (i * 17 + generation) % 23;
                let p = if i % 3 == 0 {
                    &fresh
                } else if i % 3 == 1 {
                    &a
                } else {
                    &b
                };
                events.push(event(
                    p,
                    packet,
                    if i % 2 == 0 { PlanPhase::Root } else { PlanPhase::PostChildren },
                ));
                assert_eq!(planned_events_are_unique(&events), old_unique(&events));
            }
            assert!(planned_events_are_unique(&base)); // A previous call leaves no retained set.
        }
    }

    #[test]
    fn aliased_locations_do_not_conflate_distinct_packets() {
        let mut a = (*plan()).clone();
        let alias = Locator::new().next();
        a.nodes[0].location = Some(alias);
        a.nodes[1].location = Some(alias);
        let a = Arc::new(a);
        let events = vec![
            PlannedEvent {
                plan: a.clone(),
                packet: PacketId(0),
                phase: PlanPhase::Root,
            },
            PlannedEvent {
                plan: a,
                packet: PacketId(1),
                phase: PlanPhase::Root,
            },
        ];
        assert!(planned_events_are_unique(&events));
        assert!(old_unique(&events));
    }
}
