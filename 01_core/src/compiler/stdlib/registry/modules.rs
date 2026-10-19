//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/registry/modules.md
//! @prompt-hash 950d9980
//! @layer L1
//! @updated 2026-09-26

use crate::compiler::stdlib::{
    build_emoji_module, build_sym_module, make_calc_module, make_math_module,
    make_sys_module,
};
use crate::contracts::world::SysInputs;
use crate::entities::scope::Scope;
use crate::entities::value::{Type, Value};

pub(super) fn register(scope: &mut Scope, inputs: &SysInputs, pdf: Value) {
    // P471 — módulo sym de símbolos Unicode. **P731** — `Value::Module`
    // (era `Value::Dict`; paridade vanilla `type(sym)` → `module`).
    scope.define("sym", build_sym_module());
    scope.define("calc", make_calc_module());
    // P735 — módulos `emoji` (tabela codex, 1 codepoint + face) e `pdf`
    // (attach = scope-out com erro; artifact = passthrough do body).
    scope.define("emoji", build_emoji_module());
    scope.define("pdf", pdf);
    // P476 — operadores de cor. **P736** — `color` é `Value::Type`
    // (paridade vanilla — medido: `type(color)` → `type`); fields via
    // `color_type_field` em field access.
    scope.define("color", Value::Type(Type::Color));
    // P262 — `gradient.linear(...)` via field access no tipo (P736; era
    // module dict, ADR-0087). Paridade vanilla: `type(gradient)` → `type`.
    scope.define("gradient", Value::Type(Type::Gradient));
    // P299 — `math.sin`/`math.lim`/etc. (P298.X; 42 operadores
    // pré-definidos paridade vanilla via SSoT MathOp).
    scope.define("math", make_math_module());
    // P694 — módulo `sys` (sys.version / sys.inputs); `inputs` vem de
    // `World::inputs()` (vazio por omissão).
    scope.define("sys", make_sys_module(inputs));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn public_bindings_keep_their_kinds_and_names() {
        let mut scope = Scope::new();
        register(
            &mut scope,
            &SysInputs::default(),
            crate::compiler::stdlib::make_pdf_module(
                crate::entities::compiler_features::Features::empty(),
            ),
        );
        let Value::Func(func) = scope.get("pdf").unwrap() else {
            panic!("pdf function");
        };
        assert_eq!(func.name(), Some("pdf"));
    }
}
