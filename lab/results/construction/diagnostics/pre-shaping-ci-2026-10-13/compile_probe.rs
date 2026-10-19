#![deny(deprecated)]
use typst_core::entities::layout_types::{FrameItem, Point, Pt, TextStyle};
fn main() {
    let item = FrameItem::Text {
        pos: Point { x: Pt(12.0), y: Pt(24.0) },
        text: "pré-shaping".into(),
        style: TextStyle::default(),
        bidi: None,
    };
    let FrameItem::Text { pos, text, bidi, .. } = item else {
        panic!("o carrier de pré-shaping deve permanecer Text");
    };
    assert_eq!(pos.x.0, 12.0);
    assert_eq!(pos.y.0, 24.0);
    assert_eq!(text.as_str(), "pré-shaping");
    assert!(bidi.is_none());
}
