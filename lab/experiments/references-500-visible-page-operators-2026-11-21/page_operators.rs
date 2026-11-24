pub(crate) fn same_page(a: &Page, b: &Page) -> bool {
    a.width.to_bits() == b.width.to_bits()
        && a.height.to_bits() == b.height.to_bits()
        && a.bleed.is_zero() && b.bleed.is_zero()
        && a.fill == b.fill
        && same_items(&a.background, &b.background)
        && same_items(&a.items, &b.items)
        && same_items(&a.foreground, &b.foreground)
}

fn point(a: &Point, b: &Point) -> bool {
    a.x.0.to_bits() == b.x.0.to_bits() && a.y.0.to_bits() == b.y.0.to_bits()
}

fn style(a: &TextStyle, b: &TextStyle) -> bool {
    let color_bits = |c: &typst_core::entities::layout_types::Color| {
        let (r,g,b,a) = c.to_rgba_f32();
        [r,g,b,a].map(f32::to_bits)
    };
    a == b && a.size.0.to_bits() == b.size.0.to_bits()
        && a.tracking.map(|v| (v.abs.to_pt().to_bits(),v.em.to_bits()))
            == b.tracking.map(|v| (v.abs.to_pt().to_bits(),v.em.to_bits()))
        && a.fill.as_ref().map(color_bits)
            == b.fill.as_ref().map(color_bits)
}

fn same_bidi(
    a: &Option<typst_core::entities::bidi_context::BidiSpan>,
    b: &Option<typst_core::entities::bidi_context::BidiSpan>,
) -> bool {
    use typst_core::entities::bidi_context::ParagraphBidiSegment::*;
    match (a,b) {
        (None,None) => true,
        (Some(a),Some(b)) => a.segment() == b.segment() && a.range() == b.range()
            && (Arc::ptr_eq(a.input(), b.input()) || {
                let (a,b) = (a.input(), b.input());
                a.base_dir() == b.base_dir() && a.segments().len() == b.segments().len()
                    && a.segments().iter().zip(b.segments()).all(|(a,b)| match (a,b) {
                        (Text {text,dir},Text {text:t,dir:d}) => text == t && dir == d,
                        (Space {origin},Space {origin:o}) => origin == o,
                        (Linebreak,Linebreak) => true,
                        _ => false,
                    })
            }),
        _ => false,
    }
}

fn same_items(a: &[FrameItem], b: &[FrameItem]) -> bool {
    a.len() == b.len() && a.iter().zip(b).all(|(a, b)| match (a,b) {
        (FrameItem::Text { pos, text, style: s, bidi },
         FrameItem::Text { pos: p, text: t, style: z, bidi: d }) =>
            point(pos,p) && text==t && style(s,z) && same_bidi(bidi,d),
        (FrameItem::TextShaped { pos, text, glyphs, style: s, units_per_em, font, bidi },
         FrameItem::TextShaped { pos: p, text: t, glyphs: g, style: z, units_per_em: em, font: f, bidi: d }) =>
            point(pos,p) && text==t && glyphs==g && style(s,z) && units_per_em==em
                && match (font,f) {
                    (Some(a),Some(b)) => std::ptr::eq(a.as_slice(), b.as_slice()) || a == b,
                    (None,None) => true,
                    _ => false,
                } && same_bidi(bidi,d),
        (FrameItem::TextSeparator { pos, advance, style: s, origin, bidi },
         FrameItem::TextSeparator { pos: p, advance: x, style: z, origin: o, bidi: d }) =>
            point(pos,p) && advance.0.to_bits()==x.0.to_bits() && style(s,z) && origin==o && same_bidi(bidi,d),
        (FrameItem::TextReplacement { text, items }, FrameItem::TextReplacement { text:t,items:b }) =>
            text==t && same_items(items,b),
        (FrameItem::Semantic { kind, placement, alt, items },
         FrameItem::Semantic { kind:k, placement:p, alt:t, items:b }) =>
            kind==k && placement==p && alt==t && same_items(items,b),
        (FrameItem::Link { target, items, pos, size },
         FrameItem::Link { target:t, items:b, pos:p, size:s }) =>
            target==t && point(pos,p) && size==s && same_items(items,b),
        (FrameItem::Group { paint_root, pos, matrix: m, clip_mask:None, inner_width, inner_height, items },
         FrameItem::Group { paint_root:r, pos:p, matrix:n, clip_mask:None, inner_width:w, inner_height:h, items:b }) =>
            paint_root==r && point(pos,p) && inner_width.to_bits()==w.to_bits() && inner_height.to_bits()==h.to_bits()
                && [m.a,m.b,m.c,m.d,m.tx,m.ty].map(f64::to_bits) == [n.a,n.b,n.c,n.d,n.tx,n.ty].map(f64::to_bits)
                && same_items(items,b),
        _ => false,
    })
}

