//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/model/document.md
//! @prompt 00_nucleo/prompts/rules/model/asset.md
//! @prompt 00_nucleo/prompts/rules/stdlib/structural.md
//! @prompt-hash 5defd191
//! @layer L1
//! @updated 2026-06-26
//!
//! Funções nativas estruturais (strong, emph, raw, heading).
//! Extraído de `stdlib.rs` no Passo 96.5 conforme ADR-0037.

use std::sync::Arc;

use crate::entities::file_id::FileId;
use ecow::EcoString;

use super::expect_no_named;

use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::elements::outline::OutlineTarget;
use crate::entities::elements::outline::OutlineElem;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;
use crate::rules::eval::EvalContext;

// ── Sentinelas e construtores de nós estruturais (Passo 69) ─────────────────

/// `strong(body)` — emite `Content::Styled([Bold(true)], body)`
/// (Passo 101) ou serve como selector em show rules.
pub fn native_strong(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    let body = match args.items.first() {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::Str(s)) => Content::text(s.as_str()),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "strong() espera content ou string, recebeu {}",
                    other.type_name()
                ),
            )])
        }
        None => Content::Empty,
    };
    Ok(Value::Content(Content::strong(body)))
}

/// `emph(body)` — emite `Content::Styled([Italic(true)], body)`
/// (Passo 101) ou serve como selector em show rules.
pub fn native_emph(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    let body = match args.items.first() {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::Str(s)) => Content::text(s.as_str()),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("emph() espera content ou string, recebeu {}", other.type_name()),
            )])
        }
        None => Content::Empty,
    };
    Ok(Value::Content(Content::emph(body)))
}

/// `raw(text)` — cria `Content::Raw` ou serve como selector em show rules.
/// Aceita apenas string — não faz sentido semântico aceitar Content aqui.
pub fn native_raw(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    let text: EcoString = match args.items.first() {
        Some(Value::Str(s)) => s.clone(),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("raw() espera string, recebeu {}", other.type_name()),
            )])
        }
        None => EcoString::default(),
    };
    Ok(Value::Content(Content::raw(text, None, false)))
}

// ── `heading()` — função nativa + selector para show rules (P451) ───────────

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
    let level = match args.items.first() {
        Some(Value::Int(n)) => {
            let level = *n as u8;
            if level == 0 || level > 6 {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!("heading(): level deve estar entre 1 e 6, recebeu {}", n),
                )]);
            }
            level
        }
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("heading(): level espera int, recebeu {}", other.type_name()),
            )])
        }
        None => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "heading() exige level como primeiro argumento posicional".to_string(),
            )])
        }
    };

    let body = match args.items.get(1) {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::Str(s)) => Content::text(s.as_str()),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("heading(): body espera content ou string, recebeu {}", other.type_name()),
            )])
        }
        None => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "heading() exige body como segundo argumento posicional".to_string(),
            )])
        }
    };

    let numbering = match args.named.get("numbering") {
        Some(Value::Str(s)) => Some(s.clone()),
        Some(Value::None) | None => None,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("heading(numbering:): espera string, recebeu {}", other.type_name()),
            )])
        }
    };

    let content = if let Some(pattern) = numbering {
        Content::heading_numbered_with_pattern(level, body, Some(pattern))
    } else {
        Content::heading(level, body)
    };
    Ok(Value::Content(content))
}

/// `outline(title: content?, depth: int?, indent: bool?)` — emite
/// `Content::Outline(OutlineElem { title, depth, indent })`.
///
/// P457: parâmetros settable do vanilla. Defaults: title=None (renderiza
/// "Índice" no layout), depth=3, indent=true.
pub fn native_outline(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    // Título: named `title` tem prioridade; fallback para primeiro argumento
    // posicional (vanilla aceita `outline([Título])`).
    let title_named = args.named.get("title").and_then(|v| match v {
        Value::Content(c) => Some(Ok(Some(c.clone()))),
        Value::Str(s) => Some(Ok(Some(Content::text(s.as_str())))),
        Value::None => None,
        other => Some(Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!(
                "outline(title:): espera content ou string, recebeu {}",
                other.type_name()
            ),
        )])),
    });

    let title_positional = if args.items.is_empty() {
        None
    } else {
        match args.items.first() {
            Some(Value::Content(c)) => Some(Ok(Some(c.clone()))),
            Some(Value::Str(s)) => Some(Ok(Some(Content::text(s.as_str())))),
            Some(other) => Some(Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "outline(): título espera content ou string, recebeu {}",
                    other.type_name()
                ),
            )])),
            None => None,
        }
    };

    let title = match (title_named, title_positional) {
        (Some(Ok(Some(_))), Some(Ok(Some(_)))) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "outline(): não pode usar título posicional e named `title` simultaneamente".to_string(),
            )])
        }
        (Some(Ok(t)), _) => t,
        (Some(Err(e)), _) => return Err(e),
        (None, Some(Ok(t))) => t,
        (None, Some(Err(e))) => return Err(e),
        (None, None) => None,
    };

    let depth = match args.named.get("depth") {
        Some(Value::Int(n)) => {
            let d = *n as usize;
            if d == 0 {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    "outline(depth:): depth deve ser >= 1".to_string(),
                )]);
            }
            d
        }
        Some(Value::None) | None => 3,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("outline(depth:): espera int, recebeu {}", other.type_name()),
            )])
        }
    };

    let indent = match args.named.get("indent") {
        Some(Value::Bool(b)) => *b,
        Some(Value::None) | None => true,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("outline(indent:): espera bool, recebeu {}", other.type_name()),
            )])
        }
    };

    // **P472** — argumento `target:` opcional: "headings" | "figures" | "tables"
    let target = match args.named.get("target") {
        Some(Value::Str(s)) => match s.as_str() {
            "headings" | "heading" => OutlineTarget::Headings,
            "figures"  | "figure"  => OutlineTarget::Figures,
            "tables"   | "table"   => OutlineTarget::Tables,
            other => return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("outline(target:): valor \"{}\" desconhecido; esperado \"headings\", \"figures\" ou \"tables\"", other),
            )]),
        },
        Some(Value::None) | None => OutlineTarget::Headings,
        Some(other) => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("outline(target:): espera string, recebeu {}", other.type_name()),
        )]),
    };

    Ok(Value::Content(Content::Outline(std::sync::Arc::new(
        OutlineElem::with_target(title, depth, indent, target)
    ))))
}

/// **P472** — `lof(title:?)` — emite `Content::Outline` com `target = Figures`.
/// Alias de `outline(target: "figures")`. Title opcional.
pub fn native_lof(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let title = match args.named.get("title") {
        Some(Value::Content(c)) => Some(c.clone()),
        Some(Value::Str(s)) => Some(Content::text(s.as_str())),
        Some(Value::None) | None => None,
        Some(other) => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("lof(title:): espera content ou string, recebeu {}", other.type_name()),
        )]),
    };
    Ok(Value::Content(Content::lof(title)))
}

/// **P472** — `lot(title:?)` — emite `Content::Outline` com `target = Tables`.
/// Alias de `outline(target: "tables")`. Title opcional.
pub fn native_lot(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let title = match args.named.get("title") {
        Some(Value::Content(c)) => Some(c.clone()),
        Some(Value::Str(s)) => Some(Content::text(s.as_str())),
        Some(Value::None) | None => None,
        Some(other) => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("lot(title:): espera content ou string, recebeu {}", other.type_name()),
        )]),
    };
    Ok(Value::Content(Content::lot(title)))
}

// ── Passo 154B (ADR-0060 Fase 1) — terms + divider ──────────────────────────

/// `divider()` — emite `Content::Divider` (separador horizontal).
/// Não aceita argumentos.
pub fn native_divider(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    super::expect_no_named(&args.named)?;
    if !args.items.is_empty() {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "divider() não aceita argumentos posicionais".to_string(),
        )]);
    }
    Ok(Value::Content(Content::divider()))
}

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

