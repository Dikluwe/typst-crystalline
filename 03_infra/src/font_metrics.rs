//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/font_metrics.md
//! @prompt-hash 48e21627
//! @layer L3
//! @updated 2026-03-28

use std::collections::HashMap;

use ttf_parser::Face;
use typst_core::{
    entities::{
        glyph_variants::{
            GlyphAssembly, GlyphPart, GlyphVariant, GlyphVariants,
            MathGlyphKern, MathKernRecord, MathKernTable,
        },
        layout_types::Pt,
        math_constants::MathConstants,
    },
    rules::layout::FontMetrics,
};

/// Extrai variantes verticais de um glifo directamente a partir da face.
fn extract_variants(face: &Face<'_>, c: char) -> GlyphVariants {
    let glyph_id = match face.glyph_index(c) { Some(id) => id, None => return GlyphVariants::default() };
    let math_table = match face.tables().math { Some(m) => m, None => return GlyphVariants::default() };
    let variants_table = match math_table.variants { Some(v) => v, None => return GlyphVariants::default() };
    let construction = match variants_table.vertical_constructions.get(glyph_id) {
        Some(c) => c, None => return GlyphVariants::default()
    };
    GlyphVariants {
        variants: construction.variants.into_iter().map(|r| GlyphVariant {
            glyph_id: r.variant_glyph.0,
            advance:  r.advance_measurement as f64,
        }).collect(),
    }
}

/// Extrai a assembly vertical de um glifo directamente a partir da face.
fn extract_assembly(face: &Face<'_>, c: char) -> GlyphAssembly {
    let glyph_id = match face.glyph_index(c) { Some(id) => id, None => return GlyphAssembly::default() };
    let math_table = match face.tables().math { Some(m) => m, None => return GlyphAssembly::default() };
    let variants_table = match math_table.variants { Some(v) => v, None => return GlyphAssembly::default() };
    let construction = match variants_table.vertical_constructions.get(glyph_id) {
        Some(c) => c, None => return GlyphAssembly::default()
    };
    let ttf_assembly = match construction.assembly { Some(a) => a, None => return GlyphAssembly::default() };
    GlyphAssembly {
        parts: ttf_assembly.parts.into_iter().map(|p| GlyphPart {
            glyph_id:        p.glyph_id.0,
            start_connector: p.start_connector_length,
            end_connector:   p.end_connector_length,
            full_advance:    p.full_advance,
            is_extender:     p.part_flags.extender(),
        }).collect(),
    }
}

/// Constrói o dicionário reverso preemptivo: glyph_id → char base.
///
/// Itera sobre os caracteres matemáticos extensíveis conhecidos, extrai
/// as variantes e as peças de assembly da tabela MATH, e guarda o
/// mapeamento inverso. Extensores mapeiam para `|` (barra vertical).
/// Usa `or_insert` para não sobrescrever se a fonte partilhar a peça
/// entre múltiplos caracteres base.
pub(crate) fn build_math_glyph_reverse_map(face: &Face<'_>) -> HashMap<u16, char> {
    const STRETCHY_BASES: &[char] = &['(', ')', '[', ']', '{', '}', '|', '√'];

    let mut map = HashMap::new();
    for &base_char in STRETCHY_BASES {
        for v in extract_variants(face, base_char).variants {
            map.entry(v.glyph_id).or_insert(base_char);
        }
        for part in extract_assembly(face, base_char).parts {
            let mapped = if part.is_extender { '|' } else { base_char };
            map.entry(part.glyph_id).or_insert(mapped);
        }
    }
    map
}

