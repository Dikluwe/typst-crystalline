//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/layout.md
//! @prompt-hash 089621fc
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
    geometry::ShapeKind,
    image_sizer::{ImageSizer, NullImageSizer},
    layout_types::{Align2D, FrameItem, HAlign, Page, PageConfig, PagedDocument,
        Point, Pt, TextStyle, TransformMatrix, VAlign},
    location::Location,
    locator::Locator,
    style_chain::StyleChain,
};
use crate::rules::introspect::locatable::is_locatable;

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

// Hyphenation puro (Passo 144, ADR-0057): wrap sobre `hypher`.
mod hyphenation;

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
pub struct Layouter<'a, M: FontMetrics, S: ImageSizer = NullImageSizer> {
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
    // P190I (M6 fechado): `counter: CounterStateLegacy` ELIMINADO —
    // struct eliminada. Layouter consumers usam Introspector path
    // puro via `self.introspector` (P184D / P190G/H/I migrations).
    /// P168 (M5 sub-passo 2): introspector populado paralelamente ao
    /// `counter` legacy. Consumer actual é `references.rs::layout_ref`
    /// (figure-ref); outros consumers migram em M9+.
    ///
    /// **P204C (M8)** — migrado de `TagIntrospector` por valor para
    /// `Tracked<'a, dyn Introspector + 'a>` per ADR-0073 (paridade
    /// vanilla literal). Caller constrói `TagIntrospector` + `.track()`
    /// e passa o handle a `Layouter::new`. Lifetime `'a` atado à
    /// fonte do tracked (introspector concreto deve outlive Layouter).
    pub(super) introspector: comemo::Tracked<'a, dyn crate::entities::introspector::Introspector + 'a>,
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
    /// **P185C (mecanismo M3 da ADR-0068)** — gerador determinístico
    /// de `Location`s, sincronizado-por-construção com o `Locator`
    /// do walk de introspect (per P185A §3.3). Avança em cada chamada
    /// a `layout_content` cujo content satisfaz `is_locatable`.
    /// Nenhum consumer ainda — fica para P187/P188.
    pub(super) locator: Locator,
    /// **P185C** — `Location` do último content locatable processado.
    /// `None` antes de processar qualquer locatable. Consumers
    /// location-aware (`is_numbering_active_at`, `flat_counter_at`,
    /// P185B) consultam este campo em vez de snapshot final
    /// (cf. ADR-0068 PROPOSTO).
    pub(super) current_location: Option<Location>,
    /// **P190C (M6 categoria Page tracking)** — state Layouter-runtime
    /// dedicado. Campos `label_pages` + `known_page_numbers` movidos
    /// de `CounterStateLegacy` para `LayouterRuntimeState` por não
    /// serem derivados de Content pre-pass (Layouter-runtime apenas).
    /// Pattern arquitectural "Layouter-runtime → struct dedicada"
    /// estabelecido em P190C; replicado em P190D para `is_readonly`
    /// + `lang`.
    pub runtime: crate::entities::layouter_runtime_state::LayouterRuntimeState,
}

