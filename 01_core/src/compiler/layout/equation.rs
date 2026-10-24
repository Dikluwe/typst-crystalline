//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/equation.md
//! @prompt-hash e5db1252
//! @layer L1
//! @updated 2026-10-06
//!
//! Braço `Content::Equation` do `layout_content`. Extraído de `layout/mod.rs`
//! no Passo 96.7 conforme ADR-0037. P456: numeração de bloco formatada pelo
//! pattern da chain e posicionada à direita da página.
//! P944: o `math_style` fixa `font = New Computer Modern Math` e
//! `weight = 450` para toda a equação (show_set do vanilla,
//! equation.rs:197-201) — o `#set text(font:)` do documento não se aplica
//! dentro de equações.
#![allow(deprecated)] // P483 — FrameItem::Text fallback path legítimo
use ecow::EcoString;

use crate::compiler::math;
use crate::entities::{
    content::Content,
    counter::CounterKey,
    counter_format::format_counter,
    element_kind::ElementKind,
    elements::equation::EquationElem,
    font_list::FontList,
    image_sizer::ImageSizer,
    layout_types::{
        Align2D, FrameItem, HAlign, MathSize, Point, Pt, SemanticKind, SemanticPlacement,
        TextEdge, TextStyle, VAlign,
    },
    selector::Selector,
};

use super::metrics::FontMetrics;

