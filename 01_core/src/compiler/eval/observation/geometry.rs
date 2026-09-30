//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/observation/geometry.md
//! @prompt-hash 8017b030
//! @layer L1
//! @updated 2026-09-26
//!
//! Relação observacional causal — família `geometry`.

use super::*;

impl ObservationEq for crate::entities::layout_types::Align2D {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { h: _, v: _ } = self;
        ObservationRelation::Same
            .and(self.h.observation_relation(&other.h))
            .and(self.v.observation_relation(&other.v))
    }
}
impl ObservationEq for crate::entities::layout_types::HAlign {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Left => match other {
                Self::Left => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Center => match other {
                Self::Center => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Right => match other {
                Self::Right => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Start => match other {
                Self::Start => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::End => match other {
                Self::End => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::layout_types::VAlign {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Top => match other {
                Self::Top => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Horizon => match other {
                Self::Horizon => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Bottom => match other {
                Self::Bottom => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::color::SpotColorant {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { name: _, fallback: _ } = self;
        ObservationRelation::Same
            .and(self.name.observation_relation(&other.name))
            .and(self.fallback.observation_relation(&other.fallback))
    }
}
impl<T: ObservationEq> ObservationEq for crate::entities::rel::Rel<T> {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { rel: _, abs: _ } = self;
        ObservationRelation::Same
            .and(self.rel.observation_relation(&other.rel))
            .and(self.abs.observation_relation(&other.abs))
    }
}
impl ObservationEq for crate::entities::layout_types::Length {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { abs: _, em: _ } = self;
        ObservationRelation::Same
            .and(self.abs.observation_relation(&other.abs))
            .and(self.em.observation_relation(&other.em))
    }
}
impl ObservationEq for crate::entities::layout_types::Abs {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self(_) = self;
        ObservationRelation::Same.and(self.0.observation_relation(&other.0))
    }
}
impl ObservationEq for crate::entities::layout_types::TextEdge {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Metric(a0) => match other {
                Self::Metric(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Length(a0) => match other {
                Self::Length(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::color::Color {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Srgb { r: a0, g: a1, b: a2, a: a3 } => match other {
                Self::Srgb { r: b0, g: b1, b: b2, a: b3 } => ObservationRelation::Same
                    .and(a0.observation_relation(b0))
                    .and(a1.observation_relation(b1))
                    .and(a2.observation_relation(b2))
                    .and(a3.observation_relation(b3)),
                _ => ObservationRelation::Different,
            },
            Self::Luma { l: a0, a: a1 } => match other {
                Self::Luma { l: b0, a: b1 } => ObservationRelation::Same
                    .and(a0.observation_relation(b0))
                    .and(a1.observation_relation(b1)),
                _ => ObservationRelation::Different,
            },
            Self::LinearRgb { r: a0, g: a1, b: a2, a: a3 } => match other {
                Self::LinearRgb { r: b0, g: b1, b: b2, a: b3 } => {
                    ObservationRelation::Same
                        .and(a0.observation_relation(b0))
                        .and(a1.observation_relation(b1))
                        .and(a2.observation_relation(b2))
                        .and(a3.observation_relation(b3))
                }
                _ => ObservationRelation::Different,
            },
            Self::Oklab { l: a0, a: a1, b: a2, alpha: a3 } => match other {
                Self::Oklab { l: b0, a: b1, b: b2, alpha: b3 } => {
                    ObservationRelation::Same
                        .and(a0.observation_relation(b0))
                        .and(a1.observation_relation(b1))
                        .and(a2.observation_relation(b2))
                        .and(a3.observation_relation(b3))
                }
                _ => ObservationRelation::Different,
            },
            Self::Oklch { l: a0, c: a1, h: a2, alpha: a3 } => match other {
                Self::Oklch { l: b0, c: b1, h: b2, alpha: b3 } => {
                    ObservationRelation::Same
                        .and(a0.observation_relation(b0))
                        .and(a1.observation_relation(b1))
                        .and(a2.observation_relation(b2))
                        .and(a3.observation_relation(b3))
                }
                _ => ObservationRelation::Different,
            },
            Self::Hsl { h: a0, s: a1, l: a2, a: a3 } => match other {
                Self::Hsl { h: b0, s: b1, l: b2, a: b3 } => ObservationRelation::Same
                    .and(a0.observation_relation(b0))
                    .and(a1.observation_relation(b1))
                    .and(a2.observation_relation(b2))
                    .and(a3.observation_relation(b3)),
                _ => ObservationRelation::Different,
            },
            Self::Hsv { h: a0, s: a1, v: a2, a: a3 } => match other {
                Self::Hsv { h: b0, s: b1, v: b2, a: b3 } => ObservationRelation::Same
                    .and(a0.observation_relation(b0))
                    .and(a1.observation_relation(b1))
                    .and(a2.observation_relation(b2))
                    .and(a3.observation_relation(b3)),
                _ => ObservationRelation::Different,
            },
            Self::Cmyk { c: a0, m: a1, y: a2, k: a3 } => match other {
                Self::Cmyk { c: b0, m: b1, y: b2, k: b3 } => ObservationRelation::Same
                    .and(a0.observation_relation(b0))
                    .and(a1.observation_relation(b1))
                    .and(a2.observation_relation(b2))
                    .and(a3.observation_relation(b3)),
                _ => ObservationRelation::Different,
            },
            Self::Spot(a) => match other {
                Self::Spot(b) => ObservationRelation::Same
                    .and(a.colorant.observation_relation(&b.colorant))
                    .and(a.tint.observation_relation(&b.tint)),
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::paint::Paint {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Solid(a0) => match other {
                Self::Solid(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Gradient(a0) => match other {
                Self::Gradient(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Tiling(a0) => match other {
                Self::Tiling(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::geometry::Stroke {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self {
            paint: _,
            thickness: _,
            cap: _,
            join: _,
            dash: _,
            miter_limit: _,
            specified: _,
            overhang: _,
        } = self;
        ObservationRelation::Same
            .and(self.paint.observation_relation(&other.paint))
            .and(self.thickness.observation_relation(&other.thickness))
            .and(self.cap.observation_relation(&other.cap))
            .and(self.join.observation_relation(&other.join))
            .and(self.dash.observation_relation(&other.dash))
            .and(self.miter_limit.observation_relation(&other.miter_limit))
            .and(self.specified.observation_relation(&other.specified))
            .and(self.overhang.observation_relation(&other.overhang))
    }
}
impl ObservationEq for crate::entities::geometry::DashPattern {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { array: _, phase: _ } = self;
        ObservationRelation::Same
            .and(self.array.observation_relation(&other.array))
            .and(self.phase.observation_relation(&other.phase))
    }
}
impl ObservationEq for crate::entities::geometry::DashLength {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Length(a0) => match other {
                Self::Length(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::LineWidth => match other {
                Self::LineWidth => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::geometry::LineJoin {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Miter => match other {
                Self::Miter => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Round => match other {
                Self::Round => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Bevel => match other {
                Self::Bevel => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::geometry::StrokeFields {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self {
            paint: _,
            thickness: _,
            cap: _,
            join: _,
            dash: _,
            miter_limit: _,
        } = self;
        ObservationRelation::Same
            .and(self.paint.observation_relation(&other.paint))
            .and(self.thickness.observation_relation(&other.thickness))
            .and(self.cap.observation_relation(&other.cap))
            .and(self.join.observation_relation(&other.join))
            .and(self.dash.observation_relation(&other.dash))
            .and(self.miter_limit.observation_relation(&other.miter_limit))
    }
}
impl ObservationEq for crate::entities::geometry::LineCap {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Butt => match other {
                Self::Butt => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Round => match other {
                Self::Round => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Square => match other {
                Self::Square => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::geometry::FillRule {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::NonZero => match other {
                Self::NonZero => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::EvenOdd => match other {
                Self::EvenOdd => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl<T: ObservationEq> ObservationEq for crate::entities::geometry::ShapeKind<T> {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Rect => match other {
                Self::Rect => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::RoundedRect { radii: a0 } => match other {
                Self::RoundedRect { radii: b0 } => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Ellipse => match other {
                Self::Ellipse => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Line { dx: a0, dy: a1 } => match other {
                Self::Line { dx: b0, dy: b1 } => ObservationRelation::Same
                    .and(a0.observation_relation(b0))
                    .and(a1.observation_relation(b1)),
                _ => ObservationRelation::Different,
            },
            Self::Path(a0) => match other {
                Self::Path(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::geometry::PathItem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::MoveTo(a0) => match other {
                Self::MoveTo(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::LineTo(a0) => match other {
                Self::LineTo(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::CubicTo(a0, a1, a2) => match other {
                Self::CubicTo(b0, b1, b2) => ObservationRelation::Same
                    .and(a0.observation_relation(b0))
                    .and(a1.observation_relation(b1))
                    .and(a2.observation_relation(b2)),
                _ => ObservationRelation::Different,
            },
            Self::ClosePath => match other {
                Self::ClosePath => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::layout_types::Point {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { x: _, y: _ } = self;
        ObservationRelation::Same
            .and(self.x.observation_relation(&other.x))
            .and(self.y.observation_relation(&other.y))
    }
}
impl ObservationEq for crate::entities::layout_types::Pt {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self(_) = self;
        ObservationRelation::Same.and(self.0.observation_relation(&other.0))
    }
}
impl<T: ObservationEq> ObservationEq for crate::entities::corners::Corners<T> {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self {
            top_left: _,
            top_right: _,
            bottom_right: _,
            bottom_left: _,
        } = self;
        ObservationRelation::Same
            .and(self.top_left.observation_relation(&other.top_left))
            .and(self.top_right.observation_relation(&other.top_right))
            .and(self.bottom_right.observation_relation(&other.bottom_right))
            .and(self.bottom_left.observation_relation(&other.bottom_left))
    }
}
impl<T: ObservationEq> ObservationEq for crate::entities::sides::Sides<T> {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { left: _, top: _, right: _, bottom: _ } = self;
        ObservationRelation::Same
            .and(self.left.observation_relation(&other.left))
            .and(self.top.observation_relation(&other.top))
            .and(self.right.observation_relation(&other.right))
            .and(self.bottom.observation_relation(&other.bottom))
    }
}
impl ObservationEq for crate::entities::layout_types::TrackSizing {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Fixed(a0) => match other {
                Self::Fixed(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Auto => match other {
                Self::Auto => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Fraction(a0) => match other {
                Self::Fraction(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::layout_types::PlaceScope {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Column => match other {
                Self::Column => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Parent => match other {
                Self::Parent => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::layout_types::TransformMatrix {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { a: _, b: _, c: _, d: _, tx: _, ty: _ } = self;
        ObservationRelation::Same
            .and(self.a.observation_relation(&other.a))
            .and(self.b.observation_relation(&other.b))
            .and(self.c.observation_relation(&other.c))
            .and(self.d.observation_relation(&other.d))
            .and(self.tx.observation_relation(&other.tx))
            .and(self.ty.observation_relation(&other.ty))
    }
}
impl ObservationEq for crate::entities::gradient::GradientStop {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { color: _, offset: _ } = self;
        ObservationRelation::Same
            .and(self.color.observation_relation(&other.color))
            .and(self.offset.observation_relation(&other.offset))
    }
}
impl ObservationEq for crate::entities::gradient::RelativeTo {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Self_ => match other {
                Self::Self_ => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Parent => match other {
                Self::Parent => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::gradient::Linear {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self {
            stops: _,
            angle: _,
            space: _,
            relative: _,
            anti_alias: _,
        } = self;
        ObservationRelation::Same
            .and(self.stops.observation_relation(&other.stops))
            .and(self.angle.observation_relation(&other.angle))
            .and(self.space.observation_relation(&other.space))
            .and(self.relative.observation_relation(&other.relative))
            .and(self.anti_alias.observation_relation(&other.anti_alias))
    }
}
impl ObservationEq for crate::entities::gradient::Radial {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self {
            stops: _,
            center: _,
            radius: _,
            focal_center: _,
            focal_radius: _,
            space: _,
            relative: _,
            anti_alias: _,
        } = self;
        ObservationRelation::Same
            .and(self.stops.observation_relation(&other.stops))
            .and(self.center.observation_relation(&other.center))
            .and(self.radius.observation_relation(&other.radius))
            .and(self.focal_center.observation_relation(&other.focal_center))
            .and(self.focal_radius.observation_relation(&other.focal_radius))
            .and(self.space.observation_relation(&other.space))
            .and(self.relative.observation_relation(&other.relative))
            .and(self.anti_alias.observation_relation(&other.anti_alias))
    }
}
impl ObservationEq for crate::entities::gradient::Conic {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self {
            stops: _,
            center: _,
            angle: _,
            space: _,
            relative: _,
            anti_alias: _,
        } = self;
        ObservationRelation::Same
            .and(self.stops.observation_relation(&other.stops))
            .and(self.center.observation_relation(&other.center))
            .and(self.angle.observation_relation(&other.angle))
            .and(self.space.observation_relation(&other.space))
            .and(self.relative.observation_relation(&other.relative))
            .and(self.anti_alias.observation_relation(&other.anti_alias))
    }
}
impl ObservationEq for crate::entities::gradient::Gradient {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Linear(a0) => match other {
                Self::Linear(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Radial(a0) => match other {
                Self::Radial(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Conic(a0) => match other {
                Self::Conic(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::tiling::Tiling {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self {
            body: _,
            size: _,
            relative: _,
            spacing: _,
            offset: _,
            angle: _,
        } = self;
        ObservationRelation::Same
            .and(self.body.observation_relation(&other.body))
            .and(self.size.observation_relation(&other.size))
            .and(self.relative.observation_relation(&other.relative))
            .and(self.spacing.observation_relation(&other.spacing))
            .and(self.offset.observation_relation(&other.offset))
            .and(self.angle.observation_relation(&other.angle))
    }
}
impl ObservationEq for crate::entities::tiling::TilingBody {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Content(a0) => match other {
                Self::Content(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Image(a0) => match other {
                Self::Image(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Gradient(a0) => match other {
                Self::Gradient(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Color(a0) => match other {
                Self::Color(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::tiling::TilingRelative {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Auto => match other {
                Self::Auto => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Itself => match other {
                Self::Itself => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Parent => match other {
                Self::Parent => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::layout_types::Ratio {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self(_) = self;
        ObservationRelation::Same.and(self.0.observation_relation(&other.0))
    }
}
impl ObservationEq for crate::entities::color::ColorSpace {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Spot(a) => match other {
                Self::Spot(b) => a.observation_relation(b),
                _ => ObservationRelation::Different,
            },
            Self::Oklab => match other {
                Self::Oklab => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Oklch => match other {
                Self::Oklch => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Srgb => match other {
                Self::Srgb => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Luma => match other {
                Self::Luma => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::LinearRgb => match other {
                Self::LinearRgb => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Hsl => match other {
                Self::Hsl => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Hsv => match other {
                Self::Hsv => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Cmyk => match other {
                Self::Cmyk => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl<T: ObservationEq> ObservationEq for crate::entities::axes::Axes<T> {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { x: _, y: _ } = self;
        ObservationRelation::Same
            .and(self.x.observation_relation(&other.x))
            .and(self.y.observation_relation(&other.y))
    }
}
impl ObservationEq for crate::entities::layout_types::Size {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { width: _, height: _ } = self;
        ObservationRelation::Same
            .and(self.width.observation_relation(&other.width))
            .and(self.height.observation_relation(&other.height))
    }
}
impl ObservationEq for crate::entities::layout_types::Angle {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        self.to_rad().observation_relation(&other.to_rad())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::layout_types::Abs;

    #[test]
    fn distingue_medida_e_bits_de_zero() {
        assert_eq!(
            Abs::pt(12.0).observation_relation(&Abs::pt(12.0)),
            ObservationRelation::Same
        );
        assert_eq!(
            Abs::pt(12.0).observation_relation(&Abs::pt(13.0)),
            ObservationRelation::Different
        );
        assert_eq!(
            Abs::pt(0.0).observation_relation(&Abs::pt(-0.0)),
            ObservationRelation::Different
        );
    }
}
