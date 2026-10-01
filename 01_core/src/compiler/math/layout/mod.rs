//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/math/layout/_comum.md
//! @prompt-hash f79bca0c
//! @layer L1
//! @updated 2026-09-30

#![allow(deprecated)] // P483 — FrameItem::Text fallback path legítimo
use std::cell::Cell;
use std::sync::Arc;

use ecow::EcoString;

use super::symbols;
use crate::compiler::layout::{vanilla_defaults::PAR_LEADING, FontMetrics};
use crate::entities::{
    content::Content,
    elements::math_attach::MathAttachSlot,
    layout_types::{Color, FrameItem, Length, MathSize, Point, Pt, TextStyle},
    math_constants::MathConstants,
    math_style::{is_math_italic_default, map_glyph, map_glyph_vs, MathStyleKind},
};

// Sub-métodos do layout matemático extraídos por fase (Passo 96.8, ADR-0037).
mod accent;
mod assembly;
mod attach;
pub mod callbacks;
mod cancel;
mod cases;
mod delimited;
mod external_bridge;
mod frac;
mod grid_composition;
mod large_operator;
mod line_decorations;
mod matrix;
mod node_wrappers;
mod op;
mod root;
mod sequence_composition;
mod spacing;
mod stretch;
mod stretchy;
mod style_transform;
mod underline;
mod underover;
mod vec;
use self::style_transform::{apply_math_default, apply_math_style};

#[cfg(test)]
pub(crate) use attach::p1444_primes_recipe;
#[cfg(test)]
pub(crate) use delimited::{p1436_effective_size, p1436_target_pt};
#[cfg(test)]
pub(crate) use delimited::{p1437_lr_mid_height, p1437_mid_target_pt};
#[cfg(test)]
pub(crate) use spacing::p1439_boundary_classes;
#[cfg(test)]
pub(crate) use spacing::p1440_boundary_classes;
#[cfg(test)]
pub(crate) use spacing::p1441_boundary_classes;
#[cfg(test)]
pub(crate) use spacing::p1442_boundary_classes;
#[cfg(test)]
pub(crate) use spacing::p1443_boundary_classes;
#[cfg(test)]
pub(crate) use spacing::p1444_boundary_classes;
#[cfg(test)]
pub(crate) use spacing::p1448_boundary_classes;
#[cfg(test)]
pub(crate) use spacing::p1449_boundary_classes;
#[cfg(test)]
pub(crate) use spacing::p1450_boundary_classes;
#[cfg(test)]
pub(crate) use spacing::p1451_boundary_classes;
#[cfg(test)]
pub(crate) use spacing::{p1436_boundary_classes, p1437_boundary_classes};

fn lower_math_overbrace(
    elem: &crate::entities::elements::math_underover::MathOverbraceElem,
) -> Content {
    let brace = Content::MathText("⏞".into());
    let inner = Content::math_underover(elem.body.clone(), None, Some(brace));
    match &elem.annotation {
        Some(annotation) => {
            Content::math_underover(inner, None, Some(annotation.clone()))
        }
        None => inner,
    }
}

fn lower_math_underbrace(
    elem: &crate::entities::elements::math_underover::MathUnderbraceElem,
) -> Content {
    let brace = Content::MathText("⏟".into());
    let inner = Content::math_underover(elem.body.clone(), Some(brace), None);
    match &elem.annotation {
        Some(annotation) => {
            Content::math_underover(inner, Some(annotation.clone()), None)
        }
        None => inner,
    }
}

fn lower_math_underbracket(
    elem: &crate::entities::elements::math_underover::MathUnderbracketElem,
) -> Content {
    let bracket = Content::MathText("⎵".into());
    let inner = Content::math_underover(elem.body.clone(), Some(bracket), None);
    match &elem.annotation {
        Some(annotation) => {
            Content::math_underover(inner, Some(annotation.clone()), None)
        }
        None => inner,
    }
}

fn lower_math_underparen(
    elem: &crate::entities::elements::math_underover::MathUnderparenElem,
) -> Content {
    let paren = Content::MathText("⏝".into());
    let inner = Content::math_underover(elem.body.clone(), Some(paren), None);
    match &elem.annotation {
        Some(annotation) => {
            Content::math_underover(inner, Some(annotation.clone()), None)
        }
        None => inner,
    }
}

fn lower_math_undershell(
    elem: &crate::entities::elements::math_underover::MathUndershellElem,
) -> Content {
    let shell = Content::MathText("⏡".into());
    let inner = Content::math_underover(elem.body.clone(), Some(shell), None);
    match &elem.annotation {
        Some(annotation) => {
            Content::math_underover(inner, Some(annotation.clone()), None)
        }
        None => inner,
    }
}

fn lower_math_overbracket(
    elem: &crate::entities::elements::math_underover::MathOverbracketElem,
) -> Content {
    let bracket = Content::MathText("⎴".into());
    let inner = Content::math_underover(elem.body.clone(), None, Some(bracket));
    match &elem.annotation {
        Some(annotation) => {
            Content::math_underover(inner, None, Some(annotation.clone()))
        }
        None => inner,
    }
}

fn lower_math_overparen(
    elem: &crate::entities::elements::math_underover::MathOverparenElem,
) -> Content {
    let paren = Content::MathText("⏜".into());
    let inner = Content::math_underover(elem.body.clone(), None, Some(paren));
    match &elem.annotation {
        Some(annotation) => {
            Content::math_underover(inner, None, Some(annotation.clone()))
        }
        None => inner,
    }
}

fn lower_math_overshell(
    elem: &crate::entities::elements::math_underover::MathOvershellElem,
) -> Content {
    let shell = Content::MathText("⏠".into());
    let inner = Content::math_underover(elem.body.clone(), None, Some(shell));
    match &elem.annotation {
        Some(annotation) => {
            Content::math_underover(inner, None, Some(annotation.clone()))
        }
        None => inner,
    }
}

#[cfg(test)]
pub(crate) fn p1439_overbrace_lowering(
    elem: &crate::entities::elements::math_underover::MathOverbraceElem,
) -> Content {
    lower_math_overbrace(elem)
}

#[cfg(test)]
pub(crate) fn p1448_underbrace_lowering(
    elem: &crate::entities::elements::math_underover::MathUnderbraceElem,
) -> Content {
    lower_math_underbrace(elem)
}

#[cfg(test)]
pub(crate) fn p1449_underbracket_lowering(
    elem: &crate::entities::elements::math_underover::MathUnderbracketElem,
) -> Content {
    lower_math_underbracket(elem)
}

#[cfg(test)]
pub(crate) fn p1450_underparen_lowering(
    elem: &crate::entities::elements::math_underover::MathUnderparenElem,
) -> Content {
    lower_math_underparen(elem)
}

#[cfg(test)]
pub(crate) fn p1451_undershell_lowering(
    elem: &crate::entities::elements::math_underover::MathUndershellElem,
) -> Content {
    lower_math_undershell(elem)
}

#[cfg(test)]
pub(crate) fn p1440_overbracket_lowering(
    elem: &crate::entities::elements::math_underover::MathOverbracketElem,
) -> Content {
    lower_math_overbracket(elem)
}

#[cfg(test)]
pub(crate) fn p1442_overparen_lowering(
    elem: &crate::entities::elements::math_underover::MathOverparenElem,
) -> Content {
    lower_math_overparen(elem)
}

#[cfg(test)]
pub(crate) fn p1443_overshell_lowering(
    elem: &crate::entities::elements::math_underover::MathOvershellElem,
) -> Content {
    lower_math_overshell(elem)
}

#[cfg(test)]
#[derive(Debug)]
pub(crate) struct P1441OverlineGeometry {
    pub(crate) width: f64,
    pub(crate) ascent: f64,
    pub(crate) descent: f64,
    pub(crate) body_ascent: f64,
    pub(crate) body_descent: f64,
    pub(crate) body_y: f64,
    pub(crate) body_cramped: bool,
    pub(crate) line_start_x: f64,
    pub(crate) line_end_x: f64,
    pub(crate) line_y: f64,
    pub(crate) line_thickness: f64,
}

