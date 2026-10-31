//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/mod.md
//! @prompt-hash 259f3279
//! @layer L1
//! @updated 2026-10-06

//! Stdlib nativa mínima — Passo 17.
//!
//! Interface `fn(&mut EvalContext, &Args) -> SourceResult<Value>` (Passo 71, DEBT-24):
//! aceita positional e named args. Funções sem I/O usam `_ctx`.
//!
//! Reestruturado por cluster em Passo 96.5 conforme ADR-0037.

use ecow::EcoString;
use indexmap::IndexMap;
use rustc_hash::FxBuildHasher;

use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;

// ── Submódulos por cluster (Passo 96.5, ADR-0037) ───────────────────────────
mod assert;
mod calc;
mod figure_image;
mod format;
mod foundations;
mod html;
mod label;
mod layout;
mod panic;
mod r#ref;
mod shapes;
mod structural;

// Internal cast bridges preserve the single structural classifier owner.
pub(in crate::compiler) use structural::symbol_accent_func;
pub(crate) use structural::{is_callable_lr_symbol, is_known_math_accent};
mod text;
mod transforms;
// P262 — Gradient stdlib (Linear only per ADR-0087).
pub(crate) mod gradients;
// P311b.3 — 12 funções math style (bb/cal/frak/etc.).
mod math_style;
// P387 (ADR-0111) — data import: read/csv/json/yaml/toml/cbor/xml.
mod loading;
// P697 — builtin `plugin()` (nível 2 de P696: sintaxe + leitura de bytes).
mod plugin;
// P394 — runtime de re-avaliação `eval(source)`.
mod eval;
// P396 — constructor `tiling(...)` e helpers visuais.
mod visualize;
// P403 — constructors stdlib para tipos primitivos L1: decimal, duration, version.
mod primitives_constructors;
// P466 — métodos de instância para array, dict e str.
mod collections;
// P471 — módulo `sym` com tabela de símbolos Unicode.
pub(crate) mod sym;
// P735 — módulo `emoji` (tabela codex-0.2.0, 1 codepoint) e módulo `pdf`.
mod emoji;
mod pdf;
// P476 — módulo `color` com operadores lighten/darken/mix/negate.
// P742 — pub(crate): o despacho de métodos de instância em `eval/` acede
// a `is_color_instance_method` e às nativas.
pub(crate) mod color;
// P694 — módulo `sys` (sys.version / sys.inputs).
mod sys;
// P506 — runtime state/counter/context.
pub(crate) mod context;
pub(crate) mod counter;
pub(crate) mod state;
// P847 — função global `numbering()` e `format_pattern` partilhado
// (extraído de `structural.rs`; um ficheiro, um prompt).
mod numbering;
mod registry;
pub(crate) use registry::make_stdlib_with_features;

