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

// ── Passo 157B (ADR-0060 Fase 2 sub-passo 2) — table cell ───────────────────

/// Coage `Value` para `Option<usize>` per ADR-0064 Caso A.
///
/// `Value::Auto` ou `Value::None` → `None` (None ↔ Auto vanilla).
/// `Value::Int(n)` com `n >= min as i64` → `Some(n as usize)`.
/// Outros tipos ou `n < min` → erro hard com diagnóstico claro.
///
/// Helper privado P157B; param `min` permite reuso para `x`/`y`
/// (min=0; auto-placement) e `colspan`/`rowspan` (min=1; paridade
/// vanilla `NonZeroUsize`).
fn extract_usize_or_none_min(
    val: &Value,
    fn_name: &str,
    field: &str,
    min: usize,
) -> SourceResult<Option<usize>> {
    match val {
        Value::Auto => Ok(None),
        Value::None => Ok(None),
        Value::Int(n) => {
            if *n < min as i64 {
                Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!("{}({}:): valor {} < {} (mínimo)", fn_name, field, n, min),
                )])
            } else {
                Ok(Some(*n as usize))
            }
        }
        other => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("{}({}:) espera int ou auto, recebeu {}", fn_name, field, other.type_name()),
        )]),
    }
}

/// `table_cell(body, x: none, y: none, colspan: none, rowspan: none)` →
/// `Content::TableCell`.
///
/// **Segundo sub-passo Model Fase 2** (ADR-0060). Subset minimal
/// per diagnóstico P157B §1.
///
/// **Naming `table_cell` flat** (não `table.cell` vanilla) per
/// diagnóstico P157B §8: FieldAccess actual em cristalino não
/// suporta namespacing de funcs (`Value::Func.subname` não existe).
/// Divergência intencional documentada per ADR-0033.
///
/// **Atributos**:
/// - `body` posicional obrigatório (Content ou Str).
/// - `x: usize`/`auto`/`none` (named); ADR-0064 Caso A; `None` ↔
///   Auto auto-placement.
/// - `y` análogo.
/// - `colspan: usize`/`auto`/`none` (named); ADR-0064 Caso C;
///   `None` ↔ default 1; zero rejeitado (paridade `NonZeroUsize`).
/// - `rowspan` análogo.
///
/// **Atributos vanilla scope-out** (6 fields): `align`/`stroke`/
/// `fill`/`inset`/`breakable` per cell + internals (`kind`,
/// `is_repeated`).
///
/// **Limitação per ADR-0054 graded**: `x`/`y`/`colspan`/`rowspan`
/// armazenados mas **ignorados em layout** — algoritmo de placement
/// diferido em **DEBT-34e**. Layouter renderiza `body` no contexto
/// actual.
pub fn native_table_cell(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    let body = match args.items.first() {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::Str(s))     => Content::text(s.as_str()),
        Some(other) => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("table_cell() espera content ou string como primeiro argumento, recebeu {}", other.type_name()),
        )]),
        None => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "table_cell() exige body como argumento posicional".to_string(),
        )]),
    };

    let mut x:       Option<usize> = None;
    let mut y:       Option<usize> = None;
    let mut colspan: Option<usize> = None;
    let mut rowspan: Option<usize> = None;

    for (key, value) in args.named.iter() {
        match key.as_str() {
            // ADR-0064 Caso A — auto-placement; min=0.
            "x" => x = extract_usize_or_none_min(value, "table_cell", "x", 0)?,
            "y" => y = extract_usize_or_none_min(value, "table_cell", "y", 0)?,
            // ADR-0064 Caso C — span >= 1; min=1 (paridade NonZeroUsize).
            "colspan" => colspan = extract_usize_or_none_min(value, "table_cell", "colspan", 1)?,
            "rowspan" => rowspan = extract_usize_or_none_min(value, "table_cell", "rowspan", 1)?,
            other => return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("table_cell(): argumento nomeado inesperado '{}' (atributos avançados scope-out per ADR-0054 graded — refino futuro)", other),
            )]),
        }
    }

    Ok(Value::Content(Content::TableCell {
        body: Box::new(body),
        x, y, colspan, rowspan,
    }))
}

// ── Passo 157C (ADR-0060 Fase 2 sub-passo 3 — fecha table foundations) ─────

