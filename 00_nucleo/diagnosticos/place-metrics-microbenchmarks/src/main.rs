//! Diagnostic public-API benchmarks; no productive API exposure or copying.
use criterion::{black_box, BatchSize, BenchmarkId, Criterion};
use sha2::{Digest, Sha256};
use std::{path::PathBuf, sync::Arc, time::Duration};
use typst_core::{
    compiler::layout::{layout, FontMetrics},
    contracts::world::World,
    entities::{
        content::Content,
        elements::{place::PlaceElem, shape::ShapeElem},
        font_list::{FontFamily, FontList},
        geometry::{FillRule, ShapeKind},
        layout_types::{Align2D, FrameItem, HAlign, PlaceScope, Pt, TextStyle, VAlign},
        value::Value,
    },
};
use typst_infra::{font_metrics::FallbackFontMetrics, world::SystemWorld};

#[derive(Clone, Copy)]
enum Op {
    Vertical,
    Edges,
    Advance,
    Shaped,
}
fn query(op: Op, m: &FallbackFontMetrics<'_>, style: &TextStyle) -> f64 {
    match op {
        Op::Vertical => {
            let (a, b) = m.vertical_metrics(black_box(Pt(11.0)), black_box(style));
            a.0 + b.0
        }
        Op::Edges => {
            let (a, b) = m.text_edges(black_box(Pt(11.0)), black_box(style));
            a.0 + b.0
        }
        Op::Advance => {
            m.advance(black_box("Hello world"), black_box(Pt(11.0)), black_box(style))
                .0
        }
        Op::Shaped => {
            m.advance_shaped(
                black_box("Hello world"),
                black_box(Pt(11.0)),
                black_box(style),
            )
            .expect("real font shaping")
            .0
        }
    }
}
fn shape() -> Content {
    Content::Shape(Arc::new(ShapeElem {
        kind: ShapeKind::Rect,
        width: Some(Box::new(Value::Float(80.0))),
        height: Some(Box::new(Value::Float(40.0))),
        fill: None,
        stroke: None,
        fill_rule: FillRule::NonZero,
    }))
}
fn nested_place(depth: usize) -> Content {
    (0..depth).fold(shape(), |body, _| {
        Content::Place(Arc::new(PlaceElem {
            alignment: Align2D { h: Some(HAlign::Left), v: Some(VAlign::Top) },
            dx: 3.0,
            dy: 5.0,
            scope: PlaceScope::Column,
            float: false,
            clearance: None,
            body,
        }))
    })
}
fn shapes(items: &[FrameItem]) -> usize {
    items
        .iter()
        .map(|item| match item {
            FrameItem::Shape { .. } => 1,
            FrameItem::Group { items, .. }
            | FrameItem::Semantic { items, .. }
            | FrameItem::Link { items, .. } => shapes(items),
            _ => 0,
        })
        .sum()
}
fn main() {
    assert!(!cfg!(debug_assertions), "refuse debug measurements");
    let args: Vec<_> = std::env::args().collect();
    let root = PathBuf::from(&args[1]);
    let dest = PathBuf::from(&args[2]);
    let reverse = args.iter().any(|x| x == "--reverse");
    std::fs::create_dir_all(&dest).unwrap();
    let world = SystemWorld::new(&root, "main.typ").unwrap().with_embedded_fonts();
    let style = TextStyle {
        size: Pt(11.0),
        font: FontList::new(vec![FontFamily::new("Libertinus Serif".into())]),
        ..Default::default()
    };
    let retained = FallbackFontMetrics::new(&world);
    let mut ops = vec![
        ("vertical", Op::Vertical),
        ("text_edges", Op::Edges),
        ("advance", Op::Advance),
        ("advance_shaped", Op::Shaped),
    ];
    // All retained query caches and the selected world's font handles are warm.
    let values: Vec<_> = ops
        .iter()
        .map(|(name, op)| (*name, query(*op, &retained, &style)))
        .collect();
    for (_, op) in &ops {
        let a = query(*op, &retained, &style);
        let b = query(*op, &FallbackFontMetrics::new(&world), &style);
        let c = query(*op, &retained.clone(), &style);
        assert!((a - b).abs() < 1e-9 && (a - c).abs() < 1e-9);
    }
    let fonts:Vec<_>=world.book().select_family("libertinus serif").map(|index| {let f=world.font(index).unwrap();serde_json::json!({"index":index,"sha256":format!("{:x}",Sha256::digest(f.as_slice()))})}).collect();
    assert!(!fonts.is_empty());
    std::fs::write(dest.join("setup.json"),serde_json::to_vec_pretty(&serde_json::json!({"selected_family_faces":fonts,"query_values":values,"font_size_pt":11.0,"text":"Hello world","world_policy":"embedded only; acquired outside timing","setup_and_consistency_checks_untimed":true})).unwrap()).unwrap();
    if args.iter().any(|x| x == "--smoke") {
        for depth in [0, 1, 4, 16] {
            let doc = layout(&nested_place(depth));
            assert_eq!(doc.pages.iter().map(|p| shapes(&p.items)).sum::<usize>(), 1);
        }
        println!("untimed smoke passed");
        return;
    }
    let mut c = Criterion::default()
        .sample_size(50)
        .nresamples(10000)
        .warm_up_time(Duration::from_millis(500))
        .measurement_time(Duration::from_secs(1))
        .without_plots()
        .output_directory(&dest);
    if reverse {
        ops.reverse();
    }
    let mut g = c.benchmark_group("font_metrics_embedded_warm_world");
    for (name, op) in ops {
        for fresh in if reverse { [true, false] } else { [false, true] } {
            if fresh {
                g.bench_function(format!("{name}/fresh_metric_instance"), |b| {
                    b.iter_batched_ref(
                        || FallbackFontMetrics::new(&world),
                        |m| black_box(query(op, m, &style)),
                        BatchSize::PerIteration,
                    )
                });
            } else {
                g.bench_function(format!("{name}/retained_metric_instance"), |b| {
                    b.iter(|| black_box(query(op, &retained, &style)))
                });
            }
        }
    }
    g.bench_function("constructor_and_drop", |b| {
        b.iter(|| black_box(FallbackFontMetrics::new(black_box(&world))))
    });
    g.bench_function("shared_cache_clone_and_drop", |b| {
        b.iter(|| black_box(retained.clone()))
    });
    g.bench_function("control_batched_ref", |b| {
        b.iter_batched_ref(|| (), |unit| black_box(*unit), BatchSize::PerIteration)
    });
    g.finish();
    let mut g = c.benchmark_group("fixed_metrics_layout_composition");
    let mut depths = vec![0, 1, 4, 16];
    if reverse {
        depths.reverse();
    }
    for depth in depths {
        let content = nested_place(depth);
        let doc = layout(&content);
        assert_eq!(doc.pages.iter().map(|p| shapes(&p.items)).sum::<usize>(), 1);
        g.bench_with_input(
            BenchmarkId::new("nested_place", depth),
            &content,
            |b, input| b.iter(|| black_box(layout(black_box(input)))),
        );
    }
    g.finish();
    c.final_summary();
}
