//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/physical_memo/introspection.md
//! @prompt-hash 6a9ab63e
//! @layer L1
//! @updated 2026-11-13

use super::Journal;
use crate::entities::bib_entry::BibEntry;
use crate::entities::counter::CounterKey;
use crate::entities::element_kind::ElementKind;
use crate::entities::introspector::{Introspector, TagIntrospector};
use crate::entities::label::Label;
use crate::entities::location::Location;
use crate::entities::selector::Selector;
use crate::entities::value::Value;

pub(in crate::compiler::layout) struct RecordingIntrospector<'a> {
    pub(in crate::compiler::layout) input: &'a TagIntrospector,
    pub(in crate::compiler::layout) journal: &'a Journal,
}
#[derive(PartialEq)]
enum CounterView {
    Page,
    Kind(ElementKind),
    Str(String),
}
fn counter_view(k: &CounterKey) -> Option<CounterView> {
    match k {
        CounterKey::Page => Some(CounterView::Page),
        CounterKey::Str(s) => (s.len() <= 16384).then(|| CounterView::Str(s.to_string())),
        CounterKey::Selector(s) => {
            if let Selector::Kind(k) = s {
                Some(CounterView::Kind(*k))
            } else {
                None
            }
        }
    }
}
fn numbering_view(
    n: Option<&crate::entities::numbering::Numbering>,
) -> Option<Option<String>> {
    match n {
        None => Some(None),
        Some(crate::entities::numbering::Numbering::Pattern(s)) => {
            (s.len() <= 16384).then(|| Some(s.to_string()))
        }
        Some(crate::entities::numbering::Numbering::Func(_)) => None,
    }
}

trait Footprint {
    fn footprint(&self) -> Option<usize>;
}
fn footprint<T: Footprint + ?Sized>(value: &T) -> Option<usize> {
    value.footprint().filter(|n| *n <= 64 * 1024)
}
impl<T: Footprint + ?Sized> Footprint for &T {
    fn footprint(&self) -> Option<usize> {
        footprint(*self)
    }
}
impl Footprint for str {
    fn footprint(&self) -> Option<usize> {
        Some(self.len())
    }
}
impl Footprint for String {
    fn footprint(&self) -> Option<usize> {
        self.len().checked_add(std::mem::size_of::<Self>())
    }
}
impl Footprint for Label {
    fn footprint(&self) -> Option<usize> {
        footprint(&self.0)
    }
}
impl<T: Footprint> Footprint for Option<T> {
    fn footprint(&self) -> Option<usize> {
        std::mem::size_of::<Self>().checked_add(match self {
            Some(v) => footprint(v)?,
            None => 0,
        })
    }
}
impl<T: Footprint> Footprint for [T] {
    fn footprint(&self) -> Option<usize> {
        if self.len() > 8192 {
            return None;
        }
        let mut n = 0usize;
        for v in self {
            n = n.checked_add(footprint(v)?)?;
            if n > 64 * 1024 {
                return None;
            }
        }
        Some(n)
    }
}
impl<T: Footprint> Footprint for Vec<T> {
    fn footprint(&self) -> Option<usize> {
        footprint(self.as_slice())
    }
}
impl<A: Footprint, B: Footprint> Footprint for (A, B) {
    fn footprint(&self) -> Option<usize> {
        footprint(&self.0)?.checked_add(footprint(&self.1)?)
    }
}
impl Footprint for CounterView {
    fn footprint(&self) -> Option<usize> {
        match self {
            CounterView::Str(s) => footprint(s),
            CounterView::Kind(_) | CounterView::Page => Some(std::mem::size_of::<Self>()),
        }
    }
}
fn selector_size(s: &Selector) -> Option<usize> {
    if let Selector::Kind(_) | Selector::Location(_) = s {
        Some(std::mem::size_of::<Selector>())
    } else if let Selector::Label(label) = s {
        (label.0.len() <= 16384)
            .then_some(label.0.len() + std::mem::size_of::<Selector>())
    } else {
        None
    }
}
impl Footprint for usize {
    fn footprint(&self) -> Option<usize> {
        Some(std::mem::size_of::<Self>())
    }
}
impl Footprint for u32 {
    fn footprint(&self) -> Option<usize> {
        Some(std::mem::size_of::<Self>())
    }
}
impl Footprint for bool {
    fn footprint(&self) -> Option<usize> {
        Some(std::mem::size_of::<Self>())
    }
}
impl Footprint for Location {
    fn footprint(&self) -> Option<usize> {
        Some(std::mem::size_of::<Self>())
    }
}
impl Footprint for crate::entities::label_kind::UnreferencableKind {
    fn footprint(&self) -> Option<usize> {
        Some(std::mem::size_of::<Self>())
    }
}
impl Footprint for std::num::NonZeroUsize {
    fn footprint(&self) -> Option<usize> {
        Some(std::mem::size_of::<Self>())
    }
}
impl Footprint for crate::entities::position::Position {
    fn footprint(&self) -> Option<usize> {
        Some(std::mem::size_of::<Self>())
    }
}
impl Footprint for crate::entities::position::HtmlPosition {
    fn footprint(&self) -> Option<usize> {
        Some(std::mem::size_of::<Self>())
    }
}

