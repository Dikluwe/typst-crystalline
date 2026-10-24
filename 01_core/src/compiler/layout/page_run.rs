//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/page_run.md
//! @prompt-hash 12838397
//! @layer L1
//! @updated 2026-08-24
//!
//! Layout lexical de `PageRunElem`: aplica configuração local e restaura.

use crate::entities::elements::page_run::PageRunElem;
use crate::entities::image_sizer::ImageSizer;

use super::{FontMetrics, Layouter};

pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<'_, M, S>,
    e: &PageRunElem,
) {
    if layouter.is_sub_frame {
        layouter.layout_errors.push(
            crate::entities::source_result::SourceDiagnostic::error(
                crate::entities::span::Span::detached(),
                "page configuration is not allowed inside of containers".to_string(),
            ),
        );
        return;
    }

    // A fronteira inicial é weak: só fecha a página anterior quando ela já
    // contém material. Uma linha pendente também conta como conteúdo.
    if !super::pagebreak::page_is_empty(layouter) {
        layouter.flush_line();
        layouter.new_page();
    }

    // An explicit run requires its page, including an explicitly empty body.
    layouter.weak_page_successor = false;
    let previous = layouter.page_config.clone();
    let previous_running = layouter
        .page_running_pass
        .as_ref()
        .map(|pass| (pass.slots.clone(), pass.environment.clone(), pass.chain.clone()));
    let mut local = previous.clone();
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
        &mut local, dir, e.paper, e.flipped, e.binding, e.width, e.height, e.margin,
    );
    if let Some(numbering) = &e.numbering {
        local.numbering = numbering.clone();
    }
    if let Some(value) = e.number_align {
        local.number_align = value;
    }
    if let Some(value) = &e.header {
        local.header = value.clone();
    }
    if let Some(value) = e.header_ascent {
        local.header_ascent = value;
    }
    if let Some(value) = &e.footer {
        local.footer = value.clone();
    }
    if let Some(value) = e.footer_descent {
        local.footer_descent = value;
    }
    if let Some(value) = &e.supplement {
        local.supplement = value.clone();
    }
    if let Some(columns) = e.columns {
        local.columns = Some(columns);
    }
    if let Some(bleed) = e.bleed {
        local.bleed_spec = bleed.fold(local.bleed_spec);
    }
    if let Some(fill) = &e.fill {
        local.fill = fill.clone();
    }
    if let Some(background) = &e.background {
        local.background = background.clone();
    }
    if let Some(foreground) = &e.foreground {
        local.foreground = foreground.clone();
    }

    use super::PageRunningSlot;
    use crate::compiler::introspect::page_running::PlanEdge;
    if let Some(value) = &e.header {
        layouter.bind_running_slot(
            PageRunningSlot::Header,
            PlanEdge::PageRunHeader,
            matches!(value, crate::entities::page_running::PageMarginal::Content(_)),
        );
    }
    if let Some(value) = &e.footer {
        layouter.bind_running_slot(
            PageRunningSlot::Footer,
            PlanEdge::PageRunFooter,
            matches!(value, crate::entities::page_running::PageMarginal::Content(_)),
        );
    }
    if let Some(value) = &e.background {
        layouter.bind_running_slot(
            PageRunningSlot::Background,
            PlanEdge::PageRunBackground,
            value.is_some(),
        );
    }
    if let Some(value) = &e.foreground {
        layouter.bind_running_slot(
            PageRunningSlot::Foreground,
            PlanEdge::PageRunForeground,
            value.is_some(),
        );
    }
    layouter.install_page_config(local);
    layouter.layout_planned_child(PlanEdge::PageRunBody, &e.body);

    if super::html_frame::items_have_material(&layouter.regions.current.current_line) {
        layouter.flush_line();
    }
    // Boundary final: comita inclusive body vazio, mas a página vazia criada
    // para o irmão seguinte não deve ser materializada por `finish`.
    if !layouter.weak_page_successor || !super::pagebreak::page_is_empty(layouter) {
        layouter.new_page();
    }
    layouter.weak_page_successor = false;
    layouter.page_run_boundary_empty = true;
    if let (Some(pass), Some(previous_slots)) =
        (&mut layouter.page_running_pass, previous_running)
    {
        pass.slots = previous_slots.0;
        pass.environment = previous_slots.1;
        pass.chain = previous_slots.2;
    }
    layouter.install_page_config(previous);
}