#[cfg(test)]
pub(crate) fn p1441_overline_geometry() -> P1441OverlineGeometry {
    use crate::compiler::layout::FixedMetrics;

    struct Metrics;
    impl FontMetrics for Metrics {
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
        fn math_constants(&self, _style: &TextStyle) -> MathConstants {
            let mut constants = MathConstants::fallback();
            constants.overbar_vertical_gap = 200.0;
            constants.overbar_rule_thickness = 70.0;
            constants.overbar_extra_ascender = 80.0;
            constants.underbar_vertical_gap = 500.0;
            constants.underbar_rule_thickness = 110.0;
            constants.underbar_extra_descender = 130.0;
            constants
        }
        fn char_italics_correction(&self, _c: char, _size: Pt, _style: &TextStyle) -> Pt {
            Pt(1.25)
        }
    }

    let style = TextStyle::regular(Pt(10.0));
    let layouter = MathLayouter::new(&Metrics, true, &style);
    let body = Content::MathText("f".into());
    let cramped_style = TextStyle { cramped: true, ..style.clone() };
    let body_box = layouter.layout_node(&body, &cramped_style);
    let overline = layouter.layout_node(&Content::math_overline(body), &style);
    let (body_y, body_cramped) = overline
        .items
        .iter()
        .find_map(|item| match item {
            FrameItem::Text { pos, style, .. } => Some((pos.y.val(), style.cramped)),
            FrameItem::TextShaped { pos, style, .. } => {
                Some((pos.y.val(), style.cramped))
            }
            _ => None,
        })
        .expect("body textual de overline");
    let (line_start_x, line_end_x, line_y, line_thickness) = overline
        .items
        .iter()
        .find_map(|item| match item {
            FrameItem::Line { start, end, thickness, .. } => {
                Some((start.x.val(), end.x.val(), start.y.val(), *thickness))
            }
            _ => None,
        })
        .expect("regra superior de overline");
    P1441OverlineGeometry {
        width: overline.width,
        ascent: overline.ascent,
        descent: overline.descent,
        body_ascent: body_box.ascent,
        body_descent: body_box.descent,
        body_y,
        body_cramped,
        line_start_x,
        line_end_x,
        line_y,
        line_thickness,
    }
}

/// Caixa tipográfica de um nó matemático.
/// Todas as medidas são em pontos, relativas à baseline da equação.
/// `ascent` > 0 (acima da baseline), `descent` > 0 (abaixo da baseline).
//
// Visibilidade `pub(super)` nos campos (Passo 96.8): os submódulos
// `attach.rs`, `root.rs`, `frac.rs`, `stretchy.rs`, `grid_layout.rs`
// constroem e compõem `MathBox` directamente — os acessos são quase
// todos em construção (`MathBox { .. }`) ou leitura simples. A ADR-0037
// Regra 3 autoriza `pub(super)` quando métodos não agregam invariante.
#[derive(Debug, Clone)]
pub(super) struct MathBox {
    pub(super) width: f64,
    pub(super) ascent: f64,
    pub(super) descent: f64,
    /// Items com posições relativas ao topo esquerdo deste MathBox.
    pub(super) items: Vec<FrameItem>,
}

impl MathBox {
    fn height(&self) -> f64 {
        self.ascent + self.descent
    }

    /// Converte para FrameItems com posição no frame pai.
    ///
    /// `x_origin`: deslocamento horizontal no frame pai.
    /// `baseline_y`: posição da baseline no frame pai (y cresce para baixo).
    ///
    /// Conversão: `parent_y = baseline_y - ascent + local_y`
    ///   - `local_y = 0` → topo do box (acima da baseline)
    ///   - `local_y = ascent` → na baseline
    fn place(self, x_origin: f64, baseline_y: f64) -> Vec<FrameItem> {
        self.items
            .into_iter()
            .map(|mut item| {
                match item {
                    FrameItem::TextSeparator { ref mut pos, .. }
                    | FrameItem::Text { ref mut pos, .. } => {
                        pos.x = Pt(pos.x.val() + x_origin);
                        pos.y = Pt(baseline_y - self.ascent + pos.y.val());
                    }
                    FrameItem::TextShaped { ref mut pos, .. } => {
                        pos.x = Pt(pos.x.val() + x_origin);
                        pos.y = Pt(baseline_y - self.ascent + pos.y.val());
                    }
                    FrameItem::Line { ref mut start, ref mut end, .. } => {
                        start.x = Pt(start.x.val() + x_origin);
                        end.x = Pt(end.x.val() + x_origin);
                        start.y = Pt(baseline_y - self.ascent + start.y.val());
                        end.y = Pt(baseline_y - self.ascent + end.y.val());
                    }
                    FrameItem::Glyph { ref mut pos, .. } => {
                        pos.x = Pt(pos.x.val() + x_origin);
                        pos.y = Pt(baseline_y - self.ascent + pos.y.val());
                    }
                    FrameItem::Image { .. } => {} // imagens não ocorrem em contexto math
                    FrameItem::Shape { .. } => {} // formas não ocorrem em contexto math
                    FrameItem::Group { .. } => {} // grupos não ocorrem em contexto math
                    FrameItem::Link { .. } => {}  // links não ocorrem em contexto math
                    FrameItem::TextReplacement { ref mut items, .. }
                    | FrameItem::Semantic { ref mut items, .. } => {
                        for child in items {
                            *child = offset_item(
                                child.clone(),
                                Pt(x_origin),
                                Pt(baseline_y - self.ascent),
                            );
                        }
                    }
                }
                item
            })
            .collect()
    }
}

/// **P813** — Extensão geométrica de uma equação laid out, em pontos:
/// largura total e limites de tinta acima/abaixo da baseline. Produzido
/// por `MathLayouter::layout_equation_measured`; consumido pelo layout de
/// equações de bloco (`compiler/layout/equation.rs`) para centragem
/// horizontal e espaçamento vertical (paridade vanilla — ver
/// `compiler/layout/equation.md`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EquationExtent {
    /// Limite direito máximo dos items (`pos.x + advance`).
    pub width: f64,
    /// Tinta máxima acima da baseline (>= 0).
    pub ascent: f64,
    /// Tinta máxima abaixo da baseline (>= 0).
    pub descent: f64,
    /// Número de linhas do run matemático exterior.
    pub line_count: usize,
    /// Baseline da primeira linha relativa à baseline da equação.
    pub first_baseline: f64,
    /// Baseline da última linha relativa à baseline da equação.
    pub last_baseline: f64,
}