// Bounded immutable return domain, not a language-equality operation.
fn heading_body_is_closed(content: &crate::entities::content::Content) -> bool {
    use crate::entities::content::Content;
    fn visit(
        content: &Content,
        depth: usize,
        nodes: &mut usize,
        payload: &mut usize,
    ) -> bool {
        if depth > 8 || *nodes >= 128 {
            return false;
        }
        *nodes += 1;
        let Some(size) = payload.checked_add(std::mem::size_of::<Content>()) else {
            return false;
        };
        if size > 16 * 1024 {
            return false;
        }
        *payload = size;
        match content {
            Content::Text(text) => {
                let Some(size) = payload.checked_add(text.len()) else {
                    return false;
                };
                if size > 16 * 1024 {
                    return false;
                }
                *payload = size;
                true
            }
            Content::Empty
            | Content::Space
            | Content::SpaceFromNewline
            | Content::Parbreak => true,
            Content::Sequence(parts) => {
                parts.iter().all(|part| visit(part, depth + 1, nodes, payload))
            }
            Content::Strong(element) => visit(&element.body, depth + 1, nodes, payload),
            Content::Emph(element) => visit(&element.body, depth + 1, nodes, payload),
            Content::Source(..)
            | Content::ShowRecipe(..)
            | Content::Realized(..)
            | Content::Located(..)
            | Content::Symbol(..)
            | Content::HtmlElem(..)
            | Content::HtmlFrame(..)
            | Content::Par { .. }
            | Content::Heading(..)
            | Content::Title(..)
            | Content::Raw(..)
            | Content::ListItem(..)
            | Content::EnumItem(..)
            | Content::Link(..)
            | Content::Equation(..)
            | Content::MathSequence(..)
            | Content::MathIdent(..)
            | Content::MathText(..)
            | Content::MathFrac(..)
            | Content::MathAttach(..)
            | Content::MathPrimes(..)
            | Content::MathRoot(..)
            | Content::MathStretch(..)
            | Content::MathDelimited(..)
            | Content::MathLr(..)
            | Content::MathMid(..)
            | Content::MathOverbrace(..)
            | Content::MathUnderbrace(..)
            | Content::MathUnderbracket(..)
            | Content::MathUnderparen(..)
            | Content::MathUndershell(..)
            | Content::MathOverbracket(..)
            | Content::MathOverparen(..)
            | Content::MathOvershell(..)
            | Content::MathAlignPoint(..)
            | Content::Linebreak(..)
            | Content::MathMatrix(..)
            | Content::MathCases(..)
            | Content::MathAccent(..)
            | Content::MathCancel(..)
            | Content::MathUnderline(..)
            | Content::MathOverline(..)
            | Content::MathVec(..)
            | Content::MathClassOverride(..)
            | Content::MathLimitsOverride(..)
            | Content::MathUnderover(..)
            | Content::MathOp(..)
            | Content::MathStyled(..)
            | Content::Label(..)
            | Content::Ref(..)
            | Content::CounterDisplay(..)
            | Content::CounterUpdate(..)
            | Content::Outline(..)
            | Content::OutlineEntry(..)
            | Content::Figure(..)
            | Content::FigureCaption(..)
            | Content::Image(..)
            | Content::Shape(..)
            | Content::Curve(..)
            | Content::Transform(..)
            | Content::Grid(..)
            | Content::GridHeader(..)
            | Content::GridFooter(..)
            | Content::GridCell(..)
            | Content::SetPage(..)
            | Content::PageRun(..)
            | Content::Align(..)
            | Content::Place(..)
            | Content::Flush(..)
            | Content::Styled(..)
            | Content::Divider(..)
            | Content::Terms(..)
            | Content::TermItem(..)
            | Content::Quote(..)
            | Content::Document { .. }
            | Content::Asset { .. }
            | Content::SmartQuote(..)
            | Content::PdfAttach(..)
            | Content::PdfArtifact(..)
            | Content::Underline(..)
            | Content::Strike(..)
            | Content::Overline(..)
            | Content::SmallCaps { .. }
            | Content::Pad(..)
            | Content::Hide(..)
            | Content::HSpace(..)
            | Content::VSpace(..)
            | Content::Pagebreak(..)
            | Content::Colbreak(..)
            | Content::Stack(..)
            | Content::Boxed(..)
            | Content::Block(..)
            | Content::TableCell(..)
            | Content::Bibliography(..)
            | Content::Cite(..)
            | Content::Footnote(..)
            | Content::FootnoteEntry(..)
            | Content::TableHeader(..)
            | Content::TableFooter(..)
            | Content::GridHLine(..)
            | Content::GridVLine(..)
            | Content::TableHLine(..)
            | Content::TableVLine(..)
            | Content::Table(..)
            | Content::Repeat(..)
            | Content::Columns(..)
            | Content::Metadata(..)
            | Content::State(..)
            | Content::StateUpdate(..)
            | Content::StateDisplay(..)
            | Content::CounterDisplayCallback(..)
            | Content::ContextBlock(..)
            | Content::Dynamic(..) => false,
        }
    }
    visit(content, 0, &mut 0, &mut 0)
}

