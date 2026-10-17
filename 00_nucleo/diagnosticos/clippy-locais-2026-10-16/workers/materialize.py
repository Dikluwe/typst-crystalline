from pathlib import Path
import json,re,subprocess,datetime,gzip,difflib,hashlib
out=Path('00_nucleo/diagnosticos/clippy-locais-2026-10-16')
assert json.loads((out/'red-details.json').read_text())['exit']==101
assert json.loads((out/'red-tests-details.json').read_text())['exit']==101
before=json.load(gzip.open(out/'before.json.gz'))
for p in before['plans']:assert hashlib.sha256(Path(p).read_bytes()).hexdigest()==before['files'][p]['sha256'],p
records=[]
def edit(path,old,new):
 p=Path(path);s=p.read_text();assert s.count(old)==1,(path,old);p.write_text(s.replace(old,new,1))
p='03_infra/src/export/svg.rs'
edit(p,'#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]\npub struct SvgOptions','#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]\npub struct SvgOptions')
edit(p,'impl Default for SvgOptions {\n    fn default() -> Self {\n        Self { pretty: false, render_bleed: false }\n    }\n}\n\n','')
p='03_infra/src/export/mod.rs'
for name,variant,body in [('StreamMode','Verbose','StreamMode::Verbose'),('PdfTags','Enabled','Self::Enabled')]:
 edit(p,f'#[derive(Debug, Clone, Copy, PartialEq, Eq)]\npub enum {name} {{\n    {variant},',f'#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]\npub enum {name} {{\n    #[default]\n    {variant},')
 edit(p,f'impl Default for {name} {{\n    fn default() -> Self {{\n        {body}\n    }}\n}}\n\n','')
edit('03_infra/src/shaper/cache.rs','''        if !self.map.contains_key(&slot_idx) {
            let cached = world.font(slot_idx).and_then(CachedFace::new);
            self.map.insert(slot_idx, cached);
        }
        self.map
            .get(&slot_idx)
            .and_then(|o| o.as_ref())
            .map(|arc| arc.as_ref())''','''        self.map
            .entry(slot_idx)
            .or_insert_with(|| world.font(slot_idx).and_then(CachedFace::new))
            .as_deref()''')
for name,module in [('html','p1465_tests'),('svg','p1462_tests')]:
 p=Path(f'03_infra/src/pipeline/{name}.rs');s=p.read_text();start=s.index(f'#[cfg(test)]\nmod {module} {{');end=s.index('\n}\n',start)+3;block=s[start:end]
 p.write_text(s[:start]+s[end:].rstrip()+'\n\n'+block)
 records.append(dict(source=str(p),moved_test_module=module,old_block=block))
edit('03_infra/src/watch.rs','    thread: Option<thread::JoinHandle<()>>,\n    live: bool,','    thread: Option<thread::JoinHandle<()>>,')
edit('03_infra/src/watch.rs','Ok(Self { addr, state, stop, thread: Some(thread), live })','Ok(Self { addr, state, stop, thread: Some(thread) })')
edit('03_infra/src/plugin_host/tests.rs','const I64: u8 = 0x7e;\n','')
for number in ['1467','1468','1469']:
 p=next(Path('04_wiring/tests').glob(f'p{number}_*.rs'));s=p.read_text()
 def replacement(match):
  n=match[1]
  return rf'assert!({n}.success(), "stdout:\n{{}}\nstderr:\n{{}}", {n}.stdout, {n}.stderr);'
 s,count=re.subn(r'assert!\((\w+)\.success\(\), "stderr:\\n\{\}", \1\.stderr\);',replacement,s)
 assert count>0,(p,count);p.write_text(s);records.append(dict(source=str(p),success_diagnostics_with_both_streams=count))
p=Path('03_infra/src/export/tests/mod.rs');s=p.read_text();start=s.index('#[test]\nfn p273_6_layouter_parent_bbox_consumed_by_block_arm()');end=s.index('\n#[test]',start+1)
test='''#[test]
fn p273_6_layouter_parent_bbox_consumed_by_block_arm() {
    use std::sync::Arc;
    use typst_core::entities::geometry::ShapeKind;
    use typst_core::entities::gradient::{Gradient, GradientStop, Linear};
    use typst_core::entities::layout_types::{Angle, ColorSpace, FrameItem, Length, Pt, Ratio, Rect};
    use typst_core::entities::paint::Paint;
    use typst_core::entities::sides::Sides;
    use typst_core::entities::value::Value;

    fn emitted_bbox(items: &[FrameItem]) -> Option<Rect> {
        items.iter().find_map(|item| match item {
            FrameItem::Shape { parent_bbox_at_emit, .. } => *parent_bbox_at_emit,
            FrameItem::Group { items, .. }
            | FrameItem::Link { items, .. }
            | FrameItem::Semantic { items, .. }
            | FrameItem::TextReplacement { items, .. } => emitted_bbox(items),
            FrameItem::TextSeparator { .. }
            | FrameItem::Text { .. }
            | FrameItem::TextShaped { .. }
            | FrameItem::Line { .. }
            | FrameItem::Glyph { .. }
            | FrameItem::Image { .. } => None,
        })
    }
    let gradient = Gradient::Linear(Arc::new(Linear {
        stops: Arc::from(vec![
            GradientStop::new(Color::rgb(255, 0, 0), Ratio(0.0)),
            GradientStop::new(Color::rgb(0, 0, 255), Ratio(1.0)),
        ]),
        angle: Angle::rad(0.0),
        space: ColorSpace::Oklab,
        relative: Some(RelativeTo::Parent),
        anti_alias: true,
    }));
    let shape = Content::shape(
        ShapeKind::Rect,
        Some(Box::new(Value::Length(Length::pt(50.0)))),
        Some(Box::new(Value::Length(Length::pt(30.0)))),
        Some(Paint::Gradient(gradient)),
        None,
    );
    let block = Content::block(
        shape.clone(),
        Some(Length::pt(200.0)),
        Some(Length::pt(100.0)),
        Sides::uniform(Length::ZERO),
        false,
    );
    let document = layout(&block);
    assert_eq!(document.pages.len(), 1);
    let bbox = emitted_bbox(&document.pages[0].items).expect("Block must emit the parent bbox for its Shape");
    assert_eq!(bbox.w, Pt(200.0));
    assert_eq!(bbox.h, Pt(100.0));
    let coords = parent_test_linear_coords(&export_pdf(&document, StreamMode::Verbose));
    assert!((coords[2] - coords[0] - 200.0).abs() < 1e-6, "Parent gradient must span the Block width: {coords:?}");
    assert_ne!(coords, parent_test_linear_coords(&export_pdf(&layout(&shape), StreamMode::Verbose)));
}
'''
p.write_text(s[:start]+test+s[end:])
subprocess.run(['rustfmt','--edition','2021','--config','skip_children=true',*before['plans']],check=True)
(out/'materialization.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),records=records),indent=2)+'\n')
diff=''.join(''.join(difflib.unified_diff(v['text'].splitlines(True),Path(p).read_text().splitlines(True),fromfile=p+' (before)',tofile=p)) for p,v in before['files'].items())
(out/'stage-before-seal.diff').write_text(diff)
print('Materializadas as correções prescritas, sem mudanças de API pública.')
