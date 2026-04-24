//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/layout.md
//! @prompt-hash 518a9856
//! @layer L1
//! @updated 2026-04-21

pub mod counters;
pub mod figure;
pub mod image;
pub mod outline;
pub mod references;

use ecow::EcoString;

use std::sync::Arc;

use crate::entities::{
    content::Content,
    counter_state::CounterState,
    geometry::ShapeKind,
    image_sizer::{ImageSizer, NullImageSizer},
    layout_types::{Align2D, FrameItem, HAlign, Page, PageConfig, PagedDocument,
        Point, Pt, TextStyle, TransformMatrix, VAlign},
    style_chain::StyleChain,
};

// FontMetrics / FixedMetrics extraídos para metrics.rs (Passo 96.7, ADR-0037).
mod metrics;
pub use crate::rules::layout::metrics::{FixedMetrics, FontMetrics};

// Braços pesados do `layout_content` extraídos por cluster (Passo 96.7).
mod grid;
mod placement;
mod equation;

// Helpers livres usados pelo Layouter e pelos braços extraídos.
mod helpers;
use crate::rules::layout::helpers::{
    collect_sub_items, heading_scale, item_pos, measure_content, resolve_pt,
    translate_frame_item,
};

// Gestão de cursor: word/space, layout_word, flush_line, new_page.
mod cursor;

// ── Constantes de página ───────────────────────────────────────────────────

const DEFAULT_FONT_SIZE: f64 = 12.0;

// ── Layouter ──────────────────────────────────────────────────────────────

/// Máquina de estado de layout.
///
/// Consome `Content` e produz `PagedDocument`.
/// `font_size` é campo do Layouter — as métricas recebem-no por chamada
/// para suportar tamanhos mistos (rich text).
// Visibilidade `pub(super)` nos campos abaixo (Passo 96.7): os submódulos
// `grid.rs`, `placement.rs`, `equation.rs` recebem braços pesados do match
// `layout_content` e precisam de ler/escrever cursor, estado de célula,
// métricas e items acumulados. Criar getters/setters para cada acesso seria
// ruído sem ganho de invariante — a ADR-0037 Regra 3 autoriza `pub(super)`
// em campos quando métodos não agregam. A API externa (`pub fn layout`,
// `pub fn layout_content`) continua inalterada.
pub struct Layouter<M: FontMetrics, S: ImageSizer = NullImageSizer> {
    pub(super) metrics:      M,
    sizer:                   S,
    pub(super) font_size_pt: Pt,
    /// Estilo activo resolvido — vista achatada de `self.chain` cacheada
    /// para evitar resolver em cada leitura de `.size` no hot path do layout.
    /// Mantido sincronizado com `self.chain` por cada push/pop (Passo 100,
    /// ADR-0039).
    pub(super) style:        TextStyle,
    /// Cadeia de estilos activa — source-of-truth do estilo (Passo 100,
    /// ADR-0039). `Content::Styled` faz push; o save/restore de Strong/
    /// Emph/Heading/Text também passa por esta cadeia. `self.style` é a
    /// vista achatada (cache) que o layout consulta directamente.
    pub(super) chain:        StyleChain,
    /// Configuração da página activa. Mutável via Content::SetPage (Passo 81).
    pub page_config: PageConfig,
    pub(super) pages:        Vec<Page>,
    /// Items acumulados na página actual (ainda não fechada).
    pub(super) current_items: Vec<FrameItem>,
    pub(super) cursor_x:      Pt,
    pub(super) cursor_y:      Pt,      // posição da baseline actual
    /// Origem horizontal da linha actual (Passo 81.5).
    ///
    /// Normalmente `Pt(page_config.margin)`. Em sub-layouts de células de
    /// Grid, toma o valor de `cell_x` para que `flush_line()` reinicie o
    /// cursor à origem da célula em vez da margem da página.
    pub(super) line_start_x:  Pt,
    pub(super) current_line:  Vec<FrameItem>,
    pub counter:              CounterState,
    /// Índice de progresso por kind para figuras (Passo 75, DEBT-14).
    /// kind → número de figuras já dispostas. Reiniciado por invocação de layout().
    figure_progress: std::collections::HashMap<String, usize>,
    /// Indica que o contexto de layout actual não tem altura delimitada
    /// (ex: célula de grid Auto, box sem height explícito). Passo 82.
    ///
    /// Quando true, `VAlign::Bottom` e `VAlign::Horizon` em `Content::Align`
    /// decaem para `VAlign::Top` — não existe "fundo" para ancorar.
    /// Definido como true por `layout_sub_frame_with_width` e restaurado
    /// ao regressar ao contexto pai.
    pub(super) is_height_unconstrained: bool,
    /// Altura conhecida da célula de Grid em curso. Passo 83.
    ///
    /// Quando `Some(h)`, `Content::Align` usa `h` como `available_h` em
    /// `resolve_alignment` — `VAlign::Bottom` ancora ao limite inferior
    /// da célula e `VAlign::Horizon` centra verticalmente. Sobrepõe-se a
    /// `is_height_unconstrained`. Salvo e restaurado por célula no braço
    /// `Content::Grid`. `None` no fluxo normal da página.
    pub(super) cell_available_h: Option<f64>,
    /// Coordenadas X/Y do canto superior esquerdo da célula activa e
    /// largura da célula. Passo 84.6 (encerra DEBT-37).
    ///
    /// Quando todos `Some` em conjunto com `cell_available_h`,
    /// `Content::Place { scope: Column, .. }` ancora à célula.
    /// Salvos e restaurados por célula no braço `Content::Grid`.
    pub(super) cell_origin_x: Option<f64>,
    pub(super) cell_origin_y: Option<f64>,
    pub(super) cell_origin_w: Option<f64>,
}

