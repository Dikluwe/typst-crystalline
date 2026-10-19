//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/eval/rules/math_set.md
//! @prompt-hash 44180227
//! @layer L1
//! @updated 2026-09-30
//!
//! Comportamento de `eval/rules::math_set`.

use super::*;

/// Helper partilhado para construir warning de propriedade não suportada
/// em `#set` (Passo 107, encerra DEBT-49). Formato consistente para o
/// utilizador final.
///
/// Passo 134: parâmetro `adr_ref` opcional — `Some("0040")` referencia
/// catálogo vivo (set text); `None` produz hint genérico (set par, até
/// ADR dedicada existir).
pub(crate) fn unsupported_property_warn(
    target: &str,
    field: &str,
    adr_ref: Option<&str>,
) -> (String, String) {
    let msg = format!("{target}: propriedade '{field}' ainda não suportada");
    let hint = match adr_ref {
        Some(adr) => format!("ver ADR-{adr} para propriedades cobertas por set {target}"),
        None => format!("propriedades de set {target} ainda não são capturadas"),
    };
    (msg, hint)
}

/// Helper para warning de `#set` com target desconhecido (Passo 107).
/// Lista actualizada em Passo 133 para incluir `par` (activado).
pub(crate) fn unsupported_target_warn(target: &str) -> (String, String) {
    (
        format!("set: target '{target}' ainda não suportado"),
        "targets suportados: heading, page, figure, text, par, table, smartquote"
            .to_string(),
    )
}

/// **P1030** — parâmetros que **a linguagem** define para cada elemento math,
/// derivados dos campos não-`#[required]` dos elementos math do vanilla
/// (`lab/.../typst-library/src/math/`). Ver `eval.md` §P1030.
///
/// Um parâmetro **fora** desta tabela é erro (`unexpected argument`, a mesma
/// mensagem do vanilla); **dentro** dela, ou é levado à construção
/// (`MATH_SET_LIGADOS`) ou avisa nomeando elemento e parâmetro.
pub(crate) fn math_elem_params(elem: &str) -> Option<&'static [&'static str]> {
    Some(match elem {
        "equation" => &["block", "numbering", "number-align", "supplement", "alt"],
        "mat" => &["delim", "align", "augment", "gap", "row-gap", "column-gap"],
        "vec" => &["delim", "align", "gap"],
        "cases" => &["delim", "reverse", "gap"],
        "cancel" => &["length", "inverted", "cross", "angle", "stroke", "background"],
        "accent" => &["size", "dotless"],
        "frac" => &["style"],
        "lr" => &["size"],
        "stretch" => &["size"],
        "op" => &["limits"],
        "attach" => &["t", "b", "tl", "bl", "tr", "br"],
        "limits" => &["inline"],
        // Elementos math **sem** parâmetro configurável: os seus campos são
        // `#[required]`, `#[positional]` ou variádicos. Estão aqui de
        // propósito — é o que faz `#set math.binom(lower:)` dar
        // `unexpected argument: lower`, como o vanilla (medido P1030).
        "binom" | "root" | "mid" | "class" | "scripts" | "primes" | "underline"
        | "overline" | "underbrace" | "overbrace" | "underbracket" | "overbracket"
        | "underparen" | "overparen" | "undershell" | "overshell" => &[],
        _ => return None,
    })
}

