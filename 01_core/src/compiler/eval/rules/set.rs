//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/rules/set.md
//! @prompt-hash 1793adbe
//! @layer L1
//! @updated 2026-09-30
//!
//! Comportamento de `eval/rules::set`.

use super::*;

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
                Value::Int(i) => Ok(Some(
                    crate::entities::layout_types::PageDimension::Length(*i as f64),
                )),
                Value::Auto => {
                    Ok(Some(crate::entities::layout_types::PageDimension::Auto))
                }
                Value::None => Ok(None),
                other => {
                    Err(vec![type_mismatch("length, float, int, or auto", other, span)])
                }
            }
        }
        fn extract_pt(
            val: &Value,
            span: Span,
            size_pt: f64,
        ) -> SourceResult<Option<f64>> {
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
                                    (
                                        None,
                                        Some(crate::entities::layout_types::HAlign::Left)
                                    )
                                ) =>
                            {
                                Some(crate::entities::page_geometry::PageBinding::Left)
                            }
                            Value::Align(a)
                                if matches!(
                                    (a.v, a.h),
                                    (
                                        None,
                                        Some(
                                            crate::entities::layout_types::HAlign::Right
                                        )
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
                            margin =
                                Some(crate::entities::layout_types::PageMarginSpec {
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
                            margin = Some(
                                crate::entities::layout_types::PageMarginSpec::auto(),
                            );
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
                            Value::Str(s) => Some(Some(
                                crate::entities::numbering::Numbering::Pattern(s),
                            )),
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
                        let horizontal = align
                            .h
                            .unwrap_or(crate::entities::layout_types::HAlign::Center);
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
                        number_align =
                            Some(crate::entities::page_running::PageNumberAlign {
                                horizontal,
                                vertical,
                            });
                    }
                    "header" => {
                        header = Some(match val {
                            Value::Auto => {
                                crate::entities::page_running::PageMarginal::Auto
                            }
                            Value::None => {
                                crate::entities::page_running::PageMarginal::None
                            }
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
                    "header-ascent" => {
                        header_ascent = Some(extract_rel_length(&val, span)?)
                    }
                    "footer" => {
                        footer = Some(match val {
                            Value::Auto => {
                                crate::entities::page_running::PageMarginal::Auto
                            }
                            Value::None => {
                                crate::entities::page_running::PageMarginal::None
                            }
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
                                let physical = dict.get("left").is_some()
                                    || dict.get("right").is_some();
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
                                    right: field(if logical {
                                        "outside"
                                    } else {
                                        "right"
                                    })?
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
                                crate::entities::page_canvas::PageFill::Paint(
                                    color.into(),
                                )
                            }
                            Value::Gradient(gradient) => {
                                crate::entities::page_canvas::PageFill::Paint(
                                    gradient.into(),
                                )
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
                crate::entities::layout_types::PageDimension::Length(x) => {
                    Value::Float(x)
                }
            };
            *engine.styles = engine.styles.push_custom("page.width", v);
        }
        if let Some(h) = height {
            let v = match h {
                crate::entities::layout_types::PageDimension::Auto => Value::Auto,
                crate::entities::layout_types::PageDimension::Length(x) => {
                    Value::Float(x)
                }
            };
            *engine.styles = engine.styles.push_custom("page.height", v);
        }
        if let Some(ml) = margin_left {
            *engine.styles =
                engine.styles.push_custom("page.margin-left", Value::Float(ml));
        }
        if let Some(mr) = margin_right {
            *engine.styles =
                engine.styles.push_custom("page.margin-right", Value::Float(mr));
        }
        if let Some(mt) = margin_top {
            *engine.styles =
                engine.styles.push_custom("page.margin-top", Value::Float(mt));
        }
        if let Some(mb) = margin_bottom {
            *engine.styles =
                engine.styles.push_custom("page.margin-bottom", Value::Float(mb));
        }

        return Ok(Value::Content(Content::SetPage(std::sync::Arc::new(
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
        ))));
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

    // **F-5b fatia 2 (P373, §3a.13/§3a.14)**: o render do `#set text` deixa de assar
    // num `StyleDelta` tipado capturado no node — passa a viajar na chain pelo canal
    // `custom` `"text.<campo>"` (Value canónico), **transparente à morfologia**
    // (`is_semantically_empty` ignora o custom → `#set text X == X`) e levado ao
    // layout pelo transporte aninhado (`eval_markup`). A validação/erro-hard de
    // `lang`/`font` permanece (paridade ADR-0033/0052/0053); só a **saída** muda
    // (typed → custom). O layout (merge arm) decodifica o Value de volta.
    for arg in set.args().items() {
        if let Arg::Named(named) = arg {
            let key = named.name().as_str().to_owned();
            let val = eval_expr(named.expr(), scopes, ctx, engine)?;
            match key.as_str() {
                // P665: `bold` e `italic` como argumentos nomeados de `#set text`
                // não existem no vanilla; o vanilla usa `weight: "bold"` e
                // `style: "italic"`. Rejeitar para manter a linguagem alinhada.
                "bold" | "italic" => {
                    return Err(vec![SourceDiagnostic::error(
                        named.name().to_untyped().span(),
                        format!("unexpected argument: {}", key),
                    )]);
                }
                "size" => {
                    // P816 (achado #3c de P810): tipo errado deixa de ser
                    // ignorado em silêncio — erro hard `expected length,
                    // found {type}` (+ hint para Int), paridade vanilla
                    // (`foundations/cast.rs:325-343`).
                    let span = named.expr().span();
                    match val {
                        Value::Length(l) => {
                            *engine.styles =
                                engine.styles.push_custom("text.size", Value::Length(l));
                        }
                        other => {
                            return Err(vec![expected_length_error(&other, span)]);
                        }
                    }
                }
                "fill" => {
                    if let Value::Color(c) = val {
                        *engine.styles =
                            engine.styles.push_custom("text.fill", Value::Color(c));
                    }
                }
                "weight" => {
                    // Int direto ou nome simbólico (FontWeight::from_name) → u16
                    // canónico em Value::Int. P636: rejeitar tipos inválidos.
                    let span = named.expr().span();
                    match &val {
                        Value::Int(n) => {
                            if let Ok(w) = u16::try_from(*n) {
                                *engine.styles = engine
                                    .styles
                                    .push_custom("text.weight", Value::Int(w as i64));
                            } else {
                                return Err(vec![SourceDiagnostic::error(
                                    span,
                                    "font weight must be between 100 and 900".to_string(),
                                )]);
                            }
                        }
                        Value::Str(s) => {
                            if let Some(fw) = FontWeight::from_name(s.as_str()) {
                                *engine.styles = engine.styles.push_custom(
                                    "text.weight",
                                    Value::Int(fw.to_number() as i64),
                                );
                            } else {
                                return Err(vec![SourceDiagnostic::error(
                                    span,
                                    format!("unknown font weight name: {s}"),
                                )]);
                            }
                        }
                        Value::None => {}
                        other => {
                            return Err(vec![type_mismatch(
                                "int or string",
                                other,
                                span,
                            )]);
                        }
                    }
                }
                "style" => {
                    // P665: `style` é a forma canónica do vanilla para itálico.
                    // Aceita os três valores standard; guarda em `text.style`.
                    let span = named.expr().span();
                    if let Value::Str(s) = &val {
                        match s.as_str() {
                            "normal" | "italic" | "oblique" => {
                                *engine.styles = engine
                                    .styles
                                    .push_custom("text.style", Value::Str(s.clone()));
                            }
                            _ => {
                                return Err(vec![SourceDiagnostic::error(
                                    span,
                                    format!("unknown font style name: {s}"),
                                )]);
                            }
                        }
                    } else {
                        return Err(vec![type_mismatch("string", &val, span)]);
                    }
                }
                "variations" => {
                    // P836 (achado #21 de P831): campo `#[fold] #[ghost]`
                    // settable no vanilla (`text/mod.rs:846-850`). Validação
                    // partilhada com o constructor — mensagens/hints verbatim
                    // do vanilla (`variations.rs:217-236`, `tag.rs:85-117`).
                    let span = named.expr().span();
                    crate::entities::font_variations::FontVariations::from_value(
                        &val, span,
                    )?;
                    *engine.styles = engine.styles.push_custom("text.variations", val);
                }
                "tracking" => {
                    // P816: mesma validação de tipo de `size` (Length no
                    // vanilla, `text/mod.rs:333`).
                    let span = named.expr().span();
                    match val {
                        Value::Length(l) => {
                            *engine.styles = engine
                                .styles
                                .push_custom("text.tracking", Value::Length(l));
                        }
                        other => {
                            return Err(vec![expected_length_error(&other, span)]);
                        }
                    }
                }
                "lang" => {
                    // ADR-0052: valida via `Lang::from_str` (erro hard em
                    // inválido); guarda o código canónico (`Value::Str`).
                    if let Value::Str(s) = val {
                        match Lang::from_str(&s) {
                            Ok(lang) => {
                                *engine.styles = engine.styles.push_custom(
                                    "text.lang",
                                    Value::Str(lang.as_str().into()),
                                );
                            }
                            Err(msg) => {
                                return Err(vec![SourceDiagnostic::error(
                                    named.expr().span(),
                                    msg.to_string(),
                                )]);
                            }
                        }
                    }
                }
                "font" => {
                    // P407 (DEBT-52): string/array/dict aceites. Inspeccionamos o
                    // AST do argumento porque `Value::Dict` tem keys `EcoString`, não
                    // suportando regex keys (que o vanilla exige para `text.font`).
                    // Persiste na chain custom como `Value::Array` de items:
                    //   - `Value::Str(name)` → literal, variants vazio (P292/P373).
                    //   - `Value::Dict` com "name" (Str|Regex) + "variants" (Array[Str])
                    //     → dict form com regex keys.
                    // Zero tipo novo em `Value`: reusa Array/Dict/Str/Regex.
                    let span = named.expr().span();
                    let font_expr = named.expr();
                    let arr: Vec<Value> = match font_expr {
                        Expr::Str(node) => vec![Value::Str(EcoString::from(node.get()?))],
                        Expr::Array(arr_node) => {
                            let mut items = Vec::new();
                            for item in arr_node.items() {
                                if let ArrayItem::Pos(expr) = item {
                                    if let Expr::Str(node) = expr {
                                        items.push(Value::Str(EcoString::from(
                                            node.get()?,
                                        )));
                                    } else {
                                        return Err(vec![SourceDiagnostic::error(
                                            span,
                                            "font array must contain only strings"
                                                .to_string(),
                                        )]);
                                    }
                                }
                            }
                            if items.is_empty() {
                                return Err(vec![SourceDiagnostic::error(
                                    span,
                                    "font array must not be empty".to_string(),
                                )]);
                            }
                            items
                        }
                        Expr::Dict(dict_node) => {
                            // P414: detectar dict named fields (tem chave `family`)
                            // vs formato legado P407 (chaves são nomes de fonte).
                            const NAMED_FIELDS: &[&str] =
                                &["family", "variant", "weight", "style", "fallback"];
                            let is_named_fields = dict_node.items().any(|item| {
                                matches!(
                                    item,
                                    DictItem::Named(named_item)
                                        if NAMED_FIELDS.contains(&named_item.name().as_str())
                                )
                            });

                            if is_named_fields {
                                font_dict::parse_font_dict_named_fields(
                                    dict_node, span, scopes, ctx, engine,
                                )?
                            } else {
                                font_dict::parse_font_dict_legacy(
                                    dict_node, span, scopes, ctx, engine,
                                )?
                            }
                        }
                        _ => {
                            return Err(vec![SourceDiagnostic::error(
                                span,
                                "font expects a string, array of strings, or dict"
                                    .to_string(),
                            )]);
                        }
                    };
                    // P816 (achado #3b de P810): warning `unknown font
                    // family` para nomes literais ausentes do FontBook —
                    // paridade vanilla (`check_font_list`,
                    // `text/mod.rs:1577-1588`, chamado no parse do arg
                    // `font` em `text/mod.rs:170-176`). Nomes em regex
                    // (dict `Value::Regex`) não são verificáveis — tal
                    // como no vanilla, que só avisa sobre `FontFamily`
                    // literais. Nome impresso lowercased (normalização
                    // de `FontFamily::new` do vanilla).
                    let missing_families: Vec<String> = {
                        let book = engine.world.book();
                        arr.iter()
                            .filter_map(|item| match item {
                                Value::Str(s) => Some(s.as_str()),
                                Value::Dict(d) => match d.get("name") {
                                    Some(Value::Str(s)) => Some(s.as_str()),
                                    _ => None,
                                },
                                _ => None, // neutro: N16[β] — valores não-nominais filtrados em extracção de famílias tipográficas
                            })
                            .filter(|name| book.select_family(name).next().is_none())
                            .map(|name| name.to_lowercase())
                            .collect()
                    };
                    for family in missing_families {
                        engine.sink.warn_note(
                            span,
                            &format!("unknown font family: {family}"),
                            "",
                        );
                    }
                    *engine.styles =
                        engine.styles.push_custom("text.font", Value::Array(arr));
                }
                "top-edge" => {
                    // P837 (achados #22/#23 de P831) — paridade do cast
                    // `TopEdge` do vanilla (`text/mod.rs:1169-1177`):
                    // métrica do domínio enumerado ou `Length`; o resto é
                    // erro hard verbatim.
                    match val {
                        Value::Str(s) if TOP_EDGE_METRICS.contains(&s.as_str()) => {
                            *engine.styles =
                                engine.styles.push_custom("text.top-edge", Value::Str(s));
                        }
                        Value::Length(l) => {
                            *engine.styles = engine
                                .styles
                                .push_custom("text.top-edge", Value::Length(l));
                        }
                        other => {
                            return Err(vec![edge_cast_error(
                                true,
                                &other,
                                named.expr().span(),
                            )]);
                        }
                    }
                }
                "bottom-edge" => {
                    // P837 — idem, cast `BottomEdge` (`text/mod.rs:1217-1225`).
                    match val {
                        Value::Str(s) if BOTTOM_EDGE_METRICS.contains(&s.as_str()) => {
                            *engine.styles = engine
                                .styles
                                .push_custom("text.bottom-edge", Value::Str(s));
                        }
                        Value::Length(l) => {
                            *engine.styles = engine
                                .styles
                                .push_custom("text.bottom-edge", Value::Length(l));
                        }
                        other => {
                            return Err(vec![edge_cast_error(
                                false,
                                &other,
                                named.expr().span(),
                            )]);
                        }
                    }
                }
                "costs" => {
                    let entries =
                        crate::compiler::text_costs::parse(&val, named.expr().span())?;
                    let pagination = entries.iter().any(|(key, _)| {
                        matches!(*key, "text.costs.widow" | "text.costs.orphan")
                    });
                    for (key, value) in entries {
                        *engine.styles = engine.styles.push_custom(key, value);
                    }
                    if pagination {
                        engine.sink.warn_note(named.expr().span(),
                        "text.costs: proteção de widow/orphan limitada a parágrafos preparados com métricas verticais uniformes",
                        "colunas, notas e conteúdo misto ainda não têm proteção regional certificada");
                    }
                }
                "overhang" => match val {
                    Value::Bool(_) => {
                        *engine.styles = engine.styles.push_custom("text.overhang", val);
                    }
                    other => {
                        return Err(vec![break_policy_error(
                            "expected boolean",
                            &other,
                            named.expr().span(),
                        )])
                    }
                },
                "hyphenate" => match val {
                    Value::Bool(_) | Value::Auto => {
                        *engine.styles = engine.styles.push_custom("text.hyphenate", val);
                    }
                    other => {
                        return Err(vec![break_policy_error(
                            "expected boolean or auto",
                            &other,
                            named.expr().span(),
                        )])
                    }
                },
                "dir" => {
                    if let Value::Dir(dir) = val {
                        if dir.is_vertical() {
                            return Err(vec![SourceDiagnostic::error(
                                named.expr().span(),
                                "text direction must be horizontal".to_string(),
                            )]);
                        }
                        *engine.styles =
                            engine.styles.push_custom("text.dir", Value::Dir(dir));
                    }
                }
                _ => {
                    // P816 (achado #3a de P810): nome fora da lista de
                    // propriedades settable do `TextElem` vanilla é erro
                    // hard `unexpected argument: {name}` (paridade
                    // `foundations/args.rs:262`, exit 1). Nomes válidos no
                    // vanilla mas ainda não capturados (ex.: `stroke`,
                    // `stroke`, `baseline`) mantêm o warning de scope-out
                    // do Passo 107 (encerra DEBT-49, hint ADR-0040).
                    if !VANILLA_TEXT_SET_PROPS.contains(&key.as_str()) {
                        return Err(vec![SourceDiagnostic::error(
                            named.name().to_untyped().span(),
                            format!("unexpected argument: {}", key),
                        )]);
                    }
                    let (msg, hint) =
                        unsupported_property_warn("text", &key, Some("0040"));
                    engine.sink.warn_note(named.name().to_untyped().span(), &msg, &hint);
                }
            }
        }
    }

    Ok(Value::None)
}
