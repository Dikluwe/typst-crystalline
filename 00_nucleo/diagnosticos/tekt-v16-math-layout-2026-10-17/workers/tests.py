from pathlib import Path
import json,gzip,re,datetime
out=Path(__file__).resolve().parents[1];b=json.load(gzip.open(out/'before.json.gz'));scope=json.load(open(out/'scope.json'));receipt=json.load(open(out/'l0-before-code.json'));assert receipt['code_unchanged'];source='01_core/src/compiler/math/layout/source_transparency_tests.rs';parent='01_core/src/compiler/math/layout/mod.rs';accent='01_core/src/compiler/math/layout/accent.rs';assert not Path(source).exists();assert Path(parent).read_text()==b['files'][parent] and Path(accent).read_text()==b['files'][accent]
s=(out/'workers/source_family_v2.rs').read_text();metrics=s[s.index('#[derive(Clone,Copy)]'):s.index('fn src(')];helpers=s[s.index('fn src('):s.index('fn visit(')];main=s[s.index('fn main()'):];a=main.index('let c=match name{')+len('let c=');z=main.index(';let(items,e)');case=main[a:z];assert case.endswith('}')
header='''//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/math/layout/source-transparency-tests.md
//! @prompt-hash 00000000
//! @layer L1
//! @updated 2026-10-17
#![cfg(test)]

use super::MathLayouter;
use std::{num::NonZeroU16, sync::Arc};
use crate::{compiler::layout::{FixedMetrics, FontMetrics}, entities::{
    content::{Content, SourceContent}, elements::math_attach::MathAttachSlot,
    glyph_variants::{GlyphVariants, GlyphVariant, MathGlyphKern, MathKernTable, MathKernRecord},
    file_id::FileId, span::Span, layout_types::{TextStyle, Pt, Length, FrameItem, Point, TextSeparatorOrigin},
    rel::Rel, math_class::MathClass,
}};
'''
observe='''
#[derive(Debug)]
struct Visual { text: String, x: f64, y: f64, size: f64 }
#[derive(Debug)]
struct Observation {
    width: f64, ascent: f64, descent: f64,
    line_count: usize, first: f64, last: f64,
    visual: Vec<Visual>, replacements: Vec<String>,
}
fn visit(items: &[FrameItem], out: &mut Observation) {
    for item in items {
        match item {
            FrameItem::TextReplacement { text, items } => {
                out.replacements.push(text.to_string()); visit(items, out);
            }
            FrameItem::Semantic { items, .. } => visit(items, out),
            FrameItem::Text { pos, text, style, .. }
            | FrameItem::TextShaped { pos, text, style, .. } => out.visual.push(Visual {
                text: text.to_string(), x: pos.x.val(), y: pos.y.val(), size: style.size.val(),
            }),
            FrameItem::Glyph { pos, base_char, size, .. } => out.visual.push(Visual {
                text: base_char.to_string(), x: pos.x.val(), y: pos.y.val(), size: size.val(),
            }),
            FrameItem::TextSeparator { .. } | FrameItem::Line { .. } | FrameItem::Image { .. }
            | FrameItem::Shape { .. } | FrameItem::Group { .. } | FrameItem::Link { .. } => {
                panic!("Unknown math observation: {item:?}")
            }
        }
    }
}
fn observe(content: &Content, block: bool) -> Observation {
    let style = TextStyle::regular(Pt(12.0));
    let layouter = MathLayouter::new(&Metrics, block, &style);
    let (items, e) = layouter.layout_equation_measured(content, &style);
    let mut result = Observation { width: e.width, ascent: e.ascent, descent: e.descent,
        line_count: e.line_count, first: e.first_baseline, last: e.last_baseline,
        visual: vec![], replacements: vec![] };
    visit(&items, &mut result); result
}
fn close(left: f64, right: f64) {
    assert!((left - right).abs() < 1e-9, "geometria diferente: {left} versus {right}");
}
fn equivalent(plain: &Observation, source: &Observation) {
    close(plain.width, source.width); close(plain.ascent, source.ascent); close(plain.descent, source.descent);
    assert_eq!(plain.line_count, source.line_count); close(plain.first, source.first); close(plain.last, source.last);
    assert_eq!(plain.visual.len(), source.visual.len());
    for (a, z) in plain.visual.iter().zip(&source.visual) {
        assert_eq!(a.text, z.text); close(a.x, z.x); close(a.y, z.y); close(a.size, z.size);
    }
    assert_eq!(plain.replacements, source.replacements);
}
fn visual<'a>(observation: &'a Observation, text: &str) -> &'a Visual {
    let mut candidates = observation.visual.iter().filter(|v| v.text == text);
    let one = candidates.next().expect("observação atribuível ausente");
    assert!(candidates.next().is_none(), "observação ambígua"); one
}
fn assert_case(name: &str) {
    let plain = observe(&case(name, false), false);
    let source = observe(&case(name, true), false);
    if name == "limits-base" {
        let scripts = observe(&attach(Content::math_limits_override(text("∑"), false, true, true), text("a")), false);
        assert!(plain.ascent > scripts.ascent, "limits inline deve empilhar acima dos scripts");
    }
    if name == "weak-between" {
        let gapless = observe(&seq(vec![text("a"), text("x")]), false);
        close(plain.width - gapless.width, 2.4);
    }
    if name == "stretch-body" {
        let natural = observe(&text("→"), false);
        assert!(plain.width >= natural.width * 1.5);
    }
    if name == "matrix-cell" || name == "matrix-align-point" {
        let one = visual(&plain, "1"); let three = visual(&plain, "333");
        let style = TextStyle::regular(Pt(12.0));
        close(one.x + Metrics.advance("1", Pt(one.size), &style).val(),
            three.x + Metrics.advance("333", Pt(three.size), &style).val());
        close(visual(&plain, "2").x, visual(&plain, "4").x);
    }
    equivalent(&plain, &source);
}
macro_rules! source_case {
    ($test:ident, $case:literal) => {
        #[test] fn $test() { assert_case($case); }
    };
}
'''
names=json.loads(re.search(r'for name in (\[.*?\])\{for on',s).group(1));assert len(names)==23
code=header+'\n'+metrics+'\n'+helpers+'\nfn case(name: &str, on: bool) -> Content {\n    '+case+'\n}\n'+observe+'\n'+''.join('source_case!(v16_math_source_'+n.replace('-','_')+', '+json.dumps(n)+');\n' for n in names)+'''
#[test]
fn v16_math_source_display_lines_keep_full_extent_metadata() {
    let body = seq(vec![text("a"), Content::linebreak(), text("b")]);
    let plain = observe(&body, true); let source = observe(&src(body, true), true);
    assert_eq!(plain.line_count, 2); assert!(plain.last > plain.first);
    equivalent(&plain, &source);
}
#[test]
#[should_panic(expected = "Unknown math observation")]
fn v16_math_source_observer_rejects_unknown_separator() {
    let style = TextStyle::regular(Pt(12.0));
    let mut observation = observe(&text("a"), false);
    visit(&[FrameItem::TextSeparator { pos: Point { x: Pt(0.0), y: Pt(0.0) },
        advance: Pt(0.0), style, origin: TextSeparatorOrigin::Markup, bidi: None }], &mut observation);
}
#[test]
fn v16_math_source_flatten_retains_first_causal_origins() {
    let id = FileId::from_raw(NonZeroU16::new(1).unwrap());
    let first = Span::from_range(id, 10..11); let parent = Span::from_range(id, 20..21);
    let child = text("x").with_source_span(first);
    let container = seq(vec![child, text("y")]).with_source_span(parent);
    let mut flat = vec![]; super::flatten_math_sequence_nodes(&[container.clone()], &mut flat);
    assert_eq!(flat.len(), 2); assert_eq!(flat[0].source_span(), first);
    assert_eq!(flat[1].source_span(), parent); assert_eq!(container.source_span(), parent);
}
'''
Path(source).write_text(code);declaration='\nmod source_transparency_tests;\n';Path(parent).write_text(Path(parent).read_text()+declaration)
accent_tests='''

#[cfg(test)]
mod v16_source_origin_tests {
    use super::*;
    use crate::entities::{file_id::FileId, span::Span};
    #[test]
    fn v16_math_source_dotless_preserves_origin_and_changes_only_render_view() {
        let span = Span::from_range(FileId::from_raw(std::num::NonZeroU16::new(1).unwrap()), 2..5);
        let original = Content::MathText("i".into()).with_source_span(span);
        let rendered = dotless_base(&original);
        assert_eq!(rendered.plain_text(), "ı"); assert_eq!(rendered.source_span(), span);
        assert_eq!(original.plain_text(), "i"); assert_eq!(original.source_span(), span);
    }
}
''';Path(accent).write_text(Path(accent).read_text()+accent_tests)
record={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'new_source':source,'new_source_text':code,'parent':parent,'declaration':declaration,'accent':accent,'accent_tests':accent_tests,'expected_files':{p:Path(p).read_text() for p in [source,parent,accent]},'new_tests':27};(out/'tests-before-code.json').write_text(json.dumps(record,ensure_ascii=False,indent=2)+'\n');print('27 regressions/controls and private module composition added before production repair')
