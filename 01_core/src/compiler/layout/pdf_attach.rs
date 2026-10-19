//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/pdf_attach.md
//! @prompt-hash efcf18ef
//! @layer L1
//! @updated 2026-08-30

use std::sync::Arc;

use crate::entities::elements::pdf_attach::PdfAttachElem;

use super::{FontMetrics, ImageSizer, Layouter};

pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    elem: &PdfAttachElem,
) {
    layouter.attachments.push(Arc::new(elem.clone()));
}
