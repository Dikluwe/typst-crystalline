//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/engine/math/layout/spacing.md
//! @prompt-hash 92542790
//! @layer L1
//! @updated 2026-07-17
//!
//! Espaçamento automático inter-símbolo por `MathClass` — P772y. Paridade
//! `math/ir/process.rs::spacing()` + `math/mod.rs` (THIN/MEDIUM/THICK)
//! (vanilla). Fora de escopo (registado, não silencioso): a condição
//! "unless in script size" de cada regra vanilla (cristalino não tem um
//! `MathSize` discreto — só um `script_percent_scale_down` contínuo
//! aplicado ad-hoc em `attach.rs`/`frac.rs`/`root.rs`) e a regra "spaced
//! frames" (`#h()` explícito dentro de math, que no vanilla devolve o
//! espaço textual em vez da tabela de classes).

use crate::entities::content::Content;
use crate::entities::math_class::{default_math_class, MathClass};

/// Em units — paridade `math/mod.rs` (vanilla).
const THIN: f64 = 1.0 / 6.0;
const MEDIUM: f64 = 2.0 / 9.0;
const THICK: f64 = 5.0 / 18.0;

/// Classe base de um nó `Content` matemático, antes da assimetria de
/// delimitadores e da promoção Vary→Binary. Composite nodes (frac, attach,
/// root, matrix, cases, accent, cancel, underover, op) usam `Normal` —
/// paridade com o vanilla, que não define `class` explícito nesses
/// elementos (fallback `unwrap_or(MathClass::Normal)`).
fn base_math_class(content: &Content) -> MathClass {
    match content {
        Content::MathIdent(name) => name
            .chars()
            .next()
            .and_then(default_math_class)
            .unwrap_or(MathClass::Alphabetic),
        Content::MathText(text) => text
            .chars()
            .next()
            .and_then(default_math_class)
            .unwrap_or(MathClass::Normal),
        Content::MathStyled(m) => base_math_class(&m.body),
        Content::MathClassOverride(e) => e.class,
        _ => MathClass::Normal,
    }
}

/// `(lclass, rclass)` efectivos de um nó, para efeitos de espaçamento.
/// Paridade `MathItem::lclass()/rclass()` (vanilla `math/ir/item.rs`):
/// idênticos a `.class()` excepto em `MathDelimited`, onde a assimetria
/// abertura/fecho aplica sempre (cristalino não tem o caso `Fenced`
/// opcional do vanilla — os dois delimitadores estão sempre presentes).
pub(super) fn node_math_class(content: &Content) -> (MathClass, MathClass) {
    match content {
        Content::MathDelimited(_) => (MathClass::Opening, MathClass::Closing),
        other => {
            let class = base_math_class(other);
            (class, class)
        }
    }
}

/// Promove `MathClass::Vary` para `MathClass::Binary` quando precedido por
/// um item cuja `rclass` é `Normal | Alphabetic | Closing | Fence` — uso
/// como operador binário (`a+b`) em vez de prefixo unário (`-x`). Paridade
/// `process.rs` (vanilla), condição sobre `prev.class()`.
pub(super) fn promote_vary(
    class: MathClass,
    prev_rclass: Option<MathClass>,
) -> MathClass {
    if class == MathClass::Vary
        && matches!(
            prev_rclass,
            Some(
                MathClass::Normal
                    | MathClass::Alphabetic
                    | MathClass::Closing
                    | MathClass::Fence
            )
        )
    {
        MathClass::Binary
    } else {
        class
    }
}

/// Espaço extra (pt) entre dois nós adjacentes, dado o `rclass` efectivo
/// do nó à esquerda e o `lclass` efectivo do nó à direita. Paridade
/// `math/ir/process.rs::spacing()` (vanilla) — ordem dos ramos é
/// significativa (primeiro match ganha, tal como o vanilla).
pub(super) fn spacing_between(
    l_rclass: MathClass,
    r_lclass: MathClass,
    size_pt: f64,
) -> f64 {
    use MathClass::*;
    match (l_rclass, r_lclass) {
        // Sem espaço antes de pontuação; thin depois de pontuação.
        (_, Punctuation) => 0.0,
        (Punctuation, _) => THIN * size_pt,

        // Sem espaço depois de abertura / antes de fecho.
        (Opening, _) | (_, Closing) => 0.0,

        // Thick à volta de relações, excepto entre duas relações seguidas.
        (Relation, Relation) => 0.0,
        (Relation, _) => THICK * size_pt,
        (_, Relation) => THICK * size_pt,

        // Medium à volta de operadores binários.
        (Binary, _) => MEDIUM * size_pt,
        (_, Binary) => MEDIUM * size_pt,

        // Thin à volta de operadores grandes, excepto antes de abertura/fence.
        (Large, Opening) | (Large, Fence) => 0.0,
        (Large, _) => THIN * size_pt,
        (_, Large) => THIN * size_pt,

        _ => 0.0,
    }
}

