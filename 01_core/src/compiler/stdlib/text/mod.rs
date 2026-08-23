//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/text.md
//! @prompt-hash 6de851f6
//! @layer L1
//! @updated 2026-08-13
//!
//! Hub do módulo `text`: nativas globais que produzem, transformam ou decoram
//! texto. Extraído de `stdlib.rs` em 2026-04-23 conforme ADR-0037 e fatiado em
//! hub + 8 nós em 2026-08-13 conforme ADR-0109.
//!
//! `text.rs` era um agregado plano — as nativas são chamadas por nome a partir
//! de `stdlib/mod.rs`, sem dispatcher. Por isso este hub **não** tem tabela de
//! despacho nem lógica: é só fronteira de reexportação. A suite de testes vive
//! em `stdlib/mod.rs`, que partilha um harness único com o resto do stdlib.

mod case;
mod constructor;
mod deco;
mod lorem;
mod shift;
mod smallcaps;
mod smartquote;

pub use case::{native_lower, native_replace, native_upper};
pub use constructor::native_text;
pub use deco::{native_highlight, native_overline, native_strike, native_underline};
pub use lorem::native_lorem;
pub use shift::{native_subscript, native_superscript};
pub use smallcaps::native_smallcaps;
pub use smartquote::native_smartquote;
