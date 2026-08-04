//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/math_constants.md
//! @prompt-hash 3bd1a686
//! @layer L1
//! @updated 2026-04-11

use crate::entities::layout_types::Pt;

/// Constantes matemáticas extraídas da tabela OpenType MATH.
///
/// Valores em unidades de design (design units). Para converter
/// para pontos tipográficos: `pt = size * (value / upem)`.
///
/// Fonte: OpenType spec §6.3.4 — MathConstants table.
/// Fallback: valores baseados em STIX Two Math como referência.
#[derive(Debug, Clone)]
pub struct MathConstants {
    /// units_per_em da fonte — necessário para conversão.
    pub upem: f64,

    // ── Fracções ─────────────────────────────────────────
    /// Espessura da barra de fracção.
    pub fraction_rule_thickness: f64,
    /// Gap mínimo entre numerador e barra.
    ///
    /// **P920** — piso mínimo da fórmula real de gap (`fraction.md` §P920),
    /// não o gap directo — ver `fraction_numerator_shift_up` abaixo.
    pub fraction_num_gap: f64,
    /// Gap mínimo entre barra e denominador.
    ///
    /// **P920** — piso mínimo, mesmo tratamento de `fraction_num_gap` acima.
    pub fraction_denom_gap: f64,
    /// **P920** — deslocamento vertical (acima do eixo matemático) do
    /// numerador de uma fracção. Consumido pela fórmula real do vanilla
    /// (`frac.md` §P920) para calcular o gap numerador/barra a partir da
    /// tinta real do numerador, em vez de usar `fraction_num_gap`
    /// directamente como gap.
    pub fraction_numerator_shift_up: f64,
    /// **P920** — deslocamento vertical (abaixo do eixo matemático) do
    /// denominador de uma fracção. Mesmo mecanismo de
    /// `fraction_numerator_shift_up`, para o lado do denominador.
    pub fraction_denominator_shift_down: f64,

    // ── Scripts (sup/sub) ────────────────────────────────
    /// Deslocamento vertical do superscript.
    pub superscript_shift_up: f64,
    /// **P915** — deslocamento vertical do superscript em contexto
    /// "cramped" (substitui `superscript_shift_up` só nesse caso — mesmo
    /// mecanismo do vanilla, `scripts.rs:325-330`, nenhum outro termo da
    /// fórmula é afectado).
    pub superscript_shift_up_cramped: f64,
    /// Deslocamento vertical do subscript.
    pub subscript_shift_down: f64,
    /// Deslocamento mínimo para o fundo do superscript.
    pub superscript_bottom_min: f64,
    /// Deslocamento máximo para o fundo do superscript quando acompanhado de subscript.
    pub superscript_bottom_max_with_subscript: f64,
    /// Queda máxima da baseline do superscript.
    pub superscript_baseline_drop_max: f64,
    /// Gap mínimo entre subscript e superscript.
    pub sub_superscript_gap_min: f64,
    /// Limite máximo do topo do subscript.
    pub subscript_top_max: f64,
    /// Queda mínima da baseline do subscript.
    pub subscript_baseline_drop_min: f64,

    // ── Radicais ─────────────────────────────────────────
    /// Gap vertical entre radicando e overline.
    pub radical_vertical_gap: f64,
    /// Espessura da overline do radical.
    pub radical_rule_thickness: f64,

    // ── Eixo matemático ──────────────────────────────────
    /// Altura do eixo matemático (centro de +, =, …) acima da baseline.
    /// Tipicamente ~500 du (para upem=1000). Fracções, delimitadores e
    /// raízes são centrados neste eixo.
    pub axis_height: f64,

    // ── Escala de scripts ────────────────────────────────
    /// Percentagem de escala para script (sup/sub/frac).
    /// Tipicamente 70–80%. Armazenado como 0.0–1.0.
    pub script_percent_scale_down: f64,
    /// Percentagem para script-script (sup de sup).
    /// Tipicamente 50–60%. Armazenado como 0.0–1.0.
    pub script_script_percent_scale_down: f64,

