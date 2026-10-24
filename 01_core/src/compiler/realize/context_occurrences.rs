//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/realize/context_occurrences.md
//! @prompt-hash a786a731
//! @layer L1
//! @updated 2026-10-18

use std::sync::Arc;

use crate::compiler::eval::EvalContext;
use crate::compiler::introspect::{
    introspect_with_introspector, walk::reserve_locations,
};
use crate::entities::content::Content;
use crate::entities::location::Location;
use crate::entities::locator::{Locator, LocatorSeed};

/// One allocation domain for the contextual producers of a compilation session.
pub struct ContextOccurrenceArena {
    locator: Locator,
    owner: Arc<()>,
    context_id_floor: u64,
    rule_id_floor: crate::entities::show::RuleId,
}

/// A producer's retained allocation slots, replayed in actual execution order.
pub struct ContextOccurrenceFrame {
    slots: OccurrenceSlots,
    owner: Arc<()>,
}

#[derive(Default)]
pub(crate) struct OccurrenceSlots {
    locations: Vec<(LocatorSeed, Location)>,
    owner: Option<Arc<()>>,
    cursor: usize,
    context_ids: Vec<u64>,
    context_cursor: usize,
    instantiate_contexts: bool,
    capture_tokens: Vec<Arc<crate::entities::version::Version>>,
    capture_cursor: usize,
    running_cohorts: Vec<RunningCohort>,
    running_active: bool,
    context_blocks:
        Vec<Option<Arc<crate::entities::elements::context_block::ContextBlockElem>>>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum RunningStage {
    Realize,
    Bind,
}

pub(super) enum RunningToken {
    Captured(Arc<crate::entities::version::Version>),
    Source(
        Arc<crate::compiler::introspect::page_running::SourcePlan>,
        crate::compiler::introspect::page_running::PacketId,
    ),
}

impl RunningToken {
    fn same(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Captured(a), Self::Captured(b)) => Arc::ptr_eq(a, b),
            (Self::Source(a, x), Self::Source(b, y)) => Arc::ptr_eq(a, b) && x == y,
            (Self::Captured(_), Self::Source(_, _))
            | (Self::Source(_, _), Self::Captured(_)) => false,
        }
    }
}

struct RunningCohort {
    stage: RunningStage,
    key: crate::compiler::layout::PageRunningKey,
    token: RunningToken,
    slots: OccurrenceSlots,
}

impl ContextOccurrenceArena {
    /// Reserve the canonical occurrences of the actual original content.
    pub fn new(origin: &Content) -> Self {
        let mut locator = Locator::new();
        reserve_locations(origin, &mut locator);
        for location in introspect_with_introspector(origin).elements.keys() {
            locator.observe(*location);
        }
        let mut context_id_floor = 0;
        let mut rule_id_floor = 0;
        reserve_operational_source(
            origin,
            &mut locator,
            &mut context_id_floor,
            &mut rule_id_floor,
        );
        Self {
            locator,
            owner: Arc::new(()),
            context_id_floor,
            rule_id_floor,
        }
    }

    /// Issue an exclusive frame for a new producer in this arena.
    pub fn frame(&mut self) -> ContextOccurrenceFrame {
        ContextOccurrenceFrame {
            slots: OccurrenceSlots {
                owner: Some(Arc::clone(&self.owner)),
                ..OccurrenceSlots::default()
            },
            owner: Arc::clone(&self.owner),
        }
    }

    /// Run once with this producer's slots and restore the caller on return.
    /// An `Err` return retains the slots for a subsequent execution of the frame.
    pub fn evaluate<R>(
        &mut self,
        frame: &mut ContextOccurrenceFrame,
        ctx: &mut EvalContext,
        operation: impl FnOnce(&mut EvalContext) -> R,
    ) -> R {
        assert!(Arc::ptr_eq(&self.owner, &frame.owner), "frame belongs to another arena");
        ctx.next_context_id = ctx.next_context_id.max(self.context_id_floor);
        ctx.next_rule_id = ctx.next_rule_id.max(self.rule_id_floor);
        frame.slots.cursor = 0;
        frame.slots.context_cursor = 0;
        frame.slots.capture_cursor = 0;
        let previous_slots =
            ctx.context_occurrence_slots.replace(std::mem::take(&mut frame.slots));
        std::mem::swap(&mut self.locator, &mut ctx.realization_locator);
        let result = operation(ctx);
        std::mem::swap(&mut self.locator, &mut ctx.realization_locator);
        frame.slots =
            std::mem::replace(&mut ctx.context_occurrence_slots, previous_slots).expect(
                "contextual allocation frame remains installed during evaluation",
            );
        result
    }
}

