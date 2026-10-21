//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/eval/operators/equality.md
//! @prompt-hash f8077a4c
//! @layer L1
//! @updated 2026-10-06
//!
//!
//! Igualdade da linguagem (`==`/`!=`, com coerção Int↔Float recursiva e
//! `Content` morfológico) e pertença (`in`/`not in`). A fronteira de erro é
//! do nó `error_formatting`.

use crate::entities::ast::expr::BinOp;
use crate::entities::color::Color;
use crate::entities::content::Content;
use crate::entities::counter::CounterKey;
use crate::entities::selector::Selector;
use crate::entities::value::Value;

use super::error_formatting::binary_mismatch;

/// Avalia `==`/`!=`/`in`/`not in` com a semântica da linguagem (ADR-0025:
/// distinta do `PartialEq` derivado, que fica para o Rust).
pub(crate) fn apply_binary(op: BinOp, lhs: Value, rhs: Value) -> Result<Value, String> {
    match (op, lhs, rhs) {
        // ── Comparações ──────────────────────────────────────────────────────
        // ADR-0025: coerção Int↔Float em Eq/Neq e ordenação, como no original.
        // derive(PartialEq) mantido para IndexMap, testes Rust, e estruturas de dados —
        // mas eval_binary_op replica a semântica do Typst (1 == 1.0 → true).
        (BinOp::Eq, Value::Int(a), Value::Float(b)) => Ok(Value::Bool((a as f64) == b)),
        (BinOp::Eq, Value::Float(a), Value::Int(b)) => Ok(Value::Bool(a == (b as f64))),
        (BinOp::Neq, Value::Int(a), Value::Float(b)) => Ok(Value::Bool((a as f64) != b)),
        (BinOp::Neq, Value::Float(a), Value::Int(b)) => Ok(Value::Bool(a != (b as f64))),
        // P345 (ADR-0107): o `==` da linguagem sobre conteúdo é **morfológico** —
        // compara texto/markup/estilo semântico (`*bold*`) e **ignora** o estilo de
        // render (o `TextStyle` assado, o transporte β1, o numbering assado da chain)
        // via `Content::morph_canon`. Fecha o Achado 2 (`it.body == [a]` casa) na
        // camada da linguagem. **Não** toca o `derive(PartialEq)` do Rust — dois
        // sistemas (ADR-0025): a forma canônica é comparada com o `==` estrutural.
        (BinOp::Eq, Value::Content(a), Value::Content(b)) => {
            Ok(Value::Bool(content_language_eq(&a, &b)))
        }
        (BinOp::Neq, Value::Content(a), Value::Content(b)) => {
            Ok(Value::Bool(!content_language_eq(&a, &b)))
        }
        // P684 — comparação Version directa sobre todos os componentes (zero-pad),
        // sem qualquer tratamento especial de `pre`/`build` (não existem no Typst).
        (BinOp::Eq, Value::Version(a), Value::Version(b)) => Ok(Value::Bool(a == b)),
        (BinOp::Neq, Value::Version(a), Value::Version(b)) => Ok(Value::Bool(a != b)),
        // P785b — Comparação Ratio ↔ Relative (com abs zero, ex: 50% == (10pt + 50%).ratio)
        (BinOp::Eq, Value::Ratio(r), Value::Relative(rel))
        | (BinOp::Eq, Value::Relative(rel), Value::Ratio(r)) => {
            Ok(Value::Bool(rel.abs.is_zero() && (rel.rel - r.0).abs() < 1e-9))
        }
        (BinOp::Neq, Value::Ratio(r), Value::Relative(rel))
        | (BinOp::Neq, Value::Relative(rel), Value::Ratio(r)) => {
            Ok(Value::Bool(!rel.abs.is_zero() || (rel.rel - r.0).abs() >= 1e-9))
        }
        (BinOp::Eq, a, b) => Ok(Value::Bool(values_eq(&a, &b))),
        (BinOp::Neq, a, b) => Ok(Value::Bool(!values_eq(&a, &b))),
        // ── P706 — `in` / `not in` ────────────────────────────────────────────
        // `Str in Dict` testa se a chave existe; `Str in Str` testa substring;
        // `any in Array` testa igualdade de elemento (mesma coerção Int/Float
        // do `==`, `value_eq` abaixo — cobre arrays aninhados e tipos mistos
        // sem código extra, via `PartialEq` recursivo de `Value`).
        (BinOp::In, Value::Str(s), Value::Dict(d)) => {
            Ok(Value::Bool(d.contains_key(s.as_str())))
        }
        (BinOp::NotIn, Value::Str(s), Value::Dict(d)) => {
            Ok(Value::Bool(!d.contains_key(s.as_str())))
        }
        (BinOp::In, Value::Str(needle), Value::Str(haystack)) => {
            Ok(Value::Bool(haystack.as_str().contains(needle.as_str())))
        }
        (BinOp::NotIn, Value::Str(needle), Value::Str(haystack)) => {
            Ok(Value::Bool(!haystack.as_str().contains(needle.as_str())))
        }
        (BinOp::In, needle, Value::Array(arr)) => {
            Ok(Value::Bool(arr.iter().any(|item| value_eq(&needle, item))))
        }
        (BinOp::NotIn, needle, Value::Array(arr)) => {
            Ok(Value::Bool(!arr.iter().any(|item| value_eq(&needle, item))))
        } // ── Fronteira ───────────────────────────────────────────────────
        (op, lhs, rhs) => Err(binary_mismatch(op, &lhs, &rhs)),
    }
}

