//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/grid/measurement_cache.md
//! @prompt-hash ac9711c0
//! @layer L1
//! @updated 2026-10-09

use super::super::{
    DecoSegment, FontMetrics, ImageSizer, Layouter, PendingAlignXEntry,
    PendingAlignYEntry,
};
use super::{Content, FrameItem, PlacedCell};
use crate::entities::layout_types::TextStyle;

pub(super) type Product = (
    f64,
    Vec<FrameItem>,
    Vec<DecoSegment>,
    Vec<PendingAlignXEntry>,
    Vec<PendingAlignYEntry>,
);

struct Entry {
    row: usize,
    col: usize,
    origin_x: f64,
    width: f64,
    last_width: f64,
    last_bottom: Option<f64>,
    product: Product,
}

pub(super) struct MeasuredCells {
    entries: Vec<Option<Entry>>,
    num_cols: usize,
    parent_style: TextStyle,
    margins: (f64, f64),
    decoration: bool,
}

fn simple_placement(content: &Content) -> bool {
    let content = content.source_body();
    match content {
        Content::GridCell(e) => {
            e.x.is_none()
                && e.y.is_none()
                && e.colspan.unwrap_or(1) == 1
                && e.rowspan.unwrap_or(1) == 1
        }
        Content::TableCell(e) if e.resolved.is_some() => {
            let r = e.resolved.as_ref().unwrap();
            r.colspan == 1 && r.rowspan == 1
        }
        Content::TableCell(e) => {
            e.x.is_none()
                && e.y.is_none()
                && e.colspan.unwrap_or(1) == 1
                && e.rowspan.unwrap_or(1) == 1
        }
        _ => true, // Raw carriers have no explicit placement fields.
    }
}

fn token(content: &Content) -> Option<&ecow::EcoString> {
    let content = content.source_body();
    match content {
        Content::Styled(styled) if local_cell_transport_only(&styled.styles) => {
            token(&styled.body)
        }
        Content::Text(text) => {
            if !text.is_empty() && text.bytes().all(|b| b.is_ascii_graphic()) {
                Some(text)
            } else {
                None
            }
        }
        Content::Sequence(parts) => {
            let mut material =
                parts.iter().filter(|part| !matches!(part, Content::Empty));
            let text = token(material.next()?)?;
            material.next().is_none().then_some(text)
        }
        Content::GridCell(e) => {
            if simple_placement(content) && e.align.is_none() {
                token(&e.body)
            } else {
                None
            }
        }
        Content::TableCell(e) => {
            if simple_placement(content) && e.align.is_none() {
                match &e.resolved {
                    Some(r) if r.align.is_none() => token(&r.visual),
                    Some(_) => None,
                    None => token(&e.body),
                }
            } else {
                None
            }
        }
        _ => None, // No locatable, style scope, context, callback or unknown carrier.
    }
}

fn local_cell_transport_only(styles: &crate::entities::style::Styles) -> bool {
    let d = styles.delta();
    d.bold.is_none()
        && d.bold_from_strong.is_none()
        && d.italic.is_none()
        && d.italic_from_emph.is_none()
        && d.size.is_none()
        && d.fill.is_none()
        && d.heading_level.is_none()
        && d.weight.is_none()
        && d.tracking.is_none()
        && d.leading.is_none()
        && d.top_edge.is_none()
        && d.bottom_edge.is_none()
        && d.lang.is_none()
        && d.font.is_none()
        && d.subscript.is_none()
        && d.superscript.is_none()
        && d.highlight.is_none()
        && d.highlight_radius.is_none()
        && d.highlight_extent.is_none()
        && d.subscript_size.is_none()
        && d.superscript_size.is_none()
        && !d.custom.is_empty()
        && d.custom.iter().all(|(key, _)| {
            key.as_str().starts_with("cell.")
                || matches!(key.as_str(), "eval.table.local" | "eval.table.local.recipe")
        })
}

fn ambient<M: FontMetrics, S: ImageSizer>(l: &Layouter<M, S>) -> bool {
    !l.is_sub_frame
        && l.regions.cell.is_none()
        && !l.column_mode
        && l.page_columns.is_none()
        && !l.smallcaps
        && !l.style.math
        && l.cell_align.is_none()
        && l.floats_pending.is_empty()
        && !l.flow_suffix_transaction_active
        && !l.flow_suffix_checkpoint_pending
        && l.chain.custom("par.line.numbering").is_none()
}

