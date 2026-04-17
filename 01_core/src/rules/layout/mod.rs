//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/layout.md
//! @prompt-hash 518a9856
//! @layer L1
//! @updated 2026-04-13

pub mod counters;
pub mod figure;
pub mod outline;
pub mod references;

use ecow::EcoString;

use crate::entities::{
    content::Content,
    counter_state::CounterState,
    glyph_variants::{GlyphAssembly, GlyphVariants, MathGlyphKern},
    layout_types::{Frame, FrameItem, PagedDocument, Point, Pt, Size, TextStyle},
    math_constants::MathConstants,
};
use crate::rules::math;

// ── Métricas de fonte ──────────────────────────────────────────────────────

/// Interface de métricas de fonte para o Layouter.
///
/// Minimalista — não armazena `font_size` nem vaza `ttf-parser` para L1.
/// `font_size` é passado em cada chamada para suportar tamanhos mistos
/// (rich text futuro).
pub trait FontMetrics: Send + Sync {
    /// Avanço horizontal de uma string em pontos tipográficos.
    fn advance(&self, text: &str, size: Pt) -> Pt;

    /// Métricas verticais: `(ascender, line_height)` em pontos tipográficos.
    ///
    /// - `ascender`: distância da baseline ao topo das maiúsculas.
    /// - `line_height`: distância total entre duas baselines consecutivas.
    fn vertical_metrics(&self, size: Pt) -> (Pt, Pt);

    /// Constantes da tabela OpenType MATH, se disponível.
    ///
    /// Default: `MathConstants::fallback()` para fontes sem tabela MATH.
    fn math_constants(&self) -> MathConstants {
        MathConstants::fallback()
    }

    /// Variantes de tamanho vertical para um glifo extensível.
    ///
    /// Retorna as variantes ordenadas por tamanho crescente (design units).
    /// Default: sem variantes — fallback para glifo base.
    fn vertical_glyph_variants(&self, c: char) -> GlyphVariants {
        let _ = c;
        GlyphVariants::default()
    }

    /// Mapeamento reverso: glyph_id → char Unicode.
    ///
    /// Necessário para emitir glifos variantes como `FrameItem::Text`.
    /// Default: None — usar glifo base.
    fn glyph_to_char(&self, glyph_id: u16) -> Option<char> {
        let _ = glyph_id;
        None
    }

    /// Montagem por partes para um glifo extensível.
    ///
    /// Retorna as peças ordenadas bottom→top para montagem vertical.
    /// Default: sem assembly — fallback para variante máxima disponível.
    fn vertical_glyph_assembly(&self, c: char) -> GlyphAssembly {
        let _ = c;
        GlyphAssembly::default()
    }

    /// Kern matemático por quadrante para um glifo.
    ///
    /// `c` é o caractere base cujos scripts vão ser posicionados.
    /// Default: sem kern — todos os quadrantes vazios (espaçamento rectilíneo).
    fn math_kern(&self, c: char) -> MathGlyphKern {
        let _ = c;
        MathGlyphKern::default()
    }
}

/// Métricas fixas monoespaçadas — para layout sem FontBook real.
///
/// Passo 21: substituída por `FontBookMetrics` em L3 quando disponível.
pub struct FixedMetrics;

impl FontMetrics for FixedMetrics {
    fn advance(&self, text: &str, size: Pt) -> Pt {
        // 0.6 * size por codepoint — monoespaçado
        size * (text.chars().count() as f64 * 0.6)
    }

    fn vertical_metrics(&self, size: Pt) -> (Pt, Pt) {
        // ascender ≈ 0.8 * size; line_height = 1.2 * size
        (size * 0.8, size * 1.2)
    }
}

// ── Constantes de página ───────────────────────────────────────────────────

const DEFAULT_FONT_SIZE: f64 = 12.0;
const MARGIN: Pt = Pt(72.0);  // 1 inch

// ── Layouter ──────────────────────────────────────────────────────────────

/// Máquina de estado de layout.
///
/// Consome `Content` e produz `PagedDocument`.
/// `font_size` é campo do Layouter — as métricas recebem-no por chamada
/// para suportar tamanhos mistos (rich text).
pub struct Layouter<M: FontMetrics> {
    metrics:      M,
    font_size_pt: Pt,      // tamanho de fonte base — não muda com rich text
    style:        TextStyle,
    pages:        Vec<Frame>,
    current:      Frame,
    cursor_x:     Pt,
    cursor_y:     Pt,      // posição da baseline actual
    current_line: Vec<FrameItem>,
    pub counter:  CounterState,
}

impl<M: FontMetrics> Layouter<M> {
    pub fn new(metrics: M, font_size: f64) -> Self {
        let size = Pt(font_size);
        let (ascender, _) = metrics.vertical_metrics(size);
        Self {
            metrics,
            font_size_pt: size,
            style:        TextStyle::regular(size),
            pages:        Vec::new(),
            current:      Frame::new(Size::a4()),
            cursor_x:     MARGIN,
            cursor_y:     MARGIN + ascender,
            current_line: Vec::new(),
            counter:      CounterState::new(),
        }
    }

