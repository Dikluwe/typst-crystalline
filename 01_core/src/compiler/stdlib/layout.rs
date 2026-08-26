//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/layout.md
//! @prompt-hash 6fb8937d
//! @layer L1
//! @updated 2026-04-23
//!
//! Funções nativas de layout (align, place, grid, page).
//! Extraído de `stdlib.rs` no Passo 96.5 conforme ADR-0037.

use super::expect_no_named;
use crate::entities::file_id::FileId;
use ecow::EcoString;

use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::dir::Dir;
use crate::entities::layout_types::{Abs, Align2D, Length, TrackSizing};
use crate::entities::parity::Parity;
use crate::entities::sides::Sides;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;

// ── Align / Place (Passo 82) ────────────────────────────────────────────────

/// `align(alignment, body)` → `Content::Align`.
///
/// `alignment` aceita `Value::Align` (sintaxe preferida pós-Passo 84.5,
/// ex: `align(center + bottom, ...)`) ou `Value::Str` (sintaxe legacy,
/// ex: `align("center", ...)`) — ver DEBT-36 (encerrado).
/// `body` é o primeiro argumento posicional do tipo Content.
pub fn native_align(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    expect_no_named(&args.named)?;

    let alignment = extract_alignment(args, Align2D::default());

    let body = args
        .items
        .iter()
        .find_map(|v| if let Value::Content(c) = v { Some(c.clone()) } else { None })
        .ok_or_else(|| {
            vec![SourceDiagnostic::error(
                args.span,
                "align() exige um bloco de conteúdo".to_string(),
            )]
        })?;

    Ok(Value::Content(Content::align(alignment, body)))
}

/// `place(alignment, dx?, dy?, scope?, body)` → `Content::Place`.
///
/// `dx`/`dy` em pt deslocam o conteúdo a partir da posição alinhada.
/// `scope` (Passo 84.6, encerra DEBT-37): `"column"` (default — ancora à
/// célula activa de Grid, ou à página fora de Grid) ou `"parent"` (ancora
/// sempre à página). Aceita string ou omissão.
pub fn native_place(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    for key in args.named.keys() {
        // P223 — accept "float" e "clearance" novos named args.
        if !["dx", "dy", "scope", "float", "clearance"].contains(&key.as_str()) {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("argumento nomeado inesperado em place(): '{}'", key),
            )]);
        }
    }

    fn extract_pt(val: &Value) -> f64 {
        match val {
            Value::Float(f) => *f,
            Value::Int(i) => *i as f64,
            Value::Length(l) => l.abs.to_pt(),
            _ => 0.0, // neutro: N16[β] — Value não-dimensional retorna 0.0 na extracção de pontos
        }
    }

    // Default "top-left" para Place, "left" (default Align2D) para Align.
    let alignment = extract_alignment(
        args,
        Align2D {
            h: Some(crate::entities::layout_types::HAlign::Left),
            v: Some(crate::entities::layout_types::VAlign::Top),
        },
    );

    let dx = args.named.get("dx").map(extract_pt).unwrap_or(0.0);
    let dy = args.named.get("dy").map(extract_pt).unwrap_or(0.0);

    let scope = match args.named.get("scope") {
        Some(Value::Str(s)) => match s.as_str() {
            "column" => crate::entities::layout_types::PlaceScope::Column,
            "parent" => crate::entities::layout_types::PlaceScope::Parent,
            other => {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!(
                    "place(): scope deve ser \"column\" ou \"parent\", recebeu \"{}\"",
                    other
                ),
                )])
            }
        },
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("place(): scope deve ser string, recebeu {}", other.type_name()),
            )])
        }
        None => crate::entities::layout_types::PlaceScope::default(),
    };

    // P223 — extract `float` (default false). Semantic real adiada per
    // ADR-0054 graded; precedente N=4 cumulativo weak/breakable/float.
    let float = match args.named.get("float") {
        Some(Value::Bool(b)) => *b,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("place(float): espera Bool, recebeu {}", other.type_name()),
            )])
        }
        None => false,
    };

    // P223 — extract `clearance` (default None). Reuso extract_length (N=8 → 9).
    // Validar não-negativo (paridade pattern P156I Stack.spacing).
    let clearance = match args.named.get("clearance") {
        Some(val) => {
            let len = extract_length(val).ok_or_else(|| {
                vec![SourceDiagnostic::error(
                    args.span,
                    format!(
                        "place(clearance): espera length, recebeu {}",
                        val.type_name()
                    ),
                )]
            })?;
            if len.abs.0 < 0.0 || len.em < 0.0 {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
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
            args.span,
            "place: scope \"parent\" requer float: true (paridade vanilla; sem float, scope \"parent\" não tem semantic flutuante; fecha divergência DEBT-37 documentada)".to_string(),
        )]);
    }

    let body = args
        .items
        .iter()
        .find_map(|v| if let Value::Content(c) = v { Some(c.clone()) } else { None })
        .ok_or_else(|| {
            vec![SourceDiagnostic::error(
                args.span,
                "place() exige um bloco de conteúdo".to_string(),
            )]
        })?;

    Ok(Value::Content(Content::place(alignment, dx, dy, scope, float, clearance, body)))
}

/// Helper Passo 84.5: extrai alinhamento do primeiro argumento posicional
/// que case com `Value::Align` ou `Value::Str`. Sintaxe preferida `Value::Align`,
/// sintaxe legacy via `Align2D::from_string`. Caso nenhum case, retorna `default`.
fn extract_alignment(args: &Args, default: Align2D) -> Align2D {
    args.items
        .iter()
        .find_map(|v| match v {
            Value::Align(a) => Some(*a),
            Value::Str(s) => Some(Align2D::from_string(s.as_str())),
            _ => None, // neutro: N16[β] — Value não-Alignment retorna None na projecção de alinhamento
        })
        .unwrap_or(default)
}

// ── Grid (Passo 80) ────────────────────────────────────────────────────────

fn parse_track_sizing(val: &Value) -> Option<TrackSizing> {
    match val {
        Value::Float(f) => Some(TrackSizing::Fixed(*f)),
        Value::Length(l) => Some(TrackSizing::Fixed(l.abs.to_pt())),
        Value::Fraction(fr) => Some(TrackSizing::Fraction(*fr)),
        Value::Auto => Some(TrackSizing::Auto),
        Value::Str(s) if s.as_str() == "auto" => Some(TrackSizing::Auto),
        _ => None, // neutro: N16[β] — Value não-TrackSizing retorna None na projecção de track
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
        None => vec![], // neutro: N16[β] — argumento opcional de tracks ausente retorna lista vazia
    }
}

