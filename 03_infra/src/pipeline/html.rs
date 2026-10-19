//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/pipeline/html.md
//! @prompt-hash 8a45c2d6
//! @layer L3
//! @updated 2026-10-05
//!
//! Orquestra o target semântico HTML e suas ilhas html.frame. Conserva o gate de features, a precedência de pretty, o target causal e a preparação contextual. Fornece posições intrínsecas à sessão seletiva sem criar paginação global HTML.

use super::*;

/// Pipeline semântico HTML, sem layout paginado.
pub fn compile_to_html_string(
    world: &dyn World,
    source: &Source,
) -> (Result<String, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>) {
    compile_to_html_string_with_features_and_serialization(
        world,
        source,
        typst_core::entities::compiler_features::Features::default(),
        crate::export::HtmlSerializationMode::Crystalline,
    )
}

pub fn compile_to_html_string_with_features(
    world: &dyn World,
    source: &Source,
    features: typst_core::entities::compiler_features::Features,
) -> (Result<String, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>) {
    compile_to_html_string_with_features_and_serialization(
        world,
        source,
        features,
        crate::export::HtmlSerializationMode::Crystalline,
    )
}

pub fn compile_to_html_string_with_features_and_serialization(
    world: &dyn World,
    source: &Source,
    features: typst_core::entities::compiler_features::Features,
    mode: crate::export::HtmlSerializationMode,
) -> (Result<String, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>) {
    compile_to_html_string_with_features_and_serialization_and_pretty_override(
        world, source, features, mode, None,
    )
}

pub fn compile_to_html_string_with_features_and_serialization_and_pretty(
    world: &dyn World,
    source: &Source,
    features: typst_core::entities::compiler_features::Features,
    mode: crate::export::HtmlSerializationMode,
    pretty: bool,
) -> (Result<String, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>) {
    compile_to_html_string_with_features_and_serialization_and_pretty_override(
        world,
        source,
        features,
        mode,
        Some(pretty),
    )
}

pub(super) fn resolve_html_pretty(
    pretty_override: Option<bool>,
    options: &DocumentFormatOptions,
) -> bool {
    pretty_override.unwrap_or(options.html.pretty)
}

pub fn compile_to_html_string_with_features_and_serialization_and_pretty_override(
    world: &dyn World,
    source: &Source,
    features: typst_core::entities::compiler_features::Features,
    mode: crate::export::HtmlSerializationMode,
    pretty_override: Option<bool>,
) -> (Result<String, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>) {
    if !features.contains(typst_core::entities::compiler_features::Feature::Html) {
        return (
            Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "html export is only available when `--features html` is passed",
            )
            .with_hint("html export is under active development and incomplete")
            .with_hint(
                "see https://github.com/typst/typst/issues/5512 for more information",
            )]),
            vec![],
        );
    }
    let (result, mut warnings) = eval_to_module_with_sink_target_features(
        world,
        source,
        false,
        EvalTarget::Html,
        features,
    );
    let html = result.and_then(|module| {
        let content = module.content().cloned().unwrap_or(Content::Empty);
        let format_options = collect_document_format_options(&content)?;
        let pretty = resolve_html_pretty(pretty_override, &format_options);
        let mut intr = module
            .realization_introspector()
            .cloned()
            .unwrap_or_else(|| introspect_with_introspector(&content));
        let (supplements, supplement_warnings) =
            super::runtime::prepare_heading_supplements(
                &mut intr,
                world,
                source,
                EvalTarget::Html,
                features,
            );
        warnings.extend(supplement_warnings);
        supplements?;
        let (prepared, contextual_warnings) = context_stabilization::prepare_html(
            content, &intr, world, source, false, features,
        );
        warnings.extend(contextual_warnings);
        let prepared = prepared?;
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
            let fonts = resolve_export_fonts(&doc, world)?;
            let glyph_fonts = svg_glyph_font_context_for_page(&page, world);
            let options = crate::export::SvgOptions { pretty, ..Default::default() };
            Ok(options.export_html_frame_with_fonts(&page, &fonts, &glyph_fonts))
        };
        mode.export_with_resolved_frames(&prepared, pretty, &resolve_frame)
            .map_err(|error| vec![error])
    });
    (html, warnings)
}

