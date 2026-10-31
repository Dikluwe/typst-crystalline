//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/eval/math/call_fallback.md
//! @prompt-hash afcfd96b
//! @layer L1
//! @updated 2026-09-30

use super::*;

/// Resolve funções globais, operadores e símbolos fora do catálogo nativo.
pub(super) fn eval_math_call_fallback(
    name: String,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
    call: crate::entities::ast::expr::FuncCall<'_>,
) -> SourceResult<Content> {
    // **P510** — chamada a funções do scope global em modo math
    // (ex.: `bb(x)`, `bold(x + y)`). Antes do fallback P302/P303,
    // tentar resolver o nome no scope global; se for uma `Func`,
    // avaliar os args como conteúdo math e aplicar.
    // P1292-B — `underline` has two distinct public identities:
    // the global binding is the textual decoration, whereas a
    // bare call in math syntax is sugar for `math.underline`.
    // Resolve only this collision through the math module; code
    // mode continues to see the global textual function.
    let maybe_func = if name == "class" {
        Some(Value::Func(crate::entities::func::Func::native(
            "class",
            crate::compiler::stdlib::native_math_class_for_metadata,
        )))
    } else if name == "underline" {
        match scopes.get("math") {
            Some(Value::Module(module)) => module.scope().get("underline").cloned(),
            _ => None,
        }
    } else {
        scopes.get(&name).cloned()
    };
    let maybe_func = if let Some(Value::Symbol(symbol)) = &maybe_func {
        crate::compiler::stdlib::symbol_accent_func(symbol.value.as_str(), call.span())
            .map(Value::Func)
            .or(maybe_func)
    } else {
        maybe_func
    };
    if let Some(Value::Func(func)) = maybe_func {
        // **P899** — reaproveita `eval_math_arg_value` (já usado
        // pelo caminho de callee namespaced P772y, ex.:
        // `math.class(...)`), em vez de forçar todos os
        // argumentos por `eval_math_expr` + `Value::Content`.
        // Sem isto, um literal string (`bb("R")`) chegava a
        // `native_bb` já embrulhado em `Content::Text` (prosa,
        // via `value_to_display_content` — braço genérico de
        // `eval_math_expr`), nunca como `Value::Str` — o corpo
        // ficava fora do alcance de `apply_math_style`
        // (só cobre `MathIdent`/`MathText`/`MathSequence`/
        // `MathMatrix`), sintoma: `$ bb("R") $` compilava mas
        // sem estilo nenhum (`typst-passo-899-relatorio.md`,
        // achado da revisão pós Fase A — o teste directo de
        // `wrap_math_style` não cobria este caminho de
        // despacho). Unifica a semântica dos dois caminhos de
        // chamada em modo math (bare global vs namespaced).
        let mut args = eval_p1293_math_args(scopes, ctx, engine, call.args())?;
        super::call_dispatch::transport_native_call_span(&func, &mut args, call.span());
        return match apply_func(func, args, scopes, ctx, engine)? {
            Value::Content(c) => Ok(c),
            other => Err(vec![SourceDiagnostic::error(
                call.span(),
                format!(
                    "{}() em modo math deve devolver content, recebeu {}",
                    name,
                    other.type_name()
                ),
            )]),
        };
    }

    let pos_args: Vec<Expr<'_>> = call
        .args()
        .items()
        .filter_map(|a| match a {
            Arg::Pos(e) => Some(e),
            _ => None,
        })
        .collect();
    let (base, accent_func) = if let Some(op) = lookup_math_op(scopes, &name) {
        (op, None)
    } else if let Some(sym) = crate::compiler::math::symbols::ident_to_unicode(&name) {
        // **P958** — símbolo com args (`Gamma(z)` → Γ(𝑧)):
        // a cadeia de símbolos do braço standalone
        // (`Expr::MathIdent`) espelhada aqui — antes, o
        // fallback saltava de `lookup_math_op` directo para
        // o literal, e `Gamma(z)` saía `Gamma(𝑧)`. Ver
        // `compiler/eval.md` §P958.
        let func = crate::compiler::stdlib::symbol_accent_func(sym, call.span());
        (Content::Symbol(sym.into()), func)
    } else if let Some(sym) = crate::compiler::stdlib::sym::sym_lookup(&name) {
        // **P958** — idem via módulo `sym` (símbolos bare de
        // P795), com warning de depreciação verbatim (P820).
        if let Some(msg) = crate::compiler::stdlib::sym::sym_deprecation(&name) {
            engine.sink.warn_note(call.callee().span(), msg, "");
        }
        let func =
            crate::compiler::stdlib::symbol_accent_func(sym.value.as_str(), call.span());
        (Content::Symbol(sym.value), func)
    } else {
        return Err(vec![unknown_variable_math(
            call.callee().span(),
            &name,
            scopes.has_global(&name),
        )]);
    };
    if let Some(func) = accent_func {
        let mut args = eval_p1293_math_args(scopes, ctx, engine, call.args())?;
        super::call_dispatch::transport_native_call_span(&func, &mut args, call.span());
        return match apply_func(func, args, scopes, ctx, engine)? {
            Value::Content(content) => Ok(content),
            other => Err(vec![SourceDiagnostic::error(
                call.span(),
                format!(
                    "{}() em modo math deve devolver content, recebeu {}",
                    name,
                    other.type_name()
                ),
            )]),
        };
    }
    if pos_args.is_empty() {
        // `sin()` / `undef()` — args vazios; só base sem wrapper.
        return Ok(base);
    }
    let body = if pos_args.len() == 1 {
        eval_math_expr(scopes, ctx, engine, pos_args[0])?
    } else {
        // Múltiplos args: separados por `, ` (paridade vanilla).
        let mut items: Vec<Content> = Vec::new();
        for (i, expr) in pos_args.iter().enumerate() {
            if i > 0 {
                items.push(Content::MathText(",".into()));
            }
            items.push(eval_math_expr(scopes, ctx, engine, *expr)?);
        }
        Content::MathSequence(std::sync::Arc::from(items))
    };
    let delimited = Content::math_delimited('(', body, ')');
    Ok(Content::MathSequence(std::sync::Arc::from(vec![base, delimited])))
}

