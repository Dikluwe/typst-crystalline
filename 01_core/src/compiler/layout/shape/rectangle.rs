//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/shape/rectangle.md
//! @prompt-hash 4527f10a
//! @layer L1

use super::super::{FontMetrics, ImageSizer, Layouter};
use crate::entities::corners::Corners;
use crate::entities::elements::shape::ShapeElem;
use crate::entities::frame_paint::{FramePaint, FrameStroke};
use crate::entities::geometry::{LineCap, LineJoin, PathItem, ShapeKind};
use crate::entities::layout_types::{FrameItem, Point, Pt};
use crate::entities::paint::Paint;

fn point(x: f64, y: f64) -> Point {
    Point { x: Pt(x), y: Pt(y) }
}

#[derive(Clone, Copy)]
struct Corner {
    x: f64,
    y: f64,
    rx: f64,
    ry: f64,
    sharp: bool,
}
impl Corner {
    fn at(self, angle: f64) -> Point {
        if self.sharp {
            return point(self.x, self.y);
        }
        point(self.x + self.rx * libm::cos(angle), self.y + self.ry * libm::sin(angle))
    }
    fn arc(self, path: &mut Vec<PathItem>, from: f64, to: f64) {
        if self.sharp || self.rx == 0.0 || self.ry == 0.0 {
            path.push(PathItem::LineTo(self.at(to)));
            return;
        }
        // rationale: cubic approximation of an ellipse arc, tangent at both ends.
        let k = 4.0 / 3.0 * libm::tan((to - from) / 4.0);
        let a = self.at(from);
        let b = self.at(to);
        path.push(PathItem::CubicTo(
            point(
                a.x.0 - k * self.rx * libm::sin(from),
                a.y.0 + k * self.ry * libm::cos(from),
            ),
            point(
                b.x.0 + k * self.rx * libm::sin(to),
                b.y.0 - k * self.ry * libm::cos(to),
            ),
            b,
        ));
    }
}
fn corners(w: f64, h: f64, radii: [f64; 4], half: [f64; 4], outer: bool) -> [Corner; 4] {
    std::array::from_fn(|i| {
        let r = radii[i];
        let left = i == 0 || i == 3;
        let top = i < 2;
        let hx = half[if left { 3 } else { 1 }];
        let hy = half[if top { 0 } else { 2 }];
        if r == 0.0 {
            let x = if outer { -hx } else { hx.min(w / 2.0) };
            let y = if outer { -hy } else { hy.min(h / 2.0) };
            return Corner {
                x: if left { x } else { w - x },
                y: if top { y } else { h - y },
                rx: 0.0,
                ry: 0.0,
                sharp: true,
            };
        }
        let (cx, cy, rx, ry) = if outer {
            (r, r, r + hx, r + hy)
        } else {
            let hx = hx.min(w / 2.0);
            let hy = hy.min(h / 2.0);
            (r.max(hx), r.max(hy), (r - hx).max(0.0), (r - hy).max(0.0))
        };
        Corner {
            x: if left { cx } else { w - cx },
            y: if top { cy } else { h - cy },
            rx,
            ry,
            sharp: false,
        }
    })
}
fn section(
    path: &mut Vec<PathItem>,
    cs: &[Corner; 4],
    side: usize,
    reverse: bool,
    move_to: bool,
) {
    // Top is TL225°→270° + TR270°→315°, rotating clockwise by each side.
    let a = side;
    let b = (side + 1) % 4;
    let start = std::f64::consts::PI * (1.25 + side as f64 * 0.5);
    let tangent = start + std::f64::consts::FRAC_PI_4;
    let end = start + std::f64::consts::FRAC_PI_2;
    let first = if reverse { cs[b].at(end) } else { cs[a].at(start) };
    if move_to {
        path.push(PathItem::MoveTo(first));
    } else {
        path.push(PathItem::LineTo(first));
    }
    if reverse {
        cs[b].arc(path, end, tangent);
        path.push(PathItem::LineTo(cs[a].at(tangent)));
        cs[a].arc(path, tangent, start);
    } else {
        cs[a].arc(path, start, tangent);
        path.push(PathItem::LineTo(cs[b].at(tangent)));
        cs[b].arc(path, tangent, end);
    }
}
fn band(w: f64, h: f64, r: [f64; 4], half: [f64; 4], side: usize) -> Vec<PathItem> {
    let outer = corners(w, h, r, half, true);
    let inner = corners(w, h, r, half, false);
    let mut p = Vec::new();
    section(&mut p, &outer, side, false, true);
    section(&mut p, &inner, side, true, false);
    p.push(PathItem::ClosePath);
    p
}
fn centerline(w: f64, h: f64, r: [f64; 4], side: usize) -> Vec<PathItem> {
    let cs = corners(w, h, r, [0.0; 4], true);
    let mut p = Vec::new();
    section(&mut p, &cs, side, false, true);
    p
}
fn push<M: FontMetrics, S: ImageSizer>(
    l: &mut Layouter<M, S>,
    e: &ShapeElem,
    frame: (Point, f64, f64),
    kind: ShapeKind<Pt>,
    fill: Option<FramePaint>,
    stroke: Option<FrameStroke>,
) {
    let (pos, w, h) = frame;
    l.regions.current.current_items.push(FrameItem::Shape {
        pos,
        kind,
        width: w,
        height: h,
        fill,
        stroke,
        fill_rule: e.fill_rule,
        parent_bbox_at_emit: l.parent_bbox,
    });
}

