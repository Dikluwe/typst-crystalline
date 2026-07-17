//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/engine/model/document.md
//! @prompt 00_nucleo/prompts/engine/model/asset.md
//! @prompt 00_nucleo/prompts/engine/stdlib/structural.md
//! @prompt-hash fd54a3ab
//! @layer L1
//! @updated 2026-06-29
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
use crate::entities::geometry::Stroke;
use crate::entities::layout_types::{Align2D, Color, HAlign, VAlign};
use crate::entities::paint::Paint;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;
use crate::engine::eval::EvalContext;

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

/// `raw(text, lang:?, block:?)` — cria `Content::Raw` ou serve como selector em show rules.
/// Aceita apenas string — não faz sentido semântico aceitar Content aqui.
/// P502: `lang` e `block` named opcionais; syntax highlighting real continua scope-out.
pub fn native_raw(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
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

    for (key, _value) in args.named.iter() {
        match key.as_str() {
            "lang" | "block" => {}
            other => {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!("raw(): argumento nomeado inesperado '{}'", other),
                )])
            }
        }
    }

    let lang = match args.named.get("lang") {
        Some(Value::Str(s)) => Some(s.clone()),
        Some(Value::None) | None => None,
        Some(v) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("raw(lang:): espera string, recebeu {}", v.type_name()),
            )]);
        }
    };

    let block = match args.named.get("block") {
        Some(Value::Bool(b)) => *b,
        Some(Value::None) | None => false,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("raw(block:): espera bool, recebeu {}", other.type_name()),
            )])
        }
    };

    Ok(Value::Content(Content::raw(text, lang, block)))
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
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "heading() exige body".to_string(),
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
                format!("heading(bookmarked:): espera bool, recebeu {}", other.type_name()),
            )])
        }
    };

    let content = if let Some(pattern) = numbering {
        Content::heading_numbered_with_pattern_outlined_bookmarked(
            level, body, Some(pattern), outlined, bookmarked,
        )
    } else {
        Content::heading_with_outlined_and_bookmarked(level, body, outlined, bookmarked)
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

    use crate::entities::elements::outline::OutlineIndent;
    let indent = match args.named.get("indent") {
        Some(Value::Bool(b)) => OutlineIndent::Bool(*b),
        Some(Value::Length(l)) => OutlineIndent::Length(*l),
        Some(Value::Func(f)) => OutlineIndent::Function(f.clone()),
        Some(Value::Auto) | Some(Value::None) | None => OutlineIndent::Auto,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("outline(indent:): espera length, function, auto ou bool, recebeu {}", other.type_name()),
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

/// `title(body?)` — emite `Content::Title`.
///
/// **P765a**: paridade com vanilla CLI 0.15.0. Aceita body posicional
/// (content ou string) ou named `body`. Se omitido, usa o metadado
/// `document.title` definido por `#set document(title: ...)`. Se este
/// também estiver ausente, produz erro.
pub fn native_title(
    ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    // Named `body` tem prioridade sobre posicional (paridade vanilla).
    let body_named = args.named.get("body").and_then(|v| match v {
        Value::Content(c) => Some(Ok(c.clone())),
        Value::Str(s) => Some(Ok(Content::text(s.as_str()))),
        Value::None => None,
        other => Some(Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("title(body:): espera content ou string, recebeu {}", other.type_name()),
        )])),
    });

    let body_positional = if args.items.is_empty() {
        None
    } else {
        match args.items.first() {
            Some(Value::Content(c)) => Some(Ok(c.clone())),
            Some(Value::Str(s)) => Some(Ok(Content::text(s.as_str()))),
            Some(other) => Some(Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("title(): body espera content ou string, recebeu {}", other.type_name()),
            )])),
            None => None,
        }
    };

    let body = match (body_named, body_positional) {
        (Some(Ok(_)), Some(Ok(_))) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "title(): não pode usar body posicional e named `body` simultaneamente".to_string(),
            )])
        }
        (Some(Ok(b)), _) => b,
        (Some(Err(e)), _) => return Err(e),
        (None, Some(Ok(b))) => b,
        (None, Some(Err(e))) => return Err(e),
        (None, None) => match &ctx.document_info.title {
            Some(t) => Content::text(t.as_str()),
            None => {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    "title() exige body ou metadado document.title".to_string(),
                )])
            }
        },
    };

    Ok(Value::Content(Content::Title(std::sync::Arc::new(
        crate::entities::elements::title::TitleElem::new(body)
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
    use crate::engine::stdlib::layout::{extract_stroke, extract_tracks};

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
    // P512 — separar linhas hline/vline dos children e calcular row/col
    // efectivos com base na ordem de aparecimento e no número de colunas.
    // P772v — `header`/`footer` extraídos aqui, à parte, mesmo padrão de
    // `native_grid` (P772i): não são argumentos nomeados no vanilla
    // (`table.header(...)`/`table.footer(...)` são elementos-filho
    // posicionais), não incrementam row/col, e só um de cada é suportado
    // (mesmo scope-out de P772i — múltiplos headers por `level` e
    // repeat-across-páginas não implementados).
    let mut header: Option<Content> = None;
    let mut footer: Option<Content> = None;
    let mut children: Vec<Content> = Vec::with_capacity(args.items.len());
    let mut hlines: Vec<crate::entities::elements::table_hline::TableHLineElem> = Vec::new();
    let mut vlines: Vec<crate::entities::elements::table_vline::TableVLineElem> = Vec::new();
    let num_cols = columns.len().max(1);
    let mut row = 0usize;
    let mut col = 0usize;
    for v in args.items.iter() {
        match v {
            Value::Content(Content::TableHLine(e)) => {
                let mut h = (**e).clone();
                if h.position == "auto" {
                    h.position = if col == 0 {
                        EcoString::from("top")
                    } else {
                        EcoString::from("bottom")
                    };
                }
                h.row = row;
                hlines.push(h);
            }
            Value::Content(Content::TableVLine(e)) => {
                let mut vline = (**e).clone();
                if vline.position == "auto" {
                    vline.position = EcoString::from("left");
                }
                vline.col = col;
                vlines.push(vline);
            }
            Value::Content(c @ Content::TableHeader(_)) => {
                if header.is_some() {
                    return Err(vec![SourceDiagnostic::error(
                        Span::detached(),
                        "table: não pode haver mais do que um header (scope-out — múltiplos headers por `level` não suportados)".to_string(),
                    )]);
                }
                header = Some(c.clone());
            }
            Value::Content(c @ Content::TableFooter(_)) => {
                if footer.is_some() {
                    return Err(vec![SourceDiagnostic::error(
                        Span::detached(),
                        "table: não pode haver mais do que um footer".to_string(),
                    )]);
                }
                footer = Some(c.clone());
            }
            Value::Content(c) => {
                children.push(c.clone());
                col += 1;
                if col >= num_cols {
                    col = 0;
                    row += 1;
                }
            }
            Value::Str(s) => {
                children.push(Content::text(s.as_str()));
                col += 1;
                if col >= num_cols {
                    col = 0;
                    row += 1;
                }
            }
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
    // P726 — `stroke: none` aceite (= sem traço, paridade vanilla).
    let stroke = match args.named.get("stroke") {
        Some(Value::None) | None => None,
        Some(val) => Some(extract_stroke(val, "table", "stroke")?),
    };
    // P228 — extract fill (Opção α: apenas Value::Color).
    // P726 — `fill: none` aceite (= sem preenchimento, paridade vanilla).
    let fill = match args.named.get("fill") {
        Some(Value::Color(c)) => Some(*c),
        Some(Value::None) | None => None,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("table(fill): espera Color, recebeu {}", other.type_name()),
            )])
        }
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
            hlines,
            vlines,
            header,
            footer,
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
            // P726 — `stroke: none` / `fill: none` aceites (= omitido, paridade vanilla).
            "stroke" => stroke = if matches!(value, Value::None) {
                None
            } else {
                Some(crate::engine::stdlib::layout::extract_stroke(value, "table_cell", "stroke")?)
            },
            "fill" => match value {
                Value::Color(c) => fill = Some(*c),
                Value::None => fill = None,
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
    // P772i — colectar todos os argumentos posicionais, não só o primeiro
    // (mesmo bug de `native_grid_header`, ver comentário lá).
    let mut cell_values: Vec<Content> = Vec::with_capacity(args.items.len());
    for v in args.items.iter() {
        match v {
            Value::Content(c) => cell_values.push(c.clone()),
            Value::Str(s)     => cell_values.push(Content::text(s.as_str())),
            other => return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("table_header() espera content ou string como argumento posicional, recebeu {}", other.type_name()),
            )]),
        }
    }
    if cell_values.is_empty() {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "table_header() exige pelo menos uma célula como argumento posicional".to_string(),
        )]);
    }
    let body = Content::sequence(cell_values);

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
    // P772i — colectar todos os argumentos posicionais, não só o primeiro
    // (mesmo bug de `native_grid_header`, ver comentário lá).
    let mut cell_values: Vec<Content> = Vec::with_capacity(args.items.len());
    for v in args.items.iter() {
        match v {
            Value::Content(c) => cell_values.push(c.clone()),
            Value::Str(s)     => cell_values.push(Content::text(s.as_str())),
            other => return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("table_footer() espera content ou string como argumento posicional, recebeu {}", other.type_name()),
            )]),
        }
    }
    if cell_values.is_empty() {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "table_footer() exige pelo menos uma célula como argumento posicional".to_string(),
        )]);
    }
    let body = Content::sequence(cell_values);

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
            // P726 — `stroke: none` / `fill: none` aceites (= omitido, paridade vanilla).
            "stroke" => stroke = if matches!(value, Value::None) {
                None
            } else {
                Some(crate::engine::stdlib::layout::extract_stroke(value, "grid_cell", "stroke")?)
            },
            "fill" => match value {
                Value::Color(c) => fill = Some(*c),
                Value::None => fill = None,
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
    // P772i — colectar TODOS os argumentos posicionais (`grid.header[A][B]`
    // = múltiplas células, sintaxe de vários blocos de conteúdo trailing),
    // não só o primeiro. `args.items.first()` descartava silenciosamente
    // todas as células a partir da segunda — confirmado por repro directo
    // (`grid.header[Nome][Idade]` só renderizava "Nome"). Ver
    // 00_nucleo/diagnosticos/paridade-producao-p772i.md.
    let mut cell_values: Vec<Content> = Vec::with_capacity(args.items.len());
    for v in args.items.iter() {
        match v {
            Value::Content(c) => cell_values.push(c.clone()),
            Value::Str(s)     => cell_values.push(Content::text(s.as_str())),
            other => return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("grid_header() espera content ou string como argumento posicional, recebeu {}", other.type_name()),
            )]),
        }
    }
    if cell_values.is_empty() {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "grid_header() exige pelo menos uma célula como argumento posicional".to_string(),
        )]);
    }
    let body = Content::sequence(cell_values);

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
    // P772i — colectar todos os argumentos posicionais, não só o primeiro
    // (mesmo bug de `native_grid_header`, ver comentário lá).
    let mut cell_values: Vec<Content> = Vec::with_capacity(args.items.len());
    for v in args.items.iter() {
        match v {
            Value::Content(c) => cell_values.push(c.clone()),
            Value::Str(s)     => cell_values.push(Content::text(s.as_str())),
            other => return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("grid_footer() espera content ou string como argumento posicional, recebeu {}", other.type_name()),
            )]),
        }
    }
    if cell_values.is_empty() {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "grid_footer() exige pelo menos uma célula como argumento posicional".to_string(),
        )]);
    }
    let body = Content::sequence(cell_values);

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
                let loaded = crate::engine::eval::bibliography::load_bib_entries_from_path(
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
        let resolved = Arc::new(crate::engine::eval::bibliography::resolve_style(
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

/// `footnote(body, numbering:?)` — emite `Content::Footnote { body, numbering }`.
/// Body posicional obrigatório (content ou string).
/// P502: `numbering` named opcional; uso no marcador scope-out per ADR-0054.
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

    let mut numbering: Option<EcoString> = None;
    for (k, v) in args.named.iter() {
        match k.as_str() {
            "numbering" => {
                numbering = match v {
                    Value::Str(s) => Some(s.clone()),
                    Value::None => None,
                    _ => {
                        return Err(vec![SourceDiagnostic::error(
                            Span::detached(),
                            format!("footnote(numbering:): espera string, recebeu {}", v.type_name()),
                        )])
                    }
                };
            }
            other => {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!("footnote(): argumento nomeado '{}' não suportado (cosméticos scope-out per ADR-0054 graded)", other),
                )]);
            }
        }
    }

    Ok(Value::Content(Content::footnote_with_numbering(body, numbering)))
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

