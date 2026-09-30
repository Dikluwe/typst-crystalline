//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/html/namespace.md
//! @prompt-hash 80358491
//! @layer L1
//! @updated 2026-09-30

use super::*;

/// Constrói o valor `html` como `Value::Func` com namespace fechado
/// (`elem`, `frame`, as tags tipadas) — paridade com o vanilla ratificado,
/// onde `type(html) == function` (ver `compiler/stdlib/html.md` P1471).
/// Chamar `html(...)` diretamente é sempre erro; só os membros do
/// namespace são chamáveis.
pub fn make_html_module() -> Value {
    let mut scope = Scope::new();
    scope.define("elem", Value::Func(Func::native("elem", native_html_elem)));
    scope.define("frame", Value::Func(Func::native("frame", native_html_frame)));
    for (name, native) in TYPED_TAGS {
        scope.define(*name, Value::Func(Func::native(name, *native)));
    }
    Value::Func(Func::native_with_namespace("html", native_html_root, Arc::new(scope)))
}

/// `html(...)` não pode ser construído manualmente — mesmo diagnóstico
/// medido no vanilla ratificado `586e1bd43fae6c9a973218163d3165c53ab8d16d`
/// para `pdf()`/`html()` (`compiler/stdlib/html.md` P1471).
pub(super) fn native_html_root(
    _ctx: &mut EvalContext,
    _args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    Err(vec![SourceDiagnostic::error(Span::detached(), "cannot be constructed manually")])
}