/// **P706** — igualdade de valor usada por `in`/`not in` sobre `Array`.
/// Mesma coerção Int/Float do `BinOp::Eq` (medido: `1 in (1.0, 2.0)` → `true`
/// no vanilla); tudo o resto delega ao `PartialEq` derivado de `Value`
/// (recursivo — cobre array-de-arrays sem código extra).
/// **P818-h** — delega em [`values_eq`]: a coerção passa a propagar-se a
/// elementos aninhados (medido: `(1,) in ((1.0,), (2,))` → `true` no vanilla).
fn value_eq(a: &Value, b: &Value) -> bool {
    values_eq(a, b)
}

/// **P818** — igualdade da linguagem com coerção `Int ↔ Float` **recursiva**
/// (paridade vanilla: `Value::eq` é `ops::equal`, que coage, e
/// `Array`/`Dict` comparam elemento a elemento com ela — medido:
/// `(1,2) == (1.0,2.0)` → `true`, `(a: 1) == (a: 1.0)` → `true`).
/// Cobre ainda as igualdades mistas `Length ↔ Relative` (rel zero,
/// `ops.rs:458-460` — P818-f) e `Ratio ↔ Relative` (abs zero, mesma
/// tolerância do braço dedicado P785b) em posição aninhada, e `Content`
/// morfológico (P345) aninhado. O resto delega no `PartialEq` derivado.
pub(crate) fn values_eq(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Int(i), Value::Float(f)) | (Value::Float(f), Value::Int(i)) => {
            (*i as f64) == *f
        }
        (
            Value::Color(Color::Srgb { r: r0, g: g0, b: b0, a: a0 }),
            Value::Color(Color::Srgb { r: r1, g: g1, b: b1, a: a1 }),
        ) => {
            // Language equality rounds half away from zero; to-hex instead
            // uses ties-to-even. Preserve the stored components and Rust Eq.
            let channel = |x: &f32| (*x * 255.0).round() as u8;
            [r0, g0, b0, a0].map(channel) == [r1, g1, b1, a1].map(channel)
        }
        (Value::Array(x), Value::Array(y)) => {
            x.len() == y.len() && x.iter().zip(y.iter()).all(|(u, v)| values_eq(u, v))
        }
        (Value::Dict(x), Value::Dict(y)) => {
            x.len() == y.len()
                && x.iter().all(|(k, v)| y.get(k).is_some_and(|w| values_eq(v, w)))
        }
        (Value::Length(l), Value::Relative(r))
        | (Value::Relative(r), Value::Length(l)) => l == &r.abs && r.rel == 0.0,
        (Value::Ratio(rat), Value::Relative(rel))
        | (Value::Relative(rel), Value::Ratio(rat)) => {
            rel.abs.is_zero() && (rel.rel - rat.0).abs() < 1e-9
        }
        (Value::Content(x), Value::Content(y)) => content_language_eq(x, y),
        (Value::Content(x), Value::LocatedContent(y, _)) => {
            content_values_eq(x, None, y.content(), y.fields())
        }
        (Value::LocatedContent(x, _), Value::Content(y)) => {
            content_values_eq(x.content(), x.fields(), y, None)
        }
        (Value::LocatedContent(x, _), Value::LocatedContent(y, _)) => {
            content_values_eq(x.content(), x.fields(), y.content(), y.fields())
        }
        (Value::Selector(x), Value::Selector(y)) => selectors_eq(x, y),
        (Value::Counter(x), Value::Counter(y)) => counter_keys_eq(&x.key, &y.key),
        (a, b) => a == b,
    }
}

/// Comparison-only projection: Styled identity is its child, not its map.
/// Keep the public morphology projection and Rust equality untouched.
fn content_language_eq(a: &Content, b: &Content) -> bool {
    fn project(content: &Content) -> Content {
        content
            .morph_canon()
            .map_content(&mut |node| Ok(Some(comparison_root(node))))
            .expect("content equality: comparison projection is total")
    }
    project(a) == project(b)
}

/// Normalize only comparison roots, including a Located body's suppressed root.
/// This ephemeral value never leaves the language comparison operation.
fn comparison_root(content: &Content) -> Content {
    use crate::entities::content::{LocatedContent, RealizedContent, SourceContent};
    use std::sync::Arc;

    match content {
        Content::Source(e) => Content::Source(Arc::new(SourceContent {
            body: comparison_root(&e.body),
            span: e.span,
        })),
        Content::ShowRecipe(e) => {
            let mut projected = e.as_ref().clone();
            projected.body = comparison_root(&e.body);
            Content::ShowRecipe(Arc::new(projected))
        }
        Content::Located(e) => Content::Located(Arc::new(LocatedContent {
            body: comparison_root(&e.body),
            location: e.location,
        })),
        Content::Realized(e) => Content::Realized(Arc::new(RealizedContent {
            original: e.original.clone(),
            visual: comparison_root(&e.visual),
        })),
        Content::ContextBlock(e) => {
            let mut projected = e.as_ref().clone();
            // Context has no exposed language fields. Keep its nominal kind
            // and unevaluated closure; omit only the private comparison id.
            projected.id = 0;
            Content::ContextBlock(Arc::new(projected))
        }
        Content::Styled(styled) => {
            let mut body = styled.body.clone();
            while let Content::Styled(inner) = &body {
                body = inner.body.clone();
            }
            Content::styled(Box::new(body), crate::entities::style::Styles::new())
        }
        other => other.clone(),
    }
}

