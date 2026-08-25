//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/sym.md
//! @prompt-hash 0843830c
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
static SYM_SIMPLE: &[(&str, &str)] = &[
    ("eq", "="),
    ("eq.not", "≠"),
    ("lt", "<"),
    ("lt.eq", "≤"),
    ("minus", "−"),
    ("times", "×"),
    ("div", "÷"),
    // **P894** — `dot` bare é U+22C5 (DOT OPERATOR), paridade vanilla
    // (codex `sym.txt`: `dot` bare = variante `.op`). `.c` preserva o
    // U+00B7 (MIDDLE DOT) que estava incorrectamente em `dot` bare.
    ("dot", "⋅"),
    ("dot.c", "·"),
    ("alpha", "α"),
    ("beta", "β"),
    ("gamma", "γ"),
    ("delta", "δ"),
    ("zeta", "ζ"),
    ("eta", "η"),
    ("iota", "ι"),
    ("kappa", "κ"),
    ("lambda", "λ"),
    ("mu", "μ"),
    ("nu", "ν"),
    ("xi", "ξ"),
    ("pi", "π"),
    ("tau", "τ"),
    ("upsilon", "υ"),
    ("chi", "χ"),
    ("psi", "ψ"),
    ("omega", "ω"),
    ("infinity", "∞"),
    // **P895** — `oo`: atalho de `infinity`, paridade vanilla (`codex`: `oo ∞`).
    ("oo", "∞"),
    ("sum", "∑"),
    ("product", "∏"),
    ("in", "∈"),
    ("not.in", "∉"),
    // **P895** — Hebraico usado em teoria de conjuntos/cardinais (beth,
    // paridade `codex`: `beth ב`).
    ("beth", "ב"),
    // **P895** — atalho de `propto`/relação "proporcional a" (`codex`: `prop ∝`).
    ("prop", "∝"),
    ("and", "∧"),
    ("or", "∨"),
    ("not", "¬"),
    ("forall", "∀"),
    ("exists", "∃"),
    ("dagger", "†"),
    ("star", "⋆"),
    ("bullet", "•"),
    ("circle", "○"),
    ("square", "□"),
    ("copyright", "©"),
    ("trademark", "™"),
    ("registered", "®"),
];

/// Constrói as variantes do grupo `arrow`.
fn arrow_variants() -> Vec<SymbolVariant> {
    vec![
        (EcoString::default(), '→'.into()),
        ("long.bar".into(), '⟼'.into()),
        ("bar".into(), '↦'.into()),
        ("curve".into(), '⤷'.into()),
        ("turn".into(), '⮎'.into()),
        ("dashed".into(), '⇢'.into()),
        ("dotted".into(), '⤑'.into()),
        ("double".into(), '⇒'.into()),
        ("double.bar".into(), '⤇'.into()),
        ("double.long".into(), '⟹'.into()),
        ("double.long.bar".into(), '⟾'.into()),
        ("double.not".into(), '⇏'.into()),
        ("double.struck".into(), '⤃'.into()),
        ("filled".into(), '➡'.into()),
        ("hook".into(), '↪'.into()),
        ("long".into(), '⟶'.into()),
        ("r".into(), '→'.into()),
        ("r.filled".into(), '➡'.into()),
        ("long.squiggly".into(), '⟿'.into()),
        ("loop".into(), '↬'.into()),
        ("not".into(), '↛'.into()),
        ("quad".into(), '⭆'.into()),
        ("squiggly".into(), '⇝'.into()),
        ("stop".into(), '⇥'.into()),
        ("stroked".into(), '⇨'.into()),
        ("struck".into(), '⇸'.into()),
        ("dstruck".into(), '⇻'.into()),
        ("tail".into(), '↣'.into()),
        ("tail.struck".into(), '⤔'.into()),
        ("tail.dstruck".into(), '⤕'.into()),
        ("tilde".into(), '⥲'.into()),
        ("triple".into(), '⇛'.into()),
        ("twohead".into(), '↠'.into()),
        ("twohead.bar".into(), '⤅'.into()),
        ("twohead.struck".into(), '⤀'.into()),
        ("twohead.dstruck".into(), '⤁'.into()),
        ("twohead.tail".into(), '⤖'.into()),
        ("twohead.tail.struck".into(), '⤗'.into()),
        ("twohead.tail.dstruck".into(), '⤘'.into()),
        ("open".into(), '⇾'.into()),
        ("wave".into(), '↝'.into()),
        ("l".into(), '←'.into()),
        ("l.double".into(), '⇔'.into()),
        ("l.double.long".into(), '⟺'.into()),
        ("l.double.not".into(), '⇎'.into()),
        ("l.double.struck".into(), '⤄'.into()),
        ("l.filled".into(), '⬌'.into()),
        ("l.long".into(), '⟷'.into()),
        ("l.not".into(), '↮'.into()),
        ("l.stroked".into(), '⬄'.into()),
        ("l.struck".into(), '⇹'.into()),
        ("l.dstruck".into(), '⇼'.into()),
        ("l.open".into(), '⇿'.into()),
        ("l.wave".into(), '↭'.into()),
    ]
}