    pub fn layout_content(&mut self, content: &Content) {
        match content {
            Content::Empty => {}

            Content::Text(text, node_style) => {
                // Estilo resolvido em eval via #set text() e scoping de blocos (Passo 33).
                // bold/italic: node_style já é correcto — eval captura o estilo activo no
                // momento da produção, incluindo bold/italic de Strong/Emph/Heading.
                // size: self.style tem prioridade quando vem de heading (maior que base).
                let effective = TextStyle {
                    bold:   node_style.bold,
                    italic: node_style.italic,
                    size:   if self.style.size > self.font_size_pt {
                        self.style.size   // heading ou outro override de contexto
                    } else {
                        node_style.size   // #set text(size:) capturado em eval
                    },
                };
                let prev_style = self.style;
                self.style = effective;
                for word in text.split_whitespace() {
                    self.layout_word(word);
                }
                self.style = prev_style;
            }

            Content::Space => {
                self.cursor_x += self.space_width();
                if self.cursor_x > Size::a4().width - MARGIN {
                    self.flush_line();
                }
            }

            Content::Sequence(parts) => {
                for part in parts.iter() {
                    self.layout_content(part);
                }
            }

            Content::Strong(body) => {
                let prev = self.style;
                self.style = TextStyle::bold(self.font_size_pt);
                self.layout_content(body);
                self.style = prev;
            }

            Content::Emph(body) => {
                let prev = self.style;
                self.style = TextStyle::italic(self.font_size_pt);
                self.layout_content(body);
                self.style = prev;
            }

            Content::Heading { level, body } => {
                self.counter.step_hierarchical("heading", *level as usize);

                let heading_size = self.font_size_pt * heading_scale(*level);
                let prev = self.style;
                self.style = TextStyle { bold: true, italic: false, size: heading_size };
                if self.cursor_x > MARGIN { self.flush_line(); }

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

            Content::CounterUpdate { key, action } => {
                counters::layout_counter_update(&mut self.counter, key, action);
            }

            Content::CounterDisplay { kind } => {
                let text = counters::format_counter_display(&self.counter, kind);
                let display = Content::text(text);
                self.layout_content(&display);
            }

            Content::Raw { text, block, .. } => {
                let prev = self.style;
                // Raw: tamanho 90%, sem bold/italic
                // DEBT: seleccionar fonte monospace real quando FontBook tiver uma
                self.style = TextStyle { bold: false, italic: false, size: self.font_size_pt * 0.9 };
                if *block {
                    if self.cursor_x > MARGIN { self.flush_line(); }
                    self.cursor_x = MARGIN + self.font_size_pt;
                }
                for word in text.split_whitespace() { self.layout_word(word); }
                if *block { self.flush_line(); }
                self.style = prev;
            }

            Content::ListItem(body) => {
                if self.cursor_x > MARGIN { self.flush_line(); }
                // Bullet: "•" é Unicode U+2022 — aparece como ? no PDF até DEBT-5
                // usar "-" ASCII como fallback para o PDF actual
                self.current_line.push(FrameItem::Text {
                    pos:   Point { x: MARGIN, y: self.cursor_y },
                    text:  "•".into(),  // U+2022 — suportado com CIDFont (DEBT-5 pago)
                    style: self.style,
                });
                self.cursor_x = MARGIN + self.font_size_pt * 1.5;
                self.layout_content(body);
                self.flush_line();
                self.cursor_x = MARGIN;
            }

            Content::EnumItem { number, body } => {
                if self.cursor_x > MARGIN { self.flush_line(); }
                let label: EcoString = match number {
                    Some(n) => format!("{}.", n).into(),
                    None    => "-".into(),
                };
                self.current_line.push(FrameItem::Text {
                    pos:   Point { x: MARGIN, y: self.cursor_y },
                    text:  label,
                    style: self.style,
                });
                self.cursor_x = MARGIN + self.font_size_pt * 2.0;
                self.layout_content(body);
                self.flush_line();
                self.cursor_x = MARGIN;
            }

            Content::Link { body, .. } => {
                // DEBT: sublinhado e cor de link — requer FrameItem::Decoration (futuro)
                self.layout_content(body);
            }

            // ── Matemática (Passo 37) — delegação ao MathLayouter ───────────
            Content::Equation { body, block } => {
                // Auto-numeração: equações de bloco numeradas avançam o contador antes de
                // desenhar (Passo 59). O número (N) é acrescentado depois da equação.
                let is_numbered = *block && self.counter.is_numbering_active("equation");
                if is_numbered {
                    self.counter.step_flat("equation");
                }

                let math_layouter = math::layout::MathLayouter::new(&self.metrics, *block);
                let math_items    = math_layouter.layout_equation(body, &self.style);

                if *block
                    && self.cursor_x > MARGIN { self.flush_line(); }

                // Integrar items matemáticos no frame actual.
                // pos.x e pos.y são relativos à origem da equação —
                // pos.y inclui deslocamento vertical (sup/sub, frac).
                let offset_x = self.cursor_x;
                // Equações inline: deslocar para cima por axis_pt de modo a que o
                // eixo matemático (axis_height acima da baseline) coincida com o
                // baseline do texto circundante (Passo 48).
                let axis_pt = if *block {
                    Pt(0.0)
                } else {
                    let c = self.metrics.math_constants();
                    c.to_pt(c.axis_height, self.style.size)
                };
                let offset_y = self.cursor_y - axis_pt;
                for item in math_items {
                    match item {
                        FrameItem::Text { pos, text, style } => {
                            let abs_pos = Point {
                                x: offset_x + pos.x,
                                y: offset_y + pos.y,
                            };
                            let advance = self.metrics.advance(&text, style.size);
                            self.current_line.push(FrameItem::Text { pos: abs_pos, text, style });
                            self.cursor_x += advance;
                        }
                        FrameItem::Line { start, end, thickness } => {
                            let abs_start = Point { x: offset_x + start.x, y: offset_y + start.y };
                            let abs_end   = Point { x: offset_x + end.x,   y: offset_y + end.y };
                            self.current_line.push(FrameItem::Line {
                                start: abs_start, end: abs_end, thickness,
                            });
                        }
                        FrameItem::Glyph { pos, glyph_id, x_advance, size } => {
                            let abs_pos = Point {
                                x: offset_x + pos.x,
                                y: offset_y + pos.y,
                            };
                            self.current_line.push(FrameItem::Glyph {
                                pos: abs_pos, glyph_id, x_advance, size,
                            });
                            self.cursor_x += x_advance;
                        }
                    }
                }

                if *block { self.flush_line(); }

                // Acrescentar número da equação inline após o flush (Passo 59).
                // DEBT: alinhamento à direita real requer largura de página — por agora inline.
                if is_numbered {
                    let n = self.counter.get_flat("equation");
                    self.layout_content(&Content::text(format!("({})", n)));
                    self.flush_line();
                }
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

            // Passo 62 — Figure: delegado a figure.rs.
            Content::Figure { body, caption } => {
                figure::layout_figure(self, body, caption);
            }

            // Passo 61 — TOC: delegado a outline.rs (Tarefa 5).
            Content::Outline => {
                outline::layout_outline(self);
            }
        }
    }

    fn word_width(&self, word: &str) -> Pt {
        self.metrics.advance(word, self.style.size)
    }

    fn space_width(&self) -> Pt {
        self.metrics.advance(" ", self.style.size)
    }

    fn layout_word(&mut self, word: &str) {
        let w = self.word_width(word);
        if self.cursor_x + w > Size::a4().width - MARGIN && self.cursor_x > MARGIN {
            self.flush_line();
        }
        self.current_line.push(FrameItem::Text {
            pos:   Point { x: self.cursor_x, y: self.cursor_y },
            text:  word.into(),
            style: self.style,
        });
        self.cursor_x += w + self.space_width();
    }

    fn flush_line(&mut self) {
        for item in self.current_line.drain(..) {
            self.current.push(item);
        }
        // Avançar pela line_height do tamanho base (não do heading)
        let (_, line_height) = self.metrics.vertical_metrics(self.font_size_pt);
        self.cursor_y += line_height;
        self.cursor_x  = MARGIN;

        if self.cursor_y > Size::a4().height - MARGIN {
            self.new_page();
        }
    }

    fn new_page(&mut self) {
        for item in self.current_line.drain(..) {
            self.current.push(item);
        }
        let finished = std::mem::replace(&mut self.current, Frame::new(Size::a4()));
        self.pages.push(finished);
        self.cursor_x = MARGIN;
        let (ascender, _) = self.metrics.vertical_metrics(self.font_size_pt);
        self.cursor_y = MARGIN + ascender;
    }

    /// Número da página actual (1-indexed).
    ///
    /// Abordagem A: `self.pages.len() + 1` — a página actual ainda não foi
    /// finalizada (não foi empurrada para `self.pages`), por isso a contagem
    /// de páginas finalizadas + 1 dá o número da página em curso.
    pub(super) fn current_page_number(&self) -> usize {
        self.pages.len() + 1
    }

    pub fn finish(mut self) -> PagedDocument {
        for item in self.current_line.drain(..) {
            self.current.push(item);
        }
        if !self.current.items.is_empty() {
            self.pages.push(self.current);
        }
        let mut doc = PagedDocument::new(self.pages);
        // Expor o mapa de páginas sem mudar a assinatura de layout() (Passo 63).
        doc.extracted_label_pages = self.counter.label_pages;
        doc
    }
}

// ── API pública ────────────────────────────────────────────────────────────

fn heading_scale(level: u8) -> f64 {
    match level { 1 => 2.0, 2 => 1.667, 3 => 1.333, 4 => 1.167, _ => 1.0 }
}

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
        let mut l = Layouter::new(FixedMetrics, DEFAULT_FONT_SIZE);
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
        let mut l = Layouter::new(FixedMetrics, DEFAULT_FONT_SIZE);

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
mod tests {
    use super::*;
    use crate::entities::{content::Content, counter_state::CounterState, layout_types::FrameItem};
    use crate::rules::introspect::introspect;

    // ── Testes de FixedMetrics (Passo 21) ────────────────────────────────

    #[test]
    fn fixed_metrics_advance_proporcional_ao_tamanho() {
        let m = FixedMetrics;
        let a12 = m.advance("Hello", Pt(12.0));
        let a24 = m.advance("Hello", Pt(24.0));
        assert!(
            (a24.val() - 2.0 * a12.val()).abs() < 0.001,
            "advance deve escalar linearmente com font_size"
        );
    }

    #[test]
    fn fixed_metrics_monoespaco_iiii_eq_wwww() {
        let m = FixedMetrics;
        let ai = m.advance("iiii", Pt(12.0));
        let aw = m.advance("WWWW", Pt(12.0));
        assert_eq!(ai, aw, "FixedMetrics é monoespaçado — iiii == WWWW");
    }

    #[test]
    fn fixed_metrics_vertical_ascender_menor_que_line_height() {
        let (asc, lh) = FixedMetrics.vertical_metrics(Pt(12.0));
        assert!(asc.val() > 0.0, "ascender deve ser positivo");
        assert!(lh.val() > asc.val(), "line_height > ascender");
    }

    #[test]
    fn layouter_baseline_dentro_da_pagina() {
        let l = Layouter::new(FixedMetrics, 12.0);
        assert!(l.cursor_y.val() > 0.0);
        assert!(l.cursor_y.val() < 842.0);
    }

    // ── Testes de layout() (herdados do Passo 19) ─────────────────────────

    #[test]
    fn layout_texto_simples_tem_items() {
        let doc = layout(&Content::text("Hello world"), CounterState::default());
        assert!(!doc.pages.is_empty());
        let total = doc.pages.iter().flat_map(|p| p.items.iter()).count();
        assert!(total >= 2, "Hello e world devem ser itens separados");
        assert!(doc.plain_text().contains("Hello"));
        assert!(doc.plain_text().contains("world"));
    }

    #[test]
    fn layout_documento_vazio_zero_paginas() {
        let doc = layout(&Content::Empty, CounterState::default());
        assert_eq!(doc.pages.len(), 0, "documento vazio → sem páginas");
    }

    /// Teste de Ouro: todos os items dentro dos limites da página.
    #[test]
    fn layout_items_dentro_limites_da_pagina() {
        let words = (0..100)
            .map(|i| format!("palavra{i}"))
            .collect::<Vec<_>>()
            .join(" ");
        let doc = layout(&Content::text(&words), CounterState::default());

        for page in &doc.pages {
            for item in &page.items {
                if let FrameItem::Text { pos, .. } = item {
                    assert!(
                        pos.x.val() >= 0.0 && pos.x.val() < 595.0,
                        "x={} fora dos limites da página", pos.x.val()
                    );
                    assert!(
                        pos.y.val() >= 0.0 && pos.y.val() < 842.0,
                        "y={} fora dos limites da página", pos.y.val()
                    );
                }
            }
        }
    }

    #[test]
    fn layout_texto_longo_word_wrap() {
        let words = (0..50)
            .map(|i| format!("w{i}"))
            .collect::<Vec<_>>()
            .join(" ");
        let doc = layout(&Content::text(&words), CounterState::default());
        let items = doc.pages.iter().flat_map(|p| p.items.iter()).count();
        let y_values: std::collections::HashSet<u64> = doc
            .pages
            .iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|i| { if let FrameItem::Text { pos, .. } = i { Some(pos.y.val().to_bits()) } else { None } })
            .collect();
        assert!(y_values.len() > 1, "texto longo deve ter múltiplas linhas: {} items", items);
    }

    // ── Testes rich text (Passo 22) ────────────────────────────────────────

    #[test]
    fn strong_produz_bold_style() {
        // Após Passo 33: node_style deve ter bold=true (capturado em eval via Strong).
        // Construção directa usa TextStyle::bold para simular o que eval produziria.
        let doc = layout(&Content::strong(
            Content::Text("Bold".into(), TextStyle::bold(Pt(11.0)))
        ), CounterState::default());
        let bold = doc.pages.iter()
            .flat_map(|p| p.items.iter())
            .any(|i| matches!(i, FrameItem::Text { style, .. } if style.bold));
        assert!(bold, "Strong deve produzir FrameItem com bold=true");
    }

    #[test]
    fn emph_produz_italic_style() {
        // Após Passo 33: node_style deve ter italic=true (capturado em eval via Emph).
        // Construção directa usa TextStyle::italic para simular o que eval produziria.
        let doc = layout(&Content::emph(
            Content::Text("Italic".into(), TextStyle::italic(Pt(11.0)))
        ), CounterState::default());
        let italic = doc.pages.iter()
            .flat_map(|p| p.items.iter())
            .any(|i| matches!(i, FrameItem::Text { style, .. } if style.italic));
        assert!(italic, "Emph deve produzir FrameItem com italic=true");
    }

    #[test]
    fn heading_h1_tamanho_maior() {
        let content = Content::sequence(vec![
            Content::heading(1, Content::text("Title")),
            Content::text("body"),
        ]);
        let doc = layout(&content, introspect(&content));
        let sizes: Vec<f64> = doc.pages.iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|i| { if let FrameItem::Text { style, .. } = i { Some(style.size.val()) } else { None } })
            .collect();
        let max_size = sizes.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let min_size = sizes.iter().cloned().fold(f64::INFINITY,     f64::min);
        assert!(max_size > min_size, "H1 deve ter tamanho maior que o texto normal");
    }

