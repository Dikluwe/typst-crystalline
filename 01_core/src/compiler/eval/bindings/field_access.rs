//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/bindings/field_access.md
//! @prompt-hash 5df0c0a6
//! @layer L1
//! @updated 2026-08-12
//!
//! Acesso a campo (`a.b`) sobre valores e sobre `Content`, os métodos de
//! `Content`, e as mensagens de erro de campo e de callee não-chamável.
//!
//! Extraído de `compiler/eval/bindings.rs` no Passo 1013 conforme ADR-0109
//! (atomização — forma B, free function no arquivo da unidade).

use ecow::EcoString;
use indexmap::IndexMap;
use rustc_hash::FxBuildHasher;

use crate::compiler::scopes::Scopes;
use crate::compiler::stdlib::native_str_from_unicode;
use crate::entities::args::Args;
use crate::entities::engine::Engine;
use crate::entities::func::Func;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::{Type, Value};

use crate::compiler::eval::{eval_expr, EvalContext};

use crate::compiler::eval::operators::error_formatting::vanilla_type_name;

use super::method_dispatch::{expect_positional, finish_args};

pub(in crate::compiler::eval) fn eval_field_access(
    access: crate::entities::ast::expr::FieldAccess<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    use crate::entities::ast::AstNode;

    let target = eval_expr(access.target(), scopes, ctx, engine)?;
    let field = access.field().as_str();

    // P509 — field access em coleções despacha para métodos de instância.
    if let Some(result) = crate::compiler::stdlib::try_dispatch_collection_method(
        target.clone(),
        field,
        crate::entities::args::Args::positional(vec![]),
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

    // P820 (achado #7 de P810) — acesso a símbolo **depreciado** do módulo
    // `sym` (`#sym.join`): warning verbatim do vanilla com span no campo
    // (medido: `#sym.join` → warning @1:5, exit 0). O mecanismo geral do
    // vanilla é `Deprecation` em `Binding` (`foundations/scope.rs:288`);
    // aqui, data-driven pela tabela `SYM_DEPRECATED` do módulo `sym`.
    if let Value::Module(ref m) = target {
        if m.name() == "sym" {
            if let Some(msg) = crate::compiler::stdlib::sym::sym_deprecation(field) {
                engine.sink.warn_note(access.field().span(), msg, "");
            }
        }
    }

    eval_value_field_access(target, field, access.span())
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
                format!("array does not contain field \"{field}\""),
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
        Value::Func(f) => match f.namespace() {
            Some(ns) => ns.get(field).cloned().ok_or_else(|| {
                vec![SourceDiagnostic::error(
                    span,
                    format!("function does not contain field \"{field}\""),
                )]
            }),
            None => Err(vec![SourceDiagnostic::error(
                span,
                "cannot access fields on type function".to_string(),
            )]),
        },
        // P685 — Field access em valor-tipo
        Value::Type(t) => match (t, field) {
            (Type::Int, "min") => Ok(Value::Int(i64::MIN)),
            (Type::Int, "max") => Ok(Value::Int(i64::MAX)),
            (Type::Str, "from-unicode") => {
                Ok(Value::Func(Func::native("str.from-unicode", native_str_from_unicode)))
            }
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
            _ => Err(vec![SourceDiagnostic::error(
                span,
                format!("type {} does not contain field \"{field}\"", t.name()),
            )]),
        },
        // P679 — Field access em Value::Module
        Value::Module(m) => m.scope().get(field).cloned().ok_or_else(|| {
            vec![SourceDiagnostic::error(
                span,
                format!("module '{}' does not contain field \"{field}\"", m.name()),
            )]
        }),
        // P765a — Field access em Value::Symbol
        Value::Symbol(s) => s.modified(field).map(Value::Symbol).ok_or_else(|| {
            vec![SourceDiagnostic::error(
                span,
                format!("unknown symbol modifier '{field}'"),
            )]
        }),
        other => Err(vec![SourceDiagnostic::error(
            span,
            format!("cannot access fields on type {}", other.type_name()),
        )]),
    }
}

