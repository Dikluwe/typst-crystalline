//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/engine/stdlib/text.md
//! @prompt-hash fde4d5d8
//! @layer L1
//! @updated 2026-06-22
//!
//! Funções nativas de texto (upper, lower, replace, lorem, smallcaps).
//! Extraído de `stdlib.rs` no Passo 96.5 conforme ADR-0037.

use super::{err, expect_no_named};
use crate::entities::file_id::FileId;

use crate::engine::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::layout_types::{Color, Length};
use crate::entities::regex::Regex;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::style::{Style, Styles};
use crate::entities::value::Value;

use super::shapes::parse_color;

// ── P492 — constructor `text(...)` ──────────────────────────────────────────

/// `text(...)` — constructor de elemento de texto (parcial).
///
/// P492: suporta `text(red, it)` (fill posicional inferido por tipo) e
/// `text(fill: red, it)`. Aceita body como `Content` ou `Str`.
pub fn native_text(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    // Determinar fill: named arg prevalece; senão, primeiro posicional Color.
    let mut fill: Option<Color> = None;
    let mut body_idx = 0usize;

    if let Some(v) = args.named.get("fill") {
        fill = match v {
            Value::None => None,
            Value::Color(c) => Some(*c),
            other => {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!("text(fill:) espera color, recebeu {}", other.type_name()),
                )])
            }
        };
    }

    // Se o primeiro posicional for Color e fill ainda não definido, usá-lo como fill.
    if fill.is_none() {
        if let Some(Value::Color(c)) = args.items.first() {
            fill = Some(*c);
            body_idx = 1;
        }
    }

    let body = match args.items.get(body_idx) {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::Str(s)) => Content::text(s.as_str()),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!(
                    "text() espera content ou string como body, recebeu {}",
                    other.type_name()
                ),
            )])
        }
        None => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "text() requer body como argumento".to_string(),
            )])
        }
    };

    // Validar que não há argumentos posicionais além do fill + body.
    if args.items.len() > body_idx + 1 {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!(
                "text() recebeu {} argumentos posicionais (espera 1-2)",
                args.items.len()
            ),
        )]);
    }

    // Rejeitar named args desconhecidos (exceto fill).
    for key in args.named.keys() {
        if key.as_str() != "fill" {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("text() argumento nomeado desconhecido: '{}'", key),
            )]);
        }
    }

    let styled = match fill {
        Some(c) => Content::Styled(Box::new(body), Styles::from_iter([Style::Fill(c)])),
        None => body,
    };
    Ok(Value::Content(styled))
}

// ── `upper()` / `lower()` / `replace()` — motor map_text (Passo 67) ─────────

/// `upper(str | content)` → texto em maiúsculas.
pub fn native_upper(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Str(s)] => Ok(Value::Str(s.to_uppercase())),
        [Value::Content(c)] => {
            let mut f = |text: &str| text.to_uppercase();
            Ok(Value::Content(c.map_text(&mut f)))
        }
        [other] => err(format!(
            "upper() espera string ou content, recebeu {}",
            other.type_name()
        )),
        _ => err("upper() requer 1 argumento".to_string()),
    }
}

/// `lower(str | content)` → texto em minúsculas.
pub fn native_lower(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Str(s)] => Ok(Value::Str(s.to_lowercase())),
        [Value::Content(c)] => {
            let mut f = |text: &str| text.to_lowercase();
            Ok(Value::Content(c.map_text(&mut f)))
        }
        [other] => err(format!(
            "lower() espera string ou content, recebeu {}",
            other.type_name()
        )),
        _ => err("lower() requer 1 argumento".to_string()),
    }
}

