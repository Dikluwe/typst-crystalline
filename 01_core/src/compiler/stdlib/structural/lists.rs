//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/structural/lists.md
//! @prompt-hash 847eb4ad
//! @layer L1
//! @updated 2026-08-12
//!
//! Nativas de lista: `list`, `enum`, `terms`.
//!
//! Extraído de `stdlib/structural.rs` no Passo 1014 conforme ADR-0109
//! (atomização — forma B, free function no arquivo da unidade).

use crate::entities::file_id::FileId;

pub(crate) use crate::compiler::stdlib::shape_frame::Named;
use crate::compiler::eval::operators::error_formatting::vanilla_type_name;
use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::elements::terms::{ListKind, TermsElem};
use crate::entities::layout_types::{Align2D, HAlign, Length};
use crate::entities::style_chain::StyleChain;
use std::sync::Arc;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;

// ── Sentinelas e construtores de nós estruturais (Passo 69) ─────────────────

/// `terms(..itens, …)` — itens, pares em array, ou pares nomeados (extensão).
pub fn native_terms(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    container(ListKind::Terms, args)
}

/// `list(..itens, …)`.
pub fn native_list(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    container(ListKind::List, args)
}

/// `enum(..itens, …)`.
pub fn native_enum(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    container(ListKind::Enum, args)
}

/// Nomes declaráveis, na ordem de `fields()`.
pub(crate) fn names(kind: ListKind) -> &'static [&'static str] {
    match kind {
        ListKind::Terms => {
            &["tight", "separator", "indent", "hanging-indent", "spacing"]
        }
        ListKind::List => {
            &["tight", "marker", "indent", "body-indent", "spacing", "marker-align"]
        }
        ListKind::Enum => &[
            "tight",
            "numbering",
            "start",
            "full",
            "reversed",
            "indent",
            "body-indent",
            "spacing",
            "number-align",
        ],
    }
}

/// Valor padrão de uma propriedade.
pub(crate) fn default(kind: ListKind, name: &str) -> Value {
    match name {
        "tight" => Value::Bool(true),
        "full" | "reversed" => Value::Bool(false),
        "indent" => Value::Length(Length::ZERO),
        // ref: lab/typst-original/crates/typst-library/src/model/list.rs:100-102
        "body-indent" => Value::Length(Length::em(0.5)),
        // ref: lab/typst-original/crates/typst-library/src/model/terms.rs:76
        "hanging-indent" => Value::Length(Length::em(2.0)),
        "numbering" => Value::Str("1.".into()),
        "marker" => Value::Array(
            ["•", "‣", "–"]
                .into_iter()
                .map(|marker| Value::Content(Content::text(marker)))
                .collect(),
        ),
        "marker-align" | "number-align" => Value::Align(Align2D {
            h: Some(HAlign::End),
            v: None,
        }),
        "separator" if kind == ListKind::Terms => Value::Content(separator()),
        _ => Value::Auto,
    }
}

/// O separador padrão de termo e descrição.
pub(crate) fn separator() -> Content {
    // ref: lab/typst-original/crates/typst-library/src/model/terms.rs:53
    Content::HSpace(Arc::new(crate::entities::elements::h_space::HSpaceElem {
        amount: crate::entities::elements::h_space::Spacing::Absolute(Length::em(0.6)),
        weak: true,
        weak_explicit: true,
    }))
}

/// Propriedade em vigor: lexical, declarada ou padrão.
pub(crate) fn property(e: &TermsElem, name: &str, chain: &StyleChain) -> Value {
    e.lexical(name)
        .or_else(|| chain.custom(&format!("{}.{name}", e.kind.name())))
        .cloned()
        .unwrap_or_else(|| default(e.kind, name))
}

pub(crate) fn kind_of(element: &str) -> Option<ListKind> {
    [ListKind::Terms, ListKind::List, ListKind::Enum]
        .into_iter()
        .find(|kind| kind.name() == element)
}

/// Valor normalizado de uma propriedade cujo tipo já foi conferido.
pub(crate) fn admit(name: &str, value: &Value, span: Span) -> SourceResult<Value> {
    if name == "marker" {
        if let Value::Array(markers) = value {
            if markers.is_empty() {
                return Err(vec![SourceDiagnostic::error(
                    span,
                    "array must contain at least one marker",
                )]);
            }
            return Ok(Value::Array(
                markers.iter().map(|marker| Value::Content(display(marker))).collect(),
            ));
        }
    }
    if matches!(name, "marker" | "separator") && !matches!(value, Value::Func(_)) {
        return Ok(Value::Content(display(value)));
    }
    if let ("start", Value::Int(number)) = (name, value) {
        admit_number(*number, span)?;
    }
    Ok(value.clone())
}