impl<'a, M: FontMetrics, S: ImageSizer> Layouter<'a, M, S> {
    /// **P204C (M8)** — `introspector` parameter agora obrigatório
    /// (migrado de `TagIntrospector` field assignment para
    /// `Tracked<'a, dyn Introspector + 'a>` aceite no construtor).
    /// Caller constrói `TagIntrospector` (provavelmente via
    /// `introspect_with_introspector`) + `.track()` e passa o handle.
    pub fn new(
        metrics:      M,
        sizer:        S,
        font_size:    f64,
        introspector: comemo::Tracked<'a, dyn crate::entities::introspector::Introspector + 'a>,
    ) -> Self {
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
            // P190I: counter field eliminated.
            // P204C: field passa a ser Tracked, recebido por parameter.
            introspector,
            figure_progress: std::collections::HashMap::new(),
            is_height_unconstrained: false,
            cell_available_h:        None,
            cell_origin_x:           None,
            cell_origin_y:           None,
            cell_origin_w:           None,
            locator:                 Locator::new(),
            current_location:        None,
            runtime:                 crate::entities::layouter_runtime_state::LayouterRuntimeState::default(),
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

    /// **P185C (mecanismo M3 da ADR-0068)** — avança `self.locator` e
    /// actualiza `self.current_location` se `content` for locatable.
    /// Mirror exacto do gating do walk de introspect
    /// (`introspect.rs:329` — `do_extract_payload(content).is_some()`):
    /// invariante `is_locatable ↔ extract_payload.is_some()` (provada
    /// em `locatable.rs:11`) garante sincronização-por-construção das
    /// duas sequências de `Location`s.
    ///
    /// **P204D (M8)** — emit Position single-pass per ADR-0073.
    /// Para cada locatable, popular `runtime.positions` com
    /// `Position { page: pages.len() + 1, point: (cursor_x,
    /// cursor_y) }`. Single canonical site — mirror do gating
    /// que set `current_location`. Idempotência via `insert`.
    fn advance_locator_if_locatable(&mut self, content: &Content) {
        if is_locatable(content) {
            let loc = self.locator.next();
            self.current_location = Some(loc);
            // P204D: emit Position concrete single-pass.
            let page = std::num::NonZeroUsize::new(self.pages.len() + 1)
                .expect("pages.len() + 1 >= 1");
            let point = crate::entities::layout_types::Point {
                x: self.cursor_x,
                y: self.cursor_y,
            };
            self.runtime.positions.insert(
                loc,
                crate::entities::position::Position { page, point },
            );
        }
    }

    pub fn layout_content(&mut self, content: &Content) {
        // P185C: gating Locator atómico no topo, antes do match.
        // Avança em sincronia com walk de introspect; current_location
        // fica disponível para consumers location-aware (P187/P188).
        self.advance_locator_if_locatable(content);

        match content {
            Content::Empty => {}

            // P169 (M9): Metadata é zero-size em layout — sem caixa,
            // sem texto, sem efeito visual. O `value` permanece
            // disponível via `Introspector::query_metadata` para
            // querying do utilizador.
            Content::Metadata { .. } => {}

            // P171 (M9): State e StateUpdate são zero-size em layout.
            // Disponíveis via `Introspector::state_value` /
            // `state_final_value`.
            Content::State { .. } => {}
            Content::StateUpdate { .. } => {}

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
                // P190F (M6 categoria Counters core): Layouter
                // mutação `self.counter.step_hierarchical` removida —
                // counter hierárquico populated via Introspector path
                // location-aware (CounterRegistry P184B + P185B). Layouter
                // só lê via `formatted_counter_at`.
                let _ = level;  // preservar binding para uso abaixo

                let heading_size = self.font_size_pt * heading_scale(*level);
                let prev = self.style.clone();
                self.style = TextStyle { bold: true, italic: false, size: heading_size, ..TextStyle::default() };
                if self.cursor_x.0 > self.page_config.margin { self.flush_line(); }

                // Prefixo numérico — apenas se numbering estiver activo.
                // P190E: location-aware via `is_numbering_active_at`.
                // P190F: fallback legacy `format_hierarchical` removido —
                // Introspector path único.
                use crate::entities::introspector::Introspector;
                let numbering_on = self.current_location
                    .map(|loc| self.introspector
                        .is_numbering_active_at("numbering_active:heading", loc))
                    .unwrap_or(false);
                if numbering_on {
                    let num_str = self.current_location
                        .and_then(|loc| self.introspector
                            .formatted_counter_at("heading", loc));
                    if let Some(num_str) = num_str {
                        let prefix = Content::text(format!("{}. ", num_str));
                        self.layout_content(&prefix);
                    }
                }

                self.layout_content(body);
                self.flush_line();
                self.style = prev;
            }

            Content::SetHeadingNumbering { active: _ } => {
                // P190G (M6 categoria Labels & TOC; Caso 1 `.H`):
                // helper `layout_set_heading_numbering` eliminado —
                // mutava `state.numbering_active` que não existe mais.
                // Caminho Introspector activo via populate_intr arm
                // StateUpdate (chave "numbering_active:heading"). No-op
                // em Layouter.
            }

            Content::SetEquationNumbering { active: _ } => {
                // P190G (M6 categoria Labels & TOC; Caso 1 `.H`):
                // helper `layout_set_equation_numbering` eliminado —
                // análogo a SetHeadingNumbering. No-op em Layouter.
            }

            Content::SetFigureNumbering { .. } => {
                // No-op: numeração baked-in em cada nó Figure (Passo 75, DEBT-14).
            }

            Content::CounterUpdate { key: _, action: _ } => {
                // P190I (M6 fechado): mutação Layouter do counter
                // ELIMINADA — `self.counter` field eliminado. Caminho
                // Introspector activo via populate_intr arm
                // CounterUpdate (P198C); intr.counters é única fonte
                // da verdade. Layouter no-op.
            }

            Content::CounterDisplay { kind } => {
                // P190I (M6 fechado): Layouter consome via Introspector
                // path location-aware. `current_location` set por
                // walk-content para locatable contents (P185C). Para
                // CounterDisplay (não-locatable), usa última location
                // emitida (snapshot até este ponto).
                use crate::entities::introspector::Introspector;
                let text = self.current_location
                    .and_then(|loc| self.introspector.formatted_counter_at(kind, loc))
                    .unwrap_or_else(|| {
                        self.introspector.formatted_counter(kind)
                            .unwrap_or_else(|| "0".to_string())
                    });
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
            // Passo 158C: kind é Option<String>; resolver default "image"
            // em uso (paridade introspect.rs walk arm).
            Content::Figure { body, caption, kind, numbering } => {
                // Calcular o prefixo de numeração antes de chamar layout_figure.
                let caption_prefix: Option<String> = if let Some(_pattern) = numbering {
                    let kind_key = kind.as_deref().unwrap_or("image");
                    let progress = self.figure_progress.entry(kind_key.to_string()).or_insert(0);
                    let idx = *progress;
                    *progress += 1;
                    // P190H (M6 categoria Figures): fallback legacy
                    // `state.figure_numbers` ELIMINADO — field eliminado
                    // de CounterStateLegacy. Caminho Introspector activo
                    // via `figure_number_at_index` (P184C/D); rede de
                    // segurança final `unwrap_or(idx + 1)` preservada
                    // (heurística para edge cases sem populate).
                    use crate::entities::introspector::Introspector;
                    let figure_number = self.introspector
                        .figure_number_at_index(kind_key, idx)
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

            // ── Passo 157A (ADR-0060 Fase 2 sub-passo 1) — table ──
            // **Primeiro sub-passo Model Fase 2**. Delega a `layout_grid`
            // clone simples per ADR-0060 §"Decisão 4" + diagnóstico
            // P157A §10. Sem modificação de `grid.rs`. TableCell
            // estruturado e Header/Footer diferidos para P157B/C.
            Content::Table { columns, rows, children } => {
                self.layout_grid(columns, rows, children);
            }

            // ── Passo 157B (ADR-0060 Fase 2 sub-passo 2) — table cell ──
            // **Segundo sub-passo Model Fase 2**. Renderiza body no
            // contexto actual (single render). `x`/`y`/colspan/rowspan
            // **armazenados mas ignorados** per ADR-0054 graded —
            // algoritmo de placement diferido em DEBT-34e (refactor
            // dedicado a placement Grid completo). Quando dentro de
            // `Content::Table`, cell aparece como child linear no
            // grid distribuído por `idx % num_cols`.
            Content::TableCell { body, x: _, y: _, colspan: _, rowspan: _ } => {
                self.layout_content(body);
            }

            // ── Passo 157C (ADR-0060 Fase 2 sub-passo 3 — fecha table foundations) ──
            // **Terceiro e último sub-passo Model Fase 2**. Par simétrico
            // TableHeader/TableFooter — renderiza body no contexto actual
            // (single render). `repeat` **armazenado mas ignorado** per
            // ADR-0054 graded — algoritmo de repetição em page breaks
            // diferido em DEBT-56 (refactor multi-region; column flow +
            // header/footer repeat). Quando dentro de `Content::Table`,
            // header/footer aparecem como children lineares no grid.
            Content::TableHeader { body, repeat: _ } => {
                self.layout_content(body);
            }
            Content::TableFooter { body, repeat: _ } => {
                self.layout_content(body);
            }

            // ── Passo 159A (ADR-0060 Fase 2 — Bibliography + Cite par acoplado) ──
            // Render placeholder per ADR-0033 + ADR-0054 graded:
            // Bibliography renderiza title (se Some) + lista de entries
            // formatadas como `"[{key}] {author}. {title} ({year})."`;
            // Cite renderiza placeholder `"[{key}]"` + supplement.
            // Refinos futuros (CSL styles, form variants, hayagriva)
            // NÃO reservados per política P158.
            Content::Bibliography { entries, title } => {
                if let Some(t) = title {
                    self.layout_content(t);
                    self.flush_line();
                }
                for e in entries {
                    let line = format_bib_entry(e);
                    self.layout_content(&Content::text(line));
                    self.flush_line();
                }
            }
            Content::Cite { key, supplement, form } => {
                // Passo 159C: render placeholder por form com lookup
                // P190B (M6 categoria Bibliography eliminada) — consumer
                // migrado para Introspector path completo. Fallback legacy
                // a `state.bib_entries`/`bib_numbers` removido porque
                // fields foram eliminados de `CounterStateLegacy`. Caminho
                // Introspector activo desde P181H (BibStore populated via
                // from_tags arm Bibliography). Paridade preservada por
                // construção — output observable inalterado.
                use crate::entities::citation_form::CitationForm;
                use crate::entities::introspector::Introspector;
                let resolved_form = form.unwrap_or_default();
                let entry = self.introspector.bib_entry_for_key(key);
                let text = match (resolved_form, entry) {
                    (CitationForm::Normal, _) => {
                        // P190B: Introspector path apenas — sem fallback legacy.
                        self.introspector
                            .bib_number_for_key(key)
                            .map(|n| format!("[{}]", n))
                            .unwrap_or_else(|| format!("[{}]", key))
                    }
                    (CitationForm::Prose,  Some(e))   => format!("{} ({})", e.author, e.year),
                    (CitationForm::Author, Some(e))   => e.author.clone(),
                    (CitationForm::Year,   Some(e))   => e.year.to_string(),
                    (_, None)                         => format!("[{}]", key),
                };
                self.layout_content(&Content::text(text));
                if let Some(s) = supplement {
                    self.layout_content(s);
                }
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

            // ── Passo 154B (ADR-0060 Fase 1) — terms + divider ──────────────
            Content::Divider => {
                use crate::entities::geometry::Stroke;
                use crate::entities::layout_types::Color;
                if self.cursor_x.0 > self.page_config.margin { self.flush_line(); }
                let margin   = self.page_config.margin;
                let width_pt = self.page_config.width - 2.0 * margin;
                self.current_items.push(FrameItem::Shape {
                    pos:    Point { x: Pt(margin), y: self.cursor_y },
                    kind:   ShapeKind::Line { dx: width_pt, dy: 0.0 },
                    width:  width_pt,
                    height: 0.5,
                    fill:   None,
                    stroke: Some(Stroke { paint: Color::rgb(0, 0, 0), thickness: 0.5 }),
                });
                self.cursor_y += self.font_size_pt * 0.6;
            }

            Content::Terms { items } => {
                if self.cursor_x.0 > self.page_config.margin { self.flush_line(); }
                for item in items {
                    self.layout_content(item);
                }
            }

            Content::TermItem { term, description } => {
                if self.cursor_x.0 > self.page_config.margin { self.flush_line(); }
                let margin_pt = Pt(self.page_config.margin);
                self.cursor_x = margin_pt + self.font_size_pt * 1.5;
                // O termo aparece em negrito — convenção de listas de definições.
                let prev_chain = self.chain.clone();
                let prev_style = self.style.clone();
                use crate::entities::style::{Style, Styles};
                self.chain = self.chain.push_styles(&Styles::from_iter([Style::Bold(true)]));
                self.style = TextStyle::from(&self.chain);
                self.layout_content(term);
                self.chain = prev_chain;
                self.style = prev_style;
                self.layout_content(&Content::text(": "));
                self.layout_content(description);
                self.flush_line();
                self.cursor_x = margin_pt;
            }

            // ── Passo 156C / 156L (ADR-0061 Fase 1 + Fase 3 refino) — pad + hide ──
            Content::Pad { body, sides } => {
                // P156L: cada side é Option<Length>; None ↔ default
                // vanilla zero (resolvido aqui em vez de em native_pad).
                // `right` é scope-out neste passo: o Layouter actual não
                // tem mecânica de "largura útil" por arm — width-aware
                // wrap vive em `flush_line`/`layout_word` que consultam
                // `page_config.width`. Aceitar perfil ADR-0054 graded:
                // pad reduz horizontalmente (left+top) e avança
                // verticalmente (bottom) consistentemente.
                let font = self.font_size_pt.val();
                let left   = sides.left  .map_or(0.0, |l| l.resolve_pt(font));
                let top    = sides.top   .map_or(0.0, |l| l.resolve_pt(font));
                let bottom = sides.bottom.map_or(0.0, |l| l.resolve_pt(font));

                if self.cursor_x.0 > self.line_start_x.0 {
                    self.flush_line();
                }
                self.cursor_y += Pt(top);

                let saved_line_start = self.line_start_x;
                self.line_start_x = saved_line_start + Pt(left);
                self.cursor_x     = self.line_start_x;

                self.layout_content(body);
                self.flush_line();

                self.cursor_y += Pt(bottom);
                self.line_start_x = saved_line_start;
                self.cursor_x     = saved_line_start;
            }

            Content::Hide { body } => {
                // Calcula o avanço sem emitir items (per ADR-0054 graded).
                // Drena items pré-existentes para um buffer temporário,
                // executa o body, e descarta os items gerados — mantém
                // apenas o avanço de cursor.
                let saved_items = std::mem::take(&mut self.current_items);
                let saved_line  = std::mem::take(&mut self.current_line);
                self.layout_content(body);
                self.current_items = saved_items;
                self.current_line  = saved_line;
            }

            // ── Passo 156D (ADR-0061 Fase 1, sub-passo 2) — h + v spacing ──
            // `weak` armazenado mas comportamento de collapse adiado
            // (perfil ADR-0054 graded). Refino futuro se necessário.
            Content::HSpace { amount, weak: _ } => {
                let pt = amount.resolve_pt(self.font_size_pt.val());
                self.cursor_x += Pt(pt);
            }
            Content::VSpace { amount, weak: _ } => {
                let pt = amount.resolve_pt(self.font_size_pt.val());
                // Termina linha em curso se houver content pendente — caso
                // contrário texto na linha actual fica meio-render.
                if self.cursor_x.0 > self.line_start_x.0 {
                    self.flush_line();
                }
                self.cursor_y += Pt(pt);
            }

            // ── Passo 156I (ADR-0061 Fase 2 sub-passo 3) — stack compositivo ──
            // **Último sub-passo Fase 2 (atinge target 72% Layout)**.
            // Container compositivo: itera children + spacing + dir.
            // Structural (força flush_line antes; cada child em "linha"
            // própria para TTB/BTT; inline para LTR/RTL).
            //
            // Implementação simples per ADR-0054 graded: BTT/RTL
            // implementadas como reverse iteration (children[len-1..0])
            // — geometricamente similar a TTB/LTR mas com order
            // visualmente invertido. Refino futuro pode aplicar
            // posicionamento absoluto reverso real (sob forma de
            // FrameItem positioning).
            Content::Stack { children, dir, spacing } => {
                let font = self.font_size_pt.val();
                let space_pt = spacing.map_or(0.0, |l| l.resolve_pt(font));

                // Stack é STRUCTURAL: força flush_line antes.
                if self.cursor_x.0 > self.line_start_x.0 {
                    self.flush_line();
                }

                let n = children.len();
                if n == 0 { return; }

                // Iteração base — forward para LTR/TTB, reverse para
                // RTL/BTT (per ADR-0054 graded; geometria reverse real
                // adiada para refino futuro).
                let iter: Box<dyn Iterator<Item = (usize, &Content)>> = if dir.is_reverse() {
                    Box::new(children.iter().rev().enumerate())
                } else {
                    Box::new(children.iter().enumerate())
                };

                if dir.is_vertical() {
                    // TTB/BTT: layout cada child em "linha" própria;
                    // spacing entre via cursor_y advance.
                    for (i, child) in iter {
                        if i > 0 && space_pt > 0.0 {
                            self.cursor_y += Pt(space_pt);
                        }
                        self.layout_content(child);
                        self.flush_line();
                    }
                } else {
                    // LTR/RTL: layout inline; spacing entre via
                    // cursor_x advance.
                    for (i, child) in iter {
                        if i > 0 && space_pt > 0.0 {
                            self.cursor_x += Pt(space_pt);
                        }
                        self.layout_content(child);
                    }
                    self.flush_line();
                }
            }

            // ── Passo 156H (ADR-0061 Fase 2 sub-passo 2) — box inline container ──
            // Container INLINE: NÃO força flush_line. Aplica inset.left
            // + body + inset.right como avanço de cursor.x na linha
            // actual. width/height/baseline armazenados mas semantic real
            // adiada per ADR-0054 graded (consistente com Block):
            //   - `width`: limitar largura útil exigiria refactor
            //     multi-region (DEBT-56).
            //   - `height` em contexto inline alteraria line_height —
            //     refino futuro.
            //   - `baseline` exige offset vertical mid-linha — não
            //     suportado por cursor.rs actual.
            // `inset.top`/`inset.bottom` em contexto inline são complexos;
            // armazenados mas não aplicados (refino futuro).
            Content::Boxed { body, width, height, inset, baseline } => {
                let font = self.font_size_pt.val();
                let inset_left  = inset.left.resolve_pt(font);
                let inset_right = inset.right.resolve_pt(font);

                // Box é INLINE: avança cursor.x apenas (sem flush_line).
                // Aplica inset_left antes do body.
                self.cursor_x += Pt(inset_left);

                // Layout do body in-place na linha actual.
                let _ = width;    // armazenado; refino futuro
                let _ = height;   // armazenado; refino futuro
                let _ = baseline; // armazenado; refino futuro

                self.layout_content(body);

                // Aplica inset_right após body.
                self.cursor_x += Pt(inset_right);
            }

            // ── Passo 156G (ADR-0061 Fase 2 sub-passo 1) — block container ──
            // Container que ocupa nova "linha lógica" (força flush_line se
            // houver conteúdo pendente), aplica inset (análogo a Pad),
            // reserva altura mínima se `height: Some(h)`, e respeita a
            // largura disponível através do mecanismo `line_start_x`/
            // `flush_line` existente.
            //
            // `breakable` armazenado mas semantic real (impedir quebra
            // mid-block) defere — exigiria refactor multi-region (per
            // DEBT-56). Per ADR-0054 graded.
            //
            // `width` actualmente reduz a largura útil temporariamente
            // (cursor.x começa em line_start_x + offset). `width: None`
            // == auto (largura completa).
            Content::Block { body, width, height, inset, breakable: _ } => {
                let font = self.font_size_pt.val();
                let inset_left   = inset.left.resolve_pt(font);
                let inset_top    = inset.top.resolve_pt(font);
                let inset_bottom = inset.bottom.resolve_pt(font);
                // inset.right é scope-out (mesma razão que Pad.right
                // em P156C — refino com refactor multi-region).

                // 1. Termina linha em curso.
                if self.cursor_x.0 > self.line_start_x.0 {
                    self.flush_line();
                }

                // 2. Captura cursor.y para verificar height mínimo no fim.
                let start_y = self.cursor_y.0;

                // 3. Aplica inset top.
                self.cursor_y += Pt(inset_top);

                // 4. Aplica inset left (e width se especificado).
                let saved_line_start = self.line_start_x;
                self.line_start_x = saved_line_start + Pt(inset_left);
                self.cursor_x     = self.line_start_x;

                // Se width especificado, comportamento simplificado: o body
                // é layouted normalmente. Hard-limiting da largura exigiria
                // refactor do Layouter para multi-region. Aceitar per
                // ADR-0054 graded; documentar como scope-out parcial.
                let _ = width; // armazenado mas não consumido neste passo

                // 5. Layout do body.
                self.layout_content(body);
                self.flush_line();

                // 6. Aplica inset bottom.
                self.cursor_y += Pt(inset_bottom);

                // 7. Se height: Some(h), garantir que avançámos pelo menos h.
                if let Some(h) = height {
                    let h_pt = h.resolve_pt(font);
                    let consumed = self.cursor_y.0 - start_y;
                    if consumed < h_pt {
                        self.cursor_y += Pt(h_pt - consumed);
                    }
                }

                // 8. Restaura line_start_x.
                self.line_start_x = saved_line_start;
                self.cursor_x     = saved_line_start;
            }

            // ── Passo 156J (ADR-0061 Fase 3 sub-passo 1) — repeat ──
            // **Primeira aplicação Fase 3**. Variant + paridade
            // estrutural (single-render do body no contexto actual).
            // Algoritmo dinâmico de quantidade-para-encher (vanilla
            // calcula floor(available / (body_width + gap))) está
            // diferido per ADR-0054 graded — exige refactor inline-
            // region não disponível no Layouter actual (mesma razão
            // que `Block.width`/`Boxed.width` em P156G/H).
            //
            // `gap` armazenado mas não emite spacing entre cópias
            // (só uma cópia neste passo). `justify` armazenado mas
            // sem distribuição de espaço residual (idem).
            Content::Repeat { body, gap: _, justify: _ } => {
                // Layout single-render: emite o body uma vez no
                // contexto actual. Suficiente para paridade estrutural
                // (variant disponível em todo o pipeline) e para que
                // counters/labels dentro do body resolvam via walk.
                self.layout_content(body);
            }

            // ── Passo 156E (ADR-0061 Fase 1, sub-passo 3) — pagebreak ──
            // `weak` armazenado mas collapse defere (consistente P156D).
            // Layouter reusa `new_page` (cursor.rs:128) que commits items
            // actuais a Page e reseta cursor.
            Content::Pagebreak { weak: _, to } => {
                // 1. Termina linha em curso (caso contrário fica meio-render).
                if self.cursor_x.0 > self.line_start_x.0 {
                    self.flush_line();
                }
                // 2. Força nova página (mesmo se actual está vazia — vanilla
                //    pagebreak() é "event" sempre observável).
                self.new_page();
                // 3. Se `to` exige paridade específica, verifica; se não bate,
                //    insere página vazia adicional para ajustar.
                if let Some(parity) = to {
                    let next_page_number = self.pages.len() + 1;
                    if !parity.matches(next_page_number) {
                        self.new_page();
                    }
                }
            }

            // ── Passo 155 (ADR-0060 Fase 1, sub-passo 2) — quote ───────────
            Content::Quote { body, attribution, block, quotes } => {
                use crate::rules::lang::quotes::{DEFAULT_QUOTES, localize_quotes};
                let lang = self.chain.lang();
                let (open, close) = if *quotes {
                    match &lang {
                        Some(l) => localize_quotes(l),
                        None    => DEFAULT_QUOTES,
                    }
                } else {
                    ("", "")
                };
                if *block {
                    if self.cursor_x.0 > self.page_config.margin { self.flush_line(); }
                    let margin_pt = Pt(self.page_config.margin);
                    self.cursor_x = margin_pt + self.font_size_pt * 1.5;
                    if !open.is_empty() {
                        self.layout_content(&Content::text(open));
                    }
                    self.layout_content(body);
                    if !close.is_empty() {
                        self.layout_content(&Content::text(close));
                    }
                    if let Some(a) = attribution {
                        self.flush_line();
                        self.cursor_x = margin_pt + self.font_size_pt * 1.5;
                        self.layout_content(&Content::text("— "));
                        self.layout_content(a);
                    }
                    self.flush_line();
                    self.cursor_x = margin_pt;
                } else {
                    if !open.is_empty() {
                        self.layout_content(&Content::text(open));
                    }
                    self.layout_content(body);
                    if !close.is_empty() {
                        self.layout_content(&Content::text(close));
                    }
                    if let Some(a) = attribution {
                        self.layout_content(&Content::text(" — "));
                        self.layout_content(a);
                    }
                }
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
        // P190C (M6 categoria Page tracking): label_pages movido para
        // LayouterRuntimeState.
        doc.extracted_label_pages = self.runtime.label_pages;
        // P205B (F3): sealing point — extrai runtime.positions para
        // sub-store sealed `SealedPositions` per ADR-0074. Tracked
        // via comemo; consumer migration em P205C.
        doc.extracted_positions = crate::entities::sealed_positions::SealedPositions::from_runtime(
            self.runtime.positions,
        );
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

            // Passo 156C / 156L: Pad / Hide para grid measurement.
            // P156L: cada side é Option<Length>; None ↔ zero.
            Content::Pad { body, sides } => {
                let font  = self.font_size_pt.val();
                let left   = sides.left  .map_or(0.0, |l| l.resolve_pt(font));
                let right  = sides.right .map_or(0.0, |l| l.resolve_pt(font));
                let top    = sides.top   .map_or(0.0, |l| l.resolve_pt(font));
                let bottom = sides.bottom.map_or(0.0, |l| l.resolve_pt(font));
                let constrained = (max_width - left - right).max(0.0);
                let (w, h) = self.measure_content_constrained(body, constrained);
                (w + left + right, h + top + bottom)
            }
            Content::Hide { body } => {
                self.measure_content_constrained(body, max_width)
            }

            // Passo 156D: HSpace/VSpace dimensões para grid measurement.
            Content::HSpace { amount, .. } => {
                (amount.resolve_pt(self.font_size_pt.val()), 0.0)
            }
            Content::VSpace { amount, .. } => {
                (0.0, amount.resolve_pt(self.font_size_pt.val()))
            }

            // Passo 156E: Pagebreak é event sem dimensões dentro de cell.
            // Em grid measurement, ignora-se (não consome largura/altura).
            Content::Pagebreak { .. } => (0.0, 0.0),

            // Passo 156I: Stack dimensões para grid measurement.
            // TTB/BTT: max widths; sum heights + (n-1) * spacing.
            // LTR/RTL: sum widths + (n-1) * spacing; max heights.
            Content::Stack { children, dir, spacing } => {
                let font = self.font_size_pt.val();
                let space_pt = spacing.map_or(0.0, |l| l.resolve_pt(font));
                let n = children.len();
                if n == 0 { return (0.0, 0.0); }

                if dir.is_vertical() {
                    let mut max_w = 0.0_f64;
                    let mut sum_h = 0.0_f64;
                    for child in children.iter() {
                        let (w, h) = self.measure_content_constrained(child, max_width);
                        max_w = max_w.max(w);
                        sum_h += h;
                    }
                    let total_h = sum_h + ((n - 1) as f64) * space_pt;
                    (max_w, total_h)
                } else {
                    let mut sum_w = 0.0_f64;
                    let mut max_h = 0.0_f64;
                    for child in children.iter() {
                        let (w, h) = self.measure_content_constrained(child, max_width);
                        sum_w += w;
                        max_h = max_h.max(h);
                    }
                    let total_w = sum_w + ((n - 1) as f64) * space_pt;
                    (total_w, max_h)
                }
            }

            // Passo 156H: Boxed (Box inline) dimensões para grid
            // measurement. Análogo a Block (mesma lógica width/height/
            // inset; baseline ignorado em medição).
            Content::Boxed { body, width, height, inset, baseline: _ } => {
                let font = self.font_size_pt.val();
                let inset_l = inset.left.resolve_pt(font);
                let inset_r = inset.right.resolve_pt(font);
                let inset_t = inset.top.resolve_pt(font);
                let inset_b = inset.bottom.resolve_pt(font);
                let body_max = match width {
                    Some(w) => w.resolve_pt(font).min(max_width - inset_l - inset_r),
                    None    => (max_width - inset_l - inset_r).max(0.0),
                };
                let (bw, bh) = self.measure_content_constrained(body, body_max);
                let total_w = bw + inset_l + inset_r;
                let body_h_with_inset = bh + inset_t + inset_b;
                let total_h = match height {
                    Some(h) => h.resolve_pt(font).max(body_h_with_inset),
                    None    => body_h_with_inset,
                };
                (total_w, total_h)
            }

            // Passo 156J: Repeat dimensões para grid measurement.
            // Single-render do body (consistente com layout_content
            // arm). Algoritmo dinâmico de quantidade defere per
            // ADR-0054 graded.
            Content::Repeat { body, .. } => {
                self.measure_content_constrained(body, max_width)
            }

            // Passo 156G: Block dimensões para grid measurement.
            // Inset adiciona aos lados; height: Some(h) força mínimo;
            // width: Some(w) prefere essa largura mas constrained por max.
            Content::Block { body, width, height, inset, breakable: _ } => {
                let font = self.font_size_pt.val();
                let inset_l = inset.left.resolve_pt(font);
                let inset_r = inset.right.resolve_pt(font);
                let inset_t = inset.top.resolve_pt(font);
                let inset_b = inset.bottom.resolve_pt(font);
                let body_max = match width {
                    Some(w) => w.resolve_pt(font).min(max_width - inset_l - inset_r),
                    None    => (max_width - inset_l - inset_r).max(0.0),
                };
                let (bw, bh) = self.measure_content_constrained(body, body_max);
                let total_w = bw + inset_l + inset_r;
                let body_h_with_inset = bh + inset_t + inset_b;
                let total_h = match height {
                    Some(h) => h.resolve_pt(font).max(body_h_with_inset),
                    None    => body_h_with_inset,
                };
                (total_w, total_h)
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
/// Recebe o `CounterStateLegacy` produzido por `introspect::introspect`.
/// Se o documento não contiver `Content::Outline` (`has_outline = false`),
/// corre uma única passagem — o fixpoint de páginas só serve a TOC.
/// Caso contrário, itera até convergência (máximo 5 vezes).
///
/// Para métricas de fonte reais: `03_infra::layout::layout_with_font()`.
/// Helper privado P159D + P159E + P159G — formata `BibEntry`
/// para render Bibliography. Concatenação condicional dos
/// fields opcionais quando presentes; backwards compat preserva
/// formato P159E quando todos os 6 fields P159G são `None`.
///
/// Ordem APA-like extendida (decisões diagnósticos P159D §10 +
/// P159E §8.2 + P159G §8.2):
/// `[key] author. title (Ed. editor) (series) journal vol. volume,`
/// `pp. pages. location: publisher (year). isbn:XXX url, doi:YYY [note].`
///
/// **P159G**: editor/series após title; location antes de
/// publisher; organization substitutivo a publisher quando
/// publisher ausente; isbn antes de url/doi; note ao final.
fn format_bib_entry(e: &crate::entities::bib_entry::BibEntry) -> String {
    let mut out = format!("[{}] {}. {}", e.key, e.author, e.title);
    // P159G — editor/series após title.
    if let Some(ed) = &e.editor    { out.push_str(&format!(" (Ed. {})", ed)); }
    if let Some(se) = &e.series    { out.push_str(&format!(" ({})", se)); }
    // P159D — journal/volume/pages.
    if let Some(j)  = &e.journal   { out.push_str(&format!(" {}", j)); }
    if let Some(v)  = &e.volume    { out.push_str(&format!(" vol. {}", v)); }
    if let Some(p)  = &e.pages     { out.push_str(&format!(", pp. {}", p)); }
    // P159G — location antes de publisher; organization substitutivo
    // a publisher quando publisher ausente.
    let pub_slot: Option<String> = match (&e.publisher, &e.organization) {
        (Some(pb), _)    => Some(pb.clone()),
        (None, Some(o))  => Some(o.clone()),
        (None, None)     => None,
    };
    match (&e.location, &pub_slot) {
        (Some(l), Some(pb)) => out.push_str(&format!(". {}: {}", l, pb)),
        (Some(l), None)     => out.push_str(&format!(". {}", l)),
        (None,    Some(pb)) => out.push_str(&format!(". {}", pb)),
        (None,    None)     => {}
    }
    out.push_str(&format!(" ({}).", e.year));
    // P159G — isbn antes de url/doi.
    if let Some(i)  = &e.isbn      { out.push_str(&format!(" isbn:{}", i)); }
    // P159E — par natural url/doi após (year). per Opção C.
    match (&e.url, &e.doi) {
        (Some(u), Some(d)) => out.push_str(&format!(" {}, doi:{}.", u, d)),
        (Some(u), None)    => out.push_str(&format!(" {}.", u)),
        (None,    Some(d)) => out.push_str(&format!(" doi:{}.", d)),
        (None,    None)    => {
            // Fechar com `.` se isbn presente sem url/doi.
            if e.isbn.is_some() { out.push('.'); }
        }
    }
    // P159G — note ao final.
    if let Some(n)  = &e.note      { out.push_str(&format!(" [{}]", n)); }
    out
}

pub fn layout(content: &Content) -> PagedDocument {
    // P190I (M6 fechado): `initial_state: CounterStateLegacy` parameter
    // ELIMINADO — struct eliminada. layout() corre
    // `introspect_with_introspector` internamente para obter
    // `TagIntrospector` populated. API breaking change comparada
    // com versões anteriores; callers externos adaptados.
    let intr = crate::rules::introspect::introspect_with_introspector(
        content,
    );
    layout_with_introspector(content, intr)
}

/// Entry point P168 (M5 sub-passo 2): aceita `TagIntrospector` adicional
/// para que consumers como `references.rs::layout_ref` (figure-ref) possam
/// usar `query_by_label` em vez de `state.figure_label_numbers` legacy.
///
/// Caller típico:
/// ```ignore
/// let intr = introspect_with_introspector(&content);
/// let doc = layout_with_introspector(&content, intr);
/// ```
///
/// **P190I (M6 fechado)**: signature drop `initial_state:
/// CounterStateLegacy` parameter — struct eliminada.
pub fn layout_with_introspector(
    content: &Content,
    introspector: crate::entities::introspector::TagIntrospector,
) -> PagedDocument {
    use std::collections::HashMap;
    use crate::entities::label::Label;

    // ── Short-circuit: sem TOC, não há necessidade de fixpoint ──────────────
    // A condição correcta é "tem Content::Outline?", não
    // `headings_for_toc.is_empty()`. Um documento com títulos mas sem
    // `#outline()` não precisa do ciclo.
    //
    // P189B (M5): walk puro — flag obtida via Introspector
    // (`kind_index[Outline]` populado por `from_tags` P178) em vez de
    // `state.has_outline` (mutação removida em `introspect.rs:610`).
    // Field `CounterStateLegacy::has_outline` fica morto; cleanup em M6.
    use crate::entities::element_kind::ElementKind;
    let has_outline = introspector.kind_index.contains_key(&ElementKind::Outline);

    // P204C (M8): construir Tracked uma vez. introspector é binding
    // local (owned por valor desde signature) e outlive todos os
    // Layouters criados abaixo (single-pass ou fixpoint loop).
    use comemo::Track;
    let intr_dyn: &dyn crate::entities::introspector::Introspector = &introspector;
    let intr_tracked = intr_dyn.track();

    if !has_outline {
        let mut l = Layouter::new(FixedMetrics, NullImageSizer, DEFAULT_FONT_SIZE, intr_tracked);
        // P204C (M8): introspector já fornecido a Layouter::new via
        // tracked. Mutações pós-construção (`l.introspector =
        // introspector`) eliminadas porque Tracked é borrow.
        // P190G (M6 categoria Labels & TOC eliminada) + restantes
        // limpezas mantidas — sem trabalho aqui.
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
        let mut l = Layouter::new(FixedMetrics, NullImageSizer, DEFAULT_FONT_SIZE, intr_tracked);

        // P204C (M8): assignment `l.introspector = introspector.clone()`
        // eliminado — Tracked partilhado entre iterações via construtor.
        // Tracked é Copy; cada iteração reusa o mesmo handle.
        // P190G (M6 categoria Labels & TOC eliminada): assignments
        // `resolved_labels`/`headings_for_toc` removidos — fields já
        // não existem. Layouter consumers (`references.rs:64`,
        // `outline.rs:38`) migrados para Introspector path puro.
        // P190E/P190G: assignment `numbering_active` removido — field
        // eliminado em P190G Caso 1.
        // P190B: assignments bib_* removidos — fields já não existem.

        // Injectar páginas da iteração anterior para leitura pelo outline.rs.
        // label_pages (onde references.rs escreve) começa vazio via Layouter::new().
        // P190C (M6 categoria Page tracking): known_page_numbers movido
        // para LayouterRuntimeState.
        l.runtime.known_page_numbers = known_page_numbers.clone();

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
