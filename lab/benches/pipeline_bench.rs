//! Production entrypoint benchmarks for the atomized L3 pipeline.
//!
//! Sources/worlds/fonts are prepared outside the measured loop. Each iteration
//! includes eval, stabilization, layout where applicable, resources and export.
//! These are warm-process benchmarks, not isolated timings of private helpers.

#[path = "support/profile.rs"]
mod profile;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use typst_core::contracts::world::World;
use typst_core::entities::compiler_features::{Feature, Features};
use typst_core::entities::source::Source;
use typst_infra::export::{HtmlSerializationMode, PdfSerializationMode, StreamMode};
use typst_infra::pipeline::*;
use typst_infra::world::SystemWorld;

fn fixture(text: &str, embedded: bool) -> (SystemWorld, Source) {
    let world = SystemWorld::for_eval(env!("CARGO_MANIFEST_DIR"));
    let world = if embedded { world.with_embedded_fonts() } else { world };
    let source = Source::new(world.main(), text.to_owned())
        .expect("valid pipeline benchmark source fixture");
    (world, source)
}

fn features(bundle: bool) -> Features {
    let mut features = Features::default();
    features.enable(Feature::Html);
    if bundle {
        features.enable(Feature::Bundle);
    }
    features
}

fn bundle_options() -> BundleCompileOptions {
    BundleCompileOptions {
        full_error: false,
        features: features(true),
        document_id: Some([0; 16]),
        pdf_stream_mode: StreamMode::Verbose,
        pdf_serialization: PdfSerializationMode::Compact,
        pdf_tagged_override: None,
        pdf_pages_override: None,
        pdf_standard_override: None,
        png_ppi_override: None,
        html_serialization: HtmlSerializationMode::Crystalline,
        pretty_override: None,
    }
}

fn pdf(c: &mut Criterion) {
    profile::require_optimized();
    let mut group = c.benchmark_group("pipeline/pdf");
    for (name, source, embedded, tagged) in [
        ("fallback", "#set page(width: 64pt, height: 64pt)\nHello", false, Some(false)),
        ("single_font", "#set page(width: 64pt, height: 64pt)\nHello", true, Some(false)),
        ("multifont", "#set page(width: 128pt, height: 64pt)\n#text(font: \"Libertinus Serif\")[One] #text(font: \"New Computer Modern\")[Two]", true, Some(false)),
        ("tagged", "#set page(width: 128pt, height: 64pt)\n= Heading\nHello", true, Some(true)),
        ("context_stable", "#set page(width: 64pt, height: 64pt)\n#context [#counter(page).get().first()]", false, Some(false)),
        ("line_callback", "#set page(width: 128pt, height: 64pt)\n#set par.line(numbering: n => str(n))\nHello", true, Some(false)),
    ] {
        let (world, source) = fixture(source, embedded);
        let run = || compile_to_pdf_bytes_with_timings_full_error_and_document_id_with_features_and_tagged_and_pretty_override(
            &world, &source, false, Some([0; 16]), StreamMode::Verbose,
            tagged, Features::default(), None,
        );
        assert!(!run().0.expect(name).is_empty());
        group.bench_function(name, |b| b.iter(|| {
            let (result, warnings, timings) = run();
            black_box((result.expect(name), warnings, timings));
        }));
    }
    group.finish();
}

fn html(c: &mut Criterion) {
    profile::require_optimized();
    let mut group = c.benchmark_group("pipeline/html");
    for (name, text) in
        [("semantic", "Hello *world*"), ("frame", "#html.frame(box(width: 64pt)[Hello])")]
    {
        let (world, source) = fixture(text, true);
        let run =
            || compile_to_html_string_with_features(&world, &source, features(false));
        assert!(run().0.expect(name).contains("<html"));
        group.bench_function(name, |b| {
            b.iter(|| {
                let (result, warnings) = run();
                black_box((result.expect(name), warnings));
            })
        });
    }
    group.finish();
}

fn images(c: &mut Criterion) {
    profile::require_optimized();
    let (world, source) =
        fixture("#set page(width: 32pt, height: 32pt, margin: 2pt)\nHi", true);
    let mut raster = c.benchmark_group("pipeline/raster");
    for (name, override_ppi) in [("document_ppi", None), ("override_144", Some(144.0))] {
        let run = || {
            compile_to_png_bytes_with_timings_full_error_and_features_and_ppi_override(
                &world,
                &source,
                false,
                Features::default(),
                override_ppi,
            )
        };
        assert!(run().0.expect(name).starts_with(b"\x89PNG"));
        raster.bench_function(name, |b| {
            b.iter(|| {
                let (result, warnings, timings) = run();
                black_box((result.expect(name), warnings, timings));
            })
        });
    }
    raster.finish();

    let mut svg = c.benchmark_group("pipeline/svg");
    for (name, text, embedded) in [
        ("without_fonts", "#set page(width: 32pt, height: 32pt)\nHi", false),
        ("with_fonts", "#set page(width: 32pt, height: 32pt)\nHi", true),
        ("glyphs", "#set page(width: 128pt, height: 64pt)\n$ underbrace(a+b+c) $", true),
        (
            "destination",
            "#set page(width: 128pt, height: 64pt)\n= Heading <h>\nHello",
            true,
        ),
    ] {
        let (world, source) = fixture(text, embedded);
        let run =
            || compile_to_svg_string_with_timings_full_error(&world, &source, false);
        assert!(run().0.expect(name).contains("<svg"));
        svg.bench_function(name, |b| {
            b.iter(|| {
                let (result, warnings, timings) = run();
                black_box((result.expect(name), warnings, timings));
            })
        });
    }
    svg.finish();
}

fn bundle(c: &mut Criterion) {
    profile::require_optimized();
    let mut group = c.benchmark_group("pipeline/bundle");
    for (name, text, count) in [
        ("one", "#document(\"one.html\")[Hello]", 1),
        ("several", "#document(\"one.html\")[One]\n#document(\"two.html\")[Two]\n#document(\"three.html\")[Three]", 3),
        ("mixed", "#document(\"one.html\")[Hello]\n#document(\"two.pdf\")[#set page(width: 64pt, height: 64pt)\nHi]", 2),
    ] {
        let (world, source) = fixture(text, true);
        let run = || compile_to_bundle_files(&world, &source, bundle_options());
        assert_eq!(run().0.expect(name).len(), count);
        group.bench_function(name, |b| b.iter(|| {
            let (result, warnings, timings) = run();
            black_box((result.expect(name), warnings, timings));
        }));
    }
    group.finish();
}

criterion_group!(benches, pdf, html, images, bundle);
criterion_main!(benches);