/// Calcula os `n - 1` espaços entre `n` nós adjacentes de uma sequência
/// matemática, aplicando promoção Vary→Binary sequencialmente (rclass do
/// nó anterior, já promovido, alimenta a decisão do nó seguinte).
///
/// `in_script` (**P891**) — `true` quando toda a sequência está dentro de
/// um script (sub/super-índice) de `MathAttach` (`TextStyle::math_script`).
/// Paridade `process.rs::spacing()` vanilla, condição "unless in script
/// size": quando verdadeiro, nenhuma regra de `spacing_between` é aplicada
/// — todos os gaps são 0 (suprime por completo, não reduz).
pub(super) fn compute_gaps(nodes: &[Content], size_pt: f64, in_script: bool) -> Vec<f64> {
    if in_script {
        return vec![0.0; nodes.len().saturating_sub(1)];
    }

    let mut gaps = Vec::with_capacity(nodes.len().saturating_sub(1));
    let mut prev_rclass: Option<MathClass> = None;

    for node in nodes {
        let (raw_l, raw_r) = node_math_class(node);
        let l = promote_vary(raw_l, prev_rclass);
        // Item não-fenced (a esmagadora maioria): lclass == rclass == class,
        // logo a promoção do lclass aplica-se igualmente ao rclass efectivo.
        // `MathDelimited` é sempre assimétrico (raw_l != raw_r) e nunca é
        // `Vary`, portanto nunca é promovido — `r` fica com o raw original.
        let r = if raw_l == raw_r { l } else { raw_r };

        if let Some(pr) = prev_rclass {
            gaps.push(spacing_between(pr, l, size_pt));
        }
        prev_rclass = Some(r);
    }

    gaps
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ident(s: &str) -> Content {
        Content::MathIdent(s.into())
    }
    fn text(s: &str) -> Content {
        Content::MathText(s.into())
    }

    // ── node_math_class ──────────────────────────────────────────────

    #[test]
    fn ident_letra_e_alphabetic() {
        assert_eq!(
            node_math_class(&ident("a")),
            (MathClass::Alphabetic, MathClass::Alphabetic)
        );
    }

    #[test]
    fn text_igual_e_relation() {
        assert_eq!(
            node_math_class(&text("=")),
            (MathClass::Relation, MathClass::Relation)
        );
    }

    #[test]
    fn text_mais_e_vary() {
        assert_eq!(node_math_class(&text("+")), (MathClass::Vary, MathClass::Vary));
    }

    #[test]
    fn text_virgula_e_punctuation() {
        assert_eq!(
            node_math_class(&text(",")),
            (MathClass::Punctuation, MathClass::Punctuation)
        );
    }

    #[test]
    fn class_override_forca_classe() {
        let c = Content::math_class_override(MathClass::Relation, text("♥"));
        assert_eq!(node_math_class(&c), (MathClass::Relation, MathClass::Relation));
    }

    #[test]
    fn frac_default_normal() {
        let c = Content::math_frac(ident("a"), ident("b"));
        assert_eq!(node_math_class(&c), (MathClass::Normal, MathClass::Normal));
    }

    // ── promote_vary ─────────────────────────────────────────────────

    #[test]
    fn vary_promovido_apos_alphabetic() {
        assert_eq!(
            promote_vary(MathClass::Vary, Some(MathClass::Alphabetic)),
            MathClass::Binary
        );
    }

    #[test]
    fn vary_promovido_apos_closing() {
        assert_eq!(
            promote_vary(MathClass::Vary, Some(MathClass::Closing)),
            MathClass::Binary
        );
    }

    #[test]
    fn vary_nao_promovido_no_inicio() {
        assert_eq!(promote_vary(MathClass::Vary, None), MathClass::Vary);
    }

    #[test]
    fn vary_nao_promovido_apos_relation() {
        assert_eq!(
            promote_vary(MathClass::Vary, Some(MathClass::Relation)),
            MathClass::Vary
        );
    }

    #[test]
    fn classe_nao_vary_e_ignorada() {
        assert_eq!(
            promote_vary(MathClass::Relation, Some(MathClass::Alphabetic)),
            MathClass::Relation
        );
    }

    // ── spacing_between ──────────────────────────────────────────────

    #[test]
    fn relation_e_thick() {
        assert_eq!(
            spacing_between(MathClass::Relation, MathClass::Alphabetic, 10.0),
            THICK * 10.0
        );
    }

    #[test]
    fn binary_e_medium() {
        assert_eq!(
            spacing_between(MathClass::Alphabetic, MathClass::Binary, 10.0),
            MEDIUM * 10.0
        );
    }

    #[test]
    fn opening_e_zero() {
        assert_eq!(spacing_between(MathClass::Opening, MathClass::Alphabetic, 10.0), 0.0);
    }

    #[test]
    fn closing_e_zero() {
        assert_eq!(spacing_between(MathClass::Alphabetic, MathClass::Closing, 10.0), 0.0);
    }

    #[test]
    fn duas_relacoes_seguidas_e_zero() {
        assert_eq!(spacing_between(MathClass::Relation, MathClass::Relation, 10.0), 0.0);
    }

    #[test]
    fn punctuation_thin_a_seguir() {
        assert_eq!(
            spacing_between(MathClass::Punctuation, MathClass::Alphabetic, 10.0),
            THIN * 10.0
        );
    }

    #[test]
    fn antes_de_punctuation_e_zero() {
        assert_eq!(
            spacing_between(MathClass::Alphabetic, MathClass::Punctuation, 10.0),
            0.0
        );
    }

    #[test]
    fn normal_normal_e_zero() {
        assert_eq!(spacing_between(MathClass::Normal, MathClass::Normal, 10.0), 0.0);
    }

    #[test]
    fn relation_domina_thick_maior_que_medium() {
        assert!(THICK > MEDIUM);
        assert!(MEDIUM > THIN);
    }

    // ── compute_gaps ─────────────────────────────────────────────────

    #[test]
    fn a_igual_b_produz_thick_dos_dois_lados() {
        let nodes = vec![ident("a"), text("="), ident("b")];
        let gaps = compute_gaps(&nodes, 10.0, false);
        assert_eq!(gaps, vec![THICK * 10.0, THICK * 10.0]);
    }

    #[test]
    fn a_mais_b_promove_vary_e_produz_medium() {
        let nodes = vec![ident("a"), text("+"), ident("b")];
        let gaps = compute_gaps(&nodes, 10.0, false);
        assert_eq!(gaps, vec![MEDIUM * 10.0, MEDIUM * 10.0]);
    }

    #[test]
    fn mais_no_inicio_nao_promove_fica_zero() {
        // `+b` (unário): sem item anterior, Vary não promove → sem regra
        // aplicável → 0.
        let nodes = vec![text("+"), ident("b")];
        let gaps = compute_gaps(&nodes, 10.0, false);
        assert_eq!(gaps, vec![0.0]);
    }

    #[test]
    fn abre_e_fecha_parenteses_produz_zero() {
        let c = Content::math_delimited('(', ident("a"), ')');
        let nodes = vec![c];
        let gaps = compute_gaps(&nodes, 10.0, false);
        assert!(gaps.is_empty());
    }

    #[test]
    fn no_unico_nao_produz_gaps() {
        let nodes = vec![ident("a")];
        assert!(compute_gaps(&nodes, 10.0, false).is_empty());
    }

    #[test]
    fn zero_nos_nao_produz_gaps() {
        let nodes: Vec<Content> = vec![];
        assert!(compute_gaps(&nodes, 10.0, false).is_empty());
    }

    // ── P891: in_script suprime todas as regras ──────────────────────

    #[test]
    fn p891_in_script_suprime_thick_de_relation() {
        // `i=0` dentro de script: sem in_script, `=` (Relation) produziria
        // THICK dos dois lados (par Alphabetic/Relation e Relation/Normal,
        // paridade `a_igual_b_produz_thick_dos_dois_lados`). Em script size,
        // vanilla suprime por completo (`process.rs::spacing`, condição
        // "unless in script size").
        let nodes = vec![ident("i"), text("="), text("0")];
        let gaps = compute_gaps(&nodes, 10.0, true);
        assert_eq!(gaps, vec![0.0, 0.0]);
    }

    #[test]
    fn p891_in_script_suprime_medium_de_binary() {
        let nodes = vec![ident("a"), text("+"), ident("b")];
        let gaps = compute_gaps(&nodes, 10.0, true);
        assert_eq!(gaps, vec![0.0, 0.0]);
    }

    #[test]
    fn p891_fora_de_script_continua_normal() {
        // Confirma que a mudança não afecta o comportamento pré-existente
        // quando in_script é false (regressão contra o caso base já testado).
        let nodes = vec![ident("a"), text("="), ident("b")];
        let gaps = compute_gaps(&nodes, 10.0, false);
        assert_eq!(gaps, vec![THICK * 10.0, THICK * 10.0]);
    }
}
