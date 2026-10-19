//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/structural/footnote.md
//! @prompt-hash 1c231121
//! @layer L1
//! @updated 2026-08-13
//!
//! `footnote` — construção da nota (o render vive em `compiler/layout/footnote.rs`).
//!
//! Separado de `structural/flow.rs` em 2026-08-13: a fronteira anterior
//! estava sustentada por um cluster de co-mudança que não existia (artefacto
//! de atribuição da ferramenta). Fronteira nova medida no L0 deste nó.

use crate::compiler::eval::operators::error_formatting::vanilla_type_name;
use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::file_id::FileId;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;
use ecow::EcoString;

// ── Passo 157A (ADR-0060 Fase 2 sub-passo 1) — table minimal ────────────────

/// `footnote(body, numbering:?)` — emite `Content::Footnote { body, numbering }`.
/// Body posicional obrigatório (content ou string).
/// P502: `numbering` named opcional; uso no marcador scope-out per ADR-0054.
pub fn native_footnote(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let body = match args.items.first() {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::Str(s)) => Content::text(s.as_str()),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "footnote() espera content ou string, recebeu {}",
                    other.type_name()
                ),
            )])
        }
        None => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "footnote() exige body como argumento posicional".to_string(),
            )])
        }
    };

    let mut numbering: Option<EcoString> = None;
    for (k, v) in args.named.iter() {
        match k.as_str() {
            "numbering" => {
                numbering = match v {
                    Value::Str(s) => Some(s.clone()),
                    Value::None => None,
                    _ => {
                        return Err(vec![SourceDiagnostic::error(
                            Span::detached(),
                            format!(
                                "footnote(numbering:): espera string, recebeu {}",
                                v.type_name()
                            ),
                        )])
                    }
                };
            }
            other => {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!("footnote(): argumento nomeado '{}' não suportado (cosméticos scope-out per ADR-0054 graded)", other),
                )]);
            }
        }
    }

    Ok(Value::Content(Content::footnote_with_numbering(body, numbering)))
}

// ── P1473 — `footnote.entry(note, separator?, clearance?, gap?, indent?)` ──

/// `footnote.entry(note, separator?, clearance?, gap?, indent?)` — escopo
/// reduzido (construção sem realização). `note` é `Content` posicional
/// obrigatório (mesma coerção `Str → Content::text` de `footnote`). Os
/// quatro nomeados são opcionais (`separator: Content`, demais `Length`),
/// aceites e **preservados sem efeito de layout** — mesmo padrão já vigente
/// em `ListItemElem` para `marker_align`/`indent`/`tight`. `.realize()`
/// (link/superscript via `Counter`), `ShowSet` e
/// `compiler/layout/footnote.rs` não mudam nesta fatia — ver
/// `compiler/stdlib/structural/footnote.md` §P1473.
pub fn native_footnote_entry(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    use crate::entities::layout_types::Length;

    let note = match args.items.first() {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::Str(s)) => Content::text(s.as_str()),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("expected content, found {}", vanilla_type_name(other)),
            )])
        }
        None => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "missing argument: note".to_string(),
            )])
        }
    };
    if args.items.len() > 1 {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "unexpected argument".to_string(),
        )]);
    }

    let mut separator: Option<Content> = None;
    let mut clearance: Option<Length> = None;
    let mut gap: Option<Length> = None;
    let mut indent: Option<Length> = None;
    for (k, v) in args.named.iter() {
        match k.as_str() {
            "separator" => {
                separator = match v {
                    Value::Content(c) => Some(c.clone()),
                    Value::None => None,
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            Span::detached(),
                            format!(
                                "expected content, found {}",
                                vanilla_type_name(other)
                            ),
                        )]);
                    }
                };
            }
            "clearance" => clearance = Some(extract_entry_length(v)?),
            "gap" => gap = Some(extract_entry_length(v)?),
            "indent" => indent = Some(extract_entry_length(v)?),
            other => {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!("unexpected argument: {other}"),
                )]);
            }
        }
    }

    Ok(Value::Content(Content::footnote_entry(note, separator, clearance, gap, indent)))
}

fn extract_entry_length(
    v: &Value,
) -> SourceResult<crate::entities::layout_types::Length> {
    match v {
        Value::Length(l) => Ok(*l),
        other => Err(vec![crate::compiler::eval::rules::expected_length_error(
            other,
            Span::detached(),
        )]),
    }
}

// ── Passo 296 — `accent()` + `cancel()` math (P-math-accent-cancel) ──────
//
// **A.0.0 N=4 (HIV)**: features ausentes apesar de Tabela A.4
// marcar `parcial`. Materialização from-scratch. Padrão "variant
// rico" N=4 **preservado** (A.2 → (a) minimal; sem cosméticos
// `size`/`length`/`inverted`/`cross`/`angle`/`stroke`).