pub(super) fn emit<M: FontMetrics, S: ImageSizer>(
    l: &mut Layouter<M, S>,
    e: &ShapeElem,
    pos: Point,
    w: f64,
    h: f64,
    legacy: ShapeKind<Pt>,
) -> bool {
    if e.rect_radius.is_none() && e.rect_stroke.is_none() {
        return false;
    }
    let old = match legacy {
        ShapeKind::Rect => [0.0; 4],
        ShapeKind::RoundedRect { radii } => [
            radii.top_left.0,
            radii.top_right.0,
            radii.bottom_right.0,
            radii.bottom_left.0,
        ],
        ShapeKind::Ellipse | ShapeKind::Line { .. } | ShapeKind::Path(_) => return false,
    };
    let limit = w.min(h).max(0.0) / 2.0;
    let r = if let Some(c) = e.rect_radius {
        [c.top_left, c.top_right, c.bottom_right, c.bottom_left].map(|v| {
            (v.abs.resolve_pt(l.style.size.0) + v.rel * w.min(h))
                .max(0.0)
                .min(limit)
        })
    } else {
        old.map(|v| v.max(0.0).min(limit))
    };
    let kind = if r.iter().all(|v| *v == 0.0) {
        ShapeKind::Rect
    } else {
        ShapeKind::RoundedRect {
            radii: Corners::new(Pt(r[0]), Pt(r[1]), Pt(r[2]), Pt(r[3])),
        }
    };
    let fill = super::super::tiling::resolve_paint(l, e.fill.as_ref());
    let Some(sides) = e.rect_stroke.as_ref() else {
        let stroke = super::super::tiling::resolve_stroke(l, e.stroke.as_ref());
        push(l, e, (pos, w, h), kind, fill, stroke);
        return true;
    };
    if sides.top == sides.right && sides.top == sides.bottom && sides.top == sides.left {
        let stroke = super::super::tiling::resolve_stroke(l, sides.top.as_ref());
        push(l, e, (pos, w, h), kind, fill, stroke);
        return true;
    }
    if fill.is_some() {
        push(l, e, (pos, w, h), kind, fill, None);
    }
    let source = [&sides.top, &sides.right, &sides.bottom, &sides.left];
    let prepared = source.map(|s| super::super::tiling::resolve_stroke(l, s.as_ref()));
    let half = prepared
        .each_ref()
        .map(|s| s.as_ref().map_or(0.0, |s| s.thickness.max(0.0) / 2.0));
    let mut groups: Vec<(Paint, FramePaint, Vec<PathItem>)> = Vec::new();
    for side in 0..4 {
        let (Some(declared), Some(stroke)) =
            (source[side].as_ref(), prepared[side].as_ref())
        else {
            continue;
        };
        if stroke.thickness <= 0.0 {
            continue;
        }
        if stroke.dash.is_some()
            || stroke.cap != LineCap::Butt
            || stroke.join != LineJoin::Miter
            || stroke.miter_limit != 4.0
        {
            push(
                l,
                e,
                (pos, w, h),
                ShapeKind::Path(centerline(w, h, r, side)),
                None,
                Some(stroke.clone()),
            );
            continue;
        }
        let paint = &declared.paint;
        let path = band(w, h, r, half, side);
        if let Some(group) = groups.iter_mut().find(|g| &g.0 == paint) {
            group.2.extend(path);
        } else {
            groups.push((paint.clone(), stroke.paint.clone(), path));
        }
    }
    for (_, paint, path) in groups {
        push(l, e, (pos, w, h), ShapeKind::Path(path), Some(paint), None);
    }
    true
}