// ── P815 — método inexistente / dict-key-call (eval_field_callee) ───────────

/// **P815** — mirror de `element_or_type_with_name` do vanilla
/// (`typst-eval/src/call.rs:359-365`): `("element", nome do elemento)` para
/// content, `("type", nome longo do tipo)` nos restantes.
fn element_or_type_with_name(value: &Value) -> (&'static str, String) {
    if let Value::Content(c) = value {
        ("element", c.elem_name().to_string())
    } else {
        ("type", vanilla_type_name(value).to_string())
    }
}

/// **P815** — mirror do ramo de erro de `eval_field_callee` do vanilla
/// (`typst-eval/src/call.rs:258-345`), para um callee `target.field` chamado
/// como função depois de todos os despachos de método legítimos terem falhado.
///
/// Devolve `None` para `Symbol`/`Func`/`Type`/`Module` — os únicos tipos que
/// o vanilla deixa chamar campos directamente (`call.rs:258-263`) —, caindo
/// no caminho genérico existente. Nos restantes alvos:
///
/// - **O campo existe** (dict key, campo de content/length/args/…):
///   - dict → `cannot directly call dictionary keys as functions` + 2 hints;
///   - args → `cannot directly call named argument fields as functions` + 2 hints;
///   - outros → `` `{field}` is not a valid method for {kind} `{name}` `` + hint.
///   O primeiro hint muda se o valor guardado for função:
///   `to call the stored function, wrap the field access in parentheses:
///   `({full_text})(..)`` — senão `to access the `{field}` {key|argument|
///   field}, remove the function arguments: `{full_text}``.
/// - **O campo não existe** → `{kind} {name} has no method `{field}``
///   (ex.: `type integer has no method `foo``, `element strong has no method
///   `zzz``).
///
/// As mensagens são verbatim do vanilla (medidas por sonda em P810/P815 — a
/// mensagem é o observável, ADR-0107). Nota P810/P815: o vanilla **não** usa
/// distância de edição aqui (`call.rs:339-340`: "We don't try as hard on the
/// error here to avoid assuming the user's intent") — a hipótese do prompt
/// está refutada pela fonte e pela sonda.
pub(in crate::compiler::eval) fn field_callee_error(
    target: &Value,
    access: crate::entities::ast::expr::FieldAccess<'_>,
) -> Option<Vec<SourceDiagnostic>> {
    use crate::entities::ast::AstNode;

    if matches!(
        target,
        Value::Symbol(_) | Value::Func(_) | Value::Type(_) | Value::Module(_)
    ) {
        return None;
    }

    let field = access.field().as_str();
    let span = access.span();
    let is_dict = matches!(target, Value::Dict(_));
    let is_named = matches!(target, Value::Args(_));

    match eval_value_field_access(target.clone(), field, span) {
        Ok(callee_value) => {
            let mut err = if is_dict {
                SourceDiagnostic::error(
                    span,
                    "cannot directly call dictionary keys as functions",
                )
            } else if is_named {
                SourceDiagnostic::error(
                    span,
                    "cannot directly call named argument fields as functions",
                )
            } else {
                let (kind, name) = element_or_type_with_name(target);
                SourceDiagnostic::error(
                    span,
                    format!("`{field}` is not a valid method for {kind} `{name}`"),
                )
            };
            let full_text = access.to_untyped().clone().into_text();
            if matches!(callee_value, Value::Func(_)) {
                err = err.with_hint(format!(
                    "to call the stored function, wrap the field access in parentheses: `({full_text})(..)`"
                ));
            } else {
                let what = if is_dict {
                    "key"
                } else if is_named {
                    "argument"
                } else {
                    "field"
                };
                err = err.with_hint(format!(
                    "to access the `{field}` {what}, remove the function arguments: `{full_text}`"
                ));
            }
            if is_dict {
                err = err.with_hint(
                    "dictionary keys cannot be used with method syntax as keys could conflict with built-in method names".to_string(),
                );
            } else if is_named {
                err = err.with_hint(
                    "named arguments cannot be used with method syntax as argument names could conflict with built-in method names".to_string(),
                );
            }
            Some(vec![err])
        }
        Err(_) => {
            let (kind, name) = element_or_type_with_name(target);
            Some(vec![SourceDiagnostic::error(
                span,
                format!("{kind} {name} has no method `{field}`"),
            )])
        }
    }
}