/// `grid(columns?, rows?, ...cells)` → `Content::Grid`.
pub fn native_grid(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    for key in args.named.keys() {
        // P224 + P227 + P228 — accept named args (gutter/align/inset/stroke/fill).
        // P772i — `header`/`footer` removidos desta whitelist: não são
        // argumentos nomeados no vanilla (`grid.header(...)`/`grid.footer(...)`
        // são elementos-filho passados posicionalmente, extraídos abaixo do
        // loop de children). `#grid(header: ..)` agora erra, paridade vanilla
        // (que rejeitaria com "unexpected argument"). Ver
        // 00_nucleo/diagnosticos/paridade-producao-p772i.md.
        if !["columns", "rows", "gutter", "align", "inset", "stroke", "fill"]
            .contains(&key.as_str())
        {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("argumento nomeado inesperado em grid(): '{}'", key),
            )]);
        }
    }
    let mut columns = extract_tracks(args.named.get("columns"));
    let mut rows = extract_tracks(args.named.get("rows"));
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
            let len = extract_length(val).ok_or_else(|| {
                vec![SourceDiagnostic::error(
                    args.span,
                    format!("grid(gutter): espera length, recebeu {}", val.type_name()),
                )]
            })?;
            if len.abs.0 < 0.0 || len.em < 0.0 {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
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
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("grid(align): espera alignment, recebeu {}", other.type_name()),
            )])
        }
        None => None,
    };

    // P224.A — extract inset (subset graded: Length uniforme apenas;
    // per-side refino futuro candidato).
    let inset: crate::entities::sides::Sides<Length> = match args.named.get("inset") {
        Some(Value::Length(l)) => crate::entities::sides::Sides::uniform(*l),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("grid(inset): espera length, recebeu {}", other.type_name()),
            )])
        }
        None => crate::entities::sides::Sides::uniform(Length::pt(0.0)),
    };

    // P772i — `header`/`footer` deixam de ser argumentos nomeados; são
    // extraídos do loop de children abaixo, como `Content::GridHeader`/
    // `Content::GridFooter` (paridade vanilla — elementos-filho, não named
    // args; ver typst-layout/src/engine.rs `GRID_CELL_RULE`/`grid/mod.rs`
    // `#[elem(name = "header")]`). Só um header e um footer são suportados
    // (repeat-across-páginas e múltiplos headers por `level` são scope-out
    // explícito — ver 00_nucleo/diagnosticos/paridade-producao-p772i.md).
    // P822 — footer fora do fim passa a ser erro (paridade vanilla
    // `resolve.rs:1889`, "footer must end at the last row"): qualquer
    // célula do corpo depois do footer é rejeitada no loop abaixo.
    let mut header: Option<Content> = None;
    let mut footer: Option<Content> = None;

    let mut cells: Vec<Content> = Vec::new();
    let mut hlines: Vec<crate::entities::elements::grid_hline::GridHLineElem> =
        Vec::new();
    let mut vlines: Vec<crate::entities::elements::grid_vline::GridVLineElem> =
        Vec::new();
    // P512 — separar linhas e calcular row/col efectivos com base na ordem
    // dos children e no número de colunas (posicionamento automático).
    let num_cols = columns.len().max(1);
    let mut row = 0usize;
    let mut col = 0usize;
    for v in args.items.iter() {
        if let Value::Content(c) = v {
            match c {
                Content::GridHLine(e) => {
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
                Content::GridVLine(e) => {
                    let mut vline = (**e).clone();
                    if vline.position == "auto" {
                        vline.position = EcoString::from("left");
                    }
                    vline.col = col;
                    vlines.push(vline);
                }
                // P772i — GridHeader/GridFooter não são células normais:
                // não incrementam col/row (são extraídos e colados
                // separadamente pelo layouter — ver grid.rs).
                Content::GridHeader(_) => {
                    if header.is_some() {
                        return Err(vec![SourceDiagnostic::error(
                            args.span,
                            "grid: não pode haver mais do que um header (scope-out — múltiplos headers por `level` não suportados)".to_string(),
                        )]);
                    }
                    header = Some(c.clone());
                }
                Content::GridFooter(_) => {
                    if footer.is_some() {
                        return Err(vec![SourceDiagnostic::error(
                            args.span,
                            "grid: não pode haver mais do que um footer".to_string(),
                        )]);
                    }
                    footer = Some(c.clone());
                }
                other => {
                    // P822 — paridade vanilla `resolve.rs:1889`: o footer
                    // tem de terminar na última linha; qualquer célula do
                    // corpo depois do footer é erro (mensagem verbatim).
                    // Linhas (hline/vline) depois do footer são aceites
                    // (medido no vanilla — sonda P822).
                    if footer.is_some() {
                        return Err(vec![SourceDiagnostic::error(
                            args.span,
                            "footer must end at the last row".to_string(),
                        )]);
                    }
                    cells.push(other.clone());
                    col += 1;
                    if col >= num_cols {
                        col = 0;
                        row += 1;
                    }
                }
            }
        }
    }
    // P227 — extract stroke (Length/Color/Stroke shorthand via extract_stroke).
    // P726 — `stroke: none` aceite (= sem traço, paridade vanilla).
    let stroke = match args.named.get("stroke") {
        Some(Value::None) | None => None,
        Some(val) => Some(extract_stroke(val, "grid", "stroke")?),
    };

    // P228 — extract fill (Opção α: apenas Value::Color; rejeita outros).
    // P726 — `fill: none` aceite (= sem preenchimento, paridade vanilla).
    let fill = match args.named.get("fill") {
        Some(Value::Color(c)) => Some(*c),
        Some(Value::None) | None => None,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("grid(fill): espera Color, recebeu {}", other.type_name()),
            )])
        }
    };

    Ok(Value::Content(Content::Grid(std::sync::Arc::new(
        crate::entities::elements::grid::GridElem {
            columns,
            rows,
            cells,
            hlines,
            vlines,
            gutter,
            align,
            inset,
            header,
            footer,
            stroke,
            fill,
        },
    ))))
}

// Lote F-2 S4 / D4 (P335): a antiga forma-função `page(...)` foi removida.
// P1140.26 restaura o constructor vigente abaixo como `PageRun`, não como o
// `SetPage` isolado da implementação histórica rejeitada.

