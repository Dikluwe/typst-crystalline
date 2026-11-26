//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/layout.md
//! @prompt-hash 21acb2d0
//! @layer L1
//! @updated 2026-09-30
//!
//! Hub estático das funções de stdlib/layout; `align` permanece neste owner.
//! Os demais comportamentos residem nos módulos filhos.

use super::expect_no_named;
use crate::entities::file_id::FileId;
use ecow::EcoString;

use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::dir::Dir;
use crate::entities::layout_types::{Abs, Align2D, Length, TrackSizing};
use crate::entities::parity::Parity;
use crate::entities::sides::Sides;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;

// ── Align / Place (Passo 82) ────────────────────────────────────────────────

mod block;
mod box_elem;
mod casting;
mod columns;
mod grid;
mod hide;
mod measure;
mod pad;
mod page;
mod pagebreak;
mod place;
mod repeat;
mod spacing;
mod stack;
mod stroke;

pub(super) use self::block::extract_parity;
pub use self::block::native_block;
pub(crate) use self::box_elem::{declare_box, declared_box};
pub(crate) use self::block::{
    declare_block, declare_sides, declare_stroke, declared_block,
};
pub use self::box_elem::native_box;
pub(super) use self::casting::extract_block_height;
pub(super) use self::casting::extract_block_width;
pub(super) use self::casting::extract_box_width;
pub(crate) use self::casting::extract_corners_length_value;
pub(super) use self::casting::extract_length;
pub(super) use self::casting::extract_sides_from_value;
pub(super) use self::casting::extract_sides_lengths;
pub(crate) use self::casting::extract_stroke;
pub(super) use self::casting::page_type_error;
pub use self::columns::native_colbreak;
pub use self::columns::native_columns;
pub(super) use self::grid::extract_tracks;
pub use self::grid::native_grid;
pub(crate) use self::grid::{
    extract_gutters, grid_cell_fields, grid_child, grid_field, grid_fields, grid_spec,
    named_value_span,
    GRID_PROPERTIES,
};
pub use self::hide::native_hide;
pub use self::measure::extract_measure_body;
pub use self::measure::native_layout;
pub use self::measure::native_measure;
pub use self::pad::native_pad;

/// Properties of layout elements that `set` declares and their layout reads
/// when the call does not give them.
const LAYOUT_DECLARED: [(&str, &[&str]); 10] = [
    ("pad", &["left", "top", "right", "bottom", "x", "y", "rest"]),
    ("place", &["dx", "dy"]),
    ("stack", &["dir", "spacing"]),
    ("repeat", &["gap", "justify"]),
    ("outline", &["title", "depth", "target"]),
    ("quote", &["block", "quotes"]),
    ("raw", &["tab-size", "align", "theme"]),
    ("figure.caption", &["position"]),
    ("pagebreak", &["to"]),
    ("linebreak", &["justify"]),
];

/// The style entries a `set` of a layout element carries, or `None` when the
/// property has no effect yet. `pad` sides are expanded from `x`, `y` and
/// `rest` so that each side is read on its own.
pub(crate) fn declare_layout(
    element: &str,
    name: &str,
    value: &Value,
) -> Option<Vec<(String, Value)>> {
    let (_, names) = LAYOUT_DECLARED.iter().find(|(known, _)| *known == element)?;
    if !names.contains(&name) {
        return None;
    }
    let sides: &[&str] = match (element, name) {
        ("pad", "x") => &["left", "right"],
        ("pad", "y") => &["top", "bottom"],
        ("pad", "rest") => &["left", "top", "right", "bottom"],
        _ => &[],
    };
    if sides.is_empty() {
        Some(vec![(format!("{element}.{name}"), value.clone())])
    } else {
        Some(sides.iter().map(|side| (format!("pad.{side}"), value.clone())).collect())
    }
}

/// The `given` bits of a layout element call: one per declared property
/// named in the call, in the order of `LAYOUT_DECLARED`.
pub(crate) fn layout_given(element: &str, args: &Args) -> u8 {
    let Some((_, names)) = LAYOUT_DECLARED.iter().find(|(known, _)| *known == element) else {
        return 0;
    };
    names
        .iter()
        .enumerate()
        .filter(|(_, name)| args.named.contains_key(**name))
        .fold(0, |bits, (index, _)| bits | (1 << index))
}

