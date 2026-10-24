//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/cursor.md
//! @prompt-hash 758a8dcb
//! @layer L1
//! @updated 2026-10-07
//!
//! Gestão do cursor do Layouter: palavras/wrap, paginação, colunas e floats.
//! O fechamento de linha pertence ao módulo irmão `line_commit` (P1503).
//! Extraído de `layout/mod.rs` no Passo 96.7 conforme ADR-0037.
#![allow(deprecated)] // P483 — FrameItem::Text fallback path legítimo
use crate::entities::{
    dir::Dir,
    image_sizer::ImageSizer,
    layout_types::{FrameItem, HAlign, Pt},
};

use super::metrics::FontMetrics;
impl<'a, M: FontMetrics, S: ImageSizer> super::Layouter<'a, M, S> {
    pub(super) fn space_width(&self) -> Pt {
        super::text_measure::space(&self.metrics, &self.style)
    }

    /// **P1120** — regista a extensão vertical de um item inline com caixa
    /// própria (`#box` com `height`, `#rotate`/`#scale`/`#skew`), medida a
    /// partir da baseline da linha: `ascent` acima, `descent` abaixo. A linha
    /// fecha com `altura = max(ascent) + max(descent)` (ver `flush_line`).
    pub(super) fn note_inline_extent(&mut self, ascent: f64, descent: f64) {
        if ascent > self.line_inline_ascent {
            self.line_inline_ascent = ascent;
        }
        if descent > self.line_inline_descent {
            self.line_inline_descent = descent;
        }
    }

    /// **P1120** — desce `dy` todos os items da linha em curso. Usado quando a
    /// baseline da linha tem de descer porque um item inline pede mais ascent
    /// do que o texto (o topo da linha já estava fixado). Items `Group`
    /// deslocam só a sua origem — os filhos são locais à matriz do grupo.
    pub(super) fn shift_current_line_y(&mut self, dy: f64) {
        for item in &mut self.regions.current.current_line {
            match item {
                FrameItem::TextSeparator { pos, .. }
                | FrameItem::Text { pos, .. }
                | FrameItem::TextShaped { pos, .. }
                | FrameItem::Glyph { pos, .. }
                | FrameItem::Shape { pos, .. }
                | FrameItem::Image { pos, .. }
                | FrameItem::Group { pos, .. } => {
                    pos.y = Pt(pos.y.0 + dy);
                }
                FrameItem::Line { start, end, .. } => {
                    start.y = Pt(start.y.0 + dy);
                    end.y = Pt(end.y.0 + dy);
                }
                FrameItem::Link { .. } => {
                    crate::compiler::layout::helpers::shift_frame_item_y(item, dy);
                }
                FrameItem::TextReplacement { items, .. }
                | FrameItem::Semantic { items, .. } => {
                    for child in items {
                        crate::compiler::layout::helpers::shift_frame_item_y(child, dy);
                    }
                }
            }
        }
    }

    /// **P448** — baseline ajustada pelo offset vertical do estilo (subscrito/
    /// sobrescrito). O `cursor_y` mantém-se como baseline principal da linha;
    /// o offset é aplicado só ao posicionamento do glyph.
    pub(super) fn baseline_y(&self) -> Pt {
        let offset_pt = self.style.baseline_offset.resolve_pt(self.style.size.val());
        self.regions.current.cursor_y + Pt(offset_pt)
    }

    /// **P751/P762** — fixa a baseline inicial da página/coluna no momento em que o
    /// primeiro conteúdo real é emitido. Enquanto `initial_baseline_pending` é
    /// `true`, `cursor_y` representa o topo útil (margem). Esta função adiciona
    /// o offset do `top-edge` do estilo activo (default `cap-height`), convertendo
    /// `cursor_y` na baseline da primeira linha. Depois de fixada, a flag é desactivada.
    pub(super) fn ensure_initial_baseline(&mut self) {
        self.ensure_baseline_for(super::region_progress::Admission::Generic);
    }

    pub(super) fn ensure_text_baseline(&mut self) {
        self.ensure_baseline_for(super::region_progress::Admission::Text);
    }

    pub(super) fn ensure_line_baseline(&mut self, top: f64) {
        self.ensure_baseline_for(super::region_progress::Admission::KnownTop(top));
    }

