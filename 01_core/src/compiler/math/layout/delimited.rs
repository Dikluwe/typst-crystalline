//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/math/layout/delimited.md
//! @prompt-hash 8d1f1474
//! @layer L1
//! @updated 2026-10-31
//!
//! Método `layout_delimited` de `MathLayouter`. Extraído de `math/layout/mod.rs`
//! no Passo 96.8 conforme ADR-0037.

use crate::compiler::layout::FontMetrics;
use crate::entities::{
    content::Content,
    elements::math_delimited::{MathLrElem, MathMidElem},
    layout_types::{Length, TextStyle},
    rel::Rel,
    value::Value,
};

use super::MathBox;

fn positional_lr_delimiters(body: &Content) -> Option<(char, Content, char)> {
    let Content::MathSequence(items) = body else { return None };
    if items.len() < 2 {
        return None;
    }

    let open = super::single_math_leaf_char(items.first()?)?;
    let close = super::single_math_leaf_char(items.last()?)?;
    let open_class = super::spacing::node_math_class(items.first()?).0;
    let close_class = super::spacing::node_math_class(items.last()?).1;
    use crate::entities::math_class::MathClass::{Closing, Fence, Opening};
    if !matches!(open_class, Opening | Closing | Fence)
        || !matches!(close_class, Opening | Closing | Fence)
    {
        return None;
    }

    let middle = match &items[1..items.len() - 1] {
        [] => Content::Empty,
        [only] => only.clone(),
        many => Content::MathSequence(std::sync::Arc::from(many.to_vec())),
    };
    Some((open, middle, close))
}

fn effective_lr_size(elem: &MathLrElem, cascaded: Option<Rel<Length>>) -> Rel<Length> {
    if elem.size_explicit {
        elem.size
    } else {
        cascaded.unwrap_or_else(|| Rel::from_percent(100.0))
    }
}

#[cfg(test)]
fn lr_target_pt(body_height: f64, font_size: f64, size: Rel<Length>) -> f64 {
    (lr_requested_target_pt(body_height, font_size, size) - 0.1 * font_size).max(0.0)
}

fn lr_requested_target_pt(body_height: f64, font_size: f64, size: Rel<Length>) -> f64 {
    size.resolve(Length::pt(body_height)).resolve_pt(font_size).max(0.0)
}

/// Extensão de um corpo delimitado relativa ao eixo matemático. O mesmo
/// cálculo deve alimentar cercas implícitas e `lr`: a altura bruta de uma
/// fração assimétrica pode caber numa variante menor que a sua extensão
/// equilibrada (vanilla `fenced.rs::relative_to_from_fragments`).
fn fenced_body_target_pt(body: &Content, body_box: &MathBox, axis_pt: f64) -> f64 {
    if matches!(body, Content::MathFrac(e) if !e.line) {
        body_box.height()
    } else {
        2.0 * (body_box.ascent - axis_pt).max(body_box.descent + axis_pt)
    }
}

#[cfg(test)]
pub(crate) fn p1437_mid_target_pt(body_height: f64, font_size: f64, scale: f64) -> f64 {
    lr_target_pt(body_height, font_size, Rel::from_percent(scale * 100.0))
}

