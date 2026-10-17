//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/math/arguments.md
//! @prompt-hash e16d971a
//! @layer L1
//! @updated 2026-10-06

use super::*;

pub(super) fn extract_math_named<'a>(
    node: &'a crate::entities::syntax_node::SyntaxNode,
) -> Option<crate::entities::ast::expr::Named<'a>> {
    if node.kind() == crate::entities::syntax_kind::SyntaxKind::Named {
        return crate::entities::ast::AstNode::from_untyped(node);
    }
    None
}

/// **P772y** — avalia um argumento posicional/nomeado de uma `FuncCall`
/// namespaced em modo math (ex.: `math.class("relation", body)`). Um
/// literal string avalia directamente para `Value::Str` (paridade com
/// chamadas de código normal — `class` em `math.class` é uma string, não
/// content); literais escalares (int/float/bool/numeric) avaliam em modo
/// código (P825 — paridade vanilla: `#class(3, x)` reporta
/// `found integer`); qualquer outro expr passa por `eval_math_expr` e
/// embrulha-se em `Value::Content`, tal como o mecanismo P510 já fazia
/// para `bb(x)`/`bold(x + y)`.
pub(super) fn eval_math_arg_value(
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
    expr: Expr<'_>,
) -> SourceResult<Value> {
    match expr {
        Expr::Str(s) => Ok(Value::Str(EcoString::from(s.get()?))),
        // The math parser keeps an argument expression in a synthetic `Math`
        // wrapper. Peel that wrapper only when its sole child is executable;
        // every ordinary math body keeps the existing Content path.
        Expr::Math(math) => {
            if math.exprs().any(|expr| matches!(expr, Expr::MathAlignPoint(_))) {
                return Ok(Value::Content(eval_math_content(scopes, ctx, engine, math)?));
            }
            let mut children = math.exprs();
            let first = children.next();
            if matches!(first, Some(Expr::MathIdent(_))) && children.next().is_none() {
                return eval_math_arg_value(scopes, ctx, engine, first.unwrap());
            }
            let text = math.to_untyped().clone().into_text();
            let reparsed = crate::compiler::parse::parse_anchored(
                text.as_str(),
                crate::entities::syntax_mode::SyntaxMode::Code,
                math.span(),
            );
            let mut code_exprs = reparsed.children().filter_map(Expr::from_untyped);
            let first = code_exprs.next();
            let second = code_exprs.next();
            drop(code_exprs);
            match (first, second) {
                (
                    Some(
                        inner @ (Expr::Str(_)
                        | Expr::Int(_)
                        | Expr::Float(_)
                        | Expr::Bool(_)
                        | Expr::Numeric(_)
                        | Expr::None(_)
                        | Expr::Auto(_)
                        | Expr::Parenthesized(_)
                        | Expr::CodeBlock(_)
                        | Expr::Array(_)
                        | Expr::Dict(_)
                        | Expr::Closure(_)
                        | Expr::Contextual(_)),
                    ),
                    None,
                ) => eval_expr(inner, scopes, ctx, engine),
                _ => Ok(Value::Content(eval_math_content(scopes, ctx, engine, math)?)),
            }
        }
        // **P825 (sub-A)** — literais escalares avaliam em modo código
        // (paridade vanilla: após `#`, os args são código — `class(3, x)`
        // reporta `found integer`, não `found content`). Só afecta
        // chamadas namespaced via `#` — chamadas bare de módulos são
        // rejeitadas antes (sub-B).
        Expr::Int(_)
        | Expr::Float(_)
        | Expr::Bool(_)
        | Expr::Numeric(_)
        | Expr::None(_)
        | Expr::Auto(_)
        | Expr::Parenthesized(_)
        | Expr::CodeBlock(_)
        | Expr::Array(_)
        | Expr::Dict(_)
        | Expr::Closure(_)
        | Expr::Contextual(_) => eval_expr(expr, scopes, ctx, engine),
        other => {
            // Lexical math arguments retain their actual Value until the
            // constructor cast. Only Code Ident may consult global values;
            // an unresolved MathIdent keeps the mathematical namespace/hints.
            if let Expr::MathIdent(ident) = &other {
                if let Some(value) = scopes.get_local(ident.as_str()) {
                    return Ok(value.clone());
                }
            }
            let ident_name = match &other {
                Expr::Ident(i) => Some(i.as_str()),
                _ => None,
            };
            if let Some(name) = ident_name {
                let desvia = scopes.get_local(name).is_none()
                    && matches!(scopes.get(name),
                        Some(v) if !matches!(v,
                            Value::Content(_) | Value::Func(_) | Value::Module(_)));
                if desvia {
                    return eval_math_callee(scopes, ctx, engine, other);
                }
            }
            Ok(Value::Content(eval_math_expr(scopes, ctx, engine, other)?))
        }
    }
}

