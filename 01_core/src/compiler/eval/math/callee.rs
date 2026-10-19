//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/math/callee.md
//! @prompt-hash 6106f2b4
//! @layer L1
//! @updated 2026-09-30

use super::*;

/// **P772y** — resolve um `Expr::FieldAccess`/`Expr::MathIdent` em modo
/// math a um `Value`, quando o alvo não pode passar pelo `eval_expr`
/// genérico. Usado originalmente só para o callee de uma `FuncCall` (ex.:
/// `math.class(...)`, `calc.foo(...)`); **P782** reutiliza-o também para
/// `Expr::FieldAccess` standalone numa sequência math (`sym.suit.heart`,
/// bare ou via `#`), não só como callee. Não delega directamente ao
/// `eval_expr` genérico porque este trata `Expr::MathIdent` como fronteira
/// deliberada (devolve `Value::None` — `eval/mod.rs`, comentário
/// "Fronteira deliberada") — o alvo de um field access em modo math (ex.
/// `math` em `math.class`, `sym` em `sym.suit`) precisa de ser resolvido
/// no scope directamente quando é um `MathIdent`. Cobre os tipos de target
/// relevantes (`Value::Module`/`Dict`/`Symbol`); outros tipos produzem
/// erro claro em vez de silenciosamente devolver `Content::Empty`.
pub(super) fn eval_math_callee(
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
    expr: Expr<'_>,
) -> SourceResult<Value> {
    match expr {
        Expr::FieldAccess(access) => {
            let target = eval_math_callee(scopes, ctx, engine, access.target())?;
            let field = access.field().as_str();
            let warning_module = match &target {
                Value::Module(module) => Some(module.clone()),
                _ => None,
            };
            let value =
                super::bindings::eval_value_field_access(target, field, access.span())?;
            if let Some(module) = warning_module {
                super::bindings::warn_deprecated_symbol_binding(
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
        Expr::MathIdent(ident) => {
            let name = ident.get();
            if let Some(val) = scopes.get(name).cloned() {
                // **P825 (sub-achado B de P810 §12)** — módulos globais
                // (`math`, `sym`, `calc`, `emoji`, …) NÃO são acessíveis
                // bare em modo math — paridade vanilla medida:
                // `error: unknown variable: math` + 3 hints. Excepção
                // medida: `std` É acessível bare (`$ std.math.class(...) $`
                // compila nos dois). Funções (`Value::Func`), bindings de
                // utilizador e outros valores não são afectados. Reforço da
                // validação de P782 (não duplicação): o caminho via `#`
                // (target `Expr::Ident` de código) passa pelo braço
                // `other => eval_expr` e não é tocado.
                let lexical = scopes.top.get(name).is_some()
                    || scopes.scopes.iter().rev().any(|scope| scope.get(name).is_some());
                if matches!(val, Value::Module(_)) && name != "std" && !lexical {
                    return Err(vec![unknown_variable_math(ident.span(), name, true)]);
                }
                Ok(val)
            } else if let Some(module) = lookup_math_catalog_submodule(scopes, name) {
                // Só o alvo de um acesso composto chega aqui: um módulo
                // isolado não passa a texto/símbolo.
                Ok(module)
            } else if let Some(sym) = crate::compiler::stdlib::sym::sym_lookup(name) {
                // P820 — símbolo depreciado (ex.: `join`): resolve, mas com
                // warning verbatim do vanilla (span na raiz — medido
                // `$join.r$` → warning @1:1).
                if let Some(msg) = crate::compiler::stdlib::sym::sym_deprecation(name) {
                    engine.sink.warn_note(ident.span(), msg, "");
                }
                Ok(Value::Symbol(sym))
            } else {
                // P820 (sub-achado (b) de P810 §7): a mensagem portuguesa
                // `variável desconhecida` era divergente — o vanilla usa
                // sempre `unknown variable` + hints (`$foo.bar$` medido).
                Err(vec![unknown_variable_math(
                    ident.span(),
                    name,
                    scopes.has_global(name),
                )])
            }
        }
        other => eval_expr(other, scopes, ctx, engine),
    }
}
