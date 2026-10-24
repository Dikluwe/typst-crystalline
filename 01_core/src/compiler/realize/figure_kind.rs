//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/realize/figure_kind.md
//! @prompt-hash 740e13a1
//! @layer L1
//! @updated 2026-10-19

use crate::entities::content::Content;
use crate::entities::elements::figure::{FigureKind, FigureSupplement};
use crate::entities::elements::grid::{GridChildOrder, GridElem};
use crate::entities::func::{Func, FuncRepr};
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;

pub(in crate::compiler) fn kind_from_value(
    value: &Value,
    span: Span,
) -> SourceResult<Option<FigureKind>> {
    if matches!(value, Value::Auto) {
        return Ok(None);
    }
    if let Value::Str(name) = value {
        return Ok(Some(FigureKind::Name(name.clone())));
    }
    if let Value::Func(function) = value {
        if matches!(function.repr(), FuncRepr::Element(_))
            || crate::compiler::eval::bindings::native_element_fields(function).is_some()
        {
            return Ok(Some(FigureKind::Element(function.clone())));
        }
        return Err(vec![SourceDiagnostic::error(span, "expected element")]);
    }
    if let Value::Type(ty) = value {
        let message = match ty {
            crate::entities::value::Type::Int
            | crate::entities::value::Type::Float
            | crate::entities::value::Type::Str
            | crate::entities::value::Type::Array
            | crate::entities::value::Type::Dictionary
            | crate::entities::value::Type::Datetime
            | crate::entities::value::Type::Stroke
            | crate::entities::value::Type::Regex
            | crate::entities::value::Type::Tiling
            | crate::entities::value::Type::Bytes
            | crate::entities::value::Type::Decimal
            | crate::entities::value::Type::Duration
            | crate::entities::value::Type::Version
            | crate::entities::value::Type::Selector
            | crate::entities::value::Type::Symbol
            | crate::entities::value::Type::Arguments
            | crate::entities::value::Type::State
            | crate::entities::value::Type::Counter
            | crate::entities::value::Type::Label
            | crate::entities::value::Type::Path
            | crate::entities::value::Type::Type
            | crate::entities::value::Type::SpotColorant => {
                "expected element".to_string()
            }
            crate::entities::value::Type::Bool => {
                "type boolean does not have a constructor".to_string()
            }
            crate::entities::value::Type::Relative => {
                "type relative length does not have a constructor".to_string()
            }
            crate::entities::value::Type::None
            | crate::entities::value::Type::Auto
            | crate::entities::value::Type::Module
            | crate::entities::value::Type::Function
            | crate::entities::value::Type::Content
            | crate::entities::value::Type::Length
            | crate::entities::value::Type::Ratio
            | crate::entities::value::Type::Angle
            | crate::entities::value::Type::Color
            | crate::entities::value::Type::Fraction
            | crate::entities::value::Type::Alignment
            | crate::entities::value::Type::Gradient
            | crate::entities::value::Type::Direction
            | crate::entities::value::Type::Location => {
                format!("type {} does not have a constructor", ty.name())
            }
        };
        return Err(vec![SourceDiagnostic::error(span, message)]);
    }
    if let Value::Symbol(symbol) = value {
        let value = symbol.value.as_str();
        let callable = crate::compiler::stdlib::is_known_math_accent(value)
            || crate::compiler::stdlib::is_callable_lr_symbol(value);
        let message = if callable {
            "expected element".to_string()
        } else {
            format!("symbol {value} is not callable")
        };
        return Err(vec![SourceDiagnostic::error(span, message)]);
    }
    Err(vec![SourceDiagnostic::error(
        span,
        format!(
            "expected function, string, or auto, found {}",
            crate::compiler::eval::operators::error_formatting::vanilla_type_name(value)
        ),
    )])
}

pub(in crate::compiler) fn infer(body: &Content) -> SourceResult<FigureKind> {
    Ok(inspect(body, None, Span::detached(), false)?
        .map(|(kind, _)| kind)
        .unwrap_or_else(|| {
            FigureKind::Element(Func::native(
                "image",
                crate::compiler::stdlib::native_image,
            ))
        }))
}

pub(in crate::compiler) fn find(
    body: &Content,
    kind: &FigureKind,
) -> SourceResult<Option<Content>> {
    match kind {
        FigureKind::Name(_) => {
            validate_body(body)?;
            Ok(Some(body.clone()))
        }
        FigureKind::Element(_) => Ok(inspect(body, Some(kind), Span::detached(), false)?
            .map(|(_, content)| content)),
    }
}

pub(in crate::compiler) fn validate_body(body: &Content) -> SourceResult<()> {
    inspect(body, None, Span::detached(), true).map(|_| ())
}

