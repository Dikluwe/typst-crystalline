//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/bindings/field_access/fields.md
//! @prompt-hash 6a6a54a9
//! @layer L1
//! @updated 2026-10-07

use super::*;

pub(in crate::compiler::eval) fn check_module_field_feature(
    target: &Value,
    field: &str,
    span: Span,
    features: crate::entities::compiler_features::Features,
) -> SourceResult<()> {
    // P1484 — gate apenas no std efetivo; módulos e dicts homônimos não são gated.
    if field == "asset"
        && matches!(target, Value::Module(module) if is_effective_std_module(module))
        && !features.contains(crate::entities::compiler_features::Feature::Bundle)
    {
        return Err(vec![SourceDiagnostic::error(
            span,
            "cannot access field `asset` because the `bundle` feature is not enabled",
        )
        .with_hint("try enabling the `bundle` feature")
        .with_hint("see https://typst.app/help/compiler-features for more details")]);
    }

    // `std.html` e `format.html` existem internamente em todos os perfis, mas
    // o acesso público é feature-gated. A checagem ocorre antes do lookup e,
    // portanto, antes de qualquer avaliação dos argumentos de uma set rule.
    if field == "html"
        && matches!(target, Value::Module(module) if is_effective_std_module(module) || is_effective_format_module(module))
        && !features.contains(crate::entities::compiler_features::Feature::Html)
    {
        return Err(vec![SourceDiagnostic::error(
            span,
            "cannot access field `html` because the `html` feature is not enabled",
        )
        .with_hint("try enabling the `html` feature")
        .with_hint("see https://typst.app/help/compiler-features for more details")]);
    }

    if field == "bundle"
        && matches!(target, Value::Module(module) if is_effective_format_module(module))
        && !features.contains(crate::entities::compiler_features::Feature::Bundle)
    {
        return Err(vec![SourceDiagnostic::error(
            span,
            "cannot access field `bundle` because the `bundle` feature is not enabled",
        )
        .with_hint("try enabling the `bundle` feature")
        .with_hint("see https://typst.app/help/compiler-features for more details")]);
    }

    Ok(())
}

pub(in crate::compiler::eval) fn eval_field_access(
    access: crate::entities::ast::expr::FieldAccess<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    let target = eval_expr(access.target(), scopes, ctx, engine)?;
    eval_field_access_with_target(access, target, scopes, ctx, engine)
}

