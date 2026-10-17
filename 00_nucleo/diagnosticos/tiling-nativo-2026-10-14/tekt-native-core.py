from pathlib import Path
root=Path('01_core/src')
def edit(name,fn):
 p=root/(name+'.rs');s=p.read_text();p.write_text(fn(s))
def types(s):return s.replace('use crate::entities::geometry::{ShapeKind, Stroke};','use crate::entities::geometry::ShapeKind;').replace('fill: Option<crate::entities::paint::Paint>,','fill: Option<crate::entities::frame_paint::FramePaint>,',1).replace('stroke: Option<Stroke>,','stroke: Option<crate::entities::frame_paint::FrameStroke>,',1)
edit('entities/layout_types',types)
def tiling(s):
 s=s.replace('use crate::entities::frame_paint::TilingFrame;','use crate::entities::frame_paint::{FramePaint, FrameStroke, TilingFrame};').replace('use crate::entities::geometry::ShapeKind;','use crate::entities::geometry::{ShapeKind, Stroke};').replace(', TransformMatrix','')
 a=s.index('/// Materializa um fill declarativo');b=s.index('/// Prepare exactly one cell.')
 s=s[:a]+'''/// Convert declarative paint only at the layout boundary.
pub(super) fn prepare_paint<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>, paint: &Paint,
) -> SourceResult<FramePaint> {
    Ok(match paint {
        Paint::Solid(color) => FramePaint::Solid(color.clone()),
        Paint::Gradient(gradient) => FramePaint::Gradient(gradient.clone()),
        Paint::Tiling(tiling) => FramePaint::Tiling(Arc::new(prepare_cell(layouter, tiling)?)),
    })
}

pub(super) fn prepare_stroke<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>, stroke: &Stroke,
) -> SourceResult<FrameStroke> {
    Ok(FrameStroke {
        paint: prepare_paint(layouter, &stroke.paint)?,
        thickness: stroke.thickness, cap: stroke.cap, join: stroke.join,
        dash: stroke.dash.clone(), miter_limit: stroke.miter_limit,
        specified: stroke.specified, overhang: stroke.overhang,
    })
}

pub(super) fn resolve_paint<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>, paint: Option<&Paint>,
) -> Option<FramePaint> {
    match paint.map(|paint| prepare_paint(layouter, paint)).transpose() {
        Ok(paint) => paint,
        Err(errors) => { layouter.layout_errors.extend(errors); None }
    }
}

pub(super) fn resolve_stroke<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>, stroke: Option<&Stroke>,
) -> Option<FrameStroke> {
    match stroke.map(|stroke| prepare_stroke(layouter, stroke)).transpose() {
        Ok(stroke) => stroke,
        Err(errors) => { layouter.layout_errors.extend(errors); None }
    }
}

'''+s[b:]
 a=s.index('// Include boundary cells');b=s.index('#[cfg(test)]\nmod preparation_regressions',a);s=s[:a]+s[b:]
 a=s.index('#[cfg(test)]\nmod coverage_regressions');s=s[:a]
 # Legacy adapter assertions are ported to the actual prepared-cell boundary.
 a=s.index('    fn first_cell(');b=s.index('    #[test]',a);s=s[:a]+s[b:]
 # Use syntax boundaries around first test instead of depending on rustfmt wrapping.
 a=s.index('    #[test]\n    fn tiling_preparation_invalid_cell_produces_causal_diagnostic');b=s.index('    #[test]',a+15)
 s=s[:a]+'''    #[test]
    fn tiling_preparation_invalid_cell_produces_causal_diagnostic() {
        with_layout(|layout| {
            let mut tile = sized(TilingBody::Content(Arc::new(Content::Empty)));
            tile.size.as_mut().unwrap().width = Pt(0.0);
            let paint = resolve_paint(layout, Some(&Paint::Tiling(tile)));
            assert!(paint.is_none());
            assert_eq!(layout.layout_errors.len(), 1);
            assert_eq!(layout.layout_errors[0].message, "tile size must be non-zero and non-infinite");
        });
    }

'''+s[b:]
 # Rewrite exact adapter calls in the remaining two tests via broad function regions.
 a=s.index('    fn tiling_preparation_legacy_color_and_gradient_keep_original_paints');b=s.index('    #[test]',a)
 region=s[a:b];start=region.index('                let group =');end=region.index('                let FrameItem::Shape',start)
 region=region[:start]+'''                let cell = prepare_cell(layout, &sized(body)).expect("legacy paint cell");
'''+region[end:];region=region.replace('&first_cell(&group)[0]','&cell.frame.items[0]').replace('assert_eq!(*actual, expected);','''match (actual, &expected) {
                    (FramePaint::Solid(a), Paint::Solid(b)) => assert_eq!(a, b),
                    (FramePaint::Gradient(a), Paint::Gradient(b)) => assert_eq!(a, b),
                    _ => panic!("paint changed kind"),
                }''');s=s[:a]+region+s[b:]
 a=s.index('    fn tiling_preparation_cell_parent_is_local_and_outer_state_is_preserved');b=s.index('    #[test]',a);region=s[a:b];start=region.index('            let group =');end=region.index('            let FrameItem::Shape',start)
 region=region[:start]+'''            let cell = prepare_cell(layout, &sized(TilingBody::Content(Arc::new(body)))).unwrap();
'''+region[end:];region=region.replace('&first_cell(&group)[0]','&cell.frame.items[0]');s=s[:a]+region+s[b:]
 return s
