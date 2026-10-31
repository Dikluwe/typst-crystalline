//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/set_page.md
//! @prompt-hash 03574bea
//! @layer L1
//! @updated 2026-10-25
//!
//! Layout de `Content::SetPage` — aplica nova configuração de página.
//! Extraído de `layout/mod.rs` no P425 (ADR-0109 forma B).
//! **P867** — suporte a `width: auto` / `height: auto` (PageDimension).

use crate::entities::{content::SetPageDelta, image_sizer::ImageSizer, layout_types::Pt};

use super::metrics::FontMetrics;
use super::Layouter;

/// Aplica uma nova configuração de página (largura, altura, margem, colunas)
/// e força nova página se a configuração mudou.
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<'_, M, S>,
    delta: &SetPageDelta,
) {
    let mut new_config = layouter.page_config.clone();
    let mut changed = false;
    if delta.paper.is_some()
        || delta.flipped.is_some()
        || delta.binding.is_some()
        || delta.width.is_some()
        || delta.height.is_some()
        || delta.margin.is_some()
    {
        let dir = layouter
            .chain
            .custom("text.dir")
            .and_then(|v| {
                if let crate::entities::value::Value::Dir(d) = v {
                    Some(*d)
                } else {
                    None
                }
            })
            .unwrap_or(crate::entities::dir::Dir::LTR);
        super::page_geometry::apply(
            &mut new_config,
            dir,
            delta.paper,
            delta.flipped,
            delta.binding,
            delta.width,
            delta.height,
            delta.margin,
        );
        changed = true;
    }
    // Nota P598: `margin` ausente não altera `margin_is_auto`. O default
    // já é automático; uma vez fixada pelo utilizador, só um `margin`
    // explícito (incluindo `auto`, se o eval o distinguir no futuro) a
    // mudaria. Nesta fase, o eval representa tanto ausente como `auto` por
    // `None`, pelo que mantemos o estado anterior.
    if let Some(value) = &delta.numbering {
        new_config.numbering = value.clone();
        changed = true;
    }
    if let Some(value) = &delta.number_align {
        new_config.number_align = *value;
        changed = true;
    }
    if let Some(value) = &delta.header {
        new_config.header = value.clone();
        changed = true;
    }
    if let Some(value) = &delta.header_ascent {
        new_config.header_ascent = *value;
        changed = true;
    }
    if let Some(value) = &delta.footer {
        new_config.footer = value.clone();
        changed = true;
    }
    if let Some(value) = &delta.footer_descent {
        new_config.footer_descent = *value;
        changed = true;
    }
    if let Some(value) = &delta.supplement {
        new_config.supplement = value.clone();
        changed = true;
    }
    if delta.columns != new_config.columns {
        new_config.columns = delta.columns;
        changed = true;
    }
    if let Some(spec) = &delta.bleed {
        new_config.bleed_spec = (*spec).fold(new_config.bleed_spec);
        changed = true;
    }
    if let Some(value) = &delta.fill {
        new_config.fill = value.clone();
        changed = true;
    }
    if let Some(layer) = &delta.background {
        new_config.background = layer.as_ref().map(|content| (**content).clone());
        changed = true;
    }
    if let Some(layer) = &delta.foreground {
        new_config.foreground = layer.as_ref().map(|content| (**content).clone());
        changed = true;
    }

    // P598 — se a margem é automática, recalculá-la sempre que as
    // dimensões da página mudarem (ou quando se volta para auto).
    let auto = new_config.auto_margin();
    new_config.margin.refresh_auto(auto);

    if changed {
        if !layouter.current_page_is_empty() {
            layouter.flush_line();
            layouter.new_page();
            if !layouter.is_sub_frame {
                layouter.footnote_entry_scope = None;
            }
        }
        use super::PageRunningSlot;
        use crate::compiler::introspect::page_running::PlanEdge;
        if let Some(value) = &delta.header {
            layouter.bind_running_slot(
                PageRunningSlot::Header,
                PlanEdge::SetPageHeader,
                matches!(value, crate::entities::page_running::PageMarginal::Content(_)),
            );
        }
        if let Some(value) = &delta.footer {
            layouter.bind_running_slot(
                PageRunningSlot::Footer,
                PlanEdge::SetPageFooter,
                matches!(value, crate::entities::page_running::PageMarginal::Content(_)),
            );
        }
        if let Some(value) = &delta.background {
            layouter.bind_running_slot(
                PageRunningSlot::Background,
                PlanEdge::SetPageBackground,
                value.is_some(),
            );
        }
        if let Some(value) = &delta.foreground {
            layouter.bind_running_slot(
                PageRunningSlot::Foreground,
                PlanEdge::SetPageForeground,
                value.is_some(),
            );
        }
        layouter.page_config = new_config;
        // P216A: sincronizar region.width/height com PageConfig
        // (Caminho B1 — redundância controlada).
        layouter.regions.current.width = layouter.page_config.width;
        layouter.regions.current.height = layouter.page_config.height;
        layouter.regions.current.cursor_x = Pt(layouter.page_config.margin.left);
        layouter.regions.current.line_start_x = Pt(layouter.page_config.margin.left);
        // **P761/P762** — quando a baseline inicial ainda está pendente,
        // `ensure_initial_baseline()` adicionará o offset do `top-edge`
        // (com a fonte activa nesse momento). Não adiantar esse offset aqui,
        // porque `self.style` ainda pode ser a fonte default e o offset
        // seria somado duas vezes.
        layouter.regions.current.cursor_y = if layouter.initial_baseline_pending {
            Pt(layouter.page_config.margin.top)
        } else {
            let (top, _) =
                layouter.metrics.text_edges(layouter.style.size, &layouter.style);
            Pt(layouter.page_config.margin.top) + top
        };
        // DEBT-35b: se available_width() vier a ter cache, invalidar aqui.
    }
}

