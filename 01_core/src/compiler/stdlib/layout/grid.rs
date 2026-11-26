//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/layout/grid.md
//! @prompt-hash 06f79103
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

/// Named arguments shared by `grid` and `table`.
pub(crate) const GRID_PROPERTIES: [&str; 9] = [
    "columns",
    "rows",
    "gutter",
    "column-gutter",
    "row-gutter",
    "fill",
    "align",
    "stroke",
    "inset",
];

/// Gutter tracks from `gutter`, `column-gutter` or `row-gutter`.
pub(crate) fn extract_gutters(
    value: &Value,
    span: Span,
) -> SourceResult<Vec<crate::entities::rel::Rel<Length>>> {
    use crate::compiler::eval::operators::error_formatting::vanilla_type_name;
    use crate::entities::rel::Rel;
    let zero = || Rel { rel: 0.0, abs: Length::ZERO };
    let one = |value: &Value| {
        if let Value::Length(length) = value {
            Some(Rel { rel: 0.0, abs: *length })
        } else if let Value::Ratio(ratio) = value {
            Some(Rel { rel: ratio.0, abs: Length::ZERO })
        } else if let Value::Relative(rel) = value {
            Some(*rel)
        } else if matches!(value, Value::Auto | Value::Fraction(_)) {
            // Auto and fractional gutters take no fixed space.
            Some(zero())
        } else {
            None
        }
    };
    let error = |value: &Value| {
        vec![SourceDiagnostic::error(
            span,
            format!(
                "expected auto, relative length, fraction, integer, or array, found {}",
                vanilla_type_name(value)
            ),
        )]
    };
    match value {
        Value::Array(items) => {
            items.iter().map(|item| one(item).ok_or_else(|| error(item))).collect()
        }
        Value::Int(count) if *count >= 0 => Ok(vec![zero(); *count as usize]),
        other => one(other).map(|gutter| vec![gutter]).ok_or_else(|| error(other)),
    }
}

/// The span of the value of a named argument, or of the whole call.
pub(crate) fn named_value_span(args: &Args, name: &str) -> Span {
    args.occurrences
        .as_ref()
        .and_then(|occurrences| {
            occurrences.iter().rev().find(|arg| arg.name.as_deref() == Some(name))
        })
        .map_or(args.span, |arg| arg.value_span.or(arg.span).or(args.span))
}

/// The span of the n-th positional argument, or of the whole call.
pub(crate) fn positional_value_span(args: &Args, index: usize) -> Span {
    args.occurrences
        .as_ref()
        .and_then(|items| items.iter().filter(|arg| arg.name.is_none()).nth(index))
        .map_or(args.span, |arg| arg.value_span.or(arg.span).or(args.span))
}

/// A positional child as content: text and `none` are content too.
pub(crate) fn grid_child(args: &Args, index: usize) -> SourceResult<Content> {
    use crate::compiler::eval::operators::error_formatting::vanilla_type_name;
    match &args.items[index] {
        Value::Content(content) => Ok(content.clone()),
        Value::Str(text) => Ok(Content::text(text.as_str())),
        Value::None => Ok(Content::Empty),
        Value::Symbol(symbol) => Ok(Content::Symbol(symbol.value.clone())),
        other => Err(vec![SourceDiagnostic::error(
            positional_value_span(args, index),
            format!("expected content, found {}", vanilla_type_name(other)),
        )]),
    }
}

/// The declared surface of a `grid` or `table` call.
pub(crate) fn grid_spec(
    args: &Args,
) -> SourceResult<crate::entities::elements::grid::GridSpec> {
    let mut spec = crate::entities::elements::grid::GridSpec::default();
    let gutter = |name: &str| {
        args.named
            .get(name)
            .map(|value| extract_gutters(value, named_value_span(args, name)))
            .transpose()
    };
    let both = gutter("gutter")?.unwrap_or_default();
    spec.column_gutter = gutter("column-gutter")?.unwrap_or_else(|| both.clone());
    spec.row_gutter = gutter("row-gutter")?.unwrap_or(both);
    let order: Vec<_> = args
        .occurrences
        .as_ref()
        .map(|items| items.iter().filter_map(|arg| arg.name.clone()).collect())
        .unwrap_or_default();
    let mut names: Vec<_> = order
        .into_iter()
        .filter(|name| args.named.contains_key(name.as_str()))
        .collect();
    for name in args.named.keys() {
        if !names.iter().any(|known| known == name) {
            names.push(name.clone());
        }
    }
    names.dedup();
    spec.named = names
        .into_iter()
        .filter_map(|name| args.named.get(name.as_str()).map(|value| (name, value.clone())))
        .collect();
    Ok(spec)
}

