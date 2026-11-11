//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/fontdb/selective/single_reserve_tests.md
//! @prompt-hash a495cc2c
//! @layer L3
//! @updated 2026-11-10

use super::*;
use std::io::{self, Cursor, Read, Seek, SeekFrom};

fn sfnt(tables: &[([u8; 4], Vec<u8>)]) -> Vec<u8> {
    let mut b = vec![0; 12 + 16 * tables.len()];
    b[..4].copy_from_slice(b"\0\x01\0\0");
    b[4..6].copy_from_slice(&(tables.len() as u16).to_be_bytes());
    for (i, (tag, body)) in tables.iter().enumerate() {
        let at = 12 + 16 * i;
        let offset = b.len() as u32;
        b[at..at + 4].copy_from_slice(tag);
        b[at + 4..at + 8].copy_from_slice(&(0x12340000 + i as u32).to_be_bytes());
        b[at + 8..at + 12].copy_from_slice(&offset.to_be_bytes());
        b[at + 12..at + 16].copy_from_slice(&(body.len() as u32).to_be_bytes());
        b.extend_from_slice(body);
    }
    b
}
fn compare(b: &[u8], index: u32) {
    let expected = original_project(&mut Cursor::new(b), b.len() as u64, index);
    let got = project_for_test(&mut Cursor::new(b), b.len() as u64, index);
    assert_eq!(got, expected, "projected bytes and logical requested-byte count");
}
fn post(version: u32, len: usize, count: u16) -> Vec<u8> {
    let mut p = vec![0x59; len];
    if len >= 4 {
        p[..4].copy_from_slice(&version.to_be_bytes());
    }
    if len >= 34 {
        p[32..34].copy_from_slice(&count.to_be_bytes());
    }
    p
}
fn collection(a: &[u8], b: &[u8], version: u32) -> Vec<u8> {
    let base_a = 20usize;
    let base_b = base_a + a.len();
    let mut out = b"ttcf".to_vec();
    out.extend_from_slice(&version.to_be_bytes());
    out.extend_from_slice(&2u32.to_be_bytes());
    out.extend_from_slice(&(base_a as u32).to_be_bytes());
    out.extend_from_slice(&(base_b as u32).to_be_bytes());
    for (data, base) in [(a, base_a), (b, base_b)] {
        let mut face = data.to_vec();
        let n = u16::from_be_bytes(face[4..6].try_into().unwrap()) as usize;
        for i in 0..n {
            let at = 12 + 16 * i + 8;
            let offset = u32::from_be_bytes(face[at..at + 4].try_into().unwrap());
            face[at..at + 4].copy_from_slice(&(offset + base as u32).to_be_bytes());
        }
        out.extend_from_slice(&face);
    }
    out
}