// ── P829 — métodos de `content`: func/has/at/fields/location ───────────────

/// **P829** — estado de um campo de content para os métodos `has`/`at`/
/// `fields` (paridade vanilla `Content::has`/`at`/`fields`,
/// `foundations/content/mod.rs:510-590`): o vanilla distingue campo
/// **assente no constructor** de default resolvido pela chain — medido:
/// `heading[H].has("level")` → false, `heading(level: 2)[H].has("level")` →
/// true, markup `= H` assenta `depth` (não `level`).
enum ContentField {
    /// Assente — `has` → true, `at` devolve o valor.
    Set(Value),
    /// Declarado no elemento mas não assente — `has` → false; `at` erra
    /// `field "{f}" in {elem} is not known at this point` (verbatim vanilla).
    Unset,
    /// Não existe no elemento — `at` erra `{elem} does not have field "{f}"`.
    Undeclared,
}

/// **P829** — classifica um campo de content para `has`/`at`. Distinto de
/// `Content::get_field` (field access de show rules, que devolve valores
/// baked): aqui só conta o que foi assente no constructor, espelhando o
/// `field.has()`/`get()` do vanilla. A máscara `set_fields` de `HeadingElem`
/// (P829) é a única pista de "explicitamente assente" que o modelo cristalino
/// retém; nos restantes elementos os campos expostos são sempre assentes
/// (body/text), logo o fallback via `get_field` é exacto. `label` fica
/// scope-out: no cristalino a label é um nó irmão (`Content::Label`), não
/// metadado do elemento — divergência registada no L0.
fn content_field(c: &crate::entities::content::Content, field: &str) -> ContentField {
    use crate::entities::content::Content;
    use crate::entities::elements::heading::{
        HEADING_SET_DEPTH, HEADING_SET_LEVEL, HEADING_SET_OUTLINED,
    };
    match c {
        Content::Styled(child, styles) if equation_descendant(child).is_some() => {
            let key = match field {
                "numbering" => Some("equation.numbering"),
                "number-align" => Some("equation.number-align"),
                "supplement" => Some("equation.supplement"),
                "alt" => Some("equation.alt"),
                _ => None,
            };
            if let Some(value) = key.and_then(|key| {
                styles
                    .delta()
                    .custom
                    .iter()
                    .rev()
                    .find(|(candidate, _)| candidate.as_str() == key)
                    .map(|(_, value)| value.clone())
            }) {
                ContentField::Set(value)
            } else {
                content_field(child, field)
            }
        }
        Content::Strong(e) => match field {
            "body" => ContentField::Set(Value::Content(e.body.clone())),
            // `delta` é declarado em strong no vanilla mas nunca assente no
            // cristalino (`native_strong` não aceita named — scope-out).
            "delta" => ContentField::Unset,
            _ => ContentField::Undeclared,
        },
        Content::Emph(e) => match field {
            "body" => ContentField::Set(Value::Content(e.body.clone())),
            _ => ContentField::Undeclared,
        },
        Content::Title(t) => match field {
            "body" => ContentField::Set(Value::Content(t.body.clone())),
            _ => ContentField::Undeclared,
        },
        Content::Text(t) => match field {
            "text" => ContentField::Set(Value::Str(t.clone())),
            _ => ContentField::Undeclared,
        },
        Content::Linebreak(e) => match field {
            "justify" if e.justify_explicit => ContentField::Set(Value::Bool(e.justify)),
            "justify" => ContentField::Unset,
            _ => ContentField::Undeclared,
        },
        Content::Heading(h) => match field {
            "body" => ContentField::Set(Value::Content(h.body.clone())),
            "level" => {
                if h.set_fields & HEADING_SET_LEVEL != 0 {
                    ContentField::Set(Value::Int(h.level as i64))
                } else {
                    ContentField::Unset
                }
            }
            "depth" => {
                if h.set_fields & HEADING_SET_DEPTH != 0 {
                    ContentField::Set(Value::Int(h.level as i64))
                } else {
                    ContentField::Undeclared
                }
            }
            "outlined" => {
                if h.set_fields & HEADING_SET_OUTLINED != 0 {
                    ContentField::Set(Value::Bool(h.outlined))
                } else {
                    ContentField::Unset
                }
            }
            "bookmarked" => match h.bookmarked {
                Some(b) => ContentField::Set(Value::Bool(b)),
                None => ContentField::Unset,
            },
            _ => ContentField::Undeclared,
        },
        Content::Equation(e) => match field {
            "block" => ContentField::Set(Value::Bool(e.block)),
            "body" => ContentField::Set(Value::Content(e.body.clone())),
            _ => ContentField::Undeclared,
        },
        other => match other.get_field(field) {
            Some(v) => ContentField::Set(v),
            None => ContentField::Undeclared,
        },
    }
}

