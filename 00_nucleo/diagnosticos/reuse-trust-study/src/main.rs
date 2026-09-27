//! Diagnostic lifecycle controls. No performance measurement or production change.
#![allow(deprecated)]
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{fs, path::PathBuf};
use typst_core::{
    contracts::world::World,
    entities::{
        content::Content,
        font_list::FontList,
        layout_types::{FrameItem, Page, PagedDocument, Point, Pt, TextStyle},
    },
};
use typst_infra::{fonts::FontSlot, shaper::shape_document, world::SystemWorld};
fn hash(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn item(text: &str, math: bool) -> FrameItem {
    FrameItem::Text {
        pos: Point { x: Pt(0.0), y: Pt(0.0) },
        text: text.into(),
        style: TextStyle {
            size: Pt(12.0),
            font: Some(FontList::single("New Computer Modern Math".into())),
            math,
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
fn main() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let scratch = root.join("runtime");
    fs::create_dir_all(&scratch).unwrap();
    let inputs: Value = serde_json::from_slice(include_bytes!("../inputs.json")).unwrap();
    for row in inputs.as_array().unwrap() {
        assert_eq!(
            hash(&fs::read(row["path"].as_str().unwrap()).unwrap()),
            row["sha256"].as_str().unwrap()
        );
    }
    let paths: Vec<_> = inputs
        .as_array()
        .unwrap()
        .iter()
        .map(|r| PathBuf::from(r["path"].as_str().unwrap()))
        .collect();
    let source = scratch.join("main.typ");
    fs::write(&source, "First snapshot").unwrap();
    let retained = SystemWorld::new(&scratch, "main.typ").unwrap();
    let original = retained.source(retained.main()).unwrap();
    fs::write(&source, "Edited snapshot").unwrap();
    let stale = retained.source(retained.main()).unwrap();
    let clean = SystemWorld::new(&scratch, "main.typ").unwrap();
    let edited = clean.source(clean.main()).unwrap();
    assert_eq!(original.text(), stale.text());
    assert_ne!(stale.text(), edited.text());
    let source_case = json!({"initial":original.text(),"retained_after_edit":stale.text(),"fresh_after_edit":edited.text(),"retained_reads_previous_snapshot":true,"note":"Current per-World snapshot contract; demonstrates why retaining this entire World without file invalidation is unsafe for recompilation, not a defect of the current fresh-World watch."});
    let font_path = scratch.join("reappearing.otf");
    assert!(!font_path.exists());
    let failed = FontSlot::new(font_path.clone(), 0);
    assert!(failed.get().is_none());
    fs::copy(&paths[0], &font_path).unwrap();
    assert!(failed.get().is_none());
    let fresh = FontSlot::new(font_path.clone(), 0);
    assert!(fresh.get().is_some());
    fs::remove_file(&font_path).unwrap();
    let failure_case = json!({"missing_first_get":false,"same_slot_after_file_creation":false,"replacement_slot_after_file_creation":true,"note":"OnceLock caches None. File recovery requires replacing/invalidation of the slot; source bytes remain owned by live handles."});
    let w1 = SystemWorld::for_eval(&scratch)
        .with_fonts(vec![FontSlot::new(paths[2].clone(), 0)]);
    let a = w1.font(0).unwrap();
    let again = w1.font(0).unwrap();
    let w2 = SystemWorld::for_eval(&scratch)
        .with_fonts(vec![FontSlot::new(paths[2].clone(), 0)]);
    let independent = w2.font(0).unwrap();
    assert_eq!(a.as_slice().as_ptr(), again.as_slice().as_ptr());
    assert_ne!(a.as_slice().as_ptr(), independent.as_slice().as_ptr());
    assert_eq!(a.as_slice(), independent.as_slice());
    let acquisition = json!({"same_world_shares_buffer":true,"fresh_world_has_distinct_live_buffer":true,"contents_equal":true,"sfnt_bytes":a.as_slice().len(),"sha256":hash(a.as_slice()),"note":"Both handles kept alive; public explicit-slot builder isolates acquisition, not whole CLI discovery. Pointer identity is only a lifecycle control."});
    let before = w1.candidates_for_char('中');
    let swapped = w1.with_fonts(vec![FontSlot::new(paths[0].clone(), 0)]);
    let after = swapped.candidates_for_char('中');
    let clean_swap = SystemWorld::for_eval(&scratch)
        .with_fonts(vec![FontSlot::new(paths[0].clone(), 0)]);
    assert!(!before.is_empty());
    assert!(after.is_empty());
    assert_eq!(after, clean_swap.candidates_for_char('中'));
    let replacement = json!({"candidates_before":before,"candidates_after":after,"matches_fresh_catalog":true,"note":"Builder replacement clears coverage tied to old slot indices; old acquired Font handles remain valid."});
    let shape_world = SystemWorld::for_eval(&scratch)
        .with_fonts(vec![FontSlot::new(paths[1].clone(), 0)]);
    let mut shape_cases = Vec::new();
    for text in ["ffi", "office", "ab", "Hello", "123", "sin"] {
        let normal =
            signatures(&shape_document(&shape_world, doc(vec![item(text, false)])));
        let math = signatures(&shape_document(&shape_world, doc(vec![item(text, true)])));
        let forward = signatures(&shape_document(
            &shape_world,
            doc(vec![item(text, false), item(text, true)]),
        ));
        let reverse = signatures(&shape_document(
            &shape_world,
            doc(vec![item(text, true), item(text, false)]),
        ));
        assert_eq!(forward.len(), 2);
        assert_eq!(reverse.len(), 2);
        shape_cases.push(json!({"text":text,"normal_fresh":normal[0],"math_fresh":math[0],"forward":forward,"reverse":reverse,"forward_matches_fresh":forward[0]==normal[0]&&forward[1]==math[0],"reverse_matches_fresh":reverse[0]==math[0]&&reverse[1]==normal[0],"fresh_modes_differ":normal!=math}));
    }
    let x = 400.0_f32;
    let y = 400.00003_f32;
    assert_ne!(x.to_bits(), y.to_bits());
    assert_eq!(format!("{x:.4}"), format!("{y:.4}"));
    let output = json!({"source_snapshot":source_case,"negative_font_cache":failure_case,"ttc_acquisition_lifetime":acquisition,"catalog_replacement":replacement,"shape_mixed_mode_vs_fresh":shape_cases,"key_precision_probe":{"axis_x_bits":x.to_bits(),"axis_y_bits":y.to_bits(),"formatted_x":format!("{x:.4}"),"formatted_y":format!("{y:.4}"),"note":"Demonstrates a formatting collision used by current run keys; not a production private-key invocation or demonstrated variable-font output defect."},"no_timing_measurements":true});
    fs::write(&source, "First snapshot").unwrap();
    println!("{}", serde_json::to_string_pretty(&output).unwrap());
}
