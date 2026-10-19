//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/operators/join.md
//! @prompt-hash 69eb240c
//! @layer L1
//! @updated 2026-08-12
//!
//!
//! `join` — combinação sequencial dos valores de um code block e dos corpos
//! de `for`/`while` (paridade `ops::join`, `foundations/ops.rs:24-45`).
//! A acumulação vive nos consumidores (`eval/mod.rs`, `control_flow.rs`).

// **P1017** — `vanilla_type_name` é a canónica em
// `eval/operators/error_formatting.rs`; a cópia privada que aqui existia foi
// removida.
use crate::compiler::eval::operators::error_formatting::vanilla_type_name;
use crate::entities::args::Args;
use crate::entities::bytes::Bytes;
use crate::entities::content::Content;
use crate::entities::span::Span;
use crate::entities::value::Value;

/// Concatena somente os filhos dos operandos sequenciais imediatos.
/// Source conserva origem; wrappers e sequências já internas são valores opacos.
pub(super) fn concat_content(lhs: Content, rhs: Content) -> Content {
    let left_sequence =
        matches!(lhs.source_body(), Content::Empty | Content::Sequence(_));
    let right_sequence =
        matches!(rhs.source_body(), Content::Empty | Content::Sequence(_));
    if !left_sequence && !right_sequence {
        return Content::sequence(vec![lhs, rhs]);
    }
    let span = if left_sequence { lhs.source_span() } else { rhs.source_span() };
    let mut parts = Vec::new();
    if let Content::Sequence(children) = lhs.source_body() {
        parts.extend(children.iter().cloned());
    } else if !matches!(lhs.source_body(), Content::Empty) {
        parts.push(lhs);
    }
    if let Content::Sequence(children) = rhs.source_body() {
        parts.extend(children.iter().cloned());
    } else if !matches!(rhs.source_body(), Content::Empty) {
        parts.push(rhs);
    }
    let content = if parts.is_empty() {
        Content::Empty
    } else {
        // A soma com uma Sequence conserva essa identidade até com um filho.
        Content::Sequence(parts.into())
    };
    content.with_source_span(span)
}

/// **P728** — `join` de valores produzidos pelas expressões de um code
/// block (paridade vanilla `ops::join`, `foundations/ops.rs:24-45`).
/// `None` é identidade nos dois lados; texto/content/array/dict/args/bytes
/// concatenam ou fazem merge; qualquer outra combinação é erro de tipo
/// (`cannot join X with Y`, medido: `{ 1; 2 }` erra no vanilla).
pub(crate) fn join(lhs: Value, rhs: Value) -> Result<Value, String> {
    match (lhs, rhs) {
        (a, Value::None) => Ok(a),
        (Value::None, b) => Ok(b),
        (Value::Str(a), Value::Str(b)) => Ok(Value::Str(a + b.as_str())),
        (Value::Symbol(a), Value::Symbol(b)) => {
            Ok(Value::Str(format!("{}{}", a.value, b.value).into()))
        }
        (Value::Str(a), Value::Symbol(b)) => {
            Ok(Value::Str(format!("{}{}", a, b.value).into()))
        }
        (Value::Symbol(a), Value::Str(b)) => {
            Ok(Value::Str(format!("{}{}", a.value, b).into()))
        }
        (Value::Bytes(a), Value::Bytes(b)) => {
            let mut v = a.as_slice().to_vec();
            v.extend_from_slice(b.as_slice());
            Ok(Value::Bytes(Bytes::new(v)))
        }
        (Value::Content(a), Value::Content(b)) => {
            Ok(Value::Content(concat_content(a, b)))
        }
        (Value::Content(a), Value::Str(b)) => {
            Ok(Value::Content(concat_content(a, Content::text(b))))
        }
        (Value::Str(a), Value::Content(b)) => {
            Ok(Value::Content(concat_content(Content::text(a), b)))
        }
        (Value::Content(a), Value::Symbol(b)) => {
            Ok(Value::Content(concat_content(a, Content::text(b.value.to_string()))))
        }
        (Value::Symbol(a), Value::Content(b)) => {
            Ok(Value::Content(concat_content(Content::text(a.value.to_string()), b)))
        }
        (Value::Array(mut a), Value::Array(b)) => {
            a.extend(b);
            Ok(Value::Array(a))
        }
        (Value::Dict(mut a), Value::Dict(b)) => {
            a.extend(b);
            Ok(Value::Dict(a))
        }
        (Value::Args(a), Value::Args(b)) => {
            let mut left = a.occurrence_sequence();
            let right = b.occurrence_sequence();
            left.retain(|occurrence| {
                !occurrence.name.as_ref().is_some_and(|name| {
                    right.iter().any(|other| other.name.as_ref() == Some(name))
                })
            });
            left.extend(right);
            Ok(Value::Args(Args::from_occurrences(Span::detached(), left)))
        }
        (a, b) => Err(format!(
            "cannot join {} with {}",
            vanilla_type_name(&a),
            vanilla_type_name(&b)
        )),
    }
}

