//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/engine/stdlib/eval.md
//! @prompt-hash bcd9b3aa
//! @layer L1
//! @updated 2026-06-22
//!
//! Função nativa `eval(source)` — P394.

use std::sync::Arc;

use comemo::TrackedMut;

use crate::entities::args::Args;
use crate::entities::ast::expr::Expr;
use crate::entities::ast::AstNode;
use crate::entities::engine::Engine;
use crate::entities::file_id::FileId;
use crate::entities::source::Source;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;
use crate::engine::eval::{eval_expr, EvalContext};
use crate::engine::parse::parse_code;
use crate::engine::scopes::Scopes;

/// `eval(source)` → re-parseia e re-avalia `source` como código Typst no
/// contexto actual.
///
/// Recebe um único argumento posicional `Str`. O string é parseado em modo
/// código (`parse_code`), e cada expressão do bloco é avaliada no scope/engine
/// actuais. O valor da última expressão é devolvido. `#set`/`#show` dentro do
/// `eval` são confinados a uma engine local (não afectam o chamador).
///
/// **Divergência vs vanilla:** vanilla `eval` default mode é `"markup"` e
/// requer `mode: "code"` para este comportamento. Cristalino P394 adopta
/// modo código por simplificação — `mode:` fica scope-out.
pub fn native_eval(
    ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
    scopes: &mut Scopes<'_>,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    super::expect_no_named(&args.named)?;

    match args.items.as_slice() {
        [Value::Str(source)] => {
            let source = Source::detached_with_parser(source.as_str(), parse_code);
            let root = source.root();

            // Erros de sintaxe no string avaliado propagam como erro semântico.
            if root.erroneous() {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    "erro de sintaxe em eval()".to_string(),
                )]);
            }

            // Confinar #set/#show do eval a engine local (paridade CodeBlock).
            let mut local_styles = engine.styles.clone();
            let mut local_show_rules = Arc::clone(engine.show_rules);
            let mut local_sink = TrackedMut::reborrow_mut(&mut *engine.sink);
            let mut last = Value::None;
            {
                let mut local_engine = Engine {
                    world: engine.world,
                    route: engine.route,
                    styles: &mut local_styles,
                    show_rules: &mut local_show_rules,
                    active_guards: &mut *engine.active_guards,
                    current_file: engine.current_file,
                    sink: &mut local_sink,
                };
                for child in root.children() {
                    if let Some(expr) = Expr::from_untyped(child) {
                        last = eval_expr(expr, scopes, ctx, &mut local_engine)?;
                    }
                }
            }
            Ok(last)
        }
        [other] => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("eval() espera string, recebeu {}", other.type_name()),
        )]),
        _ => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "eval() requer 1 argumento (source)".to_string(),
        )]),
    }
}