/// **P1030** — `#set math.<elemento>(...)`, alvo pontuado.
///
/// `eval_set_rule` extrai o alvo por `text_str()`, que devolve `""` para um
/// `Expr::FieldAccess`; sem este caminho, todo o `#set math.*` caía no
/// fallback com alvo vazio e era ignorado em silêncio (só o aviso
/// `set: target '' ainda não suportado`). O braço de `math.equation` +
/// `numbering` continua **antes** deste e intacto.
pub(crate) fn eval_set_math_rule(
    elem: &str,
    set: SetRule<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
    target_span: Span,
) -> SourceResult<Value> {
    let Some(params) = math_elem_params(elem) else {
        // Nome de `math` que não é elemento com parâmetros. O vanilla
        // distingue três casos, medidos em P1030 Fase C: membro inexistente
        // (`module \`math\` does not contain \`foobar\``), função não-elemento
        // (`only element functions can be used in set rules`, ex. `math.abs`) e
        // símbolo (`symbol π is not callable`). Distingui-los exige resolver o
        // nome no scope do módulo — **fora da fatia 1**. Até lá avisa nomeando
        // o alvo, em vez de errar com uma mensagem que poderia ser falsa.
        engine.sink.warn_note(
            target_span,
            &format!(
                "#set math.{elem}(...) não é reconhecido como elemento configurável"
            ),
            "no vanilla isto é erro; a paridade da mensagem está por fazer",
        );
        return Ok(Value::None);
    };

    let mut seen = Vec::new();
    for arg in set.args().items() {
        let Arg::Named(named) = arg else { continue };
        let nome = named.name().as_str();

        if seen.contains(&nome) {
            return Err(vec![SourceDiagnostic::error(
                named.name().to_untyped().span(),
                format!("duplicate argument: {nome}"),
            )]);
        }
        seen.push(nome);

        if !params.contains(&nome) {
            // Paridade com o vanilla, que rejeita o argumento em vez de o
            // aceitar e ignorar (medido P1030 Fase A: `#set math.binom(lower:)`
            // → `error: unexpected argument: lower`).
            return Err(vec![SourceDiagnostic::error(
                named.name().to_untyped().span(),
                if matches!(elem, "accent" | "cases") {
                    format!("unknown property: {nome}")
                } else {
                    format!("unexpected argument: {nome}")
                },
            )]);
        }

        if MATH_SET_LIGADOS.contains(&(elem, nome)) {
            let val = eval_expr(named.expr(), scopes, ctx, engine)?;
            if (elem, nome) == ("frac", "style") {
                match &val {
                    Value::Str(value)
                        if matches!(
                            value.as_str(),
                            "vertical" | "skewed" | "horizontal"
                        ) => {}
                    Value::Str(_) => {
                        return Err(vec![SourceDiagnostic::error(
                            named.expr().span(),
                            "expected \"vertical\", \"skewed\", or \"horizontal\""
                                .to_string(),
                        )]);
                    }
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            named.expr().span(),
                            format!(
                                "expected \"vertical\", \"skewed\", or \"horizontal\", found {}",
                                match other {
                                    Value::Int(_) => "integer",
                                    Value::Str(_) => "string",
                                    Value::Bool(_) => "boolean",
                                    _ => other.type_name(),
                                }
                            ),
                        )]);
                    }
                }
            }
            if matches!((elem, nome), ("accent", "size") | ("stretch", "size")) {
                match &val {
                    Value::Relative(_) | Value::Ratio(_) | Value::Length(_) => {}
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            named.expr().span(),
                            format!(
                                "expected relative length, found {}",
                                match other {
                                    Value::Int(_) => "integer",
                                    Value::Bool(_) => "boolean",
                                    Value::Str(_) => "string",
                                    _ => other.type_name(),
                                }
                            ),
                        )]);
                    }
                }
            }
            if (elem, nome) == ("accent", "dotless") && !matches!(&val, Value::Bool(_)) {
                return Err(vec![SourceDiagnostic::error(
                    named.expr().span(),
                    format!(
                        "expected boolean, found {}",
                        match &val {
                            Value::Int(_) => "integer",
                            Value::Str(_) => "string",
                            _ => val.type_name(),
                        }
                    ),
                )]);
            }
            if elem == "cases" {
                let mut options = crate::compiler::stdlib::MathCasesOptions::default();
                crate::compiler::stdlib::apply_math_cases_option(
                    &mut options,
                    nome,
                    &val,
                    named.expr().span(),
                    false,
                )?;
            }
            if elem == "mat" {
                let mut options = crate::compiler::stdlib::MathMatrixOptions::default();
                crate::compiler::stdlib::apply_math_matrix_option(
                    &mut options,
                    nome,
                    &val,
                    named.expr().span(),
                    false,
                )?;
            }
            *engine.styles =
                engine.styles.push_custom(format!("math.{elem}.{nome}"), val);
        } else {
            engine.sink.warn_note(
                named.name().to_untyped().span(),
                &format!(
                    "#set math.{elem}({nome}:) é da linguagem mas ainda não tem efeito"
                ),
                "o parâmetro é aceite e ignorado; o documento compila sem ele",
            );
        }
    }

    Ok(Value::None)
}
