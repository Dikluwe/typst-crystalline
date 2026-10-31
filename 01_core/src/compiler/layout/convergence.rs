//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/convergence.md
//! @prompt-hash a4cb9132
//! @layer L1
//! @updated 2026-09-30

use super::*;

#[expect(
    clippy::too_many_arguments,
    reason = "C010: Coordenador privado recebe exatamente os inputs da fachada seeded: métricas/sizer injetados, introspector, font_size e dois transcripts selados com seed"
)]
pub(super) fn run<M: FontMetrics + Clone, S: ImageSizer + Clone>(
    content: &Content,
    introspector: crate::entities::introspector::TagIntrospector,
    metrics: M,
    sizer: S,
    font_size: f64,
    math_callback_store: Option<
        &crate::compiler::math::layout::callbacks::SealedMathCallbacks,
    >,
    par_line_callback_store: Option<&SealedParLineCallbacks>,
    location_start: LocatorSeed,
) -> LayoutPassOutcome {
    use crate::entities::introspector::Introspector;
    use crate::entities::label::Label;
    use std::collections::HashMap;

    // Marcador legado de outline: preservado, mas não encurta o ciclo de
    // convergência, que compara mapas em todos os documentos.
    //
    // P189B (M5): walk puro — flag obtida via Introspector
    // (`kind_index[Outline]` populado por `from_tags` P178) em vez de
    // `state.has_outline` (mutação removida em `introspect.rs:610`).
    // Field `CounterStateLegacy::has_outline` fica morto; cleanup em M6.
    use crate::entities::element_kind::ElementKind;
    let has_outline = introspector.kind_index.contains_key(&ElementKind::Outline);

    // P204C (M8): construir Tracked uma vez. introspector é binding
    // local (owned por valor desde signature) e outlive todos os
    // Layouters criados abaixo (single-pass ou fixpoint loop).
    use comemo::Track;
    let has_injected_page_store = !introspector.page_store.is_empty();
    let mut known_page_store = introspector.page_store.clone();
    let intr_dyn: &dyn crate::entities::introspector::Introspector = &introspector;
    let intr_tracked = intr_dyn.track();

    // P418/P420/P429 — Pré-renderização CSL: descobre style/locale do primeiro
    // BibliographyElem e constrói cache para citações/bibliografia.
    // Só activa CSL quando style é explicitamente fornecido; caso contrário
    // preserva o fallback local `format_bib_entry_body` (compatibilidade P159A-G).
    // P429: style custom `.csl` já foi resolvido em eval time e injectado no
    // `BibStore` do TagIntrospector pelo pipeline; built-ins continuam
    // resolvidos aqui por nome quando não houver entrada na tabela lateral.
    let bib_style = find_first_bibliography_style(content, &introspector);
    let citation_order = introspector.citation_order();
    let mut layout_errors: Vec<SourceDiagnostic> = Vec::new();
    let prepared_bibliography = bib_style.and_then(|style| {
        let result = if let Some(resolved) = style.resolved_style {
            crate::compiler::layout::bib_csl::prepare_cache_with_style(
                introspector.bib_store.entries(),
                &resolved,
                style.locale.as_deref(),
                Some(citation_order),
            )
        } else if let Some(name) = style.style.as_deref() {
            crate::compiler::layout::bib_csl::prepare_cache(
                introspector.bib_store.entries(),
                Some(name),
                style.locale.as_deref(),
                Some(citation_order),
            )
        } else {
            return None;
        };
        match result {
            Ok(cache) => Some(cache),
            Err(diagnostics) => {
                layout_errors.extend(diagnostics);
                None
            }
        }
    });

    let _ = has_outline;

    // ── Fixpoint: documentos com TOC ────────────────────────────────────────
    const MAX_ITERATIONS: usize = 5;

    // Mapa de páginas da iteração anterior — lido por `outline.rs`.
    // NÃO é o mesmo campo onde `references.rs` escreve durante o layout.
    // Separação leitura/escrita: Layouter lê de `known_page_numbers` e
    // escreve em `label_pages` (que começa vazio em cada iteração via Layouter::new()).
    let mut known_page_numbers: HashMap<Label, usize> = HashMap::new();
    // P488 — carry-forward páginas de figuras/tabelas entre iterações (LoF/LoT).
    let mut known_figure_page_numbers: Vec<usize> = Vec::new();
    let mut known_table_page_numbers: Vec<usize> = Vec::new();
    for _ in 0..MAX_ITERATIONS {
        let callback_pass =
            crate::compiler::math::layout::callbacks::MathCallbackPassState::new(
                math_callback_store,
            );
        let par_line_callback_pass =
            ParLineCallbackPassState::new(par_line_callback_store);
        let mut l =
            Layouter::new(metrics.clone(), sizer.clone(), font_size, intr_tracked);
        l.math_callback_pass = Some(&callback_pass);
        l.par_line_callback_pass = Some(&par_line_callback_pass);
        l.bib_render_cache =
            prepared_bibliography.as_ref().map(|prepared| prepared.cache.clone());
        l.bib_layout = prepared_bibliography
            .as_ref()
            .and_then(|prepared| prepared.plan.clone());
        l.locator = Locator::from_seed(location_start);
        l.layout_errors = layout_errors.clone();

        // P204C (M8): assignment `l.introspector = introspector.clone()`
        // eliminado — Tracked partilhado entre iterações via construtor.
        // Tracked é Copy; cada iteração reusa o mesmo handle.
        // P190G (M6 categoria Labels & TOC eliminada): assignments
        // `resolved_labels`/`headings_for_toc` removidos — fields já
        // não existem. Layouter consumers (`references.rs:64`,
        // `outline.rs:38`) migrados para Introspector path puro.
        // P190E/P190G: assignment `numbering_active` removido — field
        // eliminado em P190G Caso 1.
        // P190B: assignments bib_* removidos — fields já não existem.

        // Injectar páginas da iteração anterior para leitura pelo outline.rs.
        // label_pages (onde references.rs escreve) começa vazio via Layouter::new().
        // P190C (M6 categoria Page tracking): known_page_numbers movido
        // para LayouterRuntimeState.
        l.runtime.known_page_numbers = known_page_numbers.clone();
        l.runtime.known_page_store = known_page_store.clone();
        // P488 — injectar páginas de figuras/tabelas da iteração anterior.
        l.runtime.known_figure_page_numbers = known_figure_page_numbers.clone();
        l.runtime.known_table_page_numbers = known_table_page_numbers.clone();

        l.layout_content(content);
        let doc = l.finish();

        // Convergência: mapa de páginas gerado == mapa da iteração anterior?
        // P488: estendido para incluir páginas de figuras/tabelas (LoF/LoT).
        if doc.extracted_label_pages == known_page_numbers
            && doc.extracted_figure_page_numbers == known_figure_page_numbers
            && doc.extracted_table_page_numbers == known_table_page_numbers
        {
            let math = callback_pass.finish_transcript();
            let par_line = par_line_callback_pass.finish();
            if math.is_ok() && par_line.is_ok() {
                return LayoutPassOutcome::Complete(doc);
            }
            return LayoutPassOutcome::Pending(LayoutCallbackRequests {
                math: math.err().unwrap_or_default(),
                par_line: par_line.err().unwrap_or_default(),
            });
        }

        // Actualizar para a próxima iteração.
        known_page_numbers = doc.extracted_label_pages.clone();
        if !has_injected_page_store {
            if let Some(total) = std::num::NonZeroUsize::new(doc.pages.len()) {
                known_page_store = crate::entities::page_store::PageStore::from_runtime(
                    total,
                    doc.pages.iter().map(|page| page.numbering.clone()).collect(),
                    doc.pages.iter().map(|page| page.supplement.clone()).collect(),
                );
            }
        }
        // P488 — actualizar carry-forward de figuras/tabelas.
        known_figure_page_numbers = doc.extracted_figure_page_numbers.clone();
        known_table_page_numbers = doc.extracted_table_page_numbers.clone();
        // A tentativa foi rejeitada pelo fixpoint de TOC. O `callback_pass`
        // e seu transcript são descartados junto com este documento.
    }

    // Limite atingido sem convergência. Não existe candidato cujo transcript
    // possa ser realizado nem documento que possa atravessar a API.
    LayoutPassOutcome::Pending(LayoutCallbackRequests {
        math: Vec::new(),
        par_line: Vec::new(),
    })
}
/// Planned facade coordination preserves the legacy callback coordinator.
pub(super) fn run_planned<M: FontMetrics + Clone, S: ImageSizer + Clone>(
    body: &PageRunningBody,
    introspector: crate::entities::introspector::TagIntrospector,
    metrics: M,
    sizer: S,
    font_size: f64,
    inputs: PageRunningLayoutInputs<'_>,
) -> PageRunningPassOutcome {
    use crate::entities::introspector::Introspector;
    use crate::entities::label::Label;
    use std::collections::HashMap;

    // Marcador legado de outline: preservado, mas não encurta o ciclo de
    // convergência, que compara mapas em todos os documentos.
    //
    // P189B (M5): walk puro — flag obtida via Introspector
    // (`kind_index[Outline]` populado por `from_tags` P178) em vez de
    // `state.has_outline` (mutação removida em `introspect.rs:610`).
    // Field `CounterStateLegacy::has_outline` fica morto; cleanup em M6.
    use crate::entities::element_kind::ElementKind;
    let has_outline = introspector.kind_index.contains_key(&ElementKind::Outline);
    let content = body.content();
    let location_start = body.plan.allocation_start;
    let math_callback_store = inputs.math;
    let par_line_callback_store = inputs.par_line;

    // P204C (M8): construir Tracked uma vez. introspector é binding
    // local (owned por valor desde signature) e outlive todos os
    // Layouters criados abaixo (single-pass ou fixpoint loop).
    use comemo::Track;
    let mut known_page_store = body
        .numbering_input
        .as_ref()
        .unwrap_or(&introspector.page_store)
        .clone();
    let has_injected_page_store = !known_page_store.is_empty();
    let intr_dyn: &dyn crate::entities::introspector::Introspector = &introspector;
    let intr_tracked = intr_dyn.track();

    // P418/P420/P429 — Pré-renderização CSL: descobre style/locale do primeiro
    // BibliographyElem e constrói cache para citações/bibliografia.
    // Só activa CSL quando style é explicitamente fornecido; caso contrário
    // preserva o fallback local `format_bib_entry_body` (compatibilidade P159A-G).
    // P429: style custom `.csl` já foi resolvido em eval time e injectado no
    // `BibStore` do TagIntrospector pelo pipeline; built-ins continuam
    // resolvidos aqui por nome quando não houver entrada na tabela lateral.
    let bib_style = find_first_bibliography_style(content, &introspector);
    let citation_order = introspector.citation_order();
    let mut layout_errors: Vec<SourceDiagnostic> = Vec::new();
    let prepared_bibliography = bib_style.and_then(|style| {
        let result = if let Some(resolved) = style.resolved_style {
            crate::compiler::layout::bib_csl::prepare_cache_with_style(
                introspector.bib_store.entries(),
                &resolved,
                style.locale.as_deref(),
                Some(citation_order),
            )
        } else if let Some(name) = style.style.as_deref() {
            crate::compiler::layout::bib_csl::prepare_cache(
                introspector.bib_store.entries(),
                Some(name),
                style.locale.as_deref(),
                Some(citation_order),
            )
        } else {
            return None;
        };
        match result {
            Ok(cache) => Some(cache),
            Err(diagnostics) => {
                layout_errors.extend(diagnostics);
                None
            }
        }
    });

    let _ = has_outline;

    // ── Fixpoint: documentos com TOC ────────────────────────────────────────
    const MAX_ITERATIONS: usize = 5;

    // Mapa de páginas da iteração anterior — lido por `outline.rs`.
    // NÃO é o mesmo campo onde `references.rs` escreve durante o layout.
    // Separação leitura/escrita: Layouter lê de `known_page_numbers` e
    // escreve em `label_pages` (que começa vazio em cada iteração via Layouter::new()).
    let mut known_page_numbers: HashMap<Label, usize> = HashMap::new();
    // P488 — carry-forward páginas de figuras/tabelas entre iterações (LoF/LoT).
    let mut known_figure_page_numbers: Vec<usize> = Vec::new();
    let mut known_table_page_numbers: Vec<usize> = Vec::new();
    for _ in 0..MAX_ITERATIONS {
        let callback_pass =
            crate::compiler::math::layout::callbacks::MathCallbackPassState::new(
                math_callback_store,
            );
        let par_line_callback_pass =
            ParLineCallbackPassState::new(par_line_callback_store);
        let mut l =
            Layouter::new(metrics.clone(), sizer.clone(), font_size, intr_tracked);
        l.math_callback_pass = Some(&callback_pass);
        l.par_line_callback_pass = Some(&par_line_callback_pass);
        l.bib_render_cache =
            prepared_bibliography.as_ref().map(|prepared| prepared.cache.clone());
        l.bib_layout = prepared_bibliography
            .as_ref()
            .and_then(|prepared| prepared.plan.clone());
        l.locator = Locator::from_seed(location_start);
        l.layout_errors = layout_errors.clone();

        // P204C (M8): assignment `l.introspector = introspector.clone()`
        // eliminado — Tracked partilhado entre iterações via construtor.
        // Tracked é Copy; cada iteração reusa o mesmo handle.
        // P190G (M6 categoria Labels & TOC eliminada): assignments
        // `resolved_labels`/`headings_for_toc` removidos — fields já
        // não existem. Layouter consumers (`references.rs:64`,
        // `outline.rs:38`) migrados para Introspector path puro.
        // P190E/P190G: assignment `numbering_active` removido — field
        // eliminado em P190G Caso 1.
        // P190B: assignments bib_* removidos — fields já não existem.

        // Injectar páginas da iteração anterior para leitura pelo outline.rs.
        // label_pages (onde references.rs escreve) começa vazio via Layouter::new().
        // P190C (M6 categoria Page tracking): known_page_numbers movido
        // para LayouterRuntimeState.
        l.runtime.known_page_numbers = known_page_numbers.clone();
        l.runtime.known_page_store = known_page_store.clone();
        // P488 — injectar páginas de figuras/tabelas da iteração anterior.
        l.runtime.known_figure_page_numbers = known_figure_page_numbers.clone();
        l.runtime.known_table_page_numbers = known_table_page_numbers.clone();

        l.page_running_pass =
            Some(page_running_requests::PlannedPass::new(body, inputs.running));
        l.layout_planned_root(body, page_running_requests::TraceSection::Body);
        let (doc, running) = l.finish_with_running();
        let mut running =
            running.expect("planned pass remains installed until terminal composition");

        // Convergência: mapa de páginas gerado == mapa da iteração anterior?
        // P488: estendido para incluir páginas de figuras/tabelas (LoF/LoT).
        if doc.extracted_label_pages == known_page_numbers
            && doc.extracted_figure_page_numbers == known_figure_page_numbers
            && doc.extracted_table_page_numbers == known_table_page_numbers
        {
            let math = callback_pass.finish_transcript();
            let par_line = par_line_callback_pass.finish();
            if math.is_ok() && par_line.is_ok() {
                let requests = running.pending();
                running.trace.data.complete = requests.is_empty()
                    && running.consumed_all()
                    && running.coverage_complete();
                if running.trace.data.complete {
                    return PageRunningPassOutcome::complete(PageRunningDocument {
                        document: doc,
                        trace: running.trace,
                    });
                }
                return PageRunningPassOutcome::pending(PageRunningRequests {
                    callbacks: LayoutCallbackRequests { math: vec![], par_line: vec![] },
                    running: requests,
                    trace: running.trace,
                });
            }
            return PageRunningPassOutcome::pending(PageRunningRequests {
                callbacks: LayoutCallbackRequests {
                    math: math.err().unwrap_or_default(),
                    par_line: par_line.err().unwrap_or_default(),
                },
                running: running.pending(),
                trace: running.trace,
            });
        }

        // Actualizar para a próxima iteração.
        known_page_numbers = doc.extracted_label_pages.clone();
        if !has_injected_page_store {
            if let Some(total) = std::num::NonZeroUsize::new(doc.pages.len()) {
                known_page_store = crate::entities::page_store::PageStore::from_runtime(
                    total,
                    doc.pages.iter().map(|page| page.numbering.clone()).collect(),
                    doc.pages.iter().map(|page| page.supplement.clone()).collect(),
                );
            }
        }
        // P488 — actualizar carry-forward de figuras/tabelas.
        known_figure_page_numbers = doc.extracted_figure_page_numbers.clone();
        known_table_page_numbers = doc.extracted_table_page_numbers.clone();
        // A tentativa foi rejeitada pelo fixpoint de TOC. O `callback_pass`
        // e seu transcript são descartados junto com este documento.
    }

    // Limite atingido sem convergência. Não existe candidato cujo transcript
    // possa ser realizado nem documento que possa atravessar a API.
    PageRunningPassOutcome::pending(PageRunningRequests {
        callbacks: LayoutCallbackRequests { math: vec![], par_line: vec![] },
        running: vec![],
        trace: page_running_requests::PageRunningTrace::new(body.plan.clone()),
    })
}
/// Dados do primeiro `Content::Bibliography` encontrado no documento.
///
/// P429: `resolved_style` vem do `BibStore` do `TagIntrospector` (tabela
/// lateral indexada pela chave do `BibliographyElem`), não do próprio elemento.
struct FirstBibliographyStyle {
    resolved_style: Option<Arc<IndependentStyle>>,
    style: Option<EcoString>,
    locale: Option<EcoString>,
}

