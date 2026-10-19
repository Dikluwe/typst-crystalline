//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/tests/symbols.md
//! @prompt-hash 9effdea0
//! @layer L1
//! @updated 2026-09-26

use super::*;

#[test]
fn p471_value_symbol_type_name() {
    use crate::entities::symbol::Symbol;
    let s = Symbol::new('α', "alpha");
    assert_eq!(Value::Symbol(s).type_name(), "symbol");
}

#[test]
fn p471_sym_dict_contem_arrow() {
    use super::sym::build_sym_module;
    // P731 — `sym` passa a ser `Value::Module` (era `Value::Dict`).
    let d = build_sym_module();
    if let Value::Module(m) = d {
        assert!(m.scope().get("arrow").is_some());
        assert!(m.scope().get("alpha").is_some());
    } else {
        panic!("esperado Value::Module");
    }
}

#[test]
fn p471_sym_dict_arrow_e_simbolo_correcto() {
    use super::sym::build_sym_module;
    // P731 — `sym` passa a ser `Value::Module` (era `Value::Dict`).
    let d = build_sym_module();
    if let Value::Module(m) = d {
        if let Some(Value::Symbol(s)) = m.scope().get("arrow") {
            assert_eq!(s.value, "→");
        } else {
            panic!("esperado Value::Symbol para arrow");
        }
    } else {
        panic!("esperado Value::Module");
    }
}