/// Whether a value applies per cell (array or function) rather than to all.
pub(crate) fn celled(value: Option<&Value>) -> bool {
    matches!(value, Some(Value::Array(_) | Value::Func(_)))
}

/// Fields of `grid` and `table`, in the language's declaration order.
const GRID_FIELDS: [&str; 9] = [
    "columns",
    "rows",
    "column-gutter",
    "row-gutter",
    "inset",
    "align",
    "fill",
    "stroke",
    "children",
];

fn track_value(track: &TrackSizing) -> Value {
    match track {
        TrackSizing::Auto => Value::Auto,
        TrackSizing::Fixed(pt) => Value::Relative(crate::entities::rel::Rel {
            rel: 0.0,
            abs: Length::pt(*pt),
        }),
        TrackSizing::Fraction(fr) => Value::Fraction(*fr),
    }
}

/// Sides written with `x`, `y` and `rest` expanded, in left-top-right-bottom order.
fn sides_value(value: &Value, map: impl Fn(&Value) -> Value) -> Value {
    let Value::Dict(dict) = value else { return map(value) };
    let mut out = indexmap::IndexMap::default();
    for (side, axis) in [("left", "x"), ("top", "y"), ("right", "x"), ("bottom", "y")] {
        if let Some(found) =
            dict.get(side).or_else(|| dict.get(axis)).or_else(|| dict.get("rest"))
        {
            out.insert(EcoString::from(side), map(found));
        }
    }
    Value::Dict(out)
}

fn relative(value: &Value) -> Value {
    match value {
        Value::Length(length) => {
            Value::Relative(crate::entities::rel::Rel { rel: 0.0, abs: *length })
        }
        other => other.clone(),
    }
}

/// A child as the cell the language shows it as.
fn as_grid_cell(content: &Content) -> Content {
    match content.source_body() {
        Content::GridCell(_) => content.clone(),
        _ => Content::GridCell(std::sync::Arc::new(
            crate::entities::elements::grid_cell::GridCellElem {
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
            },
        )),
    }
}

fn as_table_cell(content: &Content) -> Content {
    match content.source_body() {
        Content::TableCell(_) => content.clone(),
        _ => Content::table_cell(content.clone(), None, None, None, None),
    }
}

/// Children of a `grid` or `table`, in source order where it is known.
fn grid_children(content: &Content) -> Vec<Value> {
    use crate::entities::elements::grid::GridChildOrder;
    let mut out = Vec::new();
    if let Content::Grid(e) = content {
        match &e.source_order {
            Some(order) => {
                for item in order {
                    out.push(match item {
                        GridChildOrder::Cell(i) => as_grid_cell(&e.cells[*i]),
                        GridChildOrder::Header => e.header.clone().unwrap_or(Content::Empty),
                        GridChildOrder::Footer => e.footer.clone().unwrap_or(Content::Empty),
                        GridChildOrder::HLine(i) => {
                            Content::GridHLine(std::sync::Arc::new(e.hlines[*i].clone()))
                        }
                        GridChildOrder::VLine(i) => {
                            Content::GridVLine(std::sync::Arc::new(e.vlines[*i].clone()))
                        }
                    });
                }
            }
            None => out.extend(e.cells.iter().map(as_grid_cell)),
        }
    } else if let Content::Table(e) = content {
        match &e.items {
            Some(items) => out.extend(items.iter().map(|item| match item.source_body() {
                Content::TableHeader(_) | Content::TableFooter(_) => item.clone(),
                _ => as_table_cell(item),
            })),
            None => {
                out.extend(e.header.clone());
                out.extend(e.children.iter().map(as_table_cell));
                out.extend(e.footer.clone());
            }
        }
    }
    out.into_iter().map(Value::Content).collect()
}

