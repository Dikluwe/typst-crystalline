//! Diagnostic microbenchmarks using existing public font APIs.
mod historical;
use criterion::{black_box, BatchSize, BenchmarkId, Criterion};
use sha2::{Digest, Sha256};
use std::{collections::BTreeSet, fs, path::PathBuf, time::Duration};
use typst_core::entities::font_book::Coverage;
use typst_infra::fonts::{font_info_from_bytes, FontSlot};

struct Fixture {
    id: String,
    path: PathBuf,
    bytes: Vec<u8>,
    index: u32,
    codepoints: Vec<u32>,
    coverage: Coverage,
}
fn sha(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
// Diagnostic extraction matching production's Unicode subtable iteration;
// not a call to private production extract_coverage.
fn collect(face: &ttf_parser::Face<'_>) -> Vec<u32> {
    let mut points = Vec::new();
    if let Some(cmap) = face.tables().cmap {
        for sub in cmap.subtables {
            if sub.is_unicode() {
                sub.codepoints(&mut |c| points.push(c));
            }
        }
    }
    points
}
fn fixtures() -> (Vec<Fixture>, serde_json::Value) {
    let inputs: serde_json::Value =
        serde_json::from_slice(include_bytes!("../inputs.json")).unwrap();
    let mut out = Vec::new();
    let mut controls = Vec::new();
    for input in inputs.as_array().unwrap() {
        let path = PathBuf::from(input["path"].as_str().unwrap());
        let bytes = fs::read(&path).unwrap();
        assert_eq!(sha(&bytes), input["sha256"].as_str().unwrap(), "font bytes changed");
        let index = input["face_index"].as_u64().unwrap() as u32;
        let face = ttf_parser::Face::parse(&bytes, index).unwrap();
        let points = collect(&face);
        assert!(!points.is_empty());
        let expected: BTreeSet<_> = points.iter().copied().collect();
        let coverage = Coverage::from_codepoints(points.clone());
        let old = historical::Coverage::from_codepoints(points.clone());
        assert_eq!(
            coverage.iter().collect::<Vec<_>>(),
            expected.iter().copied().collect::<Vec<_>>()
        );
        assert_eq!(old.iter().collect::<Vec<_>>(), coverage.iter().collect::<Vec<_>>());
        for point in (0..=0x10FFFF).step_by(73).chain(expected.iter().copied()) {
            assert_eq!(coverage.contains(point), expected.contains(&point));
            assert_eq!(old.contains(point), expected.contains(&point));
        }
        assert_eq!(format!("{coverage:?}"), format!("{old:?}"));
        let formats: Vec<_> = face
            .tables()
            .cmap
            .unwrap()
            .subtables
            .into_iter()
            .filter(|s| s.is_unicode())
            .map(|s| match s.format {
                ttf_parser::cmap::Format::ByteEncodingTable(_) => 0,
                ttf_parser::cmap::Format::HighByteMappingThroughTable(_) => 2,
                ttf_parser::cmap::Format::SegmentMappingToDeltaValues(_) => 4,
                ttf_parser::cmap::Format::TrimmedTableMapping(_) => 6,
                ttf_parser::cmap::Format::MixedCoverage => 8,
                ttf_parser::cmap::Format::TrimmedArray(_) => 10,
                ttf_parser::cmap::Format::SegmentedCoverage(_) => 12,
                ttf_parser::cmap::Format::ManyToOneRangeMappings(_) => 13,
                ttf_parser::cmap::Format::UnicodeVariationSequences(_) => 14,
            })
            .collect();
        let mut encoded = Vec::new();
        for c in &points {
            encoded.extend(c.to_le_bytes());
        }
        let metadata = font_info_from_bytes(&bytes, index).unwrap();
        assert!(metadata.coverage.is_empty());
        let slot = FontSlot::new(path.clone(), index);
        assert!(slot.get().is_some());
        controls.push(serde_json::json!({"id":input["id"],"path":path,"font_sha256":sha(&bytes),"face_index":index,"unicode_subtable_formats":formats,"collected_codepoints":points.len(),"distinct_codepoints":expected.len(),"codepoint_sequence_le_sha256":sha(&encoded),"canonical_coverage_sha256":sha(format!("{coverage:?}").as_bytes()),"family":metadata.family,"query_points":{"hit_early":expected.first().unwrap(),"hit_middle":expected.iter().nth(expected.len()/2).unwrap(),"hit_late":expected.last().unwrap(),"miss_after":0x10FFFFu32,"miss_gap":(0..=*expected.last().unwrap()).find(|point|!expected.contains(point)).unwrap()},"historical_equals_current_and_set_oracle":true}));
        out.push(Fixture {
            id: input["id"].as_str().unwrap().into(),
            path,
            bytes,
            index,
            codepoints: points,
            coverage,
        });
    }
    (out, serde_json::json!({"fixtures":controls,"preflight_outside_all_timers":true}))
}
fn main() {
    assert_eq!(env!("FONT_MICRO_OPT_LEVEL"), "3", "requires opt-level 3");
    assert!(!cfg!(debug_assertions), "debug builds must not report timings");
    let (fixtures, controls) = fixtures();
    if std::env::args().any(|a| a == "--check") {
        println!("{}", serde_json::to_string_pretty(&controls).unwrap());
        return;
    }
    let dest = PathBuf::from(
        std::env::var("FONT_MICRO_OUTPUT")
            .expect("explicit isolated output directory required"),
    );
    fs::create_dir_all(&dest).unwrap();
    fs::write(
        dest.join("fixture-controls.json"),
        serde_json::to_vec_pretty(&controls).unwrap(),
    )
    .unwrap();
    let mut c = Criterion::default()
        .sample_size(30)
        .nresamples(5000)
        .warm_up_time(Duration::from_millis(200))
        .measurement_time(Duration::from_millis(600))
        .without_plots()
        .output_directory(&dest);
    let reverse = std::env::var("FONT_MICRO_REVERSE").as_deref() == Ok("1");
    for f in if reverse {
        fixtures.iter().rev().collect::<Vec<_>>()
    } else {
        fixtures.iter().collect()
    } {
        let mut g = c.benchmark_group(&f.id);
        for old in if reverse { [true, false] } else { [false, true] } {
            if old {
                g.bench_function("constructor/historical_extracted", |b| {
                    b.iter_batched(
                        || f.codepoints.clone(),
                        |p| historical::Coverage::from_codepoints(black_box(p)),
                        BatchSize::PerIteration,
                    )
                });
            } else {
                g.bench_function("constructor/current_public", |b| {
                    b.iter_batched(
                        || f.codepoints.clone(),
                        |p| Coverage::from_codepoints(black_box(p)),
                        BatchSize::PerIteration,
                    )
                });
            }
        }
        g.bench_function("clone_and_build/current_public", |b| {
            b.iter_batched(
                || (),
                |()| Coverage::from_codepoints(black_box(&f.codepoints).clone()),
                BatchSize::PerIteration,
            )
        });
        let face = ttf_parser::Face::parse(&f.bytes, f.index).unwrap();
        g.bench_function("cmap/decode_collect_diagnostic", |b| {
            b.iter_batched(|| (), |()| collect(black_box(&face)), BatchSize::PerIteration)
        });
        g.bench_function("metadata/public_bytes", |b| {
            b.iter_batched(
                || (),
                |()| {
                    font_info_from_bytes(black_box(&f.bytes), black_box(f.index)).unwrap()
                },
                BatchSize::PerIteration,
            )
        });
        let members: Vec<_> = f.coverage.iter().collect();
        let max = *members.last().unwrap();
        let queries = [
            ("hit_early", members[0]),
            ("hit_middle", members[members.len() / 2]),
            ("hit_late", max),
            ("miss_after", 0x10FFFF),
            ("miss_gap", (0..=max).find(|p| !f.coverage.contains(*p)).unwrap()),
        ];
        for (name, point) in queries {
            assert_eq!(f.coverage.contains(point), name.starts_with("hit"));
            g.bench_with_input(BenchmarkId::new("contains", name), &point, |b, p| {
                b.iter(|| black_box(&f.coverage).contains(black_box(*p)))
            });
        }
        g.bench_function("slot/first_get_warm_filesystem", |b| {
            b.iter_batched_ref(
                || FontSlot::new(f.path.clone(), f.index),
                |s| black_box(s).get().unwrap(),
                BatchSize::PerIteration,
            )
        });
        let slot = FontSlot::new(f.path.clone(), f.index);
        assert!(slot.get().is_some());
        g.bench_function("slot/retained_get", |b| {
            b.iter_batched(
                || (),
                |()| black_box(&slot).get().unwrap(),
                BatchSize::PerIteration,
            )
        });
        g.bench_function("control/black_box_bool", |b| {
            b.iter(|| black_box(black_box(false)))
        });
        g.bench_function("control/batched_unit", |b| {
            b.iter_batched(|| (), |()| black_box(()), BatchSize::PerIteration)
        });
        g.finish();
    }
    c.final_summary();
}
