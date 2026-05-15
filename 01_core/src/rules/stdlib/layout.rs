//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/stdlib.md
//! @prompt-hash 68fc3823
//! @layer L1
//! @updated 2026-04-23
//!
//! Funções nativas de layout (align, place, grid, page).
//! Extraído de `stdlib.rs` no Passo 96.5 conforme ADR-0037.

use super::expect_no_named;
use crate::entities::file_id::FileId;

use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::dir::Dir;
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
        // P223 — accept "float" e "clearance" novos named args.
        if !["dx", "dy", "scope", "float", "clearance"].contains(&key.as_str()) {
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

    // P223 — extract `float` (default false). Semantic real adiada per
    // ADR-0054 graded; precedente N=4 cumulativo weak/breakable/float.
    let float = match args.named.get("float") {
        Some(Value::Bool(b)) => *b,
        Some(other) => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("place(float): espera Bool, recebeu {}", other.type_name()),
        )]),
        None => false,
    };

    // P223 — extract `clearance` (default None). Reuso extract_length (N=8 → 9).
    // Validar não-negativo (paridade pattern P156I Stack.spacing).
    let clearance = match args.named.get("clearance") {
        Some(val) => {
            let len = extract_length(val).ok_or_else(|| vec![SourceDiagnostic::error(
                Span::detached(),
                format!("place(clearance): espera length, recebeu {}", val.type_name()),
            )])?;
            if len.abs.0 < 0.0 || len.em < 0.0 {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    "place(clearance): valor negativo não suportado".to_string(),
                )]);
            }
            Some(len)
        }
        None => None,
    };

    // P223 — DEBT-37 §"Divergência" restaurada (Decisão 3 Opção α):
    // vanilla `place` com `scope: "parent"` exige `float: true`.
    if matches!(scope, crate::entities::layout_types::PlaceScope::Parent) && !float {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "place: scope \"parent\" requer float: true (paridade vanilla; sem float, scope \"parent\" não tem semantic flutuante; fecha divergência DEBT-37 documentada)".to_string(),
        )]);
    }

    let body = args.items.iter()
        .find_map(|v| if let Value::Content(c) = v { Some(c.clone()) } else { None })
        .ok_or_else(|| vec![SourceDiagnostic::error(Span::detached(),
            "place() exige um bloco de conteúdo".to_string())])?;

    Ok(Value::Content(Content::Place {
        alignment,
        dx,
        dy,
        scope,
        float,
        clearance,
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

/// `pub(super)` per P157A — reuso N=2 cross-módulo (sibling
/// `stdlib/structural.rs::native_table`). Subpadrão emergente
/// análogo a `extract_length` (N=7); promoção a helper público
/// diferida até atingir N=3-4.
pub(super) fn extract_tracks(val: Option<&Value>) -> Vec<TrackSizing> {
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
        // P224 + P227 + P228 — accept named args
        // (gutter/align/inset/header/footer/stroke/fill).
        if !["columns", "rows", "gutter", "align", "inset", "header", "footer", "stroke", "fill"]
            .contains(&key.as_str())
        {
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

    // P224.A — extract gutter (Length opcional; negativo rejeitado).
    let gutter = match args.named.get("gutter") {
        Some(val) => {
            let len = extract_length(val).ok_or_else(|| vec![SourceDiagnostic::error(
                Span::detached(),
                format!("grid(gutter): espera length, recebeu {}", val.type_name()),
            )])?;
            if len.abs.0 < 0.0 || len.em < 0.0 {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    "grid(gutter): valor negativo não suportado".to_string(),
                )]);
            }
            Some(len)
        }
        None => None,
    };

    // P224.A — extract align (Value::Align direct; default None == top-left).
    let align = match args.named.get("align") {
        Some(Value::Align(a)) => Some(*a),
        Some(other) => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("grid(align): espera alignment, recebeu {}", other.type_name()),
        )]),
        None => None,
    };

    // P224.A — extract inset (subset graded: Length uniforme apenas;
    // per-side refino futuro candidato).
    let inset: crate::entities::sides::Sides<Length> = match args.named.get("inset") {
        Some(Value::Length(l)) => crate::entities::sides::Sides::uniform(*l),
        Some(other) => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("grid(inset): espera length, recebeu {}", other.type_name()),
        )]),
        None => crate::entities::sides::Sides::uniform(Length::pt(0.0)),
    };

    // P224.B — extract header/footer (Content opcional).
    let header = match args.named.get("header") {
        Some(Value::Content(c)) => Some(Box::new(c.clone())),
        Some(other) => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("grid(header): espera content, recebeu {}", other.type_name()),
        )]),
        None => None,
    };
    let footer = match args.named.get("footer") {
        Some(Value::Content(c)) => Some(Box::new(c.clone())),
        Some(other) => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("grid(footer): espera content, recebeu {}", other.type_name()),
        )]),
        None => None,
    };

    let cells: Vec<Content> = args.items.iter()
        .filter_map(|v| if let Value::Content(c) = v { Some(c.clone()) } else { None })
        .collect();
    // P227 — extract stroke (Length/Color/Stroke shorthand via extract_stroke).
    let stroke = match args.named.get("stroke") {
        Some(val) => Some(extract_stroke(val, "grid", "stroke")?),
        None => None,
    };

    // P228 — extract fill (Opção α: apenas Value::Color; rejeita outros).
    let fill = match args.named.get("fill") {
        Some(Value::Color(c)) => Some(*c),
        Some(other) => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("grid(fill): espera Color, recebeu {}", other.type_name()),
        )]),
        None => None,
    };

    Ok(Value::Content(Content::Grid {
        columns, rows, cells,
        gutter, align, inset, header, footer,
        stroke, fill,
    }))
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

