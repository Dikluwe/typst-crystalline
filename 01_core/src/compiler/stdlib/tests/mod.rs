//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/tests/mod.md
//! @prompt-hash 090892f6
//! @layer L1
//! @updated 2026-09-26

use super::calc::{
    calc_abs,
    calc_acos,
    calc_acosh,
    calc_asin,
    calc_asinh,
    calc_atan,
    calc_atan2,
    calc_atanh,
    calc_binom,
    calc_ceil,
    calc_clamp,
    calc_cos,
    calc_cosh,
    calc_div_euclid,
    // P308 — função erro de Gauss (paridade calc 41/41).
    calc_erf,
    calc_even,
    calc_exp,
    calc_fact,
    calc_floor,
    calc_fract,
    calc_gcd,
    calc_lcm,
    calc_ln,
    calc_log,
    calc_max,
    calc_min,
    calc_norm,
    calc_odd,
    calc_perm,
    calc_pow,
    calc_quo,
    calc_rem,
    calc_rem_euclid,
    calc_root,
    calc_round,
    // P283 — trig/hyperbolic/log/exp.
    calc_sin,
    calc_sinh,
    calc_sqrt,
    calc_tan,
    calc_tanh,
    // P306 — aritmética inteira, combinatória, norma, raiz.
    calc_trunc,
    make_calc_module as p283_make_calc_module,
};
use super::shapes::parse_color;
use super::*;
use crate::compiler::eval::EvalContext;
use crate::compiler::layout::FixedMetrics;
use crate::compiler::scopes::Scopes;
use crate::contracts::world::World;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::counter::CounterKey;
use crate::entities::engine::Engine;
use crate::entities::file_id::FileId;
use crate::entities::font_book::FontBook;
use crate::entities::layout_types::{Angle, Color, Length, Ratio};
use crate::entities::show::{RuleId, ShowRule};
use crate::entities::sink::Sink;
use crate::entities::source::Source;
use crate::entities::style_chain::StyleChain;
use crate::entities::world_types::{
    Bytes, Datetime, FileError, FileResult, Font, Library, Route,
};
use comemo::{Track, TrackedMut};
use std::num::NonZeroU16;
use std::sync::Arc;

#[macro_use]
mod support;
use support::*;

mod arguments;
mod assertions;
mod bibliography;
mod calc;
mod color;
mod contextual;
mod eval;
mod figure_image;
mod footnote;
mod foundations;
mod gradients;
mod introspection;
mod layout_containers;
mod layout_context;
mod layout_spacing;
mod math;
mod shapes;
mod stroke;
mod symbols;
mod table_grid;
mod text;
mod transforms;