/// `replace(fonte, padrão, substituição, count: N)` → string ou content com substituição.
///
/// `count` é global ao documento: persiste entre nós de texto via `FnMut`.
pub fn native_replace(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    // Validar named args: apenas "count" é aceite.
    for key in args.named.keys() {
        if key.as_str() != "count" {
            return err(format!("argumento nomeado inesperado: '{}'", key));
        }
    }

    if args.items.len() < 3 {
        return err(
            "replace() requer 3 argumentos: fonte, padrão, substituição".to_string()
        );
    }

    let pattern = match &args.items[1] {
        Value::Str(s) => s.to_string(),
        other => {
            return err(format!(
                "replace(): padrão deve ser string, recebeu {}",
                other.type_name()
            ))
        }
    };
    let replacement = match &args.items[2] {
        Value::Str(s) => s.to_string(),
        other => {
            return err(format!(
                "replace(): substituição deve ser string, recebeu {}",
                other.type_name()
            ))
        }
    };

    // Bloquear padrão vazio: replacen("", ...) entra em ciclo infinito.
    if pattern.is_empty() {
        return err("replace(): o padrão de busca não pode estar vazio".to_string());
    }

    let mut remaining_count: Option<i64> =
        args.named.get("count").and_then(|v| match v {
            Value::Int(i) => Some(*i),
            _ => None,
        });

    // A closure carrega `remaining_count` como estado mutável.
    // `map_text` usa `&mut F`, portanto o estado persiste entre nós do AST.
    // Isto garante que `count: N` é global ao documento, não por nó de texto.
    let mut do_replace = |text: &str| -> String {
        match remaining_count.as_mut() {
            Some(c) if *c <= 0 => text.to_string(),
            Some(c) => {
                let limit = *c as usize;
                let count_used = text.matches(pattern.as_str()).take(limit).count();
                let result = text.replacen(pattern.as_str(), replacement.as_str(), limit);
                *c -= count_used as i64;
                result
            }
            None => text.replace(pattern.as_str(), replacement.as_str()),
        }
    };

    match &args.items[0] {
        Value::Str(s) => Ok(Value::Str(do_replace(s.as_str()).into())),
        Value::Content(c) => Ok(Value::Content(c.map_text(&mut do_replace))),
        other => err(format!(
            "replace(): 1º argumento deve ser string ou content, recebeu {}",
            other.type_name()
        )),
    }
}

// ── Passo 284 — decoração textual (underline / strike / overline) ───────────
//
// Paridade vanilla `text/deco.rs`: três funções com `body` posicional
// obrigatório + cosméticos `stroke`/`offset`/`extent` opcionais (named).
// Atributos scope-out (`evade`, `background`, objecto Stroke rico) per
// diagnóstico P284 §A.1 + ADR-0054 graded.

/// Discriminador interno para escolher o construtor de `Content::*` no
/// helper partilhado `build_decoration`. Não é exposto em L0 — é apenas
/// um eixo de variação dentro de `text.rs`.
#[derive(Clone, Copy)]
enum DecoKind {
    Underline,
    Strike,
    Overline,
}

fn build_decoration(kind: DecoKind, args: &Args, fn_name: &str) -> SourceResult<Value> {
    let body = match args.items.as_slice() {
        [Value::Content(c)] => c.clone(),
        [Value::Str(s)] => Content::text(s.as_str()),
        [other] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!(
                    "{fn_name}() espera content ou string, recebeu {}",
                    other.type_name()
                ),
            )])
        }
        [] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("{fn_name}() exige body como argumento posicional"),
            )])
        }
        _ => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!(
                    "{fn_name}() recebeu {} argumentos posicionais (espera 1)",
                    args.items.len()
                ),
            )])
        }
    };

    let mut stroke: Option<Color> = None;
    let mut offset: Option<Length> = None;
    let mut extent: Option<Length> = None;

    for (key, value) in args.named.iter() {
        match key.as_str() {
            "stroke" => {
                // Apenas Color (paint puro) — Stroke rico vanilla é
                // scope-out per diagnóstico §A.1. `none` desactiva default.
                stroke =
                    match value {
                        Value::None => None,
                        v => match parse_color(v) {
                            Some(c) => Some(c),
                            None => return Err(vec![SourceDiagnostic::error(
                                args.span,
                                format!(
                                    "{fn_name}(stroke:) espera color ou none, recebeu {}",
                                    v.type_name()
                                ),
                            )]),
                        },
                    };
            }
            "offset" => {
                offset = match value {
                    Value::None => None,
                    Value::Length(l) => Some(*l),
                    Value::Int(i) => Some(Length::pt(*i as f64)),
                    Value::Float(f) => Some(Length::pt(*f)),
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            args.span,
                            format!(
                                "{fn_name}(offset:) espera length, recebeu {}",
                                other.type_name()
                            ),
                        )])
                    }
                };
            }
            "extent" => {
                extent = match value {
                    Value::None => None,
                    Value::Length(l) => Some(*l),
                    Value::Int(i) => Some(Length::pt(*i as f64)),
                    Value::Float(f) => Some(Length::pt(*f)),
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            args.span,
                            format!(
                                "{fn_name}(extent:) espera length, recebeu {}",
                                other.type_name()
                            ),
                        )])
                    }
                };
            }
            // P284 §A.1 scope-out: erros amigáveis para parâmetros vanilla
            // adiados, em vez de "argumento inesperado" genérico.
            "evade" | "background" => {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!(
                        "{fn_name}({}:) não suportado neste passo \
                         (P284 §A.1 scope-out / ADR-0054 graded)",
                        key,
                    ),
                )]);
            }
            other => {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!("{fn_name}(): argumento nomeado inesperado '{}'", other),
                )])
            }
        }
    }

    // Modelo D (Lote 4 P319): construtores ergonómicos (body sem Box).
    let c = match kind {
        DecoKind::Underline => Content::underline(body, stroke, offset, extent),
        DecoKind::Strike => Content::strike(body, stroke, offset, extent),
        DecoKind::Overline => Content::overline(body, stroke, offset, extent),
    };
    Ok(Value::Content(c))
}

