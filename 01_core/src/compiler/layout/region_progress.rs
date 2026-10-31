//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/region_progress.md
//! @prompt-hash 8508ea21
//! @layer L1
//! @updated 2026-09-28

use super::{FontMetrics, ImageSizer, Layouter};
use crate::entities::layout_types::Pt;

pub(super) fn next_region<M: FontMetrics, S: ImageSizer>(layouter: &mut Layouter<M, S>) {
    if !layouter.accept_flow_suffix_before_region_transition() {
        return;
    }
    // Uma transição normal materializa novamente a página corrente. O
    // page-run marca explicitamente sua página pós-boundary depois desta
    // chamada, distinguindo-a de `pagebreak()` público.
    layouter.page_run_boundary_empty = false;
    layouter.weak_page_successor = false;
    // **P538c** — se estiver em fluxo contínuo de colunas, fechar a
    // coluna actual e avançar para a seguinte da mesma página, se
    // possível. Só cria página física quando todas as colunas da
    // página actual estiverem cheias.
    if layouter.page_columns.is_some() {
        layouter.close_current_column();
        let count = layouter.page_columns.unwrap_or(1);
        if layouter.current_column + 1 < count {
            layouter.start_next_column();
            layouter.footnote_reserved_height = 0.0;
            if layouter.block_inline_region.is_some() {
                super::block::install_inline_region(layouter);
            }
            reset_closed_region_flow(layouter);
            super::paragraph_indent::reopen(layouter);
            layouter.refresh_footnote_reservation();
            return;
        }
        // Última coluna da página: juntar items de todas as colunas
        // no current_items e restaurar dimensões da página física antes
        // de a criar.
        layouter.merge_column_items();
        layouter.regions.current.width = layouter.page_config.width;
        layouter.regions.current.height = layouter.page_config.height;
    }

    // **P867** — dimensões finais quando `width: auto` / `height: auto`.
    // Calculadas antes de flush de floats/footnotes porque esses
    // mecanismos usam a altura da página como referencial.
    let page_width = if layouter.page_config.width.is_infinite() {
        layouter.compute_page_width()
    } else {
        layouter.page_config.width
    };
    let page_height = if layouter.page_config.height.is_infinite() {
        layouter.compute_page_height()
    } else {
        layouter.page_config.height
    };
    let footnote_bottom_y = if layouter.page_config.height.is_infinite() {
        Some(page_height - layouter.page_config.margin.bottom)
    } else {
        None
    };

    // P245 (M9d / M7+4) — flush floats pendentes na página actual
    // antes da transição. Top floats emit no topo, bottom no fundo.
    super::place::admit_fitting_floats(layouter);

    // P304 (P295.1) — flush footnote bodies pendentes no rodapé
    // antes de saving a Page. Items posicionados em Y absoluto
    // bottom-up; tornam-se parte dos `current_items` da página.
    layouter.flush_pending_footnote_bodies(footnote_bottom_y);

    layouter.commit_page(page_width, page_height);
    layouter.page_has_unpainted_material = false;
    let next_page = std::num::NonZeroUsize::new(layouter.pages.len() + 1).unwrap();
    layouter.page_config.margin =
        crate::entities::layout_types::PageMargins::resolve_for_page(
            layouter.page_config.margin_spec,
            layouter.page_config.auto_margin(),
            layouter.page_config.binding.resolve(layouter.page_config.page_dir),
            next_page,
        );
    layouter.regions.current.cursor_x = Pt(layouter.page_config.margin.left);
    layouter.regions.current.line_start_x = Pt(layouter.page_config.margin.left);
    layouter.regions.current.height = layouter.page_config.height;
    // **P761/P762** — quando a baseline inicial ainda está pendente,
    // `ensure_initial_baseline()` adicionará o offset do `top-edge`. Só
    // pre-posicionamos a baseline quando o offset já foi fixado.
    layouter.regions.current.cursor_y = if layouter.initial_baseline_pending {
        layouter.line_assumed_ascent = None;
        Pt(layouter.page_config.margin.top)
    } else {
        let (top, _) = layouter.metrics.text_edges(layouter.style.size, &layouter.style);
        layouter.line_assumed_ascent = Some(top.0);
        Pt(layouter.page_config.margin.top) + top
    };
    // P245/P1292-D — a nova região ainda não incorporou inserções;
    // o owner Place reconstruirá as reservas ao admitir a fila.
    layouter.cursor_y_top_reserve = 0.0;
    layouter.cursor_y_bottom_reserve = 0.0;
    layouter.footnote_reserved_height = 0.0;
    // P813 — o avanço do último flush pertence à página fechada.
    reset_closed_region_flow(layouter);

    // P251 (M9d / M7+5; ADR-0079 Categoria C.2 parcial) — flush
    // pending cell tails (row break TableCell cell-level) NO TOPO
    // da nova página. Items rebased pelo `cursor_y` actual (já
    // posicionado pós-margin + ascender). Subpadrão "DeferredX
    // buffer + flush em new_page" N=1 → 2 cumulativo (P245
    // floats + P251 cell tails).
    layouter.flush_pending_cell_tails();

    // **P538c** — após criar página física, se estiver em modo colunas,
    // preparar a primeira coluna da nova página.
    if layouter.page_columns.is_some() {
        layouter.current_column = 0;
        layouter.column_page_items.clear();
        layouter.start_column(0);
    }
    if layouter.block_inline_region.is_some() {
        super::block::install_inline_region(layouter);
    }
    super::paragraph_indent::reopen(layouter);
    layouter.refresh_footnote_reservation();
}