/// A field of a `grid` or `table` as the language reads it.
pub(crate) fn grid_field(content: &Content, field: &str) -> Option<Option<Value>> {
    let (spec, columns, rows) = if let Content::Grid(e) = content {
        (e.spec.as_deref(), &e.columns, &e.rows)
    } else if let Content::Table(e) = content {
        (e.spec.as_deref(), &e.columns, &e.rows)
    } else {
        return None;
    };
    if !GRID_FIELDS.contains(&field) {
        return None;
    }
    let named = |name: &str| spec.and_then(|spec| spec.named(name));
    let gutter = |name: &str, gutters: &[crate::entities::rel::Rel<Length>]| {
        (named(name).is_some() || named("gutter").is_some())
            .then(|| Value::Array(gutters.iter().map(|g| Value::Relative(*g)).collect()))
    };
    Some(match field {
        "children" => Some(Value::Array(grid_children(content))),
        "columns" => named("columns")
            .map(|_| Value::Array(columns.iter().map(track_value).collect())),
        "rows" => named("rows").map(|_| Value::Array(rows.iter().map(track_value).collect())),
        "column-gutter" => spec.and_then(|spec| gutter("column-gutter", &spec.column_gutter)),
        "row-gutter" => spec.and_then(|spec| gutter("row-gutter", &spec.row_gutter)),
        "inset" => named("inset").map(|value| sides_value(value, relative)),
        "stroke" => named("stroke").map(|value| sides_value(value, Value::clone)),
        other => named(other).cloned(),
    })
}

/// The fields of a `grid` or `table` that are set, in declaration order.
pub(crate) fn grid_fields(content: &Content) -> Option<Vec<(&'static str, Value)>> {
    if !matches!(content, Content::Grid(_) | Content::Table(_)) {
        return None;
    }
    Some(
        GRID_FIELDS
            .iter()
            .filter_map(|name| grid_field(content, name).flatten().map(|value| (*name, value)))
            .collect(),
    )
}