/// Whether the selector's composition tree uses the P1339 element carrier.
pub(crate) fn selector_contains_element(selector: &Selector) -> bool {
    match selector {
        Selector::NativeElement(_) | Selector::Element { .. } => true,
        Selector::And(items) | Selector::Or(items) => {
            items.iter().any(selector_contains_element)
        }
        Selector::Where { base, .. } => selector_contains_element(base),
        Selector::Within { base, ancestor } => {
            selector_contains_element(base) || selector_contains_element(ancestor)
        }
        Selector::Before { base, end, .. } => {
            selector_contains_element(base) || selector_contains_element(end)
        }
        Selector::After { base, start, .. } => {
            selector_contains_element(base) || selector_contains_element(start)
        }
        Selector::Kind(_)
        | Selector::Label(_)
        | Selector::Location(_)
        | Selector::Regex(_) => false,
    }
}

/// Language equality only refines compositions containing the new carrier.
/// Old selector pairs retain their historical structural comparison.
/// Figure kind filters have nominal constructor identity, independently of Func language equality.
pub(crate) fn figure_kind_values_eq(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Func(a), Value::Func(b)) => {
            crate::entities::elements::figure::FigureKind::Element(a.clone())
                == crate::entities::elements::figure::FigureKind::Element(b.clone())
        }
        (Value::Func(_), Value::Str(_)) | (Value::Str(_), Value::Func(_)) => false,
        (a, b) => values_eq(a, b),
    }
}

fn figure_function(function: &crate::entities::func::Func) -> bool {
    function.native_fn_addr().is_some_and(|call| {
        std::ptr::fn_addr_eq(
            call,
            crate::compiler::stdlib::native_figure as fn(_, _, _, _) -> _,
        )
    })
}

fn figure_selector(selector: &Selector) -> bool {
    match selector {
        Selector::Kind(kind) => {
            *kind == crate::entities::element_kind::ElementKind::Figure
        }
        Selector::NativeElement(function) | Selector::Element { function, .. } => {
            figure_function(function)
        }
        Selector::Where { base, .. }
        | Selector::Within { base, .. }
        | Selector::Before { base, .. }
        | Selector::After { base, .. } => figure_selector(base),
        Selector::And(selectors) | Selector::Or(selectors) => {
            selectors.iter().any(figure_selector)
        }
        Selector::Label(_) | Selector::Location(_) | Selector::Regex(_) => false,
    }
}

pub(crate) fn selectors_eq(a: &Selector, b: &Selector) -> bool {
    if !selector_contains_element(a) && !selector_contains_element(b) {
        return a == b;
    }
    match (a, b) {
        (Selector::NativeElement(a), Selector::NativeElement(b)) => {
            match (a.repr(), b.repr()) {
                (
                    crate::entities::func::FuncRepr::Native(a),
                    crate::entities::func::FuncRepr::Native(b),
                ) => std::ptr::fn_addr_eq(a.call, b.call),
                (
                    crate::entities::func::FuncRepr::Format(ak, a),
                    crate::entities::func::FuncRepr::Format(bk, b),
                ) => ak == bk && std::ptr::fn_addr_eq(a.call, b.call),
                (
                    crate::entities::func::FuncRepr::NativeWithEngine(a),
                    crate::entities::func::FuncRepr::NativeWithEngine(b),
                ) => std::ptr::fn_addr_eq(a.call, b.call),
                (
                    crate::entities::func::FuncRepr::Closure(_)
                    | crate::entities::func::FuncRepr::Native(_)
                    | crate::entities::func::FuncRepr::Format(_, _)
                    | crate::entities::func::FuncRepr::NativeWithEngine(_)
                    | crate::entities::func::FuncRepr::Element(_)
                    | crate::entities::func::FuncRepr::Plugin(_)
                    | crate::entities::func::FuncRepr::With(_),
                    crate::entities::func::FuncRepr::Closure(_)
                    | crate::entities::func::FuncRepr::Native(_)
                    | crate::entities::func::FuncRepr::Format(_, _)
                    | crate::entities::func::FuncRepr::NativeWithEngine(_)
                    | crate::entities::func::FuncRepr::Element(_)
                    | crate::entities::func::FuncRepr::Plugin(_)
                    | crate::entities::func::FuncRepr::With(_),
                ) => false,
            }
        }
        (
            Selector::Element { function: a, fields: af },
            Selector::Element { function: b, fields: bf },
        ) => {
            (if figure_function(a) || figure_function(b) {
                figure_function(a) && figure_function(b)
            } else {
                a == b
            }) && af.len() == bf.len()
                && af.iter().zip(bf.iter()).all(|((an, av), (bn, bv))| {
                    an == bn
                        && if an.as_str() == "kind"
                            && figure_function(a)
                            && figure_function(b)
                        {
                            figure_kind_values_eq(av, bv)
                        } else {
                            values_eq(av, bv)
                        }
                })
        }
        (Selector::And(a), Selector::And(b)) | (Selector::Or(a), Selector::Or(b)) => {
            a.len() == b.len() && a.iter().zip(b.iter()).all(|(a, b)| selectors_eq(a, b))
        }
        (
            Selector::Where { base: a, field: af, value: av },
            Selector::Where { base: b, field: bf, value: bv },
        ) => {
            selectors_eq(a, b)
                && af == bf
                && if af.as_str() == "kind" && figure_selector(a) && figure_selector(b) {
                    figure_kind_values_eq(av, bv)
                } else {
                    values_eq(av, bv)
                }
        }
        (
            Selector::Within { base: a, ancestor: aa },
            Selector::Within { base: b, ancestor: ba },
        ) => selectors_eq(a, b) && selectors_eq(aa, ba),
        (
            Selector::Before { base: a, end: ae, inclusive: ai },
            Selector::Before { base: b, end: be, inclusive: bi },
        ) => ai == bi && selectors_eq(a, b) && selectors_eq(ae, be),
        (
            Selector::After { base: a, start: as_, inclusive: ai },
            Selector::After { base: b, start: bs, inclusive: bi },
        ) => ai == bi && selectors_eq(a, b) && selectors_eq(as_, bs),
        (
            Selector::Kind(_)
            | Selector::NativeElement(_)
            | Selector::Element { .. }
            | Selector::Label(_)
            | Selector::Location(_)
            | Selector::And(_)
            | Selector::Or(_)
            | Selector::Regex(_)
            | Selector::Where { .. }
            | Selector::Within { .. }
            | Selector::Before { .. }
            | Selector::After { .. },
            Selector::Kind(_)
            | Selector::NativeElement(_)
            | Selector::Element { .. }
            | Selector::Label(_)
            | Selector::Location(_)
            | Selector::And(_)
            | Selector::Or(_)
            | Selector::Regex(_)
            | Selector::Where { .. }
            | Selector::Within { .. }
            | Selector::Before { .. }
            | Selector::After { .. },
        ) => false,
    }
}