/// P766 — grupo `plus` (uso real: `sym.plus.o` no corpus).
fn plus_variants() -> Vec<SymbolVariant> {
    vec![
        (EcoString::default(), '+'.into()),
        ("o".into(), '⊕'.into()),
        ("o.l".into(), '⨭'.into()),
        ("o.r".into(), '⨮'.into()),
        ("o.arrow".into(), '⟴'.into()),
        ("o.big".into(), '⨁'.into()),
        ("dot".into(), '∔'.into()),
        ("double".into(), '⧺'.into()),
        ("minus".into(), '±'.into()),
        ("square".into(), '⊞'.into()),
        ("triangle".into(), '⨹'.into()),
        ("triple".into(), '⧻'.into()),
        ("hat".into(), '⨣'.into()),
    ]
}

/// P766 — grupo `gt` (uso real: `sym.gt.eq.tri.not` no corpus).
fn gt_variants() -> Vec<SymbolVariant> {
    vec![
        (EcoString::default(), '>'.into()),
        ("o".into(), '⧁'.into()),
        ("dot".into(), '⋗'.into()),
        ("quest".into(), '⩼'.into()),
        ("approx".into(), '⪆'.into()),
        ("arc".into(), '⪧'.into()),
        ("arc.eq".into(), '⪩'.into()),
        ("closed".into(), '⊳'.into()),
        ("closed.eq".into(), '⊵'.into()),
        ("closed.eq.not".into(), '⋭'.into()),
        ("closed.not".into(), '⋫'.into()),
        ("double".into(), '≫'.into()),
        ("double.nested".into(), '⪢'.into()),
        ("eq".into(), '≥'.into()),
        ("eq.slant".into(), '⩾'.into()),
        ("eq.lt".into(), '⋛'.into()),
        ("eq.not".into(), '≱'.into()),
        ("equiv".into(), '≧'.into()),
        ("lt".into(), '≷'.into()),
        ("lt.not".into(), '≹'.into()),
        ("neq".into(), '⪈'.into()),
        ("napprox".into(), '⪊'.into()),
        ("nequiv".into(), '≩'.into()),
        ("not".into(), '≯'.into()),
        ("ntilde".into(), '⋧'.into()),
        ("tilde".into(), '≳'.into()),
        ("tilde.not".into(), '≵'.into()),
        ("tri".into(), '⊳'.into()),
        ("tri.eq".into(), '⊵'.into()),
        ("tri.eq.not".into(), '⋭'.into()),
        ("tri.not".into(), '⋫'.into()),
        ("triple".into(), '⋙'.into()),
        ("triple.nested".into(), '⫸'.into()),
    ]
}