pub(in crate::compiler::eval) fn eval_field_access_with_target(
    access: crate::entities::ast::expr::FieldAccess<'_>,
    target: Value,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    use crate::entities::ast::AstNode;
    let field = access.field().as_str();

    check_module_field_feature(&target, field, access.field().span(), ctx.features)?;

    // P509 — field access em coleções despacha para métodos de instância.
    if let Some(result) = crate::compiler::stdlib::try_dispatch_collection_method(
        target.clone(),
        field,
        crate::entities::args::Args::positional(vec![]),
        None,
        scopes,
        ctx,
        engine,
    ) {
        return result;
    }

    if let Value::Func(f) = &target {
        if let Some(call) = f.native_fn_addr() {
            use crate::compiler::stdlib::{
                native_move, native_rotate, native_scale, native_skew,
            };
            let family = [
                (native_move as fn(_, _, _, _) -> _, "move"),
                (native_rotate as fn(_, _, _, _) -> _, "rotate"),
                (native_scale as fn(_, _, _, _) -> _, "scale"),
                (native_skew as fn(_, _, _, _) -> _, "skew"),
            ]
            .into_iter()
            .find(|(candidate, _)| std::ptr::fn_addr_eq(call, *candidate));
            if let Some((_, name)) = family {
                use crate::entities::layout_types::{
                    Align2D, Angle, HAlign, Length, Ratio, VAlign,
                };
                let default = match (name, field) {
                    ("move", "dx" | "dy") => {
                        Some(Value::Relative(crate::entities::rel::Rel {
                            rel: 0.0,
                            abs: Length::ZERO,
                        }))
                    }
                    ("rotate", "angle") | ("skew", "ax" | "ay") => {
                        Some(Value::Angle(Angle::deg(0.0)))
                    }
                    ("scale", "x" | "y") => Some(Value::Ratio(Ratio(1.0))),
                    ("rotate" | "scale" | "skew", "origin") => {
                        Some(Value::Align(Align2D {
                            h: Some(HAlign::Center),
                            v: Some(VAlign::Horizon),
                        }))
                    }
                    ("rotate" | "scale" | "skew", "reflow") => Some(Value::Bool(false)),
                    _ => None,
                };
                if let Some(default) = default {
                    if !ctx.in_context {
                        return Err(vec![SourceDiagnostic::error(
                            access.span(),
                            "can only be used when context is known",
                        )
                        .with_hint("try wrapping this in a `context` expression")]);
                    }
                    let value = engine.styles.custom(&format!("{name}.{field}")).cloned();
                    return Ok(match (value, default) {
                        (Some(Value::Align(a)), Value::Align(d)) => {
                            Value::Align(Align2D { h: a.h.or(d.h), v: a.v.or(d.v) })
                        }
                        (Some(value), _) => value,
                        (None, default) => default,
                    });
                }
            }
        }
    }

    // P792 — `text.<campo>`: quando o target é Value::Func("text") e o field é
    // um parâmetro de estilo, ler da StyleChain activa.
    if let Value::Func(ref f) = target {
        if f.name() == Some("text") {
            match field {
                "size" => {
                    return Ok(Value::Length(crate::entities::layout_types::Length::pt(
                        engine.styles.size(),
                    )));
                }
                "lang" => {
                    let lang = match engine.styles.custom("text.lang") {
                        Some(Value::Str(s)) => s.clone(),
                        _ => "en".into(),
                    };
                    return Ok(Value::Str(lang));
                }
                _ => {} // cai no eval_value_field_access normal
            }
        }
    }

    // P1464 — o módulo global e a função format.pdf partilham o gate
    // a11y-extras. A função é reconhecida por identidade nativa e namespace,
    // inclusive através de aplicação parcial.
    if is_a11y_pdf_target(&target)
        && matches!(field, "table-summary" | "header-cell" | "data-cell")
        && !ctx
            .features
            .contains(crate::entities::compiler_features::Feature::A11yExtras)
    {
        return Err(vec![SourceDiagnostic::error(
            access.field().span(),
            format!(
                "cannot access field `{field}` because the `a11y-extras` feature is not enabled"
            ),
        )
        .with_hint("try enabling the `a11y-extras` feature")
        .with_hint("see https://typst.app/help/compiler-features for more details")]);
    }

    // Guardar só a identidade Arc do módulo, para consultar metadata
    // depois de um lookup válido sem clonar os demais tipos de Value.
    let warning_module = match &target {
        Value::Module(module) => Some(module.clone()),
        _ => None,
    };

    let span = if matches!(
        &target,
        Value::Module(_)
            | Value::Type(_)
            | Value::Dict(_)
            | Value::Content(_)
            | Value::Float(_)
            | Value::Int(_)
            | Value::Str(_)
            | Value::Bool(_)
            | Value::None
            | Value::Auto
            | Value::Array(_)
    ) || matches!(&target, Value::Func(f) if native_public_name(f).is_some() || is_user_defined_closure(f))
    {
        access.field().span()
    } else {
        access.span()
    };
    let value = eval_value_field_access(target, field, span)?;
    if let Some(module) = warning_module {
        warn_deprecated_symbol_binding(
            scopes,
            &module,
            field,
            &value,
            access.field().span(),
            engine,
        );
    }
    Ok(value)
}