impl Introspector for RecordingIntrospector<'_> {
    fn query_by_kind(&self, kind: ElementKind) -> Vec<Location> {
        let output = self.input.query_by_kind(kind);
        let output_charge = footprint(&output);
        if output_charge.is_some() {
            let expected = output.clone();
            self.journal.index(output_charge.unwrap_or(0) + 128, move |input| {
                input.query_by_kind(kind) == expected
            });
        } else {
            self.journal.decline("query_by_kind");
        }
        output
    }
    fn query_by_label(&self, label: &Label) -> Option<Location> {
        let output = self.input.query_by_label(label);
        let output_charge = footprint(&output);
        if (label.0.len() <= 16384) && output_charge.is_some() {
            let expected = output;
            let label = label.to_owned();
            self.journal
                .index(output_charge.unwrap_or(0) + 128 + label.0.len(), move |input| {
                    input.query_by_label(&label) == expected
                });
        } else {
            self.journal.decline("query_by_label");
        }
        output
    }
    fn query_first(&self, kind: ElementKind) -> Option<Location> {
        let output = self.input.query_first(kind);
        let output_charge = footprint(&output);
        if output_charge.is_some() {
            let expected = output;
            self.journal.index(output_charge.unwrap_or(0) + 128, move |input| {
                input.query_first(kind) == expected
            });
        } else {
            self.journal.decline("query_first");
        }
        output
    }
    fn query_unique(&self, kind: ElementKind) -> Option<Location> {
        let output = self.input.query_unique(kind);
        let output_charge = footprint(&output);
        if output_charge.is_some() {
            let expected = output;
            self.journal.index(output_charge.unwrap_or(0) + 128, move |input| {
                input.query_unique(kind) == expected
            });
        } else {
            self.journal.decline("query_unique");
        }
        output
    }
    fn position_of(
        &self,
        location: Location,
    ) -> Option<crate::entities::position::Position> {
        let output = self.input.position_of(location);
        let output_charge = footprint(&output);
        if output_charge.is_some() {
            let expected =
                output.map(|p| (p.page, p.point.x.0.to_bits(), p.point.y.0.to_bits()));
            self.journal.index(output_charge.unwrap_or(0) + 128, move |input| {
                input
                    .position_of(location)
                    .map(|p| (p.page, p.point.x.0.to_bits(), p.point.y.0.to_bits()))
                    == expected
            });
        } else {
            self.journal.decline("position_of");
        }
        output
    }
    fn html_position_of(
        &self,
        location: Location,
    ) -> Option<crate::entities::position::HtmlPosition> {
        let output = self.input.html_position_of(location);
        let output_charge = footprint(&output);
        if output_charge.is_some() {
            let expected = output.map(|p| (p.point.x.0.to_bits(), p.point.y.0.to_bits()));
            self.journal.index(output_charge.unwrap_or(0) + 128, move |input| {
                input
                    .html_position_of(location)
                    .map(|p| (p.point.x.0.to_bits(), p.point.y.0.to_bits()))
                    == expected
            });
        } else {
            self.journal.decline("html_position_of");
        }
        output
    }
    fn figure_number_for_label(&self, label: &Label) -> Option<usize> {
        let output = self.input.figure_number_for_label(label);
        let output_charge = footprint(&output);
        if (label.0.len() <= 16384) && output_charge.is_some() {
            let expected = output;
            let label = label.to_owned();
            self.journal
                .index(output_charge.unwrap_or(0) + 128 + label.0.len(), move |input| {
                    input.figure_number_for_label(&label) == expected
                });
        } else {
            self.journal.decline("figure_number_for_label");
        }
        output
    }
    fn query_metadata(&self) -> &[Value] {
        let output = self.input.query_metadata();
        if output.is_empty() {
            let expected = true;
            self.journal
                .index(128, move |input| input.query_metadata().is_empty() == expected);
        } else {
            self.journal.decline("query_metadata");
        }
        output
    }
    fn formatted_counter(&self, key: &CounterKey) -> Option<String> {
        let output = self.input.formatted_counter(key);
        let output_charge = footprint(&output);
        if (counter_view(key).is_some()) && output_charge.is_some() {
            let expected = output.clone();
            let key = key.to_owned();
            self.journal.index(
                counter_view(&key).and_then(|v| footprint(&v)).unwrap_or(0)
                    + output_charge.unwrap_or(0)
                    + 128,
                move |input| input.formatted_counter(&key) == expected,
            );
        } else {
            self.journal.decline("formatted_counter");
        }
        output
    }
    fn state_value(&self, key: &str, location: Location) -> Option<&Value> {
        let output = self.input.state_value(key, location);
        if key.len() <= 16384 && output.is_none() {
            let expected = true;
            let key = key.to_owned();
            self.journal.index(128 + key.len(), move |input| {
                input.state_value(&key, location).is_none() == expected
            });
        } else {
            self.journal.decline("state_value");
        }
        output
    }
    fn state_final_value(&self, key: &str) -> Option<&Value> {
        let output = self.input.state_final_value(key);
        if key.len() <= 16384 && output.is_none() {
            let expected = true;
            let key = key.to_owned();
            self.journal.index(128 + key.len(), move |input| {
                input.state_final_value(&key).is_none() == expected
            });
        } else {
            self.journal.decline("state_final_value");
        }
        output
    }
    fn state_display_value(
        &self,
        key: String,
        location: Location,
    ) -> Option<crate::entities::content::Content> {
        let output = self.input.state_display_value(key.clone(), location);
        if key.len() <= 16384 && output.is_none() {
            let expected = true;
            self.journal.index(128 + key.len(), move |input| {
                input.state_display_value(key.clone(), location).is_none() == expected
            });
        } else {
            self.journal.decline("state_display_value");
        }
        output
    }
    fn counter_display_value(
        &self,
        key: CounterKey,
        location: Location,
    ) -> Option<crate::entities::content::Content> {
        let output = self.input.counter_display_value(key.clone(), location);
        if counter_view(&key).is_some() && output.is_none() {
            let expected = true;
            self.journal.index(
                counter_view(&key).and_then(|v| footprint(&v)).unwrap_or(0) + 128,
                move |input| {
                    input.counter_display_value(key.clone(), location).is_none()
                        == expected
                },
            );
        } else {
            self.journal.decline("counter_display_value");
        }
        output
    }
    fn query(&self, selector: &Selector) -> Vec<Location> {
        let output = self.input.query(selector);
        let output_charge = footprint(&output);
        if ((matches!(
            selector,
            Selector::Kind(_) | Selector::Label(_) | Selector::Location(_)
        )) && output_charge.is_some())
            && selector_size(selector).is_some()
        {
            let expected = output.clone();
            let selector = selector.to_owned();
            self.journal.index(
                selector_size(&selector).unwrap_or(0) + output_charge.unwrap_or(0) + 128,
                move |input| input.query(&selector) == expected,
            );
        } else {
            self.journal.decline("query");
        }
        output
    }
    fn element_at(
        &self,
        location: Location,
    ) -> Option<&crate::entities::content::Content> {
        let output = self.input.element_at(location);
        if output.is_none() {
            let expected = true;
            self.journal.index(128, move |input| {
                input.element_at(location).is_none() == expected
            });
        } else if let Some(crate::entities::content::Content::Heading(heading)) = output {
            if heading_body_is_closed(&heading.body) {
                let expected = std::sync::Arc::downgrade(heading);
                let charge = 128
                    + std::mem::size_of::<crate::entities::elements::heading::HeadingElem>(
                    )
                    + 2 * std::mem::size_of::<usize>();
                self.journal.index(charge, move |input| {
                    let Some(owner) = expected.upgrade() else {
                        return false;
                    };
                    match input.element_at(location) {
                        Some(crate::entities::content::Content::Heading(current)) => {
                            std::sync::Arc::ptr_eq(&owner, current)
                        }
                        _ => false,
                    }
                });
            } else {
                self.journal.decline("element_at");
            }
        } else {
            self.journal.decline("element_at");
        }
        output
    }
    fn formatted_counter_at(
        &self,
        key: &CounterKey,
        location: Location,
    ) -> Option<String> {
        let output = self.input.formatted_counter_at(key, location);
        let output_charge = footprint(&output);
        if (counter_view(key).is_some()) && output_charge.is_some() {
            let expected = output.clone();
            let key = key.to_owned();
            self.journal.index(
                counter_view(&key).and_then(|v| footprint(&v)).unwrap_or(0)
                    + output_charge.unwrap_or(0)
                    + 128,
                move |input| input.formatted_counter_at(&key, location) == expected,
            );
        } else {
            self.journal.decline("formatted_counter_at");
        }
        output
    }
    fn counter_values_at(
        &self,
        key: &CounterKey,
        location: Location,
    ) -> Option<&[usize]> {
        let output = self.input.counter_values_at(key, location);
        let output_charge = footprint(&output);
        if (counter_view(key).is_some()) && output_charge.is_some() {
            let expected = output.map(<[usize]>::to_vec);
            let key = key.to_owned();
            self.journal.index(
                counter_view(&key).and_then(|v| footprint(&v)).unwrap_or(0)
                    + output_charge.unwrap_or(0)
                    + 128,
                move |input| {
                    input.counter_values_at(&key, location).map(<[usize]>::to_vec)
                        == expected
                },
            );
        } else {
            self.journal.decline("counter_values_at");
        }
        output
    }
    fn counter_final_values(&self, key: &CounterKey) -> Option<&[usize]> {
        let output = self.input.counter_final_values(key);
        let output_charge = footprint(&output);
        if (counter_view(key).is_some()) && output_charge.is_some() {
            let expected = output.map(<[usize]>::to_vec);
            let key = key.to_owned();
            self.journal.index(
                counter_view(&key).and_then(|v| footprint(&v)).unwrap_or(0)
                    + output_charge.unwrap_or(0)
                    + 128,
                move |input| {
                    input.counter_final_values(&key).map(<[usize]>::to_vec) == expected
                },
            );
        } else {
            self.journal.decline("counter_final_values");
        }
        output
    }
    fn counter_key_for_label(&self, label: &Label) -> Option<&CounterKey> {
        let output = self.input.counter_key_for_label(label);
        if label.0.len() <= 16384 && output.is_none_or(|key| counter_view(key).is_some())
        {
            let expected = output.map(counter_view);
            let label = label.to_owned();
            self.journal.index(
                footprint(&expected).unwrap_or(64 * 1024) + 128 + label.0.len(),
                move |input| {
                    input.counter_key_for_label(&label).map(counter_view) == expected
                },
            );
        } else {
            self.journal.decline("counter_key_for_label");
        }
        output
    }
    fn heading_has_numbering(&self, location: Location) -> Option<bool> {
        let output = self.input.heading_has_numbering(location);
        let output_charge = footprint(&output);
        if output_charge.is_some() {
            let expected = output;
            self.journal.index(output_charge.unwrap_or(0) + 128, move |input| {
                input.heading_has_numbering(location) == expected
            });
        } else {
            self.journal.decline("heading_has_numbering");
        }
        output
    }
    fn equation_has_numbering(&self, location: Location) -> Option<bool> {
        let output = self.input.equation_has_numbering(location);
        let output_charge = footprint(&output);
        if output_charge.is_some() {
            let expected = output;
            self.journal.index(output_charge.unwrap_or(0) + 128, move |input| {
                input.equation_has_numbering(location) == expected
            });
        } else {
            self.journal.decline("equation_has_numbering");
        }
        output
    }
    fn equation_numbering_pattern(&self, location: Location) -> Option<&str> {
        let output = self.input.equation_numbering_pattern(location);
        let output_charge = footprint(&output);
        if output_charge.is_some() {
            let expected = output.map(str::to_owned);
            self.journal.index(output_charge.unwrap_or(0) + 128, move |input| {
                input.equation_numbering_pattern(location).map(str::to_owned) == expected
            });
        } else {
            self.journal.decline("equation_numbering_pattern");
        }
        output
    }
    fn equation_numbering_content(
        &self,
        location: Location,
    ) -> Option<crate::entities::content::Content> {
        let output = self.input.equation_numbering_content(location);
        if output.is_none() {
            let expected = true;
            self.journal.index(128, move |input| {
                input.equation_numbering_content(location).is_none() == expected
            });
        } else {
            self.journal.decline("equation_numbering_content");
        }
        output
    }
    fn equation_supplement_content(
        &self,
        location: Location,
    ) -> Option<crate::entities::content::Content> {
        let output = self.input.equation_supplement_content(location);
        if output.is_none() {
            let expected = true;
            self.journal.index(128, move |input| {
                input.equation_supplement_content(location).is_none() == expected
            });
        } else {
            self.journal.decline("equation_supplement_content");
        }
        output
    }
    fn unreferencable_label_kind(
        &self,
        label: &Label,
    ) -> Option<crate::entities::label_kind::UnreferencableKind> {
        let output = self.input.unreferencable_label_kind(label);
        let output_charge = footprint(&output);
        if (label.0.len() <= 16384) && output_charge.is_some() {
            let expected = output;
            let label = label.to_owned();
            self.journal
                .index(output_charge.unwrap_or(0) + 128 + label.0.len(), move |input| {
                    input.unreferencable_label_kind(&label) == expected
                });
        } else {
            self.journal.decline("unreferencable_label_kind");
        }
        output
    }
    fn bib_entry_for_key(&self, key: &str) -> Option<&BibEntry> {
        let output = self.input.bib_entry_for_key(key);
        if key.len() <= 16384 && output.is_none() {
            let expected = true;
            let key = key.to_owned();
            self.journal.index(128 + key.len(), move |input| {
                input.bib_entry_for_key(&key).is_none() == expected
            });
        } else {
            self.journal.decline("bib_entry_for_key");
        }
        output
    }
    fn bib_number_for_key(&self, key: &str) -> Option<u32> {
        let output = self.input.bib_number_for_key(key);
        let output_charge = footprint(&output);
        if (key.len() <= 16384) && output_charge.is_some() {
            let expected = output;
            let key = key.to_owned();
            self.journal
                .index(output_charge.unwrap_or(0) + 128 + key.len(), move |input| {
                    input.bib_number_for_key(&key) == expected
                });
        } else {
            self.journal.decline("bib_number_for_key");
        }
        output
    }
    fn citation_number_for_key(&self, key: &str) -> Option<u32> {
        let output = self.input.citation_number_for_key(key);
        let output_charge = footprint(&output);
        if (key.len() <= 16384) && output_charge.is_some() {
            let expected = output;
            let key = key.to_owned();
            self.journal
                .index(output_charge.unwrap_or(0) + 128 + key.len(), move |input| {
                    input.citation_number_for_key(&key) == expected
                });
        } else {
            self.journal.decline("citation_number_for_key");
        }
        output
    }
    fn citation_order(&self) -> &[String] {
        let output = self.input.citation_order();
        let output_charge = footprint(&output);
        if output_charge.is_some() {
            let expected = output.to_vec();
            self.journal.index(output_charge.unwrap_or(0) + 128, move |input| {
                input.citation_order().to_vec() == expected
            });
        } else {
            self.journal.decline("citation_order");
        }
        output
    }
    fn back_refs_for_key(&self, key: &str) -> Vec<usize> {
        let output = self.input.back_refs_for_key(key);
        let output_charge = footprint(&output);
        if (key.len() <= 16384) && output_charge.is_some() {
            let expected = output.clone();
            let key = key.to_owned();
            self.journal
                .index(output_charge.unwrap_or(0) + 128 + key.len(), move |input| {
                    input.back_refs_for_key(&key) == expected
                });
        } else {
            self.journal.decline("back_refs_for_key");
        }
        output
    }
    fn figures_for_lof(&self) -> &[(usize, String)] {
        let output = self.input.figures_for_lof();
        let output_charge = footprint(&output);
        if output_charge.is_some() {
            let expected = output.to_vec();
            self.journal.index(output_charge.unwrap_or(0) + 128, move |input| {
                input.figures_for_lof().to_vec() == expected
            });
        } else {
            self.journal.decline("figures_for_lof");
        }
        output
    }
    fn tables_for_lot(&self) -> &[(usize, String)] {
        let output = self.input.tables_for_lot();
        let output_charge = footprint(&output);
        if output_charge.is_some() {
            let expected = output.to_vec();
            self.journal.index(output_charge.unwrap_or(0) + 128, move |input| {
                input.tables_for_lot().to_vec() == expected
            });
        } else {
            self.journal.decline("tables_for_lot");
        }
        output
    }
    fn figure_number_at_index(&self, kind: &str, idx: usize) -> Option<usize> {
        let output = self.input.figure_number_at_index(kind, idx);
        let output_charge = footprint(&output);
        if (kind.len() <= 16384) && output_charge.is_some() {
            let expected = output;
            let kind = kind.to_owned();
            self.journal
                .index(output_charge.unwrap_or(0) + 128 + kind.len(), move |input| {
                    input.figure_number_at_index(&kind, idx) == expected
                });
        } else {
            self.journal.decline("figure_number_at_index");
        }
        output
    }
    fn flat_counter_at(&self, key: &CounterKey, location: Location) -> Option<usize> {
        let output = self.input.flat_counter_at(key, location);
        let output_charge = footprint(&output);
        if (counter_view(key).is_some()) && output_charge.is_some() {
            let expected = output;
            let key = key.to_owned();
            self.journal.index(
                counter_view(&key).and_then(|v| footprint(&v)).unwrap_or(0)
                    + output_charge.unwrap_or(0)
                    + 128,
                move |input| input.flat_counter_at(&key, location) == expected,
            );
        } else {
            self.journal.decline("flat_counter_at");
        }
        output
    }
    fn resolved_label_for(&self, label: &Label) -> Option<&str> {
        let output = self.input.resolved_label_for(label);
        let output_charge = footprint(&output);
        if (label.0.len() <= 16384) && output_charge.is_some() {
            let expected = output.map(str::to_owned);
            let label = label.to_owned();
            self.journal.index(
                output_charge.unwrap_or(0) + 128 + label.0.len(),
                move |input| {
                    input.resolved_label_for(&label).map(str::to_owned) == expected
                },
            );
        } else {
            self.journal.decline("resolved_label_for");
        }
        output
    }
    fn headings_for_toc(
        &self,
    ) -> &[(Label, Option<String>, crate::entities::content::Content, usize)] {
        let output = self.input.headings_for_toc();
        if output.is_empty() {
            let expected = true;
            self.journal
                .index(128, move |input| input.headings_for_toc().is_empty() == expected);
        } else {
            self.journal.decline("headings_for_toc");
        }
        output
    }
    fn headings_for_bookmarks(
        &self,
    ) -> &[(Label, Option<String>, crate::entities::content::Content, usize)] {
        let output = self.input.headings_for_bookmarks();
        if output.is_empty() {
            let expected = true;
            self.journal.index(128, move |input| {
                input.headings_for_bookmarks().is_empty() == expected
            });
        } else {
            self.journal.decline("headings_for_bookmarks");
        }
        output
    }
    fn query_labelled(&self) -> Vec<(Label, Location)> {
        let output = self.input.query_labelled();
        let output_charge = footprint(&output);
        if output_charge.is_some() {
            let expected = output.clone();
            self.journal.index(output_charge.unwrap_or(0) + 128, move |input| {
                input.query_labelled() == expected
            });
        } else {
            self.journal.decline("query_labelled");
        }
        output
    }
    fn label_count(&self, label: &Label) -> usize {
        let output = self.input.label_count(label);
        let output_charge = footprint(&output);
        if (label.0.len() <= 16384) && output_charge.is_some() {
            let expected = output;
            let label = label.to_owned();
            self.journal
                .index(output_charge.unwrap_or(0) + 128 + label.0.len(), move |input| {
                    input.label_count(&label) == expected
                });
        } else {
            self.journal.decline("label_count");
        }
        output
    }
    fn pages(&self, location: Location) -> Option<std::num::NonZeroUsize> {
        let output = self.input.pages(location);
        let output_charge = footprint(&output);
        if output_charge.is_some() {
            let expected = output;
            self.journal.index(output_charge.unwrap_or(0) + 128, move |input| {
                input.pages(location) == expected
            });
        } else {
            self.journal.decline("pages");
        }
        output
    }
    fn page(&self, location: Location) -> Option<std::num::NonZeroUsize> {
        let output = self.input.page(location);
        let output_charge = footprint(&output);
        if output_charge.is_some() {
            let expected = output;
            self.journal.index(output_charge.unwrap_or(0) + 128, move |input| {
                input.page(location) == expected
            });
        } else {
            self.journal.decline("page");
        }
        output
    }
    fn page_numbering(
        &self,
        location: Location,
    ) -> Option<&crate::entities::numbering::Numbering> {
        let output = self.input.page_numbering(location);
        if numbering_view(output).is_some() {
            let expected = numbering_view(output);
            self.journal
                .index(footprint(&expected).unwrap_or(64 * 1024) + 128, move |input| {
                    numbering_view(input.page_numbering(location)) == expected
                });
        } else {
            self.journal.decline("page_numbering");
        }
        output
    }
    fn page_supplement(
        &self,
        location: Location,
    ) -> Option<&crate::entities::content::Content> {
        let output = self.input.page_supplement(location);
        if output.is_none() {
            let expected = true;
            self.journal.index(128, move |input| {
                input.page_supplement(location).is_none() == expected
            });
        } else {
            self.journal.decline("page_supplement");
        }
        output
    }
}