/// The same relation identifies demanded filtered counters and their updates.
pub(crate) fn counter_keys_eq(a: &CounterKey, b: &CounterKey) -> bool {
    match (a, b) {
        (CounterKey::Selector(a), CounterKey::Selector(b)) => selectors_eq(a, b),
        _ => a == b,
    }
}

type ContentFields =
    indexmap::IndexMap<ecow::EcoString, Value, rustc_hash::FxBuildHasher>;

fn raw_content_fields(content: &crate::entities::content::Content) -> ContentFields {
    let projected = crate::compiler::eval::bindings::eval_content_method(
        content,
        "fields",
        crate::entities::args::Args::positional(Vec::new()),
        crate::entities::span::Span::detached(),
    );
    match projected {
        Ok(Value::Dict(fields)) => fields,
        _ => unreachable!("content.fields with no arguments always returns a dictionary"),
    }
}

fn content_values_eq(
    a: &crate::entities::content::Content,
    a_fields: Option<&ContentFields>,
    b: &crate::entities::content::Content,
    b_fields: Option<&ContentFields>,
) -> bool {
    if a_fields.is_none() && b_fields.is_none() {
        return content_language_eq(a, b);
    }
    if a.elem_name() != b.elem_name() {
        return false;
    }
    let figure_fields = crate::compiler::realize::figure::is_figure(a)
        && crate::compiler::realize::figure::is_figure(b);
    let a_raw;
    let b_raw;
    let a = match a_fields {
        Some(fields) => fields,
        None => {
            a_raw = raw_content_fields(a);
            &a_raw
        }
    };
    let b = match b_fields {
        Some(fields) => fields,
        None => {
            b_raw = raw_content_fields(b);
            &b_raw
        }
    };
    let count = |fields: &ContentFields| {
        fields.keys().filter(|key| key.as_str() != "label").count()
    };
    count(a) == count(b)
        && a.iter()
            .filter(|(key, _)| key.as_str() != "label")
            .all(|(key, value)| {
                b.get(key).is_some_and(|other| {
                    if figure_fields && key.as_str() == "kind" {
                        figure_kind_values_eq(value, other)
                    } else {
                        values_eq(value, other)
                    }
                })
            })
}

#[cfg(test)]
mod p1339_tests {
    use super::*;

    fn element(value: Value) -> Selector {
        Selector::Element {
            function: crate::entities::func::Func::native(
                "heading",
                crate::compiler::stdlib::native_heading,
            ),
            fields: [("level".into(), value)].into_iter().collect(),
        }
    }

    #[test]
    fn p1339_element_equality_is_recursive_and_counter_consistent() {
        let a = element(Value::Array(vec![Value::Int(1)]));
        let b = element(Value::Array(vec![Value::Float(1.0)]));
        assert!(selectors_eq(&a, &b));
        assert!(counter_keys_eq(
            &CounterKey::Selector(a.clone()),
            &CounterKey::Selector(b.clone())
        ));
        assert!(selectors_eq(
            &Selector::And(vec![a].into()),
            &Selector::And(vec![b].into())
        ));
        let nan = element(Value::Float(f64::NAN));
        assert!(!selectors_eq(&nan, &nan));
    }

