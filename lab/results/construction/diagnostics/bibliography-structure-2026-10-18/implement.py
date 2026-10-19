from pathlib import Path
root=Path('01_core/src/compiler/layout')
f=root/'bib_csl.rs';s=f.read_text();s=s.replace('use std::collections::HashMap;','pub(super) mod plan;\n\nuse std::collections::HashMap;\nuse std::sync::Arc;',1)
pos=s.index('/// Style CSL default')
s=s[:pos]+'''pub(super) struct PreparedBibliography {
    pub(super) cache: BibRenderCache,
    pub(super) plan: Option<Arc<plan::BibliographyPlan>>,
}

'''+s[pos:]
needle='''    let style_name = style.unwrap_or(DEFAULT_STYLE);''';s=s.replace(needle,'''    prepare_cache(entries, style, locale, citation_order).map(|prepared| prepared.cache)
}

pub(super) fn prepare_cache(
    entries: &[BibEntry],
    style: Option<&str>,
    locale: Option<&str>,
    citation_order: Option<&[String]>,
) -> SourceResult<PreparedBibliography> {
    let style_name = style.unwrap_or(DEFAULT_STYLE);''',1)
s=s.replace('    build_cache_with_style(entries, &independent, locale, citation_order)','    prepare_cache_with_style(entries, &independent, locale, citation_order)',1)
needle='''    let locales = build_locales(locale);''';s=s.replace(needle,'''    prepare_cache_with_style(entries, independent, locale, citation_order)
        .map(|prepared| prepared.cache)
}

pub(super) fn prepare_cache_with_style(
    entries: &[BibEntry],
    independent: &IndependentStyle,
    locale: Option<&str>,
    citation_order: Option<&[String]>,
) -> SourceResult<PreparedBibliography> {
    let locales = build_locales(locale);''',1)
s=s.replace('return Ok(BibRenderCache { citations: HashMap::new(), bibliography: None });','return Ok(PreparedBibliography {\n            cache: BibRenderCache { citations: HashMap::new(), bibliography: None },\n            plan: None,\n        });',1)
s=s.replace('''    let bibliography = rendered.bibliography.as_ref().map(render_bibliography);

    Ok(BibRenderCache { citations, bibliography })''','''    let plan = rendered.bibliography.as_ref()
        .map(|bib| Arc::new(plan::BibliographyPlan::from_rendered(bib)));
    let bibliography = plan.as_ref().map(|plan| plan.legacy_content());

    Ok(PreparedBibliography { cache: BibRenderCache { citations, bibliography }, plan })''',1)
a=s.index('/// Renderiza uma `RenderedBibliography`');b=s.index('/// Converte `ElemChildren`',a);s=s[:a]+s[b:];f.write_text(s)
f=root/'convergence.rs';s=f.read_text().replace('let bib_render_cache = bib_style.and_then','let prepared_bibliography = bib_style.and_then',1).replace('bib_csl::build_cache_with_style(','bib_csl::prepare_cache_with_style(',1).replace('bib_csl::build_cache(','bib_csl::prepare_cache(',1)
s=s.replace('        l.bib_render_cache = bib_render_cache.clone();','''        l.bib_render_cache = prepared_bibliography.as_ref().map(|prepared| prepared.cache.clone());
        l.bib_layout = prepared_bibliography.as_ref().and_then(|prepared| prepared.plan.clone());''',1);f.write_text(s)
f=root/'mod.rs';s=f.read_text();needle='    pub(super) bib_render_cache: Option<crate::compiler::layout::bib_csl::BibRenderCache>,';assert needle in s;s=s.replace(needle,needle+'\n    bib_layout: Option<Arc<bib_csl::plan::BibliographyPlan>>,',1);s=s.replace('            bib_render_cache: None,','            bib_render_cache: None,\n            bib_layout: None,',1);f.write_text(s)
f=root/'checkpoint.rs';s=f.read_text();needle='    bib_render_cache: Option<crate::compiler::layout::bib_csl::BibRenderCache>,';assert needle in s;s=s.replace(needle,needle+'\n    bib_layout: Option<std::sync::Arc<super::bib_csl::plan::BibliographyPlan>>,',1);s=s.replace('            bib_render_cache: self.bib_render_cache.clone(),','            bib_render_cache: self.bib_render_cache.clone(),\n            bib_layout: self.bib_layout.clone(),',1);s=s.replace('        self.bib_render_cache = checkpoint.bib_render_cache;','        self.bib_render_cache = checkpoint.bib_render_cache;\n        self.bib_layout = checkpoint.bib_layout;',1);f.write_text(s)
f=root/'bibliography.rs';s=f.read_text();needle='''    if !body.is_empty() {
        let block = Content::block(''';assert needle in s
s=s.replace(needle,'''    if let Some(plan) = layouter.bib_layout.clone() {
        use super::bib_csl::plan::BibliographyPlan;
        match plan.as_ref() {
            BibliographyPlan::Columns(entries) => {
                let spacing = super::paragraph::spacing_for_size(&layouter.chain, layouter.style.size.0);
                let grid = prefix_columns(entries, spacing);
                layouter.layout_content(&entry_block(grid));
            }
            // Hanging transport is retained; its physical paragraph consumer
            // belongs to the next A2-HangingTransport/A2-ParIndent step.
            BibliographyPlan::Hanging(entries) | BibliographyPlan::Plain(entries) => {
                for entry in entries {
                    layouter.layout_content(&entry_block(entry.body.clone()));
                }
            }
        }
        return;
    }
    if !body.is_empty() {
        layouter.layout_content(&entry_block(body));
    }
}

fn entry_block(body: Content) -> Content {
    Content::block(''',1)
a=s.index('''        layouter.layout_content(&block);''');b=s.index('\nfn fallback_body',a)
s=s[:a]+'''    )
}

fn prefix_columns(entries: &[super::bib_csl::plan::Entry], spacing: f64) -> Content {
    use crate::entities::{elements::grid::GridElem, layout_types::{Length, TrackSizing}, sides::Sides};
    use std::sync::Arc;
    let mut rows = Vec::with_capacity(entries.len().saturating_mul(2));
    let mut cells = Vec::with_capacity(entries.len().saturating_mul(4));
    for (index, entry) in entries.iter().enumerate() {
        if index > 0 {
            rows.push(TrackSizing::Fixed(spacing));
            cells.extend([Content::Empty, Content::Empty]);
        }
        rows.push(TrackSizing::Auto);
        cells.push(entry.prefix.clone().unwrap_or(Content::Empty));
        cells.push(entry.body.clone());
    }
    Content::Grid(Arc::new(GridElem {
        columns: vec![TrackSizing::Auto, TrackSizing::Auto],
        rows, cells, hlines: vec![], vlines: vec![], gutter: Some(Length::em(0.65)),
        align: None, inset: Sides::uniform(Length::ZERO), header: None, footer: None,
        stroke: None, fill: None,
    }))
}
'''+s[b:]
# Remove old local binding terminator left by extracting the Block helper.
s=s.replace('''            true,
        );
    )
}''','''            true,
    )
}''',1);f.write_text(s)