/// P1473 (Agente B) — contrato RED de `footnote.entry` (escopo reduzido:
/// construção sem realização). Fonte:
/// `lab/history/construction/records/00_nucleo/materialization/typst-passo-1473.md` +
/// `00_nucleo/prompts/core/compiler/stdlib/structural/footnote.md` §P1473 +
/// `00_nucleo/prompts/core/entities/elements/footnote_entry.md`. Escrito sem ver
/// a implementação (isolamento Agente A/B).
///
/// `Content::FootnoteEntry`/`FootnoteEntryElem` ainda não existem neste
/// baseline. Mesma estratégia de `figure_image.rs` §P1473: observáveis
/// internos via `Content::elem_name()`/`Content::plain_text()`/Debug
/// (métodos genéricos já exaustivamente casados sobre `Content`), nunca
/// referência directa à variante/entidade ainda inexistente — evita
/// quebrar a compilação do crate inteiro por causa de um tipo que só o
/// Agente A vai introduzir.
///
/// **Inferência marcada (ADR-0108):** o L0 não transcreve o texto exacto
/// das mensagens de `missing argument`/tipo errado para `footnote.entry`
/// (ao contrário de `figure.caption`, que as dá verbatim). Os testes de
/// diagnóstico abaixo assumem a convenção já uniforme nas rotas irmãs do
/// mesmo passo (`missing argument: note`, `expected content, found
/// <tipo>`, mesmo padrão de `list.item`/`figure.caption`). Seria refutado
/// por uma redação futura do L0 que documente um texto diferente — nesse
/// caso só estas duas asserções precisam de ajuste, não o resto do
/// ficheiro.
#[cfg(test)]
mod p1473_tests {
    use super::*;
    use crate::compiler::eval::eval_expression;
    use crate::entities::source_result::SourceResult;

    #[derive(Default)]
    struct NullWorld {
        library: crate::entities::world_types::Library,
        book: crate::entities::font_book::FontBook,
    }
    impl crate::contracts::world::World for NullWorld {
        fn library(&self) -> &crate::entities::world_types::Library {
            &self.library
        }
        fn book(&self) -> &crate::entities::font_book::FontBook {
            &self.book
        }
        fn main(&self) -> FileId {
            FileId::from_raw(std::num::NonZeroU16::new(1473).unwrap())
        }
        fn source(
            &self,
            _: FileId,
        ) -> crate::entities::world_types::FileResult<crate::entities::source::Source>
        {
            Err(crate::entities::world_types::FileError::NotFound)
        }
        fn file(
            &self,
            _: FileId,
        ) -> crate::entities::world_types::FileResult<crate::entities::world_types::Bytes>
        {
            Err(crate::entities::world_types::FileError::NotFound)
        }
        fn font(&self, _: usize) -> Option<crate::entities::world_types::Font> {
            None
        }
        fn today(
            &self,
            _: Option<crate::entities::duration::Duration>,
        ) -> Option<crate::entities::world_types::Datetime> {
            None
        }
    }

    fn eval(src: &str) -> SourceResult<Value> {
        eval_expression(&NullWorld::default(), src).0
    }

    fn expect_ok(src: &str) -> Value {
        match eval(src) {
            Ok(v) => v,
            Err(errors) => panic!("{src}: esperava Ok, obteve Err({errors:?})"),
        }
    }

    fn expect_err_message(src: &str) -> String {
        match eval(src) {
            Err(errors) => {
                assert_eq!(
                    errors.len(),
                    1,
                    "{src}: esperava exactamente 1 diagnóstico, obteve {errors:?}"
                );
                errors[0].message.clone()
            }
            Ok(v) => panic!("{src}: esperava Err, obteve Ok({v:?})"),
        }
    }

    fn content_plain_text(src: &str) -> String {
        match expect_ok(src) {
            Value::Content(c) => c.plain_text(),
            other => panic!("{src}: esperava Value::Content, obteve {other:?}"),
        }
    }

    #[test]
    fn p1473_footnote_entry_type_is_function() {
        assert_eq!(expect_ok("type(footnote.entry) == function"), Value::Bool(true));
    }

    #[test]
    fn p1473_footnote_entry_constructs_note_only() {
        let value = expect_ok("footnote.entry([hello])");
        let Value::Content(content) = &value else {
            panic!("esperava Value::Content, obteve {value:?}");
        };
        assert_eq!(content.elem_name(), "footnote.entry");
        assert_eq!(content.plain_text(), content_plain_text("[hello]"));
        let debug = format!("{content:?}");
        for field in ["separator: None", "clearance: None", "gap: None", "indent: None"] {
            assert!(debug.contains(field), "{field} ausente em: {debug}");
        }
    }

    #[test]
    fn p1473_footnote_entry_constructs_with_all_four_named() {
        let value = expect_ok(
            "footnote.entry([hello], separator: [--], clearance: 2em, gap: 1em, indent: 0.5em)",
        );
        let Value::Content(content) = &value else {
            panic!("esperava Value::Content, obteve {value:?}");
        };
        let debug = format!("{content:?}");
        for field in
            ["separator: Some(", "clearance: Some(", "gap: Some(", "indent: Some("]
        {
            assert!(debug.contains(field), "{field} ausente em: {debug}");
        }
    }

    #[test]
    fn p1473_footnote_entry_missing_note() {
        // Inferência marcada (ver doc do módulo): convenção `missing
        // argument: <nome>` partilhada com `list.item`/`figure.caption`.
        assert_eq!(expect_err_message("footnote.entry()"), "missing argument: note");
    }

    #[test]
    fn p1473_footnote_entry_wrong_type() {
        // Inferência marcada (ver doc do módulo): convenção `expected
        // content, found <tipo>` partilhada com `list.item`/`figure.caption`.
        assert_eq!(
            expect_err_message("footnote.entry(5)"),
            "expected content, found integer"
        );
    }

    #[test]
    fn p1473_footnote_parent_keeps_function_kind_and_direct_call() {
        assert_eq!(expect_ok("type(footnote) == function"), Value::Bool(true));
        eval("footnote([note text])")
            .unwrap_or_else(|e| panic!("chamada directa de footnote() regrediu: {e:?}"));
    }
}
