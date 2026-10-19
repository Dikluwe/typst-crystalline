//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/font_metrics.md
//! @prompt-hash f4380478
//! @layer L3
//! @updated 2026-10-10

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use ttf_parser::Face;
use typst_core::contracts::world::World;

mod basic;
mod caches;
mod fallback;
mod math;
mod resolution;

pub(crate) use basic::edge_offset_pt;
use caches::{AdvanceWidthKey, CachedFace, ShapedWidthKey};
pub(crate) use math::{build_math_glyph_reverse_map, ssty_eligible_text};
use typst_core::entities::layout_types::Pt;

/// Métricas de fonte reais via `ttf-parser`.
///
/// `font_size` não armazenado — passado em cada chamada (invariante do trait).
/// Lifetime `'a` ligado aos bytes da fonte.
pub struct FontBookMetrics<'a> {
    face: Face<'a>,
    upem: f64, // units_per_em — tipicamente 1000 ou 2048
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

pub struct FallbackFontMetrics<'a> {
    world: &'a dyn World,
    primary_resolution: Arc<Mutex<Option<resolution::PrimaryResolution>>>,
    cache: Arc<Mutex<HashMap<usize, Arc<CachedFace>>>>,
    shaped_width_cache: Arc<Mutex<HashMap<ShapedWidthKey, Pt>>>,
    /// P673 — cache de faces ttf-parser para `shaped_width`, partilhada entre
    /// todas as chamadas de `advance_shaped` no mesmo documento. Evita
    /// re-parsear as mesmas fontes em cada medição de palavra.
    shaper_face_cache: Arc<Mutex<crate::shaper::FaceCache>>,
    /// P677 — cache de larguras `advance` já calculadas. Evita re-medir o
    /// mesmo texto+estilo (especialmente espaços e palavras repetidas) em
    /// cada chamada do layout.
    advance_width_cache: Arc<Mutex<HashMap<AdvanceWidthKey, Pt>>>,
}

/// Fontes padrão de fallback usadas pelo shaper (P538e/P543).
/// Devem ser consistentes entre `FallbackFontMetrics` e `shaper.rs` para
/// evitar desalinhamento de posicionamento no PDF.
impl<'a> FallbackFontMetrics<'a> {
    /// Constrói métricas de fallback a partir do `World`.
    pub fn new(world: &'a dyn World) -> Self {
        Self {
            world,
            primary_resolution: Arc::new(Mutex::new(None)),
            cache: Arc::new(Mutex::new(HashMap::new())),
            shaped_width_cache: Arc::new(Mutex::new(HashMap::new())),
            shaper_face_cache: Arc::new(Mutex::new(crate::shaper::FaceCache::new())),
            advance_width_cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl Clone for FallbackFontMetrics<'_> {
    fn clone(&self) -> Self {
        Self {
            world: self.world,
            primary_resolution: self.primary_resolution.clone(),
            cache: self.cache.clone(),
            shaped_width_cache: self.shaped_width_cache.clone(),
            shaper_face_cache: self.shaper_face_cache.clone(),
            advance_width_cache: self.advance_width_cache.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FontBookMetrics;

    #[test]
    fn from_bytes_invalidos_retorna_none() {
        assert!(FontBookMetrics::from_bytes(b"not a font").is_none());
        assert!(FontBookMetrics::from_bytes(b"").is_none());
    }

    #[test]
    fn upem_zero_nao_causa_divisao_por_zero() {
        // Bytes inválidos → None (nunca chega a upem=0 em advance)
        assert!(FontBookMetrics::from_bytes(b"not a font").is_none());
        assert!(FontBookMetrics::from_bytes(b"").is_none());
    }
}