/// `quote(body, attribution: ?, block: false, quotes: true)` — emite
/// `Content::Quote`. Body posicional obrigatório (content ou string);
/// outros argumentos via named.
pub fn native_quote(
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
                    "quote() espera content ou string, recebeu {}",
                    other.type_name()
                ),
            )])
        }
        None => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "quote() exige body como argumento posicional".to_string(),
            )])
        }
    };

    let mut attribution: Option<Content> = None;
    let mut block: bool = false;
    let mut quotes: bool = true;

    for (key, value) in args.named.iter() {
        match key.as_str() {
            "attribution" => {
                attribution = match value {
                    Value::Content(c) => Some(c.clone()),
                    Value::Str(s) => Some(Content::text(s.as_str())),
                    Value::None => None,
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            Span::detached(),
                            format!(
                            "quote(attribution:) espera content/string/none, recebeu {}",
                            other.type_name()
                        ),
                        )])
                    }
                };
            }
            "block" => match value {
                Value::Bool(b) => block = *b,
                other => {
                    return Err(vec![SourceDiagnostic::error(
                        Span::detached(),
                        format!(
                            "quote(block:) espera bool, recebeu {}",
                            other.type_name()
                        ),
                    )])
                }
            },
            "quotes" => match value {
                Value::Bool(b) => quotes = *b,
                other => {
                    return Err(vec![SourceDiagnostic::error(
                        Span::detached(),
                        format!(
                            "quote(quotes:) espera bool, recebeu {}",
                            other.type_name()
                        ),
                    )])
                }
            },
            other => {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!("quote(): argumento nomeado inesperado '{}'", other),
                )])
            }
        }
    }

    Ok(Value::Content(Content::quote(body, attribution, block, quotes)))
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
pub fn native_table(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    use crate::entities::layout_types::TrackSizing;
    use crate::rules::stdlib::layout::{extract_stroke, extract_tracks};

    for key in args.named.keys() {
        // P227 + P228 — accept stroke + fill (paridade native_grid).
        // P459 — accept caption para numeração automática.
        if !["columns", "rows", "stroke", "fill", "caption"].contains(&key.as_str()) {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("argumento nomeado inesperado em table(): '{}' (atributos avançados scope-out per ADR-0054 graded — refino futuro)", key),
            )]);
        }
    }
    let mut columns = extract_tracks(args.named.get("columns"));
    let mut rows = extract_tracks(args.named.get("rows"));
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
            Value::Str(s) => children.push(Content::text(s.as_str())),
            other => {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!(
                        "table(): children devem ser content ou string, recebeu {}",
                        other.type_name()
                    ),
                )])
            }
        }
    }
    // P227 — extract stroke (paridade Grid via extract_stroke shorthand).
    let stroke = match args.named.get("stroke") {
        Some(val) => Some(extract_stroke(val, "table", "stroke")?),
        None => None,
    };
    // P228 — extract fill (Opção α: apenas Value::Color).
    let fill = match args.named.get("fill") {
        Some(Value::Color(c)) => Some(*c),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("table(fill): espera Color, recebeu {}", other.type_name()),
            )])
        }
        None => None,
    };
    // P459 — caption opcional (Content ou Str); None se omitido.
    let caption = match args.named.get("caption") {
        Some(Value::Content(c)) => Some(c.clone()),
        Some(Value::Str(s)) => Some(Content::text(s.as_str())),
        Some(Value::None) | None => None,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("table(caption): espera content ou string, recebeu {}", other.type_name()),
            )])
        }
    };
    Ok(Value::Content(Content::Table(std::sync::Arc::new(
        crate::entities::elements::table::TableElem {
            columns,
            rows,
            children,
            stroke,
            fill,
            caption,
        },
    ))))
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
            format!(
                "{}({}:) espera int ou auto, recebeu {}",
                fn_name,
                field,
                other.type_name()
            ),
        )]),
    }
}

/// P235 — extrai Align2D de Value individual (Align directo, ou Str
/// parsed via Align2D::from_string). Helper privado paralelo a
/// `extract_alignment(args)` em stdlib/layout.rs mas aceita `&Value`
/// (named arg single value).
fn extract_align_value(
    val: &Value,
    fn_name: &str,
    field: &str,
) -> SourceResult<crate::entities::layout_types::Align2D> {
    match val {
        Value::Align(a) => Ok(*a),
        Value::Str(s) => {
            Ok(crate::entities::layout_types::Align2D::from_string(s.as_str()))
        }
        other => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!(
                "{}({}:) espera alignment ou string, recebeu {}",
                fn_name,
                field,
                other.type_name()
            ),
        )]),
    }
}

/// P235 — extrai `Sides<Length>` de Value individual. Aceita `Length`
/// uniforme (paridade P156H Block.inset usage). Parsing complexo
/// (sides explícitos por axis) candidato refino futuro.
fn extract_inset_value(
    val: &Value,
    fn_name: &str,
    field: &str,
) -> SourceResult<crate::entities::sides::Sides<crate::entities::layout_types::Length>> {
    use crate::entities::layout_types::Length;
    use crate::entities::sides::Sides;
    match val {
        Value::Length(l) => Ok(Sides::uniform(*l)),
        Value::Float(f)  => Ok(Sides::uniform(Length::pt(*f))),
        Value::Int(n)    => Ok(Sides::uniform(Length::pt(*n as f64))),
        other => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("{}({}:) espera length uniforme, recebeu {} (sides explícitos refino futuro)",
                fn_name, field, other.type_name()),
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
pub fn native_table_cell(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
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

    let mut x: Option<usize> = None;
    let mut y: Option<usize> = None;
    let mut colspan: Option<usize> = None;
    let mut rowspan: Option<usize> = None;

    let mut stroke: Option<crate::entities::geometry::Stroke> = None;
    let mut fill: Option<crate::entities::layout_types::Color> = None;
    // P235 — 3 named args algorítmicos paralelo GridCell.
    let mut align: Option<crate::entities::layout_types::Align2D> = None;
    let mut inset: Option<
        crate::entities::sides::Sides<crate::entities::layout_types::Length>,
    > = None;
    let mut breakable: Option<bool> = None;
    for (key, value) in args.named.iter() {
        match key.as_str() {
            // ADR-0064 Caso A — auto-placement; min=0.
            "x" => x = extract_usize_or_none_min(value, "table_cell", "x", 0)?,
            "y" => y = extract_usize_or_none_min(value, "table_cell", "y", 0)?,
            // ADR-0064 Caso C — span >= 1; min=1 (paridade NonZeroUsize).
            "colspan" => colspan = extract_usize_or_none_min(value, "table_cell", "colspan", 1)?,
            "rowspan" => rowspan = extract_usize_or_none_min(value, "table_cell", "rowspan", 1)?,
            // P230 — stroke/fill per-cell paridade GridCell (refino paralelo).
            "stroke" => stroke = Some(crate::rules::stdlib::layout::extract_stroke(value, "table_cell", "stroke")?),
            "fill" => match value {
                Value::Color(c) => fill = Some(*c),
                other => return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!("table_cell(fill): espera Color, recebeu {}", other.type_name()),
                )]),
            },
            // P235 — algorítmicos per-cell paralelo GridCell.
            "align" => align = Some(extract_align_value(value, "table_cell", "align")?),
            "inset" => inset = Some(extract_inset_value(value, "table_cell", "inset")?),
            "breakable" => match value {
                Value::Bool(b) => breakable = Some(*b),
                other => return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!("table_cell(breakable): espera Bool, recebeu {}", other.type_name()),
                )]),
            },
            other => return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("table_cell(): argumento nomeado inesperado '{}' (atributos avançados scope-out per ADR-0054 graded — refino futuro)", other),
            )]),
        }
    }

    Ok(Value::Content(Content::TableCell(std::sync::Arc::new(
        crate::entities::elements::table_cell::TableCellElem {
            body,
            x,
            y,
            colspan,
            rowspan,
            stroke,
            fill,
            align,
            inset,
            breakable,
        },
    ))))
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
        Some(Value::None) => Ok(default),
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
pub fn native_table_header(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
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

    Ok(Value::Content(Content::table_header(body, repeat)))
}

