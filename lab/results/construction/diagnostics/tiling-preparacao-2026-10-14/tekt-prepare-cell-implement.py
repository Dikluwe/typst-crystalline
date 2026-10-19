from pathlib import Path
p=Path('01_core/src/compiler/layout/tiling.rs');s=p.read_text()
s=s.replace('use crate::entities::geometry::ShapeKind;', '''use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::frame_paint::TilingFrame;
use crate::entities::geometry::ShapeKind;
use crate::entities::gradient::RelativeTo;''',1)
s=s.replace('use crate::entities::layout_types::{FrameItem, Point, Pt, TransformMatrix};', '''use crate::entities::layout_types::{Frame, FrameItem, Length, Point, Pt, Rect, Size, TransformMatrix};
use crate::entities::paint::Paint;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;''',1)
a=s.index('    let TilingBody::Content(body) = &tiling.body else {')
b=s.index('    if !pos.x.0.is_finite()',a)
s=s[:a]+'''    if !width.is_finite() || !height.is_finite() || width <= 0.0 || height <= 0.0 {
        return None;
    }
    let cell = match prepare_cell(layouter, tiling) {
        Ok(cell) => cell,
        Err(errors) => {
            layouter.layout_errors.extend(errors);
            return None;
        }
    };
    let cell_width = cell.frame.size.width.0;
    let cell_height = cell.frame.size.height.0;
    let pitch_x = cell_width + cell.spacing.width.0;
    let pitch_y = cell_height + cell.spacing.height.0;
    let offset_x = cell.offset.x.0;
    let offset_y = cell.offset.y.0;
    let angle = cell.angle.to_rad();
    let body_items = cell.frame.items;

'''+s[b:]
index=s.index('// Include boundary cells')
s=s[:index]+'''/// Prepare exactly one cell. Target coverage and exporter repetition are absent
/// from this boundary; the returned frame establishes an implicit paint root.
pub(super) fn prepare_cell<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    tiling: &Tiling,
) -> SourceResult<TilingFrame> {
    let explicit = tiling.size;
    if explicit.is_some_and(|size| !valid_size(size)) {
        return Err(cell_error("tile size must be non-zero and non-infinite"));
    }
    let spacing = tiling.spacing.unwrap_or(Size { width: Pt(0.0), height: Pt(0.0) });
    if !spacing.width.0.is_finite() || !spacing.height.0.is_finite() {
        return Err(cell_error("tile spacing must be finite"));
    }
    if tiling.offset.x.abs.em != 0.0 || tiling.offset.y.abs.em != 0.0 {
        return Err(cell_error("tile offset must not be font-relative"));
    }
    if !tiling.offset.x.rel.is_finite()
        || !tiling.offset.y.rel.is_finite()
        || !tiling.offset.x.abs.abs.0.is_finite()
        || !tiling.offset.y.abs.abs.0.is_finite()
    {
        return Err(cell_error("tile offset must be finite"));
    }
    if !tiling.angle.to_rad().is_finite() {
        return Err(cell_error("tile angle must be finite"));
    }
    if let Some(size) = explicit {
        validate_placement(tiling, size, spacing)?;
    }

    // Rust legacy bodies cross the same L1 boundary explicitly. They never
    // acquire a target dependency or representative-color conversion.
    let bridged;
    let body = match &tiling.body {
        TilingBody::Content(body) => body.as_ref(),
        TilingBody::Image(image) => {
            bridged = Content::Image(Arc::new(image.clone()));
            &bridged
        }
        TilingBody::Color(_) | TilingBody::Gradient(_) => {
            let Some(size) = explicit else {
                return Err(cell_error("tile size must be explicit for a color or gradient body"));
            };
            let paint = match &tiling.body {
                TilingBody::Color(color) => Paint::Solid(color.clone()),
                TilingBody::Gradient(gradient) => Paint::Gradient(gradient.clone()),
                _ => unreachable!(),
            };
            bridged = Content::shape(
                ShapeKind::Rect,
                Some(Box::new(Value::Length(Length::pt(size.width.0)))),
                Some(Box::new(Value::Length(Length::pt(size.height.0)))),
                Some(paint),
                None,
            );
            &bridged
        }
    };

    let checkpoint = layouter.capture_flow_suffix_checkpoint();
    let errors_before = layouter.layout_errors.len();
    let warnings_before = layouter.layout_warnings.len();
    layouter.parent_bbox = explicit.map(|size| Rect {
        x: Pt(0.0), y: Pt(0.0), w: size.width, h: size.height,
    });
    let (body_height, body_items, _, _, _) = layouter.layout_sub_frame_at_geometric_origin(
        body,
        SubLayoutRegion {
            origin_x: 0.0,
            width: explicit.map_or(f64::INFINITY, |size| size.width.0),
            height: explicit.map(|size| size.height.0),
            align_rtl: false,
            unconstrained_height: explicit.is_none(),
        },
    );
    let body_width = layouter.last_sub_frame_flow_width
        .max(layouter.last_sub_frame_width)
        .max(super::helpers::line_content_right(&body_items, &layouter.metrics));
    let errors = layouter.layout_errors.split_off(errors_before);
    let warnings = layouter.layout_warnings.split_off(warnings_before);
    layouter.restore_flow_suffix_checkpoint(checkpoint);
    layouter.layout_warnings.extend(warnings);
    if !errors.is_empty() {
        return Err(errors);
    }
    let size = explicit.unwrap_or(Size { width: Pt(body_width), height: Pt(body_height) });
    if !size.width.0.is_finite() || !size.height.0.is_finite() {
        return Err(cell_error("tile size must be non-zero and non-infinite"));
    }
    if size.width.0 <= 0.0 || size.height.0 <= 0.0 {
        return Err(vec![SourceDiagnostic::error(Span::detached(), "tile size must be non-zero")
            .with_hint("try setting the size manually")]);
    }
    let offset = validate_placement(tiling, size, spacing)?;
    Ok(TilingFrame {
        frame: Frame { size, items: body_items },
        spacing,
        offset,
        angle: tiling.angle,
        relative: match tiling.relative {
            TilingRelative::Auto | TilingRelative::Itself => RelativeTo::Self_,
            TilingRelative::Parent => RelativeTo::Parent,
        },
    })
}

fn valid_size(size: Size) -> bool {
    size.width.0.is_finite() && size.height.0.is_finite()
        && size.width.0 > 0.0 && size.height.0 > 0.0
}

fn cell_error(message: &str) -> Vec<SourceDiagnostic> {
    vec![SourceDiagnostic::error(Span::detached(), message)]
}

fn validate_placement(tiling: &Tiling, size: Size, spacing: Size) -> SourceResult<Point> {
    let pitch_x = size.width.0 + spacing.width.0;
    let pitch_y = size.height.0 + spacing.height.0;
    if !pitch_x.is_finite() || !pitch_y.is_finite() || pitch_x <= 0.0 || pitch_y <= 0.0 {
        return Err(cell_error("tile pitch must be positive and finite"));
    }
    let x = tiling.offset.x.rel * pitch_x + tiling.offset.x.abs.abs.0;
    let y = tiling.offset.y.rel * pitch_y + tiling.offset.y.abs.abs.0;
    if !x.is_finite() || !y.is_finite() {
        return Err(cell_error("tile offset must be finite"));
    }
    Ok(Point { x: Pt(x), y: Pt(y) })
}

'''+s[index:]
p.write_text(s)
