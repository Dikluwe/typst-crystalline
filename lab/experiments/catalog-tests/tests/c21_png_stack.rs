// Public API geometry controls for the composed PNG helper. These tests do
// not establish a CLI caller or measure a performance improvement.
use std::path::PathBuf;
use typst_core::entities::content::Content;
use typst_core::entities::layout_types::{Color, Page, PagedDocument};
use typst_core::entities::page_canvas::{PageBleed, PageFill};
use typst_core::entities::paint::Paint;
use typst_infra::export::{render_document_to_png, RenderOptions};

const OPTIONS: RenderOptions = RenderOptions { pixel_per_pt: 1.0, render_bleed: false };

fn page(width: f64, height: f64, rgb: [u8; 3], number: usize) -> Page {
    Page {
        width,
        height,
        numbering: None,
        logical_number: number,
        supplement: Content::Empty,
        // Deliberately nonzero: render_bleed=false must keep body dimensions.
        bleed: PageBleed { left: 4.0, right: 6.0, top: 8.0, bottom: 10.0 },
        fill: PageFill::Paint(Paint::Solid(Color::rgb(rgb[0], rgb[1], rgb[2]))),
        background: vec![],
        foreground: vec![],
        items: vec![],
    }
}

fn dimensions(png: &[u8]) -> (u32, u32) {
    assert!(png.len() >= 33, "PNG must contain complete signature and IHDR");
    assert_eq!(&png[..8], b"\x89PNG\r\n\x1a\n");
    assert_eq!(u32::from_be_bytes(png[8..12].try_into().unwrap()), 13);
    assert_eq!(&png[12..16], b"IHDR");
    let width = u32::from_be_bytes(png[16..20].try_into().unwrap());
    let height = u32::from_be_bytes(png[20..24].try_into().unwrap());
    assert!(width > 0 && height > 0);
    (width, height)
}

fn save(name: &str, bytes: &[u8], expected: (u32, u32)) -> PathBuf {
    assert_eq!(dimensions(bytes), expected);
    let temporary = PathBuf::from(
        std::env::var_os("TMPDIR")
            .expect("C must set TMPDIR to the task's RAM directory"),
    );
    assert!(
        temporary.is_absolute() && temporary.starts_with("/dev/shm"),
        "PNG fixtures must be written only to task RAM"
    );
    let path = temporary.join(format!("c21-{name}-{}.png", std::process::id()));
    std::fs::write(&path, bytes).unwrap();
    println!("png_fixture={} width={} height={}", path.display(), expected.0, expected.1);
    path
}

#[test]
fn empty_document_returns_one_by_one_png_fallback() {
    let doc = PagedDocument::new(vec![]);
    let bytes = render_document_to_png(&doc, &OPTIONS, 2.0);
    save("empty", &bytes, (1, 1));
}

#[test]
fn single_page_keeps_body_dimensions_and_ignores_gap_and_bleed() {
    let doc = PagedDocument::new(vec![page(17.0, 9.0, [255, 0, 0], 1)]);
    let bytes = render_document_to_png(&doc, &OPTIONS, 20.0);
    let path = save("single-red", &bytes, (17, 9));
    println!(
        "external_pixel_check={} coordinate=8,4 expected_rgba=255,0,0,255",
        path.display()
    );
}

fn mixed_pages() -> PagedDocument {
    PagedDocument::new(vec![
        page(7.0, 3.0, [255, 0, 0], 1),
        page(11.0, 5.0, [0, 255, 0], 2),
        page(5.0, 2.0, [0, 0, 255], 3),
    ])
}

#[test]
fn mixed_page_sizes_stack_at_maximum_width_without_gap() {
    let bytes = render_document_to_png(&mixed_pages(), &OPTIONS, 0.0);
    let path = save("mixed-zero-gap", &bytes, (11, 10));
    for (x, y, rgba) in [
        (2, 1, "255,0,0,255"),
        (2, 5, "0,255,0,255"),
        (2, 8, "0,0,255,255"),
        (9, 1, "255,255,255,255"),
    ] {
        println!(
            "external_pixel_check={} coordinate={x},{y} expected_rgba={rgba}",
            path.display()
        );
    }
}

#[test]
fn controlled_gap_is_between_pages_and_does_not_add_outer_margin() {
    let bytes = render_document_to_png(&mixed_pages(), &OPTIONS, 2.0);
    let path = save("mixed-two-pixel-gap", &bytes, (11, 14));
    for (x, y, rgba) in [
        (2, 1, "255,0,0,255"),
        (2, 6, "0,255,0,255"),
        (2, 12, "0,0,255,255"),
        (2, 3, "255,255,255,255"),
        (2, 10, "255,255,255,255"),
        (9, 12, "255,255,255,255"),
    ] {
        println!(
            "external_pixel_check={} coordinate={x},{y} expected_rgba={rgba}",
            path.display()
        );
    }
}
