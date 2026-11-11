//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/fontdb/selective.md
//! @prompt-hash 914757a8
//! @layer L3
//! @updated 2026-11-10

use std::io::{Read, Seek};
use std::path::Path;
use typst_core::entities::font_book::FontInfo;

#[cfg(unix)]
mod positional_reader {
    use std::io::{self, Read, Seek, SeekFrom};

    pub(super) trait Backend {
        fn read_at(&self, buf: &mut [u8], offset: u64) -> io::Result<usize>;
    }

    impl Backend for std::fs::File {
        fn read_at(&self, buf: &mut [u8], offset: u64) -> io::Result<usize> {
            std::os::unix::fs::FileExt::read_at(self, buf, offset)
        }
    }

    pub(super) struct PositionalReader<'a, R: ?Sized + Backend> {
        source: &'a R,
        cursor: u64,
        size: u64,
    }

    impl<'a, R: ?Sized + Backend> PositionalReader<'a, R> {
        pub(super) fn new(source: &'a R, size: u64) -> Self {
            Self { source, cursor: 0, size }
        }
    }

    impl<R: ?Sized + Backend> Read for PositionalReader<'_, R> {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            if buf.is_empty() {
                return Ok(0);
            }
            let n = self.source.read_at(buf, self.cursor)?;
            if n > buf.len() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "backend returned excessive length",
                ));
            }
            let next = self.cursor.checked_add(n as u64).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "cursor overflow")
            })?;
            self.cursor = next;
            Ok(n)
        }
    }

    impl<R: ?Sized + Backend> Seek for PositionalReader<'_, R> {
        fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
            let next = match pos {
                SeekFrom::Start(n) => Some(n),
                SeekFrom::Current(n) => {
                    u64::try_from(i128::from(self.cursor) + i128::from(n)).ok()
                }
                SeekFrom::End(n) => {
                    u64::try_from(i128::from(self.size) + i128::from(n)).ok()
                }
            }
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "invalid seek"))?;
            self.cursor = next;
            Ok(next)
        }
    }
}

#[cfg(unix)]
use positional_reader::PositionalReader;

const MAX_TABLES: usize = 4096;
const MAX_BUFFER: usize = 8 * 1024 * 1024;
const MAX_READ: u64 = 16 * 1024 * 1024;

