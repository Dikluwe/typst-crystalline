
type LabIndexOccurrence = (
    PacketId, PlanPhase, std::num::NonZeroUsize,
    crate::compiler::layout::page_running_requests::TraceSection,
);

pub(in crate::compiler) struct LabSemanticSnapshot {
    source: Arc<SourcePlan>,
    positions: crate::entities::sealed_positions::SealedPositions,
    pages: crate::entities::page_store::PageStore,
    occurrences: Vec<LabIndexOccurrence>,
    starts: Vec<LocatorSeed>,
    slots: Vec<(LocatorSeed, Location)>,
    index: TagIntrospector,
    features: Features,
}

fn lab_index_occurrences(
    trace: &crate::compiler::layout::PageRunningTrace,
) -> Option<Vec<LabIndexOccurrence>> {
    let mut semantic = Vec::new();
    for event in trace.ordered() {
        if !Arc::ptr_eq(&event.plan, &trace.data.source) {
            return None;
        }
        let node = event.plan.node(event.packet)?;
        if node.location.is_some() || !node.auxiliary_locations.is_empty()
            || node.label.is_some() || event.plan.required(event.packet, event.phase)
        {
            semantic.push((event.packet, event.phase, event.page, event.section));
        }
    }
    Some(semantic)
}

fn lab_index_payload_agrees(a: &Content, b: &Content) -> bool {
    match (a,b) {
        (Content::Heading(a), Content::Heading(b)) => {
            a.level == b.level && a.outlined == b.outlined && a.bookmarked == b.bookmarked
                && a.set_fields == b.set_fields && a.body.plain_text() == b.body.plain_text()
        }
        (Content::Label(a), Content::Label(b)) => a.name == b.name && a.auto == b.auto,
        (Content::Located(a), Content::Located(b)) => a.location == b.location,
        (Content::Ref(a), Content::Ref(b)) => a.name == b.name && a.form == b.form
            && a.supplement.is_none() && b.supplement.is_none(),
        (Content::Outline(a), Content::Outline(b)) => {
            a.target == b.target && a.depth == b.depth && a.parent_key() == b.parent_key()
                && a.show_rules().is_empty() && b.show_rules().is_empty()
                && a.captured_styles() == b.captured_styles()
                && a.title.as_ref().map(Content::plain_text) == b.title.as_ref().map(Content::plain_text)
                && a.realized_body().map(Content::plain_text) == b.realized_body().map(Content::plain_text)
        }
        (Content::OutlineEntry(a), Content::OutlineEntry(b)) => {
            a.level == b.level && a.element_location() == b.element_location()
                && a.fill_set == b.fill_set && a.fill.as_ref().map(Content::plain_text) == b.fill.as_ref().map(Content::plain_text)
                && a.element.plain_text() == b.element.plain_text()
                && a.prefix().as_ref().map(Content::plain_text) == b.prefix().as_ref().map(Content::plain_text)
        }
        (Content::ContextBlock(a), Content::ContextBlock(b)) => a.id == b.id
            && crate::compiler::realize::reference_supplements::is_reference_supplement_producer(a)
            && crate::compiler::realize::reference_supplements::is_reference_supplement_producer(b),
        (Content::Styled(a), Content::Styled(b)) => a.styles == b.styles,
        (Content::Source(a), Content::Source(b)) => a.span == b.span,
        _ => false,
    }
}

impl LabSemanticSnapshot {
    fn rebind(
        &self,
        source: &SourcePlan,
        occurrences: &[LabIndexOccurrence],
    ) -> Result<(), usize> {
        if self.occurrences != occurrences {
            return Err(3);
        }
        for &(packet, _, _, _) in occurrences {
            let (Some(a), Some(b)) = (self.source.node(packet), source.node(packet)) else {
                return Err(4);
            };
            if a.role != b.role || a.source_parent != b.source_parent
                || a.environment != b.environment || a.label != b.label || a.location != b.location
                || a.auxiliary_locations != b.auxiliary_locations || a.lang != b.lang
                || a.span != b.span || a.math != b.math || a.original != b.original
                || a.conditional_ref != b.conditional_ref
                || !lab_index_payload_agrees(&a.content, &b.content)
            {
                return Err(10_000 + packet.0);
            }
            let (Some(a),Some(b)) = (self.source.environment(a.environment), source.environment(b.environment)) else {
                return Err(4);
            };
            if !a.chain.raw_deltas().eq(b.chain.raw_deltas()) {
                return Err(20_000 + packet.0);
            }
        }
        Ok(())
    }
}

#[inline(never)]
fn lab_index_reuse_receipt(result: usize) {
    std::hint::black_box(result);
}

/// Called only through the closed references fixture where the prior physical
/// product was already certified. Single paged document, no running requests.
pub fn lab_introspect_suffix_index(
    documents: &[PageRunningIndexInput<'_>],
    features: Features,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
    session: &crate::compiler::layout::paragraph_session::ParagraphSession,
) -> SourceResult<(TagIntrospector, Vec<LocatorSeed>)> {
    use crate::compiler::realize::context_occurrences::{lab_consumed_index_slots, lab_replay_index_slots};
    let [input] = documents else { return introspect_with_page_running(documents,features,ctx,engine) };
    let Some((trace,running)) = input.running else {
        return introspect_with_page_running(documents,features,ctx,engine);
    };
    if input.target != EvalTarget::Paged || !trace.data.complete
        || !running.resolutions().is_empty() || !trace.data.requests.is_empty()
        || !Arc::ptr_eq(&trace.data.source, &input.body.plan)
    {
        return introspect_with_page_running(documents,features,ctx,engine);
    }
    let Some(semantic) = lab_index_occurrences(trace) else {
        return introspect_with_page_running(documents,features,ctx,engine);
    };
    if let Some(old_source) = trace.data.lab_index_prior.as_ref() {
        let cached=session.lab_previous_index.lock().unwrap().clone();
        let hit = (|| {
            let memo=cached.as_ref().ok_or(1_usize)?;
            if !Arc::ptr_eq(&memo.source, old_source) || features != memo.features
                || !memo.pages.same_scalar_snapshot(&trace.data.pages)
                || !memo.positions.lab_exact_subset(&trace.data.positions)
                || !trace.data.positions.lab_exact_subset(&memo.positions)
                || memo.source.document_boundary != trace.data.source.document_boundary
            {
                return Err(2);
            }
            memo.rebind(&trace.data.source, &semantic)?;
            if !lab_replay_index_slots(ctx, &memo.slots) {
                return Err(5);
            }
            Ok((memo.index.clone(), memo.starts.clone()))
        })();
        match hit {
            Ok(index) => {
                lab_index_reuse_receipt(100_000 + semantic.len());
                return Ok(index);
            }
            Err(reason) => lab_index_reuse_receipt(reason),
        }
    }
    let (index, starts) = introspect_with_page_running(documents,features,ctx,engine)?;
    if trace.data.lab_index_prior.is_none() {
        if let Some(slots) = lab_consumed_index_slots(ctx) {
            *session.lab_previous_index.lock().unwrap() = Some(Arc::new(LabSemanticSnapshot {
                source: trace.data.source.clone(), positions: trace.data.positions.clone(),
                pages: trace.data.pages.clone(), occurrences: semantic, starts: starts.clone(),
                slots, index: index.clone(), features,
            }));
        }
    }
    Ok((index, starts))
}