/// P227 — Coage `Value` para `Stroke` aceitando shorthands paridade
/// vanilla (Opção β):
/// - `Value::Length(l)` → `Stroke { paint: Color::BLACK, thickness: l.to_pt() }`.
/// - `Value::Color(c)` → `Stroke { paint: c, thickness: 1.0 }` (default 1pt).
/// - `Value::Stroke(s)` → `s.clone()`.
/// - Outros tipos: erro hard.
///
/// `thickness <= 0` rejeitado (paridade vanilla).
pub(super) fn extract_stroke(val: &Value, fn_name: &str, field: &str) -> SourceResult<crate::entities::geometry::Stroke> {
    use crate::entities::geometry::Stroke;
    use crate::entities::layout_types::Color;
    let stroke = match val {
        Value::Length(l) => {
            let thickness = l.abs.to_pt();
            Stroke { paint: Color::rgb(0, 0, 0), thickness }
        }
        Value::Color(c) => Stroke { paint: *c, thickness: 1.0 },
        Value::Stroke(s) => s.clone(),
        other => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("{}({}): espera Length / Color / Stroke, recebeu {}",
                fn_name, field, other.type_name()),
        )]),
    };
    if stroke.thickness <= 0.0 {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("{}({}): thickness deve ser > 0 (recebeu {})",
                fn_name, field, stroke.thickness),
        )]);
    }
    Ok(stroke)
}

/// `pad(body, left: ?, right: ?, top: ?, bottom: ?, x: ?, y: ?, rest: ?)`
/// → `Content::Pad`.
///
/// Resolve a precedência vanilla: específico (`left`/`right`/`top`/`bottom`) >
/// eixo (`x` cobre left+right; `y` cobre top+bottom) > `rest` (cobre os
/// quatro lados). Lados não especificados ficam `None` (P156L refino;
/// per ADR-0064 Caso C — `None` ↔ default vanilla zero, resolvido em
/// momento de uso no Layouter).
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

    let sides = extract_sides_lengths(args, "pad")?;

    Ok(Value::Content(Content::Pad {
        body: Box::new(body),
        sides,
    }))
}