    #[test]
    fn p1339_empty_element_is_not_legacy_kind_and_field_order_is_observable() {
        let mut a = element(Value::Int(1));
        let Selector::Element { function, fields } = &mut a else { unreachable!() };
        let empty = Selector::Element {
            function: function.clone(),
            fields: Default::default(),
        };
        assert!(!selectors_eq(
            &empty,
            &Selector::Kind(crate::entities::element_kind::ElementKind::Heading)
        ));
        fields.push(("outlined".into(), Value::Bool(true)));
        let mut b = a.clone();
        let Selector::Element { fields, .. } = &mut b else { unreachable!() };
        fields.make_mut().reverse();
        assert!(!selectors_eq(&a, &b));
        let legacy = |value| Selector::Where {
            base: Box::new(Selector::Kind(
                crate::entities::element_kind::ElementKind::Heading,
            )),
            field: "level".into(),
            value: Box::new(value),
        };
        assert!(!selectors_eq(&legacy(Value::Int(1)), &legacy(Value::Float(1.0))));
    }
}

#[cfg(test)]
mod color_bounds_tests {
    use super::*;
    use crate::entities::color::Color;

    #[test]
    fn rgb_language_equality_preserves_precision_and_rounds_channels() {
        let half = Value::Color(Color::srgb_f32(0.5, 0.0, 0.0, 1.0));
        let byte = Value::Color(Color::rgb(128, 0, 0));
        assert_ne!(half, byte, "structural equality remains exact");
        assert!(values_eq(&half, &byte));
        assert!(values_eq(
            &Value::Color(Color::srgb_f32(0.0, 0.0, 0.0, 0.5)),
            &Value::Color(Color::srgb_f32(0.0, 0.0, 0.0, 128.0 / 255.0)),
        ));
        let tie = Value::Color(Color::srgb_f32(90.5 / 255.0, 0.0, 0.0, 1.0));
        assert!(values_eq(&tie, &Value::Color(Color::rgb(91, 0, 0))));
        assert!(!values_eq(&tie, &Value::Color(Color::rgb(90, 0, 0))));
        assert!(!values_eq(&Value::Color(Color::srgb_f32(0.49, 0.0, 0.0, 1.0)), &byte));
        assert!(!values_eq(
            &Value::Color(Color::srgb_f32(0.0, 0.0, 0.0, 0.49)),
            &Value::Color(Color::srgb_f32(0.0, 0.0, 0.0, 128.0 / 255.0)),
        ));
        assert!(!values_eq(
            &Value::Color(Color::linear_rgb(0.5, 0.0, 0.0, 1.0)),
            &Value::Color(Color::linear_rgb(128.0 / 255.0, 0.0, 0.0, 1.0)),
        ));
        assert!(!values_eq(
            &Value::Color(Color::rgb(0, 0, 0)),
            &Value::Color(Color::luma(0.0))
        ));
    }

    #[test]
    fn rgb_language_equality_propagates_to_containers_and_membership() {
        let a = Value::Color(Color::srgb_f32(0.5, 0.0, 0.0, 1.0));
        let b = Value::Color(Color::rgb(128, 0, 0));
        assert_eq!(apply_binary(BinOp::Eq, a.clone(), b.clone()), Ok(Value::Bool(true)));
        assert_eq!(
            apply_binary(BinOp::Neq, a.clone(), b.clone()),
            Ok(Value::Bool(false))
        );
        assert_eq!(
            apply_binary(BinOp::In, a.clone(), Value::Array(vec![b.clone()])),
            Ok(Value::Bool(true))
        );
        assert_eq!(
            apply_binary(BinOp::NotIn, a.clone(), Value::Array(vec![b.clone()])),
            Ok(Value::Bool(false))
        );
        assert!(values_eq(
            &Value::Array(vec![a.clone()]),
            &Value::Array(vec![b.clone()])
        ));
        assert!(values_eq(
            &Value::Dict([("color".into(), a)].into_iter().collect()),
            &Value::Dict([("color".into(), b)].into_iter().collect()),
        ));
    }
}

#[cfg(test)]
mod styled_language_tests {
    use super::*;
    use crate::entities::content::Content;
    use crate::entities::layout_types::Length;
    use crate::entities::style::{Style, Styles};

    fn sized(text: &str, length: Length) -> Content {
        Content::styled(
            Box::new(Content::text(text)),
            Styles::from_iter([Style::Size(length)]),
        )
    }

    #[test]
    fn styled_language_sizes_and_fill_do_not_change_child_equality() {
        let a = sized("A", Length::em(2.0));
        for b in [
            sized("A", Length::ZERO),
            sized("A", Length::pt(24.0)),
            Content::styled(
                Box::new(Content::text("A")),
                Styles::from_iter([Style::Fill(Color::rgb(0, 0, 255))]),
            ),
        ] {
            assert_ne!(a, b, "Rust equality must remain structural");
            assert_ne!(
                a.morph_canon(),
                b.morph_canon(),
                "Do not change the global morphology projection"
            );
            assert!(values_eq(&Value::Content(a.clone()), &Value::Content(b)));
        }
    }

    #[test]
    fn styled_language_nested_constructors_coalesce_without_erasing_presence() {
        let inner = sized("A", Length::pt(3.0));
        let nested = Content::styled(
            Box::new(inner),
            Styles::from_iter([Style::Size(Length::em(2.0))]),
        );
        let one = sized("A", Length::pt(24.0));
        assert!(values_eq(&Value::Content(nested), &Value::Content(one)));
    }

