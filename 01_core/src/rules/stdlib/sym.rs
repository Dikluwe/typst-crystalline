//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/stdlib/sym.md
//! @prompt-hash b90e8df3
//! @layer L1
//! @updated 2026-06-26
//!
//! Módulo `sym` — tabela estática de ~50 símbolos Unicode prioritários.
//!
//! Divergência declarada: o vanilla suporta modificadores encadeados
//! (`sym.arrow.r.double`) via `Modifier` struct. O cristalino implementa
//! apenas nomes compostos pré-definidos como chaves planas (`"arrow.r"`).
//! O acesso `sym.arrow.r` em eval falha no segundo FieldAccess (scope-out).
//!
//! **P731** — o módulo passou de `Value::Dict` a `Value::Module`
//! (paridade vanilla — medido: `type(sym)` → `module`).

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

/// Constrói o `Value::Module` que representa o módulo `sym` no scope.
///
/// Apenas as entradas com nome simples (sem `.`) ficam acessíveis via
/// eval FieldAccess (`sym.arrow`). Entradas compostas estão na tabela
/// para `sym_lookup` mas não no scope do módulo (evita conflito de tipo
/// entre `sym.eq` = Symbol e `sym.eq.not` = Symbol no mesmo nível).
///
/// **P731** — era `build_sym_dict` a devolver `Value::Dict`; passa a
/// `Value::Module` (paridade vanilla — medido: `type(sym)` → `module`).
pub fn build_sym_module() -> Value {
    let mut scope = crate::entities::scope::Scope::new();
    for (name, ch) in SYM_TABLE {
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
    fn build_sym_module_contem_simples() {
        let module = build_sym_module();
        if let Value::Module(m) = module {
            let s = m.scope();
            assert!(s.get("arrow").is_some());
            assert!(s.get("alpha").is_some());
            assert!(s.get("eq").is_some());
            // compostos não entram no scope
            assert!(s.get("eq.not").is_none());
            assert!(s.get("arrow.r").is_none());
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