pub(in crate::compiler::eval) fn eval_value_field_access(
    target: Value,
    field: &str,
    span: Span,
) -> SourceResult<Value> {
    use crate::entities::source_result::SourceDiagnostic;
    match target {
        Value::Dict(d) => d.get(field).cloned().ok_or_else(|| {
            vec![SourceDiagnostic::error(
                span,
                format!("dictionary does not contain key \"{field}\""),
            )]
        }),
        // Field access em elementos estruturados — usado por show rules (Passo 68).
        Value::LocatedContent(c, _) => {
            let value = match c.fields() {
                Some(fields) => fields.get(field).cloned(),
                None => {
                    if is_presence_content(c.content())
                        || is_matrix_or_cell_content(c.content())
                    {
                        return declared_content_field_access(c.content(), field, span);
                    }
                    c.content().get_field(field)
                }
            };
            if value.is_none()
                && (is_presence_content(c.content())
                    || is_matrix_or_cell_content(c.content()))
                && matches!(
                    content_field(c.content(), field),
                    ContentField::Set(_) | ContentField::Unset
                )
            {
                return Err(vec![SourceDiagnostic::error(
                    span,
                    format!(
                        "field \"{field}\" in {} is not known at this point",
                        c.content().elem_name()
                    ),
                )]);
            }
            value.ok_or_else(|| {
                vec![SourceDiagnostic::error(
                    span,
                    format!(
                        "{} does not have field \"{field}\"",
                        c.content().elem_name()
                    ),
                )]
            })
        }
        Value::Content(c) => {
            if is_presence_content(&c) || is_matrix_or_cell_content(&c) {
                declared_content_field_access(&c, field, span)
            } else {
                c.get_field(field).ok_or_else(|| {
                    vec![SourceDiagnostic::error(
                        span,
                        format!("{} does not have field \"{field}\"", c.elem_name()),
                    )]
                })
            }
        }
        // P785b — Field access em Value::Relative (RelativeLength / Rel)
        Value::Relative(rel) => match field {
            "ratio" => Ok(Value::Ratio(crate::entities::layout_types::Ratio(rel.rel))),
            "length" => Ok(Value::Length(rel.abs)),
            _ => Err(vec![SourceDiagnostic::error(
                span,
                format!("relative length does not contain field \"{field}\""),
            )]),
        },
        // P785b — Field access em Value::Align (Alignment)
        Value::Align(align) => match field {
            "x" => Ok(align
                .h
                .map(|h| {
                    Value::Align(crate::entities::layout_types::Align2D {
                        h: Some(h),
                        v: None,
                    })
                })
                .unwrap_or(Value::None)),
            "y" => Ok(align
                .v
                .map(|v| {
                    Value::Align(crate::entities::layout_types::Align2D {
                        h: None,
                        v: Some(v),
                    })
                })
                .unwrap_or(Value::None)),
            _ => Err(vec![SourceDiagnostic::error(
                span,
                format!("alignment does not contain field \"{field}\""),
            )]),
        },
        // P785b — Field access em Value::Length
        Value::Length(len) => match field {
            "em" => Ok(Value::Float(len.em)),
            "abs" => Ok(Value::Length(crate::entities::layout_types::Length::pt(
                len.abs.to_pt(),
            ))),
            _ => Err(vec![SourceDiagnostic::error(
                span,
                format!("length does not contain field \"{field}\""),
            )]),
        },
        // P785b — Field access em Value::Stroke
        Value::Stroke(stroke) => match field {
            "paint" => Ok(Value::Color(stroke.paint.to_color())),
            "thickness" => Ok(Value::Length(crate::entities::layout_types::Length::pt(
                stroke.thickness,
            ))),
            _ => Err(vec![SourceDiagnostic::error(
                span,
                format!("stroke does not contain field \"{field}\""),
            )]),
        },
        // P684 — Field access em Value::Version
        Value::Version(v) => match field {
            "major" => Ok(Value::Int(v.component(0) as i64)),
            "minor" => Ok(Value::Int(v.component(1) as i64)),
            "patch" => Ok(Value::Int(v.component(2) as i64)),
            _ => Err(vec![SourceDiagnostic::error(
                span,
                format!("version does not contain field \"{field}\""),
            )]),
        },
        // P412 — Field access em Value::Duration
        Value::Duration(d) => {
            const NANOS_PER_SECOND: f64 = 1_000_000_000.0;
            const NANOS_PER_MINUTE: f64 = 60_000_000_000.0;
            const NANOS_PER_HOUR: f64 = 3_600_000_000_000.0;
            const NANOS_PER_DAY: f64 = 86_400_000_000_000.0;
            match field {
                "seconds" => Ok(Value::Float(d.nanos as f64 / NANOS_PER_SECOND)),
                "minutes" => Ok(Value::Float(d.nanos as f64 / NANOS_PER_MINUTE)),
                "hours" => Ok(Value::Float(d.nanos as f64 / NANOS_PER_HOUR)),
                "days" => Ok(Value::Float(d.nanos as f64 / NANOS_PER_DAY)),
                _ => Err(vec![SourceDiagnostic::error(
                    span,
                    format!("duration does not contain field \"{field}\""),
                )]),
            }
        }
        // P493a — Field access em Value::Array
        Value::Array(arr) => match field {
            "len" => Ok(Value::Int(arr.len() as i64)),
            "first" => Ok(arr.first().cloned().unwrap_or(Value::None)),
            "last" => Ok(arr.last().cloned().unwrap_or(Value::None)),
            _ => Err(vec![SourceDiagnostic::error(
                span,
                "cannot access fields on type array",
            )]),
        },
        // P504 — Field access em Value::Args
        Value::Args(a) => match field {
            "named" => Ok(Value::Dict(a.named.clone())),
            "positional" => Ok(Value::Array(a.items.clone())),
            _ => Err(vec![SourceDiagnostic::error(
                span,
                format!("arguments does not contain field \"{field}\""),
            )]),
        },
        // P493b — Field access em Value::Func com namespace
        Value::Func(f) => match f.namespace().and_then(|ns| ns.get(field)) {
            Some(value) => Ok(value.clone()),
            None => Err(vec![SourceDiagnostic::error(
                span,
                match native_public_name(&f) {
                    Some(name) => {
                        format!("function `{name}` does not contain field `{field}`")
                    }
                    None if is_user_defined_closure(&f) => {
                        "cannot access fields on user-defined functions".to_string()
                    }
                    None => "cannot access fields on type function".to_string(),
                },
            )]),
        },
        // P685 — Field access em valor-tipo
        Value::Type(t) => match (t, field) {
            (Type::Version, _) => crate::compiler::stdlib::version_type_field(field)
                .ok_or_else(|| {
                    vec![SourceDiagnostic::error(
                        span,
                        format!("type version does not contain field \"{field}\""),
                    )]
                }),
            (Type::Float, _) => crate::compiler::stdlib::float_type_field(field)
                .ok_or_else(|| {
                    vec![SourceDiagnostic::error(
                        span,
                        format!("type float does not contain field \"{field}\""),
                    )]
                }),
            (Type::Int, _) => {
                crate::compiler::stdlib::int_type_field(field).ok_or_else(|| {
                    vec![SourceDiagnostic::error(
                        span,
                        format!("type int does not contain field \"{field}\""),
                    )]
                })
            }
            (Type::Counter, _) => crate::compiler::stdlib::counter::counter_type_field(
                field,
            )
            .ok_or_else(|| {
                vec![SourceDiagnostic::error(
                    span,
                    format!("type counter does not contain field \"{field}\""),
                )]
            }),
            (Type::Content, _) => content_type_field(field).ok_or_else(|| {
                vec![SourceDiagnostic::error(
                    span,
                    format!("type content does not contain field \"{field}\""),
                )]
            }),
            (Type::Str, "from-unicode") => {
                Ok(Value::Func(Func::native("str.from-unicode", native_str_from_unicode)))
            }
            (Type::Array, _) => {
                crate::compiler::stdlib::collection_type_field(Type::Array, field)
                    .ok_or_else(|| {
                        vec![SourceDiagnostic::error(
                            span,
                            format!("type array does not contain field `{field}`"),
                        )]
                    })
            }
            (Type::Dictionary, _) => {
                crate::compiler::stdlib::collection_type_field(Type::Dictionary, field)
                    .ok_or_else(|| {
                        vec![SourceDiagnostic::error(
                            span,
                            format!("type dictionary does not contain field \"{field}\""),
                        )]
                    })
            }
            (Type::Str, _) => {
                crate::compiler::stdlib::collection_type_field(Type::Str, field)
                    .ok_or_else(|| {
                        vec![SourceDiagnostic::error(
                            span,
                            format!("type str does not contain field \"{field}\""),
                        )]
                    })
            }
            (Type::Bytes, _) => {
                crate::compiler::stdlib::collection_type_field(Type::Bytes, field)
                    .ok_or_else(|| {
                        vec![SourceDiagnostic::error(
                            span,
                            format!("type bytes does not contain field \"{field}\""),
                        )]
                    })
            }
            (Type::Arguments, _) => {
                crate::compiler::stdlib::collection_type_field(Type::Arguments, field)
                    .ok_or_else(|| {
                        vec![SourceDiagnostic::error(
                            span,
                            format!("type arguments does not contain field \"{field}\""),
                        )]
                    })
            }
            (
                Type::Direction
                | Type::Angle
                | Type::Function
                | Type::Alignment
                | Type::Duration
                | Type::Length
                | Type::Selector
                | Type::State
                | Type::Location,
                _,
            ) => crate::compiler::eval::call_dispatch::p1284_type_field(t, field)
                .ok_or_else(|| {
                    vec![SourceDiagnostic::error(
                        span,
                        format!("type {} does not contain field \"{field}\"", t.name()),
                    )]
                }),
            (Type::SpotColorant, "tint") => Ok(Value::Func(Func::native(
                "tint",
                crate::compiler::stdlib::color::native_spot_tint,
            ))),
            (Type::Color, _) => crate::compiler::stdlib::color_type_field(field)
                .ok_or_else(|| {
                    vec![SourceDiagnostic::error(
                        span,
                        format!("type color does not contain field `{field}`"),
                    )]
                }),
            (Type::Gradient, _) => crate::compiler::stdlib::gradient_type_field(field)
                .ok_or_else(|| {
                    vec![SourceDiagnostic::error(
                        span,
                        format!("type gradient does not contain field `{field}`"),
                    )]
                }),
            (Type::Datetime, _) => crate::compiler::stdlib::datetime_type_field(field)
                .ok_or_else(|| {
                    vec![SourceDiagnostic::error(
                        span,
                        format!("type datetime does not contain field \"{field}\""),
                    )]
                }),
            _ => Err(vec![SourceDiagnostic::error(
                span,
                format!("type {} does not contain field \"{field}\"", t.name()),
            )]),
        },
        // P679 — Field access em Value::Module
        Value::Module(m) => m.scope().get(field).cloned().ok_or_else(|| {
            let name = m.name();
            vec![SourceDiagnostic::error(
                span,
                format!("module `{name}` does not contain `{field}`"),
            )]
        }),
        // P765a — Field access em Value::Symbol
        Value::Symbol(s) => {
            let modified = s.modified(field).or_else(|| {
                if field != "tri"
                    || s.applied.iter().any(|m| !matches!(m.as_str(), "eq" | "not"))
                {
                    return None;
                }
                let sym_lookup = crate::compiler::stdlib::sym::sym_lookup;
                for name in ["gt", "lt"] {
                    let Some(parent) = sym_lookup(name) else {
                        continue;
                    };
                    let canonical =
                        s.applied.iter().try_fold(parent, |value, modifier| {
                            value.modified(modifier.as_str())
                        });
                    if canonical.as_ref() == Some(&s) {
                        let mut path = name.to_owned();
                        for modifier in &s.applied {
                            path.push('.');
                            path.push_str(modifier.as_str());
                        }
                        path.push_str(".tri");
                        return sym_lookup(&path);
                    }
                }
                None
            });
            modified.map(Value::Symbol).ok_or_else(|| {
                vec![SourceDiagnostic::error(
                    span,
                    format!("unknown symbol modifier '{field}'"),
                )]
            })
        }
        other @ (Value::Bool(_) | Value::Int(_) | Value::Str(_)) => {
            Err(vec![SourceDiagnostic::error(
                span,
                format!("cannot access fields on type {}", vanilla_type_name(&other)),
            )])
        }
        other => Err(vec![SourceDiagnostic::error(
            span,
            format!("cannot access fields on type {}", other.type_name()),
        )]),
    }
}

