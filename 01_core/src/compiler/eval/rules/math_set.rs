//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/eval/rules/math_set.md
//! @prompt-hash 045196dc
//! @layer L1
//! @updated 2026-10-31
//!
//! Comportamento de `eval/rules::math_set`.

use super::*;

/// Helper partilhado para construir warning de propriedade não suportada
/// em `#set` (Passo 107, encerra DEBT-49). Formato consistente para o
/// utilizador final.
///
/// Passo 134: parâmetro `adr_ref` opcional — `Some("0040")` referencia
/// catálogo vivo (set text); `None` produz hint genérico (set par, até
/// ADR dedicada existir).
pub(crate) fn unsupported_property_warn(
    target: &str,
    field: &str,
    adr_ref: Option<&str>,
) -> (String, String) {
    let msg = format!("{target}: propriedade '{field}' ainda não suportada");
    let hint = match adr_ref {
        Some(adr) => format!("ver ADR-{adr} para propriedades cobertas por set {target}"),
        None => format!("propriedades de set {target} ainda não são capturadas"),
    };
    (msg, hint)
}

/// Helper para warning de `#set` com target desconhecido (Passo 107).
/// Lista actualizada em Passo 133 para incluir `par` (activado).
pub(crate) fn unsupported_target_warn(target: &str) -> (String, String) {
    (
        format!("set: target '{target}' ainda não suportado"),
        "targets suportados: heading, page, figure, text, par, table, smartquote"
            .to_string(),
    )
}

/// **P1030** — parâmetros que **a linguagem** define para cada elemento math,
/// derivados dos campos não-`#[required]` dos elementos math do vanilla
/// (`lab/.../typst-library/src/math/`). Ver `eval.md` §P1030.
///
/// Um parâmetro **fora** desta tabela é erro (`unexpected argument`, a mesma
/// mensagem do vanilla); **dentro** dela, ou é levado à construção
/// (`MATH_SET_LIGADOS`) ou avisa nomeando elemento e parâmetro.
pub(crate) fn math_elem_params(elem: &str) -> Option<&'static [&'static str]> {
    Some(match elem {
        "equation" => &["block", "numbering", "number-align", "supplement", "alt"],
        "mat" => &["delim", "align", "augment", "gap", "row-gap", "column-gap"],
        "vec" => &["delim", "align", "gap"],
        "cases" => &["delim", "reverse", "gap"],
        "cancel" => &["length", "inverted", "cross", "angle", "stroke", "background"],
        "accent" => &["size", "dotless"],
        "frac" => &["style"],
        "lr" => &["size"],
        "stretch" => &["size"],
        "op" => &["limits"],
        "attach" => &["t", "b", "tl", "bl", "tr", "br"],
        "limits" => &["inline"],
        // Elementos math **sem** parâmetro configurável: os seus campos são
        // `#[required]`, `#[positional]` ou variádicos. Estão aqui de
        // propósito — é o que faz `#set math.binom(lower:)` dar
        // `unexpected argument: lower`, como o vanilla (medido P1030).
        "binom" | "root" | "mid" | "class" | "scripts" | "primes" | "underline"
        | "overline" | "underbrace" | "overbrace" | "underbracket" | "overbracket"
        | "underparen" | "overparen" | "undershell" | "overshell" => &[],
        _ => return None,
    })
}