impl<'a, M: FontMetrics, S: ImageSizer> super::Layouter<'a, M, S> {
    /// Layout de `Content::Equation { body, block }`.
    /// `numbering_active`: gate tipado vindo da chain.
    pub(super) fn layout_equation(
        &mut self,
        body: &Content,
        block: bool,
        numbering_active: bool,
    ) {
        // **P751** — fixar a baseline inicial com o estilo activo antes de
        // posicionar texto/equação real.
        // **P813** — capturar a flag ANTES do ensure: equações de bloco no
        // topo da página/região suprimem o spacing acima (paridade vanilla,
        // medido: `$x^2$` sozinho → baseline = margin + ascent).
        let was_initial_baseline_pending = self.initial_baseline_pending;
        // Auto-numeração: equações de bloco numeradas avançam o contador antes de
        // desenhar (Passo 59). O número (N) é acrescentado depois da equação.
        // Lote F-2 S2 (P335): o "ativo" é **assado** no `EquationElem` (escopo
        // léxico via chain, fecha o canal global StateRegistry). O **valor** do
        // contador continua via Introspector (`flat_counter_at`), gateado pelo
        // mesmo `numbering_active` assado (via payload, em `from_tags`).
        // P456: o pattern vem da chain como `Value::Str`; ausência = não numerada.
        let is_numbered = block && numbering_active;
        let number_align = match self.chain.custom("equation.number-align") {
            Some(crate::entities::value::Value::Align(align)) => *align,
            _ => Align2D { h: Some(HAlign::End), v: Some(VAlign::Horizon) },
        };
        // P190F (M6 categoria Counters core): Layouter mutação
        // `self.counter.step_flat` removida — counter equation
        // populated via Introspector path (CounterRegistry +
        // gate em `from_tags` arm Equation P186E activado por
        // SetEquationNumbering P199B). Layouter só lê.

        // **P784** — marca `math: true` uma única vez, no ponto de entrada do
        // motor de layout matemático; herdado por `..style.clone()` em toda a
        // árvore de layout math (`layout_text_node` e os restantes sub-layouts
        // nunca tocam `.font`, só `.italic`/`.size`). Consumido em L3
        // (`shaper.rs`) para engatar sempre a cadeia de fallback matemática —
        // não só quando a fonte já resolvida coincidentemente tem tabela MATH
        // (a fonte de corpo por omissão, `Libertinus Serif`, não tem).
        // **P944** — o mesmo ponto único fixa `font`/`weight` de toda a
        // equação (inline e bloco), replicando o `EquationElem::show_set` do
        // vanilla (`lab/typst-original/crates/typst-library/src/math/
        // equation.rs:197-201`): `TextElem::font = FontList(["New Computer
        // Modern Math"])` e `TextElem::weight = 450`. O `#set text(font:)`
        // do documento **não** se aplica dentro de equações — sem este
        // override, a fonte de texto "New Computer Modern" (tabela MATH
        // stub: `LowerLimitGapMin = 0`, sem `MathVariants`) tornava-se a
        // primária e os delimitadores não esticavam / limites colidiam.
        // Nós `MathStyled` (`mono`/`serif`/`sans`/`upright`/`bold`) não
        // tocam `style.font` — actuam por transformação de codepoints
        // Unicode (`apply_math_style`) e factores de tamanho — pelo que este
        // override não os afecta (e eles não o afectam).
        // **P945** — o mesmo ponto único fixa também o nível MathSize
        // discreto do vanilla (`EquationElem::size` = Display/Text conforme
        // `block`, `lab/typst-original/crates/typst-library/src/math/
        // equation.rs:189-195`) — consumido pelas descidas de nível
        // (`denominator_style` em matrizes/casos, scripts, fracções). Ver
        // `compiler/layout/equation.md` §P945.
        let math_style = TextStyle {
            math: true,
            math_size: if block { MathSize::Display } else { MathSize::Text },
            font: Some(FontList::single(EcoString::from("New Computer Modern Math"))),
            weight: Some(450),
            ..self.style.clone()
        };
        // **P893** — `&math_style` propagado para que `FallbackFontMetrics`
        // resolva as constantes MATH reais da fonte activa, em vez de
        // `MathConstants::fallback()` incondicional.
        let equation_location = self
            .current_location
            .expect("equation layout must have a stable location");
        // Helpers and older unit tests may construct `Layouter` directly,
        // outside the callback-aware document entrypoint. They still need a
        // pure transcript for callback-free equations; production always
        // installs the attempt-wide state before reaching this point.
        let saved_chain = self.block_chain_active;
        let saved_below = self.prev_block_below_pending;
        let saved_prev_descent = self.prev_block_equation_descent;
        let planned_frame = self.planned_equation_body_cursor().map(|cursor| {
            let content = math::layout::prepare_direct_body(body);
            let saved_style = self.style.clone();
            let saved_lexical_chain = self.chain.clone();
            self.chain =
                self.chain.push_styles(&math::layout::external_styles(&math_style));
            self.style = math_style.clone();
            let (height, items, _, _, _) = self.layout_sub_frame_planned(
                &content,
                super::SubLayoutRegion {
                    origin_x: 0.0,
                    width: f64::INFINITY,
                    height: None,
                    align_rtl: false,
                    unconstrained_height: true,
                },
                false,
                Some(cursor),
            );
            let width = self.last_sub_frame_width;
            self.chain = saved_lexical_chain;
            self.style = saved_style;
            math::layout::PlannedExternalFrame { content, height, width, items }
        });
        let fallback_callback_pass =
            math::layout::callbacks::MathCallbackPassState::new(None);
        let callback_pass = self.math_callback_pass.unwrap_or(&fallback_callback_pass);
        let math_layouter = math::layout::MathLayouter::new_with_context(
            &self.metrics,
            block,
            &math_style,
            equation_location,
            Pt(self.regions.effective().height),
            &self.chain,
            callback_pass,
        );
        // **P813** — equações de bloco precisam da extensão geométrica
        // (largura + ascent/descent de tinta) para centragem e espaçamento;
        // os items são os mesmos de `layout_equation`.
        // **P1108** — Preservar o estado de colapso de margens antes da medição matemática
        // (que pode rodar sub_frame/layout_node interno).

        let planned_external = planned_frame.is_some();
        let (math_items, extent) = {
            let (items, mut ext) = match planned_frame {
                Some(frame) => {
                    math_layouter.layout_equation_from_external(frame, &math_style)
                }
                None => math_layouter.layout_equation_measured(body, &math_style),
            };
            if block && numbering_active {
                let (top_edge, _) = self.metrics.text_edges(math_style.size, &math_style);
                ext.ascent = ext.ascent.max(top_edge.0);
            }
            (items, Some(ext))
        };

        self.block_chain_active = saved_chain;
        self.prev_block_below_pending = saved_below;
        self.prev_block_equation_descent = saved_prev_descent;

        // P1292 amendment-3 — the raw math extent is normalized to the
        // paragraph frame before it reaches the canonical line aggregator.
        // Bounds edges are the frame's own bounds; all other edges retain the
        // typed TextStyle/FontMetrics resolution. Block equations keep using
        // the unmodified raw extent below.
        if !block {
            let ext = extent.as_ref().expect("equation extent is always measured");
            let leading = math_style
                .leading
                .as_ref()
                .map(|length| length.resolve_pt(math_style.size.val()))
                .unwrap_or(math_style.size.val() * super::vanilla_defaults::PAR_LEADING);
            let slack = leading * 0.7;
            let (top_edge, bottom_edge) =
                self.metrics.text_edges(math_style.size, &math_style);
            let top_is_bounds = matches!(
                math_style.top_edge.as_ref(),
                Some(TextEdge::Metric(name)) if name.as_str() == "bounds"
            );
            let bottom_is_bounds = matches!(
                math_style.bottom_edge.as_ref(),
                Some(TextEdge::Metric(name)) if name.as_str() == "bounds"
            );
            let edge_ascent = if top_is_bounds { ext.ascent } else { top_edge.0 };
            let edge_descent =
                if bottom_is_bounds { ext.descent } else { -bottom_edge.0 };
            let inline_ascent = edge_ascent.max(ext.ascent - slack);
            let inline_descent = edge_descent.max(ext.descent - slack).max(0.0);
            self.ensure_line_baseline(inline_ascent);
            self.note_inline_extent(inline_ascent, inline_descent);
        }

        // **P896** — `Some(largura_da_equação)` quando a centragem teve de
        // ser adiada (`width: auto`, valor ainda infinito neste ponto).
        let mut pending_center_width: Option<f64> = None;
        let mut offset_x = self.regions.current.cursor_x;

        if block {
            let ext = extent.expect("bloco tem extent medido (P813)");
            // **P813** / **P1087** — spacing vertical de bloco 1.2em (BLOCK_SPACING)
            // acima e abaixo (paridade vanilla BlockElem::above/below default —
            // lab/typst-original/crates/typst-library/src/layout/container.rs:342).
            let spacing =
                Pt(self.style.size.val() * super::vanilla_defaults::BLOCK_SPACING);
            if self.is_sub_frame
                && !super::html_frame::items_have_material(
                    &self.regions.current.current_items,
                )
                && !super::html_frame::items_have_material(
                    &self.regions.current.current_line,
                )
            {
                // Topo do sub-frame: baseline = ascender / ext.ascent
                self.regions.current.cursor_y = Pt(ext.ascent);
                self.prev_block_below_pending = 0.0;
                self.initial_baseline_pending = false;
            } else if was_initial_baseline_pending
                || (!self.is_sub_frame
                    && !self.column_mode
                    && self.page_columns.is_none()
                    && super::pagebreak::page_is_empty(self)
                    && self.flow_placement_end.is_none()
                    && !self.block_chain_active)
            {
                // Topo da página: baseline = margin + ext.ascent
                self.regions.current.cursor_y =
                    Pt(self.page_config.margin.top + ext.ascent);
                self.prev_block_below_pending = 0.0;
                self.initial_baseline_pending = false;
            } else {
                let pages_before = self.pages.len();
                let prev_baseline = if super::html_frame::items_have_material(
                    &self.regions.current.current_line,
                ) {
                    let b = self.regions.current.cursor_y.0;
                    self.flush_line();
                    b
                } else {
                    self.prev_line_baseline
                };
                if self.pages.len() == pages_before {
                    // **P1088/P1108/P1121** — Protocolo de colapso de margens nominal:
                    // - Se vier de Heading (prev_block_equation_descent == 0 e prev_block_below_pending > 0),
                    //   usa o gap colapsado do Heading (8.129pt).
                    // - Se vier de outra equação de bloco (ou se a cadeia de bloco estiver activa entre equações),
                    //   o espaçamento de bloco nominal é SEMPRE BLOCK_SPACING = 1.2em (13.2pt para fonte 11pt).
                    // **P1088/P1108/P1124** — Colapso de margens de bloco:
                    // - Se vier de Heading (prev_block_equation_descent == 0 e prev_block_below_pending > 0),
                    //   o gap colapsado é o below do Heading (8.25pt/8.129pt per P1063/P1108).
                    // - Se vier de outra equação de bloco, o gap nominal é BLOCK_SPACING (13.2pt).
                    let is_heading_preceding = self.block_chain_active
                        && self.prev_block_equation_descent == 0.0
                        && self.prev_block_below_pending > 0.0;
                    let gap = if is_heading_preceding {
                        self.prev_block_below_pending
                    } else {
                        spacing.val()
                    };
                    let prev_descent = self.prev_block_equation_descent;

                    // Avanço nominal exato de equação de bloco
                    self.regions.current.cursor_y =
                        Pt(prev_baseline + prev_descent + gap + ext.ascent);
                }
            }
            // The block baseline already incorporates its raw ascent. Hand
            // that same extent to whole-line admission before any migration.
            self.line_assumed_ascent = Some(ext.ascent);
            self.note_inline_extent(ext.ascent, ext.descent);
            self.prev_line_baseline = self.regions.current.cursor_y.0;

            // **P1102** — A posição horizontal da equação de bloco deve ser capturada
            // APÓS o `flush_line()` acima fechar a linha anterior.
            // Para equações de bloco, a linha começa em `line_start_x` (a margem).
            offset_x = self.regions.current.cursor_x;

            // **P813** — centragem horizontal na região (paridade vanilla
            // ShowSet `align(center)` para equações de bloco —
            // lab/typst-original/crates/typst-library/src/math/equation.rs:190).
            // Sem clamp: equação mais larga que a região sangra centrada,
            // como no vanilla (`position(size - width)` com center).
            //
            // **P895** — `regions.current.width` é `f64::INFINITY` quando
            // `#set page(width: auto)` está activo (o valor final só é
            // resolvido depois, em `compute_page_width()`, a partir do
            // conteúdo já colocado). Centrar contra uma largura ainda por
            // resolver não tem resposta bem definida — em vez de propagar
            // infinito (que corrompia a `MediaBox` exportada, achado de
            // `typst-passo-894-relatorio.md`), a equação fica encostada à
            // margem (sem offset de centragem), paridade com o comportamento
            // observado no vanilla para o caso de uma única equação de bloco
            // com `width: auto` (o próprio conteúdo define a largura da
            // página, logo `usable == largura da equação` e a centragem
            // degenera para offset zero de qualquer forma).
            if self.regions.current.width.is_finite() {
                let usable =
                    self.regions.current.width - self.page_config.margin.horizontal();
                // rationale: P1064 Classe 1A — centragem de equação em bloco ((usable - ext.width) / 2.0)
                offset_x = Pt(self.page_config.margin.left + (usable - ext.width) / 2.0);
            } else {
                // **P896** — largura ainda não resolvida: registar para
                // correcção adiada em `finish()`/`new_page()`, quando a
                // largura final da página for conhecida (ver
                // `pending_equation_centering`, `layout/mod.rs`).
                pending_center_width = Some(ext.width);
            }
        }

        // Integrar items matemáticos no frame actual.
        // Os items vêm com posições relativas à **baseline** da fórmula
        // (y = 0 na baseline; sup/sub deslocados a partir dela).
        // **P800** — Equações inline: a baseline da fórmula coincide com a
        // baseline do texto circundante (paridade vanilla medida por
        // `mutool trace` — texto e math partilham o mesmo y). Basta somar
        // `cursor_y`. A regra anterior (Passo 48: deslocar por `axis_pt`
        // para "alinhar o eixo matemático à baseline") deslocava a fórmula
        // ~0.5em para cima do texto — refutada por medição. O eixo só
        // governa o centrado interno (frac/delims/root, `apply_axis_offset`).
        // Bloco (P813): `cursor_y` foi posicionado acima em
        // `baseline_anterior + spacing + ascent_ink`.
        let offset_y = self.regions.current.cursor_y;
        // **P896** — capturado antes do loop para saber exactamente quantos
        // items esta equação empurra para `current_line` (nem todos os
        // arms do match abaixo empurram um item — `TextShaped`/`Image`/
        // `Link` são no-ops em modo math inline; `Shape`/`Group` empurram
        // desde P994, conteúdo externo embutido por `layout_external`) e a
        // partir de que índice esses items vão ficar em `current_items`
        // depois do `flush_line()` (que só acontece mais abaixo, `if block`).
        let current_line_len_before_eq = self.regions.current.current_line.len();
        for item in math_items {
            match item {
                FrameItem::TextReplacement { text, mut items } => {
                    for child in &mut items {
                        super::helpers::offset_frame_item(child, offset_x.0, offset_y.0);
                    }
                    let envelope = FrameItem::TextReplacement { text, items };
                    let extent = super::helpers::item_pos(&envelope).0
                        + super::helpers::item_width(&envelope, &self.metrics);
                    self.regions.current.cursor_x =
                        Pt(self.regions.current.cursor_x.0.max(extent));
                    self.regions.current.current_line.push(envelope);
                }
                FrameItem::TextSeparator { pos, advance, style, origin, bidi } => {
                    self.regions.current.current_line.push(FrameItem::TextSeparator {
                        pos: Point { x: offset_x + pos.x, y: offset_y + pos.y },
                        advance,
                        style,
                        origin,
                        bidi,
                    })
                }
                FrameItem::Text { pos, text, style, bidi } => {
                    let abs_pos = Point { x: offset_x + pos.x, y: offset_y + pos.y };
                    let advance = self.metrics.advance(&text, style.size, &style);
                    self.regions.current.current_line.push(FrameItem::Text {
                        pos: abs_pos,
                        text,
                        style,
                        bidi,
                    });
                    // **P967** — o cursor acompanha o EXTENT real (pos.x +
                    // advance, que já inclui o espaçamento de classe
                    // embutido nas posições pela `compute_gaps`, P772y),
                    // não a soma simples de advances — sem isto, texto após
                    // equação inline com relações ficava ~gaps antes do fim
                    // real e SOBREPUNHA o fim da equação (medido:
                    // `$3x + y = 9$ dado`). Ver `equation.md` §P967.
                    let extent_x = abs_pos.x + advance;
                    if extent_x > self.regions.current.cursor_x {
                        self.regions.current.cursor_x = extent_x;
                    }
                }
                FrameItem::Line { start, end, thickness, color, cap } => {
                    let abs_start =
                        Point { x: offset_x + start.x, y: offset_y + start.y };
                    let abs_end = Point { x: offset_x + end.x, y: offset_y + end.y };
                    self.regions.current.current_line.push(FrameItem::Line {
                        start: abs_start,
                        end: abs_end,
                        thickness,
                        // P285: equação preserva cor da Line original (math
                        // frac/sqrt usam None → preto bit-exact).
                        color,
                        cap,
                    });
                }
                FrameItem::TextShaped {
                    pos,
                    text,
                    style,
                    glyphs,
                    units_per_em,
                    bidi,
                    font,
                } => {
                    let abs_pos = Point { x: offset_x + pos.x, y: offset_y + pos.y };
                    let advance = glyphs
                        .iter()
                        .map(|g| {
                            g.x_advance as f64 / units_per_em as f64 * style.size.val()
                        })
                        .sum::<f64>();
                    self.regions.current.current_line.push(FrameItem::TextShaped {
                        pos: abs_pos,
                        text,
                        style,
                        glyphs,
                        units_per_em,
                        bidi,
                        font,
                    });
                    let extent_x = abs_pos.x + Pt(advance);
                    if extent_x > self.regions.current.cursor_x {
                        self.regions.current.cursor_x = extent_x;
                    }
                }
                FrameItem::Glyph { pos, glyph_id, x_advance, size, style, base_char } => {
                    let abs_pos = Point { x: offset_x + pos.x, y: offset_y + pos.y };
                    self.regions.current.current_line.push(FrameItem::Glyph {
                        pos: abs_pos,
                        glyph_id,
                        x_advance,
                        size,
                        style,
                        base_char,
                    });
                    let extent_x = abs_pos.x + x_advance;
                    if extent_x > self.regions.current.cursor_x {
                        self.regions.current.cursor_x = extent_x;
                    }
                }
                FrameItem::Image { .. } if planned_external => {
                    let mut item = item;
                    super::helpers::offset_frame_item(&mut item, offset_x.0, offset_y.0);
                    self.regions.current.current_line.push(item);
                }
                FrameItem::Image { .. } => {} // legacy unplanned path
                // **P994** — `Shape`/`Group` OCORREM desde P994: o catch-all
                // de `layout_node` (`layout_external`) embute conteúdo externo
                // (`box()`/`block()`/`pad()`/…) como `Group` (com `Shape`s de
                // borda/preenchimento dentro). Integração como o braço `Text`:
                // posição absoluta + avanço do cursor pela largura interna.
                FrameItem::Shape {
                    pos,
                    kind,
                    width,
                    height,
                    fill,
                    stroke,
                    fill_rule,
                    parent_bbox_at_emit,
                } => {
                    let abs_pos = Point { x: offset_x + pos.x, y: offset_y + pos.y };
                    let extent_x = abs_pos.x + Pt(width);
                    self.regions.current.current_line.push(FrameItem::Shape {
                        pos: abs_pos,
                        kind,
                        width,
                        height,
                        fill,
                        stroke,
                        fill_rule,
                        parent_bbox_at_emit,
                    });
                    if extent_x > self.regions.current.cursor_x {
                        self.regions.current.cursor_x = extent_x;
                    }
                }
                FrameItem::Group {
                    paint_root,
                    pos,
                    matrix,
                    clip_mask,
                    inner_width,
                    inner_height,
                    items,
                } => {
                    let abs_pos = Point { x: offset_x + pos.x, y: offset_y + pos.y };
                    let extent_x = abs_pos.x + Pt(inner_width);
                    self.regions.current.current_line.push(FrameItem::Group {
                        paint_root,
                        pos: abs_pos,
                        matrix,
                        clip_mask,
                        inner_width,
                        inner_height,
                        items,
                    });
                    if extent_x > self.regions.current.cursor_x {
                        self.regions.current.cursor_x = extent_x;
                    }
                }
                FrameItem::Link { .. } if planned_external => {
                    let mut item = item;
                    super::helpers::offset_frame_item(&mut item, offset_x.0, offset_y.0);
                    self.regions.current.current_line.push(item);
                }
                FrameItem::Link { .. } => {} // legacy unplanned path
                FrameItem::Semantic { kind, placement, alt, mut items } => {
                    if planned_external {
                        for child in &mut items {
                            super::helpers::offset_frame_item(
                                child, offset_x.0, offset_y.0,
                            );
                        }
                    }
                    self.regions.current.current_line.push(FrameItem::Semantic {
                        kind,
                        placement,
                        alt,
                        items,
                    });
                }
            }
        }

        let mut equation_items = self
            .regions
            .current
            .current_line
            .split_off(current_line_len_before_eq);
        let ext = extent.as_ref().expect("equation extent is always measured");
        let frame_pos = Point { x: offset_x, y: offset_y - Pt(ext.ascent) };
        for item in &mut equation_items {
            super::helpers::offset_frame_item(item, -frame_pos.x.0, -frame_pos.y.0);
        }
        let equation_frame = FrameItem::Group {
            paint_root: false,
            pos: frame_pos,
            matrix: crate::entities::layout_types::TransformMatrix::identity(),
            clip_mask: None,
            inner_width: ext.width,
            inner_height: ext.ascent + ext.descent,
            items: equation_items,
        };
        let alt = match self.chain.custom("equation.alt") {
            Some(crate::entities::value::Value::Str(text)) => Some(text.clone()),
            _ => None,
        };
        self.regions.current.current_line.push(FrameItem::Semantic {
            kind: SemanticKind::Formula,
            placement: if block {
                SemanticPlacement::Block
            } else {
                SemanticPlacement::Inline
            },
            alt,
            items: vec![equation_frame],
        });

        // **P896** — se a centragem ficou pendente, o índice inicial dos
        // items desta equação em `current_items` (depois do `flush_line()`
        // abaixo) é o comprimento actual de `current_items` mais o que já
        // estava em `current_line` antes desta equação começar a empurrar
        // os seus próprios items (preserva ordem: `flush_line` drena
        // `current_line` para `current_items` em sequência).
        if let Some(eq_width) = pending_center_width {
            if !self.is_sub_frame {
                let start_idx =
                    self.regions.current.current_items.len() + current_line_len_before_eq;
                let items_pushed =
                    self.regions.current.current_line.len() - current_line_len_before_eq;
                self.pending_equation_centering.push((
                    start_idx,
                    items_pushed,
                    eq_width,
                    offset_x.val(),
                ));
            }
        }

        if let Some(ref ext) = extent {
            let total_extent_x = offset_x + Pt(ext.width);
            if total_extent_x > self.regions.current.cursor_x {
                self.regions.current.cursor_x = total_extent_x;
            }
        }

        // Guardar a baseline da linha antes do flush; usada para posicionar o
        // número à direita verticalmente alinhado com a equação.
        let mut equation_baseline_y = self.regions.current.cursor_y;

        if block {
            self.flush_line();
            // Closure may migrate the entire formula. Its settled baseline
            // owns both the following block margin and marginal numbering.
            equation_baseline_y = Pt(self.prev_line_baseline);
            let ext = extent.expect("bloco tem extent medido (P813)");
            let spacing =
                Pt(self.style.size.val() * super::vanilla_defaults::BLOCK_SPACING);
            self.prev_block_equation_descent = ext.descent;
            self.last_equation_descent = ext.descent;
            self.prev_block_below_pending = spacing.0;
            self.block_chain_active = true;
            self.prev_margin_is_parbreak = false;
            self.last_block_descent_y = Some(equation_baseline_y.0 + ext.descent);
        }

        // Acrescentar número da equação à direita da página (P456).
        if is_numbered {
            // P190F (M6 categoria Counters core): fallback legacy
            // `self.counter.get_flat("equation")` removido. Caminho
            // Introspector único — gate em `from_tags::Equation`
            // (P186E) activado por SetEquationNumbering (P199B);
            // CounterRegistry chave "equation" populated.
            use crate::entities::introspector::Introspector;
            let number_content = self
                .current_location
                .and_then(|loc| self.introspector.equation_numbering_content(loc))
                .or_else(|| {
                    let loc = self.current_location?;
                    let pattern = self.introspector.equation_numbering_pattern(loc)?;
                    let key = CounterKey::Selector(Selector::Kind(ElementKind::Equation));
                    let n = self.introspector.flat_counter_at(&key, loc)?;
                    let text =
                        format_counter(&[n], pattern).unwrap_or_else(|| n.to_string());
                    Some(Content::text(&text))
                })
                .unwrap_or(Content::Empty);
            let formatted = number_content.plain_text();
            let rtl = matches!(
                self.chain.custom("text.dir"),
                Some(crate::entities::value::Value::Dir(crate::entities::dir::Dir::RTL))
            );
            let physical_h = match number_align.h.unwrap_or(HAlign::End) {
                HAlign::Start => {
                    if rtl {
                        HAlign::Right
                    } else {
                        HAlign::Left
                    }
                }
                HAlign::End => {
                    if rtl {
                        HAlign::Left
                    } else {
                        HAlign::Right
                    }
                }
                h => h,
            };
            let number_ink =
                self.metrics.text_ink_bounds(&formatted, math_style.size, &math_style);
            let number_baseline_y = match number_align.v.unwrap_or(VAlign::Horizon) {
                VAlign::Top => equation_baseline_y + Pt(extent.unwrap().first_baseline),
                VAlign::Bottom => equation_baseline_y + Pt(extent.unwrap().last_baseline),
                VAlign::Horizon if extent.unwrap().line_count <= 1 => equation_baseline_y,
                VAlign::Horizon => {
                    let ext = extent.unwrap();
                    let equation_center = (ext.descent - ext.ascent) / 2.0;
                    let number_center = (number_ink.1.val() - number_ink.0.val()) / 2.0;
                    equation_baseline_y + Pt(equation_center - number_center)
                }
            };

            // **P895/P896** — mesma condição da centragem acima: sem uma
            // largura de página resolvida (`width: auto`), não há uma
            // margem direita bem definida contra a qual alinhar o número
            // agora. P895 suprimia o número neste caso (evitava o
            // `infinito`, achado de `typst-passo-894-relatorio.md`); P896
            // regista os dados para posicionar o número depois, em
            // `finish()`/`new_page()`, quando a largura final da página é
            // conhecida (`pending_equation_numbering`, `layout/mod.rs`).
            let number_text: ecow::EcoString = formatted.into();
            // **P944** (emenda da revisão cética) — o número é composto com a
            // chain que inclui o show_set da equação (vanilla:
            // `layout_frame(engine, &counter, …, styles)`,
            // `lab/typst-original/crates/typst-layout/src/math/mod.rs:217`) —
            // usa `math_style` (New Computer Modern Math, 450), não o estilo
            // do documento; a medição da largura usa o mesmo estilo.
            let number_width =
                self.metrics.advance(&number_text, math_style.size, &math_style);
            if self.regions.current.width.is_finite() {
                let number_x = match physical_h {
                    HAlign::Left => Pt(self.page_config.margin.left),
                    HAlign::Right => {
                        Pt(self.regions.current.width - self.page_config.margin.right)
                            - number_width
                    }
                    HAlign::Center | HAlign::Start | HAlign::End => {
                        unreachable!("cast de number-align resolve para margem física")
                    }
                };

                self.regions.current.current_items.push(FrameItem::Text {
                    pos: Point { x: number_x, y: number_baseline_y },
                    text: number_text,
                    style: math_style.clone(),
                    bidi: None,
                });
            } else {
                // **P987** — registar também a largura da equação e o offset
                // x aplicado (fallback, o mesmo valor do pending de
                // centragem acima): o fixup posiciona o número a
                // `content_end + gutter` da SUA equação e a largura da
                // página auto reserva a calha (vanilla
                // `typst-layout/src/math/mod.rs:209-330`).
                let eq_width = extent.expect("numerada é bloco (P813)").width;
                self.pending_equation_numbering.push((
                    number_baseline_y.val(),
                    number_text,
                    math_style.clone(),
                    number_width.val(),
                    eq_width,
                    offset_x.val(),
                    physical_h,
                ));
            }
        }
    }