// ── P772y — `math.class(class, body)` ────────────────────────────────────
//
// Força a `MathClass` de `body`, override do valor inferido automaticamente
// por `default_math_class`/`spacing::node_math_class`. Afecta apenas o
// espaçamento automático (`rules/math/layout/spacing.rs`); `body` é
// layoutado normalmente (`MathLayouter::layout_node`). Vanilla `ClassElem`
// (`math/mod.rs`).

/// `class(class, body)` — emite `Content::MathClassOverride { class, body }`.
/// `class` é obrigatório e uma das 15 strings vanilla
/// (`math_class.rs::parse_math_class`); `body` é obrigatório (content ou
/// string).
pub fn native_math_class(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let class_name = match args.items.first() {
        Some(Value::Str(s)) => s.clone(),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("class() espera string, recebeu {}", other.type_name()),
            )])
        }
        None => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "class() exige o nome da classe como 1.º argumento posicional".to_string(),
            )])
        }
    };
    let class = match crate::entities::math_class::parse_math_class(class_name.as_str()) {
        Some(c) => c,
        None => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("class(): '{}' não é uma MathClass reconhecida", class_name),
            )])
        }
    };

    let body = match args.items.get(1) {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::Str(s)) => Content::text(s.as_str()),
        // P772y — `math.class("relation", sym.suit.heart)`: símbolo
        // Unicode como body (paridade com a conversão de markup, P471).
        Some(Value::Symbol(s)) => Content::Text(EcoString::from(s.ch)),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "class() body espera content, string ou symbol, recebeu {}",
                    other.type_name()
                ),
            )])
        }
        None => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "class() exige body como 2.º argumento posicional".to_string(),
            )])
        }
    };

    for k in args.named.keys() {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("class(): argumento nomeado '{}' não suportado", k),
        )]);
    }

    Ok(Value::Content(Content::math_class_override(class, body)))
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

    // **P772y** — `math.class(class, body)`: override manual de `MathClass`
    // para efeitos de espaçamento automático. Vive no scope do módulo
    // `math` (não no scope global, ao contrário de `cancel`/`accent`).
    dict.insert(
        "class".into(),
        Value::Func(crate::entities::func::Func::native("class", native_math_class)),
    );

    // **P731** — `Value::Module` (paridade vanilla — medido: `type(math)` →
    // `module`), não `Value::Dict`. `eval/math.rs::lookup_math_op` lê o
    // scope do módulo.
    let mut scope = crate::entities::scope::Scope::new();
    for (name, value) in dict {
        scope.define(name.as_str(), value);
    }
    Value::Module(crate::entities::module::Module::new("math", scope))
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
    // P504 — alinhamento do marker (aceite; efeito visual scope-out).
    let marker_align: Option<Align2D> = match args.named.get("marker-align") {
        None => None,
        Some(Value::Align(a)) => Some(*a),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("list(marker-align:) espera alignment, recebeu {}", other.type_name()),
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
                format!("list(body-indent:) espera length, recebeu {}", other.type_name()),
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
            Value::Str(s)     => Content::text(s.as_str()),
            other => {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!("list(): item deve ser content ou string, recebeu {}", other.type_name()),
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
                format!("enum(start:) espera inteiro >= 1, recebeu {}", other.type_name()),
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
                format!("enum(body-indent:) espera length, recebeu {}", other.type_name()),
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
            Value::Str(s)     => Content::text(s.as_str()),
            other => {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!("enum(): item deve ser content ou string, recebeu {}", other.type_name()),
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

/// Stroke padrão para `grid.hline`/`grid.vline`/`table.hline`/`table.vline`
/// quando `stroke` é omitido: 1pt preto com overhang vanilla (`true`).
fn default_hline_stroke() -> Stroke {
    Stroke { paint: Paint::Solid(Color::rgb(0, 0, 0)), thickness: 1.0, overhang: true }
}

/// Helper P512 — extrai `start`/`end` comuns a hline/vline.
fn extract_line_range(
    args: &Args,
    fn_name: &str,
) -> SourceResult<(usize, Option<usize>)> {
    let start = match args.named.get("start") {
        Some(v) => match v {
            Value::Int(n) if *n >= 0 => *n as usize,
            other => {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!("{}(start): espera int >= 0, recebeu {}", fn_name, other.type_name()),
                )])
            }
        },
        None => 0,
    };
    let end = match args.named.get("end") {
        Some(v) => match v {
            Value::Auto | Value::None => None,
            Value::Int(n) if *n >= 0 => Some(*n as usize),
            other => {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!("{}(end): espera int >= 0 ou auto, recebeu {}", fn_name, other.type_name()),
                )])
            }
        },
        None => None,
    };
    Ok((start, end))
}

