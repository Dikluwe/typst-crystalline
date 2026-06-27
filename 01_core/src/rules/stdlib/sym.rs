//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/stdlib/sym.md
//! @prompt-hash 02ac0712
//! @layer L1
//! @updated 2026-06-26
//!
//! Módulo `sym` — tabela estática de ~50 símbolos Unicode prioritários.
//!
//! Divergência declarada: o vanilla suporta modificadores encadeados
//! (`sym.arrow.r.double`) via `Modifier` struct. O cristalino implementa
//! apenas nomes compostos pré-definidos como chaves planas (`"arrow.r"`).
//! O acesso `sym.arrow.r` em eval falha no segundo FieldAccess (scope-out).

use indexmap::IndexMap;
use rustc_hash::FxBuildHasher;

use crate::entities::symbol::Symbol;
use crate::entities::value::Value;

/// Tabela estática de símbolos: `(nome, char)`.
///
/// Inclui entradas simples (`"arrow"`) e compostas pré-definidas
/// (`"arrow.r"`, `"eq.not"`). Chaves compostas são acessíveis via
/// `sym_lookup` (L1 directo) mas não via eval FieldAccess encadeado.
pub static SYM_TABLE: &[(&str, char)] = &[
    ("arrow",       '→'),
    ("arrow.l",     '←'),
    ("arrow.r",     '→'),
    ("arrow.t",     '↑'),
    ("arrow.b",     '↓'),
    ("arrow.lr",    '↔'),
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

/// Procura um símbolo pelo nome (incluindo nomes compostos pré-definidos).
pub fn sym_lookup(name: &str) -> Option<Symbol> {
    SYM_TABLE
        .iter()
        .find(|(n, _)| *n == name)
        .map(|(n, ch)| Symbol::new(*ch, *n))
}

/// Constrói o `Value::Dict` que representa o módulo `sym` no scope.
///
/// Apenas as entradas com nome simples (sem `.`) ficam acessíveis via
/// eval FieldAccess (`sym.arrow`). Entradas compostas estão na tabela
/// para `sym_lookup` mas não no Dict (evita conflito de tipo entre
/// `sym.eq` = Symbol e `sym.eq.not` = Symbol no mesmo nível).
pub fn build_sym_dict() -> Value {
    let mut map: IndexMap<ecow::EcoString, Value, FxBuildHasher> =
        IndexMap::with_hasher(FxBuildHasher::default());
    for (name, ch) in SYM_TABLE {
        if !name.contains('.') {
            map.insert(
                ecow::EcoString::from(*name),
                Value::Symbol(Symbol::new(*ch, *name)),
            );
        }
    }
    Value::Dict(map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sym_lookup_simples() {
        let s = sym_lookup("arrow").unwrap();
        assert_eq!(s.ch, '→');
        assert_eq!(s.name.as_str(), "arrow");
    }

    #[test]
    fn sym_lookup_composto() {
        let s = sym_lookup("eq.not").unwrap();
        assert_eq!(s.ch, '≠');
    }

    #[test]
    fn sym_lookup_inexistente() {
        assert!(sym_lookup("inexistente").is_none());
    }

    #[test]
    fn build_sym_dict_contem_simples() {
        let dict = build_sym_dict();
        if let Value::Dict(d) = dict {
            assert!(d.contains_key("arrow"));
            assert!(d.contains_key("alpha"));
            assert!(d.contains_key("eq"));
            // compostos não entram no dict
            assert!(!d.contains_key("eq.not"));
            assert!(!d.contains_key("arrow.r"));
        } else {
            panic!("esperado Value::Dict");
        }
    }

    #[test]
    fn build_sym_dict_arrow_e_symbol() {
        let dict = build_sym_dict();
        if let Value::Dict(d) = dict {
            let v = d.get("arrow").unwrap();
            if let Value::Symbol(s) = v {
                assert_eq!(s.ch, '→');
            } else {
                panic!("esperado Value::Symbol");
            }
        } else {
            panic!("esperado Value::Dict");
        }
    }
}