    /// Braço `Content::Equation` do `layout_content` (atomização ADR-0109
    /// P382): decodifica o gate de numeração da chain e delega a
    /// `layout_equation`. Content-preserving — era inline no `layout_content`.
    pub(super) fn layout_equation_arm(&mut self, e: &EquationElem) {
        // F-5a de-bake (P364, §3a.9): o gate vive **só na chain**; lido
        // de `self.chain`; Str e Func ativam, None desativa.
        let numbering_active = matches!(
            self.chain.custom("equation.numbering"),
            Some(crate::entities::value::Value::Str(_))
                | Some(crate::entities::value::Value::Func(_))
        );
        self.layout_equation(&e.body, e.block, numbering_active);
    }

    /// Fallback de nós matemáticos que aparecem **fora** de um `Content::Equation`
    /// (atomização ADR-0109 P382). Normalmente não ocorrem directamente no
    /// layout — o despacho real é `MathLayouter::layout_node`. Se aparecerem,
    /// renderiza como texto. Content-preserving — era inline no `layout_content`.
    pub(super) fn layout_math_fallback(&mut self, content: &Content) {
        let text = content.plain_text();
        for word in text.split_whitespace() {
            self.layout_word(word);
        }
    }
}

impl<'a, M: FontMetrics, S: ImageSizer> super::Layouter<'a, M, S> {
    /// **P896** — resolve a centragem/numeração de equação de bloco que
    /// ficou adiada (`width: auto`, valor ainda infinito no momento em que
    /// a equação foi posicionada — ver `equation.rs`). Chamado por
    /// `finish()`/`new_page()`, depois de `page_width`/`page_height` já
    /// estarem resolvidos para valores finitos, sobre o `items` local que
    /// vai ser movido para a `Page` final.
    ///
    /// Centragem: desloca os items da equação (pelo índice/gama gravados)
    /// pela diferença entre o offset correcto (agora que `page_width` é
    /// finito) e o offset já aplicado (a margem, valor de fallback usado em
    /// `equation.rs`). Numeração: constrói o `FrameItem::Text` do número
    /// (posição à direita só era resolúvel agora) e acrescenta-o a `items`.
    ///
    /// Esvazia os dois `Vec` pendentes — são sempre resolvidos por completo
    /// de uma vez (uma página não fecha parcialmente).
    pub(super) fn apply_pending_equation_fixups(
        &mut self,
        items: &mut Vec<FrameItem>,
        page_width: f64,
    ) {
        let usable = page_width - self.page_config.margin.horizontal();
        for (start_idx, count, eq_width, applied_offset) in
            std::mem::take(&mut self.pending_equation_centering)
        {
            // rationale: P1064 Classe 1A — centragem horizontal de equação ((usable - eq_width) / 2.0)
            let correct_offset = self.page_config.margin.left + (usable - eq_width) / 2.0;
            let delta = correct_offset - applied_offset;
            if delta == 0.0 {
                continue;
            }
            for item in items.iter_mut().skip(start_idx).take(count) {
                super::helpers::shift_frame_item_x(item, delta);
            }
        }

        for (baseline_y, text, style, number_width, eq_width, _applied_offset, align) in
            std::mem::take(&mut self.pending_equation_numbering)
        {
            // **P987** — o número fica junto ao fim do conteúdo centrado da
            // SUA equação (mesma fórmula do fixup de centragem acima) mais a
            // calha `NUMBER_GUTTER = 0.5em` (invariante vanilla
            // `number_x = content_end_x + gutter`), em vez da margem direita
            // da largura computada da página inteira.
            let gutter = 0.5 * style.size.val();
            let number_x = match align {
                HAlign::Left => self.page_config.margin.left,
                HAlign::Right => {
                    // rationale: P1064 Classe 1A — centragem de equação com número ((usable - eq_width) / 2.0)
                    self.page_config.margin.left
                        + (usable - eq_width) / 2.0
                        + eq_width
                        + gutter
                }
                HAlign::Center | HAlign::Start | HAlign::End => {
                    page_width - self.page_config.margin.right - number_width
                }
            };
            items.push(FrameItem::Text {
                pos: Point { x: Pt(number_x), y: Pt(baseline_y) },
                text,
                style,
                bidi: None,
            });
        }
    }
}

