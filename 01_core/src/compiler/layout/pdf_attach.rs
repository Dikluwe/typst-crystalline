//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/pdf_attach.md
//! @prompt-hash 46cf2c34
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
