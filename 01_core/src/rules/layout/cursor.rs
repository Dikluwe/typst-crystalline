//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/layout.md
//! @prompt-hash 518a9856
//! @layer L1
//! @updated 2026-04-23
//!
//! Gestão do cursor do Layouter: largura de palavra, layout de palavra,
//! `flush_line`, `new_page`, número de página actual.
//! Extraído de `layout/mod.rs` no Passo 96.7 conforme ADR-0037.

use crate::entities::{
    image_sizer::ImageSizer,
    layout_types::{FrameItem, Page, Point, Pt},
};

use super::metrics::FontMetrics;

impl<M: FontMetrics, S: ImageSizer> super::Layouter<M, S> {
    pub(super) fn word_width(&self, word: &str) -> Pt {
        self.metrics.advance(word, self.style.size)
    }

    pub(super) fn space_width(&self) -> Pt {
        self.metrics.advance(" ", self.style.size)
    }

    pub(super) fn layout_word(&mut self, word: &str) {
        let w = self.word_width(word);
        let right_margin = self.page_config.width - self.page_config.margin;
        if self.cursor_x.0 + w.0 > right_margin && self.cursor_x.0 > self.page_config.margin {
            self.flush_line();
        }
        self.current_line.push(FrameItem::Text {
            pos:   Point { x: self.cursor_x, y: self.cursor_y },
            text:  word.into(),
            style: self.style,
        });
        self.cursor_x += w + self.space_width();
    }

    pub(super) fn flush_line(&mut self) {
        // Avançar cursor_y apenas se havia items pendentes na linha actual
        // (Passo 83). Caso contrário, flush_line é um no-op semanticamente
        // — evita acumular line_height em cascata quando Shape/Image/Heading
        // chamam flush_line por segurança antes do seu próprio push.
        let had_items = !self.current_line.is_empty();
        for item in self.current_line.drain(..) {
            self.current_items.push(item);
        }
        if had_items {
            let (_, line_height) = self.metrics.vertical_metrics(self.font_size_pt);
            self.cursor_y += line_height;
        }
        // Reiniciar ao início da linha actual — margem da página, ou cell_x
        // se estivermos dentro de um sub-layout de Grid (Passo 81.5).
        self.cursor_x = self.line_start_x;

        if self.cursor_y.0 > self.page_config.height - self.page_config.margin {
            self.new_page();
        }
    }

    pub(super) fn new_page(&mut self) {
        let page = Page {
            width:  self.page_config.width,
            height: self.page_config.height,
            items:  std::mem::take(&mut self.current_items),
        };
        self.pages.push(page);
        self.cursor_x = Pt(self.page_config.margin);
        self.line_start_x = Pt(self.page_config.margin);
        let (ascender, _) = self.metrics.vertical_metrics(self.font_size_pt);
        self.cursor_y = Pt(self.page_config.margin) + ascender;
    }

    /// Número da página actual (1-indexed).
    ///
    /// Abordagem A: `self.pages.len() + 1` — a página actual ainda não foi
    /// finalizada (não foi empurrada para `self.pages`), por isso a contagem
    /// de páginas finalizadas + 1 dá o número da página em curso.
    pub(super) fn current_page_number(&self) -> usize {
        self.pages.len() + 1
    }
}

#[cfg(test)]
mod smoke {
    #[test]
    fn module_compila_e_carrega() {
        // V2 smoke test — submódulo extraído no Passo 96.7 (ADR-0037).
        // A cobertura funcional vive em `layout/tests.rs`.
    }
}