/// P1140.26 — `page(paper?, ..named, body)` → page-run lexical.
pub fn native_page(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
    _scopes: &mut crate::compiler::scopes::Scopes<'_>,
    engine: &mut crate::entities::engine::Engine<'_>,
) -> SourceResult<Value> {
    use crate::entities::elements::page_run::PageRunElem;
    use crate::entities::layout_types::{HAlign, PageDimension, PageMarginSpec, VAlign};
    use crate::entities::page_canvas::{PageBleedSpec, PageFill};
    use crate::entities::page_geometry::{PageBinding, Paper};
    use crate::entities::page_running::{
        PageMarginal, PageNumberAlign, PageNumberVAlign,
    };
    use crate::entities::page_supplement::PageSupplement;
    use crate::entities::rel::Rel;
    use std::sync::Arc;

    const NAMED: &[&str] = &[
        "paper",
        "width",
        "height",
        "flipped",
        "margin",
        "bleed",
        "binding",
        "columns",
        "fill",
        "numbering",
        "supplement",
        "number-align",
        "header",
        "header-ascent",
        "footer",
        "footer-descent",
        "background",
        "foreground",
    ];
    for key in args.named.keys() {
        if !NAMED.contains(&key.as_str()) {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("unexpected argument `{key}` in page()"),
            )]);
        }
    }

    let mut positional = args.items.as_slice();
    let positional_paper = match positional.first() {
        Some(Value::Str(name))
            if Paper::from_name(name).is_some() && positional.len() >= 2 =>
        {
            positional = &positional[1..];
            Paper::from_name(name)
        }
        _ => None,
    };
    if positional.len() != 1 {
        let message = if positional.is_empty() {
            "page() requires a positional body"
        } else {
            "unexpected positional argument in page()"
        };
        return Err(vec![SourceDiagnostic::error(args.span, message)]);
    }
    let body = match &positional[0] {
        Value::Content(content) => content.clone(),
        Value::Str(text) => Content::text(text),
        other => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("page() body expects content, found {}", other.type_name()),
            )])
        }
    };

    let named_paper = match args.named.get("paper") {
        Some(Value::Str(name)) => Some(Paper::from_name(name).ok_or_else(|| {
            vec![SourceDiagnostic::error(args.span, "unknown paper size")]
        })?),
        Some(other) => return Err(vec![page_type_error(args.span, "string", other)]),
        None => None,
    };
    if named_paper.is_some() && positional_paper.is_some() {
        return Err(vec![SourceDiagnostic::error(args.span, "paper specified twice")]);
    }
    let paper = named_paper.or(positional_paper);
    let size_pt = engine.styles.size();

    let dimension = |name: &str| -> SourceResult<Option<PageDimension>> {
        let Some(value) = args.named.get(name) else { return Ok(None) };
        match value {
            Value::Length(length) => {
                Ok(Some(PageDimension::Length(length.resolve_pt(size_pt))))
            }
            Value::Float(value) => Ok(Some(PageDimension::Length(*value))),
            Value::Int(value) => Ok(Some(PageDimension::Length(*value as f64))),
            Value::Auto => Ok(Some(PageDimension::Auto)),
            other => Err(vec![page_type_error(
                args.span,
                "length, float, int, or auto",
                other,
            )]),
        }
    };
    let rel = |value: &Value| -> SourceResult<Rel<Length>> {
        match value {
            Value::Relative(value) => Ok(*value),
            Value::Ratio(value) => Ok(Rel { rel: value.get(), abs: Length::ZERO }),
            Value::Length(value) => Ok(Rel { rel: 0.0, abs: *value }),
            Value::Float(value) => Ok(Rel { rel: 0.0, abs: Length::pt(*value) }),
            Value::Int(value) => Ok(Rel { rel: 0.0, abs: Length::pt(*value as f64) }),
            other => Err(vec![page_type_error(args.span, "relative length", other)]),
        }
    };
    let absolute = |value: &Value| -> SourceResult<Option<f64>> {
        match value {
            Value::Length(value) => Ok(Some(value.resolve_pt(size_pt))),
            Value::Float(value) => Ok(Some(*value)),
            Value::Int(value) => Ok(Some(*value as f64)),
            Value::None => Ok(None),
            other => Err(vec![page_type_error(args.span, "length", other)]),
        }
    };

    let margin = match args.named.get("margin") {
        None => None,
        Some(Value::Auto) => Some(PageMarginSpec::auto()),
        Some(Value::Dict(dict)) => {
            if let Some(key) = dict.keys().find(|key| {
                !matches!(
                    key.as_str(),
                    "left"
                        | "right"
                        | "top"
                        | "bottom"
                        | "inside"
                        | "outside"
                        | "x"
                        | "y"
                        | "rest"
                )
            }) {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!("unknown margin key: {key}"),
                )]);
            }
            let logical = dict.get("inside").is_some() || dict.get("outside").is_some();
            let physical = dict.get("left").is_some() || dict.get("right").is_some();
            if logical && physical {
                return Err(vec![SourceDiagnostic::error(args.span, "`inside` and `outside` are mutually exclusive with `left` and `right`")]);
            }
            let field = |key: &str| -> SourceResult<Option<f64>> {
                dict.get(key).map(&absolute).transpose().map(Option::flatten)
            };
            let x = field("x")?;
            let y = field("y")?;
            let rest = field("rest")?;
            Some(PageMarginSpec {
                left: field(if logical { "inside" } else { "left" })?.or(x).or(rest),
                right: field(if logical { "outside" } else { "right" })?.or(x).or(rest),
                top: field("top")?.or(y).or(rest),
                bottom: field("bottom")?.or(y).or(rest),
                two_sided: if logical || physical { Some(logical) } else { None },
            })
        }
        Some(value) => absolute(value)?.map(PageMarginSpec::uniform),
    };

    let bleed = match args.named.get("bleed") {
        None => None,
        Some(Value::Dict(dict)) => {
            if let Some(key) = dict.keys().find(|key| {
                !matches!(
                    key.as_str(),
                    "left"
                        | "right"
                        | "top"
                        | "bottom"
                        | "inside"
                        | "outside"
                        | "x"
                        | "y"
                        | "rest"
                )
            }) {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!("unknown bleed key: {key}"),
                )]);
            }
            let logical = dict.get("inside").is_some() || dict.get("outside").is_some();
            let physical = dict.get("left").is_some() || dict.get("right").is_some();
            if logical && physical {
                return Err(vec![SourceDiagnostic::error(args.span, "`inside` and `outside` are mutually exclusive with `left` and `right`")]);
            }
            let field = |key: &str| -> SourceResult<Option<Rel<Length>>> {
                dict.get(key).map(&rel).transpose()
            };
            let x = field("x")?;
            let y = field("y")?;
            let rest = field("rest")?;
            Some(PageBleedSpec {
                left: field(if logical { "inside" } else { "left" })?.or(x).or(rest),
                right: field(if logical { "outside" } else { "right" })?.or(x).or(rest),
                top: field("top")?.or(y).or(rest),
                bottom: field("bottom")?.or(y).or(rest),
                two_sided: if logical || physical { Some(logical) } else { None },
            })
        }
        Some(Value::Auto) => {
            return Err(vec![page_type_error(
                args.span,
                "relative length or dictionary",
                &Value::Auto,
            )])
        }
        Some(value) => Some(PageBleedSpec::uniform(rel(value)?)),
    };

    let flipped = match args.named.get("flipped") {
        Some(Value::Bool(value)) => Some(*value),
        Some(other) => return Err(vec![page_type_error(args.span, "bool", other)]),
        None => None,
    };
    let binding = match args.named.get("binding") {
        Some(Value::Auto) => Some(PageBinding::Auto),
        Some(Value::Align(align))
            if align.v.is_none() && align.h == Some(HAlign::Left) =>
        {
            Some(PageBinding::Left)
        }
        Some(Value::Align(align))
            if align.v.is_none() && align.h == Some(HAlign::Right) =>
        {
            Some(PageBinding::Right)
        }
        Some(other) => {
            return Err(vec![page_type_error(args.span, "auto, left, or right", other)])
        }
        None => None,
    };
    let columns = match args.named.get("columns") {
        Some(Value::Int(value)) if *value >= 1 => Some(*value as usize),
        Some(Value::Int(_)) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "columns must be at least 1",
            )])
        }
        Some(other) => return Err(vec![page_type_error(args.span, "integer", other)]),
        None => None,
    };
    let fill = match args.named.get("fill") {
        Some(Value::Auto) => Some(PageFill::Auto),
        Some(Value::None) => Some(PageFill::None),
        Some(Value::Color(value)) => Some(PageFill::Paint((*value).into())),
        Some(Value::Gradient(value)) => Some(PageFill::Paint(value.clone().into())),
        Some(Value::Tiling(value)) => Some(PageFill::Paint((**value).clone().into())),
        Some(other) => {
            return Err(vec![page_type_error(args.span, "auto, none, or paint", other)])
        }
        None => None,
    };
    let numbering = match args.named.get("numbering") {
        Some(Value::Str(value)) => {
            Some(Some(crate::entities::numbering::Numbering::Pattern(value.clone())))
        }
        Some(Value::Func(value)) => {
            Some(Some(crate::entities::numbering::Numbering::Func(value.clone())))
        }
        Some(Value::None) => Some(None),
        Some(other) => {
            return Err(vec![page_type_error(
                args.span,
                "string, function, or none",
                other,
            )])
        }
        None => None,
    };
    let supplement = match args.named.get("supplement") {
        Some(Value::Auto) => Some(PageSupplement::Auto),
        Some(Value::None) => Some(PageSupplement::None),
        Some(Value::Content(value)) => {
            Some(PageSupplement::Content(Arc::new(value.clone())))
        }
        Some(Value::Str(value)) => {
            Some(PageSupplement::Content(Arc::new(Content::text(value))))
        }
        Some(other) => {
            return Err(vec![page_type_error(
                args.span,
                "auto, none, string, or content",
                other,
            )])
        }
        None => None,
    };
    let number_align = match args.named.get("number-align") {
        Some(Value::Align(align)) => {
            let vertical = match align.v.unwrap_or(VAlign::Bottom) {
                VAlign::Top => PageNumberVAlign::Top,
                VAlign::Bottom => PageNumberVAlign::Bottom,
                VAlign::Horizon => {
                    return Err(vec![SourceDiagnostic::error(
                        args.span,
                        "page number-align cannot use horizon",
                    )])
                }
            };
            Some(PageNumberAlign {
                horizontal: align.h.unwrap_or(HAlign::Center),
                vertical,
            })
        }
        Some(other) => return Err(vec![page_type_error(args.span, "alignment", other)]),
        None => None,
    };
    let marginal = |name: &str| -> SourceResult<Option<PageMarginal>> {
        Ok(match args.named.get(name) {
            Some(Value::Auto) => Some(PageMarginal::Auto),
            Some(Value::None) => Some(PageMarginal::None),
            Some(Value::Content(value)) => {
                Some(PageMarginal::Content(Arc::new(value.clone())))
            }
            Some(other) => {
                return Err(vec![page_type_error(
                    args.span,
                    "auto, none, or content",
                    other,
                )])
            }
            None => None,
        })
    };
    let layer = |name: &str| -> SourceResult<Option<Option<Content>>> {
        Ok(match args.named.get(name) {
            Some(Value::None) => Some(None),
            Some(Value::Content(value)) => Some(Some(value.clone())),
            Some(other) => {
                return Err(vec![page_type_error(args.span, "content or none", other)])
            }
            None => None,
        })
    };

    Ok(Value::Content(Content::PageRun(Arc::new(PageRunElem {
        paper,
        flipped,
        binding,
        width: dimension("width")?,
        height: dimension("height")?,
        margin,
        numbering,
        number_align,
        header: marginal("header")?,
        header_ascent: args.named.get("header-ascent").map(&rel).transpose()?,
        footer: marginal("footer")?,
        footer_descent: args.named.get("footer-descent").map(&rel).transpose()?,
        supplement,
        columns,
        bleed,
        fill,
        background: layer("background")?,
        foreground: layer("foreground")?,
        body,
    }))))
}