/// Helper Passo 156L: parse named args left/top/right/bottom + atalhos
/// x/y/rest, retornando `Sides<Option<Length>>`. Precedência vanilla:
/// específico > eixo > rest. Lados não declarados ficam `None`.
///
/// Pré-decisão (per diagnóstico §5.2): helper privado, não-genérico
/// (toma `Length` directamente). Promoção a genérico/público diferida
/// até segundo reuso (padrão N=2 mínimo para promoção).
///
/// Validação: cada lado declarado rejeita negativos (perfil ADR-0054
/// graded — vanilla aceita; cristalino diverge intencionalmente).
/// Named arg desconhecido rejeitado.
fn extract_sides_lengths(args: &Args, fn_name: &str) -> SourceResult<Sides<Option<Length>>> {
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
            format!("{}({}:) espera length, recebeu {}", fn_name, key, value.type_name()),
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
                format!("{}(): argumento nomeado inesperado '{}'", fn_name, other),
            )]),
        }
    }

    // Precedência: específico > eixo > rest.
    let resolved_left   = left  .or(x_axis).or(rest);
    let resolved_right  = right .or(x_axis).or(rest);
    let resolved_top    = top   .or(y_axis).or(rest);
    let resolved_bottom = bottom.or(y_axis).or(rest);

    // Validação: rejeitar negativos em qualquer lado declarado.
    for (label, opt) in [("left", resolved_left), ("right", resolved_right),
                         ("top",  resolved_top),  ("bottom", resolved_bottom)] {
        if let Some(len) = opt {
            if len.abs.0 < 0.0 || len.em < 0.0 {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!("{}({}:): padding negativo não suportado neste passo (P156C/L)", fn_name, label),
                )]);
            }
        }
    }

    Ok(Sides {
        left:   resolved_left,
        top:    resolved_top,
        right:  resolved_right,
        bottom: resolved_bottom,
    })
}

