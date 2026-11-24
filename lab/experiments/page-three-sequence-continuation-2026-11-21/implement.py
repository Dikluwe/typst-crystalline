# Executed by stage.py with the isolated paths and exact-replacement helper.
s=rust['emit'].read_text()
s=change(s,'enum SelectedResult {','pub(super) enum SelectedResult {')
s=change(s,'struct PageThreeEmitter {','pub(super) struct PageThreeEmitter {')
s=change(s,'fn lab_resume_page_three<M:', 'pub(super) fn lab_resume_page_three<M:')
a=s.index('    let identity = product.paintable.then_some(&product.paint_id);',s.index('pub(super) fn emit_source_product'))
b=s.index('\npub(super) fn emit<M',a)
s=s[:a]+'''    match lab_begin_source_product(l, product, width, justify, lend) {
        SelectedResult::Complete(result) => result,
        SelectedResult::Suspended(checkpoint) => lab_resume_page_three(l, checkpoint),
    }
}

pub(super) fn lab_begin_source_product<M: FontMetrics, S: ImageSizer>(
    l: &mut Layouter<M, S>,
    product: &Arc<Product>,
    width: f64,
    justify: bool,
    lend: bool,
) -> SelectedResult {
    let identity = product.paintable.then_some(&product.paint_id);
    match (&product.hyphenated, identity) {
        (Some(lines), Some(id)) => SelectedResult::Complete(
            emit_prepared_impl(l, lines, width, justify, Some(id), lend),
        ),
        (None, Some(_)) => emit_impl(l, SelectedEmission {
            prepared: &product.prepared,
            lines: &product.lines,
            width,
            justify,
            owner: Some(product),
            lend,
        }),
        (Some(lines), None) => {
            emit_prepared_lines(l, lines, width, justify);
            SelectedResult::Complete(None)
        }
        (None, None) => {
            emit(l, &product.prepared, &product.lines, width, justify);
            SelectedResult::Complete(None)
        }
    }
}
''' + s[b:]
rust['emit'].write_text(s)

s=rust['plan'].read_text()
a=s.index('pub(super) fn try_parts<M:')
b=s.index('    let Some((strategy, width, font, justify))',s.index('fn try_parts_impl',a))
s=s[:a]+'''type CloseTail = super::paragraph_paint::CloseTail;

pub(super) enum SourceParagraph {
    Refused,
    Complete(Option<CloseTail>),
    Suspended(ParagraphCheckpoint),
}

pub(super) struct ParagraphCheckpoint {
    emitter: Box<paragraph_emit::PageThreeEmitter>,
    next: usize,
    count: usize,
}

#[inline(never)]
pub(super) fn lab_resume_paragraph<M: FontMetrics, S: ImageSizer>(
    l: &mut Layouter<M, S>,
    checkpoint: ParagraphCheckpoint,
) -> Option<CloseTail> {
    assert_eq!(checkpoint.next, checkpoint.count, "only terminal source segment was suspended");
    paragraph_emit::lab_resume_page_three(l, checkpoint.emitter)
}

pub(super) fn try_parts<M: FontMetrics, S: ImageSizer>(
    l: &mut Layouter<M, S>,
    parts: &[Content],
) -> bool {
    match try_parts_impl(l, parts, false) {
        SourceParagraph::Refused => false,
        SourceParagraph::Complete(_) => true,
        SourceParagraph::Suspended(checkpoint) => {
            let _ = lab_resume_paragraph(l, checkpoint);
            true
        }
    }
}

pub(super) fn try_source_parts<M: FontMetrics, S: ImageSizer>(
    l: &mut Layouter<M, S>,
    parts: &[Content],
) -> SourceParagraph {
    try_parts_impl(l, parts, true)
}

#[inline(never)]
fn try_parts_impl<M: FontMetrics, S: ImageSizer>(
    l: &mut Layouter<M, S>,
    parts: &[Content],
    lend: bool,
) -> SourceParagraph {
''' + s[b:]
a=s.index('    let Some((strategy, width, font, justify))',s.index('fn try_parts_impl'))
b=s.index('\n#[cfg(test)]',a)
body=s[a:b].replace('return false','return SourceParagraph::Refused')
body=change(body,'    for (index, segment) in prepared.iter().enumerate() {','    let mut close = None;\n    for (index, segment) in prepared.iter().enumerate() {')
body=change(body,'''                if let Some(close) = close.as_deref_mut().filter(|_| index + 1 == prepared.len()) {
                    *close = paragraph_emit::emit_source_product(l, product, width, justify, true);
                } else {''','''                if lend && index + 1 == prepared.len() {
                    match paragraph_emit::lab_begin_source_product(l, product, width, justify, true) {
                        paragraph_emit::SelectedResult::Complete(tail) => close = tail,
                        paragraph_emit::SelectedResult::Suspended(emitter) => {
                            let checkpoint = ParagraphCheckpoint {
                                emitter, next: index + 1, count: prepared.len(),
                            };
                            if prepared.len() == 1 {
                                return SourceParagraph::Suspended(checkpoint);
                            }
                            close = lab_resume_paragraph(l, checkpoint);
                        }
                    }
                } else {''')