#[cfg(test)]
mod physical_retention_index_tests {
    use super::*;
    use crate::{compiler::layout::FixedMetrics, entities::content::Content};
    #[test]
    fn physical_retention_index_validates_actual_reads_not_unread_data() {
        let journal = Journal::default();
        let mut input = TagIntrospector::empty();
        input
            .kind_index
            .insert(ElementKind::Heading, vec![Location::from_raw(1)]);
        let recorded = RecordingIntrospector { input: &input, journal: &journal };
        assert_eq!(
            recorded.query_by_kind(ElementKind::Heading),
            vec![Location::from_raw(1)]
        );
        assert!(journal.validates(&FixedMetrics, &input));
        input
            .kind_index
            .insert(ElementKind::Outline, vec![Location::from_raw(2)]);
        assert!(journal.validates(&FixedMetrics, &input));
        input
            .kind_index
            .insert(ElementKind::Heading, vec![Location::from_raw(3)]);
        assert!(!journal.validates(&FixedMetrics, &input));
    }
    #[test]
    fn physical_retention_index_content_hash_equality_is_never_a_certificate() {
        let journal = Journal::default();
        let mut input = TagIntrospector::empty();
        let key = ("s".to_string(), Location::from_raw(1));
        input.state_displays.insert(key.clone(), Content::Space);
        let recorded = RecordingIntrospector { input: &input, journal: &journal };
        assert!(matches!(
            recorded.state_display_value(key.0.clone(), key.1),
            Some(Content::Space)
        ));
        assert!(!journal.validates(&FixedMetrics, &input));
        input.state_displays.insert(key, Content::SpaceFromNewline);
        assert!(!journal.validates(&FixedMetrics, &input));
    }
    #[test]
    fn physical_retention_index_absence_invalidates_when_content_appears() {
        let journal = Journal::default();
        let mut input = TagIntrospector::empty();
        let key = ("s".to_string(), Location::from_raw(1));
        let recorded = RecordingIntrospector { input: &input, journal: &journal };
        assert!(recorded.state_display_value(key.0.clone(), key.1).is_none());
        assert!(journal.validates(&FixedMetrics, &input));
        input.state_displays.insert(key, Content::Space);
        assert!(!journal.validates(&FixedMetrics, &input));
    }
}

