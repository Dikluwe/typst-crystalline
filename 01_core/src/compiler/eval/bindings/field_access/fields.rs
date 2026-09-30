//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/bindings/field_access/fields.md
//! @prompt-hash 5bf71ab6
//! @layer L1
//! @updated 2026-09-30

use super::*;

pub(in crate::compiler::eval) fn eval_field_access(
    access: crate::entities::ast::expr::FieldAccess<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    use crate::entities::ast::AstNode;

    let target = eval_expr(access.target(), scopes, ctx, engine)?;
    let field = access.field().as_str();

    // P1484 — gate apenas no std efetivo; módulos e dicts homônimos não são gated.
    if field == "asset"
        && matches!(&target, Value::Module(module) if is_effective_std_module(module))
        && !ctx
            .features
            .contains(crate::entities::compiler_features::Feature::Bundle)
    {
        return Err(vec![SourceDiagnostic::error(
            access.field().span(),
            "cannot access field `asset` because the `bundle` feature is not enabled",
        )
        .with_hint("try enabling the `bundle` feature")
        .with_hint("see https://typst.app/help/compiler-features for more details")]);
    }

    // P1465 — `format.html` existe no módulo efetivo em todos os perfis, mas
    // o acesso público é feature-gated. A checagem ocorre antes do lookup e,
    // portanto, antes de qualquer avaliação dos argumentos de uma set rule.
    if field == "html"
        && matches!(&target, Value::Module(module) if is_effective_format_module(module))
        && !ctx
            .features
            .contains(crate::entities::compiler_features::Feature::Html)
    {
        return Err(vec![SourceDiagnostic::error(
            access.field().span(),
            "cannot access field `html` because the `html` feature is not enabled",
        )
        .with_hint("try enabling the `html` feature")
        .with_hint("see https://typst.app/help/compiler-features for more details")]);
    }

    if field == "bundle"
        && matches!(&target, Value::Module(module) if is_effective_format_module(module))
        && !ctx
            .features
            .contains(crate::entities::compiler_features::Feature::Bundle)
    {
        return Err(vec![SourceDiagnostic::error(
            access.field().span(),
            "cannot access field `bundle` because the `bundle` feature is not enabled",
        )
        .with_hint("try enabling the `bundle` feature")
        .with_hint("see https://typst.app/help/compiler-features for more details")]);
    }

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
                None => c.content().get_field(field),
            };
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
        Value::Content(c) => c.get_field(field).ok_or_else(|| {
            vec![SourceDiagnostic::error(
                span,
                format!("{} does not have field \"{field}\"", c.elem_name()),
            )]
        }),
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
                            format!("type array does not contain field \"{field}\""),
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
