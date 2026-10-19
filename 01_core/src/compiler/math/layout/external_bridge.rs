//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/math/layout/external_bridge.md
//! @prompt-hash 6e39a9f2
//! @layer L1
//! @updated 2026-10-01

use super::*;

impl<'a, M: FontMetrics> MathLayouter<'a, M> {
    /// **P994** — layout de conteúdo não-matemático embutido em math
    /// (equivalente ao `ExternalItem`/`layout_external` do vanilla,
    /// `ir/resolve.rs:228-230` → `typst-layout/src/math/mod.rs:585-603`).
    ///
    /// Constrói um `Layouter` temporário com as métricas REAIS da equação
    /// (`self.metrics` coagido a `&dyn FontMetrics` — `impl FontMetrics
    /// for &dyn FontMetrics`, `compiler/layout/metrics.rs:369`), cadeia de
    /// estilos reconstruída de `style` (é o que faz `#text(size: 20pt)`
    /// aplicar-se ao conteúdo embutido) e introspector vazio (mesma
    /// limitação registada de `measure()`: labels/links dentro de
    /// conteúdo externo em math não resolvem). Corre
    /// `layout_sub_frame` numa região ilimitada (precedente
    /// `measure_content_real`, `compiler/layout/mod.rs:2123-2158`), ancora o
    /// resultado na baseline da equação (passo 6 abaixo — paridade
    /// vanilla: a fórmula `height/2 + axis` só se aplica a frames SEM
    /// baseline declarada) e devolve os items ACHATADOS (sem wrapper
    /// `Group` — passo 7 abaixo; a convenção Y do export PDF para
    /// `Group` com filhos de texto é incoerente, medição Fase B.2).
    ///
    /// Guardas (aceites na adenda pós-Fase B do L0):
    /// - `items` vazio E `plain_text` vazio → caixa vazia (preserva o
    ///   comportamento do catch-all antigo para conteúdo vazio);
    /// - `items` vazio MAS `plain_text` não vazio → fallback ao antigo
    ///   `layout_text_node` (não perder texto silenciosamente se o
    ///   `Layouter` ignorar uma variante com texto).
    pub(super) fn layout_external(
        &self,
        content: &Content,
        style: &TextStyle,
    ) -> MathBox {
        use comemo::Track;

        use crate::compiler::layout::{Layouter, SubLayoutRegion};
        use crate::entities::image_sizer::NullImageSizer;
        use crate::entities::introspector::{Introspector, TagIntrospector};
        use crate::entities::style::{Style, Styles};
        use crate::entities::style_chain::StyleChain;

        // 1. Layouter temporário com as métricas reais (coerção &M → &dyn).
        let metrics_dyn: &dyn FontMetrics = self.metrics;
        let intr = TagIntrospector::empty();
        let intr_dyn: &dyn Introspector = &intr;
        let mut layouter = Layouter::new(
            metrics_dyn,
            NullImageSizer,
            style.size.val(),
            intr_dyn.track(),
        );

        // 2. Cadeia reconstruída dos campos de `TextStyle` com variante
        //    `Style` correspondente (os dois lados existem); `font`/`weight`
        //    propagam o show-set da equação (P944) — paridade vanilla, cujo
        //    `EquationElem::show_set` cobre o conteúdo embutido.
        let mut styles = vec![
            Style::Size(crate::entities::layout_types::Length::pt(style.size.val())),
            Style::bold(style.bold),
            Style::italic(style.italic),
        ];
        if let Some(c) = style.fill.clone() {
            styles.push(Style::Fill(c));
        }
        if let Some(h) = style.heading_level {
            styles.push(Style::HeadingLevel(h));
        }
        if let Some(w) = style.weight {
            styles.push(Style::Weight(w));
        }
        if let Some(t) = style.tracking {
            styles.push(Style::Tracking(t));
        }
        if let Some(l) = style.leading {
            styles.push(Style::Leading(l));
        }
        if let Some(l) = style.lang {
            styles.push(Style::Lang(l));
        }
        if let Some(f) = &style.font {
            styles.push(Style::Font(f.clone()));
        }
        layouter.chain =
            StyleChain::default_chain().push_styles(&Styles::from_iter(styles));
        layouter.style = style.clone();

        // 3. Sub-frame sem limites (largura infinita — decisão registada no
        //    L0: o vanilla usa `ctx.region`; em página auto o efeito
        //    coincide). `deco`/órfãos descartados com a instância (P908).
        let (height, items, _deco, _orphaned_x, _orphaned_y) = layouter.layout_sub_frame(
            content,
            SubLayoutRegion {
                origin_x: 0.0,
                width: f64::INFINITY,
                height: None,
                align_rtl: false,
                unconstrained_height: true,
            },
        );

        // 4. Guardas (ver doc acima).
        if items.is_empty() {
            let text: EcoString = content.plain_text().into();
            if text.trim().is_empty() {
                return MathBox {
                    width: 0.0,
                    ascent: 0.0,
                    descent: 0.0,
                    items: vec![],
                };
            }
            return self.layout_text_node(&text, style);
        }

        // 5. Largura = largura real do sub-frame (cursor advance) ou limite direito dos itens
        let items_width = items
            .iter()
            .map(|item| match item {
                FrameItem::Shape { pos, width, .. } => pos.x.val().max(0.0) + *width,
                other => {
                    let (x, _) = crate::compiler::layout::helpers::item_pos(other);
                    x + crate::compiler::layout::helpers::item_width(other, self.metrics)
                }
            })
            .fold(0.0, f64::max);
        let width = layouter.last_sub_frame_width.max(items_width);

        // 6. Âncora vertical (paridade vanilla `layout_external`,
        //    `typst-layout/src/math/mod.rs:585-603`): o vanilla só aplica
        //    `height/2 + axis` QUANDO o frame não declara baseline
        //    (`if !frame.has_baseline()`). Medição (Fase B.2): os items do
        //    sub-frame são baseline-ancorados (o `pos.y` de um `Text` é a
        //    baseline da linha) e o `height` devolvido é medido DESDE a
        //    primeira baseline (`sub_frame.rs`, `cell_height = end_y −
        //    start_y`) — a fórmula incondicional fazia o conteúdo flutuar
        //    ~10pt acima da baseline (`e1-text-size-italic.typ`). Com
        //    texto, a baseline interna (primeiro `Text`/`Glyph`, por ordem
        //    do documento) assenta na baseline da equação; sem texto,
        //    aplica-se a fórmula do vanilla sobre os extents de tinta.
        let mut first_baseline = None;
        let mut ink_top = None;
        let mut ink_bottom = None;
        self.scan_external_verticals(
            &items,
            0.0,
            &mut first_baseline,
            &mut ink_top,
            &mut ink_bottom,
        );
        let axis_pt = self.constants.to_pt(self.constants.axis_height, style.size).val();
        let (anchor, ascent, descent) = match (first_baseline, ink_top, ink_bottom) {
            (Some(b), Some(top), Some(bot)) => {
                (b, (b - top).max(0.0), (bot - b).max(0.0))
            }
            (Some(b), Some(top), None) => (b, (b - top).max(0.0), (height - b).max(0.0)),
            (Some(b), None, Some(bot)) => (b, b, (bot - b).max(0.0)),
            (Some(b), None, None) => (b, b, (height - b).max(0.0)),
            // Sem texto: frame sem baseline declarada → fórmula do vanilla
            // sobre os extents reais (`H/2 + axis` medido do topo da tinta).
            (None, Some(top), Some(bot)) => {
                // rationale: P1064 Classe 1C — ponto médio de tinta ao eixo (top + (bot - top) / 2.0 + axis_pt)
                let a = top + (bot - top) / 2.0 + axis_pt;
                (a, a - top, (bot - a).max(0.0))
            }
            // Sem extents mensuráveis: fórmula original.
            _ => {
                // rationale: P1064 Classe 1C — meia-altura de frame ao eixo (height / 2.0 + axis_pt)
                let a = height / 2.0 + axis_pt;
                (a, a, (height / 2.0 - axis_pt).max(0.0))
            }
        };

        // 7. **Achatar** — os items entram DIRECTAMENTE na `MathBox`
        //    (transladados por `−anchor`), SEM wrapper `FrameItem::Group`.
        //    Medição Fase B.2: o caminho PDF de `Group` com filhos de texto
        //    tem a convenção de eixo Y incoerente (pré-existente — o texto
        //    de `box(height:, clip: true)` também desaparece do render; o
        //    `cm` de `stream.rs` não inverte Y para matrix identidade,
        //    enquanto o renderer raster assume filhos Y-down) — um texto
        //    dentro de um Group sai espelhado em Y (erro medido = 2×y_local,
        //    exactamente). Achatar faz o conteúdo embutido fluir pelos
        //    caminhos de emissão de topo (Text/Shape/Line/Glyph), provados
        //    por toda a renderização math. Grupos ANINHADOS dentro do
        //    conteúdo (ex.: clip de `block(clip:)`) passam intactos pelo
        //    `offset_item` — mesmo estado que têm fora de math.
        let items = items
            .into_iter()
            .map(|item| offset_item(item, Pt(0.0), Pt(-anchor)))
            .collect();

        MathBox { width, ascent, descent, items }
    }