fn page_type_error(span: Span, expected: &str, found: &Value) -> SourceDiagnostic {
    SourceDiagnostic::error(
        span,
        format!("expected {expected}, found {}", found.type_name()),
    )
}
// tinha uso (zero call-sites). A geometria de página fica no modelo
// marcador/nova-página por desenho (ver `debt-stylechain-nao-materializada.md`
// §Geometria de página).

// ── Passo 156C (ADR-0061 Fase 1 sub-passo 1) — pad + hide ───────────────────

/// Coage `Value` para `Length`. Aceita `Length`, `Float` (interpretado em pt),
/// `Int` (idem). Retorna `None` para outros tipos.
fn extract_length(val: &Value) -> Option<Length> {
    match val {
        Value::Length(l) => Some(*l),
        Value::Float(f) => Some(Length { abs: Abs(*f), em: 0.0 }),
        Value::Int(i) => Some(Length { abs: Abs(*i as f64), em: 0.0 }),
        // **P475** — `Rel<Length>` aceite: parte relativa (`rel`) truncada para zero
        // (resolução percentual requer contexto de layout — scope-out P475).
        Value::Relative(r) => Some(r.abs),
        // **P842 (#32)** — o literal percentual é agora `Value::Ratio`;
        // mantém o scope-out P475 (parte relativa truncada para zero) para
        // não regredir `h(50%)`/`pad(50%)` etc. face ao comportamento
        // pré-P842 (que truncava `Relative.rel`).
        Value::Ratio(_) => Some(Length::ZERO),
        _ => None, // neutro: N16[β] — Value não-dimensionável retorna None no fallback de largura
    }
}

/// **P475** — Parse `Value` para `Sides<Length>`: uniforme (Length / Relative / Float / Int)
/// ou dict `{left?, right?, top?, bottom?, x?, y?, rest?}` per-side.
/// Precedência dentro do dict: específico > eixo (`x`/`y`) > `rest`.
/// Lados não declarados num dict ficam `Length::ZERO`.
/// Parte `rel` de `Value::Relative` truncada (scope-out: requer contexto de layout).
fn extract_sides_from_value(
    val: &Value,
    fn_name: &str,
    field: &str,
) -> SourceResult<Sides<Length>> {
    let reject_neg = |l: Length, key: &str| -> SourceResult<Length> {
        if l.abs.0 < 0.0 || l.em < 0.0 {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("{}({}:{}): valor negativo não suportado", fn_name, field, key),
            )]);
        }
        Ok(l)
    };
    // Uniforme: Length, Float, Int, Relative.
    if let Some(l) = extract_length(val) {
        let l = reject_neg(l, "")?;
        return Ok(Sides::uniform(l));
    }
    // Dict per-side: {left, right, top, bottom, x, y, rest}.
    if let Value::Dict(d) = val {
        let mut left: Option<Length> = None;
        let mut right: Option<Length> = None;
        let mut top: Option<Length> = None;
        let mut bottom: Option<Length> = None;
        let mut x_axis: Option<Length> = None;
        let mut y_axis: Option<Length> = None;
        let mut rest: Option<Length> = None;
        for (k, v) in d.iter() {
            let l = extract_length(v).ok_or_else(|| {
                vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!(
                        "{}({}:) dict: chave '{}' espera length, recebeu {}",
                        fn_name,
                        field,
                        k,
                        v.type_name()
                    ),
                )]
            })?;
            let l = reject_neg(l, k.as_str())?;
            match k.as_str() {
                "left" => left = Some(l),
                "right" => right = Some(l),
                "top" => top = Some(l),
                "bottom" => bottom = Some(l),
                "x" => x_axis = Some(l),
                "y" => y_axis = Some(l),
                "rest" => rest = Some(l),
                other => {
                    return Err(vec![SourceDiagnostic::error(
                        Span::detached(),
                        format!(
                            "{}({}:) dict: chave inesperada '{}'",
                            fn_name, field, other
                        ),
                    )])
                }
            }
        }
        return Ok(Sides {
            left: left.or(x_axis).or(rest).unwrap_or(Length::ZERO),
            right: right.or(x_axis).or(rest).unwrap_or(Length::ZERO),
            top: top.or(y_axis).or(rest).unwrap_or(Length::ZERO),
            bottom: bottom.or(y_axis).or(rest).unwrap_or(Length::ZERO),
        });
    }
    Err(vec![SourceDiagnostic::error(
        Span::detached(),
        format!(
            "{}({}:) espera length ou dict, recebeu {}",
            fn_name,
            field,
            val.type_name()
        ),
    )])
}

