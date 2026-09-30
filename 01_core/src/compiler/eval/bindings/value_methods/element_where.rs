//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/bindings/value_methods/element_where.md
//! @prompt-hash 35c00c2c
//! @layer L1
//! @updated 2026-09-30

use super::*;

/// Constrói um grupo público sem avaliar novamente receiver ou argumentos.
pub(crate) fn eval_element_where(
    function: &crate::entities::func::Func,
    args: &Args,
) -> SourceResult<Selector> {
    let Some((name, valid, _)) = native_element_fields(function) else {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "`where()` can only be called on element functions",
        )]);
    };
    // A view mantém a primeira posição e o último valor de cada chave.
    for field in args.named.keys() {
        if field.as_str() != "label" && !valid.contains(&field.as_str()) {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("element `{name}` does not have field `{field}`"),
            )]);
        }
    }
    if let Some(extra) = args.occurrence_sequence().iter().find(|arg| arg.name.is_none())
    {
        return Err(vec![SourceDiagnostic::error(extra.span, "unexpected argument")]);
    }
    Ok(Selector::Element {
        function: function.clone(),
        fields: args
            .named
            .iter()
            .map(|(name, value)| (name.clone(), value.clone()))
            .collect(),
    })
}

pub(crate) fn native_function_where(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _file: crate::entities::file_id::FileId,
) -> SourceResult<Value> {
    let (receiver, span, rest) =
        crate::compiler::eval::call_dispatch::take_p1339_self(args)?;
    let Value::Func(function) = receiver else {
        return Err(vec![SourceDiagnostic::error(
            span,
            format!("expected function, found {}", vanilla_type_name(&receiver)),
        )]);
    };
    eval_element_where(&function, &rest).map(Value::Selector)
}