#[cfg(test)]
mod issue8583_geometry_tests {
    use super::*;
    fn contains(path: &[PathItem], q: Point) -> bool {
        let mut p = point(0.0, 0.0);
        let mut first = p;
        let mut inside = false;
        let cross = |a: Point, b: Point| {
            (a.y.0 > q.y.0) != (b.y.0 > q.y.0)
                && q.x.0 < (b.x.0 - a.x.0) * (q.y.0 - a.y.0) / (b.y.0 - a.y.0) + a.x.0
        };
        for item in path {
            match *item {
                PathItem::MoveTo(v) => {
                    p = v;
                    first = v;
                }
                PathItem::LineTo(v) => {
                    inside ^= cross(p, v);
                    p = v;
                }
                PathItem::ClosePath => {
                    inside ^= cross(p, first);
                    p = first;
                }
                PathItem::CubicTo(a, b, c) => {
                    let start = p;
                    for i in 1..=64 {
                        let t = i as f64 / 64.0;
                        let u = 1.0 - t;
                        let v = point(
                            u * u * u * start.x.0
                                + 3.0 * u * u * t * a.x.0
                                + 3.0 * u * t * t * b.x.0
                                + t * t * t * c.x.0,
                            u * u * u * start.y.0
                                + 3.0 * u * u * t * a.y.0
                                + 3.0 * u * t * t * b.y.0
                                + t * t * t * c.y.0,
                        );
                        inside ^= cross(p, v);
                        p = v;
                    }
                }
            }
        }
        inside
    }
    #[test]
    fn issue8583_border_covers_the_outside_of_the_original_fill_corner() {
        let radii = [0.0, 0.0, 30.0, 30.0];
        let half = [0.0, 0.125, 11.0, 0.125];
        let all: Vec<_> = (0..4)
            .filter(|i| half[*i] > 0.0)
            .flat_map(|i| band(60.0, 60.0, radii, half, i))
            .collect();
        // This point is outside the fill circle but in the requested border.
        // The separated upstream stroke leaves a white wedge in this region.
        assert!(contains(&all, point(3.0, 44.0)));
        assert!(!contains(&all, point(3.0, 58.0)), "do not fill the whole corner box");
        assert!(!contains(&all, point(30.0, 20.0)), "do not turn border into fill");
    }
    #[test]
    fn issue8583_side_bands_partition_sharp_and_collapsed_inner_corners() {
        let half = [1.0, 3.0, 4.0, 2.0];
        assert!(contains(&band(40.0, 30.0, [0.0; 4], half, 0), point(20.0, 0.0)));
        assert!(!contains(&band(40.0, 30.0, [0.0; 4], half, 0), point(20.0, 5.0)));
        let p = band(40.0, 30.0, [1.0; 4], [30.0; 4], 2);
        for item in p {
            match item {
                PathItem::MoveTo(p) | PathItem::LineTo(p) => {
                    assert!(p.x.0.is_finite() && p.y.0.is_finite())
                }
                PathItem::CubicTo(a, b, c) => {
                    for p in [a, b, c] {
                        assert!(p.x.0.is_finite() && p.y.0.is_finite());
                    }
                }
                PathItem::ClosePath => {}
            }
        }
    }
}