/// Desloca um `FrameItem` por `(dx, dy)`.
pub(super) fn offset_item(item: FrameItem, dx: Pt, dy: Pt) -> FrameItem {
    match item {
        FrameItem::TextSeparator { pos, advance, style, origin } => {
            FrameItem::TextSeparator {
                pos: Point { x: pos.x + dx, y: pos.y + dy },
                advance,
                style,
                origin,
            }
        }
        FrameItem::Text { pos, text, style } => FrameItem::Text {
            pos: Point {
                x: Pt(pos.x.val() + dx.val()),
                y: Pt(pos.y.val() + dy.val()),
            },
            text,
            style,
        },
        FrameItem::TextShaped { pos, glyphs, style, text, units_per_em } => {
            FrameItem::TextShaped {
                pos: Point {
                    x: Pt(pos.x.val() + dx.val()),
                    y: Pt(pos.y.val() + dy.val()),
                },
                glyphs,
                style,
                text,
                units_per_em,
            }
        }
        FrameItem::Line { start, end, thickness, color, cap } => FrameItem::Line {
            start: Point {
                x: Pt(start.x.val() + dx.val()),
                y: Pt(start.y.val() + dy.val()),
            },
            end: Point {
                x: Pt(end.x.val() + dx.val()),
                y: Pt(end.y.val() + dy.val()),
            },
            thickness,
            // P285: reflector preserva cor original (translação não afecta paint).
            color,
            cap,
        },
        FrameItem::Glyph { pos, glyph_id, x_advance, size, style, base_char } => {
            FrameItem::Glyph {
                pos: Point {
                    x: Pt(pos.x.val() + dx.val()),
                    y: Pt(pos.y.val() + dy.val()),
                },
                glyph_id,
                x_advance,
                size,
                style,
                base_char,
            }
        }
        FrameItem::Image {
            pos,
            data,
            alt,
            width,
            height,
            intrinsic_width,
            intrinsic_height,
            orientation,
            ..
        } => FrameItem::Image {
            pos: Point {
                x: Pt(pos.x.val() + dx.val()),
                y: Pt(pos.y.val() + dy.val()),
            },
            data,
            alt,
            width,
            height,
            intrinsic_width,
            intrinsic_height,
            clip_rect: None,
            orientation,
        },
        FrameItem::Shape {
            pos,
            kind,
            width,
            height,
            fill,
            stroke,
            fill_rule,
            parent_bbox_at_emit,
        } => FrameItem::Shape {
            pos: Point {
                x: Pt(pos.x.val() + dx.val()),
                y: Pt(pos.y.val() + dy.val()),
            },
            kind,
            width,
            height,
            fill,
            stroke,
            fill_rule,
            parent_bbox_at_emit,
        },
        FrameItem::Group {
            pos,
            matrix,
            clip_mask,
            inner_width,
            inner_height,
            items,
        } => FrameItem::Group {
            pos: Point {
                x: Pt(pos.x.val() + dx.val()),
                y: Pt(pos.y.val() + dy.val()),
            },
            matrix,
            clip_mask,
            inner_width,
            inner_height,
            items,
        },
        FrameItem::Link { target, items, pos, size } => FrameItem::Link {
            target,
            items,
            pos: Point {
                x: Pt(pos.x.val() + dx.val()),
                y: Pt(pos.y.val() + dy.val()),
            },
            size,
        },
        FrameItem::TextReplacement { text, items } => FrameItem::TextReplacement {
            text,
            items: items.into_iter().map(|child| offset_item(child, dx, dy)).collect(),
        },
        FrameItem::Semantic { kind, placement, alt, items } => FrameItem::Semantic {
            kind,
            placement,
            alt,
            items: items.into_iter().map(|child| offset_item(child, dx, dy)).collect(),
        },
    }
}

/// **P994** — `true` se o conteúdo exige o `layout_external` (o
/// `ExternalItem`/`BoxItem` do vanilla, `ir/resolve.rs:228-230, 192-194`):
/// contém — possivelmente sob wrappers `Sequence`/`Styled` — uma equação
/// embutida (`$…$` dentro de `text()`/`box()`/…) ou um container de
/// layout (`Boxed`/`Align`/`Pad`/`Block`), que a realização math do
/// vanilla NÃO consegue representar como corrido de texto.
///
/// Todo o resto (folhas de texto/markup, `MathOp`, `MathStyled`, e
/// `Sequence`/`Styled` só com esses dentro — ex.: output de funções de
/// utilizador, árvore medida em P966) é realizável COMO math
/// (`ir/resolve.rs:127-146`, `resolve_into_self`) e fica no caminho de
/// texto baseline-alinhado do catch-all. Mediado na Fase B: rotear esse
/// conteúdo pelo `layout_external` centrava-o no eixo (~4.5pt fora da
/// baseline da matemática vizinha, `$ 9 & "dado" $`, `$ sin(x) $`) e
/// aninhava-o num `Group` — regressão real de posicionamento, não só de
/// estrutura de items.
fn needs_external_layout(content: &Content) -> bool {
    match content {
        Content::Equation(_)
        | Content::Boxed(_)
        | Content::Align(_)
        | Content::Pad(_)
        | Content::Block(_) => true,
        Content::Sequence(items) => items.iter().any(needs_external_layout),
        // **P1027** — `Styled` que carrega propriedades de texto que o catch-all
        // de math não consegue aplicar (ele chama `layout_text_node` com o
        // `TextStyle` da equação, descartando o delta do `Styled`) sobe para
        // `layout_external`, onde a cadeia reconstruída aplica os overrides.
        // Markup puro e `Styled` semanticamente vazio mantêm o caminho
        // baseline-alinhado protegido pela adenda 2 de P994.
        Content::Styled(styled) => {
            let d = styled.styles.delta();
            d.size.is_some()
                || d.fill.is_some()
                || d.weight.is_some()
                || d.font.is_some()
                || d.tracking.is_some()
                || d.leading.is_some()
                || d.lang.is_some()
                || d.bold.is_some()
                || d.italic.is_some()
                || needs_external_layout(&styled.body)
        }
        _ => false, // neutro: N16[γ] — fallback aberto: nós não listados assumem rota de texto math; novas variantes de container/layout exigirão braço explícito sob evolução de AST
    }
}

/// Verifica se uma sequência de nós matemáticos precisa de layout em grelha.
///
/// Retorna `true` se houver pelo menos um `MathAlignPoint` ou `Linebreak`.
/// Se `false`, o layout linear existente é usado sem custo adicional.
fn needs_grid_layout(nodes: &[Content]) -> bool {
    nodes
        .iter()
        .any(|c| matches!(c, Content::MathAlignPoint(_) | Content::Linebreak(_)))
}

/// Particiona uma sequência flat em linhas e colunas.
///
/// Retorna `Vec<Vec<Vec<Content>>>`:
///   - dim 0: linhas (separadas por `Linebreak`)
///   - dim 1: colunas (separadas por `MathAlignPoint`)
///   - dim 2: items da célula
///
/// Células e linhas finais vazias são removidas.
fn partition_grid(nodes: &[Content]) -> Vec<Vec<Vec<Content>>> {
    let mut lines: Vec<Vec<Vec<Content>>> = vec![vec![vec![]]];

    for node in nodes {
        match node {
            Content::Linebreak(_) => {
                lines.push(vec![vec![]]);
            }
            Content::MathAlignPoint(_) => {
                lines.last_mut().unwrap().push(vec![]);
            }
            other => {
                lines.last_mut().unwrap().last_mut().unwrap().push(other.clone());
            }
        }
    }

    // Remover células finais vazias em cada linha
    for line in &mut lines {
        while line.last().map(|c| c.is_empty()).unwrap_or(false) {
            line.pop();
        }
    }

    // Remover linhas finais completamente vazias
    while lines.last().map(|l| l.is_empty()).unwrap_or(false) {
        lines.pop();
    }

    lines
}

/// Motor de layout matemático — stateless.
///
/// Recebe `Content` matemático e produz um `Vec<FrameItem>` com posições
/// relativas à origem `(0, 0)`, prontas para integração no layouter principal.
///
/// **Passo 36**: `MathIdent` e `MathText` → `FrameItem::Text`.
/// **Passo 37**: `MathFrac` (numerador/denominador) e `MathAttach`
///   (sup/sub com posicionamento vertical) implementados via `MathBox`.
/// **Passo 41**: constantes OpenType MATH via `FontMetrics::math_constants()`.
/// Política de alinhamento horizontal das células numa grelha matemática.
pub(super) enum GridAlign {
    /// `MathAlignPoint` (`&`): colunas pares à direita, ímpares à esquerda.
    Alternating,
    /// `MathMatrix` (`mat`): todas as células centradas na sua coluna.
    Center,
    /// `MathCases` (`cases`): todas as colunas alinhadas à esquerda.
    Left,
    /// Vetores com alinhamento horizontal à direita.
    Right,
}

// Regra 3 (ADR-0037): os campos `pub(super)` abaixo são lidos por quase
// todos os submódulos (`attach`, `root`, `frac`, `matrix`, `cases`, `stretchy`,
// `assembly`, `delimited`). `metrics` é o acesso ao trait `FontMetrics`,
// `constants` cachea as constantes OpenType MATH da fonte activa, `block`
// controla display vs inline. São dados passivos — getters triplicariam
// acessos sem invariante. Validado no Passo 97 (DEBT-47).
pub struct MathLayouter<'a, M: FontMetrics> {
    pub(super) metrics: &'a M,
    pub(super) constants: MathConstants,
    /// True se a equação é de bloco (display mode); false se inline.
    /// Controla se operadores grandes usam limites verticais (Passo 50).
    pub(super) block: bool,
    pub(super) equation: Option<crate::entities::location::Location>,
    pub(super) region_height: Pt,
    pub(super) lexical_styles: Option<&'a crate::entities::style_chain::StyleChain>,
    pub(super) callback_pass: Option<&'a callbacks::MathCallbackPassState>,
    pub(super) cancel_occurrence: Cell<usize>,
    /// Meta vertical transitória herdada pelo `math.mid` do `math.lr` mais próximo.
    pub(super) mid_target_pt: Cell<Option<f64>>,
}