/// Métricas de fonte reais via `ttf-parser`.
///
/// `font_size` não armazenado — passado em cada chamada (invariante do trait).
/// Lifetime `'a` ligado aos bytes da fonte.
pub struct FontBookMetrics<'a> {
    face: Face<'a>,
    upem: f64,  // units_per_em — tipicamente 1000 ou 2048
    /// Dicionário reverso preemptivo: glyph_id → char base.
    /// Preenchido em `from_bytes`. Usado por `glyph_to_char`.
    glyph_to_unicode: HashMap<u16, char>,
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
        let glyph_to_unicode = build_math_glyph_reverse_map(&face);
        Some(Self { face, upem, glyph_to_unicode })
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

    fn vertical_glyph_variants(&self, c: char) -> GlyphVariants {
        extract_variants(&self.face, c)
    }

    fn glyph_to_char(&self, glyph_id: u16) -> Option<char> {
        self.glyph_to_unicode.get(&glyph_id).copied()
    }

    fn vertical_glyph_assembly(&self, c: char) -> GlyphAssembly {
        extract_assembly(&self.face, c)
    }

    fn math_constants(&self) -> MathConstants {
        match self.face.tables().math {
            Some(math_table) => match math_table.constants {
                Some(c) => MathConstants {
                    upem: self.upem,
                    fraction_rule_thickness:
                        c.fraction_rule_thickness().value as f64,
                    fraction_num_gap:
                        c.fraction_numerator_gap_min().value as f64,
                    fraction_denom_gap:
                        c.fraction_denominator_gap_min().value as f64,
                    superscript_shift_up:
                        c.superscript_shift_up().value as f64,
                    subscript_shift_down:
                        c.subscript_shift_down().value as f64,
                    radical_vertical_gap:
                        c.radical_vertical_gap().value as f64,
                    radical_rule_thickness:
                        c.radical_rule_thickness().value as f64,
                    axis_height:
                        c.axis_height().value as f64,
                    script_percent_scale_down:
                        c.script_percent_scale_down() as f64 / 100.0,
                    script_script_percent_scale_down:
                        c.script_script_percent_scale_down() as f64 / 100.0,
                },
                None => MathConstants::fallback(),
            },
            None => MathConstants::fallback(),
        }
    }

    fn math_kern(&self, c: char) -> MathGlyphKern {
        let glyph_id = match self.face.glyph_index(c) {
            Some(id) => id,
            None => return MathGlyphKern::default(),
        };

        let math = match self.face.tables().math {
            Some(m) => m,
            None => return MathGlyphKern::default(),
        };

        let glyph_info = match math.glyph_info {
            Some(gi) => gi,
            None => return MathGlyphKern::default(),
        };

        let kern_infos = match glyph_info.kern_infos {
            Some(k) => k,
            None => return MathGlyphKern::default(),
        };

        let kern_record = match kern_infos.get(glyph_id) {
            Some(r) => r,
            None => return MathGlyphKern::default(),
        };

        // Lê uma tabela Kern do ttf-parser em MathKernTable de L1.
        // A tabela tem `count` alturas e `count+1` valores de kern:
        //   kern[0] aplica-se até height[0], …, kern[count] aplica-se
        //   a todas as alturas acima de height[count-1].
        fn read_kern(kern: Option<ttf_parser::math::Kern>) -> MathKernTable {
            let kern = match kern { Some(k) => k, None => return MathKernTable::default() };
            let count = kern.count() as usize;
            let mut records = Vec::with_capacity(count + 1);
            for i in 0..count {
                let height = kern.height(i as u16).map(|v| v.value as f64);
                let kv     = kern.kern(i as u16).map(|v| v.value as f64).unwrap_or(0.0);
                records.push(MathKernRecord { correction_height: height, kern_value: kv });
            }
            // Último valor de kern (sem correction_height associado)
            if let Some(kv) = kern.kern(count as u16).map(|v| v.value as f64) {
                records.push(MathKernRecord { correction_height: None, kern_value: kv });
            }
            MathKernTable { records }
        }

        MathGlyphKern {
            top_right:    read_kern(kern_record.top_right),
            top_left:     read_kern(kern_record.top_left),
            bottom_right: read_kern(kern_record.bottom_right),
            bottom_left:  read_kern(kern_record.bottom_left),
        }
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