    #[test]
    fn styled_language_child_projection_applies_inside_nominal_nodes_and_sequences() {
        let a = sized("A", Length::em(2.0));
        let b = sized("A", Length::pt(24.0));
        assert!(values_eq(
            &Value::Content(Content::strong(a.clone())),
            &Value::Content(Content::strong(b.clone()))
        ));
        let x = Content::sequence(vec![Content::text("X"), a]);
        let y = Content::sequence(vec![Content::text("X"), b]);
        assert!(values_eq(&Value::Content(x), &Value::Content(y)));
    }

    #[test]
    fn styled_language_containers_membership_and_complement_share_the_relation() {
        let a = Value::Content(sized("A", Length::em(2.0)));
        let b = Value::Content(sized("A", Length::pt(24.0)));
        assert_eq!(
            apply_binary(BinOp::Eq, a.clone(), b.clone()).unwrap(),
            Value::Bool(true)
        );
        assert_eq!(
            apply_binary(BinOp::Neq, a.clone(), b.clone()).unwrap(),
            Value::Bool(false)
        );
        assert!(values_eq(
            &Value::Array(vec![a.clone()]),
            &Value::Array(vec![b.clone()])
        ));
        let d1 = Value::Dict([("x".into(), a.clone())].into_iter().collect());
        let d2 = Value::Dict([("x".into(), b.clone())].into_iter().collect());
        assert!(values_eq(&d1, &d2));
        assert_eq!(
            apply_binary(BinOp::In, a.clone(), Value::Array(vec![b.clone()])).unwrap(),
            Value::Bool(true)
        );
        assert_eq!(
            apply_binary(BinOp::NotIn, a, Value::Array(vec![b])).unwrap(),
            Value::Bool(false)
        );
    }

    #[test]
    fn styled_language_preserves_wrapper_kind_body_and_plain_negatives() {
        let a = Value::Content(sized("A", Length::em(2.0)));
        for b in [
            Content::text("A"),
            sized("B", Length::em(2.0)),
            Content::strong(Content::text("A")),
            Content::emph(Content::text("A")),
        ] {
            assert!(!values_eq(&a, &Value::Content(b)));
        }
        assert!(!values_eq(
            &Value::Content(Content::strong(Content::text("A"))),
            &Value::Content(Content::emph(Content::text("A")))
        ));
    }

    #[test]
    fn styled_language_ignores_style_nan_but_preserves_float_nan_and_sources() {
        use crate::entities::{file_id::FileId, span::Span};
        use std::num::NonZeroU16;
        let origin =
            Span::from_range(FileId::from_raw(NonZeroU16::new(7).unwrap()), 2..3);
        let a = sized("A", Length::pt(f64::NAN)).with_source_span(origin);
        let b = sized("A", Length::pt(12.0));
        for _ in 0..2 {
            assert!(values_eq(&Value::Content(a.clone()), &Value::Content(b.clone())));
        }
        assert_eq!(a.source_span(), origin);
        assert!(!values_eq(&Value::Float(f64::NAN), &Value::Float(f64::NAN)));
    }
}

#[cfg(test)]
mod batch_c_selector_equality_tests {
    use super::*;
    use crate::compiler::stdlib::{native_heading, native_op, native_page};
    use crate::entities::func::Func;

    #[test]
    fn v16_c_selector_equality_preserves_bare_group_order_and_flags() {
        let native = Selector::NativeElement(Func::native("alias", native_heading));
        let empty = Selector::Element {
            function: Func::native("heading", native_heading),
            fields: Default::default(),
        };
        assert!(!selectors_eq(&native, &empty));
        assert!(!selectors_eq(&empty, &native));
        let before = |inclusive| Selector::Before {
            base: Box::new(native.clone()),
            end: Box::new(empty.clone()),
            inclusive,
        };
        assert!(selectors_eq(&before(true), &before(true)));
        assert!(!selectors_eq(&before(true), &before(false)));
        let left = Selector::Or(vec![native.clone(), empty.clone()].into());
        let right = Selector::Or(vec![empty, native].into());
        assert!(!selectors_eq(&left, &right));
    }

    #[test]
    fn v16_c_selector_equality_engine_native_identity_recurses_without_names() {
        let page = Selector::NativeElement(Func::native_with_engine("page", native_page));
        let alias =
            Selector::NativeElement(Func::native_with_engine("different", native_page));
        let homonym =
            Selector::NativeElement(Func::native_with_engine("page", native_op));
        let cross_abi = Selector::NativeElement(Func::native("page", native_heading));
        assert!(selectors_eq(&page, &alias));
        assert!(!selectors_eq(&page, &homonym));
        assert!(!selectors_eq(&page, &cross_abi));
        assert!(!selectors_eq(&cross_abi, &page));
        assert!(values_eq(
            &Value::Array(vec![Value::Selector(page.clone())]),
            &Value::Array(vec![Value::Selector(alias.clone())])
        ));
        assert!(counter_keys_eq(
            &CounterKey::Selector(page.clone()),
            &CounterKey::Selector(alias.clone())
        ));
        assert!(selectors_eq(
            &Selector::And(vec![page].into()),
            &Selector::And(vec![alias].into())
        ));
    }
}