fn reserve_operational_source(
    content: &Content,
    locator: &mut Locator,
    context_floor: &mut u64,
    rule_floor: &mut crate::entities::show::RuleId,
) {
    let _ = content.map_content_pre(&mut |node| {
        match node {
            Content::Located(located) => {
                locator.observe(located.location);
                reserve_operational_source(
                    &located.body,
                    locator,
                    context_floor,
                    rule_floor,
                );
                return Ok(Some(node.clone()));
            }
            Content::Realized(realized) => {
                reserve_operational_source(
                    &realized.original,
                    locator,
                    context_floor,
                    rule_floor,
                );
                reserve_operational_source(
                    &realized.visual,
                    locator,
                    context_floor,
                    rule_floor,
                );
                return Ok(Some(node.clone()));
            }
            Content::ContextBlock(block) => {
                *context_floor = (*context_floor).max(block.id.saturating_add(1));
            }
            Content::ShowRecipe(recipe) => {
                *context_floor = (*context_floor).max(recipe.context_id_floor);
                for rule in recipe.rules.iter() {
                    *rule_floor = (*rule_floor).max(rule.id.saturating_add(1));
                }
            }
            Content::Ref(reference) => {
                if let Some(supplement) = &reference.supplement {
                    reserve_operational_source(
                        supplement,
                        locator,
                        context_floor,
                        rule_floor,
                    );
                }
                return Ok(Some(node.clone()));
            }
            Content::SetPage(_) | Content::PageRun(_) => {
                super::page_running::for_each_template(node, &mut |template| {
                    reserve_operational_source(
                        template,
                        locator,
                        context_floor,
                        rule_floor,
                    );
                });
                return super::page_running::map_body(node, |body| {
                    reserve_operational_source(body, locator, context_floor, rule_floor);
                    Ok(body.clone())
                });
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
    });
}

pub(super) fn next_capture_token(
    ctx: &mut EvalContext,
) -> Arc<crate::entities::version::Version> {
    let fresh = || Arc::new(crate::entities::version::Version::default());
    let Some(slots) = ctx
        .context_occurrence_slots
        .as_mut()
        .filter(|slots| slots.owner.is_some())
    else {
        return fresh();
    };
    let token = if let Some(token) = slots.capture_tokens.get(slots.capture_cursor) {
        token.clone()
    } else {
        let token = fresh();
        slots.capture_tokens.push(token.clone());
        token
    };
    slots.capture_cursor += 1;
    token
}

pub(super) fn with_running_cohort<R>(
    ctx: &mut EvalContext,
    stage: RunningStage,
    key: crate::compiler::layout::PageRunningKey,
    token: RunningToken,
    operation: impl FnOnce(&mut EvalContext) -> R,
) -> R {
    let mut caller = ctx
        .context_occurrence_slots
        .take()
        .expect("checked actual arena frame");
    let previous = caller
        .running_cohorts
        .iter()
        .position(|cohort| cohort.stage == stage && cohort.key == key)
        .map(|index| caller.running_cohorts.remove(index));
    let reusable = previous.filter(|cohort| {
        if !cohort.token.same(&token) {
            return false;
        }
        match cohort.slots.owner.as_ref().zip(caller.owner.as_ref()) {
            Some((retained, current)) => Arc::ptr_eq(retained, current),
            None => false,
        }
    });
    let mut slots = match reusable {
        Some(cohort) => cohort.slots,
        None => OccurrenceSlots {
            owner: caller.owner.clone(),
            ..OccurrenceSlots::default()
        },
    };
    slots.cursor = 0;
    slots.context_cursor = 0;
    slots.capture_cursor = 0;
    slots.running_active = true;
    ctx.context_occurrence_slots = Some(slots);
    let result = operation(ctx);
    let slots = ctx
        .context_occurrence_slots
        .take()
        .expect("running cohort remains installed");
    caller
        .running_cohorts
        .push(RunningCohort { stage, key, token, slots });
    ctx.context_occurrence_slots = Some(caller);
    result
}

pub(in crate::compiler) fn allocation_owner(ctx: &EvalContext) -> Option<Arc<()>> {
    ctx.context_occurrence_slots.as_ref()?.owner.clone()
}

pub(in crate::compiler) fn next_with_seed(
    ctx: &mut EvalContext,
) -> Option<(LocatorSeed, Location)> {
    allocation_owner(ctx)?;
    Some(next_allocation(ctx))
}

fn next_allocation(ctx: &mut EvalContext) -> (LocatorSeed, Location) {
    let Some(slots) = ctx.context_occurrence_slots.as_mut() else {
        let seed = ctx.realization_locator.checkpoint();
        return (seed, ctx.realization_locator.next());
    };
    let pair = if let Some(pair) = slots.locations.get(slots.cursor) {
        *pair
    } else {
        let pair = (ctx.realization_locator.checkpoint(), ctx.realization_locator.next());
        slots.locations.push(pair);
        pair
    };
    slots.cursor += 1;
    pair
}

pub(super) fn next(ctx: &mut EvalContext) -> Location {
    next_allocation(ctx).1
}

pub(super) fn next_context_id(ctx: &mut EvalContext) -> u64 {
    let proposed = ctx.next_context_id;
    let id = if let Some(slots) = ctx.context_occurrence_slots.as_mut() {
        let id = if let Some(id) = slots.context_ids.get(slots.context_cursor) {
            *id
        } else {
            slots.context_ids.push(proposed);
            proposed
        };
        slots.context_cursor += 1;
        id
    } else {
        proposed
    };
    ctx.next_context_id = ctx.next_context_id.max(id.saturating_add(1));
    id
}

pub(super) fn instantiate_context(
    source: &Arc<crate::entities::elements::context_block::ContextBlockElem>,
    ctx: &mut EvalContext,
) -> Arc<crate::entities::elements::context_block::ContextBlockElem> {
    let slot = ctx
        .context_occurrence_slots
        .as_ref()
        .filter(|slots| slots.running_active)
        .map(|slots| slots.context_cursor);
    let id = next_context_id(ctx);
    let fresh = || {
        Arc::new(crate::entities::elements::context_block::ContextBlockElem {
            id,
            closure: source.closure.clone(),
        })
    };
    let Some(index) = slot else { return fresh() };
    let slots = ctx
        .context_occurrence_slots
        .as_mut()
        .expect("running cohort stays installed");
    if slots.context_blocks.len() <= index {
        slots.context_blocks.resize_with(index + 1, || None);
    }
    let previous = &mut slots.context_blocks[index];
    if let Some(block) = previous {
        if block.id == id && std::ptr::eq(block.closure.repr(), source.closure.repr()) {
            return block.clone();
        }
    }
    let block = fresh();
    *previous = Some(block.clone());
    block
}

pub(super) fn instantiating_template(ctx: &EvalContext) -> bool {
    ctx.context_occurrence_slots
        .as_ref()
        .is_some_and(|slots| slots.instantiate_contexts)
}

pub(super) fn with_template<R>(
    ctx: &mut EvalContext,
    operation: impl FnOnce(&mut EvalContext) -> R,
) -> R {
    let temporary = ctx.context_occurrence_slots.is_none();
    let slots = ctx
        .context_occurrence_slots
        .get_or_insert_with(OccurrenceSlots::default);
    let old = std::mem::replace(&mut slots.instantiate_contexts, true);
    let result = operation(ctx);
    if temporary {
        ctx.context_occurrence_slots = None;
    } else if let Some(slots) = ctx.context_occurrence_slots.as_mut() {
        slots.instantiate_contexts = old;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::realize::locate;

    fn equation(ctx: &mut EvalContext) -> Content {
        locate(Content::equation(Content::text("x"), false), ctx)
    }

    fn location(content: &Content) -> Location {
        let Content::Located(e) = content else { panic!("prepared occurrence") };
        e.location
    }

    #[test]
    fn reserves_original_and_distinguishes_equal_producers_and_aliases() {
        let mut ctx = EvalContext::new();
        let origin = locate(Content::heading(1, Content::text("H")), &mut ctx);
        let mut arena = ContextOccurrenceArena::new(&origin);
        let mut first = arena.frame();
        let mut second = arena.frame();
        let a = arena.evaluate(&mut first, &mut ctx, equation);
        let b = arena.evaluate(&mut second, &mut ctx, equation);
        assert_ne!(location(&a), location(&origin));
        assert_ne!(location(&a), location(&b));
        let content = Content::sequence(vec![origin, a.clone(), a, b]);
        assert_eq!(introspect_with_introspector(&content).elements.len(), 3);
    }

    #[test]
    fn replay_reuses_slots_and_growth_does_not_steal_other_frames() {
        let mut ctx = EvalContext::new();
        let mut arena = ContextOccurrenceArena::new(&Content::Empty);
        let mut first = arena.frame();
        let mut second = arena.frame();
        let a = arena.evaluate(&mut first, &mut ctx, equation);
        let b = arena.evaluate(&mut second, &mut ctx, equation);
        let (again, extra) =
            arena.evaluate(&mut first, &mut ctx, |ctx| (equation(ctx), equation(ctx)));
        assert_eq!(location(&a), location(&again));
        assert_ne!(location(&extra), location(&b));
        let shrunk = arena.evaluate(&mut first, &mut ctx, equation);
        assert_eq!(location(&a), location(&shrunk));
        let current = Content::sequence(vec![shrunk, b]);
        assert_eq!(introspect_with_introspector(&current).elements.len(), 2);
        let recovered = arena.evaluate(&mut first, &mut ctx, |ctx| {
            let _ = equation(ctx);
            equation(ctx)
        });
        assert_eq!(location(&extra), location(&recovered));
    }

    #[test]
    fn error_restores_the_caller_allocator_and_retains_frame_slots() {
        let mut ctx = EvalContext::new();
        let _ = equation(&mut ctx);
        let before = ctx.realization_locator.checkpoint();
        let mut arena = ContextOccurrenceArena::new(&Content::Empty);
        let mut frame = arena.frame();
        let failed: Result<(), Content> =
            arena.evaluate(&mut frame, &mut ctx, |ctx| Err(equation(ctx)));
        assert_eq!(ctx.realization_locator.checkpoint(), before);
        assert!(ctx.context_occurrence_slots.is_none());
        let retried = arena.evaluate(&mut frame, &mut ctx, equation);
        assert_eq!(location(&failed.unwrap_err()), location(&retried));
    }

    #[test]
    fn reserves_canonical_unprepared_origins() {
        let origin = Content::sequence(vec![
            Content::heading(1, Content::text("H")),
            Content::equation(Content::text("y"), false),
        ]);
        let intr = introspect_with_introspector(&origin);
        let mut arena = ContextOccurrenceArena::new(&origin);
        let mut frame = arena.frame();
        let content = arena.evaluate(&mut frame, &mut EvalContext::new(), equation);
        assert!(!intr.elements.contains_key(&location(&content)));
        assert_eq!(
            introspect_with_introspector(&Content::sequence(vec![origin, content]))
                .elements
                .len(),
            3
        );
    }
    #[test]
    fn figure_reference_template_context_ids_are_fresh_per_frame_and_stable_after_retry_growth(
    ) {
        let mut arena = ContextOccurrenceArena::new(&Content::Empty);
        let mut first = arena.frame();
        let mut second = arena.frame();
        let mut ctx = EvalContext::new();
        let original_id = next_context_id(&mut ctx);
        let a = arena.evaluate(&mut first, &mut ctx, |ctx| {
            with_template(ctx, |ctx| {
                assert!(instantiating_template(ctx));
                next_context_id(ctx)
            })
        });
        let b = arena.evaluate(&mut second, &mut ctx, next_context_id);
        assert_ne!(a, b);
        assert_ne!(a, original_id);
        let (again, extra) = arena.evaluate(&mut first, &mut ctx, |ctx| {
            (next_context_id(ctx), next_context_id(ctx))
        });
        assert_eq!(again, a);
        assert_ne!(extra, b);
        let failed: Result<u64, u64> = arena.evaluate(&mut first, &mut ctx, |ctx| {
            with_template(ctx, |ctx| Err(next_context_id(ctx)))
        });
        assert_eq!(failed.unwrap_err(), a);
        assert!(!instantiating_template(&ctx));
        assert_eq!(arena.evaluate(&mut first, &mut ctx, next_context_id), a);
        assert!(ctx.next_context_id > extra.max(b));
    }

    #[test]
    fn figure_reference_template_scope_preserves_parent_allocator_on_error() {
        let mut ctx = EvalContext::new();
        let before = ctx.realization_locator.checkpoint();
        let result: Result<(), u64> = with_template(&mut ctx, |ctx| {
            let id = next_context_id(ctx);
            Err(id)
        });
        assert!(result.is_err());
        assert!(ctx.context_occurrence_slots.is_none());
        assert_eq!(ctx.realization_locator.checkpoint(), before);
    }

    #[test]
    fn physical_page_arena_reserves_dormant_alias_before_body_producer() {
        use crate::entities::{
            args::Args, content::SetPageDelta, elements::context_block::ContextBlockElem,
            file_id::FileId, func::Func, introspector::Introspector,
            page_running::PageMarginal, source_result::SourceResult, value::Value,
        };
        fn must_not_execute(
            _: &mut EvalContext,
            _: &Args,
            _: &dyn crate::contracts::world::World,
            _: FileId,
        ) -> SourceResult<Value> {
            panic!("arena reservation cannot execute a Context");
        }
        let alias_location = Location::from_raw(1);
        let header =
            Content::Located(Arc::new(crate::entities::content::LocatedContent {
                location: alias_location,
                body: Content::heading(1, Content::text("DORMANT")),
            }));
        let body = Content::Located(Arc::new(crate::entities::content::LocatedContent {
            location: Location::from_raw(0),
            body: Content::ContextBlock(Arc::new(ContextBlockElem {
                id: 7,
                closure: Func::native("must-not-execute", must_not_execute),
            })),
        }));
        let origin = Content::sequence(vec![
            Content::SetPage(Arc::new(SetPageDelta {
                header: Some(PageMarginal::Content(Arc::new(header))),
                ..SetPageDelta::default()
            })),
            body,
        ]);
        let mut arena = ContextOccurrenceArena::new(&origin);
        let mut frame = arena.frame();
        let mut ctx = EvalContext::new();
        let generated = arena.evaluate(&mut frame, &mut ctx, |ctx| {
            locate(Content::heading(1, Content::text("BODY PRODUCT")), ctx)
        });
        assert_ne!(
            location(&generated),
            alias_location,
            "a dormant prepared alias already owns this Location before body Context discovery"
        );
        assert!(
            introspect_with_introspector(&origin)
                .query_by_kind(crate::entities::element_kind::ElementKind::Heading)
                .is_empty(),
            "reservation cannot admit the dormant header"
        );
    }

    #[test]
    fn physical_page_arena_reserves_dormant_context_id_before_body_producer() {
        use crate::entities::{
            args::Args, content::SetPageDelta, elements::context_block::ContextBlockElem,
            file_id::FileId, func::Func, page_running::PageMarginal,
            source_result::SourceResult, value::Value,
        };
        fn inert(
            _: &mut EvalContext,
            _: &Args,
            _: &dyn crate::contracts::world::World,
            _: FileId,
        ) -> SourceResult<Value> {
            panic!("dormant Context reservation cannot execute its closure");
        }
        let header = Content::ContextBlock(Arc::new(ContextBlockElem {
            id: 77,
            closure: Func::native("dormant-context", inert),
        }));
        let origin = Content::SetPage(Arc::new(SetPageDelta {
            header: Some(PageMarginal::Content(Arc::new(header))),
            ..SetPageDelta::default()
        }));
        let mut arena = ContextOccurrenceArena::new(&origin);
        let mut frame = arena.frame();
        let produced = arena.evaluate(&mut frame, &mut EvalContext::new(), |ctx| {
            with_template(ctx, next_context_id)
        });
        assert!(
            produced > 77,
            "an existing dormant Context ID cannot be assigned to the first body producer"
        );
    }

    #[test]
    fn physical_page_arena_reserves_dormant_recipe_floors_before_body_producer() {
        use crate::entities::{
            args::Args,
            content::{SetPageDelta, ShowRecipeContent},
            file_id::FileId,
            func::Func,
            page_running::PageMarginal,
            show::{Selector, ShowRule, Transformation},
            source_result::SourceResult,
            style_chain::StyleChain,
            value::Value,
        };
        fn inert(
            _: &mut EvalContext,
            _: &Args,
            _: &dyn crate::contracts::world::World,
            _: FileId,
        ) -> SourceResult<Value> {
            panic!("dormant show reservation cannot execute its transformation");
        }
        let file = FileId::from_raw(std::num::NonZeroU16::new(1).unwrap());
        let header = Content::ShowRecipe(Arc::new(ShowRecipeContent {
            body: Content::text("DORMANT"),
            textual: false,
            context_id_floor: 101,
            rules: Arc::from([ShowRule {
                id: 150,
                selector: Selector::Text("DORMANT".into()),
                transform: Transformation::Func(Func::native("dormant-show", inert)),
            }]),
            styles: StyleChain::default_chain(),
            guards: vec![150],
            file,
        }));
        let origin = Content::SetPage(Arc::new(SetPageDelta {
            header: Some(PageMarginal::Content(Arc::new(header))),
            ..SetPageDelta::default()
        }));
        let mut arena = ContextOccurrenceArena::new(&origin);
        let mut frame = arena.frame();
        let (context, rule) =
            arena.evaluate(&mut frame, &mut EvalContext::new(), |ctx| {
                (with_template(ctx, next_context_id), ctx.next_rule_id)
            });
        assert!(
            rule > 150,
            "the first body producer must start beyond captured rule IDs"
        );
        assert!(context >= 101, "the captured recipe Context floor must also survive");
    }
}
