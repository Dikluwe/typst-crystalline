//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/foundations.md
//! @prompt-hash 1e5539d1
//! @layer L1
//! @updated 2026-08-18
//!
//! Hub do módulo `foundations`: reexportação dos nós.
//! Fatiado de `foundations.rs` monolítico no Passo 1032.
//! P1077: removido o nó `len` (extensão global removida para paridade vanilla).

pub mod cast;
pub mod color;
pub mod datetime;
pub mod float;
pub mod int;
pub mod path;
pub mod query;
pub mod regex_constructor;
pub mod repr;
pub mod selector;
pub mod str;
pub mod ty;

pub use cast::{
    native_bytes, native_datetime, native_float, native_int, native_range, native_symbol,
};
pub use color::{
    native_cmyk, native_hsl, native_hsv, native_linear_rgb, native_luma, native_oklab,
    native_oklch, native_rgb,
};
pub use datetime::datetime_type_field;
pub(crate) use float::{
    dispatch_float_method, float_type_field, is_float_instance_method,
};
pub use int::int_type_field;
pub(crate) use int::{dispatch_int_method, is_int_instance_method};
pub use path::{native_path, read_path_value, resolve_path_value};
pub use query::{
    native_here, native_locate, native_metadata, native_query, native_target,
};
pub use regex_constructor::native_regex;
pub use repr::native_repr;
pub use selector::native_selector;
pub use str::{native_str, native_str_from_unicode};
pub use ty::native_type;