#[cfg(test)]
pub(crate) fn p1437_lr_mid_height(outer_scale: f64, inner_scale: Option<f64>) -> f64 {
    use crate::compiler::layout::{FixedMetrics, FontMetrics};
    use crate::entities::glyph_variants::{GlyphVariant, GlyphVariants};
    use crate::entities::layout_types::Pt;

    struct MidMetrics;
    impl FontMetrics for MidMetrics {
        fn advance(&self, text: &str, size: Pt, style: &TextStyle) -> Pt {
            FixedMetrics.advance(text, size, style)
        }
        fn vertical_metrics(&self, size: Pt, style: &TextStyle) -> (Pt, Pt) {
            FixedMetrics.vertical_metrics(size, style)
        }
        fn cap_height(&self, size: Pt, style: &TextStyle) -> Pt {
            FixedMetrics.cap_height(size, style)
        }
        fn text_edges(&self, size: Pt, style: &TextStyle) -> (Pt, Pt) {
            FixedMetrics.text_edges(size, style)
        }
        fn vertical_glyph_variants(&self, c: char, _style: &TextStyle) -> GlyphVariants {
            if c != '|' {
                return GlyphVariants::default();
            }
            GlyphVariants {
                variants: [700.0, 1900.0, 2900.0]
                    .into_iter()
                    .enumerate()
                    .map(|(index, advance)| GlyphVariant {
                        glyph_id: 100 + index as u16,
                        advance,
                        hor_advance: 400.0,
                        top_accent_attach: None,
                    })
                    .collect(),
            }
        }
        fn glyph_to_char(&self, glyph_id: u16) -> Option<char> {
            (100..=102).contains(&glyph_id).then_some('|')
        }
    }

    let style = TextStyle::regular(Pt(10.0));
    let layouter = super::MathLayouter::new(&MidMetrics, true, &style);
    let mid = Content::math_mid(Content::MathText("|".into()));
    let body = match inner_scale {
        None => mid,
        Some(scale) => Content::math_lr(mid, Rel::from_percent(scale * 100.0), true),
    };
    let outer = MathLrElem {
        body,
        size: Rel::from_percent(outer_scale * 100.0),
        size_explicit: true,
    };
    layouter.layout_lr(&outer, &style).height()
}

#[cfg(test)]
pub(crate) fn p1436_effective_size(
    elem: &MathLrElem,
    cascaded: Option<Rel<Length>>,
) -> Rel<Length> {
    effective_lr_size(elem, cascaded)
}

#[cfg(test)]
pub(crate) fn p1436_target_pt(
    body_height: f64,
    font_size: f64,
    size: Rel<Length>,
) -> f64 {
    lr_target_pt(body_height, font_size, size)
}

impl<'a, M: FontMetrics> super::MathLayouter<'a, M> {
    /// Probe geometry without consuming the accepted callback transcript.
    fn measure_lr_body(&self, body: &Content, style: &TextStyle) -> MathBox {
        let pass = super::callbacks::MathCallbackPassState::new(None);
        let probe = super::MathLayouter {
            metrics: self.metrics,
            constants: self.constants.clone(),
            block: self.block,
            equation: self.equation,
            region_height: self.region_height,
            lexical_styles: self.lexical_styles,
            callback_pass: Some(&pass),
            cancel_occurrence: std::cell::Cell::new(0),
            mid_target_pt: std::cell::Cell::new(None),
        };
        probe.layout_node(body, style)
    }

    fn lr_size(&self, elem: &MathLrElem) -> Rel<Length> {
        let cascaded = self
            .lexical_styles
            .and_then(|styles| styles.custom("math.lr.size"))
            .map(|value| match value {
                Value::Relative(value) => *value,
                Value::Ratio(value) => Rel { rel: value.get(), abs: Length::ZERO },
                Value::Length(value) => Rel { rel: 0.0, abs: *value },
                other => {
                    panic!("invalid validated math.lr.size type: {}", other.type_name())
                }
            });
        effective_lr_size(elem, cascaded)
    }

    pub(super) fn lr_multiline_nodes(
        &self,
        nodes: &[Content],
        style: &TextStyle,
    ) -> Option<Vec<Content>> {
        if !self.block {
            return None;
        }
        let mut result: Option<Vec<Content>> = None;
        for (index, node) in nodes.iter().enumerate() {
            if let Some(view) = self.lr_multiline_view(node, style) {
                let out = result.get_or_insert_with(|| nodes[..index].to_vec());
                if let Content::MathSequence(parts) = view.source_body() {
                    out.extend(parts.iter().cloned());
                } else {
                    out.push(view);
                }
            } else if let Some(out) = &mut result {
                out.push(node.clone());
            }
        }
        result
    }

