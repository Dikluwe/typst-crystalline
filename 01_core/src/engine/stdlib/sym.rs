//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/engine/stdlib/sym.md
//! @prompt-hash 381e76a5
//! @layer L1
//! @updated 2026-07-15
//!
//! Módulo `sym` — tabela estática de símbolos Unicode prioritários.
//!
//! **P765a**: suporte a modificadores encadeados (`sym.arrow.r.filled`)
//! via `Symbol::variants`.
//! **P766**: expansão por uso real do corpus — grupos com variantes
//! (`tilde`, `integral`, `chevron`, `suit`, `tack`, `space`, `emptyset`,
//! `bracket`, `amp`) e variantes adicionais de `plus`, `gt`, `diamond`.
//!
//! **P731** — o módulo passou de `Value::Dict` a `Value::Module`
//! (paridade vanilla — medido: `type(sym)` → `module`).

use crate::entities::symbol::{Symbol, SymbolVariant};
use crate::entities::value::Value;
use ecow::EcoString;

/// Símbolos simples: nome e caractere.
/// Entradas com `.` são variantes pré-definidas de grupos simples.
static SYM_SIMPLE: &[(&str, char)] = &[
    ("eq",          '='),
    ("eq.not",      '≠'),
    ("lt",          '<'),
    ("lt.eq",       '≤'),
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
    ("sqrt",        '√'),
    ("in",          '∈'),
    ("not.in",      '∉'),
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
    ("circle",      '○'),
    ("square",      '□'),
    ("copyright",   '©'),
    ("trademark",   '™'),
    ("registered",  '®'),
];

/// Constrói as variantes do grupo `arrow`.
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

/// P766 — grupo `plus` (uso real: `sym.plus.o` no corpus).
fn plus_variants() -> Vec<SymbolVariant> {
    vec![
        (EcoString::default(), '+'),
        ("o".into(), '⊕'),
        ("o.l".into(), '⨭'),
        ("o.r".into(), '⨮'),
        ("o.arrow".into(), '⟴'),
        ("o.big".into(), '⨁'),
        ("dot".into(), '∔'),
        ("double".into(), '⧺'),
        ("minus".into(), '±'),
        ("square".into(), '⊞'),
        ("triangle".into(), '⨹'),
        ("triple".into(), '⧻'),
        ("hat".into(), '⨣'),
    ]
}

/// P766 — grupo `gt` (uso real: `sym.gt.eq.tri.not` no corpus).
fn gt_variants() -> Vec<SymbolVariant> {
    vec![
        (EcoString::default(), '>'),
        ("o".into(), '⧁'),
        ("dot".into(), '⋗'),
        ("quest".into(), '⩼'),
        ("approx".into(), '⪆'),
        ("arc".into(), '⪧'),
        ("arc.eq".into(), '⪩'),
        ("closed".into(), '⊳'),
        ("closed.eq".into(), '⊵'),
        ("closed.eq.not".into(), '⋭'),
        ("closed.not".into(), '⋫'),
        ("double".into(), '≫'),
        ("double.nested".into(), '⪢'),
        ("eq".into(), '≥'),
        ("eq.slant".into(), '⩾'),
        ("eq.lt".into(), '⋛'),
        ("eq.not".into(), '≱'),
        ("equiv".into(), '≧'),
        ("lt".into(), '≷'),
        ("lt.not".into(), '≹'),
        ("neq".into(), '⪈'),
        ("napprox".into(), '⪊'),
        ("nequiv".into(), '≩'),
        ("not".into(), '≯'),
        ("ntilde".into(), '⋧'),
        ("tilde".into(), '≳'),
        ("tilde.not".into(), '≵'),
        ("tri".into(), '⊳'),
        ("tri.eq".into(), '⊵'),
        ("tri.eq.not".into(), '⋭'),
        ("tri.not".into(), '⋫'),
        ("triple".into(), '⋙'),
        ("triple.nested".into(), '⫸'),
    ]
}

/// P766 — grupo `diamond` (uso real: `sym.diamond.small` no corpus).
fn diamond_variants() -> Vec<SymbolVariant> {
    vec![
        ("blue".into(), '🔷'),
        ("blue.small".into(), '🔹'),
        ("orange".into(), '🔶'),
        ("orange.small".into(), '🔸'),
        ("dot".into(), '💠'),
    ]
}

/// P766 — grupo `tilde` (uso real: 19 ocorrências no corpus).
fn tilde_variants() -> Vec<SymbolVariant> {
    vec![
        (EcoString::default(), '∼'),
        ("op".into(), '∼'),
        ("basic".into(), '~'),
        ("dot".into(), '⩪'),
        ("eq".into(), '≃'),
        ("eq.not".into(), '≄'),
        ("eq.rev".into(), '⋍'),
        ("equiv".into(), '≅'),
        ("equiv.not".into(), '≇'),
        ("nequiv".into(), '≆'),
        ("not".into(), '≁'),
        ("rev".into(), '∽'),
        ("rev.equiv".into(), '≌'),
        ("triple".into(), '≋'),
    ]
}

