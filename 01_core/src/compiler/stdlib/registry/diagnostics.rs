//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/registry/diagnostics.md
//! @prompt-hash d7f272db
//! @layer L1
//! @updated 2026-09-26

use crate::compiler::stdlib::{
    native_assert, native_assert_eq, native_assert_ne, native_eval, native_numbering,
    native_panic,
};
use crate::entities::func::Func;
use crate::entities::scope::Scope;
use crate::entities::value::Value;
use std::sync::Arc;

pub(super) fn register(scope: &mut Scope) {
    // P723 — `assert` ganha namespace com eq/ne (bloqueio real do cetz;
    // a premissa do passo apontava `curve`, refutada pela sonda — o
    // namespace de curve existe desde P513).
    {
        let mut assert_namespace = Scope::new();
        assert_namespace
            .define("eq", Value::Func(Func::native("assert.eq", native_assert_eq)));
        assert_namespace
            .define("ne", Value::Func(Func::native("assert.ne", native_assert_ne)));
        scope.define(
            "assert",
            Value::Func(Func::native_with_namespace(
                "assert",
                native_assert,
                Arc::new(assert_namespace),
            )),
        );
    }
    scope.define("panic", Value::Func(Func::native("panic", native_panic)));
    scope.define(
        "numbering",
        Value::Func(Func::native_with_engine("numbering", native_numbering)),
    );
    // P394: eval(source) — re-avalia string como markup Typst no contexto actual.
    scope.define("eval", Value::Func(Func::native_with_engine("eval", native_eval)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn public_bindings_keep_their_kinds_and_names() {
        let mut scope = Scope::new();
        register(&mut scope);
        let Value::Func(func) = scope.get("assert").unwrap() else {
            panic!("assert function");
        };
        assert_eq!(func.name(), Some("assert"));
    }
}