pub fn native_underline(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    build_decoration(DecoKind::Underline, args, "underline")
}

pub fn native_strike(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    build_decoration(DecoKind::Strike, args, "strike")
}

pub fn native_overline(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    build_decoration(DecoKind::Overline, args, "overline")
}

// ── Passo 408 — `smallcaps(body)` ───────────────────────────────────────────
//
// Paridade vanilla `text/smallcaps.rs::SmallcapsElem`: elemento de texto que
// transforma o body em small capitals. Consumer real requer shaping OpenType
// (`smcp` / `c2sc`) — DEBT-53 scope-out XL. Neste passo materializa-se o
// variant `Content::SmallCaps` e a stdlib; o consumer em layout é stub
// transparente (output byte-idêntico ao body).
//
// Modelo minimal: só `body` (sem atributos opcionais). Aceita `content` ou
// `string` como argumento posicional único; named args são rejeitados.

/// `smallcaps(body)` → content embrulhado em `Content::SmallCaps`.
pub fn native_smallcaps(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;

    let body = match args.items.as_slice() {
        [Value::Content(c)] => c.clone(),
        [Value::Str(s)] => Content::text(s.as_str()),
        [other] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!(
                    "smallcaps() espera content ou string, recebeu {}",
                    other.type_name()
                ),
            )])
        }
        [] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "smallcaps() exige body como argumento posicional".to_string(),
            )])
        }
        _ => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!(
                    "smallcaps() recebeu {} argumentos posicionais (espera 1)",
                    args.items.len()
                ),
            )])
        }
    };

    Ok(Value::Content(Content::smallcaps(body)))
}

// ── Passo 448 — `sub(body)` / `super(body)` ─────────────────────────────────
//
// Paridade vanilla `text/sub.rs::SubElem` e `text/superscript.rs::SuperElem`:
// elementos de texto que deslocam a baseline e reduzem o corpo. Modelo
// minimal: `Content::Styled` com `Style::Subscript`/`Style::Superscript`.

/// `sub(body, size:?)` → content embrulhado em `Content::Styled([Subscript(true)])`.
/// P471: argumento nomeado `size: Length` define tamanho explícito do script.
pub fn native_subscript(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let body = match args.items.as_slice() {
        [Value::Content(c)] => c.clone(),
        [Value::Str(s)] => Content::text(s.as_str()),
        [other] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("sub() espera content ou string, recebeu {}", other.type_name()),
            )])
        }
        [] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "sub() exige body como argumento posicional".to_string(),
            )])
        }
        _ => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!(
                    "sub() recebeu {} argumentos posicionais (espera 1)",
                    args.items.len()
                ),
            )])
        }
    };
    let mut size: Option<Length> = None;
    for (key, value) in args.named.iter() {
        match key.as_str() {
            "size" => {
                size = match value {
                    Value::Length(l) => Some(*l),
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            args.span,
                            format!(
                                "sub(size:) espera length, recebeu {}",
                                other.type_name()
                            ),
                        )])
                    }
                };
            }
            other => {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!("sub() argumento nomeado inesperado '{}'", other),
                )])
            }
        }
    }
    Ok(Value::Content(Content::sub_with_size(body, size)))
}

/// `super(body, size:?)` → content embrulhado em `Content::Styled([Superscript(true)])`.
/// P471: argumento nomeado `size: Length` define tamanho explícito do script.
pub fn native_superscript(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let body = match args.items.as_slice() {
        [Value::Content(c)] => c.clone(),
        [Value::Str(s)] => Content::text(s.as_str()),
        [other] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!(
                    "super() espera content ou string, recebeu {}",
                    other.type_name()
                ),
            )])
        }
        [] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "super() exige body como argumento posicional".to_string(),
            )])
        }
        _ => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!(
                    "super() recebeu {} argumentos posicionais (espera 1)",
                    args.items.len()
                ),
            )])
        }
    };
    let mut size: Option<Length> = None;
    for (key, value) in args.named.iter() {
        match key.as_str() {
            "size" => {
                size = match value {
                    Value::Length(l) => Some(*l),
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            args.span,
                            format!(
                                "super(size:) espera length, recebeu {}",
                                other.type_name()
                            ),
                        )])
                    }
                };
            }
            other => {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!("super() argumento nomeado inesperado '{}'", other),
                )])
            }
        }
    }
    Ok(Value::Content(Content::superscript_with_size(body, size)))
}

