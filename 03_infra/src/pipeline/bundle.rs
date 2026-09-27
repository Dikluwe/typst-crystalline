//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/pipeline/bundle.md
//! @prompt-hash 1e98fdf7
//! @layer L3
//! @updated 2026-09-27
//!
//! Orquestra o target bundle: entradas ordenadas, paths e colisões, snapshots globais, fronteiras Page por body e exportação por formato. Devolve a coleção completa ou erro; o write continua responsabilidade de L4. Reutiliza helpers de paged, html, pdf, svg e resources sem reconstruir snapshots isolados.

use super::*;

/// Executa a variante runtime do bundle preservando as fronteiras Page dos
/// bodies; usa um único Engine e devolve os seeds do mesmo walk.
pub(super) fn introspect_bundle_with_runtime_for_pipeline(
    bodies: &[Content],
    world: &dyn World,
    source: &Source,
) -> (
    SourceResult<(
        typst_core::entities::introspector::TagIntrospector,
        Vec<typst_core::entities::locator::LocatorSeed>,
    )>,
    Vec<SourceDiagnostic>,
) {
    let font_metrics = FallbackFontMetrics::new(world);
    let mut ctx = EvalContext::new();
    let mut styles = StyleChain::default_chain();
    let mut show_rules: Arc<[ShowRule]> = Arc::from([]);
    let mut active_guards = Vec::new();
    let mut sink = TypstSink::new();
    let route = Route::root().with_id(source.id());
    let result = {
        let mut tracked_sink = sink.track_mut();
        let mut local_sink = TrackedMut::reborrow_mut(&mut tracked_sink);
        let mut engine = Engine {
            world,
            font_metrics: &font_metrics,
            route: route.track(),
            styles: &mut styles,
            show_rules: &mut show_rules,
            active_guards: &mut active_guards,
            current_file: source.id(),
            sink: &mut local_sink,
        };
        typst_core::compiler::introspect::introspect_bundle_documents_with_runtime_and_starts(
            bodies,
            &mut engine,
            &mut ctx,
        )
    };
    (result, sink.into_diagnostics())
}

/// One validated file emitted by the bundle target.
#[derive(Debug, Clone)]
pub struct BundleFile {
    pub path: String,
    pub data: Vec<u8>,
    pub is_html_document: bool,
}

/// Explicit bundle options retain each CLI override until document export.
#[derive(Debug, Clone)]
pub struct BundleCompileOptions {
    pub full_error: bool,
    pub features: typst_core::entities::compiler_features::Features,
    pub document_id: Option<[u8; 16]>,
    pub pdf_stream_mode: StreamMode,
    pub pdf_serialization: PdfSerializationMode,
    pub pdf_tagged_override: Option<bool>,
    pub pdf_pages_override: Option<PageRanges>,
    pub pdf_standard_override: Option<PdfStandards>,
    pub png_ppi_override: Option<f64>,
    pub html_serialization: crate::export::HtmlSerializationMode,
    pub pretty_override: Option<bool>,
}

pub(super) fn bundle_error(message: impl Into<String>) -> Vec<SourceDiagnostic> {
    vec![SourceDiagnostic::error(Span::detached(), message.into())]
}