/// `table_footer(body, repeat: true)` → `Content::TableFooter`.
///
/// Par simétrico com `native_table_header` (P157C). Mesma decisão
/// arquitectural Caso D + DEBT-56 + naming flat. Implementação
/// idêntica linha-a-linha excepto naming `header → footer`.
pub fn native_table_footer(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
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

    Ok(Value::Content(Content::table_footer(body, repeat)))
}

// ── Passo 224 (ADR-0061 Fase 4 candidata sub-3) — grid_cell + grid_header + grid_footer ──

/// `grid_cell(body, x?, y?, colspan?, rowspan?)` → `Content::GridCell`.
///
/// **P224.C — primeira aplicação de placement algorítmico real**
/// (fecha DEBT-34e via módulo `grid_placement.rs`).
///
/// Implementação idêntica linha-a-linha a `native_table_cell` (P157B) —
/// 5 fields paridade. `x`/`y`/`colspan`/`rowspan` agora **resolvidos
/// pelo Grid layouter** (não-ignorados; DEBT-34e fechada).
///
/// Atributos vanilla scope-out per ADR-0054 graded: `align`/`fill`/
/// `stroke`/`inset`/`breakable` per-cell — refinos futuros candidatos
/// NÃO-reservados.
pub fn native_grid_cell(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let body = match args.items.first() {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::Str(s))     => Content::text(s.as_str()),
        Some(other) => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("grid_cell() espera content ou string como primeiro argumento, recebeu {}", other.type_name()),
        )]),
        None => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "grid_cell() exige body como argumento posicional".to_string(),
        )]),
    };

    let mut x: Option<usize> = None;
    let mut y: Option<usize> = None;
    let mut colspan: Option<usize> = None;
    let mut rowspan: Option<usize> = None;
    let mut stroke: Option<crate::entities::geometry::Stroke> = None;
    let mut fill: Option<crate::entities::layout_types::Color> = None;
    // P235 — 3 named args algorítmicos.
    let mut align: Option<crate::entities::layout_types::Align2D> = None;
    let mut inset: Option<
        crate::entities::sides::Sides<crate::entities::layout_types::Length>,
    > = None;
    let mut breakable: Option<bool> = None;

    for (key, value) in args.named.iter() {
        match key.as_str() {
            "x" => x = extract_usize_or_none_min(value, "grid_cell", "x", 0)?,
            "y" => y = extract_usize_or_none_min(value, "grid_cell", "y", 0)?,
            "colspan" => colspan = extract_usize_or_none_min(value, "grid_cell", "colspan", 1)?,
            "rowspan" => rowspan = extract_usize_or_none_min(value, "grid_cell", "rowspan", 1)?,
            // P230 — stroke/fill per-cell (override Grid-level via .or()).
            // Reuso `extract_stroke` helper P227 (N=1 → 2 cumulativo).
            "stroke" => stroke = Some(crate::rules::stdlib::layout::extract_stroke(value, "grid_cell", "stroke")?),
            "fill" => match value {
                Value::Color(c) => fill = Some(*c),
                other => return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!("grid_cell(fill): espera Color, recebeu {}", other.type_name()),
                )]),
            },
            // P235 — align/inset/breakable per-cell (override Grid-level
            // via .or() resolution; pattern N=2 → 3 cumulativo atinge
            // limiar formalização).
            "align" => align = Some(extract_align_value(value, "grid_cell", "align")?),
            "inset" => inset = Some(extract_inset_value(value, "grid_cell", "inset")?),
            "breakable" => match value {
                Value::Bool(b) => breakable = Some(*b),
                other => return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!("grid_cell(breakable): espera Bool, recebeu {}", other.type_name()),
                )]),
            },
            other => return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("grid_cell(): argumento nomeado inesperado '{}' (atributos avançados scope-out per ADR-0054 graded — refino futuro)", other),
            )]),
        }
    }

    Ok(Value::Content(Content::GridCell(std::sync::Arc::new(
        crate::entities::elements::grid_cell::GridCellElem {
            body,
            x,
            y,
            colspan,
            rowspan,
            stroke,
            fill,
            align,
            inset,
            breakable,
        },
    ))))
}

/// `grid_header(body, repeat: true)` → `Content::GridHeader`.
///
/// **P224.B** — paridade absoluta com `native_table_header` (P157C);
/// implementação literal excepto naming `table_header → grid_header` +
/// variant `TableHeader → GridHeader`.
///
/// `repeat: bool` ADR-0064 Caso D; default `true` (paridade vanilla).
/// Semantic real de repetição em page breaks adiada per ADR-0054 graded
/// (paridade P157C; pattern N=5 cumulativo "Field armazenado semantic
/// adiada" P156D/E/G/P223/P224).
pub fn native_grid_header(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let body = match args.items.first() {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::Str(s))     => Content::text(s.as_str()),
        Some(other) => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("grid_header() espera content ou string como primeiro argumento, recebeu {}", other.type_name()),
        )]),
        None => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "grid_header() exige body como argumento posicional".to_string(),
        )]),
    };

    for key in args.named.keys() {
        if !["repeat"].contains(&key.as_str()) {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("grid_header(): argumento nomeado inesperado '{}' (atributos avançados scope-out per ADR-0054 graded — refino futuro)", key),
            )]);
        }
    }

    let repeat = extract_bool_with_default(args, "grid_header", "repeat", true)?;

    Ok(Value::Content(Content::grid_header(body, repeat)))
}

