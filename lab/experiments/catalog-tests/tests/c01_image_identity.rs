use std::io::{Cursor, Read};
use std::sync::Arc;
use typst_core::entities::content::Content;
use typst_core::entities::layout_types::{FrameItem, Page, PagedDocument, Point, Pt};
use typst_infra::export::{export_pdf, StreamMode};

fn png(rgb: [u8; 3]) -> Vec<u8> {
    let image = image::RgbImage::from_pixel(2, 2, image::Rgb(rgb));
    let mut output = Cursor::new(Vec::new());
    image.write_to(&mut output, image::ImageFormat::Png).unwrap();
    output.into_inner()
}

fn document(images: &[Arc<Vec<u8>>]) -> PagedDocument {
    PagedDocument::new(vec![Page {
        width: 100.0,
        height: 100.0,
        numbering: None,
        logical_number: 1,
        supplement: Content::Empty,
        bleed: Default::default(),
        fill: Default::default(),
        background: vec![],
        foreground: vec![],
        items: images
            .iter()
            .enumerate()
            .map(|(i, data)| FrameItem::Image {
                format: None,
                alt: None,
                pos: Point { x: Pt(i as f64 * 10.0), y: Pt(0.0) },
                data: data.clone(),
                width: Pt(8.0),
                height: Pt(8.0),
                intrinsic_width: 2,
                intrinsic_height: 2,
                clip_rect: None,
                orientation: 1,
            })
            .collect(),
    }])
}

fn export(images: &[Arc<Vec<u8>>]) -> Vec<u8> {
    let doc = document(images);
    let result = export_pdf(&doc, StreamMode::Verbose);
    assert!(result.warnings.is_empty(), "unexpected omission: {:?}", result.warnings);
    assert!(result.output.starts_with(b"%PDF-"));
    result.output
}

// Read actual PDF image streams, rather than compare entire PDF bytes or call
// the decoder helper in place of the exporter. These opaque RGB fixtures need
// no SMask. PDF /Length bounds the binary stream, including embedded newlines.
fn pdf_pixels(pdf: &[u8]) -> Vec<Vec<u8>> {
    let marker = b"/Subtype /Image";
    let mut cursor = 0;
    let mut decoded = Vec::new();
    while let Some(offset) = pdf[cursor..].windows(marker.len()).position(|w| w == marker)
    {
        let start = cursor + offset;
        let relative = pdf[start..]
            .windows(7)
            .position(|w| w == b"stream\n")
            .expect("image stream");
        let stream = start + relative + 7;
        let header = std::str::from_utf8(&pdf[start..stream]).unwrap();
        assert!(header.contains("/Width 2 /Height 2"), "{header}");
        assert!(header.contains("/ColorSpace /DeviceRGB"), "{header}");
        assert!(header.contains("/Filter /FlateDecode"), "{header}");
        let length: usize = header
            .split("/Length ")
            .nth(1)
            .unwrap()
            .split_whitespace()
            .next()
            .unwrap()
            .parse()
            .unwrap();
        let end = stream.checked_add(length).unwrap();
        let mut pixels = Vec::new();
        flate2::read::ZlibDecoder::new(&pdf[stream..end])
            .read_to_end(&mut pixels)
            .unwrap();
        assert_eq!(pixels.len(), 12);
        decoded.push(pixels);
        cursor = end;
    }
    assert!(!decoded.is_empty(), "PDF omitted every image");
    decoded
}

#[test]
fn control_fresh_distinct_images_export_their_own_pixels() {
    std::thread::spawn(|| {
        let a = Arc::new(png([17, 89, 201]));
        let b = Arc::new(png([203, 41, 7]));
        let pixels = pdf_pixels(&export(&[a, b]));
        assert_eq!(pixels, vec![[17, 89, 201].repeat(4), [203, 41, 7].repeat(4)]);
    })
    .join()
    .unwrap();
}

#[test]
fn control_repeated_shared_image_and_repeated_export_keep_pixels() {
    std::thread::spawn(|| {
        let shared = Arc::new(png([37, 91, 155]));
        for _ in 0..2 {
            assert_eq!(
                pdf_pixels(&export(&[shared.clone(), shared.clone()])),
                vec![[37, 91, 155].repeat(4)]
            );
        }
    })
    .join()
    .unwrap();
}

#[test]
fn red_two_documents_same_arc_allocation_must_export_current_pixels() {
    std::thread::spawn(|| {
        let mut source = Arc::new(png([17, 89, 201]));
        let address = Arc::as_ptr(&source);
        let first_pdf = export(&[source.clone()]);
        assert_eq!(pdf_pixels(&first_pdf), vec![[17, 89, 201].repeat(4)]);
        // The first document is gone. This is safe unique ownership, with the
        // exact Arc allocation retained; allocator address recycling is absent.
        *Arc::get_mut(&mut source).expect("no source owner may escape export") =
            png([203, 41, 7]);
        assert_eq!(Arc::as_ptr(&source), address);
        assert_eq!(
            pdf_pixels(&export(&[source])),
            vec![[203, 41, 7].repeat(4)],
            "a second document must not inherit the first document's cached payload"
        );
        assert_eq!(pdf_pixels(&first_pdf), vec![[17, 89, 201].repeat(4)]);
    })
    .join()
    .unwrap();
}
