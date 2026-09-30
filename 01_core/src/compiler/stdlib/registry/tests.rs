//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/registry/tests.md
//! @prompt-hash 468d1dc6
//! @layer L1
//! @updated 2026-09-26

#![cfg(test)]

use super::make_stdlib_with_features;
use crate::contracts::world::SysInputs;
use crate::entities::compiler_features::{Feature, Features};
use crate::entities::scope::Scope;
use crate::entities::value::{Type, Value};

const ORDER: &[&str] = &[
    "format",
    "asset",
    "html",
    "type",
    "repr",
    "range",
    "rgb",
    "luma",
    "oklab",
    "oklch",
    "cmyk",
    "str",
    "int",
    "float",
    "path",
    "bool",
    "length",
    "ratio",
    "relative",
    "angle",
    "fraction",
    "array",
    "dictionary",
    "function",
    "content",
    "arguments",
    "module",
    "datetime",
    "bytes",
    "symbol",
    "alignment",
    "direction",
    "location",
    "decimal",
    "duration",
    "version",
    "heading",
    "title",
    "outline",
    "par",
    "lof",
    "lot",
    "strong",
    "emph",
    "raw",
    "underline",
    "strike",
    "overline",
    "smallcaps",
    "sub",
    "super",
    "highlight",
    "smartquote",
    "lorem",
    "regex",
    "figure",
    "image",
    "tiling",
    "read",
    "csv",
    "json",
    "yaml",
    "toml",
    "cbor",
    "xml",
    "plugin",
    "rect",
    "square",
    "ellipse",
    "circle",
    "line",
    "polygon",
    "curve",
    "grid",
    "move",
    "rotate",
    "scale",
    "skew",
    "align",
    "place",
    "assert",
    "panic",
    "numbering",
    "eval",
    "metadata",
    "state",
    "counter",
    "context",
    "state_update",
    "state_final",
    "state_at",
    "state_update_with",
    "state_display",
    "counter_display",
    "query",
    "selector",
    "here",
    "target",
    "locate",
    "counter_step",
    "counter_final",
    "counter_at",
    "upper",
    "lower",
    "replace",
    "terms",
    "divider",
    "quote",
    "document",
    "footnote",
    "accent",
    "cancel",
    "underover",
    "op",
    "bb",
    "bold",
    "cal",
    "display",
    "frak",
    "inline",
    "italic",
    "mono",
    "sans",
    "scr",
    "script",
    "serif",
    "sscript",
    "upright",
    "pad",
    "hide",
    "h",
    "v",
    "linebreak",
    "parbreak",
    "pagebreak",
    "page",
    "block",
    "box",
    "stack",
    "repeat",
    "columns",
    "colbreak",
    "measure",
    "layout",
    "stroke",
    "table",
    "table_cell",
    "table_header",
    "table_footer",
    "grid_cell",
    "grid_header",
    "grid_footer",
    "bibliography",
    "cite",
    "link",
    "label",
    "ref",
    "list",
    "enum",
    "sym",
    "calc",
    "emoji",
    "pdf",
    "color",
    "gradient",
    "math",
    "sys",
    "left",
    "center",
    "right",
    "start",
    "end",
    "top",
    "horizon",
    "bottom",
    "ltr",
    "rtl",
    "ttb",
    "btt",
];

fn profiles() -> impl Iterator<Item = Features> {
    (0..8).map(|mask| {
        let mut features = Features::empty();
        for (bit, feature) in
            [(1, Feature::Html), (2, Feature::A11yExtras), (4, Feature::Bundle)]
        {
            if mask & bit != 0 {
                features.enable(feature);
            }
        }
        features
    })
}

fn namespace<'a>(scope: &'a Scope, name: &str) -> &'a Scope {
    let Value::Func(func) = scope.get(name).expect(name) else {
        panic!("{name} must be callable");
    };
    func.namespace().expect("native namespace")
}

