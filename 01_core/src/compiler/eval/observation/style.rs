//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/observation/style.md
//! @prompt-hash 0b1ddc18
//! @layer L1
//! @updated 2026-09-26
//!
//! Relação observacional causal — família `style`.

use super::*;

impl ObservationEq for crate::entities::style_chain::StyleDelta {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self {
            bold: _,
            bold_from_strong: _,
            italic: _,
            italic_from_emph: _,
            size: _,
            fill: _,
            heading_level: _,
            weight: _,
            tracking: _,
            leading: _,
            top_edge: _,
            bottom_edge: _,
            lang: _,
            font: _,
            subscript: _,
            superscript: _,
            highlight: _,
            highlight_radius: _,
            highlight_extent: _,
            subscript_size: _,
            superscript_size: _,
            custom: _,
        } = self;
        ObservationRelation::Same
            .and(self.bold.observation_relation(&other.bold))
            .and(self.bold_from_strong.observation_relation(&other.bold_from_strong))
            .and(self.italic.observation_relation(&other.italic))
            .and(self.italic_from_emph.observation_relation(&other.italic_from_emph))
            .and(self.size.observation_relation(&other.size))
            .and(self.fill.observation_relation(&other.fill))
            .and(self.heading_level.observation_relation(&other.heading_level))
            .and(self.weight.observation_relation(&other.weight))
            .and(self.tracking.observation_relation(&other.tracking))
            .and(self.leading.observation_relation(&other.leading))
            .and(self.top_edge.observation_relation(&other.top_edge))
            .and(self.bottom_edge.observation_relation(&other.bottom_edge))
            .and(self.lang.observation_relation(&other.lang))
            .and(self.font.observation_relation(&other.font))
            .and(self.subscript.observation_relation(&other.subscript))
            .and(self.superscript.observation_relation(&other.superscript))
            .and(self.highlight.observation_relation(&other.highlight))
            .and(self.highlight_radius.observation_relation(&other.highlight_radius))
            .and(self.highlight_extent.observation_relation(&other.highlight_extent))
            .and(self.subscript_size.observation_relation(&other.subscript_size))
            .and(self.superscript_size.observation_relation(&other.superscript_size))
            .and(self.custom.observation_relation(&other.custom))
    }
}
impl ObservationEq for crate::entities::font_list::FontFamily {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self {
            name: _,
            variants: _,
            variant: _,
            weight: _,
            style: _,
            covers: _,
        } = self;
        ObservationRelation::Same
            .and(self.name.observation_relation(&other.name))
            .and(self.variants.observation_relation(&other.variants))
            .and(self.variant.observation_relation(&other.variant))
            .and(self.weight.observation_relation(&other.weight))
            .and(self.style.observation_relation(&other.style))
            .and(self.covers.observation_relation(&other.covers))
    }
}
impl ObservationEq for crate::entities::font_list::FontNamePattern {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Literal(a0) => match other {
                Self::Literal(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Regex(a0) => match other {
                Self::Regex(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::font_list::Covers {
    fn observation_relation(&self, _other: &Self) -> ObservationRelation {
        match *self {}
    }
}
impl ObservationEq for crate::entities::dir::Dir {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::LTR => match other {
                Self::LTR => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::RTL => match other {
                Self::RTL => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::TTB => match other {
                Self::TTB => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::BTT => match other {
                Self::BTT => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::citation_style::CitationStyle {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::AuthorDate => match other {
                Self::AuthorDate => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Numeric => match other {
                Self::Numeric => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Alphabetic => match other {
                Self::Alphabetic => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::style::Styles {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        self.delta().observation_relation(other.delta())
    }
}
impl ObservationEq for crate::entities::font_list::FontList {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        self.as_slice().observation_relation(other.as_slice())
    }
}
impl ObservationEq for crate::entities::lang::Lang {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        self.as_str().observation_relation(other.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::dir::Dir;

    #[test]
    fn distingue_direcao_tipografica() {
        assert_eq!(Dir::LTR.observation_relation(&Dir::LTR), ObservationRelation::Same);
        assert_eq!(
            Dir::LTR.observation_relation(&Dir::RTL),
            ObservationRelation::Different
        );
    }
}