/// **P1030** — `#set math.<elemento>(...)`, alvo pontuado.
///
/// `eval_set_rule` extrai o alvo por `text_str()`, que devolve `""` para um
/// `Expr::FieldAccess`; sem este caminho, todo o `#set math.*` caía no
/// fallback com alvo vazio e era ignorado em silêncio (só o aviso
/// `set: target '' ainda não suportado`). O braço de `math.equation` +
/// `numbering` continua **antes** deste e intacto.
pub(crate) fn eval_set_math_rule(
    elem: &str,
    set: SetRule<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
    target_span: Span,
) -> SourceResult<Value> {
    let Some(params) = math_elem_params(elem) else {
        // Nome de `math` que não é elemento com parâmetros. O vanilla
        // distingue três casos, medidos em P1030 Fase C: membro inexistente
        // (`module \`math\` does not contain \`foobar\``), função não-elemento
        // (`only element functions can be used in set rules`, ex. `math.abs`) e
        // símbolo (`symbol π is not callable`). Distingui-los exige resolver o
        // nome no scope do módulo — **fora da fatia 1**. Até lá avisa nomeando
        // o alvo, em vez de errar com uma mensagem que poderia ser falsa.
        engine.sink.warn_note(
            target_span,
            &format!(
                "#set math.{elem}(...) não é reconhecido como elemento configurável"
            ),
            "no vanilla isto é erro; a paridade da mensagem está por fazer",
        );
        return Ok(Value::None);
    };

    let mut seen = Vec::new();
    let mut lr_size = None;
    for arg in set.args().items() {
        let Arg::Named(named) = arg else { continue };
        let nome = named.name().as_str();

        if seen.contains(&nome) {
            return Err(vec![SourceDiagnostic::error(
                named.name().to_untyped().span(),
                format!("duplicate argument: {nome}"),
            )]);
        }
        seen.push(nome);

        if !params.contains(&nome) {
            // Paridade com o vanilla, que rejeita o argumento em vez de o
            // aceitar e ignorar (medido P1030 Fase A: `#set math.binom(lower:)`
            // → `error: unexpected argument: lower`).
            return Err(vec![SourceDiagnostic::error(
                named.name().to_untyped().span(),
                if matches!(elem, "accent" | "cases") {
                    format!("unknown property: {nome}")
                } else {
                    format!("unexpected argument: {nome}")
                },
            )]);
        }

        if MATH_SET_LIGADOS.contains(&(elem, nome)) {
            let val = eval_expr(named.expr(), scopes, ctx, engine)?;
            if (elem, nome) == ("frac", "style") {
                match &val {
                    Value::Str(value)
                        if matches!(
                            value.as_str(),
                            "vertical" | "skewed" | "horizontal"
                        ) => {}
                    Value::Str(_) => {
                        return Err(vec![SourceDiagnostic::error(
                            named.expr().span(),
                            "expected \"vertical\", \"skewed\", or \"horizontal\""
                                .to_string(),
                        )]);
                    }
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            named.expr().span(),
                            format!(
                                "expected \"vertical\", \"skewed\", or \"horizontal\", found {}",
                                match other {
                                    Value::Int(_) => "integer",
                                    Value::Str(_) => "string",
                                    Value::Bool(_) => "boolean",
                                    _ => other.type_name(),
                                }
                            ),
                        )]);
                    }
                }
            }
            if matches!(
                (elem, nome),
                ("accent", "size") | ("stretch", "size") | ("lr", "size")
            ) {
                match &val {
                    Value::Relative(_) | Value::Ratio(_) | Value::Length(_) => {}
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            named.expr().span(),
                            format!(
                                "expected relative length, found {}",
                                match other {
                                    Value::Int(_) => "integer",
                                    Value::Bool(_) => "boolean",
                                    Value::Str(_) => "string",
                                    _ => other.type_name(),
                                }
                            ),
                        )]);
                    }
                }
            }
            if (elem, nome) == ("accent", "dotless") && !matches!(&val, Value::Bool(_)) {
                return Err(vec![SourceDiagnostic::error(
                    named.expr().span(),
                    format!(
                        "expected boolean, found {}",
                        match &val {
                            Value::Int(_) => "integer",
                            Value::Str(_) => "string",
                            _ => val.type_name(),
                        }
                    ),
                )]);
            }
            if elem == "cases" {
                let mut options = crate::compiler::stdlib::MathCasesOptions::default();
                crate::compiler::stdlib::apply_math_cases_option(
                    &mut options,
                    nome,
                    &val,
                    named.expr().span(),
                    false,
                )?;
            }
            if elem == "mat" {
                let mut options = crate::compiler::stdlib::MathMatrixOptions::default();
                crate::compiler::stdlib::apply_math_matrix_option(
                    &mut options,
                    nome,
                    &val,
                    named.expr().span(),
                    false,
                )?;
            }
            if elem == "lr" {
                lr_size = Some(val);
            } else {
                *engine.styles =
                    engine.styles.push_custom(format!("math.{elem}.{nome}"), val);
            }
        } else {
            engine.sink.warn_note(
                named.name().to_untyped().span(),
                &format!(
                    "#set math.{elem}({nome}:) é da linguagem mas ainda não tem efeito"
                ),
                "o parâmetro é aceite e ignorado; o documento compila sem ele",
            );
        }
    }

    if let Some(value) = lr_size {
        *engine.styles = engine.styles.push_custom("math.lr.size", value);
    }
    Ok(Value::None)
}

#[cfg(test)]
mod issue_8973_tests {
    use crate::compiler::eval::tests::eval_for_test_keep_sink;
    use crate::contracts::world::World;
    use crate::entities::{
        content::Content,
        file_id::FileId,
        font_book::FontBook,
        layout_types::Length,
        source::Source,
        value::Value,
        world_types::{Bytes, Datetime, FileError, FileResult, Font, Library},
    };
    use std::num::NonZeroU16;

