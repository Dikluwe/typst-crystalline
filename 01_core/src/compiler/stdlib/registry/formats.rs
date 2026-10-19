//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/registry/formats.md
//! @prompt-hash e5b30691
//! @layer L1
//! @updated 2026-09-26

use crate::compiler::stdlib::{
    make_format_module, make_html_module, make_pdf_module, native_asset,
};
use crate::entities::compiler_features::Features;
use crate::entities::func::Func;
use crate::entities::scope::Scope;
use crate::entities::value::Value;

pub(super) fn register(scope: &mut Scope, features: Features) -> Value {
    let pdf = make_pdf_module(features);
    let html = make_html_module();
    let asset = Value::Func(Func::native("asset", native_asset));
    scope.define("format", Value::Module(make_format_module(&pdf, &html, &asset)));
    if features.contains(crate::entities::compiler_features::Feature::Bundle) {
        scope.define("asset", asset);
    }
    scope.define("html", html);

    pdf
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn public_bindings_keep_their_kinds_and_names() {
        let mut scope = Scope::new();
        register(&mut scope, Features::empty());
        let Value::Module(format) = scope.get("format").unwrap() else {
            panic!("format module");
        };
        assert_eq!(format.name(), "format");
        assert!(scope.get("asset").is_none());
    }
}
