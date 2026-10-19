//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/eval/call_dispatch/location.md
//! @prompt-hash 13bb7a9e
//! @layer L1
//! @updated 2026-09-30

use super::*;

/// **P792** — Despacho de métodos de instância de `Value::Location`.
///
/// Intercepção em `eval_func_call` para `loc.page()`, `loc.position()`,
/// `loc.page-numbering()`. Paridade vanilla `location.rs #[scope]`.
///
/// Acede ao `ctx.introspector`: o target HTML mantém `page()` como `none`;
/// `position()` devolve somente `x/y` quando existe posição local de ilha e `none`
/// fora dela. Em targets paginados, a ausência de posição usa os defaults seguros
/// page=1, x=0pt e y=0pt.
pub(in crate::compiler::eval) fn eval_location_method(
    loc: crate::entities::location::Location,
    method: &str,
    ctx: &mut EvalContext,
    span: Span,
) -> SourceResult<Value> {
    use crate::entities::introspector::Introspector;
    use crate::entities::layout_types::Length;

    let result = if ctx.target == super::EvalTarget::Html && method == "page" {
        Ok(Value::None)
    } else if ctx.target == super::EvalTarget::Html && method == "position" {
        Ok(match ctx.introspector.html_position_of(loc) {
            None => Value::None,
            Some(pos) => {
                let mut dict: IndexMap<EcoString, Value, FxBuildHasher> =
                    IndexMap::default();
                dict.insert("x".into(), Value::Length(Length::pt(pos.point.x.val())));
                dict.insert("y".into(), Value::Length(Length::pt(pos.point.y.val())));
                Value::Dict(dict)
            }
        })
    } else {
        match method {
            "page" => {
                // Número da página (1-based) via introspector; default 1 se não disponível.
                let page_num = ctx
                    .introspector
                    .position_of(loc)
                    .map(|p| p.page.get() as i64)
                    .unwrap_or(1);
                Ok(Value::Int(page_num))
            }
            "position" => {
                // Dict {page: int, x: length, y: length} em pontos.
                let pos = ctx.introspector.position_of(loc);
                let page_num = pos.as_ref().map(|p| p.page.get() as i64).unwrap_or(1);
                let x_pt = pos.as_ref().map(|p| p.point.x.0).unwrap_or(0.0);
                let y_pt = pos.as_ref().map(|p| p.point.y.0).unwrap_or(0.0);

                let mut dict: IndexMap<EcoString, Value, FxBuildHasher> =
                    IndexMap::default();
                dict.insert("page".into(), Value::Int(page_num));
                dict.insert("x".into(), Value::Length(Length::pt(x_pt)));
                dict.insert("y".into(), Value::Length(Length::pt(y_pt)));
                Ok(Value::Dict(dict))
            }
            "page-numbering" => Ok(match ctx.introspector.page_numbering(loc) {
                Some(crate::entities::numbering::Numbering::Pattern(pattern)) => {
                    Value::Str(pattern.clone())
                }
                Some(crate::entities::numbering::Numbering::Func(func)) => {
                    Value::Func(func.clone())
                }
                None => Value::None,
            }),
            _ => {
                unreachable!(
                    "eval_location_method chamado com método inesperado: {method}"
                )
            }
        }
    };
    use crate::compiler::eval::ContextReadRequest as Read;
    let request = match method {
        "page" => Read::LocationPage { location: loc },
        "position" => Read::LocationPosition { location: loc },
        "page-numbering" => Read::LocationPageNumbering { location: loc },
        _ => unreachable!("location method already checked"),
    };
    ctx.observe_context_read(request, span, result)
}