/// Evaluate once in the bundle target, validate the full virtual tree, then export.
pub fn compile_to_bundle_files(
    world: &dyn World,
    source: &Source,
    options: BundleCompileOptions,
) -> (Result<Vec<BundleFile>, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>, Timings) {
    use typst_core::entities::compiler_features::Feature;
    let mut timings = Timings::default();
    if !options.features.contains(Feature::Bundle) {
        return (
            Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "bundle export is only available when --features bundle is passed",
            )
            .with_hint("bundle export is experimental")]),
            Vec::new(),
            timings,
        );
    }
    let t0 = Instant::now();
    let (evaluated, mut warnings) = eval_to_module_with_sink_target_features(
        world,
        source,
        options.full_error,
        EvalTarget::Bundle,
        options.features,
    );
    timings.eval_ms = duration_ms(t0.elapsed());
    let module = match evaluated {
        Ok(module) => module,
        Err(errors) => return (Err(errors), warnings, timings),
    };
    let root = module.content().cloned().unwrap_or(Content::Empty);
    let mut entries = Vec::new();
    if let Err(errors) = collect_bundle_entries(&root, &mut entries) {
        return (Err(errors), warnings, timings);
    }
    let mut paths = HashSet::new();
    for entry in &entries {
        let path = match entry {
            Content::Document { path, .. } | Content::Asset { path, .. } => path,
            _ => unreachable!(),
        };
        if !paths.insert(path.to_string()) {
            return (
                Err(bundle_error(format!(
                    "path \x60{}\x60 occurs multiple times in the bundle",
                    path.trim_start_matches('/'),
                ))),
                warnings,
                timings,
            );
        }
    }
    let document_bodies: Vec<Content> = entries
        .iter()
        .filter_map(|entry| {
            if let Content::Document { body, .. } = entry {
                Some((**body).clone())
            } else {
                None
            }
        })
        .collect();
    let (initial_intr, _) = introspect_bundle_documents_with_starts(&document_bodies);
    let html_targets: Vec<bool> = entries
        .iter()
        .filter_map(|entry| {
            if let Content::Document { path, format, .. } = entry {
                let selected = format.as_deref().unwrap_or_else(|| {
                    path.rsplit_once('.').map(|(_, extension)| extension).unwrap_or("")
                });
                Some(selected == "html")
            } else {
                None
            }
        })
        .collect();
    let (prepared, contextual_warnings) = context_stabilization::prepare_bundle(
        document_bodies,
        &html_targets,
        &initial_intr,
        world,
        source,
        options.full_error,
        options.features,
    );
    warnings.extend(contextual_warnings);
    let (prepared_bodies, bundle_intr, document_starts) = match prepared {
        Ok(prepared) => prepared,
        Err(errors) => return (Err(errors), warnings, timings),
    };
    let mut document_index = 0;
    let mut page_offset = 0usize;
    let mut files = Vec::with_capacity(entries.len());
    for entry in entries {
        match entry {
            Content::Asset { path, data } => files.push(BundleFile {
                path: path.trim_start_matches('/').to_owned(),
                data: data.to_vec(),
                is_html_document: false,
            }),
            Content::Document { path, format, .. } => {
                let selected = format.as_deref().unwrap_or_else(|| {
                    path.rsplit_once('.').map(|(_, extension)| extension).unwrap_or("")
                });
                let location_start = document_starts[document_index];
                let body = &prepared_bodies[document_index];
                document_index += 1;
                let rendered = render_bundle_document(
                    world,
                    source,
                    body,
                    &bundle_intr,
                    location_start,
                    page_offset,
                    selected,
                    &options,
                    &mut warnings,
                );
                let (data, is_html_document, page_count) = match rendered {
                    Ok(output) => output,
                    Err(errors) => return (Err(errors), warnings, timings),
                };
                page_offset = match page_offset.checked_add(page_count) {
                    Some(next) => next,
                    None => {
                        return (
                            Err(bundle_error("page offset overflow")),
                            warnings,
                            timings,
                        )
                    }
                };
                files.push(BundleFile {
                    path: path.trim_start_matches('/').to_owned(),
                    data,
                    is_html_document,
                });
            }
            _ => unreachable!(),
        }
    }
    let realized_pages =
        bundle_intr.page_store.total_pages().map(|n| n.get()).unwrap_or(0);
    if page_offset != realized_pages {
        return (
            Err(bundle_error("bundle page store does not match rendered documents")),
            warnings,
            timings,
        );
    }
    timings.total_ms = duration_ms(t0.elapsed());
    (Ok(files), warnings, timings)
}

pub(super) fn collect_bundle_entries(
    content: &Content,
    entries: &mut Vec<Content>,
) -> SourceResult<()> {
    match content {
        Content::Sequence(children) => {
            for child in children.iter() {
                collect_bundle_entries(child, entries)?;
            }
            Ok(())
        }
        Content::Styled(styled) => {
            let inner = &styled.body;
            let styles = &styled.styles;
            let start = entries.len();
            collect_bundle_entries(inner, entries)?;
            // A root-level set rule scopes every document beneath it. The
            // bundle carrier is invisible to the layout, so transfer that
            // inherited style to each document body before introspection.
            for entry in &mut entries[start..] {
                if let Content::Document { body, .. } = entry {
                    *body = Box::new(Content::styled(body.clone(), styles.clone()));
                }
            }
            Ok(())
        }
        Content::Empty
        | Content::Space
        | Content::SpaceFromNewline
        | Content::Parbreak
        | Content::Metadata(_) => Ok(()),
        Content::Document { .. } | Content::Asset { .. } => {
            entries.push(content.clone());
            Ok(())
        }
        _ => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!(
                "{} is not allowed at the top-level in bundle export",
                content.elem_name()
            ),
        )
        .with_hint("try wrapping the content in a document instead")]),
    }
}