/// **P829** — campos assentes de um content, na ordem de declaração do
/// vanilla (para heading: level, depth, outlined, bookmarked, body — medido
/// em b8/b18: `(level: 2, body: [H])`, `(depth: 1, body: [H])`).
fn content_set_fields(
    c: &crate::entities::content::Content,
) -> Vec<(&'static str, Value)> {
    use crate::entities::content::Content;
    let candidates: &[&'static str] = match c {
        Content::Heading(_) => &["level", "depth", "outlined", "bookmarked", "body"],
        Content::Equation(_) => &["block", "body"],
        Content::Styled(child, _) if equation_descendant(child).is_some() => {
            &["block", "numbering", "number-align", "supplement", "alt", "body"]
        }
        Content::Text(_) => &["text"],
        Content::Linebreak(_) => &["justify"],
        _ => &["body"],
    };
    candidates
        .iter()
        .filter_map(|name| match content_field(c, name) {
            ContentField::Set(v) => Some((*name, v)),
            _ => None,
        })
        .collect()
}

fn equation_descendant(
    content: &crate::entities::content::Content,
) -> Option<&crate::entities::elements::equation::EquationElem> {
    use crate::entities::content::Content;
    match content {
        Content::Equation(e) => Some(e),
        Content::Styled(child, _) => equation_descendant(child),
        _ => None,
    }
}

/// **P829** — fallback de `func()` para variantes sem constructor nativo
/// exposto (Sequence, Styled, Label, math, Dynamic, …): o `Value::Func`
/// existe (repr = nome do elemento — paridade medida de `func()`) mas a
/// chamada não é suportada. Caso não medido no vanilla (elementos internos
/// não expostos no scope); mensagem própria, registada no L0.
fn content_func_not_callable(
    _ctx: &mut crate::compiler::eval::EvalContext,
    _args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: crate::entities::file_id::FileId,
) -> SourceResult<Value> {
    Err(vec![SourceDiagnostic::error(
        Span::detached(),
        "calling this element function is not supported".to_string(),
    )])
}

