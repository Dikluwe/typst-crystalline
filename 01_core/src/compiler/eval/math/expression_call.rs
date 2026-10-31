//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/eval/math/expression_call.md
//! @prompt-hash f8d029e5
//! @layer L1
//! @updated 2026-09-30

use super::*;

/// Avalia uma chamada em modo matemático na ordem lexical e nativa vigente.
pub(super) fn eval_math_func_call(
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
    call: crate::entities::ast::expr::FuncCall<'_>,
) -> SourceResult<Content> {
    let mut callee_root = call.callee();
    while let Expr::FieldAccess(access) = callee_root {
        callee_root = access.target();
    }
    if matches!(callee_root, Expr::Ident(_)) {
        let value = eval_expr(Expr::FuncCall(call), scopes, ctx, engine)?;
        return Ok(super::value_to_display_content(value).unwrap_or(Content::Empty));
    }

    // P1395 — um callee bare consulta primeiro o scope lexical. O
    // namespace math só participa quando nenhum binding local o
    // sombreia, inclusive para nomes interceptados abaixo como
    // `abs` e `sqrt`.
    if let Expr::MathIdent(ident) = call.callee() {
        let local = scopes.get_local(ident.get()).cloned();
        if let Some(value) = local {
            let value = if let Value::Symbol(symbol) = &value {
                crate::compiler::stdlib::symbol_accent_func(
                    symbol.value.as_str(),
                    call.span(),
                )
                .map(Value::Func)
                .unwrap_or(value)
            } else {
                value
            };
            if let Value::Func(func) = value {
                let mut args = eval_p1293_math_args(scopes, ctx, engine, call.args())?;
                super::call_dispatch::transport_native_call_span(
                    &func,
                    &mut args,
                    call.span(),
                );
                return match apply_func(func, args, scopes, ctx, engine)? {
                    Value::Content(content) => Ok(content),
                    other => Err(vec![SourceDiagnostic::error(
                        call.span(),
                        format!(
                            "{}() em modo math deve devolver content, recebeu {}",
                            ident.get(),
                            other.type_name()
                        ),
                    )]),
                };
            }

            let mut arguments = Vec::new();
            let mut diagnostics = Vec::new();
            for arg in call.args().items() {
                match arg {
                    Arg::Pos(expr) => {
                        if !arguments.is_empty() {
                            arguments.push(Content::MathText(", ".into()));
                        }
                        arguments.push(eval_math_expr(scopes, ctx, engine, expr)?);
                    }
                    Arg::Named(named) => {
                        let fixed = named
                            .to_untyped()
                            .clone()
                            .into_text()
                            .to_string()
                            .replacen(':', "\\:", 1);
                        diagnostics.push(
                            SourceDiagnostic::error(
                                named.span(),
                                "named-argument syntax can only be used with functions",
                            )
                            .with_hint(format!(
                                "to render the colon as text, escape it: `{fixed}`"
                            )),
                        );
                    }
                    Arg::Spread(spread) => {
                        let fixed = spread
                            .to_untyped()
                            .clone()
                            .into_text()
                            .to_string()
                            .replacen("..", ".. ", 1);
                        diagnostics.push(
                            SourceDiagnostic::error(
                                spread.span(),
                                "spread-argument syntax can only be used with functions",
                            )
                            .with_hint(format!(
                                "to render the dots as text, add a space: `{fixed}`"
                            )),
                        );
                    }
                }
            }
            if !diagnostics.is_empty() {
                return Err(diagnostics);
            }

            let base = super::value_to_display_content(value).unwrap_or(Content::Empty);
            let body = Content::MathSequence(std::sync::Arc::from(arguments));
            let parens = Content::math_delimited('(', body, ')');
            return Ok(Content::MathSequence(std::sync::Arc::from(vec![base, parens])));
        }
    }

    let name = match call.callee() {
        Expr::MathIdent(ident) => ident.get().to_string(),
        // **P899** — `dot.double(x)`: caso especial ANTES do
        // despacho namespaced genérico abaixo. Sem isto, o callee
        // `FieldAccess(MathIdent("dot"), "double")` cairia no braço
        // `other_callee`, que chama `eval_math_callee` → tenta
        // resolver "dot" como `Value::Symbol` e aplicar o
        // MODIFICADOR "double" (`Symbol::modified`) — falha com
        // "unknown symbol modifier 'double'" porque `dot` é uma
        // entrada `SYM_SIMPLE` plana (sem tabela de variantes,
        // achado de P895 para o mesmo tipo de confusão
        // símbolo-modificado vs função). `dot.double(x)` é
        // despacho de FUNÇÃO de acento (duplo ponto, U+0308), não
        // um símbolo modificado — trata-se aqui como `name =
        // "dot.double"`, caindo no mesmo `match` hardcoded abaixo
        // que `hat`/`tilde`/`dot`.
        Expr::FieldAccess(access)
            if matches!(
                (access.target(), access.field().as_str()),
                (Expr::MathIdent(t), "double") if t.get() == "dot"
            ) =>
        {
            "dot.double".to_string()
        }
        // P772y — callee namespaced (`math.class(...)`, `calc.foo(...)`):
        // fora do despacho nativo bare-ident abaixo. Resolve via
        // avaliador geral de expressões; se for `Value::Func`,
        // aplica com args avaliados em modo math — mesmo mecanismo
        // do fallback P510 (scope global) mais abaixo, generalizado
        // para callees com field access.
        other_callee => {
            // **P829-C** — callee `target.field(...)` em modo math
            // passa pelo mesmo despacho de erro de P815
            // (`field_callee_error`): o vanilla usa a MESMA rotina de
            // chamada dentro e fora de math (`call.rs:
            // eval_field_callee` — medido c1–c4: `$#d.x()$` produz o
            // erro dict-key + hints verbatim). O target é avaliado
            // uma única vez, tal como `eval_math_callee` faria no
            // braço `FieldAccess`; para alvos
            // Symbol/Func/Type/Module o fallback devolve `None` e o
            // campo resolve normalmente (ex.: `math.class`).
            let callee_value = if let Expr::FieldAccess(access) = other_callee {
                let target = eval_math_callee(scopes, ctx, engine, access.target())?;
                if let Some(err) = super::bindings::field_callee_error(&target, access) {
                    return Err(err);
                }
                super::bindings::eval_value_field_access(
                    target,
                    access.field().as_str(),
                    access.span(),
                )?
            } else {
                eval_math_callee(scopes, ctx, engine, other_callee)?
            };
            let callee_value = if let Value::Symbol(symbol) = &callee_value {
                crate::compiler::stdlib::symbol_accent_func(
                    symbol.value.as_str(),
                    call.span(),
                )
                .map(Value::Func)
                .unwrap_or(callee_value)
            } else {
                callee_value
            };
            let Value::Func(func) = callee_value else {
                return Err(vec![SourceDiagnostic::error(
                    call.span(),
                    format!(
                        "chamada em modo math espera função, recebeu {}",
                        callee_value.type_name()
                    ),
                )]);
            };
            let mut args = eval_p1293_math_args(scopes, ctx, engine, call.args())?;
            super::call_dispatch::transport_native_call_span(
                &func,
                &mut args,
                call.span(),
            );
            return match apply_func(func, args, scopes, ctx, engine)? {
                Value::Content(c) => Ok(c),
                other => Err(vec![SourceDiagnostic::error(
                    call.span(),
                    format!(
                        "chamada em modo math deve devolver content, recebeu {}",
                        other.type_name()
                    ),
                )]),
            };
        }
    };
    super::native_dispatch::dispatch_math_native(name, scopes, ctx, engine, call)
}