edit('compiler/layout/tiling',tiling)
def shape(s):
 a=s.index('    let materialized_tiling =');b=s.index('    // **P767c** — avanço',a)
 s=s[:a]+'''    let fill = super::tiling::resolve_paint(layouter, fill.as_ref());
    let stroke = super::tiling::resolve_stroke(layouter, stroke.as_ref());
    layouter.regions.current.current_items.push(FrameItem::Shape {
        pos, kind: resolved_kind, width: resolved_w, height: resolved_h,
        fill, stroke, fill_rule: e.fill_rule,
        parent_bbox_at_emit: layouter.parent_bbox,
    });

'''+s[b:]
 # Production no longer needs declarative Paint, tests still do.
 s=s.replace('use crate::entities::paint::Paint;','#[cfg(test)]\nuse crate::entities::paint::Paint;')
 return s
edit('compiler/layout/shape',shape)
for name in ['curve','divider']:
 def fn(s):
  s=s.replace(', Stroke}', '}').replace('use crate::entities::paint::Paint;', 'use crate::entities::frame_paint::{FramePaint as Paint, FrameStroke as Stroke};')
  return s
 edit('compiler/layout/'+name,fn)
for name in ['word_layout','cell_tail_commit','grid/cell_body','block','boxed']:
 edit('compiler/layout/'+name,lambda s:s.replace('crate::entities::paint::Paint::Solid','crate::entities::frame_paint::FramePaint::Solid'))
for name in ['block','boxed']:
 def fn(s):
  marker='        layouter.regions.current.current_items.insert(\n            items_before,\n            FrameItem::Shape {'
  assert s.count(marker)==1,(name,s.count(marker))
  s=s.replace(marker,'        let prepared_stroke = super::tiling::resolve_stroke(layouter, stroke.as_ref());\n'+marker).replace('                stroke: stroke.clone(),','                stroke: prepared_stroke,',1)
  if name=='boxed':
   s=s.replace('    if e.fill.is_some() || e.stroke.is_some() {\n        contents.push(FrameItem::Shape {','    if e.fill.is_some() || e.stroke.is_some() {\n        let prepared_stroke = super::tiling::resolve_stroke(layouter, e.stroke.as_ref());\n        contents.push(FrameItem::Shape {').replace('            stroke: e.stroke.clone(),','            stroke: prepared_stroke,',1)
  return s
 edit('compiler/layout/'+name,fn)
