//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/eval/rules/set/page.md
//! @prompt-hash 83424fce
//! @layer L1
//! @updated 2026-09-30
//!
//! Branch `#set page`.

use super::*;

pub(super) fn eval_set_page(
    set: SetRule<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    // #set page(width: .., height: .., margin: ..) — Passo 81.
    // Valores ausentes ficam None e preservam o valor actual em layout.
    // P636: rejeitar tipos inválidos em vez de ignorar silenciosamente.
    // **P757** — `em` em dimensões de página deve resolver contra o
    // tamanho de fonte activo na chain (default 11 pt), não contra zero.
    // Anteriormente usava-se `l.abs.to_pt()`, que descartava a componente
    // `em` e produzia `0pt` silenciosamente para valores como `7em`.
    // **P867** — `width`/`height` aceitam `auto`; `margin` mantém-se
    // length/float/int/none neste passo.
    let size_pt = engine.styles.size();
    fn extract_page_dimension(
        val: &Value,
        span: Span,
        size_pt: f64,
    ) -> SourceResult<Option<crate::entities::layout_types::PageDimension>> {
        match val {
            Value::Length(l) => {
                Ok(Some(crate::entities::layout_types::PageDimension::Length(
                    l.resolve_pt(size_pt),
                )))
            }
            Value::Float(f) => {
                Ok(Some(crate::entities::layout_types::PageDimension::Length(*f)))
            }
            Value::Int(i) => {
                Ok(Some(crate::entities::layout_types::PageDimension::Length(*i as f64)))
            }
            Value::Auto => Ok(Some(crate::entities::layout_types::PageDimension::Auto)),
            Value::None => Ok(None),
            other => Err(vec![type_mismatch("length, float, int, or auto", other, span)]),
        }
    }
    fn extract_pt(val: &Value, span: Span, size_pt: f64) -> SourceResult<Option<f64>> {
        match val {
            Value::Length(l) => Ok(Some(l.resolve_pt(size_pt))),
            Value::Float(f) => Ok(Some(*f)),
            Value::Int(i) => Ok(Some(*i as f64)),
            Value::None => Ok(None),
            other => Err(vec![type_mismatch("length, float, or int", other, span)]),
        }
    }
    fn extract_rel_length(
        val: &Value,
        span: Span,
    ) -> SourceResult<crate::entities::rel::Rel<crate::entities::layout_types::Length>>
    {
        use crate::entities::layout_types::Length;
        use crate::entities::rel::Rel;
        match val {
            Value::Relative(value) => Ok(*value),
            Value::Ratio(value) => Ok(Rel { rel: value.get(), abs: Length::ZERO }),
            Value::Length(value) => Ok(Rel { rel: 0.0, abs: *value }),
            Value::Float(value) => Ok(Rel { rel: 0.0, abs: Length::pt(*value) }),
            Value::Int(value) => Ok(Rel { rel: 0.0, abs: Length::pt(*value as f64) }),
            other => Err(vec![type_mismatch("relative length", other, span)]),
        }
    }
    let mut width: Option<crate::entities::layout_types::PageDimension> = None;
    let mut paper = None;
    let mut flipped = None;
    let mut binding = None;
    let mut height: Option<crate::entities::layout_types::PageDimension> = None;
    let mut margin: Option<crate::entities::layout_types::PageMarginSpec> = None;
    let mut margin_left = None;
    let mut margin_right = None;
    let mut margin_top = None;
    let mut margin_bottom = None;
    let mut numbering = None;
    let mut number_align = None;
    let mut header = None;
    let mut header_ascent = None;
    let mut footer = None;
    let mut footer_descent = None;
    let mut supplement = None;
    let mut columns: Option<usize> = None;
    let mut bleed = None;
    let mut fill = None;
    let mut background = None;
    let mut foreground = None;
    for arg in set.args().items() {
        if let Arg::Named(named) = arg {
            let key = named.name().as_str();
            let val = eval_expr(named.expr(), scopes, ctx, engine)?;
            let span = named.expr().span();
            match key {
                "paper" => {
                    paper = match val {
                        Value::Str(s) => {
                            crate::entities::page_geometry::Paper::from_name(&s)
                                .ok_or_else(|| {
                                    vec![SourceDiagnostic::error(
                                        span,
                                        "unknown paper size",
                                    )]
                                })
                                .map(Some)?
                        }
                        other => {
                            return Err(vec![type_mismatch("string", &other, span)]);
                        }
                    }
                }
                "flipped" => {
                    flipped = match val {
                        Value::Bool(v) => Some(v),
                        other => {
                            return Err(vec![type_mismatch("bool", &other, span)]);
                        }
                    }
                }
                "binding" => {
                    binding = match val {
                        Value::Auto => {
                            Some(crate::entities::page_geometry::PageBinding::Auto)
                        }
                        Value::Align(a)
                            if matches!(
                                (a.v, a.h),
                                (None, Some(crate::entities::layout_types::HAlign::Left))
                            ) =>
                        {
                            Some(crate::entities::page_geometry::PageBinding::Left)
                        }
                        Value::Align(a)
                            if matches!(
                                (a.v, a.h),
                                (
                                    None,
                                    Some(crate::entities::layout_types::HAlign::Right)
                                )
                            ) =>
                        {
                            Some(crate::entities::page_geometry::PageBinding::Right)
                        }
                        other => {
                            return Err(vec![type_mismatch(
                                "auto, left, or right",
                                &other,
                                span,
                            )]);
                        }
                    }
                }
                "width" => width = extract_page_dimension(&val, span, size_pt)?,
                "height" => height = extract_page_dimension(&val, span, size_pt)?,
                "margin" => match &val {
                    Value::Dict(d) => {
                        if let Some(key) = d.keys().find(|k| {
                            !matches!(
                                k.as_str(),
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
                                span,
                                format!("unknown margin key: {key}"),
                            )]);
                        }
                        let logical =
                            d.get("inside").is_some() || d.get("outside").is_some();
                        let physical =
                            d.get("left").is_some() || d.get("right").is_some();
                        if logical && physical {
                            return Err(vec![SourceDiagnostic::error(
                                    span,
                                    "`inside` and `outside` are mutually exclusive with `left` and `right`",
                                )]);
                        }
                        let extract_field = |k: &str| -> SourceResult<Option<f64>> {
                            if let Some(v) = d.get(k) {
                                extract_pt(v, span, size_pt)
                            } else {
                                Ok(None)
                            }
                        };
                        let mx = extract_field("x")?;
                        let my = extract_field("y")?;
                        let rest = extract_field("rest")?;
                        margin_left =
                            extract_field(if logical { "inside" } else { "left" })?
                                .or(mx)
                                .or(rest);
                        margin_right =
                            extract_field(if logical { "outside" } else { "right" })?
                                .or(mx)
                                .or(rest);
                        margin_top = extract_field("top")?.or(my).or(rest);
                        margin_bottom = extract_field("bottom")?.or(my).or(rest);
                        margin = Some(crate::entities::layout_types::PageMarginSpec {
                            left: margin_left,
                            right: margin_right,
                            top: margin_top,
                            bottom: margin_bottom,
                            two_sided: if logical || physical {
                                Some(logical)
                            } else {
                                None
                            },
                        });
                    }
                    Value::Auto => {
                        margin =
                            Some(crate::entities::layout_types::PageMarginSpec::auto());
                    }
                    other => {
                        if let Some(m) = extract_pt(other, span, size_pt)? {
                            margin_left = Some(m);
                            margin_right = Some(m);
                            margin_top = Some(m);
                            margin_bottom = Some(m);
                            margin = Some(
                                crate::entities::layout_types::PageMarginSpec::uniform(m),
                            );
                        }
                    }
                },
                "numbering" => {
                    numbering = match val {
                        Value::Str(s) => {
                            Some(Some(crate::entities::numbering::Numbering::Pattern(s)))
                        }
                        Value::Func(f) => {
                            Some(Some(crate::entities::numbering::Numbering::Func(f)))
                        }
                        Value::None => Some(None),
                        other => {
                            return Err(vec![type_mismatch(
                                "string, function, or none",
                                &other,
                                span,
                            )]);
                        }
                    };
                }
                "number-align" => {
                    let Value::Align(align) = val else {
                        return Err(vec![type_mismatch("alignment", &val, span)]);
                    };
                    let horizontal =
                        align.h.unwrap_or(crate::entities::layout_types::HAlign::Center);
                    let vertical = match align
                        .v
                        .unwrap_or(crate::entities::layout_types::VAlign::Bottom)
                    {
                        crate::entities::layout_types::VAlign::Top => {
                            crate::entities::page_running::PageNumberVAlign::Top
                        }
                        crate::entities::layout_types::VAlign::Bottom => {
                            crate::entities::page_running::PageNumberVAlign::Bottom
                        }
                        crate::entities::layout_types::VAlign::Horizon => {
                            return Err(vec![SourceDiagnostic::error(
                                span,
                                "page number-align cannot use horizon".to_string(),
                            )]);
                        }
                    };
                    number_align = Some(crate::entities::page_running::PageNumberAlign {
                        horizontal,
                        vertical,
                    });
                }
                "header" => {
                    header = Some(match val {
                        Value::Auto => crate::entities::page_running::PageMarginal::Auto,
                        Value::None => crate::entities::page_running::PageMarginal::None,
                        Value::Content(content) => {
                            crate::entities::page_running::PageMarginal::Content(
                                std::sync::Arc::new(content),
                            )
                        }
                        other => {
                            return Err(vec![type_mismatch(
                                "auto, none, or content",
                                &other,
                                span,
                            )]);
                        }
                    });
                }
                "header-ascent" => header_ascent = Some(extract_rel_length(&val, span)?),
                "footer" => {
                    footer = Some(match val {
                        Value::Auto => crate::entities::page_running::PageMarginal::Auto,
                        Value::None => crate::entities::page_running::PageMarginal::None,
                        Value::Content(content) => {
                            crate::entities::page_running::PageMarginal::Content(
                                std::sync::Arc::new(content),
                            )
                        }
                        other => {
                            return Err(vec![type_mismatch(
                                "auto, none, or content",
                                &other,
                                span,
                            )]);
                        }
                    });
                }
                "footer-descent" => {
                    footer_descent = Some(extract_rel_length(&val, span)?)
                }
                "supplement" => {
                    supplement = Some(match val {
                        Value::Auto => {
                            crate::entities::page_supplement::PageSupplement::Auto
                        }
                        Value::None => {
                            crate::entities::page_supplement::PageSupplement::None
                        }
                        Value::Content(content) => {
                            crate::entities::page_supplement::PageSupplement::Content(
                                std::sync::Arc::new(content),
                            )
                        }
                        Value::Str(text) => {
                            crate::entities::page_supplement::PageSupplement::Content(
                                std::sync::Arc::new(Content::text(text)),
                            )
                        }
                        other => {
                            return Err(vec![type_mismatch(
                                "auto, none, string, or content",
                                &other,
                                span,
                            )]);
                        }
                    });
                }
                "columns" => {
                    columns = match val {
                        Value::Int(n) if n >= 1 => Some(n as usize),
                        Value::Int(n) if n < 1 => {
                            return Err(vec![SourceDiagnostic::error(
                                named.span(),
                                "columns must be at least 1".to_string(),
                            )]);
                        }
                        Value::None => None,
                        other => {
                            return Err(vec![type_mismatch("int", &other, span)]);
                        }
                    };
                }
                "bleed" => {
                    use crate::entities::page_canvas::PageBleedSpec;
                    bleed = Some(match &val {
                        Value::Dict(dict) => {
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
                                    span,
                                    format!("unknown bleed key: {key}"),
                                )]);
                            }
                            let logical = dict.get("inside").is_some()
                                || dict.get("outside").is_some();
                            let physical =
                                dict.get("left").is_some() || dict.get("right").is_some();
                            if logical && physical {
                                return Err(vec![SourceDiagnostic::error(
                                        span,
                                        "`inside` and `outside` are mutually exclusive with `left` and `right`".to_string(),
                                    )]);
                            }
                            let field = |key: &str| -> SourceResult<
                                Option<
                                    crate::entities::rel::Rel<
                                        crate::entities::layout_types::Length,
                                    >,
                                >,
                            > {
                                dict.get(key)
                                    .map(|value| extract_rel_length(value, span))
                                    .transpose()
                            };
                            let x = field("x")?;
                            let y = field("y")?;
                            let rest = field("rest")?;
                            PageBleedSpec {
                                left: field(if logical { "inside" } else { "left" })?
                                    .or(x)
                                    .or(rest),
                                right: field(if logical { "outside" } else { "right" })?
                                    .or(x)
                                    .or(rest),
                                top: field("top")?.or(y).or(rest),
                                bottom: field("bottom")?.or(y).or(rest),
                                two_sided: if logical || physical {
                                    Some(logical)
                                } else {
                                    None
                                },
                            }
                        }
                        Value::Auto => {
                            return Err(vec![type_mismatch(
                                "relative length or dictionary",
                                &val,
                                span,
                            )]);
                        }
                        _ => PageBleedSpec::uniform(extract_rel_length(&val, span)?),
                    });
                }
                "fill" => {
                    fill = Some(match val {
                        Value::Auto => crate::entities::page_canvas::PageFill::Auto,
                        Value::None => crate::entities::page_canvas::PageFill::None,
                        Value::Color(color) => {
                            crate::entities::page_canvas::PageFill::Paint(color.into())
                        }
                        Value::Gradient(gradient) => {
                            crate::entities::page_canvas::PageFill::Paint(gradient.into())
                        }
                        Value::Tiling(tiling) => {
                            crate::entities::page_canvas::PageFill::Paint(
                                (*tiling).clone().into(),
                            )
                        }
                        other => {
                            return Err(vec![type_mismatch(
                                "auto, none, or paint",
                                &other,
                                span,
                            )]);
                        }
                    });
                }
                "background" => {
                    background = Some(match val {
                        Value::None => None,
                        Value::Content(content) => Some(std::sync::Arc::new(content)),
                        other => {
                            return Err(vec![type_mismatch(
                                "content or none",
                                &other,
                                span,
                            )]);
                        }
                    });
                }
                "foreground" => {
                    foreground = Some(match val {
                        Value::None => None,
                        Value::Content(content) => Some(std::sync::Arc::new(content)),
                        other => {
                            return Err(vec![type_mismatch(
                                "content or none",
                                &other,
                                span,
                            )]);
                        }
                    });
                }
                _ => {
                    return Err(vec![SourceDiagnostic::error(
                        named.span(),
                        format!("unknown page property: {key}"),
                    )]);
                }
            }
        }
    }

    // Empurrar as dimensões da página para a StyleChain para que o layout() possa ler
    if let Some(w) = width {
        let v = match w {
            crate::entities::layout_types::PageDimension::Auto => Value::Auto,
            crate::entities::layout_types::PageDimension::Length(x) => Value::Float(x),
        };
        *engine.styles = engine.styles.push_custom("page.width", v);
    }
    if let Some(h) = height {
        let v = match h {
            crate::entities::layout_types::PageDimension::Auto => Value::Auto,
            crate::entities::layout_types::PageDimension::Length(x) => Value::Float(x),
        };
        *engine.styles = engine.styles.push_custom("page.height", v);
    }
    if let Some(ml) = margin_left {
        *engine.styles = engine.styles.push_custom("page.margin-left", Value::Float(ml));
    }
    if let Some(mr) = margin_right {
        *engine.styles = engine.styles.push_custom("page.margin-right", Value::Float(mr));
    }
    if let Some(mt) = margin_top {
        *engine.styles = engine.styles.push_custom("page.margin-top", Value::Float(mt));
    }
    if let Some(mb) = margin_bottom {
        *engine.styles =
            engine.styles.push_custom("page.margin-bottom", Value::Float(mb));
    }

    Ok(Value::Content(Content::SetPage(std::sync::Arc::new(
        crate::entities::content::SetPageDelta {
            paper,
            flipped,
            binding,
            width,
            height,
            margin,
            numbering,
            number_align,
            header,
            header_ascent,
            footer,
            footer_descent,
            supplement,
            columns,
            bleed,
            fill,
            background,
            foreground,
        },
    ))))
}
