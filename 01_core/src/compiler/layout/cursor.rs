//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout.md
//! @prompt-hash 5da4bce9
//! @layer L1
//! @updated 2026-07-14
//!
//! Gestão do cursor do Layouter: largura de palavra, layout de palavra,
//! `flush_line`, `new_page`, número de página actual.
//! Extraído de `layout/mod.rs` no Passo 96.7 conforme ADR-0037.
#![allow(deprecated)] // P483 — FrameItem::Text fallback path legítimo
use crate::entities::{
    corners::Corners,
    counter_format::{count_numbering_tokens, format_counter},
    dir::Dir,
    geometry::ShapeKind,
    image_sizer::ImageSizer,
    layout_types::{FrameItem, Page, Point, Pt, TextStyle},
};

use icu_segmenter::{options::LineBreakOptions, LineSegmenter, LineSegmenterBorrowed};
use unicode_script::{Script, UnicodeScript};

use super::metrics::FontMetrics;
// P245 (M9d / M7+4) — DeferredFloat buffer entry usado por
// flush_pending_floats + emit_deferred_float.
use super::helpers::{item_pos, translate_frame_item};
use super::DeferredFloat;

impl<'a, M: FontMetrics, S: ImageSizer> super::Layouter<'a, M, S> {
    /// Largura de uma palavra em Pt, incluindo tracking entre glyphs
    /// (Passo 137, fase B.1 DEBT-52).
    ///
    /// Se `TextStyle.tracking` é `Some(length)`, acrescenta
    /// `(n - 1) × tracking_pt` onde n é o número de codepoints —
    /// paridade vanilla (entre pares de glyphs, não depois do último).
    fn word_width(&self, word: &str) -> Pt {
        // **P593** — delegado para `FontMetrics::text_width`, a fonte única de
        // verdade do nível palavra (shaping + tracking).
        self.metrics.text_width(word, self.style.size, &self.style)
    }

    pub(super) fn space_width(&self) -> Pt {
        self.metrics.advance(" ", self.style.size, &self.style)
    }

    /// **P448** — baseline ajustada pelo offset vertical do estilo (subscrito/
    /// sobrescrito). O `cursor_y` mantém-se como baseline principal da linha;
    /// o offset é aplicado só ao posicionamento do glyph.
    fn baseline_y(&self) -> Pt {
        let offset_pt = self.style.baseline_offset.resolve_pt(self.style.size.val());
        self.regions.current.cursor_y + Pt(offset_pt)
    }

    /// **P751/P762** — fixa a baseline inicial da página/coluna no momento em que o
    /// primeiro conteúdo real é emitido. Enquanto `initial_baseline_pending` é
    /// `true`, `cursor_y` representa o topo útil (margem). Esta função adiciona
    /// o offset do `top-edge` do estilo activo (default `cap-height`), convertendo
    /// `cursor_y` na baseline da primeira linha. Depois de fixada, a flag é desactivada.
    pub(super) fn ensure_initial_baseline(&mut self) {
        if self.initial_baseline_pending {
            let (top, _) = self.metrics.text_edges(self.style.size, &self.style);
            self.regions.current.cursor_y += top;
            self.initial_baseline_pending = false;
        }
    }

    /// **P449/P471** — emite um `FrameItem::Text` precedido, se necessário, por
    /// um rectângulo de highlight. P471 adiciona `radius` (RoundedRect) e
    /// `extent` (extensão horizontal). O shape cobre `ascender → line_height`.
    fn push_text(&mut self, text: ecow::EcoString, width: Pt) {
        if let Some(fill) = self.style.highlight {
            let (ascender, line_height) =
                self.metrics.vertical_metrics(self.style.size, &self.style);
            let size_pt = self.style.size.val();
            let extent_pt = self
                .style
                .highlight_extent
                .map(|e| e.resolve_pt(size_pt))
                .unwrap_or(0.0);
            let shape_kind = match self.style.highlight_radius {
                Some(r) if r.resolve_pt(size_pt) > 0.0 => {
                    ShapeKind::RoundedRect { radii: Corners::uniform(r) }
                }
                _ => ShapeKind::Rect,
            };
            self.regions.current.current_line.push(FrameItem::Shape {
                pos: Point {
                    x: Pt(self.regions.current.cursor_x.0 - extent_pt),
                    y: self.baseline_y() - ascender,
                },
                kind: shape_kind,
                width: width.0 + 2.0 * extent_pt,
                height: line_height.0,
                fill: Some(fill),
                stroke: None,
                parent_bbox_at_emit: None,
            });
        }
        self.regions.current.current_line.push(FrameItem::Text {
            pos: Point {
                x: self.regions.current.cursor_x,
                y: self.baseline_y(),
            },
            text,
            style: self.style.clone(),
        });
    }

