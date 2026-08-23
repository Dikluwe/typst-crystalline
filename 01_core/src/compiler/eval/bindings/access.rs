//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/bindings/access.md
//! @prompt-hash 93e6151e
//! @layer L1
//! @updated 2026-08-12
//!
//! Resolução de lugares mutáveis (`access`, `access_dict`) e as mensagens de
//! variável/chave desconhecida. Usa `vanilla_type_name` para nomes de tipo nas
//! mensagens de erro.
//!
//! Extraído de `compiler/eval/bindings.rs` no Passo 1013 conforme ADR-0109
//! (atomização — forma B, free function no arquivo da unidade).

use ecow::EcoString;
use indexmap::IndexMap;
use rustc_hash::FxBuildHasher;

use crate::compiler::scopes::Scopes;
use crate::entities::ast::expr::Expr;
use crate::entities::ast::AstNode;
use crate::entities::engine::Engine;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;

use crate::compiler::eval::operators::error_formatting::vanilla_type_name;
use crate::compiler::eval::{eval_expr, EvalContext};

use super::method_dispatch::{call_method_access, is_accessor_method};

/// **P716** — erro de chave ausente do vanilla (`Dict::at_mut`,
/// `foundations/dict.rs:99-104`), com o hint.
pub(super) fn missing_key(span: Span, key: &str) -> Vec<SourceDiagnostic> {
    vec![SourceDiagnostic::error(
        span,
        format!("dictionary does not contain key {key:?}"),
    )
    .with_hint("use `insert` to add or update values".to_string())]
}

/// **P772r** — erro de variável desconhecida, paridade vanilla
/// `foundations/scope.rs::unknown_variable` (linha 424-437). Usado tanto
/// para leitura (`eval_expr`, `Expr::Ident`, `eval/mod.rs`) como para
/// mutação (`access`, abaixo) — mesma função no vanilla para os dois casos.
///
/// Heurística do hint (confirmada por sonda, não assumida): qualquer
/// hífen no nome (`name.contains('-')`) — sem verificar se as partes ao
/// redor são identificadores conhecidos. Plural ("signs") quando há mais
/// de um hífen; singular ("sign") para um só. Sem hífen → sem hint.
pub(in crate::compiler::eval) fn unknown_variable(
    span: Span,
    name: &str,
) -> SourceDiagnostic {
    let diag = SourceDiagnostic::error(span, format!("unknown variable: {name}"));
    if name.contains('-') {
        let plural = if name.matches('-').count() > 1 { "s" } else { "" };
        diag.with_hint(format!(
            "if you meant to use subtraction, try adding spaces around the minus sign{plural}: `{}`",
            name.replace('-', " - ")
        ))
    } else {
        diag
    }
}

/// **P716** — mirror do trait `Access` do vanilla (`access.rs:14-27`), como
/// free function (o cristalino não tem `Vm`; recebe as três partes). Quatro
/// formas de alvo: `Ident`, `Parenthesized`, `FieldAccess`, `FuncCall` de
/// accessor method. Qualquer outra expressão avalia (para efeitos) e erra
/// "cannot mutate a temporary value" (`access.rs:21-24,71-72`).
pub(super) fn access<'s>(
    expr: Expr<'_>,
    scopes: &'s mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<&'s mut Value> {
    match expr {
        Expr::Ident(ident) => {
            let name = ident.as_str();
            if scopes.get_mut(name).is_none() {
                // P772q — `name` só existe em `captured` (closure/`context`)
                // → mensagem específica por `Capturer`, paridade vanilla
                // `Binding::write()` (`foundations/scope.rs:316-323`);
                // verificado antes de `is_constant` (P772n): paridade
                // vanilla onde `get_mut` bem sucedido sobre um binding
                // `Captured` falha em `.write()`, sem chegar a `base`.
                // Ver `rules/eval.md` §P772q, §P772n, §P772r.
                let diag = if let Some(capturer) = scopes.captured_by(name) {
                    let origin = match capturer {
                        crate::entities::scope::Capturer::Function => "function",
                        crate::entities::scope::Capturer::Context => "context expression",
                    };
                    SourceDiagnostic::error(
                        ident.span(),
                        format!(
                            "variables from outside the {origin} are read-only and cannot be modified"
                        ),
                    )
                } else if scopes.is_constant(name) {
                    SourceDiagnostic::error(
                        ident.span(),
                        format!("cannot mutate a constant: {name}"),
                    )
                } else {
                    unknown_variable(ident.span(), name)
                };
                return Err(vec![diag]);
            }
            match scopes.get_mut(name) {
                Some(slot) => Ok(slot),
                None => unreachable!("verificado acima"),
            }
        }
        Expr::Parenthesized(paren) => access(paren.expr(), scopes, ctx, engine),
        Expr::FieldAccess(fa) => {
            let span = fa.span();
            let field: EcoString = fa.field().as_str().into();
            let dict = access_dict(fa, scopes, ctx, engine)?;
            match dict.get_mut(field.as_str()) {
                Some(slot) => Ok(slot),
                None => Err(missing_key(span, field.as_str())),
            }
        }
        Expr::FuncCall(call) => {
            if let Expr::FieldAccess(fa) = call.callee() {
                if is_accessor_method(fa.field().as_str()) {
                    let span = call.span();
                    let method: EcoString = fa.field().as_str().into();
                    // Ordem do vanilla (`access.rs:62-64`): args primeiro,
                    // access do target depois.
                    let args = crate::compiler::eval::call_dispatch::eval_args(
                        call.args(),
                        scopes,
                        ctx,
                        engine,
                    )?;
                    let target = access(fa.target(), scopes, ctx, engine)?;
                    return call_method_access(target, method.as_str(), args, span);
                }
            }
            let _ = eval_expr(expr, scopes, ctx, engine)?;
            Err(vec![SourceDiagnostic::error(
                expr.span(),
                "cannot mutate a temporary value".to_string(),
            )])
        }
        other => {
            let _ = eval_expr(other, scopes, ctx, engine)?;
            Err(vec![SourceDiagnostic::error(
                other.span(),
                "cannot mutate a temporary value".to_string(),
            )])
        }
    }
}