/// P227 — Coage `Value` para `Stroke` aceitando shorthands paridade
/// vanilla (Opção β); **P252** — defaults `overhang: true` (paridade
/// vanilla user-facing; construtor Rust low-level usa `false` per
/// ADR-0054 divergência consciente):
/// - `Value::Length(l)` → `Stroke { paint: Color::BLACK, thickness: l.to_pt(), overhang: true }`.
/// - `Value::Color(c)` → `Stroke { paint: Paint::Solid(c), thickness: 1.0, overhang: true }` (default 1pt).
/// - `Value::Stroke(s)` → `s.clone()` (preserva overhang do user).
/// - Outros tipos: erro hard.
///
/// `thickness <= 0` rejeitado (paridade vanilla).
pub(super) fn extract_stroke(
    val: &Value,
    fn_name: &str,
    field: &str,
) -> SourceResult<crate::entities::geometry::Stroke> {
    use crate::entities::geometry::Stroke;
    use crate::entities::layout_types::Color;
    use crate::entities::paint::Paint;
    let stroke = match val {
        Value::Length(l) => {
            let thickness = l.abs.to_pt();
            // P252 — vanilla default overhang=true para inputs stdlib
            // (cobertura Length atalho).
            Stroke {
                paint: Paint::Solid(Color::rgb(0, 0, 0)),
                thickness,
                overhang: true,
            }
        }
        Value::Color(c) => Stroke {
            paint: Paint::Solid(*c),
            thickness: 1.0,
            overhang: true,
        },
        Value::Stroke(s) => s.clone(),
        other => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "{}({}): espera Length / Color / Stroke, recebeu {}",
                    fn_name,
                    field,
                    other.type_name()
                ),
            )])
        }
    };
    if stroke.thickness <= 0.0 {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!(
                "{}({}): thickness deve ser > 0 (recebeu {})",
                fn_name, field, stroke.thickness
            ),
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
pub fn native_pad(
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
                args.span,
                format!(
                    "pad() espera content ou string como primeiro argumento, recebeu {}",
                    other.type_name()
                ),
            )])
        }
        None => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "pad() exige body como argumento posicional".to_string(),
            )])
        }
    };

    let sides = extract_sides_lengths(args, "pad")?;

    Ok(Value::Content(Content::pad(body, sides)))
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
fn extract_sides_lengths(
    args: &Args,
    fn_name: &str,
) -> SourceResult<Sides<Option<Length>>> {
    let mut left: Option<Length> = None;
    let mut right: Option<Length> = None;
    let mut top: Option<Length> = None;
    let mut bottom: Option<Length> = None;
    let mut x_axis: Option<Length> = None;
    let mut y_axis: Option<Length> = None;
    let mut rest: Option<Length> = None;

    for (key, value) in args.named.iter() {
        let len = extract_length(value).ok_or_else(|| {
            vec![SourceDiagnostic::error(
                args.span,
                format!(
                    "{}({}:) espera length, recebeu {}",
                    fn_name,
                    key,
                    value.type_name()
                ),
            )]
        })?;
        match key.as_str() {
            "left" => left = Some(len),
            "right" => right = Some(len),
            "top" => top = Some(len),
            "bottom" => bottom = Some(len),
            "x" => x_axis = Some(len),
            "y" => y_axis = Some(len),
            "rest" => rest = Some(len),
            other => {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!("{}(): argumento nomeado inesperado '{}'", fn_name, other),
                )])
            }
        }
    }

    // Precedência: específico > eixo > rest.
    let resolved_left = left.or(x_axis).or(rest);
    let resolved_right = right.or(x_axis).or(rest);
    let resolved_top = top.or(y_axis).or(rest);
    let resolved_bottom = bottom.or(y_axis).or(rest);

    // Validação: rejeitar negativos em qualquer lado declarado.
    for (label, opt) in [
        ("left", resolved_left),
        ("right", resolved_right),
        ("top", resolved_top),
        ("bottom", resolved_bottom),
    ] {
        if let Some(len) = opt {
            if len.abs.0 < 0.0 || len.em < 0.0 {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!(
                        "{}({}:): padding negativo não suportado neste passo (P156C/L)",
                        fn_name, label
                    ),
                )]);
            }
        }
    }

    Ok(Sides {
        left: resolved_left,
        top: resolved_top,
        right: resolved_right,
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
pub(crate) fn extract_corners_length_value(
    value: &Value,
    fn_name: &str,
) -> SourceResult<crate::entities::corners::Corners<Length>> {
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
            let len = extract_length(val).ok_or_else(|| {
                vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!(
                        "{}(radius.{}:) espera length, recebeu {}",
                        fn_name,
                        key,
                        val.type_name()
                    ),
                )]
            })?;
            if len.abs.0 < 0.0 || len.em < 0.0 {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!("{}(radius.{}:) negativo rejeitado", fn_name, key),
                )]);
            }
            match key.as_str() {
                "top-left" => tl = Some(len),
                "top-right" => tr = Some(len),
                "bottom-right" => br = Some(len),
                "bottom-left" => bl = Some(len),
                "top" => top = Some(len),
                "bottom" => bottom = Some(len),
                "left" => left = Some(len),
                "right" => right = Some(len),
                "rest" => rest = Some(len),
                other => {
                    return Err(vec![SourceDiagnostic::error(
                        Span::detached(),
                        format!(
                            "{}(radius): chave de canto inesperada '{}'",
                            fn_name, other
                        ),
                    )])
                }
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
        format!(
            "{}(radius:) espera length ou dict por canto, recebeu {}",
            fn_name,
            value.type_name()
        ),
    )])
}

/// `hide(body)` → `Content::Hide`. Sem argumentos nomeados.
pub fn native_hide(
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
                args.span,
                format!("hide() espera content ou string, recebeu {}", other.type_name()),
            )])
        }
        None => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "hide() exige body como argumento posicional".to_string(),
            )])
        }
    };
    Ok(Value::Content(Content::hide(body)))
}

// ── Passo 156D (ADR-0061 Fase 1 sub-passo 2) — h + v spacing ─────────────────

/// Resolve `weak: bool` em named args (ou default false). Erro hard se
/// tipo não-bool.
fn extract_weak(args: &Args, fn_name: &str) -> SourceResult<(bool, bool)> {
    match args.named.get("weak") {
        Some(Value::Bool(b)) => Ok((*b, true)),
        Some(other) => Err(vec![SourceDiagnostic::error(
            args.span,
            format!("{}(weak:) espera bool, recebeu {}", fn_name, other.type_name()),
        )]),
        None => Ok((false, false)),
    }
}

/// Lógica partilhada por `native_h` e `native_v`:
/// extrai `amount` (posicional obrigatório), valida não-negativo,
/// resolve `weak`. Aceita Length, Float (interpretado em pt) ou Int (idem)
/// per `extract_length`. **P842 (#38)**: aceita também `Fraction`
/// (`Spacing::Fractional`, paridade vanilla `layout/spacing.rs`) — quem
/// não suporta fração (`v()` neste passo) rejeita no caller.
fn build_spacing(
    args: &Args,
    fn_name: &str,
    valid_named: &[&str],
) -> SourceResult<(crate::entities::elements::h_space::Spacing, bool, bool)> {
    use crate::entities::elements::h_space::Spacing;
    // amount posicional obrigatório
    let amount = match args.items.first() {
        Some(Value::Fraction(fr)) => {
            // P842 (#38) — validação não-negativo também para frações
            // (mesmo perfil ADR-0054 graded dos comprimentos).
            if *fr < 0.0 {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!(
                        "{}(): amount negativo não suportado neste passo (P156D)",
                        fn_name
                    ),
                )]);
            }
            Spacing::Fractional(*fr)
        }
        Some(v) => Spacing::Absolute(extract_length(v).ok_or_else(|| {
            vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "{}() espera amount como length, recebeu {}",
                    fn_name,
                    v.type_name()
                ),
            )]
        })?),
        None => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("{}() exige amount como argumento posicional", fn_name),
            )])
        }
    };

    // Validação: amount negativo rejeitado per perfil ADR-0054 graded
    // (vanilla aceita-o; cristalino diverge intencionalmente neste passo).
    if let Spacing::Absolute(l) = amount {
        if l.abs.0 < 0.0 || l.em < 0.0 {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "{}(): amount negativo não suportado neste passo (P156D)",
                    fn_name
                ),
            )]);
        }
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

    let (weak, weak_explicit) = extract_weak(args, fn_name)?;

    Ok((amount, weak, weak_explicit))
}

/// `h(amount, weak: false)` → `Content::HSpace`.
///
/// `amount` Length ou Fraction posicional obrigatório. **P842 (#38)** —
/// `Fraction` aceite (paridade vanilla medida em `temp/p842/l7_h_*.typ`:
/// `#h(1fr)` distribui o espaço restante da linha proporcionalmente);
/// antes era rejeitado ("h() espera amount como length, recebeu
/// fraction"). `weak` armazenado mas comportamento de collapse adiado
/// neste passo (perfil ADR-0054 graded).
pub fn native_h(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    use crate::entities::elements::h_space::Spacing;
    let (amount, weak, weak_explicit) = build_spacing(args, "h", &["weak"])?;
    Ok(Value::Content(match amount {
        Spacing::Absolute(l) => {
            Content::h_space_with_weak_presence(l, weak, weak_explicit)
        }
        Spacing::Fractional(fr) => {
            Content::h_space_fraction_with_weak_presence(fr, weak, weak_explicit)
        }
    }))
}

