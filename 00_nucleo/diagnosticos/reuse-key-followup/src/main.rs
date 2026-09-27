//! Behavioral diagnostic only; no production changes or timing samples.
#![allow(deprecated)]
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{fs, path::PathBuf, sync::Arc};
use typst_core::entities::font_book::FontBook;
use typst_core::{
    contracts::world::World,
    entities::{
        content::Content,
        duration::Duration,
        file_id::FileId,
        font_list::FontList,
        font_variations::FontVariations,
        layout_types::{FrameItem, Page, PagedDocument, Point, Pt, TextStyle},
        source::Source,
        world_types::{Bytes, Datetime, FileResult, Font, Library},
    },
};
use typst_infra::{fonts::FontSlot, shaper::shape_document, world::SystemWorld};
fn hash(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn item(text: &str, family: &str, v: f32) -> FrameItem {
    FrameItem::Text {
        pos: Point { x: Pt(0.0), y: Pt(0.0) },
        text: text.into(),
        style: TextStyle {
            size: Pt(12.0),
            font: Some(FontList::single(family.into())),
            variations: Some(FontVariations(vec![(*b"wght", v)])),
            ..Default::default()
        },
    }
}
fn doc(items: Vec<FrameItem>) -> PagedDocument {
    PagedDocument::new(vec![Page {
        width: 595.0,
        height: 842.0,
        logical_number: 1,
        numbering: None,
        supplement: Content::Empty,
        bleed: Default::default(),
        fill: Default::default(),
        background: vec![],
        foreground: vec![],
        items,
    }])
}
fn signatures(doc: &PagedDocument) -> Vec<String> {
    doc.pages[0]
        .items
        .iter()
        .map(|i| match i {
            FrameItem::TextShaped { glyphs, .. } => format!("{glyphs:?}"),
            _ => panic!("fixture did not shape"),
        })
        .collect()
}

struct Pool {
    world: SystemWorld,
    font: Font,
}
struct Snapshot {
    files: SystemWorld,
    pool: Arc<Pool>,
}
// Deliberately limited diagnostic World: only methods exercised below.
impl World for Snapshot {
    fn library(&self) -> &Library {
        self.files.library()
    }
    fn book(&self) -> &FontBook {
        self.pool.world.book()
    }
    fn main(&self) -> FileId {
        self.files.main()
    }
    fn source(&self, id: FileId) -> FileResult<Source> {
        self.files.source(id)
    }
    fn file(&self, id: FileId) -> FileResult<Bytes> {
        self.files.file(id)
    }
    fn font(&self, i: usize) -> Option<Font> {
        if i == 0 { Some(self.pool.font.clone()) } else { None }
    }
    fn today(&self, o: Option<Duration>) -> Option<Datetime> {
        self.files.today(o)
    }
}
fn pool(path: PathBuf, root: &std::path::Path) -> Arc<Pool> {
    let w = SystemWorld::for_eval(root).with_fonts(vec![FontSlot::new(path, 0)]);
    let font = w.font(0).unwrap();
    Arc::new(Pool { world: w, font })
}
fn metric(data: &[u8], v: f32, chars: &[char]) -> Vec<u16> {
    let mut f = ttf_parser::Face::parse(data, 0).unwrap();
    f.set_variation(ttf_parser::Tag::from_bytes(b"wght"), v).unwrap();
    chars
        .iter()
        .map(|c| f.glyph_hor_advance(f.glyph_index(*c).unwrap()).unwrap())
        .collect()
}
fn main() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let scratch = root.join("runtime");
    fs::create_dir_all(&scratch).unwrap();
    let inputs: Value = serde_json::from_slice(include_bytes!("../inputs.json")).unwrap();
    let mut cases = vec![];
    for row in inputs.as_array().unwrap() {
        let path = PathBuf::from(row["path"].as_str().unwrap());
        let data = fs::read(&path).unwrap();
        assert_eq!(hash(&data), row["sha256"].as_str().unwrap());
        let face = ttf_parser::Face::parse(&data, 0).unwrap();
        let axis = face
            .variation_axes()
            .into_iter()
            .find(|a| a.tag == ttf_parser::Tag::from_bytes(b"wght"))
            .unwrap();
        assert!(axis.min_value > 0.0);
        let w = SystemWorld::for_eval(&scratch)
            .with_fonts(vec![FontSlot::new(path.clone(), 0)]);
        let family = w.book().infos()[0].family.clone();
        let chars: Vec<_> = "xHmA0aeW"
            .chars()
            .filter(|c| face.glyph_index(*c).is_some())
            .collect();
        let mut transitions = 0;
        let mut collisions = 0;
        let mut found = None;
        let mut lo = axis.min_value;
        let mut last = metric(&data, lo, &chars);
        while lo < axis.max_value {
            let hi = (lo + 0.5).min(axis.max_value);
            let next = metric(&data, hi, &chars);
            if next != last {
                transitions += 1;
                let mut lb = lo.to_bits();
                let mut hb = hi.to_bits();
                while hb - lb > 1 {
                    let mb = lb + (hb - lb) / 2;
                    if metric(&data, f32::from_bits(mb), &chars) == last {
                        lb = mb
                    } else {
                        hb = mb
                    }
                }
                let a = f32::from_bits(lb);
                let b = f32::from_bits(hb);
                if format!("{a:.4}") == format!("{b:.4}") {
                    collisions += 1;
                    let ma = metric(&data, a, &chars);
                    let mb = metric(&data, b, &chars);
                    let c = chars[ma.iter().zip(&mb).position(|(x, y)| x != y).unwrap()];
                    let text = c.to_string();
                    let sa = signatures(&shape_document(
                        &w,
                        doc(vec![item(&text, &family, a)]),
                    ));
                    let sb = signatures(&shape_document(
                        &w,
                        doc(vec![item(&text, &family, b)]),
                    ));
                    if sa != sb {
                        let ab = signatures(&shape_document(
                            &w,
                            doc(vec![item(&text, &family, a), item(&text, &family, b)]),
                        ));
                        let ba = signatures(&shape_document(
                            &w,
                            doc(vec![item(&text, &family, b), item(&text, &family, a)]),
                        ));
                        found = Some(
                            json!({"text":text,"a":a,"b":b,"a_bits":lb,"b_bits":hb,"four_decimals":format!("{a:.4}"),"metric_a":ma,"metric_b":mb,"fresh_a":sa,"fresh_b":sb,"mixed_ab":ab,"mixed_ba":ba,"ab_matches_fresh":ab[0]==sa[0]&&ab[1]==sb[0],"ba_matches_fresh":ba[0]==sb[0]&&ba[1]==sa[0]}),
                        );
                        break;
                    }
                }
            }
            lo = hi;
            last = next;
        }
        cases.push(json!({"path":path,"family":family,"axis_min":axis.min_value,"axis_max":axis.max_value,"grid_step":0.5,"characters":chars,"metric_transitions_examined":transitions,"formatted_collisions_examined":collisions,"counterexample":found}));
    }
    let source = scratch.join("main.typ");
    fs::write(&source, "First snapshot").unwrap();
    let paths: Vec<_> = inputs
        .as_array()
        .unwrap()
        .iter()
        .map(|r| PathBuf::from(r["path"].as_str().unwrap()))
        .collect();
    let p = pool(paths[0].clone(), &scratch);
    let a = Snapshot {
        files: SystemWorld::new(&scratch, "main.typ").unwrap(),
        pool: p.clone(),
    };
    let initial = a.source(a.main()).unwrap();
    let af = a.font(0).unwrap();
    fs::write(&source, "Edited snapshot").unwrap();
    let b = Snapshot {
        files: SystemWorld::new(&scratch, "main.typ").unwrap(),
        pool: p.clone(),
    };
    let edited = b.source(b.main()).unwrap();
    let bf = b.font(0).unwrap();
    assert_ne!(initial.text(), edited.text());
    assert_eq!(af.as_slice().as_ptr(), bf.as_slice().as_ptr());
    assert_eq!(a.source(a.main()).unwrap().text(), initial.text());
    let replacement = pool(paths[1].clone(), &scratch);
    let c = Snapshot {
        files: SystemWorld::new(&scratch, "main.typ").unwrap(),
        pool: replacement,
    };
    assert_ne!(a.book().infos()[0].family, c.book().infos()[0].family);
    assert_eq!(a.font(0).unwrap().as_slice(), af.as_slice());
    let lifecycle = json!({"initial_source":initial.text(),"fresh_source_with_retained_pool":edited.text(),"same_font_buffer":true,"old_snapshot_preserved":true,"old_family":a.book().infos()[0].family,"replacement_family":c.book().infos()[0].family,"generation_replaced_as_book_font_pair":true,"limited_explicit_single_font_prototype":true});
    fs::write(&source, "First snapshot").unwrap();
    println!("{}",serde_json::to_string_pretty(&json!({"axis_cases":cases,"split_lifecycle":lifecycle,"no_timing_measurements":true})).unwrap());
}
