// C includes this file ONLY as a cfg(test) child of the RAM copy of
// typst_infra::export::images::scan. No productive source is edited in Git.
use super::*;

#[test]
fn red_completed_document_must_not_retain_payload_in_thread_cache() {
    use typst_core::entities::content::Content;
    use typst_core::entities::layout_types::{Point, Pt};
    std::thread::spawn(|| {
        PAYLOAD_CACHE.with(|cache| assert!(cache.borrow().is_empty()));
        let image = image::RgbImage::from_pixel(2, 2, image::Rgb([11, 53, 197]));
        let mut encoded = std::io::Cursor::new(Vec::new());
        image.write_to(&mut encoded, image::ImageFormat::Png).unwrap();
        let data = Arc::new(encoded.into_inner());
        let weak = Arc::downgrade(&data);
        let doc = PagedDocument::new(vec![Page {
            width: 100.0, height: 100.0, numbering: None, logical_number: 1,
            supplement: Content::Empty, bleed: Default::default(), fill: Default::default(),
            background: vec![], foreground: vec![],
            items: vec![FrameItem::Image {
                format: None, alt: None, pos: Point::ZERO, data,
                width: Pt(8.0), height: Pt(8.0), intrinsic_width: 2, intrinsic_height: 2,
                clip_rect: None, orientation: 1,
            }],
        }]);
        let exported = crate::export::export_pdf(&doc, crate::export::StreamMode::Verbose);
        assert!(exported.warnings.is_empty());
        assert!(exported.output.starts_with(b"%PDF-"));
        drop(exported);
        drop(doc);
        assert!(weak.upgrade().is_none(), "the source Arc itself must be released");
        let (entries, compressed_bytes) = PAYLOAD_CACHE.with(|cache| {
            let cache = cache.borrow();
            (cache.len(), cache.values().map(|p| p.rgb_data_compressed.len()
                + p.alpha_data_compressed.as_ref().map_or(0, Vec::len)).sum::<usize>())
        });
        println!("completed_document_cache entries={entries} compressed_bytes={compressed_bytes}");
        assert_eq!((entries, compressed_bytes), (0, 0),
            "diagnostic target: completed documents must not leave old payloads in TLS");
    }).join().unwrap();
}