/// P766 — grupo `integral` (uso real: 3 ocorrências no corpus).
fn integral_variants() -> Vec<SymbolVariant> {
    vec![
        (EcoString::default(), '∫'),
        ("arrow.hook".into(), '⨗'),
        ("ccw".into(), '⨑'),
        ("cont".into(), '∮'),
        ("cont.ccw".into(), '∳'),
        ("cont.cw".into(), '∲'),
        ("cw".into(), '∱'),
        ("dash".into(), '⨍'),
        ("dash.double".into(), '⨎'),
        ("double".into(), '∬'),
        ("quad".into(), '⨌'),
        ("inter".into(), '⨙'),
        ("slash".into(), '⨏'),
        ("square".into(), '⨖'),
        ("surf".into(), '∯'),
        ("times".into(), '⨘'),
        ("triple".into(), '∭'),
        ("union".into(), '⨚'),
        ("vol".into(), '∰'),
    ]
}

/// P766 — grupo `chevron` (uso real: 3 ocorrências no corpus).
fn chevron_variants() -> Vec<SymbolVariant> {
    vec![
        ("l".into(), '⟨'),
        ("l.curly".into(), '⧼'),
        ("l.dot".into(), '⦑'),
        ("l.closed".into(), '⦉'),
        ("l.double".into(), '⟪'),
        ("r".into(), '⟩'),
        ("r.curly".into(), '⧽'),
        ("r.dot".into(), '⦒'),
        ("r.closed".into(), '⦊'),
        ("r.double".into(), '⟫'),
    ]
}

/// P766 — grupo `suit` (uso real: 2 ocorrências no corpus).
fn suit_variants() -> Vec<SymbolVariant> {
    vec![
        ("club".into(), '♣'),
        ("diamond".into(), '♦'),
        ("heart".into(), '♥'),
        ("spade".into(), '♠'),
    ]
}

/// P766 — grupo `tack` (uso real: 1 ocorrência no corpus).
fn tack_variants() -> Vec<SymbolVariant> {
    vec![
        ("r".into(), '⊢'),
        ("r.not".into(), '⊬'),
        ("r.long".into(), '⟝'),
        ("r.short".into(), '⊦'),
        ("r.double".into(), '⊨'),
        ("rr".into(), '⊨'),
        ("r.double.not".into(), '⊭'),
        ("rr.not".into(), '⊭'),
        ("rrr".into(), '⫢'),
        ("l".into(), '⊣'),
        ("l.long".into(), '⟞'),
        ("l.short".into(), '⫞'),
        ("l.double".into(), '⫤'),
        ("ll".into(), '⫤'),
        ("t".into(), '⊥'),
        ("t.big".into(), '⟘'),
        ("t.double".into(), '⫫'),
        ("tt".into(), '⫫'),
        ("t.short".into(), '⫠'),
        ("b".into(), '⊤'),
        ("b.big".into(), '⟙'),
        ("b.double".into(), '⫪'),
        ("bb".into(), '⫪'),
        ("b.short".into(), '⫟'),
        ("l.r".into(), '⟛'),
    ]
}

/// P766 — grupo `space` (uso real: 1 ocorrência no corpus).
fn space_variants() -> Vec<SymbolVariant> {
    vec![
        (EcoString::default(), ' '),
        ("nobreak".into(), '\u{a0}'),
        ("nobreak.narrow".into(), '\u{202f}'),
        ("en".into(), '\u{2002}'),
        ("quad".into(), '\u{2003}'),
        ("third".into(), '\u{2004}'),
        ("quarter".into(), '\u{2005}'),
        ("sixth".into(), '\u{2006}'),
        ("med".into(), '\u{205f}'),
        ("fig".into(), '\u{2007}'),
        ("punct".into(), '\u{2008}'),
        ("thin".into(), '\u{2009}'),
        ("hair".into(), '\u{200a}'),
    ]
}

/// P766 — grupo `emptyset` (uso real: 1 ocorrência no corpus).
fn emptyset_variants() -> Vec<SymbolVariant> {
    vec![
        (EcoString::default(), '∅'),
        ("zero".into(), '∅'),
        ("arrow.r".into(), '⦳'),
        ("arrow.l".into(), '⦴'),
        ("bar".into(), '⦱'),
        ("circle".into(), '⦲'),
        ("rev".into(), '⦰'),
    ]
}

/// P766 — grupo `bracket` (uso real: 1 ocorrência no corpus).
fn bracket_variants() -> Vec<SymbolVariant> {
    vec![
        ("l".into(), '['),
        ("l.tick.t".into(), '⦍'),
        ("l.tick.b".into(), '⦏'),
        ("l.stroked".into(), '⟦'),
        ("r".into(), ']'),
        ("r.tick.t".into(), '⦐'),
        ("r.tick.b".into(), '⦎'),
        ("r.stroked".into(), '⟧'),
        ("t".into(), '⎴'),
        ("b".into(), '⎵'),
    ]
}

