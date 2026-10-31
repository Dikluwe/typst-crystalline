//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/page_commit.md
//! @prompt-hash 44e70c5b
//! @layer L1
//! @updated 2026-09-25
//!
//! Montagem da página física já fechada; o Cursor decide a transição.
#![allow(deprecated)]

use super::metrics::FontMetrics;
use crate::entities::{
    counter_format::{count_numbering_tokens, format_counter},
    image_sizer::ImageSizer,
    layout_types::{FrameItem, Page},
};

impl<'a, M: FontMetrics, S: ImageSizer> super::Layouter<'a, M, S> {
    pub(super) fn commit_page(&mut self, page_width: f64, page_height: f64) {
        // **P532** — guardar snapshot do numbering da página actual antes de
        // a fechar, para que o export PDF saiba se/desenha o número.
        let page_numbering = self.page_config.numbering.clone();
        let page_number = self.pages.len() + 1;
        let page = std::num::NonZeroUsize::new(page_number).unwrap();
        let margin_number = self
            .runtime
            .known_page_store
            .logical_number_for_page(page)
            .unwrap_or(page_number);

        super::divider::resolve(self, page_width);
        self.close_link_items();
        let mut items = std::mem::take(&mut self.regions.current.current_items);
        self.open_link_items();
        self.apply_pending_par_line_fixups(&mut items);
        // **P896** — resolve centragem/numeração de equação de bloco
        // adiadas por `width: auto` (ver `equation.rs`), agora que
        // `page_width`/`page_height` já estão finitos.
        self.apply_pending_equation_fixups(&mut items, page_width);
        // **P897** — mesma resolução para `Content::Align`/`Content::Place`
        // adiados por `width: auto` (ver `placement.rs`).
        self.apply_pending_align_fixups(&mut items, page_width);
        // **P898** — simétrico de P897, eixo vertical (`height: auto`).
        self.apply_pending_align_v_fixups(&mut items, page_height);
        let mut marginal_items =
            super::page_running::explicit_layers(self, page_width, page_height);

        // **P532** — se houver numeração automática, desenhar o número no rodapé.
        // **P541** — padrões compostos (≥2 tokens de numeração) precisam do total
        // de páginas, só conhecido no final; adiar para `finish()`.
        let number_marginal = match self.page_config.number_align.vertical {
            crate::entities::page_running::PageNumberVAlign::Top => {
                &self.page_config.header
            }
            crate::entities::page_running::PageNumberVAlign::Bottom => {
                &self.page_config.footer
            }
        };
        if super::page_running::numbering_enabled(number_marginal) {
            if let Some(crate::entities::numbering::Numbering::Pattern(pattern)) =
                &page_numbering
            {
                if count_numbering_tokens(pattern) >= 2 {
                    let pending = super::page_running::pending_number(
                        self,
                        margin_number,
                        pattern.clone(),
                    );
                    self.pending_page_numbering.push(pending);
                } else if let Some(text) =
                    format_counter(&[margin_number], pattern.as_str())
                {
                    let style = super::page_running::numbering_style(self);
                    let text_width = self.metrics.advance(&text, style.size, &style).0;
                    // rationale: P1064 Classe 1A — centragem horizontal de página ((page_width - text_width) / 2.0)
                    let ha = super::page_running::resolve_offset(
                        self.page_config.header_ascent,
                        self.page_config.margin.top,
                        style.size.0,
                    );
                    let fd = super::page_running::resolve_offset(
                        self.page_config.footer_descent,
                        self.page_config.margin.bottom,
                        style.size.0,
                    );
                    marginal_items.push(FrameItem::Text {
                        pos: super::page_running::number_baseline(
                            self.page_config.number_align,
                            super::page_running::number_position(
                                self.page_config.number_align,
                                page_width,
                                page_height,
                                self.page_config.margin,
                                text_width,
                                ha,
                                fd,
                            ),
                            &style,
                            &self.metrics,
                        ),
                        text: text.into(),
                        style,
                        bidi: None,
                    });
                }
            }
            if matches!(
                page_numbering,
                Some(crate::entities::numbering::Numbering::Func(_))
            ) {
                if let Some(content) = self
                    .runtime
                    .known_page_store
                    .visible_numbering_for_page(page)
                    .cloned()
                {
                    marginal_items.extend(super::page_running::realized_numbering_layer(
                        self,
                        &content,
                        page_width,
                        page_height,
                    ));
                }
            }
        }

        let bleed = crate::entities::page_canvas::PageBleed::resolve(
            self.page_config.bleed_spec,
            page_width,
            page_height,
            self.page_config.binding.resolve(self.page_config.page_dir),
            std::num::NonZeroUsize::new(self.pages.len() + 1).unwrap(),
            self.style.size.0,
        );
        let background_content = self.page_config.background.clone();
        let foreground_content = self.page_config.foreground.clone();
        let background = super::page_canvas::layout_layer(
            self,
            background_content.as_ref(),
            bleed,
            page_width,
            page_height,
            super::PageRunningSlot::Background,
        );
        let mut foreground = super::page_canvas::layout_layer(
            self,
            foreground_content.as_ref(),
            bleed,
            page_width,
            page_height,
            super::PageRunningSlot::Foreground,
        );
        foreground.extend(marginal_items);
        let page = Page {
            width: page_width,
            height: page_height,
            numbering: page_numbering,
            logical_number: page_number,
            supplement: self.page_config.supplement.resolve(self.chain.lang()),
            bleed,
            fill: self.page_config.fill.clone(),
            background,
            foreground,
            items,
        };
        self.pages.push(page);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn p1504_page_commit_possui_montagem() {
        assert!(include_str!("page_commit.rs").contains("let page = Page {"));
    }
}
