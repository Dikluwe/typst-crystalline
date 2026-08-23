//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/bindings.md
//! @prompt-hash 5a882ab7
//! @layer L1
//! @updated 2026-08-12
//!
//! Bindings do eval: `#let`, desestruturação, acesso a lugares mutáveis,
//! despacho de métodos, métodos de instância sobre valores e acesso a campo.
//! Extraído de `eval.rs` no Passo 96.1 conforme ADR-0037 (coesão por domínio).
//! Assinaturas simplificadas no Passo 109 (ADR-0044) via `Engine<'_>`.
//!
//! Passo 1013: fatiado em hub + 5 nós conforme ADR-0109. Ao contrário de
//! `operators/mod.rs`, este hub NÃO tem tabela de despacho: `bindings.rs`
//! era um agregado plano, sem ponto de entrada único. O hub é a fronteira
//! de re-exportação que mantém `bindings::<fn>` válido em `eval/mod.rs`.

mod access;
mod binding;
mod field_access;
mod method_dispatch;
mod value_methods;

pub(super) use access::unknown_variable;
pub(super) use binding::{
    destructure_let, eval_assign, eval_destruct_assignment, eval_let,
};
pub(super) use field_access::{
    eval_content_method, eval_field_access, eval_value_field_access, field_callee_error,
};
pub(super) use method_dispatch::{is_mutating_method, try_eval_mutating_method};
pub(super) use value_methods::{
    eval_color_method, eval_counter_method_value, eval_element_where,
    eval_selector_or_and, eval_selector_within, eval_state_method,
    eval_version_method_value,
};
