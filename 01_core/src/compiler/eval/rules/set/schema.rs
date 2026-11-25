//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/eval/rules/set/schema.md
//! @prompt-hash 0630759e
//! @layer L1
//! @updated 2026-11-24
//!
//! Propriedades configuráveis por elemento e os tipos que cada uma admite.

use crate::compiler::eval::bindings::native_element_fields;
use crate::compiler::eval::operators::error_formatting::vanilla_type_name;
use crate::entities::args::{ArgOccurrence, Args};
use crate::entities::func::{Func, FuncRepr};
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::value::Value;

/// Chave do elemento nativo, pela identidade da função.
pub(super) fn element_key(function: &Func) -> Option<&'static str> {
    if matches!(function.repr(), FuncRepr::With(_)) {
        return None;
    }
    let (name, fields, _) = native_element_fields(function)?;
    let has = |field: &str| fields.contains(&field);
    Some(match name {
        "cell" => "grid.cell",
        "hline" => "grid.hline",
        "vline" => "grid.vline",
        "header" => "grid.header",
        "footer" => "grid.footer",
        "caption" => "figure.caption",
        "entry" if has("note") => "footnote.entry",
        "entry" => "outline.entry",
        "line" if has("length") => "line",
        "line" if has("numbering") => "par.line",
        "line" => "curve.line",
        "move" if has("dx") => "move",
        "move" => "curve.move",
        "close" => "curve.close",
        "cubic" => "curve.cubic",
        "quad" => "curve.quad",
        "attach" if has("base") => "math.attach",
        "attach" => "pdf.attach",
        "artifact" => "pdf.artifact",
        "accent" => "math.accent",
        "cancel" => "math.cancel",
        "cases" => "math.cases",
        "equation" => "math.equation",
        "frac" => "math.frac",
        "limits" => "math.limits",
        "lr" => "math.lr",
        "mat" => "math.mat",
        "op" => "math.op",
        "stretch" => "math.stretch",
        "vec" => "math.vec",
        other => other,
    })
}

/// Confere nomes e tipos. Elementos fora da tabela não são conferidos.
pub(in crate::compiler::eval::rules) fn check(
    element: &str,
    args: &Args,
) -> SourceResult<()> {
    let Some(schema) = properties(element) else { return Ok(()) };
    for arg in args.occurrence_sequence() {
        check_argument(element, schema, &arg, args)?;
    }
    Ok(())
}

fn check_argument(
    element: &str,
    schema: &[(&str, &str)],
    arg: &ArgOccurrence,
    args: &Args,
) -> SourceResult<()> {
    let error = |span: crate::entities::span::Span, message: String| {
        Err(vec![SourceDiagnostic::error(span.or(args.span), message)])
    };
    let Some(name) = arg.name.as_deref() else {
        return if matches!(element, "columns" | "rotate" | "scale") {
            Ok(())
        } else {
            error(arg.span, "unexpected argument".into())
        };
    };
    let Some((_, types)) = schema.iter().find(|(property, _)| *property == name) else {
        return error(arg.span, format!("unexpected argument: {name}"));
    };
    if types.is_empty() || admits(types, &arg.value) {
        return Ok(());
    }
    let literal = types.contains('"') && matches!(arg.value, Value::Str(_));
    let found = if literal {
        String::new()
    } else {
        format!(", found {}", vanilla_type_name(&arg.value))
    };
    let mut diagnostic = SourceDiagnostic::error(
        arg.value_span.or(args.span),
        format!("expected {types}{found}"),
    );
    let length = admits_type_name(types, "length");
    if let (Value::Int(number), true) = (&arg.value, length) {
        diagnostic = diagnostic
            .with_hint(format!("a length needs a unit - did you mean {number}pt?"));
    }
    Err(vec![diagnostic])
}

fn type_names(types: &str) -> impl Iterator<Item = &str> {
    types
        .split(", ")
        .flat_map(|part| part.split(" or "))
        .map(|part| part.trim_start_matches("or "))
}

fn admits_type_name(types: &str, name: &str) -> bool {
    type_names(types).any(|ty| ty == name)
}

fn admits(types: &str, value: &Value) -> bool {
    type_names(types).any(|ty| admits_type(ty, value))
}

