//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/bindings/field_access/content_methods.md
//! @prompt-hash 9091b26d
//! @layer L1
//! @updated 2026-10-07

use super::*;

// ── P829 — métodos de `content`: func/has/at/fields/location ───────────────

/// **P829** — estado de um campo de content para os métodos `has`/`at`/
/// `fields` (paridade vanilla `Content::has`/`at`/`fields`,
/// `foundations/content/mod.rs:510-590`): o vanilla distingue campo
/// **assente no constructor** de default resolvido pela chain — medido:
/// `heading[H].has("level")` → false, `heading(level: 2)[H].has("level")` →
/// true, markup `= H` assenta `depth` (não `level`).
pub(super) enum ContentField {
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
/// (P829), a máscara de Raw e os carriers de presença dos demais elementos
/// preservam "explicitamente assente". O fallback via `get_field` só é usado
/// nas famílias sem distinção adicional contratada. `label` fica
/// scope-out: no cristalino a label é um nó irmão (`Content::Label`), não
/// metadado do elemento — divergência registada no L0.
pub(super) fn content_field(
    c: &crate::entities::content::Content,
    field: &str,
) -> ContentField {
    use crate::entities::content::Content;
    use crate::entities::elements::heading::{
        HEADING_SET_DEPTH, HEADING_SET_LEVEL, HEADING_SET_OUTLINED,
    };
    match c {
        Content::Source(e) => content_field(&e.body, field),
        Content::Located(e) => content_field(&e.body, field),
        Content::ShowRecipe(e) => content_field(&e.body, field),
        Content::Realized(e) => content_field(&e.visual, field),
        Content::Styled(styled) if equation_descendant(&styled.body).is_some() => {
            let styles = &styled.styles;
            let child = &styled.body;
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
        Content::Transform(e) => {
            use crate::entities::elements::transform::TransformOperation;
            let declared = match e.operation {
                TransformOperation::Matrix(_) => false,
                TransformOperation::Move { .. } => matches!(field, "dx" | "dy" | "body"),
                TransformOperation::Rotate { .. } => {
                    matches!(field, "angle" | "origin" | "reflow" | "body")
                }
                TransformOperation::Scale { .. } => {
                    matches!(field, "x" | "y" | "origin" | "reflow" | "body")
                }
                TransformOperation::Skew { .. } => {
                    matches!(field, "ax" | "ay" | "origin" | "reflow" | "body")
                }
            };
            if declared {
                c.get_field(field)
                    .map(ContentField::Set)
                    .unwrap_or(ContentField::Unset)
            } else {
                ContentField::Undeclared
            }
        }
        Content::Raw(_) => match field {
            "text" | "block" | "lang" => c
                .get_field(field)
                .map(ContentField::Set)
                .unwrap_or(ContentField::Unset),
            _ => ContentField::Undeclared,
        },
        Content::MathMatrix(_) => match field {
            "rows" | "delim" | "align" | "augment" | "row-gap" | "column-gap" => c
                .get_field(field)
                .map(ContentField::Set)
                .unwrap_or(ContentField::Unset),
            _ => ContentField::Undeclared,
        },
        Content::TableCell(_) => match field {
            "body" | "x" | "y" | "colspan" | "rowspan" | "inset" | "align" | "fill"
            | "stroke" | "breakable" => c
                .get_field(field)
                .map(ContentField::Set)
                .unwrap_or(ContentField::Unset),
            _ => ContentField::Undeclared,
        },
        Content::SmartQuote(e) => {
            let present = match field {
                "double" => e.double.is_some(),
                "enabled" => e.enabled.is_some(),
                "alternative" => e.alternative.is_some(),
                "quotes" => e.quotes.is_some(),
                _ => return ContentField::Undeclared,
            };
            if present {
                ContentField::Set(c.get_field(field).expect("declared quote field"))
            } else {
                ContentField::Unset
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
pub(super) fn content_set_fields(
    c: &crate::entities::content::Content,
) -> Vec<(&'static str, Value)> {
    use crate::entities::content::Content;
    let c = c.source_body();
    let candidates: &[&'static str] = match c {
        Content::FigureCaption(_) => &["position", "separator", "body"],
        Content::MathMatrix(_) => {
            &["delim", "align", "augment", "row-gap", "column-gap", "rows"]
        }
        Content::TableCell(_) => &[
            "body",
            "x",
            "y",
            "colspan",
            "rowspan",
            "inset",
            "align",
            "fill",
            "stroke",
            "breakable",
        ],
        Content::SmartQuote(_) => &["double", "enabled", "alternative", "quotes"],
        Content::Heading(_) => &["level", "depth", "outlined", "bookmarked", "body"],
        Content::Equation(_) => &["block", "body"],
        Content::MathPrimes(_) => &["count"],
        Content::MathLr(e) if e.size_explicit => &["size", "body"],
        Content::MathLr(_) => &["body"],
        Content::MathUnderbrace(e) if e.annotation_explicit => &["body", "annotation"],
        Content::MathUnderbrace(_) => &["body"],
        Content::MathUnderbracket(e) if e.annotation_explicit => &["body", "annotation"],
        Content::MathUnderbracket(_) => &["body"],
        Content::MathUnderparen(e) if e.annotation_explicit => &["body", "annotation"],
        Content::MathUnderparen(_) => &["body"],
        Content::MathUndershell(e) if e.annotation_explicit => &["body", "annotation"],
        Content::MathUndershell(_) => &["body"],
        Content::MathStretch(e) if e.size_explicit => &["body", "size"],
        Content::MathStretch(_) => &["body"],
        Content::Styled(styled) if equation_descendant(&styled.body).is_some() => {
            &["block", "numbering", "number-align", "supplement", "alt", "body"]
        }
        Content::Text(_) => &["text"],
        Content::Raw(_) => &["text", "block", "lang"],
        Content::Transform(e) => match e.operation {
            crate::entities::elements::transform::TransformOperation::Matrix(_) => &[],
            crate::entities::elements::transform::TransformOperation::Move { .. } => {
                &["dx", "dy", "body"]
            }
            crate::entities::elements::transform::TransformOperation::Rotate {
                ..
            } => &["angle", "origin", "reflow", "body"],
            crate::entities::elements::transform::TransformOperation::Scale {
                ..
            } => &["x", "y", "origin", "reflow", "body"],
            crate::entities::elements::transform::TransformOperation::Skew { .. } => {
                &["ax", "ay", "origin", "reflow", "body"]
            }
        },
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

pub(super) fn equation_descendant(
    content: &crate::entities::content::Content,
) -> Option<&crate::entities::elements::equation::EquationElem> {
    use crate::entities::content::Content;
    match content {
        Content::Equation(e) => Some(e),
        Content::Styled(styled) => equation_descendant(&styled.body),
        _ => None,
    }
}

/// **P829** — fallback de `func()` para variantes sem constructor nativo
/// exposto (Sequence, Styled, Label, math, Dynamic, …): o `Value::Func`
/// existe (repr = nome do elemento — paridade medida de `func()`) mas a
/// chamada não é suportada. Caso não medido no vanilla (elementos internos
/// não expostos no scope); mensagem própria, registada no L0.
pub(super) fn content_func_not_callable(
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
pub(super) fn content_elem_func(c: &crate::entities::content::Content) -> Value {
    use crate::entities::content::Content;
    let call: fn(
        &mut crate::compiler::eval::EvalContext,
        &Args,
        &dyn crate::contracts::world::World,
        crate::entities::file_id::FileId,
    ) -> SourceResult<Value> = match c {
        Content::Text(_) => crate::compiler::stdlib::native_text,
        Content::MathMatrix(_) => crate::compiler::stdlib::native_math_mat,
        Content::TableCell(_) => crate::compiler::stdlib::native_table_cell,
        Content::Table(_) => crate::compiler::stdlib::native_table,
        Content::Strong(_) => crate::compiler::stdlib::native_strong,
        Content::Emph(_) => crate::compiler::stdlib::native_emph,
        Content::Heading(_) => crate::compiler::stdlib::native_heading,
        Content::Title(_) => crate::compiler::stdlib::native_title,
        Content::Raw(_) => crate::compiler::stdlib::native_raw,
        Content::Transform(e) => match e.operation {
            crate::entities::elements::transform::TransformOperation::Matrix(_) => {
                content_func_not_callable
            }
            crate::entities::elements::transform::TransformOperation::Move { .. } => {
                crate::compiler::stdlib::native_move
            }
            crate::entities::elements::transform::TransformOperation::Rotate {
                ..
            } => crate::compiler::stdlib::native_rotate,
            crate::entities::elements::transform::TransformOperation::Scale {
                ..
            } => crate::compiler::stdlib::native_scale,
            crate::entities::elements::transform::TransformOperation::Skew { .. } => {
                crate::compiler::stdlib::native_skew
            }
        },
        Content::Figure(_) => crate::compiler::stdlib::native_figure,
        Content::FigureCaption(_) => crate::compiler::stdlib::native_figure_caption,
        Content::Link(_) => crate::compiler::stdlib::native_link,
        Content::SmallCaps { .. } => crate::compiler::stdlib::native_smallcaps,
        Content::Linebreak(_) => crate::compiler::stdlib::native_linebreak,
        Content::OutlineEntry(_) => crate::compiler::stdlib::native_outline_entry,
        _ => content_func_not_callable,
    };
    Value::Func(Func::native(c.elem_name(), call))
}

pub(crate) fn eval_outline_entry_body_method(
    content: &crate::entities::content::Content,
    args: Args,
    span: Span,
) -> SourceResult<Value> {
    finish_args(&args, span)?;
    let crate::entities::content::Content::OutlineEntry(entry) = content else {
        unreachable!("receiver validated as outline entry")
    };
    entry
        .body()
        .map(Value::Content)
        .map_err(|message| vec![SourceDiagnostic::error(span, message)])
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
pub(crate) fn eval_content_method(
    c: &crate::entities::content::Content,
    method: &str,
    args: Args,
    span: Span,
) -> SourceResult<Value> {
    eval_content_method_at(c, None, method, args, span)
}

pub(crate) fn eval_content_method_at(
    c: &crate::entities::content::Content,
    location: Option<crate::entities::location::Location>,
    method: &str,
    args: Args,
    span: Span,
) -> SourceResult<Value> {
    content_method_with_fields(c, location, None, method, args, span)
}

pub(crate) fn eval_introspected_content_method_at(
    c: &crate::entities::value::IntrospectedContent,
    location: crate::entities::location::Location,
    method: &str,
    args: Args,
    span: Span,
) -> SourceResult<Value> {
    content_method_with_fields(
        c.content(),
        Some(location),
        c.fields(),
        method,
        args,
        span,
    )
}

pub(super) fn content_method_with_fields(
    c: &crate::entities::content::Content,
    location: Option<crate::entities::location::Location>,
    fields: Option<&IndexMap<EcoString, Value, FxBuildHasher>>,
    method: &str,
    mut args: Args,
    span: Span,
) -> SourceResult<Value> {
    use crate::entities::content::Content;
    match c {
        Content::Source(e) => {
            return content_method_with_fields(
                &e.body, location, fields, method, args, span,
            )
        }
        Content::Located(e) => {
            return content_method_with_fields(
                &e.body,
                location.or(Some(e.location)),
                fields,
                method,
                args,
                span,
            )
        }
        Content::ShowRecipe(e) => {
            return content_method_with_fields(
                &e.body, location, fields, method, args, span,
            )
        }
        Content::Realized(e) => {
            return content_method_with_fields(
                &e.visual, location, fields, method, args, span,
            )
        }
        _ => {}
    }
    match method {
        "func" => {
            finish_args(&args, span)?;
            Ok(content_elem_func(c))
        }
        "fields" => {
            finish_args(&args, span)?;
            if let Some(fields) = fields {
                return Ok(Value::Dict(fields.clone()));
            }
            let mut dict: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
            for (name, value) in content_set_fields(c) {
                dict.insert(name.into(), value);
            }
            Ok(Value::Dict(dict))
        }
        "location" => {
            finish_args(&args, span)?;
            Ok(location.map(Value::Location).unwrap_or(Value::None))
        }
        "has" | "at" => {
            let field_value = expect_positional(&mut args, span, "field")?;
            let field = match field_value {
                Value::Str(s) => s,
                other => {
                    return Err(vec![SourceDiagnostic::error(
                        span,
                        format!("expected string, found {}", vanilla_type_name(&other)),
                    )]);
                }
            };
            let default =
                if method == "at" { args.remove_named("default") } else { None };
            finish_args(&args, span)?;
            let field = field.as_str();
            let resolved = match fields {
                Some(fields) => fields
                    .get(field)
                    .cloned()
                    .map(ContentField::Set)
                    .unwrap_or(ContentField::Undeclared),
                None => content_field(c, field),
            };
            match method {
                "has" => Ok(Value::Bool(matches!(resolved, ContentField::Set(_)))),
                _ => match resolved {
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

pub(super) fn static_content_receiver(
    args: &Args,
    name: &str,
) -> SourceResult<(Value, Args)> {
    let Some(first) = args.items.first() else {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!("{name} requires content as its first argument"),
        )]);
    };
    let content = match first {
        Value::Content(_) | Value::LocatedContent(_, _) => first.clone(),
        _ => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("{name} requires content as its first argument"),
            )]);
        }
    };
    let mut rest = args.clone();
    rest.remove_positional(0);
    Ok((content, rest))
}

pub(in crate::compiler::eval) fn content_type_field(field: &str) -> Option<Value> {
    let (name, call) = match field {
        "func" => ("content.func", native_content_func_static as _),
        "has" => ("content.has", native_content_has_static as _),
        "at" => ("content.at", native_content_at_static as _),
        "fields" => ("content.fields", native_content_fields_static as _),
        "location" => ("content.location", native_content_location_static as _),
        _ => return None,
    };
    Some(Value::Func(Func::native(name, call)))
}

pub(super) fn static_content_method(
    args: &Args,
    name: &str,
    method: &str,
) -> SourceResult<Value> {
    let (content, rest) = static_content_receiver(args, name)?;
    match content {
        Value::Content(content) => {
            eval_content_method_at(&content, None, method, rest, args.span)
        }
        Value::LocatedContent(content, location) => eval_introspected_content_method_at(
            &content, location, method, rest, args.span,
        ),
        _ => unreachable!("receiver was validated as content"),
    }
}

macro_rules! static_content_native {
    ($fn_name:ident, $public_name:literal, $method:literal) => {
        fn $fn_name(
            _ctx: &mut EvalContext,
            args: &Args,
            _world: &dyn crate::contracts::world::World,
            _file: crate::entities::file_id::FileId,
        ) -> SourceResult<Value> {
            static_content_method(args, $public_name, $method)
        }
    };
}

static_content_native!(native_content_func_static, "content.func", "func");
static_content_native!(native_content_has_static, "content.has", "has");
static_content_native!(native_content_at_static, "content.at", "at");
static_content_native!(native_content_fields_static, "content.fields", "fields");
static_content_native!(native_content_location_static, "content.location", "location");

#[cfg(test)]
mod matrix_morphology_tests {
    use super::*;
    #[test]
    fn matrix_morphology_fields_presence_and_native_identity() {
        let matrix = crate::entities::content::Content::math_matrix(
            vec![vec![crate::entities::content::Content::text("a")]],
            ('(', ')'),
        );
        assert_eq!(
            content_set_fields(&matrix)
                .iter()
                .map(|(n, _)| *n)
                .collect::<Vec<_>>(),
            vec!["delim", "rows"]
        );
        assert!(matches!(content_field(&matrix, "align"), ContentField::Unset));
        assert!(matches!(content_field(&matrix, "gap"), ContentField::Undeclared));
        let Value::Func(projected) = content_elem_func(&matrix) else {
            panic!("not function")
        };
        let Value::Module(math) = crate::compiler::stdlib::make_math_module() else {
            panic!("not module")
        };
        let Some(Value::Func(native)) = math.scope().get("mat") else {
            panic!("missing native")
        };
        assert!(std::ptr::fn_addr_eq(
            projected.native_fn_addr().unwrap(),
            native.native_fn_addr().unwrap()
        ));
    }
}

#[cfg(test)]
mod caption_shape_tests {
    use super::*;
    #[test]
    fn caption_shape_fields_and_function_use_native_identity() {
        use crate::entities::content::Content;
        use crate::entities::elements::figure_caption::CaptionPosition;
        let c = Content::figure_caption(
            Content::text("C"),
            Some(CaptionPosition::Top),
            Some(Content::text("S")),
        );
        let fields = content_set_fields(&c);
        assert_eq!(
            fields.iter().map(|(n, _)| *n).collect::<Vec<_>>(),
            vec!["position", "separator", "body"]
        );
        let Value::Func(f) = content_elem_func(&c) else { panic!("expected function") };
        assert!(std::ptr::fn_addr_eq(
            f.native_fn_addr().unwrap(),
            crate::compiler::stdlib::native_figure_caption as fn(_, _, _, _) -> _
        ));
        let (name, fields, locatable) =
            crate::compiler::eval::bindings::native_element_fields(&f).unwrap();
        assert_eq!(name, "caption");
        assert!(fields.contains(&"body"));
        assert!(fields.contains(&"kind"));
        assert!(locatable);
    }
}