/// P1493 — recolhe `html.frame` em ordem bottom-up e conserva a cadeia causal.
/// O frame externo sucede os aninhados e torna-se a autoridade final do ponto.
fn collect_html_frames(
    content: &Content,
    chain: &StyleChain,
) -> Vec<(Arc<typst_core::entities::html::HtmlFrameElem>, StyleChain)> {
    fn frame_count(content: &Content) -> usize {
        let mut count = 0usize;
        let _ = content.map_content(&mut |node| {
            count += usize::from(matches!(node, Content::HtmlFrame(_)));
            Ok(None)
        });
        count
    }

    let mut frames: Vec<(
        Arc<typst_core::entities::html::HtmlFrameElem>,
        Vec<typst_core::entities::style::Styles>,
    )> = Vec::new();
    let _ = content.map_content(&mut |node| {
        match node {
            Content::HtmlFrame(elem) => frames.push((elem.clone(), Vec::new())),
            Content::Styled(styled) => {
                let inner = &styled.body;
                let styles = &styled.styles;
                let count = frame_count(inner);
                for (_, ancestors_inner_first) in frames.iter_mut().rev().take(count) {
                    ancestors_inner_first.push(styles.clone());
                }
            }
            _ => {}
        }
        Ok(None)
    });

    frames
        .into_iter()
        .map(|(elem, ancestors_inner_first)| {
            let causal = ancestors_inner_first
                .iter()
                .rev()
                .fold(chain.clone(), |chain, styles| chain.push_styles(styles));
            (elem, causal)
        })
        .collect()
}

/// P1493 — realiza somente ilhas intrínsecas e produz o store HTML completo.
pub(super) fn html_frame_positions(
    content: &Content,
    candidate: &typst_core::entities::introspector::TagIntrospector,
    metrics: FallbackFontMetrics<'_>,
    source: &Source,
) -> SourceResult<
    HashMap<
        typst_core::entities::location::Location,
        typst_core::entities::position::HtmlPosition,
    >,
> {
    let failure = || {
        vec![SourceDiagnostic::error(
            source.root().span(),
            "contextual stability could not be verified",
        )]
    };
    let mut positions = HashMap::new();
    let mut owners = HashMap::new();

    for (elem, chain) in collect_html_frames(content, &StyleChain::default_chain()) {
        let mut expected_ids = HashSet::new();
        let mut unique_ids = true;
        let _ = elem.body.map_content(&mut |node| {
            if let Content::ContextBlock(context) = node {
                unique_ids &= expected_ids.insert(context.id);
            }
            Ok(None)
        });
        if !unique_ids {
            return Err(failure());
        }
        let detailed = typst_core::compiler::layout::layout_html_frame_detailed(
            &elem,
            &chain,
            metrics.clone(),
            ImageSizeImageSizer,
        );
        let mut frame_ids = HashSet::new();
        for (id, position) in detailed.positions {
            if !frame_ids.insert(id) {
                return Err(failure());
            }
            let Some(location) = candidate.context_block_locations.get(&id).copied()
            else {
                return Err(failure());
            };
            if let Some(previous_id) = owners.insert(location, id) {
                if previous_id != id {
                    return Err(failure());
                }
            }
            positions.insert(location, position);
        }
        if frame_ids != expected_ids {
            return Err(failure());
        }
    }

    Ok(positions)
}

#[cfg(test)]
mod p1465_tests {
    use super::*;

    #[test]
    fn html_pretty_precedence_is_override_then_document_then_false() {
        let mut options = DocumentFormatOptions::default();
        assert!(!resolve_html_pretty(None, &options));

        options.html.pretty = true;
        assert!(resolve_html_pretty(None, &options));
        assert!(!resolve_html_pretty(Some(false), &options));

        options.html.pretty = false;
        assert!(resolve_html_pretty(Some(true), &options));
    }
}
