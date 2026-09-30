//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/style_scope.md
//! @prompt-hash dd66dc06
//! @layer L1
//! @updated 2026-09-29

use crate::entities::{
    content::Content,
    image_sizer::ImageSizer,
    layout_types::{FrameItem, SemanticKind, SemanticPlacement, TextStyle},
    style::Styles,
    value::Value,
};

use super::metrics::FontMetrics;
use super::{html_frame, outline, sub_frame, Layouter};

pub(super) fn layout_styled<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    body: &Content,
    styles: &Styles,
) {
    if super::paragraph::is_paragraph(body) {
        let had_line =
            html_frame::items_have_material(&layouter.regions.current.current_line);
        super::parbreak::layout(layouter);
        if had_line {
            layouter.terminal_paragraph_line = true;
        }
    }
    let prev_chain = layouter.chain.clone(); // O(1) Arc::clone
    let tracking_scope = (styles.delta()
        != &crate::entities::style_chain::StyleDelta::empty())
        .then(|| layouter.text_tracking_state.enter());
    let prev_style = layouter.style.clone();
    layouter.chain = layouter.chain.push_styles(styles);
    layouter.style = TextStyle::from(&layouter.chain);
    if layouter.document_language.is_none() {
        layouter.document_language = layouter.style.lang;
    }
    let pdf_label = styles.delta().custom.iter().rev().any(|(key, value)| {
        key.as_str() == "pdf.marker.tag"
            && matches!(value, Value::Str(tag) if tag.as_str() == "label")
    });
    let outline_destination =
        styles.delta().custom.iter().rev().find_map(|(key, value)| {
            match (key.as_str(), value) {
                ("outline.link.destination", Value::Str(label)) => {
                    Some(crate::entities::label::Label(label.to_string()))
                }
                _ => None,
            }
        });
    if let Some(label) = outline_destination {
        outline::layout_destination_link(layouter, body, label);
    } else if pdf_label {
        let (_, items) = layouter.layout_sub_frame_inline(
            body,
            sub_frame::SubLayoutRegion {
                origin_x: 0.0,
                width: 0.0,
                height: None,
                align_rtl: false,
                unconstrained_height: true,
            },
        );
        layouter.regions.current.current_line.push(FrameItem::Semantic {
            kind: SemanticKind::Label,
            placement: SemanticPlacement::Inline,
            alt: None,
            items,
        });
    } else {
        layouter.layout_content(body);
    }
    let owns_par_line_style = styles
        .delta()
        .custom
        .iter()
        .any(|(key, _)| key.as_str().starts_with("par.line."));
    if owns_par_line_style
        && html_frame::items_have_material(&layouter.regions.current.current_line)
    {
        layouter.flush_line();
    }
    layouter.chain = prev_chain;
    layouter.style = prev_style;
    if let Some(scope) = tracking_scope {
        layouter.text_tracking_state.leave(scope);
    }
}

pub(super) fn layout_strong<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    body: &Content,
) {
    use crate::entities::style::{Style, Styles};
    let prev_chain = layouter.chain.clone();
    let tracking_scope = layouter.text_tracking_state.enter();

    let prev_style = layouter.style.clone();
    layouter.chain = layouter.chain.push_styles(&Styles::from_iter([Style::strong()]));
    layouter.style = TextStyle::from(&layouter.chain);
    layouter.layout_content(body);
    layouter.chain = prev_chain;
    layouter.style = prev_style;
    layouter.text_tracking_state.leave(tracking_scope);
}

pub(super) fn layout_emph<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    body: &Content,
) {
    use crate::entities::style::{Style, Styles};
    let prev_chain = layouter.chain.clone();
    let tracking_scope = layouter.text_tracking_state.enter();

    let prev_style = layouter.style.clone();
    layouter.chain = layouter.chain.push_styles(&Styles::from_iter([Style::emph()]));
    layouter.style = TextStyle::from(&layouter.chain);
    layouter.layout_content(body);
    layouter.chain = prev_chain;
    layouter.style = prev_style;
    layouter.text_tracking_state.leave(tracking_scope);
}

pub(super) fn layout_smallcaps<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    body: &Content,
) {
    let prev = layouter.smallcaps;
    layouter.smallcaps = true;
    layouter.layout_content(body);
    layouter.smallcaps = prev;
}

#[cfg(test)]
mod smoke {
    #[test]
    fn module_compila_e_carrega() {
        // Cobertura funcional: tests_styled_integration e show Strong/Emph na
        // suíte central de layout.
    }
}