impl<'a, M: FontMetrics> MathLayouter<'a, M> {
    /// **P893** — `style` propagado a `metrics.math_constants(style)`, para
    /// que `FallbackFontMetrics` resolva a face MATH activa da cadeia de
    /// fallback em vez de devolver sempre `MathConstants::fallback()`.
    pub fn new(metrics: &'a M, block: bool, style: &TextStyle) -> Self {
        let constants = metrics.math_constants(style);
        Self {
            metrics,
            constants,
            block,
            equation: None,
            region_height: Pt(0.0),
            lexical_styles: None,
            callback_pass: None,
            cancel_occurrence: Cell::new(0),
            mid_target_pt: Cell::new(None),
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new_with_context(
        metrics: &'a M,
        block: bool,
        style: &TextStyle,
        equation: crate::entities::location::Location,
        region_height: Pt,
        lexical_styles: &'a crate::entities::style_chain::StyleChain,
        callback_pass: &'a callbacks::MathCallbackPassState,
    ) -> Self {
        let mut this = Self::new(metrics, block, style);
        this.equation = Some(equation);
        this.region_height = region_height;
        this.lexical_styles = Some(lexical_styles);
        this.callback_pass = Some(callback_pass);
        this
    }

    /// Centra um MathBox no eixo matemático ajustando ascent/descent.
    ///
    /// O eixo matemático é `axis_height` (design units) acima da baseline.
    /// Após este ajuste, o centro vertical do box fica no eixo.
    ///
    /// Aplica-se a fracções, delimitadores e raízes — não a elementos inline.
    pub(super) fn apply_axis_offset(&self, mut b: MathBox, size: Pt) -> MathBox {
        let axis_pt = self.constants.to_pt(self.constants.axis_height, size).val();
        // rationale: P1064 Classe 1C — ponto médio de caixa alinhado ao eixo (axis_pt - (ascent - descent) / 2.0)
        let shift = axis_pt - (b.ascent - b.descent) / 2.0;
        b.ascent += shift;
        b.descent -= shift;
        // **P919** — bug de omissão: faltava deslocar `b.items` (os glifos
        // reais em Y) — só ascent/descent (metadados) eram ajustados. Sinal
        // `-shift` (não `+shift`): o midpoint do box pré-offset
        // (`(descent-ascent)/2` local) tem de aterrar em `-axis_pt` (eixo
        // fica ACIMA da baseline, y cresce para baixo); resolvendo
        // `y_mid + d = -axis_pt` dá `d = -shift`. Confirmado pelos ascent/
        // descent já ajustados acima (`ascent += shift` ⇔ o topo, que sobe
        // por `d`, fica `ascent_pre + shift` acima da nova baseline).
        b.items = b
            .items
            .into_iter()
            .map(|item| offset_item(item, Pt(0.0), Pt(-shift)))
            .collect();
        b
    }

    /// **P918** — Converte a altura de tinta de uma grelha (`ascent+descent`,
    /// margem de 10%, P912) para design units, para dimensionar o
    /// delimitador esticável que a envolve. Partilhado por `layout_cases` e
    /// `layout_matrix` — fórmula confirmada idêntica nos dois (ver
    /// `_comum.md` §P918).
    pub(super) fn grid_delim_target_du(
        &self,
        grid_box: &MathBox,
        style: &TextStyle,
    ) -> f64 {
        let grid_height_pt = (grid_box.ascent + grid_box.descent) * 1.1;
        if style.size.val() > 0.0 {
            grid_height_pt * self.constants.upem / style.size.val()
        } else {
            0.0
        }
    }

    /// **P945** — descida de **um nível MathSize** do vanilla
    /// (`style_for_denominator` = `style_for_numerator` + `cramped`,
    /// `lab/typst-original/crates/typst-library/src/math/style.rs:343-363`),
    /// usando o campo `TextStyle::math_size` (`entities/layout_types.md`
    /// §P945):
    ///
    /// | `style.math_size` | novo `math_size` | factor sobre `style.size` |
    /// |---|---|---|
    /// | `Display` | `Text` | ×1.0 |
    /// | `Text` | `Script` | ×`script_percent_scale_down` |
    /// | `Script` | `ScriptScript` | ×`sscript/script` |
    /// | `ScriptScript` | `ScriptScript` | ×1.0 |
    ///
    /// `cramped: true` sempre (é o denominador). Consumidores: `matrix.rs`
    /// e `cases.rs` (células) — substitui o ×`script_percent_scale_down`
    /// incondicional de P923, medido correcto só em inline (Text→Script).
    /// Ver `_comum.md` §P945.
    pub(super) fn denominator_style(&self, style: &TextStyle) -> TextStyle {
        let (math_size, factor) = match style.math_size {
            MathSize::Display => (MathSize::Text, 1.0),
            MathSize::Text => {
                (MathSize::Script, self.constants.script_percent_scale_down)
            }
            MathSize::Script => (
                MathSize::ScriptScript,
                self.constants.script_script_percent_scale_down
                    / self.constants.script_percent_scale_down,
            ),
            MathSize::ScriptScript => (MathSize::ScriptScript, 1.0),
        };
        TextStyle {
            size: style.size * factor,
            math_size,
            cramped: true,
            ..style.clone()
        }
    }

    /// **P952** — descida de **um nível MathSize** do vanilla
    /// (`style_for_numerator`,
    /// `lab/typst-original/crates/typst-library/src/math/style.rs:343-350`),
    /// simétrico de `denominator_style` (§P945 acima) — mesma tabela:
    ///
    /// | `style.math_size` | novo `math_size` | factor sobre `style.size` |
    /// |---|---|---|
    /// | `Display` | `Text` | ×1.0 |
    /// | `Text` | `Script` | ×`script_percent_scale_down` |
    /// | `Script` | `ScriptScript` | ×`sscript/script` |
    /// | `ScriptScript` | `ScriptScript` | ×1.0 |
    ///
    /// **`cramped` NÃO é forçado** — herda-o do ambiente (o vanilla
    /// `style_for_numerator` não aplica `style_cramped()`; só o denominador
    /// o faz). Consumidor: `frac.rs` (numerador — P952), em substituição do
    /// ×`script_percent_scale_down` incondicional de P915, medido correcto
    /// só em inline (Text→Script). Ver `_comum.md` §"numerator_style
    /// (novo helper, P952)" e `frac.md` §P952.
    pub(super) fn numerator_style(&self, style: &TextStyle) -> TextStyle {
        let (math_size, factor) = match style.math_size {
            MathSize::Display => (MathSize::Text, 1.0),
            MathSize::Text => {
                (MathSize::Script, self.constants.script_percent_scale_down)
            }
            MathSize::Script => (
                MathSize::ScriptScript,
                self.constants.script_script_percent_scale_down
                    / self.constants.script_percent_scale_down,
            ),
            MathSize::ScriptScript => (MathSize::ScriptScript, 1.0),
        };
        TextStyle {
            size: style.size * factor,
            math_size,
            ..style.clone()
        }
    }

    /// Ponto de entrada: recebe o body de uma equação e produz `Vec<FrameItem>`.
    ///
    /// Os items retornados têm posições **relativas à baseline da fórmula**
    /// (baseline em `y = 0`; scripts acima têm `y` negativo) — o layouter
    /// principal ajusta para posição absoluta na página somando a baseline
    /// pretendida (P800: para inline, a baseline do texto circundante).
    pub fn layout_equation(&self, body: &Content, style: &TextStyle) -> Vec<FrameItem> {
        // **P809** — itálico matemático por defeito aplicado UMA vez aqui
        // (paridade codex `MathStyle::select`). **P812** — via
        // `apply_math_default` (não `apply_math_style`): esta PRESERVA os
        // nós `MathStyled` (a sua composição — incluindo o default de
        // itálico para `bold(x)` — corre no handler dedicado em
        // `layout_node`, que também aplica o factor de tamanho); a versão
        // P809 com `apply_math_style` no topo consumia o wrapper e
        // destruía display/script/sscript (regressão medida em P812-A).
        let transformed = apply_math_default(body);
        let math_box = self.layout_node(&transformed, style);
        // `place` com baseline_y = ascent ⇒ parent_y = local_y: os items
        // internos já são compostos com y=0 na baseline (ex.: attach põe sup
        // em -sup_offset), logo o resultado fica baseline-relativo.
        let baseline_y = math_box.ascent;
        math_box.place(0.0, baseline_y)
    }

    /// **P813** — como `layout_equation`, mas devolve também a extensão
    /// geométrica da equação (`EquationExtent`), calculada dos **mesmos**
    /// items (não é um segundo caminho de layout):
    ///
    /// - `width`: limite direito máximo (`pos.x + advance`) — para a
    ///   centragem horizontal de equações de bloco;
    /// - `ascent`/`descent`: limites de **tinta** acima/abaixo da baseline,
    ///   via `FontMetrics::text_ink_bounds` — paridade vanilla, cujo frame
    ///   math usa as bounding boxes dos glyphs (não as métricas globais da
    ///   fonte). Para `FrameItem::Glyph` (delimitadores extensíveis, sem
    ///   texto Unicode) usa-se `cap_height` como aproximação.
    ///
    /// Consumidor: `compiler/layout/equation.rs` (centragem + espaçamento
    /// vertical de equações de bloco — P813).
    pub fn layout_equation_measured(
        &self,
        body: &Content,
        style: &TextStyle,
    ) -> (Vec<FrameItem>, EquationExtent) {
        let transformed = apply_math_default(body);
        let (math_box, line_baselines) = match &transformed {
            Content::MathSequence(nodes) => {
                if self.block && needs_grid_layout(nodes) {
                    self.layout_grid_with_baselines(nodes, style)
                } else {
                    (self.layout_node(&transformed, style), vec![0.0])
                }
            }
            _ => (self.layout_node(&transformed, style), vec![0.0]),
        };
        let baseline_y = math_box.ascent;
        let extent = EquationExtent {
            width: math_box.width,
            ascent: math_box.ascent,
            descent: math_box.descent,
            line_count: line_baselines.len().max(1),
            first_baseline: line_baselines.first().copied().unwrap_or(0.0),
            last_baseline: line_baselines.last().copied().unwrap_or(0.0),
        };
        let items = math_box.place(0.0, baseline_y);
        (items, extent)
    }

    /// Percorre a árvore de Content matemático recursivamente, produzindo um `MathBox`.
    pub(super) fn layout_node(&self, content: &Content, style: &TextStyle) -> MathBox {
        match content {
            Content::MathIdent(name) => {
                // **P809** — o itálico por defeito (codepoint) já foi aplicado
                // no topo (`layout_equation`); aqui é só layout. A flag de
                // fonte italic já não é usada para variáveis de 1 letra (o
                // codepoint carrega o estilo — paridade vanilla).
                let text_style = TextStyle { italic: false, ..style.clone() };
                // **P952** — operador grande (classe `Large`) de 1 carácter
                // em Display: estica via variante vertical (alvo
                // `display_operator_min_height`, sem short_fall). Inline e
                // scripts mantêm o glifo base (`_comum.md` §P952).
                if name.chars().count() == 1 {
                    let c = name.chars().next().unwrap();
                    if symbols::is_large_operator(c) {
                        return self.layout_large_operator_display(c, &text_style);
                    }
                }
                self.layout_text_node(name, &text_style)
            }
            Content::MathText(text) => {
                // **P952** — mesmo guard do braço `MathIdent` acima: o lexer
                // pode entregar `∑`/`∫` por qualquer um dos dois braços.
                if text.chars().count() == 1 {
                    let c = text.chars().next().unwrap();
                    if symbols::is_large_operator(c) {
                        return self.layout_large_operator_display(c, style);
                    }
                }
                self.layout_text_node(text, style)
            }

            Content::MathSequence(nodes) => self.layout_sequence(nodes, style),

            // Modelo D (Lote 2 P317): destructuring de `Arc<…Elem>` — mesma lógica.
            Content::MathFrac(e) => {
                use crate::entities::elements::math_frac::MathFracStyle;
                let effective = if !e.line {
                    MathFracStyle::Vertical
                } else if let Some(explicit) = e.style {
                    explicit
                } else {
                    match self
                        .lexical_styles
                        .and_then(|styles| styles.custom("math.frac.style"))
                    {
                        None => MathFracStyle::Vertical,
                        Some(crate::entities::value::Value::Str(value)) => match value
                            .as_str()
                        {
                            "vertical" => MathFracStyle::Vertical,
                            "skewed" => MathFracStyle::Skewed,
                            "horizontal" => MathFracStyle::Horizontal,
                            other => panic!("invalid validated math.frac.style: {other}"),
                        },
                        Some(other) => panic!(
                            "invalid validated math.frac.style type: {}",
                            other.type_name()
                        ),
                    }
                };
                match effective {
                    MathFracStyle::Vertical => {
                        self.layout_frac_with_line(&e.num, &e.den, e.line, style)
                    }
                    MathFracStyle::Skewed => {
                        self.layout_frac_skewed(&e.num, &e.den, style)
                    }
                    MathFracStyle::Horizontal => self.layout_frac_horizontal(
                        &e.num,
                        &e.den,
                        e.num_deparenthesized,
                        e.denom_deparenthesized,
                        style,
                    ),
                }
            }

            Content::MathAttach(e) => self.layout_attach_slots(
                &e.base, &e.t, &e.b, &e.tl, &e.bl, &e.tr, &e.br, style,
            ),

            Content::MathPrimes(e) => self.layout_primes(e, style),

            Content::MathRoot(e) => {
                self.layout_root(e.index.as_ref(), &e.radicand, style)
            }

            Content::MathStretch(e) => stretch::layout(self, e, style),

            Content::MathDelimited(e) => {
                self.layout_delimited(e.open, &e.body, e.close, style)
            }

            Content::MathLr(e) => self.layout_lr(e, style),

            Content::MathMid(e) => self.layout_mid(e, style),

            Content::MathOverbrace(e) => self.layout_overbrace(e, style),

            Content::MathUnderbrace(e) => self.layout_underbrace(e, style),

            Content::MathUnderbracket(e) => self.layout_underbracket(e, style),

            Content::MathUnderparen(e) => self.layout_underparen(e, style),

            Content::MathUndershell(e) => self.layout_undershell(e, style),

            Content::MathOverbracket(e) => self.layout_overbracket(e, style),

            Content::MathOverparen(e) => self.layout_overparen(e, style),

            Content::MathOvershell(e) => self.layout_overshell(e, style),

            Content::MathMatrix(e) => self.layout_matrix_elem(e, style),

            Content::MathCases(e) => self.layout_cases_elem(e, style),

            // P296 — Math accent/cancel handlers dedicados.
            Content::MathAccent(e) => self.layout_accent_elem(e, style),

            Content::MathCancel(e) => {
                let occurrence = self.cancel_occurrence.get();
                self.cancel_occurrence.set(occurrence + 1);
                self.layout_cancel_elem(e, style, occurrence)
            }

            Content::MathUnderline(e) => underline::layout(self, e, style),
            Content::MathOverline(e) => underline::layout_overline(self, e, style),
            Content::MathVec(e) => vec::layout(self, e, style),

            // P772y — `math.class(class, body)`: override de classe afecta
            // apenas espaçamento (spacing.rs); o layout do body é normal.
            Content::MathClassOverride(e) => self.layout_class_override(e, style),

            // P992 — `limits(body)`/`scripts(body)`: override afecta só o
            // discriminador `is_limits` de `layout_attach`; o layout do
            // body é normal (transparente, mesmo padrão de MathClassOverride).
            Content::MathLimitsOverride(e) => self.layout_node(&e.body, style),

            // P297 — Math underover (paralelo P296 layout_accent/cancel).
            Content::MathUnderover(e) => {
                self.layout_underover(&e.base, e.under.as_ref(), e.over.as_ref(), style)
            }

            // P298 — Math op (trivial delegate; limits flag consumido em layout_attach).
            Content::MathOp(e) => self.layout_op(&e.text, style),

            // P311b.4/P1291 — Math style wrapper: resolve a cadeia por eixos.
            // No mesmo eixo o setter mais interno vence; glyph, tamanho,
            // bold, italic e cramped permanecem ortogonais.
            Content::MathStyled(m) => self.layout_styled_node(m, style),

            // **P895** — espaçamentos nomeados de modo math (`thin`/`med`/
            // `thick`/`quad`/`wide`, registados em `make_math_module()` como
            // `Content::HSpace`) precisam de contribuir a largura real ao
            // `MathBox`. Sem este arm, `HSpace` caía no catch-all abaixo
            // (`other.plain_text()` — `HSpace` não tem texto, `width: 0.0`),
            // produzindo o mesmo resultado (nenhum espaço) para os 5 nomes.
            // Só `Spacing::Absolute` é resolvido — `Fractional` (`1fr`) não
            // tem "espaço restante" bem definido dentro de uma sequência
            // math de largura própria; fica scope-out (largura 0), caso não
            // exercitado pelos 5 nomes registados (todos `Absolute`).
            Content::HSpace(e) => {
                let width = match &e.amount {
                    crate::entities::elements::h_space::Spacing::Absolute(len) => {
                        len.resolve_pt(style.size.val())
                    }
                    crate::entities::elements::h_space::Spacing::Fractional(_) => 0.0,
                };
                MathBox { width, ascent: 0.0, descent: 0.0, items: vec![] }
            }

            // **P990-C/P1132q** — `strike(...)` em contexto math: sem este arm,
            // `Content::Strike` caía no catch-all `plain_text()` abaixo —
            // perdia o itálico por defeito (o corpo nunca passava por
            // `layout_node` recursivo) E a linha (o catch-all não desenha
            // decorações). O corpo é layoutado como math normal (itálico já
            // aplicado por `apply_math_default`, braço adicionado em
            // conjunto). A medição P1132q retifica a hipótese da linha: o
            // vanilla converte strike em decoração de run textual, mas os
            // glifos math deste corpo não materializam essa decoração.
            // Preserva-se, portanto, somente o layout matemático do corpo.
            // Ver `_comum.md` §P990-C.
            Content::Strike(e) => self.layout_node(&e.body, style),
            Content::Overline(e) => {
                self.layout_overline(&e.body, e.stroke.clone(), e.offset, e.extent, style)
            }
            Content::Underline(e) => self.layout_underline(
                &e.body,
                e.stroke.clone(),
                e.offset,
                e.extent,
                style,
            ),

            // **P994** — o catch-all divide-se em dois (medição da Fase B,
            // registada no relatório do passo; ver `needs_external_layout`):
            //
            // - Conteúdo realizável como corrido de TEXTO math (markup e
            //   containers math sem equação/caixa dentro — corpo de
            //   `MathOp` como `sin`, strings `"dado"` em math, output de
            //   funções de utilizador — árvore medida em P966) mantém o
            //   caminho de texto baseline-alinhado: no vanilla este
            //   conteúdo é re-realizado COMO math
            //   (`ir/resolve.rs:127-146`, `resolve_into_self`), nunca vira
            //   `ExternalItem`. Mediado: pelo `layout_external` ficava
            //   centrado no eixo (deslocado ~4.5pt da baseline da
            //   matemática vizinha, `$ 9 & "dado" $`) e aninhado num
            //   `Group` — regressão real de posicionamento, não só de
            //   estrutura de items.
            // - Conteúdo com `Equation`/`Boxed`/`Align`/`Pad`/`Block` lá
            //   dentro — NÃO realizável como math — delega ao `Layouter`
            //   normal (`layout_external`, Opção β do `_comum.md` §P994):
            //   layoutado a sério num sub-frame isolado e os items
            //   ancorados na baseline da equação, em vez de achatados por
            //   `plain_text()` (que matava itálico, `^`/`_`, tamanhos e
            //   caixas — diagnóstico de P993).
            other if !needs_external_layout(other) => {
                let text: EcoString = other.plain_text().into();
                if text.trim().is_empty() {
                    MathBox {
                        width: 0.0,
                        ascent: 0.0,
                        descent: 0.0,
                        items: vec![],
                    }
                } else {
                    // `Content::Text` é `TextItem` no vanilla e passa pelo
                    // layout inline (shaping/kerning), ao contrário das
                    // folhas `MathText`/`MathIdent`, que são glifos math.
                    // P1293 — `Content::Text` direto é `TextItem`, não
                    // `GlyphFragment`: projetamos o estilo efetivo inteiro,
                    // preservamos `math = true` (fonte/fallback/shaping) e
                    // marcamos somente a proveniência ortogonal antes de
                    // medir e emitir. Assim avanço, tinta, MathBox e
                    // FrameItem usam a mesma via, sem subtrair IC à mão.
                    // MathIdent/MathText, números e containers permanecem no
                    // caminho vigente com `math_text_item = false`.
                    let text_style = if matches!(other, Content::Text(_)) {
                        TextStyle { math_text_item: true, ..style.clone() }
                    } else {
                        style.clone()
                    };
                    let mut text_box = self.layout_text_node(&text, &text_style);
                    text_box.width = self
                        .metrics
                        .text_width(&text, text_style.size, &text_style)
                        .val();
                    text_box
                }
            }
            other => self.layout_external(other, style),
        }
    }

    /// **P906** — guard partilhado por `layout_underover`/`layout_accent`
    /// (agora em `underover.rs`/`accent.rs`, ver P909): se `c` é
    /// `Content::MathText(s)` com exactamente 1 carácter, estica-o no eixo X
    /// via `layout_stretchy_glyph_horizontal` para cobrir `min_width_du`;
    /// para qualquer outro conteúdo (multi-carácter, sequência, etc.)
    /// comportamento inalterado (`layout_node`). Ver `math/layout/_comum.md`
    /// §P906. `pub(super)` (P909) — chamado de `accent.rs`/`underover.rs`,
    /// arquivos irmãos, não descendentes deste módulo.
    pub(super) fn layout_stretchy_or_node(
        &self,
        c: &Content,
        min_width_du: f64,
        style: &TextStyle,
        short_fall_em: f64,
    ) -> MathBox {
        if let Content::MathText(s) = c {
            if s.chars().count() == 1 {
                let ch = s.chars().next().unwrap();
                return self.layout_stretchy_glyph_horizontal(
                    ch,
                    min_width_du,
                    style,
                    short_fall_em,
                );
            }
        }
        self.layout_node(c, style)
    }

    /// Nó folha: texto com métricas tipográficas.
    ///
    /// Posição do item dentro do MathBox: `(0, 0)` — relativo ao topo esquerdo.
    pub(super) fn layout_text_node(
        &self,
        text: &EcoString,
        style: &TextStyle,
    ) -> MathBox {
        if text.is_empty() {
            return MathBox {
                width: 0.0,
                ascent: 0.0,
                descent: 0.0,
                items: vec![],
            };
        }
        let width = self.metrics.advance(text, style.size, style).val();
        // **P1088** — `text_ink_bounds_signed` (P989) permite descent negativo para
        // glifos que flutuam acima da baseline (ex.: asterisco `*`, combining marks),
        // evitando falsa colisão no cálculo de gap de sub/sobrescrito (scripts.rs:464-466).
        let ib = self.metrics.text_ink_bounds_signed(text, style.size, style);
        let ascent = ib.0.val();
        let descent = ib.1.val();
        MathBox {
            width,
            ascent,
            descent,
            items: vec![FrameItem::Text {
                pos: Point { x: Pt(0.0), y: Pt(0.0) },
                text: text.clone(),
                style: style.clone(),
            }],
        }
    }
}

/// Um `TextItem` inline só perde a cola terminal quando encosta a um
/// delimitador matemático estrutural. Texto/markup adjacente conserva o frame.
pub(super) fn should_tighten_text_frame(node: &Content, next: Option<&Content>) -> bool {
    matches!((node, next), (Content::Text(_), Some(Content::MathDelimited(_))))
}

/// Caractere de uma folha matemática simples, sem atravessar containers.
fn single_math_leaf_char(content: &Content) -> Option<char> {
    let text = match content {
        Content::Text(text) | Content::MathText(text) | Content::MathIdent(text) => text,
        _ => return None,
    };
    let mut chars = text.chars();
    match (chars.next(), chars.next()) {
        (Some(c), None) => Some(c),
        _ => None,
    }
}

/// Achata estrutura matemática, mas preserva caixas de markup. `Sequence`
/// técnica só é aberta quando transporta `HSpace` no próprio nível (`dif`).
pub(super) fn flatten_math_sequence_nodes(nodes: &[Content], out: &mut Vec<Content>) {
    for node in nodes {
        match node {
            Content::MathSequence(children) => flatten_math_sequence_nodes(children, out),
            Content::Sequence(children)
                if children.iter().all(|child| {
                    matches!(
                        child,
                        Content::Text(_) | Content::MathText(_) | Content::MathIdent(_)
                    )
                }) =>
            {
                out.push(node.clone())
            }
            Content::Sequence(children) => flatten_math_sequence_nodes(children, out),
            other => out.push(other.clone()),
        }
    }
}

#[cfg(test)]
mod p311b_tests {
    use super::*;

    fn mk_ident(s: &str) -> Content {
        Content::MathIdent(s.into())
    }

    #[test]
    fn p311b4_apply_math_style_bb_substitutes_chars() {
        let body = mk_ident("x");
        let out = apply_math_style(&body, Some(MathStyleKind::DoubleStruck), None, None);
        match out {
            Content::MathIdent(s) => assert_eq!(s.as_str(), "\u{1D569}"),
            other => panic!("esperado MathIdent, obteve {other:?}"),
        }
    }

    #[test]
    fn p311b4_apply_math_style_bold_italic_orthogonal() {
        let body = mk_ident("x");
        let out = apply_math_style(&body, None, Some(true), Some(true));
        // Plain Bold Italic 'x' = U+1D499 (base U+1D468 + 26 + 23).
        match out {
            Content::MathIdent(s) => assert_eq!(s.as_str(), "\u{1D499}"),
            other => panic!("esperado MathIdent, obteve {other:?}"),
        }
    }

    #[test]
    fn p809_apply_math_style_bold_compoe_italic_default() {
        // P809 — paridade vanilla medida: `$bold(x)$` → bold-ITALIC (U+1D499),
        // não bold upright. O default de italic em folhas de 1 carácter é
        // `is_math_italic_default` (codex `MathStyle::select`).
        let body = mk_ident("x");
        let out = apply_math_style(&body, None, Some(true), None);
        match out {
            Content::MathIdent(s) => assert_eq!(
                s.as_str(),
                "\u{1D499}",
                "bold(x) deve compor com o itálico por defeito"
            ),
            other => panic!("esperado MathIdent, obteve {other:?}"),
        }
    }

    #[test]
    fn p809_apply_math_style_upright_continua_plain() {
        // Controlo: upright(x) com italic Some(false) explícito → plain.
        let body = mk_ident("x");
        let out = apply_math_style(&body, None, None, Some(false));
        match out {
            Content::MathIdent(s) => assert_eq!(s.as_str(), "x"),
            other => panic!("esperado MathIdent, obteve {other:?}"),
        }
    }

    #[test]
    fn p1291_apply_math_style_bb_cal_inner_wins() {
        // Medição vanilla P1291: bb(cal(x)) preserva o setter mais interno.
        let inner = Content::math_styled(
            Some(MathStyleKind::Chancery),
            None,
            None,
            mk_ident("x"),
            None,
        );
        let out = apply_math_style(&inner, Some(MathStyleKind::DoubleStruck), None, None);
        let expected =
            apply_math_style(&mk_ident("x"), Some(MathStyleKind::Chancery), None, None);
        assert_eq!(out, expected, "inner cal must win over outer bb");
    }

    #[test]
    fn p1291_glyph_exterior_preserva_inline_e_cramped_interiores() {
        use crate::compiler::layout::FixedMetrics;

        let inner_inline = Content::math_styled(
            Some(MathStyleKind::Inline),
            None,
            None,
            mk_ident("x"),
            Some(true),
        );
        let outer_bb = Content::math_styled(
            Some(MathStyleKind::DoubleStruck),
            None,
            None,
            inner_inline,
            None,
        );
        let input_style = TextStyle {
            math_size: MathSize::Display,
            ..TextStyle::regular(Pt(12.0))
        };
        let layouter = MathLayouter::new(&FixedMetrics, true, &input_style);
        let box_ = layouter.layout_node(&outer_bb, &input_style);
        let Some(FrameItem::Text { text, style, .. }) = box_.items.first() else {
            panic!("styled leaf must yield a text item: {:?}", box_.items)
        };
        assert_eq!(text.as_str(), "\u{1D569}", "outer bb glyph must remain");
        assert_eq!(style.math_size, MathSize::Text, "inner inline size was lost");
        assert!(style.cramped, "inner inline cramped was lost");
    }

    #[test]
    fn p1291_scripts_override_mantem_attachments_laterais() {
        use crate::compiler::layout::FixedMetrics;

        fn x_of(items: &[FrameItem], needle: &str) -> f64 {
            items
                .iter()
                .find_map(|item| match item {
                    FrameItem::Text { pos, text, .. } if text.as_str() == needle => {
                        Some(pos.x.val())
                    }
                    _ => None,
                })
                .unwrap_or_else(|| panic!("missing {needle:?} in {items:?}"))
        }

        let style = TextStyle::regular(Pt(12.0));
        let layouter = MathLayouter::new(&FixedMetrics, true, &style);
        let attach = Content::math_attach_scripts(
            Content::math_limits_override(
                Content::MathText("∑".into()),
                false,
                true,
                false,
            ),
            MathAttachSlot::Omitted,
            MathAttachSlot::Omitted,
            MathAttachSlot::Present(Content::MathText("1".into())),
            MathAttachSlot::Present(Content::MathText("2".into())),
        );
        let items = layouter.layout_equation(&attach, &style);
        let base_x = x_of(&items, "∑");
        let sup_x = x_of(&items, "2");
        assert!(
            sup_x > base_x + 3.6,
            "scripts(sum) must put superscript laterally: base={base_x}, sup={sup_x}"
        );
    }

    #[test]
    fn p311b4_apply_math_style_bold_preserves_inner_bb() {
        // bold(bb(x)) — inner Bb preservado; outer bold ortogonal aplicado.
        let inner = Content::math_styled(
            Some(MathStyleKind::DoubleStruck),
            None,
            None,
            mk_ident("x"),
            None,
        );
        let out = apply_math_style(&inner, None, Some(true), None);
        // Bold Double-Struck small x: U+1D569 (DS x não tem variant bold no
        // plano; nossa tabela aplica DS base). Aceita qualquer variant
        // contendo DS via verificação parcial.
        match out {
            Content::MathIdent(s) => {
                let c = s.chars().next().unwrap();
                let u = c as u32;
                // Espera dentro do plano DS U+1D552-U+1D56B (lowercase) ou
                // similar; aceita variantes encoded.
                assert!(
                    u >= 0x1D552 && u <= 0x1D56B,
                    "esperado DS lowercase, obteve U+{:X}",
                    u
                );
            }
            other => panic!("esperado MathIdent, obteve {other:?}"),
        }
    }

    #[test]
    fn p311b4_apply_math_style_recurses_through_mathfrac() {
        let frac = Content::math_frac(mk_ident("a"), mk_ident("b"));
        let out = apply_math_style(&frac, Some(MathStyleKind::DoubleStruck), None, None);
        match out {
            Content::MathFrac(e) => {
                match (&e.num, &e.den) {
                    (Content::MathIdent(n), Content::MathIdent(d)) => {
                        assert_eq!(n.as_str(), "\u{1D552}"); // DS a
                        assert_eq!(d.as_str(), "\u{1D553}"); // DS b
                    }
                    other => panic!("esperado MathIdent/MathIdent, obteve {other:?}"),
                }
            }
            other => panic!("esperado MathFrac, obteve {other:?}"),
        }
    }

    #[test]
    fn p311b4_apply_math_style_size_variant_passthrough_glyph() {
        // script(x) — kind Script é size variant: não tem mapping de glifo
        // próprio, mas o eixo itálico é ortogonal (P812 — paridade vanilla
        // medida: `$script(x)$` → 𝑥 U+1D465). Antes de P812 a asserção era
        // passthrough plain "x" — incorrecta face ao vanilla.
        let out =
            apply_math_style(&mk_ident("x"), Some(MathStyleKind::Script), None, None);
        match out {
            Content::MathIdent(s) => assert_eq!(s.as_str(), "\u{1D465}"),
            other => panic!("esperado MathIdent, obteve {other:?}"),
        }
    }

    #[test]
    fn p311b4_apply_math_style_math_op_passthrough() {
        // bb(op("sin")) — operadores texto não devem receber variant.
        let op = Content::math_op(Content::text("sin"), false);
        let out = apply_math_style(&op, Some(MathStyleKind::DoubleStruck), None, None);
        match out {
            Content::MathOp(e) => {
                assert_eq!(e.text.plain_text(), "sin");
                assert!(!e.limits);
            }
            other => panic!("esperado MathOp, obteve {other:?}"),
        }
    }
}

#[cfg(test)]
mod p1293_layout_owner_gap_tests {
    use super::*;
    use crate::compiler::layout::{FixedMetrics, FontMetrics};

    fn style() -> TextStyle {
        TextStyle::regular(Pt(12.0))
    }

    #[test]
    fn p1293_binom_separator_terminal_whitespace_tem_contribuicao_unica() {
        let style = style();
        let layouter = MathLayouter::new(&FixedMetrics, false, &style);
        let measured = layouter.layout_sequence(
            &[
                Content::MathIdent("k".into()),
                Content::MathText(", ".into()),
                Content::MathIdent("j".into()),
            ],
            &style,
        );
        let canonical = layouter.layout_sequence(
            &[
                Content::MathIdent("k".into()),
                Content::MathText(",".into()),
                Content::MathIdent("j".into()),
            ],
            &style,
        );
        assert_eq!(measured.width, canonical.width);
        assert_eq!(measured.items.len(), canonical.items.len());
    }

    #[test]
    fn p1293_whitespace_terminal_sem_irmao_material_permanece() {
        let style = style();
        let layouter = MathLayouter::new(&FixedMetrics, false, &style);
        let with_space = layouter.layout_sequence(
            &[Content::MathIdent("k".into()), Content::MathText(", ".into())],
            &style,
        );
        let without_space = layouter.layout_sequence(
            &[Content::MathIdent("k".into()), Content::MathText(",".into())],
            &style,
        );
        assert!(with_space.width > without_space.width);
    }

    #[test]
    fn p1293_separador_normalizado_nao_reativa_fallback_de_texto_em_script() {
        let mut style = style();
        style.math_script = true;
        style.math_size = MathSize::Script;
        let layouter = MathLayouter::new(&FixedMetrics, false, &style);
        let measured = layouter.layout_sequence(
            &[
                Content::Text("k".into()),
                Content::MathText(", ".into()),
                Content::Text("j".into()),
            ],
            &style,
        );
        let canonical = layouter.layout_sequence(
            &[
                Content::Text("k".into()),
                Content::MathText(",".into()),
                Content::MathText("j".into()),
            ],
            &style,
        );
        assert_eq!(measured.width, canonical.width);
    }

    struct MetricsComIcNoAdvance;

    impl FontMetrics for MetricsComIcNoAdvance {
        fn advance(&self, text: &str, size: Pt, style: &TextStyle) -> Pt {
            let base = FixedMetrics.advance(text, size, style);
            if style.math && !style.math_text_item && text.chars().count() == 1 {
                base + Pt(0.5)
            } else {
                base
            }
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

        fn char_italics_correction(&self, _c: char, _size: Pt, _style: &TextStyle) -> Pt {
            Pt(0.5)
        }
    }

    #[test]
    fn p1293_content_text_direto_neutraliza_ic_sem_afetar_folhas_math() {
        let style = TextStyle {
            math: true,
            cramped: true,
            math_size: MathSize::Script,
            ..style()
        };
        let layouter = MathLayouter::new(&MetricsComIcNoAdvance, false, &style);
        let nominal = FixedMetrics.advance("x", style.size, &style).val();

        let direct = layouter.layout_node(&Content::Text("x".into()), &style);
        assert_eq!(direct.width, nominal);
        let Some(FrameItem::Text { style: emitted, .. }) = direct.items.first() else {
            panic!("Content::Text direto deve emitir FrameItem::Text")
        };
        assert!(emitted.math, "TextItem direto deve conservar o contexto math");
        assert!(
            emitted.math_text_item,
            "TextItem direto deve transportar somente a nova proveniência"
        );
        assert_eq!(emitted.size, style.size);
        assert_eq!(emitted.math_size, style.math_size);
        assert_eq!(emitted.cramped, style.cramped);

        for math_leaf in [
            Content::MathIdent("x".into()),
            Content::MathText("x".into()),
            Content::MathText("7".into()),
        ] {
            let measured = layouter.layout_node(&math_leaf, &style);
            assert_eq!(measured.width, nominal + 0.5);
            let Some(FrameItem::Text { style: emitted, .. }) = measured.items.first()
            else {
                panic!("folha math deve emitir FrameItem::Text")
            };
            assert!(emitted.math, "folha math não pode perder a via matemática");
            assert!(
                !emitted.math_text_item,
                "folha math não pode adquirir proveniência TextItem"
            );
        }
    }

    #[test]
    fn p1293_ic_so_e_neutralizada_em_content_text_direto_de_um_glifo() {
        let style = TextStyle { math: true, ..style() };
        let layouter = MathLayouter::new(&MetricsComIcNoAdvance, false, &style);
        let wrapped = Content::Sequence(Arc::from(
            vec![Content::Text("x".into()), Content::Empty].into_boxed_slice(),
        ));
        let wrapped_box = layouter.layout_node(&wrapped, &style);
        let wrapped_nominal =
            MetricsComIcNoAdvance.text_width("x", style.size, &style).val();
        assert_eq!(wrapped_box.width, wrapped_nominal);
        let Some(FrameItem::Text { style: emitted, .. }) = wrapped_box.items.first()
        else {
            panic!("container textual deve emitir FrameItem::Text")
        };
        assert!(
            emitted.math,
            "container não deve ser reclassificado como TextItem direto"
        );
        assert!(
            !emitted.math_text_item,
            "container não deve adquirir proveniência de folha direta"
        );

        let multiple = layouter.layout_node(&Content::Text("xy".into()), &style);
        let multiple_nominal =
            MetricsComIcNoAdvance.text_width("xy", style.size, &style).val();
        assert_eq!(multiple.width, multiple_nominal);
        let Some(FrameItem::Text { style: emitted, .. }) = multiple.items.first() else {
            panic!("Content::Text direto multiglifo deve emitir FrameItem::Text")
        };
        assert!(emitted.math, "TextItem multiglifo mantém a cadeia matemática");
        assert!(
            emitted.math_text_item,
            "todo Content::Text direto deve transportar a proveniência"
        );
    }
}

#[cfg(test)]
mod tests;

#[cfg(test)]
mod p1425_tests;

#[cfg(test)]
mod p1426_tests;

#[cfg(test)]
mod p1427_tests;

#[cfg(test)]
mod p1429_tests;

#[cfg(test)]
mod p1430_tests;

#[cfg(test)]
mod p1431_tests;