impl MeasuredCells {
    pub(super) fn new<M: FontMetrics, S: ImageSizer>(
        l: &Layouter<M, S>,
        cells: &[Content],
        num_cols: usize,
    ) -> Self {
        let enabled = ambient(l)
            && cells.iter().enumerate().all(|(i, c)| {
                simple_placement(c)
                    && match c.source_body() {
                        Content::TableCell(e) if e.resolved.is_some() => {
                            let r = e.resolved.as_ref().unwrap();
                            r.x == i % num_cols && r.y == i / num_cols
                        }
                        _ => true,
                    }
            });
        Self {
            entries: if enabled {
                (0..cells.len()).map(|_| None).collect()
            } else {
                Vec::new()
            },
            num_cols,
            parent_style: l.style.clone(),
            margins: (l.page_config.margin.left, l.page_config.margin.right),
            decoration: l.decoration_lines_collector.is_some(),
        }
    }

    fn environment<M: FontMetrics, S: ImageSizer>(&self, l: &Layouter<M, S>) -> bool {
        ambient(l)
            && self.parent_style == l.style
            && self.margins == (l.page_config.margin.left, l.page_config.margin.right)
            && self.decoration == l.decoration_lines_collector.is_some()
    }

    pub(super) fn capture<M: FontMetrics, S: ImageSizer>(
        &mut self,
        l: &Layouter<M, S>,
        index: usize,
        content: &Content,
        origin_x: f64,
        width: f64,
        product: Product,
    ) {
        if index >= self.entries.len()
            || !self.environment(l)
            || !origin_x.is_finite()
            || !width.is_finite()
            || width <= 0.0
            || !product.0.is_finite()
            || !l.last_sub_frame_width.is_finite()
            || l.last_sub_frame_width > width + 1e-9
            || !product.3.is_empty()
            || !product.4.is_empty()
        {
            return;
        }
        let Some(text) = token(content) else {
            return;
        };
        #[allow(deprecated)]
        let same_text = matches!(product.1.as_slice(), [FrameItem::Text { text: measured, .. }] if measured == text);
        if !same_text {
            return;
        }
        self.entries[index] = Some(Entry {
            row: index / self.num_cols,
            col: index % self.num_cols,
            origin_x,
            width,
            last_width: l.last_sub_frame_width,
            last_bottom: l.last_sub_frame_bottom,
            product,
        });
    }

    pub(super) fn take<M: FontMetrics, S: ImageSizer>(
        &mut self,
        l: &mut Layouter<M, S>,
        placed: &PlacedCell,
        origin_x: f64,
        width: f64,
    ) -> Option<Product> {
        if !self.environment(l) || placed.colspan != 1 || placed.rowspan != 1 {
            return None;
        }
        let index = placed.row.checked_mul(self.num_cols)?.checked_add(placed.col)?;
        let entry = self.entries.get_mut(index)?.take()?;
        if entry.row != placed.row
            || entry.col != placed.col
            || entry.origin_x != origin_x
            || entry.width != width
        {
            return None;
        }
        let text = token(&placed.body)?;
        #[allow(deprecated)]
        let same = matches!(entry.product.1.as_slice(), [FrameItem::Text { text: measured, style, .. }]
            if measured == text && *style == super::super::text::effective_style(l));
        if !same {
            return None;
        }
        l.last_sub_frame_width = entry.last_width;
        l.last_sub_frame_bottom = entry.last_bottom;
        Some(entry.product)
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::{sub_frame::SubLayoutRegion, FixedMetrics};
    use super::*;
    use crate::entities::{
        image_sizer::NullImageSizer,
        introspector::{Introspector, TagIntrospector},
        layout_types::Length,
        value::Value,
    };
    use comemo::Track;

    #[test]
    fn changed_origin_invalidates_product_and_does_not_retry_it() {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            FixedMetrics,
            NullImageSizer,
            12.0,
            (&intr as &dyn Introspector).track(),
        );
        let body = Content::text("Alpha");
        let mut cache = MeasuredCells::new(&l, std::slice::from_ref(&body), 1);
        let product = l.layout_sub_frame(
            &body,
            SubLayoutRegion {
                origin_x: 50.0,
                width: 100.0,
                height: None,
                align_rtl: true,
                unconstrained_height: true,
            },
        );
        cache.capture(&l, 0, &body, 50.0, 100.0, product);
        assert!(cache.entries[0].is_some(), "fixture must begin with a captured product");
        let placed = PlacedCell { body, row: 0, col: 0, colspan: 1, rowspan: 1 };
        assert!(cache.take(&mut l, &placed, 51.0, 100.0).is_none());
        assert!(cache.take(&mut l, &placed, 50.0, 100.0).is_none());
    }

