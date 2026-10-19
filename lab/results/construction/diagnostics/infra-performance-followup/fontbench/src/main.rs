//! Compare actual pinned vanilla APIs with current Tekt APIs; diagnostic only.
use criterion::{BatchSize, Criterion, black_box};
use sha2::{Digest, Sha256};
use std::{collections::BTreeSet, fs, path::PathBuf, time::Duration};
use typst_core::entities::font_book::{Coverage as TC, FontInfo as TI};
use typst_infra::fonts::{FontSlot, font_info_from_bytes};
use typst_kit::fonts::{FontPath, FontStore};
use typst_library::text::{Coverage as VC, FontInfo as VI};

fn sha(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn collect(face: &ttf_parser::Face<'_>) -> Vec<u32> {
    let mut p = Vec::new();
    if let Some(cmap) = face.tables().cmap {
        for s in cmap.subtables {
            if s.is_unicode() {
                s.codepoints(&mut |c| p.push(c));
            }
        }
    }
    p
}
// This glue measures a composed diagnostic using public metadata/coverage APIs.
// The private production extractor is neither exposed nor called here.
fn tekt_full(bytes: &[u8], index: u32) -> TI {
    let mut info = font_info_from_bytes(bytes, index).unwrap();
    let face = ttf_parser::Face::parse(bytes, index).unwrap();
    info.coverage = TC::from_codepoints(collect(&face));
    info
}
fn store(path: &PathBuf, index: u32, info: &VI) -> FontStore {
    let mut s = FontStore::new();
    s.push((FontPath { path: path.clone(), index }, info.clone()));
    s
}
struct Fixture {
    id: String,
    path: PathBuf,
    bytes: Vec<u8>,
    index: u32,
    points: Vec<u32>,
    tc: TC,
    vc: VC,
    vi: VI,
    queries: Vec<(&'static str, u32)>,
}
fn fixtures() -> (Vec<Fixture>, serde_json::Value) {
    let inputs: serde_json::Value =
        serde_json::from_slice(include_bytes!("../inputs.json")).unwrap();
    let mut fixtures = Vec::new();
    let mut controls = Vec::new();
    for input in inputs.as_array().unwrap() {
        let path = PathBuf::from(input["path"].as_str().unwrap());
        let bytes = fs::read(&path).unwrap();
        assert_eq!(sha(&bytes), input["sha256"].as_str().unwrap());
        let index = input["face_index"].as_u64().unwrap() as u32;
        let face = ttf_parser::Face::parse(&bytes, index).unwrap();
        let points = collect(&face);
        assert!(!points.is_empty());
        let expected: BTreeSet<_> = points.iter().copied().collect();
        let tc = TC::from_codepoints(points.clone());
        let vc = VC::from_vec(points.clone());
        let vi = VI::new(&bytes, index).unwrap();
        let ti = tekt_full(&bytes, index);
        let ordered: Vec<_> = expected.iter().copied().collect();
        assert_eq!(tc.iter().collect::<Vec<_>>(), ordered);
        assert_eq!(vc.iter().collect::<Vec<_>>(), ordered);
        assert_eq!(vi.coverage.iter().collect::<Vec<_>>(), ordered);
        assert_eq!(ti.coverage.iter().collect::<Vec<_>>(), ordered);
        for p in (0..=0x10FFFF).step_by(73).chain(ordered.iter().copied()) {
            assert_eq!(tc.contains(p), expected.contains(&p));
            assert_eq!(vc.contains(p), expected.contains(&p));
        }
        let queries = vec![
            ("hit_early", ordered[0]),
            ("hit_middle", ordered[ordered.len() / 2]),
            ("hit_late", *ordered.last().unwrap()),
            ("miss_after", 0x10FFFF),
            (
                "miss_gap",
                (0..=*ordered.last().unwrap())
                    .find(|p| !expected.contains(p))
                    .unwrap(),
            ),
        ];
        for (name, p) in &queries {
            assert_eq!(tc.contains(*p), name.starts_with("hit"));
            assert_eq!(vc.contains(*p), name.starts_with("hit"));
        }
        let tf = FontSlot::new(path.clone(), index).get().unwrap();
        let vs = store(&path, index, &vi);
        let vf = vs.font(0).unwrap();
        let ttf = ttf_parser::Face::parse(tf.as_slice(), 0).unwrap();
        let vttf = ttf_parser::Face::parse(vf.data(), vf.index()).unwrap();
        assert_eq!(ttf.number_of_glyphs(), vttf.number_of_glyphs());
        assert_eq!(ttf.units_per_em(), vttf.units_per_em());
        for ch in "AaéΩ∑中日한".chars() {
            assert_eq!(
                ttf.glyph_index(ch).map(|g| g.0),
                vttf.glyph_index(ch).map(|g| g.0)
            );
        }
        let mut encoded = Vec::new();
        for p in &points {
            encoded.extend(p.to_le_bytes());
        }
        controls.push(serde_json::json!({"id":input["id"],"path":path,"font_sha256":sha(&bytes),"face_index":index,"collected_codepoints":points.len(),"distinct_codepoints":ordered.len(),"sequence_le_sha256":sha(&encoded),"coverage_set_equal_across_generic_and_full_paths":true,"tekt_family":ti.family,"vanilla_family":vi.family,"glyph_count":ttf.number_of_glyphs(),"units_per_em":ttf.units_per_em(),"sample_glyph_ids_equal":true,"tekt_acquired_sfnt_sha256":sha(tf.as_slice()),"vanilla_acquired_data_sha256":sha(vf.data()),"vanilla_retained_collection_index":vf.index(),"queries":queries}));
        fixtures.push(Fixture {
            id: input["id"].as_str().unwrap().into(),
            path,
            bytes,
            index,
            points,
            tc,
            vc,
            vi,
            queries,
        });
    }
    (fixtures, serde_json::json!({"fixtures":controls,"preflight_outside_timers":true}))
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
        std::env::var("FONT_MICRO_OUTPUT").expect("explicit output required"),
    );
    fs::create_dir_all(&dest).unwrap();
    fs::write(
        dest.join("fixture-controls.json"),
        serde_json::to_vec_pretty(&controls).unwrap(),
    )
    .unwrap();
    let reverse = std::env::var("FONT_MICRO_REVERSE").as_deref() == Ok("1");
    let mut c = Criterion::default()
        .sample_size(30)
        .nresamples(5000)
        .warm_up_time(Duration::from_millis(200))
        .measurement_time(Duration::from_millis(600))
        .without_plots()
        .output_directory(&dest);
    for f in if reverse {
        fixtures.iter().rev().collect::<Vec<_>>()
    } else {
        fixtures.iter().collect()
    } {
        let mut g = c.benchmark_group(&f.id);
        let engines = if reverse { [true, false] } else { [false, true] };
        for vanilla in engines {
            if vanilla {
                g.bench_function("generic_constructor/vanilla_public", |b| {
                    b.iter_batched(
                        || f.points.clone(),
                        |p| VC::from_vec(black_box(p)),
                        BatchSize::PerIteration,
                    )
                });
            } else {
                g.bench_function("generic_constructor/tekt_public", |b| {
                    b.iter_batched(
                        || f.points.clone(),
                        |p| TC::from_codepoints(black_box(p)),
                        BatchSize::PerIteration,
                    )
                });
            }
        }
        for vanilla in engines {
            if vanilla {
                g.bench_function("clone_and_generic/vanilla_public", |b| {
                    b.iter_batched(
                        || (),
                        |()| VC::from_vec(black_box(&f.points).clone()),
                        BatchSize::PerIteration,
                    )
                });
            } else {
                g.bench_function("clone_and_generic/tekt_public", |b| {
                    b.iter_batched(
                        || (),
                        |()| TC::from_codepoints(black_box(&f.points).clone()),
                        BatchSize::PerIteration,
                    )
                });
            }
        }
        for vanilla in engines {
            if vanilla {
                g.bench_function("metadata_and_coverage/vanilla_public", |b| {
                    b.iter_batched(
                        || (),
                        |()| VI::new(black_box(&f.bytes), black_box(f.index)).unwrap(),
                        BatchSize::PerIteration,
                    )
                });
            } else {
                g.bench_function("metadata_and_coverage/tekt_composed", |b| {
                    b.iter_batched(
                        || (),
                        |()| tekt_full(black_box(&f.bytes), black_box(f.index)),
                        BatchSize::PerIteration,
                    )
                });
            }
        }
        for (name, p) in &f.queries {
            for vanilla in engines {
                if vanilla {
                    g.bench_function(format!("contains_{name}/vanilla_public"), |b| {
                        b.iter(|| black_box(&f.vc).contains(black_box(*p)))
                    });
                } else {
                    g.bench_function(format!("contains_{name}/tekt_public"), |b| {
                        b.iter(|| black_box(&f.tc).contains(black_box(*p)))
                    });
                }
            }
        }
        for vanilla in engines {
            if vanilla {
                g.bench_function("first_font/vanilla_store", |b| {
                    b.iter_batched_ref(
                        || store(&f.path, f.index, &f.vi),
                        |s| black_box(s).font(0).unwrap(),
                        BatchSize::PerIteration,
                    )
                });
            } else {
                g.bench_function("first_font/tekt_slot", |b| {
                    b.iter_batched_ref(
                        || FontSlot::new(f.path.clone(), f.index),
                        |s| black_box(s).get().unwrap(),
                        BatchSize::PerIteration,
                    )
                });
            }
        }
        let ts = FontSlot::new(f.path.clone(), f.index);
        assert!(ts.get().is_some());
        let vs = store(&f.path, f.index, &f.vi);
        assert!(vs.font(0).is_some());
        for vanilla in engines {
            if vanilla {
                g.bench_function("retained_font/vanilla_store", |b| {
                    b.iter_batched(
                        || (),
                        |()| black_box(&vs).font(0).unwrap(),
                        BatchSize::PerIteration,
                    )
                });
            } else {
                g.bench_function("retained_font/tekt_slot", |b| {
                    b.iter_batched(
                        || (),
                        |()| black_box(&ts).get().unwrap(),
                        BatchSize::PerIteration,
                    )
                });
            }
        }
        let face = ttf_parser::Face::parse(&f.bytes, f.index).unwrap();
        g.bench_function("common/cmap_collect_diagnostic", |b| {
            b.iter_batched(|| (), |()| collect(black_box(&face)), BatchSize::PerIteration)
        });
        g.bench_function("unmatched/tekt_metadata_only", |b| {
            b.iter_batched(
                || (),
                |()| {
                    font_info_from_bytes(black_box(&f.bytes), black_box(f.index)).unwrap()
                },
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
