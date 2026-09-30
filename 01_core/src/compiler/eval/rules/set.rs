//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/rules/set.md
//! @prompt-hash ebac4395
//! @layer L1
//! @updated 2026-09-30
//!
//! Comportamento de `eval/rules::set`.

use super::*;

mod page;
mod text;

use self::page::eval_set_page;
use self::text::eval_set_text;

pub(crate) fn eval_set_rule(
    set: SetRule<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    // Extrair target — deve ser um Ident (ex: "text").
    // Targets suportados: heading, page, figure, text. Outros emitem
    // warning via Sink (Passo 107, encerra DEBT-49).
    let target = set.target().to_untyped().text_str().to_owned();
    let target_span = set.target().to_untyped().span();

    if target == "outline" {
        for arg in set.args().items() {
            let Arg::Named(named) = arg else {
                return Err(vec![SourceDiagnostic::error(
                    target_span,
                    "unexpected argument",
                )]);
            };
            let key = named.name().as_str();
            if key != "indent" {
                return Err(vec![SourceDiagnostic::error(
                    named.name().to_untyped().span(),
                    format!("unexpected argument: {key}"),
                )]);
            }
            let value = eval_expr(named.expr(), scopes, ctx, engine)?;
            if !matches!(
                value,
                Value::Length(_) | Value::Func(_) | Value::Auto | Value::Bool(_)
            ) {
                return Err(vec![type_mismatch(
                    "length, function, auto, or bool",
                    &value,
                    named.expr().span(),
                )]);
            }
            *engine.styles = engine.styles.push_custom("outline.indent", value);
        }
        return Ok(Value::None);
    }

    if target == "heading" {
        // #set heading(numbering: "1.1") — activa numeração automática.
        // Outros argumentos de heading ignorados por agora (DEBT-10).
        let mut active = false;
        let mut pattern: Option<ecow::EcoString> = None;
        for arg in set.args().items() {
            if let Arg::Named(named) = arg {
                if named.name().as_str() == "numbering" {
                    // Defensivo: só String activa a numeração.
                    // Closures, none, ou outros tipos → ignorar.
                    let val = eval_expr(named.expr(), scopes, ctx, engine)
                        .unwrap_or(Value::None);
                    if let Value::Str(s) = val {
                        active = true;
                        pattern = Some(s.clone());
                    }
                }
            }
        }
        // Lote F-2 S1 (P335): em vez de um marcador global `SetHeadingNumbering`,
        // empurra para a chain léxica (`engine.styles` é escopado por
        // `local_styles`). O heading assa este valor na criação
        // (`eval/markup.rs`). Fecha o canal global → escopo de container (DEBT 99.E).
        // P451: transporta também o pattern string para formatação configurável.
        *engine.styles =
            engine.styles.push_custom("heading.numbering", Value::Bool(active));
        if let Some(pattern) = pattern {
            *engine.styles = engine
                .styles
                .push_custom("heading.numbering.pattern", Value::Str(pattern));
        }
        return Ok(Value::None);
    }

    // Lote F-2 S2 / **B1** (P335): `#set math.equation(numbering:)` — target
    // **pontuado** (`FieldAccess` `math.equation`), que `text_str()` não capta
    // (nó interno → ""). Era a lacuna do P331 (sem produtor eval). Empurra para
    // a chain léxica; a equação assa o valor (`eval/mod.rs`). Paridade vanilla:
    // `lab/.../math/equation.rs` — numeração de equação de bloco.
    // P456: guarda o pattern string (não Bool) para formatar via format_counter,
    // análogo a figure.numbering (P454) e heading.numbering.pattern (P451).
    if let Expr::FieldAccess(fa) = set.target() {
        if fa.target().to_untyped().text_str() == "format"
            && fa.field().to_untyped().text_str() == "html"
        {
            if !ctx
                .features
                .contains(crate::entities::compiler_features::Feature::Html)
            {
                return Err(vec![SourceDiagnostic::error(
                    fa.field().to_untyped().span(),
                    "cannot access field `html` because the `html` feature is not enabled",
                )
                .with_hint("try enabling the `html` feature")
                .with_hint(
                    "see https://typst.app/help/compiler-features for more details",
                )]);
            }

            let mut seen_pretty = false;
            for arg in set.args().items() {
                let value = match arg {
                    Arg::Pos(expr) => eval_expr(expr, scopes, ctx, engine)?,
                    Arg::Named(named) => eval_expr(named.expr(), scopes, ctx, engine)?,
                    Arg::Spread(spread) => eval_expr(spread.expr(), scopes, ctx, engine)?,
                };

                let Arg::Named(named) = arg else {
                    return Err(vec![SourceDiagnostic::error(
                        arg.to_untyped().span(),
                        "unexpected argument",
                    )]);
                };
                let key = named.name().as_str();
                if key != "pretty" {
                    return Err(vec![SourceDiagnostic::error(
                        named.name().to_untyped().span(),
                        format!("unexpected argument: {key}"),
                    )]);
                }
                if seen_pretty {
                    return Err(vec![SourceDiagnostic::error(
                        named.name().to_untyped().span(),
                        "duplicate argument: pretty",
                    )]);
                }
                seen_pretty = true;

                let Value::Bool(pretty) = value else {
                    let found = match &value {
                        Value::Int(_) => "integer",
                        Value::Str(_) => "string",
                        Value::Bool(_) => "boolean",
                        other => other.type_name(),
                    };
                    return Err(vec![SourceDiagnostic::error(
                        named.expr().span(),
                        format!("expected boolean, found {found}"),
                    )]);
                };
                *engine.styles =
                    engine.styles.push_custom("format.html.pretty", Value::Bool(pretty));
            }
            return Ok(Value::None);
        }
        if fa.target().to_untyped().text_str() == "format"
            && fa.field().to_untyped().text_str() == "svg"
        {
            let mut seen_pretty = false;
            for arg in set.args().items() {
                let Arg::Named(named) = arg else {
                    return Err(vec![SourceDiagnostic::error(
                        target_span,
                        "unexpected argument",
                    )]);
                };
                let key = named.name().as_str();
                if key != "pretty" {
                    return Err(vec![SourceDiagnostic::error(
                        named.name().to_untyped().span(),
                        format!("unexpected argument: {key}"),
                    )]);
                }
                if seen_pretty {
                    return Err(vec![SourceDiagnostic::error(
                        named.name().to_untyped().span(),
                        "duplicate argument: pretty",
                    )]);
                }
                seen_pretty = true;

                let value = eval_expr(named.expr(), scopes, ctx, engine)?;
                let Value::Bool(pretty) = value else {
                    let found = match &value {
                        Value::Int(_) => "integer",
                        Value::Str(_) => "string",
                        Value::Bool(_) => "boolean",
                        other => other.type_name(),
                    };
                    return Err(vec![SourceDiagnostic::error(
                        named.expr().span(),
                        format!("expected boolean, found {found}"),
                    )]);
                };
                *engine.styles =
                    engine.styles.push_custom("format.svg.pretty", Value::Bool(pretty));
            }
            return Ok(Value::None);
        }
        if fa.target().to_untyped().text_str() == "format"
            && fa.field().to_untyped().text_str() == "png"
        {
            let mut ppi = None;
            for arg in set.args().items() {
                let Arg::Named(named) = arg else {
                    return Err(vec![SourceDiagnostic::error(
                        target_span,
                        "unexpected argument",
                    )]);
                };
                let key = named.name().as_str();
                if key != "ppi" {
                    return Err(vec![SourceDiagnostic::error(
                        named.name().to_untyped().span(),
                        format!("unexpected argument: {key}"),
                    )]);
                }
                if ppi.is_some() {
                    return Err(vec![SourceDiagnostic::error(
                        named.name().to_untyped().span(),
                        "duplicate argument: ppi",
                    )]);
                }

                let value = eval_expr(named.expr(), scopes, ctx, engine)?;
                let value = match value {
                    Value::Int(value) => value as f64,
                    Value::Float(value) => value,
                    other => {
                        let found = match &other {
                            Value::Int(_) => "integer",
                            Value::Str(_) => "string",
                            Value::Bool(_) => "boolean",
                            value => value.type_name(),
                        };
                        return Err(vec![SourceDiagnostic::error(
                            named.expr().span(),
                            format!("expected float, found {found}"),
                        )]);
                    }
                };
                ppi = Some(value);
            }
            if let Some(ppi) = ppi {
                *engine.styles =
                    engine.styles.push_custom("format.png.ppi", Value::Float(ppi));
            }
            return Ok(Value::None);
        }
        if fa.target().to_untyped().text_str() == "format"
            && fa.field().to_untyped().text_str() == "pdf"
        {
            let mut seen_tagged = false;
            let mut seen_pretty = false;
            let mut seen_pages = false;
            let mut seen_standard = false;
            for arg in set.args().items() {
                let Arg::Named(named) = arg else {
                    return Err(vec![SourceDiagnostic::error(
                        target_span,
                        "unexpected argument",
                    )]);
                };
                let key = named.name().as_str();
                if key != "tagged"
                    && key != "pretty"
                    && key != "pages"
                    && key != "standard"
                {
                    return Err(vec![SourceDiagnostic::error(
                        named.name().to_untyped().span(),
                        format!("unexpected argument: {key}"),
                    )]);
                }

                if key == "tagged" {
                    if seen_tagged {
                        return Err(vec![SourceDiagnostic::error(
                            named.name().to_untyped().span(),
                            "duplicate argument: tagged",
                        )]);
                    }
                    seen_tagged = true;

                    let value = eval_expr(named.expr(), scopes, ctx, engine)?;
                    if !matches!(value, Value::Auto | Value::Bool(_)) {
                        let found = match &value {
                            Value::Int(_) => "integer",
                            Value::Str(_) => "string",
                            Value::Bool(_) => "boolean",
                            other => other.type_name(),
                        };
                        return Err(vec![SourceDiagnostic::error(
                            named.expr().span(),
                            format!("expected boolean or auto, found {found}"),
                        )]);
                    }
                    *engine.styles =
                        engine.styles.push_custom("format.pdf.tagged", value);
                } else if key == "pretty" {
                    if seen_pretty {
                        return Err(vec![SourceDiagnostic::error(
                            named.name().to_untyped().span(),
                            "duplicate argument: pretty",
                        )]);
                    }
                    seen_pretty = true;

                    let value = eval_expr(named.expr(), scopes, ctx, engine)?;
                    let Value::Bool(pretty) = value else {
                        let found = match &value {
                            Value::Int(_) => "integer",
                            Value::Str(_) => "string",
                            Value::Bool(_) => "boolean",
                            other => other.type_name(),
                        };
                        return Err(vec![SourceDiagnostic::error(
                            named.expr().span(),
                            format!("expected boolean, found {found}"),
                        )]);
                    };
                    *engine.styles = engine
                        .styles
                        .push_custom("format.pdf.pretty", Value::Bool(pretty));
                } else if key == "pages" {
                    // **P1468** — `pages`: `Value::Int`/`Dict`/`Array`/`None`,
                    // validado por completo aqui (mesmo momento do `set` que
                    // `tagged`/`pretty`); o `Value` já validado é inserido tal
                    // como recebido — a conversão para `PageRanges` tipado é
                    // feita por `compiler/layout/format_options.rs`.
                    if seen_pages {
                        return Err(vec![SourceDiagnostic::error(
                            named.name().to_untyped().span(),
                            "duplicate argument: pages",
                        )]);
                    }
                    seen_pages = true;

                    let value = eval_expr(named.expr(), scopes, ctx, engine)?;
                    let span = named.expr().span();
                    match &value {
                        Value::None => {}
                        Value::Int(_) | Value::Dict(_) => {
                            validate_pdf_page_range_element(&value, span)?;
                        }
                        Value::Array(items) => {
                            for item in items {
                                validate_pdf_page_range_element(item, span)?;
                            }
                        }
                        other => {
                            return Err(vec![SourceDiagnostic::error(
                                span,
                                format!(
                                    "expected integer, dictionary, array, or none, found {}",
                                    pdf_pages_found_name(other)
                                ),
                            )]);
                        }
                    }
                    *engine.styles = engine.styles.push_custom("format.pdf.pages", value);
                } else {
                    // **P1469** (escopo reduzido — contrato de linguagem) —
                    // `standard`: `Value::Str`/`Array<Str>`, validado por
                    // completo aqui (parsing dos 17 nomes + conflitos via
                    // `PdfStandards::new`, só para validar — o valor
                    // armazenado é o `Value::Array` normalizado, nunca o
                    // tipo L1). Não aceita `Value::None` (diferente de
                    // `pages`): a única forma de "documento não decide" é a
                    // ausência do named.
                    if seen_standard {
                        return Err(vec![SourceDiagnostic::error(
                            named.name().to_untyped().span(),
                            "duplicate argument: standard",
                        )]);
                    }
                    seen_standard = true;

                    let value = eval_expr(named.expr(), scopes, ctx, engine)?;
                    let span = named.expr().span();

                    let names: Vec<PdfStandard> = match &value {
                        Value::Str(s) => {
                            let Some(standard) = pdf_standard_from_name(s.as_str())
                            else {
                                return Err(vec![SourceDiagnostic::error(
                                    span,
                                    format!(
                                        "expected {}",
                                        pdf_standard_expected_list(true)
                                    ),
                                )]);
                            };
                            vec![standard]
                        }
                        Value::Array(items) => {
                            let mut collected = Vec::with_capacity(items.len());
                            for item in items {
                                match item {
                                    Value::Str(s) => {
                                        let Some(standard) =
                                            pdf_standard_from_name(s.as_str())
                                        else {
                                            return Err(vec![SourceDiagnostic::error(
                                                span,
                                                format!(
                                                    "expected {}",
                                                    pdf_standard_expected_list(false)
                                                ),
                                            )]);
                                        };
                                        collected.push(standard);
                                    }
                                    other => {
                                        return Err(vec![SourceDiagnostic::error(
                                            span,
                                            format!(
                                                "expected {}, found {}",
                                                pdf_standard_expected_list(false),
                                                pdf_pages_found_name(other)
                                            ),
                                        )]);
                                    }
                                }
                            }
                            collected
                        }
                        other => {
                            return Err(vec![SourceDiagnostic::error(
                                span,
                                format!(
                                    "expected {}, found {}",
                                    pdf_standard_expected_list(true),
                                    pdf_pages_found_name(other)
                                ),
                            )]);
                        }
                    };

                    if let Err(conflict) = PdfStandards::new(names.iter().copied()) {
                        return Err(vec![pdf_standards_conflict_diagnostic(
                            span, conflict,
                        )]);
                    }

                    let normalized = match &value {
                        Value::Str(_) => Value::Array(vec![value.clone()]),
                        _ => value.clone(),
                    };
                    *engine.styles =
                        engine.styles.push_custom("format.pdf.standard", normalized);
                }
            }
            return Ok(Value::None);
        }
        if fa.target().to_untyped().text_str() == "par"
            && fa.field().to_untyped().text_str() == "line"
        {
            use crate::entities::layout_types::{Align2D, HAlign};

            for arg in set.args().items() {
                let Arg::Named(named) = arg else {
                    return Err(vec![SourceDiagnostic::error(
                        target_span,
                        "unexpected argument",
                    )]);
                };
                let key = named.name().as_str();
                let span = named.expr().span();
                let value = eval_expr(named.expr(), scopes, ctx, engine)?;
                match key {
                    "numbering" => {
                        if !matches!(&value, Value::Str(_) | Value::Func(_) | Value::None)
                        {
                            return Err(vec![type_mismatch(
                                "string, function, or none",
                                &value,
                                span,
                            )]);
                        }
                    }
                    "number-align" => match &value {
                        Value::Auto => {}
                        Value::Align(Align2D { h: Some(_), v: None }) => {}
                        _ => {
                            return Err(vec![type_mismatch(
                                "horizontal alignment or auto",
                                &value,
                                span,
                            )]);
                        }
                    },
                    "number-margin" => match &value {
                        Value::Align(Align2D {
                            h:
                                Some(
                                    HAlign::Left
                                    | HAlign::Right
                                    | HAlign::Start
                                    | HAlign::End,
                                ),
                            v: None,
                        }) => {}
                        _ => {
                            return Err(vec![type_mismatch(
                                "left, right, start, or end",
                                &value,
                                span,
                            )]);
                        }
                    },
                    "number-clearance" => {
                        if !matches!(&value, Value::Length(_) | Value::Auto) {
                            return Err(vec![type_mismatch(
                                "length or auto",
                                &value,
                                span,
                            )]);
                        }
                    }
                    "numbering-scope" => match &value {
                        Value::Str(scope)
                            if matches!(scope.as_str(), "document" | "page") => {}
                        Value::Str(_) => {
                            return Err(vec![SourceDiagnostic::error(
                                span,
                                "expected \"document\" or \"page\"",
                            )]);
                        }
                        _ => return Err(vec![type_mismatch("string", &value, span)]),
                    },
                    _ => {
                        return Err(vec![SourceDiagnostic::error(
                            named.name().to_untyped().span(),
                            format!("unexpected argument: {key}"),
                        )]);
                    }
                }
                *engine.styles =
                    engine.styles.push_custom(format!("par.line.{key}"), value);
            }
            return Ok(Value::None);
        }
        if fa.target().to_untyped().text_str() == "outline"
            && fa.field().to_untyped().text_str() == "entry"
        {
            for arg in set.args().items() {
                let Arg::Named(named) = arg else {
                    return Err(vec![SourceDiagnostic::error(
                        target_span,
                        "unexpected argument",
                    )]);
                };
                let key = named.name().as_str();
                if key != "fill" {
                    return Err(vec![SourceDiagnostic::error(
                        named.name().to_untyped().span(),
                        format!("unexpected argument: {key}"),
                    )]);
                }
                let value = eval_expr(named.expr(), scopes, ctx, engine)?;
                if !matches!(value, Value::Content(_) | Value::None) {
                    return Err(vec![type_mismatch(
                        "content or none",
                        &value,
                        named.expr().span(),
                    )]);
                }
                *engine.styles = engine.styles.push_custom("outline.entry.fill", value);
            }
            return Ok(Value::None);
        }
        if fa.target().to_untyped().text_str() == "math"
            && fa.field().to_untyped().text_str() == "equation"
        {
            let numbering = set.args().items().find_map(|arg| {
                if let Arg::Named(named) = arg {
                    if named.name().as_str() == "numbering" {
                        // P636: propagar erro de avaliação (variável indefinida,
                        // por exemplo) em vez de descartar com `.ok()`.
                        return Some((
                            named.expr().span(),
                            eval_expr(named.expr(), scopes, ctx, engine),
                        ));
                    }
                }
                None
            });
            match numbering {
                Some((_, Ok(Value::Str(s)))) => {
                    *engine.styles =
                        engine.styles.push_custom("equation.numbering", Value::Str(s));
                }
                Some((_, Ok(Value::None))) => {
                    // Limpa a numeração no escopo (None = ausente).
                    *engine.styles =
                        engine.styles.push_custom("equation.numbering", Value::None);
                }
                Some((_, Ok(Value::Func(f)))) => {
                    *engine.styles =
                        engine.styles.push_custom("equation.numbering", Value::Func(f));
                }
                Some((span, Ok(other))) => {
                    return Err(vec![type_mismatch(
                        "string, function, or none",
                        &other,
                        span,
                    )]);
                }
                Some((_, Err(err))) => return Err(err),
                None => {} // neutro: N16[β] — argumento numbering omitido em set-rule: estilos inalterados
            }
            let number_align = set.args().items().find_map(|arg| {
                if let Arg::Named(named) = arg {
                    if named.name().as_str() == "number-align" {
                        return Some((
                            named.expr().span(),
                            eval_expr(named.expr(), scopes, ctx, engine),
                        ));
                    }
                }
                None
            });
            match number_align {
                Some((span, Ok(value))) => {
                    crate::compiler::stdlib::equation_number_align(&value, span)?;
                    *engine.styles =
                        engine.styles.push_custom("equation.number-align", value);
                }
                Some((_, Err(err))) => return Err(err),
                None => {}
            }
            let supplement = set.args().items().find_map(|arg| {
                if let Arg::Named(named) = arg {
                    if named.name().as_str() == "supplement" {
                        return Some((
                            named.expr().span(),
                            eval_expr(named.expr(), scopes, ctx, engine),
                        ));
                    }
                }
                None
            });
            match supplement {
                Some((span, Ok(value))) => {
                    let value =
                        crate::compiler::stdlib::equation_supplement(&value, span)?;
                    *engine.styles =
                        engine.styles.push_custom("equation.supplement", value);
                }
                Some((_, Err(err))) => return Err(err),
                None => {}
            }
            let alt = set.args().items().find_map(|arg| {
                if let Arg::Named(named) = arg {
                    if named.name().as_str() == "alt" {
                        return Some((
                            named.expr().span(),
                            eval_expr(named.expr(), scopes, ctx, engine),
                        ));
                    }
                }
                None
            });
            match alt {
                Some((span, Ok(value))) => {
                    let value = crate::compiler::stdlib::equation_alt(&value, span)?;
                    *engine.styles = engine.styles.push_custom("equation.alt", value);
                }
                Some((_, Err(err))) => return Err(err),
                None => {}
            }
            return Ok(Value::None);
        }
    }

    // **P1030** — restantes alvos pontuados `math.<elemento>` (`eval.md`
    // §P1030). Tem de vir DEPOIS do braço de `math.equation` acima, que
    // continua a ser o dono de `numbering`.
    if let Expr::FieldAccess(fa) = set.target() {
        if fa.target().to_untyped().text_str() == "math" {
            let elem = fa.field().to_untyped().text_str().to_owned();
            return eval_set_math_rule(&elem, set, scopes, ctx, engine, target_span);
        }
    }

    if target == "document" {
        // **P536** — `#set document(title: ..., author: ..., keywords: ...)`.
        // Extrai os valores, converte arrays para strings separadas por vírgula,
        // e acumula no `EvalContext`. O eval copia isto para o `Module` no final;
        // o pipeline transporta para o exportador PDF (`/Info`).
        fn value_to_eco_string(
            val: &crate::entities::value::Value,
            span: Span,
            allow_array: bool,
        ) -> SourceResult<Option<ecow::EcoString>> {
            match val {
                crate::entities::value::Value::Str(s) => Ok(Some(s.clone())),
                crate::entities::value::Value::Array(arr) if allow_array => {
                    let mut parts = Vec::new();
                    for v in arr.iter() {
                        match v {
                            crate::entities::value::Value::Str(s) => {
                                parts.push(s.as_str())
                            }
                            other => {
                                return Err(vec![type_mismatch("string", other, span)]);
                            }
                        }
                    }
                    if parts.is_empty() {
                        Ok(None)
                    } else {
                        Ok(Some(ecow::EcoString::from(parts.join(", "))))
                    }
                }
                crate::entities::value::Value::None => Ok(None),
                other => {
                    let expected =
                        if allow_array { "string or array of strings" } else { "string" };
                    Err(vec![type_mismatch(expected, other, span)])
                }
            }
        }

        for arg in set.args().items() {
            if let Arg::Named(named) = arg {
                let key = named.name().as_str();
                let val = eval_expr(named.expr(), scopes, ctx, engine)?;
                let span = named.expr().span();
                match key {
                    // P637: no vanilla, title é Option<Content> (não array);
                    // author/keywords são OneOrMultiple<EcoString>.
                    "title" => {
                        ctx.document_info.title = value_to_eco_string(&val, span, false)?
                    }
                    "author" => {
                        ctx.document_info.author = value_to_eco_string(&val, span, true)?
                    }
                    "keywords" => {
                        ctx.document_info.keywords =
                            value_to_eco_string(&val, span, true)?
                    }
                    _ => {}
                }
            }
        }
        return Ok(Value::None);
    }

    if target == "page" {
        return eval_set_page(set, scopes, ctx, engine);
    }

    if target == "figure" {
        // #set figure(numbering: "1") — activa numeração automática de figuras.
        // Lote F-2 S3 (P335): empurra para a chain léxica (`custom`), como
        // heading/equation, em vez de mutar o campo global `engine.figure_numbering`.
        // `native_figure` assa o valor lendo a chain (`closures.rs`). Escopo
        // léxico de graça (fecha o DEBT 99.E para figure).
        for arg in set.args().items() {
            if let Arg::Named(named) = arg {
                if named.name().as_str() == "numbering" {
                    let val = eval_expr(named.expr(), scopes, ctx, engine)?;
                    let span = named.expr().span();
                    match val {
                        Value::Str(s) => {
                            *engine.styles = engine
                                .styles
                                .push_custom("figure.numbering", Value::Str(s));
                        }
                        Value::None => {
                            // Limpa a numeração no escopo (Value::None = ausente).
                            *engine.styles = engine
                                .styles
                                .push_custom("figure.numbering", Value::None);
                        }
                        other => {
                            return Err(vec![type_mismatch(
                                "string or none",
                                &other,
                                span,
                            )]);
                        }
                    }
                }
            }
        }
        return Ok(Value::None);
    }

    if target == "table" {
        // P459 — `#set table(numbering: "1.")` activa numeração automática de
        // tables. O padrão vive na chain léxica (`custom("table.numbering")`),
        // análogo a figure/equation (F-2 S3). O layout lê o gate e prefixa o
        // caption quando ambos (caption + numbering) estão presentes.
        for arg in set.args().items() {
            if let Arg::Named(named) = arg {
                if named.name().as_str() == "numbering" {
                    let val = eval_expr(named.expr(), scopes, ctx, engine)?;
                    let span = named.expr().span();
                    match val {
                        Value::Str(s) => {
                            *engine.styles = engine
                                .styles
                                .push_custom("table.numbering", Value::Str(s));
                        }
                        Value::None => {
                            *engine.styles =
                                engine.styles.push_custom("table.numbering", Value::None);
                        }
                        other => {
                            return Err(vec![type_mismatch(
                                "string or none",
                                &other,
                                span,
                            )]);
                        }
                    }
                }
            }
        }
        return Ok(Value::None);
    }

    if target == "par" {
        // Passo 133: target `par` activado sem arms concretos.
        // Passo 134: arm `leading` migrado de `text` para `par`
        // (paridade ADR-0033; resolve divergência temporal do 128).
        // Outras propriedades (justify, first-line-indent, etc.)
        // continuam no fallback até serem activadas on-demand.
        // F-5b fatia 2 (P373): `leading` migra de `StyleDelta` tipado para o canal
        // `custom` `"par.leading"` (morph-transparente, transportado). O layout
        // (merge arm) decodifica de volta.
        for arg in set.args().items() {
            if let Arg::Named(named) = arg {
                let key = named.name().as_str().to_owned();
                let val = eval_expr(named.expr(), scopes, ctx, engine)?;
                match key.as_str() {
                    "leading" => {
                        if let Value::Length(l) = val {
                            *engine.styles = engine
                                .styles
                                .push_custom("par.leading", Value::Length(l));
                        }
                    }
                    "spacing" => {
                        if let Value::Length(l) = val {
                            *engine.styles = engine
                                .styles
                                .push_custom("par.spacing", Value::Length(l));
                        }
                    }
                    "justify" => match val {
                        Value::Bool(b) => {
                            *engine.styles =
                                engine.styles.push_custom("par.justify", Value::Bool(b));
                            if b {
                                engine.sink.warn_note(named.expr().span(),
                                    "par: seleção global tem suporte parcial",
                                    "seleção global certificada para palavras latinas de estilo e política uniformes em largura constante; demais casos mantêm seleção incremental");
                            }
                        }
                        other => {
                            return Err(vec![break_policy_error(
                                "expected boolean",
                                &other,
                                named.expr().span(),
                            )])
                        }
                    },
                    "linebreaks" => {
                        let expected = "expected \"simple\", \"optimized\", or auto";
                        match &val {
                            Value::Auto => {}
                            Value::Str(name)
                                if matches!(name.as_str(), "simple" | "optimized") => {}
                            Value::Str(_) => {
                                return Err(vec![SourceDiagnostic::error(
                                    named.expr().span(),
                                    expected,
                                )])
                            }
                            other => {
                                return Err(vec![break_policy_error(
                                    expected,
                                    other,
                                    named.expr().span(),
                                )])
                            }
                        }
                        *engine.styles = engine.styles.push_custom("par.linebreaks", val);
                    }
                    _ => {
                        let (msg, hint) = unsupported_property_warn("par", &key, None);
                        engine.sink.warn_note(
                            named.name().to_untyped().span(),
                            &msg,
                            &hint,
                        );
                    }
                }
            }
        }
        return Ok(Value::None);
    }

    if target == "smartquote" {
        for arg in set.args().items() {
            if let Arg::Named(named) = arg {
                let key = named.name().as_str();
                let val = eval_expr(named.expr(), scopes, ctx, engine)?;
                let span = named.expr().span();
                match key {
                    "enabled" => match val {
                        Value::Bool(b) => {
                            *engine.styles = engine
                                .styles
                                .push_custom("smartquote.enabled", Value::Bool(b));
                        }
                        other => {
                            return Err(vec![type_mismatch("bool", &other, span)]);
                        }
                    },
                    "alternative" => match val {
                        Value::Bool(value) => {
                            *engine.styles = engine.styles.push_custom(
                                "smartquote.alternative",
                                Value::Bool(value),
                            );
                        }
                        other => return Err(vec![type_mismatch("bool", &other, span)]),
                    },
                    "quotes" => {
                        crate::compiler::lang::quotes::parse_smartquote_quotes(
                            &val, span,
                        )?;
                        *engine.styles =
                            engine.styles.push_custom("smartquote.quotes", val);
                    }
                    _ => {
                        let (msg, hint) =
                            unsupported_property_warn("smartquote", key, None);
                        engine.sink.warn_note(
                            named.name().to_untyped().span(),
                            &msg,
                            &hint,
                        );
                    }
                }
            }
        }
        return Ok(Value::None);
    }

    // F-item3 (P368, `f_fronteira_e1.md` §3a.11): `#set <elemento-de-usuário>(prop:)`
    // pelo mapa aberto. Se o `target` resolve em `scopes` a um `Func::element`
    // (elemento de usuário registrado), em vez do warn, empurra `("<kind>.<prop>",
    // Value)` no canal `custom` da chain — transparente à morfologia
    // (`is_semantically_empty` ignora o custom, P366). O elemento lê pela chain no
    // layout (precedência: construído explícito > chain > default). Aditivo: os
    // `#set` nativos (acima) não mudam.
    let is_user_elem = matches!(
        scopes.get(&target),
        Some(Value::Func(f)) if f.element_name().is_some()
    );
    if is_user_elem {
        for arg in set.args().items() {
            if let Arg::Named(named) = arg {
                let key = format!("{}.{}", target, named.name().as_str());
                let val =
                    eval_expr(named.expr(), scopes, ctx, engine).unwrap_or(Value::None);
                *engine.styles = engine.styles.push_custom(key, val);
            }
        }
        return Ok(Value::None);
    }

    if target != "text" {
        let (msg, hint) = unsupported_target_warn(&target);
        engine.sink.warn_note(target_span, &msg, &hint);
        return Ok(Value::None);
    }

    eval_set_text(set, scopes, ctx, engine)
}
