//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/fonts/coverage.md
//! @prompt-hash b21488b5
//! @layer L3
//! @updated 2026-10-10

use std::ops::Range;
use ttf_parser::{cmap::Format, Face, Tag};
use typst_core::entities::font_book::Coverage;

pub(super) fn extract(face: &Face) -> Coverage {
    match collect(face) {
        Some(ranges) => Coverage::from_ranges(ranges),
        None => legacy(face),
    }
}

fn legacy(face: &Face) -> Coverage {
    let mut codepoints = Vec::new();
    let Some(cmap) = face.tables().cmap else { return Coverage::new() };
    for subtable in cmap.subtables {
        if subtable.is_unicode() {
            subtable.codepoints(&mut |codepoint| {
                codepoints.push(codepoint);
            });
        }
    }
    Coverage::from_codepoints(codepoints)
}

fn u16_at(data: &[u8], offset: usize) -> Option<u16> {
    Some(u16::from_be_bytes(data.get(offset..offset.checked_add(2)?)?.try_into().ok()?))
}
fn u32_at(data: &[u8], offset: usize) -> Option<u32> {
    Some(u32::from_be_bytes(data.get(offset..offset.checked_add(4)?)?.try_into().ok()?))
}

fn native(data: &[u8], format: u16, out: &mut Vec<Range<u32>>) -> Option<()> {
    match format {
        4 => {
            let count = usize::from(u16_at(data, 6)?) / 2;
            let starts = 16usize.checked_add(count.checked_mul(2)?)?;
            for i in 0..count {
                let delta = i.checked_mul(2)?;
                let start = u32::from(u16_at(data, starts.checked_add(delta)?)?);
                let end = u32::from(u16_at(data, 14usize.checked_add(delta)?)?);
                if start == 0xffff && end == 0xffff {
                    break;
                }
                if start <= end {
                    out.push(start..end + 1);
                }
            }
        }
        12 | 13 => {
            let count = usize::try_from(u32_at(data, 12)?).ok()?;
            for i in 0..count {
                let at = 16usize.checked_add(i.checked_mul(12)?)?;
                let start = u32_at(data, at)?;
                let end = u32_at(data, at.checked_add(4)?)?;
                if start <= end {
                    out.push(start..end.checked_add(1)?);
                }
            }
        }
        _ => return None,
    }
    Some(())
}