pub(super) fn post_line<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    had_items: bool,
    effective_line_committed: bool,
) {
    if had_items
        && !layouter.is_sub_frame
        && layouter.regions.current.cursor_y.0 > layouter.page_bottom_limit()
        && !effective_line_committed
    {
        if layouter.flow_suffix_transaction_active {
            layouter.flow_suffix_transaction_rejected = true;
        } else {
            layouter.new_page();
        }
    }
}

#[derive(Clone, Copy)]
pub(super) enum Admission {
    Generic,
    Text,
    KnownTop(f64),
}

/// Admit real continuation, rather than treating a predicted baseline as content.
pub(super) fn admit_next_text_line<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    admission: Admission,
) {
    if !layouter.is_sub_frame
        && layouter.page_config.height.is_finite()
        && layouter.regions.current.cursor_y.0 > layouter.page_bottom_limit()
        && !super::html_frame::items_have_material(&layouter.regions.current.current_line)
        && !layouter.current_page_is_empty()
    {
        let top = match admission {
            Admission::Generic => None,
            Admission::Text => Some(
                layouter.metrics.text_edges(layouter.style.size, &layouter.style).0 .0,
            ),
            Admission::KnownTop(top) => Some(top),
        };
        let candidate = top.map_or(layouter.regions.current.cursor_y.0, |top| {
            layouter.regions.current.cursor_y.0 + top
                - layouter.line_assumed_ascent.unwrap_or(top)
        });
        if candidate > layouter.page_bottom_limit() {
            layouter.new_page();
        }
    }
}

fn reset_closed_region_flow<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
) {
    layouter.pending_horizontal_paragraph = None;
    layouter.last_flush_advance = 0.0;
    layouter.flow_placement_end = None;
    layouter.block_place_region = None;
    layouter.block_place_decorations.clear();
    layouter.prev_block_equation_descent = 0.0;
    layouter.prev_block_below_pending = 0.0;
    layouter.block_chain_active = false;
}

