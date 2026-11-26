//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/pipeline/raster.md
//! @prompt-hash 3ce09a59
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

/// Every selected page as its own PNG, named by the page template rule
/// shared with SVG. Writes neither files nor stdout.
pub fn compile_to_png_output_files(
    world: &dyn World,
    source: &Source,
    full_error: bool,
    features: typst_core::entities::compiler_features::Features,
    ppi_override: Option<f64>,
    pages_override: Option<typst_core::entities::document_format_options::PageRanges>,
    output: &std::path::Path,
) -> (
    Result<Vec<(std::path::PathBuf, Vec<u8>)>, Vec<SourceDiagnostic>>,
    Vec<SourceDiagnostic>,
    Timings,
) {
    let mut timings = Timings::default();
    let (document, warnings) = compile_to_paged_document_full_error_and_features(
        world,
        source,
        full_error,
        features,
        &mut timings,
    );
    let result = document.and_then(|document| {
        let selected: Vec<_> = document
            .pages
            .iter()
            .enumerate()
            .filter(|(index, _)| {
                pages_override.as_ref().is_none_or(|pages| {
                    pages.includes_page(std::num::NonZeroUsize::new(index + 1).unwrap())
                })
            })
            .collect();
        let indexable = page_template(output, selected.len())?;
        if selected.is_empty() {
            return Ok(Vec::new());
        }
        let effective_ppi = ppi_override.unwrap_or(document.format_options.png.ppi);
        let render_options = crate::export::RenderOptions {
            pixel_per_pt: (effective_ppi / 72.0) as f32,
            render_bleed: false,
        };
        let fonts = resolve_export_fonts(&document, world).map_err(|error| vec![error])?;
        let start = Instant::now();
        let total = document.pages.len();
        let outputs = selected
            .into_iter()
            .map(|(index, page)| {
                let png = if fonts.is_empty() {
                    export_png(page, &render_options)
                } else {
                    export_png_with_fonts(page, &render_options, &fonts)
                };
                (page_destination(output, indexable, index + 1, total), png)
            })
            .collect();
        timings.render_ms = duration_ms(start.elapsed());
        Ok(outputs)
    });
    timings.total_ms = timings.eval_ms
        + timings.introspect_ms
        + timings.expand_context_ms
        + timings.layout_ms
        + timings.shape_ms
        + timings.render_ms;
    (result, warnings, timings)
}

#[cfg(test)]
mod output_files_tests {
    use super::*;
    use std::path::Path;

    const PROGRAM: &str = "#set page(width:60pt,height:40pt,margin:4pt)\n#set text(font:\"DejaVu Sans\",size:11pt)\nA\n#pagebreak()\nB\n";

    fn world() -> crate::world::SystemWorld {
        crate::world::SystemWorld::for_stdin(std::env::temp_dir(), PROGRAM.into())
            .with_fonts(vec![crate::fonts::FontSlot::new_embedded(
                "<DejaVu PNG output fixture>".into(),
                include_bytes!("../../fixtures/fonts/DejaVuSans.ttf").to_vec(),
            )])
    }

    #[test]
    fn the_first_page_keeps_the_bytes_of_the_first_page_facade() {
        let world = world();
        let source = world.source(world.main()).unwrap();
        let (files, warnings, _) = compile_to_png_output_files(
            &world,
            &source,
            false,
            Default::default(),
            None,
            None,
            Path::new("out-{p}.png"),
        );
        assert!(warnings.is_empty(), "{warnings:?}");
        let files = files.unwrap();
        assert_eq!(
            files.iter().map(|(path, _)| path.clone()).collect::<Vec<_>>(),
            [Path::new("out-1.png"), Path::new("out-2.png")]
        );
        let (legacy, _, _) = compile_to_png_bytes_with_timings_full_error_and_features_and_ppi_override(
            &world,
            &source,
            false,
            Default::default(),
            None,
        );
        assert_eq!(files[0].1, legacy.unwrap());
        assert_ne!(files[0].1, files[1].1);
    }
}
