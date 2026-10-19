//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/eval/observation/elements.md
//! @prompt-hash 976880dd
//! @layer L1
//! @updated 2026-10-07
//!
//! Relação observacional causal — família `elements`.

use super::*;

impl ObservationEq for crate::entities::elements::align::AlignElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { alignment: _, body: _ } = self;
        ObservationRelation::Same
            .and(self.alignment.observation_relation(&other.alignment))
            .and(self.body.observation_relation(&other.body))
    }
}
impl ObservationEq for crate::entities::elements::underline::UnderlineElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { body: _, stroke: _, offset: _, extent: _ } = self;
        ObservationRelation::Same
            .and(self.body.observation_relation(&other.body))
            .and(self.stroke.observation_relation(&other.stroke))
            .and(self.offset.observation_relation(&other.offset))
            .and(self.extent.observation_relation(&other.extent))
    }
}
impl ObservationEq for crate::entities::elements::v_space::VSpaceElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { amount: _, weak: _, weak_explicit: _ } = self;
        ObservationRelation::Same
            .and(self.amount.observation_relation(&other.amount))
            .and(self.weak.observation_relation(&other.weak))
            .and(self.weak_explicit.observation_relation(&other.weak_explicit))
    }
}
impl ObservationEq for crate::entities::elements::enum_item::EnumItemElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self {
            number: _,
            body: _,
            numbering: _,
            indent: _,
            body_indent: _,
            tight: _,
        } = self;
        ObservationRelation::Same
            .and(self.number.observation_relation(&other.number))
            .and(self.body.observation_relation(&other.body))
            .and(self.numbering.observation_relation(&other.numbering))
            .and(self.indent.observation_relation(&other.indent))
            .and(self.body_indent.observation_relation(&other.body_indent))
            .and(self.tight.observation_relation(&other.tight))
    }
}
impl ObservationEq for crate::entities::elements::table_footer::TableFooterElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { body: _, repeat: _ } = self;
        ObservationRelation::Same
            .and(self.body.observation_relation(&other.body))
            .and(self.repeat.observation_relation(&other.repeat))
    }
}
impl ObservationEq for crate::entities::elements::footnote::FootnoteElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { body: _, numbering: _ } = self;
        ObservationRelation::Same
            .and(self.body.observation_relation(&other.body))
            .and(self.numbering.observation_relation(&other.numbering))
    }
}
// P1473 — `footnote.entry(...)`, escopo reduzido (construção sem realização).
impl ObservationEq for crate::entities::elements::footnote_entry::FootnoteEntryElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self {
            note: _,
            separator: _,
            clearance: _,
            gap: _,
            indent: _,
        } = self;
        ObservationRelation::Same
            .and(self.note.observation_relation(&other.note))
            .and(self.separator.observation_relation(&other.separator))
            .and(self.clearance.observation_relation(&other.clearance))
            .and(self.gap.observation_relation(&other.gap))
            .and(self.indent.observation_relation(&other.indent))
    }
}
impl ObservationEq for crate::entities::elements::shape::ShapeElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self {
            kind: _,
            width: _,
            height: _,
            fill: _,
            stroke: _,
            fill_rule: _,
        } = self;
        ObservationRelation::Same
            .and(self.kind.observation_relation(&other.kind))
            .and(self.width.observation_relation(&other.width))
            .and(self.height.observation_relation(&other.height))
            .and(self.fill.observation_relation(&other.fill))
            .and(self.stroke.observation_relation(&other.stroke))
            .and(self.fill_rule.observation_relation(&other.fill_rule))
    }
}
impl ObservationEq for crate::entities::elements::grid::GridElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self {
            columns: _,
            rows: _,
            cells: _,
            hlines: _,
            vlines: _,
            gutter: _,
            align: _,
            inset: _,
            header: _,
            footer: _,
            stroke: _,
            fill: _,
        } = self;
        ObservationRelation::Same
            .and(self.columns.observation_relation(&other.columns))
            .and(self.rows.observation_relation(&other.rows))
            .and(self.cells.observation_relation(&other.cells))
            .and(self.hlines.observation_relation(&other.hlines))
            .and(self.vlines.observation_relation(&other.vlines))
            .and(self.gutter.observation_relation(&other.gutter))
            .and(self.align.observation_relation(&other.align))
            .and(self.inset.observation_relation(&other.inset))
            .and(self.header.observation_relation(&other.header))
            .and(self.footer.observation_relation(&other.footer))
            .and(self.stroke.observation_relation(&other.stroke))
            .and(self.fill.observation_relation(&other.fill))
    }
}
impl ObservationEq for crate::entities::elements::grid_hline::GridHLineElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { start: _, end: _, row: _, stroke: _, position: _ } = self;
        ObservationRelation::Same
            .and(self.start.observation_relation(&other.start))
            .and(self.end.observation_relation(&other.end))
            .and(self.row.observation_relation(&other.row))
            .and(self.stroke.observation_relation(&other.stroke))
            .and(self.position.observation_relation(&other.position))
    }
}
impl ObservationEq for crate::entities::elements::grid_vline::GridVLineElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { start: _, end: _, col: _, stroke: _, position: _ } = self;
        ObservationRelation::Same
            .and(self.start.observation_relation(&other.start))
            .and(self.end.observation_relation(&other.end))
            .and(self.col.observation_relation(&other.col))
            .and(self.stroke.observation_relation(&other.stroke))
            .and(self.position.observation_relation(&other.position))
    }
}
impl ObservationEq for crate::entities::elements::counter_update::CounterUpdateElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { key: _, action: _ } = self;
        ObservationRelation::Same
            .and(self.key.observation_relation(&other.key))
            .and(self.action.observation_relation(&other.action))
    }
}
impl ObservationEq for crate::entities::elements::label::LabelElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { name: _, body: _, auto: _ } = self;
        ObservationRelation::Same
            .and(self.name.observation_relation(&other.name))
            .and(self.body.observation_relation(&other.body))
            .and(self.auto.observation_relation(&other.auto))
    }
}
impl ObservationEq for crate::entities::elements::pdf_artifact::PdfArtifactElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { kind: _, body: _ } = self;
        ObservationRelation::Same
            .and(self.kind.observation_relation(&other.kind))
            .and(self.body.observation_relation(&other.body))
    }
}
impl ObservationEq for crate::entities::elements::pdf_artifact::ArtifactKind {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Header => match other {
                Self::Header => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Footer => match other {
                Self::Footer => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Watermark => match other {
                Self::Watermark => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::PageNumber => match other {
                Self::PageNumber => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::LineNumber => match other {
                Self::LineNumber => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Redaction => match other {
                Self::Redaction => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Bates => match other {
                Self::Bates => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Page => match other {
                Self::Page => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::PaginationOther => match other {
                Self::PaginationOther => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Layout => match other {
                Self::Layout => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Background => match other {
                Self::Background => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Other => match other {
                Self::Other => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::elements::smartquote::SmartQuoteElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { double: _, enabled: _, alternative: _, quotes: _ } = self;
        ObservationRelation::Same
            .and(self.double.observation_relation(&other.double))
            .and(self.enabled.observation_relation(&other.enabled))
            .and(self.alternative.observation_relation(&other.alternative))
            .and(self.quotes.observation_relation(&other.quotes))
    }
}
impl ObservationEq for crate::entities::elements::smartquote::SmartQuoteQuotes {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Auto => match other {
                Self::Auto => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Custom(a0) => match other {
                Self::Custom(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::elements::smartquote::SmartQuoteOverrides {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { single: _, double: _ } = self;
        ObservationRelation::Same
            .and(self.single.observation_relation(&other.single))
            .and(self.double.observation_relation(&other.double))
    }
}
impl ObservationEq for crate::entities::elements::smartquote::SmartQuotePair {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { open: _, close: _ } = self;
        ObservationRelation::Same
            .and(self.open.observation_relation(&other.open))
            .and(self.close.observation_relation(&other.close))
    }
}
impl ObservationEq for crate::entities::elements::r#ref::RefElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { name: _, supplement: _, form: _ } = self;
        ObservationRelation::Same
            .and(self.name.observation_relation(&other.name))
            .and(self.supplement.observation_relation(&other.supplement))
            .and(self.form.observation_relation(&other.form))
    }
}
impl ObservationEq for crate::entities::elements::r#ref::RefForm {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Normal => match other {
                Self::Normal => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Page => match other {
                Self::Page => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::elements::strong::StrongElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { body: _ } = self;
        ObservationRelation::Same.and(self.body.observation_relation(&other.body))
    }
}
impl ObservationEq for crate::entities::elements::raw::RawElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { text: _, lang: _, block: _, set_fields: _ } = self;
        ObservationRelation::Same
            .and(self.text.observation_relation(&other.text))
            .and(self.lang.observation_relation(&other.lang))
            .and(self.block.observation_relation(&other.block))
            .and(self.set_fields.observation_relation(&other.set_fields))
    }
}
impl ObservationEq for crate::entities::elements::bibliography::BibliographyElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { entries: _, path: _, title: _, style: _, locale: _ } = self;
        ObservationRelation::Same
            .and(self.entries.observation_relation(&other.entries))
            .and(self.path.observation_relation(&other.path))
            .and(self.title.observation_relation(&other.title))
            .and(self.style.observation_relation(&other.style))
            .and(self.locale.observation_relation(&other.locale))
    }
}
impl ObservationEq for crate::entities::elements::grid_header::GridHeaderElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { body: _, repeat: _ } = self;
        ObservationRelation::Same
            .and(self.body.observation_relation(&other.body))
            .and(self.repeat.observation_relation(&other.repeat))
    }
}
impl ObservationEq for crate::entities::elements::pdf_attach::PdfAttachElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self {
            path: _,
            data: _,
            relationship: _,
            mime_type: _,
            description: _,
        } = self;
        ObservationRelation::Same
            .and(self.path.observation_relation(&other.path))
            .and(self.data.observation_relation(&other.data))
            .and(self.relationship.observation_relation(&other.relationship))
            .and(self.mime_type.observation_relation(&other.mime_type))
            .and(self.description.observation_relation(&other.description))
    }
}
impl ObservationEq for crate::entities::elements::pdf_attach::AttachedFileRelationship {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Source => match other {
                Self::Source => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Data => match other {
                Self::Data => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Alternative => match other {
                Self::Alternative => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Supplement => match other {
                Self::Supplement => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::elements::table::TableElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self {
            source_span: _,
            rows_explicit: _,
            resolved: _,

            columns: _,
            rows: _,
            children: _,
            hlines: _,
            vlines: _,
            header: _,
            footer: _,
            stroke: _,
            fill: _,
            caption: _,
            summary: _,
            inset: _,
            align: _,
        } = self;
        ObservationRelation::Same
            .and(self.source_span.observation_relation(&other.source_span))
            .and(self.rows_explicit.observation_relation(&other.rows_explicit))
            .and(self.resolved.observation_relation(&other.resolved))
            .and(self.columns.observation_relation(&other.columns))
            .and(self.rows.observation_relation(&other.rows))
            .and(self.children.observation_relation(&other.children))
            .and(self.hlines.observation_relation(&other.hlines))
            .and(self.vlines.observation_relation(&other.vlines))
            .and(self.header.observation_relation(&other.header))
            .and(self.footer.observation_relation(&other.footer))
            .and(self.stroke.observation_relation(&other.stroke))
            .and(self.fill.observation_relation(&other.fill))
            .and(self.caption.observation_relation(&other.caption))
            .and(self.summary.observation_relation(&other.summary))
            .and(self.inset.observation_relation(&other.inset))
            .and(self.align.observation_relation(&other.align))
    }
}
impl ObservationEq for crate::entities::elements::table_vline::TableVLineElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { start: _, end: _, col: _, stroke: _, position: _ } = self;
        ObservationRelation::Same
            .and(self.start.observation_relation(&other.start))
            .and(self.end.observation_relation(&other.end))
            .and(self.col.observation_relation(&other.col))
            .and(self.stroke.observation_relation(&other.stroke))
            .and(self.position.observation_relation(&other.position))
    }
}
impl ObservationEq for crate::entities::elements::table_hline::TableHLineElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { start: _, end: _, row: _, stroke: _, position: _ } = self;
        ObservationRelation::Same
            .and(self.start.observation_relation(&other.start))
            .and(self.end.observation_relation(&other.end))
            .and(self.row.observation_relation(&other.row))
            .and(self.stroke.observation_relation(&other.stroke))
            .and(self.position.observation_relation(&other.position))
    }
}
impl ObservationEq for crate::entities::elements::h_space::HSpaceElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { amount: _, weak: _, weak_explicit: _ } = self;
        ObservationRelation::Same
            .and(self.amount.observation_relation(&other.amount))
            .and(self.weak.observation_relation(&other.weak))
            .and(self.weak_explicit.observation_relation(&other.weak_explicit))
    }
}
impl ObservationEq for crate::entities::elements::h_space::Spacing {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Absolute(a0) => match other {
                Self::Absolute(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Fractional(a0) => match other {
                Self::Fractional(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::elements::context_block::ContextBlockElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { id: _, closure: _ } = self;
        ObservationRelation::Same
            .and(self.id.observation_relation(&other.id))
            .and(self.closure.observation_relation(&other.closure))
    }
}
impl ObservationEq for crate::entities::elements::image::ImageElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self {
            path: _,
            alt: _,
            data: _,
            width: _,
            height: _,
            fit: _,
        } = self;
        ObservationRelation::Same
            .and(self.path.observation_relation(&other.path))
            .and(self.alt.observation_relation(&other.alt))
            .and(self.data.observation_relation(&other.data))
            .and(self.width.observation_relation(&other.width))
            .and(self.height.observation_relation(&other.height))
            .and(self.fit.observation_relation(&other.fit))
    }
}
impl ObservationEq for crate::entities::elements::block::BlockElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self {
            body: _,
            width: _,
            height: _,
            inset: _,
            breakable: _,
            outset: _,
            radius: _,
            clip: _,
            fill: _,
            stroke: _,
            spacing: _,
            above: _,
            below: _,
            sticky: _,
        } = self;
        ObservationRelation::Same
            .and(self.body.observation_relation(&other.body))
            .and(self.width.observation_relation(&other.width))
            .and(self.height.observation_relation(&other.height))
            .and(self.inset.observation_relation(&other.inset))
            .and(self.breakable.observation_relation(&other.breakable))
            .and(self.outset.observation_relation(&other.outset))
            .and(self.radius.observation_relation(&other.radius))
            .and(self.clip.observation_relation(&other.clip))
            .and(self.fill.observation_relation(&other.fill))
            .and(self.stroke.observation_relation(&other.stroke))
            .and(self.spacing.observation_relation(&other.spacing))
            .and(self.above.observation_relation(&other.above))
            .and(self.below.observation_relation(&other.below))
            .and(self.sticky.observation_relation(&other.sticky))
    }
}
impl ObservationEq for crate::entities::elements::block::BlockHeight {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        use crate::entities::elements::block::BlockHeight;
        match (self, other) {
            (BlockHeight::Relative(a), BlockHeight::Relative(b)) => {
                a.observation_relation(b)
            }
            (BlockHeight::Fractional(a), BlockHeight::Fractional(b)) => {
                a.observation_relation(b)
            }
            (BlockHeight::Relative(_), BlockHeight::Fractional(_))
            | (BlockHeight::Fractional(_), BlockHeight::Relative(_)) => {
                ObservationRelation::Different
            }
        }
    }
}
impl ObservationEq for crate::entities::elements::repeat::RepeatElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { body: _, gap: _, justify: _ } = self;
        ObservationRelation::Same
            .and(self.body.observation_relation(&other.body))
            .and(self.gap.observation_relation(&other.gap))
            .and(self.justify.observation_relation(&other.justify))
    }
}
impl ObservationEq for crate::entities::elements::state_update::StateUpdateElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { key: _, update: _ } = self;
        ObservationRelation::Same
            .and(self.key.observation_relation(&other.key))
            .and(self.update.observation_relation(&other.update))
    }
}
impl ObservationEq for crate::entities::elements::link::LinkElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { url: _, body: _ } = self;
        ObservationRelation::Same
            .and(self.url.observation_relation(&other.url))
            .and(self.body.observation_relation(&other.body))
    }
}
impl ObservationEq for crate::entities::elements::place::PlaceElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self {
            alignment: _,
            dx: _,
            dy: _,
            scope: _,
            float: _,
            clearance: _,
            body: _,
        } = self;
        ObservationRelation::Same
            .and(self.alignment.observation_relation(&other.alignment))
            .and(self.dx.observation_relation(&other.dx))
            .and(self.dy.observation_relation(&other.dy))
            .and(self.scope.observation_relation(&other.scope))
            .and(self.float.observation_relation(&other.float))
            .and(self.clearance.observation_relation(&other.clearance))
            .and(self.body.observation_relation(&other.body))
    }
}
impl ObservationEq
    for crate::entities::elements::counter_display_callback::CounterDisplayCallbackElem
{
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { key: _, callback: _ } = self;
        ObservationRelation::Same
            .and(self.key.observation_relation(&other.key))
            .and(self.callback.observation_relation(&other.callback))
    }
}
impl ObservationEq for crate::entities::elements::table_header::TableHeaderElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { body: _, repeat: _ } = self;
        ObservationRelation::Same
            .and(self.body.observation_relation(&other.body))
            .and(self.repeat.observation_relation(&other.repeat))
    }
}
impl ObservationEq for crate::entities::elements::title::TitleElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { body: _ } = self;
        ObservationRelation::Same.and(self.body.observation_relation(&other.body))
    }
}
impl ObservationEq for crate::entities::elements::emph::EmphElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { body: _ } = self;
        ObservationRelation::Same.and(self.body.observation_relation(&other.body))
    }
}
impl ObservationEq for crate::entities::elements::quote::QuoteElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { body: _, attribution: _, block: _, quotes: _ } = self;
        ObservationRelation::Same
            .and(self.body.observation_relation(&other.body))
            .and(self.attribution.observation_relation(&other.attribution))
            .and(self.block.observation_relation(&other.block))
            .and(self.quotes.observation_relation(&other.quotes))
    }
}
impl ObservationEq for crate::entities::elements::flush::FlushElem {
    fn observation_relation(&self, _: &Self) -> ObservationRelation {
        let Self = self;
        ObservationRelation::Same
    }
}
impl ObservationEq for crate::entities::elements::stack::StackElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { children: _, dir: _, spacing: _ } = self;
        ObservationRelation::Same
            .and(self.children.observation_relation(&other.children))
            .and(self.dir.observation_relation(&other.dir))
            .and(self.spacing.observation_relation(&other.spacing))
    }
}
impl ObservationEq for crate::entities::elements::outline::OutlineElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        ObservationRelation::Same
            .and(self.title.observation_relation(&other.title))
            .and(self.depth.observation_relation(&other.depth))
            .and(self.indent.observation_relation(&other.indent))
            .and(self.target.observation_relation(&other.target))
            .and(self.title_was_set().observation_relation(&other.title_was_set()))
    }
}
impl ObservationEq for crate::entities::elements::outline::OutlineTarget {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Headings => match other {
                Self::Headings => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Figures => match other {
                Self::Figures => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Tables => match other {
                Self::Tables => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::elements::outline::OutlineIndent {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Auto => match other {
                Self::Auto => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Bool(a0) => match other {
                Self::Bool(b0) => {
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
            Self::Function(a0) => match other {
                Self::Function(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::elements::transform::TransformElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { operation: _, body: _ } = self;
        // All operation payloads are pure scalars. NaN remains conservative.
        (if self.operation == other.operation {
            ObservationRelation::Same
        } else {
            ObservationRelation::Different
        })
        .and(self.body.observation_relation(&other.body))
    }
}
impl ObservationEq for crate::entities::elements::list_item::ListItemElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self {
            body: _,
            marker: _,
            marker_align: _,
            indent: _,
            body_indent: _,
            tight: _,
        } = self;
        ObservationRelation::Same
            .and(self.body.observation_relation(&other.body))
            .and(self.marker.observation_relation(&other.marker))
            .and(self.marker_align.observation_relation(&other.marker_align))
            .and(self.indent.observation_relation(&other.indent))
            .and(self.body_indent.observation_relation(&other.body_indent))
            .and(self.tight.observation_relation(&other.tight))
    }
}
impl ObservationEq for crate::entities::elements::boxed::BoxedElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self {
            body: _,
            width: _,
            width_fraction: _,
            height: _,
            inset: _,
            baseline: _,
            outset: _,
            radius: _,
            clip: _,
            fill: _,
            stroke: _,
        } = self;
        ObservationRelation::Same
            .and(self.body.observation_relation(&other.body))
            .and(self.width.observation_relation(&other.width))
            .and(self.width_fraction.observation_relation(&other.width_fraction))
            .and(self.height.observation_relation(&other.height))
            .and(self.inset.observation_relation(&other.inset))
            .and(self.baseline.observation_relation(&other.baseline))
            .and(self.outset.observation_relation(&other.outset))
            .and(self.radius.observation_relation(&other.radius))
            .and(self.clip.observation_relation(&other.clip))
            .and(self.fill.observation_relation(&other.fill))
            .and(self.stroke.observation_relation(&other.stroke))
    }
}
impl ObservationEq for crate::entities::elements::cite::CiteElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { key: _, supplement: _, form: _, style: _ } = self;
        ObservationRelation::Same
            .and(self.key.observation_relation(&other.key))
            .and(self.supplement.observation_relation(&other.supplement))
            .and(self.form.observation_relation(&other.form))
            .and(self.style.observation_relation(&other.style))
    }
}
impl ObservationEq for crate::entities::elements::hide::HideElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { body: _ } = self;
        ObservationRelation::Same.and(self.body.observation_relation(&other.body))
    }
}
impl ObservationEq for crate::entities::elements::grid_cell::GridCellElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self {
            body: _,
            x: _,
            y: _,
            colspan: _,
            rowspan: _,
            stroke: _,
            fill: _,
            align: _,
            inset: _,
            breakable: _,
        } = self;
        ObservationRelation::Same
            .and(self.body.observation_relation(&other.body))
            .and(self.x.observation_relation(&other.x))
            .and(self.y.observation_relation(&other.y))
            .and(self.colspan.observation_relation(&other.colspan))
            .and(self.rowspan.observation_relation(&other.rowspan))
            .and(self.stroke.observation_relation(&other.stroke))
            .and(self.fill.observation_relation(&other.fill))
            .and(self.align.observation_relation(&other.align))
            .and(self.inset.observation_relation(&other.inset))
            .and(self.breakable.observation_relation(&other.breakable))
    }
}
impl ObservationEq for crate::entities::elements::grid_footer::GridFooterElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { body: _, repeat: _ } = self;
        ObservationRelation::Same
            .and(self.body.observation_relation(&other.body))
            .and(self.repeat.observation_relation(&other.repeat))
    }
}
impl ObservationEq for crate::entities::elements::metadata::MetadataElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { value: _ } = self;
        ObservationRelation::Same.and(self.value.observation_relation(&other.value))
    }
}
impl ObservationEq for crate::entities::elements::terms::TermsElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { items: _ } = self;
        ObservationRelation::Same.and(self.items.observation_relation(&other.items))
    }
}
impl ObservationEq for crate::entities::elements::state_display::StateDisplayElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { key: _, callback: _ } = self;
        ObservationRelation::Same
            .and(self.key.observation_relation(&other.key))
            .and(self.callback.observation_relation(&other.callback))
    }
}
impl ObservationEq for crate::entities::elements::state::StateElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { key: _, init: _ } = self;
        ObservationRelation::Same
            .and(self.key.observation_relation(&other.key))
            .and(self.init.observation_relation(&other.init))
    }
}
impl ObservationEq for crate::entities::elements::figure::FigureElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { body: _, caption: _, kind: _ } = self;
        ObservationRelation::Same
            .and(self.body.observation_relation(&other.body))
            .and(self.caption.observation_relation(&other.caption))
            .and(self.kind.observation_relation(&other.kind))
    }
}
// P1473 — `figure.caption(...)`, escopo reduzido (construção sem síntese).
impl ObservationEq for crate::entities::elements::figure_caption::FigureCaptionElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { position: _, separator: _, body: _ } = self;
        ObservationRelation::Same
            .and(self.position.observation_relation(&other.position))
            .and(self.separator.observation_relation(&other.separator))
            .and(self.body.observation_relation(&other.body))
    }
}
impl ObservationEq for crate::entities::elements::curve::CurveElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { segments: _ } = self;
        ObservationRelation::Same.and(self.segments.observation_relation(&other.segments))
    }
}
impl ObservationEq for crate::entities::elements::curve::CurveSegment {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Move(a0) => match other {
                Self::Move(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Line(a0) => match other {
                Self::Line(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Cubic(a0, a1, a2) => match other {
                Self::Cubic(b0, b1, b2) => ObservationRelation::Same
                    .and(a0.observation_relation(b0))
                    .and(a1.observation_relation(b1))
                    .and(a2.observation_relation(b2)),
                _ => ObservationRelation::Different,
            },
            Self::Quad(a0, a1) => match other {
                Self::Quad(b0, b1) => ObservationRelation::Same
                    .and(a0.observation_relation(b0))
                    .and(a1.observation_relation(b1)),
                _ => ObservationRelation::Different,
            },
            Self::Close(a0) => match other {
                Self::Close(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::elements::curve::CurvePoint {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { x: _, y: _ } = self;
        ObservationRelation::Same
            .and(self.x.observation_relation(&other.x))
            .and(self.y.observation_relation(&other.y))
    }
}
impl ObservationEq for crate::entities::elements::curve::CloseMode {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Smooth => match other {
                Self::Smooth => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Straight => match other {
                Self::Straight => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::elements::strike::StrikeElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { body: _, stroke: _, offset: _, extent: _ } = self;
        ObservationRelation::Same
            .and(self.body.observation_relation(&other.body))
            .and(self.stroke.observation_relation(&other.stroke))
            .and(self.offset.observation_relation(&other.offset))
            .and(self.extent.observation_relation(&other.extent))
    }
}
impl ObservationEq for crate::entities::elements::divider::DividerElem {
    fn observation_relation(&self, _: &Self) -> ObservationRelation {
        let Self = self;
        ObservationRelation::Same
    }
}
impl ObservationEq for crate::entities::elements::linebreak::LinebreakElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { justify: _, justify_explicit: _ } = self;
        ObservationRelation::Same
            .and(self.justify.observation_relation(&other.justify))
            .and(self.justify_explicit.observation_relation(&other.justify_explicit))
    }
}
impl ObservationEq for crate::entities::elements::pad::PadElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { body: _, sides: _ } = self;
        ObservationRelation::Same
            .and(self.body.observation_relation(&other.body))
            .and(self.sides.observation_relation(&other.sides))
    }
}
impl ObservationEq for crate::entities::elements::heading::HeadingElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self {
            level: _,
            body: _,
            outlined: _,
            bookmarked: _,
            set_fields: _,
        } = self;
        ObservationRelation::Same
            .and(self.level.observation_relation(&other.level))
            .and(self.body.observation_relation(&other.body))
            .and(self.outlined.observation_relation(&other.outlined))
            .and(self.bookmarked.observation_relation(&other.bookmarked))
            .and(self.set_fields.observation_relation(&other.set_fields))
    }
}
impl ObservationEq for crate::entities::elements::counter_display::CounterDisplayElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { kind: _ } = self;
        ObservationRelation::Same.and(self.kind.observation_relation(&other.kind))
    }
}
impl ObservationEq for crate::entities::elements::table_cell::TableCellElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self {
            source_span: _,
            resolved: _,
            standalone: _,

