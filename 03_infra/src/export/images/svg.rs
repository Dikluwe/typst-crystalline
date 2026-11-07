//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/images/svg.md
//! @prompt-hash c2d910b0
//! @layer L3
//! @updated 2026-11-01

use pdf_writer::{Pdf, Ref};
use std::collections::HashMap;

pub(in crate::export) struct SvgBundle {
    pub(in crate::export) root: usize,
    pub(in crate::export) next: usize,
    pub(in crate::export) objects: Vec<(usize, Vec<u8>)>,
}

pub(in crate::export) fn prepare(data: &[u8], start: usize) -> Result<SvgBundle, String> {
    // Render data is already self-contained: never enable default path resolvers here.
    let mut opts = svg2pdf::usvg::Options::default();
    opts.image_href_resolver.resolve_string = Box::new(|_, _| None);
    let tree = svg2pdf::usvg::Tree::from_data(data, &opts).map_err(|e| e.to_string())?;
    let options = svg2pdf::ConversionOptions { embed_text: false, ..Default::default() };
    let (chunk, root) = svg2pdf::to_chunk(&tree, options).map_err(|e| e.to_string())?;
    let count = chunk.refs().len();
    let next = start
        .checked_add(count)
        .filter(|n| *n < i32::MAX as usize)
        .ok_or("SVG PDF object ID limit exceeded")?;
    let ids: HashMap<_, _> = chunk
        .refs()
        .enumerate()
        .map(|(i, id)| (id, Ref::new((start + i) as i32)))
        .collect();
    let root = ids.get(&root).ok_or("SVG PDF form has no root object")?.get() as usize;
    let mut missing = false;
    let chunk = chunk.renumber(|r| match ids.get(&r) {
        Some(id) => *id,
        None => {
            missing = true;
            r
        }
    });
    if missing {
        return Err("SVG PDF form contains an unresolved resource".into());
    }
    // The writer's xref is authoritative for binary object boundaries. Never scan
    // stream data for endobj: image/filter payloads can contain arbitrary bytes.
    let mut pdf = Pdf::new();
    pdf.extend(&chunk);
    let pdf = pdf.finish();
    let marker = b"startxref\n";
    let at = pdf
        .windows(marker.len())
        .rposition(|s| s == marker)
        .ok_or("SVG PDF xref missing")?
        + marker.len();
    let end = pdf[at..]
        .iter()
        .position(|b| *b == b'\n')
        .ok_or("SVG PDF xref offset missing")?
        + at;
    let xref = std::str::from_utf8(&pdf[at..end])
        .map_err(|e| e.to_string())?
        .parse::<usize>()
        .map_err(|e| e.to_string())?;
    let text = std::str::from_utf8(pdf.get(xref..).ok_or("SVG PDF xref out of range")?)
        .map_err(|e| e.to_string())?;
    let mut lines = text.lines();
    if lines.next() != Some("xref") {
        return Err("unsupported SVG backend xref representation".into());
    }
    let mut offsets = Vec::new();
    while let Some(line) = lines.next() {
        if line == "trailer" {
            break;
        }
        let pair = line.split_whitespace().collect::<Vec<_>>();
        if pair.len() != 2 {
            return Err("invalid SVG backend xref subsection".into());
        }
        let first = pair[0].parse::<usize>().map_err(|e| e.to_string())?;
        let count = pair[1].parse::<usize>().map_err(|e| e.to_string())?;
        for i in 0..count {
            let entry = lines
                .next()
                .ok_or("truncated SVG backend xref")?
                .split_whitespace()
                .collect::<Vec<_>>();
            if entry.len() != 3 {
                return Err("invalid SVG backend xref entry".into());
            }
            if entry[2] == "n" {
                offsets.push((
                    first + i,
                    entry[0].parse::<usize>().map_err(|e| e.to_string())?,
                ));
            }
        }
    }
    offsets.sort_by_key(|(_, at)| *at);
    let mut objects = Vec::new();
    for (index, (id, at)) in offsets.iter().enumerate() {
        let end = offsets.get(index + 1).map_or(xref, |(_, at)| *at);
        let bytes = pdf.get(*at..end).ok_or("invalid SVG backend object boundary")?;
        let header = format!("{id} 0 obj\n");
        let body = bytes
            .strip_prefix(header.as_bytes())
            .ok_or("invalid SVG backend object header")?;
        let mut stop = body.len();
        while stop > 0 && body[stop - 1].is_ascii_whitespace() {
            stop -= 1;
        }
        let body = body[..stop]
            .strip_suffix(b"endobj")
            .ok_or("invalid SVG backend object end")?;
        objects.push((*id, body.to_vec()));
    }
    if objects.len() != count {
        return Err("SVG PDF resource count mismatch".into());
    }
    Ok(SvgBundle { root, next, objects })
}