    pub(super) fn lr_multiline_view(
        &self,
        content: &Content,
        style: &TextStyle,
    ) -> Option<Content> {
        if !self.block {
            return None;
        }
        let view = match content.source_body() {
            Content::MathSequence(nodes) => self
                .lr_multiline_nodes(nodes, style)
                .map(|nodes| Content::MathSequence(nodes.into())),
            Content::MathLr(elem) => self.lr_multiline_body(elem, style),
            Content::Source(..)
            | Content::ShowRecipe(..)
            | Content::Realized(..)
            | Content::Located(..)
            | Content::Empty
            | Content::Text(..)
            | Content::Symbol(..)
            | Content::Space
            | Content::SpaceFromNewline
            | Content::Parbreak
            | Content::Sequence(..)
            | Content::HtmlElem(..)
            | Content::HtmlFrame(..)
            | Content::Par { .. }
            | Content::Heading(..)
            | Content::Title(..)
            | Content::Strong(..)
            | Content::Emph(..)
            | Content::Raw(..)
            | Content::ListItem(..)
            | Content::EnumItem(..)
            | Content::Link(..)
            | Content::Equation(..)
            | Content::MathIdent(..)
            | Content::MathText(..)
            | Content::MathFrac(..)
            | Content::MathAttach(..)
            | Content::MathPrimes(..)
            | Content::MathRoot(..)
            | Content::MathStretch(..)
            | Content::MathDelimited(..)
            | Content::MathMid(..)
            | Content::MathOverbrace(..)
            | Content::MathUnderbrace(..)
            | Content::MathUnderbracket(..)
            | Content::MathUnderparen(..)
            | Content::MathUndershell(..)
            | Content::MathOverbracket(..)
            | Content::MathOverparen(..)
            | Content::MathOvershell(..)
            | Content::MathAlignPoint(..)
            | Content::Linebreak(..)
            | Content::MathMatrix(..)
            | Content::MathCases(..)
            | Content::MathAccent(..)
            | Content::MathCancel(..)
            | Content::MathUnderline(..)
            | Content::MathOverline(..)
            | Content::MathVec(..)
            | Content::MathClassOverride(..)
            | Content::MathLimitsOverride(..)
            | Content::MathUnderover(..)
            | Content::MathOp(..)
            | Content::MathStyled(..)
            | Content::Label(..)
            | Content::Ref(..)
            | Content::CounterDisplay(..)
            | Content::CounterUpdate(..)
            | Content::Outline(..)
            | Content::OutlineEntry(..)
            | Content::Figure(..)
            | Content::FigureCaption(..)
            | Content::Image(..)
            | Content::Shape(..)
            | Content::Curve(..)
            | Content::Transform(..)
            | Content::Grid(..)
            | Content::GridHeader(..)
            | Content::GridFooter(..)
            | Content::GridCell(..)
            | Content::SetPage(..)
            | Content::PageRun(..)
            | Content::Align(..)
            | Content::Place(..)
            | Content::Flush(..)
            | Content::Styled(..)
            | Content::Divider(..)
            | Content::Terms(..)
            | Content::TermItem(..)
            | Content::Quote(..)
            | Content::Document { .. }
            | Content::Asset { .. }
            | Content::SmartQuote(..)
            | Content::PdfAttach(..)
            | Content::PdfArtifact(..)
            | Content::Underline(..)
            | Content::Strike(..)
            | Content::Overline(..)
            | Content::SmallCaps { .. }
            | Content::Pad(..)
            | Content::Hide(..)
            | Content::HSpace(..)
            | Content::VSpace(..)
            | Content::Pagebreak(..)
            | Content::Colbreak(..)
            | Content::Stack(..)
            | Content::Boxed(..)
            | Content::Block(..)
            | Content::TableCell(..)
            | Content::Bibliography(..)
            | Content::Cite(..)
            | Content::Footnote(..)
            | Content::FootnoteEntry(..)
            | Content::TableHeader(..)
            | Content::TableFooter(..)
            | Content::GridHLine(..)
            | Content::GridVLine(..)
            | Content::TableHLine(..)
            | Content::TableVLine(..)
            | Content::Table(..)
            | Content::Repeat(..)
            | Content::Columns(..)
            | Content::Metadata(..)
            | Content::State(..)
            | Content::StateUpdate(..)
            | Content::StateDisplay(..)
            | Content::CounterDisplayCallback(..)
            | Content::ContextBlock(..)
            | Content::Dynamic(..) => None,
        }?;
        let span = content.source_span();
        if span.is_detached() {
            return Some(view);
        }
        let view = if let Content::MathSequence(nodes) = view {
            Content::MathSequence(
                nodes
                    .iter()
                    .map(|node| {
                        if node.source_span().is_detached() {
                            node.clone().with_source_span(span)
                        } else {
                            node.clone()
                        }
                    })
                    .collect::<Vec<_>>()
                    .into(),
            )
        } else {
            view
        };
        Some(view.with_source_span(span))
    }