    fn ensure_baseline_for(&mut self, admission: super::region_progress::Admission) {
        if self.initial_baseline_pending {
            let top = match admission {
                super::region_progress::Admission::KnownTop(top) => Pt(top),
                super::region_progress::Admission::Generic
                | super::region_progress::Admission::Text => {
                    self.metrics.text_edges(self.style.size, &self.style).0
                }
            };
            self.regions.current.cursor_y += top;
            // P1120 — ascent assumido ao fixar esta baseline.
            self.line_assumed_ascent = Some(top.0);
            self.initial_baseline_pending = false;
            self.prev_margin_is_parbreak = false;
        } else if self.block_chain_active {
            let top = match admission {
                super::region_progress::Admission::KnownTop(top) => Pt(top),
                super::region_progress::Admission::Generic
                | super::region_progress::Admission::Text => {
                    self.metrics.text_edges(self.style.size, &self.style).0
                }
            };
            let gap = if self.prev_margin_is_parbreak {
                let above = match self.chain.custom("par.spacing") {
                    Some(crate::entities::value::Value::Length(v)) => {
                        v.resolve_pt(self.style.size.val())
                    }
                    _ => self.style.size.val() * super::vanilla_defaults::PAR_SPACING,
                };
                self.prev_block_below_pending.max(above)
            } else {
                self.prev_block_below_pending
            };
            let flow_end = self.flow_placement_y();
            let next_baseline = if self.prev_block_below_pending > 0.0
                || self.prev_margin_is_parbreak
            {
                self.prev_line_baseline + self.prev_block_equation_descent + gap + top.0
            } else if let Some(end) = flow_end {
                end + gap + top.0
            } else {
                self.regions.current.cursor_y.0
            };

            // P1373 — a linha posterior a uma região preenchida deve nascer
            // na região seguinte. Fazer o avanço antes de criar qualquer
            // item evita drenar texto fora dos bounds para a página anterior.
            let page_bottom = self.page_bottom_limit();
            let outer_leading = self
                .style
                .leading
                .map(|leading| leading.resolve_pt(self.style.size.val()))
                .unwrap_or_else(|| {
                    self.style.size.val() * super::vanilla_defaults::PAR_LEADING
                });
            let tail_boundary = (self.prev_block_below_pending - outer_leading).max(0.0);
            let snap_fractional_tail = self.fractional_flow_tail_pending
                && next_baseline >= page_bottom
                && next_baseline <= page_bottom + tail_boundary + 0.001;
            if !self.is_sub_frame
                && self.page_config.height.is_finite()
                && next_baseline >= page_bottom
                && !snap_fractional_tail
                && !self.current_page_is_empty()
            {
                self.new_page();
                return;
            }

            // **P1104** — Entrada pontual do primeiro texto de um parágrafo no protocolo de colapso
            if self.prev_block_below_pending > 0.0
                || self.prev_margin_is_parbreak
                || flow_end.is_some()
            {
                self.regions.current.cursor_y =
                    Pt(if snap_fractional_tail { page_bottom } else { next_baseline });
            }
            // P1120 — ascent assumido ao fixar esta baseline.
            self.line_assumed_ascent = Some(top.0);
            self.prev_block_below_pending = 0.0;
            self.block_chain_active = false;
            self.prev_block_equation_descent = 0.0;
            self.last_equation_descent = 0.0;
        } else {
            super::region_progress::admit_next_text_line(self, admission);
        }
    }

    pub(super) fn layout_word(&mut self, word: &str) {
        super::word_layout::word(self, word);
    }

    pub(super) fn layout_chunk(&mut self, chunk: &str) {
        super::word_layout::chunk(self, chunk);
    }

    /// P576 — desloca a linha actual para a margem direita se o seu
    /// estilo indicar `dir: rtl`. Usado por `flush_line` e por `finish`
    /// (a última linha do documento não passa por `flush_line`).
    pub(super) fn align_current_line_rtl(&mut self) {
        self.align_current_line_rtl_with_continuation(false);
    }

