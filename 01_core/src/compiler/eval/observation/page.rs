//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/observation/page.md
//! @prompt-hash 268b2547
//! @layer L1
//! @updated 2026-09-26
//!
//! Relação observacional causal — família `page`.

use super::*;

impl ObservationEq for crate::entities::elements::page_run::PageRunElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self {
            paper: _,
            flipped: _,
            binding: _,
            width: _,
            height: _,
            margin: _,
            numbering: _,
            number_align: _,
            header: _,
            header_ascent: _,
            footer: _,
            footer_descent: _,
            supplement: _,
            columns: _,
            bleed: _,
            fill: _,
            background: _,
            foreground: _,
            body: _,
        } = self;
        ObservationRelation::Same
            .and(self.paper.observation_relation(&other.paper))
            .and(self.flipped.observation_relation(&other.flipped))
            .and(self.binding.observation_relation(&other.binding))
            .and(self.width.observation_relation(&other.width))
            .and(self.height.observation_relation(&other.height))
            .and(self.margin.observation_relation(&other.margin))
            .and(self.numbering.observation_relation(&other.numbering))
            .and(self.number_align.observation_relation(&other.number_align))
            .and(self.header.observation_relation(&other.header))
            .and(self.header_ascent.observation_relation(&other.header_ascent))
            .and(self.footer.observation_relation(&other.footer))
            .and(self.footer_descent.observation_relation(&other.footer_descent))
            .and(self.supplement.observation_relation(&other.supplement))
            .and(self.columns.observation_relation(&other.columns))
            .and(self.bleed.observation_relation(&other.bleed))
            .and(self.fill.observation_relation(&other.fill))
            .and(self.background.observation_relation(&other.background))
            .and(self.foreground.observation_relation(&other.foreground))
            .and(self.body.observation_relation(&other.body))
    }
}
impl ObservationEq for crate::entities::page_canvas::PageBleedSpec {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { left: _, right: _, top: _, bottom: _, two_sided: _ } = self;
        ObservationRelation::Same
            .and(self.left.observation_relation(&other.left))
            .and(self.right.observation_relation(&other.right))
            .and(self.top.observation_relation(&other.top))
            .and(self.bottom.observation_relation(&other.bottom))
            .and(self.two_sided.observation_relation(&other.two_sided))
    }
}
impl ObservationEq for crate::entities::page_running::PageNumberAlign {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { horizontal: _, vertical: _ } = self;
        ObservationRelation::Same
            .and(self.horizontal.observation_relation(&other.horizontal))
            .and(self.vertical.observation_relation(&other.vertical))
    }
}
impl ObservationEq for crate::entities::page_running::PageNumberVAlign {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Top => match other {
                Self::Top => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Bottom => match other {
                Self::Bottom => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::page_geometry::PageBinding {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Auto => match other {
                Self::Auto => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Left => match other {
                Self::Left => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Right => match other {
                Self::Right => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::page_supplement::PageSupplement {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Auto => match other {
                Self::Auto => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::None => match other {
                Self::None => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Content(a0) => match other {
                Self::Content(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::layout_types::PageMarginSpec {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { left: _, right: _, top: _, bottom: _, two_sided: _ } = self;
        ObservationRelation::Same
            .and(self.left.observation_relation(&other.left))
            .and(self.right.observation_relation(&other.right))
            .and(self.top.observation_relation(&other.top))
            .and(self.bottom.observation_relation(&other.bottom))
            .and(self.two_sided.observation_relation(&other.two_sided))
    }
}
impl ObservationEq for crate::entities::layout_types::PageDimension {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Auto => match other {
                Self::Auto => ObservationRelation::Same,
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
impl ObservationEq for crate::entities::page_running::PageMarginal {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Auto => match other {
                Self::Auto => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::None => match other {
                Self::None => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Content(a0) => match other {
                Self::Content(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::page_canvas::PageFill {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Auto => match other {
                Self::Auto => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::None => match other {
                Self::None => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Paint(a0) => match other {
                Self::Paint(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::elements::columns::ColumnsElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { count: _, gutter: _, body: _, page_columns: _ } = self;
        ObservationRelation::Same
            .and(self.count.observation_relation(&other.count))
            .and(self.gutter.observation_relation(&other.gutter))
            .and(self.body.observation_relation(&other.body))
            .and(self.page_columns.observation_relation(&other.page_columns))
    }
}
impl ObservationEq for crate::entities::elements::pagebreak::PagebreakElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { weak: _, weak_explicit: _, to: _ } = self;
        ObservationRelation::Same
            .and(self.weak.observation_relation(&other.weak))
            .and(self.weak_explicit.observation_relation(&other.weak_explicit))
            .and(self.to.observation_relation(&other.to))
    }
}
impl ObservationEq for crate::entities::parity::Parity {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Even => match other {
                Self::Even => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Odd => match other {
                Self::Odd => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::document_info::DocumentInfo {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { title: _, author: _, keywords: _ } = self;
        ObservationRelation::Same
            .and(self.title.observation_relation(&other.title))
            .and(self.author.observation_relation(&other.author))
            .and(self.keywords.observation_relation(&other.keywords))
    }
}
impl ObservationEq for crate::entities::page_geometry::Paper {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        self.name()
            .observation_relation(other.name())
            .and(self.width_pt().observation_relation(&other.width_pt()))
            .and(self.height_pt().observation_relation(&other.height_pt()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::page_geometry::PageBinding;

    #[test]
    fn distingue_binding_de_pagina() {
        assert_eq!(
            PageBinding::Left.observation_relation(&PageBinding::Left),
            ObservationRelation::Same
        );
        assert_eq!(
            PageBinding::Left.observation_relation(&PageBinding::Right),
            ObservationRelation::Different
        );
    }
}
