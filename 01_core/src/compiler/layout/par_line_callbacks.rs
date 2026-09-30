//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/par_line_callbacks.md
//! @prompt-hash d2b57880
//! @layer L1
//! @updated 2026-09-26

use crate::entities::{
    content::Content,
    image_sizer::ImageSizer,
    layout_types::{FrameItem, HAlign, PagedDocument},
    style_chain::StyleChain,
};

use super::metrics::FontMetrics;

/// Pedido puro de realização de uma ocorrência funcional de `par.line`.
#[derive(Debug, Clone)]
pub struct ParLineCallbackRequest {
    pub func: crate::entities::func::Func,
    pub number: usize,
    pub span: crate::entities::span::Span,
    pub styles: StyleChain,
}

/// Conteúdo já realizado em L3 para uma ocorrência de numeração de linha.
#[derive(Debug, Clone)]
pub struct ParLineCallbackResolution {
    pub request: ParLineCallbackRequest,
    pub content: Content,
}

/// Store imutável e local à compilação.
#[derive(Debug, Clone, Default)]
pub struct SealedParLineCallbacks {
    resolutions: Vec<ParLineCallbackResolution>,
}

impl SealedParLineCallbacks {
    pub fn new(resolutions: Vec<ParLineCallbackResolution>) -> Self {
        Self { resolutions }
    }

    pub fn resolutions(&self) -> &[ParLineCallbackResolution] {
        &self.resolutions
    }
}

/// Requests de todas as famílias que ainda impedem um documento completo.
pub struct LayoutCallbackRequests {
    pub math: Vec<crate::compiler::math::layout::callbacks::MathCancelRequest>,
    pub par_line: Vec<ParLineCallbackRequest>,
}

pub enum LayoutPassOutcome {
    Pending(LayoutCallbackRequests),
    Complete(PagedDocument),
}

/// Transcript de `par.line` pertencente a uma única tentativa de Layouter.
pub struct ParLineCallbackPassState {
    store: Option<SealedParLineCallbacks>,
    transcript: std::cell::RefCell<Vec<ParLineCallbackRequest>>,
    consumed: std::cell::RefCell<Vec<bool>>,
    unresolved: std::cell::Cell<bool>,
}

impl ParLineCallbackPassState {
    pub(super) fn new(store: Option<&SealedParLineCallbacks>) -> Self {
        let store = store.cloned();
        let count = store.as_ref().map_or(0, |store| store.resolutions.len());
        Self {
            store,
            transcript: std::cell::RefCell::new(Vec::new()),
            consumed: std::cell::RefCell::new(vec![false; count]),
            unresolved: std::cell::Cell::new(false),
        }
    }

    pub(super) fn resolve_or_record(
        &self,
        request: ParLineCallbackRequest,
    ) -> Option<Content> {
        self.transcript.borrow_mut().push(request.clone());
        let Some(store) = &self.store else {
            self.unresolved.set(true);
            return None;
        };
        let mut consumed = self.consumed.borrow_mut();
        let found = store.resolutions.iter().enumerate().find(|(index, resolution)| {
            !consumed[*index] && par_line_requests_match(&resolution.request, &request)
        });
        if let Some((index, resolution)) = found {
            consumed[index] = true;
            Some(resolution.content.clone())
        } else {
            self.unresolved.set(true);
            None
        }
    }

    pub(super) fn finish(self) -> Result<(), Vec<ParLineCallbackRequest>> {
        let transcript = self.transcript.into_inner();
        let integral = !self.unresolved.get()
            && self.consumed.into_inner().iter().all(|consumed| *consumed);
        if integral {
            Ok(())
        } else {
            Err(transcript)
        }
    }
}

fn par_line_requests_match(
    left: &ParLineCallbackRequest,
    right: &ParLineCallbackRequest,
) -> bool {
    left.func == right.func
        && left.number == right.number
        && left.styles.collapse() == right.styles.collapse()
}

impl<'a, M: FontMetrics, S: ImageSizer> super::Layouter<'a, M, S> {
    /// P1414 — alinha todos os números da região pela maior largura nela
    /// observada. O item já nasce numa posição correta para a própria largura;
    /// aqui só aplicamos a diferença necessária para a largura máxima.
    pub(super) fn apply_pending_par_line_fixups(&mut self, items: &mut [FrameItem]) {
        let pending = std::mem::take(&mut self.pending_par_line_numbering);
        let max_width =
            pending.iter().map(|(_, _, width, _, _)| *width).fold(0.0, f64::max);
        for (index, count, width, margin, align) in pending {
            let delta = match (margin, align) {
                (HAlign::Left, HAlign::Left | HAlign::Start) => -(max_width - width),
                (HAlign::Right, HAlign::Right | HAlign::End) => max_width - width,
                _ => 0.0,
            };
            if delta != 0.0 {
                for item in items.iter_mut().skip(index).take(count) {
                    super::helpers::shift_frame_item_x(item, delta);
                }
            }
        }
    }
}

#[cfg(test)]
mod smoke {
    #[test]
    fn module_compila_e_carrega() {
        // Cobertura funcional: regressões P1414/P1415/P1417 na suíte central
        // de layout e nos consumers de columns/page commit.
    }
}
