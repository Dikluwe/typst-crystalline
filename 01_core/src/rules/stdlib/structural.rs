//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/stdlib.md
//! @prompt-hash f6cc2443
//! @layer L1
//! @updated 2026-04-23
//!
//! Funções nativas estruturais (strong, emph, raw, heading).
//! Extraído de `stdlib.rs` no Passo 96.5 conforme ADR-0037.

use ecow::EcoString;
use crate::entities::file_id::FileId;

use super::expect_no_named;

use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::span::Span;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::value::Value;
use crate::rules::eval::EvalContext;

// ── Sentinelas e construtores de nós estruturais (Passo 69) ─────────────────

/// `strong(body)` — emite `Content::Styled([Bold(true)], body)`
/// (Passo 101) ou serve como selector em show rules.
pub fn native_strong(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    let body = match args.items.first() {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::Str(s))     => Content::text(s.as_str()),
        Some(other) => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("strong() espera content ou string, recebeu {}", other.type_name()),
        )]),
        None => Content::Empty,
    };
    Ok(Value::Content(Content::strong(body)))
}

/// `emph(body)` — emite `Content::Styled([Italic(true)], body)`
/// (Passo 101) ou serve como selector em show rules.
pub fn native_emph(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    let body = match args.items.first() {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::Str(s))     => Content::text(s.as_str()),
        Some(other) => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("emph() espera content ou string, recebeu {}", other.type_name()),
        )]),
        None => Content::Empty,
    };
    Ok(Value::Content(Content::emph(body)))
}

/// `raw(text)` — cria `Content::Raw` ou serve como selector em show rules.
/// Aceita apenas string — não faz sentido semântico aceitar Content aqui.
pub fn native_raw(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    let text: EcoString = match args.items.first() {
        Some(Value::Str(s)) => s.clone(),
        Some(other) => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("raw() espera string, recebeu {}", other.type_name()),
        )]),
        None => EcoString::default(),
    };
    Ok(Value::Content(Content::Raw { text, lang: None, block: false }))
}

// ── `heading()` — sentinel para show rules (Passo 68, DEBT-21) ──────────────

/// Sentinel de `heading` como função — existe em scope para que show rules
/// do tipo `#show heading: it => ...` possam resolver o selector.
///
/// A criação real de headings usa a sintaxe de markup `= Título`.
/// Chamar `heading()` directamente retorna Err (DEBT-21).
pub fn native_heading(_ctx: &mut EvalContext, _args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    Err(vec![SourceDiagnostic::error(
        Span::detached(),
        "heading() como função directa não suportada; use a sintaxe de markup `= Título`"
            .to_string(),
    )])
}

// ── Passo 154B (ADR-0060 Fase 1) — terms + divider ──────────────────────────

/// `divider()` — emite `Content::Divider` (separador horizontal).
/// Não aceita argumentos.
pub fn native_divider(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    super::expect_no_named(&args.named)?;
    if !args.items.is_empty() {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "divider() não aceita argumentos posicionais".to_string(),
        )]);
    }
    Ok(Value::Content(Content::Divider))
}

/// `terms(named: descrição, ...)` — emite `Content::Terms` com pares
/// (chave nomeada, valor descrição). A ordem dos argumentos nomeados é
/// preservada (IndexMap). Aceita `Value::Content` ou `Value::Str` como
/// descrição. Posicionais não suportados (forma chave: descrição).
pub fn native_terms(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
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
            Value::Str(s)     => Content::text(s.as_str()),
            other => return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("terms(): descrição de '{}' deve ser content ou string, recebeu {}",
                    key, other.type_name()),
            )]),
        };
        items.push(Content::TermItem {
            term:        Box::new(term),
            description: Box::new(description),
        });
    }
    Ok(Value::Content(Content::Terms { items }))
}

// ── Passo 155 (ADR-0060 Fase 1, sub-passo 2) — quote ───────────────────────

