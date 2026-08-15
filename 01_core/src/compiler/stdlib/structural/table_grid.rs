//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/structural/table_grid.md
//! @prompt-hash 5bcdec0a
//! @layer L1
//! @updated 2026-08-12
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
        if !["columns", "rows", "stroke", "fill", "caption", "align", "inset", "gutter"].contains(&key.as_str()) {
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
                children.push(c.clone());
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
    let inset: crate::entities::sides::Sides<crate::entities::layout_types::Length> = match args.named.get("inset") {
        Some(Value::Length(l)) => crate::entities::sides::Sides::uniform(*l),
        Some(Value::None) => crate::entities::sides::Sides::uniform(crate::entities::layout_types::Length::pt(0.0)),
        None => crate::entities::sides::Sides::uniform(crate::entities::layout_types::Length::pt(5.0)),
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
                Some(crate::compiler::stdlib::layout::extract_stroke(value, "table_cell", "stroke")?)
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
            "table_header() exige pelo menos uma célula como argumento posicional"
                .to_string(),
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