/// `grid.hline(start, end, stroke, position)` — Passo 512.
pub fn native_grid_hline(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let (start, end) = extract_line_range(args, "grid.hline")?;
    // **P739A** — `stroke: none` aceite (paridade vanilla, medido): a linha
    // não é desenhada. Zero-thickness proibido (hairline em PDF — P726).
    let stroke = match args.named.get("stroke") {
        Some(Value::None) => None,
        Some(v) => Some(super::layout::extract_stroke(v, "grid.hline", "stroke")?),
        None => Some(default_hline_stroke()),
    };
    let position = match args.named.get("position") {
        Some(Value::Str(s)) => s.clone(),
        Some(Value::Align(Align2D { v: Some(VAlign::Top), .. })) => EcoString::from("top"),
        Some(Value::Align(Align2D { v: Some(VAlign::Bottom), .. })) => EcoString::from("bottom"),
        Some(Value::Auto) | Some(Value::None) | None => EcoString::from("auto"),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("grid.hline(position): espera str, alignment ou auto, recebeu {}", other.type_name()),
            )])
        }
    };
    if !matches!(position.as_str(), "top" | "bottom" | "auto") {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "grid.hline(position): deve ser 'top', 'bottom' ou 'auto'".to_string(),
        )]);
    }
    Ok(Value::Content(Content::grid_hline(start, end, 0, stroke, position)))
}