/// `quote(body, attribution: ?, block: false, quotes: true)` — emite
/// `Content::Quote`. Body posicional obrigatório (content ou string);
/// outros argumentos via named.
pub fn native_quote(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    let body = match args.items.first() {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::Str(s))     => Content::text(s.as_str()),
        Some(other) => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("quote() espera content ou string, recebeu {}", other.type_name()),
        )]),
        None => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "quote() exige body como argumento posicional".to_string(),
        )]),
    };

    let mut attribution: Option<Content> = None;
    let mut block:       bool = false;
    let mut quotes:      bool = true;

    for (key, value) in args.named.iter() {
        match key.as_str() {
            "attribution" => {
                attribution = match value {
                    Value::Content(c) => Some(c.clone()),
                    Value::Str(s)     => Some(Content::text(s.as_str())),
                    Value::None       => None,
                    other => return Err(vec![SourceDiagnostic::error(
                        Span::detached(),
                        format!("quote(attribution:) espera content/string/none, recebeu {}", other.type_name()),
                    )]),
                };
            }
            "block" => match value {
                Value::Bool(b) => block = *b,
                other => return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!("quote(block:) espera bool, recebeu {}", other.type_name()),
                )]),
            },
            "quotes" => match value {
                Value::Bool(b) => quotes = *b,
                other => return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!("quote(quotes:) espera bool, recebeu {}", other.type_name()),
                )]),
            },
            other => return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("quote(): argumento nomeado inesperado '{}'", other),
            )]),
        }
    }

    Ok(Value::Content(Content::Quote {
        body:        Box::new(body),
        attribution: attribution.map(Box::new),
        block,
        quotes,
    }))
}

// ── Passo 157A (ADR-0060 Fase 2 sub-passo 1) — table minimal ────────────────

/// `table(columns?, rows?, ...children)` → `Content::Table`.
///
/// **Primeiro sub-passo Model Fase 2** (ADR-0060). Subset minimal
/// per diagnóstico P157A §3:
/// - `columns: Vec<TrackSizing>` (named); default `[Auto]` (cells
///   numa só coluna).
/// - `rows: Vec<TrackSizing>` (named); default `[Auto]`.
/// - `children: Vec<Content>` (variadic posicional).
///
/// Cells distribuídas via `idx % num_cols` (algoritmo `layout_grid`
/// reusado per ADR-0060 §"Decisão 4"; sem modificação de
/// `grid.rs`).
///
/// **Atributos vanilla scope-out** per ADR-0054 graded e diferidos
/// para passos seguintes:
/// - `gutter`/`column_gutter`/`row_gutter` (refino XS futuro).
/// - `inset`/`align`/`fill`/`stroke` (refino M após Block/Box pattern).
/// - TableCell estruturado (P157B).
/// - TableHeader/Footer (P157C).
/// - TableHLine/VLine (cosmetic — não-foundational).
///
/// Helper `extract_tracks` reusado de `stdlib/layout.rs` (N=2;
/// `pub(super)` per P157A — sibling-module access).
pub fn native_table(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    use crate::rules::stdlib::layout::extract_tracks;
    use crate::entities::layout_types::TrackSizing;

    for key in args.named.keys() {
        if !["columns", "rows"].contains(&key.as_str()) {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("argumento nomeado inesperado em table(): '{}' (atributos avançados scope-out per ADR-0054 graded — refino futuro P157B/C)", key),
            )]);
        }
    }
    let mut columns = extract_tracks(args.named.get("columns"));
    let mut rows    = extract_tracks(args.named.get("rows"));
    // Defaults — `columns`/`rows` omitido cai em `[Auto]` (paridade
    // com Grid em P83).
    if columns.is_empty() {
        columns = vec![TrackSizing::Auto];
    }
    if rows.is_empty() {
        rows = vec![TrackSizing::Auto];
    }
    // Children variádicos posicionais (Content ou Str).
    let mut children: Vec<Content> = Vec::with_capacity(args.items.len());
    for v in args.items.iter() {
        match v {
            Value::Content(c) => children.push(c.clone()),
            Value::Str(s)     => children.push(Content::text(s.as_str())),
            other => return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("table(): children devem ser content ou string, recebeu {}", other.type_name()),
            )]),
        }
    }
    Ok(Value::Content(Content::Table { columns, rows, children }))
}

// ── `figure()` — migrada de eval.rs (Passo 64, DEBT-16) ─────────────────────

