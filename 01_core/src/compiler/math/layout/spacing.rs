//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/math/layout/spacing.md
//! @prompt-hash 45e730ed
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
        // **P992** — `limits()`/`scripts()` não afectam classe/espaçamento
        // (transparente, ao contrário de `MathClassOverride`).
        Content::MathLimitsOverride(e) => base_math_class(&e.body),
        // **P907 Parte B** — `Content::MathOp` (`min`/`max`/`lim`/`sin`/etc,
        // via `make_math_module`) é sempre `MathClass::Large` no vanilla
        // (`math/ir/resolve.rs::resolve_op`, `item.set_class(MathClass::
        // Large)` incondicional — a flag `limits` não afecta a classe, só
        // `Limits::Display`/`Never`). Sem este braço, caía no catch-all
        // `Normal` e a regra "(Large, _) => THIN" nunca disparava —
        // `min f(x)` sem espaço (achado de P903, `spacing.md`).
        Content::MathOp(_) => MathClass::Large,
        // **P907 Parte B** — `Content::MathAttach` herda a classe do seu
        // `base` (paridade `ScriptsItem::create`, vanilla, doc explícita
        // "The resulting item inherits its math class from the base") —
        // necessário para `min_(x)` (MathOp com subscrito) continuar
        // `Large`, não `Normal`. Recursivo: cobre também `x^2` (herda
        // Alphabetic de `x`) e `sum_(i=1)^n` (herda Large de `∑`).
        Content::MathAttach(e) => base_math_class(&e.base),
        // **P903** — texto literal entre aspas (`"texto"` bare em modo math,
        // produzido por `value_to_display_content(Value::Str)` →
        // `Content::Text`, distinto de `Content::MathText`) tem classe
        // `Alphabetic` no vanilla (`math/ir/item.rs::TextItem::create`,
        // comentário "The resulting item is spaced and has alphabetic math
        // class") — não `Normal` (o que caía aqui antes, via `_`). A classe
        // sozinha não basta para o espaçamento (ver `compute_gaps`, achado
        // do "spaced" flag do vanilla), mas é necessária para as regras que
        // já existem (Punctuation/Relation/Binary/Large) tratarem texto
        // literal correctamente quando adjacente a esses.
        Content::Text(_) => MathClass::Alphabetic,
        // intencional: composite nodes (frac, root, matrix, cases, accent, cancel, underover) e elementos fora de math usam MathClass::Normal per vanilla
        Content::Empty
        | Content::Space
        | Content::Parbreak
        | Content::Sequence(_)
        | Content::Par { .. }
        | Content::Heading(_)
        | Content::Title(_)
        | Content::Strong(_)
        | Content::Emph(_)
        | Content::Raw(_)
        | Content::ListItem(_)
        | Content::EnumItem(_)
        | Content::Link(_)
        | Content::Equation(_)
        | Content::MathSequence(_)
        | Content::MathFrac(_)
        | Content::MathRoot(_)
        | Content::MathDelimited(_)
        | Content::MathAlignPoint(_)
        | Content::Linebreak(_)
        | Content::MathMatrix(_)
        | Content::MathCases(_)
        | Content::MathAccent(_)
        | Content::MathCancel(_)
        | Content::MathUnderover(_)
        | Content::Label(_)
        | Content::Ref(_)
        | Content::CounterDisplay(_)
        | Content::CounterUpdate(_)
        | Content::Outline(_)
        | Content::Figure(_)
        | Content::Image(_)
        | Content::Shape(_)
        | Content::Curve(_)
        | Content::Transform(_)
        | Content::Grid(_)
        | Content::GridHeader(_)
        | Content::GridFooter(_)
        | Content::GridCell(_)
        | Content::SetPage { .. }
        | Content::Align(_)
        | Content::Place(_)
        | Content::Styled(_, _)
        | Content::Divider(_)
        | Content::Terms(_)
        | Content::TermItem(_)
        | Content::Quote(_)
        | Content::Document { .. }
        | Content::Asset { .. }
        | Content::SmartQuote(_)
        | Content::Underline(_)
        | Content::Strike(_)
        | Content::Overline(_)
        | Content::SmallCaps { .. }
        | Content::Pad(_)
        | Content::Hide(_)
        | Content::HSpace(_)
        | Content::VSpace(_)
        | Content::Pagebreak(_)
        | Content::Colbreak(_)
        | Content::Stack(_)
        | Content::Boxed(_)
        | Content::Block(_)
        | Content::TableCell(_)
        | Content::Bibliography(_)
        | Content::Cite(_)
        | Content::Footnote(_)
        | Content::TableHeader(_)
        | Content::TableFooter(_)
        | Content::GridHLine(_)
        | Content::GridVLine(_)
        | Content::TableHLine(_)
        | Content::TableVLine(_)
        | Content::Table(_)
        | Content::Repeat(_)
        | Content::Columns(_)
        | Content::Metadata(_)
        | Content::State(_)
        | Content::StateUpdate(_)
        | Content::StateDisplay(_)
        | Content::CounterDisplayCallback(_)
        | Content::ContextBlock(_)
        | Content::Dynamic(_) => MathClass::Normal,
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

