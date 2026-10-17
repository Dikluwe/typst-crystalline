//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/measurement.md
//! @prompt-hash 2c082f8e
//! @layer L1
//! @updated 2026-10-07

use crate::entities::{
    content::Content,
    geometry::ShapeKind,
    image_sizer::{ImageSizer, NullImageSizer},
    layout_types::{FrameItem, Pt, TextStyle},
    style_chain::StyleChain,
};

use super::metrics::FontMetrics;
use super::{
    block, curve, resolve_pt, sub_frame, Layouter, COLUMNS_DEFAULT_GUTTER_RATIO,
};

/// Positive certificate for sequences whose children share an inline flow.
fn inline_flow(content: &Content) -> bool {
    if matches!(
        content,
        Content::Empty | Content::Text(_) | Content::Space | Content::SpaceFromNewline
    ) {
        return true;
    }
    if let Content::Sequence(children) = content {
        return children.iter().all(inline_flow);
    }
    if let Content::Styled(e) = content {
        return inline_flow(&e.body);
    }
    if let Content::Strong(e) = content {
        return inline_flow(&e.body);
    }
    if let Content::Emph(e) = content {
        return inline_flow(&e.body);
    }
    if let Content::Source(e) = content {
        return inline_flow(&e.body);
    }
    if let Content::Realized(e) = content {
        return inline_flow(&e.visual);
    }
    if let Content::Located(e) = content {
        return inline_flow(&e.body);
    }
    false
}

impl<'a, M: FontMetrics, S: ImageSizer> Layouter<'a, M, S> {
    // ── Auxiliares de Grid (Passo 80) ─────────────────────────────────────

    /// P273.11 — Mede um Stack (children + dir + spacing) com `max_w`.
    /// Helper extraído da replicação inline P273.9 §2.2 (cleanup §9 P273.9).
    /// Decisão 1β Fase A: método em Layouter (reutiliza
    /// `measure_content_constrained` via `&self`).
    pub(super) fn measure_stack(
        &mut self,
        children: &[Content],
        dir: crate::entities::dir::Dir,
        spacing: Option<crate::entities::layout_types::Length>,
        max_w: f64,
    ) -> (f64, f64) {
        let n = children.len();
        if n == 0 {
            return (0.0, 0.0);
        }
        let space_pt = spacing.map_or(0.0, |l| l.resolve_pt(self.style.size.val()));
        if dir.is_vertical() {
            let mut max_child_w = 0.0_f64;
            let mut sum_h = 0.0_f64;
            for child in children.iter() {
                let (w, h) = self.measure_content_constrained(child, max_w);
                max_child_w = max_child_w.max(w);
                sum_h += h;
            }
            (max_child_w, sum_h + ((n - 1) as f64) * space_pt)
        } else {
            let mut sum_w = 0.0_f64;
            let mut max_child_h = 0.0_f64;
            for child in children.iter() {
                let (w, h) = self.measure_content_constrained(child, max_w);
                sum_w += w;
                max_child_h = max_child_h.max(h);
            }
            (sum_w + ((n - 1) as f64) * space_pt, max_child_h)
        }
    }

    /// Measure an inline flow in a separate domain instance, never the emitter.
    fn measure_inline_flow(&self, content: &Content, width: f64) -> (f64, f64) {
        use comemo::Track;
        let intr = crate::entities::introspector::TagIntrospector::empty();
        let metrics: &dyn FontMetrics = &self.metrics;
        let mut isolated = Layouter::new(
            metrics,
            NullImageSizer,
            self.style.size.0,
            (&intr as &dyn crate::entities::introspector::Introspector).track(),
        );
        isolated.chain = self.chain.clone();
        isolated.style = self.style.clone();
        isolated.explicit_line_alignment =
            Some(crate::entities::layout_types::HAlign::Left);
        let (height, items, _, _, _) = isolated.layout_sub_frame(
            content,
            sub_frame::SubLayoutRegion {
                origin_x: 0.0,
                width,
                height: None,
                align_rtl: false,
                unconstrained_height: true,
            },
        );
        let refs: Vec<_> = items.iter().collect();
        let visual_right = self.metrics.line_content_right(&refs);
        let literal_right = items
            .iter()
            .filter_map(|item| match item {
                FrameItem::TextSeparator {
                    pos,
                    advance,
                    origin: crate::entities::layout_types::TextSeparatorOrigin::Literal,
                    ..
                } => Some(pos.x.0 + advance.0),
                _ => None,
            })
            .fold(0.0_f64, f64::max);
        (visual_right.max(literal_right).min(width), height)
    }

