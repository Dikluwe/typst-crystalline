//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/structural/heading.md
//! @prompt-hash acf2125a
//! @layer L1
//! @updated 2026-08-13
//!
//! `heading` — cabeçalho e selector de show rules.
//!
//! Separado de `structural/sectioning.rs` em 2026-08-13: a fronteira anterior
//! estava sustentada por um cluster de co-mudança que não existia (artefacto
//! de atribuição da ferramenta). Fronteira nova medida no L0 deste nó.

use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::file_id::FileId;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;


// ── Sentinelas e construtores de nós estruturais (Passo 69) ─────────────────

/// `heading(level, body, numbering:?)` — cria um cabeçalho directamente.
/// Também serve como selector em show rules (`#show heading: it => ...`).
///
/// Argumentos:
/// - `level`: inteiro posicional 1..=6 (obrigatório).
/// - `body`: `Content` ou `Str` posicional (obrigatório).
/// - `numbering`: string named opcional; se presente, activa numeração com o
///   pattern indicado (ex.: `"1."`, `"I."`, `"(a)"`).
///
/// P451: antes era sentinel que retornava Err (DEBT-21); agora a função
/// directa é suportada para casos de uso programático.
pub fn native_heading(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    // P605 — suporte às formas vanilla:
    //   #heading[level](level como int, body trailing)
    //   #heading[body]    (body trailing, level default 1)
    //   #heading(1, [b])  (dois posicionais)
    //   #heading(level: 1, body: [b]) (named)
    // Quando o primeiro posicional é Content/Str, trata-se como body e level = 1.
    let (level, body) = match (args.items.first(), args.items.get(1)) {
        (Some(Value::Int(n)), Some(body_arg)) => {
            let level = *n as u8;
            if level == 0 || level > 6 {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!("heading(): level deve estar entre 1 e 6, recebeu {}", n),
                )]);
            }
            let body = match body_arg {
                Value::Content(c) => c.clone(),
                Value::Str(s) => Content::text(s.as_str()),
                other => {
                    return Err(vec![SourceDiagnostic::error(
                        Span::detached(),
                        format!("heading(): body espera content ou string, recebeu {}", other.type_name()),
                    )])
                }
            };
            (level, body)
        }
        (Some(Value::Int(n)), None) => {
            // Apenas level fornecido — falta body.
            let level = *n as u8;
            if level == 0 || level > 6 {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!("heading(): level deve estar entre 1 e 6, recebeu {}", n),
                )]);
            }
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "heading() exige body".to_string(),
            )]);
        }
        (Some(Value::Content(c)), _) => (1, c.clone()),
        (Some(Value::Str(s)), _) => (1, Content::text(s.as_str())),
        (Some(other), _) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("heading(): primeiro argumento deve ser int (level) ou content/string (body), recebeu {}", other.type_name()),
            )])
        }
        (None, _) => {
            // P829 — named `body:` (forma vanilla `#heading(level: 2, body: [H])`).
            match args.named.get("body") {
                Some(Value::Content(c)) => (1, c.clone()),
                Some(Value::Str(s)) => (1, Content::text(s.as_str())),
                _ => {
                    return Err(vec![SourceDiagnostic::error(
                        Span::detached(),
                        "heading() exige body".to_string(),
                    )])
                }
            }
        }
    };

    // P829 — named `level:` (forma vanilla `#heading(level: 2)[H]` — medido
    // b8: `at("level")` → 2, `has("level")` → true). O named sobrepõe-se ao
    // default 1 das formas body-only.
    let named_level = match args.named.get("level") {
        Some(Value::Int(n)) => {
            if *n < 1 || *n > 6 {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!("heading(): level deve estar entre 1 e 6, recebeu {}", n),
                )]);
            }
            Some(*n as u8)
        }
        Some(Value::None) | None => None,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("heading(level:): espera int, recebeu {}", other.type_name()),
            )])
        }
    };
    let level = named_level.unwrap_or(level);

    let numbering = match args.named.get("numbering") {
        Some(Value::Str(s)) => Some(s.clone()),
        Some(Value::None) | None => None,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "heading(numbering:): espera string, recebeu {}",
                    other.type_name()
                ),
            )])
        }
    };

    // P606 — `outlined` e `bookmarked` são flags separadas. `outlined`
    // controla o índice do documento (`#outline()`); `bookmarked` controla a
    // árvore `/Outlines` do PDF. Quando `bookmarked` não é definido (`None`),
    // o valor efectivo segue `outlined` (comportamento vanilla `auto`).
    let outlined = match args.named.get("outlined") {
        Some(Value::Bool(b)) => *b,
        Some(Value::None) | None => true,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("heading(outlined:): espera bool, recebeu {}", other.type_name()),
            )])
        }
    };
    let bookmarked = match args.named.get("bookmarked") {
        Some(Value::Bool(b)) => Some(*b),
        Some(Value::None) | None => None,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "heading(bookmarked:): espera bool, recebeu {}",
                    other.type_name()
                ),
            )])
        }
    };

    // P829 — regista os campos explicitamente assentes (paridade vanilla
    // `has`/`at`/`fields`): `level` conta como assente quando vem como
    // posicional int; `outlined` quando vem como named (mesmo `true`);
    // `bookmarked` já é auto-rastreado pelo `Option`.
    let mut set_fields = 0u8;
    if matches!(args.items.first(), Some(Value::Int(_))) || named_level.is_some() {
        set_fields |= crate::entities::elements::heading::HEADING_SET_LEVEL;
    }
    if args.named.contains_key("outlined") {
        set_fields |= crate::entities::elements::heading::HEADING_SET_OUTLINED;
    }

    let content = if let Some(pattern) = numbering {
        Content::heading_numbered_native(
            level,
            body,
            Some(pattern),
            outlined,
            bookmarked,
            set_fields,
        )
    } else {
        Content::heading_native(level, body, outlined, bookmarked, set_fields)
    };
    Ok(Value::Content(content))
}
