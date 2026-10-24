//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/eval/bindings/value_methods/element_where.md
//! @prompt-hash 60466c91
//! @layer L1
//! @updated 2026-10-06

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
/// Fonte: field_id de typst-macros::elem; lotes auditados no pin 9f2b6e871.
/// As entradas históricas preservadas ainda têm auditoria própria pendente.
/// O nome serve ao diagnóstico; só o ponteiro tipado reconhece a função.
pub(crate) fn native_element_fields(
    function: &crate::entities::func::Func,
) -> Option<(&'static str, &'static [&'static str], bool)> {
    use crate::compiler::stdlib::*;
    if let crate::entities::func::FuncRepr::Native(native) = function.repr() {
        if std::ptr::fn_addr_eq(
            native.call,
            crate::compiler::eval::bindings::native_symbol_element as fn(_, _, _, _) -> _,
        ) {
            return Some(("symbol", &["text"], false));
        }
    }
    use crate::entities::func::FormatElementKind;
    if let crate::entities::func::FuncRepr::Format(kind, _) = function.repr() {
        return match kind {
            FormatElementKind::Svg => Some(("svg", &["pretty"], false)),
            FormatElementKind::Png => Some(("png", &["ppi"], false)),
            FormatElementKind::Pdf => {
                Some(("pdf", &["pages", "standard", "tagged", "pretty"], false))
            }
            FormatElementKind::Html => Some(("html", &["pretty"], false)),
            FormatElementKind::Bundle => Some(("bundle", &[], false)),
        };
    }
    if function.element_name() == Some("raw.line") {
        return Some(("raw.line", &["number", "count", "text", "body"], false));
    }
    if let crate::entities::func::FuncRepr::NativeWithEngine(native) = function.repr() {
        if std::ptr::fn_addr_eq(native.call, native_op as fn(_, _, _, _, _, _) -> _) {
            return Some(("op", &["text", "limits"], false));
        }
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
    element!(
        crate::compiler::eval::bindings::native_sequence,
        "sequence",
        false,
        ["children"]
    );
    element!(native_html_elem, "elem", false, ["tag", "attrs", "body"]);
    element!(native_html_frame, "frame", false, ["body"]);
    element!(native_pdf_artifact, "artifact", false, ["kind", "body"]);
    element!(
        native_pdf_attach,
        "attach",
        true,
        ["path", "data", "relationship", "mime-type", "description"]
    );
    element!(native_accent, "accent", false, ["base", "accent", "size", "dotless"]);
    element!(
        native_math_attach,
        "attach",
        false,
        ["base", "t", "b", "tl", "bl", "tr", "br"]
    );
    element!(native_math_binom, "binom", false, ["upper", "lower"]);
    element!(
        native_cancel,
        "cancel",
        false,
        ["body", "length", "inverted", "cross", "angle", "stroke", "background"]
    );
    element!(native_math_cases, "cases", false, ["delim", "reverse", "gap", "children"]);
    element!(native_math_class_for_metadata, "class", false, ["class", "body"]);
    element!(native_math_frac, "frac", false, ["num", "denom", "style"]);
    element!(native_math_limits, "limits", false, ["body", "inline"]);
    element!(native_math_lr, "lr", false, ["size", "body"]);
    element!(
        native_math_mat,
        "mat",
        false,
        ["delim", "align", "augment", "row-gap", "column-gap", "rows"]
    );
    element!(native_math_mid, "mid", false, ["body"]);
    element!(native_math_overbrace, "overbrace", false, ["body", "annotation"]);
    element!(native_math_overbracket, "overbracket", false, ["body", "annotation"]);
    element!(native_math_overline, "overline", false, ["body"]);
    element!(native_math_overparen, "overparen", false, ["body", "annotation"]);
    element!(native_math_overshell, "overshell", false, ["body", "annotation"]);
    element!(native_math_primes, "primes", false, ["count"]);
    element!(native_math_root, "root", false, ["index", "radicand"]);
    element!(native_math_scripts, "scripts", false, ["body"]);
    element!(native_math_stretch, "stretch", false, ["body", "size"]);
    element!(native_math_underbrace, "underbrace", false, ["body", "annotation"]);
    element!(native_math_underbracket, "underbracket", false, ["body", "annotation"]);
    element!(native_math_underline, "underline", false, ["body"]);
    element!(native_math_underparen, "underparen", false, ["body", "annotation"]);
    element!(native_math_undershell, "undershell", false, ["body", "annotation"]);
    element!(native_math_vec, "vec", false, ["delim", "align", "gap", "children"]);
    element!(native_title, "title", true, ["body"]);
    element!(
        native_document,
        "document",
        true,
        ["path", "format", "title", "author", "description", "keywords", "date", "body"]
    );
    element!(native_divider, "divider", false, []);
    element!(native_list_item, "item", false, ["body"]);
    element!(native_enum_item, "item", false, ["number", "body"]);
    element!(native_terms_item, "item", false, ["term", "description"]);
    element!(
        native_footnote_entry,
        "entry",
        true,
        ["note", "separator", "clearance", "gap", "indent"]
    );
    element!(native_flush, "flush", false, []);
    element!(native_curve_move, "move", false, ["start", "relative"]);
    element!(native_curve_line, "line", false, ["end", "relative"]);
    element!(native_curve_quad, "quad", false, ["control", "end", "relative"]);
    element!(
        native_curve_cubic,
        "cubic",
        false,
        ["control-start", "control-end", "end", "relative"]
    );
    element!(native_curve_close, "close", false, ["mode"]);
    element!(native_asset, "asset", true, ["path", "data"]);
    element!(
        native_math_equation,
        "equation",
        true,
        ["block", "numbering", "number-align", "supplement", "alt", "body"]
    );
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
        native_figure_caption,
        "caption",
        true,
        ["position", "separator", "body", "kind", "supplement", "numbering", "counter"]
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
        Selector::NativeElement(function) | Selector::Element { function, .. } => {
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
        Selector::Kind(_)
        | Selector::Label(_)
        | Selector::Location(_)
        | Selector::Regex(_) => {}
    }
    Ok(())
}

#[cfg(test)]
mod equation_where_tests {
    use super::*;
    #[test]
    fn equation_where_recognizes_native_identity_and_public_field_schema() {
        let Value::Module(math) = crate::compiler::stdlib::make_math_module() else {
            panic!("math module")
        };
        let Some(Value::Func(eq)) = math.scope().get("equation") else {
            panic!("missing equation")
        };
        let alias =
            crate::entities::func::Func::native("alias", eq.native_fn_addr().unwrap());
        for f in [eq.clone(), alias] {
            let (name, fields, locatable) =
                native_element_fields(&f).expect("equation is an element");
            assert_eq!(name, "equation");
            assert_eq!(
                fields,
                ["block", "numbering", "number-align", "supplement", "alt", "body"]
            );
            assert!(locatable);
            let mut args = Args::positional(vec![]);
            args.named.insert("block".into(), Value::Bool(true));
            assert!(matches!(
                eval_element_where(&f, &args).unwrap(),
                Selector::Element { .. }
            ));
            validate_element_locatability(
                &eval_element_where(&f, &args).unwrap(),
                args.span,
            )
            .unwrap();
            args.named.insert("size".into(), Value::Auto);
            assert_eq!(
                eval_element_where(&f, &args).unwrap_err()[0].message,
                "element `equation` does not have field `size`"
            );
        }
        let fake = crate::entities::func::Func::native(
            "math.equation",
            crate::compiler::stdlib::native_heading,
        );
        assert_eq!(native_element_fields(&fake).unwrap().0, "heading");
        let applied = eq.clone().with(Args::positional(vec![]));
        assert!(native_element_fields(&applied).is_none());
    }
}

#[cfg(test)]
mod caption_shape_tests {
    use super::*;
    #[test]
    fn caption_shape_native_metadata_uses_callback_not_name() {
        let real = crate::entities::func::Func::native(
            "alias",
            crate::compiler::stdlib::native_figure_caption,
        );
        let (name, fields, locatable) =
            native_element_fields(&real).expect("caption element metadata missing");
        assert_eq!(name, "caption");
        assert_eq!(
            fields,
            &[
                "position",
                "separator",
                "body",
                "kind",
                "supplement",
                "numbering",
                "counter"
            ]
        );
        assert!(locatable);
        let fake = crate::entities::func::Func::native(
            "caption",
            crate::compiler::stdlib::native_assert,
        );
        assert!(native_element_fields(&fake).is_none());
    }
}

#[cfg(test)]
mod native_catalogue_tests {
    use super::*;
    use crate::compiler::stdlib::*;
    use crate::entities::func::Func;

    fn check(function: Func, name: &str, fields: &[&str], locatable: bool) {
        let (actual_name, actual_fields, actual_locatable) =
            native_element_fields(&function).expect("existing native element missing");
        assert_eq!(actual_name, name);
        assert_eq!(actual_fields, fields);
        assert_eq!(actual_locatable, locatable);
        let mut args = Args::positional(vec![]);
        for field in fields {
            args.named.insert((*field).into(), Value::Auto);
        }
        args.named.insert(
            "label".into(),
            Value::Label(crate::entities::label::Label("audit".into())),
        );
        let selector = eval_element_where(&function, &args).unwrap();
        for selector in [selector, Selector::NativeElement(function.clone())] {
            let result = validate_element_locatability(&selector, args.span);
            if locatable {
                result.unwrap();
            } else {
                assert_eq!(
                    result.unwrap_err()[0].message,
                    format!("{name} is not locatable")
                );
            }
        }
        args.named.insert("__audit_missing".into(), Value::Auto);
        assert_eq!(
            eval_element_where(&function, &args).unwrap_err()[0].message,
            format!("element `{name}` does not have field `__audit_missing`")
        );
        assert!(native_element_fields(&function.with(Args::positional(vec![]))).is_none());
    }

    #[test]
    fn native_catalogue_title() {
        check(Func::native("alias", native_title), "title", &["body"], true);
    }

    #[test]
    fn native_catalogue_document() {
        check(
            Func::native("alias", native_document),
            "document",
            &[
                "path",
                "format",
                "title",
                "author",
                "description",
                "keywords",
                "date",
                "body",
            ],
            true,
        );
    }

    #[test]
    fn native_catalogue_divider() {
        check(Func::native("alias", native_divider), "divider", &[], false);
    }

    #[test]
    fn native_catalogue_list_item() {
        check(Func::native("alias", native_list_item), "item", &["body"], false);
    }

    #[test]
    fn native_catalogue_enum_item() {
        check(
            Func::native("alias", native_enum_item),
            "item",
            &["number", "body"],
            false,
        );
    }

    #[test]
    fn native_catalogue_terms_item() {
        check(
            Func::native("alias", native_terms_item),
            "item",
            &["term", "description"],
            false,
        );
    }

    #[test]
    fn native_catalogue_footnote_entry() {
        check(
            Func::native("alias", native_footnote_entry),
            "entry",
            &["note", "separator", "clearance", "gap", "indent"],
            true,
        );
    }

    #[test]
    fn native_catalogue_flush() {
        check(Func::native("alias", native_flush), "flush", &[], false);
    }

    #[test]
    fn native_catalogue_op() {
        check(
            Func::native_with_engine("alias", native_op),
            "op",
            &["text", "limits"],
            false,
        );
    }

    #[test]
    fn native_catalogue_curve_move() {
        check(
            Func::native("alias", native_curve_move),
            "move",
            &["start", "relative"],
            false,
        );
    }

    #[test]
    fn native_catalogue_curve_line() {
        check(
            Func::native("alias", native_curve_line),
            "line",
            &["end", "relative"],
            false,
        );
    }

    #[test]
    fn native_catalogue_curve_quad() {
        check(
            Func::native("alias", native_curve_quad),
            "quad",
            &["control", "end", "relative"],
            false,
        );
    }

    #[test]
    fn native_catalogue_curve_cubic() {
        check(
            Func::native("alias", native_curve_cubic),
            "cubic",
            &["control-start", "control-end", "end", "relative"],
            false,
        );
    }

    #[test]
    fn native_catalogue_curve_close() {
        check(Func::native("alias", native_curve_close), "close", &["mode"], false);
    }

    #[test]
    fn native_catalogue_asset() {
        check(Func::native("alias", native_asset), "asset", &["path", "data"], true);
    }

    #[test]
    fn native_catalogue_homonyms_and_page_controls() {
        for name in ["title", "op", "item", "document", "asset", "move"] {
            let fake = Func::native(name, native_assert);
            assert!(native_element_fields(&fake).is_none());
        }
        let page = Func::native_with_engine("alias", native_page);
        let (name, fields, locatable) = native_element_fields(&page).unwrap();
        assert_eq!(name, "page");
        assert!(fields.contains(&"width"));
        assert!(!locatable);
        assert!(native_element_fields(&page.with(Args::positional(vec![]))).is_none());
    }
}

#[cfg(test)]
mod native_math_catalogue_tests {
    use super::*;
    use crate::entities::func::Func;

    fn binding(name: &str) -> Func {
        let Value::Module(module) = crate::compiler::stdlib::make_math_module() else {
            panic!("math module")
        };
        let Some(Value::Func(function)) = module.scope().get(name) else {
            panic!("math function missing")
        };
        function.clone()
    }

    fn check(name: &'static str, fields: &[&str], excluded: &[&str]) {
        let function = binding(name);
        let alias = Func::native("alias", function.native_fn_addr().unwrap());
        for function in [function, alias] {
            let (actual_name, actual_fields, locatable) =
                native_element_fields(&function).expect("existing math element missing");
            assert_eq!(actual_name, name);
            assert_eq!(actual_fields, fields);
            assert!(!locatable);
            let mut args = Args::positional(vec![]);
            for field in fields {
                args.named.insert((*field).into(), Value::Auto);
            }
            args.named.insert(
                "label".into(),
                Value::Label(crate::entities::label::Label("audit".into())),
            );
            let selector = eval_element_where(&function, &args).unwrap();
            for selector in [selector, Selector::NativeElement(function.clone())] {
                assert_eq!(
                    validate_element_locatability(&selector, args.span).unwrap_err()[0]
                        .message,
                    format!("{name} is not locatable")
                );
            }
            for field in
                std::iter::once("__audit_missing").chain(excluded.iter().copied())
            {
                args.named.insert(field.into(), Value::Auto);
                assert_eq!(
                    eval_element_where(&function, &args).unwrap_err()[0].message,
                    format!("element `{name}` does not have field `{field}`")
                );
                args.named.shift_remove(field);
            }
            assert!(
                native_element_fields(&function.with(Args::positional(vec![]))).is_none()
            );
        }
        let fake = Func::native(name, crate::compiler::stdlib::native_assert);
        assert!(native_element_fields(&fake).is_none());
    }

    #[test]
    fn native_math_catalogue_accent() {
        check("accent", &["base", "accent", "size", "dotless"], &[]);
    }

    #[test]
    fn native_math_catalogue_attach() {
        check("attach", &["base", "t", "b", "tl", "bl", "tr", "br"], &[]);
    }

    #[test]
    fn native_math_catalogue_binom() {
        check("binom", &["upper", "lower"], &[]);
    }

    #[test]
    fn native_math_catalogue_cancel() {
        check(
            "cancel",
            &["body", "length", "inverted", "cross", "angle", "stroke", "background"],
            &[],
        );
    }

    #[test]
    fn native_math_catalogue_cases() {
        check("cases", &["delim", "reverse", "gap", "children"], &[]);
    }

    #[test]
    fn native_math_catalogue_class() {
        check("class", &["class", "body"], &[]);
    }

    #[test]
    fn native_math_catalogue_frac() {
        check(
            "frac",
            &["num", "denom", "style"],
            &["num-deparenthesized", "denom-deparenthesized"],
        );
    }

    #[test]
    fn native_math_catalogue_limits() {
        check("limits", &["body", "inline"], &[]);
    }

    #[test]
    fn native_math_catalogue_lr() {
        check("lr", &["size", "body"], &[]);
    }

    #[test]
    fn native_math_catalogue_mat() {
        check(
            "mat",
            &["delim", "align", "augment", "row-gap", "column-gap", "rows"],
            &["gap"],
        );
    }

    #[test]
    fn native_math_catalogue_mid() {
        check("mid", &["body"], &[]);
    }

    #[test]
    fn native_math_catalogue_overbrace() {
        check("overbrace", &["body", "annotation"], &[]);
    }

    #[test]
    fn native_math_catalogue_overbracket() {
        check("overbracket", &["body", "annotation"], &[]);
    }

    #[test]
    fn native_math_catalogue_overline() {
        check("overline", &["body"], &[]);
    }

    #[test]
    fn native_math_catalogue_overparen() {
        check("overparen", &["body", "annotation"], &[]);
    }

    #[test]
    fn native_math_catalogue_overshell() {
        check("overshell", &["body", "annotation"], &[]);
    }

    #[test]
    fn native_math_catalogue_primes() {
        check("primes", &["count"], &[]);
    }

    #[test]
    fn native_math_catalogue_root() {
        check("root", &["index", "radicand"], &[]);
    }

    #[test]
    fn native_math_catalogue_scripts() {
        check("scripts", &["body"], &[]);
    }

    #[test]
    fn native_math_catalogue_stretch() {
        check("stretch", &["body", "size"], &[]);
    }

    #[test]
    fn native_math_catalogue_underbrace() {
        check("underbrace", &["body", "annotation"], &[]);
    }

    #[test]
    fn native_math_catalogue_underbracket() {
        check("underbracket", &["body", "annotation"], &[]);
    }

    #[test]
    fn native_math_catalogue_underline() {
        check("underline", &["body"], &[]);
    }

    #[test]
    fn native_math_catalogue_underparen() {
        check("underparen", &["body", "annotation"], &[]);
    }

    #[test]
    fn native_math_catalogue_undershell() {
        check("undershell", &["body", "annotation"], &[]);
    }

    #[test]
    fn native_math_catalogue_vec() {
        check("vec", &["delim", "align", "gap", "children"], &[]);
    }

    #[test]
    fn native_math_catalogue_non_elements_and_global_decorations() {
        for name in ["sqrt", "abs", "norm", "round"] {
            assert!(native_element_fields(&binding(name)).is_none());
        }
        for function in [
            Func::native("math.underline", crate::compiler::stdlib::native_underline),
            Func::native("math.overline", crate::compiler::stdlib::native_overline),
        ] {
            let (_, fields, _) = native_element_fields(&function).unwrap();
            assert!(fields.contains(&"stroke"));
        }
    }
}

#[cfg(test)]
mod native_format_catalogue_tests {
    use super::*;
    use crate::entities::func::Func;

    fn binding(path: &str) -> Func {
        use crate::entities::compiler_features::{Feature, Features};
        let mut features = Features::empty();
        for feature in [Feature::Html, Feature::A11yExtras, Feature::Bundle] {
            features.enable(feature);
        }
        let scope = crate::compiler::stdlib::make_stdlib_with_features(
            &crate::contracts::world::SysInputs::default(),
            features,
        );
        let mut parts = path.strip_prefix("std.").unwrap().split('.');
        let mut value = scope.get(parts.next().unwrap()).unwrap().clone();
        for part in parts {
            let next = if let Value::Module(module) = &value {
                module.scope().get(part)
            } else if let Value::Func(function) = &value {
                function.namespace().unwrap().get(part)
            } else {
                panic!("not a namespace")
            };
            value = next.unwrap().clone();
        }
        let Value::Func(function) = value else { panic!("function binding") };
        function
    }

    fn check(
        path: &str,
        name: &'static str,
        fields: &[&str],
        excluded: &[&str],
        locatable: bool,
        format: bool,
    ) {
        let function = binding(path);
        let alias = function.clone();
        for function in [function, alias] {
            let (actual_name, actual_fields, actual_locatable) =
                native_element_fields(&function)
                    .expect("registered format element missing");
            assert_eq!(actual_name, name);
            assert_eq!(actual_fields, fields);
            assert_eq!(actual_locatable, locatable);
            let mut args = Args::positional(vec![]);
            for field in fields {
                args.named.insert((*field).into(), Value::Auto);
            }
            args.named.insert(
                "label".into(),
                Value::Label(crate::entities::label::Label("audit".into())),
            );
            let selector = eval_element_where(&function, &args).unwrap();
            let result = validate_element_locatability(&selector, args.span);
            if locatable {
                result.unwrap();
            } else {
                assert_eq!(
                    result.unwrap_err()[0].message,
                    format!("{name} is not locatable")
                );
            }
            for field in
                std::iter::once("__audit_missing").chain(excluded.iter().copied())
            {
                args.named.insert(field.into(), Value::Auto);
                assert_eq!(
                    eval_element_where(&function, &args).unwrap_err()[0].message,
                    format!("element `{name}` does not have field `{field}`")
                );
                args.named.shift_remove(field);
            }
            let unmarked = Func::native("alias", function.native_fn_addr().unwrap());
            assert_eq!(native_element_fields(&unmarked).is_none(), format);
            assert!(
                native_element_fields(&function.with(Args::positional(vec![]))).is_none()
            );
        }
        let fake = Func::native(name, crate::compiler::stdlib::native_assert);
        assert!(native_element_fields(&fake).is_none());
    }

    #[test]
    fn native_format_catalogue_format_bundle() {
        check("std.format.bundle", "bundle", &[], &[], false, true);
    }

    #[test]
    fn native_format_catalogue_format_html() {
        check("std.format.html", "html", &["pretty"], &[], false, true);
    }

    #[test]
    fn native_format_catalogue_format_html_elem() {
        check(
            "std.format.html.elem",
            "elem",
            &["tag", "attrs", "body"],
            &["css", "parent", "role"],
            false,
            false,
        );
    }

    #[test]
    fn native_format_catalogue_format_html_frame() {
        check("std.format.html.frame", "frame", &["body"], &[], false, false);
    }

    #[test]
    fn native_format_catalogue_format_pdf() {
        check(
            "std.format.pdf",
            "pdf",
            &["pages", "standard", "tagged", "pretty"],
            &[],
            false,
            true,
        );
    }

    #[test]
    fn native_format_catalogue_format_pdf_artifact() {
        check(
            "std.format.pdf.artifact",
            "artifact",
            &["kind", "body"],
            &[],
            false,
            false,
        );
    }

    #[test]
    fn native_format_catalogue_format_pdf_attach() {
        check(
            "std.format.pdf.attach",
            "attach",
            &["path", "data", "relationship", "mime-type", "description"],
            &[],
            true,
            false,
        );
    }

    #[test]
    fn native_format_catalogue_format_png() {
        check("std.format.png", "png", &["ppi"], &[], false, true);
    }

    #[test]
    fn native_format_catalogue_format_svg() {
        check("std.format.svg", "svg", &["pretty"], &[], false, true);
    }

    #[test]
    fn native_format_catalogue_html() {
        check("std.html", "html", &["pretty"], &[], false, true);
    }

    #[test]
    fn native_format_catalogue_html_elem() {
        check(
            "std.html.elem",
            "elem",
            &["tag", "attrs", "body"],
            &["css", "parent", "role"],
            false,
            false,
        );
    }

    #[test]
    fn native_format_catalogue_html_frame() {
        check("std.html.frame", "frame", &["body"], &[], false, false);
    }

    #[test]
    fn native_format_catalogue_pdf() {
        check(
            "std.pdf",
            "pdf",
            &["pages", "standard", "tagged", "pretty"],
            &[],
            false,
            true,
        );
    }

    #[test]
    fn native_format_catalogue_pdf_artifact() {
        check("std.pdf.artifact", "artifact", &["kind", "body"], &[], false, false);
    }

    #[test]
    fn native_format_catalogue_pdf_attach() {
        check(
            "std.pdf.attach",
            "attach",
            &["path", "data", "relationship", "mime-type", "description"],
            &[],
            true,
            false,
        );
    }
}

#[cfg(test)]
mod sequence_constructor_tests {
    use super::*;

    #[test]
    fn sequence_constructor_catalogue_identity_schema_and_locatability() {
        let Value::Func(f) = crate::compiler::eval::bindings::eval_content_method(
            &crate::entities::content::Content::Empty,
            "func",
            Args::positional(vec![]),
            Span::detached(),
        )
        .unwrap() else {
            panic!("function")
        };
        assert_eq!(
            native_element_fields(&f),
            Some(("sequence", &["children"][..], false))
        );
        let alias =
            crate::entities::func::Func::native("alias", f.native_fn_addr().unwrap());
        assert_eq!(native_element_fields(&alias), native_element_fields(&f));
        let fake =
            crate::entities::func::Func::native("sequence", |_, _, _, _| Ok(Value::None));
        assert!(native_element_fields(&fake).is_none());
        assert!(
            native_element_fields(&f.clone().with(Args::positional(vec![]))).is_none()
        );
        let errors =
            validate_element_locatability(&Selector::NativeElement(f), Span::detached())
                .unwrap_err();
        assert_eq!(errors[0].message, "sequence is not locatable");
    }
}

#[cfg(test)]
mod v16_c_structural_tests {
    use super::*;

    #[test]
    fn v16_c_locatability_visits_composite_operands_and_preserves_leaf_policy() {
        use crate::entities::{
            element_kind::ElementKind, func::Func, label::Label, locator::Locator,
            regex::Regex,
        };
        let span = Span::from_range(
            crate::entities::file_id::FileId::from_raw(
                std::num::NonZeroU16::new(1).unwrap(),
            ),
            4..8,
        );
        let good = Selector::Kind(ElementKind::Heading);
        for leaf in [
            good.clone(),
            Selector::Label(Label("x".into())),
            Selector::Location(Locator::new().next()),
            Selector::Regex(Regex::new("x").unwrap()),
            Selector::NativeElement(Func::native("uncatalogued", |_, _, _, _| {
                panic!("classifier executed callback")
            })),
        ] {
            validate_element_locatability(&leaf, span).unwrap();
        }
        let function = Func::native("alias", crate::compiler::stdlib::native_math_mat);
        for bad in [
            Selector::NativeElement(function.clone()),
            Selector::Element { function, fields: Default::default() },
        ] {
            let mut cases = vec![
                bad.clone(),
                Selector::Where {
                    base: Box::new(bad.clone()),
                    field: "x".into(),
                    value: Box::new(Value::None),
                },
            ];
            for (a, b) in [(good.clone(), bad.clone()), (bad.clone(), good.clone())] {
                cases.extend([
                    Selector::And(vec![a.clone(), b.clone()].into()),
                    Selector::Or(vec![a.clone(), b.clone()].into()),
                    Selector::Within {
                        base: Box::new(a.clone()),
                        ancestor: Box::new(b.clone()),
                    },
                    Selector::Before {
                        base: Box::new(a.clone()),
                        end: Box::new(b.clone()),
                        inclusive: false,
                    },
                    Selector::After {
                        base: Box::new(a),
                        start: Box::new(b),
                        inclusive: true,
                    },
                ]);
            }
            for selector in cases {
                let e = validate_element_locatability(&selector, span).unwrap_err();
                assert_eq!(e[0].message, "mat is not locatable");
                assert_eq!(e[0].span, span);
            }
        }
    }
}