/// **P716** — mirror de `access_dict` do vanilla (`access.rs:76-107`):
/// resolve o target de um `FieldAccess` como dict mutável. Os três níveis
/// de erro para não-dict seguem o vanilla: tipos com field getters próprios
/// → "cannot mutate fields on {ty}"; tipos em `fields_on`
/// (`fields.rs:77-91`: Version, Length, Rel, Stroke, Alignment) → "fields on
/// {ty} are not yet mutable" + hint; resto → "{ty} does not have accessible
/// fields". Duration cristalino (campos de leitura, P412) fica no último
/// braço — o vanilla não o tem em `fields_on` e a mensagem é o observável.
pub(super) fn access_dict<'s>(
    fa: crate::entities::ast::expr::FieldAccess<'_>,
    scopes: &'s mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<&'s mut IndexMap<EcoString, Value, FxBuildHasher>> {
    let target_span = fa.target().span();
    match access(fa.target(), scopes, ctx, engine)? {
        Value::Dict(dict) => Ok(dict),
        value => {
            let ty = vanilla_type_name(value);
            match value {
                Value::Symbol(_)
                | Value::Content(_)
                | Value::Module(_)
                | Value::Func(_)
                | Value::Args(_) => Err(vec![SourceDiagnostic::error(
                    target_span,
                    format!("cannot mutate fields on {ty}"),
                )]),
                Value::Version(_)
                | Value::Length(_)
                | Value::Relative(_)
                | Value::Stroke(_)
                | Value::Align(_) => Err(vec![SourceDiagnostic::error(
                    target_span,
                    format!("fields on {ty} are not yet mutable"),
                )
                .with_hint(format!(
                    "try creating a new {ty} with the updated field value instead"
                ))]),
                _ => Err(vec![SourceDiagnostic::error(
                    target_span,
                    format!("{ty} does not have accessible fields"),
                )]),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::compiler::eval::operators::error_formatting::vanilla_type_name;
    use crate::entities::value::Value;

    /// **P1015/P1017** — contrato de `vanilla_type_name`, canónica após
    /// dedup. Difere de `type_name()` em int/str/bool; tudo o resto delega.
    #[test]
    fn p1017_vanilla_type_name_difere_de_type_name_so_em_tres_casos() {
        assert_eq!(vanilla_type_name(&Value::Int(1)), "integer");
        assert_eq!(vanilla_type_name(&Value::Str("x".into())), "string");
        assert_eq!(vanilla_type_name(&Value::Bool(true)), "boolean");

        // Fora dos três, é exactamente `type_name()`.
        for v in [Value::None, Value::Auto, Value::Float(1.0), Value::Array(vec![])] {
            assert_eq!(
                vanilla_type_name(&v),
                v.type_name(),
                "fora de int/str/bool, vanilla_type_name delega em type_name()"
            );
        }
    }

    /// **P1015/P1017** — a equivalência que legitimou a dedup: as cópias
    /// removidas faziam `match v.type_name()` em vez de `match v`. As duas
    /// formas coincidem porque `type_name()` é bijectiva — nenhuma outra
    /// variante devolve "int"/"str"/"bool".
    #[test]
    fn p1017_forma_por_string_e_forma_por_variante_coincidem() {
        let por_string = |v: &Value| -> &'static str {
            match v.type_name() {
                "int" => "integer",
                "str" => "string",
                "bool" => "boolean",
                other => other,
            }
        };
        for v in [
            Value::Int(1),
            Value::Str("x".into()),
            Value::Bool(false),
            Value::None,
            Value::Auto,
            Value::Float(0.5),
            Value::Array(vec![]),
        ] {
            assert_eq!(vanilla_type_name(&v), por_string(&v), "divergência em {v:?}");
        }
    }
}
