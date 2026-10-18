from pathlib import Path
base=Path('01_core/src/compiler/layout')
def edit(name,pairs):
 f=base/name;s=f.read_text()
 for old,new in pairs:
  assert s.count(old)==1,(name,old,s.count(old));s=s.replace(old,new)
 f.write_text(s)
edit('paragraph.rs',[
 ('    parbreak::layout(layouter);\n    let previous_dir', '    parbreak::layout(layouter);\n    let previous_indent = std::mem::take(&mut layouter.paragraph_indent);\n    super::paragraph_indent::start_scope(layouter, std::slice::from_ref(body));\n    let previous_dir'),
 ('    layouter.active_paragraph_dir = previous_dir;\n    layouter.bidi_cursor = previous_bidi;', '    super::paragraph_indent::end(layouter);\n    layouter.paragraph_indent = previous_indent;\n    layouter.active_paragraph_dir = previous_dir;\n    layouter.bidi_cursor = previous_bidi;')])
edit('block.rs',[
 ('    let previous_planning =\n        std::mem::replace', '    let previous_indent = super::paragraph_indent::enter_fragment(layouter, body);\n    let previous_planning =\n        std::mem::replace'),
 ('    layouter.paragraph_planning_suppressed = previous_planning;','    layouter.paragraph_planning_suppressed = previous_planning;\n    layouter.paragraph_indent = previous_indent;')])
for name,var,body,flush in [('heading.rs','heading_indent','body','    let heading_baseline ='),('title.rs','title_indent','body','    layouter.flush_line();\n    layouter.style = prev;')]:
 f=base/name;s=f.read_text();needle='    layouter.layout_content(body);';assert s.count(needle)==1
 s=s.replace(needle,'    super::paragraph_indent::end(layouter);\n    let '+var+' = std::mem::replace(&mut layouter.paragraph_indent, super::paragraph_indent::State::inline());\n'+needle)
 if name=='heading.rs':
  needle='    layouter.style = prev;\n    layouter.chain = prev_chain;';s=s.replace(needle,'    layouter.paragraph_indent = '+var+';\n'+needle)
 else:s=s.replace(flush,'    layouter.flush_line();\n    layouter.paragraph_indent = '+var+';\n    layouter.style = prev;')
 f.write_text(s)
for name in ['list_item.rs','enum_item.rs']:
 edit(name,[('    layouter.layout_content(&e.body);\n    layouter.flush_line();', '    let body_indent_state = super::paragraph_indent::enter_fragment(layouter, &e.body);\n    layouter.layout_content(&e.body);\n    layouter.flush_line();\n    layouter.paragraph_indent = body_indent_state;')])
edit('quote.rs',[
 ('        let margin_pt = Pt(layouter.page_config.margin.left);', '        let quote_indent_state = super::paragraph_indent::enter_fragment(layouter, &e.body);\n        let margin_pt = Pt(layouter.page_config.margin.left);'),
 ('        layouter.regions.current.cursor_x = margin_pt;','        layouter.regions.current.cursor_x = margin_pt;\n        layouter.paragraph_indent = quote_indent_state;')])
# Raw returns a private local scope guard; its two productive callers restore it.
f=base/'raw.rs';s=f.read_text();a=s.index('pub(super) fn begin_block');b=s.index('pub(super) fn end_block',a);part=s[a:b];part=part.replace(') {',') -> super::paragraph_indent::State {',1);idx=part.rfind('\n}');assert idx>=0
part=part[:idx]+'''\n    super::paragraph_indent::end(l);
    std::mem::replace(&mut l.paragraph_indent, super::paragraph_indent::State::inline())
'''+part[idx:];s=s[:a]+part+s[b:]
s=s.replace('    if owns_block {\n        begin_block(layouter, block_font, &prev);\n    }','    let raw_indent_state = owns_block.then(|| begin_block(layouter, block_font, &prev));')
s=s.replace('    if owns_block {\n        end_block(layouter, block_font);\n    }','    if let Some(state) = raw_indent_state {\n        end_block(layouter, block_font);\n        layouter.paragraph_indent = state;\n    }');f.write_text(s)
f=base/'style_scope.rs';s=f.read_text();needle='    if super::paragraph::is_paragraph(body) {';assert s.count(needle)==1
s=s.replace(needle,'''    let hanging_boundary = styles.delta().custom.iter().any(|(key, _)| {
        key.as_str() == "par.hanging-indent" || key.as_str() == "eval.par.hanging-boundary"
    });
    if hanging_boundary { super::parbreak::layout(layouter); }
'''+needle)
s=s.replace('    if owns_raw_block {\n        super::raw::begin_block(layouter, block_font, &prev_style);\n    }','    let raw_indent_state = owns_raw_block.then(|| super::raw::begin_block(layouter, block_font, &prev_style));')
s=s.replace('    if owns_raw_block {\n        super::raw::end_block(layouter, block_font);\n    }','    if let Some(state) = raw_indent_state {\n        super::raw::end_block(layouter, block_font);\n        layouter.paragraph_indent = state;\n    }')
needle='    layouter.chain = prev_chain;\n    layouter.style = prev_style;';assert s.count(needle)>=1
# Only Styled owns this declaration; Strong/Emph must remain in the same paragraph.
a=s.index('pub(super) fn layout_strong');part=s[:a];assert part.count(needle)==1;part=part.replace(needle,'    if hanging_boundary { super::parbreak::layout(layouter); }\n'+needle);s=part+s[a:];f.write_text(s)
# Size only the outer Box slot against the effective line. The local frame
# still receives the fragment context through SubFrame.
f=base/'boxed.rs';s=f.read_text();s=s.replace('layouter.available_width()', 'super::paragraph_indent::current_width(layouter, layouter.available_width())');f.write_text(s)
f=base/'region_progress.rs';s=f.read_text().replace('    }    super::paragraph_indent::reopen(layouter);','    }\n    super::paragraph_indent::reopen(layouter);');f.write_text(s)