            body: _,
            x: _,
            y: _,
            colspan: _,
            rowspan: _,
            stroke: _,
            fill: _,
            align: _,
            inset: _,
            breakable: _,
            kind: _,
            presence: _,
        } = self;
        ObservationRelation::Same
            .and(self.source_span.observation_relation(&other.source_span))
            .and(self.resolved.observation_relation(&other.resolved))
            .and(self.standalone.observation_relation(&other.standalone))
            .and(self.body.observation_relation(&other.body))
            .and(self.x.observation_relation(&other.x))
            .and(self.y.observation_relation(&other.y))
            .and(self.colspan.observation_relation(&other.colspan))
            .and(self.rowspan.observation_relation(&other.rowspan))
            .and(self.stroke.observation_relation(&other.stroke))
            .and(self.fill.observation_relation(&other.fill))
            .and(self.align.observation_relation(&other.align))
            .and(self.inset.observation_relation(&other.inset))
            .and(self.breakable.observation_relation(&other.breakable))
            .and(self.kind.observation_relation(&other.kind))
            .and(self.presence.observation_relation(&other.presence))
    }
}
impl ObservationEq for crate::entities::elements::table_cell::TableCellKind {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Auto => match other {
                Self::Auto => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Header { level: a0, scope: a1 } => match other {
                Self::Header { level: b0, scope: b1 } => ObservationRelation::Same
                    .and(a0.observation_relation(b0))
                    .and(a1.observation_relation(b1)),
                _ => ObservationRelation::Different,
            },
            Self::Data => match other {
                Self::Data => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::elements::table_cell::TableHeaderScope {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Both => match other {
                Self::Both => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Column => match other {
                Self::Column => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Row => match other {
                Self::Row => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::elements::overline::OverlineElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { body: _, stroke: _, offset: _, extent: _ } = self;
        ObservationRelation::Same
            .and(self.body.observation_relation(&other.body))
            .and(self.stroke.observation_relation(&other.stroke))
            .and(self.offset.observation_relation(&other.offset))
            .and(self.extent.observation_relation(&other.extent))
    }
}
impl ObservationEq for crate::entities::elements::term_item::TermItemElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { term: _, description: _ } = self;
        ObservationRelation::Same
            .and(self.term.observation_relation(&other.term))
            .and(self.description.observation_relation(&other.description))
    }
}
impl ObservationEq for crate::entities::elements::colbreak::ColbreakElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { weak: _, weak_explicit: _ } = self;
        ObservationRelation::Same
            .and(self.weak.observation_relation(&other.weak))
            .and(self.weak_explicit.observation_relation(&other.weak_explicit))
    }
}
impl ObservationEq for crate::entities::html::HtmlElem {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { tag: _, attrs: _, body: _ } = self;
        self.tag
            .observation_relation(&other.tag)
            .and(self.attrs.observation_relation(&other.attrs))
            .and(self.body.observation_relation(&other.body))
    }
}
impl ObservationEq for crate::entities::html::HtmlBody {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Unset => match other {
                Self::Unset => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::None => match other {
                Self::None => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Content(a) => match other {
                Self::Content(b) => a.observation_relation(b),
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for dyn crate::entities::elements::dynamic::DynElement {
    fn observation_relation(&self, _: &Self) -> ObservationRelation {
        // The extension trait does not guarantee absence of interior mutation.
        // Neither shared allocation, dyn_eq nor a Debug string proves stability.
        ObservationRelation::Unproven
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::content::Content;
    use crate::entities::elements::strong::StrongElem;

    #[test]
    fn distingue_body_de_elemento() {
        let a = StrongElem::new(Content::text("a"));
        let b = StrongElem::new(Content::text("b"));
        assert_eq!(a.observation_relation(&a), ObservationRelation::Same);
        assert_eq!(a.observation_relation(&b), ObservationRelation::Different);
    }
}

impl ObservationEq for crate::entities::elements::table_cell::CellFieldPresence {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match (self, other) {
            (Self::Omitted, Self::Omitted)
            | (Self::Auto, Self::Auto)
            | (Self::Explicit, Self::Explicit) => ObservationRelation::Same,
            (Self::Omitted, Self::Auto | Self::Explicit)
            | (Self::Auto, Self::Omitted | Self::Explicit)
            | (Self::Explicit, Self::Omitted | Self::Auto) => {
                ObservationRelation::Different
            }
        }
    }
}
impl ObservationEq for crate::entities::elements::table_cell::TableCellPresence {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self {
            x: _,
            y: _,
            colspan: _,
            rowspan: _,
            fill: _,
            stroke: _,
            align: _,
            inset: _,
            breakable: _,
        } = self;
        ObservationRelation::Same
            .and(self.x.observation_relation(&other.x))
            .and(self.y.observation_relation(&other.y))
            .and(self.colspan.observation_relation(&other.colspan))
            .and(self.rowspan.observation_relation(&other.rowspan))
            .and(self.fill.observation_relation(&other.fill))
            .and(self.stroke.observation_relation(&other.stroke))
            .and(self.align.observation_relation(&other.align))
            .and(self.inset.observation_relation(&other.inset))
            .and(self.breakable.observation_relation(&other.breakable))
    }
}

#[cfg(test)]
mod cell_presence_observation_tests {
    use super::*;
    #[test]
    fn cell_presence_is_observed_even_when_body_and_render_values_match() {
        use crate::entities::content::Content;
        use crate::entities::elements::table_cell::CellFieldPresence;
        let Content::TableCell(original) =
            Content::table_cell(Content::text("a"), None, None, None, None)
        else {
            unreachable!()
        };
        let mut explicit = (*original).clone();
        explicit.presence.fill = CellFieldPresence::Explicit;
        assert_eq!(
            original.as_ref().observation_relation(&explicit),
            ObservationRelation::Different
        );
        assert_eq!(
            explicit.observation_relation(&explicit.clone()),
            ObservationRelation::Same
        );
        let mut automatic = explicit.clone();
        automatic.presence.fill = CellFieldPresence::Auto;
        assert_eq!(
            explicit.observation_relation(&automatic),
            ObservationRelation::Different
        );
    }
}

impl ObservationEq for crate::entities::elements::table::TableGridResolved {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self {
            cells: _,
            num_cols: _,
            num_rows: _,
            header_rows: _,
            footer_start: _,
            complete: _,
        } = self;
        ObservationRelation::Same
            .and(self.cells.observation_relation(&other.cells))
            .and(self.num_cols.observation_relation(&other.num_cols))
            .and(self.num_rows.observation_relation(&other.num_rows))
            .and(self.header_rows.observation_relation(&other.header_rows))
            .and(self.footer_start.observation_relation(&other.footer_start))
            .and(self.complete.observation_relation(&other.complete))
    }
}

impl ObservationEq for crate::entities::elements::table_cell::TableCellResolved {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self {
            x: _,
            y: _,
            colspan: _,
            rowspan: _,
            fill: _,
            stroke: _,
            align: _,
            inset: _,
            breakable: _,
            visual: _,
            stage: _,
            guards: _,
        } = self;
        ObservationRelation::Same
            .and(self.x.observation_relation(&other.x))
            .and(self.y.observation_relation(&other.y))
            .and(self.colspan.observation_relation(&other.colspan))
            .and(self.rowspan.observation_relation(&other.rowspan))
            .and(self.fill.observation_relation(&other.fill))
            .and(self.stroke.observation_relation(&other.stroke))
            .and(self.align.observation_relation(&other.align))
            .and(self.inset.observation_relation(&other.inset))
            .and(self.breakable.observation_relation(&other.breakable))
            .and(self.visual.observation_relation(&other.visual))
            .and(self.stage.observation_relation(&other.stage))
            .and(self.guards.observation_relation(&other.guards))
    }
}

impl ObservationEq for crate::entities::elements::table_cell::TableCellStage {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match (self, other) {
            (Self::Prepared, Self::Prepared)
            | (Self::RecipeApplied, Self::RecipeApplied)
            | (Self::Complete, Self::Complete) => ObservationRelation::Same,
            _ => ObservationRelation::Different,
        }
    }
}

