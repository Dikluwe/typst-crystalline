//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/registry/math.md
//! @prompt-hash 57755679
//! @layer L1
//! @updated 2026-09-26

use crate::compiler::stdlib::{
    native_accent, native_bb, native_bold, native_cal, native_cancel, native_display,
    native_frak, native_inline, native_math_italic, native_mono, native_op, native_sans,
    native_scr, native_script, native_serif, native_sscript, native_underover,
    native_upright,
};
use crate::entities::func::Func;
use crate::entities::scope::Scope;
use crate::entities::value::Value;

pub(super) fn register_globals(scope: &mut Scope) {
    // Passo 296 — math accent + cancel (HIV + (a) minimal).
    scope.define("accent", Value::Func(Func::native("accent", native_accent)));
    scope.define("cancel", Value::Func(Func::native("cancel", native_cancel)));
    // Passo 297 — math underover (HV'.a + (b) Option fields).
    scope.define("underover", Value::Func(Func::native("underover", native_underover)));
    // Passo 298 — math op (HV'' adaptado; cross-variant interaction).
    scope.define("op", Value::Func(Func::native_with_engine("op", native_op)));
    // P311b.3 + P765b — 14 funções math style.
    scope.define("bb", Value::Func(Func::native("bb", native_bb)));
    scope.define("bold", Value::Func(Func::native("bold", native_bold)));
    scope.define("cal", Value::Func(Func::native("cal", native_cal)));
    scope.define("display", Value::Func(Func::native("display", native_display)));
    scope.define("frak", Value::Func(Func::native("frak", native_frak)));
    scope.define("inline", Value::Func(Func::native("inline", native_inline)));
    scope.define("italic", Value::Func(Func::native("italic", native_math_italic)));
    scope.define("mono", Value::Func(Func::native("mono", native_mono)));
    scope.define("sans", Value::Func(Func::native("sans", native_sans)));
    scope.define("scr", Value::Func(Func::native("scr", native_scr)));
    scope.define("script", Value::Func(Func::native("script", native_script)));
    scope.define("serif", Value::Func(Func::native("serif", native_serif)));
    scope.define("sscript", Value::Func(Func::native("sscript", native_sscript)));
    scope.define("upright", Value::Func(Func::native("upright", native_upright)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn public_bindings_keep_their_kinds_and_names() {
        let mut scope = Scope::new();
        register_globals(&mut scope);
        let Value::Func(func) = scope.get("op").unwrap() else {
            panic!("op function");
        };
        assert_eq!(func.name(), Some("op"));
    }
}
