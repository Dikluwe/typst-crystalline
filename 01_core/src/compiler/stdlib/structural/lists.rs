//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/structural/lists.md
//! @prompt-hash b7faf4de
//! @layer L1
//! @updated 2026-08-12
//!
//! Nativas de lista: `list`, `enum`, `terms`.
//!
//! Extraído de `stdlib/structural.rs` no Passo 1014 conforme ADR-0109
//! (atomização — forma B, free function no arquivo da unidade).

use crate::entities::file_id::FileId;

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
pub fn native_terms(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    if !args.items.is_empty() {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "terms() espera argumentos nomeados na forma `chave: descrição`".to_string(),
        )]);
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

// ── Passo 512 — Grid/Table HLine/VLine ──────────────────────────────────────