impl<M: FontMetrics, S: ImageSizer> Layouter<M, S> {
    pub fn new(metrics: M, sizer: S, font_size: f64) -> Self {
        let cfg = PageConfig::default();
        let size = Pt(font_size);
        let (ascender, _) = metrics.vertical_metrics(size);
        Self {
            metrics,
            sizer,
            font_size_pt: size,
            style:        TextStyle::regular(size),
            chain:        StyleChain::default_chain(),
            page_config:  cfg.clone(),
            pages:        Vec::new(),
            current_items: Vec::new(),
            cursor_x:     Pt(cfg.margin),
            cursor_y:     Pt(cfg.margin) + ascender,
            line_start_x: Pt(cfg.margin),
            current_line: Vec::new(),
            counter:         CounterState::new(),
            figure_progress: std::collections::HashMap::new(),
            is_height_unconstrained: false,
            cell_available_h:        None,
            cell_origin_x:           None,
            cell_origin_y:           None,
            cell_origin_w:           None,
        }
    }

    /// Largura disponível para conteúdo (exclui margens dos dois lados).
    pub(super) fn available_width(&self) -> f64 {
        f64::max(0.0, self.page_config.width - 2.0 * self.page_config.margin)
    }

    /// Altura disponível para conteúdo (exclui margens topo e base).
    #[allow(dead_code)]
    pub(super) fn available_height(&self) -> f64 {
        f64::max(0.0, self.page_config.height - 2.0 * self.page_config.margin)
    }

    /// Limite inferior da página em pontos (`height - margin`). Passo 82.
    ///
    /// Usar este método em vez de `page_config.height - page_config.margin`
    /// inline — evita confundir com `available_height()` (que subtrai 2×margin).
    pub(super) fn page_bottom_limit(&self) -> f64 {
        self.page_config.height - self.page_config.margin
    }

