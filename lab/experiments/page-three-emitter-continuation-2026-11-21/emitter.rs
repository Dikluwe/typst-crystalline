use super::paragraph_session::Product;
use crate::entities::layout_types::TextStyle;
use std::sync::Arc;

struct SelectedEmission<'a> {
    prepared: &'a Prepared,
    lines: &'a [Line],
    width: f64,
    justify: bool,
    owner: Option<&'a Arc<Product>>,
    lend: bool,
}

enum SelectedResult {
    Complete(Option<super::paragraph_paint::CloseTail>),
    Suspended(Box<PageThreeEmitter>),
}

/// Emission of a line whose admission already advanced the region.
/// The active Layouter and enclosing source traversal are retained by the caller.
struct PageThreeEmitter {
    product: Arc<Product>,
    index: usize,
    width: f64,
    justify: bool,
    lend: bool,
    restore_style: TextStyle,
    resume_entry: [u64; 6],
}

fn lab_entry<M: FontMetrics, S: ImageSizer>(l: &Layouter<M, S>) -> [u64; 6] {
    let region = &l.regions.current;
    [
        l.pages.len() as u64,
        region.cursor_x.0.to_bits(),
        region.cursor_y.0.to_bits(),
        region.line_start_x.0.to_bits(),
        region.current_line.len() as u64,
        region.current_items.len() as u64,
    ]
}

fn lab_suspend_emitter(
    eligible: bool, before: usize, after: usize, index: usize, count: usize,
) -> bool {
    eligible && before == 1 && after == 2 && index.checked_add(1) == Some(count)
}

#[inline(never)]
fn lab_resume_page_three<M: FontMetrics, S: ImageSizer>(
    l: &mut Layouter<M, S>,
    checkpoint: Box<PageThreeEmitter>,
) -> Option<super::paragraph_paint::CloseTail> {
    let PageThreeEmitter { product, index, width, justify, lend, restore_style, resume_entry } =
        *checkpoint;
    assert_eq!(lab_entry(l), resume_entry, "live region changed during emitter suspension");
    let result = emit_line(
        l, &product.prepared, product.lines[index], width, justify, false,
        Some((&product.paint_id, index)), lend,
    );
    l.style = restore_style;
    result
}

pub(super) fn emit_product<M: FontMetrics, S: ImageSizer>(
    l: &mut Layouter<M, S>,
    product: &Arc<Product>,
    width: f64,
    justify: bool,
) {
    let _ = emit_source_product(l, product, width, justify, false);
}

// Only the terminal line can hand its retained width to the source boundary.
pub(super) fn emit_source_product<M: FontMetrics, S: ImageSizer>(
    l: &mut Layouter<M, S>,
    product: &Arc<Product>,
    width: f64,
    justify: bool,
    lend: bool,
) -> Option<super::paragraph_paint::CloseTail> {
    let identity = product.paintable.then_some(&product.paint_id);
    match (&product.hyphenated, identity) {
        (Some(lines), Some(id)) => emit_prepared_impl(l, lines, width, justify, Some(id), lend),
        (None, Some(_)) => match emit_impl(l, SelectedEmission {
            prepared: &product.prepared,
            lines: &product.lines,
            width,
            justify,
            owner: Some(product),
            lend,
        }) {
            SelectedResult::Complete(result) => result,
            SelectedResult::Suspended(checkpoint) => lab_resume_page_three(l, checkpoint),
        },
        (Some(lines), None) => {
            emit_prepared_lines(l, lines, width, justify);
            None
        }
        (None, None) => {
            emit(l, &product.prepared, &product.lines, width, justify);
            None
        }
    }
}

pub(super) fn emit<M: FontMetrics, S: ImageSizer>(
    l: &mut Layouter<M, S>,
    prepared: &Prepared,
    lines: &[Line],
    width: f64,
    justify: bool,
) {
    let _ = emit_impl(l, SelectedEmission {
        prepared, lines, width, justify, owner: None, lend: false,
    });
}

#[inline(never)]
fn emit_impl<M: FontMetrics, S: ImageSizer>(
    l: &mut Layouter<M, S>,
    input: SelectedEmission<'_>,
) -> SelectedResult {
    let SelectedEmission { prepared, lines, width, justify, owner, lend } = input;
    super::paragraph_indent::start(l);
    let mut tail = None;
    let saved = l.style.clone();
    let pagination = super::paragraph_pagination::certify_selected(l, prepared, lines);
    let identity = owner.map(|product| &product.paint_id);
    let suspend_owner = owner.filter(|_| l.lab_capture_outline);
    for (index, &line) in lines.iter().enumerate() {
        let before = l.pages.len();
        super::paragraph_pagination::admit(l, pagination, index, lines.len());
        if let Some(product) = suspend_owner {
            if lab_suspend_emitter(true, before, l.pages.len(), index, lines.len()) {
                return SelectedResult::Suspended(Box::new(PageThreeEmitter {
                    product: Arc::clone(product),
                    index,
                    width,
                    justify,
                    lend,
                    restore_style: saved,
                    resume_entry: lab_entry(l),
                }));
            }
        }
        tail = emit_line(
            l, prepared, line, width, justify, index + 1 < lines.len(),
            identity.map(|id| (id, index)), lend,
        );
    }
    l.style = saved;
    SelectedResult::Complete(tail)
}