/// `grid.vline(start, end, stroke, position)` — Passo 512.
pub fn native_grid_vline(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let (start, end) = extract_line_range(args, "grid.vline")?;
    let stroke = match args.named.get("stroke") {
        Some(Value::None) => None,
        Some(v) => Some(super::layout::extract_stroke(v, "grid.vline", "stroke")?),
        None => Some(default_hline_stroke()),
    };
    let position = match args.named.get("position") {
        Some(Value::Str(s)) => s.clone(),
        Some(Value::Align(Align2D { h: Some(HAlign::Left), .. })) => EcoString::from("left"),
        Some(Value::Align(Align2D { h: Some(HAlign::Right), .. })) => EcoString::from("right"),
        Some(Value::Auto) | Some(Value::None) | None => EcoString::from("left"),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("grid.vline(position): espera str, alignment ou auto, recebeu {}", other.type_name()),
            )])
        }
    };
    if !matches!(position.as_str(), "left" | "right" | "auto") {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "grid.vline(position): deve ser 'left', 'right' ou 'auto'".to_string(),
        )]);
    }
    Ok(Value::Content(Content::grid_vline(start, end, 0, stroke, position)))
}

/// `table.hline(start, end, stroke, position)` — Passo 512.
pub fn native_table_hline(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let (start, end) = extract_line_range(args, "table.hline")?;
    let stroke = match args.named.get("stroke") {
        Some(Value::None) => None,
        Some(v) => Some(super::layout::extract_stroke(v, "table.hline", "stroke")?),
        None => Some(default_hline_stroke()),
    };
    let position = match args.named.get("position") {
        Some(Value::Str(s)) => s.clone(),
        Some(Value::Align(Align2D { v: Some(VAlign::Top), .. })) => EcoString::from("top"),
        Some(Value::Align(Align2D { v: Some(VAlign::Bottom), .. })) => EcoString::from("bottom"),
        Some(Value::Auto) | Some(Value::None) | None => EcoString::from("auto"),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("table.hline(position): espera str, alignment ou auto, recebeu {}", other.type_name()),
            )])
        }
    };
    if !matches!(position.as_str(), "top" | "bottom" | "auto") {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "table.hline(position): deve ser 'top', 'bottom' ou 'auto'".to_string(),
        )]);
    }
    Ok(Value::Content(Content::table_hline(start, end, 0, stroke, position)))
}

