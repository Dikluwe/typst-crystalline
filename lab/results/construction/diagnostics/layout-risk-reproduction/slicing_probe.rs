pub use typst_core::entities;
#[path = "../../../01_core/src/compiler/layout/slicing.rs"]
mod slicing;
use entities::layout_types::{FrameItem, Point, Pt, Rect};
fn image(y: f64) -> FrameItem {
    FrameItem::Image {
        pos: Point { x: Pt(10.0), y: Pt(y) },
        data: std::sync::Arc::new(vec![]), alt: None,
        width: Pt(80.0), height: Pt(40.0),
        intrinsic_width: 80, intrinsic_height: 40, orientation: 1,
        clip_rect: Some(Rect { x: Pt(30.0), y: Pt(y), w: Pt(40.0), h: Pt(40.0) }),
    }
}
fn main() {
    let (head, tail) = slicing::slice_frame_items_at_height(vec![image(5.0), image(20.0)], 10.0);
    for (name, items) in [("head", head), ("tail", tail)] {
        for item in items {
            if let FrameItem::Image { pos, clip_rect, .. } = item {
                println!("{} y={} clip={:?}", name, pos.y.0, clip_rect);
            }
        }
    }
}