/// P766 — grupo `amp` (uso real: 1 ocorrência no corpus).
fn amp_variants() -> Vec<SymbolVariant> {
    vec![
        (EcoString::default(), '&'),
        ("inv".into(), '⅋'),
    ]
}

fn subset_variants() -> Vec<SymbolVariant> {
    vec![
        (EcoString::default(), '⊂'),
        ("eq".into(), '⊆'),
        ("neq".into(), '⊊'),
    ]
}

/// Lista de grupos com variantes: (nome, caractere base, função de variantes).
static SYM_GROUPS: &[(&str, char, fn() -> Vec<SymbolVariant>)] = &[
    ("arrow",    '→', arrow_variants),
    ("plus",     '+', plus_variants),
    ("gt",       '>', gt_variants),
    ("diamond",  '◇', diamond_variants),
    ("tilde",    '∼', tilde_variants),
    ("integral", '∫', integral_variants),
    ("chevron",  '⟨', chevron_variants),
    ("suit",     '♣', suit_variants),
    ("tack",     '⊢', tack_variants),
    ("space",    ' ', space_variants),
    ("emptyset", '∅', emptyset_variants),
    ("bracket",  '[', bracket_variants),
    ("amp",      '&', amp_variants),
    ("subset",   '⊂', subset_variants),
];

/// Procura um símbolo pelo nome. Entradas compostas pré-definidas
/// (`"arrow.r"`, `"eq.not"`) e modifiers encadeados (`"arrow.r.filled"`,
/// `"tilde.equiv"`) são resolvidos.
pub fn sym_lookup(name: &str) -> Option<Symbol> {
    // 1. Grupos com variantes.
    for (group, base, variants_fn) in SYM_GROUPS {
        if name == *group {
            return Some(Symbol::with_variants(*base, *group, variants_fn()));
        }
        let prefix = format!("{}.", group);
        if name.starts_with(&prefix) {
            let rest = &name[prefix.len()..];
            let mut s = Symbol::with_variants(*base, *group, variants_fn());
            for modifier in rest.split('.') {
                s = s.modified(modifier)?;
            }
            return Some(s);
        }
    }

    // 2. Símbolos simples (incluindo entradas compostas pré-definidas).
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

    for (group, base, variants_fn) in SYM_GROUPS {
        scope.define(*group, Value::Symbol(Symbol::with_variants(*base, *group, variants_fn())));
    }

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
    fn sym_lookup_composto_predefinido() {
        let s = sym_lookup("eq.not").unwrap();
        assert_eq!(s.ch, '≠');
    }

    #[test]
    fn sym_lookup_arrow_modifier() {
        let s = sym_lookup("arrow.r.filled").unwrap();
        assert_eq!(s.ch, '➡');
    }

    #[test]
    fn sym_lookup_tilde_modifier() {
        let s = sym_lookup("tilde.equiv").unwrap();
        assert_eq!(s.ch, '≅');
    }

    #[test]
    fn sym_lookup_integral_modifier() {
        let s = sym_lookup("integral.double").unwrap();
        assert_eq!(s.ch, '∬');
    }

    #[test]
    fn sym_lookup_chevron_modifier() {
        let s = sym_lookup("chevron.l").unwrap();
        assert_eq!(s.ch, '⟨');
    }

    #[test]
    fn sym_lookup_suit_modifier() {
        let s = sym_lookup("suit.heart").unwrap();
        assert_eq!(s.ch, '♥');
    }

    #[test]
    fn sym_lookup_tack_modifier() {
        let s = sym_lookup("tack.r.double").unwrap();
        assert_eq!(s.ch, '⊨');
    }

    #[test]
    fn sym_lookup_space_modifier() {
        let s = sym_lookup("space.nobreak").unwrap();
        assert_eq!(s.ch, '\u{a0}');
    }

    #[test]
    fn sym_lookup_emptyset_modifier() {
        let s = sym_lookup("emptyset.rev").unwrap();
        assert_eq!(s.ch, '⦰');
    }

    #[test]
    fn sym_lookup_bracket_modifier() {
        let s = sym_lookup("bracket.l.stroked").unwrap();
        assert_eq!(s.ch, '⟦');
    }

    #[test]
    fn sym_lookup_amp_modifier() {
        let s = sym_lookup("amp.inv").unwrap();
        assert_eq!(s.ch, '⅋');
    }

    #[test]
    fn sym_lookup_plus_o() {
        let s = sym_lookup("plus.o").unwrap();
        assert_eq!(s.ch, '⊕');
    }

    #[test]
    fn sym_lookup_gt_eq_tri_not() {
        let s = sym_lookup("gt.eq.tri.not").unwrap();
        assert_eq!(s.ch, '⋭');
    }

    #[test]
    fn sym_lookup_diamond_small() {
        let s = sym_lookup("diamond.small").unwrap();
        assert_eq!(s.ch, '🔹');
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
            assert!(s.get("tilde").is_some());
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
