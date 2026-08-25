//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout.md
//! @prompt-hash a4b2dd97
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
    if !layouter.current_page_is_empty() {
        layouter.flush_line();
        layouter.new_page();
    }

    let previous = layouter.page_config.clone();
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

    layouter.install_page_config(local);
    layouter.layout_content(&e.body);

    if !layouter.regions.current.current_line.is_empty() {
        layouter.flush_line();
    }
    // Boundary final: comita inclusive body vazio, mas a página vazia criada
    // para o irmão seguinte não deve ser materializada por `finish`.
    layouter.new_page();
    layouter.page_run_boundary_empty = true;
    layouter.install_page_config(previous);
}