/// **P829** — `content.func()` (vanilla `Content::func`,
/// `foundations/content/mod.rs:516-519`): a função do elemento. Igualdade
/// por nome (P742) dá `strong[x].func() == strong` → true (medido b9).
fn content_elem_func(c: &crate::entities::content::Content) -> Value {
    use crate::entities::content::Content;
    let call: fn(
        &mut crate::compiler::eval::EvalContext,
        &Args,
        &dyn crate::contracts::world::World,
        crate::entities::file_id::FileId,
    ) -> SourceResult<Value> = match c {
        Content::Text(_) => crate::compiler::stdlib::native_text,
        Content::Strong(_) => crate::compiler::stdlib::native_strong,
        Content::Emph(_) => crate::compiler::stdlib::native_emph,
        Content::Heading(_) => crate::compiler::stdlib::native_heading,
        Content::Title(_) => crate::compiler::stdlib::native_title,
        Content::Raw(_) => crate::compiler::stdlib::native_raw,
        Content::Figure(_) => crate::compiler::stdlib::native_figure,
        Content::Link(_) => crate::compiler::stdlib::native_link,
        Content::SmallCaps { .. } => crate::compiler::stdlib::native_smallcaps,
        Content::Linebreak(_) => crate::compiler::stdlib::native_linebreak,
        _ => content_func_not_callable,
    };
    Value::Func(Func::native(c.elem_name(), call))
}

/// **P829** — mirror dos métodos do `#[scope]` de `Content` do vanilla
/// (`foundations/content/mod.rs:510-590`): `func()`, `has(field)`,
/// `at(field, default:?)`, `fields()`, `location()`. Mensagens de erro de
/// argumentos verbatim (medidas b13–b17): `missing argument: field`,
/// `expected string, found {tipo}`, `unexpected argument`,
/// `unexpected argument: {nome}`.
///
/// `location()` devolve sempre `none`: o cristalino não retém metadados de
/// location em `Content` (medido: content inline → none nos dois binários;
/// content de show rule/query → `location(..)` no vanilla — divergência
/// registada no L0, requer introspecção de locations, fora do proporcional).
pub(in crate::compiler::eval) fn eval_content_method(
    c: &crate::entities::content::Content,
    method: &str,
    mut args: Args,
    span: Span,
) -> SourceResult<Value> {
    match method {
        "func" => {
            finish_args(&args, span)?;
            Ok(content_elem_func(c))
        }
        "fields" => {
            finish_args(&args, span)?;
            let mut dict: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
            for (name, value) in content_set_fields(c) {
                dict.insert(name.into(), value);
            }
            Ok(Value::Dict(dict))
        }
        "location" => {
            finish_args(&args, span)?;
            Ok(Value::None)
        }
        "has" | "at" => {
            let field_value = expect_positional(&mut args, span, "field")?;
            let field = match field_value {
                Value::Str(s) => s,
                other => {
                    return Err(vec![SourceDiagnostic::error(
                        span,
                        format!("expected string, found {}", vanilla_type_name(&other)),
                    )])
                }
            };
            let default =
                if method == "at" { args.named.shift_remove("default") } else { None };
            finish_args(&args, span)?;
            let field = field.as_str();
            match method {
                "has" => Ok(Value::Bool(matches!(
                    content_field(c, field),
                    ContentField::Set(_)
                ))),
                _ => match content_field(c, field) {
                    ContentField::Set(v) => Ok(v),
                    ContentField::Unset => match default {
                        Some(v) => Ok(v),
                        None => Err(vec![SourceDiagnostic::error(
                            span,
                            format!(
                                "field \"{field}\" in {} is not known at this point and no default was specified",
                                c.elem_name()
                            ),
                        )]),
                    },
                    ContentField::Undeclared => match default {
                        Some(v) => Ok(v),
                        None => Err(vec![SourceDiagnostic::error(
                            span,
                            format!(
                                "{} does not have field \"{field}\" and no default was specified",
                                c.elem_name()
                            ),
                        )]),
                    },
                },
            }
        }
        _ => unreachable!("eval_content_method: método desconhecido {method}"),
    }
}
