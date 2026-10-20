//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/layout/grid.md
//! @prompt-hash c561d849
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
        Value::None
        | Value::Bool(_)
        | Value::Int(_)
        | Value::Str(_)
        | Value::Array(_)
        | Value::Dict(_)
        | Value::Module(_)
        | Value::Datetime(_)
        | Value::Func(_)
        | Value::Content(_)
        | Value::LocatedContent(_, _)
        | Value::Relative(_)
        | Value::Ratio(_)
        | Value::Angle(_)
        | Value::Color(_)
        | Value::Stroke(_)
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
        | Value::SpotColorant(_) => None,
    }
}

/// Shared Grid/Table conversion. Malformed supplied tracks are errors;
/// dimensional omissions remain the separately inventoried RM1 carrier debt.
pub(crate) fn extract_tracks(
    val: Option<&Value>,
    span: Span,
) -> SourceResult<Vec<TrackSizing>> {
    use crate::compiler::eval::operators::error_formatting::vanilla_type_name;
    match val {
        Some(Value::Array(values)) => {
            let mut tracks = Vec::with_capacity(values.len());
            for value in values {
                if let Some(track) = parse_track_sizing(value) {
                    tracks.push(track);
                } else {
                    match value {
                        // RM1: preserve measured dimensional omission until the
                        // public track carrier can retain relative dimensions.
                        Value::Ratio(_) | Value::Relative(_) => {}
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
                        | Value::Length(_)
                        | Value::Angle(_)
                        | Value::Color(_)
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
                        | Value::SpotColorant(_) => {
                            return Err(vec![SourceDiagnostic::error(
                                span,
                                format!(
                                    "expected auto, relative length, or fraction, found {}",
                                    vanilla_type_name(value)
                                ),
                            )]);
                        }
                    }
                }
            }
            Ok(tracks)
        }
        Some(Value::Int(count)) => {
            if *count <= 0 {
                return Err(vec![SourceDiagnostic::error(span, "number must be positive")]);
            }
            Ok(vec![TrackSizing::Auto; *count as usize])
        }
        // This is retained RM1 debt, not a malformed-type diagnostic or final
        // relative-track semantics. It must not become a catch-all fallback.
        Some(Value::Ratio(_) | Value::Relative(_)) | None => Ok(vec![]),
        Some(value) => parse_track_sizing(value).map(|track| vec![track]).ok_or_else(|| {
            vec![SourceDiagnostic::error(
                span,
                format!(
                    "expected auto, relative length, fraction, integer, or array, found {}",
                    vanilla_type_name(value)
                ),
            )]
        }),
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
    let value_span = |name: &str| {
        args.occurrences
            .as_ref()
            .and_then(|occurrences| {
                occurrences.iter().rev().find(|arg| arg.name.as_deref() == Some(name))
            })
            .map_or(args.span, |arg| arg.value_span.or(arg.span).or(args.span))
    };
    let mut columns = extract_tracks(args.named.get("columns"), value_span("columns"))?;
    let mut rows = extract_tracks(args.named.get("rows"), value_span("rows"))?;
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

    let mut source_order = Vec::new();
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
            use crate::entities::elements::grid::GridChildOrder;
            match c.source_body() {
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
                    source_order.push(GridChildOrder::HLine(hlines.len()));
                    hlines.push(h);
                }
                Content::GridVLine(e) => {
                    let mut vline = (**e).clone();
                    if vline.position == "auto" {
                        vline.position = EcoString::from("left");
                    }
                    vline.col = col;
                    source_order.push(GridChildOrder::VLine(vlines.len()));
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
                    source_order.push(GridChildOrder::Header);
                    header = Some(c.clone());
                }
                Content::GridFooter(_) => {
                    if footer.is_some() {
                        return Err(vec![SourceDiagnostic::error(
                            args.span,
                            "grid: não pode haver mais do que um footer".to_string(),
                        )]);
                    }
                    source_order.push(GridChildOrder::Footer);
                    footer = Some(c.clone());
                }
                _ => {
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
                    source_order.push(GridChildOrder::Cell(cells.len()));
                    cells.push(c.clone());
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
            source_order: Some(source_order),
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

#[cfg(test)]
mod batch_f_tests {
    use super::*;

    use crate::contracts::world::World;
    use crate::entities::{
        font_book::FontBook,
        source::Source,
        world_types::{Bytes, Datetime, FileError, FileResult, Font, Library},
    };
    #[derive(Default)]
    struct TestWorld {
        library: Library,
        book: FontBook,
    }
    impl World for TestWorld {
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
    fn call(args: &Args) -> SourceResult<Value> {
        let world = TestWorld::default();
        native_grid(&mut EvalContext::new(), args, &world, world.main())
    }

    fn at(start: usize) -> Span {
        Span::from_range(
            FileId::from_raw(std::num::NonZeroU16::new(1).unwrap()),
            start..start + 1,
        )
    }
    fn named(name: &str, value: Value) -> Args {
        use crate::entities::args::ArgOccurrence;
        Args::from_occurrences(
            at(1),
            vec![
                ArgOccurrence {
                    name: None,
                    value: Value::Content(Content::text("X")),
                    span: at(2),
                    value_span: at(3),
                },
                ArgOccurrence {
                    name: Some(name.into()),
                    value,
                    span: at(4),
                    value_span: at(5),
                },
            ],
        )
    }

    #[test]
    fn batch_f_grid_rejects_invalid_array_member_at_array_value_span() {
        let args = named(
            "columns",
            Value::Array(vec![
                Value::Length(Length::pt(2.0)),
                Value::Bool(true),
                Value::Length(Length::pt(3.0)),
            ]),
        );
        let errors = call(&args).unwrap_err();
        assert_eq!(
            errors[0].message,
            "expected auto, relative length, or fraction, found boolean"
        );
        assert_eq!(errors[0].span, at(5));
    }
    #[test]
    fn batch_f_grid_rejects_malformed_scalar_and_nonpositive_counts() {
        for (value, message) in [
            (Value::None, "expected auto, relative length, fraction, integer, or array, found none"),
            (Value::Str("bad".into()), "expected auto, relative length, fraction, integer, or array, found string"),
            (Value::Int(0), "number must be positive"),
            (Value::Int(-1), "number must be positive"),
        ] {
            let errors = call(&named("rows", value)).unwrap_err();
            assert_eq!(errors[0].message, message); assert_eq!(errors[0].span, at(5));
        }
    }
    #[test]
    fn batch_f_grid_columns_error_precedes_rows_and_gutter() {
        let mut args = named("columns", Value::Bool(true));
        args.named.insert("rows".into(), Value::None);
        args.named.insert("gutter".into(), Value::None);
        let errors = call(&args).unwrap_err();
        assert_eq!(
            errors[0].message,
            "expected auto, relative length, fraction, integer, or array, found boolean"
        );
        assert_eq!(errors[0].span, at(5));
    }
    #[test]
    fn batch_f_grid_preserves_valid_order_and_legacy_tracks() {
        let args = named(
            "columns",
            Value::Array(vec![
                Value::Length(Length::pt(-2.0)),
                Value::Auto,
                Value::Fraction(3.0),
                Value::Float(4.0),
                Value::Str("auto".into()),
            ]),
        );
        let Value::Content(Content::Grid(grid)) = call(&args).unwrap() else {
            panic!("grid")
        };
        assert_eq!(
            grid.columns,
            vec![
                TrackSizing::Fixed(-2.0),
                TrackSizing::Auto,
                TrackSizing::Fraction(3.0),
                TrackSizing::Fixed(4.0),
                TrackSizing::Auto
            ]
        );
        for value in [Value::Array(vec![]), Value::Int(1)] {
            let Value::Content(Content::Grid(grid)) =
                call(&named("columns", value)).unwrap()
            else {
                panic!("grid")
            };
            assert_eq!(grid.columns, vec![TrackSizing::Auto]);
        }
    }
}
