//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/math/call_fallback.md
//! @prompt-hash bcf6dc57
//! @layer L1
//! @updated 2026-09-30

use super::*;

/// Resolve funções globais, operadores e símbolos fora do catálogo nativo.
pub(super) fn eval_math_call_fallback(
    name: String,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
    call: crate::entities::ast::expr::FuncCall<'_>,
) -> SourceResult<Content> {
    // **P510** — chamada a funções do scope global em modo math
    // (ex.: `bb(x)`, `bold(x + y)`). Antes do fallback P302/P303,
    // tentar resolver o nome no scope global; se for uma `Func`,
    // avaliar os args como conteúdo math e aplicar.
    // P1292-B — `underline` has two distinct public identities:
    // the global binding is the textual decoration, whereas a
    // bare call in math syntax is sugar for `math.underline`.
    // Resolve only this collision through the math module; code
    // mode continues to see the global textual function.
    let maybe_func = if name == "underline" {
        match scopes.get("math") {
            Some(Value::Module(module)) => module.scope().get("underline").cloned(),
            _ => None,
        }
    } else {
        scopes.get(&name).cloned()
    };
    if let Some(Value::Func(func)) = maybe_func {
        // **P899** — reaproveita `eval_math_arg_value` (já usado
        // pelo caminho de callee namespaced P772y, ex.:
        // `math.class(...)`), em vez de forçar todos os
        // argumentos por `eval_math_expr` + `Value::Content`.
        // Sem isto, um literal string (`bb("R")`) chegava a
        // `native_bb` já embrulhado em `Content::Text` (prosa,
        // via `value_to_display_content` — braço genérico de
        // `eval_math_expr`), nunca como `Value::Str` — o corpo
        // ficava fora do alcance de `apply_math_style`
        // (só cobre `MathIdent`/`MathText`/`MathSequence`/
        // `MathMatrix`), sintoma: `$ bb("R") $` compilava mas
        // sem estilo nenhum (`typst-passo-899-relatorio.md`,
        // achado da revisão pós Fase A — o teste directo de
        // `wrap_math_style` não cobria este caminho de
        // despacho). Unifica a semântica dos dois caminhos de
        // chamada em modo math (bare global vs namespaced).
        let mut args = eval_p1293_math_args(scopes, ctx, engine, call.args())?;
        super::call_dispatch::transport_native_call_span(&func, &mut args, call.span());
        return match apply_func(func, args, scopes, ctx, engine)? {
            Value::Content(c) => Ok(c),
            other => Err(vec![SourceDiagnostic::error(
                call.span(),
                format!(
                    "{}() em modo math deve devolver content, recebeu {}",
                    name,
                    other.type_name()
                ),
            )]),
        };
    }

    let pos_args: Vec<Expr<'_>> = call
        .args()
        .items()
        .filter_map(|a| match a {
            Arg::Pos(e) => Some(e),
            _ => None,
        })
        .collect();
    let base = if let Some(op) = lookup_math_op(scopes, &name) {
        op
    } else if let Some(sym) = crate::compiler::math::symbols::ident_to_unicode(&name) {
        // **P958** — símbolo com args (`Gamma(z)` → Γ(𝑧)):
        // a cadeia de símbolos do braço standalone
        // (`Expr::MathIdent`) espelhada aqui — antes, o
        // fallback saltava de `lookup_math_op` directo para
        // o literal, e `Gamma(z)` saía `Gamma(𝑧)`. Ver
        // `compiler/eval.md` §P958.
        Content::MathText(sym.into())
    } else if let Some(sym) = crate::compiler::stdlib::sym::sym_lookup(&name) {
        // **P958** — idem via módulo `sym` (símbolos bare de
        // P795), com warning de depreciação verbatim (P820).
        if let Some(msg) = crate::compiler::stdlib::sym::sym_deprecation(&name) {
            engine.sink.warn_note(call.callee().span(), msg, "");
        }
        Content::MathText(sym.value.to_string().into())
    } else {
        Content::MathIdent(name.into())
    };
    if pos_args.is_empty() {
        // `sin()` / `undef()` — args vazios; só base sem wrapper.
        return Ok(base);
    }
    let body = if pos_args.len() == 1 {
        eval_math_expr(scopes, ctx, engine, pos_args[0])?
    } else {
        // Múltiplos args: separados por `, ` (paridade vanilla).
        let mut items: Vec<Content> = Vec::new();
        for (i, expr) in pos_args.iter().enumerate() {
            if i > 0 {
                items.push(Content::MathText(",".into()));
            }
            items.push(eval_math_expr(scopes, ctx, engine, *expr)?);
        }
        Content::MathSequence(std::sync::Arc::from(items))
    };
    let delimited = Content::math_delimited('(', body, ')');
    Ok(Content::MathSequence(std::sync::Arc::from(vec![base, delimited])))
}