fn root_kind(content: &Content) -> Option<FigureKind> {
    if matches!(content, Content::Table(_)) {
        Some(FigureKind::Element(Func::native(
            "table",
            crate::compiler::stdlib::native_table,
        )))
    } else if matches!(content, Content::Raw(_)) {
        Some(FigureKind::Element(Func::native(
            "raw",
            crate::compiler::stdlib::native_raw,
        )))
    } else if matches!(content, Content::Image(_)) {
        Some(FigureKind::Element(Func::native(
            "image",
            crate::compiler::stdlib::native_image,
        )))
    } else {
        None
    }
}

fn inspect(
    content: &Content,
    wanted: Option<&FigureKind>,
    span: Span,
    validate_only: bool,
) -> SourceResult<Option<(FigureKind, Content)>> {
    if let Content::Source(source) = content {
        return inspect(&source.body, wanted, source.span, validate_only);
    }
    if let Content::ShowRecipe(recipe) = content {
        return inspect(&recipe.body, wanted, span, validate_only);
    }
    if let Content::Located(located) = content {
        return inspect(&located.body, wanted, span, validate_only);
    }
    if let Content::Realized(realized) = content {
        return inspect(&realized.original, wanted, span, validate_only);
    }
    if let Content::Grid(grid) = content {
        validate_grid(grid, span)?;
    }
    if !validate_only {
        if let Some(found) = matching_root(content, wanted) {
            return Ok(Some(found));
        }
    }
    if let Content::Metadata(metadata) = content {
        return inspect_value(&metadata.value, wanted, span, validate_only);
    }
    for child in semantic_children(content, span)? {
        if let Some(found) = inspect(&child, wanted, span, validate_only)? {
            return Ok(Some(found));
        }
    }
    Ok(None)
}

fn matching_root(
    content: &Content,
    wanted: Option<&FigureKind>,
) -> Option<(FigureKind, Content)> {
    if let Some(kind) = wanted {
        match kind {
            FigureKind::Element(function) => {
                if crate::compiler::eval::selector_matching::native_element_matches(
                    content, function,
                ) {
                    return Some((kind.clone(), content.clone()));
                }
            }
            FigureKind::Name(_) => return Some((kind.clone(), content.clone())),
        }
        None
    } else {
        root_kind(content).map(|kind| (kind, content.clone()))
    }
}

fn inspect_value(
    value: &Value,
    wanted: Option<&FigureKind>,
    span: Span,
    validate_only: bool,
) -> SourceResult<Option<(FigureKind, Content)>> {
    match value {
        Value::Content(content) => inspect(content, wanted, span, validate_only),
        Value::LocatedContent(content, _) => {
            if let Some(fields) = content.fields() {
                let mut root = content.content();
                let mut span = span;
                loop {
                    if let Content::Source(e) = root {
                        root = &e.body;
                        span = e.span;
                    } else if let Content::Located(e) = root {
                        root = &e.body;
                    } else if let Content::ShowRecipe(e) = root {
                        root = &e.body;
                    } else if let Content::Realized(e) = root {
                        root = &e.original;
                    } else {
                        break;
                    }
                }
                if let Content::Grid(grid) = root {
                    validate_grid(grid, span)?;
                }
                if !validate_only {
                    if let Some(found) = matching_root(root, wanted) {
                        return Ok(Some(found));
                    }
                }
                for value in fields.values() {
                    if let Some(found) =
                        inspect_value(value, wanted, span, validate_only)?
                    {
                        return Ok(Some(found));
                    }
                }
                Ok(None)
            } else {
                inspect(content.content(), wanted, span, validate_only)
            }
        }
        Value::Array(values) => {
            for value in values.iter() {
                if let Some(found) = inspect_value(value, wanted, span, validate_only)? {
                    return Ok(Some(found));
                }
            }
            Ok(None)
        }
        Value::None
        | Value::Bool(..)
        | Value::Int(..)
        | Value::Float(..)
        | Value::Str(..)
        | Value::Dict(..)
        | Value::Module(..)
        | Value::Datetime(..)
        | Value::Func(..)
        | Value::Auto
        | Value::Length(..)
        | Value::Relative(..)
        | Value::Ratio(..)
        | Value::Angle(..)
        | Value::Color(..)
        | Value::Stroke(..)
        | Value::Fraction(..)
        | Value::Align(..)
        | Value::Location(..)
        | Value::Gradient(..)
        | Value::Regex(..)
        | Value::Tiling(..)
        | Value::Bytes(..)
        | Value::Decimal(..)
        | Value::Duration(..)
        | Value::Version(..)
        | Value::Selector(..)
        | Value::Symbol(..)
        | Value::Args(..)
        | Value::State(..)
        | Value::Counter(..)
        | Value::Label(..)
        | Value::Dir(..)
        | Value::Path(..)
        | Value::Type(..)
        | Value::SpotColorant(..) => Ok(None),
    }
}

