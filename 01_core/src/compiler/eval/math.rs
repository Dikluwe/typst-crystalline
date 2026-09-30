//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/math.md
//! @prompt-hash 4791faf2
//! @layer L1
//! @updated 2026-09-01
//!
//! Avaliação de expressões matemáticas. Extraído de `eval.rs` no Passo 96.1
//! conforme ADR-0037 (coesão por domínio).

use ecow::EcoString;

use crate::compiler::scopes::Scopes;
use crate::entities::args::{ArgOccurrence, Args};
use crate::entities::ast::expr::{Arg, Args as AstArgs, ArrayItem, Expr};
use crate::entities::ast::math::{Math, MathTextKind};
use crate::entities::ast::AstNode;
use crate::entities::content::Content;
use crate::entities::elements::math_attach::MathAttachSlot;
use crate::entities::engine::Engine;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;

use super::{
    apply_func, bindings, call_dispatch, eval_expr, value_to_display_content, EvalContext,
};

mod arguments;
mod call_fallback;
mod callee;
mod construct;
mod delimiters;
mod entry;
mod expression;
mod expression_call;
mod lookup;
mod native_dispatch;

pub(super) use self::entry::eval_math_content;
use self::{
    arguments::*, callee::*, construct::*, delimiters::*, expression::*, lookup::*,
};

#[cfg(test)]
mod atomization_tests {
    #[test]
    fn delimiter_pair_keeps_floor_morphology() {
        assert_eq!(super::delimiters::parse_delim_str("floor"), ('⌊', '⌋'));
    }
}
