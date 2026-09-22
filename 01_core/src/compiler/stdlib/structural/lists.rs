//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/structural/lists.md
//! @prompt-hash c6190f2f
//! @layer L1
//! @updated 2026-08-12
//!
//! Nativas de lista: `list`, `enum`, `terms`.
//!
//! Extraído de `stdlib/structural.rs` no Passo 1014 conforme ADR-0109
//! (atomização — forma B, free function no arquivo da unidade).

use crate::entities::file_id::FileId;

use crate::compiler::eval::operators::error_formatting::vanilla_type_name;
use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;

// ── Sentinelas e construtores de nós estruturais (Passo 69) ─────────────────

/// `terms(named: descrição, ...)` — emite `Content::Terms` com pares
/// (chave nomeada, valor descrição). A ordem dos argumentos nomeados é
/// preservada (IndexMap). Aceita `Value::Content` ou `Value::Str` como
/// descrição. Posicionais não suportados (forma chave: descrição).
///
/// **P1473** — segunda forma de chamada (paridade vanilla): variádica
/// posicional de `Content::TermItem` (produzida por `terms.item(...)` ou
/// pela sintaxe de markup `/ termo: descrição`). Se `args.items` não
/// estiver vazio, ignora `args.named` e constrói `Content::terms(items)`
/// directamente a partir dos posicionais; combinar posicional não-vazio
/// com named não-vazio é erro. A forma nomeada (acima) permanece
/// inalterada quando `args.items` está vazio.
pub fn native_terms(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    if !args.items.is_empty() {
        if !args.named.is_empty() {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "unexpected argument".to_string(),
            )]);
        }
        let mut items = Vec::with_capacity(args.items.len());
        for v in args.items.iter() {
            match v {
                Value::Content(c @ Content::TermItem(_)) => items.push(c.clone()),
                other => {
                    return Err(vec![SourceDiagnostic::error(
                        Span::detached(),
                        format!(
                            "terms(): item deve ser produzido por terms.item(...), recebeu {}",
                            other.type_name()
                        ),
                    )]);
                }
            }
        }
        return Ok(Value::Content(Content::terms(items)));
    }
    let mut items = Vec::with_capacity(args.named.len());
    for (key, value) in args.named.iter() {
        let term = Content::text(key.as_str());
        let description = match value {
            Value::Content(c) => c.clone(),
            Value::Str(s) => Content::text(s.as_str()),
            other => {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!(
                    "terms(): descrição de '{}' deve ser content ou string, recebeu {}",
                    key,
                    other.type_name()
                ),
                )])
            }
        };
        items.push(Content::term_item(term, description));
    }
    Ok(Value::Content(Content::terms(items)))
}

// ── Passo 155 (ADR-0060 Fase 1, sub-passo 2) — quote ───────────────────────

/// `list(..items, marker:?, marker-align:?, indent:?, body-indent:?, tight:?)` —
/// cria itens de lista não ordenada com marcador configurável (P470/P504/P505).
/// Cada item posicional torna-se um `Content::ListItem`.
pub fn native_list(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    use crate::entities::layout_types::{Align2D, Length};
    use crate::entities::list_marker::ListMarker;

    for key in args.named.keys() {
        if !matches!(
            key.as_str(),
            "marker" | "marker-align" | "indent" | "body-indent" | "tight"
        ) {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("argumento nomeado inesperado '{}'", key),
            )]);
        }
    }
    let marker: Option<ListMarker> = match args.named.get("marker") {
        None => None,
        Some(Value::Str(s)) => Some(ListMarker::Custom(s.clone())),
        Some(Value::Array(arr)) => {
            // P494 — marker como array de strings/contents (ex: ([•], [–], [·])).
            let mut markers = Vec::with_capacity(arr.len());
            for v in arr.iter() {
                match v {
                    Value::Str(s) => markers.push(ListMarker::Custom(s.clone())),
                    Value::Content(c) => {
                        markers.push(ListMarker::Custom(c.plain_text().into()))
                    }
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            Span::detached(),
                            format!(
                                "list(marker:) array espera strings ou content, recebeu {}",
                                other.type_name()
                            ),
                        )]);
                    }
                }
            }
            if markers.is_empty() {
                None
            } else {
                Some(ListMarker::Array(markers))
            }
        }
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "list(marker:) espera string ou array, recebeu {}",
                    other.type_name()
                ),
            )]);
        }
    };
    // P504 — alinhamento do marker (aceite; efeito visual scope-out).
    let marker_align: Option<Align2D> = match args.named.get("marker-align") {
        None => None,
        Some(Value::Align(a)) => Some(*a),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "list(marker-align:) espera alignment, recebeu {}",
                    other.type_name()
                ),
            )]);
        }
    };
    // P505 — parâmetros de indentação.
    let indent: Option<Length> = match args.named.get("indent") {
        None | Some(Value::None) => None,
        Some(Value::Length(l)) => Some(*l),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("list(indent:) espera length, recebeu {}", other.type_name()),
            )]);
        }
    };
    let body_indent: Option<Length> = match args.named.get("body-indent") {
        None | Some(Value::None) => None,
        Some(Value::Length(l)) => Some(*l),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "list(body-indent:) espera length, recebeu {}",
                    other.type_name()
                ),
            )]);
        }
    };
    let tight: Option<bool> = match args.named.get("tight") {
        None | Some(Value::None) => None,
        Some(Value::Bool(b)) => Some(*b),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("list(tight:) espera bool, recebeu {}", other.type_name()),
            )]);
        }
    };
    if args.items.is_empty() {
        return Ok(Value::Content(Content::Empty));
    }
    let mut items = Vec::with_capacity(args.items.len());
    for v in args.items.iter() {
        let body = match v {
            Value::Content(c) => c.clone(),
            Value::Str(s) => Content::text(s.as_str()),
            other => {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!(
                        "list(): item deve ser content ou string, recebeu {}",
                        other.type_name()
                    ),
                )]);
            }
        };
        let item = Content::list_item_full(
            body,
            marker.clone(),
            marker_align,
            indent,
            body_indent,
            tight,
        );
        items.push(item);
    }
    Ok(Value::Content(if items.len() == 1 {
        items.remove(0)
    } else {
        Content::Sequence(items.into())
    }))
}