/// Use the first registered declaration, even when its visual was replaced.
/// Raw-layout callers without a semantic snapshot use the active static view.
fn find_first_bibliography_style(
    content: &Content,
    introspector: &crate::entities::introspector::TagIntrospector,
) -> Option<FirstBibliographyStyle> {
    fn discover(
        content: &Content,
        introspector: &crate::entities::introspector::TagIntrospector,
    ) -> Option<FirstBibliographyStyle> {
        let mut found = None;
        content.visit_content_static(&mut |node| {
            if found.is_some() {
                return false;
            }
            if let Content::Bibliography(elem) = node {
                found = Some(FirstBibliographyStyle {
                    resolved_style: introspector
                        .bib_store
                        .style_for_key(elem.style_key())
                        .cloned(),
                    style: elem.style.clone(),
                    locale: elem.locale.clone(),
                });
                return false;
            }
            !matches!(node, Content::Document { .. } | Content::Asset { .. })
        });
        found
    }

    introspector
        .kind_index
        .get(&crate::entities::element_kind::ElementKind::Bibliography)
        .into_iter()
        .flatten()
        .filter_map(|location| introspector.elements.get(location))
        .find_map(|entry| discover(entry.content(), introspector))
        .or_else(|| discover(content, introspector))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn convergence_entrypoint_keeps_static_types() {
        let _ = run::<FixedMetrics, NullImageSizer>;
    }
}

