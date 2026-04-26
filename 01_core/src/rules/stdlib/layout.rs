//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/stdlib.md
//! @prompt-hash f6cc2443
//! @layer L1
//! @updated 2026-04-23
//!
//! Funções nativas de layout (align, place, grid, page).
//! Extraído de `stdlib.rs` no Passo 96.5 conforme ADR-0037.

use super::expect_no_named;
use crate::entities::file_id::FileId;

use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::layout_types::{Abs, Align2D, Length, TrackSizing};
use crate::entities::parity::Parity;
use crate::entities::sides::Sides;
use crate::entities::span::Span;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::value::Value;
use crate::rules::eval::EvalContext;

// ── Align / Place (Passo 82) ────────────────────────────────────────────────

/// `align(alignment, body)` → `Content::Align`.
///
/// `alignment` aceita `Value::Align` (sintaxe preferida pós-Passo 84.5,
/// ex: `align(center + bottom, ...)`) ou `Value::Str` (sintaxe legacy,
/// ex: `align("center", ...)`) — ver DEBT-36 (encerrado).
/// `body` é o primeiro argumento posicional do tipo Content.
pub fn native_align(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    expect_no_named(&args.named)?;

    let alignment = extract_alignment(args, Align2D::default());

    let body = args.items.iter()
        .find_map(|v| if let Value::Content(c) = v { Some(c.clone()) } else { None })
        .ok_or_else(|| vec![SourceDiagnostic::error(Span::detached(),
            "align() exige um bloco de conteúdo".to_string())])?;

    Ok(Value::Content(Content::Align {
        alignment,
        body: Box::new(body),
    }))
}

/// `place(alignment, dx?, dy?, scope?, body)` → `Content::Place`.
///
/// `dx`/`dy` em pt deslocam o conteúdo a partir da posição alinhada.
/// `scope` (Passo 84.6, encerra DEBT-37): `"column"` (default — ancora à
/// célula activa de Grid, ou à página fora de Grid) ou `"parent"` (ancora
/// sempre à página). Aceita string ou omissão.
pub fn native_place(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    for key in args.named.keys() {
        if !["dx", "dy", "scope"].contains(&key.as_str()) {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("argumento nomeado inesperado em place(): '{}'", key),
            )]);
        }
    }

    fn extract_pt(val: &Value) -> f64 {
        match val {
            Value::Float(f)  => *f,
            Value::Int(i)    => *i as f64,
            Value::Length(l) => l.abs.to_pt(),
            _ => 0.0,
        }
    }

    // Default "top-left" para Place, "left" (default Align2D) para Align.
    let alignment = extract_alignment(
        args,
        Align2D { h: Some(crate::entities::layout_types::HAlign::Left),
                  v: Some(crate::entities::layout_types::VAlign::Top) },
    );

    let dx = args.named.get("dx").map(extract_pt).unwrap_or(0.0);
    let dy = args.named.get("dy").map(extract_pt).unwrap_or(0.0);

    let scope = match args.named.get("scope") {
        Some(Value::Str(s)) => match s.as_str() {
            "column" => crate::entities::layout_types::PlaceScope::Column,
            "parent" => crate::entities::layout_types::PlaceScope::Parent,
            other => return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("place(): scope deve ser \"column\" ou \"parent\", recebeu \"{}\"", other),
            )]),
        },
        Some(other) => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("place(): scope deve ser string, recebeu {}", other.type_name()),
        )]),
        None => crate::entities::layout_types::PlaceScope::default(),
    };

    let body = args.items.iter()
        .find_map(|v| if let Value::Content(c) = v { Some(c.clone()) } else { None })
        .ok_or_else(|| vec![SourceDiagnostic::error(Span::detached(),
            "place() exige um bloco de conteúdo".to_string())])?;

    Ok(Value::Content(Content::Place {
        alignment,
        dx,
        dy,
        scope,
        body: Box::new(body),
    }))
}

/// Helper Passo 84.5: extrai alinhamento do primeiro argumento posicional
/// que case com `Value::Align` ou `Value::Str`. Sintaxe preferida `Value::Align`,
/// sintaxe legacy via `Align2D::from_string`. Caso nenhum case, retorna `default`.
fn extract_alignment(args: &Args, default: Align2D) -> Align2D {
    args.items.iter()
        .find_map(|v| match v {
            Value::Align(a) => Some(*a),
            Value::Str(s)   => Some(Align2D::from_string(s.as_str())),
            _ => None,
        })
        .unwrap_or(default)
}

// ── Grid (Passo 80) ────────────────────────────────────────────────────────

