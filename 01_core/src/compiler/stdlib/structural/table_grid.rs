//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/structural/table_grid.md
//! @prompt-hash f06a18fc
//! @layer L1
//! @updated 2026-10-17
//!
//! Nativas de tabela e grelha — a estrutura: `table`/`grid` e as suas
//! células, cabeçalhos e rodapés, com os extractores de argumento partilhados.
//!
//! Extraído de `stdlib/structural.rs` no Passo 1014 conforme ADR-0109
//! (atomização — forma B, free function no arquivo da unidade).

use crate::entities::file_id::FileId;
use ecow::EcoString;

use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;

// ── Sentinelas e construtores de nós estruturais (Passo 69) ─────────────────

use super::table_lines::default_hline_stroke;

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
    use crate::compiler::stdlib::layout::{extract_stroke, extract_tracks};
    use crate::entities::layout_types::TrackSizing;

    for key in args.named.keys() {
        // P227 + P228 — accept stroke + fill (paridade native_grid).
        // P459 — accept caption para numeração automática.
        if !["columns", "rows", "stroke", "fill", "caption", "align", "inset", "gutter"]
            .contains(&key.as_str())
        {
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
    let mut hlines: Vec<crate::entities::elements::table_hline::TableHLineElem> =
        Vec::new();
    let mut vlines: Vec<crate::entities::elements::table_vline::TableVLineElem> =
        Vec::new();
    let num_cols = columns.len().max(1);
    let mut row = 0usize;
    let mut col = 0usize;
    for v in args.items.iter() {
        let classified = match v {
            Value::Content(c) => Value::Content(c.source_body().clone()),
            other => other.clone(),
        };
        match &classified {
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
                if let Some(previous) = header.take() {
                    let previous_levels = table_header_levels(&previous);
                    let current_levels = table_header_levels(c);
                    if previous_levels.iter().any(|level| current_levels.contains(level))
                    {
                        return Err(vec![SourceDiagnostic::error(
                            Span::detached(),
                            "table: não pode haver mais do que um header (scope-out — múltiplos headers por `level` não suportados)".to_string(),
                        )]);
                    }
                    let (previous_body, previous_repeat) = match previous {
                        Content::TableHeader(group) => (group.body.clone(), group.repeat),
                        _ => unreachable!("header carrier must remain TableHeader"),
                    };
                    let Content::TableHeader(current) = c else {
                        unreachable!("matched TableHeader above")
                    };
                    header = Some(Content::table_header(
                        Content::sequence(vec![previous_body, current.body.clone()]),
                        previous_repeat && current.repeat,
                    ));
                } else {
                    header = Some(c.clone());
                }
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
                // P822 — paridade vanilla `resolve.rs:1889`: o footer tem
                // de terminar na última linha; qualquer célula do corpo
                // depois do footer é erro (mensagem verbatim). Linhas
                // (hline/vline) depois do footer são aceites (medido no
                // vanilla — sonda P822).
                if footer.is_some() {
                    return Err(vec![SourceDiagnostic::error(
                        Span::detached(),
                        "footer must end at the last row".to_string(),
                    )]);
                }
                children.push(if let Value::Content(original) = v {
                    original.clone()
                } else {
                    c.clone()
                });
                col += 1;
                if col >= num_cols {
                    col = 0;
                    row += 1;
                }
            }
            Value::Str(s) => {
                // P822 — mesma validação do braço Content acima.
                if footer.is_some() {
                    return Err(vec![SourceDiagnostic::error(
                        Span::detached(),
                        "footer must end at the last row".to_string(),
                    )]);
                }
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
    // P887 (achado 3 de P885) — omitido ≠ `none` explícito: vanilla dá a
    // `table.stroke` um default de `1pt + black` quando o argumento não é
    // passado (`model/table.rs:268-270`, `#[default(Celled::Value(Sides::
    // splat(Some(Some(Arc::new(Stroke::default()))))))]`, resolvendo via
    // `FixedStroke::default()` em `visualize/stroke.rs:654-665`); só
    // `stroke: none` explícito produz ausência de stroke. `grid()`
    // (`native_grid`, `layout.rs`) não tem este default no vanilla e por
    // isso mantém `Some(Value::None) | None => None` inalterado.
    let stroke = match args.named.get("stroke") {
        Some(Value::None) => None,
        None => Some(default_hline_stroke()),
        Some(val) => {
            let mut s = extract_stroke(val, "table", "stroke")?;
            match val {
                Value::Length(_) => {
                    s.specified = Default::default();
                    s.specified.thickness = true;
                }
                Value::Color(_) => {
                    s.specified = Default::default();
                    s.specified.paint = true;
                }
                Value::None
                | Value::Bool(_)
                | Value::Int(_)
                | Value::Float(_)
                | Value::Str(_)
                | Value::Array(_)
                | Value::Dict(_)
                | Value::Module(_)
                | Value::Datetime(_)
                | Value::Func(_)
                | Value::Content(_)
                | Value::LocatedContent(_, _)
                | Value::Auto
                | Value::Relative(_)
                | Value::Ratio(_)
                | Value::Angle(_)
                | Value::Stroke(_)
                | Value::Fraction(_)
                | Value::Align(_)
                | Value::Location(_)
                | Value::Gradient(_)
                | Value::Regex(_)
                | Value::Tiling(_)
                | Value::Bytes(_)
                | Value::Decimal(_)
                | Value::Duration(_)
                | Value::Version(_)
                | Value::Selector(_)
                | Value::Symbol(_)
                | Value::Args(_)
                | Value::State(_)
                | Value::Counter(_)
                | Value::Label(_)
                | Value::Dir(_)
                | Value::Path(_)
                | Value::Type(_)
                | Value::SpotColorant(_) => {}
            }
            Some(s)
        }
    };
    // P228 — extract fill (Opção α: apenas Value::Color).
    // P726 — `fill: none` aceite (= sem preenchimento, paridade vanilla).
    let fill = match args.named.get("fill") {
        Some(Value::Color(c)) => Some(c.clone()),
        Some(Value::None) | None => None,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("table(fill): espera Color, recebeu {}", other.type_name()),
            )])
        }
    };
    // P1050 — extract align (Value::Align; default None).
    let align = match args.named.get("align") {
        Some(Value::Align(a)) => Some(*a),
        Some(Value::None) | None => None,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("table(align): espera alignment, recebeu {}", other.type_name()),
            )])
        }
    };
    // P1050 — extract inset (default: Sides::uniform(5pt) paridade vanilla table).
    let inset: crate::entities::sides::Sides<crate::entities::layout_types::Length> =
        match args.named.get("inset") {
            Some(Value::Length(l)) => crate::entities::sides::Sides::uniform(*l),
            Some(Value::None) => crate::entities::sides::Sides::uniform(
                crate::entities::layout_types::Length::pt(0.0),
            ),
            None => crate::entities::sides::Sides::uniform(
                crate::entities::layout_types::Length::pt(5.0),
            ),
            Some(other) => {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!("table(inset): espera length, recebeu {}", other.type_name()),
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
                format!(
                    "table(caption): espera content ou string, recebeu {}",
                    other.type_name()
                ),
            )])
        }
    };
    Ok(Value::Content(Content::Table(std::sync::Arc::new(
        crate::entities::elements::table::TableElem {
            source_span: args.span,
            resolved: None,
            rows_explicit: args.named.contains_key("rows"),
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
            summary: None,
            inset,
            align,
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

/// Constrói a célula lexical, preservando presença de campos explícitos.
/// Posições/estilos herdados e padding permanecem com os owners de realização.
/// X/y aceitam inteiro não negativo ou auto; spans exigem inteiro positivo.
/// Auto e none explícitos não são confundidos com omissão.
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
    use crate::compiler::eval::operators::error_formatting::vanilla_type_name;
    use crate::entities::elements::table_cell::{CellFieldPresence, TableCellPresence};
    let mut presence = TableCellPresence::default();
    for (key, value) in args.named.iter() {
        let span = args
            .occurrences
            .as_ref()
            .and_then(|items| {
                items
                    .iter()
                    .rev()
                    .find(|item| item.name.as_deref() == Some(key.as_str()))
            })
            .map(|item| item.value_span)
            .unwrap_or(args.span);
        let expected = |types: &str| {
            vec![SourceDiagnostic::error(
                span,
                format!("expected {types}, found {}", vanilla_type_name(value)),
            )]
        };
        let state = if matches!(value, Value::Auto) {
            CellFieldPresence::Auto
        } else {
            CellFieldPresence::Explicit
        };
        match key.as_str() {
            "x" | "y" | "colspan" | "rowspan" => {
                let smart = matches!(key.as_str(), "x" | "y");
                let cast = match value {
                    Value::Auto if smart => None,
                    Value::Int(n) if *n >= if smart { 0 } else { 1 } => Some(*n as usize),
                    Value::Int(_) => return Err(vec![SourceDiagnostic::error(span,
                        if smart { "number must be at least zero" } else { "number must be positive" })]),
                    _ => return Err(expected(if smart { "integer or auto" } else { "integer" })),
                };
                match key.as_str() {
                    "x" => { x = cast; presence.x = state; }
                    "y" => { y = cast; presence.y = state; }
                    "colspan" => { colspan = cast; presence.colspan = true; }
                    "rowspan" => { rowspan = cast; presence.rowspan = true; }
                    _ => unreachable!("validated position field"),
                }
            }
            "stroke" => {
                presence.stroke = true;
                stroke = if matches!(value, Value::None) { None }
                else {
                    if matches!(value, Value::Auto) {
                        return Err(expected("length, color, gradient, tiling, dictionary, stroke, or none"));
                    }
                    let mut parsed = crate::compiler::stdlib::layout::extract_stroke(value, "table_cell", "stroke")
                        .map_err(|mut errors| { for error in &mut errors { error.span = span; } errors })?;
                    match value {
                        Value::Length(_) => parsed.specified.thickness = true,
                        Value::Color(_) => parsed.specified.paint = true,
                        Value::None | Value::Bool(_) | Value::Int(_) | Value::Float(_) | Value::Str(_) | Value::Array(_) | Value::Dict(_) | Value::Module(_) | Value::Datetime(_) | Value::Func(_) | Value::Content(_) | Value::LocatedContent(_, _) | Value::Auto | Value::Relative(_) | Value::Ratio(_) | Value::Angle(_) | Value::Stroke(_) | Value::Fraction(_) | Value::Align(_) | Value::Location(_) | Value::Gradient(_) | Value::Regex(_) | Value::Tiling(_) | Value::Bytes(_) | Value::Decimal(_) | Value::Duration(_) | Value::Version(_) | Value::Selector(_) | Value::Symbol(_) | Value::Args(_) | Value::State(_) | Value::Counter(_) | Value::Label(_) | Value::Dir(_) | Value::Path(_) | Value::Type(_) | Value::SpotColorant(_) => {}
                    }
                    Some(parsed)
                };
            }
            "fill" => {
                presence.fill = state;
                fill = match value {
                    Value::Auto | Value::None => None,
                    Value::Color(c) => Some(c.clone()),
                    _ => return Err(expected("color, gradient, tiling, none, or auto")),
                };
            }
            "align" => {
                presence.align = state;
                align = match value {
                    Value::Auto => None,
                    Value::None => return Err(expected("alignment or auto")),
                    Value::Bool(_) | Value::Int(_) | Value::Float(_) | Value::Str(_) | Value::Array(_) | Value::Dict(_) | Value::Module(_) | Value::Datetime(_) | Value::Func(_) | Value::Content(_) | Value::LocatedContent(_, _) | Value::Length(_) | Value::Relative(_) | Value::Ratio(_) | Value::Angle(_) | Value::Color(_) | Value::Stroke(_) | Value::Fraction(_) | Value::Align(_) | Value::Location(_) | Value::Gradient(_) | Value::Regex(_) | Value::Tiling(_) | Value::Bytes(_) | Value::Decimal(_) | Value::Duration(_) | Value::Version(_) | Value::Selector(_) | Value::Symbol(_) | Value::Args(_) | Value::State(_) | Value::Counter(_) | Value::Label(_) | Value::Dir(_) | Value::Path(_) | Value::Type(_) | Value::SpotColorant(_) => Some(extract_align_value(value, "table_cell", "align").map_err(|mut errors| { for error in &mut errors { error.span = span; } errors })?),
                };
            }
            "inset" => {
                presence.inset = state;
                inset = match value {
                    Value::Auto => None,
                    Value::None => return Err(expected("relative length, dictionary, or auto")),
                    Value::Bool(_) | Value::Int(_) | Value::Float(_) | Value::Str(_) | Value::Array(_) | Value::Dict(_) | Value::Module(_) | Value::Datetime(_) | Value::Func(_) | Value::Content(_) | Value::LocatedContent(_, _) | Value::Length(_) | Value::Relative(_) | Value::Ratio(_) | Value::Angle(_) | Value::Color(_) | Value::Stroke(_) | Value::Fraction(_) | Value::Align(_) | Value::Location(_) | Value::Gradient(_) | Value::Regex(_) | Value::Tiling(_) | Value::Bytes(_) | Value::Decimal(_) | Value::Duration(_) | Value::Version(_) | Value::Selector(_) | Value::Symbol(_) | Value::Args(_) | Value::State(_) | Value::Counter(_) | Value::Label(_) | Value::Dir(_) | Value::Path(_) | Value::Type(_) | Value::SpotColorant(_) => Some(extract_inset_value(value, "table_cell", "inset").map_err(|_| expected("relative length, dictionary, or auto"))?),
                };
            }
            "breakable" => {
                presence.breakable = state;
                breakable = match value {
                    Value::Auto => None,
                    Value::Bool(b) => Some(*b),
                    _ => return Err(expected("boolean or auto")),
                };
            }
            other => return Err(vec![SourceDiagnostic::error(span,
                format!("table_cell(): argumento nomeado inesperado '{}' (atributos avançados scope-out per ADR-0054 graded — refino futuro)", other))]),
        }
    }

    Ok(Value::Content(Content::TableCell(std::sync::Arc::new(
        crate::entities::elements::table_cell::TableCellElem {
            source_span: args.span,
            resolved: None,
            standalone: None,
            presence,
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
            kind: crate::entities::elements::table_cell::TableCellKind::Auto,
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
    let level = match args.named.get("level") {
        None => 1,
        Some(Value::Int(value)) => {
            if *value > 0 && *value <= u32::MAX as i64 {
                *value as u32
            } else {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    "number must be positive",
                )]);
            }
        }
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("expected integer, found {}", other.type_name()),
            )])
        }
    };
    let mut cell_values: Vec<Content> = Vec::with_capacity(args.items.len());
    for v in args.items.iter() {
        let content = match v {
            Value::Content(c) => c.clone(),
            Value::Str(s) => Content::text(s.as_str()),
            other => return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("table_header() espera content ou string como argumento posicional, recebeu {}", other.type_name()),
            )]),
        };
        let origin = content.source_span();
        let source_wrapped = matches!(&content, Content::Source(_));
        let mut cell = match content.source_body() {
            Content::TableCell(cell) => cell.as_ref().clone(),
            Content::Source(_)
            | Content::ShowRecipe(_)
            | Content::Realized(_)
            | Content::Located(_)
            | Content::Empty
            | Content::Text(_)
            | Content::Space
            | Content::SpaceFromNewline
            | Content::Parbreak
            | Content::Sequence(_)
            | Content::HtmlElem(_)
            | Content::HtmlFrame(_)
            | Content::Par { .. }
            | Content::Heading(_)
            | Content::Title(_)
            | Content::Strong(_)
            | Content::Emph(_)
            | Content::Raw(_)
            | Content::ListItem(_)
            | Content::EnumItem(_)
            | Content::Link(_)
            | Content::Equation(_)
            | Content::MathSequence(_)
            | Content::MathIdent(_)
            | Content::MathText(_)
            | Content::MathFrac(_)
            | Content::MathAttach(_)
            | Content::MathPrimes(_)
            | Content::MathRoot(_)
            | Content::MathStretch(_)
            | Content::MathDelimited(_)
            | Content::MathLr(_)
            | Content::MathMid(_)
            | Content::MathOverbrace(_)
            | Content::MathUnderbrace(_)
            | Content::MathUnderbracket(_)
            | Content::MathUnderparen(_)
            | Content::MathUndershell(_)
            | Content::MathOverbracket(_)
            | Content::MathOverparen(_)
            | Content::MathOvershell(_)
            | Content::MathAlignPoint(_)
            | Content::Linebreak(_)
            | Content::MathMatrix(_)
            | Content::MathCases(_)
            | Content::MathAccent(_)
            | Content::MathCancel(_)
            | Content::MathUnderline(_)
            | Content::MathOverline(_)
            | Content::MathVec(_)
            | Content::MathClassOverride(_)
            | Content::MathLimitsOverride(_)
            | Content::MathUnderover(_)
            | Content::MathOp(_)
            | Content::MathStyled(_)
            | Content::Label(_)
            | Content::Ref(_)
            | Content::CounterDisplay(_)
            | Content::CounterUpdate(_)
            | Content::Outline(_)
            | Content::OutlineEntry(_)
            | Content::Figure(_)
            | Content::FigureCaption(_)
            | Content::Image(_)
            | Content::Shape(_)
            | Content::Curve(_)
            | Content::Transform(_)
            | Content::Grid(_)
            | Content::GridHeader(_)
            | Content::GridFooter(_)
            | Content::GridCell(_)
            | Content::SetPage(_)
            | Content::PageRun(_)
            | Content::Align(_)
            | Content::Place(_)
            | Content::Flush(_)
            | Content::Styled(_)
            | Content::Divider(_)
            | Content::Terms(_)
            | Content::TermItem(_)
            | Content::Quote(_)
            | Content::Document { .. }
            | Content::Asset { .. }
            | Content::SmartQuote(_)
            | Content::PdfAttach(_)
            | Content::PdfArtifact(_)
            | Content::Underline(_)
            | Content::Strike(_)
            | Content::Overline(_)
            | Content::SmallCaps { .. }
            | Content::Pad(_)
            | Content::Hide(_)
            | Content::HSpace(_)
            | Content::VSpace(_)
            | Content::Pagebreak(_)
            | Content::Colbreak(_)
            | Content::Stack(_)
            | Content::Boxed(_)
            | Content::Block(_)
            | Content::Bibliography(_)
            | Content::Cite(_)
            | Content::Footnote(_)
            | Content::FootnoteEntry(_)
            | Content::TableHeader(_)
            | Content::TableFooter(_)
            | Content::GridHLine(_)
            | Content::GridVLine(_)
            | Content::TableHLine(_)
            | Content::TableVLine(_)
            | Content::Table(_)
            | Content::Repeat(_)
            | Content::Columns(_)
            | Content::Metadata(_)
            | Content::State(_)
            | Content::StateUpdate(_)
            | Content::StateDisplay(_)
            | Content::CounterDisplayCallback(_)
            | Content::ContextBlock(_)
            | Content::Dynamic(_) => {
                crate::entities::elements::table_cell::TableCellElem {
                    source_span: if origin.is_detached() { args.span } else { origin },
                    resolved: None,
                    standalone: None,
                    presence: Default::default(),
                    body: content.clone(),
                    x: None,
                    y: None,
                    colspan: None,
                    rowspan: None,
                    stroke: None,
                    fill: None,
                    align: None,
                    inset: None,
                    breakable: None,
                    kind: crate::entities::elements::table_cell::TableCellKind::Auto,
                }
            }
        };
        if cell.source_span.is_detached() && !origin.is_detached() {
            cell.source_span = origin;
        }
        use crate::entities::elements::table_cell::TableCellKind;
        cell.kind = match cell.kind {
            TableCellKind::Auto => TableCellKind::Header {
                level,
                scope: crate::entities::elements::table_cell::TableHeaderScope::Column,
            },
            TableCellKind::Header { .. } | TableCellKind::Data => cell.kind,
        };
        let product = Content::TableCell(std::sync::Arc::new(cell));
        let product = if source_wrapped {
            Content::Source(std::sync::Arc::new(
                crate::entities::content::SourceContent { body: product, span: origin },
            ))
        } else {
            product
        };
        cell_values.push(product);
    }
    if cell_values.is_empty() {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "table_header() exige pelo menos uma célula como argumento posicional"
                .to_string(),
        )]);
    }
    let body = Content::sequence(cell_values);

    for key in args.named.keys() {
        if !["repeat", "level"].contains(&key.as_str()) {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("table_header(): argumento nomeado inesperado '{}' (atributos avançados scope-out per ADR-0054 graded — refino futuro)", key),
            )]);
        }
    }

    let repeat = extract_bool_with_default(args, "table_header", "repeat", true)?;

    Ok(Value::Content(Content::table_header(body, repeat)))
}