/// P766 — grupo `diamond` (uso real: `sym.diamond.small` no corpus).
fn diamond_variants() -> Vec<SymbolVariant> {
    vec![
        ("blue".into(), '🔷'.into()),
        ("blue.small".into(), '🔹'.into()),
        ("orange".into(), '🔶'.into()),
        ("orange.small".into(), '🔸'.into()),
        ("dot".into(), '💠'.into()),
    ]
}

/// P766 — grupo `tilde` (uso real: 19 ocorrências no corpus).
fn tilde_variants() -> Vec<SymbolVariant> {
    vec![
        (EcoString::default(), '∼'.into()),
        ("op".into(), '∼'.into()),
        ("basic".into(), '~'.into()),
        ("dot".into(), '⩪'.into()),
        ("eq".into(), '≃'.into()),
        ("eq.not".into(), '≄'.into()),
        ("eq.rev".into(), '⋍'.into()),
        ("equiv".into(), '≅'.into()),
        ("equiv.not".into(), '≇'.into()),
        ("nequiv".into(), '≆'.into()),
        ("not".into(), '≁'.into()),
        ("rev".into(), '∽'.into()),
        ("rev.equiv".into(), '≌'.into()),
        ("triple".into(), '≋'.into()),
    ]
}

/// P766 — grupo `integral` (uso real: 3 ocorrências no corpus).
fn integral_variants() -> Vec<SymbolVariant> {
    vec![
        (EcoString::default(), '∫'.into()),
        ("arrow.hook".into(), '⨗'.into()),
        ("ccw".into(), '⨑'.into()),
        ("cont".into(), '∮'.into()),
        ("cont.ccw".into(), '∳'.into()),
        ("cont.cw".into(), '∲'.into()),
        ("cw".into(), '∱'.into()),
        ("dash".into(), '⨍'.into()),
        ("dash.double".into(), '⨎'.into()),
        ("double".into(), '∬'.into()),
        ("quad".into(), '⨌'.into()),
        ("inter".into(), '⨙'.into()),
        ("slash".into(), '⨏'.into()),
        ("square".into(), '⨖'.into()),
        ("surf".into(), '∯'.into()),
        ("times".into(), '⨘'.into()),
        ("triple".into(), '∭'.into()),
        ("union".into(), '⨚'.into()),
        ("vol".into(), '∰'.into()),
    ]
}

/// P766 — grupo `chevron` (uso real: 3 ocorrências no corpus).
fn chevron_variants() -> Vec<SymbolVariant> {
    vec![
        ("l".into(), '⟨'.into()),
        ("l.curly".into(), '⧼'.into()),
        ("l.dot".into(), '⦑'.into()),
        ("l.closed".into(), '⦉'.into()),
        ("l.double".into(), '⟪'.into()),
        ("r".into(), '⟩'.into()),
        ("r.curly".into(), '⧽'.into()),
        ("r.dot".into(), '⦒'.into()),
        ("r.closed".into(), '⦊'.into()),
        ("r.double".into(), '⟫'.into()),
    ]
}

/// P766 — grupo `suit` (uso real: 2 ocorrências no corpus).
fn suit_variants() -> Vec<SymbolVariant> {
    vec![
        ("club".into(), '♣'.into()),
        ("diamond".into(), '♦'.into()),
        ("heart".into(), '♥'.into()),
        ("spade".into(), '♠'.into()),
    ]
}

