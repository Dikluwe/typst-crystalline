//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/bindings/value_methods.md
//! @prompt-hash be0fd2fa
//! @layer L1
//! @updated 2026-08-12
//!
//! Métodos de instância avaliados sobre valores, com argumentos ainda em AST:
//! `state`, `counter` (incl. `display`/`at`), `color`, `version`, e os
//! combinadores de `selector` (`where`, `and`/`or`, `within`).
//!
//! Extraído de `compiler/eval/bindings.rs` no Passo 1013 conforme ADR-0109
//! (atomização — forma B, free function no arquivo da unidade).

use ecow::EcoVec;

use crate::compiler::scopes::Scopes;
use crate::compiler::stdlib::counter::{
    counter_at_location_owned as counter_at_location, counter_at_owned as counter_at,
    counter_display, counter_final_owned as counter_final,
    counter_get_owned as counter_get, counter_step, counter_update,
};
use crate::compiler::stdlib::state::{
    state_at_location, state_display, state_final, state_get,
};
use crate::entities::args::{ArgOccurrence, Args};
use crate::entities::ast::expr::{Arg, Expr};
use crate::entities::ast::AstNode;
use crate::entities::counter::Counter;
use crate::entities::element_kind::ElementKind;
use crate::entities::engine::Engine;
use crate::entities::selector::Selector;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::state::State;
use crate::entities::value::Value;

use crate::compiler::eval::operators::error_formatting::vanilla_type_name;
use crate::compiler::eval::{eval_expr, EvalContext};

mod color;
mod counter;
mod element_where;
mod selector;
mod state;
mod version;

pub(in crate::compiler::eval) use color::eval_color_method;
pub(in crate::compiler::eval) use counter::{
    eval_counter_method_value, eval_counter_static_method_value,
};
pub(crate) use element_where::{
    eval_element_where, native_element_fields, native_function_where,
    validate_element_locatability,
};
pub(in crate::compiler::eval) use selector::{
    eval_selector_before_after, eval_selector_or_and, eval_selector_within,
    value_to_query_selector,
};
pub(in crate::compiler::eval) use state::eval_state_method;
pub(crate) use state::resolve_state_label;
pub(in crate::compiler::eval) use version::eval_version_method_value;