/// **P242 (M9d / M7+5)** — extrai `Corners<Length>` a partir de um
/// `Value` único (named arg `radius:` em `block`/`box`). Aceita:
/// - `Value::Length(L)` (ou coerções via `extract_length`) → uniforme
///   `Corners::uniform(L)`.
/// - `Value::Dict(d)` com keys `top-left` / `top-right` /
///   `bottom-right` / `bottom-left` / `top` / `bottom` / `left` /
///   `right` / `rest`.
///
/// **Precedência** (paridade `extract_sides_lengths` per ADR-0064
/// Caso C): canto específico > eixo (top/bottom/left/right) > `rest`.
/// Cantos omitidos preservam-se em `Length::ZERO`.
///
/// **Validação**: negativos rejeitados (paridade `block(radius)` P231).
fn extract_corners_length_value(value: &Value, fn_name: &str) -> SourceResult<crate::entities::corners::Corners<Length>> {
    use crate::entities::corners::Corners;
    // Caso 1: Length uniforme.
    if let Some(len) = extract_length(value) {
        if len.abs.0 < 0.0 || len.em < 0.0 {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("{}(radius): negativo rejeitado", fn_name),
            )]);
        }
        return Ok(Corners::uniform(len));
    }
    // Caso 2: Dict por canto.
    if let Value::Dict(d) = value {
        let mut tl: Option<Length> = None;
        let mut tr: Option<Length> = None;
        let mut br: Option<Length> = None;
        let mut bl: Option<Length> = None;
        let mut top: Option<Length> = None;
        let mut bottom: Option<Length> = None;
        let mut left: Option<Length> = None;
        let mut right: Option<Length> = None;
        let mut rest: Option<Length> = None;
        for (key, val) in d.iter() {
            let len = extract_length(val).ok_or_else(|| vec![SourceDiagnostic::error(
                Span::detached(),
                format!("{}(radius.{}:) espera length, recebeu {}", fn_name, key, val.type_name()),
            )])?;
            if len.abs.0 < 0.0 || len.em < 0.0 {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!("{}(radius.{}:) negativo rejeitado", fn_name, key),
                )]);
            }
            match key.as_str() {
                "top-left"     => tl     = Some(len),
                "top-right"    => tr     = Some(len),
                "bottom-right" => br     = Some(len),
                "bottom-left"  => bl     = Some(len),
                "top"          => top    = Some(len),
                "bottom"       => bottom = Some(len),
                "left"         => left   = Some(len),
                "right"        => right  = Some(len),
                "rest"         => rest   = Some(len),
                other => return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!("{}(radius): chave de canto inesperada '{}'", fn_name, other),
                )]),
            }
        }
        // Precedência: específico > eixo > rest.
        let zero = Length::ZERO;
        let resolved_tl = tl.or(top).or(left).or(rest).unwrap_or(zero);
        let resolved_tr = tr.or(top).or(right).or(rest).unwrap_or(zero);
        let resolved_br = br.or(bottom).or(right).or(rest).unwrap_or(zero);
        let resolved_bl = bl.or(bottom).or(left).or(rest).unwrap_or(zero);
        return Ok(Corners::new(resolved_tl, resolved_tr, resolved_br, resolved_bl));
    }
    Err(vec![SourceDiagnostic::error(
        Span::detached(),
        format!("{}(radius:) espera length ou dict por canto, recebeu {}", fn_name, value.type_name()),
    )])
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
            // P231 — aceitar outset/radius/clip (parsing pós-loop).
            // P247 — aceitar fill/stroke (parsing pós-loop paralelo).
            "outset" | "radius" | "clip" | "fill" | "stroke" => {},
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

    // P231 — extract 3 cosméticos (outset/radius/clip).
    // outset: Length uniforme aceito (subset; per-side refino futuro).
    let outset = match args.named.get("outset") {
        Some(val) => {
            let len = extract_length(val).ok_or_else(|| vec![SourceDiagnostic::error(
                Span::detached(),
                format!("block(outset): espera length, recebeu {}", val.type_name()),
            )])?;
            if len.abs.0 < 0.0 || len.em < 0.0 {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    "block(outset): negativo rejeitado".to_string(),
                )]);
            }
            Sides::uniform(len)
        }
        None => Sides::uniform(Length::ZERO),
    };
    // P242 — radius `Corners<Length>` (refino face P231 `Option<Length>`).
    // Aceita Length uniforme OR Dict por canto via helper centralizado.
    let radius = match args.named.get("radius") {
        Some(val) => extract_corners_length_value(val, "block")?,
        None => crate::entities::corners::Corners::uniform(Length::ZERO),
    };
    let clip = match args.named.get("clip") {
        Some(Value::Bool(b)) => *b,
        Some(other) => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("block(clip): espera Bool, recebeu {}", other.type_name()),
        )]),
        None => false,
    };

    // P247 — fill: aceita Value::Color directo (paridade pattern Grid/
    // Table inline). Refino futuro para Paint enum quando ADR dedicada.
    let fill = match args.named.get("fill") {
        Some(Value::Color(c)) => Some(*c),
        Some(other) => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("block(fill): espera Color, recebeu {}", other.type_name()),
        )]),
        None => None,
    };

    // P247 — stroke: reusa `extract_stroke` (helper pré-existente
    // P227 stdlib/layout.rs:351). Aceita Length/Color/Stroke shorthand.
    let stroke = match args.named.get("stroke") {
        Some(val) => Some(extract_stroke(val, "block", "stroke")?),
        None      => None,
    };

    Ok(Value::Content(Content::Block {
        body: Box::new(body),
        width,
        height,
        inset,
        breakable,
        outset,
        radius,
        clip,
        fill,
        stroke,
    }))
}

// ── Passo 156I (ADR-0061 Fase 2 sub-passo 3) — stack compositivo ────────────

/// Coage `Value::Str` para `Dir` (`"ltr"`/`"rtl"`/`"ttb"`/`"btt"`).
fn extract_dir(value: &Value) -> SourceResult<Dir> {
    match value {
        Value::Str(s) => match s.as_str() {
            "ltr" => Ok(Dir::LTR),
            "rtl" => Ok(Dir::RTL),
            "ttb" => Ok(Dir::TTB),
            "btt" => Ok(Dir::BTT),
            other => Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("stack(dir:) deve ser \"ltr\"/\"rtl\"/\"ttb\"/\"btt\", recebeu \"{}\"", other),
            )]),
        },
        other => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("stack(dir:) deve ser string, recebeu {}", other.type_name()),
        )]),
    }
}