fn table_header_levels(content: &Content) -> std::collections::BTreeSet<u32> {
    fn collect(content: &Content, levels: &mut std::collections::BTreeSet<u32>) {
        match content.source_body() {
            Content::TableHeader(header) => collect(&header.body, levels),
            Content::Sequence(items) => {
                for item in items.iter() {
                    collect(item, levels);
                }
            }
            Content::TableCell(cell) => {
                use crate::entities::elements::table_cell::TableCellKind;
                match cell.kind {
                    TableCellKind::Header { level, .. } => {
                        levels.insert(level);
                    }
                    TableCellKind::Auto | TableCellKind::Data => {}
                }
            }
            Content::Source(_)
            | Content::ShowRecipe(_)
            | Content::Realized(_)
            | Content::Located(_)
            | Content::Empty
            | Content::Text(_)
            | Content::Space
            | Content::SpaceFromNewline
            | Content::Parbreak
            | Content::HtmlElem(_)
            | Content::HtmlFrame(_)
            | Content::Par { .. }
            | Content::Heading(_)
            | Content::Title(_)
            | Content::Strong(_)
            | Content::Emph(_)
            | Content::Raw(_)
            | Content::ListItem(_)
            | Content::EnumItem(_)
            | Content::Link(_)
            | Content::Equation(_)
            | Content::MathSequence(_)
            | Content::MathIdent(_)
            | Content::MathText(_)
            | Content::MathFrac(_)
            | Content::MathAttach(_)
            | Content::MathPrimes(_)
            | Content::MathRoot(_)
            | Content::MathStretch(_)
            | Content::MathDelimited(_)
            | Content::MathLr(_)
            | Content::MathMid(_)
            | Content::MathOverbrace(_)
            | Content::MathUnderbrace(_)
            | Content::MathUnderbracket(_)
            | Content::MathUnderparen(_)
            | Content::MathUndershell(_)
            | Content::MathOverbracket(_)
            | Content::MathOverparen(_)
            | Content::MathOvershell(_)
            | Content::MathAlignPoint(_)
            | Content::Linebreak(_)
            | Content::MathMatrix(_)
            | Content::MathCases(_)
            | Content::MathAccent(_)
            | Content::MathCancel(_)
            | Content::MathUnderline(_)
            | Content::MathOverline(_)
            | Content::MathVec(_)
            | Content::MathClassOverride(_)
            | Content::MathLimitsOverride(_)
            | Content::MathUnderover(_)
            | Content::MathOp(_)
            | Content::MathStyled(_)
            | Content::Label(_)
            | Content::Ref(_)
            | Content::CounterDisplay(_)
            | Content::CounterUpdate(_)
            | Content::Outline(_)
            | Content::OutlineEntry(_)
            | Content::Figure(_)
            | Content::FigureCaption(_)
            | Content::Image(_)
            | Content::Shape(_)
            | Content::Curve(_)
            | Content::Transform(_)
            | Content::Grid(_)
            | Content::GridHeader(_)
            | Content::GridFooter(_)
            | Content::GridCell(_)
            | Content::SetPage(_)
            | Content::PageRun(_)
            | Content::Align(_)
            | Content::Place(_)
            | Content::Flush(_)
            | Content::Styled(_)
            | Content::Divider(_)
            | Content::Terms(_)
            | Content::TermItem(_)
            | Content::Quote(_)
            | Content::Document { .. }
            | Content::Asset { .. }
            | Content::SmartQuote(_)
            | Content::PdfAttach(_)
            | Content::PdfArtifact(_)
            | Content::Underline(_)
            | Content::Strike(_)
            | Content::Overline(_)
            | Content::SmallCaps { .. }
            | Content::Pad(_)
            | Content::Hide(_)
            | Content::HSpace(_)
            | Content::VSpace(_)
            | Content::Pagebreak(_)
            | Content::Colbreak(_)
            | Content::Stack(_)
            | Content::Boxed(_)
            | Content::Block(_)
            | Content::Bibliography(_)
            | Content::Cite(_)
            | Content::Footnote(_)
            | Content::FootnoteEntry(_)
            | Content::TableFooter(_)
            | Content::GridHLine(_)
            | Content::GridVLine(_)
            | Content::TableHLine(_)
            | Content::TableVLine(_)
            | Content::Table(_)
            | Content::Repeat(_)
            | Content::Columns(_)
            | Content::Metadata(_)
            | Content::State(_)
            | Content::StateUpdate(_)
            | Content::StateDisplay(_)
            | Content::CounterDisplayCallback(_)
            | Content::ContextBlock(_)
            | Content::Dynamic(_) => {
                levels.insert(1);
            }
        }
    }

    let mut levels = std::collections::BTreeSet::new();
    collect(content, &mut levels);
    levels
}

