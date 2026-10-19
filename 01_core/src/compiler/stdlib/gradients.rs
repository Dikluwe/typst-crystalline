//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/gradients.md
//! @prompt-hash f6e9f264
//! @layer L1
//! @updated 2026-06-24
//!
//! Stdlib `gradient.linear` — P262 (ADR-0087 Gradient Linear-only).
//!
//! Constrói `Value::Gradient(Gradient::Linear)` a partir de stops
//! variadic positional + named `angle: Angle`.
//!
//! Aceita formatos de stop:
//! - `Color` directo (offset = None; auto-spacing).
//! - `Array [Color, Ratio]` (offset explícito).
//!
//! ColorSpace fixo Oklab (scope-out ADR-0087 — paridade vanilla
//! default). Interpolação L1 via `Linear::sample(t)`.

use crate::compiler::eval::EvalContext;
use crate::entities::args::{ArgOccurrence, Args};
use crate::entities::axes::Axes;
use crate::entities::color::{Color, ColorSpace, ColorSpaceError};
use crate::entities::dir::Dir;
use crate::entities::file_id::FileId;
use crate::entities::func::Func;
use crate::entities::gradient::{Gradient, GradientStop};
use crate::entities::layout_types::{Angle, Ratio};
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;

mod accessors;
mod conic;
mod dispatch;
mod linear;
mod modifiers;
mod parsing;
mod radial;
mod sampling;

pub use self::conic::native_gradient_conic;
pub(crate) use self::dispatch::dispatch_gradient_method;
pub use self::dispatch::{gradient_type_field, is_gradient_instance_method};
pub use self::linear::native_gradient_linear;
pub(crate) use self::modifiers::{native_gradient_repeat, native_gradient_sharp};
pub use self::radial::native_gradient_radial;
pub(crate) use self::sampling::{native_gradient_sample, native_gradient_samples};
use self::{accessors::*, parsing::*, sampling::*};

#[cfg(test)]
mod atomization_tests {
    #[test]
    fn type_dispatch_keeps_constructors_and_rejects_unknown() {
        assert!(super::dispatch::gradient_type_field("linear").is_some());
        assert!(super::dispatch::gradient_type_field("conic").is_some());
        assert!(super::dispatch::gradient_type_field("unknown").is_none());
    }
}

// Tests para `native_gradient_linear` + `native_gradient_radial`
// + `native_gradient_conic` em `01_core/src/compiler/stdlib/mod.rs`
// tests module (usa `null_ctx!` + `null_world()` + `test_file_id()`).
