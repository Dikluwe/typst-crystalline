//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/eval/observation/math.md
//! @prompt-hash 66f4a09b
//! @layer L1
//! @updated 2026-09-26
//!
//! Relação observacional causal — família `math`.

use super::*;

impl ObservationEq for crate::entities::elements::math_styled::MathStyledElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { kind: _, bold: _, italic: _, body: _, cramped: _ } = self;
        ObservationRelation::Same
            .and(self.kind.observation_relation(&other.kind))
            .and(self.bold.observation_relation(&other.bold))
            .and(self.italic.observation_relation(&other.italic))
            .and(self.body.observation_relation(&other.body))
            .and(self.cramped.observation_relation(&other.cramped))
    }
}
impl ObservationEq for crate::entities::math_style::MathStyleKind {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Plain => match other {
                Self::Plain => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::SansSerif => match other {
                Self::SansSerif => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Chancery => match other {
                Self::Chancery => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Roundhand => match other {
                Self::Roundhand => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Fraktur => match other {
                Self::Fraktur => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Monospace => match other {
                Self::Monospace => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::DoubleStruck => match other {
                Self::DoubleStruck => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Script => match other {
                Self::Script => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::SScript => match other {
                Self::SScript => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Display => match other {
                Self::Display => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Inline => match other {
                Self::Inline => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::elements::math_matrix::MathMatrixElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self {
            rows: _,
            delim: _,
            align: _,
            augment: _,
            row_gap: _,
            column_gap: _,
            explicit: _,
        } = self;
        ObservationRelation::Same
            .and(self.rows.observation_relation(&other.rows))
            .and(self.delim.observation_relation(&other.delim))
            .and(self.align.observation_relation(&other.align))
            .and(self.augment.observation_relation(&other.augment))
            .and(self.row_gap.observation_relation(&other.row_gap))
            .and(self.column_gap.observation_relation(&other.column_gap))
            .and(ObservationRelation::exact(self.explicit == other.explicit))
    }
}
impl ObservationEq for crate::entities::elements::math_matrix::MathMatrixAugment {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        self.hline
            .observation_relation(&other.hline)
            .and(self.vline.observation_relation(&other.vline))
            .and(self.stroke.observation_relation(&other.stroke))
    }
}
impl ObservationEq
    for crate::entities::elements::math_limits_override::MathLimitsOverrideElem
{
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { body: _, limits: _, inline: _, inline_explicit: _ } = self;
        ObservationRelation::Same
            .and(self.body.observation_relation(&other.body))
            .and(self.limits.observation_relation(&other.limits))
            .and(self.inline.observation_relation(&other.inline))
            .and(self.inline_explicit.observation_relation(&other.inline_explicit))
    }
}
impl ObservationEq for crate::entities::elements::math_delimited::MathDelimitedElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { open: _, body: _, close: _ } = self;
        ObservationRelation::Same
            .and(self.open.observation_relation(&other.open))
            .and(self.body.observation_relation(&other.body))
            .and(self.close.observation_relation(&other.close))
    }
}
impl ObservationEq for crate::entities::elements::math_delimited::MathLrElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { body: _, size: _, size_explicit: _ } = self;
        ObservationRelation::Same
            .and(self.body.observation_relation(&other.body))
            .and(self.size.observation_relation(&other.size))
            .and(self.size_explicit.observation_relation(&other.size_explicit))
    }
}
impl ObservationEq for crate::entities::elements::math_delimited::MathMidElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { body: _ } = self;
        ObservationRelation::Same.and(self.body.observation_relation(&other.body))
    }
}
impl ObservationEq for crate::entities::elements::math_stretch::MathStretchElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { body: _, size: _, size_explicit: _ } = self;
        ObservationRelation::Same
            .and(self.body.observation_relation(&other.body))
            .and(self.size.observation_relation(&other.size))
            .and(self.size_explicit.observation_relation(&other.size_explicit))
    }
}
#[cfg(test)]
pub(crate) fn p1437_math_mid_observation_differs(
    left: &crate::entities::elements::math_delimited::MathMidElem,
    right: &crate::entities::elements::math_delimited::MathMidElem,
) -> bool {
    left.observation_relation(right) == ObservationRelation::Different
}
#[cfg(test)]
pub(crate) fn p1436_math_lr_observation_differs(
    left: &crate::entities::elements::math_delimited::MathLrElem,
    right: &crate::entities::elements::math_delimited::MathLrElem,
) -> bool {
    left.observation_relation(right) == ObservationRelation::Different
}
impl ObservationEq for crate::entities::elements::math_op::MathOpElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { text: _, limits: _ } = self;
        ObservationRelation::Same
            .and(self.text.observation_relation(&other.text))
            .and(self.limits.observation_relation(&other.limits))
    }
}
impl ObservationEq for crate::entities::elements::equation::EquationElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { body: _, block: _ } = self;
        ObservationRelation::Same
            .and(self.body.observation_relation(&other.body))
            .and(self.block.observation_relation(&other.block))
    }
}
impl ObservationEq for crate::entities::elements::math_frac::MathFracElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self {
            num: _,
            den: _,
            style: _,
            num_deparenthesized: _,
            denom_deparenthesized: _,
            line: _,
        } = self;
        ObservationRelation::Same
            .and(self.num.observation_relation(&other.num))
            .and(self.den.observation_relation(&other.den))
            .and(if self.style == other.style {
                ObservationRelation::Same
            } else {
                ObservationRelation::Different
            })
            .and(
                self.num_deparenthesized
                    .observation_relation(&other.num_deparenthesized),
            )
            .and(
                self.denom_deparenthesized
                    .observation_relation(&other.denom_deparenthesized),
            )
            .and(self.line.observation_relation(&other.line))
    }
}
impl ObservationEq for crate::entities::elements::math_vec::MathVecElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self {
            children: _,
            delim: _,
            align: _,
            gap: _,
            explicit: _,
        } = self;
        ObservationRelation::Same
            .and(self.children.observation_relation(&other.children))
            .and(self.delim.observation_relation(&other.delim))
            .and(self.align.observation_relation(&other.align))
            .and(self.gap.observation_relation(&other.gap))
            .and(self.explicit.observation_relation(&other.explicit))
    }
}
impl ObservationEq for crate::entities::elements::math_vec::MathVecExplicit {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { delim: _, align: _, gap: _ } = self;
        ObservationRelation::Same
            .and(self.delim.observation_relation(&other.delim))
            .and(self.align.observation_relation(&other.align))
            .and(self.gap.observation_relation(&other.gap))
    }
}
impl ObservationEq for crate::entities::elements::math_underline::MathUnderlineElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { body: _ } = self;
        ObservationRelation::Same.and(self.body.observation_relation(&other.body))
    }
}
impl ObservationEq for crate::entities::elements::math_underline::MathOverlineElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { body: _ } = self;
        ObservationRelation::Same.and(self.body.observation_relation(&other.body))
    }
}

