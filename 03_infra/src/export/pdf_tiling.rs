//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/pdf_tiling.md
//! @prompt-hash 26d613ce
//! @layer L3

use std::collections::HashMap;
use std::sync::Arc;
use typst_core::entities::frame_paint::{FramePaint, TilingFrame};
use typst_core::entities::layout_types::{FrameItem, PagedDocument, TransformMatrix};

pub(super) type Key = (usize, usize);
pub(super) fn key(item: &FrameItem, cell: &Arc<TilingFrame>) -> Key {
    (item as *const FrameItem as usize, Arc::as_ptr(cell) as usize)
}
struct Pattern<'a> {
    cell: &'a TilingFrame,
    matrix: TransformMatrix,
    id: usize,
    form_id: usize,
    page: usize,
    children: Vec<usize>,
}
pub(super) struct Patterns<'a> {
    pub(super) names: Vec<HashMap<Key, String>>,
    patterns: Vec<Pattern<'a>>,
}
fn flip(height: f64) -> TransformMatrix {
    TransformMatrix::translate(0.0, height).concat(&TransformMatrix::scale(1.0, -1.0))
}
impl<'a> Patterns<'a> {
    pub(super) fn scan(doc: &'a PagedDocument, next: &mut usize) -> Self {
        let mut result = Self {
            names: vec![HashMap::new(); doc.pages.len()],
            patterns: Vec::new(),
        };
        for (page_index, page) in doc.pages.iter().enumerate() {
            let to_pdf = TransformMatrix::translate(page.bleed.left, page.bleed.bottom)
                .concat(&flip(page.height));
            for items in [&page.background, &page.items, &page.foreground] {
                result.walk(items, page_index, to_pdf, TransformMatrix::identity(), next);
            }
        }
        result
    }
    fn walk(
        &mut self,
        items: &'a [FrameItem],
        page: usize,
        to_pdf: TransformMatrix,
        to_root: TransformMatrix,
        next: &mut usize,
    ) -> Vec<usize> {
        let mut found = Vec::new();
        for item in items {
            match item {
                FrameItem::Shape { pos, fill, stroke, .. } => {
                    for paint in fill.iter().chain(stroke.iter().map(|s| &s.paint)) {
                        if let FramePaint::Tiling(cell) = paint {
                            let key = key(item, cell);
                            if let Some(name) = self.names[page].get(&key) {
                                found
                                    .push(name[1..].parse().expect("local pattern name"));
                                continue;
                            }
                            let placement = super::tiling::placement(cell, *pos, to_root)
                                .expect("invalid prepared tiling placement");
                            let matrix = to_pdf.concat(&placement);
                            let index = self.patterns.len();
                            self.names[page].insert(key, format!("T{index}"));
                            let id = *next;
                            *next += 2;
                            self.patterns.push(Pattern {
                                cell,
                                matrix,
                                id,
                                form_id: id + 1,
                                page,
                                children: Vec::new(),
                            });
                            let children = self.walk(
                                &cell.frame.items,
                                page,
                                TransformMatrix::identity(),
                                TransformMatrix::identity(),
                                next,
                            );
                            self.patterns[index].children = children;
                            found.push(index);
                        }
                    }
                }
                FrameItem::Group { pos, matrix, paint_root, items, .. } => {
                    let local =
                        TransformMatrix::translate(pos.x.0, pos.y.0).concat(matrix);
                    found.extend(self.walk(
                        items,
                        page,
                        to_pdf.concat(&local),
                        if *paint_root {
                            TransformMatrix::identity()
                        } else {
                            to_root.concat(&local)
                        },
                        next,
                    ));
                }
                FrameItem::Link { items, .. }
                | FrameItem::Semantic { items, .. }
                | FrameItem::TextReplacement { items, .. } => {
                    found.extend(self.walk(items, page, to_pdf, to_root, next))
                }
                FrameItem::Text { .. }
                | FrameItem::TextSeparator { .. }
                | FrameItem::TextShaped { .. }
                | FrameItem::Line { .. }
                | FrameItem::Glyph { .. }
                | FrameItem::Image { .. } => {}
            }
        }
        found.sort_unstable();
        found.dedup();
        found
    }
    pub(super) fn resources(&self, gradients: &str, page: usize) -> String {
        let entries = self
            .patterns
            .iter()
            .enumerate()
            .filter(|(_, p)| p.page == page)
            .map(|(i, p)| format!("/T{i} {} 0 R", p.id))
            .collect::<Vec<_>>()
            .join(" ");
        if entries.is_empty() {
            return gradients.into();
        }
        if gradients.is_empty() {
            format!("/Pattern << {entries} >>")
        } else {
            format!(
                "{} {entries} >>",
                gradients.trim_end().strip_suffix(">>").expect("pattern dictionary")
            )
        }
    }
    pub(super) fn emit(
        &self,
        page: usize,
        ctx: &mut super::stream::PageContext,
        resources: &str,
    ) -> Vec<(usize, Vec<u8>)> {
        let mut objects = Vec::new();
        let (tags, ua1) = (ctx.tags, ctx.ua1);
        ctx.tags = super::PdfTags::Disabled;
        ctx.ua1 = false;
        for pattern in self.patterns.iter().filter(|p| p.page == page) {
            let size = pattern.cell.frame.size;
            let bytes = super::stream::build_cell_stream(&pattern.cell.frame.items, ctx);
            let m = pattern.matrix;
            let descendants = pattern
                .children
                .iter()
                .map(|i| format!("/T{i} {} 0 R", self.patterns[*i].id))
                .collect::<Vec<_>>()
                .join(" ");
            // Rebuild just the pattern dictionary, retaining fonts/images/spot.
            let mut cell_resources = resources.to_string();
            if let Some(start) = cell_resources.find("/Pattern <<") {
                let end = start
                    + cell_resources[start..].find(">>").expect("pattern dictionary")
                    + 2;
                let gradients = ctx
                    .pat_refs
                    .iter()
                    .map(|r| format!("/{} {} 0 R", r.name, r.pattern_obj_id))
                    .collect::<Vec<_>>()
                    .join(" ");
                cell_resources.replace_range(
                    start..end,
                    &format!("/Pattern << {gradients} {descendants} >>"),
                );
            }
            let mut form = format!("<< /Type /XObject /Subtype /Form /BBox [0 0 {} {}] /Resources << {} >> /Length {} >>\nstream\n", size.width.0, size.height.0, cell_resources, bytes.len()).into_bytes();
            form.extend_from_slice(&bytes);
            form.extend_from_slice(b"\nendstream");
            objects.push((pattern.form_id, form));
            let px = size.width.0 + pattern.cell.spacing.width.0;
            let py = size.height.0 + pattern.cell.spacing.height.0;
            let mut period = String::new();
            for (x, y) in super::tiling::contributions(pattern.cell)
                .expect("prepared tiling contribution range")
            {
                period.push_str(&format!("q 1 0 0 1 {x} {y} cm /Cell Do Q\n"));
            }
            let mut object = format!("<< /Type /Pattern /PatternType 1 /PaintType 1 /TilingType 1 /BBox [0 0 {px} {py}] /XStep {px} /YStep {py} /Matrix [{} {} {} {} {} {}] /Resources << /XObject << /Cell {} 0 R >> >> /Length {} >>\nstream\n",m.a,m.b,m.c,m.d,m.tx,m.ty,pattern.form_id,period.len()).into_bytes();
            object.extend_from_slice(period.as_bytes());
            object.extend_from_slice(b"\nendstream");
            objects.push((pattern.id, object));
        }
        ctx.tags = tags;
        ctx.ua1 = ua1;
        objects
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use typst_core::entities::layout_types::{
        Angle, Color, Frame, Page, Point, Pt, Size,
    };
    fn shape(paint: FramePaint) -> FrameItem {
        FrameItem::Shape {
            pos: Point::ZERO,
            kind: typst_core::entities::geometry::ShapeKind::Rect,
            width: 4.0,
            height: 4.0,
            fill: Some(paint),
            stroke: None,
            fill_rule: Default::default(),
            parent_bbox_at_emit: None,
        }
    }
    fn cell(items: Vec<FrameItem>) -> FramePaint {
        FramePaint::Tiling(Arc::new(TilingFrame {
            frame: Frame {
                size: Size { width: Pt(8.0), height: Pt(8.0) },
                items,
            },
            spacing: Size { width: Pt(0.0), height: Pt(0.0) },
            offset: Point::ZERO,
            angle: Angle::deg(0.0),
            relative: typst_core::entities::gradient::RelativeTo::Self_,
        }))
    }
    #[test]
    fn shared_nested_cells_do_not_depend_on_an_excluded_page() {
        let nested = cell(vec![shape(FramePaint::Solid(Color::rgb(255, 0, 0)))]);
        let page = Page {
            width: 40.0,
            height: 40.0,
            numbering: None,
            logical_number: 1,
            supplement: typst_core::entities::content::Content::Empty,
            bleed: Default::default(),
            fill: Default::default(),
            background: vec![],
            foreground: vec![],
            items: vec![shape(cell(vec![shape(nested)]))],
        };
        let doc = PagedDocument::new(vec![page.clone(), page]);
        let patterns = Patterns::scan(&doc, &mut 10);
        let second: Vec<_> = patterns.patterns.iter().filter(|p| p.page == 1).collect();
        assert!(!second.is_empty());
        assert!(
            second
                .iter()
                .flat_map(|p| &p.children)
                .all(|i| patterns.patterns[*i].page == 1),
            "selected page must own every nested pattern resource"
        );
    }
    #[test]
    fn period_overlap_uses_four_clipped_contributions_and_one_cell_body() {
        let mut paint =
            cell(vec![shape(FramePaint::Solid(Color::srgb_f32(1.0, 0.0, 0.0, 0.5)))]);
        if let FramePaint::Tiling(c) = &mut paint {
            Arc::make_mut(c).spacing = Size { width: Pt(-4.0), height: Pt(-4.0) };
        }
        let page = Page {
            width: 40.0,
            height: 40.0,
            numbering: None,
            logical_number: 1,
            supplement: typst_core::entities::content::Content::Empty,
            bleed: Default::default(),
            fill: Default::default(),
            background: vec![],
            foreground: vec![],
            items: vec![shape(paint)],
        };
        let doc = PagedDocument::new(vec![page]);
        let patterns = Patterns::scan(&doc, &mut 10);
        let map = HashMap::new();
        let patterns_map = HashMap::new();
        let mut ctx = super::super::stream::PageContext::type1(
            &map,
            &[],
            &patterns_map,
            &[],
            super::super::StreamMode::Compact,
        );
        ctx.tiling_names = Some(&patterns.names[0]);
        let objects = patterns.emit(0, &mut ctx, &patterns.resources("", 0));
        let forms: Vec<_> = objects
            .iter()
            .map(|(_, b)| String::from_utf8_lossy(b))
            .filter(|s| s.contains("/Subtype /Form"))
            .collect();
        assert_eq!(forms.len(), 1, "body stored once, referenced for overlap");
        assert!(forms[0].contains("/BBox [0 0 8 8]"));
        let pattern = objects
            .iter()
            .map(|(_, b)| String::from_utf8_lossy(b))
            .find(|s| s.contains("/PatternType 1"))
            .unwrap();
        assert!(pattern.contains("/BBox [0 0 4 4]"));
        assert_eq!(pattern.matches(" Do").count(), 4);
        for translation in ["-4 -4 cm", "0 -4 cm", "-4 0 cm", "0 0 cm"] {
            assert!(pattern.contains(translation), "{translation}");
        }
        let svg = crate::export::svg::export_svg(&doc.pages[0], &Default::default());
        assert_eq!(svg.matches("<use ").count(), 4);
        assert_eq!(svg.matches("rgba(255,0,0,").count(), 1, "store body alpha once");
    }
}
