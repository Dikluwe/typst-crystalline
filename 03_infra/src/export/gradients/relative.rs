//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/gradients/relative.md
//! @prompt-hash 135808e0
//! @layer L3
//! @updated 2026-05-19
//!
//! Gradient `relative: RelativeTo` — resolve relative + apply parent transform (P273).
//!
//! Extraído de `gradients.rs` em P307b.2 (ADR-0100 / diagnóstico
//! P307a §5). Conteúdo bit-exact pré e pós migração.

use typst_core::entities::gradient::RelativeTo;

// ── P273 — Gradient `relative: RelativeTo` cross-variant ──
//
// Resolve `Option<RelativeTo>` (None = Auto) para `RelativeTo` (default
// `Self_`). Paridade vanilla `unwrap_or_else(|| Self_)` simplificada
// (cristalino ignora `on_text` contexto; materializável futuro).
//
// `apply_parent_transform` é estrutural — quando `parent_bbox = None`,
// retorna coords inalteradas (preservando branch `Self_`). Quando
// `parent_bbox = Some(bbox)`, escala unit-space [0, 1] coords para
// bbox real. Callsites P273 passam `None` (estrutural; futuro callsite
// preenche bbox real).
//
// Paridade vanilla `correct_transform` em `lab/typst-original/crates/typst-pdf/src/paint.rs:383+`
// (transform Rust nativo; PDF /Matrix permanece identity).

/// P273 — Resolve `Option<RelativeTo>` (Auto = None) para concreto.
pub(crate) fn resolve_relative(
    relative: Option<typst_core::entities::gradient::RelativeTo>,
) -> typst_core::entities::gradient::RelativeTo {
    relative.unwrap_or_default()
}

/// P273 — Aplica transform parent bbox a coordenadas unit-space.
///
/// `local` em unit-space [0, 1]; `parent_bbox = Some((x0, y0, x1, y1))`
/// escala para bbox real. `parent_bbox = None` retorna `local`
/// inalterado (identity transform; preserva pipeline P272 quando
/// callsite não passa parent context).
///
/// **P273.5**: `#[allow(dead_code)]` removido — função tem callsite
/// L3 real em `emit_gradient_objects` (dispatcher Linear/Radial
/// RGB-family arm) quando `gradient.relative == Some(Parent)` passa
/// `page_bbox` como fallback (decisão 3γ.1 híbrida ADR-0091
/// §"Anotação cumulativa P273.5").
pub(crate) fn apply_parent_transform(
    local: (f32, f32, f32, f32),
    parent_bbox: Option<(f32, f32, f32, f32)>,
) -> (f32, f32, f32, f32) {
    match parent_bbox {
        Some((px0, py0, px1, py1)) => {
            let dx = px1 - px0;
            let dy = py1 - py0;
            (
                px0 + local.0 * dx,
                py0 + local.1 * dy,
                px0 + local.2 * dx,
                py0 + local.3 * dy,
            )
        }
        None => local,
    }
}

