from pathlib import Path
p=Path('00_nucleo/prompts/compiler/layout/bidi_origin.md');p.write_text(p.read_text()+'\nRecusa integral não permite recertificar um filho parcial: guard privado de escopo em Sequence/Par impede reentrada; subframe suspende o guard com o cursor. Validar None para sequências com objetos opacos e folhas separadas.\n')
for owner in ['compiler/layout','compiler/layout/sequence','compiler/layout/paragraph','compiler/layout/checkpoint','compiler/layout/sub_frame']:
 p=Path('00_nucleo/prompts')/(owner+'.md');p.write_text(p.read_text()+'\nGuard privado bidi_scope_active distingue ausência de cursor certificado de ausência de escopo; após recusa integral, filhos não certificam um subconjunto parcial. Checkpoint e subframe conservam/suspendem/restauram este guard.\n')
p=Path('01_core/src/compiler/layout/mod.rs');s=p.read_text().replace('    pub(super) bidi_cursor:', '    pub(super) bidi_scope_active: bool,\n    pub(super) bidi_cursor:',1).replace('            bidi_cursor: None,','            bidi_scope_active: false,\n            bidi_cursor: None,',1);p.write_text(s)
p=Path('01_core/src/compiler/layout/sequence.rs');s=p.read_text().replace('    let enclosing_bidi = layouter.bidi_cursor.clone();','    let enclosing_scope = layouter.bidi_scope_active;\n    layouter.bidi_scope_active = true;',1).replace('if enclosing_bidi.is_none() {','if !enclosing_scope {').replace('    if enclosing_bidi.is_none() { layouter.bidi_cursor = None; }','    if !enclosing_scope { layouter.bidi_cursor = None; }') # previous replace hits one-line too
s=s.replace('    if !enclosing_scope { layouter.bidi_cursor = None; }','    if !enclosing_scope { layouter.bidi_cursor = None; }\n    layouter.bidi_scope_active = enclosing_scope;',1);p.write_text(s)
p=Path('01_core/src/compiler/layout/paragraph.rs');s=p.read_text().replace('    let previous_bidi = layouter.bidi_cursor.take();','    let previous_bidi = layouter.bidi_cursor.take();\n    let previous_scope = layouter.bidi_scope_active;\n    layouter.bidi_scope_active = true;',1).replace('    layouter.bidi_cursor = previous_bidi;','    layouter.bidi_cursor = previous_bidi;\n    layouter.bidi_scope_active = previous_scope;',1)
a=s.index('    fn audit_content(');s=s[:a]+'''    #[test]
    fn source_bidi_spans_survive_wraps_translation_and_paragraph_boundaries() {
        use crate::entities::{bidi_context::ParagraphBidiSegment,dir::Dir};
        for source in ["#set text(dir: rtl)\\nABC aa aa مرحبا", "#set text(dir: ltr)\\n123 456 مرحبا", "#set text(dir: rtl)\\n#text(dir: ltr)[ABC 123] مرحبا\\n\\nعالم"] {
            let intr=TagIntrospector::empty();let mut l=make(&intr,false);
            l.layout_content(&audit_content(source));let doc=l.finish();let mut count=0;
            for page in &doc.pages {for item in &page.items {
                if let FrameItem::Text{text,bidi:Some(span),..}=item {
                    let ParagraphBidiSegment::Text{text:original,..}=&span.input().segments()[span.segment()] else {panic!("text span must refer to text");};
                    assert_eq!(original.get(span.range()).unwrap(),text.as_str());count+=1;
                    assert!(matches!(span.input().base_dir(),Dir::RTL|Dir::LTR));
                }
            }}
            assert!(count>=3,"missing complete paragraph context for {source}");
        }
    }

'''+s[a:];p.write_text(s)
p=Path('01_core/src/compiler/layout/checkpoint.rs');s=p.read_text().replace('    bidi_cursor:', '    bidi_scope_active: bool,\n    bidi_cursor:',1).replace('            bidi_cursor: self.bidi_cursor.clone(),','            bidi_scope_active: self.bidi_scope_active,\n            bidi_cursor: self.bidi_cursor.clone(),',1).replace('        self.bidi_cursor = checkpoint.bidi_cursor;','        self.bidi_scope_active = checkpoint.bidi_scope_active;\n        self.bidi_cursor = checkpoint.bidi_cursor;',1);p.write_text(s)
p=Path('01_core/src/compiler/layout/sub_frame.rs');s=p.read_text().replace('        let saved_bidi_cursor = self.bidi_cursor.take();','        let saved_bidi_scope = std::mem::take(&mut self.bidi_scope_active);\n        let saved_bidi_cursor = self.bidi_cursor.take();').replace('        self.bidi_cursor = saved_bidi_cursor;','        self.bidi_scope_active = saved_bidi_scope;\n        self.bidi_cursor = saved_bidi_cursor;');p.write_text(s)
p=Path('01_core/src/compiler/layout/helpers.rs');s=p.read_text();a=s.index('pub(super) fn translate_frame_item');b=s.index('        FrameItem::Line',a);chunk=s[a:b].replace('origin, .. }','origin, bidi }').replace('text, style, .. }','text, style, bidi }').replace('units_per_em, .. }','units_per_em, bidi }').replace('bidi: None','bidi');s=s[:a]+chunk+s[b:];p.write_text(s)