// ── Passo 449 — `highlight(body, fill: color)` ──────────────────────────────
//
// Paridade vanilla `text/highlight.rs::HighlightElem`: fundo colorido por
// detrás do texto. Modelo minimal: `Content::Styled` com `Style::Highlight`.

/// Cor amarela padrão do Typst para highlight (`#ff236`).
fn default_highlight_color() -> Color {
    Color::rgba(255, 242, 54, 255)
}

/// `highlight(body, fill:?, radius:?, extent:?)` → content com fundo colorido.
/// P471: adiciona `radius: Length` (cantos arredondados) e `extent: Length`
/// (extensão horizontal). Cor default: amarelo vanilla `#fff236`.
pub fn native_highlight(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let body = match args.items.as_slice() {
        [Value::Content(c)] => c.clone(),
        [Value::Str(s)] => Content::text(s.as_str()),
        [other] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!(
                    "highlight() espera content ou string, recebeu {}",
                    other.type_name()
                ),
            )])
        }
        [] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "highlight() exige body como argumento posicional".to_string(),
            )])
        }
        _ => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!(
                    "highlight() recebeu {} argumentos posicionais (espera 1)",
                    args.items.len()
                ),
            )])
        }
    };

    let mut fill: Option<Color> = Some(default_highlight_color());
    let mut radius: Option<Length> = None;
    let mut extent: Option<Length> = None;
    for (key, value) in args.named.iter() {
        match key.as_str() {
            "fill" => {
                fill =
                    match value {
                        Value::None => None,
                        v => match parse_color(v) {
                            Some(c) => Some(c),
                            None => return Err(vec![SourceDiagnostic::error(
                                args.span,
                                format!(
                                    "highlight(fill:) espera color ou none, recebeu {}",
                                    v.type_name()
                                ),
                            )]),
                        },
                    };
            }
            "radius" => {
                radius =
                    match value {
                        Value::None => None,
                        Value::Length(l) => Some(*l),
                        other => return Err(vec![SourceDiagnostic::error(
                            args.span,
                            format!(
                                "highlight(radius:) espera length ou none, recebeu {}",
                                other.type_name()
                            ),
                        )]),
                    };
            }
            "extent" => {
                extent =
                    match value {
                        Value::None => None,
                        Value::Length(l) => Some(*l),
                        other => return Err(vec![SourceDiagnostic::error(
                            args.span,
                            format!(
                                "highlight(extent:) espera length ou none, recebeu {}",
                                other.type_name()
                            ),
                        )]),
                    };
            }
            other => {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!("highlight() argumento nomeado inesperado '{}'", other),
                )])
            }
        }
    }

    Ok(Value::Content(Content::highlight_full(body, fill, radius, extent)))
}

// ── Passo 287 — função `#smartquote(double, enabled, alternative)` ──────────
//
// Paridade vanilla `text/smartquote.rs::SmartQuoteElem`. Cristalino:
// - `double: bool = true`  — aspas duplas (default).
// - `enabled: bool = true` — smart-quotes activas. Quando `false`, emite
//                            o glyph ASCII literal (`"`/`'`) sem alternância
//                            lang-aware (paridade vanilla `set smartquote(enabled: false)`).
// - `alternative: bool`    — scope-out per ADR-0054 graded (rejeitado com erro
//                            educacional; passo dedicado futuro condicional).
//
// Diagnóstico P287 §A.2: variant `Content::smartquote(double)` é leaf
// (não rico) — 1 campo bool required. Quando `enabled = false`, função emite
// `Content::Text` directo (não passa pelo variant; estado open/close do
// Layouter preservado intacto).