#[cfg(test)]
mod readiness_math_code_tests {
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

    #[test]
    fn readiness_math_code_box_expressions_keep_values() {
        for (source, expected) in [
            ("#let index=2\n$arrow(#box(width:0.4em+0.3em*index,height:0.4em))$", 1.0),
            ("$arrow(#box(width:calc.max(1em,2em),height:0.4em))$", 2.0),
        ] {
            let module = evaluate(source).expect("embedded Code arguments");
            let mut widths = Vec::new();
            module.content().unwrap().visit_content_static(&mut |node| {
                if let crate::entities::content::Content::Boxed(elem) = node {
                    let width = elem.width.expect("explicit width");
                    assert_eq!(width.abs.to_pt(), 0.0);
                    widths.push(width.em);
                }
                true
            });
            assert_eq!(widths, vec![expected]);
        }
    }
    #[test]
    fn readiness_math_code_returns_are_displayed() {
        let module = evaluate("$frac(#str(12),2)$").expect("Code string return");
        let mut numerators = Vec::new();
        module.content().unwrap().visit_content_static(&mut |node| {
            if let crate::entities::content::Content::MathFrac(elem) = node {
                numerators.push(elem.num.plain_text());
            }
            true
        });
        assert_eq!(numerators, vec!["12"]);
        assert_eq!(
            evaluate("$#calc.max(1,2)$")
                .expect("Code integer return")
                .content()
                .unwrap()
                .plain_text()
                .trim(),
            "2"
        );
    }
    #[test]
    fn readiness_math_code_namespace_and_math_controls() {
        for source in [
            "$frac(1,2)$",
            "#let maker(width)=box(width:width,height:1em)\n$arrow(#maker(1em))$",
            "#let inspect(value)=[#type(value)]\n$inspect(\"x\")$",
        ] {
            evaluate(source).expect(source);
        }
        for source in [
            "$#math.class(\"relation\",x)$",
            "$#sqrt(2)$",
            "$#math.sqrt(4)$",
            "$math.sqrt(4)$",
            "$calc.max(1,2)$",
        ] {
            assert!(evaluate(source).is_err(), "{source}");
        }
    }