/// P766 — grupo `tack` (uso real: 1 ocorrência no corpus).
fn tack_variants() -> Vec<SymbolVariant> {
    vec![
        ("r".into(), '⊢'.into()),
        ("r.not".into(), '⊬'.into()),
        ("r.long".into(), '⟝'.into()),
        ("r.short".into(), '⊦'.into()),
        ("r.double".into(), '⊨'.into()),
        ("rr".into(), '⊨'.into()),
        ("r.double.not".into(), '⊭'.into()),
        ("rr.not".into(), '⊭'.into()),
        ("rrr".into(), '⫢'.into()),
        ("l".into(), '⊣'.into()),
        ("l.long".into(), '⟞'.into()),
        ("l.short".into(), '⫞'.into()),
        ("l.double".into(), '⫤'.into()),
        ("ll".into(), '⫤'.into()),
        ("t".into(), '⊥'.into()),
        ("t.big".into(), '⟘'.into()),
        ("t.double".into(), '⫫'.into()),
        ("tt".into(), '⫫'.into()),
        ("t.short".into(), '⫠'.into()),
        ("b".into(), '⊤'.into()),
        ("b.big".into(), '⟙'.into()),
        ("b.double".into(), '⫪'.into()),
        ("bb".into(), '⫪'.into()),
        ("b.short".into(), '⫟'.into()),
        ("l.r".into(), '⟛'.into()),
    ]
}

/// P766 — grupo `space` (uso real: 1 ocorrência no corpus).
fn space_variants() -> Vec<SymbolVariant> {
    vec![
        (EcoString::default(), ' '.into()),
        ("nobreak".into(), '\u{a0}'.into()),
        ("nobreak.narrow".into(), '\u{202f}'.into()),
        ("en".into(), '\u{2002}'.into()),
        ("quad".into(), '\u{2003}'.into()),
        ("third".into(), '\u{2004}'.into()),
        ("quarter".into(), '\u{2005}'.into()),
        ("sixth".into(), '\u{2006}'.into()),
        ("med".into(), '\u{205f}'.into()),
        ("fig".into(), '\u{2007}'.into()),
        ("punct".into(), '\u{2008}'.into()),
        ("thin".into(), '\u{2009}'.into()),
        ("hair".into(), '\u{200a}'.into()),
    ]
}

/// P766 — grupo `emptyset` (uso real: 1 ocorrência no corpus).
fn emptyset_variants() -> Vec<SymbolVariant> {
    vec![
        (EcoString::default(), '∅'.into()),
        ("zero".into(), '∅'.into()),
        ("arrow.r".into(), '⦳'.into()),
        ("arrow.l".into(), '⦴'.into()),
        ("bar".into(), '⦱'.into()),
        ("circle".into(), '⦲'.into()),
        ("rev".into(), '⦰'.into()),
    ]
}

/// P766 — grupo `bracket` (uso real: 1 ocorrência no corpus).
fn bracket_variants() -> Vec<SymbolVariant> {
    vec![
        ("l".into(), '['.into()),
        ("l.tick.t".into(), '⦍'.into()),
        ("l.tick.b".into(), '⦏'.into()),
        ("l.stroked".into(), '⟦'.into()),
        ("r".into(), ']'.into()),
        ("r.tick.t".into(), '⦐'.into()),
        ("r.tick.b".into(), '⦎'.into()),
        ("r.stroked".into(), '⟧'.into()),
        ("t".into(), '⎴'.into()),
        ("b".into(), '⎵'.into()),
    ]
}

/// P766 — grupo `amp` (uso real: 1 ocorrência no corpus).
fn amp_variants() -> Vec<SymbolVariant> {
    vec![(EcoString::default(), '&'.into()), ("inv".into(), '⅋'.into())]
}

fn subset_variants() -> Vec<SymbolVariant> {
    vec![
        (EcoString::default(), '⊂'.into()),
        ("eq".into(), '⊆'.into()),
        ("neq".into(), '⊊'.into()),
    ]
}

fn floor_variants() -> Vec<SymbolVariant> {
    vec![
        (EcoString::default(), '⌊'.into()),
        ("l".into(), '⌊'.into()),
        ("r".into(), '⌋'.into()),
    ]
}

fn ceil_variants() -> Vec<SymbolVariant> {
    vec![
        (EcoString::default(), '⌈'.into()),
        ("l".into(), '⌈'.into()),
        ("r".into(), '⌉'.into()),
    ]
}

