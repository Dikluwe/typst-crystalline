//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/format.md
//! @prompt-hash bc323517
//! @layer L1
//! @updated 2026-09-20

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

fn native_format(
    _ctx: &mut EvalContext,
    _args: &Args,
    _world: &dyn World,
    _current_file: FileId,
) -> SourceResult<Value> {
    Err(vec![SourceDiagnostic::error(Span::detached(), "cannot be constructed manually")])
}

fn projected_pdf_function(pdf_scope: &Scope, name: &str) -> Value {
    let Some(Value::Func(function)) = pdf_scope.get(name) else {
        panic!("pdf composition requires function binding `{name}`");
    };
    Value::Func(function.clone())
}

pub(crate) fn make_format_module(pdf: &Value) -> Module {
    let Value::Module(pdf) = pdf else {
        panic!("format.pdf composition requires the effective pdf module");
    };
    assert_eq!(pdf.name(), "pdf", "format.pdf composition requires module `pdf`");

    let mut svg_namespace = Scope::new();
    svg_namespace.define("pretty", Value::Bool(false));

    let mut png_namespace = Scope::new();
    png_namespace.define("ppi", Value::Float(144.0));

    let pdf_scope = pdf.scope();
    let gated_members = ["table-summary", "header-cell", "data-cell"];
    let gated_count = gated_members
        .iter()
        .filter(|name| pdf_scope.get(name).is_some())
        .count();
    assert!(
        matches!(gated_count, 0 | 3),
        "pdf composition requires all or none of the a11y-extras bindings"
    );

    let mut pdf_namespace = Scope::new();
    pdf_namespace.define("pages", Value::None);
    pdf_namespace.define("standard", Value::Array(vec![Value::Str("1.7".into())]));
    pdf_namespace.define("tagged", Value::Auto);
    pdf_namespace.define("pretty", Value::Bool(false));
    for name in ["attach", "artifact"] {
        pdf_namespace.define(name, projected_pdf_function(pdf_scope, name));
    }
    if gated_count == gated_members.len() {
        for name in gated_members {
            pdf_namespace.define(name, projected_pdf_function(pdf_scope, name));
        }
    }

    let mut scope = Scope::new();
    scope.define(
        "svg",
        Value::Func(Func::native_with_namespace(
            "svg",
            native_format,
            Arc::new(svg_namespace),
        )),
    );
    scope.define(
        "png",
        Value::Func(Func::native_with_namespace(
            "png",
            native_format,
            Arc::new(png_namespace),
        )),
    );
    scope.define(
        "pdf",
        Value::Func(Func::native_with_namespace(
            "pdf",
            native_format,
            Arc::new(pdf_namespace),
        )),
    );
    Module::new("format", scope)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::stdlib::pdf::make_pdf_module;
    use crate::entities::compiler_features::Features;
    use crate::entities::value::Value;

    fn make_default_format_module() -> Module {
        let pdf = make_pdf_module(Features::empty());
        make_format_module(&pdf)
    }

    #[test]
    fn u01_format_module_has_exact_name() {
        let module = make_default_format_module();

        assert_eq!(module.name(), "format");
    }

    #[test]
    fn p1464_format_module_exposes_svg_png_and_pdf_without_html() {
        let module = make_default_format_module();

        assert_eq!(module.scope().len(), 3);
        let Value::Func(svg) = module.scope().get("svg").expect("format.svg") else {
            panic!("format.svg must be a function");
        };
        assert_eq!(svg.name(), Some("svg"));
        let svg_namespace = svg.namespace().expect("format.svg namespace");
        assert_eq!(svg_namespace.len(), 1);
        assert_eq!(svg_namespace.get("pretty"), Some(&Value::Bool(false)));

        let Value::Func(png) = module.scope().get("png").expect("format.png") else {
            panic!("format.png must be a function");
        };
        assert_eq!(png.name(), Some("png"));
        let png_namespace = png.namespace().expect("format.png namespace");
        assert_eq!(png_namespace.len(), 1);
        assert_eq!(png_namespace.get("ppi"), Some(&Value::Float(144.0)));

        let Value::Func(pdf) = module.scope().get("pdf").expect("format.pdf") else {
            panic!("format.pdf must be a function");
        };
        assert_eq!(pdf.name(), Some("pdf"));
        assert!(module.scope().get("html").is_none(), "unexpected child: html");
    }
}