#[cfg(test)]
mod smoke {
    use super::*;
    use crate::compiler::layout::layout;
    use crate::entities::elements::math_attach::MathAttachSlot;
    use crate::entities::layout_types::{PageDimension, PageMarginSpec};

    fn page_auto(body: Vec<Content>) -> crate::entities::layout_types::PagedDocument {
        let mut parts = vec![Content::SetPage(std::sync::Arc::new(
            crate::entities::content::SetPageDelta {
                paper: None,
                flipped: None,
                binding: None,
                width: Some(PageDimension::Auto),
                height: Some(PageDimension::Auto),
                margin: Some(PageMarginSpec::uniform(0.0)),
                numbering: None,
                number_align: None,
                header: None,
                header_ascent: None,
                footer: None,
                footer_descent: None,
                supplement: None,
                columns: None,
                bleed: None,
                fill: None,
                background: None,
                foreground: None,
            },
        ))];
        parts.extend(body);
        layout(&Content::Sequence(parts.into()))
    }

    fn formula_group(items: &[FrameItem]) -> Option<&FrameItem> {
        for item in items {
            match item {
                FrameItem::Semantic { kind: SemanticKind::Formula, items, .. } => {
                    return items.first();
                }
                FrameItem::Group { items, .. }
                | FrameItem::TextReplacement { items, .. }
                | FrameItem::Semantic { items, .. }
                | FrameItem::Link { items, .. } => {
                    if let Some(found) = formula_group(items) {
                        return Some(found);
                    }
                }
                FrameItem::TextSeparator { .. }
                | FrameItem::Text { .. }
                | FrameItem::TextShaped { .. }
                | FrameItem::Line { .. }
                | FrameItem::Glyph { .. }
                | FrameItem::Image { .. }
                | FrameItem::Shape { .. } => {}
            }
        }
        None
    }