fn is_presence_content(content: &crate::entities::content::Content) -> bool {
    use crate::entities::content::Content;
    match content {
        Content::SmartQuote(_) => true,
        Content::Transform(e) => !matches!(
            e.operation,
            crate::entities::elements::transform::TransformOperation::Matrix(_)
        ),
        Content::Located(e) => is_presence_content(&e.body),
        Content::Realized(e) => is_presence_content(&e.visual),
        Content::Source(e) => is_presence_content(&e.body),
        Content::ShowRecipe(e) => is_presence_content(&e.body),
        Content::Text(..)
        | Content::Space
        | Content::SpaceFromNewline
        | Content::Empty
        | Content::Sequence(..)
        | Content::HtmlElem(..)
        | Content::HtmlFrame(..)
        | Content::Par { .. }
        | Content::Styled(..)
        | Content::PageRun(..)
        | Content::Heading(..)
        | Content::Title(..)
        | Content::Strong(..)
        | Content::Emph(..)
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
        | Content::Grid(..)
        | Content::GridHeader(..)
        | Content::GridFooter(..)
        | Content::GridCell(..)
        | Content::GridHLine(..)
        | Content::GridVLine(..)
        | Content::SetPage(..)
        | Content::Align(..)
        | Content::Place(..)
        | Content::Flush(..)
        | Content::Divider(..)
        | Content::Terms(..)
        | Content::TermItem(..)
        | Content::Quote(..)
        | Content::PdfAttach(..)
        | Content::PdfArtifact(..)
        | Content::Underline(..)
        | Content::Strike(..)
        | Content::Overline(..)
        | Content::SmallCaps { .. }
        | Content::Pad(..)
        | Content::Columns(..)
        | Content::Boxed(..)
        | Content::Block(..)
        | Content::Table(..)
        | Content::TableCell(..)
        | Content::TableHeader(..)
        | Content::TableFooter(..)
        | Content::TableHLine(..)
        | Content::TableVLine(..)
        | Content::Bibliography(..)
        | Content::Cite(..)
        | Content::Footnote(..)
        | Content::FootnoteEntry(..)
        | Content::Metadata(..)
        | Content::State(..)
        | Content::StateUpdate(..)
        | Content::StateDisplay(..)
        | Content::CounterDisplayCallback(..)
        | Content::Dynamic(..)
        | Content::Document { .. }
        | Content::Asset { .. }
        | Content::ContextBlock(..)
        | Content::Colbreak(..)
        | Content::HSpace(..)
        | Content::Hide(..)
        | Content::Pagebreak(..)
        | Content::Parbreak
        | Content::Repeat(..)
        | Content::Stack(..)
        | Content::VSpace(..) => false,
    }
}