/// `enum(..items, numbering:?, start:?, indent:?, body-indent:?, tight:?)` —
/// cria itens de lista ordenada com esquema de numeração configurável
/// (P470/P505). Cada item posicional torna-se um `Content::EnumItem` com
/// número 1-based respeitando `start`.
pub fn native_enum(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    use crate::entities::enum_numbering::EnumNumbering;
    use crate::entities::layout_types::Length;

    for key in args.named.keys() {
        if !matches!(
            key.as_str(),
            "numbering" | "start" | "indent" | "body-indent" | "tight"
        ) {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("argumento nomeado inesperado '{}'", key),
            )]);
        }
    }
    let numbering: Option<EnumNumbering> = match args.named.get("numbering") {
        None => None,
        Some(Value::Str(s)) => Some(EnumNumbering::from_pattern(s.as_str())),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("enum(numbering:) espera string, recebeu {}", other.type_name()),
            )]);
        }
    };
    let start: u32 = match args.named.get("start") {
        None => 1,
        Some(Value::Int(n)) if *n >= 1 => *n as u32,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "enum(start:) espera inteiro >= 1, recebeu {}",
                    other.type_name()
                ),
            )]);
        }
    };
    // P505 — parâmetros de indentação.
    let indent: Option<Length> = match args.named.get("indent") {
        None | Some(Value::None) => None,
        Some(Value::Length(l)) => Some(*l),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("enum(indent:) espera length, recebeu {}", other.type_name()),
            )]);
        }
    };
    let body_indent: Option<Length> = match args.named.get("body-indent") {
        None | Some(Value::None) => None,
        Some(Value::Length(l)) => Some(*l),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "enum(body-indent:) espera length, recebeu {}",
                    other.type_name()
                ),
            )]);
        }
    };
    let tight: Option<bool> = match args.named.get("tight") {
        None | Some(Value::None) => None,
        Some(Value::Bool(b)) => Some(*b),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("enum(tight:) espera bool, recebeu {}", other.type_name()),
            )]);
        }
    };
    if args.items.is_empty() {
        return Ok(Value::Content(Content::Empty));
    }
    let mut items = Vec::with_capacity(args.items.len());
    for (idx, v) in args.items.iter().enumerate() {
        let body = match v {
            Value::Content(c) => c.clone(),
            Value::Str(s) => Content::text(s.as_str()),
            other => {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!(
                        "enum(): item deve ser content ou string, recebeu {}",
                        other.type_name()
                    ),
                )]);
            }
        };
        let number = Some((idx as u32) + start);
        let item = Content::enum_item_full(
            number,
            body,
            numbering.clone(),
            indent,
            body_indent,
            tight,
        );
        items.push(item);
    }
    Ok(Value::Content(if items.len() == 1 {
        items.remove(0)
    } else {
        Content::Sequence(items.into())
    }))
}