/// `stack(dir: "ttb", spacing: ?, ..children)` → `Content::Stack`.
///
/// **Atributos** (Fase 2 per ADR-0054 graded; **último sub-passo Fase 2**;
/// atinge target 72% Layout):
/// - `children` variádicos posicionais (Content ou Str).
/// - `dir: Str` (`"ltr"`/`"rtl"`/`"ttb"`/`"btt"`); default `"ttb"`.
/// - `spacing: Length`; default `None` (zero).
///
/// Sem atributos vanilla scope-out (vanilla stack tem apenas estes 3).
pub fn native_stack(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    let mut dir: Dir = Dir::default();  // TTB
    let mut spacing: Option<Length> = None;

    for (key, value) in args.named.iter() {
        match key.as_str() {
            "dir"     => dir = extract_dir(value)?,
            "spacing" => {
                let len = extract_length(value).ok_or_else(|| vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!("stack(spacing:) espera length, recebeu {}", value.type_name()),
                )])?;
                if len.abs.0 < 0.0 || len.em < 0.0 {
                    return Err(vec![SourceDiagnostic::error(
                        Span::detached(),
                        "stack(spacing:): valor negativo não suportado neste passo (P156I)".to_string(),
                    )]);
                }
                spacing = Some(len);
            }
            other => return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("stack(): argumento nomeado inesperado '{}'", other),
            )]),
        }
    }

    // Children variádicos: iterar args.items, aceitar Content ou Str.
    let mut children: Vec<Content> = Vec::with_capacity(args.items.len());
    for v in args.items.iter() {
        match v {
            Value::Content(c) => children.push(c.clone()),
            Value::Str(s)     => children.push(Content::text(s.as_str())),
            other => return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("stack(): children devem ser content ou string, recebeu {}", other.type_name()),
            )]),
        }
    }

    Ok(Value::Content(Content::Stack {
        children: std::sync::Arc::from(children),
        dir,
        spacing,
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
            // P231 — aceitar outset/radius/clip (parsing pós-loop paridade block).
            // P247 — aceitar fill/stroke paralelo block.
            "outset" | "radius" | "clip" | "fill" | "stroke" => {},
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

    // P231 — extract 3 cosméticos (outset/radius/clip) paralelo Block.
    let outset = match args.named.get("outset") {
        Some(val) => {
            let len = extract_length(val).ok_or_else(|| vec![SourceDiagnostic::error(
                Span::detached(),
                format!("box(outset): espera length, recebeu {}", val.type_name()),
            )])?;
            if len.abs.0 < 0.0 || len.em < 0.0 {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    "box(outset): negativo rejeitado".to_string(),
                )]);
            }
            Sides::uniform(len)
        }
        None => Sides::uniform(Length::ZERO),
    };
    // P242 — radius `Corners<Length>` paralelo block. Aceita Length
    // uniforme OR Dict por canto via helper centralizado.
    let radius = match args.named.get("radius") {
        Some(val) => extract_corners_length_value(val, "box")?,
        None => crate::entities::corners::Corners::uniform(Length::ZERO),
    };
    let clip = match args.named.get("clip") {
        Some(Value::Bool(b)) => *b,
        Some(other) => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("box(clip): espera Bool, recebeu {}", other.type_name()),
        )]),
        None => false,
    };

    // P247 — fill paralelo block.
    let fill = match args.named.get("fill") {
        Some(Value::Color(c)) => Some(*c),
        Some(other) => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("box(fill): espera Color, recebeu {}", other.type_name()),
        )]),
        None => None,
    };

    // P247 — stroke reusa `extract_stroke` paralelo block.
    let stroke = match args.named.get("stroke") {
        Some(val) => Some(extract_stroke(val, "box", "stroke")?),
        None      => None,
    };

    Ok(Value::Content(Content::Boxed {
        body: Box::new(body),
        width, height, inset, baseline,
        outset, radius, clip,
        fill, stroke,
    }))
}

