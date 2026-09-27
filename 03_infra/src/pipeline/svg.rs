//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/pipeline/svg.md
//! @prompt-hash 578981e3
//! @layer L3
//! @updated 2026-09-26
//!
//! Possui as fachadas SVG, o contexto de destinos por página e a associação de fontes de glifos. Compila pelo caminho paginado comum, conserva pretty, primeira página, identidade de fontes e Unknown nos casos sem associação comprovada.

use super::*;

/// Pipeline completo `Source` → string SVG (primeira página).
pub fn compile_to_svg_string(
    world: &dyn World,
    source: &Source,
) -> (Result<String, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>) {
    compile_to_svg_string_full_error(world, source, false)
}

/// Variante internal com `full_error`.
#[doc(hidden)]
pub fn compile_to_svg_string_full_error(
    world: &dyn World,
    source: &Source,
    full_error: bool,
) -> (Result<String, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>) {
    let (result, warnings, _) =
        compile_to_svg_string_with_timings_full_error(world, source, full_error);
    (result, warnings)
}

/// Variante instrumentada de `compile_to_svg_string`.
#[doc(hidden)]
pub fn compile_to_svg_string_with_timings_full_error(
    world: &dyn World,
    source: &Source,
    full_error: bool,
) -> (Result<String, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>, Timings) {
    compile_to_svg_string_with_timings_full_error_and_features(
        world,
        source,
        full_error,
        typst_core::entities::compiler_features::Features::default(),
    )
}

#[doc(hidden)]
pub fn compile_to_svg_string_with_timings_full_error_and_features(
    world: &dyn World,
    source: &Source,
    full_error: bool,
    features: typst_core::entities::compiler_features::Features,
) -> (Result<String, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>, Timings) {
    compile_to_svg_string_with_timings_full_error_and_features_and_pretty_override(
        world, source, full_error, features, None,
    )
}

#[doc(hidden)]
pub fn compile_to_svg_string_with_timings_full_error_and_features_and_pretty(
    world: &dyn World,
    source: &Source,
    full_error: bool,
    features: typst_core::entities::compiler_features::Features,
    pretty: bool,
) -> (Result<String, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>, Timings) {
    compile_to_svg_string_with_timings_full_error_and_features_and_pretty_override(
        world,
        source,
        full_error,
        features,
        Some(pretty),
    )
}

fn resolve_svg_pretty(pretty_override: Option<bool>, document: &PagedDocument) -> bool {
    pretty_override.unwrap_or(document.format_options.svg.pretty)
}

/// Variante SVG que preserva a ausência de override até consultar as
/// opções resolvidas do documento.
#[doc(hidden)]
pub fn compile_to_svg_string_with_timings_full_error_and_features_and_pretty_override(
    world: &dyn World,
    source: &Source,
    full_error: bool,
    features: typst_core::entities::compiler_features::Features,
    pretty_override: Option<bool>,
) -> (Result<String, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>, Timings) {
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
            let pretty = resolve_svg_pretty(pretty_override, &doc);
            let fonts = match resolve_and_instantiate_fonts(&doc, world) {
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
            let destinations = svg_destination_context_for_page(&doc, 1);
            let glyph_fonts = svg_glyph_font_context_for_page(&page, world);
            let options = crate::export::SvgOptions { pretty, ..Default::default() };
            let svg = if fonts.is_empty() {
                export_svg_with_context(&page, &options, &destinations)
            } else {
                export_svg_with_fonts_and_contexts(
                    &page,
                    &options,
                    &fonts,
                    &destinations,
                    &glyph_fonts,
                )
            };
            timings.render_ms = duration_ms(Instant::now().duration_since(t_render));
            timings.total_ms = timings.eval_ms
                + timings.introspect_ms
                + timings.expand_context_ms
                + timings.layout_ms
                + timings.shape_ms
                + timings.render_ms;
            Ok(svg)
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

#[cfg(test)]
mod p1462_tests {
    use super::*;

    #[test]
    fn svg_pretty_precedence_is_override_then_document_then_false() {
        let mut document = PagedDocument::new(vec![]);
        assert!(!resolve_svg_pretty(None, &document));

        document.format_options.svg.pretty = true;
        assert!(resolve_svg_pretty(None, &document));
        assert!(!resolve_svg_pretty(Some(false), &document));

        document.format_options.svg.pretty = false;
        assert!(resolve_svg_pretty(Some(true), &document));
    }
}

pub(super) fn svg_destination_context_for_page(
    doc: &PagedDocument,
    selected_page_number: usize,
) -> SvgDestinationContext {
    SvgDestinationContext::from_destinations(
        doc.extracted_label_pages
            .iter()
            .filter(|(_, page)| **page == selected_page_number)
            .filter_map(|(label, _)| {
                doc.extracted_label_positions
                    .get(label)
                    .copied()
                    .map(|point| (label.clone(), point))
            }),
    )
}

pub(super) fn svg_glyph_font_context_for_page(
    page: &Page,
    world: &dyn World,
) -> SvgGlyphFontContext {
    fn collect(
        items: &[FrameItem],
        metrics: &FallbackFontMetrics<'_>,
        out: &mut Vec<(GlyphFontRequest, (FontList, FontVariant, FontVariations))>,
    ) {
        for item in items {
            match item {
                FrameItem::Glyph { style, base_char, .. } => {
                    if let Some(identity) = metrics.resolve_font_combo(*base_char, style)
                    {
                        out.push((GlyphFontRequest::new(*base_char, style), identity));
                    }
                }
                FrameItem::Group { items, .. }
                | FrameItem::Link { items, .. }
                | FrameItem::Semantic { items, .. } => collect(items, metrics, out),
                _ => {}
            }
        }
    }
    let metrics = FallbackFontMetrics::new(world);
    let mut resolved = Vec::new();
    collect(&page.background, &metrics, &mut resolved);
    collect(&page.items, &metrics, &mut resolved);
    collect(&page.foreground, &metrics, &mut resolved);
    SvgGlyphFontContext::from_resolutions(resolved)
}