#[test]
fn stable_directory_order_duplicates_empty_selected_and_omitted_records_match() {
    let mut b = sfnt(&[
        (*b"name", b"abc".to_vec()),
        (*b"zzzz", vec![99; 8]),
        (*b"cmap", vec![2; 11]),
        (*b"name", b"abc".to_vec()),
        (*b"head", vec![]),
        (*b"post", post(0x30000, 512, 0)),
    ]);
    // An omitted table's declared payload can be invalid: it remains ignored.
    b[12 + 16 + 8..12 + 16 + 16].fill(0xff);
    // Alias a selected payload, retaining both directory records and copies.
    let first_offset = b[20..24].to_vec();
    b[12 + 3 * 16 + 8..12 + 3 * 16 + 12].copy_from_slice(&first_offset);
    compare(&b, 0);
    let (out, requested) = project(&mut Cursor::new(&b), b.len() as u64, 0).unwrap();
    assert_eq!(&out[..12], &b[..12]);
    for i in 0..6 {
        let at = 12 + 16 * i;
        assert_eq!(&out[at..at + 8], &b[at..at + 8]);
    }
    assert_eq!(&out[12 + 16 + 8..12 + 16 + 16], &[0; 8]);
    assert_eq!(out.len(), 12 + 96 + 3 + 11 + 3 + 32);
    assert_eq!(requested, 12 + 96 + 3 + 11 + 3 + 34 + 32);
    let starts = [108usize, 111, 122, 125];
    assert_eq!(&out[starts[0]..starts[0] + 3], b"abc");
    assert_eq!(&out[starts[2]..starts[2] + 3], b"abc");
}
#[test]
fn post_versions_prefix_lengths_and_duplicates_preserve_requested_counts() {
    for v in [0, 0x10000, 0x20000, 0x25000, 0x30000, 0x40000] {
        for len in [0, 1, 4, 16, 31, 32, 33, 34, 35, 38, 1000] {
            for n in [0, 1, 2, u16::MAX] {
                let b = sfnt(&[
                    (*b"post", post(v, len, n)),
                    (*b"head", vec![7; 54]),
                    (*b"post", post(v, len, n)),
                ]);
                compare(&b, 0);
            }
        }
    }
}
#[test]
fn full_post_declaration_is_checked_before_any_shortened_prefix_read() {
    let mut b = sfnt(&[(*b"post", post(0x30000, 32, 0))]);
    b[24..28].copy_from_slice(&u32::MAX.to_be_bytes());
    let mut r = Logged::new(&b);
    assert!(project(&mut r, b.len() as u64, 0).is_none());
    assert!(r.reads.iter().all(|&(start, _)| start < 28));
    compare(&b, 0);
}
#[test]
fn full_post_declaration_beyond_eof_declines_even_when_shortened_prefix_and_payload_fit()
{
    // Both the 34-byte probe and the eventual shortened payload fit. The complete
    // declared table does not. Rejecting Reader.get(prefix) cannot enforce this.
    for version in [0x20000, 0x30000] {
        let mut b = sfnt(&[(*b"post", post(version, 128, 2))]);
        let declared = (b.len() + 500) as u32;
        b[24..28].copy_from_slice(&declared.to_be_bytes());
        let mut r = Logged::new(&b);
        assert!(
            project(&mut r, b.len() as u64, 0).is_none(),
            "full post declaration must be validated for version {version:x}"
        );
        assert!(
            r.reads.iter().all(|&(start, _)| start < 28),
            "a rejected declaration must not read its otherwise fitting prefix"
        );
        compare(&b, 0);
    }
}
#[test]
fn selected_bounds_zero_offsets_and_omitted_invalid_bounds_remain_distinct() {
    for tag in [*b"head", *b"hhea", *b"maxp", *b"name", *b"OS/2", *b"post", *b"cmap"] {
        for (off, len) in [(u32::MAX, 0), (0, u32::MAX), (28, 1), (28, 0)] {
            let mut b = sfnt(&[(tag, vec![])]);
            b[20..24].copy_from_slice(&off.to_be_bytes());
            b[24..28].copy_from_slice(&len.to_be_bytes());
            compare(&b, 0);
            let got = project(&mut Cursor::new(&b), b.len() as u64, 0);
            assert_eq!(got.is_some(), off == 28 && len == 0);
        }
    }
    let mut b = sfnt(&[(*b"glyf", vec![])]);
    b[20..28].fill(0xff);
    assert!(project(&mut Cursor::new(&b), b.len() as u64, 0).is_some());
    compare(&b, 0);
}
#[test]
fn ttc_absolute_offsets_selected_index_and_versions_keep_identical_projection() {
    let a = sfnt(&[(*b"name", b"first".to_vec()), (*b"post", post(0x20000, 128, 4))]);
    let b = sfnt(&[(*b"cmap", vec![7; 33]), (*b"head", vec![8; 54])]);
    for version in [0x10000, 0x20000] {
        let c = collection(&a, &b, version);
        for index in 0..=2 {
            compare(&c, index);
        }
        assert_eq!(
            project(&mut Cursor::new(&c), c.len() as u64, 0).unwrap().0,
            project(&mut Cursor::new(&a), a.len() as u64, 0).unwrap().0
        );
        assert_eq!(
            project(&mut Cursor::new(&c), c.len() as u64, 1).unwrap().0,
            project(&mut Cursor::new(&b), b.len() as u64, 0).unwrap().0
        );
    }
    let mut c = collection(&a, &b, 0x10000);
    c[12..16].copy_from_slice(&0u32.to_be_bytes());
    compare(&c, 0);
    assert!(project(&mut Cursor::new(&c), c.len() as u64, 0).is_none());
    c[8..12].copy_from_slice(&u32::MAX.to_be_bytes());
    compare(&c, 0);
}
#[test]
fn header_signature_count_cap_and_empty_projection_are_preserved() {
    for sig in [*b"\0\x01\0\0", *b"OTTO", *b"true", *b"xxxx"] {
        let mut b = sfnt(&[]);
        b[..4].copy_from_slice(&sig);
        compare(&b, 0);
        assert_eq!(
            project(&mut Cursor::new(&b), b.len() as u64, 0).is_some(),
            sig != *b"xxxx"
        );
    }
    let accepted = sfnt(&vec![(*b"glyf", vec![]); MAX_TABLES]);
    compare(&accepted, 0);
    assert!(project(&mut Cursor::new(&accepted), accepted.len() as u64, 0).is_some());
    let declined = sfnt(&vec![(*b"glyf", vec![]); MAX_TABLES + 1]);
    compare(&declined, 0);
    assert!(project(&mut Cursor::new(&declined), declined.len() as u64, 0).is_none());
    for len in 0..12 {
        let b = vec![0; len];
        compare(&b, 0);
    }
    let b = sfnt(&[]);
    compare(&b, 1);
    assert!(project(&mut Cursor::new(&b), b.len() as u64, 1).is_none());
}
#[test]
fn exact_complete_buffer_limit_is_allowed_and_overedge_declines_before_large_request() {
    let b = sfnt(&[(*b"cmap", vec![0x63; MAX_BUFFER - 28])]);
    compare(&b, 0);
    let (out, _) = project(&mut Cursor::new(&b), b.len() as u64, 0).unwrap();
    assert_eq!(out.len(), MAX_BUFFER);
    assert_eq!(out[28], 0x63);
    assert_eq!(out[MAX_BUFFER - 1], 0x63);
    drop(out);
    let b = sfnt(&[(*b"cmap", vec![0x63; MAX_BUFFER - 27])]);
    let mut reader = Logged::new(&b);
    let out = project(&mut reader, b.len() as u64, 0);
    assert!(out.is_none());
    assert!(
        reader.reads.iter().all(|&(offset, _)| offset < 28),
        "reject oversized total before payload I/O"
    );
    compare(&b, 0);
}
#[test]
fn reader_requested_budget_exact_edge_overflow_and_rejection_before_io() {
    let b = [0u8; 2];
    let mut cursor = Logged::new(&b);
    let mut r = Reader {
        source: &mut cursor,
        size: 2,
        requested: MAX_READ - 1,
    };
    assert_eq!(r.get(0, 1), Some(vec![0]));
    assert_eq!(r.requested, MAX_READ);
    assert!(r.get(1, 1).is_none());
    assert_eq!(r.requested, MAX_READ);
    assert_eq!(cursor.reads.len(), 1);
    let mut cursor = Logged::new(&b);
    let mut r = Reader { source: &mut cursor, size: 2, requested: u64::MAX };
    assert!(r.get(0, 1).is_none());
    assert!(r.get(u64::MAX, 1).is_none());
    assert!(cursor.reads.is_empty());
    assert!(cursor.seeks.is_empty());
}
struct Logged<'a> {
    inner: Cursor<&'a [u8]>,
    reads: Vec<(u64, usize)>,
    seeks: Vec<SeekFrom>,
    max_chunk: usize,
    interrupts: usize,
    fail_at: Option<u64>,
    bad_seek: bool,
}
impl<'a> Logged<'a> {
    fn new(b: &'a [u8]) -> Self {
        Self {
            inner: Cursor::new(b),
            reads: vec![],
            seeks: vec![],
            max_chunk: usize::MAX,
            interrupts: 0,
            fail_at: None,
            bad_seek: false,
        }
    }
}
impl Read for Logged<'_> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let pos = self.inner.position();
        self.reads.push((pos, buf.len()));
        if self.interrupts > 0 {
            self.interrupts -= 1;
            return Err(io::ErrorKind::Interrupted.into());
        }
        if self.fail_at.is_some_and(|p| pos <= p && p < pos + buf.len() as u64) {
            return Err(io::ErrorKind::Other.into());
        }
        let n = buf.len().min(self.max_chunk);
        self.inner.read(&mut buf[..n])
    }
}
impl Seek for Logged<'_> {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.seeks.push(pos);
        if self.bad_seek {
            return Err(io::ErrorKind::Other.into());
        }
        self.inner.seek(pos)
    }
}
#[test]
fn short_reads_interrupted_and_seek_read_eof_errors_remain_conservative() {
    let b = sfnt(&[
        (*b"name", vec![9; 100]),
        (*b"post", post(0x20000, 300, 9)),
        (*b"cmap", vec![7; 60]),
    ]);
    let expected = original_project(&mut Cursor::new(&b), b.len() as u64, 0).unwrap();
    let mut short = Logged::new(&b);
    short.max_chunk = 3;
    short.interrupts = 4;
    assert_eq!(project(&mut short, b.len() as u64, 0), Some(expected));
    for offset in [0, 12, 60, 160, 200, 460, 519] {
        let mut r = Logged::new(&b);
        r.fail_at = Some(offset);
        assert!(project(&mut r, b.len() as u64, 0).is_none(), "read fault at {offset}");
    }
    let mut r = Logged::new(&b);
    r.bad_seek = true;
    assert!(project(&mut r, b.len() as u64, 0).is_none());
    let truncated = &b[..b.len() - 1];
    assert!(project(&mut Cursor::new(truncated), b.len() as u64, 0).is_none());
}

