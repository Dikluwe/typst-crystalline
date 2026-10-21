//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/pipeline/html.md
//! @prompt-hash 9049e636
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


#[cfg(test)]
mod batch_g_html_frame_tests {
    use super::*;
    use typst_core::entities::{
        content::{LocatedContent,RealizedContent,ShowRecipeContent,SourceContent},
        elements::context_block::ContextBlockElem,func::Func,html::HtmlFrameElem,
        location::Location,style::{Style,Styles},
    };
    fn located(body:Content)->Content {
        Content::Located(Arc::new(LocatedContent {body,location:Location::from_raw(430)}))
    }
    fn frame(body:Content)->Content {Content::HtmlFrame(Arc::new(HtmlFrameElem {body}))}
    fn styled(body:Content,size:f64)->Content {
        Content::styled(Box::new(body),Styles::from_iter([Style::Size(typst_core::entities::layout_types::Length::pt(size))]))
    }
    fn context(id:u64)->Content {
        Content::ContextBlock(Arc::new(ContextBlockElem {id,closure:Func::native("must-not-evaluate",
            |_,_,_,_| panic!("intrinsic position scan must not evaluate context"))}))
    }
    #[test]
    fn batch_g_html_located_frame_discovery_and_style_counts_agree() {
        let source=|body|Content::Source(Arc::new(SourceContent {body,span:Span::detached()}));
        for content in [
            located(frame(Content::text("A"))),
            located(located(frame(Content::text("A")))),
            located(source(frame(Content::text("A")))),
            source(located(frame(Content::text("A")))),
        ] {
            let frames=collect_html_frames(&styled(content,19.0),&StyleChain::default_chain());
            assert_eq!(frames.len(),1,"transported frame must contribute exactly once");
            assert_eq!(frames[0].0.body.plain_text(),"A");
            assert_eq!(frames[0].1.size(),19.0);
        }
        let content=styled(Content::sequence(vec![
            located(styled(frame(Content::text("INNER")),27.0)),
            frame(Content::text("SIBLING")),
        ]),19.0);
        let frames=collect_html_frames(&content,&StyleChain::default_chain());
        assert_eq!(frames.iter().map(|(f,_)|f.body.plain_text()).collect::<Vec<_>>(),["INNER","SIBLING"]);
        assert_eq!(frames.iter().map(|(_,s)|s.size()).collect::<Vec<_>>(),[27.0,19.0],"frame_count must not style unrelated siblings");
    }
    #[test]
    fn batch_g_html_nested_frames_and_visual_carriers_are_once_only() {
        let nested=located(frame(Content::sequence(vec![frame(Content::text("INNER")),Content::text("OUTER")])));
        let frames=collect_html_frames(&nested,&StyleChain::default_chain());
        assert_eq!(frames.iter().map(|(f,_)|f.body.plain_text()).collect::<Vec<_>>(),["INNER","INNEROUTER"]);
        let realized=Content::Realized(Arc::new(RealizedContent {
            original:frame(Content::text("ORIGINAL")),visual:frame(Content::text("VISUAL")),
        }));
        let recipe=Content::ShowRecipe(Arc::new(ShowRecipeContent {
            body:frame(Content::text("RECIPE")),textual:false,context_id_floor:0,
            rules:Arc::from([]),styles:StyleChain::empty(),guards:vec![],
            file:typst_core::entities::file_id::FileId::from_raw(std::num::NonZeroU16::new(1).unwrap()),
        }));
        let frames=collect_html_frames(&Content::sequence(vec![located(realized),located(recipe)]),&StyleChain::default_chain());
        assert_eq!(frames.iter().map(|(f,_)|f.body.plain_text()).collect::<Vec<_>>(),["VISUAL","RECIPE"]);
    }
    #[test]
    fn batch_g_html_located_frame_positions_require_real_matching_context_ids() {
        let world=crate::world::SystemWorld::for_stdin(std::path::PathBuf::from("/tmp"),String::new());
        let source=world.source(world.main()).unwrap();
        let mut candidate=typst_core::entities::introspector::TagIntrospector::empty();
        let location=Location::from_raw(440);
        candidate.context_block_locations.insert(11,location);
        let body=located(frame(Content::sequence(vec![Content::text("A"),context(11)])));
        let positions=html_frame_positions(&body,&candidate,FallbackFontMetrics::new(&world),&source).unwrap();
        assert!(positions.contains_key(&location),"Located frame cannot disappear from position collection");
        assert_eq!(positions.len(),1);
        assert!(candidate.page_store.total_pages().is_none(),"intrinsic island must not add public pages");
        let missing=frame(Content::sequence(vec![Content::text("A"),context(12)]));
        assert!(html_frame_positions(&missing,&candidate,FallbackFontMetrics::new(&world),&source).is_err());
        let duplicate=frame(Content::sequence(vec![Content::text("A"),context(11),context(11)]));
        assert!(html_frame_positions(&duplicate,&candidate,FallbackFontMetrics::new(&world),&source).is_err());
    }
    #[test]
    fn batch_g_html_located_context_root_keeps_expected_id_matching_strict() {
        let world=crate::world::SystemWorld::for_stdin(std::path::PathBuf::from("/tmp"),String::new());
        let source=world.source(world.main()).unwrap();
        let mut candidate=typst_core::entities::introspector::TagIntrospector::empty();
        let location=Location::from_raw(441);candidate.context_block_locations.insert(21,location);
        let body=frame(located(context(21)));
        let positions=html_frame_positions(&body,&candidate,FallbackFontMetrics::new(&world),&source)
            .expect("actual located context marker must be included by the expected-id scan");
        assert_eq!(positions.len(),1);
        assert!(positions.contains_key(&location));
    }
}
