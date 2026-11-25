//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/terms.md
//! @prompt-hash 194506e3
//! @layer L1
//! @updated 2026-06-19
//!
//! Atomização (ADR-0109, P380): o layout de `Terms` movido do monólito
//! `layout_content` para o arquivo da feature (forma B). Content-preserving.

use crate::compiler::realize::lists;
use crate::compiler::stdlib::list_like;
use crate::entities::content::Content;
use crate::entities::dir::Dir;
use crate::entities::elements::block::BlockElem;
use crate::entities::elements::terms::{ListKind, TermsElem};
use crate::entities::layout_types::{
    Align2D, FrameItem, HAlign, Length, Point, Pt, TextStyle, TransformMatrix, VAlign,
};
use crate::entities::rel::Rel;
use crate::entities::style::Styles;
use crate::entities::value::Value;

use super::sub_frame::{IntrinsicPiece, SubLayoutRegion};
use super::{FontMetrics, ImageSizer, Layouter};

/// Geometria de um item dentro do seu conjunto.
pub(super) struct ItemFrame {
    above: Option<f64>,
    below: Option<f64>,
    /// Recuo total do corpo, do lado do início.
    offset: f64,
    label: Option<Label>,
    styles: Styles,
    pub(super) hanging: f64,
    pub(super) separator: Content,
}

struct Label {
    piece: IntrinsicPiece,
    x: f64,
    vertical: Option<VAlign>,
}

fn points<M: FontMetrics, S: ImageSizer>(
    l: &Layouter<M, S>,
    e: &TermsElem,
    name: &str,
) -> Option<f64> {
    if let Value::Length(length) = list_like::property(e, name, &l.chain) {
        Some(length.resolve_pt(l.style.size.val()))
    } else {
        None
    }
}

pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e: &TermsElem,
) {
    let children = e.children();
    let size = layouter.style.size.val();
    let tight = list_like::property(e, "tight", &layouter.chain) == Value::Bool(true);
    let leading = match layouter.chain.custom("par.leading") {
        Some(Value::Length(length)) => length.resolve_pt(size),
        _ => layouter
            .style
            .leading
            .map(|length| length.resolve_pt(size))
            .unwrap_or(size * super::vanilla_defaults::PAR_LEADING),
    };
    let spacing = points(layouter, e, "spacing");
    let gutter = spacing.unwrap_or_else(|| {
        if tight {
            leading
        } else {
            super::paragraph::spacing_for_size(&layouter.chain, size)
        }
    });
    let attached = tight
        && super::html_frame::items_have_causal_line(
            &layouter.regions.current.current_line,
        );
    let indent = points(layouter, e, "indent").unwrap_or(0.0);
    let hanging = points(layouter, e, "hanging-indent").unwrap_or(0.0);
    let body_indent = points(layouter, e, "body-indent").unwrap_or(0.0);
    let numbers = lists::numbers(e, &layouter.chain);
    let separator = if let Value::Content(separator) =
        list_like::property(e, "separator", &layouter.chain)
    {
        separator
    } else {
        Content::Empty
    };
    let (mut pieces, align) = labels(layouter, e, &numbers, indent + body_indent);
    let column = pieces.iter().map(|piece| piece.width).fold(0.0_f64, f64::max);
    let rtl = super::paragraph::chain_direction(&layouter.chain) == Dir::RTL;
    for (index, item) in children.iter().enumerate() {
        let label = (!pieces.is_empty()).then(|| {
            let piece = pieces.remove(0);
            let free = column - piece.width;
            let start = match align.h {
                Some(HAlign::Center) => free / 2.0,
                Some(HAlign::End) => free,
                Some(HAlign::Right) if !rtl => free,
                Some(HAlign::Left) if rtl => free,
                Some(HAlign::Left | HAlign::Right | HAlign::Start) | None => 0.0,
            };
            Label { piece, x: indent + start, vertical: align.v }
        });
        layouter.list_frame = Some(ItemFrame {
            above: if index > 0 {
                Some(gutter)
            } else {
                attached.then_some(spacing.unwrap_or(leading))
            },
            below: (index + 1 < children.len()).then_some(gutter),
            offset: if e.kind == ListKind::Terms {
                indent + hanging
            } else {
                indent + column + body_indent
            },
            label,
            styles: lists::body_styles(
                e.kind,
                numbers.get(index).copied().unwrap_or_default(),
                &layouter.chain,
            ),
            hanging,
            separator: separator.clone(),
        });
        layouter.layout_planned_child(
            crate::compiler::introspect::page_running::PlanEdge::TermsItem(index),
            item,
        );
        layouter.list_frame = None;
    }
}