#[cfg(test)]
mod a2_csl_tests {
    use super::*;
    use crate::entities::content::{LocatedContent, RealizedContent, ShowRecipeContent};
    use crate::entities::elements::bibliography::BibliographyElem;
    use crate::entities::file_id::FileId;
    use crate::entities::introspector::TagIntrospector;
    use crate::entities::location::Location;
    use crate::entities::span::Span;
    use crate::entities::style::Styles;
    use crate::entities::style_chain::StyleChain;

    fn bibliography(style: Option<&str>) -> Content {
        Content::Bibliography(Arc::new(BibliographyElem {
            entries: vec![],
            path: Some("test.bib".into()),
            title: None,
            style: style.map(Into::into),
            locale: Some("pt-PT".into()),
        }))
    }

    fn file() -> FileId {
        FileId::from_raw(std::num::NonZeroU16::new(1).unwrap())
    }

    fn realized(original: Content, visual: Content) -> Content {
        Content::Realized(Arc::new(RealizedContent { original, visual }))
    }

    fn style(content: &Content, intr: &TagIntrospector) -> Option<String> {
        find_first_bibliography_style(content, intr)
            .and_then(|found| found.style.map(|style| style.to_string()))
    }

    #[test]
    fn a2_csl_raw_carriers_keep_declaration_style() {
        let b = bibliography(Some("mla"));
        let cases = [
            b.clone().with_source_span(Span::from_range(file(), 2..4)),
            Content::Located(Arc::new(LocatedContent {
                location: Location::from_raw(41),
                body: b.clone(),
            })),
            realized(Content::text("consumed"), b.clone()),
            Content::ShowRecipe(Arc::new(ShowRecipeContent {
                body: b,
                textual: false,
                context_id_floor: 0,
                rules: Arc::from([]),
                styles: StyleChain::empty(),
                guards: vec![],
                file: file(),
            })),
        ];
        for content in cases {
            assert_eq!(
                style(&content, &TagIntrospector::empty()).as_deref(),
                Some("mla")
            );
        }
    }