    /// Calcula a coordenada `(x, y)` do canto superior esquerdo de um item
    /// dado o alinhamento, as dimensões do conteúdo, e a área disponível.
    /// Passo 82.
    ///
    /// `origin_x` e `origin_y` definem o canto superior esquerdo da área
    /// de referência (`line_start_x` para Align; `line_start_x`/`margin` para Place).
    #[allow(clippy::too_many_arguments)]
    pub(super) fn resolve_alignment(
        &self,
        align:       Align2D,
        content_w:   f64,
        content_h:   f64,
        available_w: f64,
        available_h: f64,
        origin_x:    f64,
        origin_y:    f64,
    ) -> (f64, f64) {
        let x = match align.h.unwrap_or(HAlign::Left) {
            HAlign::Left   => origin_x,
            HAlign::Center => origin_x + (available_w - content_w) / 2.0,
            HAlign::Right  => origin_x + (available_w - content_w),
        };

        let y = match align.v.unwrap_or(VAlign::Top) {
            VAlign::Top     => origin_y,
            VAlign::Horizon => origin_y + (available_h - content_h) / 2.0,
            VAlign::Bottom  => origin_y + (available_h - content_h),
        };

        (x, y)
    }

    /// Fonte de verdade estrutural: a página actual não tem nenhum item visual.
    ///
    /// Verifica tanto `current_items` (linhas já fechadas) como `current_line`
    /// (items ainda pendentes de flush) — uma linha não fechada ainda constitui
    /// conteúdo visível na página.
    fn current_page_is_empty(&self) -> bool {
        self.current_items.is_empty() && self.current_line.is_empty()
    }