pub(super) fn acquire(path: &Path, index: u32) -> Option<FontInfo> {
    #[cfg(unix)]
    {
        let file = std::fs::File::open(path).ok()?;
        let before = Identity::from_metadata(&file.metadata().ok()?)?;
        let mut positional = PositionalReader::new(&file, before.len);
        let (bytes, _) = project(&mut positional, before.len, index)?;
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
    let mut directory = input.get(offset.checked_add(12)?, count.checked_mul(16)?)?;
    let mut total = header.len().checked_add(directory.len())?;
    if total > MAX_BUFFER {
        return None;
    }
    let mut plan = Vec::new();
    plan.try_reserve_exact(count).ok()?;
    // Prefixes are read while planning, before payload emission. Charge the
    // original logical prefix/payload sequence as well as Reader's actual reads.
    let mut logical_requested = input.requested;
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
            directory
                .get_mut(entry.checked_add(8)?..entry.checked_add(16)?)?
                .fill(0);
            continue;
        }
        // Validate the full declaration before using any shortened post prefix.
        if original_offset.checked_add(u64::try_from(original_len).ok()?)? > size {
            return None;
        }
        let mut take = original_len;
        if tag == b"post" {
            let prefix_len = take.min(34);
            logical_requested =
                logical_requested.checked_add(u64::try_from(prefix_len).ok()?)?;
            if logical_requested > MAX_READ {
                return None;
            }
            let prefix = input.get(original_offset, prefix_len)?;
            take = if u32_at(&prefix, 0) == Some(0x00020000) {
                u16_at(&prefix, 32)
                    .map(|n| 34 + usize::from(n) * 2)
                    .unwrap_or(take)
                    .min(take)
            } else {
                take.min(32)
            };
        }
        total = total.checked_add(take)?;
        if total > MAX_BUFFER {
            return None;
        }
        logical_requested = logical_requested.checked_add(u64::try_from(take).ok()?)?;
        if logical_requested > MAX_READ {
            return None;
        }
        plan.push((at, original_offset, take));
    }

    let additional = total.checked_sub(header.len())?;
    let mut out = header;
    out.try_reserve_exact(additional).ok()?;
    out.extend_from_slice(&directory);
    for (at, original_offset, take) in plan {
        let bytes = input.get(original_offset, take)?;
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
pub(super) fn project_for_test<R: Read + Seek>(
    reader: &mut R,
    size: u64,
    index: u32,
) -> Option<(Vec<u8>, u64)> {
    project(reader, size, index)
}

#[cfg(test)]
#[path = "selective/single_reserve_tests.rs"]
mod single_reserve_tests;

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

#[cfg(all(test, unix))]
mod positional_transport_tests {
    use super::positional_reader::{Backend, PositionalReader};
    use super::project;
    use std::cell::RefCell;
    use std::collections::VecDeque;
    use std::fs;
    use std::io::{self, Read, Seek, SeekFrom};
    use std::path::PathBuf;

    #[derive(Clone, Copy)]
    enum Action {
        Chunk(usize),
        Error(io::ErrorKind),
        Eof,
        Excessive,
    }

    struct Script {
        data: Vec<u8>,
        actions: RefCell<VecDeque<Action>>,
        requests: RefCell<Vec<(u64, usize)>>,
    }

    impl Script {
        fn new(data: &[u8], actions: &[Action]) -> Self {
            Self {
                data: data.to_vec(),
                actions: RefCell::new(actions.iter().copied().collect()),
                requests: RefCell::new(Vec::new()),
            }
        }
    }

    impl Backend for Script {
        fn read_at(&self, buf: &mut [u8], offset: u64) -> io::Result<usize> {
            self.requests.borrow_mut().push((offset, buf.len()));
            match self.actions.borrow_mut().pop_front() {
                Some(Action::Error(kind)) => Err(io::Error::new(kind, "injected error")),
                Some(Action::Eof) => Ok(0),
                Some(Action::Excessive) => Ok(buf.len() + 1),
                action => {
                    let start = usize::try_from(offset).unwrap_or(usize::MAX);
                    let remaining = self.data.len().saturating_sub(start);
                    let limit = match action {
                        Some(Action::Chunk(n)) => n,
                        _ => usize::MAX,
                    };
                    let n = remaining.min(buf.len()).min(limit);
                    if n > 0 {
                        buf[..n].copy_from_slice(&self.data[start..start + n]);
                    }
                    Ok(n)
                }
            }
        }
    }

    struct TempDir(PathBuf);
    impl TempDir {
        fn new() -> Self {
            let mut next = 0usize;
            loop {
                let path = std::env::temp_dir().join(format!(
                    "typst-positional-{}-{:?}-{next}",
                    std::process::id(),
                    std::thread::current().id()
                ));
                match fs::create_dir(&path) {
                    Ok(()) => return Self(path),
                    Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {
                        next = next.checked_add(1).expect("fixture namespace exhausted");
                    }
                    Err(e) => panic!("cannot create fixture: {e}"),
                }
            }
        }
    }
    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn starts_at_zero_and_advances_only_by_actual_length() {
        let backend = Script::new(b"abcdef", &[Action::Chunk(1), Action::Chunk(2)]);
        let mut reader = PositionalReader::new(&backend, 6);
        let mut out = [0; 4];
        assert_eq!(reader.read(&mut out).unwrap(), 1);
        assert_eq!(out[0], b'a');
        assert_eq!(reader.read(&mut out).unwrap(), 2);
        assert_eq!(&out[..2], b"bc");
        assert_eq!(reader.stream_position().unwrap(), 3);
        assert_eq!(*backend.requests.borrow(), vec![(0, 4), (1, 4)]);
    }

    #[test]
    fn read_exact_retries_short_reads_and_interrupted_at_same_offset() {
        let backend = Script::new(
            b"abcdef",
            &[
                Action::Error(io::ErrorKind::Interrupted),
                Action::Chunk(1),
                Action::Chunk(2),
                Action::Chunk(3),
            ],
        );
        let mut reader = PositionalReader::new(&backend, 6);
        let mut out = [0; 6];
        reader.read_exact(&mut out).unwrap();
        assert_eq!(&out, b"abcdef");
        assert_eq!(reader.stream_position().unwrap(), 6);
        assert_eq!(*backend.requests.borrow(), vec![(0, 6), (0, 6), (1, 5), (3, 3)]);
    }

    #[test]
    fn repeated_interrupted_is_retried_by_read_exact() {
        let backend = Script::new(
            b"x",
            &[
                Action::Error(io::ErrorKind::Interrupted),
                Action::Error(io::ErrorKind::Interrupted),
                Action::Chunk(1),
            ],
        );
        let mut reader = PositionalReader::new(&backend, 1);
        let mut out = [0];
        reader.read_exact(&mut out).unwrap();
        assert_eq!(&out, b"x");
        assert_eq!(*backend.requests.borrow(), vec![(0, 1), (0, 1), (0, 1)]);
    }

    #[test]
    fn interrupted_error_does_not_advance_cursor() {
        let backend =
            Script::new(b"abcdef", &[Action::Error(io::ErrorKind::Interrupted)]);
        let mut reader = PositionalReader::new(&backend, 6);
        reader.seek(SeekFrom::Start(2)).unwrap();
        let mut out = [0; 2];
        assert_eq!(reader.read(&mut out).unwrap_err().kind(), io::ErrorKind::Interrupted);
        assert_eq!(reader.stream_position().unwrap(), 2);
        reader.read_exact(&mut out).unwrap();
        assert_eq!(&out, b"cd");
        assert_eq!(*backend.requests.borrow(), vec![(2, 2), (2, 2)]);
    }

    #[test]
    fn denied_error_preserves_position_after_successful_read() {
        let backend = Script::new(
            b"abcdef",
            &[
                Action::Chunk(2),
                Action::Error(io::ErrorKind::PermissionDenied),
                Action::Chunk(1),
            ],
        );
        let mut reader = PositionalReader::new(&backend, 6);
        let mut out = [0; 2];
        reader.read_exact(&mut out).unwrap();
        assert_eq!(
            reader.read(&mut out).unwrap_err().kind(),
            io::ErrorKind::PermissionDenied
        );
        assert_eq!(reader.stream_position().unwrap(), 2);
        assert_eq!(reader.read(&mut out).unwrap(), 1);
        assert_eq!(out[0], b'c');
        assert_eq!(*backend.requests.borrow(), vec![(0, 2), (2, 2), (2, 2)]);
    }

    #[test]
    fn read_exact_preserves_other_error_without_retry() {
        let backend = Script::new(
            b"abcdef",
            &[Action::Chunk(2), Action::Error(io::ErrorKind::Other), Action::Chunk(4)],
        );
        let mut reader = PositionalReader::new(&backend, 6);
        let mut out = [0; 6];
        assert_eq!(reader.read_exact(&mut out).unwrap_err().kind(), io::ErrorKind::Other);
        assert_eq!(reader.stream_position().unwrap(), 2);
        assert_eq!(backend.requests.borrow().len(), 2);
    }

    #[test]
    fn early_eof_is_not_fabricated_success() {
        let backend = Script::new(b"abcdef", &[Action::Chunk(2), Action::Eof]);
        let mut reader = PositionalReader::new(&backend, 6);
        let mut out = [0x55; 6];
        assert_eq!(
            reader.read_exact(&mut out).unwrap_err().kind(),
            io::ErrorKind::UnexpectedEof
        );
        assert_eq!(&out[..2], b"ab");
        assert_eq!(&out[2..], &[0x55; 4]);
        assert_eq!(reader.stream_position().unwrap(), 2);
    }

    #[test]
    fn empty_reads_never_call_backend() {
        let backend = Script::new(b"abcdef", &[Action::Error(io::ErrorKind::Other)]);
        let mut reader = PositionalReader::new(&backend, 6);
        reader.seek(SeekFrom::Start(3)).unwrap();
        assert_eq!(reader.read(&mut []).unwrap(), 0);
        reader.read_exact(&mut []).unwrap();
        assert_eq!(reader.stream_position().unwrap(), 3);
        assert!(backend.requests.borrow().is_empty());
    }

    #[test]
    fn excessive_backend_length_is_rejected_without_cursor_advance() {
        let backend = Script::new(b"abcdef", &[Action::Excessive]);
        let mut reader = PositionalReader::new(&backend, 6);
        reader.seek(SeekFrom::Start(2)).unwrap();
        assert!(reader.read(&mut [0; 2]).is_err());
        assert_eq!(reader.stream_position().unwrap(), 2);
        let mut out = [0; 2];
        reader.read_exact(&mut out).unwrap();
        assert_eq!(&out, b"cd");
    }

    #[test]
    fn excessive_backend_length_is_not_accepted_by_read_exact() {
        let backend = Script::new(b"abcdef", &[Action::Excessive]);
        let mut reader = PositionalReader::new(&backend, 6);
        assert!(reader.read_exact(&mut [0; 2]).is_err());
        assert_eq!(reader.stream_position().unwrap(), 0);
    }

    #[test]
    fn eof_after_complete_data_preserves_end_position() {
        let backend = Script::new(b"ab", &[]);
        let mut reader = PositionalReader::new(&backend, 2);
        let mut out = [0; 2];
        reader.read_exact(&mut out).unwrap();
        assert_eq!(reader.read(&mut out).unwrap(), 0);
        assert_eq!(reader.stream_position().unwrap(), 2);
    }

    #[test]
    fn seek_start_changes_only_logical_position() {
        let backend = Script::new(b"abcdef", &[]);
        let mut reader = PositionalReader::new(&backend, 6);
        assert_eq!(reader.seek(SeekFrom::Start(4)).unwrap(), 4);
        assert!(backend.requests.borrow().is_empty());
        let mut out = [0; 2];
        reader.read_exact(&mut out).unwrap();
        assert_eq!(&out, b"ef");
        assert_eq!(backend.requests.borrow()[0].0, 4);
    }

    #[test]
    fn seek_current_can_move_backwards() {
        let backend = Script::new(b"abcdef", &[]);
        let mut reader = PositionalReader::new(&backend, 6);
        reader.seek(SeekFrom::Start(4)).unwrap();
        assert_eq!(reader.seek(SeekFrom::Current(-3)).unwrap(), 1);
        let mut out = [0; 2];
        reader.read_exact(&mut out).unwrap();
        assert_eq!(&out, b"bc");
    }

    #[test]
    fn negative_current_seek_preserves_previous_position() {
        let backend = Script::new(b"abcdef", &[]);
        let mut reader = PositionalReader::new(&backend, 6);
        reader.seek(SeekFrom::Start(2)).unwrap();
        assert_eq!(
            reader.seek(SeekFrom::Current(-3)).unwrap_err().kind(),
            io::ErrorKind::InvalidInput
        );
        assert_eq!(reader.stream_position().unwrap(), 2);
    }

    #[test]
    fn overflowing_current_seek_preserves_previous_position() {
        let backend = Script::new(b"", &[]);
        let mut reader = PositionalReader::new(&backend, 0);
        reader.seek(SeekFrom::Start(u64::MAX)).unwrap();
        assert_eq!(
            reader.seek(SeekFrom::Current(1)).unwrap_err().kind(),
            io::ErrorKind::InvalidInput
        );
        assert_eq!(reader.stream_position().unwrap(), u64::MAX);
    }

    #[test]
    fn seek_end_uses_captured_size() {
        let backend = Script::new(b"abcdef", &[]);
        let mut reader = PositionalReader::new(&backend, 6);
        assert_eq!(reader.seek(SeekFrom::End(-2)).unwrap(), 4);
        let mut out = [0; 2];
        reader.read_exact(&mut out).unwrap();
        assert_eq!(&out, b"ef");
    }

    #[test]
    fn negative_end_seek_preserves_position() {
        let backend = Script::new(b"abc", &[]);
        let mut reader = PositionalReader::new(&backend, 3);
        reader.seek(SeekFrom::Start(1)).unwrap();
        assert_eq!(
            reader.seek(SeekFrom::End(-4)).unwrap_err().kind(),
            io::ErrorKind::InvalidInput
        );
        assert_eq!(reader.stream_position().unwrap(), 1);
    }

    #[test]
    fn end_seek_overflow_is_rejected() {
        let backend = Script::new(b"", &[]);
        let mut reader = PositionalReader::new(&backend, u64::MAX);
        assert_eq!(
            reader.seek(SeekFrom::End(1)).unwrap_err().kind(),
            io::ErrorKind::InvalidInput
        );
        assert_eq!(reader.stream_position().unwrap(), 0);
    }

    #[test]
    fn seek_beyond_end_is_allowed_but_read_returns_eof() {
        let backend = Script::new(b"abc", &[]);
        let mut reader = PositionalReader::new(&backend, 3);
        assert_eq!(reader.seek(SeekFrom::Start(10)).unwrap(), 10);
        assert_eq!(reader.read(&mut [0; 2]).unwrap(), 0);
        assert_eq!(reader.stream_position().unwrap(), 10);
    }

    #[test]
    fn read_cursor_cannot_wrap() {
        struct AnyByte;
        impl Backend for AnyByte {
            fn read_at(&self, buf: &mut [u8], _offset: u64) -> io::Result<usize> {
                if buf.is_empty() {
                    return Ok(0);
                }
                buf[0] = b'x';
                Ok(1)
            }
        }
        let backend = AnyByte;
        let mut reader = PositionalReader::new(&backend, u64::MAX);
        reader.seek(SeekFrom::Start(u64::MAX)).unwrap();
        assert!(reader.read(&mut [0]).is_err());
        assert_eq!(reader.stream_position().unwrap(), u64::MAX);
    }

    #[test]
    fn independent_readers_do_not_share_logical_position() {
        let backend = Script::new(b"abcdef", &[]);
        let mut a = PositionalReader::new(&backend, 6);
        let mut b = PositionalReader::new(&backend, 6);
        let mut out = [0; 2];
        a.read_exact(&mut out).unwrap();
        assert_eq!(&out, b"ab");
        a.read_exact(&mut out).unwrap();
        assert_eq!(&out, b"cd");
        b.read_exact(&mut out).unwrap();
        assert_eq!(&out, b"ab");
        assert_eq!(*backend.requests.borrow(), vec![(0, 2), (2, 2), (0, 2)]);
    }

    #[test]
    fn positional_reads_preserve_physical_file_cursor() {
        let dir = TempDir::new();
        let path = dir.0.join("cursor.bin");
        fs::write(&path, b"abcdef").unwrap();
        let mut file = fs::File::open(&path).unwrap();
        file.seek(SeekFrom::Start(3)).unwrap();
        let mut reader = PositionalReader::new(&file, 6);
        let mut out = [0; 2];
        reader.read_exact(&mut out).unwrap();
        assert_eq!(&out, b"ab");
        assert_eq!(file.stream_position().unwrap(), 3);
    }

    fn sfnt_fixture() -> Vec<u8> {
        let mut data = vec![0; 32];
        data[0..4].copy_from_slice(&[0, 1, 0, 0]);
        data[4..6].copy_from_slice(&1u16.to_be_bytes());
        data[12..16].copy_from_slice(b"head");
        data[20..24].copy_from_slice(&28u32.to_be_bytes());
        data[24..28].copy_from_slice(&4u32.to_be_bytes());
        data[28..32].copy_from_slice(b"DATA");
        data
    }

    #[test]
    fn projector_accepts_correct_bytes_with_short_reads_and_interrupted() {
        let data = sfnt_fixture();
        let backend = Script::new(
            &data,
            &[
                Action::Chunk(3),
                Action::Error(io::ErrorKind::Interrupted),
                Action::Chunk(1),
                Action::Chunk(8),
            ],
        );
        let mut reader = PositionalReader::new(&backend, data.len() as u64);
        let (projected, requested) = project(&mut reader, data.len() as u64, 0).unwrap();
        assert_eq!(projected, data);
        assert_eq!(requested, 32);
        assert_eq!(&backend.requests.borrow()[..4], &[(0, 12), (3, 9), (3, 9), (4, 8)]);
    }

    #[test]
    fn projector_rejects_noninterrupted_error_in_directory() {
        let data = sfnt_fixture();
        let backend = Script::new(
            &data,
            &[Action::Chunk(12), Action::Error(io::ErrorKind::PermissionDenied)],
        );
        let mut reader = PositionalReader::new(&backend, data.len() as u64);
        assert!(project(&mut reader, data.len() as u64, 0).is_none());
        assert_eq!(backend.requests.borrow().len(), 2);
    }

    #[test]
    fn projector_rejects_partial_table_followed_by_eof() {
        let data = sfnt_fixture();
        let backend = Script::new(
            &data,
            &[Action::Chunk(12), Action::Chunk(16), Action::Chunk(2), Action::Eof],
        );
        let mut reader = PositionalReader::new(&backend, data.len() as u64);
        assert!(project(&mut reader, data.len() as u64, 0).is_none());
        assert_eq!(reader.stream_position().unwrap(), 30);
    }

    #[test]
    fn projector_rejects_excessive_backend_return() {
        let data = sfnt_fixture();
        let backend = Script::new(&data, &[Action::Excessive]);
        let mut reader = PositionalReader::new(&backend, data.len() as u64);
        assert!(project(&mut reader, data.len() as u64, 0).is_none());
        assert_eq!(reader.stream_position().unwrap(), 0);
    }

    #[test]
    fn real_font_projection_matches_generic_file_reader() {
        let data = include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/fixtures/fonts/NotoSans-Regular.ttf"
        ));
        let dir = TempDir::new();
        let path = dir.0.join("font.ttf");
        fs::write(&path, data).unwrap();
        let mut original = fs::File::open(&path).unwrap();
        let (expected, original_requested) =
            project(&mut original, data.len() as u64, 0).unwrap();
        let positional_file = fs::File::open(&path).unwrap();
        let mut reader = PositionalReader::new(&positional_file, data.len() as u64);
        let (actual, requested) = project(&mut reader, data.len() as u64, 0).unwrap();
        assert_eq!(actual, expected);
        assert_eq!(requested, original_requested);
    }
}
