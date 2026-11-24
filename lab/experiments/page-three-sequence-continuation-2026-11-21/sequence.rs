struct SequenceCheckpoint<'a> {
    parts: &'a [Content],
    route: Option<super::page_running_requests::Cursor>,
    start: usize,
    end: usize,
    text_segment_start: usize,
    owns_text_segment: bool,
    paragraph: super::paragraph_plan::ParagraphCheckpoint,
}

fn lab_sequence_boundary(parts: &[Content], start: usize, end: usize) -> bool {
    start < end
        && matches!(parts.get(end).map(Content::source_body), Some(Content::Parbreak))
}

/// Words in the laboratory receipt have a stable ABI for the exact-ELF census.
#[inline(never)]
fn lab_suffix_trial_receipt(receipt: &[u64; 11]) {
    std::hint::black_box(receipt);
}

#[inline(never)]
fn lab_resume_sequence<M: FontMetrics, S: ImageSizer>(
    l: &mut Layouter<'_, M, S>,
    checkpoint: SequenceCheckpoint<'_>,
) -> (usize, Option<super::paragraph_paint::CloseTail>) {
    let SequenceCheckpoint {
        parts, route, start, end, text_segment_start, owns_text_segment, paragraph,
    } = checkpoint;
    assert!(lab_sequence_boundary(parts, start, end), "separator changed during suspension");
    let previous = l.metrics.paragraph_session()
        .and_then(|session| session.lab_previous_physical.lock().unwrap().clone());
    let current = l.page_running_pass.as_ref().map(|pass| &pass.source);
    let old = previous.as_ref();
    let prior_pages = old.map_or(0, |product| product.document.pages.len());
    let same_plan = old.zip(current).is_some_and(|(old, plan)|
        std::sync::Arc::ptr_eq(&old.trace.data.source, plan));
    let same_owner = old.zip(current).is_some_and(|(old, plan)| {
        old.trace.data.source.allocation_owner.as_ref().zip(plan.allocation_owner.as_ref())
            .is_some_and(|(old, current)| std::sync::Arc::ptr_eq(old, current))
    });
    let pagination = l.runtime.known_page_store.total_pages().map_or(0, |n| n.get());
    // This is a pre-use certificate test. Unproven positions and reference
    // reads preclude a join even when its first three gates pass.
    let refusal = if old.is_none() {
        1 // cold or no valid previous physical result
    } else if !(same_plan && same_owner) {
        2 // cannot rebind the prior positional trace by ordinal
    } else if pagination == 0 || prior_pages != pagination {
        3 // no equal current page extent
    } else {
        4 // no validated positions/reference join
    };
    lab_suffix_trial_receipt(&[
        start as u64, end as u64, parts.len() as u64, text_segment_start as u64,
        owns_text_segment as u64, route.as_ref().map_or(u64::MAX, |cursor| cursor.packet.0 as u64),
        prior_pages as u64, same_plan as u64, same_owner as u64, pagination as u64, refusal,
    ]);
    (end, super::paragraph_plan::lab_resume_paragraph(l, paragraph))
}
