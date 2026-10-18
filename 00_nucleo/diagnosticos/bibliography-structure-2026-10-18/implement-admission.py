from pathlib import Path
root=Path('01_core/src/compiler/layout')
f=root/'bibliography.rs';s=f.read_text();marker='/// Helper privado P159D';i=s.index(marker);s=s[:i]+'''#[derive(Clone)]
pub(super) struct PendingPrefix {
    body: Content,
    chain: crate::entities::style_chain::StyleChain,
    style: crate::entities::layout_types::TextStyle,
    indent: f64,
    rtl: bool,
}

/// Emit only after the paragraph backend has admitted its actual first line.
pub(super) fn emit_pending_prefix<M: FontMetrics, S: ImageSizer>(l: &mut Layouter<M, S>) {
    use crate::entities::layout_types::{Align2D, HAlign, VAlign, PlaceScope, Rect, Pt};
    let Some(prefix) = l.pending_bib_prefix.take() else { return };
    let baseline = l.baseline_y();
    let style = std::mem::replace(&mut l.style, prefix.style);
    let chain = std::mem::replace(&mut l.chain, prefix.chain);
    let (top, _) = l.metrics.text_edges(l.style.size, &l.style);
    let offset = l.style.baseline_offset.resolve_pt(l.style.size.0);
    let region = l.block_place_region.replace(Rect {
        x: l.regions.current.line_start_x,
        y: baseline - top - Pt(offset),
        w: Pt(l.regions.current.width-l.page_config.margin.right-l.regions.current.line_start_x.0),
        h: Pt(f64::INFINITY),
    });
    let placed = Content::place(
        Align2D { h: Some(if prefix.rtl { HAlign::Right } else { HAlign::Left }), v: Some(VAlign::Top) },
        if prefix.rtl { prefix.indent } else { -prefix.indent }, 0.0,
        PlaceScope::Column, false, None, prefix.body,
    );
    l.layout_content(&placed);
    l.block_place_region = region;
    l.chain = chain;
    l.style = style;
}

'''+s[i:]
a=s.index('    for entry in entries {',s.index('fn layout_columns'));b=s.index('\n}\n\nfn fallback_body',a)
s=s[:a]+'''    for entry in entries {
        if entry.body.is_empty() {
            let previous = layouter.pending_bib_prefix.take();
            layouter.layout_content(&entry_block(entry.prefix.clone().unwrap_or(Content::Empty)));
            layouter.pending_bib_prefix = previous;
            continue;
        }
        let pending = entry.prefix.as_ref().map(|prefix| PendingPrefix {
            body: prefix.clone(), chain: layouter.chain.clone(), style: layouter.style.clone(), indent, rtl,
        });
        let previous = std::mem::replace(&mut layouter.pending_bib_prefix, pending);
        layouter.layout_content(&Content::block(entry.body.clone(), None, None, inset, true));
        layouter.pending_bib_prefix = previous;
    }'''+s[b:];f.write_text(s)
