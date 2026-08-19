//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/equation.md
//! @prompt-hash 2abc4392
//! @layer L1
//! @updated 2026-08-01
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
    layout_types::{FrameItem, MathSize, Point, Pt, TextStyle},
    selector::Selector,
};

use super::metrics::FontMetrics;

impl<'a, M: FontMetrics, S: ImageSizer> super::Layouter<'a, M, S> {
    /// Layout de `Content::Equation { body, block }`.
    /// `numbering_pattern`: pattern de numeração vindo da chain (`None` se
    /// a equação não deve ser numerada).
    pub(super) fn layout_equation(
        &mut self,
        body: &Content,
        block: bool,
        numbering_pattern: Option<&str>,
    ) {
        // **P751** — fixar a baseline inicial com o estilo activo antes de
        // posicionar texto/equação real.
        // **P813** — capturar a flag ANTES do ensure: equações de bloco no
        // topo da página/região suprimem o spacing acima (paridade vanilla,
        // medido: `$x^2$` sozinho → baseline = margin + ascent).
        let was_initial_baseline_pending = self.initial_baseline_pending;
        self.ensure_initial_baseline();
        // Auto-numeração: equações de bloco numeradas avançam o contador antes de
        // desenhar (Passo 59). O número (N) é acrescentado depois da equação.
        // Lote F-2 S2 (P335): o "ativo" é **assado** no `EquationElem` (escopo
        // léxico via chain, fecha o canal global StateRegistry). O **valor** do
        // contador continua via Introspector (`flat_counter_at`), gateado pelo
        // mesmo `numbering_active` assado (via payload, em `from_tags`).
        // P456: o pattern vem da chain como `Value::Str`; ausência = não numerada.
        let is_numbered = block && numbering_pattern.is_some();
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
        let math_layouter = math::layout::MathLayouter::new(&self.metrics, block, &math_style);
        // **P813** — equações de bloco precisam da extensão geométrica
        // (largura + ascent/descent de tinta) para centragem e espaçamento;
        // os items são os mesmos de `layout_equation`.
        let (math_items, extent) = if block {
            let (items, ext) = math_layouter.layout_equation_measured(body, &math_style);
            (items, Some(ext))
        } else {
            (math_layouter.layout_equation(body, &math_style), None)
        };

        let mut offset_x = self.regions.current.cursor_x;
        // **P896** — `Some(largura_da_equação)` quando a centragem teve de
        // ser adiada (`width: auto`, valor ainda infinito neste ponto).
        let mut pending_center_width: Option<f64> = None;
        if block {
            let ext = extent.expect("bloco tem extent medido (P813)");
            // **P813** — spacing vertical de bloco 1.2em acima e abaixo
            // (paridade vanilla `BlockElem::above/below` default —
            // lab/typst-original/crates/typst-library/src/layout/container.rs:342).
            // **P813** / **P1087** — spacing vertical de bloco 1.2em (BLOCK_SPACING)
            // acima e abaixo (paridade vanilla BlockElem::above/below default —
            // lab/typst-original/crates/typst-library/src/layout/container.rs:342).
            let spacing = Pt(self.style.size.val() * super::vanilla_defaults::BLOCK_SPACING);
            if was_initial_baseline_pending {
                // Topo da página: spacing acima suprimido; baseline da
                // equação = margin + ascent_ink (medido em P813). O
                // `ensure_initial_baseline` deixou cursor_y = margin +
                // top_edge do texto — converter para o ascent da equação.
                let (top_text, _) =
                    self.metrics.text_edges(self.style.size, &self.style);
                self.regions.current.cursor_y += Pt(ext.ascent) - top_text;
                self.prev_block_below_pending = 0.0;
            } else {
                let pages_before = self.pages.len();
                let prev_baseline =
                    if self.regions.current.cursor_x.0 > self.page_config.margin {
                        let b = self.regions.current.cursor_y.0;
                        self.flush_line();
                        b
                    } else if self.prev_line_baseline > 0.0 {
                        self.prev_line_baseline + self.prev_block_equation_descent
                    } else {
                        // Linha já fechada (ex.: após Parbreak ou equação anterior).
                        // **P952** — equação→equação: incluir a `descent_ink`
                        // da equação anterior (vanilla aresta-a-aresta);
                        // 0.0 para qualquer outro conteúdo (P813 inalterado).
                        self.regions.current.cursor_y.0
                            - self.last_flush_advance
                            + self.prev_block_equation_descent
                    };
                if self.pages.len() == pages_before {
                    // **P1088** — Protocolo Genérico de Colapso de Margens de Bloco (distribute.rs:205):
                    // - Se o bloco anterior tiver weakness 3 (ex: Heading com below explícito = 8.25pt),
                    //   weakness 3 vence o above default (weakness 4) da equação (keep_weak_rel_spacing).
                    // - Se a margem veio de Parbreak (weakness 4), colapsa pelo max(prev, curr).
                    let gap = if self.block_chain_active {
                        if !self.prev_margin_is_parbreak {
                            self.prev_block_below_pending
                        } else {
                            self.prev_block_below_pending.max(spacing.val())
                        }
                    } else {
                        spacing.val()
                    };
                    self.regions.current.cursor_y = Pt(prev_baseline) + Pt(gap) + Pt(ext.ascent);
                }
            }
            self.prev_line_baseline = self.regions.current.cursor_y.0;
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
                let usable = self.regions.current.width - 2.0 * self.page_config.margin;
                // rationale: P1064 Classe 1A — centragem de equação em bloco ((usable - ext.width) / 2.0)
                offset_x = Pt(self.page_config.margin + (usable - ext.width) / 2.0);
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
                FrameItem::Text { pos, text, style } => {
                    let abs_pos = Point { x: offset_x + pos.x, y: offset_y + pos.y };
                    let advance = self.metrics.advance(&text, style.size, &style);
                    self.regions.current.current_line.push(FrameItem::Text {
                        pos: abs_pos,
                        text,
                        style,
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
                FrameItem::Line { start, end, thickness, color } => {
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
                    });
                }
                FrameItem::TextShaped { .. } => {} // TextShaped não ocorre antes do shaper em math inline
                FrameItem::Image { .. } => {}      // imagens não ocorrem em math inline
                // **P994** — `Shape`/`Group` OCORREM desde P994: o catch-all
                // de `layout_node` (`layout_external`) embute conteúdo externo
                // (`box()`/`block()`/`pad()`/…) como `Group` (com `Shape`s de
                // borda/preenchimento dentro). Integração como o braço `Text`:
                // posição absoluta + avanço do cursor pela largura interna.
                FrameItem::Shape { pos, kind, width, height, fill, stroke, parent_bbox_at_emit } => {
                    let abs_pos = Point { x: offset_x + pos.x, y: offset_y + pos.y };
                    let extent_x = abs_pos.x + Pt(width);
                    self.regions.current.current_line.push(FrameItem::Shape {
                        pos: abs_pos,
                        kind,
                        width,
                        height,
                        fill,
                        stroke,
                        parent_bbox_at_emit,
                    });
                    if extent_x > self.regions.current.cursor_x {
                        self.regions.current.cursor_x = extent_x;
                    }
                }
                FrameItem::Group { pos, matrix, clip_mask, inner_width, inner_height, items } => {
                    let abs_pos = Point { x: offset_x + pos.x, y: offset_y + pos.y };
                    let extent_x = abs_pos.x + Pt(inner_width);
                    self.regions.current.current_line.push(FrameItem::Group {
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
                FrameItem::Link { .. } => {}       // links não ocorrem em math inline
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
                    // **P967** — idem ao braço Text: extent real por item.
                    let extent_x = abs_pos.x + x_advance;
                    if extent_x > self.regions.current.cursor_x {
                        self.regions.current.cursor_x = extent_x;
                    }
                }
            }
        }

        // **P896** — se a centragem ficou pendente, o índice inicial dos
        // items desta equação em `current_items` (depois do `flush_line()`
        // abaixo) é o comprimento actual de `current_items` mais o que já
        // estava em `current_line` antes desta equação começar a empurrar
        // os seus próprios items (preserva ordem: `flush_line` drena
        // `current_line` para `current_items` em sequência).
        if let Some(eq_width) = pending_center_width {
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

        // Guardar a baseline da linha antes do flush; usada para posicionar o
        // número à direita verticalmente alinhado com a equação.
        let equation_baseline_y = self.regions.current.cursor_y;

        if block {
            let pages_before = self.pages.len();
            self.flush_line();
            // **P813** — a baseline seguinte é posicionada pelo modelo do
            // vanilla: baseline_equacao + descent_ink + spacing(1.2em) +
            // top_edge do texto seguinte (medido em P813). O avanço normal
            // do flush (top+bottom+leading da linha math) é substituído —
            // no vanilla não há leading entre filhos do flow, há o spacing
            // do bloco.
            if self.pages.len() == pages_before {
                let ext = extent.expect("bloco tem extent medido (P813)");
                let spacing = Pt(self.style.size.val() * super::vanilla_defaults::BLOCK_SPACING);
                let (top_text, _) =
                    self.metrics.text_edges(self.style.size, &self.style);
                self.regions.current.cursor_y =
                    equation_baseline_y + Pt(ext.descent) + spacing + top_text;
                // **P813** — manter `last_flush_advance` coerente com o
                // override: um bloco seguinte (ex.: outra equação) recupera
                // a baseline desta equação como
                // `cursor_y - last_flush_advance`.
                self.last_flush_advance =
                    self.regions.current.cursor_y.0 - equation_baseline_y.0;
                // **P952** — registar a `descent_ink` desta equação para a
                // próxima (espaçamento aresta-a-aresta equação→equação).
                self.prev_block_equation_descent = ext.descent;
                self.prev_block_below_pending = spacing.0;
                self.block_chain_active = true;
                self.prev_margin_is_parbreak = false;
            }
        }

        // Acrescentar número da equação à direita da página (P456).
        if is_numbered {
            // P190F (M6 categoria Counters core): fallback legacy
            // `self.counter.get_flat("equation")` removido. Caminho
            // Introspector único — gate em `from_tags::Equation`
            // (P186E) activado por SetEquationNumbering (P199B);
            // CounterRegistry chave "equation" populated.
            use crate::entities::introspector::Introspector;
            let equation_key = CounterKey::Selector(Selector::Kind(ElementKind::Equation));
            let n = self
                .current_location
                .and_then(|loc| self.introspector.flat_counter_at(&equation_key, loc))
                .unwrap_or(0);
            let pattern = numbering_pattern.unwrap_or("(1)");
            let formatted =
                format_counter(&[n], pattern).unwrap_or_else(|| n.to_string());

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
                let right_x =
                    Pt(self.regions.current.width - self.page_config.margin) - number_width;

                self.regions.current.current_items.push(FrameItem::Text {
                    pos: Point { x: right_x, y: equation_baseline_y },
                    text: number_text,
                    style: math_style.clone(),
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
                    equation_baseline_y.val(),
                    number_text,
                    math_style.clone(),
                    number_width.val(),
                    eq_width,
                    offset_x.val(),
                ));
            }
        }
    }

    /// Braço `Content::Equation` do `layout_content` (atomização ADR-0109
    /// P382): decodifica o gate de numeração da chain e delega a
    /// `layout_equation`. Content-preserving — era inline no `layout_content`.
    pub(super) fn layout_equation_arm(&mut self, e: &EquationElem) {
        // F-5a de-bake (P364, §3a.9): o gate vive **só na chain**; lido
        // de `self.chain`. P456: pattern é `Value::Str` (análogo a
        // figure.numbering em P454).
        let numbering_pattern: Option<String> =
            self.chain.custom("equation.numbering").and_then(|v| match v {
                crate::entities::value::Value::Str(s) => Some(s.to_string()),
                _ => None, // neutro: N16[β] — valor de numbering não-Str extraído como None (paridade vanilla equation.rs)
            });
        let pat = numbering_pattern.as_deref();
        self.layout_equation(&e.body, e.block, pat);
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

#[cfg(test)]
mod smoke {
    #[test]
    fn module_compila_e_carrega() {
        // V2 smoke test — submódulo extraído no Passo 96.7 (ADR-0037).
        // A cobertura funcional vive em `layout/tests.rs`.
    }
}