// ── Passo 156J (ADR-0061 Fase 3 sub-passo 1) — repeat ──────────────────────

/// `repeat(body, gap: ?, justify: true)` → `Content::Repeat`.
///
/// **Primeira aplicação Fase 3** declarada em ADR-0061. Caso de uso
/// primário: TOC dot leaders `#box(width: 1fr, repeat[.])`.
///
/// **Atributos** (paridade vanilla `RepeatElem`):
/// - `body` posicional obrigatório (Content ou Str).
/// - `gap: Length`/`Float`/`Int` (em pt). Ausente == zero (padrão
///   Smart→Option N=6 da série).
/// - `justify: bool`. Default **`true`** (paridade vanilla;
///   distribuição de espaço residual diferida per ADR-0054).
///
/// **Limitação aceite (perfil ADR-0054 graded)**: o algoritmo de
/// runtime que calcula `floor(available / (body_width + gap))`
/// está diferido — Layouter executa single-render do body
/// (suficiente para paridade estrutural, exhaustive pattern-match
/// e walk de counters/labels dentro do body).
pub fn native_repeat(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    let body = match args.items.first() {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::Str(s))     => Content::text(s.as_str()),
        Some(other) => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("repeat() espera content ou string como primeiro argumento, recebeu {}", other.type_name()),
        )]),
        None => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "repeat() exige body como argumento posicional".to_string(),
        )]),
    };

    let mut gap:     Option<Length> = None;
    let mut justify: bool = true;  // default vanilla

    for (key, value) in args.named.iter() {
        match key.as_str() {
            "gap" => {
                let len = extract_length(value).ok_or_else(|| vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!("repeat(gap:) espera length, recebeu {}", value.type_name()),
                )])?;
                if len.abs.0 < 0.0 || len.em < 0.0 {
                    return Err(vec![SourceDiagnostic::error(
                        Span::detached(),
                        "repeat(gap:): valor negativo não suportado neste passo (P156J)".to_string(),
                    )]);
                }
                gap = Some(len);
            }
            "justify" => match value {
                Value::Bool(b) => justify = *b,
                other => return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!("repeat(justify:) espera bool, recebeu {}", other.type_name()),
                )]),
            },
            other => return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("repeat(): argumento nomeado inesperado '{}'", other),
            )]),
        }
    }

    Ok(Value::Content(Content::Repeat {
        body: Box::new(body),
        gap,
        justify,
    }))
}

/// **P218 helper** — extrai `count: usize` posicional obrigatório
/// para `native_columns`. Rejeita `count = 0` (paridade `NonZeroUsize`
/// vanilla per ADR-0054 graded).
///
/// Distinto de `extract_length` (Length) e `extract_usize_or_none_min`
/// (P157B; named opcional). N=1 pós-P218; promoção a helper público
/// diferida a N=2-3 reuso.
fn extract_count(args: &Args, fn_name: &str) -> SourceResult<usize> {
    match args.items.first() {
        Some(Value::Int(n)) => {
            if *n < 1 {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!("{}(count): count deve ser >= 1, recebeu {}", fn_name, n),
                )]);
            }
            Ok(*n as usize)
        }
        Some(other) => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("{}(count): espera Int, recebeu {}", fn_name, other.type_name()),
        )]),
        None => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("{}: argumento posicional count obrigatório ausente", fn_name),
        )]),
    }
}

