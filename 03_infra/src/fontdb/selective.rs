//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/fontdb/selective.md
//! @prompt-hash 720680b5
//! @layer L3
//! @updated 2026-10-07

use std::io::{Read, Seek};
use std::path::Path;
use typst_core::entities::font_book::FontInfo;

const MAX_TABLES: usize = 4096;
const MAX_BUFFER: usize = 8 * 1024 * 1024;
const MAX_READ: u64 = 16 * 1024 * 1024;

pub(super) fn acquire(path: &Path, index: u32) -> Option<FontInfo> {
    #[cfg(unix)]
    {
        let mut file = std::fs::File::open(path).ok()?;
        let before = Identity::from_metadata(&file.metadata().ok()?)?;
        let (bytes, _) = project(&mut file, before.len, index)?;
        let mut info = crate::fonts::font_info_from_bytes(&bytes, 0)?;
        let face = ttf_parser::Face::parse(&bytes, 0).ok()?;
        info.coverage = crate::fonts::extract_coverage(&face);
        let handle = Identity::from_metadata(&file.metadata().ok()?)?;
        let current_path = Identity::from_metadata(&std::fs::metadata(path).ok()?)?;
        if before != handle || before != current_path {
            return None;
        }
        Some(info)
    }
    #[cfg(not(unix))]
    {
        let _ = (path, index);
        None
    }
}