/// `grid_footer(body, repeat: true)` → `Content::GridFooter`.
///
/// **P224.B** — par simétrico com `native_grid_header`. Implementação
/// idêntica linha-a-linha excepto naming `header → footer`.
pub fn native_grid_footer(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let body = match args.items.first() {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::Str(s))     => Content::text(s.as_str()),
        Some(other) => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("grid_footer() espera content ou string como primeiro argumento, recebeu {}", other.type_name()),
        )]),
        None => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "grid_footer() exige body como argumento posicional".to_string(),
        )]),
    };

    for key in args.named.keys() {
        if !["repeat"].contains(&key.as_str()) {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("grid_footer(): argumento nomeado inesperado '{}' (atributos avançados scope-out per ADR-0054 graded — refino futuro)", key),
            )]);
        }
    }

    let repeat = extract_bool_with_default(args, "grid_footer", "repeat", true)?;

    Ok(Value::Content(Content::grid_footer(body, repeat)))
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
fn extract_bib_entries(
    val: Option<&Value>,
) -> SourceResult<Vec<crate::entities::bib_entry::BibEntry>> {
    use crate::entities::bib_entry::BibEntry;
    let arr = match val {
        Some(Value::Array(a)) => a,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "bibliography(entries:) espera array de dict, recebeu {}",
                    other.type_name()
                ),
            )])
        }
        None => return Ok(Vec::new()), // entries vazio aceitável
    };

    let mut entries = Vec::with_capacity(arr.len());
    for (idx, val) in arr.iter().enumerate() {
        let dict = match val {
            Value::Dict(d) => d,
            other => {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!(
                        "bibliography(entries: [{}]) espera dict, recebeu {}",
                        idx,
                        other.type_name()
                    ),
                )])
            }
        };

        let key = match dict.get("key") {
            Some(Value::Str(s)) => s.to_string(),
            Some(other) => {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!(
                        "bibliography(entries: [{}].key) espera string, recebeu {}",
                        idx,
                        other.type_name()
                    ),
                )])
            }
            None => {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!(
                        "bibliography(entries: [{}]) sem field obrigatório 'key'",
                        idx
                    ),
                )])
            }
        };

        let author = match dict.get("author") {
            Some(Value::Str(s)) => s.to_string(),
            Some(other) => {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!(
                        "bibliography(entries: [{}].author) espera string, recebeu {}",
                        idx,
                        other.type_name()
                    ),
                )])
            }
            None => {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!(
                        "bibliography(entries: [{}]) sem field obrigatório 'author'",
                        idx
                    ),
                )])
            }
        };

        let title = match dict.get("title") {
            Some(Value::Str(s)) => s.to_string(),
            Some(other) => {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!(
                        "bibliography(entries: [{}].title) espera string, recebeu {}",
                        idx,
                        other.type_name()
                    ),
                )])
            }
            None => {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!(
                        "bibliography(entries: [{}]) sem field obrigatório 'title'",
                        idx
                    ),
                )])
            }
        };

        let year = match dict.get("year") {
            Some(Value::Int(n)) if *n >= 0 => *n as u32,
            Some(Value::Int(n)) => {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!(
                        "bibliography(entries: [{}].year) espera int >= 0, recebeu {}",
                        idx, n
                    ),
                )])
            }
            Some(other) => {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!(
                        "bibliography(entries: [{}].year) espera int, recebeu {}",
                        idx,
                        other.type_name()
                    ),
                )])
            }
            None => {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!(
                        "bibliography(entries: [{}]) sem field obrigatório 'year'",
                        idx
                    ),
                )])
            }
        };

        // Passo 159D — fields opcionais. Helper inline para
        // parsing uniforme de field opcional Str.
        let optional_str = |field: &str| -> SourceResult<Option<String>> {
            match dict.get(field) {
                Some(Value::Str(s)) => Ok(Some(s.to_string())),
                Some(other) => Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!(
                        "bibliography(entries: [{}].{}) espera string, recebeu {}",
                        idx,
                        field,
                        other.type_name()
                    ),
                )]),
                None => Ok(None),
            }
        };
        let volume = optional_str("volume")?;
        let pages = optional_str("pages")?;
        let journal = optional_str("journal")?;
        let publisher = optional_str("publisher")?;
        // Passo 159E — par natural url/doi (reuso optional_str
        // inline helper; cumulativo N=2 P159D + N=2 P159E = N=4).
        let url = optional_str("url")?;
        let doi = optional_str("doi")?;
        // Passo 159G — 6 fields restantes comuns hayagriva
        // (cumulativo N=4 + N=2 + N=6 = N=12 usos do helper).
        let editor = optional_str("editor")?;
        let series = optional_str("series")?;
        let note = optional_str("note")?;
        let isbn = optional_str("isbn")?;
        let location = optional_str("location")?;
        let organization = optional_str("organization")?;

        let mut entry = BibEntry::new(key, author, title, year);
        entry.volume = volume;
        entry.pages = pages;
        entry.journal = journal;
        entry.publisher = publisher;
        entry.url = url;
        entry.doi = doi;
        entry.editor = editor;
        entry.series = series;
        entry.note = note;
        entry.isbn = isbn;
        entry.location = location;
        entry.organization = organization;
        entries.push(entry);
    }
    Ok(entries)
}

/// `bibliography(entries: array, title: ?, style: ?, locale: ?)` → `Content::Bibliography`.
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
/// - `style: Str` (named); nome CSL built-in (ex: `"ieee"`, `"apa"`).
/// - `locale: Str` (named); locale override (ex: `"en-US"`, `"pt-PT"`).
///
/// **Atributos vanilla scope-out** per ADR-0054 graded:
/// `sources` (parsing externo), `full`, `lang`, `region`.
/// Refinos futuros NÃO reservados per política P158.
///
/// **P418** — input cristalino continua literal; quando `style` é
/// fornecido, o layout pode usar hayagriva/citationberg para CSL.
///
/// **P419** — input via path (`#bibliography("refs.bib")`) carrega
/// `.bib`/`.yaml`/`.json` de disco via `World::read_bytes` + hayagriva.
///
/// **P429 (DEBT-63)** — o style resolvido não fica no `BibliographyElem`;
/// é registado no `EvalContext` e transportado pelo `Module` até ao
/// `BibStore` do `TagIntrospector` no pipeline.
pub fn native_bibliography(
    ctx: &mut EvalContext,
    args: &Args,
    world: &dyn crate::contracts::world::World,
    current_file: FileId,
) -> SourceResult<Value> {
    // Validar named args.
    for key in args.named.keys() {
        if !["entries", "title", "style", "locale"].contains(&key.as_str()) {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("bibliography(): argumento nomeado inesperado '{}' (atributos avançados scope-out per ADR-0054 graded — refino futuro NÃO reservado)", key),
            )]);
        }
    }

    // P419: primeiro arg posicional pode ser path (Str) ou entries (Array/Dict).
    let (entries, path) = if let Some(named) = args.named.get("entries") {
        (extract_bib_entries(Some(named))?, None)
    } else if let Some(first) = args.items.first() {
        match first {
            Value::Str(s) => {
                let path = s.as_str();
                let loaded = crate::rules::eval::bibliography::load_bib_entries_from_path(
                    world, current_file, path,
                )?;
                (loaded, Some(s.clone()))
            }
            _ => (extract_bib_entries(Some(first))?, None),
        }
    } else {
        (Vec::new(), None)
    };

    // title: named opcional. P479 — default heading "Bibliography" quando não especificado.
    // Sem arg → heading L1 "Bibliography" (paridade vanilla; walk recursivo em e.title conta-o).
    // title: none → None (sem título). title: content/str → conteúdo do arg (sem wrapper).
    let title = match args.named.get("title") {
        Some(Value::Content(c)) => Some(c.clone()),
        Some(Value::Str(s))     => Some(Content::text(s.as_str())),
        Some(Value::None)       => None,
        Some(other)             => Some(Content::text(other.type_name())),
        None => Some(Content::heading(1, Content::text("Bibliography"))),
    };

    let style = args.named.get("style").and_then(|v| match v {
        Value::Str(s) => Some(s.clone()),
        Value::None => None,
        _ => None,
    });

    let locale = args.named.get("locale").and_then(|v| match v {
        Value::Str(s) => Some(s.clone()),
        Value::None => None,
        _ => None,
    });

    // P420/P429 — resolve style (built-in ou custom .csl) em eval time e
    // regista-o no EvalContext para transporte via Module → BibStore.
    let elem = crate::entities::elements::bibliography::BibliographyElem {
        entries,
        path,
        title,
        style,
        locale,
    };
    if let Some(s) = &elem.style {
        let resolved = Arc::new(crate::rules::eval::bibliography::resolve_style(
            world,
            current_file,
            s.as_str(),
        )?);
        ctx.register_bibliography_style(&elem, resolved);
    }

    Ok(Value::Content(Content::Bibliography(Arc::new(elem))))
}

/// `cite(key, supplement: ?, form: ?, style: ?)` → `Content::Cite`.
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
/// - `style: Str` (named); P468;
///   `"numeric"`/`"author-date"`/`"alphabetic"` ou `auto`/`none`/
///   ausente ↔ None (resolvido a Numeric default em layout fallback).
///
/// **Sem validação cross-reference** `key ∈ Bibliography.keys`
/// — diferida per ADR-0017 Introspection runtime adiada.
/// `cite("inexistente")` produz placeholder `[inexistente]`
/// sem erro; forms `Prose`/`Author`/`Year` caem no fallback
/// `[key]` se key não encontrada.
pub fn native_cite(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    // key posicional obrigatório.
    let key = match args.items.first() {
        Some(Value::Str(s)) if !s.is_empty() => s.to_string(),
        Some(Value::Str(_)) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "cite() key não pode ser vazia".to_string(),
            )])
        }
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("cite() espera key como string, recebeu {}", other.type_name()),
            )])
        }
        None => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "cite() exige key como argumento posicional".to_string(),
            )])
        }
    };

    // Validar named args.
    for k in args.named.keys() {
        if !["supplement", "form", "style"].contains(&k.as_str()) {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("cite(): argumento nomeado inesperado '{}' (atributos avançados scope-out per ADR-0054 graded — refino futuro NÃO reservado)", k),
            )]);
        }
    }

    let supplement = args.named.get("supplement").and_then(|v| match v {
        Value::Content(c) => Some(c.clone()),
        Value::Str(s) => Some(Content::text(s.as_str())),
        Value::None => None,
        other => Some(Content::text(other.type_name())),
    });

    let form = extract_citation_form(args.named.get("form"))?;
    let style = extract_citation_style(args.named.get("style"))?;

    Ok(Value::Content(Content::cite_with_style(
        key, supplement, form, style,
    )))
}