fn parse_track_sizing(val: &Value) -> Option<TrackSizing> {
    match val {
        Value::Float(f)    => Some(TrackSizing::Fixed(*f)),
        Value::Length(l)   => Some(TrackSizing::Fixed(l.abs.to_pt())),
        Value::Fraction(fr) => Some(TrackSizing::Fraction(*fr)),
        Value::Auto        => Some(TrackSizing::Auto),
        Value::Str(s) if s.as_str() == "auto" => Some(TrackSizing::Auto),
        _ => None,
    }
}

fn extract_tracks(val: Option<&Value>) -> Vec<TrackSizing> {
    match val {
        Some(Value::Array(arr)) => arr.iter().filter_map(parse_track_sizing).collect(),
        // `grid(rows: 3)` ou `grid(columns: 3)` → 3 tracks Auto (Passo 83).
        Some(Value::Int(n)) if *n > 0 => vec![TrackSizing::Auto; *n as usize],
        Some(v) => parse_track_sizing(v).into_iter().collect(),
        None    => vec![],
    }
}

/// `grid(columns?, rows?, ...cells)` → `Content::Grid`.
pub fn native_grid(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    for key in args.named.keys() {
        if !["columns", "rows"].contains(&key.as_str()) {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("argumento nomeado inesperado em grid(): '{}'", key),
            )]);
        }
    }
    let mut columns = extract_tracks(args.named.get("columns"));
    let mut rows    = extract_tracks(args.named.get("rows"));
    // Defaults Passo 83 — `rows` omitido ou tuplo vazio → uma linha Auto repetida
    // para todas as linhas geradas pelo número de cells. Idem para columns.
    if columns.is_empty() {
        columns = vec![TrackSizing::Auto];
    }
    if rows.is_empty() {
        rows = vec![TrackSizing::Auto];
    }
    let cells: Vec<Content> = args.items.iter()
        .filter_map(|v| if let Value::Content(c) = v { Some(c.clone()) } else { None })
        .collect();
    Ok(Value::Content(Content::Grid { columns, rows, cells }))
}

/// `#set page(width: w, height: h, margin: m)` — configura as dimensões da página (Passo 81).
pub fn native_page(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    fn extract_pt(val: &Value) -> Option<f64> {
        match val {
            Value::Length(l) => Some(l.abs.to_pt()),
            Value::Float(f)  => Some(*f),
            Value::Int(i)    => Some(*i as f64),
            _                => None,
        }
    }

    for key in args.named.keys() {
        if !["width", "height", "margin"].contains(&key.as_str()) {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("argumento nomeado inesperado em page(): '{}'", key),
            )]);
        }
    }

    let width  = args.named.get("width") .and_then(extract_pt);
    let height = args.named.get("height").and_then(extract_pt);
    let margin = args.named.get("margin").and_then(extract_pt);

    Ok(Value::Content(Content::SetPage { width, height, margin }))
}

// ── Passo 156C (ADR-0061 Fase 1 sub-passo 1) — pad + hide ───────────────────

/// Coage `Value` para `Length`. Aceita `Length`, `Float` (interpretado em pt),
/// `Int` (idem). Retorna `None` para outros tipos.
fn extract_length(val: &Value) -> Option<Length> {
    match val {
        Value::Length(l) => Some(*l),
        Value::Float(f)  => Some(Length { abs: Abs(*f),         em: 0.0 }),
        Value::Int(i)    => Some(Length { abs: Abs(*i as f64),  em: 0.0 }),
        _                => None,
    }
}