#[cfg(unix)]
#[derive(Debug, PartialEq, Eq)]
struct Identity {
    dev: u64,
    ino: u64,
    len: u64,
    mtime: i64,
    mtime_ns: i64,
    ctime: i64,
    ctime_ns: i64,
}
#[cfg(unix)]
impl Identity {
    fn from_metadata(metadata: &std::fs::Metadata) -> Option<Self> {
        use std::os::unix::fs::MetadataExt;
        if !metadata.is_file() {
            return None;
        }
        Some(Self {
            dev: metadata.dev(),
            ino: metadata.ino(),
            len: metadata.len(),
            mtime: metadata.mtime(),
            mtime_ns: metadata.mtime_nsec(),
            ctime: metadata.ctime(),
            ctime_ns: metadata.ctime_nsec(),
        })
    }
}
fn u16_at(bytes: &[u8], at: usize) -> Option<u16> {
    Some(u16::from_be_bytes(bytes.get(at..at.checked_add(2)?)?.try_into().ok()?))
}
fn u32_at(bytes: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_be_bytes(bytes.get(at..at.checked_add(4)?)?.try_into().ok()?))
}
struct Reader<'a, R> {
    source: &'a mut R,
    size: u64,
    requested: u64,
}
impl<R: Read + Seek> Reader<'_, R> {
    fn get(&mut self, offset: u64, len: usize) -> Option<Vec<u8>> {
        let length = u64::try_from(len).ok()?;
        let next = self.requested.checked_add(length)?;
        if offset.checked_add(length)? > self.size || len > MAX_BUFFER || next > MAX_READ
        {
            return None;
        }
        let mut bytes = Vec::new();
        bytes.try_reserve_exact(len).ok()?;
        bytes.resize(len, 0);
        self.requested = next;
        self.source.seek(std::io::SeekFrom::Start(offset)).ok()?;
        self.source.read_exact(&mut bytes).ok()?;
        Some(bytes)
    }
}
fn project<R: Read + Seek>(
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

#[cfg(test)]
mod tests {
    use super::*;
    fn info(b: &[u8], i: u32, c: bool) -> Option<FontInfo> {
        let mut v = crate::fonts::font_info_from_bytes(b, i)?;
        if c {
            let f = ttf_parser::Face::parse(b, i).ok()?;
            v.coverage = crate::fonts::extract_coverage(&f);
        }
        Some(v)
    }
    use std::io::Cursor;
    const FONT: &[u8] = include_bytes!("../../fixtures/fonts/NotoSans-Regular.ttf");
    fn compare(data: &[u8], index: u32, coverage: bool) {
        let expected = info(data, index, coverage);
        let projected = project(&mut Cursor::new(data), data.len() as u64, index);
        let actual = projected
            .and_then(|(b, _)| info(&b, 0, coverage))
            .or_else(|| info(data, index, coverage));
        assert_eq!(actual, expected);
    }
    fn replace_table(mut data: Vec<u8>, tag: &[u8; 4], body: &[u8]) -> Vec<u8> {
        let n = u16::from_be_bytes(data[4..6].try_into().unwrap()) as usize;
        let at = (0..n).map(|i| 12 + i * 16).find(|&i| &data[i..i + 4] == tag).unwrap();
        let offset = data.len() as u32;
        data[at + 8..at + 12].copy_from_slice(&offset.to_be_bytes());
        data[at + 12..at + 16].copy_from_slice(&(body.len() as u32).to_be_bytes());
        data.extend_from_slice(body);
        data
    }
    #[test]
    fn fixtures_preserve_metadata_and_coverage() {
        for p in std::fs::read_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/fixtures/fonts"))
            .unwrap()
        {
            let p = p.unwrap().path();
            if matches!(p.extension().and_then(|x| x.to_str()), Some("ttf" | "otf")) {
                let data = std::fs::read(p).unwrap();
                compare(&data, 0, false);
                compare(&data, 0, true);
            }
        }
    }
    #[test]
    fn collection_offsets_and_face_indices() {
        let mut out = b"ttcf\0\x01\0\0\0\0\0\x02\0\0\0\x14\0\0\0\x14".to_vec();
        let mut face = FONT.to_vec();
        let n = u16::from_be_bytes(face[4..6].try_into().unwrap()) as usize;
        for i in 0..n {
            let at = 12 + i * 16 + 8;
            let o = u32::from_be_bytes(face[at..at + 4].try_into().unwrap()) + 20;
            face[at..at + 4].copy_from_slice(&o.to_be_bytes());
        }
        out.extend_from_slice(&face);
        for index in 0..=2 {
            compare(&out, index, true);
        }
        compare(FONT, 1, true);
    }
    #[test]
    fn post_prefix_retains_validation_and_fixed_pitch() {
        for version in [0u32, 0x10000, 0x20000, 0x25000, 0x30000, 0x40000] {
            for len in [0, 4, 16, 31, 32, 33, 34, 35, 38, 1000] {
                let mut p = vec![0; len];
                if len >= 4 {
                    p[..4].copy_from_slice(&version.to_be_bytes());
                }
                if len >= 16 {
                    p[12..16].copy_from_slice(&1u32.to_be_bytes());
                }
                if len >= 34 {
                    p[32..34].copy_from_slice(&2u16.to_be_bytes());
                }
                let d = replace_table(FONT.to_vec(), b"post", &p);
                compare(&d, 0, false);
            }
        }
    }
    #[test]
    fn optional_and_mandatory_table_bounds() {
        for tag in [b"post", b"head", b"name", b"OS/2"] {
            let mut d = FONT.to_vec();
            let n = u16::from_be_bytes(d[4..6].try_into().unwrap()) as usize;
            let at = (0..n).map(|i| 12 + i * 16).find(|&i| &d[i..i + 4] == tag).unwrap();
            d[at + 8..at + 12].copy_from_slice(&u32::MAX.to_be_bytes());
            compare(&d, 0, false);
        }
    }
    #[test]
    fn malformed_headers_reject_without_panic() {
        for len in [0, 1, 4, 8, 11, 12, 20, 40] {
            compare(&FONT[..len], 0, true);
        }
        let mut b = FONT.to_vec();
        b[..4].copy_from_slice(b"xxxx");
        compare(&b, 0, true);
        let b = b"ttcf\0\x01\0\0\xff\xff\xff\xff";
        assert!(project(&mut Cursor::new(b), b.len() as u64, 0).is_none());
    }
    #[test]
    fn omitted_payload_and_prefix_reduce_reads() {
        let mut p = vec![0; 1_000_000];
        p[..4].copy_from_slice(&0x30000u32.to_be_bytes());
        let d = replace_table(FONT.to_vec(), b"post", &p);
        let (b, read) = project(&mut Cursor::new(&d), d.len() as u64, 0).unwrap();
        assert!(read < d.len() as u64 / 10);
        assert_eq!(info(&b, 0, false), info(&d, 0, false));
    }

    #[test]
    fn valid_projection_has_identical_metadata_and_coverage() {
        let (b, _) = project(&mut Cursor::new(FONT), FONT.len() as u64, 0)
            .expect("valid fixture must take selective path");
        assert_eq!(info(&b, 0, true), info(FONT, 0, true));
    }

    #[test]
    fn budgets_reject_before_read_and_exact_edge_is_allowed() {
        let mut cursor = Cursor::new(vec![0; MAX_BUFFER]);
        let mut reader = Reader {
            source: &mut cursor,
            size: MAX_BUFFER as u64,
            requested: MAX_READ - MAX_BUFFER as u64,
        };
        assert_eq!(reader.get(0, MAX_BUFFER).unwrap().len(), MAX_BUFFER);
        assert_eq!(reader.requested, MAX_READ);
        assert!(reader.get(0, 1).is_none());
        let mut cursor = Cursor::new(FONT);
        let mut reader = Reader {
            source: &mut cursor,
            size: FONT.len() as u64,
            requested: 0,
        };
        assert!(reader.get(u64::MAX, 2).is_none());
        assert!(reader.get(0, MAX_BUFFER + 1).is_none());
        assert_eq!(reader.requested, 0);
        let mut bytes = FONT.to_vec();
        bytes[4..6].copy_from_slice(&((MAX_TABLES + 1) as u16).to_be_bytes());
        assert!(project(&mut Cursor::new(&bytes), bytes.len() as u64, 0).is_none());
        let bytes = replace_table(FONT.to_vec(), b"name", &vec![0; MAX_BUFFER]);
        assert!(project(&mut Cursor::new(&bytes), bytes.len() as u64, 0).is_none());
    }
    #[test]
    fn read_and_seek_failures_decline_without_panicking() {
        struct Fault {
            seek: bool,
        }
        impl Read for Fault {
            fn read(&mut self, _: &mut [u8]) -> std::io::Result<usize> {
                Err(std::io::ErrorKind::Other.into())
            }
        }
        impl Seek for Fault {
            fn seek(&mut self, _: std::io::SeekFrom) -> std::io::Result<u64> {
                if self.seek {
                    Err(std::io::ErrorKind::Other.into())
                } else {
                    Ok(0)
                }
            }
        }
        for seek in [false, true] {
            assert!(project(&mut Fault { seek }, FONT.len() as u64, 0).is_none());
        }
    }
    #[test]
    fn projected_directory_keeps_order_and_omits_unneeded_payload() {
        let (bytes, _) = project(&mut Cursor::new(FONT), FONT.len() as u64, 0).unwrap();
        let n = u16_at(FONT, 4).unwrap() as usize;
        for i in 0..n {
            let at = 12 + 16 * i;
            assert_eq!(&bytes[at..at + 8], &FONT[at..at + 8]);
            if !matches!(
                &FONT[at..at + 4],
                b"head" | b"hhea" | b"maxp" | b"name" | b"OS/2" | b"post" | b"cmap"
            ) {
                assert_eq!(&bytes[at + 8..at + 16], &[0; 8]);
            }
        }
        let mut duplicate = FONT.to_vec();
        duplicate[12..28].copy_from_slice(&FONT[28..44]);
        let (bytes, _) =
            project(&mut Cursor::new(&duplicate), duplicate.len() as u64, 0).unwrap();
        assert_eq!(&bytes[12..20], &bytes[28..36]);
    }
    #[cfg(unix)]
    #[test]
    fn file_identity_rejects_modified_replaced_and_nonregular_inputs() {
        let dir = std::env::temp_dir()
            .join(format!("selective-identity-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("font.ttf");
        std::fs::write(&path, FONT).unwrap();
        let file = std::fs::File::open(&path).unwrap();
        let initial = Identity::from_metadata(&file.metadata().unwrap()).unwrap();
        assert_eq!(acquire(&path, 0), info(FONT, 0, true));
        std::fs::write(&path, &FONT[..FONT.len() - 1]).unwrap();
        assert_ne!(initial, Identity::from_metadata(&file.metadata().unwrap()).unwrap());
        std::fs::rename(&path, dir.join("old.ttf")).unwrap();
        std::fs::write(&path, FONT).unwrap();
        assert_ne!(
            initial,
            Identity::from_metadata(&std::fs::metadata(&path).unwrap()).unwrap()
        );
        assert!(Identity::from_metadata(&std::fs::metadata(&dir).unwrap()).is_none());
        std::fs::remove_dir_all(dir).unwrap();
    }
}