/// Marks on a constructed layout element which properties its call gave.
pub(crate) fn with_given(value: Value, element: &str, args: &Args) -> Value {
    let given = layout_given(element, args);
    let Value::Content(content) = value else { return value };
    Value::Content(match content {
        Content::Place(mut e) => {
            std::sync::Arc::make_mut(&mut e).given = given;
            Content::Place(e)
        }
        Content::Stack(mut e) => {
            std::sync::Arc::make_mut(&mut e).given = given;
            Content::Stack(e)
        }
        Content::Repeat(mut e) => {
            std::sync::Arc::make_mut(&mut e).given = given;
            Content::Repeat(e)
        }
        Content::Outline(mut e) => {
            std::sync::Arc::make_mut(&mut e).given = given;
            Content::Outline(e)
        }
        Content::Quote(mut e) => {
            std::sync::Arc::make_mut(&mut e).given = given;
            Content::Quote(e)
        }
        other => other,
    })
}

/// The `set` declaration of a property the call did not give.
fn declared_for<'a>(
    chain: &'a crate::entities::style_chain::StyleChain,
    element: &str,
    name: &str,
    given: u8,
) -> Option<&'a Value> {
    let (_, names) = LAYOUT_DECLARED.iter().find(|(known, _)| *known == element)?;
    let index = names.iter().position(|known| *known == name)?;
    if given & (1 << index) != 0 {
        return None;
    }
    chain.custom(&format!("{element}.{name}"))
}

/// A layout element with its `set` declarations applied, when any applies.
pub(crate) fn effective_layout(
    content: &Content,
    chain: &crate::entities::style_chain::StyleChain,
    font: f64,
) -> Option<Content> {
    let as_length = |value: &Value| {
        if let Value::Length(length) = value {
            Some(*length)
        } else if let Value::Relative(rel) = value {
            (rel.rel == 0.0).then_some(rel.abs)
        } else {
            None
        }
    };
    let length = |value: &Value| as_length(value).map(|length| length.resolve_pt(font));
    match content {
        Content::Place(e) => {
            let dx = declared_for(chain, "place", "dx", e.given).and_then(length);
            let dy = declared_for(chain, "place", "dy", e.given).and_then(length);
            if dx.is_none() && dy.is_none() {
                return None;
            }
            let mut out = e.as_ref().clone();
            out.dx = dx.unwrap_or(out.dx);
            out.dy = dy.unwrap_or(out.dy);
            Some(Content::Place(std::sync::Arc::new(out)))
        }
        Content::Stack(e) => {
            let dir = declared_for(chain, "stack", "dir", e.given)
                .and_then(|value| self::stack::extract_dir(value).ok());
            let spacing = declared_for(chain, "stack", "spacing", e.given).and_then(as_length);
            if dir.is_none() && spacing.is_none() {
                return None;
            }
            let mut out = e.as_ref().clone();
            out.dir = dir.unwrap_or(out.dir);
            out.spacing = spacing.or(out.spacing);
            Some(Content::Stack(std::sync::Arc::new(out)))
        }
        Content::Repeat(e) => {
            let gap = declared_for(chain, "repeat", "gap", e.given).and_then(as_length);
            let justify = declared_for(chain, "repeat", "justify", e.given)
                .and_then(|value| if let Value::Bool(justify) = value { Some(*justify) } else { None });
            if gap.is_none() && justify.is_none() {
                return None;
            }
            let mut out = e.as_ref().clone();
            out.gap = gap.or(out.gap);
            out.justify = justify.unwrap_or(out.justify);
            Some(Content::Repeat(std::sync::Arc::new(out)))
        }
        Content::Quote(e) => {
            let block = declared_for(chain, "quote", "block", e.given)
                .and_then(|value| if let Value::Bool(value) = value { Some(*value) } else { None });
            let quotes = declared_for(chain, "quote", "quotes", e.given)
                .and_then(|value| if let Value::Bool(value) = value { Some(*value) } else { None });
            if block.is_none() && quotes.is_none() {
                return None;
            }
            let mut out = e.as_ref().clone();
            out.block = block.unwrap_or(out.block);
            if let Some(quotes) = quotes {
                out.quotes = quotes;
                out.given |= 2;
            }
            Some(Content::Quote(std::sync::Arc::new(out)))
        }
        other => {
            // Only the elements above have declarations.
            let _ = other;
            None
        }
    }
}

/// An outline with `set outline(title:, depth:)` applied where its call
/// left them out.
pub(crate) fn declared_outline(
    outline: &crate::entities::elements::outline::OutlineElem,
    chain: &crate::entities::style_chain::StyleChain,
) -> crate::entities::elements::outline::OutlineElem {
    let title = chain.custom("outline.title").map(|value| {
        if let Value::Content(title) = value {
            Some(title.clone())
        } else if let Value::Str(title) = value {
            Some(Content::text(title.as_str()))
        } else {
            None
        }
    });
    let depth = chain.custom("outline.depth").and_then(|value| {
        if let Value::Int(depth) = value {
            usize::try_from(*depth).ok()
        } else {
            None
        }
    });
    outline.with_declarations(title, depth)
}

