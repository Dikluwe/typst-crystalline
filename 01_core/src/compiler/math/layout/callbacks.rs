//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/math/layout/callbacks.md
//! @prompt-hash 8ac56b4f
//! @layer L1
//! @updated 2026-08-31
//!
//! Pure, pass-local transport for layout callbacks.

use std::cell::{Cell, RefCell};

use crate::entities::{
    func::Func,
    layout_types::{Angle, PagedDocument, TextStyle},
    location::Location,
    span::Span,
    style_chain::{StyleChain, StyleDelta},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MathCancelRequestId {
    pub equation: Location,
    pub occurrence: usize,
    pub line: u8,
}

#[derive(Debug, Clone)]
pub struct MathCancelRequest {
    pub id: MathCancelRequestId,
    pub func: Func,
    pub default: Angle,
    pub span: Span,
    pub styles: StyleChain,
}

#[derive(Debug, Clone)]
pub struct MathCancelResolution {
    pub request: MathCancelRequest,
    pub angle: Angle,
}

#[derive(Debug, Clone, Default)]
pub struct SealedMathCallbacks {
    resolutions: Vec<MathCancelResolution>,
}

impl SealedMathCallbacks {
    pub fn new(resolutions: Vec<MathCancelResolution>) -> Self {
        Self { resolutions }
    }

    pub fn resolutions(&self) -> &[MathCancelResolution] {
        &self.resolutions
    }
}

pub enum MathLayoutPassOutcome {
    Pending(Vec<MathCancelRequest>),
    Complete(PagedDocument),
}

pub struct MathCallbackPassState {
    store: Option<SealedMathCallbacks>,
    transcript: RefCell<Vec<MathCancelRequest>>,
    consumed: RefCell<Vec<bool>>,
    unresolved: Cell<bool>,
}

impl MathCallbackPassState {
    pub fn new(store: Option<&SealedMathCallbacks>) -> Self {
        let store = store.cloned();
        let count = store.as_ref().map_or(0, |s| s.resolutions.len());
        Self {
            store,
            transcript: RefCell::new(Vec::new()),
            consumed: RefCell::new(vec![false; count]),
            unresolved: Cell::new(false),
        }
    }

    pub fn resolve_or_record(&self, request: MathCancelRequest) -> Option<Angle> {
        self.transcript.borrow_mut().push(request.clone());
        let Some(store) = &self.store else {
            self.unresolved.set(true);
            return None;
        };
        let mut consumed = self.consumed.borrow_mut();
        let found = store.resolutions.iter().enumerate().find(|(index, resolution)| {
            !consumed[*index] && requests_match(&resolution.request, &request)
        });
        if let Some((index, resolution)) = found {
            consumed[index] = true;
            Some(resolution.angle)
        } else {
            self.unresolved.set(true);
            None
        }
    }

    pub fn finish(self, document: PagedDocument) -> MathLayoutPassOutcome {
        let transcript = self.transcript.into_inner();
        let integral = !self.unresolved.get()
            && self.consumed.into_inner().iter().all(|consumed| *consumed);
        if integral {
            MathLayoutPassOutcome::Complete(document)
        } else {
            MathLayoutPassOutcome::Pending(transcript)
        }
    }
}

fn requests_match(left: &MathCancelRequest, right: &MathCancelRequest) -> bool {
    left.id == right.id
        && left.func == right.func
        && left.default == right.default
        && left.styles.collapse() == right.styles.collapse()
}

pub(super) fn snapshot_styles(chain: &StyleChain, style: &TextStyle) -> StyleChain {
    chain.push(StyleDelta {
        bold: Some(style.bold),
        bold_from_strong: None,
        italic: Some(style.italic),
        italic_from_emph: None,
        size: Some(style.size.val()),
        fill: style.fill,
        heading_level: style.heading_level,
        weight: style.weight,
        tracking: style.tracking,
        leading: style.leading,
        top_edge: style.top_edge.clone(),
        bottom_edge: style.bottom_edge.clone(),
        lang: style.lang,
        font: style.font.clone(),
        subscript: Some(style.subscript),
        superscript: Some(style.superscript),
        highlight: Some(style.highlight),
        highlight_radius: style.highlight_radius,
        highlight_extent: style.highlight_extent,
        subscript_size: style.subscript_size,
        superscript_size: style.superscript_size,
        custom: Vec::new(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1291_passagem_vazia_devolve_documento_complete() {
        let outcome = MathCallbackPassState::new(None).finish(PagedDocument::new(vec![]));
        match outcome {
            MathLayoutPassOutcome::Complete(document) => {
                assert!(document.pages.is_empty())
            }
            MathLayoutPassOutcome::Pending(_) => {
                panic!("passagem vazia não pode ficar Pending")
            }
        }
    }
}