fn bundle_html_semantic_content(
    content: &Content,
    warnings: &mut Vec<SourceDiagnostic>,
) -> Content {
    use typst_core::entities::html::{HtmlBody, HtmlElem};

    match content {
        // A frame is paginated independently. Its body must not be converted
        // into HTML nodes before the frame resolver sees it.
        Content::HtmlFrame(_) => content.clone(),
        Content::Label(label) => bundle_html_semantic_content(&label.body, warnings),
        Content::Heading(heading) => {
            // Vanilla reserves h1 for the document title.
            let level = heading.level.saturating_add(1);
            let (tag, attrs) = if level <= 6 {
                (format!("h{level}").into(), None)
            } else {
                let mut attrs = typst_core::entities::html::HtmlAttrs::default();
                attrs.insert("role".into(), "heading".into());
                attrs.insert("aria-level".into(), level.to_string().into());
                warnings.push(
                    SourceDiagnostic::warning(
                        Span::detached(),
                        format!("heading of level {} was transformed to <div role=\"heading\" aria-level=\"{}\">, which is not supported by all assistive technology", heading.level, level),
                    ).with_hint(format!("HTML only supports <h1> to <h6>, not <h{}>", level))
                     .with_hint("you may want to restructure your document so that it doesn't contain deep headings"),
                );
                ("div".into(), Some(attrs))
            };
            Content::HtmlElem(Arc::new(HtmlElem::new(
                tag,
                attrs,
                HtmlBody::Content(Box::new(bundle_html_semantic_content(
                    &heading.body,
                    warnings,
                ))),
            )))
        }
        Content::Sequence(items) => Content::sequence(
            items
                .iter()
                .map(|item| bundle_html_semantic_content(item, warnings))
                .collect(),
        ),
        Content::Styled(styled) => Content::styled(
            bundle_html_semantic_content(&styled.body, warnings),
            styled.styles.clone(),
        ),
        Content::Par { body } => Content::Par {
            body: Box::new(bundle_html_semantic_content(body, warnings)),
        },
        Content::Strong(strong) => {
            Content::strong(bundle_html_semantic_content(&strong.body, warnings))
        }
        Content::Emph(emph) => {
            Content::emph(bundle_html_semantic_content(&emph.body, warnings))
        }
        Content::HtmlElem(elem) => Content::HtmlElem(Arc::new(HtmlElem::new(
            elem.tag.clone(),
            elem.attrs.clone(),
            elem.body
                .map_content(|body| bundle_html_semantic_content(body, warnings)),
        ))),
        _ => content.clone(),
    }
}