    fn lr_multiline_body(&self, elem: &MathLrElem, style: &TextStyle) -> Option<Content> {
        if !self.block {
            return None;
        }
        let view = self.lr_multiline_view(&elem.body, style);
        let body = view.as_ref().unwrap_or(&elem.body);
        let Content::MathSequence(nodes) = body.source_body() else {
            return None;
        };
        let mut flat = Vec::new();
        super::flatten_math_sequence_nodes(nodes, &mut flat);
        if !flat.iter().any(|node| {
            matches!(
                node.source_body(),
                Content::Linebreak(_) | Content::MathAlignPoint(_)
            )
        }) {
            return None;
        }
        let fence = |node: &Content, first: bool| {
            use crate::entities::math_class::MathClass::{Closing, Fence, Opening};
            let character = super::single_math_leaf_char(node)?;
            let classes = super::spacing::node_math_class(node);
            matches!(if first { classes.0 } else { classes.1 }, Opening | Closing | Fence)
                .then_some(character)
        };
        let open = flat.first().and_then(|node| fence(node, true));
        let close = flat.last().and_then(|node| fence(node, false));
        if close.is_some() {
            flat.pop();
        }
        if open.is_some() && !flat.is_empty() {
            flat.remove(0);
        }
        let mut cells: Vec<(Content, Option<Content>)> = Vec::new();
        let mut current = Vec::new();
        let sequence = |nodes: Vec<Content>| match nodes.len() {
            0 => Content::Empty,
            1 => nodes.into_iter().next().unwrap(),
            _ => Content::MathSequence(nodes.into()),
        };
        for node in flat {
            if matches!(
                node.source_body(),
                Content::Linebreak(_) | Content::MathAlignPoint(_)
            ) {
                cells.push((sequence(std::mem::take(&mut current)), Some(node)));
            } else {
                current.push(node);
            }
        }
        cells.push((sequence(current), None));
        let size = self.lr_size(elem);
        let reference = if size.rel == 0.0 {
            0.0
        } else {
            let axis = self.constants.to_pt(self.constants.axis_height, style.size).val();
            cells
                .iter()
                .map(|(body, _)| {
                    if matches!(body.source_body(), Content::Empty) {
                        0.0
                    } else {
                        fenced_body_target_pt(
                            body,
                            &self.measure_lr_body(body, style),
                            axis,
                        )
                    }
                })
                .fold(0.0_f64, f64::max)
        };
        let target = lr_requested_target_pt(reference, style.size.val(), size);
        let last = cells.len() - 1;
        let mut output = Vec::new();
        for (index, (body, marker)) in cells.into_iter().enumerate() {
            let left = if index == 0 { open.unwrap_or('\0') } else { '\0' };
            let right = if index == last { close.unwrap_or('\0') } else { '\0' };
            if left != '\0'
                || right != '\0'
                || !matches!(body.source_body(), Content::Empty)
            {
                output.push(Content::math_lr(
                    Content::math_delimited(left, body, right),
                    Rel { rel: 0.0, abs: Length::pt(target) },
                    true,
                ));
            }
            if let Some(marker) = marker {
                output.push(marker);
            }
        }
        Some(Content::MathSequence(output.into()))
    }

    pub(super) fn layout_mid(&self, elem: &MathMidElem, style: &TextStyle) -> MathBox {
        let Some(target_pt) = self.mid_target_pt.get() else {
            return self.layout_node(&elem.body, style);
        };
        let text = match &elem.body {
            Content::Text(text)
            | Content::MathText(text)
            | Content::Symbol(text)
            | Content::MathIdent(text) => Some(text.as_str()),
            _ => None,
        };
        let Some(character) = text.and_then(|text| {
            let mut chars = text.chars();
            let first = chars.next()?;
            chars.next().is_none().then_some(first)
        }) else {
            return self.layout_node(&elem.body, style);
        };
        let target_du = if style.size.val() > 0.0 {
            target_pt * self.constants.upem / style.size.val()
        } else {
            0.0
        };
        self.layout_stretchy_delimiter(character, target_du, style)
    }

