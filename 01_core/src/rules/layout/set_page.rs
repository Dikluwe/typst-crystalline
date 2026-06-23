//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/layout.md
//! @prompt-hash 12536b5c
//! @layer L1
//! @updated 2026-06-23
//!
//! Layout de `Content::SetPage` — aplica nova configuração de página.
//! Extraído de `layout/mod.rs` no P425 (ADR-0109 forma B).

use crate::entities::{
    image_sizer::ImageSizer,
    layout_types::Pt,
};

use super::metrics::FontMetrics;
use super::Layouter;

/// Aplica uma nova configuração de página (largura, altura, margem) e
/// força nova página se a configuração mudou.
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<'_, M, S>,
    width:    &Option<f64>,
    height:   &Option<f64>,
    margin:   &Option<f64>,
) {
    let mut new_config = layouter.page_config.clone();
    let mut changed = false;

    if let Some(w) = width {
        new_config.width = *w;
        changed = true;
    }
    if let Some(h) = height {
        new_config.height = *h;
        changed = true;
    }
    if let Some(m) = margin {
        new_config.margin = *m;
        changed = true;
    }

    if changed {
        if !layouter.current_page_is_empty() {
            layouter.flush_line();
            layouter.new_page();
        }
        layouter.page_config = new_config;
        // P216A: sincronizar region.width/height com PageConfig
        // (Caminho B1 — redundância controlada).
        layouter.regions.current.width = layouter.page_config.width;
        layouter.regions.current.height = layouter.page_config.height;
        layouter.regions.current.cursor_x = Pt(layouter.page_config.margin);
        layouter.regions.current.cursor_y = Pt(layouter.page_config.margin);
        layouter.regions.current.line_start_x = Pt(layouter.page_config.margin);
        // DEBT-35b: se available_width() vier a ter cache, invalidar aqui.
    }
}
