//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/registry/figure_image.md
//! @prompt-hash 19444b15
//! @layer L1
//! @updated 2026-09-26

use crate::compiler::stdlib::{native_figure, native_figure_caption, native_image};
use crate::entities::func::Func;
use crate::entities::scope::Scope;
use crate::entities::value::{Type, Value};
use std::sync::Arc;

pub(super) fn register(scope: &mut Scope) {
    // P1473 — `figure` ganha namespace com `caption`.
    {
        let mut figure_namespace = Scope::new();
        figure_namespace.define(
            "caption",
            Value::Func(Func::native("figure.caption", native_figure_caption)),
        );
        scope.define(
            "figure",
            Value::Func(Func::native_with_namespace(
                "figure",
                native_figure,
                Arc::new(figure_namespace),
            )),
        );
    }
    scope.define("image", Value::Func(Func::native("image", native_image)));
    // P396 — constructor `tiling(...)` (pattern fill).
    scope.define("tiling", Value::Type(Type::Tiling));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn public_bindings_keep_their_kinds_and_names() {
        let mut scope = Scope::new();
        register(&mut scope);
        let Value::Func(func) = scope.get("figure").unwrap() else {
            panic!("figure function");
        };
        assert_eq!(func.name(), Some("figure"));
    }
}
