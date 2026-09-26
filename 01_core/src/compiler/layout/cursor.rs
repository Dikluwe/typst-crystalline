//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/cursor.md
//! @prompt-hash b481813d
//! @layer L1
//! @updated 2026-09-25
//!
//! Gestão do cursor do Layouter: palavras/wrap, paginação, colunas e floats.
//! O fechamento de linha pertence ao módulo irmão `line_commit` (P1503).
//! Extraído de `layout/mod.rs` no Passo 96.7 conforme ADR-0037.
#![allow(deprecated)] // P483 — FrameItem::Text fallback path legítimo
use crate::entities::{
    corners::Corners,
    dir::Dir,
    geometry::ShapeKind,
    image_sizer::ImageSizer,
    layout_types::{FrameItem, Point, Pt},
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
        // **P593** — delegado para `FontMetrics::text_width`, a fonte única de
        // verdade do nível palavra (shaping + tracking).
        self.metrics.text_width(word, self.style.size, &self.style)
    }

    pub(super) fn space_width(&self) -> Pt {
        self.metrics.advance(" ", self.style.size, &self.style)
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
                FrameItem::Text { pos, .. }
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
                FrameItem::Semantic { items, .. } => {
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
            // P1120 — ascent assumido ao fixar esta baseline.
            self.line_assumed_ascent = top.0;
            self.initial_baseline_pending = false;
            self.prev_margin_is_parbreak = false;
        } else if self.block_chain_active {
            let (top, _) = self.metrics.text_edges(self.style.size, &self.style);
            let next_baseline = if self.prev_block_below_pending > 0.0 {
                self.prev_line_baseline
                    + self.prev_block_equation_descent
                    + self.prev_block_below_pending
                    + top.0
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
                && next_baseline > page_bottom
                && next_baseline <= page_bottom + tail_boundary + 0.001;
            if self.page_config.height.is_finite()
                && next_baseline >= page_bottom
                && !snap_fractional_tail
                && !self.current_page_is_empty()
            {
                self.new_page();
                return;
            }

            // **P1104** — Entrada pontual do primeiro texto de um parágrafo no protocolo de colapso
            if self.prev_block_below_pending > 0.0 {
                self.regions.current.cursor_y =
                    Pt(if snap_fractional_tail { page_bottom } else { next_baseline });
            }
            // P1120 — ascent assumido ao fixar esta baseline.
            self.line_assumed_ascent = top.0;
            self.prev_block_below_pending = 0.0;
            self.block_chain_active = false;
            self.prev_block_equation_descent = 0.0;
            self.last_equation_descent = 0.0;
        }
    }

    /// **P449/P471** — emite um `FrameItem::Text` precedido, se necessário, por
    /// um rectângulo de highlight. P471 adiciona `radius` (RoundedRect) e
    /// `extent` (extensão horizontal). O shape cobre `ascender → line_height`.
    fn push_text(&mut self, text: ecow::EcoString, width: Pt) {
        if let Some(fill) = self.style.highlight.clone() {
            let (ascender, line_height) =
                self.metrics.vertical_metrics(self.style.size, &self.style);
            let size_pt = self.style.size.val();
            let extent_pt = self
                .style
                .highlight_extent
                .map(|e| e.resolve_pt(size_pt))
                .unwrap_or(0.0);
            let shape_kind = match self.style.highlight_radius {
                Some(r) if r.resolve_pt(size_pt) > 0.0 => super::resolve_shape_kind(
                    &ShapeKind::RoundedRect { radii: Corners::uniform(r) },
                    self.style.size,
                ),
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
                fill: Some(crate::entities::paint::Paint::Solid(fill)),
                stroke: None,
                fill_rule: Default::default(),
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
        if super::word_segment::word_needs_line_segmentation(word) {
            return super::word_segment::layout_segmented_word(self, word);
        }

        // **P593** — usar `FontMetrics::text_width` (shaping + tracking) como
        // única fonte de largura de palavra.
        let w = self.metrics.text_width(word, self.style.size, &self.style);
        let right_margin = self.regions.current.width - self.page_config.margin.right;
        if self.regions.current.cursor_x.0 + w.0 > right_margin
            && self.regions.current.cursor_x.0 > self.page_config.margin.left
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
        let right_margin = self.regions.current.width - self.page_config.margin.right;
        if self.regions.current.cursor_x.0 + w.0 > right_margin
            && self.regions.current.cursor_x.0 > self.page_config.margin.left
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
        let right_margin = self.regions.current.width - self.page_config.margin.right;
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

    pub(super) fn new_page(&mut self) {
        if !self.accept_flow_suffix_before_region_transition() {
            return;
        }
        // Uma transição normal materializa novamente a página corrente. O
        // page-run marca explicitamente sua página pós-boundary depois desta
        // chamada, distinguindo-a de `pagebreak()` público.
        self.page_run_boundary_empty = false;
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
            Some(page_height - self.page_config.margin.bottom)
        } else {
            None
        };

        // P245 (M9d / M7+4) — flush floats pendentes na página actual
        // antes da transição. Top floats emit no topo, bottom no fundo.
        self.flush_fitting_pending_floats();

        // P304 (P295.1) — flush footnote bodies pendentes no rodapé
        // antes de saving a Page. Items posicionados em Y absoluto
        // bottom-up; tornam-se parte dos `current_items` da página.
        self.flush_pending_footnote_bodies(footnote_bottom_y);

        self.commit_page(page_width, page_height);
        let next_page = std::num::NonZeroUsize::new(self.pages.len() + 1).unwrap();
        self.page_config.margin =
            crate::entities::layout_types::PageMargins::resolve_for_page(
                self.page_config.margin_spec,
                self.page_config.auto_margin(),
                self.page_config.binding.resolve(self.page_config.page_dir),
                next_page,
            );
        self.regions.current.cursor_x = Pt(self.page_config.margin.left);
        self.regions.current.line_start_x = Pt(self.page_config.margin.left);
        self.regions.current.height = self.page_config.height;
        // **P761/P762** — quando a baseline inicial ainda está pendente,
        // `ensure_initial_baseline()` adicionará o offset do `top-edge`. Só
        // pre-posicionamos a baseline quando o offset já foi fixado.
        self.regions.current.cursor_y = if self.initial_baseline_pending {
            Pt(self.page_config.margin.top)
        } else {
            let (top, _) = self.metrics.text_edges(self.style.size, &self.style);
            Pt(self.page_config.margin.top) + top
        };
        // P245/P1292-D — a nova região ainda não incorporou inserções;
        // o owner Place reconstruirá as reservas ao admitir a fila.
        self.cursor_y_top_reserve = 0.0;
        self.cursor_y_bottom_reserve = 0.0;
        // P813 — o avanço do último flush pertence à página fechada.
        self.last_flush_advance = 0.0;
        self.prev_block_equation_descent = 0.0;
        self.prev_block_below_pending = 0.0;
        self.block_chain_active = false;

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