/// **P218 (DEBT-56 sub-fase b — Layout Fase 3)** —
/// `columns(count, body, gutter: ?)` → `Content::Columns`.
///
/// Forma: `#columns(2)[body]` ou `#columns(2, gutter: 1em)[body]`.
///
/// Aditivo P218 — arm Layouter é stub transparente em P217 (consumer
/// real P219 sub-fase (b) DEBT-56).
///
/// Validações:
/// - `count >= 1` rejeita `count = 0` (paridade `NonZeroUsize`
///   vanilla per ADR-0054 graded).
/// - `gutter` negativo rejeitado (paridade `Stack.spacing` P156I,
///   `Repeat.gap` P156J).
/// - Named arg desconhecido rejeitado.
/// - Body `Value::Content` ou `Value::Str` obrigatório.
pub fn native_columns(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    // 1. Extract count (posicional [0] obrigatório).
    let count = extract_count(args, "columns")?;

    // 2. Extract body (posicional [1], Content ou Str).
    let body = match args.items.get(1) {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::Str(s))     => Content::text(s.as_str()),
        Some(other) => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("columns(body): espera Content ou Str, recebeu {}", other.type_name()),
        )]),
        None => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "columns: argumento posicional body obrigatório ausente".to_string(),
        )]),
    };

    // 3. Validate no extra positionals.
    if args.items.len() > 2 {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("columns: aceita 2 posicionais (count, body), recebeu {}", args.items.len()),
        )]);
    }

    // 4. Extract gutter (named opcional, Option<Length>); validar negativo.
    let mut gutter: Option<Length> = None;
    for (key, value) in args.named.iter() {
        match key.as_str() {
            "gutter" => {
                let len = extract_length(value).ok_or_else(|| vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!("columns(gutter:) espera length, recebeu {}", value.type_name()),
                )])?;
                if len.abs.0 < 0.0 || len.em < 0.0 {
                    return Err(vec![SourceDiagnostic::error(
                        Span::detached(),
                        "columns(gutter:): valor negativo não suportado".to_string(),
                    )]);
                }
                gutter = Some(len);
            }
            other => return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("columns(): argumento nomeado inesperado '{}'", other),
            )]),
        }
    }

    Ok(Value::Content(Content::Columns {
        count,
        gutter,
        body: Box::new(body),
    }))
}

/// `colbreak(weak: false)` → `Content::Colbreak` — Passo 220
/// (ADR-0078 PROPOSTO sub-fase b 4/4 — fecha sub-fase b).
///
/// Forma: `#colbreak()` ou `#colbreak(weak: true)`.
///
/// Sem argumentos posicionais. `weak` armazenado mas semantic
/// de collapse adiada (paridade `Pagebreak.weak` P156E).
/// **Sem `to:`** — vanilla `ColbreakElem` não tem (paridade só
/// faz sentido em páginas).
///
/// **Semantic graded P220** — colbreak downgrade a pagebreak
/// pós-P219 (Opção B graded; sem multi-region flow real).
/// Refino multi-region salto entre colunas reais é
/// P-Layout-Fase4 candidato (não-reservado).
pub fn native_colbreak(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    if !args.items.is_empty() {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "colbreak() não aceita argumentos posicionais".to_string(),
        )]);
    }

    let mut weak: bool = false;

    for (key, value) in args.named.iter() {
        match key.as_str() {
            "weak" => match value {
                Value::Bool(b) => weak = *b,
                other => return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!("colbreak(weak:) espera bool, recebeu {}", other.type_name()),
                )]),
            },
            other => return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("colbreak(): argumento nomeado inesperado '{}' (esperado: weak)", other),
            )]),
        }
    }

    Ok(Value::Content(Content::Colbreak { weak }))
}