pub fn native_smartquote(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    if !args.items.is_empty() {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!(
                "smartquote() não aceita argumentos posicionais (recebeu {})",
                args.items.len()
            ),
        )]);
    }

    let mut double: bool = true; // vanilla default
    let mut enabled: bool = true; // vanilla default

    for (key, value) in args.named.iter() {
        match key.as_str() {
            "double" => match value {
                Value::Bool(b) => double = *b,
                other => {
                    return Err(vec![SourceDiagnostic::error(
                        args.span,
                        format!(
                            "smartquote(double:) espera bool, recebeu {}",
                            other.type_name()
                        ),
                    )])
                }
            },
            "enabled" => match value {
                Value::Bool(b) => enabled = *b,
                other => {
                    return Err(vec![SourceDiagnostic::error(
                        args.span,
                        format!(
                            "smartquote(enabled:) espera bool, recebeu {}",
                            other.type_name()
                        ),
                    )])
                }
            },
            // Scope-out per diagnóstico P287 §A.2: `alternative` rejeitada
            // com erro educacional (ADR-0054 graded); passo dedicado futuro.
            "alternative" | "quotes" => {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!(
                        "smartquote({}:) não suportado neste passo \
                         (P287 §A.2 scope-out / ADR-0054 graded)",
                        key,
                    ),
                )]);
            }
            other => {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!("smartquote(): argumento nomeado inesperado '{}'", other),
                )])
            }
        }
    }

    if !enabled {
        // Paridade vanilla `enabled: false` — emite glyph ASCII literal
        // sem alternância lang-aware. Não passa pelo variant SmartQuote
        // (consumer Layouter não vê este caso).
        let glyph: &str = if double { "\"" } else { "'" };
        return Ok(Value::Content(Content::text(glyph)));
    }

    Ok(Value::Content(Content::smartquote(double)))
}

// ── Passo 391 — `lorem(n)` ───────────────────────────────────────────────────

/// Vocabulário dummy Lorem Ipsum usado por `native_lorem`.
const LOREM_WORDS: &[&str] = &[
    "Lorem",
    "ipsum",
    "dolor",
    "sit",
    "amet",
    "consectetur",
    "adipiscing",
    "elit",
    "sed",
    "do",
    "eiusmod",
    "tempor",
    "incididunt",
    "ut",
    "labore",
    "et",
    "dolore",
    "magna",
    "aliqua",
    "Ut",
    "enim",
    "ad",
    "minim",
    "veniam",
    "quis",
    "nostrud",
    "exercitation",
    "ullamco",
    "laboris",
    "nisi",
    "ut",
    "aliquip",
    "ex",
    "ea",
    "commodo",
    "consequat",
    "Duis",
    "aute",
    "irure",
    "dolor",
    "in",
    "reprehenderit",
    "in",
    "voluptate",
    "velit",
    "esse",
    "cillum",
    "dolore",
    "eu",
    "fugiat",
    "nulla",
    "pariatur",
    "Excepteur",
    "sint",
    "occaecat",
    "cupidatat",
    "non",
    "proident",
    "sunt",
    "in",
    "culpa",
    "qui",
    "officia",
    "deserunt",
    "mollit",
    "anim",
    "id",
    "est",
    "laborum",
];

/// `lorem(n)` → `Value::Str` com `n` palavras de texto dummy.
///
/// `n` deve ser um `Int` ≥ 0. Argumentos nomeados são rejeitados.
/// O texto exacto não precisa de coincidir com o vanilla; a paridade é
/// semântica — exactamente `n` palavras de Lorem Ipsum.
pub fn native_lorem(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;

    let n = match args.items.as_slice() {
        [Value::Int(n)] => *n,
        [other] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("lorem() espera um inteiro, recebeu {}", other.type_name()),
            )])
        }
        _ => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "lorem() requer 1 argumento inteiro",
            )])
        }
    };

    if n < 0 {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "lorem() não aceita números negativos",
        )]);
    }

    if n == 0 {
        return Ok(Value::Str("".into()));
    }

    let n = n as usize;
    let len = LOREM_WORDS.len();
    let mut words: Vec<&str> = Vec::with_capacity(n);
    for i in 0..n {
        words.push(LOREM_WORDS[i % len]);
    }

    Ok(Value::Str(words.join(" ").into()))
}

/// `regex(pattern)` → `Value::Regex` (P393).
///
/// Recebe um único argumento posicional `Str` com uma pattern regex válida.
/// Argumentos nomeados são rejeitados. Pattern inválida → erro de eval.
pub fn native_regex(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;

    match args.items.as_slice() {
        [Value::Str(pattern)] => match Regex::new(pattern.as_str()) {
            Ok(re) => Ok(Value::Regex(re)),
            Err(e) => Err(vec![SourceDiagnostic::error(
                args.span,
                format!("regex inválida: {}", e),
            )]),
        },
        [other] => err(format!("regex() espera string, recebeu {}", other.type_name())),
        _ => err("regex() requer 1 argumento (pattern)".to_string()),
    }
}
