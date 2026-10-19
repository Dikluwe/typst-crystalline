//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/pipeline/raster.md
//! @prompt-hash 6ab85489
//! @layer L3
//! @updated 2026-09-26
//!
//! Possui as fachadas PNG: compila pelo caminho paginado comum, resolve PPI documental/override uma vez, converte por 72, resolve fontes e exporta a primeira página com o fallback vazio vigente. Conserva erros, warnings e timings.

use super::*;

/// Pipeline completo `Source` → bytes PNG (primeira página).
pub fn compile_to_png_bytes(
    world: &dyn World,
    source: &Source,
) -> (Result<Vec<u8>, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>) {
    compile_to_png_bytes_full_error(world, source, false)
}

/// Variante internal com `full_error`.
#[doc(hidden)]
pub fn compile_to_png_bytes_full_error(
    world: &dyn World,
    source: &Source,
    full_error: bool,
) -> (Result<Vec<u8>, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>) {
    let (result, warnings, _) =
        compile_to_png_bytes_with_timings_full_error(world, source, full_error);
    (result, warnings)
}

/// Variante instrumentada de `compile_to_png_bytes`.
#[doc(hidden)]
pub fn compile_to_png_bytes_with_timings_full_error(
    world: &dyn World,
    source: &Source,
    full_error: bool,
) -> (Result<Vec<u8>, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>, Timings) {
    compile_to_png_bytes_with_timings_full_error_and_features(
        world,
        source,
        full_error,
        typst_core::entities::compiler_features::Features::default(),
    )
}

#[doc(hidden)]
pub fn compile_to_png_bytes_with_timings_full_error_and_features(
    world: &dyn World,
    source: &Source,
    full_error: bool,
    features: typst_core::entities::compiler_features::Features,
) -> (Result<Vec<u8>, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>, Timings) {
    compile_to_png_bytes_with_timings_full_error_and_features_and_ppi_override(
        world, source, full_error, features, None,
    )
}

/// Variante PNG que preserva a ausência de override até consultar as opções
/// resolvidas do documento.
#[doc(hidden)]
pub fn compile_to_png_bytes_with_timings_full_error_and_features_and_ppi_override(
    world: &dyn World,
    source: &Source,
    full_error: bool,
    features: typst_core::entities::compiler_features::Features,
    ppi_override: Option<f64>,
) -> (Result<Vec<u8>, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>, Timings) {
    let mut timings = Timings::default();
    let (doc_result, warnings) = compile_to_paged_document_full_error_and_features(
        world,
        source,
        full_error,
        features,
        &mut timings,
    );
    let result = match doc_result {
        Ok(doc) => {
            let effective_ppi = ppi_override.unwrap_or(doc.format_options.png.ppi);
            let pixel_per_pt = (effective_ppi / 72.0) as f32;
            let render_options =
                crate::export::RenderOptions { pixel_per_pt, render_bleed: false };
            let fonts = match resolve_export_fonts(&doc, world) {
                Ok(f) => f,
                Err(diag) => {
                    timings.total_ms = timings.eval_ms
                        + timings.introspect_ms
                        + timings.expand_context_ms
                        + timings.layout_ms
                        + timings.shape_ms;
                    return (Err(vec![diag]), warnings, timings);
                }
            };
            let t_render = Instant::now();
            let page = doc.pages.first().cloned().unwrap_or_else(|| Page {
                width: 0.0,
                height: 0.0,
                numbering: None,
                logical_number: 1,
                supplement: typst_core::entities::content::Content::Empty,
                bleed: Default::default(),
                fill: Default::default(),
                background: vec![],
                foreground: vec![],
                items: vec![],
            });
            let png = if fonts.is_empty() {
                export_png(&page, &render_options)
            } else {
                export_png_with_fonts(&page, &render_options, &fonts)
            };
            timings.render_ms = duration_ms(Instant::now().duration_since(t_render));
            timings.total_ms = timings.eval_ms
                + timings.introspect_ms
                + timings.expand_context_ms
                + timings.layout_ms
                + timings.shape_ms
                + timings.render_ms;
            Ok(png)
        }
        Err(errors) => {
            timings.total_ms = timings.eval_ms
                + timings.introspect_ms
                + timings.expand_context_ms
                + timings.layout_ms
                + timings.shape_ms;
            Err(errors)
        }
    };
    (result, warnings, timings)
}