#[test]
fn actual_font_fixtures_preserve_projection_metadata_and_exact_coverage() {
    let fontdir =
        std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/fixtures/fonts"));
    for name in [
        "DejaVuSans.ttf",
        "NotoSans-Regular.ttf",
        "NimbusSans-Regular.otf",
        "UbuntuSans-Variable.ttf",
        "Cantarell-VF.otf",
        "NewCMMath-Book.otf",
    ] {
        let b = std::fs::read(fontdir.join(name)).unwrap();
        compare(&b, 0);
        let (out, _) = project(&mut Cursor::new(&b), b.len() as u64, 0).unwrap();
        let mut expected = crate::fonts::font_info_from_bytes(&b, 0).unwrap();
        let mut actual = crate::fonts::font_info_from_bytes(&out, 0).unwrap();
        expected.coverage =
            crate::fonts::extract_coverage(&ttf_parser::Face::parse(&b, 0).unwrap());
        actual.coverage =
            crate::fonts::extract_coverage(&ttf_parser::Face::parse(&out, 0).unwrap());
        assert_eq!(actual, expected, "{name}");
    }
}

#[test]
fn missing_cmap_is_some_metadata_with_empty_coverage_not_a_decline() {
    let mut b = std::fs::read(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/fixtures/fonts/NotoSans-Regular.ttf"
    ))
    .unwrap();
    let count = u16_at(&b, 4).unwrap() as usize;
    let at = (0..count)
        .map(|i| 12 + 16 * i)
        .find(|&at| &b[at..at + 4] == b"cmap")
        .unwrap();
    b[at..at + 4].copy_from_slice(b"zzzz");
    compare(&b, 0);
    let (out, _) =
        project(&mut Cursor::new(&b), b.len() as u64, 0).expect("no cmap still projects");
    let info = crate::fonts::font_info_from_bytes(&out, 0)
        .expect("metadata survives missing cmap");
    assert!(!info.family.is_empty());
    assert!(crate::fonts::extract_coverage(&ttf_parser::Face::parse(&out, 0).unwrap())
        .is_empty());
    assert!(project(&mut Cursor::new(b"not a font"), 10, 0).is_none());
}
#[test]
fn declined_selective_post_keeps_parent_fallback_face_and_exact_coverage() {
    let mut b = std::fs::read(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/fixtures/fonts/NotoSans-Regular.ttf"
    ))
    .unwrap();
    let count = u16_at(&b, 4).unwrap() as usize;
    let at = (0..count)
        .map(|i| 12 + 16 * i)
        .find(|&at| &b[at..at + 4] == b"post")
        .unwrap();
    b[at + 8..at + 12].copy_from_slice(&u32::MAX.to_be_bytes());
    let temp = TempDir::new();
    let dir = &temp.0;
    let path = dir.join("declined.ttf");
    std::fs::write(&path, &b).unwrap();
    assert!(acquire(&path, 0).is_none());
    let (mut slots, book) = crate::fontdb::load_fonts_from_dir(dir);
    assert_eq!(slots.len(), 1);
    assert_eq!(book.len(), 1);
    assert_eq!(slots.remove(0).path, path);
    let mut expected = crate::fonts::font_info_from_bytes(&b, 0).unwrap();
    expected.coverage =
        crate::fonts::extract_coverage(&ttf_parser::Face::parse(&b, 0).unwrap());
    assert!(expected.coverage.contains('A' as u32));
    assert_eq!(book.infos(), &[expected]);
}