/// `pad(body, left: ?, right: ?, top: ?, bottom: ?, x: ?, y: ?, rest: ?)`
/// → `Content::Pad`.
///
/// Resolve a precedência vanilla: específico (`left`/`right`/`top`/`bottom`) >
/// eixo (`x` cobre left+right; `y` cobre top+bottom) > `rest` (cobre os
/// quatro lados). Lados não especificados ficam a zero.
///
/// `body` posicional obrigatório (Content ou Str).
/// Padding negativo rejeitado por agora (perfil ADR-0054 graded; vanilla
/// aceita-o mas a semântica em cristalino fica para passo posterior).
pub fn native_pad(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    let body = match args.items.first() {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::Str(s))     => Content::text(s.as_str()),
        Some(other) => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("pad() espera content ou string como primeiro argumento, recebeu {}", other.type_name()),
        )]),
        None => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "pad() exige body como argumento posicional".to_string(),
        )]),
    };

    let mut left:   Option<Length> = None;
    let mut right:  Option<Length> = None;
    let mut top:    Option<Length> = None;
    let mut bottom: Option<Length> = None;
    let mut x_axis: Option<Length> = None;
    let mut y_axis: Option<Length> = None;
    let mut rest:   Option<Length> = None;

    for (key, value) in args.named.iter() {
        let len = extract_length(value).ok_or_else(|| vec![SourceDiagnostic::error(
            Span::detached(),
            format!("pad({}:) espera length, recebeu {}", key, value.type_name()),
        )])?;
        match key.as_str() {
            "left"   => left   = Some(len),
            "right"  => right  = Some(len),
            "top"    => top    = Some(len),
            "bottom" => bottom = Some(len),
            "x"      => x_axis = Some(len),
            "y"      => y_axis = Some(len),
            "rest"   => rest   = Some(len),
            other => return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("pad(): argumento nomeado inesperado '{}'", other),
            )]),
        }
    }

    // Precedência: específico > eixo > rest. Para cada lado, escolhe o
    // valor mais específico declarado.
    let resolved_left   = left  .or(x_axis).or(rest).unwrap_or(Length::ZERO);
    let resolved_right  = right .or(x_axis).or(rest).unwrap_or(Length::ZERO);
    let resolved_top    = top   .or(y_axis).or(rest).unwrap_or(Length::ZERO);
    let resolved_bottom = bottom.or(y_axis).or(rest).unwrap_or(Length::ZERO);

    // Validação: padding negativo rejeitado (perfil ADR-0054 graded).
    // Vanilla aceita; cristalino diverge intencionalmente neste passo
    // (margens negativas adiadas para passo dedicado quando layout
    // overflow tiver semântica clara).
    for (label, len) in [("left", resolved_left), ("right", resolved_right),
                         ("top", resolved_top),   ("bottom", resolved_bottom)] {
        if len.abs.0 < 0.0 || len.em < 0.0 {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("pad({}:): padding negativo não suportado neste passo (P156C)", label),
            )]);
        }
    }

    Ok(Value::Content(Content::Pad {
        body:    Box::new(body),
        padding: Sides::new(resolved_left, resolved_top, resolved_right, resolved_bottom),
    }))
}

/// `hide(body)` → `Content::Hide`. Sem argumentos nomeados.
pub fn native_hide(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    expect_no_named(&args.named)?;
    let body = match args.items.first() {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::Str(s))     => Content::text(s.as_str()),
        Some(other) => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("hide() espera content ou string, recebeu {}", other.type_name()),
        )]),
        None => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "hide() exige body como argumento posicional".to_string(),
        )]),
    };
    Ok(Value::Content(Content::Hide { body: Box::new(body) }))
}

// ── Passo 156D (ADR-0061 Fase 1 sub-passo 2) — h + v spacing ─────────────────

/// Resolve `weak: bool` em named args (ou default false). Erro hard se
/// tipo não-bool.
fn extract_weak(args: &Args, fn_name: &str) -> SourceResult<bool> {
    match args.named.get("weak") {
        Some(Value::Bool(b)) => Ok(*b),
        Some(other) => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("{}(weak:) espera bool, recebeu {}", fn_name, other.type_name()),
        )]),
        None => Ok(false),
    }
}

/// Lógica partilhada por `native_h` e `native_v`:
/// extrai `amount` (Length, posicional obrigatório), valida não-negativo,
/// resolve `weak`. Aceita Length, Float (interpretado em pt) ou Int (idem)
/// per `extract_length`.
fn build_spacing(
    args: &Args,
    fn_name: &str,
    valid_named: &[&str],
) -> SourceResult<(Length, bool)> {
    // amount posicional obrigatório
    let amount = match args.items.first() {
        Some(v) => extract_length(v).ok_or_else(|| vec![SourceDiagnostic::error(
            Span::detached(),
            format!("{}() espera amount como length, recebeu {}", fn_name, v.type_name()),
        )])?,
        None => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("{}() exige amount como argumento posicional", fn_name),
        )]),
    };

    // Validação: amount negativo rejeitado per perfil ADR-0054 graded
    // (vanilla aceita-o; cristalino diverge intencionalmente neste passo).
    if amount.abs.0 < 0.0 || amount.em < 0.0 {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("{}(): amount negativo não suportado neste passo (P156D)", fn_name),
        )]);
    }

    // Validação: rejeitar named args desconhecidos.
    for key in args.named.keys() {
        if !valid_named.contains(&key.as_str()) {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("{}(): argumento nomeado inesperado '{}'", fn_name, key),
            )]);
        }
    }

    let weak = extract_weak(args, fn_name)?;

    Ok((amount, weak))
}

