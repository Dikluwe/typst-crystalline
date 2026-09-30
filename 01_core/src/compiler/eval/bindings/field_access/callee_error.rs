//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/bindings/field_access/callee_error.md
//! @prompt-hash ff776f1f
//! @layer L1
//! @updated 2026-09-30

use super::*;

// ── P815 — método inexistente / dict-key-call (eval_field_callee) ───────────

/// **P815** — mirror de `element_or_type_with_name` do vanilla
/// (`typst-eval/src/call.rs:359-365`): `("element", nome do elemento)` para
/// content, `("type", nome longo do tipo)` nos restantes.
pub(super) fn element_or_type_with_name(value: &Value) -> (&'static str, String) {
    match value {
        Value::Content(c) => ("element", c.elem_name().to_string()),
        Value::LocatedContent(c, _) => ("element", c.content().elem_name().to_string()),
        _ => ("type", vanilla_type_name(value).to_string()),
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