fn admits_type(ty: &str, value: &Value) -> bool {
    if let Some(literal) = ty.strip_prefix('"').and_then(|ty| ty.strip_suffix('"')) {
        return matches!(value, Value::Str(text) if text.as_str() == literal);
    }
    match ty {
        "alignment" => matches!(value, Value::Align(_)),
        "angle" => matches!(value, Value::Angle(_)),
        "array" => matches!(value, Value::Array(_)),
        "auto" => matches!(value, Value::Auto),
        "boolean" => matches!(value, Value::Bool(_)),
        "bytes" => matches!(value, Value::Bytes(_)),
        "color" => matches!(value, Value::Color(_)),
        "content" => matches!(
            value,
            Value::Content(_)
                | Value::LocatedContent(..)
                | Value::Str(_)
                | Value::Symbol(_)
                | Value::None
        ),
        "datetime" => matches!(value, Value::Datetime(_)),
        "dictionary" => matches!(value, Value::Dict(_)),
        "direction" => matches!(value, Value::Dir(_)),
        "fraction" => matches!(value, Value::Fraction(_)),
        "function" => matches!(value, Value::Func(_) | Value::Type(_)),
        "gradient" => matches!(value, Value::Gradient(_)),
        "integer" => matches!(value, Value::Int(_)),
        "label" => matches!(value, Value::Label(_)),
        "length" => matches!(value, Value::Length(_)),
        "location" => matches!(value, Value::Location(_)),
        "none" => matches!(value, Value::None),
        "path" => matches!(value, Value::Path(_)),
        "ratio" => matches!(value, Value::Ratio(_)),
        "relative length" => {
            matches!(value, Value::Length(_) | Value::Ratio(_) | Value::Relative(_))
        }
        "selector" => matches!(value, Value::Selector(_)),
        "string" => matches!(value, Value::Str(_) | Value::Symbol(_)),
        "stroke" => matches!(value, Value::Stroke(_)),
        "symbol" => matches!(value, Value::Symbol(_)),
        "tiling" => matches!(value, Value::Tiling(_)),
        _ => false,
    }
}