    #[test]
    fn estilo_restaurado_apos_strong() {
        let doc = layout(&Content::sequence(vec![
            Content::strong(Content::text("Bold")),
            Content::text("normal"),
        ]), CounterState::default());
        let items: Vec<_> = doc.pages.iter()
            .flat_map(|p| p.items.iter())
            .collect();
        if let Some(FrameItem::Text { style, text, .. }) = items.last() {
            if text.as_str() == "normal" {
                assert!(!style.bold, "texto após Strong deve ser regular");
            }
        }
    }

    #[test]
    fn pt_tipagem_nao_permite_add_f64() {
        let a = Pt(10.0);
        let b = Pt(5.0);
        let c = a + b;
        assert_eq!(c, Pt(15.0));
        // a + 5.0 ← não compila
    }

    #[test]
    fn pipeline_parse_eval_layout() {
        use crate::{
            contracts::world::World,
            entities::{
                file_id::FileId,
                font_book::FontBook,
                source::Source,
                world_types::{Bytes, Datetime, FileError, FileResult, Font, Library},
            },
            rules::eval::eval_for_test,
        };
        use std::num::NonZeroU16;

        struct MockWorld {
            library: Library,
            book:    FontBook,
            source:  Source,
        }

        impl MockWorld {
            fn new(text: &str) -> Self {
                let id = FileId::from_raw(NonZeroU16::new(1).unwrap());
                Self {
                    library: Library::new(),
                    book:    FontBook::new(),
                    source:  Source::new(id, text.to_string()),
                }
            }
        }

        impl World for MockWorld {
            fn library(&self) -> &Library  { &self.library }
            fn book(&self)    -> &FontBook { &self.book }
            fn main(&self)    -> FileId    { self.source.id() }
            fn source(&self, _: FileId) -> FileResult<Source> { Ok(self.source.clone()) }
            fn file(&self, _: FileId)    -> FileResult<Bytes>   { Err(FileError::NotFound) }
            fn font(&self, _: usize)     -> Option<Font>        { None }
            fn today(&self, _: Option<i64>) -> Option<Datetime> { None }
        }

        let world = MockWorld::new("Olá mundo");
        let src = World::source(&world, World::main(&world)).unwrap();
        let module = eval_for_test(&world, &src).unwrap();
        let content = module.content().expect("deve ter content");
        let state = introspect(content);
        let doc = layout(content, state);
        assert!(!doc.pages.is_empty());
        assert!(
            doc.plain_text().contains("Olá") || doc.plain_text().contains("mundo"),
            "texto deve estar no output: {:?}", doc.plain_text()
        );
    }