    fn text_x(items: &[FrameItem], target: &str, origin_x: f64) -> Option<f64> {
        for item in items {
            match item {
                FrameItem::Text { pos, text, .. }
                | FrameItem::TextShaped { pos, text, .. }
                    if text.as_str() == target =>
                {
                    return Some(origin_x + pos.x.0);
                }
                FrameItem::Group { pos, items, .. } => {
                    if let Some(found) = text_x(items, target, origin_x + pos.x.0) {
                        return Some(found);
                    }
                }
                FrameItem::TextReplacement { items, .. }
                | FrameItem::Semantic { items, .. }
                | FrameItem::Link { items, .. } => {
                    if let Some(found) = text_x(items, target, origin_x) {
                        return Some(found);
                    }
                }
                FrameItem::TextSeparator { .. }
                | FrameItem::Text { .. }
                | FrameItem::TextShaped { .. }
                | FrameItem::Line { .. }
                | FrameItem::Glyph { .. }
                | FrameItem::Image { .. }
                | FrameItem::Shape { .. } => {}
            }
        }
        None
    }

    fn attach_with_post(label: &str) -> Content {
        Content::equation(
            Content::math_attach(
                Content::MathIdent("x".into()),
                MathAttachSlot::Omitted,
                MathAttachSlot::Omitted,
                MathAttachSlot::Omitted,
                MathAttachSlot::Omitted,
                MathAttachSlot::Present(Content::Text(label.into())),
                MathAttachSlot::Omitted,
            ),
            false,
        )
    }