    pub(super) fn layout_word(&mut self, word: &str) {
        // **P751** — fixar a baseline inicial com o estilo activo antes de
        // posicionar o primeiro texto real.
        self.ensure_initial_baseline();

        // **P756** — texto em scripts sem espaços (CJK, Thai, Lao, Myanmar,
        // Khmer) precisa de segmentação de linha antes do layout_word normal.
        // Se a palavra contiver tais scripts, fragmentamos com icu_segmenter
        // e emitimos cada fragmento via layout_chunk.
        if word_needs_line_segmentation(word) {
            return layout_segmented_word(self, word);
        }

        // **P593** — usar `FontMetrics::text_width` (shaping + tracking) como
        // única fonte de largura de palavra.
        let w = self.metrics.text_width(word, self.style.size, &self.style);
        let right_margin = self.regions.current.width - self.page_config.margin;
        if self.regions.current.cursor_x.0 + w.0 > right_margin
            && self.regions.current.cursor_x.0 > self.page_config.margin
        {
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
                    let available = right_margin - self.regions.current.cursor_x.0;
                    for &point in break_points.iter().rev() {
                        let prefix: String = word.chars().take(point).collect();
                        let prefix_with_hyphen = format!("{}-", prefix);
                        let pw = self.word_width(&prefix_with_hyphen);
                        if pw.0 <= available {
                            self.push_text(prefix_with_hyphen.into(), pw);
                            self.regions.current.cursor_x += pw;
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
        self.push_text(word.into(), w);
        // **P545** — o avanço de `space_width()` foi removido de
        // `layout_word`. O espaço entre palavras é responsabilidade do
        // caller: o layout de `Content::Text` (múltiplas palavras num
        // token) ou `Content::Space` (tokens adjacentes no markup).
        // Isto evita inserir espaço automático entre fragmentos de texto
        // produzidos por interpolações `#{expr}` adjacentes.
        self.regions.current.cursor_x += w;
    }

    /// P446 — emite um fragmento de texto sem adicionar o espaço de separação
    /// de palavras. Usado por smallcaps para compor uma palavra a partir de
    /// runs de tamanhos diferentes (minúsculas → maiúsculas a 0.8×).
    pub(super) fn layout_chunk(&mut self, chunk: &str) {
        // **P751** — fixar a baseline inicial com o estilo activo antes de
        // posicionar o primeiro texto real.
        self.ensure_initial_baseline();
        let w = self.word_width(chunk);
        let right_margin = self.regions.current.width - self.page_config.margin;
        if self.regions.current.cursor_x.0 + w.0 > right_margin
            && self.regions.current.cursor_x.0 > self.page_config.margin
        {
            self.flush_line();
        }
        self.push_text(chunk.into(), w);
        self.regions.current.cursor_x += w;
    }

    /// P576 — desloca a linha actual para a margem direita se o seu
    /// estilo indicar `dir: rtl`. Usado por `flush_line` e por `finish`
    /// (a última linha do documento não passa por `flush_line`).
    pub(super) fn align_current_line_rtl(&mut self) {
        let is_rtl =
            self.regions.current.current_line.iter().find_map(|item| match item {
                FrameItem::Text { style, .. } | FrameItem::TextShaped { style, .. } => {
                    style.dir
                }
                _ => None,
            }) == Some(Dir::RTL);
        if !is_rtl {
            return;
        }
        let right_margin = self.regions.current.width - self.page_config.margin;
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
        let line_refs: Vec<&FrameItem> =
            self.regions.current.current_line.iter().collect();
        let content_right = self.metrics.line_content_right(&line_refs);
        let offset = right_margin - content_right;
        let translated: Vec<FrameItem> = self
            .regions
            .current
            .current_line
            .drain(..)
            .map(|item| {
                let (ix, iy) = super::helpers::item_pos(&item);
                super::helpers::translate_frame_item(item, Pt(ix + offset), Pt(iy))
            })
            .collect();
        self.regions.current.current_line = translated;
    }

    /// **P842 (#38)** — expande os spacings fracionários (`h(Nfr)`)
    /// pendentes na linha actual, distribuindo o espaço restante
    /// proporcionalmente (paridade vanilla `Spacing::Fractional`: o fr
    /// consome o espaço entre o fim do conteúdo e a margem direita; vários
    /// fr partilham na razão dos valores; sem restante positivo, fr = 0).
    /// Chamado por `flush_line` e por `finish` antes do drain da linha.
    pub(super) fn expand_fr_spacings(&mut self) {
        if self.regions.current.pending_fr.is_empty() {
            return;
        }
        let right_margin = self.regions.current.width - self.page_config.margin;
        // **P867** — `width: auto` não define espaço restante; fracionários
        // não expandem (limpar pending sem alterar posições).
        if !right_margin.is_finite() {
            self.regions.current.pending_fr.clear();
            return;
        }
        let content_right = {
            let line_refs: Vec<&FrameItem> =
                self.regions.current.current_line.iter().collect();
            self.metrics.line_content_right(&line_refs)
        };
        let remaining = (right_margin - content_right).max(0.0);
        let pending = std::mem::take(&mut self.regions.current.pending_fr);
        if remaining > 0.0 {
            let total_fr: f64 = pending.iter().map(|(_, fr)| fr).sum();
            // Ordenado por índice de inserção. Cada fr translada os items
            // à sua direita APENAS pelo seu próprio share — os shares dos
            // fr anteriores já foram aplicados nas iterações anteriores
            // (o item à direita de dois fr recebe a soma dos dois shares,
            // uma parcela por iteração).
            let mut sorted = pending;
            sorted.sort_by_key(|(idx, _)| *idx);
            for (idx, fr) in sorted {
                let share = remaining * fr / total_fr;
                let line = std::mem::take(&mut self.regions.current.current_line);
                self.regions.current.current_line = line
                    .into_iter()
                    .enumerate()
                    .map(|(i, item)| {
                        if i >= idx {
                            let (ix, iy) = super::helpers::item_pos(&item);
                            super::helpers::translate_frame_item(item, Pt(ix + share), Pt(iy))
                        } else {
                            item
                        }
                    })
                    .collect();
            }
            // O cursor reflete a linha já expandida (decorações e medidas
            // subsequentes vêem o fim real da linha).
            self.regions.current.cursor_x = Pt(right_margin);
        }
    }

    pub(super) fn flush_line(&mut self) {
        // **P842 (#38)** — expandir h(Nfr) pendentes antes de qualquer
        // medição da linha (collector de decorações, leading, RTL align).
        self.expand_fr_spacings();
        // Avançar cursor_y apenas se havia items pendentes na linha actual
        // (Passo 83). Caso contrário, flush_line é um no-op semanticamente
        // — evita acumular line_height em cascata quando Shape/Image/Heading
        // chamam flush_line por segurança antes do seu próprio push.
        let had_items = !self.regions.current.current_line.is_empty();

        // P286 — hook decorações wrap-aware. Se o collector está activo
        // (consumer Underline/Strike/Overline o ligou) e há items na
        // linha que vai ser drenada, regista o segmento `(line_start_x,
        // cursor_x, cursor_y)` antes do advance. Backward-compat
        // estricta: collector None → nenhum overhead.
        if had_items {
            if let Some(coll) = self.decoration_lines_collector.as_mut() {
                coll.push(super::DecoSegment {
                    start_x: self.regions.current.line_start_x,
                    end_x: self.regions.current.cursor_x,
                    baseline_y: self.regions.current.cursor_y,
                });
            }
        }

        // Determinar o tamanho máximo de fonte presente nos items da linha e
        // o estilo do item que o possui (clonado para não manter borrow da
        // current_line durante o drain). Se a linha não tiver items de texto,
        // usa self.style como fallback.
        #[allow(deprecated)]
        let (max_font_size, max_style) = self
            .regions
            .current
            .current_line
            .iter()
            .filter_map(|item| match item {
                crate::entities::layout_types::FrameItem::Text { style, .. }
                | crate::entities::layout_types::FrameItem::TextShaped {
                    style, ..
                } => Some((style.size, style.clone())),
                _ => None, // neutro: N16[β] — itens não-textuais não contribuem métricas de tamanho para cálculo de linha
            })
            .fold((self.style.size, self.style.clone()), |max, (size, style)| {
                if size.0 > max.0 .0 {
                    (size, style)
                } else {
                    max
                }
            });

        // **P762** — consumer leading com default do vanilla (0,65 em).
        // `self.style` pode ter sido restaurado ao outer scope antes de
        // flush_line ser chamado. Em vez disso, peek no último item da
        // current_line — resolve o leading com base no seu próprio tamanho.
        #[allow(deprecated)]
        let line_leading_pt = self
            .regions
            .current
            .current_line
            .iter()
            .rev()
            .find_map(|item| match item {
                crate::entities::layout_types::FrameItem::Text { style, .. }
                | crate::entities::layout_types::FrameItem::TextShaped {
                    style, ..
                } => Some(
                    style
                        .leading
                        .map(|l| l.resolve_pt(style.size.val()))
                        // PAR_LEADING, ver vanilla_defaults.rs
                        .unwrap_or_else(|| style.size.val() * super::vanilla_defaults::PAR_LEADING),
                ),
                _ => None, // neutro: N16[β] — itens não-textuais não contribuem leading para espaçamento entre linhas
            })
            .unwrap_or_else(|| {
                // Sem texto na linha: usar o estilo activo do layouter.
                self.style
                    .leading
                    .map(|l| l.resolve_pt(self.style.size.val()))
                    // PAR_LEADING, ver vanilla_defaults.rs
                    .unwrap_or_else(|| self.style.size.val() * super::vanilla_defaults::PAR_LEADING)
            });

        // P576 — alinhamento de parágrafo RTL.
        self.align_current_line_rtl();

        for item in self.regions.current.current_line.drain(..) {
            self.regions.current.current_items.push(item);
        }
        if had_items {
            // **P762** — avanço entre linhas = top-edge + |bottom-edge| + leading,
            // em vez de line_height (ascender + descender + lineGap).
            let (top, bottom) = self.metrics.text_edges(max_font_size, &max_style);
            let advance = top + Pt(-bottom.0) + Pt(line_leading_pt);
            // **P813** — registar o avanço aplicado, para que consumidores
            // posteriores (equações de bloco) recuperem a baseline da linha
            // anterior (`cursor_y - last_flush_advance`).
            self.last_flush_advance = advance.0;
            // P952 — uma linha de texto fechada não é equação de bloco.
            self.prev_block_equation_descent = 0.0;
            self.regions.current.cursor_y += advance;
        }
        // Reiniciar ao início da linha actual — margem da página, ou cell_x
        // se estivermos dentro de um sub-layout de Grid (Passo 81.5).
        self.regions.current.cursor_x = self.regions.current.line_start_x;

        if self.regions.current.cursor_y.0 > self.page_bottom_limit() {
            self.new_page();
        }
    }

    pub(super) fn new_page(&mut self) {
        // **P538c** — se estiver em fluxo contínuo de colunas, fechar a
        // coluna actual e avançar para a seguinte da mesma página, se
        // possível. Só cria página física quando todas as colunas da
        // página actual estiverem cheias.
        if self.page_columns.is_some() {
            self.close_current_column();
            let count = self.page_columns.unwrap_or(1);
            if self.current_column + 1 < count {
                self.start_next_column();
                return;
            }
            // Última coluna da página: juntar items de todas as colunas
            // no current_items e restaurar dimensões da página física antes
            // de a criar.
            self.merge_column_items();
            self.regions.current.width = self.page_config.width;
            self.regions.current.height = self.page_config.height;
        }

        // **P867** — dimensões finais quando `width: auto` / `height: auto`.
        // Calculadas antes de flush de floats/footnotes porque esses
        // mecanismos usam a altura da página como referencial.
        let page_width = if self.page_config.width.is_infinite() {
            self.compute_page_width()
        } else {
            self.page_config.width
        };
        let page_height = if self.page_config.height.is_infinite() {
            self.compute_page_height()
        } else {
            self.page_config.height
        };
        let footnote_bottom_y = if self.page_config.height.is_infinite() {
            Some(page_height - self.page_config.margin)
        } else {
            None
        };

        // P245 (M9d / M7+4) — flush floats pendentes na página actual
        // antes da transição. Top floats emit no topo, bottom no fundo.
        self.flush_pending_floats();

        // P304 (P295.1) — flush footnote bodies pendentes no rodapé
        // antes de saving a Page. Items posicionados em Y absoluto
        // bottom-up; tornam-se parte dos `current_items` da página.
        self.flush_pending_footnote_bodies(footnote_bottom_y);

        // **P532** — guardar snapshot do numbering da página actual antes de
        // a fechar, para que o export PDF saiba se/desenha o número.
        let page_numbering = self.page_config.numbering.clone();
        let page_number = self.pages.len() + 1;

        let mut items = std::mem::take(&mut self.regions.current.current_items);
        // **P896** — resolve centragem/numeração de equação de bloco
        // adiadas por `width: auto` (ver `equation.rs`), agora que
        // `page_width`/`page_height` já estão finitos.
        self.apply_pending_equation_fixups(&mut items, page_width);
        // **P897** — mesma resolução para `Content::Align`/`Content::Place`
        // adiados por `width: auto` (ver `placement.rs`).
        self.apply_pending_align_fixups(&mut items, page_width);
        // **P898** — simétrico de P897, eixo vertical (`height: auto`).
        self.apply_pending_align_v_fixups(&mut items, page_height);

        // **P532** — se houver numeração automática, desenhar o número no rodapé.
        // **P541** — padrões compostos (≥2 tokens de numeração) precisam do total
        // de páginas, só conhecido no final; adiar para `finish()`.
        if let Some(pattern) = &page_numbering {
            if count_numbering_tokens(pattern) >= 2 {
                self.pending_page_numbering.push((
                    self.pages.len(),
                    page_number,
                    pattern.clone(),
                ));
            } else if let Some(text) = format_counter(&[page_number], pattern.as_str()) {
                let style = TextStyle::from(&self.chain);
                let text_width = self.metrics.advance(&text, style.size, &style).0;
                // rationale: P1064 Classe 1A — centragem horizontal de página ((page_width - text_width) / 2.0)
            let x = (page_width - text_width) / 2.0;
                // Coordenadas do layout: origem no canto superior-esquerdo,
                // Y cresce para baixo. O PDF inverte Y; posicionar perto do
                // fundo da página requer Y próximo de height - margin/2.
                // rationale: P1064 Classe 1C — ponto médio da margem de rodapé (margin / 2.0)
            let y = page_height - self.page_config.margin / 2.0;
                items.push(FrameItem::Text {
                    pos: Point { x: Pt(x), y: Pt(y) },
                    text: text.into(),
                    style,
                });
            }
        }

        let page = Page {
            width: page_width,
            height: page_height,
            numbering: page_numbering,
            items,
        };
        self.pages.push(page);
        self.regions.current.cursor_x = Pt(self.page_config.margin);
        self.regions.current.line_start_x = Pt(self.page_config.margin);
        // **P761/P762** — quando a baseline inicial ainda está pendente,
        // `ensure_initial_baseline()` adicionará o offset do `top-edge`. Só
        // pre-posicionamos a baseline quando o offset já foi fixado.
        self.regions.current.cursor_y = if self.initial_baseline_pending {
            Pt(self.page_config.margin)
        } else {
            let (top, _) = self.metrics.text_edges(self.style.size, &self.style);
            Pt(self.page_config.margin) + top
        };
        // P245 — reset reservas na nova página.
        self.cursor_y_top_reserve = 0.0;
        self.cursor_y_bottom_reserve = 0.0;
        // P813 — o avanço do último flush pertence à página fechada.
        self.last_flush_advance = 0.0;
        self.prev_block_equation_descent = 0.0;

        // P251 (M9d / M7+5; ADR-0079 Categoria C.2 parcial) — flush
        // pending cell tails (row break TableCell cell-level) NO TOPO
        // da nova página. Items rebased pelo `cursor_y` actual (já
        // posicionado pós-margin + ascender). Subpadrão "DeferredX
        // buffer + flush em new_page" N=1 → 2 cumulativo (P245
        // floats + P251 cell tails).
        self.flush_pending_cell_tails();

        // **P538c** — após criar página física, se estiver em modo colunas,
        // preparar a primeira coluna da nova página.
        if self.page_columns.is_some() {
            self.current_column = 0;
            self.column_page_items.clear();
            self.start_column(0);
        }
    }

    /// **P538c** — fecha a coluna actual em fluxo contínuo: flush das
    /// notas de rodapé pendentes, translada os items da coluna para a
    /// posição horizontal absoluta correcta e guarda-os no buffer de
    /// colunas da página.
    fn close_current_column(&mut self) {
        // Flush das footnotes no fundo da coluna actual.
        let prev_column_mode = self.column_mode;
        self.column_mode = true;
        self.flush_pending_footnote_bodies(None);
        self.column_mode = prev_column_mode;

        // Transladar items da coluna actual de coordenadas locais (origem
        // na margem) para coordenadas absolutas na página.
        let dx = self.column_origin_x - self.page_config.margin;
        let items = std::mem::take(&mut self.regions.current.current_items);
        let translated = items
            .into_iter()
            .map(|item| {
                let (x, y) = item_pos(&item);
                translate_frame_item(item, Pt(x + dx), Pt(y))
            })
            .collect();
        self.column_page_items.push(translated);
    }

    /// **P538c** — funde os items acumulados de todas as colunas da
    /// página actual no `current_items` da região actual, preparando a
    /// criação de uma nova página física.
    pub(super) fn merge_column_items(&mut self) {
        let mut merged: Vec<FrameItem> = Vec::new();
        for column in std::mem::take(&mut self.column_page_items) {
            merged.extend(column);
        }
        // A coluna actual já foi fechada e está em column_page_items;
        // limpar current_items por segurança e repor merged.
        self.regions.current.current_items = merged;
    }

    /// **P538c** — fecha a coluna actual, funde as colunas da página e
    /// restaura o estado do Layouter para layout de página normal.
    /// Chamado por `columns::layout` no fim do modo fluxo contínuo.
    pub(super) fn finish_columns(&mut self) {
        if self.page_columns.is_none() {
            return;
        }
        self.close_current_column();
        self.merge_column_items();
        // Restaurar estado de página normal.
        self.page_columns = None;
        self.current_column = 0;
        self.column_page_items.clear();
        self.column_x_offsets.clear();
        self.column_mode = false;
        self.column_origin_x = 0.0;
        self.column_width = self.regions.current.width; // já é a largura da página
        self.regions.current.width = self.page_config.width;
        self.regions.current.cursor_x = Pt(self.page_config.margin);
        self.regions.current.line_start_x = Pt(self.page_config.margin);
    }

    /// **P538c** — avança o cursor para a coluna seguinte da mesma página.
    fn start_next_column(&mut self) {
        self.current_column += 1;
        self.start_column(self.current_column);
    }

    /// **P538c** — configura a região actual para a coluna `idx`.
    fn start_column(&mut self, idx: usize) {
        let count = self.page_columns.unwrap_or(1);
        if idx >= count {
            return;
        }
        self.column_origin_x = self.column_x_offsets[idx];
        self.regions.current.width = self.column_width;
        self.regions.current.cursor_x = Pt(self.page_config.margin);
        self.regions.current.line_start_x = Pt(self.page_config.margin);
        // **P761/P762** — quando a baseline inicial ainda está pendente,
        // `ensure_initial_baseline()` adicionará o offset do `top-edge`.
        self.regions.current.cursor_y = if self.initial_baseline_pending {
            Pt(self.page_config.margin)
        } else {
            let (top, _) = self.metrics.text_edges(self.style.size, &self.style);
            Pt(self.page_config.margin) + top
        };
        self.regions.current.current_line.clear();
    }

    /// **P245 (M9d / M7+4)** — flush dos floats pendentes na página
    /// actual. Top floats stack do topo; bottom stack do fundo;
    /// alignment.x aplica-se horizontalmente. `floats_pending.clear()`
    /// após emissão.
    pub(super) fn flush_pending_floats(&mut self) {
        if self.floats_pending.is_empty() {
            return;
        }
        use crate::entities::layout_types::{Align2D, FrameItem, HAlign, Point, VAlign};
        let margin = self.page_config.margin;
        let page_w = self.regions.current.width;
        let page_h = self.regions.current.height;
        let avail_w = page_w - 2.0 * margin;
        let area_top = margin;
        let area_bot = page_h - margin;

        let floats: Vec<DeferredFloat> = std::mem::take(&mut self.floats_pending);

        // **P867** — com `height: auto`, não há fundo fixo; floats bottom
        // perdem o referencial e decaem para top. Com `width: auto`, o
        // alinhamento horizontal não tem referencial e decai para left.
        let height_auto = !page_h.is_finite();
        let width_auto = !page_w.is_finite();

        // Separar top floats (alignment.v == Top) vs outros (default
        // bottom paridade vanilla).
        let (mut top_floats, mut bot_floats): (Vec<_>, Vec<_>) = floats
            .into_iter()
            .partition(|f| matches!(f.alignment.v, Some(VAlign::Top)));

        // **P867** — com `height: auto`, não há fundo fixo; floats bottom
        // perdem o referencial e decaem para top.
        if height_auto {
            top_floats.append(&mut bot_floats);
        }

        // Stack top floats do topo para baixo (cursor_y_top start area_top).
        let mut y_top_cursor = area_top;
        for f in top_floats.drain(..) {
            let f_y = y_top_cursor;
            let avail = if width_auto { f.body_width } else { avail_w };
            self.emit_deferred_float(&f, f_y, margin, avail);
            y_top_cursor += f.body_height + f.clearance;
        }

        // Stack bottom floats do fundo para cima (cursor_y_bot start area_bot).
        // Clearance afasta float do fundo (e do float seguinte stack-up).
        // **P867** — bot_floats só é processado quando `height` é finito.
        if !height_auto {
            let mut y_bot_cursor = area_bot;
            for f in bot_floats {
                y_bot_cursor -= f.clearance + f.body_height;
                let f_y = y_bot_cursor;
                self.emit_deferred_float(&f, f_y, margin, avail_w);
            }
        }

        let _ = Align2D { h: None::<HAlign>, v: None::<VAlign> }; // marker import use
        let _ = FrameItem::Group {
            pos: Point { x: Pt(0.0), y: Pt(0.0) },
            matrix: crate::entities::layout_types::TransformMatrix::identity(),
            clip_mask: None,
            inner_width: 0.0,
            inner_height: 0.0,
            items: Vec::new(),
        }; // marker
    }

    /// **P245 (M9d / M7+4)** — emite um `DeferredFloat` na posição
    /// final calculada. Aplica `alignment.x` para posicionamento
    /// horizontal dentro da largura útil da página. Translada items
    /// locais (origem 0,0 + ascender) para coordenadas finais.
    fn emit_deferred_float(
        &mut self,
        f: &DeferredFloat,
        target_y: f64,
        margin: f64,
        avail_w: f64,
    ) {
        use crate::entities::layout_types::{FrameItem, HAlign, Point};
        // `layout_sub_frame` posicionou items com ascender
        // offset (cursor_y = ascender inicial). Para alinhar shapes ao
        // target_y final exacto (não baseline), subtrair ascender do
        // offset de translação — paridade pattern `layout_place`
        // (placement.rs).
        let (ascender, _) = self.metrics.vertical_metrics(self.style.size, &self.style);
        // **P908** — cópia LÓGICA (pré-ascender) de `target_y`, para
        // rebasear `origin_y`/`applied_y` das entradas órfãs (quantidades
        // sem ascender embutido — mesma distinção física-vs-lógica de
        // `layout_align`/`layout_place`, `00_nucleo/prompts/compiler/
        // layout.md` §P908). O `target_y` abaixo (pós-subtracção) só é
        // correcto para os `FrameItem`s reais, cujo `pos.y` já é baseline.
        let target_y_logical = target_y;
        let target_y = target_y - ascender.0;
        // Calcular X conforme alignment.x.
        let x_offset = match f.alignment.h {
            // rationale: P1064 Classe 1A — centragem horizontal de container ((avail_w - body_w) / 2.0)
                Some(HAlign::Center) => (avail_w - f.body_width) / 2.0,
            Some(HAlign::Right) | Some(HAlign::End) => avail_w - f.body_width,
            _ => 0.0, // None / Left / Start default. // neutro: N16[α] — alignment None/Left/Start: x_offset = 0.0 (esgotamento de enum horizontal)
        };
        let target_x = margin + x_offset.max(0.0);

        // **P908** — rebasear as entradas órfãs de `pending_align_*`
        // carregadas por este `DeferredFloat` (um `Content::Align`/
        // `Content::Place` aninhado dentro do body deste float, sob
        // `width`/`height: auto`) com a MESMA translação `(target_x,
        // target_y)` aplicada aos `body_items` reais abaixo. `path[0]`
        // passa a apontar para a posição real em `current_items` (soma o
        // comprimento actual, capturado ANTES do merge). Como
        // `flush_pending_floats` corre sempre antes de
        // `apply_pending_align_fixups` na mesma `new_page()`/`finish()`
        // (ver `00_nucleo/prompts/compiler/layout.md` §P908), a entrada
        // resolve-se contra a página correcta sem sequenciamento novo.
        let insertion_base = self.regions.current.current_items.len();
        for (mut path, count, align, content_w, origin_x, applied_x) in
            f.orphaned_align_x.clone()
        {
            path[0] += insertion_base;
            self.pending_align_centering.push((
                path,
                count,
                align,
                content_w,
                origin_x + target_x,
                applied_x + target_x,
            ));
        }
        for (mut path, count, align, content_h, origin_y, dy, applied_y) in
            f.orphaned_align_y.clone()
        {
            path[0] += insertion_base;
            self.pending_align_v_centering.push((
                path,
                count,
                align,
                content_h,
                origin_y + target_y_logical,
                dy,
                applied_y + target_y_logical,
            ));
        }

        // Translate items: cada item ganha offset (target_x, target_y).
        for item in &f.body_items {
            let translated = match item.clone() {
                FrameItem::Text { pos, text, style } => FrameItem::Text {
                    pos: Point { x: pos.x + Pt(target_x), y: pos.y + Pt(target_y) },
                    text,
                    style,
                },
                FrameItem::TextShaped { pos, glyphs, style, text, units_per_em } => {
                    FrameItem::TextShaped {
                        pos: Point { x: pos.x + Pt(target_x), y: pos.y + Pt(target_y) },
                        glyphs,
                        style,
                        text,
                        units_per_em,
                    }
                }
                FrameItem::Shape {
                    pos,
                    kind,
                    width,
                    height,
                    fill,
                    stroke,
                    parent_bbox_at_emit,
                } => FrameItem::Shape {
                    pos: Point { x: pos.x + Pt(target_x), y: pos.y + Pt(target_y) },
                    kind,
                    width,
                    height,
                    fill,
                    stroke,
                    parent_bbox_at_emit,
                },
                FrameItem::Group {
                    pos,
                    matrix,
                    clip_mask,
                    inner_width,
                    inner_height,
                    items,
                } => FrameItem::Group {
                    pos: Point { x: pos.x + Pt(target_x), y: pos.y + Pt(target_y) },
                    matrix,
                    clip_mask,
                    inner_width,
                    inner_height,
                    items,
                },
                FrameItem::Line { start, end, thickness, color } => FrameItem::Line {
                    start: Point {
                        x: start.x + Pt(target_x),
                        y: start.y + Pt(target_y),
                    },
                    end: Point { x: end.x + Pt(target_x), y: end.y + Pt(target_y) },
                    thickness,
                    // P285: cursor reflector preserva cor (translação).
                    color,
                },
                FrameItem::Glyph { pos, glyph_id, x_advance, size, style, base_char } => FrameItem::Glyph {
                    pos: Point { x: pos.x + Pt(target_x), y: pos.y + Pt(target_y) },
                    glyph_id,
                    x_advance,
                    size,
                    style,
                    base_char,
                },
                FrameItem::Image {
                    pos,
                    data,
                    width,
                    height,
                    intrinsic_width,
                    intrinsic_height,
                    orientation,
                    ..
                } => FrameItem::Image {
                    pos: Point { x: pos.x + Pt(target_x), y: pos.y + Pt(target_y) },
                    data,
                    width,
                    height,
                    intrinsic_width,
                    intrinsic_height,
                    clip_rect: None,
                    orientation,
                },
                FrameItem::Link { target, items, pos, size } => FrameItem::Link {
                    target,
                    items: items
                        .into_iter()
                        .map(|child| {
                            let (ix, iy) = item_pos(&child);
                            translate_frame_item(
                                child,
                                Pt(target_x + ix),
                                Pt(target_y + iy),
                            )
                        })
                        .collect(),
                    pos: Point { x: pos.x + Pt(target_x), y: pos.y + Pt(target_y) },
                    size,
                },
            };
            self.regions.current.current_items.push(translated);
        }

        // **P772x** — mesma translação para segmentos de decoração do body,
        // reinseridos no collector ambiente SE ainda estiver activo neste
        // ponto (best-effort — ver comentário em `DeferredFloat::deco_segments`,
        // `mod.rs`: floats que só flusham muito depois do consumer
        // `Underline`/`Strike`/`Overline` ter retornado não têm collector
        // para reinserir e o segmento é descartado, não é um crash).
        if let Some(coll) = self.decoration_lines_collector.as_mut() {
            for seg in &f.deco_segments {
                coll.push(super::DecoSegment {
                    start_x: Pt(target_x + seg.start_x.val()),
                    end_x: Pt(target_x + seg.end_x.val()),
                    baseline_y: Pt(target_y + seg.baseline_y.val()),
                });
            }
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

    /// **P251 (M9d / M7+5; ADR-0079 Categoria C.2 parcial; cita
    /// ADR-0082 PROPOSTO N=2 segunda aplicação citante)** — flush
    /// dos cell tails pendentes no **topo** da página actual (chamado
    /// por `new_page` após cursor_y setup). Items rebased pelo
    /// `cursor_y` actual; cursor_y avança pela altura do tail emitido.
    ///
    /// Z-order paridade P248: fill atrás → items → stroke à frente.
    /// Bounds do fill/stroke usam tail extent (não cell original).
    ///
    /// Limit 3 forwardings consecutivos (paridade vanilla heurística;
    /// `forwarded_count >= 3` descarta silenciosamente — mitigação
    /// loop infinito caso tail recursivo).
    pub(super) fn flush_pending_cell_tails(&mut self) {
        use crate::compiler::layout::slicing::rebase_item_y;
        use crate::entities::geometry::ShapeKind;
        use crate::entities::layout_types::{FrameItem, Point};
        if self.pending_cell_tails.is_empty() {
            return;
        }
        let tails: Vec<crate::compiler::layout::DeferredCellTail> =
            std::mem::take(&mut self.pending_cell_tails);
        let cursor_top = self.regions.current.cursor_y.0;
        let mut max_y_after = cursor_top;
        for tail in tails {
            if tail.forwarded_count >= 3 {
                // P251 — limit forwarding; descarta silenciosamente
                // (paridade vanilla heurística max-iter).
                continue;
            }
            // Calcula altura do tail (max pos.y dos items locais).
            let mut tail_h = 0.0_f64;
            for item in tail.items.iter() {
                let y = match item {
                    FrameItem::Text { pos, .. } => pos.y.0,
                    FrameItem::TextShaped { pos, .. } => pos.y.0,
                    FrameItem::Line { start, .. } => start.y.0,
                    FrameItem::Glyph { pos, .. } => pos.y.0,
                    FrameItem::Image { pos, .. } => pos.y.0,
                    FrameItem::Shape { pos, .. } => pos.y.0,
                    FrameItem::Group { pos, .. } => pos.y.0,
                    FrameItem::Link { .. } => 0.0,
                };
                tail_h = tail_h.max(y);
            }
            // Z-order step 1: fill atrás.
            if let Some(c) = tail.fill {
                self.regions.current.current_items.push(FrameItem::Shape {
                    pos: Point { x: Pt(tail.origin_x), y: Pt(cursor_top) },
                    kind: ShapeKind::Rect,
                    width: tail.width,
                    height: tail_h,
                    fill: Some(c),
                    stroke: None,
                    parent_bbox_at_emit: None,
                });
            }
            // **P908** — rebasear as entradas órfãs deste tail (um
            // `Content::Align`/`Content::Place` aninhado dentro da célula
            // cujos items caíram inteiramente no tail, sob `width`/
            // `height: auto`) com a MESMA translação `+cursor_top`
            // aplicada aos items reais abaixo (`rebase_item_y`); `path[0]`
            // passa a apontar para a posição real em `current_items`
            // (soma o comprimento actual, capturado ANTES do merge,
            // já incluindo o fill do passo 1). Ver
            // `00_nucleo/prompts/compiler/layout.md` §P908.
            let insertion_base = self.regions.current.current_items.len();
            for (mut path, count, align, content_w, origin_x, applied_x) in
                tail.orphaned_align_x
            {
                path[0] += insertion_base;
                self.pending_align_centering.push((
                    path, count, align, content_w, origin_x, applied_x,
                ));
            }
            for (mut path, count, align, content_h, origin_y, dy, applied_y) in
                tail.orphaned_align_y
            {
                path[0] += insertion_base;
                self.pending_align_v_centering.push((
                    path,
                    count,
                    align,
                    content_h,
                    origin_y + cursor_top,
                    dy,
                    applied_y + cursor_top,
                ));
            }
            // Z-order step 2: items rebased.
            for item in tail.items {
                let final_item = rebase_item_y(item, cursor_top);
                self.regions.current.current_items.push(final_item);
            }
            // Z-order step 3: stroke à frente.
            if let Some(s) = tail.stroke {
                self.regions.current.current_items.push(FrameItem::Shape {
                    pos: Point { x: Pt(tail.origin_x), y: Pt(cursor_top) },
                    kind: ShapeKind::Rect,
                    width: tail.width,
                    height: tail_h,
                    fill: None,
                    stroke: Some(s),
                    parent_bbox_at_emit: None,
                });
            }
            max_y_after = max_y_after.max(cursor_top + tail_h);
        }
        // Avança cursor_y para depois dos tails emitidos.
        self.regions.current.cursor_y = Pt(max_y_after);
    }
}

// ── P756 — segmentação de linha para scripts sem espaços ────────────────────

/// Devolve um segmentador de linha LSTM configurado com os dados compilados.
///
/// `icu_segmenter` com `compiled_data` embute as tabelas Unicode e o modelo
/// LSTM como constantes estáticas em tempo de compilação; não há I/O em
/// runtime. Por isso pode residir em L1. O segmentador é reconstruído a
/// cada invocação para evitar estado global mutável (V13).
fn line_segmenter() -> LineSegmenterBorrowed<'static> {
    LineSegmenter::new_lstm(LineBreakOptions::default())
}

/// Verifica se uma palavra contém caracteres de scripts sem espaços que
/// exigem segmentação de linha (CJK, Thai, Lao, Myanmar, Khmer).
fn word_needs_line_segmentation(word: &str) -> bool {
    word.chars().any(|c| {
        matches!(
            c.script(),
            Script::Han
                | Script::Hiragana
                | Script::Katakana
                | Script::Thai
                | Script::Lao
                | Script::Myanmar
                | Script::Khmer
        )
    })
}

/// Verifica se o texto (ou a linguagem activa) justifica o tailoring de
/// aspas para chinês/japonês: `U+201C` (`“`) não inicia linha e `U+201D`
/// (`”`) não termina linha.
fn needs_cjk_quote_tailoring(
    word: &str,
    lang: Option<&crate::entities::lang::Lang>,
) -> bool {
    let is_cjk_lang = lang.map_or(false, |l| {
        let s = l.as_str();
        s == "zh" || s == "ja"
    });
    let has_cjk_script = word.chars().any(is_cjk_context_char);
    is_cjk_lang || has_cjk_script
}

/// Verifica se um caractere pertence a um contexto CJK: ideogramas,
/// hiragana, katakana, hangul ou pontuação/formas CJK comuns.
fn is_cjk_context_char(c: char) -> bool {
    matches!(
        c.script(),
        Script::Han
            | Script::Hiragana
            | Script::Katakana
            | Script::Hangul
            | Script::Bopomofo
    ) || matches!(c as u32,
        0x3000..=0x303F      // CJK Symbols and Punctuation
        | 0xFF00..=0xFFEF    // Halfwidth and Fullwidth Forms
        | 0xFE10..=0xFE1F    // Vertical Forms
        | 0xFE30..=0xFE4F    // CJK Compatibility Forms
    )
}

/// Remove oportunidades de quebra que colocariam `“` no início de linha ou
/// `”` no fim de linha, replicando o efeito do `CJ_SEGMENTER` do vanilla.
fn filter_cjk_quote_breakpoints(text: &str, mut breakpoints: Vec<usize>) -> Vec<usize> {
    const OPEN: char = '\u{201C}';
    const CLOSE: char = '\u{201D}';

    let chars: Vec<char> = text.chars().collect();
    let mut byte_to_char = vec![0usize; text.len() + 1];
    let mut byte_pos = 0;
    for (i, c) in chars.iter().enumerate() {
        byte_to_char[byte_pos] = i;
        byte_pos += c.len_utf8();
    }
    byte_to_char[byte_pos] = chars.len();

    breakpoints.retain(|&bp| {
        let idx = byte_to_char[bp];
        // Não quebrar antes de aspas de abertura.
        if idx < chars.len() && chars[idx] == OPEN {
            return false;
        }
        // Não quebrar depois de aspas de fecho.
        if idx > 0 && chars[idx - 1] == CLOSE {
            return false;
        }
        true
    });
    breakpoints
}

/// Fragmenta uma palavra nos breakpoints do segmentador e emite cada
/// fragmento via `layout_chunk`. Usado quando `layout_word` detecta um run
/// de script sem espaços.
fn layout_segmented_word<M: FontMetrics, S: ImageSizer>(
    layouter: &mut super::Layouter<'_, M, S>,
    word: &str,
) {
    let mut breakpoints: Vec<usize> = line_segmenter().segment_str(word).collect();

    if needs_cjk_quote_tailoring(word, layouter.style.lang.as_ref()) {
        breakpoints = filter_cjk_quote_breakpoints(word, breakpoints);
    }

    for w in breakpoints.windows(2) {
        let fragment = &word[w[0]..w[1]];
        if !fragment.is_empty() {
            layouter.layout_chunk(fragment);
        }
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