    /// Mede conteúdo com restrição de largura máxima.
    ///
    /// Usado pelo algoritmo de grid para determinar a largura das colunas Auto.
    /// Retorna `(width, height)` em pontos.
    pub(super) fn measure_content_constrained(
        &mut self,
        content: &Content,
        max_width: f64,
    ) -> (f64, f64) {
        match content {
            Content::Realized(e) => {
                self.measure_content_constrained(&e.visual, max_width)
            }
            Content::Located(e) => self.measure_content_constrained(&e.body, max_width),
            Content::Source(e) => self.measure_content_constrained(&e.body, max_width),
            Content::ShowRecipe(e) => {
                self.measure_content_constrained(&e.body, max_width)
            }
            Content::Text(text) => {
                let mut max_line_w = 0.0_f64;
                let mut current_w = 0.0_f64;
                let mut line_count = 1usize;
                // **P624** — usa `text_width` (P593) para herdar shaping em
                // scripts contextuais, em vez de `advance` directo.
                let space_w =
                    self.metrics.text_width(" ", self.style.size, &self.style).0;

                for word in text.split_whitespace() {
                    let word_w =
                        self.metrics.text_width(word, self.style.size, &self.style).0;
                    let next_w = if current_w > 0.0 {
                        current_w + space_w + word_w
                    } else {
                        word_w
                    };
                    if next_w > max_width && current_w > 0.0 {
                        max_line_w = max_line_w.max(current_w);
                        line_count += 1;
                        current_w = word_w;
                    } else {
                        current_w = next_w;
                    }
                }
                max_line_w = max_line_w.max(current_w);
                let (_, line_height) =
                    self.metrics.vertical_metrics(self.style.size, &self.style);
                (max_line_w.min(max_width), line_height.0 * line_count as f64)
            }

            Content::Sequence(children) => {
                if max_width.is_finite() && max_width > 0.0 && inline_flow(content) {
                    return self.measure_inline_flow(content, max_width);
                }
                let mut total_h = 0.0_f64;
                let mut max_w = 0.0_f64;
                for child in children.iter() {
                    let (w, h) = self.measure_content_constrained(child, max_width);
                    total_h += h;
                    max_w = max_w.max(w);
                }
                (max_w, total_h)
            }

            Content::Shape(e) => {
                let (kind, width, height) = (&e.kind, &e.width, &e.height);
                match kind {
                    // P242 — RoundedRect partilha dimensões com Rect.
                    ShapeKind::Rect
                    | ShapeKind::RoundedRect { .. }
                    | ShapeKind::Ellipse
                    | ShapeKind::Path(_) => {
                        let w = resolve_pt(width.as_deref(), max_width).min(max_width);
                        let h = resolve_pt(height.as_deref(), 0.0);
                        (w, h)
                    }
                    ShapeKind::Line { dx, dy } => (dx.abs().min(max_width), dy.abs()),
                }
            }

            // Passo 513 — medição de `Content::Curve` via bbox do path.
            Content::Curve(e) => {
                let items =
                    curve::path_items_from_curve(&e.segments, self.style.size.val());
                let (min_x, min_y, max_x, max_y) =
                    crate::entities::geometry::path_bbox(&items);
                let w = (max_x - min_x).max(0.0).min(max_width);
                let h = (max_y - min_y).max(0.0);
                (w, h)
            }

            // Passo 156C / 156L: Pad / Hide para grid measurement.
            // P156L: cada side é Option<Length>; None ↔ zero.
            Content::Pad(e) => {
                let sides = &e.sides;
                let font = self.style.size.val();
                let left = sides.left.map_or(0.0, |l| l.resolve_pt(font));
                let right = sides.right.map_or(0.0, |l| l.resolve_pt(font));
                let top = sides.top.map_or(0.0, |l| l.resolve_pt(font));
                let bottom = sides.bottom.map_or(0.0, |l| l.resolve_pt(font));
                let constrained = (max_width - left - right).max(0.0);
                let (w, h) = self.measure_content_constrained(&e.body, constrained);
                (w + left + right, h + top + bottom)
            }
            Content::Hide(e) => self.measure_content_constrained(&e.body, max_width),

            // P408: smallcaps — stub transparente em medição (paridade layout).
            Content::SmallCaps { body } => {
                self.measure_content_constrained(body, max_width)
            }

            // Passo 156D: HSpace/VSpace dimensões para grid measurement.
            // **P842 (#38)** — `Fractional` mede 0 em contexto de medição
            // (o fr só expande contra o espaço restante de uma linha real;
            // numa medição isolada não há restante definido).
            Content::HSpace(e) => (
                match e.amount {
                    crate::entities::elements::h_space::Spacing::Absolute(l) => {
                        l.resolve_pt(self.style.size.val())
                    }
                    crate::entities::elements::h_space::Spacing::Fractional(_) => 0.0,
                },
                0.0,
            ),
            Content::VSpace(e) => (0.0, e.amount.resolve_pt(self.style.size.val())),

            // Passo 156E/220: Pagebreak/Colbreak — events sem dimensões em cell.
            Content::Pagebreak(_) => (0.0, 0.0),
            Content::Colbreak(_) => (0.0, 0.0),

            // Passo 156I: Stack dimensões para grid measurement.
            // TTB/BTT: max widths; sum heights + (n-1) * spacing.
            // LTR/RTL: sum widths + (n-1) * spacing; max heights.
            // P273.11 — delega ao helper Layouter::measure_stack (substitui
            // replicação inline ~25 LOC; bit-exact preserved).
            Content::Stack(e) => {
                self.measure_stack(&e.children, e.dir, e.spacing, max_width)
            }

            // Passo 156H: Boxed (Box inline) dimensões para grid
            // measurement. Análogo a Block (mesma lógica width/height/
            // inset; baseline ignorado em medição).
            Content::Boxed(e) => {
                let (body, width, height, inset) =
                    (&e.body, &e.width, &e.height, &e.inset);
                let font = self.style.size.val();
                let inset_l = inset.left.resolve_pt(font);
                let inset_r = inset.right.resolve_pt(font);
                let inset_t = inset.top.resolve_pt(font);
                let inset_b = inset.bottom.resolve_pt(font);
                let body_max = if e.width_fraction.is_none() {
                    (width.map_or(max_width, |w| w.resolve_pt(font)) - inset_l - inset_r)
                        .max(0.0)
                } else {
                    (max_width - inset_l - inset_r).max(0.0)
                };
                let (bw, bh) = self.measure_content_constrained(body, body_max);
                let natural_w = bw + inset_l + inset_r;
                let natural_h = bh + inset_t + inset_b;
                let (total_w, total_h) = if e.width_fraction.is_none() {
                    (
                        width.map_or(natural_w, |w| w.resolve_pt(font).max(0.0)),
                        height.map_or(natural_h, |h| h.resolve_pt(font).max(0.0)),
                    )
                } else {
                    (
                        natural_w,
                        height.map_or(natural_h, |h| h.resolve_pt(font).max(natural_h)),
                    )
                };
                (total_w, total_h)
            }

            // Passo 156J: Repeat dimensões para grid measurement.
            // Single-render do body (consistente with layout_content
            // arm). Algoritmo dinâmico de quantidade defere per
            // ADR-0054 graded.
            Content::Repeat(e) => self.measure_content_constrained(&e.body, max_width),

            // P219 (DEBT-56 sub-fase b 3/4): Columns dimensões para grid
            // measurement. Consumer real graded — calcula column_width
            // (paralelo a layout_content arm); medir body com width
            // reduzida; retorna full_width (columns ocupa width inteira)
            // + body_h (single-render graded).
            Content::Columns(e) => {
                let count_f = if e.count == 0 { 1.0 } else { e.count as f64 };
                let gutter_pt = match e.gutter {
                    Some(g) => g.resolve_pt(self.style.size.0),
                    None => max_width * COLUMNS_DEFAULT_GUTTER_RATIO,
                };
                let column_width = if count_f >= 1.0 {
                    (max_width - (count_f - 1.0) * gutter_pt) / count_f
                } else {
                    max_width
                };
                let (_body_w, body_h) =
                    self.measure_content_constrained(&e.body, column_width);
                (max_width, body_h)
            }

            // Passo 156G: Block dimensões para grid measurement.
            // Inset adiciona aos lados; height: Some(h) força mínimo;
            // width: Some(w) prefere essa largura mas constrained por max.
            Content::Block(e) => {
                let (body, width, height, inset) =
                    (&e.body, &e.width, &e.height, &e.inset);
                let font = self.style.size.val();
                let inset_l = inset.left.resolve_pt(font);
                let inset_r = inset.right.resolve_pt(font);
                let inset_t = inset.top.resolve_pt(font);
                let inset_b = inset.bottom.resolve_pt(font);
                let body_max = match width {
                    Some(w) => block::resolve_width_pt(*w, font, max_width)
                        .unwrap_or(max_width)
                        .min(max_width - inset_l - inset_r),
                    None => (max_width - inset_l - inset_r).max(0.0),
                };
                let (bw, bh) = self.measure_content_constrained(body, body_max);
                // P772f — largura explícita do bloco é a largura reportada
                // (o corpo mede-se dentro dela, mas não a substitui). Antes
                // usava sempre `bw`, colapsando a 0 quando o corpo era um
                // `Content::Align`/`Content::Place` sem braço próprio (ex:
                // `block(width: 3cm, align(top, place(..)))` numa coluna
                // auto de grid) — a coluna auto-dimensionava para 0pt.
                let total_w = match width {
                    Some(w) => block::resolve_width_pt(*w, font, max_width)
                        .unwrap_or(bw + inset_l + inset_r)
                        .min(max_width),
                    None => bw + inset_l + inset_r,
                };
                let body_h_with_inset = bh + inset_t + inset_b;
                let total_h = match height {
                    Some(crate::entities::elements::block::BlockHeight::Relative(h)) => {
                        block::resolve_height_pt(*h, font, self.available_height())
                            .unwrap_or(body_h_with_inset)
                            .max(body_h_with_inset)
                    }
                    Some(crate::entities::elements::block::BlockHeight::Fractional(
                        _,
                    )) => self
                        .fractional_block_height
                        .unwrap_or(body_h_with_inset)
                        .max(body_h_with_inset),
                    None => body_h_with_inset,
                };
                (total_w, total_h)
            }

            // P772f — `align` não tem tamanho intrínseco próprio: reporta o
            // tamanho do corpo (o wrapper só reposiciona dentro do espaço
            // disponível, não o redimensiona). Braço em falta colapsava a
            // 0 e fazia colunas auto de grid com células `align(...)`
            // colidirem na mesma posição x (ambas medidas a largura 0).
            Content::Align(e) => self.measure_content_constrained(&e.body, max_width),

            Content::Styled(styled) => {
                let body = &styled.body;
                let styles = &styled.styles;
                let prev_chain = self.chain.clone();
                let prev_style = self.style.clone();
                self.chain = self.chain.push_styles(styles);
                self.style = TextStyle::from(&self.chain);
                let res = self.measure_content_constrained(body, max_width);
                self.chain = prev_chain;
                self.style = prev_style;
                res
            }

            Content::Strong(e) => {
                use crate::entities::style::{Style, Styles};
                let prev_chain = self.chain.clone();
                let prev_style = self.style.clone();
                self.chain =
                    self.chain.push_styles(&Styles::from_iter([Style::strong()]));
                self.style = TextStyle::from(&self.chain);
                let res = self.measure_content_constrained(&e.body, max_width);
                self.chain = prev_chain;
                self.style = prev_style;
                res
            }

            Content::Emph(e) => {
                use crate::entities::style::{Style, Styles};
                let prev_chain = self.chain.clone();
                let prev_style = self.style.clone();
                self.chain = self.chain.push_styles(&Styles::from_iter([Style::emph()]));
                self.style = TextStyle::from(&self.chain);
                let res = self.measure_content_constrained(&e.body, max_width);
                self.chain = prev_chain;
                self.style = prev_style;
                res
            }

            _ => (0.0, 0.0), // neutro: N16[γ] — Content sem dimensão fixa explícita retorna (0.0, 0.0) (fallback de dimensão estática)
        }
    }
}