fn declared_content_field_access(
    content: &crate::entities::content::Content,
    field: &str,
    span: Span,
) -> SourceResult<Value> {
    match content_field(content, field) {
        ContentField::Set(value) => Ok(value),
        ContentField::Unset => Err(vec![SourceDiagnostic::error(
            span,
            format!(
                "field \"{field}\" in {} is not known at this point",
                content.elem_name()
            ),
        )]),
        ContentField::Undeclared => Err(vec![SourceDiagnostic::error(
            span,
            format!("{} does not have field \"{field}\"", content.elem_name()),
        )]),
    }
}

fn is_matrix_or_cell_content(content: &crate::entities::content::Content) -> bool {
    use crate::entities::content::Content;
    match content {
        Content::MathMatrix(_) | Content::TableCell(_) => true,
        Content::Located(e) => is_matrix_or_cell_content(&e.body),
        Content::Source(e) => is_matrix_or_cell_content(&e.body),
        Content::ShowRecipe(e) => is_matrix_or_cell_content(&e.body),
        Content::Realized(e) => is_matrix_or_cell_content(&e.visual),
        _ => false,
    }
}

#[cfg(test)]
mod smartquote_presence_tests {
    use super::*;
    use crate::entities::{content::Content, elements::smartquote::SmartQuoteElem};
    #[test]
    fn smartquote_semantic_direct_field_access_distinguishes_unknown_and_absent() {
        let omitted = Content::SmartQuote(std::sync::Arc::new(SmartQuoteElem {
            double: None,
            enabled: None,
            alternative: None,
            quotes: None,
        }));
        let span = Span::detached();
        let error =
            eval_value_field_access(Value::Content(omitted.clone()), "double", span)
                .unwrap_err();
        assert_eq!(
            error[0].message,
            "field \"double\" in smartquote is not known at this point"
        );
        assert_eq!(error[0].span, span);
        let error =
            eval_value_field_access(Value::Content(omitted), "other", span).unwrap_err();
        assert_eq!(error[0].message, "smartquote does not have field \"other\"");
        assert_eq!(
            eval_value_field_access(
                Value::Content(Content::smartquote(false)),
                "double",
                span
            )
            .unwrap(),
            Value::Bool(false)
        );
    }
}

