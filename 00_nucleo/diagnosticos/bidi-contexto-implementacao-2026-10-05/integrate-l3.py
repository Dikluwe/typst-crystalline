from pathlib import Path
p=Path('03_infra/src/layout_bidi.rs');s=p.read_text();a=s.index('pub fn reorder_bidi_document(');b=s.index('fn reorder_bidi_page(',a)
s=s[:a]+'''pub fn reorder_bidi_document(doc: PagedDocument, metrics: &dyn FontMetrics) -> PagedDocument {
    let registry=crate::bidi_context::Registry::new(&doc);let resolved=registry.resolve();
    reorder_bidi_document_with_context(doc,metrics,&resolved)
}
pub(crate) fn reorder_bidi_document_with_context(mut doc:PagedDocument,metrics:&dyn FontMetrics,resolved:&crate::bidi_context::Resolved<'_>)->PagedDocument {
    for page in &mut doc.pages {reorder_page_with_context(page,metrics,resolved);}doc
}
#[cfg(test)]
fn reorder_bidi_page(page:&mut Page,metrics:&dyn FontMetrics) {
    let doc=PagedDocument::new(vec![page.clone()]);let registry=crate::bidi_context::Registry::new(&doc);let resolved=registry.resolve();
    reorder_page_with_context(page,metrics,&resolved);
}
'''+s[b:];s=s.replace('fn reorder_bidi_page(page: &mut Page, metrics: &dyn FontMetrics) {','fn reorder_page_with_context(page: &mut Page, metrics: &dyn FontMetrics, resolved:&crate::bidi_context::Resolved<\'_>) {',1)
a=s.index('    // P1140.14');b=s.index('\n}\n',a)
s=s[:a]+'''    let mut replacements=std::collections::BTreeMap::new();
    let mut removed=std::collections::BTreeSet::new();
    let mut fallback_rtl_baselines=Vec::new();
    for (i,(y,line)) in lines.iter().enumerate() {
        if let Some((indices,visual))=context_line(&page.items,line,metrics,resolved) {
            let first=*indices.first().expect("certified nonempty line");
            replacements.insert(first,visual);removed.extend(indices.into_iter().skip(1));
        } else if line_is_rtl[i] {
            // A partial certificate must not be rewritten by a heuristic.
            if line.iter().any(|&i|crate::bidi_context::span(&page.items[i]).is_some()) {continue;}
            reorder_bidi_line(&mut page.items,line,metrics);fallback_rtl_baselines.push(*y);
        }
    }
    if !replacements.is_empty() {
        let mut output=Vec::new();
        for (index,item) in std::mem::take(&mut page.items).into_iter().enumerate() {
            if let Some(visual)=replacements.remove(&index){output.extend(visual);}
            else if !removed.contains(&index){output.push(item);}
        }
        page.items=output;
    }
    let suffix_indices:Vec<_>=page.items.iter().enumerate().filter(|(_,item)| {
        crate::bidi_context::span(item).is_none() && fallback_rtl_baselines.iter().any(|y|(item_baseline_y(item).0-y).abs()<=Y_TOLERANCE_PT)
    }).map(|(i,_)|i).collect();
    split_ltr_suffixes_line(&mut page.items,&suffix_indices,metrics);'''+s[b:]
