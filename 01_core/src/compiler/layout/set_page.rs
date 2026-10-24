//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/set_page.md
//! @prompt-hash 8b7a4115
//! @layer L1
//! @updated 2026-07-14
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