fn semantic_children(content: &Content, span: Span) -> SourceResult<Vec<Content>> {
    let mut children = Vec::new();
    match content {
        Content::Source(e) => children.push(e.body.clone()),
        Content::ShowRecipe(e) => children.push(e.body.clone()),
        Content::Located(e) => children.push(e.body.clone()),
        Content::Realized(e) => children.push(e.original.clone()),
        Content::Heading(e) => children.push(e.body.clone()),
        Content::Title(e) => children.push(e.body.clone()),
        Content::Strong(e) => children.push(e.body.clone()),
        Content::Emph(e) => children.push(e.body.clone()),
        Content::ListItem(e) => children.push(e.body.clone()),
        Content::EnumItem(e) => children.push(e.body.clone()),
        Content::Link(e) => children.push(e.body.clone()),
        Content::Equation(e) => children.push(e.body.clone()),
        Content::MathStretch(e) => children.push(e.body.clone()),
        Content::MathDelimited(e) => children.push(e.body.clone()),
        Content::MathLr(e) => children.push(e.body.clone()),
        Content::MathMid(e) => children.push(e.body.clone()),
        Content::MathCancel(e) => children.push(e.body.clone()),
        Content::MathUnderline(e) => children.push(e.body.clone()),
        Content::MathOverline(e) => children.push(e.body.clone()),
        Content::MathClassOverride(e) => children.push(e.body.clone()),
        Content::MathLimitsOverride(e) => children.push(e.body.clone()),
        Content::MathStyled(e) => children.push(e.body.clone()),
        Content::Label(e) => children.push(e.body.clone()),
        Content::Transform(e) => children.push(e.body.clone()),
        Content::GridHeader(e) => children.push(e.body.clone()),
        Content::GridFooter(e) => children.push(e.body.clone()),
        Content::GridCell(e) => children.push(e.body.clone()),
        Content::Align(e) => children.push(e.body.clone()),
        Content::Place(e) => children.push(e.body.clone()),
        Content::Styled(e) => children.push(e.body.clone()),
        Content::PdfArtifact(e) => children.push(e.body.clone()),
        Content::Underline(e) => children.push(e.body.clone()),
        Content::Strike(e) => children.push(e.body.clone()),
        Content::Overline(e) => children.push(e.body.clone()),
        Content::Pad(e) => children.push(e.body.clone()),
        Content::Hide(e) => children.push(e.body.clone()),
        Content::Boxed(e) => children.push(e.body.clone()),
        Content::Block(e) => children.push(e.body.clone()),
        Content::Footnote(e) => children.push(e.body.clone()),
        Content::TableHeader(e) => children.push(e.body.clone()),
        Content::TableFooter(e) => children.push(e.body.clone()),
        Content::Repeat(e) => children.push(e.body.clone()),
        Content::Columns(e) => children.push(e.body.clone()),
        Content::HtmlFrame(e) => children.push(e.body.clone()),
        Content::Par { body } | Content::SmallCaps { body } => {
            children.push((**body).clone())
        }
        Content::Sequence(parts) | Content::MathSequence(parts) => {
            children.extend(parts.iter().cloned())
        }
        Content::HtmlElem(e) => {
            if let crate::entities::html::HtmlBody::Content(body) = &e.body {
                children.push((**body).clone());
            }
        }
        Content::MathFrac(e) => {
            children.push(e.num.clone());
            children.push(e.den.clone());
        }
        Content::MathAttach(e) => {
            children.push(e.base.clone());
            for slot in [&e.t, &e.b, &e.tl, &e.bl, &e.tr, &e.br] {
                if let crate::entities::elements::math_attach::MathAttachSlot::Present(
                    c,
                ) = slot
                {
                    children.push(c.clone());
                }
            }
        }
        Content::MathRoot(e) => {
            children.extend(e.index.iter().cloned());
            children.push(e.radicand.clone());
        }
        Content::MathOverbrace(e) => {
            children.push(e.body.clone());
            if e.annotation_explicit {
                children.extend(e.annotation.iter().cloned());
            }
        }
        Content::MathUnderbrace(e) => {
            children.push(e.body.clone());
            if e.annotation_explicit {
                children.extend(e.annotation.iter().cloned());
            }
        }
        Content::MathUnderbracket(e) => {
            children.push(e.body.clone());
            if e.annotation_explicit {
                children.extend(e.annotation.iter().cloned());
            }
        }
        Content::MathUnderparen(e) => {
            children.push(e.body.clone());
            if e.annotation_explicit {
                children.extend(e.annotation.iter().cloned());
            }
        }
        Content::MathUndershell(e) => {
            children.push(e.body.clone());
            if e.annotation_explicit {
                children.extend(e.annotation.iter().cloned());
            }
        }
        Content::MathOverbracket(e) => {
            children.push(e.body.clone());
            if e.annotation_explicit {
                children.extend(e.annotation.iter().cloned());
            }
        }
        Content::MathOverparen(e) => {
            children.push(e.body.clone());
            if e.annotation_explicit {
                children.extend(e.annotation.iter().cloned());
            }
        }
        Content::MathOvershell(e) => {
            children.push(e.body.clone());
            if e.annotation_explicit {
                children.extend(e.annotation.iter().cloned());
            }
        }
        Content::MathMatrix(e) => {
            for row in &e.rows {
                children.extend(row.iter().cloned());
            }
        }
        Content::MathCases(e) => children.extend(e.children.iter().cloned()),
        Content::MathVec(e) => children.extend(e.children.iter().cloned()),
        Content::MathAccent(e) => children.push(e.base.clone()),
        Content::MathUnderover(e) => {
            children.push(e.base.clone());
            children.extend(e.under.iter().cloned());
            children.extend(e.over.iter().cloned());
        }
        Content::MathOp(e) => children.push(e.text.clone()),
        Content::Figure(e) => {
            children.push(e.body.clone());
            children.extend(e.caption.iter().cloned());
            if let Some(supplement) = &e.resolved_supplement {
                children.extend(supplement.iter().cloned());
            } else if let Some(FigureSupplement::Content(supplement)) = &e.supplement {
                children.push(supplement.clone());
            }
        }
        Content::FigureCaption(e) => {
            children.extend(e.separator.iter().cloned());
            children.push(e.body.clone());
        }
        Content::Ref(e) => children.extend(e.supplement.iter().cloned()),
        Content::Cite(e) => children.extend(e.supplement.iter().cloned()),
        Content::Outline(e) => children.extend(e.title.iter().cloned()),
        Content::OutlineEntry(e) => {
            children.push(e.element.clone());
            children.extend(e.fill.iter().cloned());
        }
        Content::Grid(e) => {
            validate_grid(e, span)?;
            if let Some(order) = &e.source_order {
                for descriptor in order {
                    match descriptor {
                        GridChildOrder::Cell(index) => {
                            children.push(e.cells[*index].clone())
                        }
                        GridChildOrder::Header => {
                            children.extend(e.header.iter().cloned())
                        }
                        GridChildOrder::Footer => {
                            children.extend(e.footer.iter().cloned())
                        }
                        GridChildOrder::HLine(index) => {
                            children.push(Content::GridHLine(std::sync::Arc::new(
                                e.hlines[*index].clone(),
                            )))
                        }
                        GridChildOrder::VLine(index) => {
                            children.push(Content::GridVLine(std::sync::Arc::new(
                                e.vlines[*index].clone(),
                            )))
                        }
                    }
                }
            } else {
                children.extend(e.cells.iter().cloned());
                children.extend(e.header.iter().cloned());
                children.extend(e.footer.iter().cloned());
            }
        }
        Content::PageRun(e) => {
            if let Some(crate::entities::page_supplement::PageSupplement::Content(c)) =
                &e.supplement
            {
                children.push((**c).clone());
            }
            for marginal in [&e.header, &e.footer] {
                if let Some(crate::entities::page_running::PageMarginal::Content(c)) =
                    marginal
                {
                    children.push((**c).clone());
                }
            }
            for layer in [&e.background, &e.foreground] {
                if let Some(Some(c)) = layer {
                    children.push(c.clone());
                }
            }
            children.push(e.body.clone());
        }
        Content::Terms(e) => children.extend(e.items.iter().cloned()),
        Content::TermItem(e) => {
            children.push(e.term.clone());
            children.push(e.description.clone());
        }
        Content::Quote(e) => {
            children.push(e.body.clone());
            children.extend(e.attribution.iter().cloned());
        }
        Content::Document { title, body, .. } => {
            children.extend(title.iter().map(|c| (**c).clone()));
            children.push((**body).clone());
        }
        Content::Stack(e) => children.extend(e.children.iter().cloned()),
        Content::TableCell(e) => children.push(e.body.clone()),
        Content::Table(e) => {
            children.extend(e.children.iter().cloned());
            children.extend(e.header.iter().cloned());
            children.extend(e.footer.iter().cloned());
            children.extend(e.caption.iter().cloned());
        }
        Content::Bibliography(e) => children.extend(e.title.iter().cloned()),
        Content::FootnoteEntry(e) => {
            children.push(e.note.clone());
            children.extend(e.separator.iter().cloned());
        }
        Content::Metadata(_) => {} // Values are inspected by inspect_value before this child catalogue.
        Content::Image(..)
        | Content::Raw(..)
        | Content::StateUpdate(..)
        | Content::State(..)
        | Content::ContextBlock(..)
        | Content::StateDisplay(..)
        | Content::CounterDisplayCallback(..)
        | Content::Dynamic(..)
        | Content::SetPage(..)
        | Content::Empty
        | Content::Text(..)
        | Content::Space
        | Content::SpaceFromNewline
        | Content::Parbreak
        | Content::Symbol(..)
        | Content::MathIdent(..)
        | Content::MathText(..)
        | Content::MathPrimes(..)
        | Content::MathAlignPoint(..)
        | Content::Linebreak(..)
        | Content::CounterDisplay(..)
        | Content::CounterUpdate(..)
        | Content::Shape(..)
        | Content::Curve(..)
        | Content::Flush(..)
        | Content::Divider(..)
        | Content::Asset { .. }
        | Content::SmartQuote(..)
        | Content::PdfAttach(..)
        | Content::HSpace(..)
        | Content::VSpace(..)
        | Content::Pagebreak(..)
        | Content::Colbreak(..)
        | Content::GridHLine(..)
        | Content::GridVLine(..)
        | Content::TableHLine(..)
        | Content::TableVLine(..) => {}
    }
    Ok(children)
}

