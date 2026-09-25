//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/emoji.md
//! @prompt-hash cebde3f4
//! @layer L1
//! @updated 2026-09-24
//!
//! Módulo `emoji` — catálogo Unicode integral de `codex::EMOJI` no pin
//! git P1496 (dados idênticos aos de P1283), incluindo sequências
//! multi-codepoint e variantes ordenadas.

use crate::entities::symbol::Symbol;
use crate::entities::value::Value;

fn symbol_from_codex(name: &str, symbol: codex::Symbol) -> Symbol {
    let value = symbol
        .get(codex::ModifierSet::default())
        .expect("codex emoji must have a default best match")
        .0;
    let variants = symbol
        .variants()
        .map(|(modifiers, value, _)| (modifiers.as_str().into(), value.into()))
        .collect();
    Symbol::with_variants(value, name, variants)
}

fn module_from_codex(
    name: &str,
    module: codex::Module,
) -> crate::entities::module::Module {
    let mut scope = crate::entities::scope::Scope::new();
    for (binding_name, binding) in module.iter() {
        let value = match binding.def {
            codex::Def::Symbol(symbol) => {
                Value::Symbol(symbol_from_codex(binding_name, symbol))
            }
            codex::Def::Module(child) => {
                Value::Module(module_from_codex(binding_name, child))
            }
        };
        scope.define(binding_name, value);
    }
    crate::entities::module::Module::new(name, scope)
}

/// Constrói o `Value::Module` que representa o módulo `emoji` no scope.
///
/// Paridade vanilla — medido: `type(emoji)` → `module`.
pub fn build_emoji_module() -> Value {
    Value::Module(module_from_codex("emoji", codex::EMOJI))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1283_catalogo_emoji_codex_integral() {
        let Value::Module(module) = build_emoji_module() else {
            panic!("emoji must be module")
        };
        assert_eq!(module.scope().iter().count(), codex::EMOJI.iter().count());
        for (name, binding) in codex::EMOJI.iter() {
            let codex::Def::Symbol(expected) = binding.def else {
                panic!("emoji nested module")
            };
            let Some(Value::Symbol(actual)) = module.scope().get(name) else {
                panic!("missing emoji.{name}")
            };
            let base = expected.get(codex::ModifierSet::default()).unwrap().0;
            let expected_variants = expected
                .variants()
                .map(|(mods, value, _)| (mods.as_str().to_owned(), value))
                .collect::<Vec<_>>();
            let actual_variants = actual
                .variants
                .iter()
                .map(|(mods, value)| (mods.as_str().to_owned(), value.as_str()))
                .collect::<Vec<_>>();
            assert_eq!(actual.value, base, "wrong base for emoji.{name}");
            assert_eq!(
                actual_variants, expected_variants,
                "wrong variants for emoji.{name}"
            );
        }
    }

    #[test]
    fn p1283_parents_sem_bare_e_clusters_integrais() {
        let Value::Module(module) = build_emoji_module() else {
            panic!("emoji must be module")
        };
        let get = |name| match module.scope().get(name) {
            Some(Value::Symbol(symbol)) => symbol.value.as_str(),
            _ => panic!("missing emoji.{name}"),
        };
        assert_eq!(get("apple"), "🍏");
        assert_eq!(get("arrow"), "↙️");
        assert_eq!(get("bigfoot"), "🫈");
        let Some(Value::Symbol(dancing)) = module.scope().get("dancing") else {
            panic!("missing emoji.dancing")
        };
        assert!(dancing
            .variants
            .iter()
            .any(|(mods, value)| { mods == "ballet" && value == "🧑‍🩰" }));
    }
    #[test]
    fn p1496_emoji_catalog_stays_byte_equivalent_across_codex_pin() {
        let Value::Module(module) = build_emoji_module() else {
            panic!("emoji must be module")
        };
        assert_eq!(module.scope().len(), 772);
        for (name, expected) in [
            ("face", "😀"),
            ("apple", "🍏"),
            ("arrow", "↙\u{fe0f}"),
            ("heart", "❤\u{fe0f}"),
            ("bigfoot", "🫈"),
        ] {
            let Some(Value::Symbol(symbol)) = module.scope().get(name) else {
                panic!("missing emoji.{name}")
            };
            assert_eq!(symbol.value.as_str(), expected, "emoji.{name}");
        }
        let Some(Value::Symbol(heart)) = module.scope().get("heart") else {
            panic!("missing emoji.heart")
        };
        assert_eq!(heart.modified("arrow").unwrap().value.as_str(), "💘");
        assert_eq!(heart.modified("excl").unwrap().value.as_str(), "❣\u{fe0f}");
    }
}