p.write_text(s)
# insert exact-intersection implementation before legacy helpers
s=p.read_text();a=s.index('/// Devolve a baseline')
s=s[:a]+'''fn context_line(items:&[FrameItem],line:&[usize],metrics:&dyn FontMetrics,resolved:&crate::bidi_context::Resolved<'_>)->Option<(Vec<usize>,Vec<FrameItem>)> {
    use std::sync::Arc;
    let indices:Vec<_>=line.iter().copied().filter(|&i|matches!(items[i],FrameItem::Text {..}|FrameItem::TextSeparator {..})).collect();
    let first=crate::bidi_context::span(&items[*indices.first()?])?;
    let mut sources=Vec::new();
    for &index in &indices {
        let item=&items[index];let span=crate::bidi_context::span(item)?;
        if !Arc::ptr_eq(first.input(),span.input()){return None;}
        let text=match item{FrameItem::Text{text,..}=>text.as_str(),FrameItem::TextSeparator{..}=>" ",_=>return None};
        if !resolved.matches(span,text){return None;}
        sources.push((index,resolved.mapped(span)?));
    }
    let start=sources.iter().map(|(_,r)|r.start).min()?;let end=sources.iter().map(|(_,r)|r.end).max()?;
    let mut visual=Vec::new();let mut widths=Vec::new();
    for (range,rtl) in resolved.line_runs(first,start..end)? {
        let mut pieces=Vec::new();
        for (index,mapped) in &sources {
            let a=range.start.max(mapped.start);let b=range.end.min(mapped.end);if a>=b{continue;}
            let item=&items[*index];let mut piece=item.clone();
            let width=match &mut piece {
                FrameItem::Text{text,style,bidi,..}=>{
                    let relative=a-mapped.start..b-mapped.start;
                    *text=text.get(relative.clone())?.into();
                    *bidi=bidi.as_ref()?.subspan(relative);
                    text_width_for_bidi(metrics,text,style)
                }
                FrameItem::TextSeparator{advance,..}=>{if a!=mapped.start||b!=mapped.end{return None;}advance.0}
                _=>return None,
            };
            pieces.push((a,piece,width));
        }
        pieces.sort_by_key(|p|p.0);if rtl{pieces.reverse();}
        for (_,piece,width) in pieces{visual.push(piece);widths.push(width);}
    }
    if visual.is_empty(){return None;}
    let mut x=if first.input().base_dir()==typst_core::entities::dir::Dir::RTL {
        let refs:Vec<_>=indices.iter().map(|&i|&items[i]).collect();metrics.line_content_right(&refs)-widths.iter().sum::<f64>()
    }else{indices.iter().map(|&i|item_x(&items[i])).fold(f64::INFINITY,f64::min)};
    for (item,width) in visual.iter_mut().zip(widths){
        match item{FrameItem::Text{pos,..}|FrameItem::TextSeparator{pos,..}=>pos.x=Pt(x),_=>return None}x+=width;
    }
    Some((indices,visual))
}

'''+s[a:];p.write_text(s)
# shaper keeps old test helper and delegates to a context-aware inner method.
p=Path('03_infra/src/shaper/shaping.rs');s=p.read_text();a=s.index('    // P568',s.index('pub(super) fn try_shape('));s=s[:a]+'''    try_shape_context(world,pos,text,style,cache,face_cache,None,None)
}
pub(super) fn try_shape_context(world:&dyn World,pos:&Point,text:&ecow::EcoString,style:&TextStyle,cache:&mut ShapeCache,face_cache:&mut FaceCache,bidi:Option<&typst_core::entities::bidi_context::BidiSpan>,resolved:Option<&crate::bidi_context::Resolved<'_>>)->Option<Vec<FrameItem>> {
'''+s[a:];s=s.replace('    let paragraphs = bidi_runs(text.as_str());','''    let paragraphs = if let (Some(span),Some(resolved))=(bidi,resolved) {
        vec![resolved.runs(span,text)?.into_iter().map(|(r,rtl)|super::bidi::BidiRun{text:text[r.clone()].to_string(),rtl,byte_start:r.start}).collect()]
    } else {bidi_runs(text.as_str())};''',1)
s=s.replace('        for run in runs {\n            for subrun in split_run_by_font(run, &mut candidates) {','        for run in runs {\n            let mut subrun_start=run.byte_start;\n            for subrun in split_run_by_font(run, &mut candidates) {',1 if False else 0) if False else s
# target second loop only
start=s.index('pub(super) fn try_shape_context');part=s[start:];part=part.replace('        for run in runs {\n            for subrun in split_run_by_font(run, &mut candidates) {','''        for run in runs {
            let mut subrun_start=run.byte_start;
            for subrun in split_run_by_font(run, &mut candidates) {
                let subrun_range=subrun_start..subrun_start+subrun.text.len();
                subrun_start=subrun_range.end;''',1).replace('                     bidi: None,','                        bidi: bidi.and_then(|span|span.subspan(subrun_range)),',1)
