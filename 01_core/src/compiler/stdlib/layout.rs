//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/layout.md
//! @prompt-hash 0ee1ecf4
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
pub use self::box_elem::native_box;
pub(super) use self::casting::extract_block_height;
pub(super) use self::casting::extract_block_width;
pub(super) use self::casting::extract_box_width;
pub(crate) use self::casting::extract_corners_length_value;
pub(super) use self::casting::extract_length;
pub(super) use self::casting::extract_sides_from_value;
pub(super) use self::casting::extract_sides_lengths;
pub(super) use self::casting::extract_stroke;
pub(super) use self::casting::page_type_error;
pub use self::columns::native_colbreak;
pub use self::columns::native_columns;
pub(super) use self::grid::extract_tracks;
pub use self::grid::native_grid;
pub use self::hide::native_hide;
pub use self::measure::extract_measure_body;
pub use self::measure::native_layout;
pub use self::measure::native_measure;
pub use self::pad::native_pad;
pub(super) use self::page::is_horizontal_page_binding;
pub(super) use self::page::is_positional_paper;
pub use self::page::native_page;
pub use self::pagebreak::native_pagebreak;
pub use self::place::native_flush;
pub use self::place::native_place;
pub(super) use self::place::validate_flush_args;
pub(super) use self::repeat::extract_count;
pub use self::repeat::native_repeat;
pub use self::spacing::native_h;
pub use self::spacing::native_linebreak;
pub use self::spacing::native_v;
pub use self::stack::native_stack;
pub(super) use self::stroke::is_valid_miter_limit;
pub use self::stroke::native_stroke;

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
            _ => None, // neutro: N16[β] — Value não-Alignment retorna None na projecção de alinhamento
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