/// `v(amount, weak: false)` → `Content::VSpace`.
///
/// Análogo a `native_h`, produz spacing primitive vertical.
/// **P842 (#38)**: `Fraction` continua rejeitado aqui (distribuição
/// vertical fracionária é outro mecanismo — scope-out registado no
/// relatório de P842); a mensagem pré-P842 preserva-se verbatim.
pub fn native_v(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    use crate::entities::elements::h_space::Spacing;
    let (amount, weak, weak_explicit) = build_spacing(args, "v", &["weak"])?;
    match amount {
        Spacing::Absolute(l) => Ok(Value::Content(Content::v_space_with_weak_presence(
            l,
            weak,
            weak_explicit,
        ))),
        Spacing::Fractional(_) => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "v() espera amount como length, recebeu fraction".to_string(),
        )]),
    }
}

/// `linebreak(justify: false)` — P1140.10.
///
/// Este passo materializa constructor e presença do campo. A diferença de
/// layout quando `justify` é true pertence explicitamente ao P1140.11.
pub fn native_linebreak(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    if !args.items.is_empty() {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "linebreak() não aceita argumentos posicionais".to_string(),
        )]);
    }

    for key in args.named.keys() {
        if key.as_str() != "justify" {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("linebreak(): argumento nomeado inesperado '{}'", key),
            )]);
        }
    }

    let (justify, justify_explicit) = match args.named.get("justify") {
        Some(Value::Bool(value)) => (*value, true),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("linebreak(justify:) espera bool, recebeu {}", other.type_name()),
            )])
        }
        None => (false, false),
    };

    Ok(Value::Content(Content::linebreak_with_justify_presence(
        justify,
        justify_explicit,
    )))
}

// ── Passo 156E (ADR-0061 Fase 1 sub-passo 3) — pagebreak manual ──────────────

/// Coage `Value::Str` para `Parity` (`"even"` / `"odd"`).
/// Outros tipos ou strings → erro hard.
fn extract_parity(value: &Value) -> SourceResult<Parity> {
    match value {
        Value::Str(s) => match s.as_str() {
            "even" => Ok(Parity::Even),
            "odd" => Ok(Parity::Odd),
            other => Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "pagebreak(to:) deve ser \"even\" ou \"odd\", recebeu \"{}\"",
                    other
                ),
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
pub fn native_block(
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
                args.span,
                format!(
                "block() espera content ou string como primeiro argumento, recebeu {}",
                other.type_name()
            ),
            )])
        }
        // Body opcional em vanilla; aceitamos ausência como Empty.
        None => Content::Empty,
    };

    let mut width: Option<Length> = None;
    let mut height: Option<Length> = None;
    let mut inset_val: Option<&Value> = None;
    let mut breakable: bool = true;

    for (key, value) in args.named.iter() {
        match key.as_str() {
            "width" => {
                width = Some(extract_length(value).ok_or_else(|| vec![SourceDiagnostic::error(
                    args.span,
                    format!("block(width:) espera length, recebeu {}", value.type_name()),
                )])?);
            }
            "height" => {
                height = Some(extract_length(value).ok_or_else(|| vec![SourceDiagnostic::error(
                    args.span,
                    format!("block(height:) espera length, recebeu {}", value.type_name()),
                )])?);
            }
            // **P475** — inset: Length | Relative | Dict per-side.
            "inset" => { inset_val = Some(value); }
            "breakable" => match value {
                Value::Bool(b) => breakable = *b,
                other => return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!("block(breakable:) espera bool, recebeu {}", other.type_name()),
                )]),
            },
            // P231 — aceitar outset/radius/clip (parsing pós-loop).
            // P247 — aceitar fill/stroke (parsing pós-loop paralelo).
            // P250 — aceitar spacing/above/below/sticky (parsing pós-loop).
            "outset" | "radius" | "clip" | "fill" | "stroke"
                | "spacing" | "above" | "below" | "sticky" => {},
            other => return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("block(): argumento nomeado inesperado '{}' (atributos avançados scope-out per ADR-0054 graded — refino futuro)", other),
            )]),
        }
    }

    // Validação: width/height negativos rejeitados.
    for (label, len) in [("width", width), ("height", height)]
        .iter()
        .filter_map(|(l, opt)| opt.map(|len| (*l, len)))
        .collect::<Vec<_>>()
    {
        if len.abs.0 < 0.0 || len.em < 0.0 {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!(
                    "block({}:): valor negativo não suportado neste passo (P156G)",
                    label
                ),
            )]);
        }
    }

    // **P475** — inset: Length | Relative (abs) | Dict per-side.
    let inset = match inset_val {
        Some(val) => extract_sides_from_value(val, "block", "inset")?,
        None => Sides::uniform(Length::ZERO),
    };

    // P231 — outset. **P475** — aceita Dict per-side além de Length uniforme.
    let outset = match args.named.get("outset") {
        Some(val) => extract_sides_from_value(val, "block", "outset")?,
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
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("block(clip): espera Bool, recebeu {}", other.type_name()),
            )])
        }
        None => false,
    };

    // P247 — fill: aceita Value::Color directo (paridade pattern Grid/
    // Table inline). Refino futuro para Paint enum quando ADR dedicada.
    // P726 — `fill: none` aceite (= sem preenchimento, paridade vanilla).
    let fill = match args.named.get("fill") {
        Some(Value::Color(c)) => Some(*c),
        Some(Value::None) | None => None,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("block(fill): espera Color, recebeu {}", other.type_name()),
            )])
        }
    };

    // P247 — stroke: reusa `extract_stroke` (helper pré-existente
    // P227 stdlib/layout.rs:351). Aceita Length/Color/Stroke shorthand.
    // P726 — `stroke: none` aceite (= sem traço, paridade vanilla).
    let stroke = match args.named.get("stroke") {
        Some(Value::None) | None => None,
        Some(val) => Some(extract_stroke(val, "block", "stroke")?),
    };

    // P250 — 4 named args novos: spacing/above/below (Length opcionais;
    // negativos rejeitados) + sticky (Bool). Defaults None×3 + false
    // preservam output literal pre-P250.
    let extract_block_length = |key: &str| -> SourceResult<Option<Length>> {
        match args.named.get(key) {
            None => Ok(None),
            Some(val) => {
                let len = extract_length(val).ok_or_else(|| {
                    vec![SourceDiagnostic::error(
                        args.span,
                        format!(
                            "block({}:) espera length, recebeu {}",
                            key,
                            val.type_name()
                        ),
                    )]
                })?;
                if len.abs.0 < 0.0 || len.em < 0.0 {
                    return Err(vec![SourceDiagnostic::error(
                        args.span,
                        format!("block({}:): valor negativo não suportado (P250)", key),
                    )]);
                }
                Ok(Some(len))
            }
        }
    };
    let spacing = extract_block_length("spacing")?;
    let above = extract_block_length("above")?;
    let below = extract_block_length("below")?;
    let sticky = match args.named.get("sticky") {
        Some(Value::Bool(b)) => *b,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("block(sticky): espera Bool, recebeu {}", other.type_name()),
            )])
        }
        None => false,
    };

    Ok(Value::Content(Content::Block(std::sync::Arc::new(
        crate::entities::elements::block::BlockElem {
            body,
            width,
            height,
            inset,
            breakable,
            outset,
            radius,
            clip,
            fill,
            stroke,
            spacing,
            above,
            below,
            sticky,
        },
    ))))
}

// ── Passo 156I (ADR-0061 Fase 2 sub-passo 3) — stack compositivo ────────────

/// Coage `Value::Dir` para `Dir` (`ltr`/`rtl`/`ttb`/`btt`).
/// Rejeita strings, tal como o vanilla (`expected direction, found string`).
fn extract_dir(value: &Value) -> SourceResult<Dir> {
    match value {
        Value::Dir(d) => Ok(*d),
        other => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("stack(dir:) espera direction, recebeu {}", other.type_name()),
        )]),
    }
}

