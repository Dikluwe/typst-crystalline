#![no_main]

use libfuzzer_sys::fuzz_target;
use fuzzing::FuzzWorld;
use html::HtmlDocument;

fuzz_target!(|text: &str| {
    let world = FuzzWorld::new(text);
    if let Ok(document) = fusion::compile::<HtmlDocument>(&world).output {
        _ = std::hint::black_box(html::html(&document));
    }
    comemo::evict(10);
});