/// Tabela estática dos campos de linguagem, excluindo #[internal]/#[external].
/// Fonte: typst-library e field_id de typst-macros::elem (a51e02804).
/// O nome serve ao diagnóstico; só o ponteiro tipado reconhece a função.
pub(crate) fn native_element_fields(
    function: &crate::entities::func::Func,
) -> Option<(&'static str, &'static [&'static str], bool)> {
    use crate::compiler::stdlib::*;
    if function.element_name() == Some("raw.line") {
        return Some(("raw.line", &["number", "count", "text", "body"], false));
    }
    if let crate::entities::func::FuncRepr::NativeWithEngine(native) = function.repr() {
        if std::ptr::fn_addr_eq(native.call, native_page as fn(_, _, _, _, _, _) -> _) {
            return Some((
                "page",
                &[
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
                ],
                false,
            ));
        }
    }
    let addr = function.native_fn_addr()?;
    macro_rules! element {
        ($native:path, $name:literal, $locatable:literal, [$($field:literal),*]) => {
            if std::ptr::fn_addr_eq(addr, $native as fn(_, _, _, _) -> _) {
                return Some(($name, &[$($field),*], $locatable));
            }
        };
    }
    element!(native_strong, "strong", true, ["delta", "body"]);
    element!(native_emph, "emph", true, ["body"]);
    element!(
        native_heading,
        "heading",
        true,
        [
            "level",
            "depth",
            "offset",
            "numbering",
            "supplement",
            "outlined",
            "bookmarked",
            "hanging-indent",
            "body"
        ]
    );
    element!(
        native_figure,
        "figure",
        true,
        [
            "body",
            "alt",
            "placement",
            "scope",
            "caption",
            "kind",
            "supplement",
            "numbering",
            "gap",
            "outlined",
            "counter"
        ]
    );
    element!(
        native_text,
        "text",
        false,
        [
            "font",
            "fallback",
            "style",
            "weight",
            "stretch",
            "size",
            "fill",
            "stroke",
            "tracking",
            "spacing",
            "cjk-latin-spacing",
            "baseline",
            "overhang",
            "top-edge",
            "bottom-edge",
            "lang",
            "region",
            "script",
            "dir",
            "hyphenate",
            "costs",
            "kerning",
            "alternates",
            "stylistic-set",
            "ligatures",
            "discretionary-ligatures",
            "historical-ligatures",
            "number-type",
            "number-width",
            "slashed-zero",
            "fractions",
            "features",
            "variations",
            "text"
        ]
    );
    element!(
        native_raw,
        "raw",
        true,
        ["text", "block", "lang", "align", "syntaxes", "theme", "tab-size", "lines"]
    );
    element!(
        native_table,
        "table",
        true,
        [
            "columns",
            "rows",
            "column-gutter",
            "row-gutter",
            "inset",
            "align",
            "fill",
            "stroke",
            "children"
        ]
    );
    element!(
        native_grid,
        "grid",
        false,
        [
            "columns",
            "rows",
            "column-gutter",
            "row-gutter",
            "inset",
            "align",
            "fill",
            "stroke",
            "children"
        ]
    );
    element!(native_link, "link", true, ["dest", "body"]);
    element!(native_quote, "quote", true, ["block", "quotes", "attribution", "body"]);
    element!(native_footnote, "footnote", true, ["numbering", "body"]);
    element!(
        native_list,
        "list",
        true,
        [
            "tight",
            "marker",
            "indent",
            "body-indent",
            "spacing",
            "marker-align",
            "children"
        ]
    );
    element!(
        native_enum,
        "enum",
        true,
        [
            "tight",
            "numbering",
            "start",
            "full",
            "reversed",
            "indent",
            "body-indent",
            "spacing",
            "number-align",
            "children"
        ]
    );
    element!(
        native_terms,
        "terms",
        true,
        ["tight", "separator", "indent", "hanging-indent", "spacing", "children"]
    );
    element!(native_outline, "outline", true, ["title", "target", "depth", "indent"]);
    element!(native_outline_entry, "entry", true, ["level", "element", "fill"]);
    element!(native_metadata, "metadata", true, ["value"]);
    element!(native_cite, "cite", true, ["key", "supplement", "form", "style"]);
    element!(
        native_bibliography,
        "bibliography",
        true,
        ["sources", "title", "full", "style", "target", "group"]
    );
    element!(
        native_ref,
        "ref",
        true,
        ["target", "supplement", "form", "citation", "element"]
    );
    element!(
        native_image,
        "image",
        true,
        ["source", "format", "width", "height", "alt", "page", "fit", "scaling", "icc"]
    );
    element!(native_align, "align", false, ["alignment", "body"]);
    element!(
        native_block,
        "block",
        false,
        [
            "width",
            "height",
            "breakable",
            "fill",
            "stroke",
            "radius",
            "inset",
            "outset",
            "above",
            "below",
            "clip",
            "sticky",
            "body"
        ]
    );
    element!(
        native_box,
        "box",
        false,
        [
            "width", "height", "baseline", "fill", "stroke", "radius", "inset", "outset",
            "clip", "body"
        ]
    );
    element!(native_columns, "columns", false, ["count", "gutter", "balanced", "body"]);
    element!(native_pad, "pad", false, ["left", "top", "right", "bottom", "body"]);
    element!(
        native_place,
        "place",
        false,
        ["alignment", "scope", "float", "clearance", "dx", "dy", "body"]
    );
    element!(native_hide, "hide", false, ["body"]);
    element!(native_repeat, "repeat", false, ["body", "gap", "justify"]);
    element!(native_stack, "stack", false, ["dir", "spacing", "children"]);
    element!(native_h, "h", false, ["amount", "weak"]);
    element!(native_v, "v", false, ["amount", "weak"]);
    element!(native_pagebreak, "pagebreak", false, ["weak", "to"]);
    element!(native_colbreak, "colbreak", false, ["weak"]);
    element!(native_linebreak, "linebreak", false, ["justify"]);
    element!(
        native_par,
        "par",
        true,
        [
            "leading",
            "spacing",
            "justify",
            "justification-limits",
            "linebreaks",
            "first-line-indent",
            "hanging-indent",
            "body"
        ]
    );
    element!(native_parbreak, "parbreak", false, []);
    element!(
        native_par_line,
        "par.line",
        true,
        [
            "numbering",
            "number-align",
            "number-margin",
            "number-clearance",
            "numbering-scope"
        ]
    );
    element!(
        native_rect,
        "rect",
        false,
        ["width", "height", "fill", "stroke", "radius", "inset", "outset", "body"]
    );
    element!(
        native_square,
        "square",
        false,
        ["width", "height", "fill", "stroke", "radius", "inset", "outset", "body"]
    );
    element!(
        native_circle,
        "circle",
        false,
        ["width", "height", "fill", "stroke", "inset", "outset", "body"]
    );
    element!(
        native_ellipse,
        "ellipse",
        false,
        ["width", "height", "fill", "stroke", "inset", "outset", "body"]
    );
    element!(native_line, "line", false, ["start", "end", "length", "angle", "stroke"]);
    element!(
        native_polygon,
        "polygon",
        false,
        ["fill", "fill-rule", "stroke", "vertices"]
    );
    element!(native_curve, "curve", false, ["fill", "fill-rule", "stroke", "components"]);
    element!(native_move, "move", false, ["dx", "dy", "body"]);
    element!(native_scale, "scale", false, ["x", "y", "origin", "reflow", "body"]);
    element!(native_rotate, "rotate", false, ["angle", "origin", "reflow", "body"]);
    element!(native_skew, "skew", false, ["ax", "ay", "origin", "reflow", "body"]);
    element!(
        native_underline,
        "underline",
        true,
        ["stroke", "offset", "extent", "evade", "background", "body"]
    );
    element!(
        native_overline,
        "overline",
        true,
        ["stroke", "offset", "extent", "evade", "background", "body"]
    );
    element!(
        native_strike,
        "strike",
        true,
        ["stroke", "offset", "extent", "background", "body"]
    );
    element!(
        native_highlight,
        "highlight",
        true,
        ["fill", "stroke", "top-edge", "bottom-edge", "extent", "radius", "body"]
    );
    element!(native_smallcaps, "smallcaps", false, ["all", "body"]);
    element!(native_subscript, "sub", false, ["typographic", "baseline", "size", "body"]);
    element!(
        native_superscript,
        "super",
        false,
        ["typographic", "baseline", "size", "body"]
    );
    element!(
        native_smartquote,
        "smartquote",
        false,
        ["double", "enabled", "alternative", "quotes"]
    );
    element!(native_table_header, "header", false, ["repeat", "level", "children"]);
    element!(native_table_footer, "footer", false, ["repeat", "children"]);
    element!(
        native_table_hline,
        "hline",
        false,
        ["y", "start", "end", "stroke", "position"]
    );
    element!(
        native_table_vline,
        "vline",
        false,
        ["x", "start", "end", "stroke", "position"]
    );
    element!(
        native_table_cell,
        "cell",
        false,
        [
            "body",
            "x",
            "y",
            "colspan",
            "rowspan",
            "inset",
            "align",
            "fill",
            "stroke",
            "breakable"
        ]
    );
    element!(native_grid_header, "header", false, ["repeat", "level", "children"]);
    element!(native_grid_footer, "footer", false, ["repeat", "children"]);
    element!(
        native_grid_hline,
        "hline",
        false,
        ["y", "start", "end", "stroke", "position"]
    );
    element!(
        native_grid_vline,
        "vline",
        false,
        ["x", "start", "end", "stroke", "position"]
    );
    element!(
        native_grid_cell,
        "cell",
        false,
        [
            "body",
            "x",
            "y",
            "colspan",
            "rowspan",
            "inset",
            "align",
            "fill",
            "stroke",
            "breakable"
        ]
    );
    None
}

/// Valida somente as folhas novas; ramos antigos não ganham regras novas.
pub(crate) fn validate_element_locatability(
    selector: &Selector,
    span: Span,
) -> SourceResult<()> {
    match selector {
        Selector::Element { function, .. } => {
            if let Some((name, _, false)) = native_element_fields(function) {
                return Err(vec![SourceDiagnostic::error(
                    span,
                    format!("{name} is not locatable"),
                )]);
            }
        }
        Selector::And(children) | Selector::Or(children) => {
            for child in children {
                validate_element_locatability(child, span)?;
            }
        }
        Selector::Where { base, .. } => validate_element_locatability(base, span)?,
        Selector::Within { base, ancestor } => {
            validate_element_locatability(base, span)?;
            validate_element_locatability(ancestor, span)?;
        }
        Selector::Before { base, end, .. } => {
            validate_element_locatability(base, span)?;
            validate_element_locatability(end, span)?;
        }
        Selector::After { base, start, .. } => {
            validate_element_locatability(base, span)?;
            validate_element_locatability(start, span)?;
        }
        _ => {}
    }
    Ok(())
}