#[cfg(test)]
mod face_region_regressions {
    use crate::compiler::layout::{FixedMetrics, FontMetrics};
    use crate::entities::{
        content::Content,
        image_sizer::NullImageSizer,
        introspector::{Introspector, TagIntrospector},
        layout_types::{FrameItem, Length, PageMarginSpec, PageMargins, Pt, TextStyle},
        style::{Style, Styles},
    };
    use comemo::Track;
    struct Faces;
    impl FontMetrics for Faces {
        fn advance(&self, text: &str, size: Pt, style: &TextStyle) -> Pt {
            FixedMetrics.advance(text, size, style)
        }
        fn vertical_metrics(&self, size: Pt, _: &TextStyle) -> (Pt, Pt) {
            (size * 1.2, size * 1.5)
        }
        fn cap_height(&self, size: Pt, style: &TextStyle) -> Pt {
            size * if style.bold {
                1.0
            } else if style.italic {
                0.6
            } else {
                0.7
            }
        }
        fn text_edges(&self, size: Pt, style: &TextStyle) -> (Pt, Pt) {
            let (top, bottom) = FixedMetrics.text_edges(size, style);
            (
                if style.top_edge.is_some() { top } else { self.cap_height(size, style) },
                bottom,
            )
        }
    }
    fn configured(
        intr: &dyn Introspector,
        baseline: f64,
        assumed: f64,
    ) -> super::super::Layouter<'_, Faces, NullImageSizer> {
        let mut l =
            super::super::Layouter::new(Faces, NullImageSizer, 10.0, intr.track());
        l.chain = l
            .chain
            .push_styles(&Styles::from_iter([Style::Size(Length::pt(10.0))]));
        l.style = TextStyle::from(&l.chain);
        l.page_config.width = 100.0;
        l.page_config.height = f64::INFINITY;
        l.page_config.margin = PageMargins::uniform(0.0);
        l.page_config.margin_spec = PageMarginSpec::uniform(0.0);
        l.regions.current.width = 100.0;
        l.regions.current.height = f64::INFINITY;
        l.regions.current.line_start_x = Pt(0.0);
        l.regions.current.cursor_x = Pt(20.0);
        l.initial_baseline_pending = false;
        l.regions.current.cursor_y = Pt(baseline);
        l.line_assumed_ascent = Some(assumed);
        l
    }
    #[test]
    fn face_edges_actual_next_line_can_fit_below_the_prediction() {
        let intr = TagIntrospector::empty();
        let mut l = configured(&intr, 7.0, 7.0);
        l.page_config.height = 19.6;
        l.regions.current.height = 19.6;
        l.layout_content(&Content::text("FIRST"));
        l.flush_line();
        let saved = l.chain.clone();
        l.chain = l.chain.push_styles(&Styles::from_iter([Style::italic(true)]));
        l.style = TextStyle::from(&l.chain);
        l.layout_content(&Content::text("SECOND"));
        l.chain = saved;
        l.style = TextStyle::from(&l.chain);
        let doc = l.finish();
        assert_eq!(doc.pages.len(), 1);
        let ys: Vec<_> = super::super::tests::frame_items_recursive(&doc.pages[0].items)
            .into_iter()
            .filter_map(|i| match i {
                FrameItem::Text { pos, text, .. } if !text.is_empty() => Some(pos.y.0),
                _ => None,
            })
            .collect();
        assert_eq!(ys, vec![7.0, 19.5]);
    }
    #[test]
    fn face_edges_new_page_tracks_the_face_that_installed_its_baseline() {
        let intr = TagIntrospector::empty();
        let mut l = configured(&intr, 6.0, 6.0);
        l.page_config.height = 100.0;
        l.regions.current.height = 100.0;
        let saved = l.chain.clone();
        l.chain = l.chain.push_styles(&Styles::from_iter([Style::italic(true)]));
        l.style = TextStyle::from(&l.chain);
        l.layout_content(&Content::text("OLD"));
        l.flush_line();
        l.chain = saved;
        l.style = TextStyle::from(&l.chain);
        l.new_page();
        l.layout_content(&Content::text("NEXT"));
        let doc = l.finish();
        let ys: Vec<_> = doc
            .pages
            .iter()
            .flat_map(|p| super::super::tests::frame_items_recursive(&p.items))
            .filter_map(|i| match i {
                FrameItem::Text { pos, text, .. } if text.as_str() == "NEXT" => {
                    Some(pos.y.0)
                }
                _ => None,
            })
            .collect();
        assert_eq!(ys, vec![7.0]);
    }
}