fn collect(face: &Face) -> Option<Vec<Range<u32>>> {
    let mut out = Vec::new();
    let Some(cmap) = face.tables().cmap else {
        return Some(out);
    };
    let raw = face.raw_face().table(Tag::from_bytes(b"cmap"));
    let mut seen = [None; 8];
    for i in 0..cmap.subtables.len() {
        // Match the typed iterator's first-invalid-record termination boundary.
        let Some(subtable) = cmap.subtables.get(i) else {
            break;
        };
        if !subtable.is_unicode() {
            continue;
        }
        let format = match subtable.format {
            Format::SegmentMappingToDeltaValues(_) => Some(4),
            Format::SegmentedCoverage(_) => Some(12),
            Format::ManyToOneRangeMappings(_) => Some(13),
            _ => None,
        };
        let checkpoint = out.len();
        let native_ok = format
            .and_then(|format| {
                raw.and_then(|data| {
                    let record = 8usize.checked_add(usize::from(i).checked_mul(8)?)?;
                    let offset = usize::try_from(u32_at(data, record)?).ok()?;
                    let bytes = data.get(offset..)?;
                    if u16_at(bytes, 0)? != format {
                        return None;
                    }
                    let key = (offset, format);
                    if seen.contains(&Some(key)) {
                        return Some(());
                    }
                    native(bytes, format, &mut out)?;
                    if let Some(slot) = seen.iter_mut().find(|slot| slot.is_none()) {
                        *slot = Some(key);
                    }
                    Some(())
                })
            })
            .is_some();
        if native_ok {
            continue;
        }
        out.truncate(checkpoint);
        let mut active: Option<Range<u32>> = None;
        let mut overflow = false;
        subtable.codepoints(&mut |point: u32| {
            let Some(end) = point.checked_add(1) else {
                overflow = true;
                return;
            };
            if let Some(range) = active.as_mut() {
                if point == range.end {
                    range.end = end;
                    return;
                }
                if range.contains(&point) {
                    return;
                }
                if end == range.start {
                    range.start = point;
                    return;
                }
            }
            if let Some(range) = active.replace(point..end) {
                out.push(range);
            }
        });
        if overflow {
            return None;
        }
        if let Some(range) = active {
            out.push(range);
        }
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;
    fn put16(v: &mut Vec<u8>, n: u16) {
        v.extend(n.to_be_bytes());
    }
    fn put32(v: &mut Vec<u8>, n: u32) {
        v.extend(n.to_be_bytes());
    }
    fn format4(segments: &[(u16, u16)]) -> Vec<u8> {
        let mut v = vec![];
        for n in
            [4, (16 + 8 * segments.len()) as u16, 0, (2 * segments.len()) as u16, 0, 0, 0]
        {
            put16(&mut v, n);
        }
        for &(_, end) in segments {
            put16(&mut v, end);
        }
        put16(&mut v, 0);
        for &(start, _) in segments {
            put16(&mut v, start);
        }
        for _ in 0..2 * segments.len() {
            put16(&mut v, 0);
        }
        v
    }
    fn grouped(format: u16, groups: &[(u32, u32, u32)]) -> Vec<u8> {
        let mut v = vec![];
        put16(&mut v, format);
        put16(&mut v, 0);
        put32(&mut v, (16 + 12 * groups.len()) as u32);
        put32(&mut v, 0);
        put32(&mut v, groups.len() as u32);
        for &(start, end, glyph) in groups {
            for n in [start, end, glyph] {
                put32(&mut v, n);
            }
        }
        v
    }
    fn fixture(tables: &[(u16, u16, Vec<u8>)]) -> Vec<u8> {
        let mut cmap = vec![];
        put16(&mut cmap, 0);
        put16(&mut cmap, tables.len() as u16);
        let mut offset = 4 + 8 * tables.len();
        for (platform, encoding, data) in tables {
            put16(&mut cmap, *platform);
            put16(&mut cmap, *encoding);
            put32(&mut cmap, offset as u32);
            offset += data.len();
        }
        for (_, _, data) in tables {
            cmap.extend(data);
        }
        let mut font =
            include_bytes!("../../fixtures/fonts/NotoSans-Regular.ttf").to_vec();
        let count = u16::from_be_bytes(font[4..6].try_into().unwrap()) as usize;
        let at = (0..count)
            .map(|i| 12 + 16 * i)
            .find(|&at| &font[at..at + 4] == b"cmap")
            .unwrap();
        while !font.len().is_multiple_of(4) {
            font.push(0);
        }
        let offset = font.len() as u32;
        let length = cmap.len() as u32;
        font.extend(cmap);
        font[at + 8..at + 12].copy_from_slice(&offset.to_be_bytes());
        font[at + 12..at + 16].copy_from_slice(&length.to_be_bytes());
        font
    }
    fn check(data: &[u8], expected: impl IntoIterator<Item = u32>) {
        let face = Face::parse(data, 0).unwrap();
        let new = extract(&face);
        assert_eq!(new, legacy(&face));
        assert_eq!(new.iter().collect::<BTreeSet<_>>(), expected.into_iter().collect());
    }
    fn alias_records(mut data: Vec<u8>, aliases: &[(usize, usize)]) -> Vec<u8> {
        let count = u16::from_be_bytes(data[4..6].try_into().unwrap()) as usize;
        let record = (0..count)
            .map(|i| 12 + 16 * i)
            .find(|&at| &data[at..at + 4] == b"cmap")
            .unwrap();
        let cmap = u32::from_be_bytes(data[record + 8..record + 12].try_into().unwrap())
            as usize;
        for &(destination, origin) in aliases {
            let from = cmap + 8 + 8 * origin;
            let to = cmap + 8 + 8 * destination;
            let offset: [u8; 4] = data[from..from + 4].try_into().unwrap();
            data[to..to + 4].copy_from_slice(&offset);
        }
        data
    }
    #[test]
    fn repeated_native_payloads_are_collected_once_across_formats() {
        let data = alias_records(
            fixture(&[
                (0, 3, format4(&[(2, 5), (0xffff, 0xffff)])),
                (0, 4, grouped(12, &[(30, 32, 0)])),
                (3, 1, format4(&[(100, 105), (0xffff, 0xffff)])),
                (3, 10, grouped(12, &[(200, 205, 0)])),
                (0, 3, format4(&[(300, 305), (0xffff, 0xffff)])),
            ]),
            &[(2, 0), (3, 1), (4, 0)],
        );
        check(&data, (2..=5).chain(30..=32));
        let face = Face::parse(&data, 0).unwrap();
        assert_eq!(collect(&face).unwrap(), [2..6, 30..33]);
    }
    #[test]
    fn distinct_offsets_and_callback_payloads_are_not_memoized() {
        let four = format4(&[(2, 5), (0xffff, 0xffff)]);
        let data = fixture(&[(0, 3, four.clone()), (3, 1, four)]);
        check(&data, 2..=5);
        assert_eq!(collect(&Face::parse(&data, 0).unwrap()).unwrap(), [2..6, 2..6]);
        let mut six = vec![];
        for n in [6, 14, 0, 40, 2, 0, 1] {
            put16(&mut six, n);
        }
        let data = alias_records(fixture(&[(0, 3, six.clone()), (3, 1, six)]), &[(1, 0)]);
        check(&data, 40..42);
        assert_eq!(collect(&Face::parse(&data, 0).unwrap()).unwrap(), [40..42, 40..42]);
    }
    #[test]
    #[expect(
        clippy::single_range_in_vec_init,
        reason = "Esperados são coleções de Range semiabertos de cobertura, preservando o intervalo único 2..6"
    )]
    fn alias_memoization_respects_invalid_and_nonunicode_records() {
        let four = format4(&[(2, 5), (0xffff, 0xffff)]);
        let data = alias_records(
            fixture(&[
                (0, 3, four.clone()),
                (0, 3, vec![0x03, 0xe7]),
                (3, 1, four.clone()),
            ]),
            &[(2, 0)],
        );
        check(&data, 2..=5);
        assert_eq!(collect(&Face::parse(&data, 0).unwrap()).unwrap(), [2..6]);
        let data =
            alias_records(fixture(&[(3, 0, four.clone()), (0, 3, four)]), &[(1, 0)]);
        check(&data, 2..=5);
        assert_eq!(collect(&Face::parse(&data, 0).unwrap()).unwrap(), [2..6]);
    }
    #[test]
    fn native_memo_capacity_never_loses_later_payloads() {
        let mut tables: Vec<_> = (0..10)
            .map(|i| (0, 4, grouped(12, &[(10 * i, 10 * i + 1, 0)])))
            .collect();
        for _ in 0..3 {
            tables.push((3, 10, grouped(12, &[(200, 205, 0)])));
        }
        let data = alias_records(fixture(&tables), &[(10, 0), (11, 9), (12, 0)]);
        check(&data, (0..10).flat_map(|i| [10 * i, 10 * i + 1]));
        let mut expected: Vec<_> = (0..10).map(|i| 10 * i..10 * i + 2).collect();
        expected.push(90..92); // Tenth unique payload is processed again after the cache fills.
        assert_eq!(collect(&Face::parse(&data, 0).unwrap()).unwrap(), expected);
    }
    #[test]
    fn format4_retains_declared_ranges_zero_glyph_and_sentinel_stop() {
        let data =
            fixture(&[(0, 3, format4(&[(2, 5), (8, 6), (0xffff, 0xffff), (100, 105)]))]);
        check(&data, 2..=5);
    }
    #[test]
    fn formats12_and13_preserve_holes_overlap_and_unicode_max() {
        let data = fixture(&[
            (0, 4, grouped(12, &[(2, 5, 0), (8, 6, 0), (0x10ffff, 0x10ffff, 0)])),
            (3, 10, grouped(13, &[(4, 8, 0), (20, 21, u32::MAX)])),
        ]);
        check(&data, (2..=8).chain(20..=21).chain([0x10ffff]));
    }
    #[test]
    fn nonunicode_is_ignored_and_format6_retains_callback_fallback() {
        let mut six = vec![];
        for n in [6, 18, 0, 40, 4, 0, 1, 0, 2] {
            put16(&mut six, n);
        }
        let data = fixture(&[(3, 0, format4(&[(1, 4), (0xffff, 0xffff)])), (0, 3, six)]);
        check(&data, 40..44);
        let face = ttf_parser::Face::parse(&data, 0).unwrap();
        assert_eq!(collect(&face).unwrap(), vec![40..44]);
    }
    #[test]
    fn first_invalid_record_keeps_the_pinned_iterator_termination() {
        let data = fixture(&[
            (0, 0, vec![0x03, 0xe7]),
            (0, 3, format4(&[(1, 5), (0xffff, 0xffff)])),
        ]);
        check(&data, []);
    }
    #[test]
    fn unrepresentable_exclusive_endpoint_requires_legacy_face_fallback() {
        let data = fixture(&[(0, 4, grouped(12, &[(u32::MAX, u32::MAX, 0)]))]);
        let face = ttf_parser::Face::parse(&data, 0).unwrap();
        assert!(collect(&face).is_none());
    }
    #[test]
    fn truncated_format4_is_not_promoted_to_a_native_range() {
        let mut four = format4(&[(1, 5), (0xffff, 0xffff)]);
        four.truncate(17);
        let data = fixture(&[(0, 3, four)]);
        check(&data, []);
    }

    #[test]
    fn formats0_and10_keep_callback_membership() {
        let mut zero = Vec::new();
        for n in [0, 262, 0] {
            put16(&mut zero, n);
        }
        zero.extend([0; 256]);
        zero[6 + 65] = 1;
        let mut ten = Vec::new();
        put16(&mut ten, 10);
        put16(&mut ten, 0);
        for n in [26, 0, 1000, 3] {
            put32(&mut ten, n);
        }
        for n in [0, 1, 0] {
            put16(&mut ten, n);
        }
        let bytes = fixture(&[(0, 0, zero), (0, 4, ten)]);
        check(&bytes, [65, 1000, 1001, 1002]);
    }
    #[test]
    fn absent_cmap_and_corrupt_group_count_are_empty() {
        let bytes = fixture(&[]);
        check(&bytes, []);
        let mut group = grouped(12, &[(1, 4, 0)]);
        group[12..16].copy_from_slice(&3u32.to_be_bytes());
        check(&fixture(&[(0, 4, group)]), []);
        let mut bytes = fixture(&[(0, 3, format4(&[(1, 5), (0xffff, 0xffff)]))]);
        let count = u16::from_be_bytes(bytes[4..6].try_into().unwrap()) as usize;
        let at = (0..count)
            .map(|i| 12 + 16 * i)
            .find(|&at| &bytes[at..at + 4] == b"cmap")
            .unwrap();
        let offset =
            u32::from_be_bytes(bytes[at + 8..at + 12].try_into().unwrap()) as usize;
        bytes[offset + 8..offset + 12].copy_from_slice(&u32::MAX.to_be_bytes());
        check(&bytes, []);
        bytes[at..at + 4].copy_from_slice(b"zzzz");
        let face = Face::parse(&bytes, 0).unwrap();
        assert!(face.tables().cmap.is_none());
        assert_eq!(extract(&face), Coverage::new());
    }
    #[test]
    #[expect(
        clippy::single_range_in_vec_init,
        reason = "Fixture e esperado guardam um Range 40..42 para verificar append parcial e rollback de intervalos"
    )]
    fn checked_native_reader_rejects_truncation_after_a_partial_group() {
        let mut bytes = grouped(12, &[(1, 3, 0), (5, 7, 0)]);
        bytes.truncate(30);
        let mut ranges = vec![40..42];
        let checkpoint = ranges.len();
        assert!(native(&bytes, 12, &mut ranges).is_none());
        assert_eq!(ranges, [40..42, 1..4]);
        ranges.truncate(checkpoint);
        assert_eq!(ranges, [40..42]);
    }
}
