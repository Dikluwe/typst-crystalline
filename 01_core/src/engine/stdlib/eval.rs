//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/engine/stdlib/eval.md
//! @prompt-hash 619eb6f1
//! @layer L1
//! @updated 2026-07-22
//!
//! Função nativa `eval(source, mode:, scope:)` — P394; P814 (mode/scope,
//! mensagens vanilla, span sintético).

use std::sync::Arc;

use comemo::TrackedMut;

use crate::engine::eval::long_type_name;
use crate::engine::eval::{eval_expr, eval_markup, EvalContext};
use crate::engine::parse::parse_anchored;
use crate::engine::scopes::Scopes;
use crate::entities::args::Args;
use crate::entities::ast::expr::Expr;
use crate::entities::ast::AstNode;
use crate::entities::content::Content;
use crate::entities::engine::Engine;
use crate::entities::file_id::FileId;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::syntax_mode::SyntaxMode;
use crate::entities::syntax_node::SyntaxNode;
use crate::entities::value::Value;

/// `eval(source, mode:, scope:)` → re-parseia e re-avalia `source` como código
/// Typst no contexto actual.
///
/// - `source`: único argumento posicional, `Str`. Parseado no `mode` indicado
///   (`code`/`markup`/`math`; default `code`).
/// - `mode:` (named, P814): modo sintáctico do parse. O **default vanilla é
///   `SyntaxMode::Code`** (vanilla `foundations/mod.rs:279`,
///   `#[default(SyntaxMode::Code)]`) — confirmado por sonda em P810/P814. O
///   comentário que aqui existia afirmando que o default vanilla era
///   `"markup"` era **falso**.
/// - `scope:` (named, P814): dict de bindings extra, visíveis durante a
///   avaliação e confinados a ela (não vazam para o chamador).
/// - Erros de sintaxe dentro do string propagam com a **mensagem real do
///   parser** e o **span âncora** da chamada (P814 — `parse_anchored`,
///   equivalente ao `SpanMode::Uniform` do vanilla), não `<detached>`.
/// - `#set`/`#show` dentro do `eval` são confinados a uma engine local
///   (não afectam o chamador).
///
/// **Divergência registada (P814, achado para decisão de L0):** o cristalino
/// avalia no scope do chamador (L0 P394, §2 — `#let x = 5` antes do
/// `#eval("x * 2")` funciona). Medido no vanilla em P814: `eval_string`
/// cria um `Scopes` fresco (só stdlib + `scope:`), logo
/// `#let y = 10 \n #eval("y + 1")` → `error: unknown variable: y` no
/// vanilla e `11` no cristalino. Mantido o comportamento do L0 vigente;
/// registado no relatório de P814.
pub fn native_eval(
    ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
    scopes: &mut Scopes<'_>,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    // P814 — named args `mode:`/`scope:`, com as mensagens de cast do vanilla
    // (medidas por sonda; a mensagem é o observável — ADR-0107).
    let mut mode = SyntaxMode::Code;
    let mut extra_scope = None;
    for (name, value) in &args.named {
        match name.as_str() {
            "mode" => {
                mode = match value {
                    Value::Str(s) => match s.as_str() {
                        "markup" => SyntaxMode::Markup,
                        "math" => SyntaxMode::Math,
                        "code" => SyntaxMode::Code,
                        _ => {
                            return Err(vec![SourceDiagnostic::error(
                                args.span,
                                "expected \"markup\", \"math\", or \"code\"",
                            )])
                        }
                    },
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            args.span,
                            format!(
                                "expected \"markup\", \"math\", or \"code\", found {}",
                                long_type_name(other)
                            ),
                        )])
                    }
                };
            }
            "scope" => match value {
                Value::Dict(d) => extra_scope = Some(d),
                other => {
                    return Err(vec![SourceDiagnostic::error(
                        args.span,
                        format!("expected dictionary, found {}", long_type_name(other)),
                    )])
                }
            },
            other => {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!("unexpected argument: {other}"),
                )])
            }
        }
    }

    let source = match args.items.as_slice() {
        [] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "missing argument: source",
            )])
        }
        [Value::Str(s)] => s.clone(),
        [other] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("expected string, found {}", long_type_name(other)),
            )])
        }
        _ => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "unexpected argument",
            )])
        }
    };

    // P814 — span sintético: todos os nós (e erros) da árvore parseada ficam
    // ancorados ao span da chamada (`SpanMode::Uniform` do vanilla).
    let root = parse_anchored(source.as_str(), mode, args.span);

    // Erros de sintaxe no string avaliado propagam com a mensagem real do
    // parser (+ hints) e o span âncora — não erro genérico com `<detached>`.
    if root.erroneous() {
        let diags = root
            .errors()
            .into_iter()
            .map(|e| {
                let mut d = SourceDiagnostic::error(e.span, e.message.to_string());
                d.hints = e.hints.iter().map(|h| h.to_string()).collect();
                d
            })
            .collect();
        return Err(diags);
    }

    // Confinar #set/#show do eval a engine local (paridade CodeBlock) e os
    // bindings de `scope:` a um âmbito próprio (P814 — vanilla empurra um
    // `Scope` com os bindings do dict na pilha do VM do eval).
    let mut local_styles = engine.styles.clone();
    let mut local_show_rules = Arc::clone(engine.show_rules);
    let mut local_sink = TrackedMut::reborrow_mut(&mut *engine.sink);
    scopes.enter();
    if let Some(dict) = extra_scope {
        for (key, value) in dict {
            scopes.define(key.as_str(), value.clone());
        }
    }
    let result = {
        let mut local_engine = Engine {
            world: engine.world,
            route: engine.route,
            styles: &mut local_styles,
            show_rules: &mut local_show_rules,
            active_guards: &mut *engine.active_guards,
            current_file: engine.current_file,
            sink: &mut local_sink,
        };
        eval_synthetic_root(&root, mode, scopes, ctx, &mut local_engine)
    };
    scopes.exit();
    result
}

/// **P814** — avalia a raiz sintética de `eval()` conforme o modo.
///
/// - `code`: avalia cada expressão do bloco; devolve o valor da última.
/// - `markup`: avalia como markup → `Value::Content`.
/// - `math`: avalia como math e embrulha em `Content::Equation` com
///   `block: false` (paridade vanilla `EquationElem::new(..).with_block(false)`).
fn eval_synthetic_root(
    root: &SyntaxNode,
    mode: SyntaxMode,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    match mode {
        SyntaxMode::Code => {
            let mut last = Value::None;
            for child in root.children() {
                if let Some(expr) = Expr::from_untyped(child) {
                    last = eval_expr(expr, scopes, ctx, engine)?;
                }
            }
            Ok(last)
        }
        SyntaxMode::Markup => eval_markup(root, scopes, ctx, engine),
        SyntaxMode::Math => {
            let value = match Expr::from_untyped(root) {
                Some(expr) => eval_expr(expr, scopes, ctx, engine)?,
                None => Value::None,
            };
            match value {
                Value::Content(c) => Ok(Value::Content(Content::equation(c, false))),
                other => Ok(other),
            }
        }
    }
}