/// `h(amount, weak: false)` → `Content::HSpace`.
///
/// `amount` Length posicional obrigatório. `weak` armazenado mas
/// comportamento de collapse adiado neste passo (perfil ADR-0054 graded).
/// Vanilla aceita `Fraction` para amount; cristalino só `Length` neste
/// passo (refino futuro per ADR-0061 §6.3).
pub fn native_h(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    let (amount, weak) = build_spacing(args, "h", &["weak"])?;
    Ok(Value::Content(Content::HSpace { amount, weak }))
}

/// `v(amount, weak: false)` → `Content::VSpace`.
///
/// Análogo a `native_h`, produz spacing primitive vertical.
pub fn native_v(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    let (amount, weak) = build_spacing(args, "v", &["weak"])?;
    Ok(Value::Content(Content::VSpace { amount, weak }))
}

// ── Passo 156E (ADR-0061 Fase 1 sub-passo 3) — pagebreak manual ──────────────

/// Coage `Value::Str` para `Parity` (`"even"` / `"odd"`).
/// Outros tipos ou strings → erro hard.
fn extract_parity(value: &Value) -> SourceResult<Parity> {
    match value {
        Value::Str(s) => match s.as_str() {
            "even" => Ok(Parity::Even),
            "odd"  => Ok(Parity::Odd),
            other  => Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("pagebreak(to:) deve ser \"even\" ou \"odd\", recebeu \"{}\"", other),
            )]),
        },
        other => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("pagebreak(to:) deve ser string, recebeu {}", other.type_name()),
        )]),
    }
}

// ── Passo 156G (ADR-0061 Fase 2 sub-passo 1) — block container ──────────────

/// `block(body, width: ?, height: ?, inset: ?, breakable: true)` →
/// `Content::Block`.
///
/// **Atributos** (subset Fase 1 per ADR-0054 graded):
/// - `body` posicional obrigatório (Content ou Str).
/// - `width: Length` ou `Float`/`Int` (interpretado em pt). Ausente == auto.
/// - `height` análogo.
/// - `inset: Length` (uniforme nos 4 lados; refino futuro para Sides
///   completo via dict).
/// - `breakable: bool` (default `true`).
///
/// **Scope-out** (refino futuro): outset, fill, stroke, radius, clip,
/// spacing, above/below, sticky.
pub fn native_block(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    let body = match args.items.first() {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::Str(s))     => Content::text(s.as_str()),
        Some(other) => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("block() espera content ou string como primeiro argumento, recebeu {}", other.type_name()),
        )]),
        // Body opcional em vanilla; aceitamos ausência como Empty.
        None => Content::Empty,
    };

    let mut width:     Option<Length> = None;
    let mut height:    Option<Length> = None;
    let mut inset_uniform: Option<Length> = None;
    let mut breakable: bool = true;

    for (key, value) in args.named.iter() {
        match key.as_str() {
            "width" => {
                width = Some(extract_length(value).ok_or_else(|| vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!("block(width:) espera length, recebeu {}", value.type_name()),
                )])?);
            }
            "height" => {
                height = Some(extract_length(value).ok_or_else(|| vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!("block(height:) espera length, recebeu {}", value.type_name()),
                )])?);
            }
            "inset" => {
                // Aceita Length uniforme; refino futuro para dict
                // `{left, right, top, bottom}` (per ADR-0054 graded).
                inset_uniform = Some(extract_length(value).ok_or_else(|| vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!("block(inset:) espera length uniforme, recebeu {}", value.type_name()),
                )])?);
            }
            "breakable" => match value {
                Value::Bool(b) => breakable = *b,
                other => return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!("block(breakable:) espera bool, recebeu {}", other.type_name()),
                )]),
            },
            other => return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("block(): argumento nomeado inesperado '{}' (atributos avançados scope-out per ADR-0054 graded — refino futuro)", other),
            )]),
        }
    }

    // Validação: width/height/inset negativos rejeitados (consistente
    // com pad em P156C; refino futuro para layout overflow).
    for (label, len) in [("width", width), ("height", height)].iter().filter_map(|(l, opt)| opt.map(|len| (*l, len))).collect::<Vec<_>>() {
        if len.abs.0 < 0.0 || len.em < 0.0 {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("block({}:): valor negativo não suportado neste passo (P156G)", label),
            )]);
        }
    }
    if let Some(i) = inset_uniform {
        if i.abs.0 < 0.0 || i.em < 0.0 {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "block(inset:): valor negativo não suportado neste passo (P156G)".to_string(),
            )]);
        }
    }

    let inset = match inset_uniform {
        Some(l) => Sides::uniform(l),
        None    => Sides::uniform(Length::ZERO),
    };

    Ok(Value::Content(Content::Block {
        body: Box::new(body),
        width,
        height,
        inset,
        breakable,
    }))
}