fn admit_number(number: i64, span: Span) -> SourceResult<u32> {
    if number < 0 {
        return Err(vec![SourceDiagnostic::error(span, "number must be at least zero")]);
    }
    u32::try_from(number)
        .map_err(|_| vec![SourceDiagnostic::error(span, "number too large")])
}

fn display(value: &Value) -> Content {
    crate::compiler::eval::value_to_display_content(value.clone())
        .unwrap_or(Content::Empty)
}

/// Conteúdo admitido como corpo.
fn content(value: &Value, span: Span) -> SourceResult<Content> {
    if matches!(
        value,
        Value::Content(_)
            | Value::LocatedContent(..)
            | Value::Str(_)
            | Value::Symbol(_)
            | Value::None
    ) {
        Ok(display(value))
    } else {
        Err(vec![SourceDiagnostic::error(
            span,
            format!("expected content, found {}", vanilla_type_name(value)),
        )])
    }
}

fn container(kind: ListKind, args: &Args) -> SourceResult<Value> {
    let known = names(kind);
    let mut named = Named::default();
    let mut items = Vec::new();
    for arg in args.occurrence_sequence() {
        let span = arg.value_span.or(arg.span).or(args.span);
        if let Some(name) = arg.name.as_deref() {
            if kind == ListKind::Terms && !known.contains(&name) {
                if args.occurrence_sequence().iter().any(|arg| arg.name.is_none()) {
                    return Err(vec![SourceDiagnostic::error(span, "unexpected argument")]);
                }
                items.push(Content::term_item(
                    Content::text(name),
                    content(&arg.value, span)?,
                ));
                continue;
            }
            let single = Args::from_occurrences(args.span, vec![arg.clone()]);
            crate::compiler::eval::rules::check_declaration(kind.name(), &single)?;
            named.insert(name.into(), admit(name, &arg.value, span)?);
            continue;
        }
        items.push(match kind {
            ListKind::Terms => term(&arg.value, span)?,
            ListKind::List | ListKind::Enum => {
                let body = content(&arg.value, span)?;
                if crate::compiler::realize::lists::item_kind(&body) == Some(kind) {
                    body
                } else if kind == ListKind::List {
                    Content::list_item(body)
                } else {
                    Content::enum_item(None, body)
                }
            }
        });
    }
    let order: Named = known
        .iter()
        .filter_map(|name| Some(((*name).into(), named.get(*name)?.clone())))
        .collect();
    let order = (!order.is_empty()).then(|| Arc::new(order));
    Ok(Value::Content(Content::list_like(kind, items, order)))
}

fn term(value: &Value, span: Span) -> SourceResult<Content> {
    if let Value::Array(pair) = value {
        if let [term, description] = &pair[..] {
            return Ok(Content::term_item(
                content(term, span)?,
                content(description, span)?,
            ));
        }
        return Err(vec![SourceDiagnostic::error(
            span,
            "array must contain exactly two entries",
        )]);
    }
    if let Value::Content(item) = value {
        if crate::compiler::realize::lists::item_kind(item) == Some(ListKind::Terms) {
            return Ok(item.clone());
        }
    }
    Err(vec![SourceDiagnostic::error(span, "expected term item or array")])
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
        Some(value @ (Value::Str(_) | Value::Symbol(_))) => display(value),
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
            Value::Int(n) if number.is_none() => {
                number = Some(admit_number(*n, args.span)?);
            }
            Value::Auto if number.or(body.as_ref().map(|_| 0)).is_none() => {}
            Value::Content(c) if body.is_none() => body = Some(c.clone()),
            Value::Str(_) | Value::Symbol(_) if body.is_none() => {
                body = Some(display(v));
            }
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
        Some(value @ (Value::Str(_) | Value::Symbol(_))) => display(value),
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
        Some(value @ (Value::Str(_) | Value::Symbol(_))) => display(value),
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
/// `lab/history/construction/records/00_nucleo/materialization/typst-passo-1473.md` +
/// `00_nucleo/prompts/core/compiler/stdlib/structural/lists.md` §P1473. Escrito
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
