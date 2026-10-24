//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/pipeline/svg.md
//! @prompt-hash e830c9f9
//! @layer L3
//! @updated 2026-09-28
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

type SvgOutputFilesResult =
    Result<Vec<(std::path::PathBuf, Vec<u8>)>, Vec<SourceDiagnostic>>;

/// Compile all selected direct SVG outputs without publishing files or stdout.
/// Existing first-page string facades retain their independent contracts.
pub fn compile_to_svg_output_files(
    world: &dyn World,
    source: &Source,
    full_error: bool,
    features: typst_core::entities::compiler_features::Features,
    pretty_override: Option<bool>,
    pages_override: Option<typst_core::entities::document_format_options::PageRanges>,
    output: &std::path::Path,
) -> (SvgOutputFilesResult, Vec<SourceDiagnostic>, Timings) {
    let mut timings = Timings::default();
    let (document, warnings) = compile_to_paged_document_full_error_and_features(
        world,
        source,
        full_error,
        features,
        &mut timings,
    );
    let result = document.and_then(|document| {
        let selected: Vec<_> = document.pages.iter().enumerate().filter(|(index, _)| {
            pages_override.as_ref().is_none_or(|pages| {
                pages.includes_page(std::num::NonZeroUsize::new(index + 1).unwrap())
            })
        }).collect();
        let template = output.to_str().unwrap_or_default();
        let indexable = ["{p}", "{0p}", "{n}"].iter().any(|token| template.contains(token));
        if selected.len() > 1 && !indexable {
            let message = if output == std::path::Path::new("-") {
                "cannot export multiple images to stdout"
            } else {
                "cannot export multiple images without a page number template ({p}, {0p}) in the output path"
            };
            return Err(vec![SourceDiagnostic::error(Span::detached(), message)]);
        }
        if selected.is_empty() { return Ok(Vec::new()); }
        let fonts = resolve_export_fonts(&document, world).map_err(|error| vec![error])?;
        let options = crate::export::SvgOptions {
            pretty: resolve_svg_pretty(pretty_override, &document),
            ..Default::default()
        };
        let start = Instant::now();
        let total = document.pages.len();
        let width = 1 + total.checked_ilog10().unwrap_or(0) as usize;
        let outputs = selected.into_iter().map(|(index, page)| {
            let physical = index + 1;
            let destination = if indexable {
                std::path::PathBuf::from(template
                    .replace("{p}", &physical.to_string())
                    .replace("{0p}", &format!("{physical:0width$}"))
                    .replace("{n}", &format!("{physical:0width$}"))
                    .replace("{t}", &total.to_string()))
            } else { output.to_owned() };
            let destinations = svg_destination_context_for_page(&document, physical);
            let glyph_fonts = svg_glyph_font_context_for_page(page, world);
            let svg = if fonts.is_empty() {
                export_svg_with_context(page, &options, &destinations)
            } else {
                export_svg_with_fonts_and_contexts(page, &options, &fonts, &destinations, &glyph_fonts)
            };
            (destination, svg.into_bytes())
        }).collect();
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
                | FrameItem::TextReplacement { items, .. }
                | FrameItem::Semantic { items, .. } => collect(items, metrics, out),
                FrameItem::TextSeparator { .. }
                | FrameItem::Text { .. }
                | FrameItem::TextShaped { .. }
                | FrameItem::Line { .. }
                | FrameItem::Image { .. }
                | FrameItem::Shape { .. } => {}
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

#[cfg(test)]
mod remaining_svg_output_tests {
    use super::*;
    use std::num::NonZeroUsize;
    use std::path::Path;
    use typst_core::entities::document_format_options::{PageRange, PageRanges};
    const PROGRAM: &str = "#set page(width:180pt,height:72pt,margin:8pt)\n#set text(font:\"DejaVu Sans\",size:11pt)\nBEFORE\n\n#box(width:1fr,height:40pt)[LEFT#h(1fr)RIGHT]TAIL\n";
    fn world() -> crate::world::SystemWorld {
        crate::world::SystemWorld::for_stdin(std::env::temp_dir(), PROGRAM.into())
            .with_fonts(vec![crate::fonts::FontSlot::new_embedded(
                "<DejaVu SVG output fixture>".into(),
                include_bytes!("../../fixtures/fonts/DejaVuSans.ttf").to_vec(),
            )])
    }
    fn outputs(
        path: &str,
        page: Option<usize>,
    ) -> SourceResult<Vec<(std::path::PathBuf, Vec<u8>)>> {
        let world = world();
        let source = world.source(world.main()).unwrap();
        let pages = page
            .map(|p| PageRanges::new(vec![PageRange::at(NonZeroUsize::new(p).unwrap())]));
        let (result, warnings, _) = compile_to_svg_output_files(
            &world,
            &source,
            false,
            Default::default(),
            None,
            pages,
            Path::new(path),
        );
        assert!(warnings.is_empty(), "{warnings:?}");
        result
    }
    #[test]
    fn remaining_svg_outputs_include_both_actual_pages_and_expand_paths() {
        let all = outputs("out-{p}-{0p}-{n}-{t}-{p}.svg", None).unwrap();
        assert_eq!(all.len(), 2);
        assert_eq!(all[0].0, Path::new("out-1-1-1-2-1.svg"));
        assert_eq!(all[1].0, Path::new("out-2-2-2-2-2.svg"));
        assert_ne!(all[0].1, all[1].1, "actual pages contain different source text");
        let selected = outputs("out.svg", Some(2)).unwrap();
        assert_eq!(selected.len(), 1);
        assert_eq!(selected[0].1, all[1].1, "selection must use physical page2");
        assert_eq!(outputs("-", Some(2)).unwrap()[0].1, all[1].1);
    }
    #[test]
    fn remaining_svg_outputs_reject_ambiguous_destinations_and_preserve_empty_selection()
    {
        for (path, message) in [
            ("out.svg", "cannot export multiple images without a page number template ({p}, {0p}) in the output path"),
            ("out-{t}.svg", "cannot export multiple images without a page number template ({p}, {0p}) in the output path"),
            ("-", "cannot export multiple images to stdout"),
        ] {
            let errors = outputs(path, None).expect_err("multiple outputs need a valid destination");
            assert_eq!(errors.len(), 1);
            assert_eq!(errors[0].message, message);
        }
        assert!(outputs("out.svg", Some(999)).unwrap().is_empty());
    }
    #[test]
    fn remaining_svg_outputs_padding_uses_total_and_keeps_physical_indices() {
        let mut program = String::from("#set page(width:100pt,height:100pt,margin:8pt)\n#set text(font:\"DejaVu Sans\",size:11pt)\n");
        program.push_str(
            &(1..=12)
                .map(|p| format!("P{p}"))
                .collect::<Vec<_>>()
                .join("#pagebreak()"),
        );
        program.push('\n');
        let world = crate::world::SystemWorld::for_stdin(std::env::temp_dir(), program)
            .with_fonts(vec![crate::fonts::FontSlot::new_embedded(
                "<DejaVu SVG padding fixture>".into(),
                include_bytes!("../../fixtures/fonts/DejaVuSans.ttf").to_vec(),
            )]);
        let source = world.source(world.main()).unwrap();
        let pages = PageRanges::new(vec![
            PageRange::at(NonZeroUsize::new(2).unwrap()),
            PageRange::at(NonZeroUsize::new(12).unwrap()),
        ]);
        let (result, warnings, _) = compile_to_svg_output_files(
            &world,
            &source,
            false,
            Default::default(),
            Some(true),
            Some(pages),
            Path::new("out-{0p}-{n}-{p}-{t}.svg"),
        );
        assert!(warnings.is_empty());
        let result = result.unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].0, Path::new("out-02-02-2-12.svg"));
        assert_eq!(result[1].0, Path::new("out-12-12-12-12.svg"));
        assert!(result.iter().all(|(_, bytes)| bytes.contains(&b'\n')));
    }

    #[test]
    fn remaining_svg_outputs_leave_total_only_single_path_literal() {
        let outputs = outputs("out-{t}.svg", Some(1)).unwrap();
        assert_eq!(outputs.len(), 1);
        assert_eq!(outputs[0].0, Path::new("out-{t}.svg"));
    }

    #[cfg(unix)]
    #[test]
    fn remaining_svg_outputs_preserve_non_utf8_single_destination() {
        use std::os::unix::ffi::OsStringExt;
        let path = std::path::PathBuf::from(std::ffi::OsString::from_vec(
            b"out-\xff.svg".to_vec(),
        ));
        let world = world();
        let source = world.source(world.main()).unwrap();
        let pages = PageRanges::new(vec![PageRange::at(NonZeroUsize::new(1).unwrap())]);
        let (result, warnings, _) = compile_to_svg_output_files(
            &world,
            &source,
            false,
            Default::default(),
            None,
            Some(pages),
            &path,
        );
        assert!(warnings.is_empty());
        assert_eq!(result.unwrap()[0].0, path);
    }

    #[test]
    fn remaining_svg_outputs_preserve_first_page_string_api() {
        let world = world();
        let source = world.source(world.main()).unwrap();
        let (legacy, warnings) = compile_to_svg_string(&world, &source);
        assert!(warnings.is_empty());
        let selected = outputs("out.svg", Some(1)).unwrap();
        assert_eq!(selected[0].1, legacy.unwrap().into_bytes());
    }
}