    #[test]
    fn module_compila_e_carrega() {
        // V2 smoke test — submódulo extraído no Passo 96.7 (ADR-0037).
        // A cobertura funcional vive em `layout/tests.rs`.
    }

    #[test]
    fn p1293_formula_terminal_auto_preserva_extent_em_group_identidade() {
        for label in ["R", "K", "W", "RR"] {
            let doc = page_auto(vec![attach_with_post(label)]);
            let page = doc.pages.first().expect("página terminal");
            assert!(page.width.is_finite() && page.height.is_finite());
            let frame = formula_group(&page.items).expect("Semantic::Formula presente");
            let FrameItem::Group { matrix, clip_mask, inner_width, inner_height, .. } =
                frame
            else {
                panic!("Formula deve conter um único Group lógico: {frame:?}");
            };
            assert_eq!(
                *matrix,
                crate::entities::layout_types::TransformMatrix::identity()
            );
            assert!(clip_mask.is_none());
            assert!(*inner_width > 0.0 && *inner_height > 0.0);
        }
    }

    fn continuation_document(
        count: usize,
        after: bool,
    ) -> crate::entities::layout_types::PagedDocument {
        let mut parts = vec![Content::SetPage(std::sync::Arc::new(
            crate::entities::content::SetPageDelta {
                width: Some(PageDimension::Length(150.0)),
                height: Some(PageDimension::Length(90.0)),
                margin: Some(PageMarginSpec::uniform(10.0)),
                ..Default::default()
            },
        ))];
        for _ in 0..count {
            parts.push(Content::equation(
                Content::math_frac(
                    Content::MathIdent("a".into()),
                    Content::MathIdent("b".into()),
                ),
                true,
            ));
        }
        if after {
            parts.push(Content::text("AFTER"));
        }
        layout(&Content::styled(
            Content::sequence(parts),
            crate::entities::style::Styles::from_iter([
                crate::entities::style::Style::Size(
                    crate::entities::layout_types::Length::pt(12.0),
                ),
            ]),
        ))
    }

