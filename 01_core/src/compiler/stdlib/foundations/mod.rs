//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/foundations.md
//! @prompt-hash bd454e29
//! @layer L1
//! @updated 2026-08-18
//!
//! Hub do módulo `foundations`: reexportação dos nós.
//! Fatiado de `foundations.rs` monolítico no Passo 1032.
//! P1077: removido o nó `len` (extensão global removida para paridade vanilla).

pub mod cast;
pub mod color;
pub mod query;
pub mod repr;
pub mod str;
pub mod ty;

pub use cast::{
    native_bytes, native_datetime, native_float, native_int, native_range, native_symbol,
};
pub use color::{
    native_cmyk, native_hsl, native_hsv, native_linear_rgb, native_luma, native_oklab,
    native_oklch, native_rgb,
};
pub use query::{
    native_here, native_locate, native_metadata, native_query, native_selector,
    native_target,
};
pub use repr::native_repr;
pub use str::{native_regex, native_str, native_str_from_unicode};
pub use ty::native_type;
