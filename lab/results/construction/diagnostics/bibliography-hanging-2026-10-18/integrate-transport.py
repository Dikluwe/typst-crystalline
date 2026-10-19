from pathlib import Path
f=Path('01_core/src/compiler/eval/rules/set.rs');s=f.read_text();a=s.index('if native_target.is_some_and(|call| {',s.index('"hanging-indent"') if '"hanging-indent"' in s else 0) if False else s.index('        // Passo 133: target `par`')
needle='                match key.as_str() {\n                    "leading" => {';assert s.count(needle)==1
s=s.replace(needle,'''                match key.as_str() {
                    "hanging-indent" => {
                        if let Value::Length(length) = val {
                            let prior = matches!(engine.styles.custom("eval.par.hanging-boundary"), Some(Value::Bool(true)));
                            *engine.styles = engine.styles
                                .push_custom("par.hanging-indent", Value::Length(length))
                                .push_custom("eval.par.hanging-boundary", Value::Bool(!prior));
                        }
                    }
                    "leading" => {''');f.write_text(s)
f=Path('01_core/src/compiler/stdlib/structural/par.rs');s=f.read_text();old='            "first-line-indent" | "hanging-indent" | "justification-limits" => {}';assert s.count(old)==1
s=s.replace(old,'''            "hanging-indent" => {
                if let Value::Length(length) = value {
                    styles = styles.push_custom("par.hanging-indent", Value::Length(*length));
                }
            }
            "first-line-indent" | "justification-limits" => {}''');f.write_text(s)
f=Path('01_core/src/compiler/eval/rules/apply.rs');s=f.read_text();s=s.replace('                mark_heading_show_set(&mut delta, node, &work);','                mark_heading_show_set(&mut delta, node, &work);\n                mark_bibliography_hanging(&mut delta, node, &work);').replace('    mark_heading_show_set(&mut delta, &node, &work);','    mark_heading_show_set(&mut delta, &node, &work);\n    mark_bibliography_hanging(&mut delta, &node, &work);')
pos=s.index('/// Productive locatables retain their identity')
s=s[:pos]+'''/// Preserve only an override actually matched to this bibliography occurrence.
fn mark_bibliography_hanging(
    delta: &mut crate::entities::style_chain::StyleDelta,
    original: &Content,
    visual: &Content,
) {
    fn bibliography(mut c: &Content) -> bool {
        loop {
            c = c.source_body();
            if let Content::Located(e) = c { c = &e.body; }
            else if let Content::Realized(e) = c { c = &e.visual; }
            else { return matches!(c, Content::Bibliography(_)); }
        }
    }
    if bibliography(original) && bibliography(visual) {
        let value = delta.custom.iter().rev().find_map(|(key, value)| {
            if key.as_str() == "par.hanging-indent" {
                if let Value::Length(length) = value { Some(*length) } else { None }
            } else { None }
        });
        if let Some(length) = value {
            delta.custom.push(("bibliography.hanging-override".into(), Value::Length(length)));
        }
    }
}

'''+s[pos:];f.write_text(s)
f=Path('01_core/src/compiler/layout/bibliography.rs');s=f.read_text();old='''            // Hanging transport is retained; its physical paragraph consumer
            // belongs to the next A2-HangingTransport/A2-ParIndent step.
            BibliographyPlan::Hanging(entries) | BibliographyPlan::Plain(entries) => {''';assert s.count(old)==1
s=s.replace(old,'''            BibliographyPlan::Hanging(entries) => layout_hanging(layouter, entries),
            BibliographyPlan::Plain(entries) => {''')
needle='        match plan.as_ref() {';assert s.count(needle)==1
s=s.replace(needle,'''        let bibliography_chain = layouter.chain.clone();
        layouter.chain = layouter.chain.push_custom("par.hanging-indent", crate::entities::value::Value::Length(crate::entities::layout_types::Length::ZERO));
'''+needle)
needle='''        }
        return;
    }
    if !body.is_empty()''';assert s.count(needle)==1
s=s.replace(needle,'''        }
        layouter.chain = bibliography_chain;
        return;
    }
    if !body.is_empty()''')
pos=s.index('fn entry_block(')
s=s[:pos]+'''fn layout_hanging<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    entries: &[super::bib_csl::plan::Entry],
) {
    use crate::entities::{dir::Dir, layout_types::Length, sides::Sides, value::Value};
    let amount = match layouter.chain.custom("bibliography.hanging-override") {
        Some(Value::Length(length)) => *length,
        _ => Length::em(1.5),
    }.resolve_pt(layouter.style.size.0);
    let dir = super::text::effective_style(layouter).dir.unwrap_or(Dir::LTR);
    let inset = Sides {
        top: Length::ZERO, bottom: Length::ZERO,
        left: Length::pt(if dir == Dir::LTR { amount } else { 0.0 }),
        right: Length::pt(if dir == Dir::RTL { amount } else { 0.0 }),
    };
    for entry in entries {
        let previous = layouter.pending_bib_escape.replace(super::paragraph_indent::Spec::escape(amount, dir));
        layouter.layout_content(&Content::block(entry.body.clone(), None, None, inset, true));
        layouter.pending_bib_escape = previous;
    }
}

'''+s[pos:];f.write_text(s)