/// A declared length of a layout element, if one was set.
pub(crate) fn declared_length(
    chain: &crate::entities::style_chain::StyleChain,
    key: &str,
) -> Option<Length> {
    let value = chain.custom(key)?;
    if let Value::Length(length) = value {
        Some(*length)
    } else if let Value::Relative(rel) = value {
        (rel.rel == 0.0).then_some(rel.abs)
    } else {
        None
    }
}

#[cfg(test)]
pub(super) use self::page::is_horizontal_page_binding;
#[cfg(test)]
pub(super) use self::page::is_positional_paper;
pub use self::page::native_page;
pub use self::pagebreak::native_pagebreak;
pub use self::place::native_flush;
pub use self::place::native_place;
#[cfg(test)]
pub(super) use self::place::validate_flush_args;
pub(super) use self::repeat::extract_count;
pub use self::repeat::native_repeat;
pub use self::spacing::native_h;
pub use self::spacing::native_linebreak;
pub use self::spacing::native_v;
pub use self::stack::native_stack;
#[cfg(test)]
pub(super) use self::stroke::is_valid_miter_limit;
pub use self::stroke::native_stroke;
pub(super) use self::stroke::positional_stroke;

/// `align(alignment, body)` → `Content::Align`.
///
/// `alignment` aceita `Value::Align` (sintaxe preferida pós-Passo 84.5,
/// ex: `align(center + bottom, ...)`) ou `Value::Str` (sintaxe legacy,
/// ex: `align("center", ...)`) — ver DEBT-36 (encerrado).
/// `body` é o primeiro argumento posicional do tipo Content.
pub fn native_align(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;

    let alignment = extract_alignment(args, Align2D::default());

    let body = args
        .items
        .iter()
        .find_map(|v| if let Value::Content(c) = v { Some(c.clone()) } else { None })
        .ok_or_else(|| {
            vec![SourceDiagnostic::error(
                args.span,
                "align() exige um bloco de conteúdo".to_string(),
            )]
        })?;

    Ok(Value::Content(Content::align(alignment, body)))
}

/// Helper Passo 84.5: extrai alinhamento do primeiro argumento posicional
/// que case com `Value::Align` ou `Value::Str`. Sintaxe preferida `Value::Align`,
/// sintaxe legacy via `Align2D::from_string`. Caso nenhum case, retorna `default`.
fn extract_alignment(args: &Args, default: Align2D) -> Align2D {
    args.items
        .iter()
        .find_map(|v| match v {
            Value::Align(a) => Some(*a),
            Value::Str(s) => Some(Align2D::from_string(s.as_str())),
            Value::None
            | Value::Bool(..)
            | Value::Int(..)
            | Value::Float(..)
            | Value::Array(..)
            | Value::Dict(..)
            | Value::Module(..)
            | Value::Datetime(..)
            | Value::Func(..)
            | Value::Content(..)
            | Value::LocatedContent(..)
            | Value::Auto
            | Value::Length(..)
            | Value::Relative(..)
            | Value::Ratio(..)
            | Value::Angle(..)
            | Value::Color(..)
            | Value::Stroke(..)
            | Value::Fraction(..)
            | Value::Location(..)
            | Value::Gradient(..)
            | Value::Regex(..)
            | Value::Tiling(..)
            | Value::Bytes(..)
            | Value::Decimal(..)
            | Value::Duration(..)
            | Value::Version(..)
            | Value::Selector(..)
            | Value::Symbol(..)
            | Value::Args(..)
            | Value::State(..)
            | Value::Counter(..)
            | Value::Label(..)
            | Value::Dir(..)
            | Value::Path(..)
            | Value::Type(..)
            | Value::SpotColorant(..) => None,
        })
        .unwrap_or(default)
}

#[cfg(test)]
mod p1292_flush_tests {
    use super::*;

    #[test]
    fn p1292_d_zero_args_is_the_only_valid_shape() {
        assert!(validate_flush_args(&Args::positional(vec![])).is_ok());

        let positional =
            validate_flush_args(&Args::positional(vec![Value::Int(1)])).unwrap_err();
        assert_eq!(positional[0].message, "unexpected argument");

        let mut named = Args::positional(vec![]);
        named.named.insert("extra".into(), Value::Bool(true));
        let named = validate_flush_args(&named).unwrap_err();
        assert_eq!(named[0].message, "unexpected argument: extra");
    }
}

#[cfg(test)]
mod p1373_block_relative_height_tests {
    use super::*;