    fn continuation_bounds(items: &[FrameItem]) -> Vec<(f64, f64)> {
        items
            .iter()
            .filter_map(|item| {
                if let FrameItem::Semantic {
                    kind: SemanticKind::Formula, items, ..
                } = item
                {
                    if let Some(FrameItem::Group { pos, inner_height, .. }) =
                        items.first()
                    {
                        return Some((pos.y.0, pos.y.0 + inner_height));
                    }
                }
                None
            })
            .collect()
    }

    #[test]
    fn math_continuation_commits_complete_display_frames_inside_each_region() {
        let doc = continuation_document(8, false);
        let mut total = 0;
        for (index, page) in doc.pages.iter().enumerate() {
            let bounds = continuation_bounds(&page.items);
            total += bounds.len();
            assert!(!bounds.is_empty(), "no manufactured blank page {index}");
            for (top, bottom) in bounds {
                assert!(top >= 10.0 - 1e-9, "page {index}: top {top}");
                assert!(bottom <= 80.0 + 1e-9, "page {index}: bottom {bottom}");
            }
        }
        assert_eq!(total, 8);
    }

    #[test]
    fn math_continuation_restores_block_spacing_after_migration() {
        let doc = continuation_document(3, true);
        let page = doc.pages.last().unwrap();
        let (_, bottom) = *continuation_bounds(&page.items).last().expect("last display");
        let after_y = page
            .items
            .iter()
            .find_map(|item| {
                if let FrameItem::Text { pos, text, .. } = item {
                    (text.as_str() == "AFTER").then_some(pos.y.0)
                } else {
                    None
                }
            })
            .expect("following text shares the last display page");
        // Existing below=1.2em and cap-height=0.7em, for the 12pt FixedMetrics fixture.
        assert!(
            (after_y - bottom - 22.8).abs() < 1e-9,
            "bottom={bottom} after={after_y}"
        );
    }

    #[test]
    fn p1293_formula_com_marcador_nao_duplica_avanco_e_pagina_finita_preserva_width() {
        let terminal = page_auto(vec![attach_with_post("R")]);
        let marked = page_auto(vec![attach_with_post("R"), Content::text("Z")]);
        assert!(terminal.pages[0].width.is_finite());
        assert!(marked.pages[0].width.is_finite());
        let FrameItem::Group { pos, inner_width, .. } =
            formula_group(&marked.pages[0].items).expect("Formula com marcador")
        else {
            panic!("Formula deve conter Group");
        };
        let marker_x = text_x(&marked.pages[0].items, "Z", 0.0).expect("marcador Z");
        assert!(
            (marker_x - (pos.x.0 + inner_width)).abs() < 1e-9,
            "marcador seguinte deve iniciar no fim lógico da fórmula"
        );

        let finite = layout(&Content::Sequence(
            vec![
                Content::SetPage(std::sync::Arc::new(
                    crate::entities::content::SetPageDelta {
                        paper: None,
                        flipped: None,
                        binding: None,
                        width: Some(PageDimension::Length(123.0)),
                        height: Some(PageDimension::Length(80.0)),
                        margin: Some(PageMarginSpec::uniform(0.0)),
                        numbering: None,
                        number_align: None,
                        header: None,
                        header_ascent: None,
                        footer: None,
                        footer_descent: None,
                        supplement: None,
                        columns: None,
                        bleed: None,
                        fill: None,
                        background: None,
                        foreground: None,
                    },
                )),
                attach_with_post("R"),
            ]
            .into(),
        ));
        assert_eq!(finite.pages[0].width, 123.0);
        assert_eq!(finite.pages[0].height, 80.0);
        assert!(matches!(
            formula_group(&finite.pages[0].items),
            Some(FrameItem::Group { paint_root: false, .. })
        ));
    }
}

#[cfg(test)]
mod physical_page_direct_body_tests {
    use super::*;
    use crate::compiler::introspect::page_running::PlanPhase;
    use crate::compiler::{
        eval::EvalContext,
        introspect::{
            introspect_with_introspector, page_running::prepare_page_running_body,
        },
        layout::{
            layout, layout_with_page_running, FixedMetrics, PageRunningLayoutInputs,
            SealedPageRunning,
        },
        realize::context_occurrences::ContextOccurrenceArena,
    };
    use crate::entities::{
        elements::pagebreak::PagebreakElem, image_sizer::NullImageSizer, value::Value,
    };
    use std::sync::Arc;

    fn planned(
        body: Content,
        second_page: bool,
    ) -> crate::compiler::layout::PageRunningDocument {
        let equation = Content::equation(body, false);
        let origin = if second_page {
            Content::sequence(vec![
                Content::text("FIRST PAGE"),
                Content::Pagebreak(Arc::new(PagebreakElem {
                    weak: false,
                    weak_explicit: false,
                    to: None,
                })),
                equation,
            ])
        } else {
            equation
        };
        let intr = introspect_with_introspector(&origin);
        let mut arena = ContextOccurrenceArena::new(&origin);
        let mut frame = arena.frame();
        let mut ctx = EvalContext::new();
        let body = arena
            .evaluate(&mut frame, &mut ctx, |ctx| prepare_page_running_body(&origin, ctx))
            .unwrap();
        let store = SealedPageRunning::new(vec![]).unwrap();
        match layout_with_page_running(
            &body,
            intr,
            FixedMetrics,
            NullImageSizer,
            11.0,
            PageRunningLayoutInputs { math: None, par_line: None, running: Some(&store) },
        )
        .into_result()
        {
            Ok(document) => document,
            Err(_) => panic!(
                "direct Equation body facts require an actual complete paint trace"
            ),
        }
    }

    #[test]
    fn physical_page_equation_direct_metadata_keeps_membership_and_actual_page() {
        let document =
            planned(Content::metadata(Value::Str("DIRECT CHILD".into())), true);
        let metadata: Vec<_> = document
            .trace
            .data
            .admissions
            .iter()
            .filter_map(|event| {
                let node = event.plan.node(event.packet).unwrap();
                (event.phase == PlanPhase::Root
                    && matches!(&node.content, Content::Metadata(_)))
                .then(|| node.location.expect("actual Metadata occurrence"))
            })
            .collect();
        assert_eq!(metadata.len(), 1, "the actual child is admitted once");
        let position = document
            .document
            .extracted_positions
            .position_of(metadata[0])
            .expect("paint-derived Metadata position");
        assert_eq!(
            position.page.get(),
            2,
            "external local page1 is not its final physical page"
        );
        assert!(position.point.x.0.is_finite() && position.point.y.0.is_finite());
        assert_eq!(
            document.trace.data.positions.position_of(metadata[0]),
            Some(position)
        );
    }