#[test]
fn ordered_bindings_types_and_absences_across_all_profiles() {
    for features in profiles() {
        let scope = make_stdlib_with_features(&SysInputs::default(), features);
        let expected: Vec<_> = ORDER
            .iter()
            .copied()
            .filter(|name| *name != "asset" || features.contains(Feature::Bundle))
            .collect();
        assert_eq!(scope.iter().map(|(name, _)| name).collect::<Vec<_>>(), expected);
        for (name, ty) in [
            ("type", Type::Type),
            ("str", Type::Str),
            ("int", Type::Int),
            ("float", Type::Float),
            ("path", Type::Path),
            ("state", Type::State),
            ("counter", Type::Counter),
            ("color", Type::Color),
            ("gradient", Type::Gradient),
        ] {
            assert_eq!(scope.get(name), Some(&Value::Type(ty)));
        }
        for name in [
            "hsl",
            "hsv",
            "linear_rgb",
            "linear-rgb",
            "global",
            "json_encode",
            "table_hline",
            "grid_vline",
        ] {
            assert!(scope.get(name).is_none(), "unexpected global {name}");
        }
    }
}

#[test]
fn format_projections_and_feature_gates_preserve_values() {
    for features in profiles() {
        let scope = make_stdlib_with_features(&SysInputs::default(), features);
        let Value::Module(format) = scope.get("format").unwrap() else {
            panic!("format module");
        };
        for parent in ["pdf", "html"] {
            let root = namespace(&scope, parent);
            let projection = namespace(format.scope(), parent);
            for (name, binding) in root.iter() {
                assert_eq!(
                    projection.get(name),
                    Some(binding.value()),
                    "{parent}.{name}"
                );
            }
        }
        for name in ["table-summary", "header-cell", "data-cell"] {
            assert_eq!(
                namespace(&scope, "pdf").get(name).is_some(),
                features.contains(Feature::A11yExtras)
            );
        }
        assert_eq!(scope.get("asset").is_some(), features.contains(Feature::Bundle));
        if features.contains(Feature::Bundle) {
            let bundle = namespace(format.scope(), "bundle");
            assert_eq!(scope.get("asset"), bundle.get("asset"));
        }
    }
}

#[test]
fn nested_native_names_and_codec_namespaces_are_preserved() {
    let scope = make_stdlib_with_features(&SysInputs::default(), Features::empty());
    for parent in ["grid", "table"] {
        for member in ["cell", "header", "footer", "hline", "vline"] {
            let Value::Func(func) = namespace(&scope, parent).get(member).unwrap() else {
                panic!("native member");
            };
            assert_eq!(func.name(), Some(member));
        }
    }
    for parent in ["json", "yaml", "toml"] {
        let ns = namespace(&scope, parent);
        assert_eq!(ns.iter().map(|(name, _)| name).collect::<Vec<_>>(), ["encode"]);
        let Value::Func(func) = ns.get("encode").unwrap() else {
            panic!("codec encoder");
        };
        assert_eq!(func.name(), Some("encode"));
    }
    let entry = namespace(namespace(&scope, "outline"), "entry");
    assert_eq!(
        entry.iter().map(|(name, _)| name).collect::<Vec<_>>(),
        ["body", "prefix", "indented", "inner", "page"]
    );
    assert_eq!(
        namespace(&scope, "place")
            .iter()
            .map(|(name, _)| name)
            .collect::<Vec<_>>(),
        ["flush"]
    );
}

#[test]
fn inputs_are_per_build_and_scope_mutation_does_not_leak() {
    let mut inputs = SysInputs::default();
    inputs.insert("answer".into(), "42".into());
    let mut first = make_stdlib_with_features(&inputs, Features::empty());
    let second = make_stdlib_with_features(&SysInputs::default(), Features::empty());
    let Value::Module(sys) = first.get("sys").unwrap() else {
        panic!("sys module");
    };
    let Value::Dict(values) = sys.scope().get("inputs").unwrap() else {
        panic!("input dictionary");
    };
    assert_eq!(values.get("answer"), Some(&Value::Str("42".into())));
    let Value::Module(sys) = second.get("sys").unwrap() else {
        panic!("sys module");
    };
    let Value::Dict(values) = sys.scope().get("inputs").unwrap() else {
        panic!("input dictionary");
    };
    assert!(values.is_empty());
    first.define("str", Value::Int(7));
    assert_eq!(second.get("str"), Some(&Value::Type(Type::Str)));
}