#[cfg(test)]
mod readiness_math_tests {
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
                source: Source::new(id(), text.into())
                    .expect("valid Source test fixture"),
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

    fn class_count(text: &str) -> usize {
        let module = evaluate(text).expect("valid source");
        let mut count = 0;
        module.content().expect("content").visit_content_static(&mut |node| {
            if matches!(node, crate::entities::content::Content::MathClassOverride(_)) {
                count += 1;
            }
            true
        });
        count
    }
    #[test]
    fn readiness_math_class_bare_reaches_existing_native() {
        assert_eq!(class_count(r#"$class("relation",x)$"#), 1);
    }
    #[test]
    fn readiness_math_class_builtin_ignores_user_math_binding() {
        assert_eq!(
            class_count(
                r#"#let math=5
$class("relation",x)$"#
            ),
            1
        );
    }
    #[test]
    fn readiness_math_class_lexical_and_alias_controls() {
        assert_eq!(
            class_count(
                r#"#let klass=math.class
$klass("relation",x)$"#
            ),
            1
        );
        assert_eq!(
            class_count(
                r#"#let class(kind,body)=body
$class("relation",x)$"#
            ),
            0
        );
        for text in ["$f(x)$", "$Gamma(x)$", "$sin(x)$"] {
            evaluate(text).expect("resolved math head");
        }
    }
    #[test]
    fn readiness_math_unknown_call_head_precedes_all_arguments() {
        for source in [
            "$unknown()$",
            "$unknown(nothere)$",
            "$unknown(foo:0)$",
            "$unknown(..nothere)$",
        ] {
            let error = evaluate(source).expect_err("unresolved head");
            assert_eq!(error[0].message, "unknown variable `unknown`", "{source}");
            assert!(!error[0].span.is_detached(), "actual callee origin");
        }
    }
}