// ── P1473 — `list.item`/`enum.item`/`terms.item` qualificados ───────────────

/// `list.item(body)` — constructor qualificado de `Content::ListItem` com os
/// cinco overrides sempre `None` (herdam do `list` pai quando o item entra
/// numa lista real). Diagnósticos: corpo ausente → `missing argument: body`;
/// segundo positional → `unexpected argument`; `body:` nomeado → `` the
/// argument `body` is positional `` + hint; tipo errado → `expected content,
/// found <tipo>`.
pub fn native_list_item(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    if let Some(key) = args.named.keys().next() {
        let diagnostic = if key.as_str() == "body" {
            SourceDiagnostic::error(Span::detached(), "the argument `body` is positional")
                .with_hint("try removing `body:`")
        } else {
            SourceDiagnostic::error(
                Span::detached(),
                format!("unexpected argument: {key}"),
            )
        };
        return Err(vec![diagnostic]);
    }
    if args.items.len() > 1 {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "unexpected argument".to_string(),
        )]);
    }
    let body = match args.items.first() {
        Some(Value::Content(c)) => c.clone(),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("expected content, found {}", vanilla_type_name(other)),
            )]);
        }
        None => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "missing argument: body".to_string(),
            )]);
        }
    };
    Ok(Value::Content(Content::list_item(body)))
}

/// `enum.item(number?, body)` — constructor qualificado de `Content::EnumItem`.
/// Os dois positionals são casados por **tipo**, não ordem: o primeiro `Int`
/// encontrado vira `number`, o `Content` vira `body` (obrigatório). Os três
/// overrides restantes (`numbering`/`indent`/`body_indent`/`tight`) ficam
/// sempre `None`. Diagnósticos: `body` ausente → `missing argument: body`;
/// positional que não casa `Int` nem `Content` → `expected content, found
/// <tipo>` (o cast tenta o slot `body`, não relata o slot `number`); demais
/// diagnósticos espelham `list.item`.
pub fn native_enum_item(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    if let Some(key) = args.named.keys().next() {
        let diagnostic = if key.as_str() == "body" {
            SourceDiagnostic::error(Span::detached(), "the argument `body` is positional")
                .with_hint("try removing `body:`")
        } else {
            SourceDiagnostic::error(
                Span::detached(),
                format!("unexpected argument: {key}"),
            )
        };
        return Err(vec![diagnostic]);
    }
    if args.items.len() > 2 {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "unexpected argument".to_string(),
        )]);
    }
    let mut number: Option<u32> = None;
    let mut body: Option<Content> = None;
    for v in args.items.iter() {
        match v {
            Value::Int(n) if number.is_none() => number = Some(*n as u32),
            Value::Content(c) if body.is_none() => body = Some(c.clone()),
            other => {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!("expected content, found {}", vanilla_type_name(other)),
                )]);
            }
        }
    }
    let body = match body {
        Some(b) => b,
        None => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "missing argument: body".to_string(),
            )]);
        }
    };
    Ok(Value::Content(Content::enum_item(number, body)))
}

/// `terms.item(term, description)` — constructor qualificado de
/// `Content::TermItem`. Dois `Content` posicionais obrigatórios, casados por
/// **ordem** (não por tipo). Faltando qualquer um → `missing argument:
/// term`/`missing argument: description` conforme o ausente; excesso →
/// `unexpected argument`.
pub fn native_terms_item(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    if let Some(key) = args.named.keys().next() {
        let diagnostic = if matches!(key.as_str(), "term" | "description") {
            SourceDiagnostic::error(
                Span::detached(),
                format!("the argument `{key}` is positional"),
            )
            .with_hint(format!("try removing `{key}:`"))
        } else {
            SourceDiagnostic::error(
                Span::detached(),
                format!("unexpected argument: {key}"),
            )
        };
        return Err(vec![diagnostic]);
    }
    if args.items.len() > 2 {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "unexpected argument".to_string(),
        )]);
    }
    let term = match args.items.first() {
        Some(Value::Content(c)) => c.clone(),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("expected content, found {}", vanilla_type_name(other)),
            )]);
        }
        None => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "missing argument: term".to_string(),
            )]);
        }
    };
    let description = match args.items.get(1) {
        Some(Value::Content(c)) => c.clone(),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("expected content, found {}", vanilla_type_name(other)),
            )]);
        }
        None => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "missing argument: description".to_string(),
            )]);
        }
    };
    Ok(Value::Content(Content::term_item(term, description)))
}

// ── Passo 512 — Grid/Table HLine/VLine ──────────────────────────────────────