/// `link(url, body)` — hiperligação (P422).
///
/// - 1º arg posicional: URL (`Str`).
/// - 2º arg posicional opcional: body (`Content`). Se omitido, o body é o
///   próprio URL como texto.
/// - Named args não suportados.
pub fn native_link(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    super::expect_no_named(&args.named)?;
    let url = match args.items.first() {
        Some(Value::Str(s)) if !s.is_empty() => s.clone(),
        Some(Value::Str(_)) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "link() URL não pode ser vazia".to_string(),
            )])
        }
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("link() espera URL como string, recebeu {}", other.type_name()),
            )])
        }
        None => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "link() exige URL como argumento posicional".to_string(),
            )])
        }
    };

    let body = match args.items.get(1) {
        Some(Value::Content(c)) => c.clone(),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("link() espera body como content, recebeu {}", other.type_name()),
            )])
        }
        None => Content::text(url.as_str()),
    };

    Ok(Value::Content(Content::link(url, body)))
}

/// Helper privado P159C — parsing `Value::Str` para
/// `Option<CitationForm>`. Strict matching (case-sensitive);
/// `auto`/`none`/ausente → None (resolvido a Normal default em
/// layout). String inválida rejeitada com mensagem listando forms
/// válidas.
fn extract_citation_form(
    val: Option<&Value>,
) -> SourceResult<Option<crate::entities::citation_form::CitationForm>> {
    use crate::entities::citation_form::CitationForm;
    match val {
        None | Some(Value::Auto) | Some(Value::None) => Ok(None),
        Some(Value::Str(s)) => match s.as_str() {
            "normal" => Ok(Some(CitationForm::Normal)),
            "prose" => Ok(Some(CitationForm::Prose)),
            "author" => Ok(Some(CitationForm::Author)),
            "year" => Ok(Some(CitationForm::Year)),
            other => Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "cite(): form '{}' inválido (válidos: normal, prose, author, year)",
                    other
                ),
            )]),
        },
        Some(other) => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("cite(): form espera string, recebeu {}", other.type_name()),
        )]),
    }
}

/// Helper privado P468 — parsing `Value::Str` para
/// `Option<CitationStyle>`. Strict matching (case-sensitive);
/// `auto`/`none`/ausente → None (resolvido a Numeric default em
/// layout fallback). String inválida rejeitada com mensagem listando
/// styles válidos.
fn extract_citation_style(
    val: Option<&Value>,
) -> SourceResult<Option<crate::entities::citation_style::CitationStyle>> {
    use crate::entities::citation_style::CitationStyle;
    match val {
        None | Some(Value::Auto) | Some(Value::None) => Ok(None),
        Some(Value::Str(s)) => match s.as_str() {
            "numeric" => Ok(Some(CitationStyle::Numeric)),
            "author-date" => Ok(Some(CitationStyle::AuthorDate)),
            "alphabetic" => Ok(Some(CitationStyle::Alphabetic)),
            other => Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "cite(): style '{}' inválido (válidos: numeric, author-date, alphabetic)",
                    other
                ),
            )]),
        },
        Some(other) => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("cite(): style espera string, recebeu {}", other.type_name()),
        )]),
    }
}

// ── Passo 295 — `footnote()` cluster Fase 1 (marker only) ─────────────────
//
// **Fase 1 P295 (HE marker only)**: variant minimal `Content::Footnote
// { body }`. Layouter emite apenas marker `[N]` superscript inline;
// body armazenado mas não renderizado no rodapé (sub-passo P295.1).
//
// Simplifications per ADR-0054 graded vs vanilla:
// - `numbering` field scope-out (cosmético; arabic default).
// - `FootnoteBody::Reference(Label)` scope-out (multi-ref futura).
//
// Padrão "variant rico" N=4 cumulativo **inalterado** — A.2 → (a)
// minimal.

/// `footnote(body)` — emite `Content::Footnote { body }`. Body
/// posicional obrigatório (content ou string).
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

    // Validar ausência de named args (P295 Fase 1: sem cosméticos).
    for k in args.named.keys() {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("footnote(): argumento nomeado '{}' não suportado em P295 Fase 1 (numbering/cosméticos scope-out per ADR-0054 graded)", k),
        )]);
    }

    Ok(Value::Content(Content::footnote(body)))
}

// ── Passo 296 — `accent()` + `cancel()` math (P-math-accent-cancel) ──────
//
// **A.0.0 N=4 (HIV)**: features ausentes apesar de Tabela A.4
// marcar `parcial`. Materialização from-scratch. Padrão "variant
// rico" N=4 **preservado** (A.2 → (a) minimal; sem cosméticos
// `size`/`length`/`inverted`/`cross`/`angle`/`stroke`).

/// `accent(base, accent)` — emite `Content::MathAccent { base, accent }`.
/// Ambos posicionais obrigatórios (content ou string).
pub fn native_accent(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let base = match args.items.first() {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::Str(s)) => Content::text(s.as_str()),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "accent() base espera content ou string, recebeu {}",
                    other.type_name()
                ),
            )])
        }
        None => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "accent() exige base como 1.º argumento posicional".to_string(),
            )])
        }
    };
    let accent = match args.items.get(1) {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::Str(s)) => Content::text(s.as_str()),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "accent() accent espera content ou string, recebeu {}",
                    other.type_name()
                ),
            )])
        }
        None => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "accent() exige accent como 2.º argumento posicional".to_string(),
            )])
        }
    };

    // Validar ausência de named args (P296 scope-out cosméticos).
    for k in args.named.keys() {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("accent(): argumento nomeado '{}' não suportado em P296 (size/dotless scope-out per ADR-0054 graded)", k),
        )]);
    }

    Ok(Value::Content(Content::math_accent(base, accent)))
}

/// `cancel(body)` — emite `Content::MathCancel { body }`.
/// Body posicional obrigatório (content ou string).
pub fn native_cancel(
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
                    "cancel() espera content ou string, recebeu {}",
                    other.type_name()
                ),
            )])
        }
        None => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "cancel() exige body como argumento posicional".to_string(),
            )])
        }
    };

    // Validar ausência de named args (P296 scope-out cosméticos).
    for k in args.named.keys() {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("cancel(): argumento nomeado '{}' não suportado em P296 (length/inverted/cross/angle/stroke scope-out per ADR-0054 graded)", k),
        )]);
    }

    Ok(Value::Content(Content::math_cancel(body)))
}

// ── Passo 297 — `underover()` math (P296.1) ──────────────────────────────
//
// **HV'.a (A.0.0 N=5)**: vanilla typst NÃO tem `UnderoverElem`
// unificado — fragmenta em 12 elementos (UnderlineElem/OverlineElem/
// UnderbraceElem/OverbraceElem/etc.). Cristalino agrega num único
// variant `MathUnderover { base, under?, over? }` per ADR-0054
// graded.
//
// **A.2 → (b) Option `Box<Content>` estrutural**: primeira
// qualificação genuína "variant rico" N=5 desde P287 refutação.
// Promoção ADR meta adiada per P273.17 §0.

