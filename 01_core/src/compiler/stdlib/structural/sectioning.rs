//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/structural/sectioning.md
//! @prompt-hash 6866fa5b
//! @layer L1
//! @updated 2026-08-12
//!
//! Nativas de seccionamento e sumários: `heading`, `outline`, `title`,
//! `lof`/`lot`, `divider`.
//!
//! Extraído de `stdlib/structural.rs` no Passo 1014 conforme ADR-0109
//! (atomização — forma B, free function no arquivo da unidade).

use crate::entities::file_id::FileId;

use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::elements::outline::OutlineElem;
use crate::entities::elements::outline::OutlineTarget;
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
    let indent =
        match args.named.get("indent") {
            Some(Value::Bool(b)) => OutlineIndent::Bool(*b),
            Some(Value::Length(l)) => OutlineIndent::Length(*l),
            Some(Value::Func(f)) => OutlineIndent::Function(f.clone()),
            Some(Value::Auto) | Some(Value::None) | None => OutlineIndent::Auto,
            Some(other) => return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "outline(indent:): espera length, function, auto ou bool, recebeu {}",
                    other.type_name()
                ),
            )]),
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

    Ok(Value::Content(Content::Outline(std::sync::Arc::new(OutlineElem::with_target(
        title, depth, indent, target,
    )))))
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
            format!(
                "title(body:): espera content ou string, recebeu {}",
                other.type_name()
            ),
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
                format!(
                    "title(): body espera content ou string, recebeu {}",
                    other.type_name()
                ),
            )])),
            None => None,
        }
    };

    let body =
        match (body_named, body_positional) {
            (Some(Ok(_)), Some(Ok(_))) => return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "title(): não pode usar body posicional e named `body` simultaneamente"
                    .to_string(),
            )]),
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
        crate::entities::elements::title::TitleElem::new(body),
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
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "lof(title:): espera content ou string, recebeu {}",
                    other.type_name()
                ),
            )])
        }
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
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "lot(title:): espera content ou string, recebeu {}",
                    other.type_name()
                ),
            )])
        }
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
    crate::compiler::stdlib::expect_no_named(&args.named)?;
    if !args.items.is_empty() {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "divider() não aceita argumentos posicionais".to_string(),
        )]);
    }
    Ok(Value::Content(Content::divider()))
}

