#![no_main]

use libfuzzer_sys::fuzz_target;
use fusion::layout::PagedDocument;
use typst_fuzz::FuzzWorld;
use pdf::PdfOptions;

fuzz_target!(|text: &str| {
    let world = FuzzWorld::new(text);
    if let Ok(document) = fusion::compile::<PagedDocument>(&world).output {
        if let Some(page) = document.pages.first() {
            std::hint::black_box(raster::render(page, 1.0));
            std::hint::black_box(typst_svg::svg(page));
        }
        _ = std::hint::black_box(pdf::pdf(&document, &PdfOptions::default()));
    }
    comemo::evict(10);
});
