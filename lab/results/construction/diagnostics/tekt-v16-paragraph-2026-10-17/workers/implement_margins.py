from pathlib import Path
import json,hashlib
out=Path(__file__).resolve().parents[1];c=json.load(open(out/'margin-tests-before-code.json'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();assert all(sha(p)==h for p,h in c['files'].items());log=(out/'margin-red.log').read_text();assert 'test result: FAILED.' in log and 'panicked at' in log
p=Path('01_core/src/compiler/layout/paragraph.rs');s=p.read_text()
s=s.replace("Styled(&'a Content),", "Styled(&'a crate::entities::content::StyledContent),")
s=s.replace('ParagraphBody::Styled(&styled.body)', 'ParagraphBody::Styled(styled)')
s=s.replace('ParagraphBody::Styled(body) => is_paragraph(body)', 'ParagraphBody::Styled(styled) => is_paragraph(&styled.body)')
s=s.replace('ParagraphBody::Styled(body) => is_empty_body(body)', 'ParagraphBody::Styled(styled) => is_empty_body(&styled.body)')
start=s.index('fn contains_empty_text(');end=s.index('\npub(super) fn layout<',start)
s=s[:start]+'''fn empty_text_style(
    content: &Content,
    chain: &crate::entities::style_chain::StyleChain,
) -> Option<crate::entities::style_chain::StyleChain> {
    match paragraph_body(content) {
        ParagraphBody::Text(text) => text.is_empty().then(|| chain.clone()),
        ParagraphBody::Sequence(parts) => parts.iter().rev().find_map(|part| empty_text_style(part, chain)),
        ParagraphBody::Styled(styled) => empty_text_style(&styled.body, &chain.push_styles(&styled.styles)),
        ParagraphBody::Empty | ParagraphBody::Space | ParagraphBody::Par | ParagraphBody::Opaque => None,
    }
}
''' +s[end:]
a=s.index('        let virtual_top = if contains_empty_text(body) {');end=s.index('        if layouter.initial_baseline_pending {',a)
s=s[:a]+'''        let virtual_top = empty_text_style(body, &layouter.chain).map_or(0.0, |chain| {
            let style = crate::entities::layout_types::TextStyle::from(&chain);
            let (top, bottom) = layouter.metrics.text_edges(style.size, &style);
            let offset = style.baseline_offset.resolve_pt(style.size.val());
            (top.0 - offset).max(0.0) + (-bottom.0 + offset).max(0.0)
        });
''' +s[end:]
a=s.index('        if layouter.initial_baseline_pending {\n            layouter.empty_paragraph_weak_spacing += spacing;',s.index('let empty_body ='));b=s.index('        layouter.block_chain_active = true;',a)
s=s[:a]+'''        let first_empty = layouter.initial_baseline_pending && !layouter.block_chain_active;
        let above = if layouter.block_chain_active && !layouter.prev_margin_is_parbreak {
            layouter.prev_block_below_pending
        } else {
            layouter.prev_block_below_pending.max(spacing)
        };
        let end = if first_empty {
            layouter.regions.current.cursor_y.0 + virtual_top
        } else if had_previous_text || layouter.block_chain_active {
            layouter.prev_line_baseline + layouter.prev_block_equation_descent + above + virtual_top
        } else {
            layouter.regions.current.cursor_y.0 + spacing + virtual_top
        };
        // A zero-height paragraph is a causal unit; only its lower margin is pending.
        layouter.regions.current.cursor_y = Pt(end);
        layouter.prev_line_baseline = end;
        layouter.prev_block_equation_descent = 0.0;
        layouter.last_block_descent_y = Some(layouter.last_block_descent_y.map_or(end, |bottom| bottom.max(end)));
        layouter.record_flow_placement_end(end);
        layouter.prev_block_below_pending = spacing;
        layouter.initial_baseline_pending = false;
        layouter.empty_paragraph_weak_spacing = 0.0;
''' +s[b:];a=s.index('pub(super) fn layout<');e=s.index('\n#[cfg(test)]',a);s=s[:a]+s[a:e].replace('virtual_top','virtual_height')+s[e:];p.write_text(s)
p=Path('01_core/src/compiler/stdlib/structural/par.rs');s=p.read_text();s=s.replace('use crate::compiler::eval::EvalContext;','use crate::compiler::eval::EvalContext;\nuse crate::compiler::eval::rules::expected_length_error;');old='''            // Known properties awaiting their own consumers.
            "spacing"
            | "first-line-indent"''';new='''            "spacing" => {
                let span = args
                    .occurrences
                    .as_ref()
                    .and_then(|items| items.iter().rev().find(|item| item.name.as_deref() == Some("spacing")))
                    .map_or(args.span, |item| item.value_span);
                match value {
                    Value::Length(length) => {
                        styles = styles.push_custom("par.spacing", Value::Length(*length));
                    }
                    other => return Err(vec![expected_length_error(other, span)]),
                }
            }
            // Known properties awaiting their own consumers.
            "first-line-indent"''';assert old in s;s=s.replace(old,new);p.write_text(s)