/// Espaço extra (pt) entre dois nós adjacentes, dado o `rclass` efectivo do
/// nó à esquerda e o `lclass` efectivo do nó à direita, ou `None` se
/// nenhuma regra explícita de classe se aplica (caiu no catch-all — **P903**:
/// distinto de "aplica-se uma regra e o resultado é 0.0", usado por
/// `compute_gaps` para decidir se o fallback de "item espaçado" do vanilla
/// (`is_spaced()`, ver comentário em `compute_gaps`) ainda pode intervir).
/// Paridade `math/ir/process.rs::spacing()` (vanilla) — ordem dos ramos é
/// significativa (primeiro match ganha, tal como o vanilla).
pub(super) fn spacing_between_class(
    l_rclass: MathClass,
    r_lclass: MathClass,
    size_pt: f64,
) -> Option<f64> {
    use MathClass::*;
    match (l_rclass, r_lclass) {
        // Sem espaço antes de pontuação; thin depois de pontuação.
        (_, Punctuation) => Some(0.0),
        (Punctuation, _) => Some(THIN * size_pt),

        // Sem espaço depois de abertura / antes de fecho.
        (Opening, _) | (_, Closing) => Some(0.0),

        // Thick à volta de relações, excepto entre duas relações seguidas.
        (Relation, Relation) => Some(0.0),
        (Relation, _) => Some(THICK * size_pt),
        (_, Relation) => Some(THICK * size_pt),

        // Medium à volta de operadores binários.
        (Binary, _) => Some(MEDIUM * size_pt),
        (_, Binary) => Some(MEDIUM * size_pt),

        // Thin à volta de operadores grandes, excepto antes de abertura/fence.
        (Large, Opening) | (Large, Fence) => Some(0.0),
        (Large, _) => Some(THIN * size_pt),
        (_, Large) => Some(THIN * size_pt),

        _ => None,
    }
}

