//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/style_scope.md
//! @prompt-hash fc17bcb1
//! @layer L1
//! @updated 2026-10-07

use crate::entities::{
    content::Content,
    image_sizer::ImageSizer,
    layout_types::{FrameItem, Length, SemanticKind, SemanticPlacement, TextStyle},
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
    let hanging_boundary = styles.delta().custom.iter().any(|(key, _)| {
        key.as_str() == "par.hanging-indent"
            || key.as_str() == "eval.par.hanging-boundary"
    });
    if hanging_boundary {
        super::parbreak::layout(layouter);
    }
    if super::paragraph::is_paragraph(body) {
        let had_line =
            html_frame::items_have_material(&layouter.regions.current.current_line);
        super::parbreak::layout(layouter);
        if had_line {
            layouter.terminal_paragraph_line = true;
        }
    }
    let prev_planning = layouter.paragraph_planning_suppressed;
    if hanging_boundary {
        layouter.paragraph_planning_suppressed = false;
    }
    let prev_chain = layouter.chain.clone(); // O(1) Arc::clone
    let tracking_scope = (styles.delta()
        != &crate::entities::style_chain::StyleDelta::empty())
        .then(|| layouter.text_tracking_state.enter());
    let prev_style = layouter.style.clone();
    let mut delta = styles.delta().clone();
    let owns_raw_block = delta
        .custom
        .iter()
        .any(|(k, v)| k.as_str() == "raw.block" && v == &Value::Bool(true));
    let raw_block_size = delta.custom.iter().rev().find_map(|(k, v)| {
        if k.as_str() == "raw.block.size" {
            if let Value::Length(l) = v {
                Some(*l)
            } else {
                None
            }
        } else {
            None
        }
    });
    if owns_raw_block {
        delta.custom.push(("raw.block.active".into(), Value::Bool(true)));
    }

    if delta
        .custom
        .iter()
        .any(|(key, value)| key.as_str() == "raw.defaults" && value == &Value::Bool(true))
    {
        let base = super::raw::default_style(&prev_style, &prev_chain);
        delta.size = Some(Length::pt(base.size.val()));
        delta.font = base.font;
        delta.fill = base.fill;
        delta.bold = Some(base.bold);
        delta.italic = Some(base.italic);
        delta.custom.push(("raw.defaults.active".into(), Value::Bool(true)));
    } else if prev_chain.custom("raw.defaults.active") == Some(&Value::Bool(true)) {
        if let Some(Value::Length(length)) = delta
            .custom
            .iter()
            .rev()
            .find_map(|(key, value)| (key.as_str() == "text.size").then_some(value))
        {
            delta.size = Some(Length::pt(length.resolve_pt(prev_style.size.val())));
        }
    }
    let heading_show_set = delta.custom.iter().any(|(key, value)| {
        key.as_str() == "heading.show-set-size" && value == &Value::Bool(true)
    });
    delta
        .custom
        .retain(|(key, _)| key.as_str() != "heading.show-set-size");
    if heading_show_set {
        let mut target = body.source_body();
        while let Content::Located(located) = target {
            target = located.body.source_body();
        }
        if let Content::Heading(heading) = target {
            // Heading applies its default scale after this scope. Only its
            // show-set's absolute contribution must bypass that multiplication.
            let scale = super::helpers::heading_scale(heading.level);
            if let Some(size) = &mut delta.size {
                size.abs =
                    crate::entities::layout_types::Abs::pt(size.abs.to_pt() / scale);
            } else if let Some(Value::Length(size)) = delta
                .custom
                .iter_mut()
                .find_map(|(key, value)| (key.as_str() == "text.size").then_some(value))
            {
                size.abs =
                    crate::entities::layout_types::Abs::pt(size.abs.to_pt() / scale);
            }
        }
    }
    layouter.chain = layouter.chain.push(delta);
    layouter.style = TextStyle::from(&layouter.chain);
    let block_font = raw_block_size
        .map(|v| v.resolve_pt(layouter.style.size.val()))
        .unwrap_or(layouter.style.size.val());
    let raw_indent_state = owns_raw_block
        .then(|| super::raw::begin_block(layouter, block_font, &prev_style));
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
    if let Some(state) = raw_indent_state {
        super::raw::end_block(layouter, block_font);
        layouter.paragraph_indent = state;
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
    if hanging_boundary {
        super::parbreak::layout(layouter);
    }
    layouter.paragraph_planning_suppressed = prev_planning;
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
