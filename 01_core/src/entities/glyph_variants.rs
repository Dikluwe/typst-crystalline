//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/glyph_variants.md
//! @prompt-hash 6b6620e6
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
    /// Medida de avanço na direcção de crescimento (eixo de esticamento —
    /// altura para variantes verticais, largura para horizontais), em
    /// design units. Só para comparar com o alvo (`select`/
    /// `select_with_advance`/`select_variant`) — nunca para posicionar o
    /// glifo (ver `hor_advance` abaixo). **P917**.
    pub advance: f64,
    /// Avanço horizontal NATIVO do glifo (hmtx), em design units —
    /// independente do eixo de esticamento. Usado para
    /// `FrameItem::Glyph.x_advance`/largura da caixa que contém o glifo.
    /// **P917** — ver `entities/glyph_variants.md` §P917.
    pub hor_advance: f64,
    /// `TopAccentAttachment` MATH da variante, em design units. `None`
    /// aplica o fallback `(hor_advance + italics_correction)/2`.
    pub top_accent_attach: Option<f64>,
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

    /// **P917** — como `select`/`select_with_advance`, mas devolve a
    /// variante completa (incluindo `hor_advance`) em vez de só
    /// `(glyph_id, advance)`. Callers que precisam de posicionar o glifo
    /// (`x_advance`/largura de caixa) devem usar este método e ler
    /// `hor_advance`, nunca `advance` (que é a medida do eixo de
    /// esticamento, não o avanço horizontal — ver doc de `GlyphVariant`).
    pub fn select_variant(&self, min_advance: f64) -> Option<&GlyphVariant> {
        self.variants.iter().find(|v| v.advance >= min_advance)
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
/// `full_advance`: avanço total da peça sem sobreposição, ao longo do eixo
/// de empilhamento (design units) — só para o cálculo de posição/sobreposição
/// entre peças, nunca para `x_advance` (ver `hor_advance`). **P917**.
/// `is_extender`: se true, esta peça pode ser repetida para preencher altura.
/// `hor_advance`: avanço horizontal NATIVO da peça (hmtx, design units),
/// independente do eixo de empilhamento — usado para
/// `FrameItem::Glyph.x_advance`/largura da caixa. **P917**.
#[derive(Debug, Clone)]
pub struct GlyphPart {
    pub glyph_id: u16,
    pub start_connector: u16,
    pub end_connector: u16,
    pub full_advance: u16,
    pub is_extender: bool,
    pub hor_advance: f64,
}

/// Montagem por partes para um delimitador extensível.
///
/// Usado quando a altura exigida excede todas as variantes em `GlyphVariants`.
/// As peças são empilhadas verticalmente (bottom → top).
#[derive(Debug, Clone, Default)]
pub struct GlyphAssembly {
    pub parts: Vec<GlyphPart>,
    /// **P945** — `minConnectorOverlap` da tabela MATH (design units;
    /// `Default = 0` reproduz exactamente o comportamento anterior).
    /// Consumido em `resolve_assembly_repeat` e no posicionamento das peças
    /// (`compiler/math/layout/assembly.md` §P945); preenchido em L3
    /// (`infra/font_metrics.md` §P945). NewCMMath: 20du.
    pub min_overlap: u16,
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
    pub top_right: MathKernTable,
    pub top_left: MathKernTable,
    pub bottom_right: MathKernTable,
    pub bottom_left: MathKernTable,
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
            hor_advance: full_advance as f64,
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
            ..Default::default()
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
        let a = GlyphAssembly {
            parts: vec![make_part(100, false)],
            ..Default::default()
        };
        assert!(!a.is_empty());
    }

    fn variant(glyph_id: u16, advance: f64) -> GlyphVariant {
        GlyphVariant {
            glyph_id,
            advance,
            hor_advance: advance,
            top_accent_attach: None,
        }
    }

    #[test]
    fn select_with_advance_retorna_advance() {
        let v = GlyphVariants {
            variants: vec![variant(10, 500.0), variant(11, 800.0)],
        };
        let (id, adv) = v.select_with_advance(600.0).unwrap();
        assert_eq!(id, 11);
        assert_eq!(adv, 800.0);
    }

    #[test]
    fn select_variante_minima() {
        let v = GlyphVariants {
            variants: vec![
                variant(100, 500.0),
                variant(101, 800.0),
                variant(102, 1200.0),
            ],
        };
        // Pedir 600 → primeira variante >= 600 é 101 (advance=800)
        assert_eq!(v.select(600.0), Some(101));
    }

    #[test]
    fn select_variante_exacta() {
        let v = GlyphVariants {
            variants: vec![variant(100, 500.0), variant(101, 800.0)],
        };
        assert_eq!(v.select(500.0), Some(100));
    }

    #[test]
    fn select_nenhuma_suficiente() {
        let v = GlyphVariants { variants: vec![variant(100, 500.0)] };
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
        let v = GlyphVariants { variants: vec![variant(1, 100.0)] };
        assert!(!v.is_empty());
    }

    // ── Testes do Passo 917 — select_variant / hor_advance ─────────────────

    #[test]
    fn select_variant_devolve_variante_completa_com_hor_advance() {
        let v = GlyphVariants {
            variants: vec![
                GlyphVariant {
                    glyph_id: 10,
                    advance: 500.0,
                    hor_advance: 55.0,
                    top_accent_attach: None,
                },
                GlyphVariant {
                    glyph_id: 11,
                    advance: 800.0,
                    hor_advance: 60.0,
                    top_accent_attach: None,
                },
            ],
        };
        let picked = v.select_variant(600.0).unwrap();
        assert_eq!(picked.glyph_id, 11);
        assert_eq!(picked.advance, 800.0);
        assert_eq!(
            picked.hor_advance, 60.0,
            "select_variant deve expor hor_advance, distinto de advance (eixo de esticamento)"
        );
    }

    #[test]
    fn select_variant_nenhuma_suficiente_retorna_none() {
        let v = GlyphVariants { variants: vec![variant(100, 500.0)] };
        assert!(v.select_variant(1000.0).is_none());
    }

    // ── Testes do Passo 44 — MathKernTable ───────────────────────────────

    fn three_record_table() -> MathKernTable {
        MathKernTable {
            records: vec![
                MathKernRecord { correction_height: Some(300.0), kern_value: -50.0 },
                MathKernRecord { correction_height: Some(600.0), kern_value: -30.0 },
                MathKernRecord { correction_height: None, kern_value: -10.0 },
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
                MathKernRecord { correction_height: None, kern_value: -10.0 },
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
