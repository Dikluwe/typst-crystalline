//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/par_line_callbacks.md
//! @prompt-hash 226fb7f2
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

#[expect(
    clippy::large_enum_variant,
    reason = "E002: resultado único de layout por valor; Box adiciona alocação ao Complete, sem reduzir o documento"
)]
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
                (HAlign::Left, HAlign::Center | HAlign::Right | HAlign::End)
                | (HAlign::Right, HAlign::Left | HAlign::Center | HAlign::Start)
                | (
                    HAlign::Center | HAlign::Start | HAlign::End,
                    HAlign::Left
                    | HAlign::Center
                    | HAlign::Right
                    | HAlign::Start
                    | HAlign::End,
                ) => 0.0,
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

#[cfg(test)]
mod v16_d_tests {
    use super::*;
    use crate::compiler::layout::{FixedMetrics, Layouter};
    use crate::entities::{
        image_sizer::NullImageSizer,
        introspector::{Introspector, TagIntrospector},
        layout_types::{Point, Pt, TextStyle},
    };
    use comemo::Track;

    #[test]
    fn v16_d_number_fixup_covers_both_alignment_axes_and_item_ranges() {
        let aligns =
            [HAlign::Left, HAlign::Center, HAlign::Right, HAlign::Start, HAlign::End];
        // Rows are margin; columns are alignment. A narrow number occupies
        // the same regional slot as the wide number without moving neighbors.
        let expected = [
            [-12.0, 0.0, 0.0, -12.0, 0.0],
            [0.0; 5],
            [0.0, 0.0, 12.0, 0.0, 12.0],
            [0.0; 5],
            [0.0; 5],
        ];
        for (row, margin) in aligns.iter().enumerate() {
            for (column, align) in aligns.iter().enumerate() {
                let intr = TagIntrospector::empty();
                let mut l = Layouter::new(
                    FixedMetrics,
                    NullImageSizer,
                    11.0,
                    (&intr as &dyn Introspector).track(),
                );
                let mut items: Vec<_> = [0u16, 1, 2, 3, 4]
                    .into_iter()
                    .map(|glyph_id| FrameItem::Glyph {
                        pos: Point { x: Pt(50.0), y: Pt(30.0) },
                        glyph_id,
                        x_advance: Pt(8.0),
                        size: Pt(11.0),
                        style: TextStyle::regular(Pt(11.0)),
                        base_char: 'x',
                    })
                    .collect();
                l.pending_par_line_numbering =
                    vec![(1, 2, 8.0, *margin, *align), (3, 1, 20.0, *margin, *align)];
                l.apply_pending_par_line_fixups(&mut items);
                for (index, item) in items.iter().enumerate() {
                    let FrameItem::Glyph { pos, glyph_id, .. } = item else {
                        panic!("glyph preserved")
                    };
                    let delta = if index == 1 || index == 2 {
                        expected[row][column]
                    } else {
                        0.0
                    };
                    assert_eq!(
                        pos.x.0,
                        50.0 + delta,
                        "margin={margin:?}, align={align:?}, item={index}"
                    );
                    assert_eq!(pos.y.0, 30.0);
                    assert_eq!(*glyph_id, index as u16);
                }
                assert!(l.pending_par_line_numbering.is_empty());
                l.apply_pending_par_line_fixups(&mut items);
                let FrameItem::Glyph { pos, .. } = &items[1] else {
                    panic!("glyph preserved")
                };
                assert_eq!(
                    pos.x.0,
                    50.0 + expected[row][column],
                    "second call must not shift again"
                );
            }
        }
    }
}
