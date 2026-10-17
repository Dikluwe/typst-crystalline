//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/observation/value.md
//! @prompt-hash 10e83b9f
//! @layer L1
//! @updated 2026-10-06
//!
//! Relação observacional causal — família `value`.

use super::*;

impl ObservationEq for crate::entities::value::Value {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::None => match other {
                Self::None => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Bool(a0) => match other {
                Self::Bool(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Int(a0) => match other {
                Self::Int(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Float(a0) => match other {
                Self::Float(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Str(a0) => match other {
                Self::Str(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Array(a0) => match other {
                Self::Array(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Dict(a0) => match other {
                Self::Dict(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Module(a0) => match other {
                Self::Module(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Datetime(a0) => match other {
                Self::Datetime(b0) => {
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
            Self::Content(a0) => match other {
                Self::Content(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::LocatedContent(a0, a1) => match other {
                Self::LocatedContent(b0, b1) => ObservationRelation::Same
                    .and(a0.observation_relation(b0))
                    .and(a1.observation_relation(b1)),
                _ => ObservationRelation::Different,
            },
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
            Self::Relative(a0) => match other {
                Self::Relative(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Ratio(a0) => match other {
                Self::Ratio(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Angle(a0) => match other {
                Self::Angle(b0) => {
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
            Self::Stroke(a0) => match other {
                Self::Stroke(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Fraction(a0) => match other {
                Self::Fraction(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Align(a0) => match other {
                Self::Align(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Location(a0) => match other {
                Self::Location(b0) => {
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
            Self::Regex(a0) => match other {
                Self::Regex(b0) => {
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
            Self::Bytes(a0) => match other {
                Self::Bytes(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Decimal(a0) => match other {
                Self::Decimal(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Duration(a0) => match other {
                Self::Duration(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Version(a0) => match other {
                Self::Version(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Selector(a0) => match other {
                Self::Selector(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Symbol(a0) => match other {
                Self::Symbol(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Args(a0) => match other {
                Self::Args(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::State(a0) => match other {
                Self::State(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Counter(a0) => match other {
                Self::Counter(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Label(a0) => match other {
                Self::Label(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Dir(a0) => match other {
                Self::Dir(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Path(a0) => match other {
                Self::Path(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Type(a0) => match other {
                Self::Type(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            // P1473 — instância de `color.spot(name, fallback)`.
            Self::SpotColorant(a0) => match other {
                Self::SpotColorant(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
        }
    }
}
// P1473 — `color.spot(name, fallback)`, escopo reduzido (sem `.tint()`).
impl ObservationEq for crate::entities::path::VirtualRoot {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Project => match other {
                Self::Project => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Package(a0) => match other {
                Self::Package(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::package_spec::PackageSpec {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { namespace: _, name: _, version: _ } = self;
        ObservationRelation::Same
            .and(self.namespace.observation_relation(&other.namespace))
            .and(self.name.observation_relation(&other.name))
            .and(self.version.observation_relation(&other.version))
    }
}
impl ObservationEq for crate::entities::package_spec::PackageVersion {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { major: _, minor: _, patch: _ } = self;
        ObservationRelation::Same
            .and(self.major.observation_relation(&other.major))
            .and(self.minor.observation_relation(&other.minor))
            .and(self.patch.observation_relation(&other.patch))
    }
}
impl ObservationEq for crate::entities::enum_numbering::EnumNumbering {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Decimal => match other {
                Self::Decimal => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::LowerAlpha => match other {
                Self::LowerAlpha => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::UpperAlpha => match other {
                Self::UpperAlpha => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::LowerRoman => match other {
                Self::LowerRoman => ObservationRelation::Same,
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
impl ObservationEq for crate::entities::counter::CounterKey {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Page => match other {
                Self::Page => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Selector(a0) => match other {
                Self::Selector(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Str(a0) => match other {
                Self::Str(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::numbering::Numbering {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Pattern(a0) => match other {
                Self::Pattern(b0) => {
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
impl ObservationEq for crate::entities::bib_entry::BibEntry {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self {
            key: _,
            author: _,
            title: _,
            year: _,
            volume: _,
            pages: _,
            journal: _,
            publisher: _,
            url: _,
            doi: _,
            editor: _,
            series: _,
            note: _,
            isbn: _,
            location: _,
            organization: _,
        } = self;
        ObservationRelation::Same
            .and(self.key.observation_relation(&other.key))
            .and(self.author.observation_relation(&other.author))
            .and(self.title.observation_relation(&other.title))
            .and(self.year.observation_relation(&other.year))
            .and(self.volume.observation_relation(&other.volume))
            .and(self.pages.observation_relation(&other.pages))
            .and(self.journal.observation_relation(&other.journal))
            .and(self.publisher.observation_relation(&other.publisher))
            .and(self.url.observation_relation(&other.url))
            .and(self.doi.observation_relation(&other.doi))
            .and(self.editor.observation_relation(&other.editor))
            .and(self.series.observation_relation(&other.series))
            .and(self.note.observation_relation(&other.note))
            .and(self.isbn.observation_relation(&other.isbn))
            .and(self.location.observation_relation(&other.location))
            .and(self.organization.observation_relation(&other.organization))
    }
}
impl ObservationEq for crate::entities::source_result::SourceDiagnostic {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self {
            severity: _,
            span: _,
            message: _,
            hints: _,
            trace: _,
        } = self;
        ObservationRelation::Same
            .and(self.severity.observation_relation(&other.severity))
            .and(self.span.observation_relation(&other.span))
            .and(self.message.observation_relation(&other.message))
            .and(self.hints.observation_relation(&other.hints))
            .and(self.trace.observation_relation(&other.trace))
    }
}
impl ObservationEq for crate::entities::source_result::Severity {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Error => match other {
                Self::Error => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Warning => match other {
                Self::Warning => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::source_result::Tracepoint {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Call(a0) => match other {
                Self::Call(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Show(a0) => match other {
                Self::Show(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Import(a0) => match other {
                Self::Import(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Include(a0) => match other {
                Self::Include(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl<T: ObservationEq> ObservationEq for crate::entities::span::Spanned<T> {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { v: _, span: _ } = self;
        ObservationRelation::Same
            .and(self.v.observation_relation(&other.v))
            .and(self.span.observation_relation(&other.span))
    }
}
impl ObservationEq for crate::entities::show::ShowRule {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { id: _, selector: _, transform: _ } = self;
        ObservationRelation::Same
            .and(self.id.observation_relation(&other.id))
            .and(self.selector.observation_relation(&other.selector))
            .and(self.transform.observation_relation(&other.transform))
    }
}
impl ObservationEq for crate::entities::selector::Selector {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::NativeElement(a0) => match other {
                Self::NativeElement(b0) => a0.observation_relation(b0),
                _ => ObservationRelation::Different,
            },
            Self::Kind(a0) => match other {
                Self::Kind(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Label(a0) => match other {
                Self::Label(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Location(a0) => match other {
                Self::Location(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::And(a0) => match other {
                Self::And(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Or(a0) => match other {
                Self::Or(b0) => {
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
            Self::Where { base: a0, field: a1, value: a2 } => match other {
                Self::Where { base: b0, field: b1, value: b2 } => {
                    ObservationRelation::Same
                        .and(a0.observation_relation(b0))
                        .and(a1.observation_relation(b1))
                        .and(a2.observation_relation(b2))
                }
                _ => ObservationRelation::Different,
            },
            Self::Within { base: a0, ancestor: a1 } => match other {
                Self::Within { base: b0, ancestor: b1 } => ObservationRelation::Same
                    .and(a0.observation_relation(b0))
                    .and(a1.observation_relation(b1)),
                _ => ObservationRelation::Different,
            },
            Self::Before { base: a0, end: a1, inclusive: a2 } => match other {
                Self::Before { base: b0, end: b1, inclusive: b2 } => {
                    ObservationRelation::Same
                        .and(a0.observation_relation(b0))
                        .and(a1.observation_relation(b1))
                        .and(a2.observation_relation(b2))
                }
                _ => ObservationRelation::Different,
            },
            Self::After { base: a0, start: a1, inclusive: a2 } => match other {
                Self::After { base: b0, start: b1, inclusive: b2 } => {
                    ObservationRelation::Same
                        .and(a0.observation_relation(b0))
                        .and(a1.observation_relation(b1))
                        .and(a2.observation_relation(b2))
                }
                _ => ObservationRelation::Different,
            },
            Self::Element { function: a0, fields: a1 } => match other {
                Self::Element { function: b0, fields: b1 } => ObservationRelation::Same
                    .and(a0.observation_relation(b0))
                    .and(a1.observation_relation(b1)),
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::show::Selector {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::NativeElement(a) => match other {
                Self::NativeElement(b) => a.observation_relation(b),
                _ => ObservationRelation::Different,
            },
            Self::Text(a0) => match other {
                Self::Text(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::NodeKind(a0) => match other {
                Self::NodeKind(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::DynKind(a0) => match other {
                Self::DynKind(b0) => {
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
            Self::Where { base: a0, field: a1, value: a2 } => match other {
                Self::Where { base: b0, field: b1, value: b2 } => {
                    ObservationRelation::Same
                        .and(a0.observation_relation(b0))
                        .and(a1.observation_relation(b1))
                        .and(a2.observation_relation(b2))
                }
                _ => ObservationRelation::Different,
            },
            Self::And(a0) => match other {
                Self::And(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Or(a0) => match other {
                Self::Or(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Label(a0) => match other {
                Self::Label(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::element_kind::ElementKind {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Heading => match other {
                Self::Heading => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Figure => match other {
                Self::Figure => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Citation => match other {
                Self::Citation => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Metadata => match other {
                Self::Metadata => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::State => match other {
                Self::State => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::StateUpdate => match other {
                Self::StateUpdate => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Outline => match other {
                Self::Outline => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::OutlineEntry => match other {
                Self::OutlineEntry => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Bibliography => match other {
                Self::Bibliography => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Equation => match other {
                Self::Equation => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::CounterUpdate => match other {
                Self::CounterUpdate => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Table => match other {
                Self::Table => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::List => match other {
                Self::List => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Enum => match other {
                Self::Enum => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Par => match other {
                Self::Par => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Link => match other {
                Self::Link => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Raw => match other {
                Self::Raw => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Quote => match other {
                Self::Quote => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Footnote => match other {
                Self::Footnote => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::StateDisplay => match other {
                Self::StateDisplay => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::CounterDisplay => match other {
                Self::CounterDisplay => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::ContextBlock => match other {
                Self::ContextBlock => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::show::Transformation {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Func(a0) => match other {
                Self::Func(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Content(a0) => match other {
                Self::Content(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Str(a0) => match other {
                Self::Str(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Style(a0) => match other {
                Self::Style(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::args::Args {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { items: _, named: _, span: _, occurrences: _ } = self;
        ObservationRelation::Same
            .and(self.items.observation_relation(&other.items))
            .and(self.named.observation_relation(&other.named))
            .and(self.span.observation_relation(&other.span))
            .and(self.occurrences.observation_relation(&other.occurrences))
    }
}
impl ObservationEq for crate::entities::args::ArgOccurrence {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { name: _, value: _, span: _, value_span: _ } = self;
        ObservationRelation::Same
            .and(self.name.observation_relation(&other.name))
            .and(self.value.observation_relation(&other.value))
            .and(self.span.observation_relation(&other.span))
            .and(self.value_span.observation_relation(&other.value_span))
    }
}
impl ObservationEq for crate::entities::show::NodeKind {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Heading => match other {
                Self::Heading => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Figure => match other {
                Self::Figure => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Strong => match other {
                Self::Strong => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Emph => match other {
                Self::Emph => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Raw => match other {
                Self::Raw => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Equation => match other {
                Self::Equation => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::ListItem => match other {
                Self::ListItem => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Underline => match other {
                Self::Underline => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Strike => match other {
                Self::Strike => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Overline => match other {
                Self::Overline => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Smallcaps => match other {
                Self::Smallcaps => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Subscript => match other {
                Self::Subscript => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Superscript => match other {
                Self::Superscript => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Highlight => match other {
                Self::Highlight => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Link => match other {
                Self::Link => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Quote => match other {
                Self::Quote => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Footnote => match other {
                Self::Footnote => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::List => match other {
                Self::List => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Enum => match other {
                Self::Enum => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Par => match other {
                Self::Par => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::syntax_kind::SyntaxKind {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::End => match other {
                Self::End => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Error => match other {
                Self::Error => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Shebang => match other {
                Self::Shebang => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::LineComment => match other {
                Self::LineComment => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::BlockComment => match other {
                Self::BlockComment => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Markup => match other {
                Self::Markup => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Text => match other {
                Self::Text => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Space => match other {
                Self::Space => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Linebreak => match other {
                Self::Linebreak => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Parbreak => match other {
                Self::Parbreak => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Escape => match other {
                Self::Escape => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Shorthand => match other {
                Self::Shorthand => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::SmartQuote => match other {
                Self::SmartQuote => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Strong => match other {
                Self::Strong => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Emph => match other {
                Self::Emph => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Raw => match other {
                Self::Raw => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::RawLang => match other {
                Self::RawLang => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::RawDelim => match other {
                Self::RawDelim => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::RawTrimmed => match other {
                Self::RawTrimmed => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Link => match other {
                Self::Link => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Label => match other {
                Self::Label => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Ref => match other {
                Self::Ref => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::RefMarker => match other {
                Self::RefMarker => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Heading => match other {
                Self::Heading => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::HeadingMarker => match other {
                Self::HeadingMarker => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::ListItem => match other {
                Self::ListItem => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::ListMarker => match other {
                Self::ListMarker => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::EnumItem => match other {
                Self::EnumItem => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::EnumMarker => match other {
                Self::EnumMarker => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::TermItem => match other {
                Self::TermItem => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::TermMarker => match other {
                Self::TermMarker => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Equation => match other {
                Self::Equation => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Math => match other {
                Self::Math => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::MathText => match other {
                Self::MathText => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::MathIdent => match other {
                Self::MathIdent => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::MathShorthand => match other {
                Self::MathShorthand => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::MathAlignPoint => match other {
                Self::MathAlignPoint => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::MathDelimited => match other {
                Self::MathDelimited => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::MathAttach => match other {
                Self::MathAttach => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::MathPrimes => match other {
                Self::MathPrimes => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::MathFrac => match other {
                Self::MathFrac => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::MathRoot => match other {
                Self::MathRoot => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Hash => match other {
                Self::Hash => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::LeftBrace => match other {
                Self::LeftBrace => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::RightBrace => match other {
                Self::RightBrace => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::LeftBracket => match other {
                Self::LeftBracket => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::RightBracket => match other {
                Self::RightBracket => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::LeftParen => match other {
                Self::LeftParen => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::RightParen => match other {
                Self::RightParen => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Comma => match other {
                Self::Comma => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Semicolon => match other {
                Self::Semicolon => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Colon => match other {
                Self::Colon => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Star => match other {
                Self::Star => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Underscore => match other {
                Self::Underscore => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Dollar => match other {
                Self::Dollar => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Plus => match other {
                Self::Plus => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Minus => match other {
                Self::Minus => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Slash => match other {
                Self::Slash => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Hat => match other {
                Self::Hat => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Dot => match other {
                Self::Dot => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Eq => match other {
                Self::Eq => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::EqEq => match other {
                Self::EqEq => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::ExclEq => match other {
                Self::ExclEq => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Lt => match other {
                Self::Lt => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::LtEq => match other {
                Self::LtEq => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Gt => match other {
                Self::Gt => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::GtEq => match other {
                Self::GtEq => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::PlusEq => match other {
                Self::PlusEq => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::HyphEq => match other {
                Self::HyphEq => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::StarEq => match other {
                Self::StarEq => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::SlashEq => match other {
                Self::SlashEq => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Dots => match other {
                Self::Dots => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Arrow => match other {
                Self::Arrow => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Root => match other {
                Self::Root => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Bang => match other {
                Self::Bang => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Not => match other {
                Self::Not => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::And => match other {
                Self::And => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Or => match other {
                Self::Or => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::None => match other {
                Self::None => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Auto => match other {
                Self::Auto => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Let => match other {
                Self::Let => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Set => match other {
                Self::Set => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Show => match other {
                Self::Show => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Context => match other {
                Self::Context => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::If => match other {
                Self::If => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Else => match other {
                Self::Else => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::For => match other {
                Self::For => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::In => match other {
                Self::In => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::While => match other {
                Self::While => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Break => match other {
                Self::Break => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Continue => match other {
                Self::Continue => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Return => match other {
                Self::Return => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Import => match other {
                Self::Import => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Include => match other {
                Self::Include => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::As => match other {
                Self::As => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Code => match other {
                Self::Code => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Ident => match other {
                Self::Ident => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Bool => match other {
                Self::Bool => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Int => match other {
                Self::Int => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Float => match other {
                Self::Float => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Numeric => match other {
                Self::Numeric => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Str => match other {
                Self::Str => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::CodeBlock => match other {
                Self::CodeBlock => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::ContentBlock => match other {
                Self::ContentBlock => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Parenthesized => match other {
                Self::Parenthesized => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Array => match other {
                Self::Array => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Dict => match other {
                Self::Dict => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Named => match other {
                Self::Named => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Keyed => match other {
                Self::Keyed => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Unary => match other {
                Self::Unary => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Binary => match other {
                Self::Binary => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::FieldAccess => match other {
                Self::FieldAccess => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::FuncCall => match other {
                Self::FuncCall => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Args => match other {
                Self::Args => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Spread => match other {
                Self::Spread => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Closure => match other {
                Self::Closure => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Params => match other {
                Self::Params => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::LetBinding => match other {
                Self::LetBinding => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::SetRule => match other {
                Self::SetRule => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::ShowRule => match other {
                Self::ShowRule => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Contextual => match other {
                Self::Contextual => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Conditional => match other {
                Self::Conditional => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::WhileLoop => match other {
                Self::WhileLoop => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::ForLoop => match other {
                Self::ForLoop => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::ModuleImport => match other {
                Self::ModuleImport => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::ImportItems => match other {
                Self::ImportItems => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::ImportItemPath => match other {
                Self::ImportItemPath => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::RenamedImportItem => match other {
                Self::RenamedImportItem => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::ModuleInclude => match other {
                Self::ModuleInclude => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::LoopBreak => match other {
                Self::LoopBreak => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::LoopContinue => match other {
                Self::LoopContinue => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::FuncReturn => match other {
                Self::FuncReturn => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Destructuring => match other {
                Self::Destructuring => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::DestructAssignment => match other {
                Self::DestructAssignment => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::scope::Capturer {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Function => match other {
                Self::Function => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Context => match other {
                Self::Context => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl<T: ObservationEq> ObservationEq for crate::entities::ptr_eq_arc::PtrEqArc<T> {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self(_) = self;
        ObservationRelation::Same.and(self.0.observation_relation(&other.0))
    }
}
impl ObservationEq for crate::entities::state_update::StateUpdate {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Set(a0) => match other {
                Self::Set(b0) => {
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
impl ObservationEq for crate::entities::list_marker::ListMarker {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Default => match other {
                Self::Default => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Custom(a0) => match other {
                Self::Custom(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Array(a0) => match other {
                Self::Array(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::citation_form::CitationForm {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Normal => match other {
                Self::Normal => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Prose => match other {
                Self::Prose => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Author => match other {
                Self::Author => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Year => match other {
                Self::Year => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::counter::Counter {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { key: _ } = self;
        ObservationRelation::Same.and(self.key.observation_relation(&other.key))
    }
}
impl ObservationEq for crate::entities::state::State {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { key: _, init: _ } = self;
        ObservationRelation::Same
            .and(self.key.observation_relation(&other.key))
            .and(self.init.observation_relation(&other.init))
    }
}
impl ObservationEq for crate::entities::value::Type {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::None => match other {
                Self::None => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Auto => match other {
                Self::Auto => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Bool => match other {
                Self::Bool => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Int => match other {
                Self::Int => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Float => match other {
                Self::Float => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Str => match other {
                Self::Str => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Array => match other {
                Self::Array => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Dictionary => match other {
                Self::Dictionary => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Module => match other {
                Self::Module => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Datetime => match other {
                Self::Datetime => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Function => match other {
                Self::Function => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Content => match other {
                Self::Content => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Length => match other {
                Self::Length => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Ratio => match other {
                Self::Ratio => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Relative => match other {
                Self::Relative => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Angle => match other {
                Self::Angle => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Color => match other {
                Self::Color => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Stroke => match other {
                Self::Stroke => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Fraction => match other {
                Self::Fraction => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Alignment => match other {
                Self::Alignment => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Location => match other {
                Self::Location => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Gradient => match other {
                Self::Gradient => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Regex => match other {
                Self::Regex => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Tiling => match other {
                Self::Tiling => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Bytes => match other {
                Self::Bytes => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Decimal => match other {
                Self::Decimal => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Duration => match other {
                Self::Duration => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Version => match other {
                Self::Version => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Selector => match other {
                Self::Selector => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Symbol => match other {
                Self::Symbol => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Arguments => match other {
                Self::Arguments => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::State => match other {
                Self::State => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Counter => match other {
                Self::Counter => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Label => match other {
                Self::Label => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Direction => match other {
                Self::Direction => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Path => match other {
                Self::Path => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            Self::Type => match other {
                Self::Type => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
            // P1473 — `color.spot(...)`.
            Self::SpotColorant => match other {
                Self::SpotColorant => ObservationRelation::Same,
                _ => ObservationRelation::Different,
            },
        }
    }
}
impl ObservationEq for crate::entities::counter_update::CounterUpdate {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        match self {
            Self::Set(a0) => match other {
                Self::Set(b0) => {
                    ObservationRelation::Same.and(a0.observation_relation(b0))
                }
                _ => ObservationRelation::Different,
            },
            Self::Step(a0) => match other {
                Self::Step(b0) => {
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
impl ObservationEq for crate::entities::duration::Duration {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { nanos: _ } = self;
        ObservationRelation::Same.and(self.nanos.observation_relation(&other.nanos))
    }
}
impl ObservationEq for crate::entities::symbol::Symbol {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { value: _, name: _, variants: _, applied: _ } = self;
        ObservationRelation::Same
            .and(self.value.observation_relation(&other.value))
            .and(self.name.observation_relation(&other.name))
            .and(self.variants.observation_relation(&other.variants))
            .and(self.applied.observation_relation(&other.applied))
    }
}
impl<K: ObservationEq, V: ObservationEq, S> ObservationEq
    for indexmap::IndexMap<K, V, S>
{
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        if self.len() != other.len() {
            return ObservationRelation::Different;
        }
        self.iter().zip(other.iter()).fold(
            ObservationRelation::Same,
            |r, ((ak, av), (bk, bv))| {
                r.and(ak.observation_relation(bk)).and(av.observation_relation(bv))
            },
        )
    }
}
impl ObservationEq for crate::entities::value::IntrospectedContent {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        self.content()
            .observation_relation(other.content())
            .and(self.fields().observation_relation(&other.fields()))
    }
}
impl ObservationEq for crate::entities::location::Location {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        self.as_u128().observation_relation(&other.as_u128())
    }
}
impl ObservationEq for crate::entities::path::RootedPath {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        self.root()
            .observation_relation(other.root())
            .and(self.vpath().observation_relation(other.vpath()))
    }
}
impl ObservationEq for crate::entities::path::VirtualPath {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        self.get_with_slash().observation_relation(other.get_with_slash())
    }
}
impl ObservationEq for crate::entities::bytes::Bytes {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        self.as_slice().observation_relation(other.as_slice())
    }
}
impl ObservationEq for crate::entities::world_types::Bytes {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        self.as_slice().observation_relation(other.as_slice())
    }
}
impl ObservationEq for crate::entities::regex::Regex {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        self.pattern().observation_relation(other.pattern())
    }
}
impl ObservationEq for crate::entities::decimal::Decimal {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        self.0.serialize().observation_relation(&other.0.serialize())
    }
}
impl ObservationEq for crate::entities::version::Version {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        let Self { components: _ } = self;
        self.components.observation_relation(&other.components)
    }
}
impl ObservationEq for Scope {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        if self.len() != other.len() {
            return ObservationRelation::Different;
        }
        self.iter().zip(other.iter()).fold(
            ObservationRelation::Same,
            |r, ((ak, av), (bk, bv))| {
                r.and(ak.observation_relation(bk))
                    .and(av.value().observation_relation(bv.value()))
            },
        )
    }
}
impl ObservationEq for Module {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        // Module::eq is audited Arc identity, not structural or language equality.
        // Shared ModuleInner cannot be mutated (setters require Arc::get_mut).
        ObservationRelation::exact(self == other)
    }
}
impl ObservationEq for Func {
    fn observation_relation(&self, other: &Self) -> ObservationRelation {
        use crate::entities::func::FuncRepr;
        match (&*self.0, &*other.0) {
            (FuncRepr::Closure(_), FuncRepr::Closure(_)) => {
                ObservationRelation::exact(Arc::ptr_eq(&self.0, &other.0))
            }
            (FuncRepr::Native(a), FuncRepr::Native(b)) => {
                let crate::entities::func::NativeFunc { name: _, call: _, namespace: _ } =
                    a;
                ObservationRelation::exact(std::ptr::fn_addr_eq(a.call, b.call))
                    .and(a.name.observation_relation(b.name))
                    .and(a.namespace.observation_relation(&b.namespace))
            }
            (FuncRepr::Format(ak, a), FuncRepr::Format(bk, b)) => {
                let crate::entities::func::NativeFunc { name: _, call: _, namespace: _ } =
                    a;
                ObservationRelation::exact(ak == bk)
                    .and(ObservationRelation::exact(std::ptr::fn_addr_eq(a.call, b.call)))
                    .and(a.name.observation_relation(b.name))
                    .and(a.namespace.observation_relation(&b.namespace))
            }
            (FuncRepr::NativeWithEngine(a), FuncRepr::NativeWithEngine(b)) => {
                let crate::entities::func::NativeFuncWithEngine {
                    name: _,
                    call: _,
                    namespace: _,
                } = a;
                ObservationRelation::exact(std::ptr::fn_addr_eq(a.call, b.call))
                    .and(a.name.observation_relation(b.name))
                    .and(a.namespace.observation_relation(&b.namespace))
            }
            (FuncRepr::With(a), FuncRepr::With(b)) => a.observation_relation(b),
            (FuncRepr::Element(_), FuncRepr::Element(_))
            | (FuncRepr::Plugin(_), FuncRepr::Plugin(_)) => ObservationRelation::Unproven,
            (
                FuncRepr::Closure(_)
                | FuncRepr::Native(_)
                | FuncRepr::Format(_, _)
                | FuncRepr::NativeWithEngine(_)
                | FuncRepr::With(_)
                | FuncRepr::Element(_)
                | FuncRepr::Plugin(_),
                _,
            ) => ObservationRelation::Different,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::value::Value;

    #[test]
    fn native_bare_observation_distinguishes_variant_and_function_payload() {
        use crate::entities::{func::Func, selector::Selector};
        let cell = Func::native("cell", crate::compiler::stdlib::native_table_cell);
        let bare = Selector::NativeElement(cell.clone());
        assert_eq!(bare.observation_relation(&bare), ObservationRelation::Same);
        let empty = Selector::Element { function: cell, fields: Default::default() };
        assert_eq!(bare.observation_relation(&empty), ObservationRelation::Different);
        let fake = Selector::NativeElement(Func::native(
            "cell",
            crate::compiler::stdlib::native_heading,
        ));
        assert_eq!(bare.observation_relation(&fake), ObservationRelation::Different);
    }

    #[test]
    fn distingue_valores_iguais_e_payload_diferente() {
        assert_eq!(
            Value::Int(7).observation_relation(&Value::Int(7)),
            ObservationRelation::Same
        );
        assert_eq!(
            Value::Int(7).observation_relation(&Value::Int(8)),
            ObservationRelation::Different
        );
        assert_eq!(
            Value::Int(7).observation_relation(&Value::Bool(true)),
            ObservationRelation::Different
        );
    }
}

#[cfg(test)]
mod native_format_catalogue_observation_tests {
    use super::*;
    #[test]
    fn native_format_catalogue_observation_witness() {
        let scope = crate::compiler::stdlib::make_stdlib_with_features(
            &crate::contracts::world::SysInputs::default(),
            crate::entities::compiler_features::Features::empty(),
        );
        let Value::Module(module) = scope.get("format").unwrap() else {
            panic!("format module")
        };
        let Value::Func(svg) = module.scope().get("svg").unwrap() else { panic!("svg") };
        assert_eq!(svg.observation_relation(&svg.clone()), ObservationRelation::Same);
        let unmarked = crate::entities::func::Func::native_with_namespace(
            "svg",
            svg.native_fn_addr().unwrap(),
            Arc::new(svg.namespace().unwrap().clone()),
        );
        assert_eq!(svg, &unmarked);
        assert_eq!(svg.observation_relation(&unmarked), ObservationRelation::Different);
    }
}

#[cfg(test)]
mod native_format_catalogue_axis_tests {
    use super::*;
    use crate::entities::func::{FormatElementKind, Func, NativeFunc};
    fn never_called(
        _: &mut crate::compiler::eval::EvalContext,
        _: &crate::entities::args::Args,
        _: &dyn crate::contracts::world::World,
        _: crate::entities::file_id::FileId,
    ) -> crate::entities::source_result::SourceResult<Value> {
        panic!("observation executed callback")
    }
    #[test]
    fn native_format_catalogue_observation_kind_and_payload_axes() {
        let kinds = [
            FormatElementKind::Svg,
            FormatElementKind::Png,
            FormatElementKind::Pdf,
            FormatElementKind::Html,
            FormatElementKind::Bundle,
        ];
        let make = |kind| {
            Func::native_format(
                kind,
                NativeFunc { name: "same", call: never_called, namespace: None },
            )
        };
        for (i, left) in kinds.iter().enumerate() {
            for (j, right) in kinds.iter().enumerate() {
                let a = make(*left);
                let b = make(*right);
                assert_eq!(a, b);
                assert_eq!(
                    a.observation_relation(&b),
                    if i == j {
                        ObservationRelation::Same
                    } else {
                        ObservationRelation::Different
                    }
                );
            }
        }
        let base = make(FormatElementKind::Svg);
        let changed_call = Func::native_format(
            FormatElementKind::Svg,
            NativeFunc {
                name: "same",
                call: crate::compiler::stdlib::native_assert,
                namespace: None,
            },
        );
        let changed_name = Func::native_format(
            FormatElementKind::Svg,
            NativeFunc { name: "other", call: never_called, namespace: None },
        );
        let mut first = Scope::new();
        first.define("x", Value::Int(1));
        let mut second = Scope::new();
        second.define("x", Value::Int(2));
        let first = Func::native_format(
            FormatElementKind::Svg,
            NativeFunc {
                name: "same",
                call: never_called,
                namespace: Some(Arc::new(first)),
            },
        );
        let second = Func::native_format(
            FormatElementKind::Svg,
            NativeFunc {
                name: "same",
                call: never_called,
                namespace: Some(Arc::new(second)),
            },
        );
        assert_eq!(
            base.observation_relation(&changed_call),
            ObservationRelation::Different
        );
        assert_eq!(
            base.observation_relation(&changed_name),
            ObservationRelation::Different
        );
        assert_eq!(base.observation_relation(&first), ObservationRelation::Different);
        assert_eq!(first.observation_relation(&second), ObservationRelation::Different);
        assert_eq!(base.observation_relation(&base.clone()), ObservationRelation::Same);
    }
}
