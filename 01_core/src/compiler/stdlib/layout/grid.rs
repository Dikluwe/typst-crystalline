//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/layout/grid.md
//! @prompt-hash 4b2e9ad8
//! @layer L1
//! @updated 2026-09-30
//!
//! Comportamento de `stdlib/layout::grid`.

use super::*;

pub(crate) fn parse_track_sizing(val: &Value) -> Option<TrackSizing> {
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
pub(crate) fn extract_tracks(val: Option<&Value>) -> Vec<TrackSizing> {
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
        // lab/results/construction/diagnostics/paridade-producao-p772i.md.
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
    // explícito — ver lab/results/construction/diagnostics/paridade-producao-p772i.md).
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
        Some(Value::Color(c)) => Some(c.clone()),
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
