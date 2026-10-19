use unicode_bidi::{BidiInfo, Level};
fn inspect(label: &str, text: &str, base: Option<Level>, line: std::ops::Range<usize>) {
    let b = BidiInfo::new(text, base);
    let p = &b.paragraphs[0];
    let (levels, runs) = b.visual_runs(p, line.clone());
    let chunks: Vec<_> = runs.iter().map(|r| serde_json::json!({"range":[r.start,r.end],"text":&text[r.clone()],"rtl":levels[r.start].is_rtl()})).collect();
    let words: Vec<_> = text.char_indices().filter(|(i,_)|line.contains(i)).map(|(i,c)|serde_json::json!({"byte":i,"char":c.to_string(),"level":levels[i].number()})).collect();
    println!("{}", serde_json::json!({"case":label,"input":text,"base":base.map(|l|l.number()),"line":[line.start,line.end],"runs":chunks,"levels":words}));
}
fn main() {
    for (n,t) in [("latin-first", "ABC 123 مرحبا"), ("arabic-first", "مرحبا ABC 123"), ("numeric-first", "123 456 مرحبا")] {
        inspect(&format!("{n}-auto"),t,None,0..t.len());
        inspect(&format!("{n}-ltr"),t,Some(Level::ltr()),0..t.len());
        inspect(&format!("{n}-rtl"),t,Some(Level::rtl()),0..t.len());
    }
    let full="ABC 123 456 789 مرحبا";
    let start=full.find("456").unwrap();let end=full.find(" مرحبا").unwrap();
    inspect("wrap-full-paragraph",full,Some(Level::rtl()),start..end);
    let slice=&full[start..end];inspect("wrap-slice-only",slice,Some(Level::rtl()),0..slice.len());
    let embedded="\u{202a}ABC 123\u{202c} مرحبا";
    inspect("explicit-ltr-in-rtl",embedded,Some(Level::rtl()),0..embedded.len());
}