    pub fn layout_content(&mut self, content: &Content) {
        match content {
            Content::Empty => {}

            Content::Text(text, node_style) => {
                // Estilo resolvido: merge de node_style (produzido pelo eval) com
                // self.style (cache da chain, actualizada por Content::Styled no
                // Passo 100, ADR-0039).
                //
                // Regra: qualquer propriedade "activa" na chain (`true` para
                // Bold/Italic, ou size > base) tem prioridade sobre o node_style.
                // Esta regra preserva a semântica histórica (heading > base) e
                // adiciona a semântica nova: `Content::Styled([Bold(true)], body)`
                // envolvendo um Text sem bold torna-o bold.
                let effective = TextStyle {
                    bold:   node_style.bold   || self.style.bold,
                    italic: node_style.italic || self.style.italic,
                    size:   if self.style.size > self.font_size_pt {
                        self.style.size   // heading ou Content::Styled aumentou
                    } else {
                        node_style.size   // #set text(size:) capturado em eval
                    },
                    fill:          self.style.fill.or(node_style.fill),
                    heading_level: self.style.heading_level.or(node_style.heading_level),
                    // Passo 136 (Fase A — DEBT-52): propagação top-wins.
                    // self.style (chain) tem prioridade sobre node_style (eval).
                    weight:        self.style.weight.or(node_style.weight),
                    tracking:      self.style.tracking.or(node_style.tracking),
                    leading:       self.style.leading.or(node_style.leading),
                    lang:          self.style.lang.or(node_style.lang.clone()),
                    font:          self.style.font.clone().or_else(|| node_style.font.clone()),
                };
                let prev_style = self.style.clone();
                self.style = effective;
                for word in text.split_whitespace() {
                    self.layout_word(word);
                }
                self.style = prev_style;
            }

            Content::Space => {
                self.cursor_x += self.space_width();
                if self.cursor_x.0 > self.page_config.width - self.page_config.margin {
                    self.flush_line();
                }
            }

            Content::Sequence(parts) => {
                for part in parts.iter() {
                    self.layout_content(part);
                }
            }

            // Passo 101: `Content::Strong` e `Content::Emph` removidos do enum.
            // `*bold*` e `_italic_` produzem `Content::Styled([Bold(true)/Italic(true)], body)`
            // no `eval_markup` (ou via `Content::strong/emph` construtores
            // redefinidos). O arm `Content::Styled` (introduzido no Passo 100)
            // cobre ambos os casos via push/pop na `chain`.

            Content::Heading { level, body } => {
                self.counter.step_hierarchical("heading", *level as usize);

                let heading_size = self.font_size_pt * heading_scale(*level);
                let prev = self.style.clone();
                self.style = TextStyle { bold: true, italic: false, size: heading_size, ..TextStyle::default() };
                if self.cursor_x.0 > self.page_config.margin { self.flush_line(); }

                // Prefixo numérico — apenas se numbering estiver activo
                if self.counter.is_numbering_active("heading") {
                    if let Some(num_str) = self.counter.format_hierarchical("heading") {
                        let prefix = Content::text(format!("{}. ", num_str));
                        self.layout_content(&prefix);
                    }
                }

                self.layout_content(body);
                self.flush_line();
                self.style = prev;
            }

            Content::SetHeadingNumbering { active } => {
                counters::layout_set_heading_numbering(&mut self.counter, *active);
            }

            Content::SetFigureNumbering { .. } => {
                // No-op: numeração baked-in em cada nó Figure (Passo 75, DEBT-14).
            }

            Content::CounterUpdate { key, action } => {
                counters::layout_counter_update(&mut self.counter, key, action);
            }

            Content::CounterDisplay { kind } => {
                let text = counters::format_counter_display(&self.counter, kind);
                let display = Content::text(text);
                self.layout_content(&display);
            }

            Content::Raw { text, block, .. } => {
                let prev = self.style.clone();
                // Raw: tamanho 90%, sem bold/italic
                // DEBT: seleccionar fonte monospace real quando FontBook tiver uma
                self.style = TextStyle { bold: false, italic: false, size: self.font_size_pt * 0.9, ..TextStyle::default() };
                if *block {
                    if self.cursor_x.0 > self.page_config.margin { self.flush_line(); }
                    self.cursor_x = Pt(self.page_config.margin) + self.font_size_pt;
                }
                for word in text.split_whitespace() { self.layout_word(word); }
                if *block { self.flush_line(); }
                self.style = prev;
            }

            Content::ListItem(body) => {
                if self.cursor_x.0 > self.page_config.margin { self.flush_line(); }
                let margin_pt = Pt(self.page_config.margin);
                self.current_line.push(FrameItem::Text {
                    pos:   Point { x: margin_pt, y: self.cursor_y },
                    text:  "•".into(),  // U+2022 — suportado com CIDFont (DEBT-5 pago)
                    style: self.style.clone(),
                });
                self.cursor_x = margin_pt + self.font_size_pt * 1.5;
                self.layout_content(body);
                self.flush_line();
                self.cursor_x = margin_pt;
            }

            Content::EnumItem { number, body } => {
                if self.cursor_x.0 > self.page_config.margin { self.flush_line(); }
                let margin_pt = Pt(self.page_config.margin);
                let label: EcoString = match number {
                    Some(n) => format!("{}.", n).into(),
                    None    => "-".into(),
                };
                self.current_line.push(FrameItem::Text {
                    pos:   Point { x: margin_pt, y: self.cursor_y },
                    text:  label,
                    style: self.style.clone(),
                });
                self.cursor_x = margin_pt + self.font_size_pt * 2.0;
                self.layout_content(body);
                self.flush_line();
                self.cursor_x = margin_pt;
            }

            Content::Link { body, .. } => {
                // DEBT: sublinhado e cor de link — requer FrameItem::Decoration (futuro)
                self.layout_content(body);
            }

            // ── Matemática (Passo 37) — delegação ao MathLayouter ───────────
            Content::Equation { body, block } => {
                self.layout_equation(body, *block);
            }

            Content::MathSequence(_)
            | Content::MathIdent(_)
            | Content::MathText(_)
            | Content::MathFrac { .. }
            | Content::MathAttach { .. }
            | Content::MathRoot { .. }
            | Content::MathDelimited { .. }
            | Content::MathMatrix { .. }
            | Content::MathCases { .. } => {
                // Nós matemáticos internos — normalmente não aparecem directamente
                // no layout fora de Content::Equation. Se aparecerem, renderizar como texto.
                let text = content.plain_text();
                for word in text.split_whitespace() {
                    self.layout_word(word);
                }
            }

            // Marcadores estruturais de equações — ignorados fora de contexto matemático.
            Content::MathAlignPoint | Content::Linebreak => {}

            // Passo 60 — Labelled e Ref delegados a references.rs (Passo 61).
            // Passo 63 — label passada para registo de página.
            Content::Labelled { target, label } => {
                references::layout_labelled(self, target, label);
            }

            Content::Ref { target } => {
                references::layout_ref(self, target);
            }

            // Passo 62/75 — Figure: delegado a figure.rs com kind/numbering (DEBT-14/15).
            Content::Figure { body, caption, kind, numbering } => {
                // Calcular o prefixo de numeração antes de chamar layout_figure.
                let caption_prefix: Option<String> = if let Some(_pattern) = numbering {
                    let progress = self.figure_progress.entry(kind.clone()).or_insert(0);
                    let idx = *progress;
                    *progress += 1;
                    let figure_number = self.counter.figure_numbers
                        .get(kind.as_str())
                        .and_then(|v| v.get(idx))
                        .copied()
                        .unwrap_or(idx + 1);
                    Some(format!("Figura {}: ", figure_number))
                } else {
                    None
                };
                figure::layout_figure(self, body, caption, caption_prefix);
            }

            // Passo 61 — TOC: delegado a outline.rs (Tarefa 5).
            Content::Outline => {
                outline::layout_outline(self);
            }

            Content::Shape { kind, width, height, fill, stroke } => {
                let available_w = self.available_width();
                let (resolved_w, resolved_h) = match kind {
                    ShapeKind::Rect | ShapeKind::Ellipse | ShapeKind::Path(_) => {
                        let w = resolve_pt(width.as_deref(), available_w);
                        let h = resolve_pt(height.as_deref(), 0.0);
                        (w, h)
                    }
                    ShapeKind::Line { dx, dy } => (dx.abs(), dy.abs()),
                };

                if self.cursor_y.0 + resolved_h > self.page_config.height - self.page_config.margin {
                    self.new_page();
                }
                self.flush_line();

                let pos = Point { x: self.cursor_x, y: self.cursor_y };
                self.current_items.push(FrameItem::Shape {
                    pos,
                    kind:   kind.clone(),
                    width:  resolved_w,
                    height: resolved_h,
                    fill:   *fill,
                    stroke: stroke.clone(),
                });

                self.cursor_y += Pt(resolved_h);
            }

            Content::Transform { matrix, body } => {
                let (orig_w, orig_h) = measure_content(body, self.available_width());

                // Projectar os quatro cantos da AABB original através da matriz.
                let corners = [
                    matrix.apply(0.0, 0.0),
                    matrix.apply(orig_w, 0.0),
                    matrix.apply(0.0, orig_h),
                    matrix.apply(orig_w, orig_h),
                ];
                let min_x = corners.iter().map(|(x, _)| *x).fold(f64::INFINITY,     f64::min);
                let max_x = corners.iter().map(|(x, _)| *x).fold(f64::NEG_INFINITY, f64::max);
                let min_y = corners.iter().map(|(_, y)| *y).fold(f64::INFINITY,     f64::min);
                let max_y = corners.iter().map(|(_, y)| *y).fold(f64::NEG_INFINITY, f64::max);

                let _new_w = max_x - min_x;
                let new_h = max_y - min_y;

                if self.cursor_y.0 + new_h > self.page_config.height - self.page_config.margin {
                    self.new_page();
                }
                self.flush_line();

                let pos = Point { x: self.cursor_x, y: self.cursor_y };

                // Compensação de origem negativa: garante que o canto mais à esquerda/acima
                // da forma transformada coincide com pos.
                let align        = TransformMatrix::translate(-min_x, -min_y);
                let final_matrix = align.concat(matrix);

                let available_w  = self.available_width();
                let sub_items    = collect_sub_items(body, available_w);

                self.current_items.push(FrameItem::Group {
                    pos,
                    matrix:       final_matrix,
                    clip_mask:    None,
                    inner_width:  orig_w,
                    inner_height: orig_h,
                    items:        sub_items,
                });

                self.cursor_y += Pt(new_h);
            }

            Content::Grid { columns, rows, cells } => {
                self.layout_grid(columns, rows, cells);
            }

            Content::SetPage { width, height, margin } => {
                let mut new_config = self.page_config.clone();
                let mut changed    = false;

                if let Some(w) = width  { new_config.width  = *w; changed = true; }
                if let Some(h) = height { new_config.height = *h; changed = true; }
                if let Some(m) = margin { new_config.margin = *m; changed = true; }

                if changed {
                    if !self.current_page_is_empty() {
                        self.flush_line();
                        self.new_page();
                    }
                    self.page_config  = new_config;
                    self.cursor_x     = Pt(self.page_config.margin);
                    self.cursor_y     = Pt(self.page_config.margin);
                    self.line_start_x = Pt(self.page_config.margin);
                    // DEBT-35b: se available_width() vier a ter cache, invalidar aqui.
                }
            }

            Content::Image { data, width, height, .. } => {
                let dims = image::calculate_dimensions(
                    &data.0,  // &[u8] via PtrEqArc → Arc → deref
                    width.as_deref(),
                    height.as_deref(),
                    &self.sizer,
                );

                // Garantir linha limpa antes da imagem (bloco).
                self.flush_line();

                // Verificar se a imagem cabe na página actual.
                if self.cursor_y.0 + dims.height_pt > self.page_config.height - self.page_config.margin {
                    self.new_page();
                }

                // pos.y é o TOPO da bounding box — não o baseline de texto.
                // O exportador calcula pdf_y = page_height - pos.y - height.
                let pos = Point { x: Pt(self.page_config.margin), y: self.cursor_y };

                // DEBT-28 encerrado: intrinsic_width/height vêm de calculate_dimensions.
                // A segunda chamada a self.sizer.size() foi eliminada.
                let intrinsic_w = dims.intrinsic_width.unwrap_or(100);
                let intrinsic_h = dims.intrinsic_height.unwrap_or(100);

                self.current_items.push(FrameItem::Image {
                    pos,
                    data:             Arc::clone(&data.0), // .0 acede ao Arc interno de PtrEqArc
                    width:            Pt(dims.width_pt),
                    height:           Pt(dims.height_pt),
                    intrinsic_width:  intrinsic_w,
                    intrinsic_height: intrinsic_h,
                });

                self.cursor_y += Pt(dims.height_pt);

                if self.cursor_y.0 > self.page_config.height - self.page_config.margin {
                    self.new_page();
                }
            }

            Content::Align { alignment, body } => {
                self.layout_align(*alignment, body);
            }

            Content::Place { alignment, dx, dy, scope, body } => {
                self.layout_place(*alignment, *dx, *dy, *scope, body);
            }

            // Passo 100 (ADR-0039): `Content::Styled` activa push/pop na
            // `chain` interna. A vista achatada `self.style` é
            // re-sincronizada a partir da cadeia após push e depois do pop.
            // Save/restore via variável local — o `self.chain` do chamador
            // permanece íntegro se `layout_content` retornar via early
            // return (padrão Passo 98).
            Content::Styled(body, styles) => {
                let prev_chain = self.chain.clone();  // O(1) Arc::clone
                let prev_style = self.style.clone();
                self.chain = self.chain.push_styles(styles);
                self.style = TextStyle::from(&self.chain);
                self.layout_content(body);
                self.chain = prev_chain;
                self.style = prev_style;
            }
        }
    }
    pub fn finish(mut self) -> PagedDocument {
        for item in self.current_line.drain(..) {
            self.current_items.push(item);
        }
        if !self.current_items.is_empty() {
            let page = Page {
                width:  self.page_config.width,
                height: self.page_config.height,
                items:  self.current_items,
            };
            self.pages.push(page);
        }
        let mut doc = PagedDocument::new(self.pages);
        // Expor o mapa de páginas sem mudar a assinatura de layout() (Passo 63).
        doc.extracted_label_pages = self.counter.label_pages;
        doc
    }