// Re-exports públicos — preservam o path `crate::compiler::stdlib::native_X` usado
// por `make_stdlib` em `eval/mod.rs`.
pub use crate::compiler::stdlib::assert::{
    native_assert, native_assert_eq, native_assert_ne,
};
pub(crate) use crate::compiler::stdlib::calc::calc_abs;
pub use crate::compiler::stdlib::calc::make_calc_module;
pub use crate::compiler::stdlib::eval::native_eval;
pub use crate::compiler::stdlib::figure_image::{
    native_figure, native_figure_caption, native_image,
};
pub(crate) use crate::compiler::stdlib::format::make_format_module;
pub(crate) use crate::compiler::stdlib::foundations::float::dispatch_float_method_spanned;
pub(crate) use crate::compiler::stdlib::foundations::selector::element_kind_of_native_func;
pub use crate::compiler::stdlib::foundations::{
    datetime_type_field, int_type_field, native_bytes, native_cmyk, native_datetime,
    native_float, native_here, native_hsl, native_hsv, native_int, native_linear_rgb,
    native_locate, native_luma, native_metadata, native_oklab, native_oklch, native_path,
    native_query, native_range, native_regex, native_repr, native_rgb, native_selector,
    native_str, native_str_from_unicode, native_symbol, native_target, native_type,
    read_path_value, resolve_path_value,
};
pub(crate) use crate::compiler::stdlib::foundations::{
    dispatch_float_method, dispatch_int_method, float_type_field,
    is_float_instance_method, is_int_instance_method,
};
pub use crate::compiler::stdlib::html::{make_html_module, native_html_elem};
pub(crate) use crate::compiler::stdlib::primitives_constructors::{
    dispatch_version_method, native_array_bytes, version_type_field,
};
// P506 — state/counter/context como valores de primeira classe.
pub use crate::compiler::stdlib::context::native_context;
pub use crate::compiler::stdlib::counter::{
    counter_at, counter_display, counter_get, counter_step, counter_update,
    native_counter, native_counter_at, native_counter_display, native_counter_final,
    native_counter_step,
};
pub use crate::compiler::stdlib::label::native_label;
pub use crate::compiler::stdlib::numbering::{native_numbering, realize_numbering};
pub use crate::compiler::stdlib::panic::native_panic;
pub use crate::compiler::stdlib::r#ref::native_ref;
pub use crate::compiler::stdlib::state::{
    native_state, native_state_at, native_state_display, native_state_final,
    native_state_update, native_state_update_with, state_display, state_get,
    state_update, value_to_content,
};
pub(in crate::compiler) use crate::compiler::stdlib::structural::native_math_equation;
pub(crate) use crate::compiler::stdlib::structural::{
    apply_math_cases_option, apply_math_matrix_option, equation_alt, equation_content,
    equation_number_align, equation_supplement, math_abs_from_args, math_accent_content,
    math_cases_delim, math_lr_from_args, math_mid_from_args, math_norm_from_args,
    math_overbrace_from_args, math_overbracket_from_args, math_overline_from_args,
    math_overparen_from_args, math_overshell_from_args, math_round_from_args,
    math_stretch_from_args, math_underbrace_from_args, math_underbracket_from_args,
    math_underparen_from_args, math_undershell_from_args, realize_outline_entry_page,
    sqrt_content, MathCasesOptions, MathMatrixOptions,
};
pub(in crate::compiler) use structural::{table_alignment, table_alignment_at};

