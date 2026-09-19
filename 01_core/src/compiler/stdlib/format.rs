//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/format.md
//! @prompt-hash 7ce52fc6
//! @layer L1
//! @updated 2026-09-19

use std::sync::Arc;

use crate::compiler::eval::EvalContext;
use crate::contracts::world::World;
use crate::entities::args::Args;
use crate::entities::file_id::FileId;
use crate::entities::func::Func;
use crate::entities::module::Module;
use crate::entities::scope::Scope;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;

fn native_svg(
    _ctx: &mut EvalContext,
    _args: &Args,
    _world: &dyn World,
    _current_file: FileId,
) -> SourceResult<Value> {
    Err(vec![SourceDiagnostic::error(Span::detached(), "cannot be constructed manually")])
}

pub(crate) fn make_format_module() -> Module {
    let mut svg_namespace = Scope::new();
    svg_namespace.define("pretty", Value::Bool(false));

    let mut scope = Scope::new();
    scope.define(
        "svg",
        Value::Func(Func::native_with_namespace(
            "svg",
            native_svg,
            Arc::new(svg_namespace),
        )),
    );
    Module::new("format", scope)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::value::Value;

    #[test]
    fn u01_format_module_has_exact_name() {
        let module = make_format_module();

        assert_eq!(module.name(), "format");
    }

    #[test]
    fn p1462_format_module_exposes_only_svg_with_pretty_default_false() {
        let module = make_format_module();

        assert_eq!(module.scope().len(), 1);
        let Value::Func(svg) = module.scope().get("svg").expect("format.svg") else {
            panic!("format.svg must be a function");
        };
        assert_eq!(svg.name(), Some("svg"));
        assert_eq!(
            svg.namespace().and_then(|scope| scope.get("pretty")),
            Some(&Value::Bool(false))
        );
        for name in ["png", "pdf", "html", "absent"] {
            assert!(module.scope().get(name).is_none(), "unexpected child: {name}");
        }
    }
}