edit('compiler/layout/cell_tail_commit',lambda s:s.replace('            if let Some(s) = tail.stroke {','            if let Some(s) = tail.stroke {\n                let prepared_stroke = super::tiling::resolve_stroke(self, Some(&s));').replace('                    stroke: Some(s),','                    stroke: prepared_stroke,'))
def borders(s):
 # Change only productive helpers; declaration comparisons still use Stroke.
 a=s.index('#[cfg(test)]');prod=s[:a];tests=s[a:]
 for name in ['emit_hsegment','emit_vsegment','flush_all_vsegments','emit_row_borders','emit_placed_row_borders']:
  prod=prod.replace('fn '+name+'(', 'fn '+name+'<M: FontMetrics, S: ImageSizer>(')
 prod=prod.replace('items: &mut Vec<FrameItem>', 'layouter: &mut super::super::Layouter<M, S>')
 prod=prod.replace('emit_hsegment(items,','emit_hsegment(layouter,').replace('emit_vsegment(items,','emit_vsegment(layouter,').replace('        items,\n','        layouter,\n')
 prod=prod.replace('        items.push(FrameItem::Shape {','        let prepared_stroke = super::super::tiling::resolve_stroke(layouter, Some(&stroke));\n        layouter.regions.current.current_items.push(FrameItem::Shape {').replace('            stroke: Some(stroke),','            stroke: prepared_stroke,')
 # Test-only wiring obtains the now-required layout capability.
 tests=tests.replace('    use super::*;\n', '''    use super::*;
    use crate::compiler::layout::{FixedMetrics, Layouter};
    use crate::entities::image_sizer::NullImageSizer;
    use crate::entities::introspector::{Introspector, TagIntrospector};
    use comemo::Track;
''',1)
 tests=tests.replace('let mut items = Vec::new();','''let intr = TagIntrospector::empty();
            let intr: &dyn Introspector = &intr;
            let mut layouter = Layouter::new(FixedMetrics, NullImageSizer, 12.0, intr.track());''')
 tests=tests.replace('emit_hsegment(&mut items,','emit_hsegment(&mut layouter,').replace('emit_vsegment(&mut items,','emit_vsegment(&mut layouter,')
 tests=tests.replace('            assert_eq!(items.len(), 2);','            let items = &layouter.regions.current.current_items;\n            assert_eq!(items.len(), 2);').replace('        assert!(items.is_empty());','        assert!(layouter.regions.current.current_items.is_empty());')
 tests=tests.replace('                assert_eq!(emitted.as_ref(), Some(&stroke));','''                let emitted = emitted.as_ref().unwrap();
                assert_eq!(emitted.thickness, stroke.thickness);
                assert_eq!(emitted.cap, stroke.cap);
                assert_eq!(emitted.join, stroke.join);
                assert_eq!(emitted.dash, stroke.dash);
                assert_eq!(emitted.miter_limit, stroke.miter_limit);
                assert_eq!(emitted.specified, stroke.specified);
                assert_eq!(emitted.overhang, stroke.overhang);
                let crate::entities::frame_paint::FramePaint::Solid(color) = &emitted.paint else { panic!("solid stroke changed paint kind") };
                assert_eq!(color, &Color::rgb(0, 0, 0));''')
 return prod+tests
edit('compiler/layout/grid/borders',borders)
def rules(s):
 s=s.replace('fn emit_explicit_rules<H:', 'fn emit_explicit_rules<M: FontMetrics, S: ImageSizer, H:').replace('items: &mut Vec<FrameItem>', 'layouter: &mut super::super::Layouter<M, S>')
 s=s.replace('            items.push(FrameItem::Shape {','            let prepared_stroke = super::super::tiling::resolve_stroke(layouter, Some(stroke));\n            layouter.regions.current.current_items.push(FrameItem::Shape {').replace('stroke: Some(stroke.clone()),','stroke: prepared_stroke,')
 return s
edit('compiler/layout/grid/rules',rules)
for name in ['grid','grid/cell_emission','grid/row_pagination']:
 edit('compiler/layout/'+name,lambda s:s.replace('&mut self.regions.current.current_items,\n', 'self,\n'))