    #[test]
    fn a2_csl_registered_hidden_declaration_still_styles_citations() {
        let content = realized(bibliography(Some("mla")), Content::text("replacement"));
        let intr = crate::compiler::introspect::introspect_with_introspector(&content);
        assert_eq!(style(&content, &intr).as_deref(), Some("mla"));
    }

    #[test]
    fn a2_csl_semantic_order_does_not_skip_unstyled_first_declaration() {
        let content = Content::sequence(vec![
            realized(bibliography(None), Content::Empty),
            bibliography(Some("ieee")),
        ]);
        let intr = crate::compiler::introspect::introspect_with_introspector(&content);
        let found = find_first_bibliography_style(&content, &intr).unwrap();
        assert!(found.style.is_none());
        assert_eq!(found.locale.as_deref(), Some("pt-PT"));
    }

    #[test]
    fn a2_csl_raw_static_containers_are_navigable() {
        let b = bibliography(Some("mla"));
        for content in [
            Content::par(b.clone()),
            Content::columns(b.clone(), 2, None),
            Content::table_cell(b, None, None, None, None),
        ] {
            assert_eq!(
                style(&content, &TagIntrospector::empty()).as_deref(),
                Some("mla")
            );
        }
    }

    #[test]
    fn a2_csl_fallback_does_not_resurrect_original_or_embedded_document() {
        let hidden = realized(bibliography(Some("mla")), Content::text("replacement"));
        assert!(
            find_first_bibliography_style(&hidden, &TagIntrospector::empty()).is_none()
        );
        let embedded = Content::document(
            "child.pdf".into(),
            bibliography(Some("mla")),
            None,
            None,
            vec![],
            None,
            vec![],
        );
        assert!(
            find_first_bibliography_style(&embedded, &TagIntrospector::empty()).is_none()
        );
        assert_eq!(
            style(
                &Content::sequence(vec![embedded, bibliography(Some("ieee"))]),
                &TagIntrospector::empty(),
            )
            .as_deref(),
            Some("ieee"),
        );
    }

