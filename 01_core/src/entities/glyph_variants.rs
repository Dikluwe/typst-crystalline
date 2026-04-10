//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/glyph_variants.md
//! @prompt-hash e4c61708
//! @layer L1
//! @updated 2026-04-10

/// Uma variante de glifo com tamanho diferente.
///
/// `advance` é a medida na direcção de crescimento (altura para
/// variantes verticais, largura para horizontais), em design units.
#[derive(Debug, Clone)]
pub struct GlyphVariant {
    /// Identificador do glifo alternativo (glyph ID na fonte).
    pub glyph_id: u16,
    /// Medida de avanço na direcção de crescimento, em design units.
    pub advance: f64,
}

/// Variantes de tamanho para um glifo extensível.
///
/// Ordenadas por tamanho crescente. O `MathLayouter` selecciona a
/// primeira variante cuja `advance` (em design units) seja >= à
/// altura mínima necessária.
#[derive(Debug, Clone, Default)]
pub struct GlyphVariants {
    pub variants: Vec<GlyphVariant>,
}

impl GlyphVariants {
    /// Selecciona a variante mais pequena com advance >= `min_advance`.
    ///
    /// Retorna `(glyph_id, advance_du)` da variante seleccionada, ou
    /// `None` se nenhuma for grande o suficiente.
    pub fn select_with_advance(&self, min_advance: f64) -> Option<(u16, f64)> {
        self.variants
            .iter()
            .find(|v| v.advance >= min_advance)
            .map(|v| (v.glyph_id, v.advance))
    }

    /// Selecciona a variante mais pequena com advance >= min_advance.
    ///
    /// `min_advance` em design units. Retorna o glyph_id da variante
    /// seleccionada, ou None se nenhuma variante for grande o suficiente.
    pub fn select(&self, min_advance: f64) -> Option<u16> {
        self.select_with_advance(min_advance).map(|(id, _)| id)
    }

    pub fn is_empty(&self) -> bool {
        self.variants.is_empty()
    }
}

// ── GlyphPart e GlyphAssembly ──────────────────────────────────────────────

/// Uma peça individual de um delimitador montado por partes.
///
/// `glyph_id`: índice do glifo da peça.
/// `start_connector`: sobreposição mínima com a peça anterior (design units).
/// `end_connector`: sobreposição mínima com a peça seguinte (design units).
/// `full_advance`: avanço total da peça sem sobreposição (design units).
/// `is_extender`: se true, esta peça pode ser repetida para preencher altura.
#[derive(Debug, Clone)]
pub struct GlyphPart {
    pub glyph_id:       u16,
    pub start_connector: u16,
    pub end_connector:   u16,
    pub full_advance:    u16,
    pub is_extender:     bool,
}

/// Montagem por partes para um delimitador extensível.
///
/// Usado quando a altura exigida excede todas as variantes em `GlyphVariants`.
/// As peças são empilhadas verticalmente (bottom → top).
#[derive(Debug, Clone, Default)]
pub struct GlyphAssembly {
    pub parts: Vec<GlyphPart>,
}

impl GlyphAssembly {
    pub fn is_empty(&self) -> bool {
        self.parts.is_empty()
    }

    /// Calcula a altura total mínima desta assembly (sem repetição de extensores).
    ///
    /// Soma `full_advance` de todas as peças, em design units.
    pub fn min_advance(&self) -> f64 {
        self.parts.iter().map(|p| p.full_advance as f64).sum()
    }
}

// ── MathKernInfo ──────────────────────────────────────────────────────────────

/// Um registo de kern matemático: altura de correcção e valor de kern.
///
/// A tabela define kern por intervalos de altura. Para um script cuja
/// conexão está abaixo de `correction_height`, o kern aplicável é
/// `kern_value`. O último registo tem `correction_height: None` e
/// aplica-se a todas as alturas acima do penúltimo limiar.
///
/// Ambos os valores estão em design units.
#[derive(Debug, Clone)]
pub struct MathKernRecord {
    /// Altura máxima (design units) para a qual este kern se aplica.
    /// `None` no último registo.
    pub correction_height: Option<f64>,
    /// Valor de kern a aplicar (design units). Pode ser negativo.
    pub kern_value: f64,
}

/// Tabela de kern para um quadrante de um glifo matemático.
///
/// Quadrantes: top-right, top-left, bottom-right, bottom-left.
#[derive(Debug, Clone, Default)]
pub struct MathKernTable {
    pub records: Vec<MathKernRecord>,
}

impl MathKernTable {
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// Kern em design units para uma dada altura de conexão do script.
    ///
    /// Percorre os registos em ordem e retorna o kern do primeiro cujo
    /// `correction_height >= height`, ou do último (None) caso nenhum
    /// limiar seja atingido. Retorna `0.0` se a tabela estiver vazia.
    pub fn kern_at(&self, height: f64) -> f64 {
        for record in &self.records {
            match record.correction_height {
                Some(h) if h >= height => return record.kern_value,
                Some(_) => continue,
                None => return record.kern_value,
            }
        }
        0.0
    }
}