#[cfg(test)]
mod batch_c_format_selector_tests {
    use super::*;
    use crate::entities::func::Func;
    #[test]
    fn v16_c_selector_format_identity_retains_nominal_witness_and_aliases() {
        let pdf = crate::compiler::stdlib::make_pdf_module(
            crate::entities::compiler_features::Features::empty(),
        );
        let html = crate::compiler::stdlib::make_html_module();
        let asset =
            Value::Func(Func::native("asset", crate::compiler::stdlib::native_asset));
        let module = crate::compiler::stdlib::make_format_module(&pdf, &html, &asset);
        let formats = ["svg", "png", "pdf", "html", "bundle"].map(|name| {
            let Some(Value::Func(function)) = module.scope().get(name) else {
                panic!("format factory")
            };
            function.clone()
        });
        let mut wrong = Vec::new();
        for (left_index, left) in formats.iter().enumerate() {
            for (right_index, right) in formats.iter().enumerate() {
                let actual = selectors_eq(
                    &Selector::NativeElement(left.clone()),
                    &Selector::NativeElement(right.clone()),
                );
                if actual != (left_index == right_index) {
                    wrong.push((left_index, right_index));
                }
            }
            let ordinary = Func::native(
                ["svg", "png", "pdf", "html", "bundle"][left_index],
                left.native_fn_addr().unwrap(),
            );
            assert_eq!(*left, ordinary, "public equality retains existing compatibility");
            if selectors_eq(
                &Selector::NativeElement(left.clone()),
                &Selector::NativeElement(ordinary.clone()),
            ) {
                wrong.push((left_index, 5));
            }
            if selectors_eq(
                &Selector::NativeElement(ordinary),
                &Selector::NativeElement(left.clone()),
            ) {
                wrong.push((5, left_index));
            }
        }
        assert!(wrong.is_empty(), "incorrect format/native identity pairs: {wrong:?}");
    }
}

#[cfg(test)]
mod figure_kind_filter_identity_tests {
    use super::*;
    use crate::entities::func::Func;
    fn selector(function: Func, kind: Value) -> Selector {
        Selector::Element {
            function,
            fields: vec![("kind".into(), kind)].into(),
        }
    }
    #[test]
    fn figure_kind_filter_equality_preserves_aliases_and_rejects_homonyms() {
        let a = selector(
            Func::native("figure", crate::compiler::stdlib::native_figure),
            Value::Func(Func::native("table", crate::compiler::stdlib::native_table)),
        );
        let alias = selector(
            Func::native("renamed", crate::compiler::stdlib::native_figure),
            Value::Func(Func::native(
                "other-name",
                crate::compiler::stdlib::native_table,
            )),
        );
        let homonym = selector(
            Func::native("figure", crate::compiler::stdlib::native_figure),
            Value::Func(Func::native("table", crate::compiler::stdlib::native_heading)),
        );
        let name = selector(
            Func::native("figure", crate::compiler::stdlib::native_figure),
            Value::Str("table".into()),
        );
        assert!(selectors_eq(&a, &alias));
        assert!(!selectors_eq(&a, &homonym));
        assert!(!selectors_eq(&a, &name));
        // Generic Func equality deliberately retains its existing relation.
        assert!(values_eq(
            &Value::Func(Func::native("same", crate::compiler::stdlib::native_table)),
            &Value::Func(Func::native("same", crate::compiler::stdlib::native_heading))
        ));
    }
}

#[cfg(test)]
mod batch_g_context_tests {
    use super::*;
    use crate::entities::content::{LocatedContent, RealizedContent, SourceContent};
    use crate::entities::elements::context_block::ContextBlockElem;
    use crate::entities::file_id::FileId;
    use crate::entities::func::Func;
    use crate::entities::html::HtmlFrameElem;
    use crate::entities::location::Location;
    use crate::entities::span::Span;
    use crate::entities::style::{Style, Styles};
    use crate::entities::value::IntrospectedContent;
    use std::num::NonZeroU16;
    use std::sync::Arc;

    fn context(id: u64, name: &'static str) -> Content {
        Content::ContextBlock(Arc::new(ContextBlockElem {
            id,
            closure: Func::native(name, |_, _, _, _| {
                panic!("comparison executed Context")
            }),
        }))
    }

    #[test]
    fn batch_g_context_language_is_opaque_and_preserves_original_identity() {
        let a = context(17, "first");
        let b = context(99, "second");
        assert!(values_eq(&Value::Content(a.clone()), &Value::Content(b.clone())));
        let none_a = Value::LocatedContent(
            IntrospectedContent::new(a.clone(), None),
            Location::from_raw(4),
        );
        let none_b = Value::LocatedContent(
            IntrospectedContent::new(b.clone(), None),
            Location::from_raw(8),
        );
        assert!(values_eq(&Value::Content(a.clone()), &none_b));
        assert!(values_eq(&none_a, &Value::Content(b.clone())));
        assert!(values_eq(&none_a, &none_b));
        assert_ne!(a, b, "mechanical identity remains owned by Context id");
        let Content::ContextBlock(first) = &a else { unreachable!() };
        let Content::ContextBlock(second) = &b else { unreachable!() };
        assert_eq!((first.id, second.id), (17, 99));
        assert!(!Arc::ptr_eq(&first.closure.0, &second.closure.0));
        let Content::ContextBlock(projected) = a.morph_canon() else {
            panic!("public morphology erased Context kind")
        };
        assert_eq!(projected.id, 17);
        assert!(Arc::ptr_eq(&projected.closure.0, &first.closure.0));
    }