/// **P712/P858** — layout real e isolado de conteúdo para `measure()` (stdlib).
///
/// Constrói um `Layouter` isolado com `chain` = `chain` do chamador (o
/// tamanho medido depende do `#set text(size:)` activo, paridade com o
/// vanilla `measure()`, que usa `context.styles()`) e corre
/// `layout_sub_frame` (Passo 629 — o mesmo mecanismo já reutilizado por
/// `Content::Place`/`Content::Transform` e por `grid.rs` para medir
/// células) numa região efectivamente sem limites (`width:
/// f64::INFINITY`, `height: None`) — paridade com o vanilla
/// `Region::new(.., Abs::inf())` para a `measure()` sem `width`/`height`
/// explícitos. Devolve `(width, height)` em pontos, a partir dos itens
/// realmente emitidos pelo layout — não uma aproximação manual por tipo
/// de `Content` (ao contrário do antigo `measure_content` em
/// `layout/helpers.rs`, que continua a servir os seus próprios
/// consumers internos — `Content::Transform`/`Content::Place` —,
/// inalterado por este passo).
///
/// **P858** — recebe `metrics` do chamador (via `Engine::font_metrics`).
/// Durante a expansão de `#context`, a métrica real (`FallbackFontMetrics`,
/// L3) é injetada; noutros contextos, usa-se `FixedMetrics` como fallback
/// heurístico. A paridade com o vanilla é agora ao nível da língua
/// (medidas reais de fonte), não apenas proporcional.
pub fn measure_content_real(
    content: &Content,
    chain: &StyleChain,
    metrics: &dyn FontMetrics,
) -> (f64, f64) {
    let (width, height, _) = measure_content_real_with_baseline(content, chain, metrics);
    (width, height)
}