fn supset_variants() -> Vec<SymbolVariant> {
    vec![
        (EcoString::default(), '⊃'.into()),
        ("eq".into(), '⊇'.into()),
        ("neq".into(), '⊋'.into()),
    ]
}

/// **P820** — grupo `join` (codex `sym.txt:651-654`). Símbolo **depreciado**
/// no vanilla (ver `SYM_DEPRECATED`); os caracteres continuam resolvíveis.
fn join_variants() -> Vec<SymbolVariant> {
    vec![
        (EcoString::default(), '⨝'.into()),
        ("r".into(), '⟖'.into()),
        ("l".into(), '⟕'.into()),
        ("l.r".into(), '⟗'.into()),
    ]
}

/// **P820** — grupo `bowtie` (codex `sym.txt:655-663`). Sem variante bare
/// no codex: o vanilla cai na primeira variante (`stroked`, ⋈) quando não
/// há modifiers — medido `$bowtie$` → ⋈. `bowtie.big` resolve para
/// `stroked.big` (⨝) pelo algoritmo de menor número de modifiers extra
/// (`Symbol::modified`) — medido `$bowtie.big$` → ⨝.
fn planck_variants() -> Vec<SymbolVariant> {
    vec![(ecow::EcoString::default(), 'ℎ'.into()), ("reduce".into(), 'ℏ'.into())]
}

fn bowtie_variants() -> Vec<SymbolVariant> {
    vec![
        ("stroked".into(), '⋈'.into()),
        ("stroked.big".into(), '⨝'.into()),
        ("stroked.big.l".into(), '⟕'.into()),
        ("stroked.big.r".into(), '⟖'.into()),
        ("stroked.big.l.r".into(), '⟗'.into()),
        ("filled".into(), '⧓'.into()),
        ("filled.l".into(), '⧑'.into()),
        ("filled.r".into(), '⧒'.into()),
    ]
}

// **P895** (Parte B — catálogo de terceiros, `typst-passo-895-relatorio.md`)
// — os grupos abaixo eram entradas `SYM_SIMPLE` planas sem variantes; como
// `$epsilon.alt$` é sempre parseado como `FieldAccess(MathIdent("epsilon"),
// "alt")` (nunca como um único `MathIdent` "epsilon.alt"), a resolução em
// modo math passa por `Value::Symbol::modified("alt")` — que só encontra a
// variante se o símbolo base foi construído via `Symbol::with_variants`
// (`SYM_GROUPS`), nunca `Symbol::new` (`SYM_SIMPLE`). Um par de entradas
// planas "nome"/"nome.modificador" em `SYM_SIMPLE` (o padrão usado por
// `eq.not`/`dot.c`) só é alcançável chamando `sym_lookup` directamente com a
// string já combinada — não a partir de modo math real. Migrados para
// `SYM_GROUPS` para que `$epsilon.alt$` etc. funcionem de facto.

/// **P895** — grupo `epsilon` (paridade `codex`: `.alt` = ϵ, `.alt.rev` = ϶
/// fora de âmbito, não pedido).
fn epsilon_variants() -> Vec<SymbolVariant> {
    vec![(EcoString::default(), 'ε'.into()), ("alt".into(), 'ϵ'.into())]
}

/// **P895** — grupo `theta` (paridade `codex`: `.alt` = ϑ).
fn theta_variants() -> Vec<SymbolVariant> {
    vec![(EcoString::default(), 'θ'.into()), ("alt".into(), 'ϑ'.into())]
}

/// **P895** — grupo `phi` (paridade `codex`: `.alt` = ϕ).
fn phi_variants() -> Vec<SymbolVariant> {
    vec![(EcoString::default(), 'φ'.into()), ("alt".into(), 'ϕ'.into())]
}

/// **P895** — grupo `rho` (paridade `codex`: `.alt` = ϱ).
fn rho_variants() -> Vec<SymbolVariant> {
    vec![(EcoString::default(), 'ρ'.into()), ("alt".into(), 'ϱ'.into())]
}