    // ── Limites de operadores grandes ────────────────────
    /// Espaço mínimo entre a base e o limite superior (design units).
    /// Fallback: 100.0 (≈ 0.1 × upem típico de 1000).
    pub upper_limit_gap_min: f64,
    /// Espaço mínimo entre a base e o limite inferior (design units).
    /// Fallback: 100.0
    pub lower_limit_gap_min: f64,
    /// Altura mínima da baseline do limite superior acima do topo da base
    /// (design units). Piso do `max()` no shift vertical do limite
    /// (`engine/math/layout/attach.md` §P959).
    /// Fallback: 111.0 (medido em NewCMMath-Book, upem=1000).
    pub upper_limit_baseline_rise_min: f64,
    /// Distância mínima da baseline do limite inferior abaixo da base da
    /// base (design units). Piso do `max()` no shift vertical do limite
    /// (`engine/math/layout/attach.md` §P959).
    /// Fallback: 600.0 (medido em NewCMMath-Book, upem=1000).
    pub lower_limit_baseline_drop_min: f64,

    // ── Espaçamento inter-linhas (Passo 52) ──────────────
    /// Gap entre linhas de equações alinhadas (design units).
    /// OpenType MATH: MathLeading. Fallback: 20% de upem.
    pub math_leading: f64,

    // ── Acentos (P922) ──────────────────────────────────────────────────
    /// Cap para o gap entre acento e base alta (design units).
    /// OpenType MATH: AccentBaseHeight.
    pub accent_base_height: f64,

    /// Threshold para a variante "flattened" do acento quando a base é
    /// muito alta (design units). OpenType MATH: FlattenedAccentBaseHeight.
    /// Ainda não consumido.
    pub flattened_accent_base_height: f64,

    // ── Operadores grandes em Display (P952) ─────────────────────────────
    /// Altura mínima (design units) para a qual os glifos de classe
    /// `Large` (∑, ∏, ∫, ⋃, …) são esticados em Display, via variantes
    /// verticais. OpenType MATH: DisplayOperatorMinHeight.
    /// Consumido por `layout_large_operator_display` (`_comum.md` §P952).
    pub display_operator_min_height: f64,
}

impl MathConstants {
    /// Fallback com valores baseados em STIX Two Math (upem=1000).
    ///
    /// Usado quando a fonte não tem tabela MATH (ex: Helvetica).
    pub fn fallback() -> Self {
        let upem = 1000.0;
        let axis_height = 500.0;
        Self {
            upem,
            fraction_rule_thickness: 66.0,
            fraction_num_gap: 50.0,
            fraction_denom_gap: 50.0,
            // P920 — mesma disciplina de P915 (superscript_shift_up_cramped):
            // sem fonte STIX Two Math disponível para confirmar um valor
            // condizente com o resto deste fallback, usa os valores REAIS
            // medidos na fonte de produção do cristalino (`fontTools` sobre
            // `NewCMMath-Regular.otf`, upem=1000 — mesma fonte de
            // `p893_...`/P915): `FractionNumeratorShiftUp.Value=394`,
            // `FractionDenominatorShiftDown.Value=345`. Ver `entities/
            // math_constants.md` §P920.
            fraction_numerator_shift_up: 394.0,
            fraction_denominator_shift_down: 345.0,
            superscript_shift_up: 362.0,
            // P915 — sem fonte STIX Two Math cujos valores batessem com o
            // resto do fallback (ver typst-passo-915-relatorio.md); usa o
            // mesmo valor de superscript_shift_up como default neutro —
            // "sem efeito de cramped" quando não há fonte real.
            superscript_shift_up_cramped: 362.0,
            subscript_shift_down: 130.0,
            superscript_bottom_min: 125.0,
            superscript_bottom_max_with_subscript: 400.0,
            superscript_baseline_drop_max: 250.0,
            sub_superscript_gap_min: 200.0,
            subscript_top_max: 344.0,
            subscript_baseline_drop_min: 50.0,
            radical_vertical_gap: 60.0,
            radical_rule_thickness: 66.0,
            axis_height,
            script_percent_scale_down: 0.7,
            script_script_percent_scale_down: 0.5,
            upper_limit_gap_min: 100.0,
            lower_limit_gap_min: 100.0,
            // P959 — valores reais medidos de NewCMMath-Book (upem=1000,
            // fontTools): UpperLimitBaselineRiseMin=111,
            // LowerLimitBaselineDropMin=600. Mesmo padrão de P952
            // (`display_operator_min_height`=1300): o fallback documenta
            // valores reais da fonte de referência.
            upper_limit_baseline_rise_min: 111.0,
            lower_limit_baseline_drop_min: 600.0,
            math_leading: upem * 0.2, // 200.0 para upem=1000
            // P922 — default neutro em contexto sem fonte real; o cap não tem
            // efeito observável quando `FixedMetrics` é usado.
            accent_base_height: axis_height,
            flattened_accent_base_height: axis_height,
            // P952 — DisplayOperatorMinHeight de NewCMMath (1300du, upem=1000),
            // medido via `fontTools` sobre a fonte de produção — mesma
            // disciplina de P915/P920 (valores reais da fonte activa, não
            // inventados). Ver `entities/math_constants.md` §P952.
            display_operator_min_height: 1300.0,
        }
    }

