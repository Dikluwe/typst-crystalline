//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/entities/frame_paint.md
//! @prompt-hash 4b56e442
//! @layer L1
//! @updated 2026-10-14

use std::sync::Arc;

use crate::entities::geometry::{DashPattern, LineCap, LineJoin, StrokeFields};
use crate::entities::gradient::{Gradient, RelativeTo};
use crate::entities::layout_types::{Angle, Color, Frame, Point, Size};

/// Paint whose tiling cell has already been prepared by layout.
///
/// Declarative paint cannot be converted without materializing its content:
///
/// ```compile_fail
/// use typst_core::entities::{frame_paint::FramePaint, paint::Paint};
/// fn unresolved(paint: Paint) -> FramePaint { paint.into() }
/// ```
///
/// Prepared frames have no language-level equality:
///
/// ```compile_fail
/// use typst_core::entities::frame_paint::FramePaint;
/// fn assert_equality<T: PartialEq>() {}
/// assert_equality::<FramePaint>();
/// ```
#[derive(Debug, Clone)]
pub enum FramePaint {
    Solid(Color),
    Gradient(Gradient),
    Tiling(Arc<TilingFrame>),
}

/// One prepared cell, shared independently of the number of visible repetitions.
///
/// Layout validates finite dimensions, positive pitch and finite placement.
/// `spacing` is signed; pitch is `frame.size + spacing`. `offset` is resolved
/// against that pitch. The frame establishes its own paint root.
#[derive(Debug, Clone)]
pub struct TilingFrame {
    pub frame: Frame,
    pub spacing: Size,
    pub offset: Point,
    pub angle: Angle,
    pub relative: RelativeTo,
}

/// A stroke with prepared paint and unchanged contour geometry/presence.
#[derive(Debug, Clone)]
pub struct FrameStroke {
    pub paint: FramePaint,
    pub thickness: f64,
    pub cap: LineCap,
    pub join: LineJoin,
    pub dash: Option<DashPattern>,
    pub miter_limit: f64,
    pub specified: StrokeFields,
    pub overhang: bool,
}

impl Default for FrameStroke {
    fn default() -> Self {
        Self {
            paint: FramePaint::Solid(Color::rgb(0, 0, 0)),
            thickness: 1.0,
            cap: LineCap::Butt,
            join: LineJoin::Miter,
            dash: None,
            miter_limit: 4.0,
            specified: StrokeFields::default(),
            overhang: false,
        }
    }
}

impl From<Color> for FramePaint {
    fn from(color: Color) -> Self {
        Self::Solid(color)
    }
}

impl From<Gradient> for FramePaint {
    fn from(gradient: Gradient) -> Self {
        Self::Gradient(gradient)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::geometry::{DashLength, Stroke};
    use crate::entities::gradient::GradientStop;
    use crate::entities::layout_types::{Pt, Ratio};
    use crate::entities::paint::Paint;

    #[test]
    fn solid_and_gradient_transport_preserves_the_full_paint() {
        let color = Color::rgb(12, 80, 201);
        let FramePaint::Solid(actual) = FramePaint::from(color.clone()) else {
            panic!("solid paint changed kind");
        };
        assert_eq!(actual, color);
        let gradient = Gradient::linear(
            vec![
                GradientStop::new(color, Ratio(0.125)),
                GradientStop::new(Color::rgb(200, 30, 10), Ratio(0.875)),
            ],
            Angle::deg(33.0),
        );
        let FramePaint::Gradient(actual) = FramePaint::from(gradient.clone()) else {
            panic!("gradient was reduced to a representative color");
        };
        assert_eq!(actual, gradient);
    }

    #[test]
    fn paint_clones_share_the_prepared_cell_and_preserve_placement() {
        let cell = Arc::new(TilingFrame {
            frame: Frame::new(Size { width: Pt(10.0), height: Pt(8.0) }),
            spacing: Size { width: Pt(-2.0), height: Pt(3.0) },
            offset: Point { x: Pt(3.0), y: Pt(-4.0) },
            angle: Angle::deg(30.0),
            relative: RelativeTo::Parent,
        });
        let paint = FramePaint::Tiling(cell.clone());
        let FramePaint::Tiling(copy) = paint.clone() else {
            panic!("prepared cell was lost");
        };
        // Transport boundedness, not a language equality rule.
        assert!(Arc::ptr_eq(&copy, &cell));
        assert_eq!(copy.frame.size.width.0, 10.0);
        assert_eq!(copy.frame.size.height.0, 8.0);
        assert_eq!(copy.spacing.width.0, -2.0);
        assert_eq!(copy.spacing.height.0, 3.0);
        assert_eq!(copy.offset, Point { x: Pt(3.0), y: Pt(-4.0) });
        assert_eq!(copy.angle, Angle::deg(30.0));
        assert_eq!(copy.relative, RelativeTo::Parent);
        assert!(copy.frame.items.is_empty(), "an empty cell must stay transparent");
    }

    #[test]
    fn frame_stroke_default_matches_the_existing_stroke_contract() {
        let old = Stroke::default();
        let new = FrameStroke::default();
        let (Paint::Solid(expected), FramePaint::Solid(actual)) = (old.paint, new.paint)
        else {
            panic!("default stroke paint must remain solid");
        };
        assert_eq!(actual, expected);
        assert_eq!(new.thickness, old.thickness.resolve_pt(12.0));
        assert_eq!(new.cap, old.cap);
        assert_eq!(new.join, old.join);
        assert_eq!(new.dash, old.dash);
        assert_eq!(new.miter_limit, old.miter_limit);
        assert_eq!(new.specified, old.specified);
        assert_eq!(new.overhang, old.overhang);
    }

    #[test]
    fn frame_stroke_clone_retains_geometry_and_field_presence() {
        let dash = DashPattern {
            array: vec![DashLength::Length(3.0), DashLength::LineWidth],
            phase: 1.25,
        };
        let specified = StrokeFields {
            paint: true,
            thickness: true,
            cap: false,
            join: true,
            dash: true,
            miter_limit: false,
        };
        let original = FrameStroke {
            paint: FramePaint::from(Color::rgb(1, 2, 3)),
            thickness: 2.5,
            cap: LineCap::Square,
            join: LineJoin::Bevel,
            dash: Some(dash.clone()),
            miter_limit: 7.0,
            specified,
            overhang: true,
        };
        let copy = original.clone();
        assert_eq!(copy.thickness, 2.5);
        assert_eq!(copy.cap, LineCap::Square);
        assert_eq!(copy.join, LineJoin::Bevel);
        assert_eq!(copy.dash, Some(dash));
        assert_eq!(copy.miter_limit, 7.0);
        assert_eq!(copy.specified, specified);
        assert!(copy.overhang);
    }
}
