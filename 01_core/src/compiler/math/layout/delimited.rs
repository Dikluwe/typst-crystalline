//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/math/layout/delimited.md
//! @prompt-hash ceb6ae30
//! @layer L1
//! @updated 2026-04-23
//!
//! Método `layout_delimited` de `MathLayouter`. Extraído de `math/layout/mod.rs`
//! no Passo 96.8 conforme ADR-0037.

use crate::compiler::layout::FontMetrics;
use crate::entities::{
    content::Content,
    elements::math_delimited::MathLrElem,
    layout_types::{Length, TextStyle},
    rel::Rel,
    value::Value,
};

use super::MathBox;

fn effective_lr_size(elem: &MathLrElem, cascaded: Option<Rel<Length>>) -> Rel<Length> {
    if elem.size_explicit {
        elem.size
    } else {
        cascaded.unwrap_or_else(|| Rel::from_percent(100.0))
    }
}

fn lr_target_pt(body_height: f64, font_size: f64, size: Rel<Length>) -> f64 {
    let requested = size.resolve(Length::pt(body_height)).resolve_pt(font_size);
    (requested - 0.1 * font_size).max(0.0)
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
                let natural = self.layout_node(body, style);
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
                    return natural;
                };
                if size == Rel::from_percent(100.0) {
                    return natural;
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
        let body_box = self.layout_node(body, style);
        let target_pt = lr_target_pt(body_box.height(), style.size.val(), size);
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
        let balanced = !matches!(body, Content::MathFrac(e) if !e.line);
        let body_height_pt = if balanced {
            let max_extent_pt =
                (body_box.ascent - axis_pt).max(body_box.descent + axis_pt);
            2.0 * max_extent_pt
        } else {
            body_box.height()
        };
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
