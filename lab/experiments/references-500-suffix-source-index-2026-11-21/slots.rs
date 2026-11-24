
pub(in crate::compiler) fn lab_consumed_index_slots(
    ctx: &EvalContext,
) -> Option<Vec<(LocatorSeed, Location)>> {
    let slots = ctx.context_occurrence_slots.as_ref()?;
    if slots.cursor > 256 || slots.context_cursor != 0 || slots.capture_cursor != 0 {
        return None;
    }
    Some(slots.locations.get(..slots.cursor)?.to_vec())
}

pub(in crate::compiler) fn lab_replay_index_slots(
    ctx: &mut EvalContext,
    expected: &[(LocatorSeed, Location)],
) -> bool {
    let Some(slots) = ctx.context_occurrence_slots.as_ref() else { return false };
    let Some(rest) = slots.locations.get(slots.cursor..) else { return false };
    let mut locator = Locator::from_seed(ctx.realization_locator.checkpoint());
    for (i, &pair) in expected.iter().enumerate() {
        let actual = rest.get(i).copied().unwrap_or_else(|| (locator.checkpoint(), locator.next()));
        if actual != pair {
            return false;
        }
    }
    for _ in expected {
        let _ = next_allocation(ctx);
    }
    true
}
