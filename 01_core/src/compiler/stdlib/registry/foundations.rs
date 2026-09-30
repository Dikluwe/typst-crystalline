//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/registry/foundations.md
//! @prompt-hash 0282a74f
//! @layer L1
//! @updated 2026-09-26

use crate::compiler::stdlib::{
    native_cmyk, native_luma, native_oklab, native_oklch, native_range, native_repr,
    native_rgb,
};
use crate::entities::func::Func;
use crate::entities::scope::Scope;
use crate::entities::value::{Type, Value};

pub(super) fn register_types(scope: &mut Scope) {
    // P685 — `type` é um valor-tipo chamável (invoca native_type via eval_func_call).
    scope.define("type", Value::Type(Type::Type));
    scope.define("repr", Value::Func(Func::native("repr", native_repr)));
    scope.define("range", Value::Func(Func::native("range", native_range)));
    // P1300 — cinco constructors globais ratificados: `rgb`, `luma`, `cmyk`,
    // `oklab` e `oklch`; `linear-rgb`, `hsl` e `hsv` são somente `color.*`.
    scope.define("rgb", Value::Func(Func::native("rgb", native_rgb)));
    scope.define("luma", Value::Func(Func::native("luma", native_luma)));
    scope.define("oklab", Value::Func(Func::native("oklab", native_oklab)));
    scope.define("oklch", Value::Func(Func::native("oklch", native_oklch)));
    scope.define("cmyk", Value::Func(Func::native("cmyk", native_cmyk)));
    // P685 — `str`, `int`, `float` são valores-tipo chamáveis. Os campos
    // `str.from-unicode` e `int.min`/`int.max` são agora resolvidos por field
    // access em `Value::Type` (ver `eval_field_access` em bindings.rs).
    scope.define("str", Value::Type(Type::Str));
    scope.define("int", Value::Type(Type::Int));
    scope.define("float", Value::Type(Type::Float));
    scope.define("path", Value::Type(Type::Path));

    // P685 — nomes de tipo como valores de primeira classe (sem colisão com
    // nomes já registados como função/módulo). Nenhum é chamável. Permite
    // `type(x) == length`, `type(x) == ratio`, etc. (paridade vanilla).
    scope.define("bool", Value::Type(Type::Bool));
    scope.define("length", Value::Type(Type::Length));
    scope.define("ratio", Value::Type(Type::Ratio));
    // P842 (#32) — binding global `relative` (paridade vanilla medida:
    // `type(30% + 1em) == relative` → true).
    scope.define("relative", Value::Type(Type::Relative));
    scope.define("angle", Value::Type(Type::Angle));
    scope.define("fraction", Value::Type(Type::Fraction));
    scope.define("array", Value::Type(Type::Array));
    scope.define("dictionary", Value::Type(Type::Dictionary));
    scope.define("function", Value::Type(Type::Function));
    scope.define("content", Value::Type(Type::Content));
    scope.define("arguments", Value::Type(Type::Arguments));
    scope.define("module", Value::Type(Type::Module));
    scope.define("datetime", Value::Type(Type::Datetime));
    scope.define("bytes", Value::Type(Type::Bytes));
    scope.define("symbol", Value::Type(Type::Symbol));
    scope.define("alignment", Value::Type(Type::Alignment));
    scope.define("direction", Value::Type(Type::Direction));
    scope.define("location", Value::Type(Type::Location));
    // P1140.1-B — tipos chamáveis; call_dispatch delega aos construtores.
    scope.define("decimal", Value::Type(Type::Decimal));
    scope.define("duration", Value::Type(Type::Duration));
    scope.define("version", Value::Type(Type::Version));
}

pub(super) fn register_constants(scope: &mut Scope) {
    // Constantes de alinhamento (Passo 84.5, encerra DEBT-36).
    // Sintaxe preferida: `align(center, ...)`, `align(center + bottom, ...)`.
    use crate::entities::layout_types::{Align2D, HAlign, VAlign};
    scope.define("left", Value::Align(Align2D { h: Some(HAlign::Left), v: None }));
    scope.define("center", Value::Align(Align2D { h: Some(HAlign::Center), v: None }));
    scope.define("right", Value::Align(Align2D { h: Some(HAlign::Right), v: None }));
    scope.define("start", Value::Align(Align2D { h: Some(HAlign::Start), v: None }));
    scope.define("end", Value::Align(Align2D { h: Some(HAlign::End), v: None }));
    scope.define("top", Value::Align(Align2D { h: None, v: Some(VAlign::Top) }));
    scope.define("horizon", Value::Align(Align2D { h: None, v: Some(VAlign::Horizon) }));
    scope.define("bottom", Value::Align(Align2D { h: None, v: Some(VAlign::Bottom) }));

    // Constantes de direcção (Passo 576).
    use crate::entities::dir::Dir;
    scope.define("ltr", Value::Dir(Dir::LTR));
    scope.define("rtl", Value::Dir(Dir::RTL));
    scope.define("ttb", Value::Dir(Dir::TTB));
    scope.define("btt", Value::Dir(Dir::BTT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn public_bindings_keep_their_kinds_and_names() {
        let mut scope = Scope::new();
        register_types(&mut scope);
        register_constants(&mut scope);
        let Value::Func(func) = scope.get("rgb").unwrap() else {
            panic!("rgb function");
        };
        assert_eq!(func.name(), Some("rgb"));
    }
}
