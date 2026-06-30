//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/atomizacao_elementos.md
//! @prompt 00_nucleo/prompts/entities/elements/curve.md
//! @prompt 00_nucleo/prompts/rules/stdlib/curve.md
//! @layer L1
//! @updated 2026-06-30
//!
//! Atomização (ADR-0109, P513): o layout de `Content::Curve` move-se para o
//! arquivo da feature. Converte `CurveSegment` (com `Length`) para `PathItem`
//! absoluto e emite um `FrameItem::Shape` equivalente.

use crate::entities::elements::curve::{CurveElem, CurveSegment};
use crate::entities::geometry::{PathItem, ShapeKind, Stroke};
use crate::entities::layout_types::{Color, FrameItem, Point, Pt};
use crate::entities::paint::Paint;

use super::{FontMetrics, ImageSizer, Layouter};

/// Converte segmentos de curva para `PathItem` absolutos, resolvendo
/// `Length` com o font-size actual.
pub(super) fn path_items_from_curve(segments: &[CurveSegment], font_size_pt: f64) -> Vec<PathItem> {
    let mut items = Vec::new();
    let mut last_point = Point::ZERO;

    for seg in segments {
        match seg {
            CurveSegment::Move(p) => {
                let target = Point {
                    x: Pt(p.x.resolve_pt(font_size_pt)),
                    y: Pt(p.y.resolve_pt(font_size_pt)),
                };
                items.push(PathItem::MoveTo(target));
                last_point = target;
            }
            CurveSegment::Line(p) => {
                let target = Point {
                    x: Pt(p.x.resolve_pt(font_size_pt)),
                    y: Pt(p.y.resolve_pt(font_size_pt)),
                };
                items.push(PathItem::LineTo(target));
                last_point = target;
            }
            CurveSegment::Cubic(c1, c2, end) => {
                let target = Point {
                    x: Pt(end.x.resolve_pt(font_size_pt)),
                    y: Pt(end.y.resolve_pt(font_size_pt)),
                };
                items.push(PathItem::CubicTo(
                    Point {
                        x: Pt(c1.x.resolve_pt(font_size_pt)),
                        y: Pt(c1.y.resolve_pt(font_size_pt)),
                    },
                    Point {
                        x: Pt(c2.x.resolve_pt(font_size_pt)),
                        y: Pt(c2.y.resolve_pt(font_size_pt)),
                    },
                    target,
                ));
                last_point = target;
            }
            CurveSegment::Quad(c, end) => {
                let p0x = last_point.x.0;
                let p0y = last_point.y.0;
                let qx = c.x.resolve_pt(font_size_pt);
                let qy = c.y.resolve_pt(font_size_pt);
                let ex = end.x.resolve_pt(font_size_pt);
                let ey = end.y.resolve_pt(font_size_pt);
                let c1x = (p0x + 2.0 * qx) / 3.0;
                let c1y = (p0y + 2.0 * qy) / 3.0;
                let c2x = (ex + 2.0 * qx) / 3.0;
                let c2y = (ey + 2.0 * qy) / 3.0;
                let target = Point { x: Pt(ex), y: Pt(ey) };
                items.push(PathItem::CubicTo(
                    Point { x: Pt(c1x), y: Pt(c1y) },
                    Point { x: Pt(c2x), y: Pt(c2y) },
                    target,
                ));
                last_point = target;
            }
            CurveSegment::Close => {
                items.push(PathItem::ClosePath);
            }
        }
    }

    items
}

/// Layout de `Content::Curve`: resolve segmentos, calcula bbox e emite um
/// `FrameItem::Shape` com stroke preto por omissão (sem preenchimento).
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e:        &CurveElem,
) {
    let font = layouter.font_size_pt.val();
    let path_items = path_items_from_curve(&e.segments, font);

    let (min_x, min_y, max_x, max_y) = crate::entities::geometry::path_bbox(&path_items);
    let width = (max_x - min_x).max(0.0);
    let height = (max_y - min_y).max(0.0);

    if layouter.regions.current.cursor_y.0 + height > layouter.regions.current.height - layouter.page_config.margin {
        layouter.new_page();
    }
    layouter.flush_line();

    let pos = Point {
        x: layouter.regions.current.cursor_x,
        y: layouter.regions.current.cursor_y,
    };

    let stroke = Stroke {
        paint: Paint::Solid(Color::rgb(0, 0, 0)),
        thickness: 1.0,
        overhang: false,
    };

    layouter.regions.current.current_items.push(FrameItem::Shape {
        pos,
        kind: ShapeKind::Path(path_items),
        width,
        height,
        fill: None,
        stroke: Some(stroke),
        parent_bbox_at_emit: layouter.parent_bbox,
    });

    layouter.regions.current.cursor_y += Pt(height);
}