// ── Passo 156H (ADR-0061 Fase 2 sub-passo 2) — box inline container ─────────

/// `box(body, width: ?, height: ?, inset: ?, baseline: ?)` →
/// `Content::Boxed`.
///
/// **Atributos** (subset Fase 2 per ADR-0054 graded; padrão variant
/// rico reusado de `block` em P156G):
/// - `body` posicional opcional (Content ou Str; ausente → Empty).
/// - `width: Length`/`Float`/`Int` (em pt). Ausente == content-based.
/// - `height` análogo. Ausente == auto.
/// - `inset: Length` uniforme (refino futuro para Sides via dict).
/// - `baseline: Length` ajuste vertical (default zero).
///
/// **Scope-out** (refino futuro): outset, fill, stroke, radius, clip,
/// stroke-overhang.
///
/// Distinção material face a `block`: posicionamento **inline** vs
/// structural.
pub fn native_box(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    let body = match args.items.first() {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::Str(s))     => Content::text(s.as_str()),
        Some(other) => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("box() espera content ou string como primeiro argumento, recebeu {}", other.type_name()),
        )]),
        None => Content::Empty,  // body opcional (vanilla aceita)
    };

    let mut width:    Option<Length> = None;
    let mut height:   Option<Length> = None;
    let mut inset_uniform: Option<Length> = None;
    let mut baseline: Length = Length::ZERO;

    for (key, value) in args.named.iter() {
        match key.as_str() {
            "width" => {
                width = Some(extract_length(value).ok_or_else(|| vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!("box(width:) espera length, recebeu {}", value.type_name()),
                )])?);
            }
            "height" => {
                height = Some(extract_length(value).ok_or_else(|| vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!("box(height:) espera length, recebeu {}", value.type_name()),
                )])?);
            }
            "inset" => {
                inset_uniform = Some(extract_length(value).ok_or_else(|| vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!("box(inset:) espera length uniforme, recebeu {}", value.type_name()),
                )])?);
            }
            "baseline" => {
                baseline = extract_length(value).ok_or_else(|| vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!("box(baseline:) espera length, recebeu {}", value.type_name()),
                )])?;
            }
            other => return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("box(): argumento nomeado inesperado '{}' (atributos avançados scope-out per ADR-0054 graded — refino futuro)", other),
            )]),
        }
    }

    // Validação: width/height/inset negativos rejeitados (consistente
    // com block em P156G; baseline negativo ACEITE — move para cima).
    for (label, len) in [("width", width), ("height", height)].iter().filter_map(|(l, opt)| opt.map(|len| (*l, len))).collect::<Vec<_>>() {
        if len.abs.0 < 0.0 || len.em < 0.0 {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("box({}:): valor negativo não suportado neste passo (P156H)", label),
            )]);
        }
    }
    if let Some(i) = inset_uniform {
        if i.abs.0 < 0.0 || i.em < 0.0 {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "box(inset:): valor negativo não suportado neste passo (P156H)".to_string(),
            )]);
        }
    }

    let inset = match inset_uniform {
        Some(l) => Sides::uniform(l),
        None    => Sides::uniform(Length::ZERO),
    };

    Ok(Value::Content(Content::Boxed {
        body: Box::new(body),
        width, height, inset, baseline,
    }))
}

/// `pagebreak(weak: false, to: ?)` → `Content::Pagebreak`.
///
/// Sem argumentos posicionais. `weak` armazenado mas comportamento de
/// collapse adiado (perfil ADR-0054 graded; consistente com P156D).
/// `to` aceita string `"even"` ou `"odd"`; ausente → `None` (sem
/// ajuste de paridade).
pub fn native_pagebreak(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    if !args.items.is_empty() {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "pagebreak() não aceita argumentos posicionais".to_string(),
        )]);
    }

    let mut weak: bool = false;
    let mut to:   Option<Parity> = None;

    for (key, value) in args.named.iter() {
        match key.as_str() {
            "weak" => match value {
                Value::Bool(b) => weak = *b,
                other => return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!("pagebreak(weak:) espera bool, recebeu {}", other.type_name()),
                )]),
            },
            "to" => to = Some(extract_parity(value)?),
            other => return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("pagebreak(): argumento nomeado inesperado '{}'", other),
            )]),
        }
    }

    Ok(Value::Content(Content::Pagebreak { weak, to }))
}