/// `stack(dir: ttb, spacing: ?, ..children)` → `Content::Stack`.
///
/// **Atributos** (Fase 2 per ADR-0054 graded; **último sub-passo Fase 2**;
/// atinge target 72% Layout):
/// - `children` variádicos posicionais (Content ou Str).
/// - `dir: direction` (`ltr`/`rtl`/`ttb`/`btt`); default `ttb`.
/// - `spacing: Length`; default `None` (zero).
///
/// Sem atributos vanilla scope-out (vanilla stack tem apenas estes 3).
pub fn native_stack(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let mut dir: Dir = Dir::default(); // TTB
    let mut spacing: Option<Length> = None;

    for (key, value) in args.named.iter() {
        match key.as_str() {
            "dir" => dir = extract_dir(value)?,
            "spacing" => {
                let len = extract_length(value).ok_or_else(|| {
                    vec![SourceDiagnostic::error(
                        args.span,
                        format!(
                            "stack(spacing:) espera length, recebeu {}",
                            value.type_name()
                        ),
                    )]
                })?;
                if len.abs.0 < 0.0 || len.em < 0.0 {
                    return Err(vec![SourceDiagnostic::error(
                        args.span,
                        "stack(spacing:): valor negativo não suportado neste passo (P156I)".to_string(),
                    )]);
                }
                spacing = Some(len);
            }
            other => {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!("stack(): argumento nomeado inesperado '{}'", other),
                )])
            }
        }
    }

    // Children variádicos: iterar args.items, aceitar Content ou Str.
    let mut children: Vec<Content> = Vec::with_capacity(args.items.len());
    for v in args.items.iter() {
        match v {
            Value::Content(c) => children.push(c.clone()),
            Value::Str(s) => children.push(Content::text(s.as_str())),
            other => {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!(
                        "stack(): children devem ser content ou string, recebeu {}",
                        other.type_name()
                    ),
                )])
            }
        }
    }

    Ok(Value::Content(Content::stack(children, dir, spacing)))
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
pub fn native_box(
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
                args.span,
                format!(
                    "box() espera content ou string como primeiro argumento, recebeu {}",
                    other.type_name()
                ),
            )])
        }
        None => Content::Empty, // body opcional (vanilla aceita)
    };

    let mut width: Option<Length> = None;
    let mut height: Option<Length> = None;
    let mut inset_val: Option<&Value> = None;
    let mut baseline: Length = Length::ZERO;

    for (key, value) in args.named.iter() {
        match key.as_str() {
            "width" => {
                width = Some(extract_length(value).ok_or_else(|| vec![SourceDiagnostic::error(
                    args.span,
                    format!("box(width:) espera length, recebeu {}", value.type_name()),
                )])?);
            }
            "height" => {
                height = Some(extract_length(value).ok_or_else(|| vec![SourceDiagnostic::error(
                    args.span,
                    format!("box(height:) espera length, recebeu {}", value.type_name()),
                )])?);
            }
            // **P475** — inset: Length | Relative | Dict per-side.
            "inset" => { inset_val = Some(value); }
            "baseline" => {
                baseline = extract_length(value).ok_or_else(|| vec![SourceDiagnostic::error(
                    args.span,
                    format!("box(baseline:) espera length, recebeu {}", value.type_name()),
                )])?;
            }
            // P231 — aceitar outset/radius/clip (parsing pós-loop paridade block).
            // P247 — aceitar fill/stroke paralelo block.
            "outset" | "radius" | "clip" | "fill" | "stroke" => {},
            other => return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("box(): argumento nomeado inesperado '{}' (atributos avançados scope-out per ADR-0054 graded — refino futuro)", other),
            )]),
        }
    }

    // Validação: width/height negativos rejeitados (baseline negativo ACEITE).
    for (label, len) in [("width", width), ("height", height)]
        .iter()
        .filter_map(|(l, opt)| opt.map(|len| (*l, len)))
        .collect::<Vec<_>>()
    {
        if len.abs.0 < 0.0 || len.em < 0.0 {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!(
                    "box({}:): valor negativo não suportado neste passo (P156H)",
                    label
                ),
            )]);
        }
    }

    // **P475** — inset: Length | Relative (abs) | Dict per-side.
    let inset = match inset_val {
        Some(val) => extract_sides_from_value(val, "box", "inset")?,
        None => Sides::uniform(Length::ZERO),
    };

    // P231 — outset. **P475** — aceita Dict per-side além de Length uniforme.
    let outset = match args.named.get("outset") {
        Some(val) => extract_sides_from_value(val, "box", "outset")?,
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
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("box(clip): espera Bool, recebeu {}", other.type_name()),
            )])
        }
        None => false,
    };

    // P247 — fill paralelo block.
    // P726 — `fill: none` aceite (= sem preenchimento, paridade vanilla).
    let fill = match args.named.get("fill") {
        Some(Value::Color(c)) => Some(*c),
        Some(Value::None) | None => None,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("box(fill): espera Color, recebeu {}", other.type_name()),
            )])
        }
    };

    // P247 — stroke reusa `extract_stroke` paralelo block.
    // P726 — `stroke: none` aceite (= sem traço, paridade vanilla).
    let stroke = match args.named.get("stroke") {
        Some(Value::None) | None => None,
        Some(val) => Some(extract_stroke(val, "box", "stroke")?),
    };

    Ok(Value::Content(Content::Boxed(std::sync::Arc::new(
        crate::entities::elements::boxed::BoxedElem {
            body,
            width,
            height,
            inset,
            baseline,
            outset,
            radius,
            clip,
            fill,
            stroke,
        },
    ))))
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
pub fn native_repeat(
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
                args.span,
                format!(
                "repeat() espera content ou string como primeiro argumento, recebeu {}",
                other.type_name()
            ),
            )])
        }
        None => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "repeat() exige body como argumento posicional".to_string(),
            )])
        }
    };

    let mut gap: Option<Length> = None;
    let mut justify: bool = true; // default vanilla

    for (key, value) in args.named.iter() {
        match key.as_str() {
            "gap" => {
                let len = extract_length(value).ok_or_else(|| {
                    vec![SourceDiagnostic::error(
                        args.span,
                        format!(
                            "repeat(gap:) espera length, recebeu {}",
                            value.type_name()
                        ),
                    )]
                })?;
                if len.abs.0 < 0.0 || len.em < 0.0 {
                    return Err(vec![SourceDiagnostic::error(
                        args.span,
                        "repeat(gap:): valor negativo não suportado neste passo (P156J)"
                            .to_string(),
                    )]);
                }
                gap = Some(len);
            }
            "justify" => match value {
                Value::Bool(b) => justify = *b,
                other => {
                    return Err(vec![SourceDiagnostic::error(
                        args.span,
                        format!(
                            "repeat(justify:) espera bool, recebeu {}",
                            other.type_name()
                        ),
                    )])
                }
            },
            other => {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!("repeat(): argumento nomeado inesperado '{}'", other),
                )])
            }
        }
    }

    Ok(Value::Content(Content::repeat(body, gap, justify)))
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
                    args.span,
                    format!("{}(count): count deve ser >= 1, recebeu {}", fn_name, n),
                )]);
            }
            Ok(*n as usize)
        }
        Some(other) => Err(vec![SourceDiagnostic::error(
            args.span,
            format!("{}(count): espera Int, recebeu {}", fn_name, other.type_name()),
        )]),
        None => Err(vec![SourceDiagnostic::error(
            args.span,
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
pub fn native_columns(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    // 1. Extract count (posicional [0] obrigatório).
    let count = extract_count(args, "columns")?;

    // 2. Extract body (posicional [1], Content ou Str).
    let body = match args.items.get(1) {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::Str(s)) => Content::text(s.as_str()),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!(
                    "columns(body): espera Content ou Str, recebeu {}",
                    other.type_name()
                ),
            )])
        }
        None => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "columns: argumento posicional body obrigatório ausente".to_string(),
            )])
        }
    };

    // 3. Validate no extra positionals.
    if args.items.len() > 2 {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!(
                "columns: aceita 2 posicionais (count, body), recebeu {}",
                args.items.len()
            ),
        )]);
    }

    // 4. Extract gutter (named opcional, Option<Length>); validar negativo.
    let mut gutter: Option<Length> = None;
    for (key, value) in args.named.iter() {
        match key.as_str() {
            "gutter" => {
                let len = extract_length(value).ok_or_else(|| {
                    vec![SourceDiagnostic::error(
                        args.span,
                        format!(
                            "columns(gutter:) espera length, recebeu {}",
                            value.type_name()
                        ),
                    )]
                })?;
                if len.abs.0 < 0.0 || len.em < 0.0 {
                    return Err(vec![SourceDiagnostic::error(
                        args.span,
                        "columns(gutter:): valor negativo não suportado".to_string(),
                    )]);
                }
                gutter = Some(len);
            }
            other => {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!("columns(): argumento nomeado inesperado '{}'", other),
                )])
            }
        }
    }

    Ok(Value::Content(Content::columns(body, count, gutter)))
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
pub fn native_colbreak(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    if !args.items.is_empty() {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "colbreak() não aceita argumentos posicionais".to_string(),
        )]);
    }

    let mut weak: bool = false;

    for (key, value) in args.named.iter() {
        match key.as_str() {
            "weak" => match value {
                Value::Bool(b) => weak = *b,
                other => {
                    return Err(vec![SourceDiagnostic::error(
                        args.span,
                        format!(
                            "colbreak(weak:) espera bool, recebeu {}",
                            other.type_name()
                        ),
                    )])
                }
            },
            other => {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!(
                        "colbreak(): argumento nomeado inesperado '{}' (esperado: weak)",
                        other
                    ),
                )])
            }
        }
    }

    Ok(Value::Content(Content::colbreak_with_weak_presence(
        weak,
        args.named.contains_key("weak"),
    )))
}