fn render_bundle_document(
    world: &dyn World,
    source: &Source,
    body: &Content,
    bundle_intr: &typst_core::entities::introspector::TagIntrospector,
    location_start: typst_core::entities::locator::LocatorSeed,
    page_offset: usize,
    format: &str,
    options: &BundleCompileOptions,
    warnings: &mut Vec<SourceDiagnostic>,
) -> SourceResult<(Vec<u8>, bool, usize)> {
    use typst_core::entities::compiler_features::Feature;
    if format == "html" {
        if !options.features.contains(Feature::Html) {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "html export is only available when --features html is passed",
            )
            .with_hint("enable the html feature for HTML documents")]);
        }
        let prepared = body.clone();
        // Labels identify elements during global introspection, but have no
        // visual node in the semantic HTML tree. Keep them until contextual
        // evaluation is complete, then pass their bodies to the exporter.
        let prepared = bundle_html_semantic_content(&prepared, warnings);
        let format_options = collect_document_format_options(body)?;
        let pretty = resolve_html_pretty(options.pretty_override, &format_options);
        let metrics = FallbackFontMetrics::new(world);
        let resolve_frame = |elem: &typst_core::entities::html::HtmlFrameElem,
                             chain: &StyleChain|
         -> Result<String, SourceDiagnostic> {
            let mut contains_html = false;
            elem.body
                .map_content(&mut |node| {
                    contains_html |= matches!(node, Content::HtmlElem(_));
                    Ok(None)
                })
                .expect("html.frame support scan is total");
            if contains_html {
                return Err(SourceDiagnostic::error(
                    Span::detached(),
                    "html.frame body contains content unsupported by paged layout",
                ));
            }
            let page = typst_core::compiler::layout::layout_html_frame(
                elem,
                chain,
                metrics.clone(),
                ImageSizeImageSizer,
            );
            let doc = PagedDocument::new(vec![page.clone()]);
            let fonts = resolve_and_instantiate_fonts(&doc, world)?;
            let glyph_fonts = svg_glyph_font_context_for_page(&page, world);
            let svg_options = crate::export::SvgOptions { pretty, ..Default::default() };
            Ok(svg_options.export_html_frame_with_fonts(&page, &fonts, &glyph_fonts))
        };
        let html = options
            .html_serialization
            .export_with_resolved_frames(&prepared, pretty, &resolve_frame)
            .map_err(|error| vec![error])?;
        return Ok((html.into_bytes(), true, 0));
    }
    if !matches!(format, "pdf" | "svg" | "png") {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "unknown document format",
        )
        .with_hint("set the format parameter explicitly")]);
    }
    let render_content = body.clone();
    let mut render_intr = bundle_intr.clone();
    if page_offset != 0 {
        render_intr.inject_pages(localize_bundle_margin_views(
            &bundle_intr.page_store,
            page_offset,
        )?);
    }
    let (mut document, layout_warnings) = layout_with_realized_math_callbacks_from(
        &render_content,
        render_intr,
        world,
        source,
        location_start,
    )?;
    warnings.extend(layout_warnings);
    warnings.extend(
        document
            .layout_warnings
            .drain(..)
            .map(|message| SourceDiagnostic::warning(Span::detached(), message)),
    );
    if !document.layout_errors.is_empty() {
        return Err(document.layout_errors.drain(..).collect());
    }
    bind_logical_page_numbers(&mut document, &bundle_intr.page_store, page_offset)?;
    let page_count = document.pages.len();
    document.format_options = collect_document_format_options(&render_content)?;
    let document = crate::layout_bidi::reorder_bidi_document(
        document,
        &FallbackFontMetrics::new(world),
    );
    let document = crate::shaper::shape_document(world, document);
    let document = crate::shaper::fix_line_positions(world, document);
    if let Err(message) = crate::export::validate_document_images(&document) {
        return Err(bundle_error(message));
    }
    if matches!(format, "svg" | "png") && document.pages.len() != 1 {
        return Err(bundle_error(format!(
            "{} document must have exactly one page",
            format,
        )));
    }
    match format {
        "pdf" => {
            let (pages, tags, standards, page_warnings) =
                resolve_pdf_pages_tags_and_standards(
                    options.pdf_pages_override.clone(),
                    options.pdf_tagged_override,
                    options.pdf_standard_override,
                    &document.format_options,
                )?;
            warnings.extend(page_warnings);
            let serialization = if options.pretty_override.is_some()
                || document.format_options.pdf.pretty
            {
                resolve_pdf_pretty(options.pretty_override, &document.format_options)
            } else {
                options.pdf_serialization
            };
            let font_combos = collect_fonts_from_doc(&document, world);
            let resolved = resolve_fonts(&font_combos, world);
            let needs_instancer =
                resolved.iter().any(|((_, variant, variations), bytes)| {
                    super::resources::requires_font_instancing(bytes, variant, variations)
                });
            if needs_instancer && !variable_font_instancer_available() {
                for ((font_list, variant, variations), bytes) in &resolved {
                    if super::resources::requires_font_instancing(
                        bytes, variant, variations,
                    ) {
                        let name = font_list
                            .as_slice()
                            .first()
                            .and_then(|font| font.name.as_str())
                            .unwrap_or("variable font");
                        return Err(bundle_error(format!(
                            "variable font '{}' requires instantiation, but Python/fontTools is unavailable",
                            name,
                        )));
                    }
                }
            }
            let (bytes, _) =
                export_pdf_multifont_and_timings_and_document_id_and_tags_and_serialization_and_pages_and_standard(
                    &document,
                    &resolved,
                    options.document_id,
                    options.pdf_stream_mode,
                    tags,
                    serialization,
                    pages,
                    standards,
                );
            Ok((bytes, false, page_count))
        }
        "svg" => {
            let page = &document.pages[0];
            let pretty =
                options.pretty_override.unwrap_or(document.format_options.svg.pretty);
            let svg_options = crate::export::SvgOptions { pretty, ..Default::default() };
            let destinations = svg_destination_context_for_page(&document, 1);
            let fonts = resolve_and_instantiate_fonts(&document, world)
                .map_err(|diagnostic| vec![diagnostic])?;
            let glyph_fonts = svg_glyph_font_context_for_page(page, world);
            let svg = if fonts.is_empty() {
                export_svg_with_context(page, &svg_options, &destinations)
            } else {
                export_svg_with_fonts_and_contexts(
                    page,
                    &svg_options,
                    &fonts,
                    &destinations,
                    &glyph_fonts,
                )
            };
            Ok((svg.into_bytes(), false, page_count))
        }
        "png" => {
            let page = &document.pages[0];
            let ppi = options.png_ppi_override.unwrap_or(document.format_options.png.ppi);
            let render_options = crate::export::RenderOptions {
                pixel_per_pt: (ppi / 72.0) as f32,
                render_bleed: false,
            };
            let fonts = resolve_and_instantiate_fonts(&document, world)
                .map_err(|diagnostic| vec![diagnostic])?;
            let bytes = if fonts.is_empty() {
                export_png(page, &render_options)
            } else {
                export_png_with_fonts(page, &render_options, &fonts)
            };
            Ok((bytes, false, page_count))
        }
        _ => unreachable!(),
    }
}