#[cfg(test)]
pub(crate) fn p1441_math_overline_observation_differs(
    left: &crate::entities::elements::math_underline::MathOverlineElem,
    right: &crate::entities::elements::math_underline::MathOverlineElem,
) -> bool {
    left.observation_relation(right) == ObservationRelation::Different
}
impl ObservationEq for crate::entities::elements::math_cases::MathCasesElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self {
            children: _,
            delim: _,
            reverse: _,
            gap: _,
            explicit: _,
        } = self;
        ObservationRelation::Same
            .and(self.children.observation_relation(&other.children))
            .and(self.delim.observation_relation(&other.delim))
            .and(self.reverse.observation_relation(&other.reverse))
            .and(self.gap.observation_relation(&other.gap))
            .and(self.explicit.observation_relation(&other.explicit))
    }
}
impl ObservationEq for crate::entities::elements::math_cases::MathCasesExplicit {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { delim: _, reverse: _, gap: _ } = self;
        ObservationRelation::Same
            .and(self.delim.observation_relation(&other.delim))
            .and(self.reverse.observation_relation(&other.reverse))
            .and(self.gap.observation_relation(&other.gap))
    }
}
impl ObservationEq
    for crate::entities::elements::math_class_override::MathClassOverrideElem
{
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { class: _, body: _ } = self;
        ObservationRelation::Same
            .and(self.class.observation_relation(&other.class))
            .and(self.body.observation_relation(&other.body))
    }
}
impl ObservationEq for crate::entities::math_class::MathClass {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Normal => match other {
                Self::Normal => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Alphabetic => match other {
                Self::Alphabetic => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Binary => match other {
                Self::Binary => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Closing => match other {
                Self::Closing => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Diacritic => match other {
                Self::Diacritic => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Fence => match other {
                Self::Fence => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::GlyphPart => match other {
                Self::GlyphPart => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Large => match other {
                Self::Large => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Opening => match other {
                Self::Opening => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Punctuation => match other {
                Self::Punctuation => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Relation => match other {
                Self::Relation => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Space => match other {
                Self::Space => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Unary => match other {
                Self::Unary => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Vary => match other {
                Self::Vary => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Special => match other {
                Self::Special => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::elements::math_underover::MathUnderoverElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { base: _, under: _, over: _ } = self;
        ObservationRelation::Same
            .and(self.base.observation_relation(&other.base))
            .and(self.under.observation_relation(&other.under))
            .and(self.over.observation_relation(&other.over))
    }
}
impl ObservationEq for crate::entities::elements::math_underover::MathOverbraceElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { body: _, annotation: _, annotation_explicit: _ } = self;
        ObservationRelation::Same
            .and(self.body.observation_relation(&other.body))
            .and(self.annotation.observation_relation(&other.annotation))
            .and(
                self.annotation_explicit
                    .observation_relation(&other.annotation_explicit),
            )
    }
}
impl ObservationEq for crate::entities::elements::math_underover::MathUnderbraceElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { body: _, annotation: _, annotation_explicit: _ } = self;
        ObservationRelation::Same
            .and(self.body.observation_relation(&other.body))
            .and(self.annotation.observation_relation(&other.annotation))
            .and(
                self.annotation_explicit
                    .observation_relation(&other.annotation_explicit),
            )
    }
}
impl ObservationEq for crate::entities::elements::math_underover::MathUnderbracketElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { body: _, annotation: _, annotation_explicit: _ } = self;
        ObservationRelation::Same
            .and(self.body.observation_relation(&other.body))
            .and(self.annotation.observation_relation(&other.annotation))
            .and(
                self.annotation_explicit
                    .observation_relation(&other.annotation_explicit),
            )
    }
}
impl ObservationEq for crate::entities::elements::math_underover::MathUnderparenElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { body: _, annotation: _, annotation_explicit: _ } = self;
        ObservationRelation::Same
            .and(self.body.observation_relation(&other.body))
            .and(self.annotation.observation_relation(&other.annotation))
            .and(
                self.annotation_explicit
                    .observation_relation(&other.annotation_explicit),
            )
    }
}

#[cfg(test)]
pub(crate) fn p1450_math_underparen_observation_differs(
    left: &crate::entities::elements::math_underover::MathUnderparenElem,
    right: &crate::entities::elements::math_underover::MathUnderparenElem,
) -> bool {
    left.observation_relation(right) == ObservationRelation::Different
}
impl ObservationEq for crate::entities::elements::math_underover::MathUndershellElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { body: _, annotation: _, annotation_explicit: _ } = self;
        ObservationRelation::Same
            .and(self.body.observation_relation(&other.body))
            .and(self.annotation.observation_relation(&other.annotation))
            .and(if self.annotation_explicit == other.annotation_explicit {
                ObservationRelation::Same
            } else {
                ObservationRelation::Different
            })
    }
}

#[cfg(test)]
pub(crate) fn p1451_math_undershell_observation_differs(
    left: &crate::entities::elements::math_underover::MathUndershellElem,
    right: &crate::entities::elements::math_underover::MathUndershellElem,
) -> bool {
    left.observation_relation(right) == ObservationRelation::Different
}

#[cfg(test)]
pub(crate) fn p1449_math_underbracket_observation_differs(
    left: &crate::entities::elements::math_underover::MathUnderbracketElem,
    right: &crate::entities::elements::math_underover::MathUnderbracketElem,
) -> bool {
    left.observation_relation(right) == ObservationRelation::Different
}

#[cfg(test)]
pub(crate) fn p1448_math_underbrace_observation_differs(
    left: &crate::entities::elements::math_underover::MathUnderbraceElem,
    right: &crate::entities::elements::math_underover::MathUnderbraceElem,
) -> bool {
    left.observation_relation(right) == ObservationRelation::Different
}
impl ObservationEq for crate::entities::elements::math_underover::MathOverbracketElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { body: _, annotation: _, annotation_explicit: _ } = self;
        ObservationRelation::Same
            .and(self.body.observation_relation(&other.body))
            .and(self.annotation.observation_relation(&other.annotation))
            .and(
                self.annotation_explicit
                    .observation_relation(&other.annotation_explicit),
            )
    }
}
impl ObservationEq for crate::entities::elements::math_underover::MathOverparenElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { body: _, annotation: _, annotation_explicit: _ } = self;
        ObservationRelation::Same
            .and(self.body.observation_relation(&other.body))
            .and(self.annotation.observation_relation(&other.annotation))
            .and(
                self.annotation_explicit
                    .observation_relation(&other.annotation_explicit),
            )
    }
}
impl ObservationEq for crate::entities::elements::math_underover::MathOvershellElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { body: _, annotation: _, annotation_explicit: _ } = self;
        ObservationRelation::Same
            .and(self.body.observation_relation(&other.body))
            .and(self.annotation.observation_relation(&other.annotation))
            .and(
                self.annotation_explicit
                    .observation_relation(&other.annotation_explicit),
            )
    }
}