/// **P1491** — variante interna de `measure_content_real` que preserva a
/// baseline causal do frame medido. O helper público histórico mantém a
/// assinatura `(width, height)`; só o dispatch de `measure()` consome a
/// terceira coordenada.
pub(in crate::compiler) fn measure_content_real_with_baseline(
    content: &Content,
    chain: &StyleChain,
    metrics: &dyn FontMetrics,
) -> (f64, f64, f64) {
    use comemo::Track;

    let font_size = chain.size();
    let intr = crate::entities::introspector::TagIntrospector::empty();
    let intr_dyn: &dyn crate::entities::introspector::Introspector = &intr;
    let mut layouter =
        Layouter::new(metrics, NullImageSizer, font_size, intr_dyn.track());
    layouter.chain = chain.clone();
    layouter.style = TextStyle::from(chain);

    // P772x — `layouter` é uma instância isolada e efémera (só para medir);
    // não há collector ambiente possível aqui, `_deco` é sempre vazio.
    // **P908** — instância efémera, descartada inteira após a medição; não
    // há "pai" para propagar entradas órfãs de `pending_align_*` (`_x`/`_y`
    // abaixo) — mesmo descarte já correcto de antes.
    let (height, items, _deco, _orphaned_x, _orphaned_y) = layouter.layout_sub_frame(
        content,
        sub_frame::SubLayoutRegion {
            origin_x: 0.0,
            width: f64::INFINITY,
            height: None,
            align_rtl: false,
            unconstrained_height: true,
        },
    );

    let width = if items.is_empty() {
        0.0
    } else {
        let refs: Vec<&FrameItem> = items.iter().collect();
        metrics.line_content_right(&refs)
    };

    // **P860** — a altura deve ser a do frame de layout do texto, não o
    // avanço de linha de `layout_sub_frame`. Os edges padrão do vanilla
    // (`top-edge: cap-height`, `bottom-edge: baseline`) definem a altura de
    // um frame de texto; `text_edges` devolve `(top, bottom)` relativos à
    // baseline, pelo que a altura total é `top - bottom`.
    // Para conteúdo sem texto plano, mantém-se o fallback do avanço de linha.
    let text = content.plain_text();
    let (text_ascent, text_descent) = if text.is_empty() {
        (0.0, 0.0)
    } else {
        let (top, bottom) = metrics.text_edges(Pt(font_size), &layouter.style);
        (top.val(), -bottom.val())
    };

    let inline_ascent = layouter.last_sub_frame_inline_extents.0.max(0.0);
    let inline_descent = layouter.last_sub_frame_inline_extents.1.max(0.0);
    let has_inline_frame = inline_ascent > 0.0 || inline_descent > 0.0;

    let (height, baseline) = if has_inline_frame {
        let ascent = text_ascent.max(inline_ascent);
        let descent = text_descent.max(inline_descent);
        (ascent + descent, ascent)
    } else if !text.is_empty() {
        (text_ascent + text_descent, text_ascent)
    } else if height > 0.0 {
        (height, height)
    } else {
        (0.0, 0.0)
    };

    (width, height, baseline)
}

