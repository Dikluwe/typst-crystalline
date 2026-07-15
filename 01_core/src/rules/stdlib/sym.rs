//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/stdlib/sym.md
//! @prompt-hash b90e8df3
//! @layer L1
//! @updated 2026-07-15
//!
//! Módulo `sym` — tabela estática de símbolos Unicode prioritários.
//!
//! **P765a**: suporte a modificadores encadeados (`sym.arrow.r.filled`)
//! via `Symbol::variants`. O acesso `sym.arrow` devolve um symbol com
//! variantes; `sym.arrow.r.filled` aplica modifiers encadeados.
//!
//! **P731** — o módulo passou de `Value::Dict` a `Value::Module`
//! (paridade vanilla — medido: `type(sym)` → `module`).

use crate::entities::symbol::{Symbol, SymbolVariant};
use crate::entities::value::Value;
use ecow::EcoString;

/// Símbolos simples: nome e caractere.
static SYM_SIMPLE: &[(&str, char)] = &[
    ("eq",          '='),
    ("eq.not",      '≠'),
    ("lt",          '<'),
    ("gt",          '>'),
    ("lt.eq",       '≤'),
    ("gt.eq",       '≥'),
    ("plus",        '+'),
    ("minus",       '−'),
    ("times",       '×'),
    ("div",         '÷'),
    ("dot",         '·'),
    ("dots",        '…'),
    ("alpha",       'α'),
    ("beta",        'β'),
    ("gamma",       'γ'),
    ("delta",       'δ'),
    ("epsilon",     'ε'),
    ("zeta",        'ζ'),
    ("eta",         'η'),
    ("theta",       'θ'),
    ("iota",        'ι'),
    ("kappa",       'κ'),
    ("lambda",      'λ'),
    ("mu",          'μ'),
    ("nu",          'ν'),
    ("xi",          'ξ'),
    ("pi",          'π'),
    ("rho",         'ρ'),
    ("sigma",       'σ'),
    ("tau",         'τ'),
    ("upsilon",     'υ'),
    ("phi",         'φ'),
    ("chi",         'χ'),
    ("psi",         'ψ'),
    ("omega",       'ω'),
    ("infinity",    '∞'),
    ("sum",         '∑'),
    ("product",     '∏'),
    ("integral",    '∫'),
    ("sqrt",        '√'),
    ("in",          '∈'),
    ("not.in",      '∉'),
    ("subset",      '⊂'),
    ("supset",      '⊃'),
    ("union",       '∪'),
    ("sect",        '∩'),
    ("and",         '∧'),
    ("or",          '∨'),
    ("not",         '¬'),
    ("forall",      '∀'),
    ("exists",      '∃'),
    ("dagger",      '†'),
    ("star",        '⋆'),
    ("bullet",      '•'),
    ("diamond",     '◇'),
    ("circle",      '○'),
    ("square",      '□'),
    ("copyright",   '©'),
    ("trademark",   '™'),
    ("registered",  '®'),
];

/// Variantes do símbolo `arrow` (medidas no vanilla CLI 0.15.0).
fn arrow_variants() -> Vec<SymbolVariant> {
    vec![
        (EcoString::default(), '→'),
        ("long.bar".into(), '⟼'),
        ("bar".into(), '↦'),
        ("curve".into(), '⤷'),
        ("turn".into(), '⮎'),
        ("dashed".into(), '⇢'),
        ("dotted".into(), '⤑'),
        ("double".into(), '⇒'),
        ("double.bar".into(), '⤇'),
        ("double.long".into(), '⟹'),
        ("double.long.bar".into(), '⟾'),
        ("double.not".into(), '⇏'),
        ("double.struck".into(), '⤃'),
        ("filled".into(), '➡'),
        ("hook".into(), '↪'),
        ("long".into(), '⟶'),
        ("r".into(), '→'),
        ("r.filled".into(), '➡'),
        ("long.squiggly".into(), '⟿'),
        ("loop".into(), '↬'),
        ("not".into(), '↛'),
        ("quad".into(), '⭆'),
        ("squiggly".into(), '⇝'),
        ("stop".into(), '⇥'),
        ("stroked".into(), '⇨'),
        ("struck".into(), '⇸'),
        ("dstruck".into(), '⇻'),
        ("tail".into(), '↣'),
        ("tail.struck".into(), '⤔'),
        ("tail.dstruck".into(), '⤕'),
        ("tilde".into(), '⥲'),
        ("triple".into(), '⇛'),
        ("twohead".into(), '↠'),
        ("twohead.bar".into(), '⤅'),
        ("twohead.struck".into(), '⤀'),
        ("twohead.dstruck".into(), '⤁'),
        ("twohead.tail".into(), '⤖'),
        ("twohead.tail.struck".into(), '⤗'),
        ("twohead.tail.dstruck".into(), '⤘'),
        ("open".into(), '⇾'),
        ("wave".into(), '↝'),
        ("l".into(), '←'),
        ("l.double".into(), '⇔'),
        ("l.double.long".into(), '⟺'),
        ("l.double.not".into(), '⇎'),
        ("l.double.struck".into(), '⤄'),
        ("l.filled".into(), '⬌'),
        ("l.long".into(), '⟷'),
        ("l.not".into(), '↮'),
        ("l.stroked".into(), '⬄'),
        ("l.struck".into(), '⇹'),
        ("l.dstruck".into(), '⇼'),
        ("l.open".into(), '⇿'),
        ("l.wave".into(), '↭'),
    ]
}