    #[test]
    fn changed_effective_style_requires_fallback_without_changing_parent_style() {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            FixedMetrics,
            NullImageSizer,
            12.0,
            (&intr as &dyn Introspector).track(),
        );
        let body = Content::text("Alpha");
        let mut cache = MeasuredCells::new(&l, std::slice::from_ref(&body), 1);
        let product = l.layout_sub_frame(
            &body,
            SubLayoutRegion {
                origin_x: 50.0,
                width: 100.0,
                height: None,
                align_rtl: true,
                unconstrained_height: true,
            },
        );
        cache.capture(&l, 0, &body, 50.0, 100.0, product);
        assert!(cache.entries[0].is_some(), "fixture must begin with a captured product");
        let placed = PlacedCell { body, row: 0, col: 0, colspan: 1, rowspan: 1 };
        let parent = l.style.clone();
        let before = super::super::super::text::effective_style(&l);
        l.chain = l.chain.push_custom("text.tracking", Value::Length(Length::pt(2.0)));
        assert_ne!(super::super::super::text::effective_style(&l), before);
        assert_eq!(l.style, parent);
        assert!(cache.take(&mut l, &placed, 50.0, 100.0).is_none());
    }

    #[test]
    fn priority5_punctuation_consumes_one_measured_product() {
        let intr = TagIntrospector::empty();
        for text in ["12.34", "item-2", "abc_def", "a/b", "+15%", "https://a.test"] {
            let mut l = Layouter::new(
                FixedMetrics,
                NullImageSizer,
                12.0,
                (&intr as &dyn Introspector).track(),
            );
            let body = Content::text(text);
            let mut cache = MeasuredCells::new(&l, std::slice::from_ref(&body), 1);
            let p = l.layout_sub_frame(
                &body,
                SubLayoutRegion {
                    origin_x: 50.0,
                    width: 300.0,
                    height: None,
                    align_rtl: true,
                    unconstrained_height: true,
                },
            );
            let height = p.0;
            let width = l.last_sub_frame_width;
            cache.capture(&l, 0, &body, 50.0, 300.0, p);
            assert!(cache.entries[0].is_some(), "{}", text);
            let placed = PlacedCell { body, row: 0, col: 0, colspan: 1, rowspan: 1 };
            let result = cache
                .take(&mut l, &placed, 50.0, 300.0)
                .expect("ASCII graphic leaf eligible");
            assert_eq!(result.0, height);
            assert_eq!(l.last_sub_frame_width, width);
            #[allow(deprecated)]
            let measured = match &result.1[0] {
                FrameItem::Text { text, .. } => text,
                _ => panic!("text expected"),
            };
            assert_eq!(measured.as_str(), text);
            assert!(cache.take(&mut l, &placed, 50.0, 300.0).is_none());
        }
    }
    #[test]
    fn priority5_non_graphic_and_composite_carriers_remain_fallback() {
        for text in ["", "A B", "A\nB", "A\tB", "\0", "café", "東京", "אבג"] {
            assert!(token(&Content::text(text)).is_none(), "{:?}", text);
        }
        let composite =
            Content::sequence(vec![Content::text("12."), Content::text("34")]);
        assert!(token(&composite).is_none());
    }

    #[test]
    fn local_cell_transport_is_eligible_only_without_textual_style() {
        let body = Content::text("Local");
        let transport = crate::entities::style::Styles::new()
            .push_custom("cell.fill", Value::None)
            .push_custom("eval.table.local", Value::Bool(true));
        let inert = Content::styled(body.clone(), transport);
        assert_eq!(token(&inert).map(|s| s.as_str()), Some("Local"));

        let text_style = crate::entities::style::Styles::new()
            .push_custom("text.tracking", Value::Length(Length::pt(1.0)));
        assert!(token(&Content::styled(body.clone(), text_style)).is_none());
        let unknown = crate::entities::style::Styles::new()
            .push_custom("layout.unknown", Value::Bool(true));
        assert!(token(&Content::styled(body.clone(), unknown)).is_none());
        let mut typed = crate::entities::style_chain::StyleDelta::empty();
        typed.bold = Some(true);
        assert!(token(&Content::styled(
            body,
            crate::entities::style::Styles::from_delta(typed)
        ))
        .is_none());
    }
    #[test]
    fn priority5_overflow_or_transformed_product_remains_fallback() {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            FixedMetrics,
            NullImageSizer,
            12.0,
            (&intr as &dyn Introspector).track(),
        );
        let body = Content::text("12.34");
        let mut c = MeasuredCells::new(&l, std::slice::from_ref(&body), 1);
        let p = l.layout_sub_frame(
            &body,
            SubLayoutRegion {
                origin_x: 50.0,
                width: 300.0,
                height: None,
                align_rtl: true,
                unconstrained_height: true,
            },
        );
        l.last_sub_frame_width = 301.0;
        c.capture(&l, 0, &body, 50.0, 300.0, p);
        assert!(c.entries[0].is_none());
        let p = l.layout_sub_frame(
            &Content::text("Different"),
            SubLayoutRegion {
                origin_x: 50.0,
                width: 300.0,
                height: None,
                align_rtl: true,
                unconstrained_height: true,
            },
        );
        c.capture(&l, 0, &body, 50.0, 300.0, p);
        assert!(c.entries[0].is_none());
    }
    #[test]
    fn priority5_source_leaf_reuses_without_erasing_origin() {
        let id = crate::entities::file_id::FileId::from_raw(
            std::num::NonZeroU16::new(1).unwrap(),
        );
        let span = crate::entities::span::Span::from_range(id, 2..5);
        let body = Content::text("12.34").with_source_span(span);
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            FixedMetrics,
            NullImageSizer,
            12.0,
            (&intr as &dyn Introspector).track(),
        );
        let mut c = MeasuredCells::new(&l, std::slice::from_ref(&body), 1);
        let p = l.layout_sub_frame(
            &body,
            SubLayoutRegion {
                origin_x: 50.0,
                width: 300.0,
                height: None,
                align_rtl: true,
                unconstrained_height: true,
            },
        );
        c.capture(&l, 0, &body, 50.0, 300.0, p);
        assert!(c.entries[0].is_some());
        let placed = PlacedCell { body, row: 0, col: 0, colspan: 1, rowspan: 1 };
        assert!(c.take(&mut l, &placed, 50.0, 300.0).is_some());
        assert_eq!(placed.body.source_span(), span);
        assert!(matches!(placed.body, Content::Source(_)));
        assert!(c.take(&mut l, &placed, 50.0, 300.0).is_none());
    }
    #[test]
    fn priority5_source_does_not_hide_style_or_special_placement() {
        use crate::entities::content::SourceContent;
        use crate::entities::elements::grid_cell::GridCellElem;
        use std::sync::Arc;
        let wrap = |body| {
            Content::Source(Arc::new(SourceContent {
                body,
                span: crate::entities::span::Span::detached(),
            }))
        };
        let styled = wrap(Content::styled(
            Content::text("12.34"),
            crate::entities::style::Styles::default(),
        ));
        assert!(token(&styled).is_none());
        let special = wrap(Content::GridCell(Arc::new(GridCellElem {
            body: Content::text("12.34"),
            x: Some(1),
            y: None,
            colspan: None,
            rowspan: None,
            stroke: None,
            fill: None,
            align: None,
            inset: None,
            breakable: None,
        })));
        assert!(!simple_placement(&special));
        assert!(token(&special).is_none());
        let intr = TagIntrospector::empty();
        let l = Layouter::new(
            FixedMetrics,
            NullImageSizer,
            12.0,
            (&intr as &dyn Introspector).track(),
        );
        let c = MeasuredCells::new(&l, &[Content::text("a/b"), special], 2);
        assert!(c.entries.is_empty());
    }
}