/// `table.vline(start, end, stroke, position)` — Passo 512.
pub fn native_table_vline(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let (start, end) = extract_line_range(args, "table.vline")?;
    let stroke = match args.named.get("stroke") {
        Some(Value::None) => None,
        Some(v) => Some(super::layout::extract_stroke(v, "table.vline", "stroke")?),
        None => Some(default_hline_stroke()),
    };
    let position = match args.named.get("position") {
        Some(Value::Str(s)) => s.clone(),
        Some(Value::Align(Align2D { h: Some(HAlign::Left), .. })) => EcoString::from("left"),
        Some(Value::Align(Align2D { h: Some(HAlign::Right), .. })) => EcoString::from("right"),
        Some(Value::Auto) | Some(Value::None) | None => EcoString::from("left"),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("table.vline(position): espera str, alignment ou auto, recebeu {}", other.type_name()),
            )])
        }
    };
    if !matches!(position.as_str(), "left" | "right" | "auto") {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "table.vline(position): deve ser 'left', 'right' ou 'auto'".to_string(),
        )]);
    }
    Ok(Value::Content(Content::table_vline(start, end, 0, stroke, position)))
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
    use crate::engine::eval::EvalContext;
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

    #[test]
    fn native_heading_outlined_false_mantem_bookmarked_auto() {
        let mut args = Args::positional(vec![
            Value::Int(1),
            Value::Content(Content::text("X")),
        ]);
        args.named.insert("outlined".into(), Value::Bool(false));
        let v = call_heading(args).unwrap();
        let Value::Content(Content::Heading(h)) = v else {
            panic!("esperado Content::Heading, recebeu {:?}", v);
        };
        assert!(!h.outlined);
        assert_eq!(h.bookmarked, None, "bookmarked deve ficar auto (None)");
        assert!(!h.is_bookmarked(), "bookmarked efectivo segue outlined");
    }

    #[test]
    fn native_heading_bookmarked_false_mantem_outlined_true() {
        let mut args = Args::positional(vec![
            Value::Int(1),
            Value::Content(Content::text("X")),
        ]);
        args.named.insert("bookmarked".into(), Value::Bool(false));
        let v = call_heading(args).unwrap();
        let Value::Content(Content::Heading(h)) = v else {
            panic!("esperado Content::Heading, recebeu {:?}", v);
        };
        assert!(h.outlined, "outlined default deve permanecer true");
        assert_eq!(h.bookmarked, Some(false));
        assert!(!h.is_bookmarked());
    }

    #[test]
    fn native_heading_outlined_false_bookmarked_true_separados() {
        let mut args = Args::positional(vec![
            Value::Int(1),
            Value::Content(Content::text("X")),
        ]);
        args.named.insert("outlined".into(), Value::Bool(false));
        args.named.insert("bookmarked".into(), Value::Bool(true));
        let v = call_heading(args).unwrap();
        let Value::Content(Content::Heading(h)) = v else {
            panic!("esperado Content::Heading, recebeu {:?}", v);
        };
        assert!(!h.outlined);
        assert_eq!(h.bookmarked, Some(true));
        assert!(h.is_bookmarked());
    }

    #[test]
    fn native_heading_rejeita_outlined_nao_bool() {
        let mut args = Args::positional(vec![
            Value::Int(1),
            Value::Content(Content::text("X")),
        ]);
        args.named.insert("outlined".into(), Value::Int(1));
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
        let args = Args { items: vec![Value::Content(Content::text("x"))], named, span: Span::detached() };
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
        let args = Args { items: vec![Value::Content(Content::text("x"))], named, span: Span::detached() };
        assert!(call_list(args).is_err());
    }

    #[test]
    fn list_named_desconhecido_retorna_erro() {
        let mut named = indexmap::IndexMap::default();
        named.insert("foo".into(), Value::Int(1));
        let args = Args { items: vec![], named, span: Span::detached() };
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
            span: Span::detached(),
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
        let args = Args { items: vec![Value::Content(Content::text("x"))], named, span: Span::detached() };
        assert!(call_enum(args).is_err());
    }

    #[test]
    fn enum_named_desconhecido_retorna_erro() {
        let mut named = indexmap::IndexMap::default();
        named.insert("foo".into(), Value::Int(1));
        let args = Args { items: vec![], named, span: Span::detached() };
        assert!(call_enum(args).is_err());
    }

    // ── Testes P505 — indentação e tight em list/enums ───────────────────────

    #[test]
    fn list_indent_body_indent_tight_sao_propagados() {
        use crate::entities::layout_types::Length;
        let mut named = indexmap::IndexMap::default();
        named.insert("indent".into(), Value::Length(Length::em(1.5)));
        named.insert("body-indent".into(), Value::Length(Length::em(0.5)));
        named.insert("tight".into(), Value::Bool(false));
        let args = Args {
            items: vec![Value::Content(Content::text("a"))],
            named,
            span: Span::detached(),
        };
        let result = call_list(args).unwrap();
        if let Value::Content(Content::ListItem(li)) = result {
            assert_eq!(li.indent, Some(Length::em(1.5)));
            assert_eq!(li.body_indent, Some(Length::em(0.5)));
            assert_eq!(li.tight, Some(false));
        } else {
            panic!("esperado ListItem");
        }
    }

    #[test]
    fn list_indent_invalido_retorna_erro() {
        let mut named = indexmap::IndexMap::default();
        named.insert("indent".into(), Value::Str("x".into()));
        let args = Args { items: vec![Value::Content(Content::text("a"))], named, span: Span::detached() };
        assert!(call_list(args).is_err());
    }

    #[test]
    fn list_body_indent_invalido_retorna_erro() {
        let mut named = indexmap::IndexMap::default();
        named.insert("body-indent".into(), Value::Int(1));
        let args = Args { items: vec![Value::Content(Content::text("a"))], named, span: Span::detached() };
        assert!(call_list(args).is_err());
    }

    #[test]
    fn list_tight_invalido_retorna_erro() {
        let mut named = indexmap::IndexMap::default();
        named.insert("tight".into(), Value::Int(1));
        let args = Args { items: vec![Value::Content(Content::text("a"))], named, span: Span::detached() };
        assert!(call_list(args).is_err());
    }

    #[test]
    fn enum_indent_body_indent_tight_sao_propagados() {
        use crate::entities::layout_types::Length;
        let mut named = indexmap::IndexMap::default();
        named.insert("indent".into(), Value::Length(Length::em(1.5)));
        named.insert("body-indent".into(), Value::Length(Length::em(0.5)));
        named.insert("tight".into(), Value::Bool(false));
        let args = Args {
            items: vec![Value::Content(Content::text("a"))],
            named,
            span: Span::detached(),
        };
        let result = call_enum(args).unwrap();
        if let Value::Content(Content::EnumItem(ei)) = result {
            assert_eq!(ei.indent, Some(Length::em(1.5)));
            assert_eq!(ei.body_indent, Some(Length::em(0.5)));
            assert_eq!(ei.tight, Some(false));
        } else {
            panic!("esperado EnumItem");
        }
    }

    #[test]
    fn enum_indent_invalido_retorna_erro() {
        let mut named = indexmap::IndexMap::default();
        named.insert("indent".into(), Value::Str("x".into()));
        let args = Args { items: vec![Value::Content(Content::text("a"))], named, span: Span::detached() };
        assert!(call_enum(args).is_err());
    }

    #[test]
    fn enum_start_maior_que_um_propaga_numeracao() {
        let mut named = indexmap::IndexMap::default();
        named.insert("start".into(), Value::Int(3));
        let args = Args {
            items: vec![Value::Content(Content::text("a"))],
            named,
            span: Span::detached(),
        };
        let result = call_enum(args).unwrap();
        if let Value::Content(Content::EnumItem(ei)) = result {
            assert_eq!(ei.number, Some(3));
        } else {
            panic!("esperado EnumItem");
        }
    }

    // ── Passo 512 — grid/table hline/vline ───────────────────────────────────

    fn call_grid_hline(args: Args) -> SourceResult<Value> {
        native_grid_hline(
            &mut EvalContext::new(),
            &args,
            &NullWorld::default(),
            test_file_id(),
        )
    }

    fn call_table_hline(args: Args) -> SourceResult<Value> {
        native_table_hline(
            &mut EvalContext::new(),
            &args,
            &NullWorld::default(),
            test_file_id(),
        )
    }

    fn call_grid_vline(args: Args) -> SourceResult<Value> {
        native_grid_vline(
            &mut EvalContext::new(),
            &args,
            &NullWorld::default(),
            test_file_id(),
        )
    }

    fn call_table_vline(args: Args) -> SourceResult<Value> {
        native_table_vline(
            &mut EvalContext::new(),
            &args,
            &NullWorld::default(),
            test_file_id(),
        )
    }

    #[test]
    fn grid_hline_defaults() {
        let v = call_grid_hline(Args::positional(vec![])).unwrap();
        if let Value::Content(Content::GridHLine(e)) = v {
            assert_eq!(e.start, 0);
            assert_eq!(e.end, None);
            assert_eq!(e.row, 0);
            assert_eq!(e.position.as_str(), "auto");
            assert_eq!(e.stroke.as_ref().expect("stroke default").thickness, 1.0);
        } else {
            panic!("esperado GridHLine");
        }
    }

    #[test]
    fn grid_hline_position_bottom_alignment() {
        use crate::entities::layout_types::{Align2D, VAlign};
        let mut args = Args::positional(vec![]);
        args.named.insert(
            "position".into(),
            Value::Align(Align2D { h: None, v: Some(VAlign::Bottom) }),
        );
        let v = call_grid_hline(args).unwrap();
        if let Value::Content(Content::GridHLine(e)) = v {
            assert_eq!(e.position.as_str(), "bottom");
        } else {
            panic!("esperado GridHLine");
        }
    }

    #[test]
    fn grid_vline_defaults() {
        let v = call_grid_vline(Args::positional(vec![])).unwrap();
        if let Value::Content(Content::GridVLine(e)) = v {
            assert_eq!(e.start, 0);
            assert_eq!(e.end, None);
            assert_eq!(e.col, 0);
            assert_eq!(e.position.as_str(), "left");
        } else {
            panic!("esperado GridVLine");
        }
    }

    #[test]
    fn table_hline_defaults() {
        let v = call_table_hline(Args::positional(vec![])).unwrap();
        if let Value::Content(Content::TableHLine(e)) = v {
            assert_eq!(e.start, 0);
            assert_eq!(e.end, None);
            assert_eq!(e.row, 0);
            assert_eq!(e.position.as_str(), "auto");
        } else {
            panic!("esperado TableHLine");
        }
    }

    #[test]
    fn table_vline_defaults() {
        let v = call_table_vline(Args::positional(vec![])).unwrap();
        if let Value::Content(Content::TableVLine(e)) = v {
            assert_eq!(e.start, 0);
            assert_eq!(e.end, None);
            assert_eq!(e.col, 0);
            assert_eq!(e.position.as_str(), "left");
        } else {
            panic!("esperado TableVLine");
        }
    }
}