/// `underover(base, under: ?, over: ?)` — emite
/// `Content::MathUnderover`. Base posicional; under/over named
/// opcionais.
pub fn native_underover(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let base = match args.items.first() {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::Str(s)) => Content::text(s.as_str()),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "underover() base espera content ou string, recebeu {}",
                    other.type_name()
                ),
            )])
        }
        None => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "underover() exige base como argumento posicional".to_string(),
            )])
        }
    };

    // Validar named args só "under"/"over" permitidos.
    for k in args.named.keys() {
        if !["under", "over"].contains(&k.as_str()) {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("underover(): argumento nomeado inesperado '{}' (válidos: under, over)", k),
            )]);
        }
    }

    let under = args.named.get("under").and_then(|v| match v {
        Value::Content(c) => Some(c.clone()),
        Value::Str(s) => Some(Content::text(s.as_str())),
        Value::None => None,
        other => Some(Content::text(other.type_name())),
    });
    let over = args.named.get("over").and_then(|v| match v {
        Value::Content(c) => Some(c.clone()),
        Value::Str(s) => Some(Content::text(s.as_str())),
        Value::None => None,
        other => Some(Content::text(other.type_name())),
    });

    Ok(Value::Content(Content::math_underover(base, under, over)))
}

// ── Passo 298 — `op()` math (P296.2 fecho cluster math 4/4) ───────────────
//
// **HV'' adaptado (A.0.0 N=6 magnitude alta)**: cristalino tinha
// heurística limits-style hardcoded em `attach.rs` via
// `symbols::is_limit_function`/`is_large_operator`. P298 estende
// para suportar `MathOp { limits: true }` user-customizable.
//
// **Cross-variant interaction inaugural**: `MathOp.limits` afecta
// layout de `MathAttach` (modificação `is_limits` em `attach.rs`).
// Heurística pré-P298 preservada — fallback `MathIdent("lim")`
// continua a funcionar.

/// `op(text, limits: false)` — emite `Content::MathOp { text, limits }`.
/// Text posicional obrigatório; `limits` named opcional (default `false`).
pub fn native_op(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let text = match args.items.first() {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::Str(s)) => Content::text(s.as_str()),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "op() text espera content ou string, recebeu {}",
                    other.type_name()
                ),
            )])
        }
        None => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "op() exige text como argumento posicional".to_string(),
            )])
        }
    };

    // Validar named args só "limits" permitido.
    for k in args.named.keys() {
        if k.as_str() != "limits" {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("op(): argumento nomeado inesperado '{}' (válido: limits)", k),
            )]);
        }
    }

    let limits = match args.named.get("limits") {
        Some(Value::Bool(b)) => *b,
        Some(Value::None) => false,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("op(limits:) espera bool, recebeu {}", other.type_name()),
            )])
        }
        None => false,
    };

    Ok(Value::Content(Content::math_op(text, limits)))
}

// ── Passo 299 — `math` module: operadores pré-definidos (P298.X) ───────────
//
// **A.0.0 N=7 magnitude baixa-modesta**: cristalino tem `calc`
// module precedente claro (P283) via `make_calc_module()` paralelo
// `make_math_module()`. **41 operadores vanilla** registados como
// `Value::Content(Content::MathOp { ... })` — 1ª aplicação prática
// de MathOp pós-materialização P298.
//
// Lista canónica vanilla `lab/.../math/op.rs:62-105`:
// - 29 scripts-style.
// - 12 limits-style.
//
// Acesso user-facing: `math.sin x`, `math.lim_(x→0) f` (namespaced).
// Heurística pré-P299 preservada — `MathIdent("lim")` literal
// continua a funcionar via fallback `is_limit_function`.

fn op_value(text: &str, limits: bool) -> Value {
    Value::Content(Content::math_op(Content::text(text), limits))
}

/// Constrói o módulo `math` como `Value::Dict` com 41 operadores
/// vanilla pré-definidos (paralelo `make_calc_module()` P283).
pub fn make_math_module() -> Value {
    use ecow::EcoString;
    use indexmap::IndexMap;
    use rustc_hash::FxBuildHasher;
    let mut dict: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();

    // Scripts-style operators (29) — limits: false.
    for name in [
        "arccos", "arcsin", "arctan", "arg", "cos", "cosh", "cot", "coth", "csc", "csch",
        "ctg", "deg", "dim", "exp", "hom", "id", "im", "ker", "lg", "ln", "log", "mod",
        "sec", "sech", "sin", "sinc", "sinh", "tan", "tanh", "tg", "tr",
    ] {
        dict.insert(name.into(), op_value(name, false));
    }

    // Limits-style operators (12) — limits: true. Multi-word usam
    // text literal vanilla (e.g. `liminf` → "lim inf").
    for (name, text) in [
        ("det", "det"),
        ("gcd", "gcd"),
        ("lcm", "lcm"),
        ("inf", "inf"),
        ("lim", "lim"),
        ("liminf", "lim inf"),
        ("limsup", "lim sup"),
        ("max", "max"),
        ("min", "min"),
        ("Pr", "Pr"),
        ("sup", "sup"),
    ] {
        dict.insert(name.into(), op_value(text, true));
    }

    // P480 — alias `equation` no módulo math para paridade de namespace vanilla.
    // Vanilla expõe `math.equation` como selector; cristalino regista aqui para
    // que `parse_selector("math.equation")` e `scope.get("math").equation`
    // resolvam. Value::None porque não existe função nativa `equation` em L1.
    dict.insert("equation".into(), Value::None);

    Value::Dict(dict)
}

// ── `figure()` — migrada de eval.rs (Passo 64, DEBT-16) ─────────────────────

// ── Passo 397: `document(...)` e `asset(...)` ────────────────────────────────

/// `document(title:?, author:?, date:?, keywords:?)` — metadata pura do documento.
pub fn native_document(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    const ALLOWED: &[&str] = &["title", "author", "date", "keywords"];
    for key in args.named.keys() {
        if !ALLOWED.contains(&key.as_str()) {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("document() não aceita o argumento nomeado '{}'", key),
            )]);
        }
    }

    let title = match args.named.get("title") {
        Some(Value::Content(c)) => Some(c.clone()),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("title deve ser content, recebeu {}", other.type_name()),
            )]);
        }
        None => None,
    };

    let author = match args.named.get("author") {
        Some(v) => extract_string_list(v, "author")?,
        None => Vec::new(),
    };

    let date = match args.named.get("date") {
        Some(Value::Datetime(d)) => Some(*d),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("date deve ser datetime, recebeu {}", other.type_name()),
            )]);
        }
        None => None,
    };

    let keywords = match args.named.get("keywords") {
        Some(v) => extract_string_list(v, "keywords")?,
        None => Vec::new(),
    };

    Ok(Value::Content(Content::document(title, author, date, keywords)))
}

/// `asset(path, kind:?)` — placeholder de resource externo (extensão cristalina).
pub fn native_asset(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    const ALLOWED: &[&str] = &["path", "kind"];
    for key in args.named.keys() {
        if !ALLOWED.contains(&key.as_str()) {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("asset() não aceita o argumento nomeado '{}'", key),
            )]);
        }
    }

    let path = if let Some(Value::Str(s)) = args.named.get("path") {
        s.clone()
    } else if let Some(Value::Str(s)) = args.items.first() {
        s.clone()
    } else {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "asset() espera path como string posicional ou named".to_string(),
        )]);
    };

    let kind = if let Some(Value::Str(s)) = args.named.get("kind") {
        Some(s.clone())
    } else {
        infer_asset_kind(&path)
    };

    Ok(Value::Content(Content::asset(path, kind)))
}

fn extract_string_list(value: &Value, field: &str) -> SourceResult<Vec<EcoString>> {
    match value {
        Value::Str(s) => Ok(vec![s.clone()]),
        Value::Array(arr) => arr
            .iter()
            .map(|v| match v {
                Value::Str(s) => Ok(s.clone()),
                other => Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!(
                        "{} deve ser string ou array de strings; recebeu {}",
                        field,
                        other.type_name()
                    ),
                )]),
            })
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e),
        other => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!(
                "{} deve ser string ou array de strings; recebeu {}",
                field,
                other.type_name()
            ),
        )]),
    }
}

