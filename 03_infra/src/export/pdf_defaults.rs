//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/pdf_defaults.md
//! @prompt-hash b98436c5
//! @layer L3
//! @updated 2026-08-17
//!
//! Constantes canónicas de serialização PDF e padrões geométricos de exportação (Passo 1069).

/// Constante mágica para aproximação de arcos circulares e elípticos por curvas Bézier cúbicas: 4 * (sqrt(2) - 1) / 3.
/// ref: ISO 32000-1:2008 (PDF 1.7 Spec) §4.4 / Adobe PostScript Language Reference Manual
pub const BEZIER_CIRCLE_KAPPA: f64 = 0.552_284_749_831;

/// Proporção do tamanho da fonte utilizada para o offset de traço em negrito sintético (faux bold).
/// ref: lab/typst-original/crates/typst-pdf/src/text.rs
pub const FAUX_BOLD_K: f64 = 0.04;

/// Largura canónica precisa de página A4 em pontos tipográficos (72 pt/in, 210mm).
/// ref: ISO 216 / Adobe PostScript Paper Sizes
pub const A4_DEFAULT_WIDTH: f64 =
    typst_core::entities::page_geometry::Paper::A4.width_pt();

/// Altura canónica precisa de página A4 em pontos tipográficos (72 pt/in, 297mm).
/// ref: ISO 216 / Adobe PostScript Paper Sizes
pub const A4_DEFAULT_HEIGHT: f64 =
    typst_core::entities::page_geometry::Paper::A4.height_pt();

/// Largura arredondada de página A4 utilizada em rotas de fallback de metadados quando dimensões estão ausentes.
/// Preserva compatibilidade numérica exacta com os pontos de unwrap_or existentes.
pub const A4_FALLBACK_WIDTH_ROUNDED: f64 = 595.0;

/// Altura arredondada de página A4 utilizada em rotas de fallback de conversão de coordenadas Y quando páginas estão ausentes.
/// Preserva compatibilidade numérica exacta com os pontos de unwrap_or existentes.
pub const A4_FALLBACK_HEIGHT_ROUNDED: f64 = 842.0;