    // ── Auxiliares de Grid (Passo 80) ─────────────────────────────────────

    /// Mede conteúdo com restrição de largura máxima.
    ///
    /// Usado pelo algoritmo de grid para determinar a largura das colunas Auto.
    /// Retorna `(width, height)` em pontos.
    pub(super) fn measure_content_constrained(&self, content: &Content, max_width: f64) -> (f64, f64) {
        match content {
            Content::Text(text, _style) => {
                let mut max_line_w  = 0.0_f64;
                let mut current_w   = 0.0_f64;
                let mut line_count  = 1usize;
                let space_w = self.metrics.advance(" ", self.font_size_pt).0;

                for word in text.split_whitespace() {
                    let word_w = self.metrics.advance(word, self.font_size_pt).0;
                    if current_w + word_w > max_width && current_w > 0.0 {
                        max_line_w = max_line_w.max(current_w);
                        line_count += 1;
                        current_w  = word_w + space_w;
                    } else {
                        current_w += word_w + space_w;
                    }
                }
                max_line_w = max_line_w.max(current_w);
                let (_, line_height) = self.metrics.vertical_metrics(self.font_size_pt);
                (max_line_w.min(max_width), line_height.0 * line_count as f64)
            }

            Content::Sequence(children) => {
                let mut total_h = 0.0_f64;
                let mut max_w   = 0.0_f64;
                for child in children.iter() {
                    let (w, h) = self.measure_content_constrained(child, max_width);
                    total_h += h;
                    max_w    = max_w.max(w);
                }
                (max_w, total_h)
            }

            Content::Shape { kind, width, height, .. } => {
                match kind {
                    ShapeKind::Rect | ShapeKind::Ellipse | ShapeKind::Path(_) => {
                        let w = resolve_pt(width.as_deref(), max_width).min(max_width);
                        let h = resolve_pt(height.as_deref(), 0.0);
                        (w, h)
                    }
                    ShapeKind::Line { dx, dy } => (dx.abs().min(max_width), dy.abs()),
                }
            }

            _ => (0.0, 0.0),
        }
    }

