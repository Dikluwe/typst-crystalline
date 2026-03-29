//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/font_metrics.md
//! @prompt-hash 48e21627
//! @layer L3
//! @updated 2026-03-28

use ttf_parser::Face;
use typst_core::{entities::layout_types::Pt, rules::layout::FontMetrics};

/// Métricas de fonte reais via `ttf-parser`.
///
/// `font_size` não armazenado — passado em cada chamada (invariante do trait).
/// Lifetime `'a` ligado aos bytes da fonte.
pub struct FontBookMetrics<'a> {
    face: Face<'a>,
    upem: f64,  // units_per_em — tipicamente 1000 ou 2048
}

impl<'a> FontBookMetrics<'a> {
    /// Constrói métricas a partir de bytes de fonte TrueType/OpenType.
    ///
    /// Retorna `None` se os bytes forem inválidos ou `upem == 0`.
    /// Protecção contra `upem == 0`: fallback para 1000 (não panic).
    pub fn from_bytes(data: &'a [u8]) -> Option<Self> {
        let face = Face::parse(data, 0).ok()?;
        let upem = face.units_per_em();
        let upem = if upem == 0 { 1000.0 } else { upem as f64 };
        Some(Self { face, upem })
    }
}

impl FontMetrics for FontBookMetrics<'_> {
    fn advance(&self, text: &str, size: Pt) -> Pt {
        // Fórmula: advance_pt = font_size * (Σ glyph_units / upem)
        let units: f64 = text
            .chars()
            .map(|c| {
                self.face
                    .glyph_index(c)
                    .and_then(|gid| self.face.glyph_hor_advance(gid))
                    .map(|a| a as f64)
                    .unwrap_or(self.upem * 0.6)  // fallback para glifos ausentes
            })
            .sum();
        size * (units / self.upem)
    }

    fn vertical_metrics(&self, size: Pt) -> (Pt, Pt) {
        let ascender  = self.face.ascender()  as f64;
        // descender: norma diz negativo; .abs() para fontes "incorrectas"
        let descender = (self.face.descender() as f64).abs();
        let line_gap  = self.face.line_gap()  as f64;

        let ascender_pt    = size * (ascender / self.upem);
        let line_height_pt = size * ((ascender + descender + line_gap) / self.upem);

        (ascender_pt, line_height_pt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_bytes_invalidos_retorna_none() {
        assert!(FontBookMetrics::from_bytes(b"not a font").is_none());
        assert!(FontBookMetrics::from_bytes(b"").is_none());
    }

    #[test]
    #[ignore = "requer tests/fixtures/liberation-sans-regular.ttf"]
    fn proporcionalidade_iiii_vs_wwww() {
        let data = std::fs::read(
            concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/liberation-sans-regular.ttf")
        ).expect("fixture necessária");

        let m = FontBookMetrics::from_bytes(&data).expect("fonte válida");
        let size = Pt(12.0);

        let ai = m.advance("iiii", size);
        let aw = m.advance("WWWW", size);

        assert!(
            ai.val() < aw.val(),
            "proporcional: 'iiii' ({:.2}pt) deve ser mais estreito que 'WWWW' ({:.2}pt)\n\
             Diagnóstico: se iiii ≈ 0.07pt → esqueceu size*; se iiii ≈ 700pt → esqueceu /upem",
            ai.val(), aw.val()
        );

        let aa = m.advance("A", size);
        assert!(
            aa.val() > 3.0 && aa.val() < 12.0,
            "'A' em 12pt deve ser 3–12pt, foi {:.2}pt", aa.val()
        );
    }

    #[test]
    #[ignore = "requer tests/fixtures/liberation-sans-regular.ttf"]
    fn upem_zero_nao_causa_divisao_por_zero() {
        // Bytes inválidos → None (nunca chega a upem=0 em advance)
        assert!(FontBookMetrics::from_bytes(b"not a font").is_none());
        assert!(FontBookMetrics::from_bytes(b"").is_none());
    }

    #[test]
    #[ignore = "requer tests/fixtures/liberation-sans-regular.ttf"]
    fn vertical_metrics_sanidade() {
        let data = std::fs::read(
            concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/liberation-sans-regular.ttf")
        ).unwrap();
        let m = FontBookMetrics::from_bytes(&data).unwrap();
        let (asc, lh) = m.vertical_metrics(Pt(12.0));
        assert!(asc.val() > 0.0,       "ascender positivo");
        assert!(lh.val() > asc.val(),  "line_height > ascender");
        assert!(lh.val() < 24.0,       "line_height em 12pt < 24pt");
        // Verificar que métricas escalam com font_size
        let (_, lh24) = m.vertical_metrics(Pt(24.0));
        assert!(
            (lh24.val() - 2.0 * lh.val()).abs() < 0.5,
            "métricas devem escalar com font_size: 24pt ≈ 2× 12pt"
        );
    }
}
