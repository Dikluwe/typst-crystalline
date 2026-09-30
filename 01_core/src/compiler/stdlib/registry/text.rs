//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/registry/text.md
//! @prompt-hash 3a4c8e9c
//! @layer L1
//! @updated 2026-09-26

use crate::compiler::stdlib::{
    native_highlight, native_lorem, native_lower, native_overline, native_replace,
    native_smallcaps, native_smartquote, native_strike, native_subscript,
    native_superscript, native_underline, native_upper,
};
use crate::entities::func::Func;
use crate::entities::scope::Scope;
use crate::entities::value::{Type, Value};

pub(super) fn register_decorations(scope: &mut Scope) {
    // P284 (ADR-0054 graded): text decoration — underline / strike /
    // overline. Cosméticos `stroke`/`offset`/`extent` opcionais; `evade`
    // e `background` scope-out per diagnóstico §A.1.
    scope.define("underline", Value::Func(Func::native("underline", native_underline)));
    scope.define("strike", Value::Func(Func::native("strike", native_strike)));
    scope.define("overline", Value::Func(Func::native("overline", native_overline)));
    // P408: smallcaps — variant + stdlib materializados; consumer em layout é
    // stub transparente (small caps real requer shaping OpenType, DEBT-53).
    scope.define("smallcaps", Value::Func(Func::native("smallcaps", native_smallcaps)));
    // P448: subscript e superscript via Content::Styled + Style.
    scope.define("sub", Value::Func(Func::native("sub", native_subscript)));
    scope.define("super", Value::Func(Func::native("super", native_superscript)));
    // P449: highlight via Content::Styled + Style::Highlight(fill).
    scope.define("highlight", Value::Func(Func::native("highlight", native_highlight)));
    // P287 (frente `P-smartquote`): função stdlib paralela ao markup `"..."`
    // P155. `alternative`/`quotes` scope-out per diagnóstico §A.2; `enabled:
    // false` emite glyph ASCII literal (paridade vanilla).
    scope
        .define("smartquote", Value::Func(Func::native("smartquote", native_smartquote)));
    scope.define("lorem", Value::Func(Func::native("lorem", native_lorem)));
    scope.define("regex", Value::Type(Type::Regex));
}

pub(super) fn register_case(scope: &mut Scope) {
    scope.define("upper", Value::Func(Func::native("upper", native_upper)));
    scope.define("lower", Value::Func(Func::native("lower", native_lower)));
    scope.define("replace", Value::Func(Func::native("replace", native_replace)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn public_bindings_keep_their_kinds_and_names() {
        let mut scope = Scope::new();
        register_decorations(&mut scope);
        register_case(&mut scope);
        let Value::Func(func) = scope.get("upper").unwrap() else {
            panic!("upper function");
        };
        assert_eq!(func.name(), Some("upper"));
    }
}