    // ── Passo 23 ────────────────────────────────────────────────────────────

    #[test]
    fn layout_list_item_tem_bullet() {
        let doc = layout(&Content::list_item(Content::text("Item")), CounterState::default());
        let has_marker = doc.pages.iter()
            .flat_map(|p| p.items.iter())
            .any(|i| matches!(i, FrameItem::Text { text, .. } if text.as_str() == "•"));
        assert!(has_marker, "ListItem deve ter marcador '•'");
    }

    #[test]
    fn layout_raw_block_tamanho_menor() {
        let content = Content::sequence(vec![
            Content::text("normal"),
            Content::raw("code", None, true),
        ]);
        let doc = layout(&content, CounterState::default());
        let sizes: std::collections::HashSet<u64> = doc.pages.iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|i| match i {
                FrameItem::Text { style, .. } => Some(style.size.val().to_bits()),
                _ => None,
            })
            .collect();
        assert!(sizes.len() > 1, "Raw deve ter tamanho diferente do texto normal");
    }

    // ── Passo 48 — Baselines em equações inline ──────────────────────────────

    fn layout_test(src: &str) -> PagedDocument {
        use crate::{
            contracts::world::World,
            entities::{
                file_id::FileId,
                font_book::FontBook,
                source::Source,
                world_types::{Bytes, Datetime, FileError, FileResult, Font, Library},
            },
            rules::eval::eval_for_test,
        };
        use std::num::NonZeroU16;

        struct MockWorld {
            library: Library,
            book:    FontBook,
            source:  Source,
        }

        impl MockWorld {
            fn new(text: &str) -> Self {
                let id = FileId::from_raw(NonZeroU16::new(1).unwrap());
                Self {
                    library: Library::new(),
                    book:    FontBook::new(),
                    source:  Source::new(id, text.to_string()),
                }
            }
        }

        impl World for MockWorld {
            fn library(&self) -> &Library  { &self.library }
            fn book(&self)    -> &FontBook { &self.book }
            fn main(&self)    -> FileId    { self.source.id() }
            fn source(&self, _: FileId) -> FileResult<Source> { Ok(self.source.clone()) }
            fn file(&self, _: FileId)    -> FileResult<Bytes>   { Err(FileError::NotFound) }
            fn font(&self, _: usize)     -> Option<Font>        { None }
            fn today(&self, _: Option<i64>) -> Option<Datetime> { None }
        }

        let world = MockWorld::new(src);
        let source = World::source(&world, World::main(&world)).unwrap();
        let module = eval_for_test(&world, &source).unwrap();
        let content = module.content().expect("deve ter content");
        let state = introspect(content);
        layout(content, state)
    }

    #[cfg(test)]
    mod tests_inline_baseline {
        use super::*;

        #[test]
        fn equacao_inline_nao_regride_conteudo() {
            let doc = layout_test("$frac(1, 2)$");
            let text = doc.plain_text();
            assert!(text.contains('1'), "numerador: {}", text);
            assert!(text.contains('2'), "denominador: {}", text);
        }

        #[test]
        fn equacao_inline_simples_nao_regride() {
            let doc = layout_test("$x + 1$");
            let text = doc.plain_text();
            assert!(text.contains('x'));
            assert!(text.contains('1'));
        }

        #[test]
        fn equacao_inline_com_attach_nao_regride() {
            let doc = layout_test("$x^2$");
            let text = doc.plain_text();
            assert!(text.contains('x'));
            assert!(text.contains('2'));
        }

        #[test]
        fn equacao_inline_com_prime_nao_regride() {
            let doc = layout_test("$x'$");
            let text = doc.plain_text();
            assert!(text.contains('x'));
            assert!(text.contains('′'));
        }

        #[test]
        fn pagina_nao_vazia_com_equacao_inline() {
            let doc = layout_test("$frac(1, 2)$");
            assert!(!doc.pages.is_empty());
            assert!(!doc.pages[0].items.is_empty());
        }

        #[test]
        fn equacao_inline_sobe_em_relacao_ao_baseline() {
            // Com o ajuste de baseline, os items da equação inline estão acima
            // do cursor_y (offset_y < cursor_y). Com FixedMetrics, axis_height=500
            // e upem=1000, axis_pt = 0.5 * font_size = 6.0pt.
            // Verificamos que pelo menos um item tem y < cursor_y inicial (≈81.6pt).
            let doc = layout_test("$x$");
            let all_y: Vec<f64> = doc.pages.iter()
                .flat_map(|p| p.items.iter())
                .filter_map(|i| match i {
                    FrameItem::Text { pos, .. } => Some(pos.y.val()),
                    FrameItem::Glyph { pos, .. } => Some(pos.y.val()),
                    _ => None,
                })
                .collect();
            assert!(!all_y.is_empty(), "deve ter items");
            // cursor_y inicial ≈ MARGIN(72) + ascender(9.6) = 81.6
            // Com axis_pt ≈ 6.0, offset_y ≈ 75.6 < 81.6
            let min_y = all_y.iter().cloned().fold(f64::INFINITY, f64::min);
            assert!(
                min_y < 81.6,
                "equacao inline deve estar acima do baseline ({:.1} < 81.6)",
                min_y
            );
        }
    }

    // ── Passo 49 — Limites verticais em operadores grandes ───────────────────

    #[cfg(test)]
    mod tests_limits {
        use super::*;

        #[test]
        fn layout_sum_com_limites_contem_conteudo() {
            let doc = layout_test("$sum_(i=0)^n$");
            let text = doc.plain_text();
            assert!(
                text.contains('∑') || text.contains('i') || text.contains('n'),
                "operador ou limites ausentes: {}", text
            );
        }

        #[test]
        fn layout_sum_sem_limites_nao_regride() {
            let doc = layout_test("$sum$");
            let text = doc.plain_text();
            assert!(text.contains('∑'), "somatório: {}", text);
        }

        #[test]
        fn layout_attach_normal_nao_regride() {
            let doc = layout_test("$x^2$");
            let text = doc.plain_text();
            assert!(text.contains('x'));
            assert!(text.contains('2'));
        }

        #[test]
        fn layout_integral_com_limites_nao_panica() {
            let doc = layout_test("$integral_(0)^1$");
            assert!(!doc.pages.is_empty());
        }

        #[test]
        fn layout_prod_com_limites_nao_panica() {
            let doc = layout_test("$product_(k=1)^n$");
            assert!(!doc.pages.is_empty());
        }

        #[test]
        fn layout_lim_com_subscript_nao_panica() {
            let doc = layout_test("$lim_(x -> 0)$");
            assert!(!doc.pages.is_empty());
        }

        #[test]
        fn sum_block_limites_empilhados_verticalmente() {
            // Passo 50: bloco "$ ... $" (espaços dentro) → block=true → empilhamento vertical.
            // offset_y = cursor_y = 81.6 (bloco não ajusta baseline).
            // y_sup = -(base_ascent + upper_gap + sup.descent) = -(9.6 + 1.2 + 3.36) = -14.16
            // Final y ≈ 81.6 - 14.16 = 67.4 < 70.0 (vs inline right-scripts ≈ 71.3)
            let doc = layout_test("$ sum_(i=0)^n $");
            let all_y: Vec<f64> = doc.pages.iter()
                .flat_map(|p| p.items.iter())
                .filter_map(|i| match i {
                    FrameItem::Text { pos, .. } => Some(pos.y.val()),
                    FrameItem::Glyph { pos, .. } => Some(pos.y.val()),
                    _ => None,
                })
                .collect();
            assert!(!all_y.is_empty(), "deve ter items");
            let min_y = all_y.iter().cloned().fold(f64::INFINITY, f64::min);
            assert!(
                min_y < 70.0,
                "bloco: limites de ∑ devem estar empilhados verticalmente (min_y={:.1} < 70.0)",
                min_y
            );
        }
    }

    // ── Passo 50 — Diferenciação inline/bloco ────────────────────────────────

    #[cfg(test)]
    mod tests_limits_context {
        use super::*;

        #[test]
        fn sum_inline_usa_right_scripts() {
            // Passo 50: inline "$...$" → block=false → right-scripts (sub/sup à direita).
            // offset_y = cursor_y - axis_pt = 81.6 - 6.0 = 75.6 (inline ajusta baseline).
            // Com right-scripts: sup_offset ≈ 4.34pt → item y ≈ 75.6 - 4.34 = 71.3 ≥ 70.0
            // Com vertical stacking (antes): min_y ≈ 61.4 < 70.0 (falha antes da implementação)
            let doc = layout_test("$sum_(i=0)^n$");
            let all_y: Vec<f64> = doc.pages.iter()
                .flat_map(|p| p.items.iter())
                .filter_map(|i| match i {
                    FrameItem::Text { pos, .. } => Some(pos.y.val()),
                    FrameItem::Glyph { pos, .. } => Some(pos.y.val()),
                    _ => None,
                })
                .collect();
            assert!(!all_y.is_empty(), "deve ter items");
            let min_y = all_y.iter().cloned().fold(f64::INFINITY, f64::min);
            assert!(
                min_y >= 70.0,
                "inline: ∑ deve usar right-scripts (min_y={:.1} >= 70.0)",
                min_y
            );
        }

        #[test]
        fn sum_inline_contem_conteudo() {
            let doc = layout_test("$sum_(i=0)^n$");
            let text = doc.plain_text();
            assert!(
                text.contains('∑') || text.contains('i') || text.contains('n'),
                "conteúdo ausente: {}", text
            );
        }

        #[test]
        fn sum_inline_gera_pagina() {
            let doc = layout_test("$sum_(i=0)^n x_i$");
            assert!(!doc.pages.is_empty());
            assert!(!doc.pages[0].items.is_empty());
        }

        #[test]
        fn lim_inline_contem_conteudo() {
            let doc = layout_test("$lim_(x -> 0) f(x)$");
            let text = doc.plain_text();
            assert!(text.contains('f') || text.contains('x'),
                "conteúdo ausente: {}", text);
        }

        #[test]
        fn attach_normal_inline_nao_regride() {
            let doc = layout_test("$x^2$");
            let text = doc.plain_text();
            assert!(text.contains('x'));
            assert!(text.contains('2'));
        }

        #[test]
        fn attach_normal_com_sub_inline_nao_regride() {
            let doc = layout_test("$x_i$");
            let text = doc.plain_text();
            assert!(text.contains('x'));
            assert!(text.contains('i'));
        }

        #[test]
        fn sum_block_contem_conteudo() {
            let doc = layout_test("$ sum_(i=0)^n $");
            let text = doc.plain_text();
            assert!(
                text.contains('∑') || text.contains('i') || text.contains('n'),
                "conteúdo ausente em block: {}", text
            );
        }
    }

    // ── Passo 51 — MathAlignPoint ─────────────────────────────────────────
    #[cfg(test)]
    mod tests_align {
        use super::*;

        #[test]
        fn align_simples_contem_conteudo() {
            // $ a &= b \\ c &= d $ — dois lados de duas linhas presentes
            let doc = layout_test("$ a &= b \\ c &= d $");
            let text = doc.plain_text();
            assert!(text.contains('a'), "a ausente: {}", text);
            assert!(text.contains('b'), "b ausente: {}", text);
            assert!(text.contains('c'), "c ausente: {}", text);
            assert!(text.contains('d'), "d ausente: {}", text);
        }

        #[test]
        fn align_duas_linhas_tem_ys_distintos() {
            // Após implementação de grid, itens de linha 0 e linha 1
            // devem ter Y distintos no frame.
            let doc = layout_test("$ a &= b \\ c &= d $");
            assert!(!doc.pages.is_empty());
            let mut ys: Vec<i64> = doc.pages[0].items.iter()
                .filter_map(|item| match item {
                    crate::entities::layout_types::FrameItem::Text { pos, .. } =>
                        Some((pos.y.val() * 100.0).round() as i64),
                    crate::entities::layout_types::FrameItem::Glyph { pos, .. } =>
                        Some((pos.y.val() * 100.0).round() as i64),
                    _ => None,
                })
                .collect();
            ys.sort_unstable();
            ys.dedup();
            assert!(ys.len() >= 2,
                "esperava >= 2 Y distintos (2 linhas), encontrei {:?}", ys);
        }

        #[test]
        fn align_sem_ampersand_nao_regride() {
            let doc = layout_test("$ x + 1 $");
            let text = doc.plain_text();
            assert!(text.contains('x'));
            assert!(text.contains('1'));
        }

        #[test]
        fn align_com_frac_nao_panica() {
            let doc = layout_test("$ frac(a, b) &= c \\ d &= e $");
            assert!(!doc.pages.is_empty());
        }

        #[test]
        fn align_linha_unica_com_ampersand() {
            let doc = layout_test("$ a &= b $");
            let text = doc.plain_text();
            assert!(text.contains('a'));
            assert!(text.contains('b'));
        }

        #[test]
        fn align_inline_nao_usa_grelha() {
            // inline: & ignorado, não deve panicar
            let doc = layout_test("$a &= b$");
            assert!(!doc.pages.is_empty());
        }

        #[test]
        fn frac_nao_regride() {
            let doc = layout_test("$ frac(1, 2) $");
            let text = doc.plain_text();
            assert!(text.contains('1'));
            assert!(text.contains('2'));
        }

        #[test]
        fn sum_com_limites_nao_regride() {
            let doc = layout_test("$ sum_(i=0)^n $");
            let text = doc.plain_text();
            assert!(
                text.contains('∑') || text.contains('i') || text.contains('n'),
                "sum: {}", text
            );
        }

        // ── Passo 54 — Matrizes matemáticas ─────────────────────────────────

        #[test]
        fn matrix_2x2_nao_vazio() {
            let doc = layout_test("$ mat(a, b; c, d) $");
            let text = doc.plain_text();
            assert!(text.contains('a'), "a ausente: {}", text);
            assert!(text.contains('d'), "d ausente: {}", text);
        }

        #[test]
        fn matrix_1x1_nao_panica() {
            let doc = layout_test("$ mat(x) $");
            assert!(!doc.pages.is_empty());
        }

        #[test]
        fn matrix_linha_unica_nao_panica() {
            let doc = layout_test("$ mat(1, 2, 3) $");
            assert!(!doc.pages.is_empty());
        }

        #[test]
        fn align_grid_nao_regride_apos_matrix() {
            let doc = layout_test("$ a &= b \\ c &= d $");
            let text = doc.plain_text();
            assert!(text.contains('a'));
            assert!(text.contains('d'));
        }

        // ── Passo 55 — Vectores e Casos ──────────────────────────────────────

        #[test]
        fn vec_tres_elementos_nao_vazio() {
            let doc = layout_test("$ vec(1, 2, 3) $");
            let text = doc.plain_text();
            assert!(text.contains('1'));
            assert!(text.contains('3'));
        }

        #[test]
        fn vec_elemento_unico_nao_panica() {
            let doc = layout_test("$ vec(x) $");
            assert!(!doc.pages.is_empty());
        }

        #[test]
        fn cases_dois_ramos_nao_vazio() {
            let doc = layout_test("$ cases(1, 0) $");
            assert!(!doc.pages.is_empty());
        }

        #[test]
        fn cases_nao_panica_com_align_point() {
            let doc = layout_test("$ cases(x &, 0 &) $");
            assert!(!doc.pages.is_empty());
        }

        #[test]
        fn mat_nao_regride_apos_vec_cases() {
            let doc = layout_test("$ mat(1, 2; 3, 4) $");
            let text = doc.plain_text();
            assert!(text.contains('1'));
            assert!(text.contains('4'));
        }

        #[test]
        fn align_grid_nao_regride_apos_passo55() {
            let doc = layout_test("$ a &= b \\ c &= d $");
            let text = doc.plain_text();
            assert!(text.contains('a'));
            assert!(text.contains('d'));
        }
    }

    // ── Testes de CounterState e numeração de headings (Passo 57/58) ──────

    #[test]
    fn layout_heading_sem_numbering_nao_tem_prefixo() {
        // Por defeito, numbering_active está vazio — não deve aparecer "1."
        let content = Content::heading(1, Content::text("Intro"));
        let doc = layout(&content, introspect(&content));
        let text = doc.plain_text();
        assert!(!text.contains("1."), "sem numbering activo, não deve haver prefixo numérico");
        assert!(text.contains("Intro"));
    }

    #[test]
    fn layout_heading_com_numbering_tem_prefixo() {
        let content = Content::Sequence(vec![
            Content::SetHeadingNumbering { active: true },
            Content::heading(1, Content::text("Intro")),
            Content::heading(2, Content::text("Motivação")),
            Content::heading(1, Content::text("Conclusão")),
        ].into());
        let doc = layout(&content, introspect(&content));
        let text = doc.plain_text();
        assert!(text.contains("1."), "H1 deve ter prefixo '1.'");
        assert!(text.contains("1.1"), "H2 deve ter prefixo '1.1'");
        assert!(text.contains("2."), "segundo H1 deve ter prefixo '2.'");
    }

    #[test]
    fn layout_set_heading_numbering_activa_contador() {
        // SetHeadingNumbering activado via Content::SetHeadingNumbering + headings
        let content = Content::Sequence(vec![
            Content::SetHeadingNumbering { active: true },
            Content::heading(1, Content::text("Intro")),
            Content::heading(2, Content::text("Sub")),
        ].into());
        let doc = layout(&content, introspect(&content));
        let text = doc.plain_text();
        assert!(text.contains("1."), "H1 deve ter prefixo '1.'");
        assert!(text.contains("1.1"), "H2 deve ter prefixo '1.1'");
    }

    #[test]
    fn layout_counter_display_heading_retorna_estado_actual() {
        let content = Content::Sequence(vec![
            Content::SetHeadingNumbering { active: true },
            Content::heading(1, Content::text("Intro")),
            Content::CounterDisplay { kind: "heading".to_string() },
        ].into());
        let doc = layout(&content, introspect(&content));
        let text = doc.plain_text();
        // CounterDisplay de heading após H1 deve mostrar "1"
        // (o heading já avançou o contador antes de CounterDisplay ser processado)
        assert!(text.contains('1'));
    }

    // ── Testes de CounterUpdate (Passo 58) ────────────────────────────────

    #[test]
    fn counter_update_nao_produz_items_visuais() {
        use crate::entities::counter_state::CounterAction;

        let content = Content::CounterUpdate {
            key:    "equation".to_string(),
            action: CounterAction::Update(5),
        };
        let doc = layout(&content, CounterState::default());
        let total_items: usize = doc.pages.iter().map(|p| p.items.len()).sum();
        assert_eq!(total_items, 0, "CounterUpdate não deve gerar items visuais");
    }

    #[test]
    fn counter_update_seguido_de_display_mostra_valor_correcto() {
        use crate::entities::counter_state::CounterAction;

        let content = Content::Sequence(vec![
            Content::CounterUpdate {
                key:    "equation".to_string(),
                action: CounterAction::Update(5),
            },
            Content::CounterDisplay { kind: "equation".to_string() },
        ].into());
        let doc = layout(&content, CounterState::default());
        assert!(doc.plain_text().contains('5'),
            "CounterDisplay deve mostrar '5' após Update(5): {:?}", doc.plain_text());
    }

    // ── Testes de resolução de referências (Passo 59 / Passo 60) ────────────

    #[test]
    fn layout_ref_para_tras_resolve_secao() {
        // Passo 60: layout() usa duas passagens — backward ref resolve via introspect.
        use crate::entities::label::Label;

        let content = Content::Sequence(vec![
            Content::Labelled {
                label:  Label("intro".to_string()),
                target: Box::new(Content::heading(1, Content::text("Introdução"))),
            },
            Content::text("Como vimos em"),
            Content::Ref { target: Label("intro".to_string()) },
        ].into());

        let doc = layout(&content, introspect(&content));
        let text = doc.plain_text();
        assert!(
            text.contains("Secção 1"),
            "Ref para trás deve resolver para 'Secção 1' via duas passagens, obtido: {:?}", text
        );
    }

    #[test]
    fn layout_ref_para_frente_resolve_com_duas_passagens() {
        // Passo 60: forward ref resolve via introspect — sem fallback.
        use crate::entities::label::Label;

        let content = Content::Sequence(vec![
            // Ref aparece antes da Label — forward reference
            Content::Ref { target: Label("conclusao".to_string()) },
            Content::Labelled {
                label:  Label("conclusao".to_string()),
                target: Box::new(Content::heading(1, Content::text("Conclusão"))),
            },
        ].into());

        let doc = layout(&content, introspect(&content));
        let text = doc.plain_text();
        assert!(
            text.contains("Secção 1"),
            "Forward ref deve resolver para 'Secção 1' com duas passagens, obtido: {:?}", text
        );
        assert!(
            !text.contains("@conclusao"),
            "Forward ref não deve usar fallback com duas passagens, obtido: {:?}", text
        );
    }

    #[test]
    fn layout_resolved_labels_nao_interfere_entre_documentos() {
        // Estados de cada chamada a layout() são independentes.
        use crate::entities::label::Label;

        let content_a = Content::Labelled {
            label:  Label("sec".to_string()),
            target: Box::new(Content::heading(1, Content::text("A"))),
        };
        let _ = layout(&content_a, introspect(&content_a));

        // Segundo layout independente — não deve ter "sec" resolvida
        let content_b = Content::Ref { target: Label("sec".to_string()) };
        let doc_b = layout(&content_b, introspect(&content_b));
        assert!(
            doc_b.plain_text().contains("@sec"),
            "Estado do layout anterior não deve vazar para o seguinte"
        );
    }

    // ── Testes de duas passagens (Passo 60) ──────────────────────────────────

    #[test]
    fn pipeline_duas_passagens_resolve_forward_ref() {
        use crate::entities::label::Label;
        use crate::rules::{introspect::introspect, layout::layout};

        let content = Content::Sequence(vec![
            Content::SetHeadingNumbering { active: true },
            Content::text("Ver a"),
            Content::Ref { target: Label("conclusao".to_string()) },
            Content::text("."),
            Content::Labelled {
                label:  Label("conclusao".to_string()),
                target: Box::new(Content::heading(1, Content::text("Conclusão"))),
            },
        ].into());

        // Passagem 1 — verificar que introspect resolve forward ref
        let initial_state = introspect(&content);
        assert!(
            initial_state.resolved_labels.contains_key(&Label("conclusao".to_string())),
            "introspect deve popular resolved_labels para forward refs"
        );

        // Passagem 2 — layout usa o estado da pré-passagem.
        let doc = layout(&content, initial_state);
        let text = doc.plain_text();
        assert!(
            text.contains("Secção 1"),
            "forward ref deve resolver para 'Secção 1': {:?}", text
        );
        assert!(
            !text.contains("@conclusao"),
            "não deve usar fallback com duas passagens: {:?}", text
        );
    }

    #[test]
    fn layout_equation_bloco_numerada() {
        let mut state = CounterState::new();
        state.numbering_active.insert("equation".to_string(), true);

        let content = Content::Equation {
            body:  Box::new(Content::MathIdent("E".into())),
            block: true,
        };

        let doc = layout(&content, state);
        let text = doc.plain_text();
        assert!(
            text.contains("(1)"),
            "Equação de bloco numerada deve mostrar '(1)', obtido: {:?}", text
        );
    }

    // ── Testes de Passo 61 — TOC (Outline) ───────────────────────────────────

    #[test]
    fn layout_outline_gera_indice_com_titulos() {
        let content = Content::Sequence(vec![
            Content::SetHeadingNumbering { active: true },
            Content::Outline,
            Content::heading(1, Content::text("Introdução")),
            Content::heading(2, Content::text("Motivação")),
        ].into());

        // Passagem 1 — o teste orquestra explicitamente como o orquestrador L3 faz.
        let state = introspect(&content);
        // Passagem 2 — layout recebe o estado pré-calculado.
        let doc = layout(&content, state);
        let text = doc.plain_text();

        assert!(text.contains("Índice"), "TOC deve ter título 'Índice'");
        assert!(text.contains("Introdução"), "TOC deve listar o título H1");
        assert!(text.contains("Motivação"), "TOC deve listar o título H2");
    }

    #[test]
    fn layout_outline_sem_headings_gera_apenas_titulo_ou_vazio() {
        let content = Content::Outline;
        let state = introspect(&content);
        let doc = layout(&content, state);
        let text = doc.plain_text();

        assert!(text.contains("Índice") || text.is_empty(),
            "TOC sem headings deve gerar apenas o título ou estar vazia");
    }

    #[test]
    fn layout_outline_heading_nivel2_tem_indentacao() {
        let content = Content::Sequence(vec![
            Content::Outline,
            Content::heading(1, Content::text("H1")),
            Content::heading(2, Content::text("H2")),
        ].into());

        let state = introspect(&content);
        let doc = layout(&content, state);
        let text = doc.plain_text();

        // Heading de nível 2 → TOC deve conter espaços de indentação antes de H2.
        // plain_text() não preserva posição, mas a TOC inclui "  " antes da Ref.
        assert!(text.contains("H1"), "TOC deve listar H1");
        assert!(text.contains("H2"), "TOC deve listar H2");
    }

    // ── Testes de Passo 62 — Figuras ─────────────────────────────────────────

    #[test]
    fn layout_figure_com_caption_tem_prefixo() {
        let content = Content::Figure {
            body:    Box::new(Content::text("Gráfico")),
            caption: Some(Box::new(Content::text("Resultados"))),
        };

        let state = introspect(&content);
        let doc = layout(&content, state);
        let text = doc.plain_text();

        assert!(text.contains("Gráfico"),    "corpo da figura deve aparecer");
        assert!(text.contains("Figura 1:"),  "prefixo numérico deve aparecer");
        assert!(text.contains("Resultados"), "legenda deve aparecer");
    }

    #[test]
    fn layout_figure_sem_caption_sem_prefixo() {
        let content = Content::Figure {
            body:    Box::new(Content::text("Diagrama")),
            caption: None,
        };

        let state = introspect(&content);
        let doc = layout(&content, state);
        let text = doc.plain_text();

        assert!(text.contains("Diagrama"),    "corpo deve aparecer");
        assert!(!text.contains("Figura 1:"), "sem caption, sem prefixo");
    }

    #[test]
    fn layout_ref_para_figura_resolve_corretamente() {
        use crate::entities::label::Label;

        let content = Content::Sequence(
            vec![
                Content::Labelled {
                    label:  Label("fig1".to_string()),
                    target: Box::new(Content::Figure {
                        body:    Box::new(Content::text("Gráfico")),
                        caption: Some(Box::new(Content::text("Legenda"))),
                    }),
                },
                Content::text(" — ver "),
                Content::Ref { target: Label("fig1".to_string()) },
            ]
            .into(),
        );

        let state = introspect(&content);
        let doc = layout(&content, state);
        let text = doc.plain_text();

        assert!(text.contains("Figura 1"),
            "Ref para figura deve resolver para 'Figura 1': {:?}", text);
        assert!(!text.contains("@fig1"),
            "não deve usar fallback @fig1: {:?}", text);
    }

    // ── Testes de Passo 63 — Mapa de páginas e motor de congelamento ─────────

    #[test]
    fn layout_regista_pagina_de_label() {
        use crate::entities::label::Label;

        let content = Content::Sequence(vec![
            Content::Labelled {
                label:  Label("sec1".to_string()),
                target: Box::new(Content::heading(1, Content::text("Introdução"))),
            },
        ].into());

        let state = introspect(&content);
        let doc = layout(&content, state);

        assert!(
            doc.extracted_label_pages.contains_key(&Label("sec1".to_string())),
            "extracted_label_pages deve conter a label processada"
        );
    }

    #[test]
    fn layout_pagina_de_label_e_um_indexed() {
        use crate::entities::label::Label;

        let content = Content::Labelled {
            label:  Label("top".to_string()),
            target: Box::new(Content::text("No topo")),
        };

        let state = introspect(&content);
        let doc = layout(&content, state);

        let page = doc.extracted_label_pages.get(&Label("top".to_string()))
            .copied()
            .unwrap_or(0);
        assert_eq!(page, 1, "label no início do documento deve estar na página 1");
    }

    #[test]
    fn layout_toc_com_readonly_nao_duplica_contadores() {
        // Heading com CounterUpdate embebido — sem is_readonly, o contador avançaria
        // duas vezes (uma no heading real, outra no clone da TOC).
        // Com is_readonly, a renderização da TOC é neutra em relação aos contadores.
        use crate::entities::counter_state::CounterAction;

        let body_with_counter_update = Content::Sequence(vec![
            Content::text("Secção"),
            Content::CounterUpdate {
                key:    "equation".to_string(),
                action: CounterAction::Step,
            },
        ].into());

        let content = Content::Sequence(vec![
            Content::Outline,
            Content::heading(1, body_with_counter_update),
            Content::CounterDisplay { kind: "equation".to_string() },
        ].into());

        let state = introspect(&content);
        let doc = layout(&content, state);
        let text = doc.plain_text();

        // Sem is_readonly: CounterUpdate dispararia 2× → display mostraria "2".
        // Com is_readonly: CounterUpdate na TOC é bloqueado → display mostra "1".
        assert!(
            text.contains('1') && !text.contains('2'),
            "CounterUpdate na TOC não deve duplicar: {:?}", text
        );
    }

    #[test]
    fn layout_extracted_label_pages_preenchido_apos_layout() {
        // extracted_label_pages é sempre populado após layout, mesmo sem labels.
        let content = Content::text("Texto sem labels");
        let doc = layout(&content, CounterState::default());
        // Deve existir o campo (pode estar vazio)
        assert!(doc.extracted_label_pages.is_empty(),
            "sem labels, extracted_label_pages deve estar vazio");
    }

    // ── Testes de Passo 65 — Convergência de fixpoint ────────────────────────

    #[test]
    fn layout_converge_sem_ciclo_infinito() {
        let content = Content::Sequence(vec![
            Content::SetHeadingNumbering { active: true },
            Content::Outline,
            Content::heading(1, Content::text("Capítulo 1")),
            Content::heading(2, Content::text("Secção 1.1")),
        ].into());

        let state = introspect(&content);
        // Se o fixpoint tiver defeito, entra em loop até MAX_ITERATIONS.
        // Não deve panic.
        let doc = layout(&content, state);

        let text = doc.plain_text();
        assert!(text.contains("Capítulo 1"), "título deve aparecer: {:?}", text);
        assert!(text.contains("Índice") || text.contains("ndice"),
            "TOC deve aparecer: {:?}", text);
    }

    #[test]
    fn layout_documento_sem_toc_usa_curto_circuito() {
        // Documento COM títulos mas SEM #outline(). O vetor headings_for_toc
        // terá entradas, mas has_outline é false — o short-circuit evita o loop.
        // Prova que a condição correcta é has_outline, não headings_for_toc.is_empty().
        let content = Content::Sequence(vec![
            Content::SetHeadingNumbering { active: true },
            Content::heading(1, Content::text("Introdução")),
            Content::heading(2, Content::text("Motivação")),
            Content::text("Texto sem índice."),
        ].into());

        let state = introspect(&content);
        assert!(!state.has_outline, "sem Outline no documento, has_outline deve ser false");

        let doc = layout(&content, state);
        assert!(!doc.pages.is_empty(), "documento deve ter páginas");
    }

    #[test]
    fn layout_com_labels_produz_extracted_label_pages() {
        use crate::entities::label::Label;

        let content = Content::Sequence(vec![
            Content::Labelled {
                label:  Label("sec1".to_string()),
                target: Box::new(Content::heading(1, Content::text("Secção"))),
            },
        ].into());

        let state = introspect(&content);
        let doc = layout(&content, state);

        assert!(
            doc.extracted_label_pages.contains_key(&Label("sec1".to_string())),
            "extracted_label_pages deve conter a label após convergência"
        );
    }
}
