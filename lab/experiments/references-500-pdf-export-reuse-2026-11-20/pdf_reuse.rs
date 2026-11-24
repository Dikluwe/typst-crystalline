// Explicit watch-owned cache in the laboratory. All slots belong to the
// exact retained document, font epoch and PDF configuration.
use super::{FontKey, PdfSerializationMode, PdfStandards, PdfTags, StreamMode};
use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    sync::Arc,
};
use typst_core::entities::{
    layout_types::{FrameItem, PagedDocument},
    page_canvas::PageFill,
};

pub(crate) const MAX_BYTES: usize = 4 * 1024 * 1024;

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct Key(pub StreamMode, pub PdfTags, pub PdfSerializationMode, pub PdfStandards);

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct Mode {
    pub fonts: bool,
    pub operators: bool,
}
impl Mode {
    pub(crate) fn from_env() -> Self {
        match std::env::var("TEKT_LAB_PDF_REUSE").as_deref() {
            Ok("fonts") => Self { fonts: true, operators: false },
            Ok("operators") => Self { fonts: false, operators: true },
            Ok("both") => Self { fonts: true, operators: true },
            _ => Self::default(),
        }
    }
}

pub(crate) struct FaceScan {
    pub chars: Vec<char>,
    pub text_chars: Vec<char>,
    pub glyph_ids: BTreeSet<u16>,
    pub shaped_mappings: BTreeMap<u16, char>,
    pub shaped_cluster_texts: Vec<(u16, String)>,
}
pub(crate) struct FontScans {
    pub views: Vec<PagedDocument>,
    pub faces: Vec<FaceScan>,
    pub used: Vec<BTreeSet<u16>>,
}

#[derive(Clone, PartialEq, Eq)]
pub(crate) struct OperatorMaps {
    pub character: Vec<HashMap<char, u16>>,
    pub remap: Vec<HashMap<u16, u16>>,
    pub nominal: Vec<HashMap<u16, i32>>,
    pub reverse: Vec<HashMap<u16, char>>,
}

#[derive(Default)]
pub(crate) struct Parts {
    pub mode: Mode,
    pub resolved: Option<Arc<Vec<FontKey>>>,
    pub bound: Option<Arc<Vec<FontKey>>>,
    pub basis: Option<Vec<FontKey>>,
    pub scans: Option<Arc<FontScans>>,
    pub operator_maps: Option<OperatorMaps>,
    pub operators: Vec<Option<Arc<[u8]>>>,
    pub operator_bytes: usize,
}

impl Parts {
    pub fn bind_basis(&mut self, fonts: &[FontKey]) {
        let mut remaining = MAX_BYTES;
        let bounded = fonts.len() <= 16 && fonts.iter().all(|(_, bytes)| {
            if let Some(left) = remaining.checked_sub(bytes.len()) {
                remaining = left;
                true
            } else {
                false
            }
        });
        if !bounded {
            *self = Self::default();
            return;
        }
        if self.basis.as_deref() != Some(fonts) {
            self.scans = None;
            self.operator_maps = None;
            self.operators.clear();
            self.operator_bytes = 0;
            self.basis = Some(fonts.to_vec());
        }
    }
}

#[derive(Default)]
pub(crate) struct Cache {
    entry: Option<Entry>,
}
struct Entry {
    document: Arc<PagedDocument>,
    epoch: Arc<()>,
    key: Key,
    requested: Mode,
    parts: Parts,
}
impl Cache {
    pub(crate) fn select(
        &mut self,
        document: &Arc<PagedDocument>,
        epoch: Arc<()>,
        key: Key,
        mode: Mode,
    ) -> Option<&mut Parts> {
        if !self.entry.as_ref().is_some_and(|entry| {
            Arc::ptr_eq(&entry.document, document)
                && Arc::ptr_eq(&entry.epoch, &epoch)
                && entry.key == key
                && entry.requested == mode
        }) {
            self.entry = None;
            if !eligible(document) {
                return None;
            }
            self.entry = Some(Entry {
                document: document.clone(), epoch, key, requested: mode,
                parts: Parts { mode, ..Parts::default() },
            });
        }
        self.entry.as_mut().map(|entry| &mut entry.parts)
    }
    pub(crate) fn clear(&mut self) { self.entry = None; }
}

// This scope cannot emit page-resource references, except the ordered
// font references validated separately and the fixed colour-space policy.
fn eligible(document: &PagedDocument) -> bool {
    fn visit(items: &[FrameItem], left: &mut usize, bytes: &mut usize, depth: usize) -> bool {
        depth < 64 && items.iter().all(|item| {
            let Some(rest) = left.checked_sub(1) else { return false };
            *left = rest;
            match item {
                FrameItem::Text { text, .. } => spend(bytes, text.len()),
                FrameItem::TextShaped { text, glyphs, .. } => spend(bytes, text.len())
                    && glyphs.len().checked_mul(32).is_some_and(|n| spend(bytes, n)),
                FrameItem::TextSeparator { .. } => true,
                FrameItem::Semantic { items, .. } | FrameItem::Link { items, .. }
                | FrameItem::TextReplacement { items, .. }
                | FrameItem::Group { items, clip_mask: None, .. } => visit(items, left, bytes, depth + 1),
                _ => false,
            }
        })
    }
    fn spend(left: &mut usize, amount: usize) -> bool {
        if let Some(next) = left.checked_sub(amount) {
            *left = next;
            true
        } else {
            false
        }
    }
    let mut left = 32_768;
    let mut bytes = MAX_BYTES;
    document.attachments.is_empty() && document.pages.len() <= 128
        && document.pages.iter().all(|page| {
            matches!(page.fill, PageFill::Auto | PageFill::None)
                && [&page.items, &page.background, &page.foreground].into_iter()
                    .all(|items| visit(items, &mut left, &mut bytes, 0))
        })
}
