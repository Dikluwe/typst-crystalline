//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/math/lookup.md
//! @prompt-hash 99e13b28
//! @layer L1
//! @updated 2026-09-30

use super::*;

// ── Passo 301 — Auto-lookup math mode ─────────────────────────────────────
//
// P301 (HP + P301.A + (a) eval-time + (γ) híbrido): consulta scope
// `math` (P299 SSoT) antes de fallback `MathIdent`. Identifiers
// vanilla pré-definidos (`sin`/`cos`/`lim`/etc., 42 ops) resolvem
// automaticamente para `Content::MathOp` em math mode.
//
// Heurística pré-P301 preservada como fallback:
// - `is_limit_function`/`is_large_operator` em `attach.rs` continua
//   a aplicar para casos não cobertos por scope (operadores
//   Unicode literais).
// - Variables user (`x`, `f`, etc.) continuam `MathIdent` (lookup
//   retorna None).

/// Lookup helper: consulta scope `math` (P299) e retorna `MathOp`
/// clone se encontrado. None caso contrário (incluindo se valor
/// não é `Content::MathOp`). **P731** — `math` passou a `Value::Module`
/// (era `Value::Dict`); o lookup é no scope do módulo.
pub(super) fn lookup_math_op(scopes: &Scopes<'_>, name: &str) -> Option<Content> {
    let Value::Module(math_module) = scopes.get("math")? else {
        return None;
    };
    let Value::Content(c) = math_module.scope().get(name)? else {
        return None;
    };
    Some(c.clone())
}

/// P1496 — o binding lexical `math` pode ser sombreado sem alterar o
/// namespace matemático padrão. Closure sem `base` conserva a cópia
/// capturada de `std.math` antes do fallback ao binding `math`.
pub(super) fn lookup_math_catalog_submodule(
    scopes: &Scopes<'_>,
    name: &str,
) -> Option<Value> {
    let math = scopes
        .base
        .and_then(|base| base.global.get("math"))
        .or_else(|| {
            let Value::Module(std) = scopes.captured.as_ref()?.get("std")? else {
                return None;
            };
            std.scope().get("math")
        })
        .or_else(|| scopes.captured.as_ref().and_then(|captured| captured.get("math")))?;
    let Value::Module(math) = math else {
        return None;
    };
    let Value::Module(child) = math.scope().get(name)? else {
        return None;
    };
    Some(Value::Module(child.clone()))
}

/// **P780** — mensagem de erro para identificador não resolvido em modo
/// math. Paridade `unknown_variable_math` (vanilla, `foundations/
/// scope.rs:439-472`) — **distinta** de `unknown_variable` (P772r, usada
/// em código normal): hints diferentes por caso, medidos directamente
/// contra o vanilla, não assumidos:
///
/// - `none`/`auto`/`false`/`true`: hint "adicionar `#` antes" (`#none`).
/// - conhecido em `base.global` mas não em math (`in_global`): 3 hints —
///   "não disponível directamente em math", "`#nome` em código",
///   "`std.nome` em math".
/// - desconhecido de todo: 2 hints — espaçar as letras (`f o o`), ou
///   citar como texto (`"foobarbaz"`).
pub(super) fn unknown_variable_math(
    span: Span,
    name: &str,
    in_global: bool,
) -> SourceDiagnostic {
    let diag = SourceDiagnostic::error(span, format!("unknown variable `{name}`"));
    if matches!(name, "none" | "auto" | "false" | "true") {
        diag.with_hint(format!(
            "if you meant to use a literal, try adding a hash before it: `#{name}`"
        ))
    } else if in_global {
        diag.with_hint(format!(
            "`{name}` is not available directly in math, but is in the standard library"
        ))
        .with_hint(format!(
            "to access `{name}` in code mode you can add a hash: `#{name}`"
        ))
        .with_hint(format!(
            "or access `{name}` in math mode by using the `std` module: `std.{name}`"
        ))
    } else {
        let spaced: String = name.chars().flat_map(|c| [' ', c]).skip(1).collect();
        diag.with_hint(format!(
            "if you meant to display multiple letters as is, \
             try adding spaces between each letter: `{spaced}`"
        ))
        .with_hint(format!(
            "or if you meant to display this as text, try placing it in quotes: `\"{name}\"`"
        ))
    }
}
