//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/math_constants.md
//! @prompt-hash dba58f18
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
    pub fraction_num_gap: f64,
    /// Gap mínimo entre barra e denominador.
    pub fraction_denom_gap: f64,

    // ── Scripts (sup/sub) ────────────────────────────────
    /// Deslocamento vertical do superscript.
    pub superscript_shift_up: f64,
    /// Deslocamento vertical do subscript.
    pub subscript_shift_down: f64,

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

    // ── Espaçamento inter-linhas (Passo 52) ──────────────
    /// Gap entre linhas de equações alinhadas (design units).
    /// OpenType MATH: MathLeading. Fallback: 20% de upem.
    pub math_leading: f64,
}

impl MathConstants {
    /// Fallback com valores baseados em STIX Two Math (upem=1000).
    ///
    /// Usado quando a fonte não tem tabela MATH (ex: Helvetica).
    pub fn fallback() -> Self {
        let upem = 1000.0;
        Self {
            upem,
            fraction_rule_thickness:             66.0,
            fraction_num_gap:                    50.0,
            fraction_denom_gap:                  50.0,
            superscript_shift_up:               362.0,
            subscript_shift_down:               130.0,
            radical_vertical_gap:                60.0,
            radical_rule_thickness:              66.0,
            axis_height:                        500.0,
            script_percent_scale_down:           0.7,
            script_script_percent_scale_down:    0.5,
            upper_limit_gap_min:               100.0,
            lower_limit_gap_min:               100.0,
            math_leading:                      upem * 0.2,  // 200.0 para upem=1000
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
    use crate::rules::layout::FixedMetrics;
    use crate::rules::layout::FontMetrics;

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
        let c = m.math_constants();
        assert!((c.upem - 1000.0).abs() < 0.001);
        assert!(c.fraction_rule_thickness > 0.0);
        assert!(c.script_percent_scale_down > 0.0);
    }

    // ── Passo 52 — math_leading ───────────────────────────────────────────

    #[test]
    fn math_leading_fallback_e_positivo() {
        let c = MathConstants::fallback();
        // Fallback = 20% de upem = 200.0
        assert!(c.math_leading > 0.0,
            "math_leading deve ser positivo, foi {}", c.math_leading);
    }

    #[test]
    fn math_leading_fallback_e_20_pct_upem() {
        let c = MathConstants::fallback();
        let esperado = c.upem * 0.2;
        assert!((c.math_leading - esperado).abs() < 0.001,
            "esperava {} (20% de upem={}), obteve {}",
            esperado, c.upem, c.math_leading);
    }

    #[test]
    fn math_leading_to_pt_proporcional() {
        let c = MathConstants::fallback(); // upem=1000, math_leading=200
        let pt = c.to_pt(c.math_leading, Pt(10.0));
        // 10.0 * (200.0 / 1000.0) = 2.0
        assert!((pt.val() - 2.0).abs() < 0.001,
            "esperava 2.0pt, obteve {}pt", pt.val());
    }
}