/// Kern matemático para os quatro quadrantes de um glifo.
#[derive(Debug, Clone, Default)]
pub struct MathGlyphKern {
    pub top_right:    MathKernTable,
    pub top_left:     MathKernTable,
    pub bottom_right: MathKernTable,
    pub bottom_left:  MathKernTable,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_part(full_advance: u16, is_extender: bool) -> GlyphPart {
        GlyphPart {
            glyph_id: 0,
            start_connector: 50,
            end_connector: 50,
            full_advance,
            is_extender,
        }
    }

    #[test]
    fn assembly_min_advance_soma_full_advances() {
        let a = GlyphAssembly {
            parts: vec![
                make_part(400, false),
                make_part(200, true),
                make_part(400, false),
            ],
        };
        assert_eq!(a.min_advance(), 1000.0);
    }

    #[test]
    fn assembly_vazia_min_advance_zero() {
        assert_eq!(GlyphAssembly::default().min_advance(), 0.0);
    }

    #[test]
    fn assembly_vazia_is_empty() {
        assert!(GlyphAssembly::default().is_empty());
    }

    #[test]
    fn assembly_com_partes_nao_vazia() {
        let a = GlyphAssembly { parts: vec![make_part(100, false)] };
        assert!(!a.is_empty());
    }

    #[test]
    fn select_with_advance_retorna_advance() {
        let v = GlyphVariants {
            variants: vec![
                GlyphVariant { glyph_id: 10, advance: 500.0 },
                GlyphVariant { glyph_id: 11, advance: 800.0 },
            ],
        };
        let (id, adv) = v.select_with_advance(600.0).unwrap();
        assert_eq!(id, 11);
        assert_eq!(adv, 800.0);
    }

    #[test]
    fn select_variante_minima() {
        let v = GlyphVariants {
            variants: vec![
                GlyphVariant { glyph_id: 100, advance: 500.0 },
                GlyphVariant { glyph_id: 101, advance: 800.0 },
                GlyphVariant { glyph_id: 102, advance: 1200.0 },
            ],
        };
        // Pedir 600 → primeira variante >= 600 é 101 (advance=800)
        assert_eq!(v.select(600.0), Some(101));
    }

    #[test]
    fn select_variante_exacta() {
        let v = GlyphVariants {
            variants: vec![
                GlyphVariant { glyph_id: 100, advance: 500.0 },
                GlyphVariant { glyph_id: 101, advance: 800.0 },
            ],
        };
        assert_eq!(v.select(500.0), Some(100));
    }

    #[test]
    fn select_nenhuma_suficiente() {
        let v = GlyphVariants {
            variants: vec![GlyphVariant { glyph_id: 100, advance: 500.0 }],
        };
        assert_eq!(v.select(1000.0), None);
    }

    #[test]
    fn select_vazio() {
        let v = GlyphVariants::default();
        assert_eq!(v.select(100.0), None);
    }

    #[test]
    fn is_empty_vazio() {
        assert!(GlyphVariants::default().is_empty());
    }

    #[test]
    fn is_empty_com_variante() {
        let v = GlyphVariants {
            variants: vec![GlyphVariant { glyph_id: 1, advance: 100.0 }],
        };
        assert!(!v.is_empty());
    }

    // ── Testes do Passo 44 — MathKernTable ───────────────────────────────

    fn three_record_table() -> MathKernTable {
        MathKernTable {
            records: vec![
                MathKernRecord { correction_height: Some(300.0), kern_value: -50.0 },
                MathKernRecord { correction_height: Some(600.0), kern_value: -30.0 },
                MathKernRecord { correction_height: None,        kern_value: -10.0 },
            ],
        }
    }

    #[test]
    fn kern_at_abaixo_do_primeiro_limiar() {
        assert_eq!(three_record_table().kern_at(200.0), -50.0);
    }

    #[test]
    fn kern_at_no_limiar_exacto() {
        assert_eq!(three_record_table().kern_at(300.0), -50.0);
    }

    #[test]
    fn kern_at_entre_limiares() {
        assert_eq!(three_record_table().kern_at(450.0), -30.0);
    }

    #[test]
    fn kern_at_acima_de_todos_os_limiares() {
        let t = MathKernTable {
            records: vec![
                MathKernRecord { correction_height: Some(300.0), kern_value: -50.0 },
                MathKernRecord { correction_height: None,        kern_value: -10.0 },
            ],
        };
        assert_eq!(t.kern_at(999.0), -10.0);
    }

    #[test]
    fn kern_at_tabela_vazia_retorna_zero() {
        assert_eq!(MathKernTable::default().kern_at(500.0), 0.0);
    }

    #[test]
    fn math_glyph_kern_default_vazio() {
        let k = MathGlyphKern::default();
        assert!(k.top_right.is_empty());
        assert!(k.top_left.is_empty());
        assert!(k.bottom_right.is_empty());
        assert!(k.bottom_left.is_empty());
    }
}