/// Avalia uma lista de argumentos math uma vez e preserva a forma de chamada
/// para os constructors canônicos P1293 de `attach` e `binom`.
pub(super) fn eval_p1293_math_args(
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
    call_args: AstArgs<'_>,
) -> SourceResult<Args> {
    let mut occurrences = Vec::new();
    for arg in call_args.items() {
        match arg {
            Arg::Pos(expr) => occurrences.push(ArgOccurrence {
                name: None,
                value: eval_math_arg_value(scopes, ctx, engine, expr)?,
                span: expr.span(),
                value_span: expr.span(),
            }),
            Arg::Named(named_arg) => {
                let value = eval_math_arg_value(scopes, ctx, engine, named_arg.expr())?;
                occurrences.push(ArgOccurrence {
                    name: Some(named_arg.name().as_str().into()),
                    value,
                    span: named_arg.span(),
                    value_span: named_arg.expr().span(),
                });
            }
            Arg::Spread(spread) => {
                let value = eval_math_arg_value(scopes, ctx, engine, spread.expr())?;
                match value {
                    Value::None => {}
                    Value::Array(values) => {
                        occurrences.extend(values.into_iter().map(|value| {
                            ArgOccurrence {
                                name: None,
                                value,
                                span: spread.span(),
                                value_span: spread.span(),
                            }
                        }));
                    }
                    Value::Dict(values) => {
                        occurrences.extend(values.into_iter().map(|(name, value)| {
                            ArgOccurrence {
                                name: Some(name),
                                value,
                                span: spread.span(),
                                value_span: spread.span(),
                            }
                        }));
                    }
                    Value::Args(args) => occurrences.extend(args.occurrence_sequence()),
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            spread.span(),
                            format!("cannot spread {}", other.type_name()),
                        )]);
                    }
                }
            }
        }
    }
    Ok(Args::from_occurrences(call_args.span(), occurrences))
}

/// Variante de P1431: parênteses posicionais em `mat((a, b), (c, d))`
/// representam linhas. A decomposição acontece ainda no AST, antes de o
/// conteúdo matemático esconder as fronteiras entre células.
pub(super) fn eval_p1431_mat_args(
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
    call_args: AstArgs<'_>,
) -> SourceResult<Args> {
    let mut occurrences = Vec::new();
    let row_mode = call_args.items().any(|arg| matches!(arg, Arg::Pos(Expr::Array(_))));
    let mut trailing_row = Vec::new();
    for arg in call_args.items() {
        match arg {
            Arg::Pos(Expr::Array(array)) => {
                if !trailing_row.is_empty() {
                    occurrences.push(ArgOccurrence {
                        name: None,
                        value: Value::Array(std::mem::take(&mut trailing_row)),
                        span: array.span(),
                        value_span: array.span(),
                    });
                }
                let mut row = Vec::new();
                for item in array.items() {
                    let ArrayItem::Pos(expr) = item else { continue };
                    if let Some(named) = extract_math_named(expr.to_untyped()) {
                        occurrences.push(ArgOccurrence {
                            name: Some(named.name().as_str().into()),
                            value: eval_math_arg_value(
                                scopes,
                                ctx,
                                engine,
                                named.expr(),
                            )?,
                            span: named.span(),
                            value_span: named.expr().span(),
                        });
                    } else {
                        row.push(Value::Content(eval_math_expr(
                            scopes, ctx, engine, expr,
                        )?));
                    }
                }
                if !row.is_empty() {
                    occurrences.push(ArgOccurrence {
                        name: None,
                        value: Value::Array(row),
                        span: array.span(),
                        value_span: array.span(),
                    });
                }
            }
            Arg::Pos(Expr::Math(math)) if math.was_deparenthesized() => {
                let values = math
                    .exprs()
                    .map(|expr| eval_math_arg_value(scopes, ctx, engine, expr))
                    .collect::<SourceResult<Vec<_>>>()?;
                occurrences.push(ArgOccurrence {
                    name: None,
                    value: Value::Array(values),
                    span: math.span(),
                    value_span: math.span(),
                });
            }
            Arg::Pos(expr) => {
                if row_mode {
                    if let Some(named) = extract_math_named(expr.to_untyped()) {
                        occurrences.push(ArgOccurrence {
                            name: Some(named.name().as_str().into()),
                            value: eval_math_arg_value(
                                scopes,
                                ctx,
                                engine,
                                named.expr(),
                            )?,
                            span: named.span(),
                            value_span: named.expr().span(),
                        });
                    } else {
                        trailing_row.push(Value::Content(eval_math_expr(
                            scopes, ctx, engine, expr,
                        )?));
                    }
                } else {
                    occurrences.push(ArgOccurrence {
                        name: None,
                        value: eval_math_arg_value(scopes, ctx, engine, expr)?,
                        span: expr.span(),
                        value_span: expr.span(),
                    });
                }
            }
            Arg::Named(named) => occurrences.push(ArgOccurrence {
                name: Some(named.name().as_str().into()),
                value: eval_math_arg_value(scopes, ctx, engine, named.expr())?,
                span: named.span(),
                value_span: named.expr().span(),
            }),
            Arg::Spread(spread) => {
                let value = eval_math_arg_value(scopes, ctx, engine, spread.expr())?;
                match value {
                    Value::None => {}
                    Value::Array(values) => {
                        occurrences.extend(values.into_iter().map(|value| {
                            ArgOccurrence {
                                name: None,
                                value,
                                span: spread.span(),
                                value_span: spread.span(),
                            }
                        }));
                    }
                    Value::Dict(values) => {
                        occurrences.extend(values.into_iter().map(|(name, value)| {
                            ArgOccurrence {
                                name: Some(name),
                                value,
                                span: spread.span(),
                                value_span: spread.span(),
                            }
                        }));
                    }
                    Value::Args(args) => occurrences.extend(args.occurrence_sequence()),
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            spread.span(),
                            format!("cannot spread {}", other.type_name()),
                        )]);
                    }
                }
            }
        }
    }
    if !trailing_row.is_empty() {
        occurrences.push(ArgOccurrence {
            name: None,
            value: Value::Array(trailing_row),
            span: call_args.span(),
            value_span: call_args.span(),
        });
    }
    Ok(Args::from_occurrences(call_args.span(), occurrences))
}

