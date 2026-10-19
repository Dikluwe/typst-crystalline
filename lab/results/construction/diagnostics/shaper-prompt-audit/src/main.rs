//! Read-only behavioral audit via public APIs. No performance measurement.
#![allow(deprecated)]
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{fs, path::PathBuf};
use typst_core::{
    contracts::world::World,
    entities::{
        content::Content,
        duration::Duration,
        file_id::FileId,
        font_book::{FontBook, FontVariant},
        font_list::FontList,
        layout_types::{FrameItem, Length, Page, PagedDocument, Point, Pt, TextStyle},
        source::Source,
        world_types::{Bytes, Datetime, FileResult, Font, Library},
    },
};
use typst_infra::{fonts::FontSlot, shaper::shape_document, world::SystemWorld};
fn item(text: &str, style: TextStyle) -> FrameItem {
    FrameItem::Text {
        pos: Point { x: Pt(0.0), y: Pt(0.0) },
        text: text.into(),
        style,
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

fn compare(w: &dyn World, text: &str, a: TextStyle, b: TextStyle) -> Value {
    let fa = signatures(&shape_document(w, doc(vec![item(text, a.clone())])));
    let fb = signatures(&shape_document(w, doc(vec![item(text, b.clone())])));
    let ab = signatures(&shape_document(
        w,
        doc(vec![item(text, a.clone()), item(text, b.clone())]),
    ));
    let ba = signatures(&shape_document(w, doc(vec![item(text, b), item(text, a)])));
    json!({"text":text,"fresh_a":fa,"fresh_b":fb,"mixed_ab":ab,"mixed_ba":ba,"fresh_differ":fa!=fb,"ab_matches_fresh":ab==[fa[0].clone(),fb[0].clone()],"ba_matches_fresh":ba==[fb[0].clone(),fa[0].clone()]})
}
struct ControlledWorld {
    book: FontBook,
    fonts: Vec<Font>,
    library: Library,
}
impl World for ControlledWorld {
    fn library(&self) -> &Library {
        &self.library
    }
    fn book(&self) -> &FontBook {
        &self.book
    }
    fn main(&self) -> FileId {
        panic!("not exercised")
    }
    fn source(&self, _: FileId) -> FileResult<Source> {
        panic!("not exercised")
    }
    fn file(&self, _: FileId) -> FileResult<Bytes> {
        panic!("not exercised")
    }
    fn font(&self, i: usize) -> Option<Font> {
        self.fonts.get(i).cloned()
    }
    fn today(&self, _: Option<Duration>) -> Option<Datetime> {
        None
    }
    fn candidates_for_char(&self, c: char) -> Vec<usize> {
        if c == '中' { vec![5, 6] } else { vec![] }
    }
}
fn main() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let rows: Value = serde_json::from_slice(include_bytes!("../inputs.json")).unwrap();
    let paths: Vec<_> = rows
        .as_array()
        .unwrap()
        .iter()
        .map(|r| {
            let p = PathBuf::from(r["path"].as_str().unwrap());
            assert_eq!(
                format!("{:x}", Sha256::digest(fs::read(&p).unwrap())),
                r["sha256"].as_str().unwrap()
            );
            p
        })
        .collect();
    let w =
        SystemWorld::for_eval(&root).with_fonts(vec![FontSlot::new(paths[0].clone(), 0)]);
    let family = w.book().infos()[0].family.clone();
    let base = TextStyle {
        font: Some(FontList::single(family.clone().into())),
        size: Pt(12.0),
        ..Default::default()
    };
    let mut a = base.clone();
    let mut b = base.clone();
    a.tracking = Some(Length::pt(1.0));
    b.tracking = a.tracking;
    b.size = Pt(24.0);
    let size_case = compare(&w, "ab", a, b);
    let mut a = base.clone();
    let mut b = base.clone();
    a.tracking = Some(Length::pt(0.00599));
    b.tracking = Some(Length::pt(0.00601));
    let precision_case = compare(&w, "ab", a, b);
    let mut scripts = vec![];
    for path in &paths {
        let w =
            SystemWorld::for_eval(&root).with_fonts(vec![FontSlot::new(path.clone(), 0)]);
        let family = w.book().infos()[0].family.clone();
        for text in ["ffi", "office", "AV", "ab", "sin", "123"] {
            let a = TextStyle {
                font: Some(FontList::single(family.clone().into())),
                size: Pt(12.0),
                ..Default::default()
            };
            let mut b = a.clone();
            b.math = true;
            scripts.push(json!({"font":family,"result":compare(&w,text,a,b)}));
        }
    }
    let segmentation_doc = shape_document(&w, doc(vec![item("abαβ", base.clone())]));
    let segmentation: Vec<_> = segmentation_doc.pages[0]
        .items
        .iter()
        .map(|i| match i {
            FrameItem::TextShaped { text, glyphs, .. } => {
                assert!(glyphs.iter().all(|g| g.glyph_id != 0));
                text.to_string()
            }
            _ => panic!("segmentation control did not shape"),
        })
        .collect();
    let cjk =
        SystemWorld::for_eval(&root).with_fonts(vec![FontSlot::new(paths[2].clone(), 0)]);
    let mut book = FontBook::new();
    let base_info = w.book().infos()[0].clone();
    for i in 0..7 {
        let mut info =
            if i >= 5 { cjk.book().infos()[0].clone() } else { base_info.clone() };
        info.family = match i {
            0 => "Primary",
            1 => "Primary Preferred Decoy",
            2 => "ZZZ Decoy",
            5 => "Far Fallback With Long Name",
            6 => "Primary CJK",
            _ => "Unused",
        }
        .into();
        info.flags = base_info.flags.clone();
        info.variant = base_info.variant.clone();
        book.push(info);
    }
    let expected = book
        .select_fallback(Some(&book.infos()[0]), &FontVariant::default(), [5, 6])
        .unwrap();
    assert_eq!(expected, 6);
    let fonts = (0..7)
        .map(|i| if i >= 5 { cjk.font(0).unwrap() } else { w.font(0).unwrap() })
        .collect();
    let cw = ControlledWorld { book, fonts, library: Library::new() };
    let d = shape_document(
        &cw,
        doc(vec![item(
            "中",
            TextStyle {
                font: Some(FontList::single("Primary".into())),
                size: Pt(12.0),
                ..Default::default()
            },
        )]),
    );
    let actual = match &d.pages[0].items[0] {
        FrameItem::TextShaped { style, .. } => format!("{:?}", style.font),
        _ => panic!("did not shape"),
    };
    println!("{}",serde_json::to_string_pretty(&json!({"tracking_size":{"size_a_pt":12,"size_b_pt":24,"tracking_pt":1,"result":size_case},"tracking_precision":{"tracking_a_pt":0.00599,"tracking_b_pt":0.00601,"formatted_a":format!("{:.4}",0.00599),"formatted_b":format!("{:.4}",0.00601),"result":precision_case},"script_controls":scripts,"sparse_fallback_scoring":{"global_candidates":[5,6],"expected_global_slot":expected,"expected_family":cw.book.infos()[expected].family,"actual_emitted_font_list":actual,"fixture":"Controlled metadata, real CJK bytes at both fallback slots; equal coverage and run length. Decoy metadata at slots 1/2 isolates local/global index confusion."},"same_font_script_segmentation":{"input":"abαβ","font":family,"actual_subrun_texts":segmentation,"expected_script_boundaries":["ab","αβ"],"note":"Same face supports both scripts; checks explicit L0 segmentation contract, not a vanilla mechanical parity requirement."},"no_timing_measurements":true})).unwrap());
}