    /// Converte um valor em design units para Pt.
    ///
    /// `value` em design units, `size` é o tamanho de fonte.
    pub fn to_pt(&self, value: f64, size: Pt) -> Pt {
        size * (value / self.upem)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::layout::FixedMetrics;
    use crate::engine::layout::FontMetrics;

    #[test]
    fn fallback_valores_sanos() {
        let c = MathConstants::fallback();
        assert!(c.upem > 0.0);
        assert!(c.fraction_rule_thickness > 0.0);
        assert!(c.radical_rule_thickness > 0.0);
        assert!(c.script_percent_scale_down > 0.0);
        assert!(c.script_percent_scale_down <= 1.0);
        assert!(c.script_script_percent_scale_down > 0.0);
        assert!(c.script_script_percent_scale_down <= 1.0);
        assert!(c.superscript_shift_up > 0.0);
        assert!(c.subscript_shift_down > 0.0);
        assert!(c.axis_height > 0.0);
    }

    #[test]
    fn to_pt_converte_correctamente() {
        let c = MathConstants::fallback(); // upem = 1000
        let pt = c.to_pt(500.0, Pt(12.0));
        // 12.0 * (500.0 / 1000.0) = 6.0
        assert!((pt.val() - 6.0).abs() < 0.001);
    }

    #[test]
    fn to_pt_zero_value() {
        let c = MathConstants::fallback();
        let pt = c.to_pt(0.0, Pt(12.0));
        assert!((pt.val()).abs() < 0.001);
    }

    #[test]
    fn to_pt_proporcional_ao_tamanho() {
        let c = MathConstants::fallback();
        let pt12 = c.to_pt(100.0, Pt(12.0));
        let pt24 = c.to_pt(100.0, Pt(24.0));
        assert!((pt24.val() - 2.0 * pt12.val()).abs() < 0.001);
    }

    #[test]
    fn fixed_metrics_retorna_fallback() {
        let m = FixedMetrics;
        let c = m.math_constants(&crate::entities::layout_types::TextStyle::default());
        assert!((c.upem - 1000.0).abs() < 0.001);
        assert!(c.fraction_rule_thickness > 0.0);
        assert!(c.script_percent_scale_down > 0.0);
    }

    // ── Passo 52 — math_leading ───────────────────────────────────────────

    #[test]
    fn math_leading_fallback_e_positivo() {
        let c = MathConstants::fallback();
        // Fallback = 20% de upem = 200.0
        assert!(
            c.math_leading > 0.0,
            "math_leading deve ser positivo, foi {}",
            c.math_leading
        );
    }

    #[test]
    fn math_leading_fallback_e_20_pct_upem() {
        let c = MathConstants::fallback();
        let esperado = c.upem * 0.2;
        assert!(
            (c.math_leading - esperado).abs() < 0.001,
            "esperava {} (20% de upem={}), obteve {}",
            esperado,
            c.upem,
            c.math_leading
        );
    }

    #[test]
    fn math_leading_to_pt_proporcional() {
        let c = MathConstants::fallback(); // upem=1000, math_leading=200
        let pt = c.to_pt(c.math_leading, Pt(10.0));
        // 10.0 * (200.0 / 1000.0) = 2.0
        assert!((pt.val() - 2.0).abs() < 0.001, "esperava 2.0pt, obteve {}pt", pt.val());
    }
}