// Frozen received projector, used only as an extra stable-input oracle.
fn original_project<R: Read + Seek>(
    reader: &mut R,
    size: u64,
    index: u32,
) -> Option<(Vec<u8>, u64)> {
    let mut input = Reader { source: reader, size, requested: 0 };
    let first = input.get(0, 12)?;
    let offset = if first.get(..4)? == b"ttcf" {
        if !matches!(u32_at(&first, 4)?, 0x00010000 | 0x00020000) {
            return None;
        }
        let count = u32_at(&first, 8)?;
        let array_end = 12u64.checked_add(u64::from(count).checked_mul(4)?)?;
        if index >= count || array_end > size {
            return None;
        }
        let selected =
            input.get(12u64.checked_add(u64::from(index).checked_mul(4)?)?, 4)?;
        let offset = u64::from(u32_at(&selected, 0)?);
        if offset < array_end {
            return None;
        }
        offset
    } else {
        if index != 0 {
            return None;
        }
        0
    };
    let header = if offset == 0 { first } else { input.get(offset, 12)? };
    if !matches!(header.get(..4)?, b"\0\x01\0\0" | b"OTTO" | b"true") {
        return None;
    }
    let count = usize::from(u16_at(&header, 4)?);
    if count > MAX_TABLES {
        return None;
    }
    let directory = input.get(offset.checked_add(12)?, count.checked_mul(16)?)?;
    let mut out = header;
    out.try_reserve_exact(directory.len()).ok()?;
    out.extend_from_slice(&directory);
    for i in 0..count {
        let entry = i.checked_mul(16)?;
        let at = 12usize.checked_add(entry)?;
        let tag = directory.get(entry..entry.checked_add(4)?)?;
        let original_len =
            usize::try_from(u32_at(&directory, entry.checked_add(12)?)?).ok()?;
        let original_offset = u64::from(u32_at(&directory, entry.checked_add(8)?)?);
        if !matches!(
            tag,
            b"head" | b"hhea" | b"maxp" | b"name" | b"OS/2" | b"post" | b"cmap"
        ) {
            out.get_mut(at.checked_add(8)?..at.checked_add(16)?)?.fill(0);
            continue;
        }
        // Validate the full declaration before using any shortened post prefix.
        if original_offset.checked_add(u64::try_from(original_len).ok()?)? > size {
            return None;
        }
        let mut take = original_len;
        if tag == b"post" {
            let prefix = input.get(original_offset, take.min(34))?;
            take = if u32_at(&prefix, 0) == Some(0x00020000) {
                u16_at(&prefix, 32)
                    .map(|n| 34 + usize::from(n) * 2)
                    .unwrap_or(take)
                    .min(take)
            } else {
                take.min(32)
            };
        }
        let total = out.len().checked_add(take)?;
        if total > MAX_BUFFER {
            return None;
        }
        let bytes = input.get(original_offset, take)?;
        out.try_reserve_exact(take).ok()?;
        let new_offset = u32::try_from(out.len()).ok()?;
        let new_len = u32::try_from(bytes.len()).ok()?;
        out.get_mut(at.checked_add(8)?..at.checked_add(12)?)?
            .copy_from_slice(&new_offset.to_be_bytes());
        out.get_mut(at.checked_add(12)?..at.checked_add(16)?)?
            .copy_from_slice(&new_len.to_be_bytes());
        out.extend_from_slice(&bytes);
    }
    Some((out, input.requested))
}

struct TempDir(std::path::PathBuf);
impl TempDir {
    fn new() -> Self {
        for n in 0.. {
            let path = std::env::temp_dir().join(format!(
                "typst-projected-fallback-{}-{:?}-{n}",
                std::process::id(),
                std::thread::current().id()
            ));
            match std::fs::create_dir(&path) {
                Ok(()) => return Self(path),
                Err(e) if e.kind() == io::ErrorKind::AlreadyExists => continue,
                Err(e) => panic!("cannot create font fixture directory: {e}"),
            }
        }
        unreachable!()
    }
}
impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}