/// Font slots are already bound to identical faces in identical order.
/// An old CID is usable only if both the shaped and character paths agree
/// on its translation and the original glyph keeps its nominal advance.
pub(crate) fn cid_translation(old: &OperatorMaps, new: &OperatorMaps) -> Vec<HashMap<u16,u16>> {
    old.remap.iter().zip(&new.remap).enumerate().map(|(font,(before,after))| {
        let mut translated = HashMap::new();
        let mut seen = BTreeSet::new();
        for (&gid, &cid) in before {
            if cid == 0 { continue; }
            if !seen.insert(cid) {
                translated.remove(&cid);
            } else if let Some(&next) = after.get(&gid) {
                if next != 0 && old.nominal[font].get(&gid) == new.nominal[font].get(&gid) {
                    translated.insert(cid,next);
                }
            }
        }
        for (&ch, &cid) in &old.character[font] {
            if translated.get(&cid) != new.character[font].get(&ch) {
                translated.remove(&cid);
            }
        }
        translated
    }).collect()
}

/// Rewrite only fixed-width glyph operands emitted by the text operators.
/// Marked-content strings (including ActualText) and all positions stay intact.
/// An unrecognized text-show form or CID causes ordinary emission of the page.
pub(crate) fn translate_operators(bytes: &[u8], maps: &[HashMap<u16,u16>]) -> Option<Arc<[u8]>> {
    const HEX: &[u8; 16] = b"0123456789ABCDEF";
    let text = std::str::from_utf8(bytes).ok()?;
    let mut out = bytes.to_vec();
    let mut font = None;
    let mut offset = 0;
    for line in text.split_inclusive('\n') {
        let body = line.strip_suffix('\n')?.trim();
        if body == "BT" || body.ends_with(" BT") {
            font = None;
        } else if let Some(selection) = body.strip_suffix(" Tf") {
            font = selection.split_ascii_whitespace().next()?.strip_prefix("/F")?
                .parse::<usize>().ok()?.checked_sub(1);
        } else if body.ends_with(" TJ") || body.ends_with(" Tj") {
            let map = maps.get(font?)?;
            let input = body.as_bytes();
            let line_offset = offset + line.len()-line.trim_start().len();
            let mut start = 0;
            while let Some(relative) = input[start..].iter().position(|&b| b==b'<') {
                let from = start+relative+1;
                let end = from+input[from..].iter().position(|&b| b==b'>')?;
                if (end-from)%4 != 0 { return None; }
                for index in (from..end).step_by(4) {
                    let mut cid = 0u16;
                    for &digit in &input[index..index+4] {
                        let value = match digit {
                            b'0'..=b'9' => digit-b'0',
                            b'A'..=b'F' => digit-b'A'+10,
                            _ => return None,
                        };
                        cid = (cid<<4) | u16::from(value);
                    }
                    let new = *map.get(&cid)?;
                    if new != cid {
                        let into = line_offset+index;
                        for (at,shift) in [12,8,4,0].into_iter().enumerate() {
                            out[into+at] = HEX[usize::from((new>>shift)&15)];
                        }
                    }
                }
                start = end+1;
            }
            if body.contains('(') { return None; }
        } else if body.ends_with(" '") || body.ends_with(" \"") {
            return None;
        }
        offset += line.len();
    }
    Some(out.into())
}