    /// Layout de conteúdo numa célula de grid isolada.
    ///
    /// Salva o estado completo do layouter, cria um frame temporário com
    /// cursor em (cell_x, ascender), executa o layout e restaura o estado.
    /// Retorna `(height, items)` com posições locais ao frame temporário.
    pub(super) fn layout_sub_frame_with_width(
        &mut self,
        content: &Content,
        cell_x: f64,
        _cell_width: f64,
    ) -> (f64, Vec<FrameItem>) {
        // Salvar estado.
        let saved_items         = std::mem::take(&mut self.current_items);
        let saved_line          = std::mem::take(&mut self.current_line);
        let saved_x             = self.cursor_x;
        let saved_y             = self.cursor_y;
        let saved_line_start_x  = self.line_start_x;
        let saved_unconstrained = self.is_height_unconstrained;

        // Inicializar cursor local — x = cell_x, y = ascender (como o layout principal).
        // `line_start_x = cell_x` garante que `flush_line()` dentro da célula
        // (chamado por Shape, word-wrap, etc.) reinicia o cursor à coluna
        // da célula, não à margem global da página (Passo 81.5).
        self.cursor_x     = Pt(cell_x);
        self.line_start_x = Pt(cell_x);
        let (ascender, _) = self.metrics.vertical_metrics(self.font_size_pt);
        self.cursor_y = ascender;
        let start_y = self.cursor_y.0;

        // Contexto sem altura delimitada — Content::Align decai VAlign::Bottom
        // e VAlign::Horizon para Top (não há "fundo" para ancorar). Passo 82.
        self.is_height_unconstrained = true;

        self.layout_content(content);

        // Flush de itens pendentes sem avançar linha (evitar double advance).
        for item in self.current_line.drain(..) {
            self.current_items.push(item);
        }

        let end_y       = self.cursor_y.0;
        let cell_height = (end_y - start_y).max(0.0);

        // Recuperar items do sub-frame e restaurar estado.
        let cell_items      = std::mem::replace(&mut self.current_items, saved_items);
        self.cursor_x                = saved_x;
        self.cursor_y                = saved_y;
        self.line_start_x            = saved_line_start_x;
        self.current_line            = saved_line;
        self.is_height_unconstrained = saved_unconstrained;

        (cell_height, cell_items)
    }
}