/// **P895** — grupo `sigma` (paridade `codex`: `.alt` = ς).
fn sigma_variants() -> Vec<SymbolVariant> {
    vec![(EcoString::default(), 'σ'.into()), ("alt".into(), 'ς'.into())]
}

/// **P895** — grupo `dots` (só `.h` pedido/testado; `.h.c`/`.v`/`.down`/
/// `.up` do codex ficam fora de âmbito, não pedidos).
fn dots_variants() -> Vec<SymbolVariant> {
    vec![
        (EcoString::default(), '…'.into()),
        ("h".into(), '…'.into()),
        ("c".into(), '⋯'.into()),
        ("v".into(), '⋮'.into()),
        ("down".into(), '⋱'.into()),
        ("up".into(), '⋰'.into()),
    ]
}

/// **P895** — grupo `union` (só `.big` pedido/testado; outras variantes do
/// codex — `.serif`/`.arrow`/`.dot`/`.dot.big`/`.double` — ficam fora de
/// âmbito, não pedidas).
fn union_variants() -> Vec<SymbolVariant> {
    vec![(EcoString::default(), '∪'.into()), ("big".into(), '⋃'.into())]
}

/// **P895** — grupo `inter` (nome correcto — `sect` não existe no vanilla,
/// corrigido). Só `.big` pedido/testado; outras variantes do codex ficam
/// fora de âmbito.
fn inter_variants() -> Vec<SymbolVariant> {
    vec![(EcoString::default(), '∩'.into()), ("big".into(), '⋂'.into())]
}

/// **P820** (achado #7 de P810) — símbolos **depreciados** de topo,
/// nome → mensagem verbatim do vanilla. Fonte de dados: tag `@deprecated`
/// do codex `sym.txt` (vanilla 0.15.0). O vanilla tem 14 entradas
/// `@deprecated`, mas 13 são ao nível de **variante** (`gt.tri*`, `lt.tri*`,
/// `tack.*.double`) — ficam em scope-out explícito (requerem mensagem por
/// variante em `SymbolVariant`, mecanismo separado); apenas `join` é
/// depreciação de símbolo de topo, e é o caso medido em P810/P820.
static SYM_DEPRECATED: &[(&str, &str)] =
    &[("join", "`join` is deprecated, use `bowtie.big` instead")];

/// Mensagem de depreciação de um símbolo de topo, se depreciado.
/// Os call sites (eval math, field access) emitem o warning com o span
/// apropriado (ident em math; campo em `#sym.join`).
pub fn sym_deprecation(name: &str) -> Option<&'static str> {
    SYM_DEPRECATED.iter().find(|(n, _)| *n == name).map(|(_, msg)| *msg)
}

