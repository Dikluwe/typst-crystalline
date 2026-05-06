//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/layout.md
//! @prompt-hash 089621fc
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

impl<'a, M: FontMetrics, S: ImageSizer> super::Layouter<'a, M, S> {
    /// Largura de uma palavra em Pt, incluindo tracking entre glyphs
    /// (Passo 137, fase B.1 DEBT-52).
    ///
    /// Se `TextStyle.tracking` é `Some(length)`, acrescenta
    /// `(n - 1) × tracking_pt` onde n é o número de codepoints —
    /// paridade vanilla (entre pares de glyphs, não depois do último).
    fn word_width(&self, word: &str) -> Pt {
        let base = self.metrics.advance(word, self.style.size);
        let tracking_extra = self.style.tracking
            .map(|t| {
                let tracking_pt = t.resolve_pt(self.style.size.val());
                let n = word.chars().count();
                tracking_pt * n.saturating_sub(1) as f64
            })
            .unwrap_or(0.0);
        Pt(base.val() + tracking_extra)
    }

    pub(super) fn space_width(&self) -> Pt {
        self.metrics.advance(" ", self.style.size)
    }

    pub(super) fn layout_word(&mut self, word: &str) {
        let w = self.word_width(word);
        let right_margin = self.page_config.width - self.page_config.margin;
        if self.cursor_x.0 + w.0 > right_margin && self.cursor_x.0 > self.page_config.margin {
            // Passo 144 (ADR-0057): tentar hyphenation antes do
            // flush. Se `style.lang` define um idioma e `hypher`
            // produz pontos de quebra, escolher o maior prefixo
            // (com hífen literal) que cabe no espaço disponível;
            // emitir prefixo, fazer flush, e recursar com o resto.
            // Sem `lang` ou sem ponto de quebra que caiba: cai no
            // fallback `flush_line` original (palavra inteira para
            // linha seguinte — comportamento pré-144).
            if let Some(lang) = self.style.lang {
                let break_points = super::hyphenation::hyphenate(word, &lang);
                if !break_points.is_empty() {
                    let available = right_margin - self.cursor_x.0;
                    for &point in break_points.iter().rev() {
                        let prefix: String = word.chars().take(point).collect();
                        let prefix_with_hyphen = format!("{}-", prefix);
                        let pw = self.word_width(&prefix_with_hyphen);
                        if pw.0 <= available {
                            self.current_line.push(FrameItem::Text {
                                pos:   Point { x: self.cursor_x, y: self.cursor_y },
                                text:  prefix_with_hyphen.into(),
                                style: self.style.clone(),
                            });
                            self.cursor_x += pw;
                            self.flush_line();
                            let rest: String = word.chars().skip(point).collect();
                            self.layout_word(&rest);
                            return;
                        }
                    }
                }
            }
            self.flush_line();
        }
        self.current_line.push(FrameItem::Text {
            pos:   Point { x: self.cursor_x, y: self.cursor_y },
            text:  word.into(),
            style: self.style.clone(),
        });
        self.cursor_x += w + self.space_width();
    }

    pub(super) fn flush_line(&mut self) {
        // Avançar cursor_y apenas se havia items pendentes na linha actual
        // (Passo 83). Caso contrário, flush_line é um no-op semanticamente
        // — evita acumular line_height em cascata quando Shape/Image/Heading
        // chamam flush_line por segurança antes do seu próprio push.
        let had_items = !self.current_line.is_empty();

        // Passo 138 (Fase B.2 DEBT-52): consumer leading.
        // `self.style` pode ter sido restaurado ao outer scope antes de
        // flush_line ser chamado (ver arm Content::Text em layout/mod.rs
        // que faz `self.style = prev_style` após layout_word). Em vez
        // disso, peek no último FrameItem::Text da current_line — o seu
        // `.style.leading` é o valor efectivo do baseline.
        //
        // Fórmula (opt soma): `line_height = default + user_leading`.
        let line_leading_pt = self.current_line
            .iter()
            .rev()
            .find_map(|item| match item {
                crate::entities::layout_types::FrameItem::Text { style, .. } => {
                    style.leading.map(|l| l.resolve_pt(self.font_size_pt.val()))
                }
                _ => None,
            })
            .unwrap_or(0.0);

        for item in self.current_line.drain(..) {
            self.current_items.push(item);
        }
        if had_items {
            let (_, line_height) = self.metrics.vertical_metrics(self.font_size_pt);
            self.cursor_y += line_height + Pt(line_leading_pt);
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