    #[test]
    fn p1373_ratio_height_is_preserved_until_layout() {
        let height = extract_block_height(&Value::Ratio(
            crate::entities::layout_types::Ratio::from_percent(100.0),
        ))
        .expect("100% deve ser altura relativa válida");
        assert!(matches!(height,
            crate::entities::elements::block::BlockHeight::Relative(value)
                if value.rel == 1.0 && value.abs == Length::ZERO
        ));
    }
}

#[cfg(test)]
mod p1374_block_sizing_tests {
    use super::*;
    use crate::entities::elements::block::BlockHeight;

    #[test]
    fn p1374_width_ratio_and_height_fraction_keep_their_kinds() {
        let width = extract_block_width(&Value::Ratio(
            crate::entities::layout_types::Ratio::from_percent(60.0),
        ))
        .expect("60% deve ser width relativa válida");
        assert_eq!(width.rel, 0.6);
        assert_eq!(width.abs, Length::ZERO);

        let height = extract_block_height(&Value::Fraction(2.0))
            .expect("2fr deve ser height fracionária válida");
        assert!(matches!(height, BlockHeight::Fractional(2.0)));
        assert!(extract_block_width(&Value::Fraction(1.0)).is_none());
    }
}

#[cfg(test)]
mod p1491_measure_tests {
    use super::*;
    use crate::compiler::layout::{measure_content_real_with_baseline, FixedMetrics};
    use crate::entities::geometry::ShapeKind;
    use crate::entities::style_chain::StyleChain;
    use std::sync::Arc;

    #[test]
    fn measure_preserva_baseline_causal_de_texto_box_vazio_e_composto() {
        let rect = Content::shape(
            ShapeKind::Rect,
            Some(Box::new(Value::Length(Length::pt(10.0)))),
            Some(Box::new(Value::Length(Length::pt(20.0)))),
            None,
            None,
        );
        let boxed = Content::boxed(
            rect.clone(),
            None,
            None,
            Sides::uniform(Length::ZERO),
            Length::pt(3.0),
        );
        let composite =
            Content::Sequence(Arc::from(vec![Content::text("A"), boxed.clone()]));
        let measure = |content: &Content| -> (f64, f64, f64) {
            measure_content_real_with_baseline(
                content,
                &StyleChain::default_chain(),
                &FixedMetrics,
            )
        };

        let (rect_w, rect_h, rect_baseline) = measure(&rect);
        assert!((rect_w - 10.0).abs() < 1e-6);
        assert!((rect_h - 20.0).abs() < 1e-6);
        assert!((rect_baseline - 20.0).abs() < 1e-6);

        let (_, text_h, text_baseline) = measure(&Content::text("A"));
        assert!(text_h > 0.0);
        assert!((text_baseline - text_h).abs() < 1e-6);

        let (box_w, box_h, box_baseline) = measure(&boxed);
        assert!((box_h - 20.0).abs() < 1e-6);
        assert!((box_baseline - 17.0).abs() < 1e-6);

        assert_eq!(measure(&Content::Empty), (0.0, 0.0, 0.0));

        let (composite_w, composite_h, composite_baseline) = measure(&composite);
        assert!(composite_w > box_w);
        assert!((composite_h - 20.0).abs() < 1e-6);
        assert!((composite_baseline - 17.0).abs() < 1e-6);
    }
}

#[cfg(test)]
mod batch_f_alignment_tests {
    use super::*;
    #[test]
    fn batch_f_alignment_search_preserves_first_candidate() {
        let default = Align2D::from_string("right");
        let center = Align2D::from_string("center");
        let left = Align2D::from_string("left");
        let body = Value::Content(Content::text("body"));
        assert_eq!(
            extract_alignment(
                &Args::positional(vec![
                    Value::None,
                    body.clone(),
                    Value::Align(center),
                    Value::Str("left".into())
                ]),
                default
            ),
            center
        );
        assert_eq!(
            extract_alignment(
                &Args::positional(vec![
                    Value::Int(1),
                    Value::Str("left".into()),
                    Value::Align(center),
                    body
                ]),
                default
            ),
            left
        );
    }
    #[test]
    fn batch_f_alignment_non_candidates_preserve_supplied_default() {
        let default = Align2D::from_string("bottom");
        for values in [
            vec![],
            vec![Value::Content(Content::text("body"))],
            vec![
                Value::None,
                Value::Bool(true),
                Value::Int(3),
                Value::Array(vec![]),
                Value::Content(Content::text("body")),
            ],
        ] {
            assert_eq!(extract_alignment(&Args::positional(values), default), default);
        }
    }
}