/// Espaço extra (pt) entre dois nós adjacentes — versão pública que
/// preserva a assinatura/comportamento pré-P903 (catch-all vira `0.0`).
/// Usada onde só a classe importa, não se uma regra explícita de facto
/// disparou (ver `spacing_between_class`, usada directamente por
/// `compute_gaps` para o fallback de item espaçado).
pub(super) fn spacing_between(
    l_rclass: MathClass,
    r_lclass: MathClass,
    size_pt: f64,
) -> f64 {
    spacing_between_class(l_rclass, r_lclass, size_pt).unwrap_or(0.0)
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
///
/// `text_space_pt` (**P903**) — largura (pt) de um espaço de texto normal
/// no estilo/tamanho actual (medida via `FontMetrics::advance(" ", ...)`
/// pelo caller, `layout_sequence`; não um valor hardcoded). Paridade
/// vanilla (`math/ir/item.rs::TextItem::create`, `.with_spaced(true)` +
/// `process.rs::spacing()`, ramo `_ if (l.is_spaced() || r.is_spaced()) =>
/// return space`): texto literal entre aspas (`Content::Text`) é um "item
/// espaçado" — quando NENHUMA regra explícita de classe se aplica (`(pr, l)`
/// cai no catch-all de `spacing_between_class`, não apenas "resulta em
/// 0.0" — a distinção importa: pontuação/abertura/fecho têm regras
/// explícitas que **querem** 0.0 e continuam a ganhar, tal como no
/// vanilla, onde a prioridade do match decide antes do `is_spaced()` ser
/// sequer consultado) e um dos dois lados é `Content::Text`, usa-se
/// `text_space_pt` em vez de 0.0.
pub(super) fn compute_gaps(
    nodes: &[Content],
    size_pt: f64,
    in_script: bool,
    text_space_pt: f64,
) -> Vec<f64> {
    if in_script {
        return vec![0.0; nodes.len().saturating_sub(1)];
    }

    let mut gaps = Vec::with_capacity(nodes.len().saturating_sub(1));
    let mut prev_rclass: Option<MathClass> = None;
    let mut prev_is_spaced = false;

    for node in nodes {
        let (raw_l, raw_r) = node_math_class(node);
        let l = promote_vary(raw_l, prev_rclass);
        // Item não-fenced (a esmagadora maioria): lclass == rclass == class,
        // logo a promoção do lclass aplica-se igualmente ao rclass efectivo.
        // `MathDelimited` é sempre assimétrico (raw_l != raw_r) e nunca é
        // `Vary`, portanto nunca é promovido — `r` fica com o raw original.
        let r = if raw_l == raw_r { l } else { raw_r };
        // **P907 Parte A** — `is_spaced()` do vanilla (`math/ir/item.rs`):
        // `class() == Fence` é SEMPRE "spaced", incondicionalmente (`|` como
        // fence/"mid"/"tal que" — não o delimitador de `abs(x)`, que é
        // `MathDelimited`/Opening+Closing, já coberto por regra explícita
        // acima e não afectado). `Content::Text` continua o outro gatilho
        // (P903). Achado registado em P825, reconfirmado em P903.
        let is_spaced = matches!(node, Content::Text(_)) || raw_l == MathClass::Fence;

        if let Some(pr) = prev_rclass {
            let gap = match spacing_between_class(pr, l, size_pt) {
                Some(v) => v,
                None if prev_is_spaced || is_spaced => text_space_pt,
                None => 0.0,
            };
            gaps.push(gap);
        }
        prev_rclass = Some(r);
        prev_is_spaced = is_spaced;
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
        let gaps = compute_gaps(&nodes, 10.0, false, 0.0);
        assert_eq!(gaps, vec![THICK * 10.0, THICK * 10.0]);
    }

    #[test]
    fn a_mais_b_promove_vary_e_produz_medium() {
        let nodes = vec![ident("a"), text("+"), ident("b")];
        let gaps = compute_gaps(&nodes, 10.0, false, 0.0);
        assert_eq!(gaps, vec![MEDIUM * 10.0, MEDIUM * 10.0]);
    }

    #[test]
    fn mais_no_inicio_nao_promove_fica_zero() {
        // `+b` (unário): sem item anterior, Vary não promove → sem regra
        // aplicável → 0.
        let nodes = vec![text("+"), ident("b")];
        let gaps = compute_gaps(&nodes, 10.0, false, 0.0);
        assert_eq!(gaps, vec![0.0]);
    }

    #[test]
    fn abre_e_fecha_parenteses_produz_zero() {
        let c = Content::math_delimited('(', ident("a"), ')');
        let nodes = vec![c];
        let gaps = compute_gaps(&nodes, 10.0, false, 0.0);
        assert!(gaps.is_empty());
    }

    #[test]
    fn no_unico_nao_produz_gaps() {
        let nodes = vec![ident("a")];
        assert!(compute_gaps(&nodes, 10.0, false, 0.0).is_empty());
    }

    #[test]
    fn zero_nos_nao_produz_gaps() {
        let nodes: Vec<Content> = vec![];
        assert!(compute_gaps(&nodes, 10.0, false, 0.0).is_empty());
    }

    // ── P903: texto literal entre aspas recebe espaço dos dois lados ──
    //
    // Achado catalogado em P897 (secção "fora de escopo"), confirmado
    // isoladamente aqui: `$ a "texto" b $` não tinha espaço nenhum à volta
    // de "texto" — `Content::Text` caía em `MathClass::Normal` via `_` de
    // `base_math_class`, e o par (Alphabetic, Normal) não tem regra
    // explícita em `spacing_between_class`, caindo no catch-all `None`
    // (0.0 antes de P903). Paridade vanilla: `TextItem::create` marca o
    // item como "spaced" — `process.rs::spacing()` usa isso como fallback
    // de última prioridade quando nenhuma regra de classe (Punctuation/
    // Opening/Closing/Relation/Binary/Large) se aplica.

    fn literal_text(s: &str) -> Content {
        Content::Text(s.into())
    }

    #[test]
    fn texto_literal_entre_identificadores_recebe_espaco_dos_dois_lados() {
        let nodes = vec![ident("a"), literal_text("texto"), ident("b")];
        let gaps = compute_gaps(&nodes, 10.0, false, 4.2);
        assert_eq!(
            gaps,
            vec![4.2, 4.2],
            "texto literal deve ter text_space_pt dos dois lados, não 0.0"
        );
    }

    #[test]
    fn texto_literal_sozinho_nao_produz_gaps() {
        let nodes = vec![literal_text("texto")];
        assert!(compute_gaps(&nodes, 10.0, false, 4.2).is_empty());
    }

    #[test]
    fn texto_literal_e_alphabetic() {
        assert_eq!(
            node_math_class(&literal_text("texto")),
            (MathClass::Alphabetic, MathClass::Alphabetic)
        );
    }

    #[test]
    fn texto_literal_antes_de_virgula_continua_sem_espaco() {
        // Regra explícita de Punctuation (0.0, "quer" zero) continua a
        // ganhar sobre o fallback de item espaçado — mesma prioridade do
        // vanilla (o match de `spacing()` resolve as regras de classe
        // ANTES de sequer consultar `is_spaced()`).
        let nodes = vec![literal_text("texto"), text(",")];
        let gaps = compute_gaps(&nodes, 10.0, false, 4.2);
        assert_eq!(gaps, vec![0.0], "vírgula continua sem espaço antes, mesmo após texto literal");
    }

    // ── P907 Parte A: `|` como fence recebe espaço dos dois lados ────
    //
    // Achado registado em P825 (`spacing.md` "fora de escopo"), reconfirmado
    // ainda presente em P903. Paridade vanilla (`math/ir/item.rs::
    // MathItem::is_spaced`): `class() == Fence` é SEMPRE "spaced",
    // incondicionalmente — mecanismo idêntico ao já usado para
    // `Content::Text` (P903), só o gatilho muda (classe, não tipo de nó).
    // `|` como fence recebe `unicode_math_class::class('|') ==
    // Some(MathClass::Fence)` via `default_math_class` (delegação TR25,
    // já correcto — não é isto que falta).

    #[test]
    fn fence_entre_identificadores_recebe_espaco_dos_dois_lados() {
        let nodes = vec![
            Content::math_class_override(MathClass::Fence, text("|")),
            ident("x"),
        ];
        // Só um gap (2 nós) — testar o outro lado com 3 nós abaixo.
        let gaps = compute_gaps(&nodes, 10.0, false, 4.2);
        assert_eq!(gaps, vec![4.2], "fence deve ter text_space_pt à direita");
    }

    #[test]
    fn fence_dos_dois_lados_recebe_espaco_dos_dois_lados() {
        let nodes = vec![
            ident("RR"),
            Content::math_class_override(MathClass::Fence, text("|")),
            ident("x"),
        ];
        let gaps = compute_gaps(&nodes, 10.0, false, 4.2);
        assert_eq!(
            gaps,
            vec![4.2, 4.2],
            "fence deve ter text_space_pt dos dois lados, não 0.0"
        );
    }

    #[test]
    fn fence_antes_de_punctuation_continua_sem_espaco() {
        // Regra explícita de Punctuation continua a ganhar sobre o fallback
        // de item espaçado — mesma prioridade já confirmada para texto
        // literal (`texto_literal_antes_de_virgula_continua_sem_espaco`).
        let nodes = vec![
            Content::math_class_override(MathClass::Fence, text("|")),
            text(","),
        ];
        let gaps = compute_gaps(&nodes, 10.0, false, 4.2);
        assert_eq!(gaps, vec![0.0], "vírgula continua sem espaço antes, mesmo após fence");
    }

    #[test]
    fn abs_delimitado_nao_afectado_por_fence() {
        // `abs(x)` usa `Content::MathDelimited` (Opening/Closing), não
        // `MathClass::Fence` — não deve ser afectado por esta correcção.
        let c = Content::math_delimited('|', ident("x"), '|');
        let nodes = vec![ident("a"), c, ident("b")];
        let gaps = compute_gaps(&nodes, 10.0, false, 4.2);
        assert_eq!(
            gaps,
            vec![0.0, 0.0],
            "abs(x)/MathDelimited continua Opening/Closing, sem gap"
        );
    }

    // ── P907 Parte B: operadores com nome (`min`/`max`/`lim`/...) ────
    //
    // Achado registado em P903 (`spacing.md` "fora de escopo"): `min f(x)`
    // sem espaço no cristalino (`min𝑓(𝑥)`), vanilla mostra `min 𝑓(𝑥)`.
    // Causa confirmada por leitura do vanilla real (`math/ir/resolve.rs::
    // resolve_op`, não inferida): `Content::MathOp` (`min`/`max`/`lim`/
    // `sin`/etc., via `make_math_module`) corresponde a `OpElem` no
    // vanilla, que recebe SEMPRE `MathClass::Large` — independente da
    // flag `limits`. `base_math_class` do cristalino não tinha nenhum
    // braço para `Content::MathOp`, caindo no catch-all `Normal` — por
    // isso a regra "(Large, _) => THIN" nunca disparava.
    //
    // Segunda causa, só visível no caso COM subscrito (`min_(x)`):
    // `ScriptsItem::create` do vanilla (`math/ir/item.rs`) tem a doc
    // explícita "The resulting item inherits its math class from the
    // base" — `min_(x)` (MathAttach) herda `Large` de `min`, não cai em
    // Normal. `base_math_class` do cristalino não tinha braço para
    // `Content::MathAttach` (mesmo catch-all `Normal`) — por isso o caso
    // COM subscrito continuaria sem espaço mesmo depois de corrigir só o
    // `MathOp`.

    #[test]
    fn math_op_e_large() {
        let op = Content::math_op(Content::text("min"), true);
        assert_eq!(node_math_class(&op), (MathClass::Large, MathClass::Large));
    }

    #[test]
    fn math_op_sem_limits_tambem_e_large() {
        // `resolve_op` do vanilla ignora a flag `limits` para a classe —
        // `sin`/`cos`/etc (limits=false) são Large tal como `min`/`max`.
        let op = Content::math_op(Content::text("sin"), false);
        assert_eq!(node_math_class(&op), (MathClass::Large, MathClass::Large));
    }

    #[test]
    fn min_seguido_de_alphabetic_recebe_thin_sem_subscrito() {
        let nodes = vec![Content::math_op(Content::text("min"), true), ident("f")];
        let gaps = compute_gaps(&nodes, 18.0, false, 0.0);
        assert_eq!(gaps, vec![THIN * 18.0], "min sem subscrito deve ter THIN antes do conteúdo seguinte");
    }

    #[test]
    fn min_com_subscrito_recebe_thin_antes_do_conteudo_seguinte() {
        // MathAttach(min, sub: x) deve herdar Large de `min` (paridade
        // `ScriptsItem::create` vanilla), não cair em Normal.
        let min_attach = Content::math_attach(
            Content::math_op(Content::text("min"), true),
            None,
            None,
            Some(ident("x")),
            None,
        );
        let nodes = vec![min_attach, ident("f")];
        let gaps = compute_gaps(&nodes, 18.0, false, 0.0);
        assert_eq!(gaps, vec![THIN * 18.0], "min_(x) deve ter THIN antes do conteúdo seguinte");
    }

    #[test]
    fn min_antes_de_parenteses_continua_sem_espaco() {
        // (Large, Opening) => 0 — regra explícita já existente, não deve
        // regredir: `min(x)`-estilo (delimitador logo a seguir) continua
        // sem espaço extra.
        let c = Content::math_delimited('(', ident("x"), ')');
        let nodes = vec![Content::math_op(Content::text("min"), true), c];
        let gaps = compute_gaps(&nodes, 18.0, false, 0.0);
        assert_eq!(gaps, vec![0.0], "min antes de delimitador continua sem espaço extra");
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
        let gaps = compute_gaps(&nodes, 10.0, true, 0.0);
        assert_eq!(gaps, vec![0.0, 0.0]);
    }

    #[test]
    fn p891_in_script_suprime_medium_de_binary() {
        let nodes = vec![ident("a"), text("+"), ident("b")];
        let gaps = compute_gaps(&nodes, 10.0, true, 0.0);
        assert_eq!(gaps, vec![0.0, 0.0]);
    }

    #[test]
    fn p891_fora_de_script_continua_normal() {
        // Confirma que a mudança não afecta o comportamento pré-existente
        // quando in_script é false (regressão contra o caso base já testado).
        let nodes = vec![ident("a"), text("="), ident("b")];
        let gaps = compute_gaps(&nodes, 10.0, false, 0.0);
        assert_eq!(gaps, vec![THICK * 10.0, THICK * 10.0]);
    }

    // ── P1043: Pares de independência testcase() para spacing.rs:308 ─────────

    #[test]
    fn p1043_math_spacing_prev_spaced_only_isolada() {
        // C1=T, C2=F: prev_is_spaced && !is_spaced -> gap = text_space_pt
        let nodes = vec![Content::Text("a".into()), Content::Empty];
        let gaps = compute_gaps(&nodes, 10.0, false, 3.5);
        assert_eq!(gaps, vec![3.5]);
    }

    #[test]
    fn p1043_math_spacing_curr_spaced_only_isolada() {
        // C1=F, C2=T: !prev_is_spaced && is_spaced -> gap = text_space_pt
        let nodes = vec![Content::Empty, Content::Text("b".into())];
        let gaps = compute_gaps(&nodes, 10.0, false, 3.5);
        assert_eq!(gaps, vec![3.5]);
    }

    #[test]
    fn p1043_math_spacing_neither_spaced_isolada() {
        // C1=F, C2=F: !prev_is_spaced && !is_spaced -> gap = 0.0
        let nodes = vec![Content::Empty, Content::Empty];
        let gaps = compute_gaps(&nodes, 10.0, false, 3.5);
        assert_eq!(gaps, vec![0.0]);
    }

}