/// The set fields of a grid cell, in declaration order.
pub(crate) fn grid_cell_fields(content: &Content) -> Option<Vec<(&'static str, Value)>> {
    let Content::GridCell(e) = content else { return None };
    let int = |value: Option<usize>| value.map(|v| Value::Int(v as i64));
    let fields: [(&'static str, Option<Value>); 10] = [
        ("body", Some(Value::Content(e.body.clone()))),
        ("x", int(e.x)),
        ("y", int(e.y)),
        ("colspan", int(e.colspan)),
        ("rowspan", int(e.rowspan)),
        ("fill", e.fill.clone().map(Value::Color)),
        ("align", e.align.map(Value::Align)),
        (
            "inset",
            e.inset.map(|sides| {
                Value::Relative(crate::entities::rel::Rel { rel: 0.0, abs: sides.left })
            }),
        ),
        ("stroke", e.stroke.clone().map(|s| Value::Stroke(std::sync::Arc::new(s)))),
        ("breakable", e.breakable.map(Value::Bool)),
    ];
    Some(fields.into_iter().filter_map(|(name, value)| value.map(|v| (name, v))).collect())
}

/// `grid(columns?, rows?, ...cells)` → `Content::Grid`.
pub fn native_grid(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    for key in args.named.keys() {
        if !GRID_PROPERTIES.contains(&key.as_str()) {
            return Err(vec![SourceDiagnostic::error(
                named_value_span(args, key),
                format!("unexpected argument: {key}"),
            )]);
        }
    }
    let mut columns =
        extract_tracks(args.named.get("columns"), named_value_span(args, "columns"))?;
    let mut rows = extract_tracks(args.named.get("rows"), named_value_span(args, "rows"))?;
    if columns.is_empty() {
        columns = vec![TrackSizing::Auto];
    }
    if rows.is_empty() {
        rows = vec![TrackSizing::Auto];
    }
    let spec = grid_spec(args)?;

    // Uniform values apply here; arrays and functions apply per cell when
    // the grid is realized.
    let align = match args.named.get("align") {
        Some(Value::Align(a)) => Some(*a),
        Some(Value::Auto | Value::None) | None => None,
        Some(value) if celled(Some(value)) => None,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                named_value_span(args, "align"),
                format!(
                    "expected alignment, array, function, or auto, found {}",
                    crate::compiler::eval::operators::error_formatting::vanilla_type_name(
                        other
                    )
                ),
            )])
        }
    };
    let inset = match args.named.get("inset") {
        None | Some(Value::None | Value::Auto) => Sides::uniform(Length::pt(0.0)),
        Some(value) if celled(Some(value)) => Sides::uniform(Length::pt(0.0)),
        Some(value) => {
            let sides = crate::entities::elements::table_cell::TableCellProperties::style_inset(
                value,
            )
            .ok_or_else(|| {
                vec![SourceDiagnostic::error(
                    named_value_span(args, "inset"),
                    format!(
                        "expected relative length, dictionary, array, function, or none, found {}",
                        crate::compiler::eval::operators::error_formatting::vanilla_type_name(
                            value
                        )
                    ),
                )]
            })?;
            Sides::new(
                sides.left.unwrap_or(Length::ZERO),
                sides.top.unwrap_or(Length::ZERO),
                sides.right.unwrap_or(Length::ZERO),
                sides.bottom.unwrap_or(Length::ZERO),
            )
        }
    };

    let mut header: Option<Content> = None;
    let mut footer: Option<Content> = None;
    let mut source_order = Vec::new();
    let mut cells: Vec<Content> = Vec::new();
    let mut hlines: Vec<crate::entities::elements::grid_hline::GridHLineElem> =
        Vec::new();
    let mut vlines: Vec<crate::entities::elements::grid_vline::GridVLineElem> =
        Vec::new();
    // P512 — linhas guardam a linha/coluna corrente da colocação automática.
    let num_cols = columns.len().max(1);
    let mut row = 0usize;
    let mut col = 0usize;
    for index in 0..args.items.len() {
        let c = grid_child(args, index)?;
        use crate::entities::elements::grid::GridChildOrder;
        match c.source_body() {
            Content::GridHLine(e) => {
                let mut h = (**e).clone();
                if h.row == usize::MAX {
                    if h.position == "auto" {
                        h.position = if col == 0 {
                            EcoString::from("top")
                        } else {
                            EcoString::from("bottom")
                        };
                    }
                    h.row = row;
                } else if h.position == "auto" {
                    h.position = EcoString::from("top");
                }
                source_order.push(GridChildOrder::HLine(hlines.len()));
                hlines.push(h);
            }
            Content::GridVLine(e) => {
                let mut vline = (**e).clone();
                if vline.position == "auto" {
                    vline.position = EcoString::from("left");
                }
                if vline.col == usize::MAX {
                    vline.col = col;
                }
                source_order.push(GridChildOrder::VLine(vlines.len()));
                vlines.push(vline);
            }
            // P772i — header and footer are children, outside auto-placement.
            Content::GridHeader(_) => {
                if header.is_some() {
                    return Err(vec![SourceDiagnostic::error(
                        positional_value_span(args, index),
                        "grid: não pode haver mais do que um header (scope-out — múltiplos headers por `level` não suportados)".to_string(),
                    )]);
                }
                source_order.push(GridChildOrder::Header);
                header = Some(c.clone());
            }
            Content::GridFooter(_) => {
                if footer.is_some() {
                    return Err(vec![SourceDiagnostic::error(
                        positional_value_span(args, index),
                        "grid: não pode haver mais do que um footer".to_string(),
                    )]);
                }
                source_order.push(GridChildOrder::Footer);
                footer = Some(c.clone());
            }
            _ => {
                // P822 — the footer must end at the last row.
                if footer.is_some() {
                    return Err(vec![SourceDiagnostic::error(
                        positional_value_span(args, index),
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
    // P726 — `stroke: none` draws nothing.
    let stroke = match args.named.get("stroke") {
        Some(Value::None) | None => None,
        // Arrays, functions and sides apply per cell when the grid is realized.
        Some(Value::Array(_) | Value::Func(_) | Value::Dict(_)) => None,
        Some(val) => Some(extract_stroke(val, "grid", "stroke")?),
    };
    let fill = match args.named.get("fill") {
        Some(Value::Color(c)) => Some(c.clone()),
        Some(Value::None) | None => None,
        Some(value) if celled(Some(value)) => None,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                named_value_span(args, "fill"),
                format!(
                    "expected color, gradient, tiling, none, array, or function, found {}",
                    crate::compiler::eval::operators::error_formatting::vanilla_type_name(
                        other
                    )
                ),
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
            gutter: None,
            align,
            inset,
            header,
            footer,
            stroke,
            fill,
            spec: Some(std::sync::Arc::new(spec)),
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