    #[test]
    fn a2_csl_semantic_snapshot_preserves_custom_resolved_style_and_locale() {
        let b = bibliography(Some("custom.csl"));
        let Content::Bibliography(elem) = &b else { unreachable!() };
        let key = elem.style_key();
        let content = realized(b, Content::Empty);
        let mut intr =
            crate::compiler::introspect::introspect_with_introspector(&content);
        let resolved = Arc::new(
            crate::compiler::layout::bib_csl::resolve_style_name("mla").unwrap(),
        );
        intr.bib_store.add_style(key, Arc::clone(&resolved));
        let found = find_first_bibliography_style(&content, &intr).unwrap();
        assert_eq!(found.style.as_deref(), Some("custom.csl"));
        assert_eq!(found.locale.as_deref(), Some("pt-PT"));
        assert!(Arc::ptr_eq(found.resolved_style.as_ref().unwrap(), &resolved));
    }

    #[test]
    fn a2_csl_first_raw_declaration_and_no_declaration_controls() {
        let first = Content::styled(Box::new(bibliography(Some("mla"))), Styles::new());
        let content = Content::sequence(vec![first, bibliography(Some("ieee"))]);
        assert_eq!(style(&content, &TagIntrospector::empty()).as_deref(), Some("mla"));
        assert!(find_first_bibliography_style(
            &Content::sequence(vec![Content::text("plain"), Content::Empty]),
            &TagIntrospector::empty(),
        )
        .is_none());
    }
}