    fn accent_nodes(
        module: &Module,
    ) -> Vec<crate::entities::elements::math_accent::MathAccentElem> {
        let mut nodes = Vec::new();
        module.content().unwrap().visit_content_static(&mut |node| {
            if let crate::entities::content::Content::MathAccent(accent) = node {
                nodes.push(accent.as_ref().clone());
            }
            true
        });
        nodes
    }

    #[test]
    fn readiness_math_code_accent_symbol_heads() {
        for (name, expected) in [
            ("grave", '\u{0300}'),
            ("arrow", '\u{20d7}'),
            ("arrow.l", '\u{20d6}'),
            ("arrow.l.r", '\u{20e1}'),
            ("dot.double", '\u{0308}'),
            ("dot.triple", '\u{20db}'),
            ("dot.quad", '\u{20dc}'),
            ("acute.double", '\u{030b}'),
            ("harpoon.lt", '\u{20d0}'),
        ] {
            let module = evaluate(&format!("${name}(x)$")).expect(name);
            let nodes = accent_nodes(&module);
            assert_eq!(nodes.len(), 1, "{name}");
            assert_eq!(nodes[0].accent, expected, "{name}");
            assert_eq!(nodes[0].base.plain_text(), "x", "{name}");
        }
    }

    #[test]
    fn readiness_math_code_accent_local_and_code() {
        for source in [
            "#let markaccent=sym.arrow.l\n$markaccent(x)$",
            "#sym.arrow.l[x]",
            "#let markaccent=sym.arrow.l\n#markaccent[x]",
            "$#sym.arrow.l[x]$",
        ] {
            let module = evaluate(source).expect(source);
            let nodes = accent_nodes(&module);
            assert_eq!(nodes.len(), 1, "{source}");
            assert_eq!(nodes[0].accent, '\u{20d6}', "{source}");
            assert_eq!(nodes[0].base.plain_text(), "x", "{source}");
        }
    }

    #[test]
    fn readiness_math_code_accent_options() {
        for source in [
            "#let markaccent=sym.arrow.l\n$markaccent(x,size:#150%,dotless:#false)$",
            "#sym.arrow.l([x],size:150%,dotless:false)",
        ] {
            let module = evaluate(source).expect(source);
            let nodes = accent_nodes(&module);
            assert_eq!(nodes.len(), 1);
            assert_eq!(nodes[0].size.rel, 1.5);
            assert_eq!(nodes[0].size.abs, crate::entities::layout_types::Length::ZERO);
            assert!(!nodes[0].dotless);
            assert!(nodes[0].explicit.size && nodes[0].explicit.dotless);
        }
    }

    #[test]
    fn readiness_math_code_accent_arity() {
        for (tail, expected) in [
            ("$markaccent()$", "missing argument: base"),
            ("$markaccent(x,y)$", "unexpected argument"),
            ("$markaccent(x,size:#false)$", "expected relative length, found boolean"),
        ] {
            let errors = evaluate(&format!("#let markaccent=sym.arrow.l\n{tail}"))
                .expect_err(expected);
            assert!(errors.iter().any(|e| e.message == expected), "{errors:?}");
        }
    }

    #[test]
    fn readiness_math_code_accent_effects_once() {
        use crate::entities::value::Value;
        for source in [
            "#let seen=()\n$arrow(#{seen.push(1);[x]})$\n#repr(seen)",
            "#let seen=()\n#let markaccent=sym.arrow.l\n$markaccent(#{seen.push(1);[x]})$\n#repr(seen)",
            "#let seen=()\n#let markaccent=sym.arrow.l\n#markaccent({seen.push(1);[x]})\n#repr(seen)",
        ] {
            let module = evaluate(source).expect(source);
            assert_eq!(module.scope().get("seen"), Some(&Value::Array(vec![Value::Int(1)])));
            assert_eq!(accent_nodes(&module).len(), 1, "{source}");
        }
    }

    #[test]
    fn readiness_math_code_accent_noncallable_controls() {
        for source in [
            "#let markaccent=sym.beta\n$markaccent(x)$",
            "#let markaccent=\"arrow\"\n$markaccent(x)$",
            "#let a=sym.arrow.l\n$a(x)$",
        ] {
            let module = evaluate(source).expect(source);
            assert!(accent_nodes(&module).is_empty(), "{source}");
        }
        let errors = evaluate("#sym.beta(panic(\"ARG_EFFECT\"))").unwrap_err();
        assert!(errors.iter().any(|e| e.message == "expected function, found symbol"));
        assert!(!errors.iter().any(|e| e.message.contains("ARG_EFFECT")));
    }
}