assert body.endswith('    true\n}\n')
body=body[:-len('    true\n}\n')]+'    SourceParagraph::Complete(close)\n}\n'
s=s[:a]+body+s[b:]
rust['plan'].write_text(s)

s=rust['sequence'].read_text()
stub='\n#[cfg(test)]\nfn lab_sequence_boundary(_: &[Content], _: usize, _: usize) -> bool { false }\n'
assert s.endswith(stub)
s=s[:-len(stub)]
s=change(s,'/// One complete real paragraph, certified under the current original source cursor.',(exp/'sequence.rs').read_text()+'\n/// One complete real paragraph, certified under the current original source cursor.')
s=change(s,'fn try_source_segment<M:', '#[inline(never)]\nfn try_source_segment<M:')
s=change(s,') -> Option<(usize, Option<super::paragraph_paint::CloseTail>)> {',') -> Option<(usize, super::paragraph_plan::SourceParagraph)> {')
s=change(s,'''    let mut tail = None;
    if !super::paragraph_plan::try_source_parts(l, &parts[start..end], &mut tail) {
        l.paragraph_indent = saved_indent;
        return None;
    }
    // Every admitted paragraph''','''    let outcome = super::paragraph_plan::try_source_parts(l, &parts[start..end]);
    if matches!(outcome, super::paragraph_plan::SourceParagraph::Refused) {
        l.paragraph_indent = saved_indent;
        return None;
    }
    Some((end, outcome))
}

fn finish_source_segment<M: FontMetrics, S: ImageSizer>(l: &mut Layouter<M, S>) {
    // Every admitted paragraph''')
s=change(s,'    Some((end, tail))\n}', '}')
s=change(s,'''            if let Some((end, tail)) = try_source_segment(layouter, parts, part_index, planned) {
                previous_tail = tail;
                for _ in part_index + 1..end {''','''            if let Some((end, outcome)) = try_source_segment(layouter, parts, part_index, planned) {
                let (end, tail) = match outcome {
                    super::paragraph_plan::SourceParagraph::Complete(tail) => (end, tail),
                    super::paragraph_plan::SourceParagraph::Suspended(paragraph) => {
                        let route = match planned {
                            Some(PartRoutes::Source(routes)) => routes.get(end),
                            Some(PartRoutes::Mixed(routes)) => routes.get(end).and_then(Option::as_ref),
                            None => layouter.page_running_pass.as_ref().and_then(|pass| pass.cursor.as_ref()),
                        }.cloned();
                        lab_resume_sequence(layouter, SequenceCheckpoint {
                            parts, route, start: part_index, end,
                            text_segment_start, owns_text_segment, paragraph,
                        })
                    }
                    super::paragraph_plan::SourceParagraph::Refused =>
                        unreachable!("refused source segment has no continuation"),
                };
                finish_source_segment(layouter);
                previous_tail = tail;
                for _ in part_index + 1..end {''')
rust['sequence'].write_text(s)

s=rust['session'].read_text()
s=change(s,'    memo: Arc<Mutex<Memo>>,','''    memo: Arc<Mutex<Memo>>,
    pub(super) lab_previous_physical:
        Arc<Mutex<Option<Arc<super::page_running_requests::PageRunningDocument>>>>,''')
rust['session'].write_text(s)

s=rust['memo'].read_text()
mark=s.index('pub fn lab_remember(')
a=s[:mark]
b=s[mark:]
b=change(b[:b.index('#[inline(never)]')],'''    let product = Arc::new(product);
    session.0.lock().unwrap().entry = Some(Entry {''','''    let product = Arc::new(product);
    if let Some(paragraphs) = metrics.paragraph_session() {
        *paragraphs.lab_previous_physical.lock().unwrap() = Some(Arc::clone(&product));
    }
    session.0.lock().unwrap().entry = Some(Entry {''')+b[b.index('#[inline(never)]'):]
rust['memo'].write_text(a+b)