    pub(super) fn align_current_line_rtl_with_continuation(&mut self, continued: bool) {
        let is_rtl = super::paragraph::line_direction(self) == Some(Dir::RTL);
        let explicit = self.explicit_line_alignment;
        if explicit == Some(HAlign::Left) || (!is_rtl && explicit.is_none()) {
            return;
        }
        let right_margin = super::paragraph_indent::bounds(self).1;
        // **P867** — `width: auto` não tem limite direito; alinhamento RTL
        // perde o referencial, por isso decai para left.
        if !right_margin.is_finite() {
            return;
        }
        // **P592/P593** — alinhar pelo limite direito do conteúdo real, não
        // pelo cursor. O cursor pode incluir avanço de `Content::Space` final
        // (por exemplo, o newline após o texto), o que deslocaria visualmente a
        // linha RTL para a esquerda por um espaço. Usar `line_content_right`
        // evita esse erro.
        let line_refs: Vec<&FrameItem> = self
            .regions
            .current
            .current_line
            .iter()
            .enumerate()
            .filter(|(index, _)| !super::repeat::piece_paint(self, *index))
            .map(|(_, item)| item)
            .collect();
        if self.piece_alignment.is_some()
            && !line_refs.iter().any(|item| {
                super::html_frame::items_have_causal_line(std::slice::from_ref(*item))
            })
        {
            return;
        }
        let content_right = self.metrics.line_content_right(&line_refs);
        // O prefixo literal de um run RTL ocupa a borda visual direita.
        // O seu span já contém os avanços e os intervalos de tracking.
        let mut prefix_end = 0usize;
        let mut prefix_left = None;
        let mut prefix_right = None;
        for (index, item) in
            self.regions.current.current_line.iter().enumerate().take(if is_rtl {
                usize::MAX
            } else {
                0
            })
        {
            match item {
                FrameItem::TextSeparator {
                    pos,
                    advance,
                    origin: crate::entities::layout_types::TextSeparatorOrigin::Literal,
                    ..
                } => {
                    prefix_left.get_or_insert(pos.x.0);
                    prefix_right = Some(pos.x.0 + advance.0);
                    prefix_end = index + 1;
                }
                item if super::html_frame::is_context_marker(item) => {
                    prefix_end = index + 1;
                }
                _ => break,
            }
        }
        let prefix_span = prefix_left
            .zip(prefix_right)
            .map(|(left, right)| (right - left).max(0.0))
            .unwrap_or(0.0);
        let prefix_relocation =
            prefix_left.map(|left| content_right - left).unwrap_or(0.0);
        let interval = if continued {
            super::text_tracking::continued_rtl_end(&self.regions.current.current_line).0
        } else {
            0.0
        };
        let free = right_margin - content_right - interval - prefix_span;
        let offset = if explicit == Some(HAlign::Center) { free / 2.0 } else { free };
        let deferred = super::repeat::defer_piece_line(
            self,
            if explicit == Some(HAlign::Center) { HAlign::Center } else { HAlign::Right },
            free,
        );
        let translated: Vec<FrameItem> = self
            .regions
            .current
            .current_line
            .drain(..)
            .enumerate()
            .map(|(index, item)| {
                let leading_literal = index < prefix_end && matches!(item,
                    FrameItem::TextSeparator {
                        origin: crate::entities::layout_types::TextSeparatorOrigin::Literal,
                        ..
                    }
                );
                let (ix, iy) = super::helpers::item_pos(&item);
                let visual_x = ix + if deferred { 0.0 } else { offset }
                    + if leading_literal { prefix_relocation } else { 0.0 };
                super::helpers::translate_frame_item(item, Pt(visual_x), Pt(iy))
            })
            .collect();
        self.regions.current.current_line = translated;
    }

    pub(super) fn new_page(&mut self) {
        super::region_progress::next_region(self);
    }

    /// Realiza o prefixo que cabe na região activa e conserva o restante para
    /// a região seguinte. A primeira ocorrência numa região vazia progride
    /// mesmo se for maior que ela; essa combinação permanece `Unknown`, mas
    /// nunca cria loop sem progresso.
    fn flush_fitting_pending_floats(&mut self) -> usize {
        super::place::admit_fitting_floats(self)
    }

    /// P1292-D v9 — mantém o marcador logicamente activo até que todas as
    /// ocorrências que já existiam no seu ponto tenham sido realizadas. O
    /// tail é separado durante a operação, logo uma ocorrência posterior não
    /// pode receber crédito retroactivo.
    pub(super) fn finish_float_prefix_at_marker(&mut self, prefix_boundary: usize) {
        let boundary = prefix_boundary.min(self.floats_pending.len());
        if boundary == 0 {
            return;
        }

        let mut tail = self.floats_pending.split_off(boundary);
        while !self.floats_pending.is_empty() {
            self.flush_fitting_pending_floats();
            if self.floats_pending.is_empty() {
                break;
            }

            // O prefixo que não cabe termina a região pelo mesmo caminho
            // paginado já usado por overflow. `new_page` não chama
            // `flush_line` e o buffer continua activo na região seguinte.
            self.new_page();
        }

        self.floats_pending.append(&mut tail);
        // A fronteira só é armada depois de o prefixo e suas reservas terem
        // estabilizado. O próximo `layout_content` é a unidade causal do
        // sufixo; floats criados após este retorno não entram no boundary.
        if !self.flow_suffix_transaction_active {
            self.flow_suffix_checkpoint_pending = true;
        }
    }

