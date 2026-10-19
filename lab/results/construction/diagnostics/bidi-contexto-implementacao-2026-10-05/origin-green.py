from pathlib import Path
p=Path('01_core/src/compiler/layout/bidi_origin.rs');s=p.read_text();s=s.replace('pub(super) fn prepare<M:FontMetrics,S:ImageSizer>(_l:&Layouter<M,S>,_parts:&[Content],_base:Dir)->Option<Cursor> { None }',r'''
fn collect(c:&Content, chain:&StyleChain, out:&mut Vec<ParagraphBidiSegment>) -> Option<()> {
    if matches!(chain.custom("text.hyphenate"),Some(Value::Bool(true)))
        || matches!(chain.custom("par.justify"),Some(Value::Bool(true)))
        || matches!(chain.custom("par.linebreaks"),Some(Value::Str(s)) if s.as_str()=="optimized")
        || chain.custom("text.tracking").is_some() {return None;}
    if let Content::Text(text)=c {
        if text.contains(['\n','\r','\t','\u{2028}','\u{2029}'])
            || text.split(' ').any(super::word_segment::word_needs_line_segmentation) {return None;}
        let dir=chain.custom("text.dir").and_then(|v|if let Value::Dir(d)=v {Some(*d)} else {None}).unwrap_or(Dir::LTR);
        if !dir.is_horizontal(){return None;}
        out.push(ParagraphBidiSegment::Text{text:text.clone(),dir});return Some(());
    }
    if matches!(c,Content::Empty){return Some(());}
    if matches!(c,Content::Space){out.push(ParagraphBidiSegment::Space{origin:TextSeparatorOrigin::Markup});return Some(());}
    if matches!(c,Content::SpaceFromNewline){out.push(ParagraphBidiSegment::Space{origin:TextSeparatorOrigin::Newline});return Some(());}
    if matches!(c,Content::Linebreak(_)){out.push(ParagraphBidiSegment::Linebreak);return Some(());}
    if let Content::Sequence(parts)=c{for p in parts.iter(){collect(p,chain,out)?;}return Some(());}
    if let Content::Styled(styled)=c{return collect(&styled.body,&chain.push_styles(&styled.styles),out);}
    if let Content::Strong(elem)=c{return collect(&elem.body,chain,out);}
    if let Content::Emph(elem)=c{return collect(&elem.body,chain,out);}
    None
}
pub(super) fn prepare<M:FontMetrics,S:ImageSizer>(l:&Layouter<M,S>,parts:&[Content],base:Dir)->Option<Cursor> {
    if l.is_sub_frame || l.column_mode || l.page_columns.is_some() || l.regions.cell.is_some()
        || l.smallcaps || super::text::effective_style(l).tracking.is_some() {return None;}
    let mut segments=Vec::new();for c in parts {collect(c,&l.chain,&mut segments)?;}
    let needed=base==Dir::RTL || segments.iter().any(|s|matches!(s,ParagraphBidiSegment::Text{text,dir} if *dir!=base || text.chars().any(|c|matches!(c,'\u{0590}'..='\u{08ff}'|'\u{200f}'|'\u{202a}'..='\u{202e}'|'\u{2066}'..='\u{2069}'))));
    if !needed {return None;}
    Some(Cursor{input:Arc::new(ParagraphBidiInput::new(base,segments)?),next:0})
}
''');p.write_text(s)
# sequence decides eligibility before trying global planning, then installs at each boundary.
p=Path('01_core/src/compiler/layout/sequence.rs');s=p.read_text();s=s.replace('    if !layouter.paragraph_planning_suppressed\n', '''    let needs_bidi = layouter.bidi_cursor.is_some() || parts.split(|p| matches!(p,Content::Parbreak)).any(|p| {
        super::paragraph::shared_direction(p,&layouter.chain).and_then(|d|super::bidi_origin::prepare(layouter,p,d)).is_some()
    });
    if !needs_bidi && !layouter.paragraph_planning_suppressed
''',1);s=s.replace('    let enclosing_dir = layouter.active_paragraph_dir;','    let enclosing_dir = layouter.active_paragraph_dir;\n    let enclosing_bidi = layouter.bidi_cursor.clone();',1);s=s.replace('            layouter.active_paragraph_dir = super::paragraph::shared_direction(&parts[part_index..end], &layouter.chain);','''            layouter.active_paragraph_dir = super::paragraph::shared_direction(&parts[part_index..end], &layouter.chain);
            if enclosing_bidi.is_none() {
                layouter.bidi_cursor = layouter.active_paragraph_dir.and_then(|d|super::bidi_origin::prepare(layouter,&parts[part_index..end],d));
            }''',1);s=s.replace('    layouter.active_paragraph_dir = enclosing_dir;','''    layouter.active_paragraph_dir = enclosing_dir;
    if enclosing_bidi.is_none() { layouter.bidi_cursor = None; }''',1);p.write_text(s)