pub use crate::compiler::stdlib::structural::{
    make_math_module, native_accent, native_asset, native_bibliography, native_cancel,
    native_cite, native_divider, native_document, native_emph, native_enum,
    native_enum_item, native_footnote, native_footnote_entry, native_grid_cell,
    native_grid_footer, native_grid_header, native_grid_hline, native_grid_vline,
    native_heading, native_link, native_list, native_list_item, native_lof, native_lot,
    native_op, native_outline, native_outline_entry, native_outline_entry_body,
    native_outline_entry_indented, native_outline_entry_inner, native_outline_entry_page,
    native_outline_entry_prefix, native_par, native_par_line, native_parbreak,
    native_quote, native_raw, native_strong, native_table, native_table_cell,
    native_table_footer, native_table_header, native_table_hline, native_table_vline,
    native_terms, native_terms_item, native_title, native_underover,
};
pub use crate::compiler::stdlib::text::{
    native_highlight, native_lorem, native_lower, native_overline, native_replace,
    native_smallcaps, native_smartquote, native_strike, native_subscript,
    native_superscript, native_text, native_underline, native_upper,
};
// P387 (ADR-0111) — data import.
pub use crate::compiler::stdlib::layout::{
    extract_measure_body, native_align, native_block, native_box, native_colbreak,
    native_columns, native_flush, native_grid, native_h, native_hide, native_layout,
    native_linebreak, native_measure, native_pad, native_page, native_pagebreak,
    native_place, native_repeat, native_stack, native_stroke, native_v,
};
pub use crate::compiler::stdlib::loading::{
    native_cbor, native_cbor_encode, native_csv, native_json, native_read, native_toml,
    native_xml, native_yaml,
};
pub(crate) use crate::compiler::stdlib::loading::{
    native_json_encode, native_toml_encode, native_yaml_encode,
};
// P697 — builtin plugin; P819 — `plugin.transition` (namespace).
pub use crate::compiler::stdlib::plugin::{native_plugin, native_plugin_transition};
pub use crate::compiler::stdlib::shapes::{
    native_circle, native_curve, native_curve_close, native_curve_cubic,
    native_curve_line, native_curve_move, native_curve_quad, native_ellipse, native_line,
    native_polygon, native_polygon_regular, native_rect, native_square,
};
pub use crate::compiler::stdlib::transforms::{
    native_move, native_rotate, native_scale, native_skew,
};
// P262 — Gradient stdlib (Linear only per ADR-0087).
// P264 — Gradient stdlib Radial added per ADR-0088.
// P267 — Gradient stdlib Conic added per ADR-0089 (cluster 3/3).
// P736 — `gradient_type_field` (era `make_gradient_module`; tipo, não dict).
pub use crate::compiler::stdlib::gradients::{
    gradient_type_field, native_gradient_conic, native_gradient_linear,
    native_gradient_radial,
};
// P396 — constructor `tiling(...)`.
pub use crate::compiler::stdlib::visualize::native_tiling;
// P403 — constructors stdlib para tipos primitivos L1.
pub use crate::compiler::stdlib::primitives_constructors::{
    native_decimal, native_duration, native_version,
};
// P466 — dispatcher de métodos de array/dict/str.
pub(crate) use crate::compiler::stdlib::collections::{
    collection_type_field, try_dispatch_collection_method,
    try_dispatch_collection_method_with, CollectionCallSpans, CollectionDispatch,
};
// P471 — módulo sym.
pub use crate::compiler::stdlib::sym::build_sym_module;
// P735 — módulos emoji e pdf.
pub use crate::compiler::stdlib::emoji::build_emoji_module;
pub use crate::compiler::stdlib::pdf::make_pdf_module;
// P476 — operadores de cor. **P736** — `color_type_field` (era
// `make_color_module`; `color` é `Value::Type`, não `Value::Dict`).
pub use crate::compiler::stdlib::color::{color_type_field, predefined_color_bindings};
// P1473 — `color.spot(name, fallback)`, despachado via `Value::Type(Type::
// SpotColorant)` em `eval/call_dispatch.rs` (mesmo mecanismo de `int`/`str`).
pub(crate) use crate::compiler::stdlib::color::native_spot_colorant_construct;
// P694 — módulo sys.
pub use crate::compiler::stdlib::sys::make_sys_module;
// P311b.3 + P765b — 14 funções math style.
pub use crate::compiler::stdlib::math_style::{
    native_bb, native_bold, native_cal, native_display, native_frak, native_inline,
    native_math_italic, native_mono, native_sans, native_scr, native_script,
    native_serif, native_sscript, native_upright,
};

// ── Helpers partilhados ─────────────────────────────────────────────────────

/// Constrói um `SourceResult::Err` com mensagem única e span detached.
pub(super) fn err(msg: impl Into<String>) -> SourceResult<Value> {
    Err(vec![SourceDiagnostic::error(Span::detached(), msg.into())])
}

/// Verifica que não foram passados argumentos nomeados não esperados (Passo 64).
///
/// O Typst original é rigoroso: argumentos nomeados desconhecidos são
/// erros semânticos, não silenciosos. Ignorá-los criaria uma linguagem
/// permissiva que esconde typos do utilizador.
pub(super) fn expect_no_named(
    named: &IndexMap<EcoString, Value, FxBuildHasher>,
) -> SourceResult<()> {
    if let Some((key, _)) = named.iter().next() {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("argumento nomeado inesperado: '{}'", key),
        )]);
    }
    Ok(())
}

pub(in crate::compiler) use structural::{
    native_math_attach, native_math_binom, native_math_cases,
    native_math_class_for_metadata, native_math_frac, native_math_limits, native_math_lr,
    native_math_mid, native_math_overbrace, native_math_overbracket,
    native_math_overline, native_math_overparen, native_math_overshell,
    native_math_primes, native_math_root, native_math_scripts, native_math_stretch,
    native_math_underbrace, native_math_underbracket, native_math_underline,
    native_math_underparen, native_math_undershell, native_math_vec,
};

pub(in crate::compiler) use html::native_html_frame;
pub(in crate::compiler) use pdf::{native_pdf_artifact, native_pdf_attach};

#[cfg(test)]
mod tests;

pub(in crate::compiler) use structural::native_math_mat;