/// P1473 (Agente B) — contrato RED de `list.item`/`enum.item`/`terms.item`
/// qualificados e da segunda forma (posicional) de `terms()`. Fonte:
/// `00_nucleo/materialization/typst-passo-1473.md` +
/// `00_nucleo/prompts/compiler/stdlib/structural/lists.md` §P1473. Escrito
/// sem ver a implementação (isolamento Agente A/B) — usa exclusivamente a
/// avaliação de expressões Typst (`eval_expression`) como oráculo de
/// observáveis de língua, nunca reconstrução directa da implementação
/// ainda inexistente.
#[cfg(test)]
mod p1473_tests {
    use super::*;
    use crate::compiler::eval::eval_expression;
    use crate::contracts::world::World;
    use crate::entities::source_result::SourceResult;
    use crate::entities::value::Value;
    use crate::entities::world_types::{
        Bytes, Datetime, FileError, FileResult, Font, Library,
    };
    use std::num::NonZeroU16;

    #[derive(Default)]
    struct NullWorld {
        library: Library,
        book: crate::entities::font_book::FontBook,
    }

    impl World for NullWorld {
        fn library(&self) -> &Library {
            &self.library
        }
        fn book(&self) -> &crate::entities::font_book::FontBook {
            &self.book
        }
        fn main(&self) -> FileId {
            FileId::from_raw(NonZeroU16::new(1473).unwrap())
        }
        fn source(&self, _: FileId) -> FileResult<crate::entities::source::Source> {
            Err(FileError::NotFound)
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

    /// Avalia uma expressão Typst isolada com o stdlib completo (P1473 usa
    /// `list`/`enum`/`terms`/`polygon`/`figure`/`footnote`/`color` já
    /// globais — `eval_expression` (`compiler/eval/mod.rs`) monta esse
    /// scope a partir de `make_stdlib_with_features`).
    fn eval(src: &str) -> SourceResult<Value> {
        eval_expression(&NullWorld::default(), src).0
    }

    /// Avalia um literal de conteúdo (`[texto]`) — usado como oráculo do
    /// `body`/`term`/`description` esperado, sem reconstruir manualmente a
    /// árvore de `Content` que o parser produziria (evita divergir do
    /// parser real por engano).
    fn content_of(src: &str) -> Content {
        match eval(src) {
            Ok(Value::Content(c)) => c,
            other => panic!("{src}: esperava Ok(Value::Content), obteve {other:?}"),
        }
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

    fn expect_err_message_and_hints(src: &str) -> (String, Vec<String>) {
        match eval(src) {
            Err(errors) => {
                assert_eq!(
                    errors.len(),
                    1,
                    "{src}: esperava exactamente 1 diagnóstico, obteve {errors:?}"
                );
                (errors[0].message.clone(), errors[0].hints.clone())
            }
            Ok(v) => panic!("{src}: esperava Err, obteve Ok({v:?})"),
        }
    }

    // ── `list.item` ──────────────────────────────────────────────────────

    #[test]
    fn p1473_list_item_type_is_function() {
        assert_eq!(expect_ok("type(list.item) == function"), Value::Bool(true));
    }

    #[test]
    fn p1473_list_item_constructs_body_only() {
        let body = content_of("[hi]");
        assert_eq!(
            expect_ok("list.item([hi])"),
            Value::Content(Content::list_item(body)),
            "os cinco opcionais (marker/marker_align/indent/body_indent/tight) \
             devem ficar None"
        );
    }

    #[test]
    fn p1473_list_item_missing_body() {
        assert_eq!(expect_err_message("list.item()"), "missing argument: body");
    }

    #[test]
    fn p1473_list_item_extra_positional() {
        assert_eq!(expect_err_message("list.item([a], [b])"), "unexpected argument");
    }

    #[test]
    fn p1473_list_item_named_body_is_positional() {
        let (message, hints) = expect_err_message_and_hints("list.item(body: [a])");
        assert_eq!(message, "the argument `body` is positional");
        assert_eq!(hints, vec!["try removing `body:`".to_string()]);
    }

    #[test]
    fn p1473_list_item_wrong_type() {
        assert_eq!(expect_err_message("list.item(5)"), "expected content, found integer");
    }

    // ── `enum.item` ──────────────────────────────────────────────────────

    #[test]
    fn p1473_enum_item_type_is_function() {
        assert_eq!(expect_ok("type(enum.item) == function"), Value::Bool(true));
    }

    #[test]
    fn p1473_enum_item_matches_by_type_not_order() {
        // Medido no L0: enum.item(3, [a]) == enum.item([a], 3) — os dois
        // positionals são casados por TIPO, não por posição.
        assert_eq!(
            expect_ok("enum.item(3, [a]) == enum.item([a], 3)"),
            Value::Bool(true)
        );
    }

    #[test]
    fn p1473_enum_item_constructs_number_and_body() {
        let body = content_of("[a]");
        assert_eq!(
            expect_ok("enum.item(3, [a])"),
            Value::Content(Content::enum_item(Some(3), body))
        );
    }

    #[test]
    fn p1473_enum_item_missing_body() {
        assert_eq!(expect_err_message("enum.item()"), "missing argument: body");
        assert_eq!(expect_err_message("enum.item(3)"), "missing argument: body");
    }

    #[test]
    fn p1473_enum_item_wrong_type_reports_body_slot() {
        // Medido: primeiro positional Float (não casa Int nem Content) →
        // "expected content, found float" — o cast tenta o slot `body`
        // quando o valor não é Int, não relata o slot `number`.
        assert_eq!(
            expect_err_message("enum.item(1.5, [a])"),
            "expected content, found float"
        );
    }

    // ── `terms.item` ─────────────────────────────────────────────────────

    #[test]
    fn p1473_terms_item_type_is_function() {
        assert_eq!(expect_ok("type(terms.item) == function"), Value::Bool(true));
    }

    #[test]
    fn p1473_terms_item_constructs_term_and_description_in_order() {
        let term = content_of("[t]");
        let description = content_of("[d]");
        assert_eq!(
            expect_ok("terms.item([t], [d])"),
            Value::Content(Content::term_item(term, description))
        );
    }

    #[test]
    fn p1473_terms_item_missing_term() {
        assert_eq!(expect_err_message("terms.item()"), "missing argument: term");
    }

    #[test]
    fn p1473_terms_item_missing_description() {
        assert_eq!(
            expect_err_message("terms.item([t])"),
            "missing argument: description"
        );
    }

    #[test]
    fn p1473_terms_item_extra_positional() {
        assert_eq!(
            expect_err_message("terms.item([t], [d], [e])"),
            "unexpected argument"
        );
    }

    // ── `terms()` ganha forma posicional (P1473) ────────────────────────

    #[test]
    fn p1473_terms_gains_positional_form_via_terms_item() {
        let a_term = content_of("[A]");
        let a_desc = content_of("[1]");
        let b_term = content_of("[B]");
        let b_desc = content_of("[2]");
        let expected = Content::terms(vec![
            Content::term_item(a_term, a_desc),
            Content::term_item(b_term, b_desc),
        ]);
        assert_eq!(
            expect_ok("terms(terms.item([A], [1]), terms.item([B], [2]))"),
            Value::Content(expected)
        );
    }

    #[test]
    fn p1473_terms_positional_and_named_mixed_is_error() {
        // Medido/decidido no L0: positional não-vazio + named não-vazio no
        // mesmo call é erro "unexpected argument" (ancorado no primeiro
        // named) — as duas formas não se combinam.
        assert_eq!(
            expect_err_message("terms(terms.item([A], [1]), B: [2])"),
            "unexpected argument"
        );
    }

    #[test]
    fn p1473_terms_named_form_is_not_a_regression() {
        // Não-regressão: a forma nomeada pré-existente (`terms(A: [1])`)
        // continua válida — este teste deve já passar hoje (GREEN), a
        // decisão do L0 é aditiva, não substitutiva.
        let one = content_of("[1]");
        let expected = Content::terms(vec![Content::term_item(Content::text("A"), one)]);
        assert_eq!(expect_ok("terms(A: [1])"), Value::Content(expected));
    }

    // ── Seis registos com namespace — três destes seis vivem aqui ───────

    #[test]
    fn p1473_list_enum_terms_parents_keep_function_kind_and_direct_call() {
        // Não-regressão: `list`/`enum`/`terms` continuam `type()==function`
        // e a chamada directa (sem `.item`) continua a funcionar como
        // antes, depois de ganharem o namespace com `item`.
        for (parent_call, expect_ok_direct) in [
            ("list", "list([a], [b])"),
            ("enum", "enum([a], [b])"),
            ("terms", "terms(A: [1])"),
        ] {
            assert_eq!(
                expect_ok(&format!("type({parent_call}) == function")),
                Value::Bool(true),
                "{parent_call}"
            );
            // Chamada directa não deve regredir para Err.
            eval(expect_ok_direct).unwrap_or_else(|e| {
                panic!("{expect_ok_direct}: chamada directa do pai regrediu: {e:?}")
            });
        }
    }
}