    #[test]
    fn batch_g_context_language_reaches_suppressed_source_and_located_roots() {
        let span = Span::from_range(FileId::from_raw(NonZeroU16::new(1).unwrap()), 2..7);
        let a = context(17, "a");
        let b = context(99, "b");
        let left = [
            Content::Source(Arc::new(SourceContent { body: a.clone(), span })),
            Content::Located(Arc::new(LocatedContent {
                body: a.clone(),
                location: Location::from_raw(4),
            })),
            Content::Realized(Arc::new(RealizedContent {
                original: Content::text("unrelated"),
                visual: a.clone(),
            })),
        ];
        let right = [
            b.clone(),
            Content::Source(Arc::new(SourceContent { body: b.clone(), span })),
            Content::Located(Arc::new(LocatedContent {
                body: b.clone(),
                location: Location::from_raw(8),
            })),
        ];
        for x in &left {
            for y in &right {
                assert!(content_language_eq(x, y));
                assert!(content_language_eq(y, x));
            }
        }
        assert_eq!(left[0].source_span(), span);
        assert!(
            matches!(&left[1],Content::Located(e) if e.location==Location::from_raw(4) && matches!(&e.body,Content::ContextBlock(c) if c.id==17))
        );
    }

    #[test]
    fn batch_g_context_language_projects_nominal_container_children() {
        let a = context(17, "a");
        let b = context(99, "b");
        let wrap = |c: Content| {
            vec![
                Content::sequence(vec![Content::text("prefix"), c.clone()]),
                Content::styled(
                    c.clone(),
                    Styles::from_iter([Style::Size(
                        crate::entities::layout_types::Length::pt(12.0),
                    )]),
                ),
                Content::title(c.clone()),
                Content::HtmlFrame(Arc::new(HtmlFrameElem { body: c.clone() })),
                Content::strong(c.clone()),
                Content::emph(c),
            ]
        };
        for (x, y) in wrap(a).into_iter().zip(wrap(b)) {
            assert!(content_language_eq(&x, &y));
        }
    }

    #[test]
    fn batch_g_context_language_containers_and_operators_share_opaque_relation() {
        let a = Value::Content(context(17, "a"));
        let b = Value::Content(context(99, "b"));
        assert_eq!(apply_binary(BinOp::Eq, a.clone(), b.clone()), Ok(Value::Bool(true)));
        assert_eq!(
            apply_binary(BinOp::Neq, a.clone(), b.clone()),
            Ok(Value::Bool(false))
        );
        assert_eq!(
            apply_binary(BinOp::In, a.clone(), Value::Array(vec![b.clone()])),
            Ok(Value::Bool(true))
        );
        assert_eq!(
            apply_binary(BinOp::NotIn, a.clone(), Value::Array(vec![b.clone()])),
            Ok(Value::Bool(false))
        );
        let left =
            Value::Dict([("c".into(), Value::Array(vec![a]))].into_iter().collect());
        let right =
            Value::Dict([("c".into(), Value::Array(vec![b]))].into_iter().collect());
        assert!(values_eq(&left, &right));
    }

    #[test]
    fn batch_g_context_language_preserves_snapshot_field_sets_and_other_kinds() {
        let a = context(17, "a");
        let b = context(99, "b");
        let snapshot = |content: Content, n| {
            Value::LocatedContent(
                IntrospectedContent::new(
                    content,
                    Some([("meaning".into(), Value::Int(n))].into_iter().collect()),
                ),
                Location::from_raw(n as u128),
            )
        };
        assert!(values_eq(&snapshot(a.clone(), 1), &snapshot(b.clone(), 1)));
        assert!(!values_eq(&snapshot(a.clone(), 1), &snapshot(b.clone(), 2)));
        let labelled = |content: Content, label: &str, location| {
            Value::LocatedContent(
                IntrospectedContent::new(
                    content,
                    Some(
                        [
                            ("meaning".into(), Value::Int(1)),
                            (
                                "label".into(),
                                Value::Label(crate::entities::label::Label(label.into())),
                            ),
                        ]
                        .into_iter()
                        .collect(),
                    ),
                ),
                Location::from_raw(location),
            )
        };
        assert!(values_eq(
            &labelled(a.clone(), "left", 4),
            &labelled(b.clone(), "right", 8)
        ));
        let empty_some = Value::LocatedContent(
            IntrospectedContent::new(a.clone(), Some(Default::default())),
            Location::from_raw(11),
        );
        let no_snapshot = Value::LocatedContent(
            IntrospectedContent::new(b.clone(), None),
            Location::from_raw(19),
        );
        assert!(values_eq(&empty_some, &Value::Content(b.clone())));
        assert!(values_eq(&Value::Content(b.clone()), &empty_some));
        assert!(values_eq(&empty_some, &no_snapshot));
        assert!(values_eq(&no_snapshot, &empty_some));

        let extra = Value::LocatedContent(
            IntrospectedContent::new(
                b.clone(),
                Some(
                    [("meaning".into(), Value::Int(1)), ("extra".into(), Value::None)]
                        .into_iter()
                        .collect(),
                ),
            ),
            Location::from_raw(2),
        );
        assert!(!values_eq(&snapshot(a.clone(), 1), &extra));
        for other in [Content::Empty, Content::text("A"), Content::Space] {
            assert!(!content_language_eq(&a, &other));
            assert!(!content_language_eq(&other, &a));
        }
        assert!(!values_eq(&Value::Float(f64::NAN), &Value::Float(f64::NAN)));
    }
}
