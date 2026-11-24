"""Carry page-operator slots across eligible document identities."""
from pathlib import Path
import runpy
import sys

here = Path(__file__).resolve().parent
runpy.run_path(str(here.parent / "references-500-visible-edit-one-root-2026-11-21" / "prototype.py"))
root = Path(sys.argv[1]) / "03_infra/src/export"
cache = root / "lab_pdf_reuse.rs"
builder = root / "builder/fonts.rs"


def edit(path, before, after):
    text = path.read_text()
    assert text.count(before) == 1, (path, before)
    path.write_text(text.replace(before, after))


edit(cache, 'layout_types::{FrameItem, PagedDocument}', 'layout_types::{FrameItem, Page, PagedDocument, Point, TextStyle}')
edit(cache, '    pub operator_maps: Option<OperatorMaps>,',
    '    pub operator_maps: Option<OperatorMaps>,\n    pub previous_document: Option<Arc<PagedDocument>>,')
edit(cache, '''            self.scans = None;
            self.operator_maps = None;''', '''            self.scans = None;
            self.previous_document = None;
            self.operator_maps = None;''')
edit(cache, '''            self.entry = None;
            if !eligible(document) {''', '''            let prior = self.entry.take();
            if !eligible(document) {''')
edit(cache, '''            self.entry = Some(Entry {
                document: document.clone(), epoch, key, requested: mode,
                parts: Parts { mode, ..Parts::default() },
            });''', '''            let mut parts = Parts { mode, ..Parts::default() };
            if let Some(old) = prior.filter(|old| mode.operators
                && Arc::ptr_eq(&old.epoch, &epoch) && old.key == key
                && old.requested == mode && old.parts.operator_maps.is_some())
            {
                parts.previous_document = Some(old.document);
                parts.basis = old.parts.basis;
                parts.operator_maps = old.parts.operator_maps;
                parts.operators = old.parts.operators;
                parts.operator_bytes = old.parts.operator_bytes;
            }
            self.entry = Some(Entry { document: document.clone(), epoch, key, requested: mode, parts });''')
with cache.open('a') as output:
    output.write('\n' + (here/'page_operators.rs').read_text())

text = builder.read_text()
start = text.index('        if may_reuse_operators {', text.index('        let may_reuse_operators = '))
end = text.index('        for (i, page) in doc.pages.iter().enumerate()',start)
text = text[:start] + '''        let mut previous_document = None;
        let mut translation = None;
        let mut saved_operators = Vec::new();
        if let Some(parts) = &mut parts {
            let previous = parts.previous_document.take();
            let prior_ops = std::mem::take(&mut parts.operators);
            parts.operator_bytes = 0;
            if may_reuse_operators {
                let maps = crate::export::lab_pdf_reuse::OperatorMaps {
                    character: per_font_char_to_gid.clone(), remap: per_font_glyph_mapping.clone(),
                    nominal: per_font_glyph_to_nominal.clone(), reverse: per_font_glyph_reverse.clone(),
                };
                if let Some(old) = parts.operator_maps.take() {
                    if old == maps {
                        saved_operators = prior_ops;
                    } else if previous.is_some() {
                        translation = Some(crate::export::lab_pdf_reuse::cid_translation(&old, &maps));
                        saved_operators = prior_ops;
                    }
                }
                previous_document = previous;
                parts.operator_maps = Some(maps);
            }
        }
        let mut operator_hits = 0;
        let mut remapped = 0;
        let trace = std::env::var_os("TEKT_LAB_WATCH_TRACE").is_some();
        let mut reemitted = Vec::new();
''' + text[end:]
builder.write_text(text)
edit(builder, '''            if may_reuse_operators {
                if let Some(saved) = parts.as_ref().and_then(|parts| parts.operators.get(i))
                    .and_then(Option::as_ref)
                {
                    let stream_object = self.serialize_page_stream(saved);
                    self.add_bytes(stream_id, stream_object);
                    operator_hits += 1;
                    continue;
                }
            }
            let mut ctx = PageContext::with_font_scenario(''', '''            let saved = saved_operators.get_mut(i).and_then(Option::take).filter(|_| {
                previous_document.as_ref().is_none_or(|old| old.pages.get(i).is_some_and(|before|
                    crate::export::lab_pdf_reuse::same_page(before, page)))
            }).and_then(|cached| match &translation {
                None => Some(cached),
                Some(maps) => crate::export::lab_pdf_reuse::translate_operators(&cached, maps),
            });
            if let Some(saved) = saved {
                let stream_object = self.serialize_page_stream(&saved);
                self.add_bytes(stream_id, stream_object);
                operator_hits += 1;
                remapped += usize::from(translation.is_some());
                if let Some(parts) = &mut parts {
                    parts.store_operators(i, doc.pages.len(), saved);
                }
                continue;
            }
            if trace { reemitted.push(i+1); }

            let mut ctx = PageContext::with_font_scenario(''')
start = builder.read_text().index('                    if parts.mode.operators && stream_bytes.len()')
text = builder.read_text()
end = text.index('        if std::env::var_os("TEKT_LAB_WATCH_TRACE").is_some()',start)
text = text[:start] + '''                    parts.store_operators(i, doc.pages.len(), stream_bytes.into());
                }
            }
        }
        if trace {
            eprintln!("pdf-page-operators reused={operator_hits} remapped={remapped} reemitted={reemitted:?}");
        }
''' + text[end:]
builder.write_text(text)
edit(cache, '''impl Parts {
    pub fn bind_basis''', '''impl Parts {
    pub(crate) fn store_operators(&mut self, index: usize, pages: usize, operators: Arc<[u8]>) {
        if self.mode.operators && operators.len() <= MAX_BYTES - self.operator_bytes {
            self.operators.resize_with(pages, || None);
            self.operator_bytes += operators.len();
            self.operators[index] = Some(operators);
        } else {
            self.mode.operators = false;
            self.operators.clear();
            self.operator_bytes = 0;
        }
    }

    pub fn bind_basis''')
