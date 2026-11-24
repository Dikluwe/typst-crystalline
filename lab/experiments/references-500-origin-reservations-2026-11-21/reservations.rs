
/// Reservation has two independent, monotone effects. Keep the original
/// Location object so the merge still goes through Locator::observe.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct LabOriginFacts {
    location: Option<Location>,
    context_floor: u64,
}

impl LabOriginFacts {
    fn observe(&mut self, location: Location) {
        if self.location.is_none_or(|old| old.as_u128() < location.as_u128()) {
            self.location = Some(location);
        }
    }

    fn merge(&mut self, other: Self) {
        if let Some(location) = other.location {
            self.observe(location);
        }
        self.context_floor = self.context_floor.max(other.context_floor);
    }

    fn apply(self, ctx: &mut EvalContext) {
        if let Some(location) = self.location {
            ctx.realization_locator.observe(location);
        }
        ctx.next_context_id = ctx.next_context_id.max(self.context_floor);
    }
}

#[derive(Debug, Clone)]
struct LabOriginTable {
    sequence: Arc<[Content]>,
    suffixes: Vec<(usize, LabOriginFacts)>,
}

impl LabOriginTable {
    fn suffix(&self, start: usize) -> LabOriginFacts {
        let index = self.suffixes.partition_point(|(index, _)| *index < start);
        self.suffixes.get(index).map_or_else(LabOriginFacts::default, |entry| entry.1)
    }
}

#[derive(Clone, Copy)]
struct LabOriginDonor<'a> {
    table: &'a LabOriginTable,
    anchor: PacketId,
    start: usize,
}

impl<'a> LabOriginDonor<'a> {
    fn new(plan: &'a SourcePlan, frontier: usize) -> Option<Self> {
        if plan.lab_suffix.is_some() {
            return None;
        }
        let table = plan.lab_origin_table.as_ref()?;
        let anchor = plan.lab_marks.keys().copied().find(|&anchor| {
            matches!(plan.node(anchor).map(|node| &node.content),
                Some(Content::Sequence(parts)) if Arc::ptr_eq(parts, &table.sequence))
        })?;
        let children = &plan.node(anchor)?.children;
        let start = children.partition_point(|(_, packet)| packet.0 < frontier);
        let &(edge, _) = children.get(start)?;
        (edge == PlanEdge::SequencePart(start)).then_some(Self { table, anchor, start })
    }
}

struct LabOriginProof {
    sequence: Arc<[Content]>,
    anchor: PacketId,
    start: usize,
    end: usize,
}

impl LabOriginProof {
    fn covers(&self, sequence: &Arc<[Content]>, anchor: PacketId, start: usize, end: usize) -> bool {
        self.anchor == anchor && self.start == start && self.end == end
            && Arc::ptr_eq(&self.sequence, sequence)
    }
}

struct LabOriginScan<'a> {
    donor: Option<LabOriginDonor<'a>>,
    table: Option<LabOriginTable>,
    proof: Option<LabOriginProof>,
    capture: bool,
}

impl LabOriginScan<'_> {
    fn sequence(
        &mut self,
        parts: &Arc<[Content]>,
        facts: &mut LabOriginFacts,
    ) -> SourceResult<bool> {
        if self.proof.is_some() || parts.len() < 256 {
            return Ok(false);
        }
        if let Some(donor) = self.donor {
            let old = &donor.table.sequence;
            if let Ok(end) = lab_shared_part_end(old, parts) {
                if end.saturating_sub(donor.start) >= 128
                    && lab_reserve_jump_enabled()
                    && old[donor.start..end].iter().zip(&parts[donor.start..end])
                        .try_for_each(|(a, b)| lab_block_content(a, b)).is_ok()
                {
                    self.proof = Some(LabOriginProof {
                        sequence: parts.clone(), anchor: donor.anchor, start: donor.start, end,
                    });
                    for part in parts[..donor.start].iter().chain(&parts[end..]) {
                        reserve_source(part, facts, self)?;
                    }
                    facts.merge(donor.table.suffix(donor.start));
                    return Ok(true);
                }
            }
        }
        if !self.capture {
            return Ok(false);
        }
        self.capture = false;
        let donor = self.donor.take();
        let mut suffixes = Vec::new();
        for (index, part) in parts.iter().enumerate() {
            let mut part_facts = LabOriginFacts::default();
            reserve_source(part, &mut part_facts, self)?;
            if part_facts != LabOriginFacts::default() {
                suffixes.push((index, part_facts));
            }
        }
        self.donor = donor;
        let mut accumulated = LabOriginFacts::default();
        for (_, facts) in suffixes.iter_mut().rev() {
            accumulated.merge(*facts);
            *facts = accumulated;
        }
        facts.merge(accumulated);
        self.table = Some(LabOriginTable { sequence: parts.clone(), suffixes });
        Ok(true)
    }
}

fn lab_shared_part_end(old: &[Content], current: &[Content]) -> Result<usize, usize> {
    let common = old.len().min(current.len());
    let end = if old.len() == current.len() { common } else { common.saturating_sub(8) };
    if old.len().abs_diff(current.len()) > 1
        || !old[end..].iter().chain(&current[end..]).all(|part| lab_same_leaf(part, part))
    {
        return Err(23);
    }
    Ok(end)
}

// The uninstrumented lab uses the jump. A separate debugger control compares
// the same ELF's full reservation by returning false here.
#[inline(never)]
fn lab_reserve_jump_enabled() -> bool {
    std::hint::black_box(true)
}

#[inline(never)]
fn lab_reserve_receipt(skipped: usize, context_floor: u64, after_location: u128) {
    std::hint::black_box((skipped, context_floor, after_location));
}