#[rustfmt::skip]
fn properties(element: &str) -> Option<&'static [(&'static str, &'static str)]> {
    Some(match element {
    "bibliography" => &[
        ("full", "boolean"),
        ("group", "string, auto, or none"),
        ("style", ""),
        ("target", "label, function, location, selector, or auto"),
        ("title", "content, none, or auto"),
    ],
    "block" => &[
        ("above", "relative length, fraction, or auto"),
        ("below", "relative length, fraction, or auto"),
        ("breakable", "boolean"),
        ("clip", "boolean"),
        ("fill", "color, gradient, tiling, or none"),
        ("height", "auto, relative length, or fraction"),
        ("inset", "relative length or dictionary"),
        ("outset", "relative length or dictionary"),
        ("radius", "relative length or dictionary"),
        ("spacing", "relative length, fraction, or auto"),
        ("sticky", "boolean"),
        ("stroke", "length, color, gradient, tiling, dictionary, stroke, or none"),
        ("width", "relative length or auto"),
    ],
    "box" => &[
        ("baseline", "alignment, auto, relative length, or dictionary"),
        ("clip", "boolean"),
        ("fill", "color, gradient, tiling, or none"),
        ("height", "relative length or auto"),
        ("inset", "relative length or dictionary"),
        ("outset", "relative length or dictionary"),
        ("radius", "relative length or dictionary"),
        ("stroke", "length, color, gradient, tiling, dictionary, stroke, or none"),
        ("width", "auto, relative length, or fraction"),
    ],
    "circle" => &[
        ("fill", "color, gradient, tiling, or none"),
        ("height", "auto, relative length, or fraction"),
        ("inset", "relative length or dictionary"),
        ("outset", "relative length or dictionary"),
        ("radius", "length or auto"),
        ("stroke", "length, color, gradient, tiling, dictionary, stroke, none, or auto"),
        ("width", "relative length or auto"),
    ],
    "cite" => &[
        ("form", "\"normal\", \"prose\", \"full\", \"author\", \"year\", or none"),
        ("style", ""),
        ("supplement", "content or none"),
    ],
    "colbreak" => &[
        ("weak", "boolean"),
    ],
    "columns" => &[
        ("balanced", "boolean"),
        ("gutter", "relative length"),
        ("separator", "content or none"),
    ],
    "curve" => &[
        ("fill", "color, gradient, tiling, or none"),
        ("fill-rule", "\"non-zero\" or \"even-odd\""),
        ("stroke", "length, color, gradient, tiling, dictionary, stroke, none, or auto"),
    ],
    "curve.close" => &[
        ("mode", "\"smooth\" or \"straight\""),
    ],
    "curve.cubic" => &[
        ("relative", "boolean"),
    ],
    "curve.quad" => &[
        ("relative", "boolean"),
    ],
    "document" => &[
        ("author", "string or array"),
        ("date", "datetime, none, or auto"),
        ("description", "content or none"),
        ("format", "\"pdf\", \"png\", \"svg\", \"html\", or auto"),
        ("keywords", "string or array"),
        ("title", "content or none"),
    ],
    "ellipse" => &[
        ("fill", "color, gradient, tiling, or none"),
        ("height", "auto, relative length, or fraction"),
        ("inset", "relative length or dictionary"),
        ("outset", "relative length or dictionary"),
        ("stroke", "length, color, gradient, tiling, dictionary, stroke, none, or auto"),
        ("width", "relative length or auto"),
    ],
    "enum" => &[
        ("body-indent", "length"),
        ("full", "boolean"),
        ("indent", "length"),
        ("number-align", "alignment"),
        ("numbering", "string or function"),
        ("reversed", "boolean"),
        ("spacing", "length or auto"),
        ("start", "integer or auto"),
        ("tight", "boolean"),
    ],
    "figure" => &[
        ("alt", "string or none"),
        ("caption", "content or none"),
        ("gap", "length"),
        ("kind", "function, string, or auto"),
        ("numbering", "string, function, or none"),
        ("outlined", "boolean"),
        ("placement", "alignment, auto, or none"),
        ("scope", "\"column\" or \"parent\""),
        ("supplement", "content, function, none, or auto"),
    ],
    "figure.caption" => &[
        ("position", "alignment"),
        ("separator", "content or auto"),
    ],
    "footnote" => &[
        ("numbering", "string or function"),
    ],
    "footnote.entry" => &[
        ("clearance", "length"),
        ("gap", "length"),
        ("indent", "length"),
        ("separator", "content"),
    ],
    "grid" => &[
        ("align", "alignment, auto, array, or function"),
        ("column-gutter", "auto, relative length, fraction, integer, or array"),
        ("columns", "auto, relative length, fraction, integer, or array"),
        ("fill", "color, gradient, tiling, none, array, or function"),
        ("gutter", "auto, relative length, fraction, integer, or array"),
        ("inset", "relative length, dictionary, array, or function"),
        ("row-gutter", "auto, relative length, fraction, integer, or array"),
        ("rows", "auto, relative length, fraction, integer, or array"),
        ("stroke", "length, color, gradient, tiling, dictionary, stroke, none, array, or function"),
    ],
    "grid.cell" => &[
        ("align", "alignment or auto"),
        ("breakable", "boolean or auto"),
        ("colspan", "integer"),
        ("fill", "color, gradient, tiling, none, or auto"),
        ("inset", "relative length, dictionary, or auto"),
        ("rowspan", "integer"),
        ("stroke", "length, color, gradient, tiling, dictionary, stroke, or none"),
        ("x", "integer or auto"),
        ("y", "integer or auto"),
    ],
    "grid.footer" => &[
        ("repeat", "boolean"),
    ],
    "grid.header" => &[
        ("level", "integer"),
        ("repeat", "boolean"),
    ],
    "grid.hline" => &[
        ("end", "integer or none"),
        ("position", "alignment"),
        ("start", "integer"),
        ("stroke", "length, color, gradient, tiling, dictionary, stroke, or none"),
        ("y", "integer or auto"),
    ],
    "grid.vline" => &[
        ("end", "integer or none"),
        ("position", "alignment"),
        ("start", "integer"),
        ("stroke", "length, color, gradient, tiling, dictionary, stroke, or none"),
        ("x", "integer or auto"),
    ],
    "h" => &[
        ("weak", "boolean"),
    ],
    "heading" => &[
        ("bookmarked", "boolean or auto"),
        ("depth", "integer"),
        ("hanging-indent", "length or auto"),
        ("level", "integer or auto"),
        ("numbering", "string, function, or none"),
        ("offset", "integer"),
        ("outlined", "boolean"),
        ("supplement", "content, function, none, or auto"),
    ],
    "highlight" => &[
        ("bottom-edge", "\"baseline\", \"descender\", \"bounds\", or length"),
        ("extent", "length"),
        ("fill", "color, gradient, tiling, or none"),
        ("radius", "relative length or dictionary"),
        ("stroke", "length, color, gradient, tiling, dictionary, stroke, or none"),
        ("top-edge", "\"ascender\", \"cap-height\", \"x-height\", \"baseline\", \"bounds\", or length"),
    ],
    "image" => &[
        ("alt", "string or none"),
        ("fit", "\"cover\", \"contain\", or \"stretch\""),
        ("format", "\"png\", \"jpg\", \"gif\", \"webp\", dictionary, \"svg\", \"pdf\", or auto"),
        ("height", "auto, relative length, or fraction"),
        ("icc", "path, string, bytes, or auto"),
        ("page", "integer"),
        ("scaling", "\"smooth\", \"pixelated\", or auto"),
        ("width", "relative length or auto"),
    ],
    "line" => &[
        ("angle", "angle"),
        ("end", "array or none"),
        ("length", "relative length"),
        ("start", "array"),
        ("stroke", "length, color, gradient, tiling, dictionary, or stroke"),
    ],
    "linebreak" => &[
        ("justify", "boolean"),
    ],
    "list" => &[
        ("body-indent", "length"),
        ("indent", "length"),
        ("marker", "content, array, or function"),
        ("marker-align", "alignment"),
        ("spacing", "length or auto"),
        ("tight", "boolean"),
    ],
    "math.accent" => &[
        ("dotless", "boolean"),
        ("size", "relative length"),
    ],
    "math.attach" => &[
        ("b", "content or none"),
        ("bl", "content or none"),
        ("br", "content or none"),
        ("t", "content or none"),
        ("tl", "content or none"),
        ("tr", "content or none"),
    ],
    "math.cancel" => &[
        ("angle", "angle, function, or auto"),
        ("background", "boolean"),
        ("cross", "boolean"),
        ("inverted", "boolean"),
        ("length", "relative length"),
        ("stroke", "length, color, gradient, tiling, dictionary, or stroke"),
    ],
    "math.cases" => &[
        ("delim", "array, none, symbol, or string"),
        ("gap", "relative length"),
        ("reverse", "boolean"),
    ],
    "math.equation" => &[
        ("alt", "string or none"),
        ("block", "boolean"),
        ("number-align", "alignment"),
        ("numbering", "string, function, or none"),
        ("supplement", "content, function, none, or auto"),
    ],
    "math.frac" => &[
        ("style", "\"vertical\", \"skewed\", or \"horizontal\""),
    ],
    "math.limits" => &[
        ("inline", "boolean"),
    ],
    "math.lr" => &[
        ("size", "relative length"),
    ],
    "math.mat" => &[
        ("align", "alignment"),
        ("augment", "integer, dictionary, or none"),
        ("column-gap", "relative length"),
        ("delim", "array, none, symbol, or string"),
        ("gap", "relative length"),
        ("row-gap", "relative length"),
    ],
    "math.op" => &[
        ("limits", "boolean"),
    ],
    "math.stretch" => &[
        ("size", "relative length"),
    ],
    "math.vec" => &[
        ("align", "alignment"),
        ("delim", "array, none, symbol, or string"),
        ("gap", "relative length"),
    ],
    "move" => &[
        ("dx", "relative length"),
        ("dy", "relative length"),
    ],
    "outline" => &[
        ("depth", "integer or none"),
        ("indent", "relative length, function, or auto"),
        ("target", "label, function, location, or selector"),
        ("title", "content, none, or auto"),
    ],
    "outline.entry" => &[
        ("fill", "content or none"),
    ],
    "overline" => &[
        ("background", "boolean"),
        ("evade", "boolean"),
        ("extent", "length"),
        ("offset", "length or auto"),
        ("stroke", "length, color, gradient, tiling, dictionary, stroke, or auto"),
    ],
    "pad" => &[
        ("bottom", "relative length"),
        ("left", "relative length"),
        ("rest", "relative length"),
        ("right", "relative length"),
        ("top", "relative length"),
        ("x", "relative length"),
        ("y", "relative length"),
    ],
    "pagebreak" => &[
        ("to", "\"even\", \"odd\", or none"),
        ("weak", "boolean"),
    ],
    "par" => &[
        ("first-line-indent", "length or dictionary"),
        ("hanging-indent", "length"),
        ("justification-limits", "dictionary"),
        ("justify", "boolean"),
        ("leading", "length"),
        ("linebreaks", "\"simple\", \"optimized\", or auto"),
        ("spacing", "length"),
    ],
    "par.line" => &[
        ("number-align", "alignment or auto"),
        ("number-clearance", "length or auto"),
        ("number-margin", "alignment"),
        ("numbering", "string, function, or none"),
        ("numbering-scope", "\"document\" or \"page\""),
    ],
    "pdf.artifact" => &[
        ("kind", "\"header\", \"footer\", \"watermark\", \"page-number\", \"line-number\", \"redaction\", \"bates\", \"page\", \"pagination-other\", \"layout\", \"background\", or \"other\""),
    ],
    "place" => &[
        ("clearance", "length"),
        ("dx", "relative length"),
        ("dy", "relative length"),
        ("float", "boolean"),
        ("scope", "\"column\" or \"parent\""),
    ],
    "polygon" => &[
        ("fill", "color, gradient, tiling, or none"),
        ("fill-rule", "\"non-zero\" or \"even-odd\""),
        ("stroke", "length, color, gradient, tiling, dictionary, stroke, none, or auto"),
    ],
    "quote" => &[
        ("attribution", "content, label, or none"),
        ("block", "boolean"),
        ("quotes", "boolean or auto"),
    ],
    "raw" => &[
        ("align", "alignment"),
        ("block", "boolean"),
        ("lang", "string or none"),
        ("syntaxes", "path, string, bytes, or array"),
        ("tab-size", "integer"),
        ("theme", "path, string, bytes, none, or auto"),
    ],
    "rect" => &[
        ("fill", "color, gradient, tiling, or none"),
        ("height", "auto, relative length, or fraction"),
        ("inset", "relative length or dictionary"),
        ("outset", "relative length or dictionary"),
        ("radius", "relative length or dictionary"),
        ("stroke", "length, color, gradient, tiling, dictionary, stroke, none, or auto"),
        ("width", "relative length or auto"),
    ],
    "ref" => &[
        ("form", "\"normal\" or \"page\""),
        ("supplement", "content, function, none, or auto"),
    ],
    "repeat" => &[
        ("gap", "length"),
        ("justify", "boolean"),
    ],
    "rotate" => &[
        ("origin", "alignment"),
        ("reflow", "boolean"),
    ],
    "scale" => &[
        ("origin", "alignment"),
        ("reflow", "boolean"),
        ("x", "ratio, length, or auto"),
        ("y", "ratio, length, or auto"),
    ],
    "skew" => &[
        ("ax", "angle"),
        ("ay", "angle"),
        ("origin", "alignment"),
        ("reflow", "boolean"),
    ],
    "smallcaps" => &[
        ("all", "boolean"),
    ],
    "smartquote" => &[
        ("alternative", "boolean"),
        ("double", "boolean"),
        ("enabled", "boolean"),
        ("quotes", "dictionary, array, string, or auto"),
    ],
    "square" => &[
        ("fill", "color, gradient, tiling, or none"),
        ("height", "auto, relative length, or fraction"),
        ("inset", "relative length or dictionary"),
        ("outset", "relative length or dictionary"),
        ("radius", "relative length or dictionary"),
        ("size", "length or auto"),
        ("stroke", "length, color, gradient, tiling, dictionary, stroke, none, or auto"),
        ("width", "relative length or auto"),
    ],
    "stack" => &[
        ("dir", "direction"),
        ("spacing", "relative length, fraction, or none"),
    ],
    "strike" => &[
        ("background", "boolean"),
        ("extent", "length"),
        ("offset", "length or auto"),
        ("stroke", "length, color, gradient, tiling, dictionary, stroke, or auto"),
    ],
    "strong" => &[
        ("delta", "integer"),
    ],
    "sub" => &[
        ("baseline", "length or auto"),
        ("size", "length or auto"),
        ("typographic", "boolean"),
    ],
    "super" => &[
        ("baseline", "length or auto"),
        ("size", "length or auto"),
        ("typographic", "boolean"),
    ],
    "table" => &[
        ("align", "alignment, auto, array, or function"),
        ("column-gutter", "auto, relative length, fraction, integer, or array"),
        ("columns", "auto, relative length, fraction, integer, or array"),
        ("fill", "color, gradient, tiling, none, array, or function"),
        ("gutter", "auto, relative length, fraction, integer, or array"),
        ("inset", "relative length, dictionary, array, or function"),
        ("numbering", "string or none"),
        ("row-gutter", "auto, relative length, fraction, integer, or array"),
        ("rows", "auto, relative length, fraction, integer, or array"),
        ("stroke", "length, color, gradient, tiling, dictionary, stroke, none, array, or function"),
    ],
    "terms" => &[
        ("hanging-indent", "length"),
        ("indent", "length"),
        ("separator", "content"),
        ("spacing", "length or auto"),
        ("tight", "boolean"),
    ],
    "underline" => &[
        ("background", "boolean"),
        ("evade", "boolean"),
        ("extent", "length"),
        ("offset", "length or auto"),
        ("stroke", "length, color, gradient, tiling, dictionary, stroke, or auto"),
    ],
    "v" => &[
        ("weak", "boolean"),
    ],
    _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::span::Span;

    fn named(name: &str, value: Value) -> Args {
        Args::from_occurrences(
            Span::detached(),
            vec![ArgOccurrence {
                name: Some(name.into()),
                value,
                span: Span::detached(),
                value_span: Span::detached(),
            }],
        )
    }

    fn message(element: &str, args: &Args) -> String {
        check(element, args).unwrap_err()[0].message.to_string()
    }

    #[test]
    fn names_positionals_and_unlisted_elements() {
        assert_eq!(
            message("table", &named("nao", Value::None)),
            "unexpected argument: nao"
        );
        let positional = Args::positional(vec![Value::Int(1)]);
        assert_eq!(message("table", &positional), "unexpected argument");
        assert!(check("columns", &positional).is_ok());
        assert!(check("elemento-desconhecido", &positional).is_ok());
    }

    #[test]
    fn types_and_literals() {
        for (element, name, value) in [
            ("block", "breakable", Value::Bool(true)),
            ("block", "above", Value::Auto),
            ("image", "fit", Value::Str("cover".into())),
            ("image", "format", Value::Auto),
            ("outline", "title", Value::None),
            ("raw", "lang", Value::Str("rust".into())),
        ] {
            assert!(check(element, &named(name, value)).is_ok(), "{element}.{name}");
        }
        assert_eq!(
            message("image", &named("fit", Value::Str("x".into()))),
            "expected \"cover\", \"contain\", or \"stretch\""
        );
        assert_eq!(
            message("image", &named("fit", Value::Int(1))),
            "expected \"cover\", \"contain\", or \"stretch\", found integer"
        );
        assert_eq!(
            message("block", &named("above", Value::Str("x".into()))),
            "expected relative length, fraction, or auto, found string"
        );
    }

    #[test]
    fn every_listed_type_is_known() {
        let names = [
            "alignment",
            "angle",
            "array",
            "auto",
            "boolean",
            "bytes",
            "color",
            "content",
            "datetime",
            "dictionary",
            "direction",
            "fraction",
            "function",
            "gradient",
            "integer",
            "label",
            "length",
            "location",
            "none",
            "path",
            "ratio",
            "relative length",
            "selector",
            "string",
            "stroke",
            "symbol",
            "tiling",
        ];
        for element in ["block", "table", "image", "outline", "quote", "math.mat"] {
            for (_, types) in properties(element).unwrap() {
                for ty in type_names(types) {
                    assert!(ty.starts_with('"') || names.contains(&ty), "{ty}");
                }
            }
        }
    }
}