#[cfg(test)]
mod builtin_module_identity_tests {
    use crate::compiler::eval::{
        eval_expression_with_features, eval_with_full_error_target_and_features,
        EvalTarget,
    };
    use crate::contracts::world::World;
    use crate::entities::compiler_features::{Feature, Features};
    use crate::entities::element_registry::ElementRegistry;
    use crate::entities::file_id::FileId;
    use crate::entities::font_book::FontBook;
    use crate::entities::sink::Sink;
    use crate::entities::source::Source;
    use crate::entities::value::Value;
    use crate::entities::world_types::{
        Bytes, Datetime, FileError, FileResult, Font, Library, Route, Routines, Traced,
    };
    use comemo::Track;
    struct TestWorld {
        source: Source,
        library: Library,
        book: FontBook,
    }

    impl World for TestWorld {
        fn library(&self) -> &Library {
            &self.library
        }
        fn book(&self) -> &FontBook {
            &self.book
        }
        fn main(&self) -> FileId {
            self.source.id()
        }
        fn source(&self, _: FileId) -> FileResult<Source> {
            Ok(self.source.clone())
        }
        fn file(&self, _: FileId) -> FileResult<Bytes> {
            Err(FileError::NotFound)
        }
        fn font(&self, _: usize) -> Option<Font> {
            None
        }
        fn today(
            &self,
            _: Option<crate::entities::duration::Duration>,
        ) -> Option<Datetime> {
            None
        }
    }