/// Constrói o symbol `arrow` com todas as variantes.
fn arrow_symbol() -> Symbol {
    Symbol::with_variants('→', "arrow", arrow_variants())
}

/// Procura um símbolo pelo nome. Entradas compostas pré-definidas
/// (`"arrow.r"`, `"eq.not"`) devolvem um symbol simples com o caractere
/// resultante.
pub fn sym_lookup(name: &str) -> Option<Symbol> {
    if name == "arrow" {
        return Some(arrow_symbol());
    }
    if name.starts_with("arrow.") {
        let rest = &name["arrow.".len()..];
        let mut s = arrow_symbol();
        for modifier in rest.split('.') {
            s = s.modified(modifier)?;
        }
        return Some(s);
    }
    SYM_SIMPLE
        .iter()
        .find(|(n, _)| *n == name)
        .map(|(n, ch)| Symbol::new(*ch, *n))
}

/// Constrói o `Value::Module` que representa o módulo `sym` no scope.
///
/// Apenas as entradas com nome simples (sem `.`) ficam acessíveis via
/// eval FieldAccess (`sym.arrow`). Entradas compostas estão disponíveis
/// via `sym_lookup`.
pub fn build_sym_module() -> Value {
    let mut scope = crate::entities::scope::Scope::new();
    scope.define("arrow", Value::Symbol(arrow_symbol()));
    for (name, ch) in SYM_SIMPLE {
        if !name.contains('.') {
            scope.define(*name, Value::Symbol(Symbol::new(*ch, *name)));
        }
    }
    Value::Module(crate::entities::module::Module::new("sym", scope))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sym_lookup_simples() {
        let s = sym_lookup("alpha").unwrap();
        assert_eq!(s.ch, 'α');
        assert_eq!(s.name.as_str(), "alpha");
    }

    #[test]
    fn sym_lookup_composto() {
        let s = sym_lookup("eq.not").unwrap();
        assert_eq!(s.ch, '≠');
    }

    #[test]
    fn sym_lookup_arrow_modifier() {
        let s = sym_lookup("arrow.r.filled").unwrap();
        assert_eq!(s.ch, '➡');
    }

    #[test]
    fn sym_lookup_inexistente() {
        assert!(sym_lookup("inexistente").is_none());
    }

    #[test]
    fn build_sym_module_contem_simples() {
        let module = build_sym_module();
        if let Value::Module(m) = module {
            let s = m.scope();
            assert!(s.get("arrow").is_some());
            assert!(s.get("alpha").is_some());
            assert!(s.get("eq").is_some());
        } else {
            panic!("esperado Value::Module");
        }
    }

    #[test]
    fn build_sym_module_arrow_e_symbol() {
        let module = build_sym_module();
        if let Value::Module(m) = module {
            let v = m.scope().get("arrow").unwrap();
            if let Value::Symbol(s) = v {
                assert_eq!(s.ch, '→');
            } else {
                panic!("esperado Value::Symbol");
            }
        } else {
            panic!("esperado Value::Module");
        }
    }
}