fn validate_grid(grid: &GridElem, span: Span) -> SourceResult<()> {
    let Some(order) = &grid.source_order else {
        return Ok(());
    };
    let mut cells = vec![false; grid.cells.len()];
    let mut hlines = vec![false; grid.hlines.len()];
    let mut vlines = vec![false; grid.vlines.len()];
    let mut header = false;
    let mut footer = false;
    let mut invalid = false;
    for descriptor in order {
        let seen = match descriptor {
            GridChildOrder::Cell(index) => cells.get_mut(*index),
            GridChildOrder::HLine(index) => hlines.get_mut(*index),
            GridChildOrder::VLine(index) => vlines.get_mut(*index),
            GridChildOrder::Header if grid.header.is_some() => Some(&mut header),
            GridChildOrder::Footer if grid.footer.is_some() => Some(&mut footer),
            GridChildOrder::Header | GridChildOrder::Footer => None,
        };
        if let Some(seen) = seen {
            invalid |= *seen;
            *seen = true;
        } else {
            invalid = true;
        }
    }
    invalid |= cells
        .iter()
        .chain(hlines.iter())
        .chain(vlines.iter())
        .any(|seen| !seen);
    invalid |= header != grid.header.is_some() || footer != grid.footer.is_some();
    if invalid {
        Err(vec![SourceDiagnostic::error(span, "invalid grid source order")])
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    fn table() -> Content {
        Content::table(Vec::new(), Vec::new(), vec![Content::text("cell")])
    }
    fn table_kind() -> FigureKind {
        FigureKind::Element(Func::native("table", crate::compiler::stdlib::native_table))
    }
    fn raw_kind() -> FigureKind {
        FigureKind::Element(Func::native("raw", crate::compiler::stdlib::native_raw))
    }
    fn image_kind() -> FigureKind {
        FigureKind::Element(Func::native("image", crate::compiler::stdlib::native_image))
    }
    fn grid() -> GridElem {
        GridElem {
            columns: vec![],
            rows: vec![],
            cells: vec![Content::raw("raw", None, false)],
            hlines: vec![],
            vlines: vec![],
            gutter: None,
            align: None,
            inset: crate::entities::sides::Sides::uniform(
                crate::entities::layout_types::Length::ZERO,
            ),
            header: Some(table()),
            footer: None,
            stroke: None,
            fill: None,
            source_order: Some(vec![GridChildOrder::Header, GridChildOrder::Cell(0)]),
        }
    }

    #[test]
    fn figure_kind_metadata_arrays_and_authoritative_snapshot_fields() {
        let nested = Content::metadata(Value::Array(vec![
            Value::Int(9),
            Value::Array(vec![Value::Content(table())]),
        ]));
        assert_eq!(infer(&nested).unwrap(), table_kind());
        let mut dictionary = indexmap::IndexMap::default();
        dictionary.insert("body".into(), Value::Content(table()));
        assert_eq!(
            infer(&Content::metadata(Value::Dict(dictionary))).unwrap(),
            image_kind()
        );
        let hidden = Content::strong(table());
        let known_empty = crate::entities::value::IntrospectedContent::new(
            hidden.clone(),
            Some(indexmap::IndexMap::default()),
        );
        assert_eq!(
            infer(&Content::metadata(Value::LocatedContent(
                known_empty,
                crate::entities::location::Location::from_raw(55)
            )))
            .unwrap(),
            image_kind()
        );
        let unknown = crate::entities::value::IntrospectedContent::new(hidden, None);
        assert_eq!(
            infer(&Content::metadata(Value::LocatedContent(
                unknown,
                crate::entities::location::Location::from_raw(56)
            )))
            .unwrap(),
            table_kind()
        );
    }

    #[test]
    fn figure_kind_annotations_respect_explicit_presence() {
        for explicit in [false, true] {
            for content in [
                Content::MathOverbrace(Arc::new(
                    crate::entities::elements::math_underover::MathOverbraceElem {
                        body: Content::text("body"),
                        annotation: Some(table()),
                        annotation_explicit: explicit,
                    },
                )),
                Content::MathUnderbrace(Arc::new(
                    crate::entities::elements::math_underover::MathUnderbraceElem {
                        body: Content::text("body"),
                        annotation: Some(table()),
                        annotation_explicit: explicit,
                    },
                )),
                Content::MathUnderbracket(Arc::new(
                    crate::entities::elements::math_underover::MathUnderbracketElem {
                        body: Content::text("body"),
                        annotation: Some(table()),
                        annotation_explicit: explicit,
                    },
                )),
                Content::MathUnderparen(Arc::new(
                    crate::entities::elements::math_underover::MathUnderparenElem {
                        body: Content::text("body"),
                        annotation: Some(table()),
                        annotation_explicit: explicit,
                    },
                )),
                Content::MathUndershell(Arc::new(
                    crate::entities::elements::math_underover::MathUndershellElem {
                        body: Content::text("body"),
                        annotation: Some(table()),
                        annotation_explicit: explicit,
                    },
                )),
                Content::MathOverbracket(Arc::new(
                    crate::entities::elements::math_underover::MathOverbracketElem {
                        body: Content::text("body"),
                        annotation: Some(table()),
                        annotation_explicit: explicit,
                    },
                )),
                Content::MathOverparen(Arc::new(
                    crate::entities::elements::math_underover::MathOverparenElem {
                        body: Content::text("body"),
                        annotation: Some(table()),
                        annotation_explicit: explicit,
                    },
                )),
                Content::MathOvershell(Arc::new(
                    crate::entities::elements::math_underover::MathOvershellElem {
                        body: Content::text("body"),
                        annotation: Some(table()),
                        annotation_explicit: explicit,
                    },
                )),
            ] {
                assert_eq!(
                    infer(&content).unwrap(),
                    if explicit { table_kind() } else { image_kind() },
                    "annotation presence {explicit}: {content:?}"
                );
            }
        }
    }

    #[test]
    fn figure_kind_original_semantics_and_opaque_state_update() {
        let shown =
            Content::Realized(Arc::new(crate::entities::content::RealizedContent {
                original: Content::raw("raw", None, false),
                visual: table(),
            }));
        assert_eq!(infer(&shown).unwrap(), raw_kind());
        let update = Content::StateUpdate(Arc::new(
            crate::entities::elements::state_update::StateUpdateElem {
                key: "key".into(),
                update: crate::entities::state_update::StateUpdate::Set(Box::new(
                    Value::Content(table()),
                )),
            },
        ));
        assert_eq!(infer(&update).unwrap(), image_kind());
    }

    #[test]
    fn figure_kind_ref_fields_and_first_exact_element_target() {
        assert_eq!(
            infer(&Content::reference_with_supplement("unresolved", Some(table())))
                .unwrap(),
            table_kind()
        );
        let first = Content::heading(1, Content::text("first"));
        let second = Content::heading(1, Content::text("second"));
        let body = Content::table(vec![], vec![], vec![first.clone(), second]);
        let heading = FigureKind::Element(Func::native(
            "heading-alias",
            crate::compiler::stdlib::native_heading,
        ));
        assert_eq!(find(&body, &heading).unwrap(), Some(first));
        assert_eq!(infer(&body).unwrap(), table_kind());
        let homonym = FigureKind::Element(Func::native(
            "heading",
            crate::compiler::stdlib::native_table,
        ));
        assert_eq!(find(&body, &homonym).unwrap(), Some(body.clone()));
        assert_eq!(find(&body, &FigureKind::Name("table".into())).unwrap(), Some(body));
    }

    #[test]
    fn figure_kind_grid_source_order_is_not_storage_order() {
        let mut value = grid();
        assert_eq!(infer(&Content::Grid(Arc::new(value.clone()))).unwrap(), table_kind());
        value.source_order = Some(vec![GridChildOrder::Cell(0), GridChildOrder::Header]);
        assert_eq!(infer(&Content::Grid(Arc::new(value.clone()))).unwrap(), raw_kind());
        value.source_order = None;
        assert_eq!(infer(&Content::Grid(Arc::new(value))).unwrap(), raw_kind());
    }

    #[test]
    fn figure_kind_grid_explicit_line_targets_follow_descriptor_order() {
        let mut value = grid();
        let hline = crate::entities::elements::grid_hline::GridHLineElem {
            start: 0,
            end: None,
            row: 0,
            stroke: None,
            position: "top".into(),
        };
        let vline = crate::entities::elements::grid_vline::GridVLineElem {
            start: 0,
            end: None,
            col: 0,
            stroke: None,
            position: "left".into(),
        };
        value.hlines = vec![
            hline.clone(),
            crate::entities::elements::grid_hline::GridHLineElem {
                row: 1,
                ..hline.clone()
            },
        ];
        value.vlines = vec![vline.clone()];
        value.source_order = Some(vec![
            GridChildOrder::HLine(1),
            GridChildOrder::Header,
            GridChildOrder::VLine(0),
            GridChildOrder::Cell(0),
            GridChildOrder::HLine(0),
        ]);
        let expected_hline = Content::GridHLine(Arc::new(value.hlines[1].clone()));
        let expected_vline = Content::GridVLine(Arc::new(vline));
        let body = Content::Grid(Arc::new(value));
        assert_eq!(
            find(
                &body,
                &FigureKind::Element(Func::native(
                    "grid.hline",
                    crate::compiler::stdlib::native_grid_hline
                ))
            )
            .unwrap(),
            Some(expected_hline)
        );
        assert_eq!(
            find(
                &body,
                &FigureKind::Element(Func::native(
                    "grid.vline",
                    crate::compiler::stdlib::native_grid_vline
                ))
            )
            .unwrap(),
            Some(expected_vline)
        );
    }

    #[test]
    fn figure_kind_explicit_targets_find_represented_roots_inside_sequence() {
        for (element, function) in [
            (
                Content::figure_caption(Content::text("caption"), None, None),
                Func::native("caption", crate::compiler::stdlib::native_figure_caption),
            ),
            (
                Content::footnote_entry(Content::text("note"), None, None, None, None),
                Func::native("entry", crate::compiler::stdlib::native_footnote_entry),
            ),
            (
                Content::title(Content::text("title")),
                Func::native("title", crate::compiler::stdlib::native_title),
            ),
            (
                Content::divider(),
                Func::native("divider", crate::compiler::stdlib::native_divider),
            ),
            (
                Content::flush(),
                Func::native("flush", crate::compiler::stdlib::native_flush),
            ),
            (
                Content::curve_move(
                    crate::entities::layout_types::Length::ZERO,
                    crate::entities::layout_types::Length::ZERO,
                ),
                Func::native("move", crate::compiler::stdlib::native_curve_move),
            ),
        ] {
            let body = Content::sequence(vec![
                Content::text("before"),
                element.clone(),
                Content::text("after"),
            ]);
            let found = find(&body, &FigureKind::Element(function))
                .unwrap()
                .expect("exact represented target missing");
            if let (Content::Title(actual), Content::Title(expected)) = (&found, &element)
            {
                // Title deliberately has no Content PartialEq arm; retained Arc proves the exact target.
                assert!(Arc::ptr_eq(actual, expected));
            } else {
                assert_eq!(found, element);
            }
        }
        let geometry = Content::shape(
            crate::entities::geometry::ShapeKind::Ellipse,
            None,
            None,
            None,
            None,
        );
        let geometry_shape =
            if let Content::Shape(shape) = &geometry { shape } else { unreachable!() };
        let mut circle = (**geometry_shape).clone();
        circle.origin = crate::entities::elements::shape::ShapeOrigin::Circle;
        let circle = Content::Shape(Arc::new(circle));
        let body = Content::sequence(vec![geometry.clone(), circle.clone()]);
        let target = FigureKind::Element(Func::native(
            "circle-alias",
            crate::compiler::stdlib::native_circle,
        ));
        assert_eq!(find(&body, &target).unwrap(), Some(circle));
        assert_eq!(find(&geometry, &target).unwrap(), None);
    }

    #[test]
    fn figure_kind_explicit_kinds_cannot_bypass_reached_grid_order_invariant() {
        let mut value = grid();
        value.source_order = Some(vec![GridChildOrder::Header]);
        let file = crate::entities::file_id::FileId::from_raw(
            std::num::NonZeroU16::new(79).unwrap(),
        );
        let span = Span::from_range(file, 40..61);
        let body = Content::Grid(Arc::new(value)).with_source_span(span);
        assert_eq!(
            find(&body, &FigureKind::Name("custom".into())).unwrap_err()[0].span,
            span
        );
        for kind in [
            FigureKind::Name("custom".into()),
            FigureKind::Element(Func::native(
                "grid",
                crate::compiler::stdlib::native_grid,
            )),
        ] {
            let figure = Content::Figure(Arc::new(
                crate::entities::elements::figure::FigureElem {
                    body: body.clone(),
                    caption: None,
                    kind: Some(Some(kind)),
                    resolved_kind: None,
                    supplement: Some(FigureSupplement::None),
                    resolved_supplement: None,
                },
            ));
            let errors = super::super::figure::preview(
                &figure,
                &crate::entities::style_chain::StyleChain::default_chain(),
            )
            .unwrap_err();
            assert_eq!(errors[0].message, "invalid grid source order");
            assert_eq!(errors[0].span, span);
        }
        // Validation examines every reached semantic field, even after an early inferable root.
        let enclosing = Content::table(vec![], vec![], vec![body.clone()]);
        assert_eq!(validate_body(&enclosing).unwrap_err()[0].span, span);
        let mut dictionary = indexmap::IndexMap::default();
        dictionary.insert("hidden".into(), Value::Content(body));
        assert!(validate_body(&Content::metadata(Value::Dict(dictionary))).is_ok());
    }

    #[test]
    fn figure_kind_grid_rejects_every_malformed_order_before_early_match() {
        let file = crate::entities::file_id::FileId::from_raw(
            std::num::NonZeroU16::new(77).unwrap(),
        );
        let span = Span::from_range(file, 19..31);
        for order in [
            vec![GridChildOrder::Header],
            vec![
                GridChildOrder::Header,
                GridChildOrder::Cell(0),
                GridChildOrder::Cell(0),
            ],
            vec![GridChildOrder::Header, GridChildOrder::Cell(9)],
            vec![GridChildOrder::Header, GridChildOrder::Cell(0), GridChildOrder::Footer],
            vec![
                GridChildOrder::Header,
                GridChildOrder::Cell(0),
                GridChildOrder::HLine(0),
            ],
            vec![
                GridChildOrder::Header,
                GridChildOrder::Cell(0),
                GridChildOrder::VLine(0),
            ],
        ] {
            let mut value = grid();
            value.source_order = Some(order);
            let body = Content::Grid(Arc::new(value)).with_source_span(span);
            for errors in
                [infer(&body).unwrap_err(), find(&body, &raw_kind()).unwrap_err()]
            {
                assert_eq!(errors.len(), 1);
                assert_eq!(errors[0].span, span);
                assert_eq!(errors[0].message, "invalid grid source order");
            }
        }
    }

    #[test]
    fn figure_kind_selected_supplement_and_caption_semantic_order() {
        let mut figure = crate::entities::elements::figure::FigureElem {
            body: Content::text("body"),
            caption: None,
            kind: None,
            resolved_kind: None,
            supplement: Some(FigureSupplement::Content(table())),
            resolved_supplement: None,
        };
        assert_eq!(
            infer(&Content::Figure(Arc::new(figure.clone()))).unwrap(),
            table_kind()
        );
        figure.resolved_supplement = Some(None);
        assert_eq!(
            infer(&Content::Figure(Arc::new(figure.clone()))).unwrap(),
            image_kind()
        );
        figure.caption = Some(Content::raw("caption", None, false));
        figure.resolved_supplement = Some(Some(table()));
        assert_eq!(infer(&Content::Figure(Arc::new(figure))).unwrap(), raw_kind());
    }

    #[test]
    fn figure_kind_context_does_not_execute_callback() {
        let context = Content::ContextBlock(Arc::new(
            crate::entities::elements::context_block::ContextBlockElem {
                id: 4,
                closure: Func::native("effect", |_, _, _, _| {
                    panic!("semantic inspection executed Context")
                }),
            },
        ));
        assert_eq!(infer(&context).unwrap(), image_kind());
    }

    #[derive(Debug, Clone, PartialEq)]
    struct OpaqueExtension {
        body: Content,
    }
    impl std::hash::Hash for OpaqueExtension {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            std::hash::Hash::hash(&0_u8, state);
        }
    }
    impl crate::entities::elements::Element for OpaqueExtension {
        fn plain_text(&self) -> String {
            panic!("semantic inspection called extension")
        }
        fn map_content<F>(&self, _: &mut F) -> SourceResult<Content>
        where
            F: FnMut(&Content) -> SourceResult<Option<Content>>,
        {
            panic!("semantic inspection mapped extension")
        }
        fn map_text<F>(&self, _: &mut F) -> Content
        where
            F: FnMut(&str) -> String,
        {
            panic!("semantic inspection mapped extension text")
        }
        fn get_field(&self, _: &str) -> Option<Value> {
            panic!("semantic inspection enumerated extension")
        }
    }
    #[test]
    fn figure_kind_dynamic_container_is_explicitly_opaque_without_callbacks() {
        let extension = Content::dynamic(OpaqueExtension { body: table() });
        assert_eq!(infer(&extension).unwrap(), image_kind());
        assert_eq!(find(&extension, &table_kind()).unwrap(), None);
    }
}