    fn features(mask: u8) -> Features {
        let mut features = Features::empty();
        for (bit, feature) in
            [(1, Feature::Html), (2, Feature::A11yExtras), (4, Feature::Bundle)]
        {
            if mask & bit != 0 {
                features.enable(feature);
            }
        }
        features
    }
    fn world(text: &str) -> TestWorld {
        TestWorld {
            source: Source::detached(text),
            library: Library::new(),
            book: FontBook::new(),
        }
    }

    #[test]
    fn builtin_module_identity_aliases_closures_and_independent_feature_profiles() {
        let world = world("");
        for mask in 0..8 {
            for (root, field, bit, feature) in [
                ("std", "html", 1, "html"),
                ("std.format", "html", 1, "html"),
                ("format", "html", 1, "html"),
                ("std", "asset", 4, "bundle"),
                ("std.format", "bundle", 4, "bundle"),
                ("format", "bundle", 4, "bundle"),
            ] {
                for expr in [format!("{root}.{field}"), format!("{{ let s = {root}; s.{field} }}"),
                    format!("{{ let s = {root}; let get() = s.{field}; let std = \"shadow\"; get() }}")] {
                    let (result, _) = eval_expression_with_features(&world, &expr, features(mask));
                    if mask & bit == 0 {
                        let errors = result.expect_err(&expr);
                        assert_eq!(errors.len(), 1);
                        assert_eq!(errors[0].message, format!("cannot access field `{field}` because the `{feature}` feature is not enabled"), "{expr}/{mask}");
                        assert_eq!(errors[0].hints, [format!("try enabling the `{feature}` feature"), "see https://typst.app/help/compiler-features for more details".into()]);
                    } else {
                        assert!(matches!(result, Ok(Value::Func(_))), "{expr}/{mask}: {result:?}");
                    }
                }
            }
        }
    }

    #[test]
    fn builtin_module_identity_dictionaries_keep_user_fields_in_all_profiles() {
        let world = world("");
        for mask in 0..8 {
            for field in ["html", "asset", "bundle"] {
                let expr = format!("{{ let std = ({field}: 42); std.{field} }}");
                let (result, _) =
                    eval_expression_with_features(&world, &expr, features(mask));
                assert_eq!(result.unwrap(), Value::Int(42));
            }
        }
    }

    #[test]
    fn builtin_module_identity_document_gate_anchors_before_call_or_set_arguments() {
        for text in [
            "#std.html",
            "#let s = std\n#s.html",
            "#std.html(panic(\"argument executed\"))",
            "#set std.html(pretty: panic(\"argument executed\"))",
        ] {
            for target in [EvalTarget::Paged, EvalTarget::Html, EvalTarget::Bundle] {
                let world = world(text);
                let mut sink = Sink::new();
                let result = eval_with_full_error_target_and_features(
                    &Routines::new(),
                    &world,
                    Traced::default().track(),
                    sink.track_mut(),
                    Route::root().track(),
                    &world.source,
                    &ElementRegistry::new(),
                    false,
                    target,
                    Features::empty(),
                );
                let errors = result.expect_err(text);
                assert_eq!(errors.len(), 1, "{text}: {errors:?}");
                assert_eq!(errors[0].message, "cannot access field `html` because the `html` feature is not enabled", "{text}");
                let start = text.find("html").unwrap();
                assert_eq!(
                    world.source.span_byte_range(errors[0].span),
                    Some(start..start + 4),
                    "{text}"
                );
                assert_eq!(
                    errors[0].hints,
                    [
                        "try enabling the `html` feature",
                        "see https://typst.app/help/compiler-features for more details"
                    ]
                );
                assert!(errors[0].trace.is_empty());
            }
        }
    }
}
