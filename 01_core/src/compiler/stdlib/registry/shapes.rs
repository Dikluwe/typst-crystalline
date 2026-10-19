//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/registry/shapes.md
//! @prompt-hash 86645ec5
//! @layer L1
//! @updated 2026-09-26

use crate::compiler::stdlib::{
    native_circle, native_curve, native_curve_close, native_curve_cubic,
    native_curve_line, native_curve_move, native_curve_quad, native_ellipse, native_line,
    native_polygon, native_polygon_regular, native_rect, native_square,
};
use crate::entities::func::Func;
use crate::entities::scope::Scope;
use crate::entities::value::Value;
use std::sync::Arc;

pub(super) fn register(scope: &mut Scope) {
    scope.define("rect", Value::Func(Func::native("rect", native_rect)));
    scope.define("square", Value::Func(Func::native("square", native_square)));
    scope.define("ellipse", Value::Func(Func::native("ellipse", native_ellipse)));
    scope.define("circle", Value::Func(Func::native("circle", native_circle)));
    scope.define("line", Value::Func(Func::native("line", native_line)));
    // P1473 — `polygon` ganha namespace com `regular`.
    {
        let mut polygon_namespace = Scope::new();
        polygon_namespace.define(
            "regular",
            Value::Func(Func::native("polygon.regular", native_polygon_regular)),
        );
        scope.define(
            "polygon",
            Value::Func(Func::native_with_namespace(
                "polygon",
                native_polygon,
                Arc::new(polygon_namespace),
            )),
        );
    }
    // P293 (frente `P-curve-geometry`): activação posterior de
    // `PathItem::CubicTo` via stdlib novo. Reaplicação ADR-0099 para
    // `PathItem` (paralelo P285-P292 para `Style`). Hash `export.rs`
    // preservado pelo 10º passo consecutivo — emit já existe.
    // P513 — `curve` ganha namespace com move/line/cubic/quad/close.
    {
        let mut curve_namespace = Scope::new();
        curve_namespace
            .define("move", Value::Func(Func::native("curve.move", native_curve_move)));
        curve_namespace
            .define("line", Value::Func(Func::native("curve.line", native_curve_line)));
        curve_namespace.define(
            "cubic",
            Value::Func(Func::native("curve.cubic", native_curve_cubic)),
        );
        curve_namespace
            .define("quad", Value::Func(Func::native("curve.quad", native_curve_quad)));
        curve_namespace.define(
            "close",
            Value::Func(Func::native("curve.close", native_curve_close)),
        );
        scope.define(
            "curve",
            Value::Func(Func::native_with_namespace(
                "curve",
                native_curve,
                Arc::new(curve_namespace),
            )),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn public_bindings_keep_their_kinds_and_names() {
        let mut scope = Scope::new();
        register(&mut scope);
        let Value::Func(func) = scope.get("polygon").unwrap() else {
            panic!("polygon function");
        };
        assert_eq!(func.name(), Some("polygon"));
    }
}