#[cfg(test)]
mod r12_scope_tests {
    use super::*;
    use crate::entities::{
        content::Content,
        image_sizer::NullImageSizer,
        introspector::{Introspector, TagIntrospector},
        layout_types::{FrameItem, Length, PageDimension, PageMarginSpec, PageMargins},
        style::Styles,
        value::Value,
    };
    use comemo::Track;

    fn configured(indent: f64, body: Content) -> Content {
        Content::styled(
            body,
            Styles::new()
                .push_custom("footnote.entry.indent", Value::Length(Length::em(indent))),
        )
    }

    fn positions(initial: Option<f64>, next: f64) -> (f64, f64) {
        fn words(items: &[FrameItem], out: &mut Vec<(String, f64)>) {
            for item in items {
                match item {
                    FrameItem::Text { text, pos, .. }
                    | FrameItem::TextShaped { text, pos, .. } => {
                        out.push((text.to_string(), pos.x.0))
                    }
                    FrameItem::Group { items, .. }
                    | FrameItem::Link { items, .. }
                    | FrameItem::Semantic { items, .. }
                    | FrameItem::TextReplacement { items, .. } => words(items, out),
                    FrameItem::TextSeparator { .. }
                    | FrameItem::Line { .. }
                    | FrameItem::Glyph { .. }
                    | FrameItem::Image { .. }
                    | FrameItem::Shape { .. } => {}
                }
            }
        }
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            super::super::FixedMetrics,
            NullImageSizer,
            11.0,
            (&intr as &dyn Introspector).track(),
        );
        l.page_config.width = 300.0;
        l.page_config.height = 180.0;
        l.page_config.margin = PageMargins::uniform(20.0);
        l.page_config.margin_spec = PageMarginSpec::uniform(20.0);
        l.regions.current.width = 300.0;
        l.regions.current.height = 180.0;
        let body = Content::sequence(vec![
            Content::text("Alpha"),
            Content::footnote(Content::text("FIRST")),
            Content::SetPage(std::sync::Arc::new(SetPageDelta {
                height: Some(PageDimension::Length(200.0)),
                ..SetPageDelta::default()
            })),
            configured(
                next,
                Content::sequence(vec![
                    Content::text("Beta"),
                    Content::footnote(Content::text("SECOND")),
                ]),
            ),
        ]);
        let body = initial
            .map_or_else(|| body.clone(), |indent| configured(indent, body.clone()));
        l.layout_content(&body);
        let doc = l.finish();
        assert_eq!(
            doc.pages.len(),
            2,
            "the existing material SetPage transition must remain"
        );
        let mut rows = Vec::new();
        for page in &doc.pages {
            words(&page.items, &mut rows);
        }
        let find = |name: &str| {
            let found = rows.iter().filter(|r| r.0 == name).collect::<Vec<_>>();
            assert_eq!(found.len(), 1, "{name}: {rows:?}");
            found[0].1
        };
        (find("FIRST"), find("SECOND"))
    }

    #[test]
    fn r12_set_page_starts_a_new_shared_entry_scope() {
        let (first, second) = positions(None, 2.0);
        assert!((second - first - 9.35).abs() < 1e-8, "FIRST={first}, SECOND={second}");
    }

    #[test]
    fn r12_set_page_can_replace_an_inherited_entry_scope() {
        let (first, second) = positions(Some(2.0), 0.0);
        assert!((first - second - 18.7).abs() < 1e-8, "FIRST={first}, SECOND={second}");
    }
}
