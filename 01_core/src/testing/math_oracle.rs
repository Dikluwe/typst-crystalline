//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/testing/math_oracle.md
//! @prompt-hash 745c153a
//! @layer L1
//! @updated 2026-08-05
//!
//! **P969** — oráculo de fórmulas de posição do vanilla: transcrições
//! literais de fórmulas já lidas e confirmadas na frente P901–P968, uma
//! função por fórmula, com `file:line` da fonte. Usado pelos testes de
//! geometria como referência independente do motor de produção.
//!
//! Regras (do prompt): funções puras sobre primitivos em pt; proibido
//! importar código de produção (`MathLayouter`, `FontMetrics`, `Content`);
//! sem `file:line` confirmado, a fórmula não entra.

/// **P959** — shift vertical (baseline-a-baseline) do limite **superior**
/// de um operador grande em Display. Transcrição literal de
/// `compute_limit_shifts`,
/// `lab/typst-original/crates/typst-layout/src/math/scripts.rs:290-313`
/// (`t_shift = base.ascent + upper_rise_min.max(upper_gap_min +
/// t.descent())`, `:299-303`).
///
/// Argumentos em pt: `base_ascent` = ascent de tinta da base (variante de
/// Display, P959), `limit_descent` = descent do limite, `gap_min` =
/// `UpperLimitGapMin` e `rise_min` = `UpperLimitBaselineRiseMin` já
/// convertidos para pt no tamanho corrente.
pub(crate) fn large_operator_upper_shift(
    base_ascent: f64,
    limit_descent: f64,
    gap_min: f64,
    rise_min: f64,
) -> f64 {
    base_ascent + rise_min.max(gap_min + limit_descent)
}

/// **P959** — shift vertical (baseline-a-baseline) do limite **inferior**,
/// mesmo sítio (`b_shift = base.descent + lower_drop_min.max(lower_gap_min
/// + b.ascent())`, `scripts.rs:306-310`).
pub(crate) fn large_operator_lower_shift(
    base_descent: f64,
    limit_ascent: f64,
    gap_min: f64,
    drop_min: f64,
) -> f64 {
    base_descent + drop_min.max(gap_min + limit_ascent)
}

/// **P945** — `total_descent` de uma grelha (matriz/cases/multiline) na
/// forma exacta do vanilla
/// (`lab/typst-original/crates/typst-layout/src/math/table.rs:103-106`):
/// `d_1 + Σ_{r≥2}(a_r + d_r) + gap × (nrows−1)` — o descent de cada linha
/// intermédia conta **uma** vez (a forma anterior do cristalino contava
/// duas, P945).
///
/// `rows` = pares `(ascent, descent)` por linha, já com o piso do `(`
/// sintético aplicado (P921) se aplicável; `gap` = row gap em pt.
pub(crate) fn grid_total_descent(rows: &[(f64, f64)], gap: f64) -> f64 {
    let Some((_, first_descent)) = rows.first() else {
        return 0.0;
    };
    let rest: f64 = rows.iter().skip(1).map(|(a, d)| a + d).sum();
    first_descent + rest + gap * rows.len().saturating_sub(1) as f64
}

/// **P919** — baseline de uma grelha no eixo matemático, transcrição de
/// `frame.set_baseline(height/2 + axis)`
/// (`lab/typst-original/crates/typst-layout/src/math/table.rs:188`).
/// `height` = ascent + descent total da grelha; `axis` = `axis_height` em
/// pt no tamanho corrente.
pub(crate) fn grid_axis_baseline(height: f64, axis: f64) -> f64 {
    height / 2.0 + axis
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn upper_shift_escolhe_o_maior_dos_pisos() {
        // gap domina: 8.0 + max(1.332, 2.4+0.5) = 10.9
        assert!((large_operator_upper_shift(8.0, 0.5, 2.4, 1.332) - 10.9).abs() < 1e-9);
        // rise domina: 8.0 + max(6.0, 2.9) = 14.0
        assert!((large_operator_upper_shift(8.0, 0.5, 2.4, 6.0) - 14.0).abs() < 1e-9);
    }

    #[test]
    fn lower_shift_escolhe_o_maior_dos_pisos() {
        // drop domina: 2.0 + max(7.2, 2.004+2.0) = 9.2
        assert!((large_operator_lower_shift(2.0, 2.0, 2.004, 7.2) - 9.2).abs() < 1e-9);
        // gap domina: 2.0 + max(7.2, 2.004+6.0) = 10.004
        assert!((large_operator_lower_shift(2.0, 6.0, 2.004, 7.2) - 10.004).abs() < 1e-9);
    }

    #[test]
    fn grid_total_descent_forma_do_vanilla() {
        // P945: 3 linhas (10/4), gap 2pt → 4 + 14 + 14 + 4 = 36
        let rows = [(10.0, 4.0), (10.0, 4.0), (10.0, 4.0)];
        assert!((grid_total_descent(&rows, 2.0) - 36.0).abs() < 1e-9);
        assert_eq!(grid_total_descent(&[], 2.0), 0.0);
    }

    #[test]
    fn grid_axis_baseline_e_meio_mais_eixo() {
        assert!((grid_axis_baseline(20.0, 2.75) - 12.75).abs() < 1e-9);
    }
}