    struct TestWorld {
        library: Library,
        book: FontBook,
        source: Source,
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
    fn evaluate(
        source: &str,
    ) -> (
        crate::entities::source_result::SourceResult<crate::entities::module::Module>,
        Vec<crate::entities::source_result::SourceDiagnostic>,
    ) {
        let world = TestWorld {
            library: Library::new(),
            book: FontBook::new(),
            source: Source::new(
                FileId::from_raw(NonZeroU16::new(8973).unwrap()),
                source.into(),
            )
            .expect("valid source"),
        };
        let (result, sink) = eval_for_test_keep_sink(&world, &world.source);
        (result, sink.into_diagnostics())
    }
    #[test]
    fn issue_8973_lr_set_transports_relative_lengths_without_warning() {
        for (source_value, expected) in [
            ("1em", Value::Length(Length::em(1.0))),
            ("0pt", Value::Length(Length::pt(0.0))),
            ("-1em", Value::Length(Length::em(-1.0))),
            ("150%", Value::Ratio(crate::entities::layout_types::Ratio(1.5))),
            (
                "50% + 1em",
                Value::Relative(crate::entities::rel::Rel {
                    rel: 0.5,
                    abs: Length::em(1.0),
                }),
            ),
        ] {
            let source = format!("#set math.lr(size: {source_value})\n$ lr([a \\ b]) $");
            let (result, warnings) = evaluate(&source);
            let module = result.expect("valid relative length");
            assert!(warnings.is_empty(), "{source_value}: {warnings:?}");
            let mut transported = Vec::new();
            let mut lrs = 0;
            module.content().unwrap().visit_content_static(&mut |node| {
                if let Content::Styled(styled) = node {
                    if let Some(value) = styled
                        .styles
                        .delta()
                        .custom
                        .iter()
                        .find(|(key, _)| key.as_str() == "math.lr.size")
                        .map(|(_, value)| value)
                    {
                        transported.push(value.clone());
                    }
                }
                if let Content::MathLr(elem) = node {
                    lrs += 1;
                    assert!(!elem.size_explicit);
                    assert_eq!(elem.size.rel, 1.0);
                    assert!(elem.size.abs.is_zero());
                }
                true
            });
            assert_eq!(transported, vec![expected]);
            assert_eq!(lrs, 1);
        }
    }
    #[test]
    fn issue_8973_lr_set_rejects_wrong_types() {
        for (value, kind) in [
            ("auto", "auto"),
            ("none", "none"),
            ("true", "boolean"),
            ("1", "integer"),
            ("\"bad\"", "string"),
        ] {
            let (result, _) = evaluate(&format!("#set math.lr(size: {value})\n$lr(x)$"));
            let errors = result.expect_err("invalid set value must fail");
            assert_eq!(
                errors[0].message,
                format!("expected relative length, found {kind}")
            );
        }
    }
    #[test]
    fn issue_8973_lr_set_preserves_explicit_body_and_single_evaluation() {
        let (result,warnings)=evaluate("#let n=0\n#set math.lr(size: {n+=1; 1em})\n#set math.lr()\n#assert(n==1)\n$ lr(size: #3em, [a \\ b]) $");
        let module =
            result.expect("evaluate size expression once and retain explicit override");
        assert!(warnings.is_empty(), "{warnings:?}");
        let mut lrs = 0;
        module.content().unwrap().visit_content_static(&mut |node| {
            if let Content::MathLr(elem) = node {
                lrs += 1;
                assert!(elem.size_explicit);
                assert_eq!(elem.size.abs.em, 3.0);
                assert_eq!(elem.size.rel, 0.0);
                let mut breaks = 0;
                elem.body.visit_content_static(&mut |child| {
                    if matches!(child, Content::Linebreak(_)) {
                        breaks += 1;
                    }
                    true
                });
                assert_eq!(breaks, 1);
            }
            true
        });
        assert_eq!(lrs, 1);
    }
    #[test]
    fn issue_8973_lr_set_rejects_unknown_and_duplicate_names() {
        for (source, message) in [
            ("#set math.lr(size:1em,size:2em)\nx", "duplicate argument: size"),
            ("#set math.lr(size:1em,nope:2em)\nx", "unexpected argument: nope"),
        ] {
            assert_eq!(evaluate(source).0.expect_err("invalid rule")[0].message, message);
        }
    }
}