    /// **P994** — varre os items de um sub-frame externo (recursivo em
    /// `Group`/`Link`, acumulando o offset Y) e reporta: a baseline do
    /// primeiro item de texto/glifo (por ordem do documento) e os extents
    /// de tinta vertical (topo/fundo), em coordenadas locais do sub-frame.
    /// Usado por `layout_external` para a âncora vertical (ver passo 6 lá).
    fn scan_external_verticals(
        &self,
        items: &[FrameItem],
        offset_y: f64,
        first_baseline: &mut Option<f64>,
        ink_top: &mut Option<f64>,
        ink_bottom: &mut Option<f64>,
    ) {
        fn grow(top: &mut Option<f64>, bottom: &mut Option<f64>, t: f64, b: f64) {
            *top = Some(top.map_or(t, |v: f64| v.min(t)));
            *bottom = Some(bottom.map_or(b, |v: f64| v.max(b)));
        }
        for item in items {
            match item {
                FrameItem::TextSeparator { .. } => {}
                FrameItem::Text { pos, text, style, .. }
                | FrameItem::TextShaped { pos, text, style, .. } => {
                    let y = offset_y + pos.y.val();
                    if first_baseline.is_none() {
                        *first_baseline = Some(y);
                    }
                    let (up, down) =
                        self.metrics.text_ink_bounds(text, style.size, style);
                    grow(ink_top, ink_bottom, y - up.val(), y + down.val());
                }
                FrameItem::Glyph { pos, size, .. } => {
                    let y = offset_y + pos.y.val();
                    if first_baseline.is_none() {
                        *first_baseline = Some(y);
                    }
                    let (top_edge, bottom_edge) =
                        self.metrics.text_edges(*size, &TextStyle::default());
                    grow(ink_top, ink_bottom, y - top_edge.0, y + bottom_edge.0.abs());
                }
                FrameItem::Shape { pos, height, .. } => {
                    // P1100: Shapes decorativos (como borda de box()) não devem
                    // inflar os limites de tinta de conteúdo que já possui texto/glifos com baseline.
                    if first_baseline.is_none() {
                        let t = offset_y + pos.y.val();
                        grow(ink_top, ink_bottom, t, t + height);
                    }
                }
                FrameItem::Line { start, end, thickness, .. } => {
                    // rationale: P1064 Classe 1B — semi-espessura de linha (thickness / 2.0)
                    let half = thickness / 2.0;
                    let y0 = offset_y + start.y.val();
                    let y1 = offset_y + end.y.val();
                    grow(ink_top, ink_bottom, y0.min(y1) - half, y0.max(y1) + half);
                }
                FrameItem::Image { pos, height, .. } => {
                    let t = offset_y + pos.y.val();
                    grow(ink_top, ink_bottom, t, t + height.val());
                }
                FrameItem::Group { pos, items, .. }
                | FrameItem::Link { pos, items, .. } => {
                    self.scan_external_verticals(
                        items,
                        offset_y + pos.y.val(),
                        first_baseline,
                        ink_top,
                        ink_bottom,
                    );
                }
                FrameItem::TextReplacement { items, .. }
                | FrameItem::Semantic { items, .. } => {
                    self.scan_external_verticals(
                        items,
                        offset_y,
                        first_baseline,
                        ink_top,
                        ink_bottom,
                    );
                }
            }
        }
    }
}

#[cfg(test)]
mod smoke {
    #[test]
    fn modulo_compila_e_carrega() {}
}
