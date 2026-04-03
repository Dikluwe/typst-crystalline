//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/layout.md
//! @prompt-hash 019e6489
//! @layer L1
//! @updated 2026-04-03

use ecow::EcoString;

use crate::entities::{
    content::Content,
    layout_types::{Frame, FrameItem, PagedDocument, Point, Pt, Size, TextStyle},
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
                let heading_size = self.font_size_pt * heading_scale(*level);
                let prev = self.style;
                self.style = TextStyle { bold: true, italic: false, size: heading_size };
                if self.cursor_x > MARGIN { self.flush_line(); }
                self.layout_content(body);
                self.flush_line();
                self.style = prev;
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
                let math_layouter = math::layout::MathLayouter::new(&self.metrics);
                let math_items    = math_layouter.layout_equation(body, &self.style);

                if *block {
                    if self.cursor_x > MARGIN { self.flush_line(); }
                }

                // Integrar items matemáticos no frame actual.
                // pos.x e pos.y são relativos à origem da equação —
                // pos.y inclui deslocamento vertical (sup/sub, frac).
                let offset_x = self.cursor_x;
                let offset_y = self.cursor_y;
                for item in math_items {
                    match item {
                        FrameItem::Text { pos, text, style } => {
                            let abs_pos = Point {
                                x: offset_x + pos.x,
                                y: offset_y + pos.y,
                            };
                            let advance = self.metrics.advance(&text, style.size);
                            self.current_line.push(FrameItem::Text { pos: abs_pos, text, style });
                            self.cursor_x = self.cursor_x + advance;
                        }
                        FrameItem::Line { start, end, thickness } => {
                            let abs_start = Point { x: offset_x + start.x, y: offset_y + start.y };
                            let abs_end   = Point { x: offset_x + end.x,   y: offset_y + end.y };
                            self.current_line.push(FrameItem::Line {
                                start: abs_start, end: abs_end, thickness,
                            });
                        }
                    }
                }

                if *block { self.flush_line(); }
            }

            Content::MathSequence(_)
            | Content::MathIdent(_)
            | Content::MathText(_)
            | Content::MathFrac { .. }
            | Content::MathAttach { .. }
            | Content::MathRoot { .. } => {
                // Nós matemáticos internos — normalmente não aparecem directamente
                // no layout fora de Content::Equation. Se aparecerem, renderizar como texto.
                let text = content.plain_text();
                for word in text.split_whitespace() {
                    self.layout_word(word);
                }
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

    pub fn finish(mut self) -> PagedDocument {
        for item in self.current_line.drain(..) {
            self.current.push(item);
        }
        if !self.current.items.is_empty() {
            self.pages.push(self.current);
        }
        PagedDocument::new(self.pages)
    }
}

// ── API pública ────────────────────────────────────────────────────────────

fn heading_scale(level: u8) -> f64 {
    match level { 1 => 2.0, 2 => 1.667, 3 => 1.333, 4 => 1.167, _ => 1.0 }
}

/// Layout com métricas fixas monoespaçadas.
///
/// Geometricamente correcto (margens respeitadas, word-wrap, baseline correcta).
/// Para métricas de fonte reais: `03_infra::layout::layout_with_font()`.
pub fn layout(content: &Content) -> PagedDocument {
    let mut l = Layouter::new(FixedMetrics, DEFAULT_FONT_SIZE);
    l.layout_content(content);
    l.finish()
}

// ── Testes ─────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::{content::Content, layout_types::FrameItem};

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
        let doc = layout(&Content::text("Hello world"));
        assert!(!doc.pages.is_empty());
        let total = doc.pages.iter().flat_map(|p| p.items.iter()).count();
        assert!(total >= 2, "Hello e world devem ser itens separados");
        assert!(doc.plain_text().contains("Hello"));
        assert!(doc.plain_text().contains("world"));
    }

    #[test]
    fn layout_documento_vazio_zero_paginas() {
        let doc = layout(&Content::Empty);
        assert_eq!(doc.pages.len(), 0, "documento vazio → sem páginas");
    }

    /// Teste de Ouro: todos os items dentro dos limites da página.
    #[test]
    fn layout_items_dentro_limites_da_pagina() {
        let words = (0..100)
            .map(|i| format!("palavra{i}"))
            .collect::<Vec<_>>()
            .join(" ");
        let doc = layout(&Content::text(&words));

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
        let doc = layout(&Content::text(&words));
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
        ));
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
        ));
        let italic = doc.pages.iter()
            .flat_map(|p| p.items.iter())
            .any(|i| matches!(i, FrameItem::Text { style, .. } if style.italic));
        assert!(italic, "Emph deve produzir FrameItem com italic=true");
    }

    #[test]
    fn heading_h1_tamanho_maior() {
        let doc = layout(&Content::sequence(vec![
            Content::heading(1, Content::text("Title")),
            Content::text("body"),
        ]));
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
        ]));
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
        let doc = layout(content);
        assert!(!doc.pages.is_empty());
        assert!(
            doc.plain_text().contains("Olá") || doc.plain_text().contains("mundo"),
            "texto deve estar no output: {:?}", doc.plain_text()
        );
    }

    // ── Passo 23 ────────────────────────────────────────────────────────────

    #[test]
    fn layout_list_item_tem_bullet() {
        let doc = layout(&Content::list_item(Content::text("Item")));
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
        let doc = layout(&content);
        let sizes: std::collections::HashSet<u64> = doc.pages.iter()
            .flat_map(|p| p.items.iter())
            .filter_map(|i| match i {
                FrameItem::Text { style, .. } => Some(style.size.val().to_bits()),
                _ => None,
            })
            .collect();
        assert!(sizes.len() > 1, "Raw deve ter tamanho diferente do texto normal");
    }
}