    pub(super) fn layout_lr(&self, elem: &MathLrElem, style: &TextStyle) -> MathBox {
        if let Some(view) = self.lr_multiline_body(elem, style) {
            return self.layout_node(&view, style);
        }
        let size = self.lr_size(elem);
        match &elem.body {
            Content::MathDelimited(delimited) => self.layout_delimited_sized(
                delimited.open,
                &delimited.body,
                delimited.close,
                size,
                style,
            ),
            body => {
                if let Some((open, middle, close)) = positional_lr_delimiters(body) {
                    return self
                        .layout_delimited_sized(open, &middle, close, size, style);
                }
                let inherited = self.mid_target_pt.replace(None);
                let natural = self.measure_lr_body(body, style);
                let target =
                    lr_requested_target_pt(natural.height(), style.size.val(), size);
                self.mid_target_pt.set(Some(target));
                let with_mid = self.layout_node(body, style);
                self.mid_target_pt.set(inherited);
                let text = match body {
                    Content::Text(text)
                    | Content::MathText(text)
                    | Content::Symbol(text)
                    | Content::MathIdent(text) => Some(text.as_str()),
                    _ => None,
                };
                let Some(character) = text.and_then(|text| {
                    let mut chars = text.chars();
                    let first = chars.next()?;
                    chars.next().is_none().then_some(first)
                }) else {
                    return with_mid;
                };
                if size == Rel::from_percent(100.0) {
                    return with_mid;
                }
                let target = size
                    .resolve(Length::pt(natural.height()))
                    .resolve_pt(style.size.val())
                    .max(0.0);
                self.layout_stretchy_delimiter(
                    character,
                    target * self.constants.upem / style.size.val(),
                    style,
                )
            }
        }
    }

    fn layout_delimited_sized(
        &self,
        open: char,
        body: &Content,
        close: char,
        size: Rel<Length>,
        style: &TextStyle,
    ) -> MathBox {
        let inherited = self.mid_target_pt.replace(None);
        let reference = if size.rel == 0.0 {
            0.0
        } else {
            let natural_body = self.measure_lr_body(body, style);
            let axis_pt =
                self.constants.to_pt(self.constants.axis_height, style.size).val();
            fenced_body_target_pt(body, &natural_body, axis_pt)
        };
        let target_pt = lr_requested_target_pt(reference, style.size.val(), size);
        self.mid_target_pt.set(Some(target_pt));
        let body_box = self.layout_node(body, style);
        self.mid_target_pt.set(inherited);
        let target_du = if style.size.val() > 0.0 {
            target_pt * self.constants.upem / style.size.val()
        } else {
            0.0
        };
        let mut boxes = Vec::with_capacity(3);
        if open != '\0' {
            boxes.push(self.layout_stretchy_delimiter(open, target_du, style));
        }
        boxes.push(body_box);
        if close != '\0' {
            boxes.push(self.layout_stretchy_delimiter(close, target_du, style));
        }
        self.hconcat(boxes)
    }

    pub(super) fn layout_delimited(
        &self,
        open: char,
        body: &Content,
        close: char,
        style: &TextStyle,
    ) -> MathBox {
        let body_box = self.layout_node(body, style);

        // Converter altura do corpo de pt para design units. Cercas comuns são
        // balanceadas no eixo; o frac-like sem barra de `binom` segue o braço
        // `balanced=false` do vanilla e usa a altura real do corpo.
        let axis_pt = self.constants.to_pt(self.constants.axis_height, style.size).val();
        let body_height_pt = fenced_body_target_pt(body, &body_box, axis_pt);
        let min_height_du = if style.size.val() > 0.0 {
            body_height_pt * self.constants.upem / style.size.val()
        } else {
            0.0
        };

        let open_box = self.layout_stretchy_delimiter(open, min_height_du, style);
        let close_box = self.layout_stretchy_delimiter(close, min_height_du, style);

        // **P919** — vanilla (`fenced.rs::layout_fenced`, confirmado por
        // leitura): não centra o grupo delimitado como um todo — os
        // delimitadores vêm pré-centrados (mecanismo próprio, equivalente a
        // `layout_stretchy_delimiter`) e o corpo mantém a sua própria
        // baseline, inalterada. Ver `delimited.md` §P919.
        self.hconcat(vec![open_box, body_box, close_box])
    }
}

