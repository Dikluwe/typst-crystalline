//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/layout/page.md
//! @prompt-hash 889d5b03
//! @layer L1
//! @updated 2026-09-30
//!
//! Comportamento de `stdlib/layout::page`.

use super::*;

pub(crate) fn is_positional_paper(name: &str, positional_len: usize) -> bool {
    positional_len >= 2
        && crate::entities::page_geometry::Paper::from_name(name).is_some()
}

pub(crate) fn is_horizontal_page_binding(
    align: crate::entities::layout_types::Align2D,
    horizontal: crate::entities::layout_types::HAlign,
) -> bool {
    align.v.is_none() && align.h == Some(horizontal)
}

/// P1140.26 — `page(paper?, ..named, body)` → page-run lexical.
pub fn native_page(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
    _scopes: &mut crate::compiler::scopes::Scopes<'_>,
    engine: &mut crate::entities::engine::Engine<'_>,
) -> SourceResult<Value> {
    use crate::entities::elements::page_run::PageRunElem;
    use crate::entities::layout_types::{HAlign, PageDimension, PageMarginSpec, VAlign};
    use crate::entities::page_canvas::{PageBleedSpec, PageFill};
    use crate::entities::page_geometry::{PageBinding, Paper};
    use crate::entities::page_running::{
        PageMarginal, PageNumberAlign, PageNumberVAlign,
    };
    use crate::entities::page_supplement::PageSupplement;
    use crate::entities::rel::Rel;
    use std::sync::Arc;

    const NAMED: &[&str] = &[
        "paper",
        "width",
        "height",
        "flipped",
        "margin",
        "bleed",
        "binding",
        "columns",
        "fill",
        "numbering",
        "supplement",
        "number-align",
        "header",
        "header-ascent",
        "footer",
        "footer-descent",
        "background",
        "foreground",
    ];
    for key in args.named.keys() {
        if !NAMED.contains(&key.as_str()) {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("unexpected argument `{key}` in page()"),
            )]);
        }
    }

    let mut positional = args.items.as_slice();
    let positional_paper = match positional.first() {
        Some(Value::Str(name)) if is_positional_paper(name, positional.len()) => {
            positional = &positional[1..];
            Paper::from_name(name)
        }
        _ => None,
    };
    if positional.len() != 1 {
        let message = if positional.is_empty() {
            "page() requires a positional body"
        } else {
            "unexpected positional argument in page()"
        };
        return Err(vec![SourceDiagnostic::error(args.span, message)]);
    }
    let body = match &positional[0] {
        Value::Content(content) => content.clone(),
        Value::Str(text) => Content::text(text),
        other => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("page() body expects content, found {}", other.type_name()),
            )])
        }
    };

    let named_paper = match args.named.get("paper") {
        Some(Value::Str(name)) => Some(Paper::from_name(name).ok_or_else(|| {
            vec![SourceDiagnostic::error(args.span, "unknown paper size")]
        })?),
        Some(other) => return Err(vec![page_type_error(args.span, "string", other)]),
        None => None,
    };
    if named_paper.is_some() && positional_paper.is_some() {
        return Err(vec![SourceDiagnostic::error(args.span, "paper specified twice")]);
    }
    let paper = named_paper.or(positional_paper);
    let size_pt = engine.styles.size();

    let dimension = |name: &str| -> SourceResult<Option<PageDimension>> {
        let Some(value) = args.named.get(name) else { return Ok(None) };
        match value {
            Value::Length(length) => {
                Ok(Some(PageDimension::Length(length.resolve_pt(size_pt))))
            }
            Value::Float(value) => Ok(Some(PageDimension::Length(*value))),
            Value::Int(value) => Ok(Some(PageDimension::Length(*value as f64))),
            Value::Auto => Ok(Some(PageDimension::Auto)),
            other => Err(vec![page_type_error(
                args.span,
                "length, float, int, or auto",
                other,
            )]),
        }
    };
    let rel = |value: &Value| -> SourceResult<Rel<Length>> {
        match value {
            Value::Relative(value) => Ok(*value),
            Value::Ratio(value) => Ok(Rel { rel: value.get(), abs: Length::ZERO }),
            Value::Length(value) => Ok(Rel { rel: 0.0, abs: *value }),
            Value::Float(value) => Ok(Rel { rel: 0.0, abs: Length::pt(*value) }),
            Value::Int(value) => Ok(Rel { rel: 0.0, abs: Length::pt(*value as f64) }),
            other => Err(vec![page_type_error(args.span, "relative length", other)]),
        }
    };
    let absolute = |value: &Value| -> SourceResult<Option<f64>> {
        match value {
            Value::Length(value) => Ok(Some(value.resolve_pt(size_pt))),
            Value::Float(value) => Ok(Some(*value)),
            Value::Int(value) => Ok(Some(*value as f64)),
            Value::None => Ok(None),
            other => Err(vec![page_type_error(args.span, "length", other)]),
        }
    };

    let margin = match args.named.get("margin") {
        None => None,
        Some(Value::Auto) => Some(PageMarginSpec::auto()),
        Some(Value::Dict(dict)) => {
            if let Some(key) = dict.keys().find(|key| {
                !matches!(
                    key.as_str(),
                    "left"
                        | "right"
                        | "top"
                        | "bottom"
                        | "inside"
                        | "outside"
                        | "x"
                        | "y"
                        | "rest"
                )
            }) {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!("unknown margin key: {key}"),
                )]);
            }
            let logical = dict.get("inside").is_some() || dict.get("outside").is_some();
            let physical = dict.get("left").is_some() || dict.get("right").is_some();
            if logical && physical {
                return Err(vec![SourceDiagnostic::error(args.span, "`inside` and `outside` are mutually exclusive with `left` and `right`")]);
            }
            let field = |key: &str| -> SourceResult<Option<f64>> {
                dict.get(key).map(&absolute).transpose().map(Option::flatten)
            };
            let x = field("x")?;
            let y = field("y")?;
            let rest = field("rest")?;
            Some(PageMarginSpec {
                left: field(if logical { "inside" } else { "left" })?.or(x).or(rest),
                right: field(if logical { "outside" } else { "right" })?.or(x).or(rest),
                top: field("top")?.or(y).or(rest),
                bottom: field("bottom")?.or(y).or(rest),
                two_sided: if logical || physical { Some(logical) } else { None },
            })
        }
        Some(value) => absolute(value)?.map(PageMarginSpec::uniform),
    };

    let bleed = match args.named.get("bleed") {
        None => None,
        Some(Value::Dict(dict)) => {
            if let Some(key) = dict.keys().find(|key| {
                !matches!(
                    key.as_str(),
                    "left"
                        | "right"
                        | "top"
                        | "bottom"
                        | "inside"
                        | "outside"
                        | "x"
                        | "y"
                        | "rest"
                )
            }) {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!("unknown bleed key: {key}"),
                )]);
            }
            let logical = dict.get("inside").is_some() || dict.get("outside").is_some();
            let physical = dict.get("left").is_some() || dict.get("right").is_some();
            if logical && physical {
                return Err(vec![SourceDiagnostic::error(args.span, "`inside` and `outside` are mutually exclusive with `left` and `right`")]);
            }
            let field = |key: &str| -> SourceResult<Option<Rel<Length>>> {
                dict.get(key).map(&rel).transpose()
            };
            let x = field("x")?;
            let y = field("y")?;
            let rest = field("rest")?;
            Some(PageBleedSpec {
                left: field(if logical { "inside" } else { "left" })?.or(x).or(rest),
                right: field(if logical { "outside" } else { "right" })?.or(x).or(rest),
                top: field("top")?.or(y).or(rest),
                bottom: field("bottom")?.or(y).or(rest),
                two_sided: if logical || physical { Some(logical) } else { None },
            })
        }
        Some(Value::Auto) => {
            return Err(vec![page_type_error(
                args.span,
                "relative length or dictionary",
                &Value::Auto,
            )])
        }
        Some(value) => Some(PageBleedSpec::uniform(rel(value)?)),
    };

    let flipped = match args.named.get("flipped") {
        Some(Value::Bool(value)) => Some(*value),
        Some(other) => return Err(vec![page_type_error(args.span, "bool", other)]),
        None => None,
    };
    let binding = match args.named.get("binding") {
        Some(Value::Auto) => Some(PageBinding::Auto),
        Some(Value::Align(align)) if is_horizontal_page_binding(*align, HAlign::Left) => {
            Some(PageBinding::Left)
        }
        Some(Value::Align(align))
            if is_horizontal_page_binding(*align, HAlign::Right) =>
        {
            Some(PageBinding::Right)
        }
        Some(other) => {
            return Err(vec![page_type_error(args.span, "auto, left, or right", other)])
        }
        None => None,
    };
    let columns = match args.named.get("columns") {
        Some(Value::Int(value)) if *value >= 1 => Some(*value as usize),
        Some(Value::Int(_)) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "columns must be at least 1",
            )])
        }
        Some(other) => return Err(vec![page_type_error(args.span, "integer", other)]),
        None => None,
    };
    let fill = match args.named.get("fill") {
        Some(Value::Auto) => Some(PageFill::Auto),
        Some(Value::None) => Some(PageFill::None),
        Some(Value::Color(value)) => Some(PageFill::Paint((value.clone()).into())),
        Some(Value::Gradient(value)) => Some(PageFill::Paint(value.clone().into())),
        Some(Value::Tiling(value)) => Some(PageFill::Paint((**value).clone().into())),
        Some(other) => {
            return Err(vec![page_type_error(args.span, "auto, none, or paint", other)])
        }
        None => None,
    };
    let numbering = match args.named.get("numbering") {
        Some(Value::Str(value)) => {
            Some(Some(crate::entities::numbering::Numbering::Pattern(value.clone())))
        }
        Some(Value::Func(value)) => {
            Some(Some(crate::entities::numbering::Numbering::Func(value.clone())))
        }
        Some(Value::None) => Some(None),
        Some(other) => {
            return Err(vec![page_type_error(
                args.span,
                "string, function, or none",
                other,
            )])
        }
        None => None,
    };
    let supplement = match args.named.get("supplement") {
        Some(Value::Auto) => Some(PageSupplement::Auto),
        Some(Value::None) => Some(PageSupplement::None),
        Some(Value::Content(value)) => {
            Some(PageSupplement::Content(Arc::new(value.clone())))
        }
        Some(Value::Str(value)) => {
            Some(PageSupplement::Content(Arc::new(Content::text(value))))
        }
        Some(other) => {
            return Err(vec![page_type_error(
                args.span,
                "auto, none, string, or content",
                other,
            )])
        }
        None => None,
    };
    let number_align = match args.named.get("number-align") {
        Some(Value::Align(align)) => {
            let vertical = match align.v.unwrap_or(VAlign::Bottom) {
                VAlign::Top => PageNumberVAlign::Top,
                VAlign::Bottom => PageNumberVAlign::Bottom,
                VAlign::Horizon => {
                    return Err(vec![SourceDiagnostic::error(
                        args.span,
                        "page number-align cannot use horizon",
                    )])
                }
            };
            Some(PageNumberAlign {
                horizontal: align.h.unwrap_or(HAlign::Center),
                vertical,
            })
        }
        Some(other) => return Err(vec![page_type_error(args.span, "alignment", other)]),
        None => None,
    };
    let marginal = |name: &str| -> SourceResult<Option<PageMarginal>> {
        Ok(match args.named.get(name) {
            Some(Value::Auto) => Some(PageMarginal::Auto),
            Some(Value::None) => Some(PageMarginal::None),
            Some(Value::Content(value)) => {
                Some(PageMarginal::Content(Arc::new(value.clone())))
            }
            Some(other) => {
                return Err(vec![page_type_error(
                    args.span,
                    "auto, none, or content",
                    other,
                )])
            }
            None => None,
        })
    };
    let layer = |name: &str| -> SourceResult<Option<Option<Content>>> {
        Ok(match args.named.get(name) {
            Some(Value::None) => Some(None),
            Some(Value::Content(value)) => Some(Some(value.clone())),
            Some(other) => {
                return Err(vec![page_type_error(args.span, "content or none", other)])
            }
            None => None,
        })
    };

    Ok(Value::Content(Content::PageRun(Arc::new(PageRunElem {
        paper,
        flipped,
        binding,
        width: dimension("width")?,
        height: dimension("height")?,
        margin,
        numbering,
        number_align,
        header: marginal("header")?,
        header_ascent: args.named.get("header-ascent").map(&rel).transpose()?,
        footer: marginal("footer")?,
        footer_descent: args.named.get("footer-descent").map(&rel).transpose()?,
        supplement,
        columns,
        bleed,
        fill,
        background: layer("background")?,
        foreground: layer("foreground")?,
        body,
    }))))
}