// ── Auxiliares ────────────────────────────────────────────────────────────

/// Layout com convergência de fixpoint (Passo 65).
///
/// Recebe o `CounterState` produzido por `introspect::introspect`.
/// Se o documento não contiver `Content::Outline` (`has_outline = false`),
/// corre uma única passagem — o fixpoint de páginas só serve a TOC.
/// Caso contrário, itera até convergência (máximo 5 vezes).
///
/// Para métricas de fonte reais: `03_infra::layout::layout_with_font()`.
pub fn layout(content: &Content, initial_state: CounterState) -> PagedDocument {
    use std::collections::HashMap;
    use crate::entities::label::Label;

    // ── Short-circuit: sem TOC, não há necessidade de fixpoint ──────────────
    // A condição correcta é `has_outline`, não `headings_for_toc.is_empty()`.
    // Um documento com títulos mas sem #outline() não precisa do ciclo.
    if !initial_state.has_outline {
        let mut l = Layouter::new(FixedMetrics, NullImageSizer, DEFAULT_FONT_SIZE);
        l.counter.resolved_labels  = initial_state.resolved_labels;
        l.counter.headings_for_toc = initial_state.headings_for_toc;
        // numbering_active: copiado porque equações não têm nó equivalente
        // a SetHeadingNumbering — sem esta cópia, testes de L1 de equações
        // numeradas só funcionariam via eval completo.
        l.counter.numbering_active = initial_state.numbering_active;
        // NÃO copiar label_pages — começa vazio via Layouter::new().
        // NÃO copiar hierarchical, flat — reconstruídos nó a nó.
        l.layout_content(content);
        return l.finish();
    }

    // ── Fixpoint: documentos com TOC ────────────────────────────────────────
    const MAX_ITERATIONS: usize = 5;

    // Mapa de páginas da iteração anterior — lido por `outline.rs`.
    // NÃO é o mesmo campo onde `references.rs` escreve durante o layout.
    // Separação leitura/escrita: Layouter lê de `known_page_numbers` e
    // escreve em `label_pages` (que começa vazio em cada iteração via Layouter::new()).
    let mut known_page_numbers: HashMap<Label, usize> = HashMap::new();
    let mut final_doc: Option<PagedDocument> = None;

    for _ in 0..MAX_ITERATIONS {
        let mut l = Layouter::new(FixedMetrics, NullImageSizer, DEFAULT_FONT_SIZE);

        // Estado base da introspecção — copiado em cada iteração.
        l.counter.resolved_labels  = initial_state.resolved_labels.clone();
        l.counter.headings_for_toc = initial_state.headings_for_toc.clone();
        l.counter.numbering_active = initial_state.numbering_active.clone();

        // Injectar páginas da iteração anterior para leitura pelo outline.rs.
        // label_pages (onde references.rs escreve) começa vazio via Layouter::new().
        l.counter.known_page_numbers = known_page_numbers.clone();

        l.layout_content(content);
        let doc = l.finish();

        // Convergência: mapa de páginas gerado == mapa da iteração anterior?
        if doc.extracted_label_pages == known_page_numbers {
            return doc;
        }

        // Actualizar para a próxima iteração.
        known_page_numbers = doc.extracted_label_pages.clone();
        final_doc = Some(doc);
    }

    // Limite atingido sem convergência (DEBT-17: caso patológico).
    // Retornar o documento da última iteração — melhor esforço.
    // Sem `log::` em L1 — não existe ADR que o autorize.
    final_doc.expect("layout: deve produzir pelo menos um documento")
}

// ── Testes ─────────────────────────────────────────────────────────────────


#[cfg(test)]
mod tests;
