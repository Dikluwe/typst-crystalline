// Crystalline Lineage
// @prompt 00_nucleo/prompts/infra/fontdb/selective.md
// @prompt-hash 283ab985
// @layer L3
// @updated 2026-10-07

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


pub fn diagnostic_read(path:&Path,index:u32)->Option<(usize,u64)>{let mut f=std::fs::File::open(path).ok()?;let size=f.metadata().ok()?.len();let(b,n)=project(&mut f,size,index)?;Some((b.len(),n))}