impl ObservationEq
    for crate::entities::elements::table_cell::TableCellStandaloneResolved
{
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { visual: _, stage: _, guards: _, properties: _ } = self;
        self.visual
            .observation_relation(&other.visual)
            .and(self.properties.observation_relation(&other.properties))
            .and(self.stage.observation_relation(&other.stage))
            .and(self.guards.observation_relation(&other.guards))
    }
}

impl ObservationEq for crate::entities::elements::table_cell::TableCellProperties {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self {
            x: _,
            y: _,
            colspan: _,
            rowspan: _,
            fill: _,
            stroke: _,
            stroke_inherited: _,
            align: _,
            inset: _,
            breakable: _,
            presence: _,
        } = self;
        self.x
            .observation_relation(&other.x)
            .and(self.y.observation_relation(&other.y))
            .and(self.colspan.observation_relation(&other.colspan))
            .and(self.rowspan.observation_relation(&other.rowspan))
            .and(self.fill.observation_relation(&other.fill))
            .and(self.stroke.observation_relation(&other.stroke))
            .and(self.stroke_inherited.observation_relation(&other.stroke_inherited))
            .and(self.align.observation_relation(&other.align))
            .and(self.inset.observation_relation(&other.inset))
            .and(self.breakable.observation_relation(&other.breakable))
            .and(self.presence.observation_relation(&other.presence))
    }
}

#[cfg(test)]
mod raw_presence_tests {
    use super::*;
    use crate::entities::elements::raw::{RawElem, RAW_SET_BLOCK, RAW_SET_LANG};

    #[test]
    fn raw_presence_observation_distinguishes_masks_with_equal_values() {
        for a in [0, RAW_SET_BLOCK, RAW_SET_LANG, RAW_SET_BLOCK | RAW_SET_LANG] {
            for b in [0, RAW_SET_BLOCK, RAW_SET_LANG, RAW_SET_BLOCK | RAW_SET_LANG] {
                let left = RawElem {
                    text: "x".into(),
                    lang: None,
                    block: false,
                    set_fields: a,
                };
                let right = RawElem { set_fields: b, ..left.clone() };
                assert_eq!(
                    left.observation_relation(&right),
                    if a == b {
                        ObservationRelation::Same
                    } else {
                        ObservationRelation::Different
                    }
                );
            }
        }
    }
}
