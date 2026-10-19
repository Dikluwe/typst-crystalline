//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/realize/context_occurrences.md
//! @prompt-hash fcc9f12a
//! @layer L1
//! @updated 2026-10-18

use std::sync::Arc;

use crate::compiler::eval::EvalContext;
use crate::compiler::introspect::{
    introspect_with_introspector, walk::reserve_locations,
};
use crate::entities::content::Content;
use crate::entities::location::Location;
use crate::entities::locator::Locator;

/// One allocation domain for the contextual producers of a compilation session.
pub struct ContextOccurrenceArena {
    locator: Locator,
    owner: Arc<()>,
}

/// A producer's retained allocation slots, replayed in actual execution order.
pub struct ContextOccurrenceFrame {
    slots: OccurrenceSlots,
    owner: Arc<()>,
}

#[derive(Default)]
pub(crate) struct OccurrenceSlots {
    locations: Vec<Location>,
    cursor: usize,
}

impl ContextOccurrenceArena {
    /// Reserve the canonical occurrences of the actual original content.
    pub fn new(origin: &Content) -> Self {
        let mut locator = Locator::new();
        reserve_locations(origin, &mut locator);
        for location in introspect_with_introspector(origin).elements.keys() {
            locator.observe(*location);
        }
        Self { locator, owner: Arc::new(()) }
    }

    /// Issue an exclusive frame for a new producer in this arena.
    pub fn frame(&mut self) -> ContextOccurrenceFrame {
        ContextOccurrenceFrame {
            slots: OccurrenceSlots::default(),
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
        frame.slots.cursor = 0;
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

pub(super) fn next(ctx: &mut EvalContext) -> Location {
    let Some(slots) = ctx.context_occurrence_slots.as_mut() else {
        return ctx.realization_locator.next();
    };
    let location = if let Some(location) = slots.locations.get(slots.cursor) {
        *location
    } else {
        let location = ctx.realization_locator.next();
        slots.locations.push(location);
        location
    };
    slots.cursor += 1;
    location
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
}