#[cfg(test)]
mod smoke {
    #[test]
    fn module_compila_e_carrega() {}
}

#[cfg(test)]
mod issue_8973_tests {
    use super::*;
    use crate::compiler::layout::FixedMetrics;
    use crate::entities::{
        glyph_variants::{GlyphVariant, GlyphVariants},
        layout_types::{FrameItem, Point, Pt},
        style_chain::StyleChain,
    };
    struct Metrics;
    impl FontMetrics for Metrics {
        fn advance(&self, t: &str, s: Pt, st: &TextStyle) -> Pt {
            FixedMetrics.advance(t, s, st)
        }
        fn vertical_metrics(&self, s: Pt, st: &TextStyle) -> (Pt, Pt) {
            FixedMetrics.vertical_metrics(s, st)
        }
        fn cap_height(&self, s: Pt, st: &TextStyle) -> Pt {
            FixedMetrics.cap_height(s, st)
        }
        fn text_edges(&self, s: Pt, st: &TextStyle) -> (Pt, Pt) {
            FixedMetrics.text_edges(s, st)
        }
        fn vertical_glyph_variants(&self, c: char, _: &TextStyle) -> GlyphVariants {
            let start = match c {
                '[' => 100,
                ']' => 200,
                '|' => 300,
                _ => return GlyphVariants::default(),
            };
            GlyphVariants {
                variants: [700.0, 1900.0, 2900.0, 3900.0, 5900.0]
                    .into_iter()
                    .enumerate()
                    .map(|(i, advance)| GlyphVariant {
                        glyph_id: start + i as u16,
                        advance,
                        hor_advance: 300.0,
                        top_accent_attach: None,
                    })
                    .collect(),
            }
        }
    }
    fn seq(nodes: Vec<Content>) -> Content {
        Content::MathSequence(nodes.into())
    }
    fn atom(text: &str) -> Content {
        Content::MathText(text.into())
    }
    fn multi(size: Rel<Length>, explicit: bool) -> Content {
        Content::math_lr(
            seq(vec![atom("["), atom("a"), Content::linebreak(), atom("b"), atom("]")]),
            size,
            explicit,
        )
    }
    fn glyph(items: &[FrameItem], ch: char) -> (Point, u16) {
        items
            .iter()
            .find_map(|item| {
                if let FrameItem::Glyph { base_char, pos, glyph_id, .. } = item {
                    (*base_char == ch).then_some((*pos, *glyph_id))
                } else {
                    None
                }
            })
            .expect("delimiter glyph")
    }
    fn text_y(items: &[FrameItem], text: &str) -> f64 {
        items
            .iter()
            .find_map(|item| {
                if let FrameItem::Text { text: t, pos, .. } = item {
                    (t.as_str() == text).then_some(pos.y.0)
                } else {
                    None
                }
            })
            .expect("text occurrence")
    }
    #[test]
    fn issue_8973_multiline_lr_exposes_rows_and_segment_endpoints() {
        let style = TextStyle::regular(Pt(10.0));
        let layout = super::super::MathLayouter::new(&Metrics, true, &style);
        let body = multi(Rel { rel: 0.0, abs: Length::em(3.0) }, true);
        let original = body.clone();
        let (items, extent) = layout.layout_equation_measured(&body, &style);
        assert_eq!(extent.line_count, 2);
        assert!(extent.last_baseline > extent.first_baseline);
        let (open, openglyph) = glyph(&items, '[');
        let (close, closeglyph) = glyph(&items, ']');
        assert_eq!((openglyph, closeglyph), (102, 202));
        assert!(close.y.0 > open.y.0, "closing fence belongs to the last row");
        assert_eq!(body, original, "layout must not rewrite authored size/body/presence");
    }
    #[test]
    fn issue_8973_lr_break_flows_through_outer_prefix_and_suffix() {
        let style = TextStyle::regular(Pt(10.0));
        let layout = super::super::MathLayouter::new(&Metrics, true, &style);
        let body = seq(vec![
            atom("P"),
            multi(Rel { rel: 0.0, abs: Length::em(3.0) }, true),
            atom("S"),
        ]);
        let (items, extent) = layout.layout_equation_measured(&body, &style);
        assert_eq!(extent.line_count, 2);
        assert!((text_y(&items, "𝑃") - text_y(&items, "𝑎")).abs() < 1e-8);
        assert!((text_y(&items, "𝑆") - text_y(&items, "𝑏")).abs() < 1e-8);
    }
    #[test]
    fn issue_8973_lr_explicit_size_wins_over_cascade_for_both_rows() {
        let style = TextStyle::regular(Pt(10.0));
        let chain = StyleChain::default_chain()
            .push_custom("math.lr.size", Value::Length(Length::em(1.0)));
        let mut layout = super::super::MathLayouter::new(&Metrics, true, &style);
        layout.lexical_styles = Some(&chain);
        let (items, _) = layout.layout_equation_measured(
            &multi(Rel { rel: 0.0, abs: Length::em(3.0) }, true),
            &style,
        );
        assert_eq!((glyph(&items, '[').1, glyph(&items, ']').1), (102, 202));
        let (inherited, extent) = layout
            .layout_equation_measured(&multi(Rel::from_percent(100.0), false), &style);
        assert_eq!(extent.line_count, 2);
        assert_eq!((glyph(&inherited, '[').1, glyph(&inherited, ']').1), (101, 201));
    }
    #[test]
    fn issue_8973_relative_lr_uses_max_fragment_extent_not_stacked_height() {
        let style = TextStyle::regular(Pt(10.0));
        let layout = super::super::MathLayouter::new(&Metrics, true, &style);
        let frac = Content::math_frac(atom("a"), atom("b"));
        let body = Content::math_lr(
            seq(vec![atom("["), frac, Content::linebreak(), atom("c"), atom("]")]),
            Rel::from_percent(150.0),
            true,
        );
        let (items, extent) = layout.layout_equation_measured(&body, &style);
        assert_eq!(extent.line_count, 2);
        let a = glyph(&items, '[').1 - 100;
        let b = glyph(&items, ']').1 - 200;
        assert_eq!(a, b, "both fences share the maximum natural segment extent");
        assert!(a < 4, "relative target must not include the complete stacked height");
    }
    #[test]
    fn issue_8973_source_origins_and_nested_mid_scopes_remain_transparent() {
        use crate::entities::{content::SourceContent, file_id::FileId, span::Span};
        use std::{num::NonZeroU16, sync::Arc};
        let style = TextStyle::regular(Pt(10.0));
        let layout = super::super::MathLayouter::new(&Metrics, true, &style);
        let wrap = |body| {
            Content::Source(Arc::new(SourceContent {
                body,
                span: Span::from_range(
                    FileId::from_raw(NonZeroU16::new(8973).unwrap()),
                    2..8,
                ),
            }))
        };
        let source = wrap(Content::math_lr(
            seq(vec![
                wrap(atom("[")),
                wrap(atom("a")),
                wrap(Content::linebreak()),
                wrap(atom("b")),
                wrap(atom("]")),
            ]),
            Rel { rel: 0.0, abs: Length::em(3.0) },
            true,
        ));
        let original = source.clone();
        let (items, extent) = layout.layout_equation_measured(&source, &style);
        assert_eq!(extent.line_count, 2);
        assert_eq!(glyph(&items, '[').1, 102);
        assert!(glyph(&items, ']').0.y.0 > glyph(&items, '[').0.y.0);
        assert_eq!(source, original);
        let inner = Content::math_lr(
            seq(vec![atom("|"), atom("u"), atom("|")]),
            Rel { rel: 0.0, abs: Length::em(2.0) },
            true,
        );
        let body = Content::math_lr(
            seq(vec![
                atom("["),
                inner,
                Content::linebreak(),
                Content::math_mid(atom("|")),
                atom("]"),
            ]),
            Rel { rel: 0.0, abs: Length::em(3.0) },
            true,
        );
        let (items, extent) = layout.layout_equation_measured(&body, &style);
        assert_eq!(extent.line_count, 2);
        let bars: Vec<_> = items
            .iter()
            .filter_map(|item| {
                if let FrameItem::Glyph { base_char: '|', glyph_id, .. } = item {
                    Some(*glyph_id)
                } else {
                    None
                }
            })
            .collect();
        assert_eq!(bars, vec![301, 301, 302]);
        assert_eq!(layout.mid_target_pt.get(), None);
    }
}