#[cfg(test)]
mod smoke {
    #[test]
    fn realization_transports_preserve_constrained_measurement() {
        use super::*;
        use crate::compiler::layout::FixedMetrics;
        use crate::entities::content::{LocatedContent, RealizedContent};
        use crate::entities::locator::Locator;
        use comemo::Track;
        use std::sync::Arc;
        let metrics = FixedMetrics;
        let intr = crate::entities::introspector::TagIntrospector::empty();
        let intr_dyn: &dyn crate::entities::introspector::Introspector = &intr;
        let mut layouter = Layouter::new(metrics, NullImageSizer, 10.0, intr_dyn.track());
        let body = Content::text("AB");
        let expected = layouter.measure_content_constrained(&body, 1000.0);
        let realized = Content::Realized(Arc::new(RealizedContent {
            original: Content::text("A much longer consumed original"),
            visual: body.clone(),
        }));
        let located = Content::Located(Arc::new(LocatedContent {
            location: Locator::new().next(),
            body,
        }));
        assert_eq!(layouter.measure_content_constrained(&realized, 1000.0), expected);
        assert_eq!(layouter.measure_content_constrained(&located, 1000.0), expected);
    }

    #[test]
    fn module_compila_e_carrega() {
        // Cobertura funcional: `compiler::layout::tests`, inclusive os casos
        // de medida constrangida, baseline e estilos aninhados.
    }
}

#[cfg(test)]
mod inline_flow_regression {
    use super::*;
    use comemo::Track;
    #[test]
    fn inline_sequence_is_measured_as_one_flow() {
        use crate::entities::{dir::Dir, style::Styles, value::Value};
        let intr = crate::entities::introspector::TagIntrospector::empty();
        let mut l = Layouter::new(
            super::super::FixedMetrics,
            NullImageSizer,
            11.0,
            (&intr as &dyn crate::entities::introspector::Introspector).track(),
        );
        let body = Content::sequence(vec![
            Content::styled(
                Content::text("ABC"),
                Styles::new().push_custom("text.dir", Value::Dir(Dir::LTR)),
            ),
            Content::Space,
            Content::text("DEF"),
        ]);
        let expected = l.metrics.text_width("ABC DEF", l.style.size, &l.style).0;
        let before = l.regions.current.cursor_x;
        let (width, _) = l.measure_content_constrained(&body, 1000.0);
        assert!((width - expected).abs() < 1e-6, "width={width} expected={expected}");
        assert_eq!(l.regions.current.cursor_x, before);
    }
}