#[cfg(test)]
mod physical_retention_index_budget_tests {
    use super::*;
    use crate::compiler::layout::FixedMetrics;
    #[test]
    fn physical_retention_large_query_is_returned_but_not_retained() {
        let journal = Journal::default();
        let mut input = TagIntrospector::empty();
        input
            .kind_index
            .insert(ElementKind::Heading, (1..10001).map(Location::from_raw).collect());
        let recorder = RecordingIntrospector { input: &input, journal: &journal };
        assert_eq!(recorder.query_by_kind(ElementKind::Heading).len(), 10000);
        assert!(!journal.validates(&FixedMetrics, &input));
    }
}

#[cfg(test)]
mod physical_retention_element_at_tests {
    use super::*;
    use crate::compiler::layout::FixedMetrics;
    use crate::entities::{
        content::Content, elements::heading::HeadingElem, value::IntrospectedContent,
    };
    use std::sync::Arc;
    fn heading(body: Content) -> Arc<HeadingElem> {
        Arc::new(HeadingElem::new(1, body))
    }
    fn index(h: Arc<HeadingElem>) -> TagIntrospector {
        let mut i = TagIntrospector::empty();
        i.elements.insert(
            Location::from_raw(1),
            IntrospectedContent::new(Content::Heading(h), None),
        );
        i
    }
    fn record(i: &TagIntrospector) -> Journal {
        let j = Journal::default();
        let r = RecordingIntrospector { input: i, journal: &j };
        assert!(r.element_at(Location::from_raw(1)).is_some());
        j
    }
    #[test]
    fn same_heading_owner_validates_and_unread_data_is_ignored() {
        let h = heading(Content::text("Original"));
        let mut i = index(h.clone());
        let j = record(&i);
        assert!(
            j.validates(&FixedMetrics, &i),
            "present same-owner heading must be certifiable"
        );
        i.elements.insert(
            Location::from_raw(2),
            IntrospectedContent::new(Content::Space, None),
        );
        assert!(j.validates(&FixedMetrics, &i));
        if let Some(Content::Heading(actual)) = i.element_at(Location::from_raw(1)) {
            assert!(Arc::ptr_eq(&h, actual));
        } else {
            panic!("original output changed")
        }
    }
    #[test]
    fn equal_new_origin_changed_fields_and_removal_invalidate() {
        let h = heading(Content::text("Original"));
        let i = index(h.clone());
        let j = record(&i);
        assert!(j.validates(&FixedMetrics, &i));
        let mut bodies = vec![
            (*h).clone(),
            HeadingElem::new(2, Content::text("Original")),
            HeadingElem::new(1, Content::text("Changed")),
        ];
        let mut changed = (*h).clone();
        changed.outlined = false;
        bodies.push(changed);
        let mut changed = (*h).clone();
        changed.bookmarked = Some(false);
        bodies.push(changed);
        let mut changed = (*h).clone();
        changed.set_fields = 0;
        bodies.push(changed);
        for body in bodies {
            assert!(
                !j.validates(&FixedMetrics, &index(Arc::new(body))),
                "new owner must miss even with equal fields"
            );
        }
        assert!(!j.validates(&FixedMetrics, &TagIntrospector::empty()));
        let mut variant = TagIntrospector::empty();
        variant.elements.insert(
            Location::from_raw(1),
            IntrospectedContent::new(Content::Space, None),
        );
        assert!(!j.validates(&FixedMetrics, &variant));
    }
    #[test]
    fn present_observation_does_not_keep_heading_alive() {
        let h = heading(Content::text("Original"));
        let weak = Arc::downgrade(&h);
        let i = index(h);
        let j = record(&i);
        assert!(j.validates(&FixedMetrics, &i));
        drop(i);
        assert!(weak.upgrade().is_none(), "journal must not retain the body graph");
        assert!(!j.validates(&FixedMetrics, &index(heading(Content::text("Original")))));
    }
    #[test]
    fn absence_to_presence_and_unsupported_bodies_stay_conservative() {
        let i = TagIntrospector::empty();
        let j = Journal::default();
        let r = RecordingIntrospector { input: &i, journal: &j };
        assert!(r.element_at(Location::from_raw(1)).is_none());
        assert!(j.validates(&FixedMetrics, &i));
        assert!(!j.validates(&FixedMetrics, &index(heading(Content::text("X")))));
        for body in [
            Content::heading(1, Content::text("nested")),
            Content::text("x".repeat(16385)),
            Content::sequence(vec![Content::Space; 129]),
        ] {
            let i = index(heading(body));
            let j = record(&i);
            assert!(!j.validates(&FixedMetrics, &i));
        }
        let mut body = Content::text("X");
        for _ in 0..10 {
            body = Content::sequence(vec![body, Content::Space]);
        }
        let i = index(heading(body));
        assert!(!record(&i).validates(&FixedMetrics, &i));
    }
    #[test]
    fn plain_nested_body_and_journal_budget_are_checked() {
        let i = index(heading(Content::sequence(vec![
            Content::text("X"),
            Content::Space,
            Content::text("Y"),
        ])));
        let j = record(&i);
        assert!(j.validates(&FixedMetrics, &i));
        let j = Journal::default();
        let r = RecordingIntrospector { input: &i, journal: &j };
        for _ in 0..16385 {
            assert!(r.element_at(Location::from_raw(1)).is_some());
        }
        assert!(!j.validates(&FixedMetrics, &i), "old journal bound remains enforced");
    }
}