fn infer_asset_kind(path: &EcoString) -> Option<EcoString> {
    let ext = path.rsplit('.').next()?;
    Some(match ext.to_lowercase().as_str() {
        "png" | "jpg" | "jpeg" | "gif" | "svg" | "webp" => EcoString::from("image"),
        "ttf" | "otf" | "woff" | "woff2" => EcoString::from("font"),
        "wasm" => EcoString::from("wasm"),
        "txt" | "csv" | "json" | "yaml" | "yml" | "toml" | "xml" => {
            EcoString::from("data")
        }
        _ => return None,
    })
}

// ── P470 — native_list / native_enum ────────────────────────────────────────

/// `list(..items, marker:?)` — cria itens de lista não ordenada com marcador
/// configurável (P470). Cada item posicional torna-se um `Content::ListItem`.
pub fn native_list(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    use crate::entities::list_marker::ListMarker;

    for key in args.named.keys() {
        if key.as_str() != "marker" {
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
                    Value::Content(c) => markers.push(ListMarker::Custom(c.plain_text().into())),
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
                format!("list(marker:) espera string ou array, recebeu {}", other.type_name()),
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
            Value::Str(s)     => Content::text(s.as_str()),
            other => {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!("list(): item deve ser content ou string, recebeu {}", other.type_name()),
                )]);
            }
        };
        let item = match &marker {
            None    => Content::list_item(body),
            Some(m) => Content::list_item_with_marker(body, m.clone()),
        };
        items.push(item);
    }
    Ok(Value::Content(if items.len() == 1 {
        items.remove(0)
    } else {
        Content::Sequence(items.into())
    }))
}