f=root/'mod.rs';s=f.read_text();needle='    bib_layout: Option<Arc<bib_csl::plan::BibliographyPlan>>,';assert needle in s;s=s.replace(needle,needle+'\n    pending_bib_prefix: Option<bibliography::PendingPrefix>,\n    block_inline_region: Option<block::InlineRegion>,',1);s=s.replace('            bib_layout: None,','            bib_layout: None,\n            pending_bib_prefix: None,\n            block_inline_region: None,',1);f.write_text(s)
f=root/'checkpoint.rs';s=f.read_text();needle='    bib_layout: Option<std::sync::Arc<super::bib_csl::plan::BibliographyPlan>>,';assert needle in s;s=s.replace(needle,needle+'\n    pending_bib_prefix: Option<super::bibliography::PendingPrefix>,\n    block_inline_region: Option<super::block::InlineRegion>,',1);s=s.replace('            bib_layout: self.bib_layout.clone(),','            bib_layout: self.bib_layout.clone(),\n            pending_bib_prefix: self.pending_bib_prefix.clone(),\n            block_inline_region: self.block_inline_region,',1);s=s.replace('        self.bib_layout = checkpoint.bib_layout;','        self.bib_layout = checkpoint.bib_layout;\n        self.pending_bib_prefix = checkpoint.pending_bib_prefix;\n        self.block_inline_region = checkpoint.block_inline_region;',1);f.write_text(s)
f=root/'sub_frame.rs';s=f.read_text();needle='        let saved_block_auto_width = std::mem::take(&mut self.block_auto_width);';assert needle in s;s=s.replace(needle,needle+'\n        let saved_pending_bib_prefix = self.pending_bib_prefix.take();\n        let saved_block_inline_region = self.block_inline_region.take();',1);s=s.replace('        self.block_auto_width = saved_block_auto_width;','        self.block_auto_width = saved_block_auto_width;\n        self.pending_bib_prefix = saved_pending_bib_prefix;\n        self.block_inline_region = saved_block_inline_region;',1);f.write_text(s)
f=root/'word_layout.rs';s=f.read_text();needle=') {\n    if let Some(fill) = layouter.style.highlight.clone() {';assert needle in s;s=s.replace(needle,') {\n    super::bibliography::emit_pending_prefix(layouter);\n    if let Some(fill) = layouter.style.highlight.clone() {',1);f.write_text(s)
f=root/'text.rs';s=f.read_text();needle='        super::text_baseline::admit(layouter);';assert needle in s;s=s.replace(needle,needle+'\n        super::bibliography::emit_pending_prefix(layouter);',1);f.write_text(s)
f=root/'block.rs';s=f.read_text();i=s.index('/// Resolve altura relativa');s=s[:i]+'''#[derive(Clone, Copy)]
pub(super) struct InlineRegion {
    offset: f64,
    width: f64,
}

/// Reinstall the active body's horizontal geometry in the current region.
pub(super) fn install_inline_region<M: FontMetrics, S: ImageSizer>(l: &mut Layouter<M, S>) {
    let natural_width = if l.column_mode { l.column_width } else { l.page_config.width };
    let (offset, width) = l.block_inline_region.map(|r| (r.offset,r.width))
        .unwrap_or((0.0,natural_width-l.page_config.margin.horizontal()));
    let start = l.page_config.margin.left+offset;
    l.regions.current.line_start_x = Pt(start);
    l.regions.current.cursor_x = Pt(start);
    l.regions.current.width = start+width+l.page_config.margin.right;
}

'''+s[i:];needle='    layouter.layout_content(body);\n    layouter.flush_line();';assert needle in s;s=s.replace(needle,'''    let previous_inline_region = layouter.block_inline_region.replace(InlineRegion {
        offset: layouter.regions.current.line_start_x.0-layouter.page_config.margin.left,
        width: inner_w,
    });
    layouter.layout_content(body);
    layouter.flush_line();
    layouter.block_inline_region = previous_inline_region;''',1)
needle='''    layouter.regions.current.line_start_x = saved_line_start;
    layouter.regions.current.cursor_x = saved_line_start;
    layouter.regions.current.width = saved_width;''';assert needle in s;s=s.replace(needle,'''    if same_region {
        layouter.regions.current.line_start_x = saved_line_start;
        layouter.regions.current.cursor_x = saved_line_start;
        layouter.regions.current.width = saved_width;
    } else {
        install_inline_region(layouter);
    }''',1);f.write_text(s)
f=root/'region_progress.rs';s=f.read_text();needle='            layouter.start_next_column();';assert needle in s;s=s.replace(needle,needle+'\n            if layouter.block_inline_region.is_some() {\n                super::block::install_inline_region(layouter);\n            }',1);needle='''        layouter.start_column(0);
    }
}''';assert needle in s;s=s.replace(needle,'''        layouter.start_column(0);
    }
    if layouter.block_inline_region.is_some() {
        super::block::install_inline_region(layouter);
    }
}''',1);f.write_text(s)
# Expand the exact-scope verifier to all prescribed existing owners.
f=Path('00_nucleo/diagnosticos/bibliography-structure-2026-10-18/seal.py');s=f.read_text();a=s.index("expected={'00_nucleo/prompts/'");b=s.index('\nassert ',a);s=s[:a]+'''expected=set()
for owner in roles:
 if owner=='compiler/layout/bib_csl/plan.md':continue
 expected.add('00_nucleo/prompts/'+owner)
 consumer='mod.rs' if owner=='compiler/layout.md' else owner.removeprefix('compiler/layout/').removesuffix('.md')+'.rs'
 expected.add('01_core/src/compiler/layout/'+consumer)
'''+s[b:];f.write_text(s)