#[cfg(test)]
pub(crate) fn p1439_math_overbrace_observation_differs(
    left: &crate::entities::elements::math_underover::MathOverbraceElem,
    right: &crate::entities::elements::math_underover::MathOverbraceElem,
) -> bool {
    left.observation_relation(right) == ObservationRelation::Different
}

#[cfg(test)]
pub(crate) fn p1440_math_overbracket_observation_differs(
    left: &crate::entities::elements::math_underover::MathOverbracketElem,
    right: &crate::entities::elements::math_underover::MathOverbracketElem,
) -> bool {
    left.observation_relation(right) == ObservationRelation::Different
}
#[cfg(test)]
pub(crate) fn p1442_math_overparen_observation_differs(
    left: &crate::entities::elements::math_underover::MathOverparenElem,
    right: &crate::entities::elements::math_underover::MathOverparenElem,
) -> bool {
    left.observation_relation(right) == ObservationRelation::Different
}
#[cfg(test)]
pub(crate) fn p1443_math_overshell_observation_differs(
    left: &crate::entities::elements::math_underover::MathOvershellElem,
    right: &crate::entities::elements::math_underover::MathOvershellElem,
) -> bool {
    left.observation_relation(right) == ObservationRelation::Different
}
impl ObservationEq for crate::entities::elements::math_cancel::MathCancelElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self {
            body: _,
            length: _,
            inverted: _,
            cross: _,
            angle: _,
            stroke: _,
            background: _,
            span: _,
            explicit: _,
        } = self;
        ObservationRelation::Same
            .and(self.body.observation_relation(&other.body))
            .and(self.length.observation_relation(&other.length))
            .and(self.inverted.observation_relation(&other.inverted))
            .and(self.cross.observation_relation(&other.cross))
            .and(self.angle.observation_relation(&other.angle))
            .and(self.stroke.observation_relation(&other.stroke))
            .and(self.background.observation_relation(&other.background))
            .and(self.span.observation_relation(&other.span))
            .and(self.explicit.observation_relation(&other.explicit))
    }
}
impl ObservationEq for crate::entities::elements::math_cancel::MathCancelExplicit {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self {
            length: _,
            inverted: _,
            cross: _,
            angle: _,
            stroke: _,
            background: _,
        } = self;
        ObservationRelation::Same
            .and(self.length.observation_relation(&other.length))
            .and(self.inverted.observation_relation(&other.inverted))
            .and(self.cross.observation_relation(&other.cross))
            .and(self.angle.observation_relation(&other.angle))
            .and(self.stroke.observation_relation(&other.stroke))
            .and(self.background.observation_relation(&other.background))
    }
}
impl ObservationEq for crate::entities::elements::math_cancel::MathCancelAngle {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Auto => match other {
                Self::Auto => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Angle(a0) => match other {
                Self::Angle(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Func(a0) => match other {
                Self::Func(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::elements::math_align_point::MathAlignPointElem {
    fn observation_relation(&self, _: &Self) -> ObservationRelation {
        let Self = self;
        ObservationRelation::Same
    }
}
impl ObservationEq for crate::entities::elements::math_root::MathRootElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { index: _, radicand: _, index_explicit: _ } = self;
        ObservationRelation::Same
            .and(self.index.observation_relation(&other.index))
            .and(self.radicand.observation_relation(&other.radicand))
            .and(self.index_explicit.observation_relation(&other.index_explicit))
    }
}
impl ObservationEq for crate::entities::elements::math_accent::MathAccentElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { base: _, accent: _, .. } = self;
        ObservationRelation::Same
            .and(self.base.observation_relation(&other.base))
            .and(self.accent.observation_relation(&other.accent))
            .and(self.size.observation_relation(&other.size))
            .and(self.dotless.observation_relation(&other.dotless))
            .and(self.explicit.size.observation_relation(&other.explicit.size))
            .and(self.explicit.dotless.observation_relation(&other.explicit.dotless))
    }
}
impl ObservationEq for crate::entities::elements::math_attach::MathAttachElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { base: _, t: _, b: _, tl: _, bl: _, tr: _, br: _ } = self;
        ObservationRelation::Same
            .and(self.base.observation_relation(&other.base))
            .and(self.t.observation_relation(&other.t))
            .and(self.b.observation_relation(&other.b))
            .and(self.tl.observation_relation(&other.tl))
            .and(self.bl.observation_relation(&other.bl))
            .and(self.tr.observation_relation(&other.tr))
            .and(self.br.observation_relation(&other.br))
    }
}
impl ObservationEq for crate::entities::elements::math_attach::MathPrimesElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { count: _ } = self;
        ObservationRelation::Same.and(self.count.observation_relation(&other.count))
    }
}
#[cfg(test)]
pub(crate) fn p1444_math_primes_observation_differs(
    left: &crate::entities::elements::math_attach::MathPrimesElem,
    right: &crate::entities::elements::math_attach::MathPrimesElem,
) -> bool {
    left.observation_relation(right) == ObservationRelation::Different
}
impl ObservationEq for crate::entities::elements::math_attach::MathAttachSlot {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Omitted => match other {
                Self::Omitted => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::ExplicitNone => match other {
                Self::ExplicitNone => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Present(a0) => match other {
                Self::Present(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
        }
    }
}