#[cfg(test)]
mod concat_tests {
    use super::*;
    use crate::entities::file_id::FileId;
    use std::num::NonZeroU16;
    use std::sync::Arc;

    fn content(value: Value) -> Content {
        let Value::Content(content) = value else { panic!("expected Content") };
        content
    }

    #[test]
    fn concat_sequence_operands_and_none_identity() {
        let left = Content::sequence(vec![Content::text("A"), Content::text("B")]);
        let right = Content::sequence(vec![Content::text("C"), Content::text("D")]);
        let result =
            content(join(Value::Content(left.clone()), Value::Content(right)).unwrap());
        let Content::Sequence(parts) = result.source_body() else {
            panic!("expected Sequence")
        };
        assert_eq!(parts.len(), 4);
        assert!(parts
            .iter()
            .all(|part| matches!(part.source_body(), Content::Text(_))));
        assert_eq!(result.plain_text(), "ABCD");
        let alias = content(join(Value::Content(left.clone()), Value::None).unwrap());
        let (Content::Sequence(old), Content::Sequence(same)) = (&left, &alias) else {
            panic!("expected Sequence")
        };
        assert!(Arc::ptr_eq(old, same));
    }

    #[test]
    fn concat_empty_content_is_not_empty_text() {
        for (left, right) in
            [(Content::Empty, Content::text("C")), (Content::text("C"), Content::Empty)]
        {
            let result =
                content(join(Value::Content(left), Value::Content(right)).unwrap());
            let Content::Sequence(parts) = result.source_body() else {
                panic!("sequence identity must survive")
            };
            assert_eq!(parts.len(), 1);
            assert_eq!(parts[0].plain_text(), "C");
        }
        let empty = content(
            join(Value::Content(Content::Empty), Value::Content(Content::Empty)).unwrap(),
        );
        assert!(matches!(empty.source_body(), Content::Empty));
        let result = content(
            join(Value::Content(Content::text("A")), Value::Content(Content::text("")))
                .unwrap(),
        );
        let Content::Sequence(parts) = result.source_body() else {
            panic!("expected Sequence")
        };
        assert_eq!(parts.len(), 2);
        assert!(matches!(parts[1].source_body(), Content::Text(text) if text.is_empty()));
    }

    #[test]
    fn concat_sources_aliases_and_opaque_wrappers() {
        let file = FileId::from_raw(NonZeroU16::new(1).unwrap());
        let left_span = Span::from_range(file, 1..2);
        let right_span = Span::from_range(file, 3..4);
        let nested = Content::sequence(vec![Content::text("A"), Content::text("B")]);
        let left = Content::sequence(vec![nested.clone(), Content::text("C")])
            .with_source_span(left_span);
        let right = Content::sequence(vec![
            Content::strong(Content::text("D")),
            Content::text("E"),
        ])
        .with_source_span(right_span);
        let alias = left.clone();
        let result = content(join(Value::Content(left), Value::Content(right)).unwrap());
        assert_eq!(result.source_span(), left_span);
        let Content::Sequence(parts) = result.source_body() else {
            panic!("expected Sequence")
        };
        assert_eq!(parts.len(), 4);
        assert_eq!(parts[0], nested);
        assert!(matches!(parts[2].source_body(), Content::Strong(_)));
        let Content::Sequence(old) = alias.source_body() else {
            panic!("expected Sequence")
        };
        assert_eq!(old.len(), 2);
        assert_eq!(alias.source_span(), left_span);
        let result = content(
            join(Value::Content(Content::text("Z")), Value::Content(alias)).unwrap(),
        );
        assert_eq!(result.source_span(), left_span);
    }

    #[test]
    fn concat_styled_sequence_remains_an_opaque_child() {
        let styled = Content::styled(
            Content::sequence(vec![Content::text("A"), Content::text("B")]),
            crate::entities::style::Styles::default(),
        );
        let result = content(
            join(Value::Content(styled.clone()), Value::Content(Content::text("C")))
                .unwrap(),
        );
        let Content::Sequence(parts) = result.source_body() else {
            panic!("expected Sequence")
        };
        assert_eq!(parts.len(), 2);
        assert_eq!(parts[0], styled);
        assert!(matches!(parts[0].source_body(), Content::Styled(_)));
    }

    #[test]
    fn concat_accumulation_is_flat() {
        let mut value = Value::None;
        for _ in 0..256 {
            value = join(value, Value::Content(Content::text("X"))).unwrap();
        }
        let result = content(value);
        let Content::Sequence(parts) = result.source_body() else {
            panic!("expected Sequence")
        };
        assert_eq!(parts.len(), 256);
        assert!(parts
            .iter()
            .all(|part| matches!(part.source_body(), Content::Text(_))));
    }
}