# nested cursor is not restored to the unconsumed clone; consumed visits belong to parent!
p=Path('01_core/src/compiler/layout/paragraph.rs');s=p.read_text().replace('    let previous_dir = layouter.active_paragraph_dir;','    let previous_dir = layouter.active_paragraph_dir;\n    let previous_bidi = layouter.bidi_cursor.take();',1).replace('    let empty_body = is_empty_body(body);','    layouter.bidi_cursor = super::bidi_origin::prepare(layouter,std::slice::from_ref(body),chain_direction(&layouter.chain));\n    let empty_body = is_empty_body(body);',1).replace('if !empty_body && !super::paragraph_plan::try_layout(layouter, body)', 'if !empty_body && (layouter.bidi_cursor.is_some() || !super::paragraph_plan::try_layout(layouter, body))',1).replace('    layouter.active_paragraph_dir = previous_dir;','    layouter.active_paragraph_dir = previous_dir;\n    layouter.bidi_cursor = previous_bidi;',1);p.write_text(s)
p=Path('01_core/src/compiler/layout/text.rs');s=p.read_text().replace('    if text.is_empty() {','    let original = layouter.bidi_cursor.as_mut().and_then(|c|c.next());\n    let previous_span = layouter.text_bidi.take();\n    if text.is_empty() {',1).replace('                style: layouter.style.clone(),\n             bidi: None,','                style: layouter.style.clone(),\n                bidi: original,',1).replace('        layouter.style = prev_style;\n        return;','        layouter.style = prev_style;\n        layouter.text_bidi = previous_span;\n        return;',1).replace('    if !layouter.paragraph_planning_suppressed','    if original.is_none() && !layouter.paragraph_planning_suppressed',1).replace('    let mut parts = text.split', '    let mut offset = 0usize;\n    let mut parts = text.split',1).replace('        if !part.is_empty() {','        layouter.text_bidi = original.as_ref().and_then(|s|s.subspan(offset..offset+part.len()));\n        if !part.is_empty() {',1).replace('        if parts.peek().is_some() {','        offset += part.len();\n        if parts.peek().is_some() {\n            layouter.text_bidi = original.as_ref().and_then(|s|s.subspan(offset..offset+1));\n            offset += 1;',1).replace('    layouter.style = prev_style;\n}', '    layouter.style = prev_style;\n    layouter.text_bidi = previous_span;\n}',1);p.write_text(s)
p=Path('01_core/src/compiler/layout/space.rs');s=p.read_text().replace('            origin,\n         bidi: None,','            origin,\n            bidi: layouter.text_bidi.clone(),',1);s=s.replace('    // Não criar linha', '    let previous_span = layouter.text_bidi.take();\n    layouter.text_bidi = layouter.bidi_cursor.as_mut().and_then(|c|c.next());\n    // Não criar linha',1);start=s.index('pub(super) fn layout');end=s.index('\n}\n',start);s=s[:end]+'\n    layouter.text_bidi = previous_span;'+s[end:];p.write_text(s)
p=Path('01_core/src/compiler/layout/word_layout.rs');s=p.read_text().replace('        style: layouter.style.clone(),\n     bidi: None,','        style: layouter.style.clone(),\n        bidi: layouter.text_bidi.clone(),',1);p.write_text(s)
p=Path('01_core/src/compiler/layout/linebreak.rs');s=p.read_text().replace('    if let Some((x, y))','    if let Some(cursor) = layouter.bidi_cursor.as_mut() { cursor.next(); }\n    if let Some((x, y))',1);p.write_text(s)
p=Path('01_core/src/compiler/layout/checkpoint.rs');s=p.read_text().replace('    active_paragraph_dir:', '    bidi_cursor: Option<super::bidi_origin::Cursor>,\n    text_bidi: Option<crate::entities::bidi_context::BidiSpan>,\n    active_paragraph_dir:',1).replace('            active_paragraph_dir: self.active_paragraph_dir,','            bidi_cursor: self.bidi_cursor.clone(),\n            text_bidi: self.text_bidi.clone(),\n            active_paragraph_dir: self.active_paragraph_dir,',1).replace('        self.active_paragraph_dir = checkpoint.active_paragraph_dir;','        self.bidi_cursor = checkpoint.bidi_cursor;\n        self.text_bidi = checkpoint.text_bidi;\n        self.active_paragraph_dir = checkpoint.active_paragraph_dir;',1);p.write_text(s)
p=Path('01_core/src/compiler/layout/sub_frame.rs');s=p.read_text().replace('        let saved_paragraph_dir = self.active_paragraph_dir.take();','        let saved_bidi_cursor = self.bidi_cursor.take();\n        let saved_text_bidi = self.text_bidi.take();\n        let saved_paragraph_dir = self.active_paragraph_dir.take();').replace('        self.active_paragraph_dir = saved_paragraph_dir;','        self.bidi_cursor = saved_bidi_cursor;\n        self.text_bidi = saved_text_bidi;\n        self.active_paragraph_dir = saved_paragraph_dir;');p.write_text(s)
