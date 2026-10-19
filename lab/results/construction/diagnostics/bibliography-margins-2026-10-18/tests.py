from pathlib import Path
import gzip,json,datetime,shutil
p=Path('00_nucleo/diagnosticos/bibliography-margins-2026-10-18')
Path('01_core/src/compiler/layout/block.rs').open('a').write('''
#[cfg(test)]
mod a2_margin_priority_tests {
    use super::*;
    use crate::entities::{
        content::Content, corners::Corners, image_sizer::NullImageSizer,
        introspector::{Introspector, TagIntrospector}, sides::Sides,
    };
    use comemo::Track;
    use std::sync::Arc;

    fn block(text: &str) -> BlockElem {
        BlockElem {
            body: Content::text(text), width: None, height: None,
            inset: Sides::uniform(Length::ZERO), breakable: true,
            outset: Sides::uniform(Length::ZERO),
            radius: Corners::uniform(Length::ZERO), clip: false, fill: None,
            stroke: None, spacing: None, above: None, below: None, sticky: false,
        }
    }

    fn positions(below: Option<f64>, above: Option<f64>, ambient: f64, spacing: Option<f64>) -> (f64, f64) {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(super::super::FixedMetrics, NullImageSizer, 12.0,
            (&intr as &dyn Introspector).track());
        l.chain = l.chain.push_custom("par.spacing", Value::Length(Length::pt(ambient)));
        let mut first = block("BEFORE");
        first.below = below.map(Length::pt);
        first.spacing = spacing.map(Length::pt);
        let mut second = block("BODY");
        second.above = above.map(Length::pt);
        l.layout_content(&Content::Block(Arc::new(first)));
        l.layout_content(&Content::Block(Arc::new(second)));
        l.layout_content(&Content::text("OUTSIDE"));
        let doc = l.finish();
        let y = |word| doc.pages.iter().flat_map(|page|
            crate::compiler::layout::tests::frame_items_recursive(&page.items))
            .find_map(|item| match item {
                FrameItem::Text { text, pos, .. } if text.as_str() == word => Some(pos.y.0),
                _ => None,
            }).expect("marker survives");
        (y("BODY"), y("OUTSIDE"))
    }

    #[test]
    fn a2_margin_explicit_below_beats_automatic_above() {
        for ambient in [0.0, 5.0, 24.0] {
            for below in [0.0, 3.0, 30.0] {
                assert_eq!(positions(Some(below), None, ambient, None),
                    positions(Some(below), Some(0.0), ambient, None),
                    "ambient={ambient}, explicit below={below}");
            }
        }
    }

    #[test]
    fn a2_margin_explicit_above_beats_automatic_below() {
        for ambient in [0.0, 5.0, 24.0] {
            for above in [0.0, 3.0, 30.0] {
                assert_eq!(positions(None, Some(above), ambient, None),
                    positions(Some(0.0), Some(above), ambient, None),
                    "ambient={ambient}, explicit above={above}");
            }
        }
    }

    #[test]
    fn a2_margin_equal_priorities_keep_the_larger_gap() {
        for ambient in [0.0, 5.0, 24.0] {
            assert_eq!(positions(None, None, ambient, None),
                positions(Some(ambient), Some(ambient), ambient, None));
            for (below, above) in [(3.0, 30.0), (30.0, 3.0), (0.0, 0.0)] {
                assert_eq!(positions(Some(below), Some(above), ambient, None),
                    positions(Some(f64::max(below, above)), Some(0.0), ambient, None));
            }
        }
    }

    #[test]
    fn a2_margin_explicit_spacing_is_not_automatic() {
        for ambient in [5.0, 24.0] {
            for spacing in [0.0, 3.0, 30.0] {
                assert_eq!(positions(None, None, ambient, Some(spacing)),
                    positions(Some(spacing), Some(0.0), ambient, Some(spacing)));
                assert_eq!(positions(Some(0.0), None, ambient, Some(spacing)),
                    positions(Some(0.0), Some(0.0), ambient, Some(spacing)));
            }
        }
    }
}
''')
Path('01_core/src/compiler/layout/heading.rs').open('a').write('''
#[cfg(test)]
mod a2_margin_priority_tests {
    use super::*;
    use crate::entities::{
        image_sizer::NullImageSizer, introspector::{Introspector, TagIntrospector},
        layout_types::{FrameItem, Length}, sides::Sides,
    };
    use comemo::Track;
    use std::sync::Arc;

    fn title_y(ambient: f64, level: u8, below: Option<f64>, initial: bool) -> f64 {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(super::super::FixedMetrics, NullImageSizer, 12.0,
            (&intr as &dyn Introspector).track());
        l.chain = l.chain.push_custom("par.spacing", Value::Length(Length::pt(ambient)));
        if !initial {
            let mut block = crate::entities::elements::block::BlockElem {
                body: Content::text("BEFORE"), width: None, height: None,
                inset: Sides::uniform(Length::ZERO), breakable: true,
                outset: Sides::uniform(Length::ZERO),
                radius: crate::entities::corners::Corners::uniform(Length::ZERO),
                clip: false, fill: None, stroke: None, spacing: None,
                above: None, below: below.map(Length::pt), sticky: false,
            };
            block.below = below.map(Length::pt);
            l.layout_content(&Content::Block(Arc::new(block)));
        }
        l.layout_content(&Content::heading(level, Content::text("TITLE")));
        assert_eq!(l.style.size.0, 12.0);
        let doc = l.finish();
        doc.pages.iter().flat_map(|page|
            crate::compiler::layout::tests::frame_items_recursive(&page.items))
            .find_map(|item| match item {
                FrameItem::Text { text, pos, .. } if text.as_str() == "TITLE" => Some(pos.y.0),
                _ => None,
            }).expect("heading survives")
    }

    #[test]
    fn a2_margin_heading_above_overrides_automatic_previous_margin() {
        for level in [1, 2, 3] {
            for ambient in [0.0, 5.0, 24.0, 60.0] {
                assert_eq!(title_y(ambient, level, None, false),
                    title_y(ambient, level, Some(0.0), false),
                    "level={level}, ambient={ambient}");
                assert_eq!(title_y(ambient, level, None, true),
                    title_y(0.0, level, None, true));
            }
        }
    }

    #[test]
    fn a2_margin_heading_keeps_larger_explicit_previous_margin() {
        for level in [1, 2, 3] {
            let above = 12.0 * if level == 1 { 1.8 } else { 1.44 };
            let zero = title_y(24.0, level, Some(0.0), false);
            for below in [3.0, 30.0] {
                let shift = (below - above).max(0.0);
                assert!((title_y(24.0, level, Some(below), false) - zero - shift).abs() < 1e-8);
            }
        }
    }
}
''')
# Remove a redundant test setup assignment before the RED run.
f=Path('01_core/src/compiler/layout/heading.rs');s=f.read_text().replace('let mut block = crate::entities::elements::block::BlockElem {','let block = crate::entities::elements::block::BlockElem {').replace('            block.below = below.map(Length::pt);\n','');f.write_text(s)
(p/'tests-before-code.json.gz').write_bytes(gzip.compress(json.dumps({'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'files':{str(f):f.read_text() for f in [Path('01_core/src/compiler/layout/block.rs'),Path('01_core/src/compiler/layout/heading.rs')]}}).encode()))
shutil.copy2('/tmp/prescribe-bib-margins.py',p/'prescribe.py');shutil.copy2('/tmp/test-bib-margins.py',p/'tests.py')