#[cfg(test)]
mod residue_tests {
    use crate::compiler::eval::tests::eval_for_test;
    use crate::contracts::world::World;
    use crate::entities::font_book::FontBook;
    use crate::entities::source_result::SourceResult;
    use crate::entities::world_types::{
        Bytes, Datetime, FileError, FileResult, Font, Library,
    };
    use crate::entities::{file_id::FileId, module::Module, source::Source};
    use std::num::NonZeroU16;
    struct TestWorld {
        library: Library,
        book: FontBook,
        source: Source,
    }
    impl TestWorld {
        fn new(text: &str) -> Self {
            Self {
                library: Library::new(),
                book: FontBook::new(),
                source: Source::new(id(), text.into()),
            }
        }
    }
    impl World for TestWorld {
        fn library(&self) -> &Library {
            &self.library
        }
        fn book(&self) -> &FontBook {
            &self.book
        }
        fn main(&self) -> FileId {
            self.source.id()
        }
        fn source(&self, _: FileId) -> FileResult<Source> {
            Ok(self.source.clone())
        }
        fn file(&self, _: FileId) -> FileResult<Bytes> {
            Err(FileError::NotFound)
        }
        fn font(&self, _: usize) -> Option<Font> {
            None
        }
        fn today(
            &self,
            _: Option<crate::entities::duration::Duration>,
        ) -> Option<Datetime> {
            None
        }
    }
    fn id() -> FileId {
        FileId::from_raw(NonZeroU16::new(1431).unwrap())
    }
    fn evaluate(text: &str) -> SourceResult<Module> {
        let world = TestWorld::new(text);
        eval_for_test(&world, &world.source)
    }

    #[test]
    fn math_argument_local_bindings_keep_value_type() {
        assert!(evaluate(
            "#let aa=left
$mat(align:aa,a)$"
        )
        .is_ok());
        assert!(evaluate(
            "#let aa=none
$mat(delim:aa,a)$"
        )
        .is_ok());
        let errors = evaluate(
            "#let aa=auto
$mat(delim:aa,a)$",
        )
        .unwrap_err();
        assert_eq!(
            errors[0].message,
            "expected array, none, symbol, or string, found auto"
        );
    }

    #[test]
    fn math_argument_bare_literals_and_globals_keep_math_resolution() {
        for (source, name) in
            [("$mat(delim:none,a)$", "none"), ("$mat(align:left,a)$", "left")]
        {
            let error = evaluate(source).expect_err("bare math identifier must fail");
            assert_eq!(error[0].message, format!("unknown variable `{name}`"));
            assert!(!error[0].hints.is_empty());
        }
        assert!(evaluate("$mat(delim:#none,align:#left,a)$").is_ok());
        assert!(evaluate(
            "#let left = [q]
$mat(left)$"
        )
        .is_ok());
    }
}