/// Adapta apenas as vistas usadas pela margem automática ao índice local do
/// documento. As vistas de referência continuam globais para `here()`/labels
/// entre documentos; o store original permanece intacto para o bind final.
fn localize_bundle_margin_views(
    store: &typst_core::entities::page_store::PageStore,
    offset: usize,
) -> SourceResult<typst_core::entities::page_store::PageStore> {
    use typst_core::entities::page_store::PageStore;

    let total = store
        .total_pages()
        .ok_or_else(|| bundle_error("missing realized bundle page store"))?;
    let count = total.get();
    if offset > count {
        return Err(bundle_error("bundle page offset exceeds realized page store"));
    }
    let page_at = |index: usize| {
        std::num::NonZeroUsize::new(index + 1).expect("page indices start at one")
    };
    let mut numberings = Vec::with_capacity(count);
    let mut logical_numbers = Vec::with_capacity(count);
    let mut numbering_spans = Vec::with_capacity(count);
    let mut visible_numberings = Vec::with_capacity(count);
    let mut reference_numberings = Vec::with_capacity(count);
    let mut supplements = Vec::with_capacity(count);
    for local in 0..count {
        let local_page = page_at(local);
        // O documento renderizado nunca deve exceder `count - offset`;
        // slots excedentes não são consultados e conservam a vista original.
        let global = if local < count - offset { local + offset } else { local };
        let global_page = page_at(global);
        numberings.push(store.numbering_for_page(local_page).cloned());
        logical_numbers.push(
            store
                .logical_number_for_page(global_page)
                .ok_or_else(|| bundle_error("missing realized logical page number"))?,
        );
        numbering_spans.push(
            store
                .numbering_span_for_page(local_page)
                .ok_or_else(|| bundle_error("missing realized page numbering span"))?,
        );
        visible_numberings.push(store.visible_numbering_for_page(global_page).cloned());
        reference_numberings
            .push(store.reference_numbering_for_page(local_page).cloned());
        supplements.push(
            store
                .supplement_for_page(local_page)
                .cloned()
                .ok_or_else(|| bundle_error("missing realized page supplement"))?,
        );
    }
    Ok(PageStore::from_realized(
        total,
        numberings,
        logical_numbers,
        numbering_spans,
        visible_numberings,
        reference_numberings,
        supplements,
    ))
}