/// Extrai e valida o argumento `body` de `measure(...)` — 1 posicional
/// (`Content` ou `Str`), sem argumentos nomeados (`width`/`height` override
/// scope-out, Opção β graded per ADR-0054). Partilhado (P712) entre
/// `native_measure` (fallback de invocação indirecta) e a intercepção real
/// em `eval_func_call` (`eval/closures.rs`, com acesso a `engine.styles`).
pub fn extract_measure_body(args: &Args) -> SourceResult<Content> {
    let body = match args.items.first() {
        Some(Value::Content(c)) => c.clone(),
        Some(Value::Str(s)) => Content::text(s.as_str()),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!(
                    "measure(body): espera Content ou Str, recebeu {}",
                    other.type_name()
                ),
            )])
        }
        None => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "measure: argumento posicional body obrigatório ausente".to_string(),
            )])
        }
    };

    if args.items.len() > 1 {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!("measure: aceita 1 posicional (body), recebeu {}", args.items.len()),
        )]);
    }

    if let Some(key) = args.named.keys().next() {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!("measure: named arg `{}` não suportado (paridade graded; refino futuro candidato NÃO-reservado per ADR-0054)", key),
        )]);
    }

    Ok(body)
}

/// `measure(body) -> dict(width: length, height: length)` —
/// Passo 222 (Fase 4 Layout candidata; ADR-0066 §"Plano
/// promoção" Bloco C cross-módulo primeira materialização
/// parcial).
///
/// **P712** — este `NativeFn` é apenas o fallback para invocação indirecta
/// de `measure` (ex.: passado como valor de primeira classe a outra função,
/// sem sintaxe de chamada directa). A invocação real e directa
/// (`measure(...)`/`std.measure(...)`) é interceptada em `eval_func_call`
/// (`eval/closures.rs` §P712, mesmo padrão de `.to-absolute()`/`.with()`),
/// porque precisa de `engine.styles` (tamanho medido depende do `#set
/// text(size:)` activo) e do gate `ctx.in_context` — nenhum dos dois
/// acessível pela assinatura genérica `NativeFn(ctx, args, world, file)`.
/// Sem consumidor medido para o caminho indirecto (nenhum documento de
/// teste ou `cetz` passa `measure` como valor); falha alto em vez de
/// devolver `(0, 0)` silenciosamente (ADR-0108).
/// **P792** — `layout(func)` — stub de despacho.
///
/// A lógica real vive em `eval/closures.rs::eval_func_call` (intercepção
/// pelo `native_fn_addr`, mesmo padrão de `native_measure`/P712).
/// Este stub nunca é invocado directamente — serve apenas para:
/// 1. Existir no scope global como `Value::Func("layout")`.
/// 2. Expor um endereço de função identificável para a intercepção.
///
/// Paridade vanilla: `layout/layout.rs:66` `#[func]` nativo global.
pub fn native_layout(
    _ctx: &mut EvalContext,
    _args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    Err(vec![SourceDiagnostic::error(
        Span::detached(),
        "layout(): invocação indirecta não suportada — usar layout(size => ...) — P792"
            .to_string(),
    )])
}

pub fn native_measure(
    _ctx: &mut EvalContext,
    _args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    Err(vec![SourceDiagnostic::error(
        Span::detached(),
        "measure(): invocação indirecta (fora de measure(...)/std.measure(...) \
         directo) não suportada — P712"
            .to_string(),
    )])
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
pub fn native_stroke(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    use crate::entities::geometry::Stroke;
    use crate::entities::layout_types::Color;
    use crate::entities::paint::Paint;

    if !args.items.is_empty() {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "stroke() não aceita argumentos posicionais".to_string(),
        )]);
    }

    let paint = match args.named.get("paint") {
        Some(Value::Color(c)) => *c,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("stroke(paint): espera Color, recebeu {}", other.type_name()),
            )])
        }
        None => Color::rgb(0, 0, 0),
    };

    let thickness = match args.named.get("thickness") {
        Some(val) => {
            let len = extract_length(val).ok_or_else(|| {
                vec![SourceDiagnostic::error(
                    args.span,
                    format!(
                        "stroke(thickness): espera length, recebeu {}",
                        val.type_name()
                    ),
                )]
            })?;
            len.abs.to_pt()
        }
        None => 1.0,
    };

    if thickness <= 0.0 {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!("stroke(thickness): deve ser > 0 (recebeu {})", thickness),
        )]);
    }

    // P252 — overhang opcional; default vanilla `true` quando ausente.
    let overhang = match args.named.get("overhang") {
        Some(Value::Bool(b)) => *b,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("stroke(overhang): espera Bool, recebeu {}", other.type_name()),
            )])
        }
        None => true,
    };

    for key in args.named.keys() {
        if !["paint", "thickness", "overhang"].contains(&key.as_str()) {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("stroke(): argumento nomeado inesperado '{}' (esperado: paint, thickness, overhang)", key),
            )]);
        }
    }

    Ok(Value::Stroke(Stroke { paint: Paint::Solid(paint), thickness, overhang }))
}

/// `pagebreak(weak: false, to: ?)` → `Content::Pagebreak`.
///
/// Sem argumentos posicionais. `weak` armazenado mas comportamento de
/// collapse adiado (perfil ADR-0054 graded; consistente com P156D).
/// `to` aceita string `"even"` ou `"odd"`; ausente → `None` (sem
/// ajuste de paridade).
pub fn native_pagebreak(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    if !args.items.is_empty() {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "pagebreak() não aceita argumentos posicionais".to_string(),
        )]);
    }

    let mut weak: bool = false;
    let mut to: Option<Parity> = None;

    for (key, value) in args.named.iter() {
        match key.as_str() {
            "weak" => match value {
                Value::Bool(b) => weak = *b,
                other => {
                    return Err(vec![SourceDiagnostic::error(
                        args.span,
                        format!(
                            "pagebreak(weak:) espera bool, recebeu {}",
                            other.type_name()
                        ),
                    )])
                }
            },
            "to" => to = Some(extract_parity(value)?),
            other => {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!("pagebreak(): argumento nomeado inesperado '{}'", other),
                )])
            }
        }
    }

    Ok(Value::Content(Content::pagebreak_with_weak_presence(
        weak,
        args.named.contains_key("weak"),
        to,
    )))
}