/// Coage `Value` para `bool` com default arbitrário per ADR-0064
/// Caso D (vanilla `bool` com default não-`false`; cristalino
/// usa `bool` directo com documentação explícita do default).
///
/// `Value::Bool(b)` → `b`.
/// `Value::None` ou ausência → `default`.
/// Outros tipos → erro hard com diagnóstico claro.
///
/// Helper privado P157C; param `default` permite reuso para
/// `repeat` (default true) e futuros bool fields com defaults
/// arbitrários (e.g. P158 figure-kinds).
///
/// Distinção vs `extract_weak` (em `stdlib/layout.rs`): este
/// helper é genérico no `field` e no `default`, enquanto
/// `extract_weak` é específico para key="weak" default=false.
/// Helpers separados preservam separação de domínios per
/// ADR-0037.
fn extract_bool_with_default(
    args: &Args,
    fn_name: &str,
    field: &str,
    default: bool,
) -> SourceResult<bool> {
    match args.named.get(field) {
        Some(Value::Bool(b)) => Ok(*b),
        Some(Value::None)    => Ok(default),
        Some(other) => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("{}({}:) espera bool, recebeu {}", fn_name, field, other.type_name()),
        )]),
        None => Ok(default),
    }
}

/// `table_header(body, repeat: true)` → `Content::TableHeader`.
///
/// **Terceiro e último sub-passo Model Fase 2** (ADR-0060 §"Decisão 1"
/// sub-passo 3 — fecha "table foundations" declarado).
/// Par simétrico com `native_table_footer`.
///
/// **Naming `table_header` flat** (não vanilla `table.header`)
/// per padrão P157B — FieldAccess actual cristalino não suporta
/// namespacing de funcs.
///
/// **Atributos**:
/// - `body` posicional obrigatório (Content ou Str).
/// - `repeat: bool` (named); ADR-0064 Caso D; default `true`
///   (paridade vanilla — divergência intencional do default Rust
///   `bool::default() == false`).
///
/// **Atributos vanilla scope-out** per ADR-0054 graded:
/// - `level: NonZeroU32` (hierarquia Header) — refino futuro.
/// - `repeat-rows: Smart<usize>` — refino futuro.
/// - Children variádicos estruturados (`Vec<TableItem>`) —
///   divergência aceite per ADR-0033 (cristalino usa `body`).
///
/// **Limitação per ADR-0054 graded**: `repeat` armazenado mas
/// **ignorado em layout** — algoritmo de repetição em page breaks
/// diferido em **DEBT-56** (refactor multi-region).
pub fn native_table_header(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    let body = match args.items.first() {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::Str(s))     => Content::text(s.as_str()),
        Some(other) => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("table_header() espera content ou string como primeiro argumento, recebeu {}", other.type_name()),
        )]),
        None => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "table_header() exige body como argumento posicional".to_string(),
        )]),
    };

    for key in args.named.keys() {
        if !["repeat"].contains(&key.as_str()) {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("table_header(): argumento nomeado inesperado '{}' (atributos avançados scope-out per ADR-0054 graded — refino futuro)", key),
            )]);
        }
    }

    let repeat = extract_bool_with_default(args, "table_header", "repeat", true)?;

    Ok(Value::Content(Content::TableHeader {
        body: Box::new(body),
        repeat,
    }))
}

/// `table_footer(body, repeat: true)` → `Content::TableFooter`.
///
/// Par simétrico com `native_table_header` (P157C). Mesma decisão
/// arquitectural Caso D + DEBT-56 + naming flat. Implementação
/// idêntica linha-a-linha excepto naming `header → footer`.
pub fn native_table_footer(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    let body = match args.items.first() {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::Str(s))     => Content::text(s.as_str()),
        Some(other) => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("table_footer() espera content ou string como primeiro argumento, recebeu {}", other.type_name()),
        )]),
        None => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "table_footer() exige body como argumento posicional".to_string(),
        )]),
    };

    for key in args.named.keys() {
        if !["repeat"].contains(&key.as_str()) {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("table_footer(): argumento nomeado inesperado '{}' (atributos avançados scope-out per ADR-0054 graded — refino futuro)", key),
            )]);
        }
    }

    let repeat = extract_bool_with_default(args, "table_footer", "repeat", true)?;

    Ok(Value::Content(Content::TableFooter {
        body: Box::new(body),
        repeat,
    }))
}

// ── `figure()` — migrada de eval.rs (Passo 64, DEBT-16) ─────────────────────