/// Rótulos compostos e o seu alinhamento.
fn labels<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e: &TermsElem,
    numbers: &[u32],
    indents: f64,
) -> (Vec<IntrinsicPiece>, Align2D) {
    let name = match e.kind {
        ListKind::Terms => return (Vec::new(), Align2D { h: None, v: None }),
        ListKind::List => "marker-align",
        ListKind::Enum => "number-align",
    };
    let align = if let Value::Align(align) = list_like::property(e, name, &layouter.chain) {
        Align2D { h: align.h.or(Some(HAlign::Start)), v: align.v }
    } else {
        Align2D { h: Some(HAlign::End), v: None }
    };
    let width = (layouter.available_width() - indents).max(0.0);
    let pieces = numbers
        .iter()
        .enumerate()
        .map(|(index, number)| {
            let prepared = e.label(index);
            let cursor = prepared.and_then(|(child, _)| {
                layouter.planned_child_cursor(
                    crate::compiler::introspect::page_running::PlanEdge::TermsItem(child),
                )
            });
            let content = match prepared {
                Some((_, content)) => content.clone(),
                None => generated(layouter, e, *number),
            };
            let mut piece = layouter.layout_intrinsic_piece(
                std::slice::from_ref(&content),
                &[cursor],
                SubLayoutRegion {
                    origin_x: 0.0,
                    width,
                    height: None,
                    align_rtl: false,
                    unconstrained_height: true,
                },
            );
            let own = piece.width;
            layouter.settle_intrinsic_piece(&mut piece, own);
            piece
        })
        .collect();
    (pieces, align)
}

fn generated<M: FontMetrics, S: ImageSizer>(
    l: &Layouter<M, S>,
    e: &TermsElem,
    number: u32,
) -> Content {
    if e.kind == ListKind::List {
        let marker = list_like::property(e, "marker", &l.chain);
        return lists::marker_at(&marker, lists::depth(&l.chain));
    }
    let Value::Str(pattern) = list_like::property(e, "numbering", &l.chain) else {
        return Content::Empty;
    };
    let mut numbers = lists::parents(&l.chain);
    let level = numbers.len();
    let full = list_like::property(e, "full", &l.chain) == Value::Bool(true);
    numbers.push(number);
    Content::text(if full {
        crate::compiler::stdlib::numbering_text(&pattern, &numbers)
    } else {
        crate::compiler::stdlib::numbering_level(&pattern, level, number)
    })
}

/// Corpo de um item ao lado do seu rótulo, numa moldura de bloco.
pub(super) fn item_body<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    frame: ItemFrame,
    body: &Content,
    emit: impl FnOnce(&mut Layouter<M, S>, &Content),
) {
    let rtl = super::paragraph::chain_direction(&layouter.chain) == Dir::RTL;
    let mut block = BlockElem::plain(body.clone());
    if rtl {
        block.inset.right = Length::pt(frame.offset);
    } else {
        block.inset.left = Length::pt(frame.offset);
    }
    if !layouter.block_auto_width
        && layouter.available_width().is_finite()
        && (!layouter.is_sub_frame || layouter.explicit_line_alignment.is_some())
    {
        block.width = Some(Rel::from_percent(100.0));
    }
    block.above = frame.above.map(Length::pt);
    block.below = frame.below.map(Length::pt);
    let (top, _) = layouter.metrics.text_edges(layouter.style.size, &layouter.style);
    let room = frame.label.as_ref().map_or(top.0, |label| label.piece.height.max(top.0));
    let chain = layouter.chain.clone();
    let style = layouter.style.clone();
    let origin = layouter.regions.current.line_start_x.0;
    let right = layouter.regions.current.width - layouter.page_config.margin.right;
    // A moldura abre o seu próprio escopo de linha de base.
    let mut baseline = None;
    let start = super::block::layout_frame(layouter, &block, room, |layouter, body| {
        layouter.chain = layouter.chain.push_styles(&frame.styles);
        layouter.style = TextStyle::from(&layouter.chain);
        emit(layouter, body);
        layouter.flush_line();
        baseline = layouter.flow_baseline;
        layouter.chain = chain;
        layouter.style = style;
    });
    if let Some(label) = frame.label {
        place_label(layouter, label, start, baseline.flatten(), rtl.then_some(right), origin);
    }
}

