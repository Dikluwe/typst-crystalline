//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/math/layout/delimited.md
//! @prompt-hash 5451ffc8
//! @layer L1
//! @updated 2026-04-23
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
    pub(super) fn layout_mid(&self, elem: &MathMidElem, style: &TextStyle) -> MathBox {
        let Some(target_pt) = self.mid_target_pt.get() else {
            return self.layout_node(&elem.body, style);
        };
        let text = match &elem.body {
            Content::Text(text) | Content::MathText(text) | Content::MathIdent(text) => {
                Some(text.as_str())
            }
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
        let size = effective_lr_size(elem, cascaded);
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
                let natural = self.layout_node(body, style);
                let target =
                    lr_requested_target_pt(natural.height(), style.size.val(), size);
                self.mid_target_pt.set(Some(target));
                let with_mid = self.layout_node(body, style);
                self.mid_target_pt.set(inherited);
                let text = match body {
                    Content::Text(text)
                    | Content::MathText(text)
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
        let natural_body = self.layout_node(body, style);
        let axis_pt = self.constants.to_pt(self.constants.axis_height, style.size).val();
        let target_pt = lr_requested_target_pt(
            fenced_body_target_pt(body, &natural_body, axis_pt),
            style.size.val(),
            size,
        );
        self.mid_target_pt.set(Some(target_pt));
        let body_box = self.layout_node(body, style);
        self.mid_target_pt.set(inherited);
        let target_du = if style.size.val() > 0.0 {
            target_pt * self.constants.upem / style.size.val()
        } else {
            0.0
        };
        let open_box = self.layout_stretchy_delimiter(open, target_du, style);
        let close_box = self.layout_stretchy_delimiter(close, target_du, style);
        self.hconcat(vec![open_box, body_box, close_box])
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