s=s[:start]+part;p.write_text(s)
# pass resolver only through existing private traversal
p=Path('03_infra/src/shaper/traversal.rs');s=p.read_text().replace('use super::shaping::try_shape;','use super::shaping::try_shape_context;');s=s.replace('pub fn shape_document(world: &dyn World, mut doc: PagedDocument) -> PagedDocument {','''pub fn shape_document(world: &dyn World, doc: PagedDocument) -> PagedDocument {
    let registry=crate::bidi_context::Registry::new(&doc);let resolved=registry.resolve();
    shape_document_with_context(world,doc,&resolved)
}
pub(crate) fn shape_document_with_context(world:&dyn World,mut doc:PagedDocument,resolved:&crate::bidi_context::Resolved<'_>)->PagedDocument {''',1).replace('shape_page(world, page, &mut cache, &mut face_cache);','shape_page(world, page, &mut cache, &mut face_cache, resolved);',1)
s=s.replace('    face_cache: &mut FaceCache,\n) {','    face_cache: &mut FaceCache,\n    resolved: &crate::bidi_context::Resolved<\'_>,\n) {',1).replace('        face_cache: &mut FaceCache,\n    ) {','        face_cache: &mut FaceCache,\n        resolved: &crate::bidi_context::Resolved<\'_>,\n    ) {',1).replace('shaped.extend(shape_item(world, item, cache, face_cache));','shaped.extend(shape_item_inner(world, item, cache, face_cache, false, Some(resolved)));',1)
for layer in ['background','items','foreground']:s=s.replace(f'shape_layer(world, &mut page.{layer}, cache, face_cache);',f'shape_layer(world, &mut page.{layer}, cache, face_cache, resolved);')
s=s.replace('shape_item_inner(world, item, cache, face_cache, false)','shape_item_inner(world, item, cache, face_cache, false, None)',1).replace('    inside_replacement: bool,\n','    inside_replacement: bool,\n    resolved: Option<&crate::bidi_context::Resolved<\'_>>,\n',1).replace('FrameItem::Text { pos, text, style , .. }','FrameItem::Text { pos, text, style, bidi }',1).replace('try_shape(world, pos, text, style, cache, face_cache)','try_shape_context(world, pos, text, style, cache, face_cache, bidi.as_ref(), resolved)',1)
s=s.replace('face_cache, inside_replacement))','face_cache, inside_replacement, resolved))').replace('face_cache, true))','face_cache, true, resolved))');p.write_text(s)
p=Path('03_infra/src/shaper.rs');s=p.read_text()+'\npub(crate) use traversal::shape_document_with_context;\n';p.write_text(s)
p=Path('03_infra/src/pipeline/paged.rs');s=p.read_text();a=s.index('    let doc = match ordinary_layout_metrics.as_ref().or(selected_layout_metrics.as_ref()) {',s.index('let t4 ='));s=s[:a]+'''    let bidi_registry=crate::bidi_context::Registry::new(&doc);
    let bidi_resolved=bidi_registry.resolve();
'''+s[a:];s=s.replace('crate::layout_bidi::reorder_bidi_document(doc, metrics)','crate::layout_bidi::reorder_bidi_document_with_context(doc, metrics, &bidi_resolved)').replace('crate::layout_bidi::reorder_bidi_document(doc, &FallbackFontMetrics::new(world))','crate::layout_bidi::reorder_bidi_document_with_context(doc, &FallbackFontMetrics::new(world), &bidi_resolved)').replace('crate::shaper::shape_document(world, doc)','crate::shaper::shape_document_with_context(world, doc, &bidi_resolved)');p.write_text(s)
p=Path('03_infra/src/shaper/reconciliation.rs');s=p.read_text().replace('        let is_rtl_line = sorted.iter().any','        let certified_dir=sorted.iter().find_map(|&i|crate::bidi_context::span(&page.items[i]).map(|s|s.input().base_dir()));\n        let is_rtl_line = certified_dir.map(|d|d==Dir::RTL).unwrap_or_else(||sorted.iter().any',1).replace('        });\n\n        if is_rtl_line {','        }));\n\n        if is_rtl_line {',1);p.write_text(s)