/// Lista de grupos com variantes: (nome, caractere base, função de variantes).
pub(crate) static SYM_GROUPS: &[(&str, &str, fn() -> Vec<SymbolVariant>)] = &[
    ("arrow", "→", arrow_variants),
    ("plus", "+", plus_variants),
    ("gt", ">", gt_variants),
    ("diamond", "◇", diamond_variants),
    ("tilde", "∼", tilde_variants),
    ("integral", "∫", integral_variants),
    ("chevron", "⟨", chevron_variants),
    ("suit", "♣", suit_variants),
    ("tack", "⊢", tack_variants),
    ("space", " ", space_variants),
    ("emptyset", "∅", emptyset_variants),
    ("bracket", "[", bracket_variants),
    ("amp", "&", amp_variants),
    ("subset", "⊂", subset_variants),
    ("supset", "⊃", supset_variants),
    ("join", "⨝", join_variants),
    ("bowtie", "⋈", bowtie_variants),
    ("epsilon", "ε", epsilon_variants),
    ("theta", "θ", theta_variants),
    ("phi", "φ", phi_variants),
    ("rho", "ρ", rho_variants),
    ("sigma", "σ", sigma_variants),
    ("dots", "…", dots_variants),
    ("union", "∪", union_variants),
    ("inter", "∩", inter_variants),
    ("planck", "ℎ", planck_variants),
    ("floor", "⌊", floor_variants),
    ("ceil", "⌈", ceil_variants),
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
        scope.define(
            *group,
            Value::Symbol(Symbol::with_variants(*base, *group, variants_fn())),
        );
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
        assert_eq!(s.value, "α");
        assert_eq!(s.name.as_str(), "alpha");
    }

    #[test]
    fn sym_lookup_composto_predefinido() {
        let s = sym_lookup("eq.not").unwrap();
        assert_eq!(s.value, "≠");
    }

    #[test]
    fn sym_lookup_arrow_modifier() {
        let s = sym_lookup("arrow.r.filled").unwrap();
        assert_eq!(s.value, "➡");
    }

    #[test]
    fn sym_lookup_tilde_modifier() {
        let s = sym_lookup("tilde.equiv").unwrap();
        assert_eq!(s.value, "≅");
    }

    #[test]
    fn sym_lookup_integral_modifier() {
        let s = sym_lookup("integral.double").unwrap();
        assert_eq!(s.value, "∬");
    }

    #[test]
    fn sym_lookup_chevron_modifier() {
        let s = sym_lookup("chevron.l").unwrap();
        assert_eq!(s.value, "⟨");
    }

    #[test]
    fn sym_lookup_suit_modifier() {
        let s = sym_lookup("suit.heart").unwrap();
        assert_eq!(s.value, "♥");
    }

    #[test]
    fn sym_lookup_tack_modifier() {
        let s = sym_lookup("tack.r.double").unwrap();
        assert_eq!(s.value, "⊨");
    }

    /// **P894** — `dot` bare deve ser U+22C5 (DOT OPERATOR, paridade vanilla
    /// `codex` `dot.op`, o valor por omissão do grupo), não U+00B7 (MIDDLE
    /// DOT) — esse é o valor de `dot.c`, uma variante distinta.
    #[test]
    fn sym_lookup_dot_bare_e_dot_operator() {
        let s = sym_lookup("dot").unwrap();
        assert_eq!(s.value, "⋅", "dot bare deve ser U+22C5, não U+00B7");
    }

    #[test]
    fn sym_lookup_dot_c_e_middle_dot() {
        let s = sym_lookup("dot.c").unwrap();
        assert_eq!(s.value, "·", "dot.c preserva o U+00B7 anteriormente em dot bare");
    }

    // **P895** (Parte B — catálogo de terceiros, `typst-passo-895-relatorio.md`)
    // — variantes `.alt` de letras gregas, em falta.
    #[test]
    fn sym_lookup_epsilon_alt() {
        assert_eq!(sym_lookup("epsilon.alt").unwrap().value, "ϵ");
    }
    #[test]
    fn sym_lookup_theta_alt() {
        assert_eq!(sym_lookup("theta.alt").unwrap().value, "ϑ");
    }
    #[test]
    fn sym_lookup_phi_alt() {
        assert_eq!(sym_lookup("phi.alt").unwrap().value, "ϕ");
    }
    #[test]
    fn sym_lookup_rho_alt() {
        assert_eq!(sym_lookup("rho.alt").unwrap().value, "ϱ");
    }
    #[test]
    fn sym_lookup_sigma_alt() {
        assert_eq!(sym_lookup("sigma.alt").unwrap().value, "ς");
    }

    #[test]
    fn sym_lookup_dots_h() {
        assert_eq!(sym_lookup("dots.h").unwrap().value, "…");
    }

    #[test]
    fn sym_lookup_union_big() {
        assert_eq!(sym_lookup("union.big").unwrap().value, "⋃");
    }

    /// `inter` (não `sect`, nome inexistente no vanilla — corrigido).
    #[test]
    fn sym_lookup_inter_bare() {
        assert_eq!(sym_lookup("inter").unwrap().value, "∩");
    }
    #[test]
    fn sym_lookup_inter_big() {
        assert_eq!(sym_lookup("inter.big").unwrap().value, "⋂");
    }

    #[test]
    fn sym_lookup_oo() {
        assert_eq!(sym_lookup("oo").unwrap().value, "∞");
    }
    #[test]
    fn sym_lookup_beth() {
        assert_eq!(sym_lookup("beth").unwrap().value, "ב");
    }
    #[test]
    fn sym_lookup_prop() {
        assert_eq!(sym_lookup("prop").unwrap().value, "∝");
    }

    #[test]
    fn sym_lookup_space_modifier() {
        let s = sym_lookup("space.nobreak").unwrap();
        assert_eq!(s.value, "\u{a0}");
    }

    #[test]
    fn sym_lookup_emptyset_modifier() {
        let s = sym_lookup("emptyset.rev").unwrap();
        assert_eq!(s.value, "⦰");
    }

    #[test]
    fn sym_lookup_bracket_modifier() {
        let s = sym_lookup("bracket.l.stroked").unwrap();
        assert_eq!(s.value, "⟦");
    }

    #[test]
    fn sym_lookup_amp_modifier() {
        let s = sym_lookup("amp.inv").unwrap();
        assert_eq!(s.value, "⅋");
    }

    #[test]
    fn sym_lookup_plus_o() {
        let s = sym_lookup("plus.o").unwrap();
        assert_eq!(s.value, "⊕");
    }

    #[test]
    fn sym_lookup_gt_eq_tri_not() {
        let s = sym_lookup("gt.eq.tri.not").unwrap();
        assert_eq!(s.value, "⋭");
    }

    #[test]
    fn sym_lookup_diamond_small() {
        let s = sym_lookup("diamond.small").unwrap();
        assert_eq!(s.value, "🔹");
    }

    #[test]
    fn sym_lookup_inexistente() {
        assert!(sym_lookup("inexistente").is_none());
    }

    // ── P820 — `join`/`bowtie` + mecanismo de depreciação ───────────────

    #[test]
    fn p820_sym_lookup_join_base() {
        let s = sym_lookup("join").unwrap();
        assert_eq!(s.value, "⨝");
    }

    #[test]
    fn p820_sym_lookup_join_variante_r() {
        let s = sym_lookup("join.r").unwrap();
        assert_eq!(s.value, "⟖");
    }

    #[test]
    fn p820_sym_lookup_bowtie_base() {
        // Sem caractere bare no codex: o vanilla cai na primeira variante
        // (`stroked`, ⋈) — medido `$bowtie$` → ⋈.
        let s = sym_lookup("bowtie").unwrap();
        assert_eq!(s.value, "⋈");
    }

    #[test]
    fn p820_sym_lookup_bowtie_big() {
        // Medido no vanilla: `$bowtie.big$` → ⨝ (variante `stroked.big`).
        let s = sym_lookup("bowtie.big").unwrap();
        assert_eq!(s.value, "⨝");
    }

    #[test]
    fn p820_sym_lookup_bowtie_filled() {
        let s = sym_lookup("bowtie.filled").unwrap();
        assert_eq!(s.value, "⧓");
    }

    #[test]
    fn p820_sym_deprecation_join() {
        assert_eq!(
            sym_deprecation("join"),
            Some("`join` is deprecated, use `bowtie.big` instead")
        );
    }

    #[test]
    fn p820_sym_deprecation_nao_depreciado_none() {
        assert_eq!(sym_deprecation("alpha"), None);
        assert_eq!(sym_deprecation("bowtie"), None);
        assert_eq!(sym_deprecation("inexistente"), None);
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
                assert_eq!(s.value, "→");
            } else {
                panic!("esperado Value::Symbol");
            }
        } else {
            panic!("esperado Value::Module");
        }
    }
}