    /// Drena o buffer ao fechar o documento, mas continua a usar a mesma
    /// distribuição por região. Cada transição conserva o sufixo que não
    /// coube; nenhuma ocorrência é descartada ou emitida duas vezes.
    pub(super) fn flush_pending_floats(&mut self) {
        let mut remaining_regions = self.floats_pending.len() + 1;
        while !self.floats_pending.is_empty() && remaining_regions > 0 {
            self.flush_fitting_pending_floats();
            if self.floats_pending.is_empty() {
                break;
            }

            self.new_page();
            remaining_regions -= 1;
        }
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

    #[test]
    fn p1503_fechamento_tem_owner_separado() {
        // Critério estrutural RED antes da extração. A semântica de layout
        // continua coberta pelos testes de caracterização já existentes.
        let cursor = include_str!("cursor.rs");
        let signature = concat!("pub(super) fn ", "flush_line(");
        assert!(
            !cursor.lines().any(|line| line.trim_start().starts_with(signature)),
            "flush_line deve pertencer a compiler/layout/line_commit.rs"
        );
    }

    #[test]
    fn p1504_pagina_fisica_tem_owner_separado() {
        let cursor = include_str!("cursor.rs");
        let signature = concat!("let page = ", "Page {");
        assert!(
            !cursor.contains(signature),
            "a montagem da pagina fisica deve pertencer a page_commit.rs"
        );
    }

    #[test]
    fn p1504_colunas_fisicas_tem_owner_da_feature() {
        let cursor = include_str!("cursor.rs");
        let signature = concat!("fn close_current_", "column(");
        assert!(
            !cursor.contains(signature),
            "close_current_column deve pertencer a columns.rs"
        );
    }

    #[test]
    fn p1504_cell_tails_tem_owner_separado() {
        let cursor = include_str!("cursor.rs");
        let signature = concat!("fn flush_pending_", "cell_tails(");
        assert!(
            !cursor.contains(signature),
            "flush_pending_cell_tails deve pertencer a cell_tail_commit.rs"
        );
    }

    #[test]
    fn p1504_segmentacao_tem_owner_separado() {
        let cursor = include_str!("cursor.rs");
        let signature = concat!("fn word_needs_line_", "segmentation(");
        assert!(
            !cursor.contains(signature),
            "segmentacao de palavra deve pertencer a word_segment.rs"
        );
    }

    #[test]
    fn p1504_emissao_float_tem_owner_place() {
        let cursor = include_str!("cursor.rs");
        let signature = concat!("fn translate_deferred_", "float(");
        assert!(
            !cursor.contains(signature),
            "translate_deferred_float deve pertencer a place.rs"
        );
    }

    #[test]
    fn p1504_replay_tem_owner_separado() {
        let cursor = include_str!("cursor.rs");
        let signature = concat!("fn layout_flow_suffix_", "transaction(");
        assert!(
            !cursor.contains(signature),
            "layout_flow_suffix_transaction deve pertencer a flow_suffix.rs"
        );
    }
}

#[cfg(test)]
mod rtl_literal_prefix_tests {
    use super::super::{FixedMetrics, FontMetrics, Layouter};
    use crate::entities::{
        dir::Dir,
        image_sizer::NullImageSizer,
        introspector::{Introspector, TagIntrospector},
        layout_types::{FrameItem, Length, PageMargins, Point, Pt, TextSeparatorOrigin},
    };
    use comemo::Track;

    #[test]
    fn literal_prefix_reserves_the_visual_right_edge_in_rtl() {
        let intr = TagIntrospector::empty();
        for t in [0.0, 0.2, -0.2] {
            let mut l = Layouter::new(
                FixedMetrics,
                NullImageSizer,
                100.0,
                (&intr as &dyn Introspector).track(),
            );
            l.page_config.margin = PageMargins::uniform(8.0);
            l.regions.current.width = 100.0;
            let mut style = l.style.clone();
            style.dir = Some(Dir::RTL);
            style.tracking = Some(Length::pt(t));
            let space = l.metrics.advance(" ", style.size, &style).0;
            let left = 8.0;
            let prefix = 2.0 * space + t;
            for x in [left, left + space + t] {
                l.regions.current.current_line.push(FrameItem::TextSeparator {
                    pos: Point { x: Pt(x), y: Pt(10.0) },
                    advance: Pt(space),
                    style: style.clone(),
                    origin: TextSeparatorOrigin::Literal,
                    bidi: None,
                });
            }
            let word_x = left + prefix + t;
            l.regions.current.current_line.push(FrameItem::Text {
                pos: Point { x: Pt(word_x), y: Pt(10.0) },
                text: "abc".into(),
                style,
                bidi: None,
            });
            let content_right = l.metrics.line_content_right(
                &l.regions.current.current_line.iter().collect::<Vec<_>>(),
            );
            l.align_current_line_rtl();
            let FrameItem::Text { pos: word, .. } = &l.regions.current.current_line[2]
            else {
                panic!()
            };
            let FrameItem::TextSeparator { pos: first, .. } =
                &l.regions.current.current_line[0]
            else {
                panic!()
            };
            let FrameItem::TextSeparator { pos: second, advance, .. } =
                &l.regions.current.current_line[1]
            else {
                panic!()
            };
            assert!((word.x.0 - (word_x + 92.0 - content_right - prefix)).abs() < 1e-9);
            assert!((first.x.0 - (92.0 - prefix)).abs() < 1e-9);
            assert!((second.x.0 + advance.0 - 92.0).abs() < 1e-9);
        }
    }
}
