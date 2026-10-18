from pathlib import Path
base=Path('01_core/src/compiler/layout')
def edit(name,replacements):
 f=base/name;s=f.read_text()
 for old,new in replacements:
  assert s.count(old)==1,(name,old,s.count(old));s=s.replace(old,new)
 f.write_text(s)
edit('mod.rs',[
 ('    pending_bib_prefix: Option<bibliography::PendingPrefix>,','    pending_bib_prefix: Option<bibliography::PendingPrefix>,\n    paragraph_indent: paragraph_indent::State,\n    pending_bib_escape: Option<paragraph_indent::Spec>,'),
 ('            pending_bib_prefix: None,','            pending_bib_prefix: None,\n            paragraph_indent: paragraph_indent::State::default(),\n            pending_bib_escape: None,'),
 ('    fn layout_content_body(&mut self, content: &Content) {','    fn layout_content_body(&mut self, content: &Content) {\n        paragraph_indent::before_content(self, content);'),
 ('        if matches!(content, Content::Equation(_) | Content::Image(_) | Content::Boxed(_))','        paragraph_indent::after_content(self, content);\n        if matches!(content, Content::Equation(_) | Content::Image(_) | Content::Boxed(_))')])
edit('checkpoint.rs',[
 ('    pending_bib_prefix: Option<super::bibliography::PendingPrefix>,','    pending_bib_prefix: Option<super::bibliography::PendingPrefix>,\n    paragraph_indent: super::paragraph_indent::State,\n    pending_bib_escape: Option<super::paragraph_indent::Spec>,'),
 ('            pending_bib_prefix: self.pending_bib_prefix.clone(),','            pending_bib_prefix: self.pending_bib_prefix.clone(),\n            paragraph_indent: self.paragraph_indent.clone(),\n            pending_bib_escape: self.pending_bib_escape,'),
 ('        self.pending_bib_prefix = checkpoint.pending_bib_prefix;','        self.pending_bib_prefix = checkpoint.pending_bib_prefix;\n        self.paragraph_indent = checkpoint.paragraph_indent;\n        self.pending_bib_escape = checkpoint.pending_bib_escape;')])
edit('sequence.rs', [('    let needs_bidi = layouter.bidi_cursor.is_some()', '    super::paragraph_indent::start_scope(layouter, parts);\n    let needs_bidi = layouter.bidi_cursor.is_some()')])
edit('text.rs', [('    let previous_span = layouter.text_bidi.take();','    super::paragraph_indent::start(layouter);\n    let previous_span = layouter.text_bidi.take();'),
 ('        super::bibliography::emit_pending_prefix(layouter);','        super::paragraph_indent::admit(layouter);\n        super::bibliography::emit_pending_prefix(layouter);')])
edit('text_baseline.rs',[('    layouter.ensure_initial_baseline();','    layouter.ensure_initial_baseline();\n    super::paragraph_indent::admit(layouter);')])
edit('word_layout.rs', [('    layouter.ensure_initial_baseline();\n    push_text(layouter, text.into(), width, replacement);','    layouter.ensure_initial_baseline();\n    super::paragraph_indent::admit(layouter);\n    push_text(layouter, text.into(), width, replacement);')])
f=base/'word_layout.rs';s=f.read_text();s=s.replace('layouter.regions.current.width - layouter.page_config.margin.right,','super::paragraph_indent::bounds(layouter).1,').replace('layouter.regions.current.line_start_x.0,','super::paragraph_indent::bounds(layouter).0,');f.write_text(s)
edit('space.rs',[('            > layouter.regions.current.width - layouter.page_config.margin.right','            > super::paragraph_indent::bounds(layouter).1')])
edit('cursor.rs',[('        let right_margin = self.regions.current.width - self.page_config.margin.right;','        let right_margin = super::paragraph_indent::bounds(self).1;')])
f=base/'line_commit.rs';s=f.read_text();assert s.count('let right_margin = self.regions.current.width - self.page_config.margin.right;')==2;s=s.replace('let right_margin = self.regions.current.width - self.page_config.margin.right;','let right_margin = super::paragraph_indent::bounds(self).1;');s=s.replace('        super::region_progress::post_line(self, had_items, effective_line_committed);','        super::paragraph_indent::commit(self, had_items);\n        super::region_progress::post_line(self, had_items, effective_line_committed);');f.write_text(s)
edit('parbreak.rs',[('        if let Some(end) = flow_end {\n            layouter.record_flow_placement_end(end);\n        }\n    }\n}', '        if let Some(end) = flow_end {\n            layouter.record_flow_placement_end(end);\n        }\n    }\n    super::paragraph_indent::end(layouter);\n}')])
# The checkpoint-local context is metadata; all existing region restoration stays in place.
edit('sub_frame.rs',[
 ('        let saved_pending_bib_prefix = self.pending_bib_prefix.take();','        let saved_pending_bib_prefix = self.pending_bib_prefix.take();\n        let saved_pending_bib_escape = self.pending_bib_escape.take();\n        let saved_paragraph_indent = std::mem::replace(&mut self.paragraph_indent, super::paragraph_indent::State::fragment(content));'),
 ('        self.pending_bib_prefix = saved_pending_bib_prefix;','        self.pending_bib_prefix = saved_pending_bib_prefix;\n        self.pending_bib_escape = saved_pending_bib_escape;\n        self.paragraph_indent = saved_paragraph_indent;')])
# The non-isolated inline helper gets the same scope guard, with the outer cursor/line retained.
f=base/'sub_frame.rs';s=f.read_text();a=s.index('    pub(super) fn layout_sub_frame_inline(');b=s.index('\n    pub(super) fn ',a+5) if '\n    pub(super) fn ' in s[a+5:] else s.index('\n}',a)
part=s[a:b];needle='        self.layout_content(content);';assert part.count(needle)==1
part=part.replace(needle, '        let saved_indent = std::mem::replace(&mut self.paragraph_indent, super::paragraph_indent::State::fragment(content));\n        let saved_escape = self.pending_bib_escape.take();\n'+needle)
# Restore immediately before returning the result, after its tail was committed.
idx=part.rfind('\n        (');assert idx>=0,part[-700:]
part=part[:idx]+'\n        self.paragraph_indent = saved_indent;\n        self.pending_bib_escape = saved_escape;'+part[idx:];s=s[:a]+part+s[b:];f.write_text(s)
# Actual regional geometry is installed by its existing owners before reapplying offset.
f=base/'region_progress.rs';s=f.read_text();s=s.replace('            reset_closed_region_flow(layouter);\n            return;','            reset_closed_region_flow(layouter);\n            super::paragraph_indent::reopen(layouter);\n            return;');needle='    if layouter.block_inline_region.is_some() {\n        super::block::install_inline_region(layouter);\n    }\n}';assert s.count(needle)==1;s=s.replace(needle,needle[:-2]+'    super::paragraph_indent::reopen(layouter);\n}');f.write_text(s)