/// `measure(body) -> dict(width: length, height: length)` —
/// Passo 222 (Fase 4 Layout candidata; ADR-0066 §"Plano
/// promoção" Bloco C cross-módulo primeira materialização
/// parcial).
///
/// Forma: `#measure([Hello world])` ou `#measure("text")`.
///
/// **Semantic graded P222** — opera sobre Content evaluated
/// (single-pass via helper `measure_content` em `layout/helpers.rs`).
/// Runtime queries genuínas (counter values, labels resolution)
/// continuam diferidas per ADR-0066 PROPOSTO. Width override
/// (`measure(body, width: 5cm)`) **scope-out** per Opção β
/// graded ADR-0054; refino futuro candidato NÃO-reservado.
///
/// Retorna `Value::Dict` com keys `"width"` + `"height"` ambos
/// `Value::Length` (paridade vanilla `measure(body).width`
/// observable). Para conteúdo complexo (texto multi-linha,
/// equações), helper retorna aproximação conservadora (0, 0)
/// — limitação documentada.
pub fn native_measure(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    use crate::rules::layout::helpers::measure_content;
    use ecow::EcoString;
    use indexmap::IndexMap;
    use rustc_hash::FxBuildHasher;

    // 1. Extract body (posicional [0], Content ou Str).
    let body = match args.items.first() {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::Str(s))     => Content::text(s.as_str()),
        Some(other) => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("measure(body): espera Content ou Str, recebeu {}", other.type_name()),
        )]),
        None => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "measure: argumento posicional body obrigatório ausente".to_string(),
        )]),
    };

    // 2. Reject extra positionals.
    if args.items.len() > 1 {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("measure: aceita 1 posicional (body), recebeu {}", args.items.len()),
        )]);
    }

    // 3. Reject all named args (paridade Opção β graded;
    //    `width` override scope-out per ADR-0054).
    if let Some(key) = args.named.keys().next() {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("measure: named arg `{}` não suportado (paridade graded; refino futuro candidato NÃO-reservado per ADR-0054)", key),
        )]);
    }

    // 4. Chamar helper privado `measure_content` (pub(crate) em P222).
    //    available_w default = f64::INFINITY (sem constraint width);
    //    helper retorna (0, 0) para Empty + conteúdo complexo.
    let (width_pt, height_pt) = measure_content(&body, f64::INFINITY);

    // 5. Build Dict { width: Length, height: Length }.
    let mut dict: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
    dict.insert("width".into(),  Value::Length(Length::pt(width_pt)));
    dict.insert("height".into(), Value::Length(Length::pt(height_pt)));

    Ok(Value::Dict(dict))
}

/// `stroke(paint: ?, thickness: ?)` → `Value::Stroke` — Passo 227
/// (ADR-0079 PROPOSTO Categoria A.1 Fase 5 Layout candidata).
///
/// Forma: `#stroke(thickness: 2pt)` ou `#stroke(paint: red, thickness: 1pt)`.
///
/// **Defaults paridade vanilla**:
/// - `paint: Color` (named opcional; default `Color::rgb(0, 0, 0)` BLACK).
/// - `thickness: Length` (named opcional; default 1.0 pt).
///
/// **Validações**:
/// - Sem argumentos posicionais.
/// - `thickness > 0` (rejeita 0 e negativos).
/// - Named args restritos a `paint` + `thickness`.
pub fn native_stroke(_ctx: &mut EvalContext, args: &Args, _world: &dyn crate::contracts::world::World, _current_file: FileId, _figure_numbering: Option<&str>) -> SourceResult<Value> {
    use crate::entities::geometry::Stroke;
    use crate::entities::layout_types::Color;

    if !args.items.is_empty() {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "stroke() não aceita argumentos posicionais".to_string(),
        )]);
    }

    let paint = match args.named.get("paint") {
        Some(Value::Color(c)) => *c,
        Some(other) => return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("stroke(paint): espera Color, recebeu {}", other.type_name()),
        )]),
        None => Color::rgb(0, 0, 0),
    };

    let thickness = match args.named.get("thickness") {
        Some(val) => {
            let len = extract_length(val).ok_or_else(|| vec![SourceDiagnostic::error(
                Span::detached(),
                format!("stroke(thickness): espera length, recebeu {}", val.type_name()),
            )])?;
            len.abs.to_pt()
        }
        None => 1.0,
    };

    if thickness <= 0.0 {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("stroke(thickness): deve ser > 0 (recebeu {})", thickness),
        )]);
    }

    for key in args.named.keys() {
        if !["paint", "thickness"].contains(&key.as_str()) {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("stroke(): argumento nomeado inesperado '{}' (esperado: paint, thickness)", key),
            )]);
        }
    }

    Ok(Value::Stroke(Stroke { paint, thickness }))
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