/// `enum(..items, numbering:?)` — cria itens de lista ordenada com esquema de
/// numeração configurável (P470). Cada item posicional torna-se um
/// `Content::EnumItem` com número 1-based.
pub fn native_enum(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    use crate::entities::enum_numbering::EnumNumbering;

    for key in args.named.keys() {
        if key.as_str() != "numbering" && key.as_str() != "start" {
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
                format!("enum(start:) espera inteiro >= 1, recebeu {}", other.type_name()),
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
            Value::Str(s)     => Content::text(s.as_str()),
            other => {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!("enum(): item deve ser content ou string, recebeu {}", other.type_name()),
                )]);
            }
        };
        let number = Some((idx as u32) + start);
        let item = match &numbering {
            None    => Content::enum_item(number, body),
            Some(n) => Content::enum_item_with_numbering(number, body, n.clone()),
        };
        items.push(item);
    }
    Ok(Value::Content(if items.len() == 1 {
        items.remove(0)
    } else {
        Content::Sequence(items.into())
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::args::Args;
    use crate::entities::file_id::FileId;
    use crate::entities::font_book::FontBook;
    use crate::entities::source::Source;
    use crate::entities::value::Value;
    use crate::entities::world_types::{
        Bytes, Datetime, FileError, FileResult, Font, Library,
    };
    use crate::rules::eval::EvalContext;
    use std::num::NonZeroU16;

    #[derive(Default)]
    struct NullWorld {
        library: Library,
        book: FontBook,
    }
    impl crate::contracts::world::World for NullWorld {
        fn library(&self) -> &Library {
            &self.library
        }
        fn book(&self) -> &FontBook {
            &self.book
        }
        fn main(&self) -> FileId {
            FileId::from_raw(NonZeroU16::new(1).unwrap())
        }
        fn source(&self, _: FileId) -> FileResult<Source> {
            Err(FileError::NotFound)
        }
        fn file(&self, _: FileId) -> FileResult<Bytes> {
            Err(FileError::NotFound)
        }
        fn font(&self, _: usize) -> Option<Font> {
            None
        }
        fn today(&self, _: Option<i64>) -> Option<Datetime> {
            None
        }
    }

    fn test_file_id() -> FileId {
        FileId::from_raw(NonZeroU16::new(1).unwrap())
    }

    fn call_document(args: Args) -> SourceResult<Value> {
        native_document(
            &mut EvalContext::new(),
            &args,
            &NullWorld::default(),
            test_file_id(),
        )
    }

    fn call_asset(args: Args) -> SourceResult<Value> {
        native_asset(
            &mut EvalContext::new(),
            &args,
            &NullWorld::default(),
            test_file_id(),
        )
    }

    fn named_args(pairs: &[(&str, Value)]) -> Args {
        let mut args = Args::positional(vec![]);
        for (k, v) in pairs {
            args.named.insert((*k).into(), v.clone());
        }
        args
    }

    #[test]
    fn native_document_title_content() {
        let mut args = Args::positional(vec![]);
        args.named
            .insert("title".into(), Value::Content(Content::text("Título")));
        let v = call_document(args).unwrap();
        let Value::Content(Content::Document { title, author, date, keywords }) = v
        else {
            panic!("esperado Content::Document, recebeu {:?}", v);
        };
        assert_eq!(title.as_ref().map(|b| b.plain_text()), Some("Título".to_string()));
        assert!(author.is_empty());
        assert!(date.is_none());
        assert!(keywords.is_empty());
    }

    #[test]
    fn native_document_author_str() {
        let args = named_args(&[("author", Value::Str("Ana".into()))]);
        let v = call_document(args).unwrap();
        let Value::Content(Content::Document { author, .. }) = v else {
            panic!("esperado Document")
        };
        assert_eq!(author, vec![EcoString::from("Ana")]);
    }

    #[test]
    fn native_document_author_array() {
        let args = named_args(&[(
            "author",
            Value::Array(vec![Value::Str("Ana".into()), Value::Str("Bob".into())]),
        )]);
        let v = call_document(args).unwrap();
        let Value::Content(Content::Document { author, .. }) = v else {
            panic!("esperado Document")
        };
        assert_eq!(author, vec![EcoString::from("Ana"), EcoString::from("Bob")]);
    }

    #[test]
    fn native_document_keywords_str() {
        let args = named_args(&[("keywords", Value::Str("typst".into()))]);
        let v = call_document(args).unwrap();
        let Value::Content(Content::Document { keywords, .. }) = v else {
            panic!("esperado Document")
        };
        assert_eq!(keywords, vec![EcoString::from("typst")]);
    }

    #[test]
    fn native_document_keywords_array() {
        let args = named_args(&[(
            "keywords",
            Value::Array(vec![Value::Str("a".into()), Value::Str("b".into())]),
        )]);
        let v = call_document(args).unwrap();
        let Value::Content(Content::Document { keywords, .. }) = v else {
            panic!("esperado Document")
        };
        assert_eq!(keywords, vec![EcoString::from("a"), EcoString::from("b")]);
    }

    #[test]
    fn native_document_date() {
        let dt = Datetime::new_date(2026, 6, 22).unwrap();
        let args = named_args(&[("date", Value::Datetime(dt))]);
        let v = call_document(args).unwrap();
        let Value::Content(Content::Document { date, .. }) = v else {
            panic!("esperado Document")
        };
        assert_eq!(date, Some(dt));
    }

    #[test]
    fn native_document_no_args() {
        let args = Args::positional(vec![]);
        let v = call_document(args).unwrap();
        let Value::Content(Content::Document { title, author, date, keywords }) = v
        else {
            panic!("esperado Document")
        };
        assert!(title.is_none());
        assert!(author.is_empty());
        assert!(date.is_none());
        assert!(keywords.is_empty());
    }

    #[test]
    fn native_document_rejects_unknown_named() {
        let args = named_args(&[("foo", Value::Str("x".into()))]);
        assert!(call_document(args).is_err());
    }

    #[test]
    fn native_document_rejects_bad_author_type() {
        let args = named_args(&[("author", Value::Int(1))]);
        assert!(call_document(args).is_err());
    }

    #[test]
    fn native_asset_path_positional() {
        let args = Args::positional(vec![Value::Str("logo.png".into())]);
        let v = call_asset(args).unwrap();
        let Value::Content(Content::Asset { path, kind }) = v else {
            panic!("esperado Asset")
        };
        assert_eq!(path, EcoString::from("logo.png"));
        assert_eq!(kind, Some(EcoString::from("image")));
    }

    #[test]
    fn native_asset_path_named() {
        let args = named_args(&[("path", Value::Str("font.ttf".into()))]);
        let v = call_asset(args).unwrap();
        let Value::Content(Content::Asset { path, kind }) = v else {
            panic!("esperado Asset")
        };
        assert_eq!(path, EcoString::from("font.ttf"));
        assert_eq!(kind, Some(EcoString::from("font")));
    }

    #[test]
    fn native_asset_kind_explicit() {
        let mut args = Args::positional(vec![Value::Str("x".into())]);
        args.named.insert("kind".into(), Value::Str("custom".into()));
        let v = call_asset(args).unwrap();
        let Value::Content(Content::Asset { kind, .. }) = v else {
            panic!("esperado Asset")
        };
        assert_eq!(kind, Some(EcoString::from("custom")));
    }

    #[test]
    fn native_asset_kind_unknown_extension() {
        let args = Args::positional(vec![Value::Str("file.xyz".into())]);
        let v = call_asset(args).unwrap();
        let Value::Content(Content::Asset { kind, .. }) = v else {
            panic!("esperado Asset")
        };
        assert_eq!(kind, None);
    }

    #[test]
    fn native_asset_rejects_missing_path() {
        let args = Args::positional(vec![Value::Int(1)]);
        assert!(call_asset(args).is_err());
    }

    fn call_heading(args: Args) -> SourceResult<Value> {
        native_heading(
            &mut EvalContext::new(),
            &args,
            &NullWorld::default(),
            test_file_id(),
        )
    }

    #[test]
    fn native_heading_sem_numbering_emite_heading_simples() {
        let args = Args::positional(vec![
            Value::Int(1),
            Value::Content(Content::text("Título")),
        ]);
        let v = call_heading(args).unwrap();
        let Value::Content(Content::Heading(h)) = v else {
            panic!("esperado Content::Heading, recebeu {:?}", v);
        };
        assert_eq!(h.level, 1);
        assert_eq!(h.body.plain_text(), "Título");
    }

    #[test]
    fn native_heading_com_numbering_emite_styled() {
        let mut args = Args::positional(vec![
            Value::Int(2),
            Value::Str("Sub".into()),
        ]);
        args.named.insert("numbering".into(), Value::Str("1.1".into()));
        let v = call_heading(args).unwrap();
        let Value::Content(Content::Styled(inner, styles)) = v else {
            panic!("esperado Content::Styled, recebeu {:?}", v);
        };
        assert!(matches!(inner.as_ref(), Content::Heading(h) if h.level == 2));
        let custom = &styles.delta().custom;
        assert!(
            custom.iter().any(|(k, v)| k == "heading.numbering" && matches!(v, Value::Bool(true))),
            "gate heading.numbering deve estar activo"
        );
        assert!(
            custom.iter().any(|(k, v)| k == "heading.numbering.pattern" && matches!(v, Value::Str(s) if s == "1.1")),
            "pattern deve ser transportado na chain"
        );
    }

    #[test]
    fn native_heading_rejeita_level_fora_do_range() {
        let args = Args::positional(vec![
            Value::Int(0),
            Value::Content(Content::text("X")),
        ]);
        assert!(call_heading(args).is_err());
    }

    #[test]
    fn native_heading_rejeita_body_invalido() {
        let args = Args::positional(vec![Value::Int(1), Value::Int(42)]);
        assert!(call_heading(args).is_err());
    }

    // ── Testes native_list / native_enum (P470) ──────────────────────────────

    fn call_list(args: Args) -> SourceResult<Value> {
        native_list(&mut EvalContext::new(), &args, &NullWorld::default(), test_file_id())
    }

    fn call_enum(args: Args) -> SourceResult<Value> {
        native_enum(&mut EvalContext::new(), &args, &NullWorld::default(), test_file_id())
    }

    #[test]
    fn list_sem_itens_devolve_empty() {
        let result = call_list(Args::positional(vec![])).unwrap();
        assert_eq!(result, Value::Content(Content::Empty));
    }

    #[test]
    fn list_dois_itens_sem_marker() {
        use crate::entities::elements::list_item::ListItemElem;
        use crate::entities::list_marker::ListMarker;
        let args = Args::positional(vec![
            Value::Content(Content::text("a")),
            Value::Content(Content::text("b")),
        ]);
        let result = call_list(args).unwrap();
        if let Value::Content(Content::Sequence(seq)) = result {
            assert_eq!(seq.len(), 2);
            for item in seq.iter() {
                if let Content::ListItem(li) = item {
                    assert_eq!(li.marker, None);
                } else {
                    panic!("esperado ListItem");
                }
            }
        } else {
            panic!("esperado Sequence");
        }
    }

    #[test]
    fn list_com_marker_custom() {
        use crate::entities::list_marker::ListMarker;
        let mut named = indexmap::IndexMap::default();
        named.insert("marker".into(), Value::Str("→".into()));
        let args = Args { items: vec![Value::Content(Content::text("x"))], named };
        let result = call_list(args).unwrap();
        if let Value::Content(Content::ListItem(li)) = result {
            assert_eq!(li.marker, Some(ListMarker::Custom("→".into())));
        } else {
            panic!("esperado ListItem com marker");
        }
    }

    #[test]
    fn list_marker_invalido_retorna_erro() {
        let mut named = indexmap::IndexMap::default();
        named.insert("marker".into(), Value::Int(1));
        let args = Args { items: vec![Value::Content(Content::text("x"))], named };
        assert!(call_list(args).is_err());
    }

    #[test]
    fn list_named_desconhecido_retorna_erro() {
        let mut named = indexmap::IndexMap::default();
        named.insert("foo".into(), Value::Int(1));
        let args = Args { items: vec![], named };
        assert!(call_list(args).is_err());
    }

    #[test]
    fn enum_sem_itens_devolve_empty() {
        let result = call_enum(Args::positional(vec![])).unwrap();
        assert_eq!(result, Value::Content(Content::Empty));
    }

    #[test]
    fn enum_dois_itens_sem_numbering() {
        let args = Args::positional(vec![
            Value::Content(Content::text("a")),
            Value::Content(Content::text("b")),
        ]);
        let result = call_enum(args).unwrap();
        if let Value::Content(Content::Sequence(seq)) = result {
            assert_eq!(seq.len(), 2);
            let first = &seq[0];
            if let Content::EnumItem(ei) = first {
                assert_eq!(ei.number, Some(1));
                assert_eq!(ei.numbering, None);
            } else {
                panic!("esperado EnumItem");
            }
            let second = &seq[1];
            if let Content::EnumItem(ei) = second {
                assert_eq!(ei.number, Some(2));
            } else {
                panic!("esperado EnumItem");
            }
        } else {
            panic!("esperado Sequence");
        }
    }

    #[test]
    fn enum_com_lower_alpha() {
        use crate::entities::enum_numbering::EnumNumbering;
        let mut named = indexmap::IndexMap::default();
        named.insert("numbering".into(), Value::Str("a)".into()));
        let args = Args {
            items: vec![Value::Content(Content::text("x")), Value::Content(Content::text("y"))],
            named,
        };
        let result = call_enum(args).unwrap();
        if let Value::Content(Content::Sequence(seq)) = result {
            if let Content::EnumItem(ei) = &seq[0] {
                assert_eq!(ei.numbering, Some(EnumNumbering::LowerAlpha));
            } else {
                panic!("esperado EnumItem");
            }
        } else {
            panic!("esperado Sequence");
        }
    }

    #[test]
    fn enum_numbering_invalido_retorna_erro() {
        let mut named = indexmap::IndexMap::default();
        named.insert("numbering".into(), Value::Int(1));
        let args = Args { items: vec![Value::Content(Content::text("x"))], named };
        assert!(call_enum(args).is_err());
    }

    #[test]
    fn enum_named_desconhecido_retorna_erro() {
        let mut named = indexmap::IndexMap::default();
        named.insert("foo".into(), Value::Int(1));
        let args = Args { items: vec![], named };
        assert!(call_enum(args).is_err());
    }
}
