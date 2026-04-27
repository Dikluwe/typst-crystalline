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

// ── Passo 159A (ADR-0060 Fase 2 — Bibliography + Cite par acoplado) ────────

/// Coage `Value::Array<Value::Dict>` para `Vec<BibEntry>` per
/// diagnóstico P159A §5 + P159D §5.1 + P159E §5.1 + P159G §5.1.
/// Cada Dict valida 4 fields obrigatórios (`key`/`author`/
/// `title`/`year`) + 4 opcionais comuns (`volume`/`pages`/
/// `journal`/`publisher` — Passo 159D) + 2 opcionais identificadores
/// digitais (`url`/`doi` — Passo 159E) + 6 opcionais restantes
/// comuns (`editor`/`series`/`note`/`isbn`/`location`/
/// `organization` — Passo 159G).
///
/// Helper privado P159A extendido em P159D + P159E + P159G; sem
/// promoção (N=1; política consistente N=2-3 mínima — `optional_str`
/// inline helper **N=12 cumulativos** largamente acima limiar).
///
/// **Validações hard**:
/// - Argumento posicional deve ser `Value::Array`.
/// - Cada elemento Array deve ser `Value::Dict`.
/// - Dict deve ter 4 keys obrigatórias.
/// - `key`/`author`/`title` devem ser `Value::Str`.
/// - `year` deve ser `Value::Int` >= 0.
/// - 12 opcionais (volume/pages/journal/publisher/url/doi/
///   editor/series/note/isbn/location/organization) — se
///   presentes, devem ser `Value::Str`; ausência aceite.
fn extract_bib_entries(val: Option<&Value>) -> SourceResult<Vec<crate::entities::bib_entry::BibEntry>> {
    use crate::entities::bib_entry::BibEntry;
    let arr = match val {
        Some(Value::Array(a)) => a,
        Some(other) => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("bibliography(entries:) espera array de dict, recebeu {}", other.type_name()),
        )]),
        None => return Ok(Vec::new()),  // entries vazio aceitável
    };

    let mut entries = Vec::with_capacity(arr.len());
    for (idx, val) in arr.iter().enumerate() {
        let dict = match val {
            Value::Dict(d) => d,
            other => return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("bibliography(entries: [{}]) espera dict, recebeu {}", idx, other.type_name()),
            )]),
        };

        let key = match dict.get("key") {
            Some(Value::Str(s)) => s.to_string(),
            Some(other) => return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("bibliography(entries: [{}].key) espera string, recebeu {}", idx, other.type_name()),
            )]),
            None => return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("bibliography(entries: [{}]) sem field obrigatório 'key'", idx),
            )]),
        };

        let author = match dict.get("author") {
            Some(Value::Str(s)) => s.to_string(),
            Some(other) => return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("bibliography(entries: [{}].author) espera string, recebeu {}", idx, other.type_name()),
            )]),
            None => return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("bibliography(entries: [{}]) sem field obrigatório 'author'", idx),
            )]),
        };

        let title = match dict.get("title") {
            Some(Value::Str(s)) => s.to_string(),
            Some(other) => return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("bibliography(entries: [{}].title) espera string, recebeu {}", idx, other.type_name()),
            )]),
            None => return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("bibliography(entries: [{}]) sem field obrigatório 'title'", idx),
            )]),
        };

        let year = match dict.get("year") {
            Some(Value::Int(n)) if *n >= 0 => *n as u32,
            Some(Value::Int(n)) => return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("bibliography(entries: [{}].year) espera int >= 0, recebeu {}", idx, n),
            )]),
            Some(other) => return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("bibliography(entries: [{}].year) espera int, recebeu {}", idx, other.type_name()),
            )]),
            None => return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("bibliography(entries: [{}]) sem field obrigatório 'year'", idx),
            )]),
        };

        // Passo 159D — fields opcionais. Helper inline para
        // parsing uniforme de field opcional Str.
        let optional_str = |field: &str| -> SourceResult<Option<String>> {
            match dict.get(field) {
                Some(Value::Str(s)) => Ok(Some(s.to_string())),
                Some(other) => Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!("bibliography(entries: [{}].{}) espera string, recebeu {}", idx, field, other.type_name()),
                )]),
                None => Ok(None),
            }
        };
        let volume    = optional_str("volume")?;
        let pages     = optional_str("pages")?;
        let journal   = optional_str("journal")?;
        let publisher = optional_str("publisher")?;
        // Passo 159E — par natural url/doi (reuso optional_str
        // inline helper; cumulativo N=2 P159D + N=2 P159E = N=4).
        let url       = optional_str("url")?;
        let doi       = optional_str("doi")?;
        // Passo 159G — 6 fields restantes comuns hayagriva
        // (cumulativo N=4 + N=2 + N=6 = N=12 usos do helper).
        let editor       = optional_str("editor")?;
        let series       = optional_str("series")?;
        let note         = optional_str("note")?;
        let isbn         = optional_str("isbn")?;
        let location     = optional_str("location")?;
        let organization = optional_str("organization")?;

        let mut entry = BibEntry::new(key, author, title, year);
        entry.volume       = volume;
        entry.pages        = pages;
        entry.journal      = journal;
        entry.publisher    = publisher;
        entry.url          = url;
        entry.doi          = doi;
        entry.editor       = editor;
        entry.series       = series;
        entry.note         = note;
        entry.isbn         = isbn;
        entry.location     = location;
        entry.organization = organization;
        entries.push(entry);
    }
    Ok(entries)
}

