//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/flush.md
//! @prompt-hash c105f59a
//! @layer L1
//! @updated 2026-09-01
//!
//! Layout de `place.flush()`: realiza somente o prefixo de floats que já está
//! pendente no buffer activo.

use crate::entities::elements::flush::FlushElem;

use super::{FontMetrics, ImageSizer, Layouter};

pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    _element: &FlushElem,
) {
    let prefix_boundary = layouter.floats_pending.len();
    layouter.finish_float_prefix_at_marker(prefix_boundary);
}

#[cfg(test)]
mod tests {
    use comemo::Track;

    use super::*;
    use crate::compiler::layout::{DeferredFloat, FixedMetrics, NullImageSizer};
    use crate::entities::introspector::{Introspector, TagIntrospector};
    use crate::entities::layout_types::Align2D;

    fn deferred() -> DeferredFloat {
        DeferredFloat {
            alignment: Align2D::default(),
            body_items: Vec::new(),
            body_height: 1.0,
            body_width: 1.0,
            clearance: 0.0,
            deco_segments: Vec::new(),
            orphaned_align_x: Vec::new(),
            orphaned_align_y: Vec::new(),
        }
    }

    #[test]
    fn p1292_d_drains_only_the_prefix_pending_at_the_marker() {
        let introspector = TagIntrospector::empty();
        let introspector_dyn: &dyn Introspector = &introspector;
        let mut layouter =
            Layouter::new(FixedMetrics, NullImageSizer, 11.0, introspector_dyn.track());

        layouter.floats_pending.push(deferred());
        layout(&mut layouter, &FlushElem);
        assert!(layouter.floats_pending.is_empty());

        // Um float criado depois do marcador não recebe crédito retroactivo.
        layouter.floats_pending.push(deferred());
        assert_eq!(layouter.floats_pending.len(), 1);

        layout(&mut layouter, &FlushElem);
        assert!(layouter.floats_pending.is_empty());
        // Marcador aninhado/sem pendências é neutro.
        layout(&mut layouter, &FlushElem);
        assert!(layouter.floats_pending.is_empty());
    }
}