    #[test]
    fn physical_page_equation_direct_figure_paints_once_with_actual_page() {
        let document =
            planned(Content::figure(Content::text("ONLY-ONCE"), None, None, None), true);
        let figures: Vec<_> = document
            .trace
            .data
            .admissions
            .iter()
            .filter_map(|event| {
                let node = event.plan.node(event.packet).unwrap();
                (event.phase == PlanPhase::Root
                    && matches!(&node.content, Content::Figure(_)))
                .then(|| node.location.expect("actual Figure occurrence"))
            })
            .collect();
        assert_eq!(figures.len(), 1, "the actual Figure body is admitted once");
        assert_eq!(
            document.document.plain_text().matches("ONLY-ONCE").count(),
            1,
            "the actual external painter runs once"
        );
        let position = document
            .document
            .extracted_positions
            .position_of(figures[0])
            .expect("paint-derived Figure position");
        assert_eq!(position.page.get(), 2);
        assert_eq!(document.trace.data.positions.position_of(figures[0]), Some(position));
    }

    #[test]
    fn physical_page_equation_terminal_math_preserves_opaque_children_and_render() {
        let math = Content::MathSequence(
            vec![
                Content::metadata(Value::Str("OPAQUE".into())),
                Content::MathIdent("x".into()),
            ]
            .into(),
        );
        let legacy = layout(&Content::equation(math.clone(), false));
        let document = planned(math, false);
        assert_eq!(document.document.plain_text(), legacy.plain_text());
        assert_eq!(document.document.pages.len(), legacy.pages.len());
        assert!(
            !document.trace.data.admissions.iter().any(|event| matches!(
                &event.plan.node(event.packet).unwrap().content,
                Content::Metadata(_)
            )),
            "the existing terminal MathSequence partition is not opened"
        );
    }
    #[test]
    fn physical_page_equation_zero_paint_action_keeps_legacy_formula_extent() {
        fn formula_extent(items: &[FrameItem]) -> Option<(f64, f64)> {
            for item in items {
                match item {
                    FrameItem::Semantic {
                        kind: SemanticKind::Formula, items, ..
                    } => {
                        for child in items {
                            if let FrameItem::Group {
                                inner_width, inner_height, ..
                            } = child
                            {
                                return Some((*inner_width, *inner_height));
                            }
                        }
                    }
                    FrameItem::Semantic { items, .. }
                    | FrameItem::Group { items, .. }
                    | FrameItem::Link { items, .. }
                    | FrameItem::TextReplacement { items, .. } => {
                        if let Some(extent) = formula_extent(items) {
                            return Some(extent);
                        }
                    }
                    FrameItem::TextSeparator { .. }
                    | FrameItem::Text { .. }
                    | FrameItem::TextShaped { .. }
                    | FrameItem::Glyph { .. }
                    | FrameItem::Shape { .. }
                    | FrameItem::Line { .. }
                    | FrameItem::Image { .. } => {}
                }
            }
            None
        }
        let action = Content::counter_update(
            "zero-paint",
            crate::entities::counter_update::CounterUpdate::step(),
        );
        let legacy = layout(&Content::equation(action.clone(), false));
        let actual = planned(action, false);
        assert_eq!(
            actual
                .trace
                .data
                .admissions
                .iter()
                .filter(|event| {
                    event.phase == PlanPhase::Root
                        && matches!(
                            &event.plan.node(event.packet).unwrap().content,
                            Content::CounterUpdate(_)
                        )
                })
                .count(),
            1,
            "the zero-paint action is actually admitted"
        );
        let expected = formula_extent(&legacy.pages[0].items).expect("legacy formula");
        let observed =
            formula_extent(&actual.document.pages[0].items).expect("planned formula");
        assert_eq!(observed, expected, "source markers do not acquire mathematical ink");
        assert_eq!(actual.document.plain_text(), legacy.plain_text());
        assert_eq!(actual.document.pages.len(), legacy.pages.len());
    }
}

#[cfg(test)]
mod r3_geometry_tests {
    use super::*;
    use crate::compiler::layout::{layout_with_introspector_and_metrics, FixedMetrics};
    use crate::entities::{image_sizer::NullImageSizer, introspector::TagIntrospector};

    #[derive(Clone)]
    struct Edges;
    impl FontMetrics for Edges {
        fn advance(&self, text: &str, size: Pt, style: &TextStyle) -> Pt {
            FixedMetrics.advance(text, size, style)
        }
        fn vertical_metrics(&self, size: Pt, style: &TextStyle) -> (Pt, Pt) {
            FixedMetrics.vertical_metrics(size, style)
        }
        fn cap_height(&self, _: Pt, style: &TextStyle) -> Pt {
            Pt(if style.math { 7.0 } else { 9.0 })
        }
        fn text_edges(&self, size: Pt, style: &TextStyle) -> (Pt, Pt) {
            (self.cap_height(size, style), Pt(0.0))
        }
    }

    #[test]
    fn r3_geometry_initial_inline_math_uses_measured_top_and_mixed_maximum() {
        for placement in [0, 1, 2, 3, 4, 5] {
            let equation = Content::equation(Content::MathIdent("a".into()), false);
            let body = match placement {
                0 => equation,
                1 => Content::sequence(vec![Content::text("T"), equation]),
                2 => Content::sequence(vec![equation, Content::text("T")]),
                3 => Content::sequence(vec![equation.clone(), Content::Space, equation]),
                4 => Content::sequence(vec![
                    equation,
                    Content::Linebreak(std::sync::Arc::new(
                        crate::entities::elements::linebreak::LinebreakElem {
                            justify: false,
                            justify_explicit: false,
                        },
                    )),
                ]),
                5 => Content::sequence(vec![equation, Content::Space]),
                _ => unreachable!(),
            };
            let body = Content::sequence(vec![
                Content::SetPage(std::sync::Arc::new(
                    crate::entities::content::SetPageDelta {
                        margin: Some(
                            crate::entities::layout_types::PageMarginSpec::uniform(0.0),
                        ),
                        ..Default::default()
                    },
                )),
                body,
            ]);
            let document = layout_with_introspector_and_metrics(
                &body,
                TagIntrospector::empty(),
                Edges,
                NullImageSizer,
                12.0,
            )
            .unwrap();
            let formula = document.pages[0]
                .items
                .iter()
                .find(|item| {
                    matches!(
                        item,
                        FrameItem::Semantic { kind: SemanticKind::Formula, .. }
                    )
                })
                .expect("formula retained");
            let FrameItem::Semantic { items, .. } = formula else { panic!("formula") };
            let FrameItem::Group { pos, items, .. } = &items[0] else { panic!("group") };
            let FrameItem::Text { pos: glyph, .. } = &items[0] else { panic!("glyph") };
            assert_eq!(
                pos.y.0 + glyph.y.0,
                if placement == 0 || placement == 5 { 7.0 } else { 9.0 }
            );
        }
    }
}