fn place_label<M: FontMetrics, S: ImageSizer>(
    l: &mut Layouter<M, S>,
    label: Label,
    start: super::block::FrameStart,
    body_baseline: Option<f64>,
    right: Option<f64>,
    origin: f64,
) {
    let Label { piece, x, vertical } = label;
    let same = l.pages.len() == start.page && l.current_column == start.column;
    let mut offset = 0.0;
    if let Some(align) = vertical {
        let end = if same { l.regions.current.cursor_y.0 } else { start.y };
        let free = (end - start.y - piece.height).max(0.0);
        offset = match align {
            VAlign::Top => 0.0,
            VAlign::Horizon => free / 2.0,
            VAlign::Bottom => free,
        };
    } else if let (Some(Some(own)), Some(body)) = (piece.baseline, body_baseline) {
        offset = body - start.y - own;
        if offset < 0.0 {
            if same {
                if let Some(FrameItem::Group { items, .. }) =
                    l.regions.current.current_items.get_mut(start.items)
                {
                    for item in items {
                        super::helpers::shift_frame_item_y(item, -offset);
                    }
                }
                extend(l, l.regions.current.cursor_y.0 - offset);
            }
            offset = 0.0;
        }
    }
    let y = start.y + offset;
    if same && y + piece.height > l.regions.current.cursor_y.0 {
        extend(l, y + piece.height);
    }
    let x = match right {
        Some(right) if right.is_finite() => right - x - piece.width,
        Some(_) | None => origin + x,
    };
    let mut group = FrameItem::Group {
        paint_root: false,
        pos: Point { x: Pt(x), y: Pt(y) },
        matrix: TransformMatrix::identity(),
        clip_mask: None,
        inner_width: piece.width,
        inner_height: piece.height,
        items: piece.items,
    };
    if !same {
        super::helpers::shift_frame_item_x(&mut group, start.column_dx);
    }
    // O rótulo precede o corpo na ordem de leitura.
    if l.pages.len() != start.page {
        if let Some(page) = l.pages.get_mut(start.page) {
            page.items.insert(start.items.min(page.items.len()), group);
        }
    } else if let Some(column) =
        l.column_page_items.get_mut(start.column).filter(|_| !same)
    {
        column.insert(start.items.min(column.len()), group);
    } else if same {
        // Dentro da moldura do item, em coordenadas dela.
        let index = start.items;
        if let Some(FrameItem::Group { pos, items, .. }) =
            l.regions.current.current_items.get_mut(index)
        {
            super::helpers::shift_frame_item_x(&mut group, -pos.x.0);
            super::helpers::shift_frame_item_y(&mut group, -pos.y.0);
            items.insert(0, group);
            bump(&mut l.pending_align_centering, &[index], 0);
            bump(&mut l.pending_align_v_centering, &[index], 0);
        } else {
            l.regions.current.current_items.push(group);
        }
    } else {
        let index = start.items.min(l.regions.current.current_items.len());
        l.regions.current.current_items.insert(index, group);
        bump(&mut l.pending_align_centering, &[], index);
        bump(&mut l.pending_align_v_centering, &[], index);
    }
    if let Some(collector) = l.decoration_lines_collector.as_mut() {
        collector.extend(piece.decorations.into_iter().map(|segment| super::DecoSegment {
            start_x: segment.start_x + Pt(x),
            end_x: segment.end_x + Pt(x),
            baseline_y: segment.baseline_y + Pt(y),
            piece_line: segment.piece_line,
        }));
    }
}

/// Índices diferidos que o rótulo inserido deslocou.
trait Deferred {
    fn path(&mut self) -> &mut Vec<usize>;
}

impl Deferred for super::PendingAlignXEntry {
    fn path(&mut self) -> &mut Vec<usize> {
        &mut self.0
    }
}

impl Deferred for super::PendingAlignYEntry {
    fn path(&mut self) -> &mut Vec<usize> {
        &mut self.0
    }
}

fn bump<T: Deferred>(entries: &mut [T], prefix: &[usize], index: usize) {
    for entry in entries {
        let path = entry.path();
        if path.len() > prefix.len()
            && path.starts_with(prefix)
            && path[prefix.len()] >= index
        {
            path[prefix.len()] += 1;
        }
    }
}

/// Estende o item até `end`, quando o rótulo é mais alto do que o corpo.
fn extend<M: FontMetrics, S: ImageSizer>(l: &mut Layouter<M, S>, end: f64) {
    l.regions.current.cursor_y = Pt(end);
    l.prev_line_baseline = end;
    l.last_block_descent_y = Some(end);
    l.record_flow_placement_end(end);
}

#[cfg(test)]
mod tests {
    use crate::compiler::layout::{layout, tests::frame_items_recursive};
    use crate::entities::content::Content;
    use crate::entities::layout_types::FrameItem;

    fn texts(content: &Content) -> Vec<String> {
        frame_items_recursive(&layout(content).pages[0].items)
            .into_iter()
            .filter_map(|item| match item {
                FrameItem::Text { text, .. } => Some(text.to_string()),
                FrameItem::TextReplacement { .. }
                | FrameItem::TextSeparator { .. }
                | FrameItem::Semantic { .. }
                | FrameItem::TextShaped { .. }
                | FrameItem::Line { .. }
                | FrameItem::Glyph { .. }
                | FrameItem::Image { .. }
                | FrameItem::Shape { .. }
                | FrameItem::Group { .. }
                | FrameItem::Link { .. } => None,
            })
            .collect()
    }

    #[test]
    fn label_precedes_its_body() {
        let list = crate::compiler::realize::lists::group(&Content::sequence(vec![
            Content::enum_item(None, Content::text("a")),
            Content::enum_item(Some(7), Content::text("b")),
        ]));
        assert_eq!(texts(&list), ["1.", "a", "7.", "b"]);
    }
}