/// `table_footer(body, repeat: true)` → `Content::TableFooter`.
///
/// Converte cada argumento em uma célula crua; preserva Cell explícita e
/// sua origem. Ao contrário do header, não promove o kind Auto.
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
        let content = match v {
            Value::Content(c) => c.clone(),
            Value::Str(s) => Content::text(s.as_str()),
            other => return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("table_footer() espera content ou string como argumento posicional, recebeu {}", other.type_name()),
            )]),
        };
        let origin = content.source_span();
        let source_wrapped = matches!(&content, Content::Source(_));
        if matches!(content.source_body(), Content::TableCell(_)) {
            cell_values.push(content);
        } else {
            let Content::TableCell(mut cell) =
                Content::table_cell(content, None, None, None, None)
            else {
                unreachable!()
            };
            std::sync::Arc::make_mut(&mut cell).source_span =
                if origin.is_detached() { args.span } else { origin };
            let product = Content::TableCell(cell);
            let product = if source_wrapped {
                Content::Source(std::sync::Arc::new(
                    crate::entities::content::SourceContent {
                        body: product,
                        span: origin,
                    },
                ))
            } else {
                product
            };
            cell_values.push(product);
        }
    }
    if cell_values.is_empty() {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "table_footer() exige pelo menos uma célula como argumento posicional"
                .to_string(),
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
                Some(crate::compiler::stdlib::layout::extract_stroke(value, "grid_cell", "stroke")?)
            },
            "fill" => match value {
                Value::Color(c) => fill = Some(c.clone()),
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
    // lab/results/construction/diagnostics/paridade-producao-p772i.md.
    let mut cell_values: Vec<Content> = Vec::with_capacity(args.items.len());
    for v in args.items.iter() {
        let classified = match v {
            Value::Content(c) => Value::Content(c.source_body().clone()),
            other => other.clone(),
        };
        match &classified {
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
            "grid_header() exige pelo menos uma célula como argumento posicional"
                .to_string(),
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
        let classified = match v {
            Value::Content(c) => Value::Content(c.source_body().clone()),
            other => other.clone(),
        };
        match &classified {
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
            "grid_footer() exige pelo menos uma célula como argumento posicional"
                .to_string(),
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

#[cfg(test)]
mod cell_presence_tests {
    use super::*;
    use crate::contracts::world::World;
    use crate::entities::{
        args::ArgOccurrence,
        font_book::FontBook,
        source::Source,
        world_types::{Bytes, Datetime, FileError, FileResult, Font, Library},
    };
    #[derive(Default)]
    struct NullWorld {
        library: Library,
        book: FontBook,
    }
    impl World for NullWorld {
        fn library(&self) -> &Library {
            &self.library
        }
        fn book(&self) -> &FontBook {
            &self.book
        }
        fn main(&self) -> FileId {
            FileId::from_raw(std::num::NonZeroU16::new(1).unwrap())
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
        fn today(
            &self,
            _: Option<crate::entities::duration::Duration>,
        ) -> Option<Datetime> {
            None
        }
    }
    fn cell(field: &str, value: Value) -> SourceResult<Value> {
        let world = NullWorld::default();
        let args = Args::from_occurrences(
            Span::detached(),
            vec![
                ArgOccurrence {
                    name: None,
                    value: Value::Content(Content::text("a")),
                    span: Span::detached(),
                    value_span: Span::detached(),
                },
                ArgOccurrence {
                    name: Some(field.into()),
                    value,
                    span: Span::detached(),
                    value_span: Span::from_range(world.main(), 7..11),
                },
            ],
        );
        native_table_cell(&mut EvalContext::new(), &args, &world, world.main())
    }
    #[test]
    fn cell_presence_explicit_none_remains_known() {
        for name in ["fill", "stroke"] {
            let Value::Content(c) = cell(name, Value::None).unwrap() else {
                panic!("cell content")
            };
            assert_eq!(c.get_field(name), Some(Value::None));
            assert_eq!(
                Content::table_cell(Content::text("a"), None, None, None, None)
                    .get_field(name),
                None
            );
        }
    }
    #[test]
    fn cell_presence_auto_survives_body_transformation() {
        for name in ["x", "y", "fill", "align", "inset", "breakable"] {
            let Value::Content(c) = cell(name, Value::Auto).unwrap() else {
                panic!("cell content")
            };
            assert_eq!(c.get_field(name), Some(Value::Auto));
            assert_eq!(
                c.map_text(&mut |s| s.to_uppercase()).get_field(name),
                Some(Value::Auto)
            );
            let mapped = c
                .map_content(&mut |c| {
                    if matches!(c, Content::Text(_)) {
                        Ok(Some(Content::text("b")))
                    } else {
                        Ok(None)
                    }
                })
                .unwrap();
            assert_eq!(mapped.get_field(name), Some(Value::Auto));
            let pre = c.map_content_pre(&mut |_| Ok(None)).unwrap();
            assert_eq!(pre.get_field(name), Some(Value::Auto));
            let normalized = crate::compiler::eval::table::normalize_table_cell(
                &Value::Content(c.clone()),
            )
            .unwrap();
            assert_eq!(
                normalized.presence,
                match &c {
                    Content::TableCell(e) => e.presence,
                    _ => unreachable!(),
                }
            );
            assert_eq!(
                mapped.get_field("body"),
                Some(Value::Content(Content::text("b")))
            );
        }
    }
    #[test]
    fn cell_presence_invalid_null_and_auto_use_value_span() {
        for (name, value, message) in [
            ("x", Value::None, "expected integer or auto, found none"),
            ("colspan", Value::Auto, "expected integer, found auto"),
            ("breakable", Value::None, "expected boolean or auto, found none"),
        ] {
            let error = cell(name, value).unwrap_err();
            assert_eq!(error[0].message, message);
            assert_eq!(
                error[0].span,
                Span::from_range(NullWorld::default().main(), 7..11)
            );
        }
    }
}

#[cfg(test)]
mod v16_table_tests {

    use crate::contracts::world::World;
    use crate::entities::{
        file_id::FileId,
        font_book::FontBook,
        source::Source,
        world_types::{Bytes, Datetime, FileError, FileResult, Font, Library},
    };
    struct TestWorld {
        library: Library,
        book: FontBook,
        source: Source,
    }
    impl TestWorld {
        fn new(text: &str) -> Self {
            Self {
                library: Library::new(),
                book: FontBook::new(),
                source: Source::new(
                    FileId::from_raw(std::num::NonZeroU16::new(1).unwrap()),
                    text.into(),
                ),
            }
        }
    }
    impl World for TestWorld {
        fn library(&self) -> &Library {
            &self.library
        }
        fn book(&self) -> &FontBook {
            &self.book
        }
        fn main(&self) -> FileId {
            self.source.id()
        }
        fn source(&self, _: FileId) -> FileResult<Source> {
            Ok(self.source.clone())
        }
        fn file(&self, _: FileId) -> FileResult<Bytes> {
            Err(FileError::NotFound)
        }
        fn font(&self, _: usize) -> Option<Font> {
            None
        }
        fn today(
            &self,
            _: Option<crate::entities::duration::Duration>,
        ) -> Option<Datetime> {
            None
        }
    }

    use super::*;
    use crate::entities::elements::table_cell::{
        TableCellElem, TableCellKind, TableCellStage, TableCellStandaloneResolved,
        TableHeaderScope,
    };
    use std::sync::Arc;
    fn span(at: usize) -> Span {
        Span::from_range(TestWorld::new("").main(), at..at + 1)
    }
    fn sourced(c: Content) -> Content {
        Content::Source(Arc::new(crate::entities::content::SourceContent {
            body: c,
            span: span(1),
        }))
    }
    fn unwrap_value(v: Value) -> Content {
        let Value::Content(c) = v else { panic!("content") };
        c
    }
    fn raw(kind: TableCellKind) -> Content {
        let world = TestWorld::new("");
        let mut args = Args::positional(vec![Value::Content(Content::text("H"))]);
        args.span = span(3);
        for (name, n) in [("x", 0), ("colspan", 2), ("rowspan", 3)] {
            args.named.insert(name.into(), Value::Int(n));
        }
        let mut c = unwrap_value(
            native_table_cell(&mut EvalContext::new(), &args, &world, world.main())
                .unwrap(),
        );
        let Content::TableCell(cell) = &mut c else { panic!("cell") };
        Arc::make_mut(cell).kind = kind;
        c
    }
    fn header(c: Content) -> Content {
        let world = TestWorld::new("");
        let mut args = Args::positional(vec![Value::Content(c)]);
        args.span = span(9);
        args.named.insert("level".into(), Value::Int(2));
        unwrap_value(
            native_table_header(&mut EvalContext::new(), &args, &world, world.main())
                .unwrap(),
        )
    }
    fn header_cell(h: &Content) -> &TableCellElem {
        let Content::TableHeader(h) = h.source_body() else { panic!("header") };
        let Content::TableCell(c) = h.body.source_body() else { panic!("cell") };
        c
    }
    fn footer(c: Vec<Content>) -> Content {
        let world = TestWorld::new("");
        let mut args = Args::positional(c.into_iter().map(Value::Content).collect());
        args.span = span(9);
        unwrap_value(
            native_table_footer(&mut EvalContext::new(), &args, &world, world.main())
                .unwrap(),
        )
    }
    fn footer_body(c: &Content) -> &Content {
        let Content::TableFooter(f) = c.source_body() else { panic!("footer") };
        &f.body
    }
    fn assert_header(kind: TableCellKind, expected: TableCellKind) {
        let h = header(sourced(raw(kind)));
        let c = header_cell(&h);
        assert_eq!((c.x, c.colspan, c.rowspan), (Some(0), Some(2), Some(3)));
        assert_eq!(c.kind, expected);
        assert_eq!(c.source_span, span(3));
        assert!(c.presence.colspan && c.presence.rowspan);
        let Content::TableHeader(h) = h.source_body() else { panic!("header") };
        assert_eq!(h.body.source_span(), span(1));
        assert!(!matches!(c.body.source_body(), Content::TableCell(_)));
    }
    #[test]
    fn v16_table_header_source_auto() {
        assert_header(
            TableCellKind::Auto,
            TableCellKind::Header { level: 2, scope: TableHeaderScope::Column },
        );
    }
    #[test]
    fn v16_table_header_source_data() {
        assert_header(TableCellKind::Data, TableCellKind::Data);
    }
    #[test]
    fn v16_table_header_source_explicit_header() {
        let k = TableCellKind::Header { level: 7, scope: TableHeaderScope::Row };
        assert_header(k, k);
    }
    #[test]
    fn v16_table_header_raw_uses_value_origin() {
        let h = header(sourced(Content::text("raw")));
        let c = header_cell(&h);
        assert_eq!(c.source_span, span(1));
        assert!(c.stage().is_none());
        assert_eq!(c.body.plain_text(), "raw");
    }
    #[test]
    fn v16_table_header_preserves_prepared_occurrence() {
        let mut c = raw(TableCellKind::Data);
        let Content::TableCell(e) = &mut c else { panic!("cell") };
        let e = Arc::make_mut(e);
        e.standalone = Some(Arc::new(TableCellStandaloneResolved {
            visual: Content::text("shown"),
            stage: TableCellStage::Complete,
            guards: vec![41],
            properties: None,
        }));
        let h = header(sourced(c));
        let c = header_cell(&h);
        assert_eq!(c.stage(), Some(TableCellStage::Complete));
        assert_eq!(c.guards(), &[41]);
        assert_eq!(c.kind, TableCellKind::Data);
    }
    #[test]
    fn v16_table_footer_one_sequence_argument_is_one_raw_cell() {
        let arg = Content::Sequence(vec![Content::text("a"), Content::text("b")].into());
        let f = footer(vec![sourced(arg)]);
        let Content::TableCell(c) = footer_body(&f).source_body() else {
            panic!("one raw cell per argument")
        };
        assert_eq!(c.body.plain_text(), "ab");
        assert!(c.stage().is_none());
        assert!(c.resolved.is_none());
        assert_eq!(c.source_span, span(1));
        assert_eq!(footer_body(&f).source_span(), span(1));
    }
    #[test]
    fn v16_table_footer_explicit_cell_keeps_origin_and_properties() {
        let f = footer(vec![sourced(raw(TableCellKind::Data))]);
        assert_eq!(footer_body(&f).source_span(), span(1));
        let Content::TableCell(c) = footer_body(&f).source_body() else { panic!("cell") };
        assert_eq!((c.colspan, c.rowspan), (Some(2), Some(3)));
        assert_eq!(c.kind, TableCellKind::Data);
        assert_eq!(c.source_span, span(3));
    }
    #[test]
    fn v16_table_footer_two_arguments_are_two_raw_cells() {
        let f = footer(vec![Content::text("a"), Content::text("b")]);
        let Content::Sequence(items) = footer_body(&f).source_body() else {
            panic!("two cells")
        };
        assert_eq!(items.len(), 2);
        for c in items.iter() {
            let Content::TableCell(e) = c.source_body() else { panic!("raw cell") };
            assert!(e.stage().is_none());
        }
    }
    fn headers(a: u32, b: u32) -> SourceResult<Value> {
        let h = |level| {
            Content::table_header(
                sourced(raw(TableCellKind::Header {
                    level,
                    scope: TableHeaderScope::Column,
                })),
                true,
            )
        };
        let world = TestWorld::new("");
        native_table(
            &mut EvalContext::new(),
            &Args::positional(vec![Value::Content(h(a)), Value::Content(h(b))]),
            &world,
            world.main(),
        )
    }
    #[test]
    fn v16_table_header_source_levels_remain_distinct() {
        assert!(headers(2, 3).is_ok());
    }
    #[test]
    fn v16_table_header_source_duplicate_level_still_errors() {
        let e = headers(2, 2).unwrap_err();
        assert!(e[0].message.contains("mais do que um header"));
    }
}