/// `bibliography(entries: array, title: ?)` → `Content::Bibliography`.
///
/// **Primeiro sub-passo Bibliography + Cite Model Fase 2** (par
/// acoplado com `cite`). Subset minimal per ADR-0054 graded
/// e diagnóstico P159A §1.
///
/// **Naming `bibliography` flat** (sem namespacing — paridade
/// padrão P157B).
///
/// **Atributos**:
/// - `entries`: `Array<Dict>` posicional ou named; cada Dict
///   tem keys obrigatórias `key`/`author`/`title`/`year`.
/// - `title: Content`/`Str` (named); ADR-0064 Caso A
///   (`Smart<Option<Content>>` vanilla → `Option<Box<Content>>`
///   cristalino); None ↔ ausente.
///
/// **Atributos vanilla scope-out** per ADR-0054 graded:
/// `sources` (parsing externo), `full`, `style` (CSL), `lang`,
/// `region`. Refinos futuros NÃO reservados per política P158.
///
/// **Limitação per ADR-0054 graded**: input cristalino é
/// **literal** `Vec<BibEntry>` — sem hayagriva, sem CSL parsing.
/// Layouter renderiza placeholder `"[{key}] {author}. {title}
/// ({year})."` per linha.
pub fn native_bibliography(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    // Validar named args.
    for key in args.named.keys() {
        if !["entries", "title"].contains(&key.as_str()) {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("bibliography(): argumento nomeado inesperado '{}' (atributos avançados scope-out per ADR-0054 graded — refino futuro NÃO reservado)", key),
            )]);
        }
    }

    // entries: pode ser posicional (primeiro item) ou named.
    let entries = if let Some(named) = args.named.get("entries") {
        extract_bib_entries(Some(named))?
    } else {
        // Tenta posicional.
        extract_bib_entries(args.items.first())?
    };

    // title: named opcional.
    let title = args.named.get("title").and_then(|v| match v {
        Value::Content(c) => Some(Box::new(c.clone())),
        Value::Str(s)     => Some(Box::new(Content::text(s.as_str()))),
        Value::None       => None,
        other             => Some(Box::new(Content::text(other.type_name()))),
    });

    Ok(Value::Content(Content::Bibliography { entries, title }))
}

/// `cite(key, supplement: ?, form: ?)` → `Content::Cite`.
///
/// Par com `bibliography` (acoplamento semântico vanilla
/// inseparável — cite referencia entries de bibliography).
///
/// **Naming `cite` flat** (paridade P157B).
///
/// **Atributos**:
/// - `key`: `Str` posicional obrigatório (referência a entry).
/// - `supplement: Content`/`Str` (named); ADR-0064 Caso A;
///   None ↔ ausente.
/// - `form: Str` (named); ADR-0064 Caso A (Passo 159C);
///   `"normal"`/`"prose"`/`"author"`/`"year"` ou `auto`/`none`/
///   ausente ↔ None (resolvido a Normal default em layout).
///
/// **Atributos vanilla scope-out** per ADR-0054 graded:
/// `style` (CSL override). Refinos futuros NÃO reservados.
///
/// **Sem validação cross-reference** `key ∈ Bibliography.keys`
/// — diferida per ADR-0017 Introspection runtime adiada.
/// `cite("inexistente")` produz placeholder `[inexistente]`
/// sem erro; forms `Prose`/`Author`/`Year` caem no fallback
/// `[key]` se key não encontrada.
pub fn native_cite(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    // key posicional obrigatório.
    let key = match args.items.first() {
        Some(Value::Str(s)) if !s.is_empty() => s.to_string(),
        Some(Value::Str(_)) => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "cite() key não pode ser vazia".to_string(),
        )]),
        Some(other) => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("cite() espera key como string, recebeu {}", other.type_name()),
        )]),
        None => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "cite() exige key como argumento posicional".to_string(),
        )]),
    };

    // Validar named args.
    for k in args.named.keys() {
        if !["supplement", "form"].contains(&k.as_str()) {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("cite(): argumento nomeado inesperado '{}' (atributos avançados scope-out per ADR-0054 graded — refino futuro NÃO reservado)", k),
            )]);
        }
    }

    let supplement = args.named.get("supplement").and_then(|v| match v {
        Value::Content(c) => Some(Box::new(c.clone())),
        Value::Str(s)     => Some(Box::new(Content::text(s.as_str()))),
        Value::None       => None,
        other             => Some(Box::new(Content::text(other.type_name()))),
    });

    let form = extract_citation_form(args.named.get("form"))?;

    Ok(Value::Content(Content::Cite { key, supplement, form }))
}

/// Helper privado P159C — parsing `Value::Str` para
/// `Option<CitationForm>`. Strict matching (case-sensitive);
/// `auto`/`none`/ausente → None (resolvido a Normal default em
/// layout). String inválida rejeitada com mensagem listando forms
/// válidas.
fn extract_citation_form(val: Option<&Value>) -> SourceResult<Option<crate::entities::citation_form::CitationForm>> {
    use crate::entities::citation_form::CitationForm;
    match val {
        None | Some(Value::Auto) | Some(Value::None) => Ok(None),
        Some(Value::Str(s)) => match s.as_str() {
            "normal" => Ok(Some(CitationForm::Normal)),
            "prose"  => Ok(Some(CitationForm::Prose)),
            "author" => Ok(Some(CitationForm::Author)),
            "year"   => Ok(Some(CitationForm::Year)),
            other    => Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("cite(): form '{}' inválido (válidos: normal, prose, author, year)", other),
            )]),
        },
        Some(other) => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("cite(): form espera string, recebeu {}", other.type_name()),
        )]),
    }
}

// ── `figure()` — migrada de eval.rs (Passo 64, DEBT-16) ─────────────────────

