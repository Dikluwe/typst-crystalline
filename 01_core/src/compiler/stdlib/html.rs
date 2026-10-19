//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/html.md
//! @prompt-hash e85e6756
//! @layer L1

use std::sync::Arc;

use ecow::EcoString;

use crate::compiler::eval::operators::error_formatting::vanilla_type_name;
use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::dir::Dir;
use crate::entities::file_id::FileId;
use crate::entities::func::Func;
#[cfg(test)]
use crate::entities::html::{Feature, Features};
use crate::entities::html::{HtmlAttrs, HtmlBody, HtmlElem, HtmlFrameElem};
use crate::entities::scope::Scope;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;

#[path = "html/html_constructor_surface_tests.rs"]
mod html_constructor_surface_tests;

#[path = "html/html_frame_tests.rs"]
mod html_frame_tests;

mod attr_cast;
mod attr_specs;
mod attr_values;
mod elem;
mod frame;
mod global_attrs;
mod namespace;
mod tag_attrs;
mod typed_element;
mod typed_tags;

pub use self::elem::native_html_elem;
pub(in crate::compiler) use self::frame::native_html_frame;
pub use self::namespace::make_html_module;
use self::{
    attr_cast::*, attr_specs::*, attr_values::*, global_attrs::*, tag_attrs::*,
    typed_element::*, typed_tags::*,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn atomized_html_keeps_typed_dispatch_and_name_validation() {
        let Value::Func(module) = namespace::make_html_module() else {
            panic!("html must be a function")
        };
        assert!(module.namespace().unwrap().get("div").is_some());
        assert!(typed_tags::TYPED_TAGS.iter().any(|(tag, _)| *tag == "track"));
        assert!(global_attrs::GLOBAL_ATTRS.iter().any(|spec| spec.name == "lang"));
        assert!(tag_attrs::INPUT_ATTRS.iter().any(|spec| spec.name == "type"));
        assert!(elem::validate_name(&"bad tag".into(), "tag", Span::detached()).is_err());
    }

    const P1168_TAGS: &[&str] =
        &["div", "span", "p", "h1", "h2", "h3", "h4", "h5", "h6", "strong", "em", "ul"];

    #[test]
    fn p1168_html_module_expoe_exatamente_o_primeiro_lote_tipado() {
        let module = make_html_module();
        let Value::Func(module) = &module else { panic!("html must be a function") };
        for tag in P1168_TAGS {
            assert!(
                module.namespace().unwrap().get(tag).is_some(),
                "binding ausente: html.{tag}"
            );
        }
        assert!(
            module.namespace().unwrap().get("frame").is_some(),
            "html.frame pertence ao P1424"
        );
    }

    #[test]
    fn p1169_1_html_module_expoe_ol_e_li() {
        let module = make_html_module();
        let Value::Func(module) = &module else { panic!("html must be a function") };
        assert!(
            module.namespace().unwrap().get("ol").is_some(),
            "binding ausente: html.ol"
        );
        assert!(
            module.namespace().unwrap().get("li").is_some(),
            "binding ausente: html.li"
        );
    }

    #[test]
    fn p1170_1_html_module_expoe_a() {
        let module = make_html_module();
        let Value::Func(module) = &module else { panic!("html must be a function") };
        assert!(
            module.namespace().unwrap().get("a").is_some(),
            "binding ausente: html.a"
        );
    }

    #[test]
    fn p1171_1_html_module_expoe_br_sem_body() {
        let module = make_html_module();
        let Value::Func(module) = &module else { panic!("html must be a function") };
        assert!(
            module.namespace().unwrap().get("br").is_some(),
            "binding ausente: html.br"
        );

        let br = elem(
            native_typed_html_void(
                "br",
                &[],
                &typed_args(&[("id", Value::Str("b".into()))], None),
            )
            .unwrap(),
        );
        assert_eq!(
            br.attrs.as_ref().unwrap().get("id").map(|value| value.as_str()),
            Some("b")
        );
        assert!(matches!(br.body, HtmlBody::Unset));
        assert!(native_typed_html_void(
            "br",
            &[],
            &typed_args(&[], Some(Content::text("X"))),
        )
        .is_err());
    }

    #[test]
    fn p1173_1_html_module_expoe_lote_global_only_2() {
        let module = make_html_module();
        let Value::Func(module) = &module else { panic!("html must be a function") };
        for tag in [
            "abbr", "address", "article", "aside", "b", "bdi", "bdo", "cite", "code",
            "dfn", "i", "kbd",
        ] {
            assert!(
                module.namespace().unwrap().get(tag).is_some(),
                "binding ausente: html.{tag}"
            );
        }

        let abbr = elem(
            native_typed_html(
                "abbr",
                &[],
                &typed_args(
                    &[
                        ("id", Value::Str("sigla".into())),
                        ("class", Value::Str("termo".into())),
                        ("hidden", Value::Bool(true)),
                    ],
                    Some(Content::text("A")),
                ),
            )
            .unwrap(),
        );
        let attrs = abbr.attrs.as_ref().unwrap();
        assert_eq!(attrs.get("id").map(|value| value.as_str()), Some("sigla"));
        assert_eq!(attrs.get("class").map(|value| value.as_str()), Some("termo"));
        assert_eq!(attrs.get("hidden").map(|value| value.as_str()), Some(""));
        assert!(matches!(abbr.body, HtmlBody::Content(_)));
        assert!(native_typed_html(
            "abbr",
            &[],
            &typed_args(&[("href", Value::Str("/fora".into()))], None),
        )
        .is_err());
    }

    #[test]
    fn p1174_1_html_module_expoe_lote_global_only_3() {
        let module = make_html_module();
        let Value::Func(module) = &module else { panic!("html must be a function") };
        for tag in [
            "dd",
            "dl",
            "dt",
            "figcaption",
            "figure",
            "footer",
            "header",
            "hgroup",
            "legend",
            "main",
            "mark",
            "menu",
        ] {
            assert!(
                module.namespace().unwrap().get(tag).is_some(),
                "binding ausente: html.{tag}"
            );
        }

        let mark = elem(
            native_typed_html(
                "mark",
                &[],
                &typed_args(
                    &[
                        ("id", Value::Str("m".into())),
                        ("class", Value::Str("hi".into())),
                        ("hidden", Value::Bool(true)),
                    ],
                    Some(Content::text("M")),
                ),
            )
            .unwrap(),
        );
        let attrs = mark.attrs.as_ref().unwrap();
        assert_eq!(attrs.get("id").map(|value| value.as_str()), Some("m"));
        assert_eq!(attrs.get("class").map(|value| value.as_str()), Some("hi"));
        assert_eq!(attrs.get("hidden").map(|value| value.as_str()), Some(""));
        assert!(matches!(mark.body, HtmlBody::Content(_)));
        assert!(native_typed_html(
            "mark",
            &[],
            &typed_args(&[("href", Value::Str("/fora".into()))], None),
        )
        .is_err());
    }

    #[test]
    fn p1175_1_html_module_expoe_lote_global_only_4() {
        let module = make_html_module();
        let Value::Func(module) = &module else { panic!("html must be a function") };
        for tag in [
            "nav", "picture", "pre", "s", "samp", "search", "section", "small", "sub",
            "sup", "u", "var",
        ] {
            assert!(
                module.namespace().unwrap().get(tag).is_some(),
                "binding ausente: html.{tag}"
            );
        }

        let picture = elem(
            native_typed_html(
                "picture",
                &[],
                &typed_args(
                    &[("id", Value::Str("pic".into())), ("hidden", Value::Bool(true))],
                    Some(Content::text("P")),
                ),
            )
            .unwrap(),
        );
        assert_eq!(
            picture.attrs.as_ref().unwrap().get("id").map(|value| value.as_str()),
            Some("pic")
        );
        assert!(matches!(picture.body, HtmlBody::Content(_)));
        assert!(native_typed_html(
            "picture",
            &[],
            &typed_args(&[("href", Value::Str("/fora".into()))], None),
        )
        .is_err());
    }

    #[test]
    fn p1176_1_html_module_expoe_lote_residual_normal() {
        let module = make_html_module();
        let Value::Func(module) = &module else { panic!("html must be a function") };
        for tag in ["datalist", "noscript", "summary"] {
            assert!(
                module.namespace().unwrap().get(tag).is_some(),
                "binding ausente: html.{tag}"
            );
        }

        let summary = elem(
            native_typed_html(
                "summary",
                &[],
                &typed_args(
                    &[("id", Value::Str("s".into())), ("hidden", Value::Bool(true))],
                    Some(Content::text("S")),
                ),
            )
            .unwrap(),
        );
        assert_eq!(
            summary.attrs.as_ref().unwrap().get("id").map(|value| value.as_str()),
            Some("s")
        );
        assert!(matches!(summary.body, HtmlBody::Content(_)));
        assert!(native_typed_html(
            "summary",
            &[],
            &typed_args(&[("href", Value::Str("/fora".into()))], None),
        )
        .is_err());
    }

    #[test]
    fn p1177_1_html_module_expoe_familia_ruby() {
        let module = make_html_module();
        let Value::Func(module) = &module else { panic!("html must be a function") };
        for tag in ["ruby", "rp", "rt"] {
            assert!(
                module.namespace().unwrap().get(tag).is_some(),
                "binding ausente: html.{tag}"
            );
        }

        let ruby = elem(
            native_typed_html(
                "ruby",
                &[],
                &typed_args(
                    &[("id", Value::Str("r".into())), ("hidden", Value::Bool(true))],
                    Some(Content::text("R")),
                ),
            )
            .unwrap(),
        );
        assert_eq!(
            ruby.attrs.as_ref().unwrap().get("id").map(|value| value.as_str()),
            Some("r")
        );
        assert!(matches!(ruby.body, HtmlBody::Content(_)));
        assert!(native_typed_html(
            "ruby",
            &[],
            &typed_args(&[("href", Value::Str("/fora".into()))], None),
        )
        .is_err());
    }

    #[test]
    fn p1178_1_html_module_expoe_familia_documento() {
        let module = make_html_module();
        let Value::Func(module) = &module else { panic!("html must be a function") };
        for tag in ["html", "head", "body", "title"] {
            assert!(
                module.namespace().unwrap().get(tag).is_some(),
                "binding ausente: html.{tag}"
            );
        }

        let title = elem(
            native_typed_html(
                "title",
                &[],
                &typed_args(
                    &[("id", Value::Str("t".into())), ("hidden", Value::Bool(true))],
                    Some(Content::text("T")),
                ),
            )
            .unwrap(),
        );
        assert_eq!(
            title.attrs.as_ref().unwrap().get("id").map(|value| value.as_str()),
            Some("t")
        );
        assert!(matches!(title.body, HtmlBody::Content(_)));
        assert!(native_typed_html(
            "title",
            &[],
            &typed_args(&[("href", Value::Str("/fora".into()))], None),
        )
        .is_err());
    }

    #[test]
    fn p1170_1_a_converte_oito_especificos_e_preserva_ordem_body() {
        let anchor = elem(
            native_typed_html(
                "a",
                A_ATTRS,
                &typed_args(
                    &[
                        ("id", Value::Str("k".into())),
                        ("download", Value::Str("d".into())),
                        ("href", Value::Str("/x".into())),
                        ("hreflang", Value::Str("pt".into())),
                        (
                            "ping",
                            Value::Array(vec![
                                Value::Str("/p1".into()),
                                Value::Str("/p2".into()),
                            ]),
                        ),
                        ("referrerpolicy", Value::None),
                        (
                            "rel",
                            Value::Array(vec![
                                Value::Str("noopener".into()),
                                Value::Str("noreferrer".into()),
                            ]),
                        ),
                        ("target", Value::Str("named-frame".into())),
                        ("type", Value::Str("text/plain".into())),
                    ],
                    Some(Content::text("X")),
                ),
            )
            .unwrap(),
        );
        assert_eq!(
            anchor
                .attrs
                .as_ref()
                .unwrap()
                .iter()
                .map(|(k, v)| (k.as_str(), v.as_str()))
                .collect::<Vec<_>>(),
            vec![
                ("id", "k"),
                ("download", "d"),
                ("href", "/x"),
                ("hreflang", "pt"),
                ("ping", "/p1 /p2"),
                ("referrerpolicy", ""),
                ("rel", "noopener noreferrer"),
                ("target", "named-frame"),
                ("type", "text/plain"),
            ]
        );
        assert_eq!(anchor.body.content(), Some(&Content::text("X")));
    }

    #[test]
    fn p1170_1_a_restringe_enums_listas_e_nao_vaza_especificos() {
        for value in REFERRER_POLICIES {
            assert!(native_typed_html(
                "a",
                A_ATTRS,
                &typed_args(&[("referrerpolicy", Value::Str((*value).into()))], None),
            )
            .is_ok());
        }
        for value in REL_TYPES {
            assert!(native_typed_html(
                "a",
                A_ATTRS,
                &typed_args(&[("rel", Value::Str((*value).into()))], None),
            )
            .is_ok());
        }
        for (name, value) in [
            ("download", Value::Bool(true)),
            ("ping", Value::Array(vec![Value::Int(1)])),
            ("referrerpolicy", Value::Str("x".into())),
            ("rel", Value::Str("x".into())),
            ("target", Value::Int(1)),
            ("value", Value::Int(1)),
        ] {
            assert!(native_typed_html("a", A_ATTRS, &typed_args(&[(name, value)], None))
                .is_err());
        }
        let target = native_typed_html(
            "a",
            A_ATTRS,
            &typed_args(&[("target", Value::Int(1))], None),
        )
        .unwrap_err();
        assert!(target[0].message.contains(
            "expected \"_blank\", \"_self\", \"_parent\", \"_top\", or string"
        ));
        assert!(native_typed_html(
            "ol",
            OL_ATTRS,
            &typed_args(&[("href", Value::Str("x".into()))], None),
        )
        .is_err());
    }

    #[test]
    fn p1169_1_casts_especificos_preservam_ordem_presence_int_e_body() {
        let ol = elem(
            native_typed_html(
                "ol",
                OL_ATTRS,
                &typed_args(
                    &[
                        ("id", Value::Str("o".into())),
                        ("start", Value::Int(-2)),
                        ("reversed", Value::Bool(true)),
                        ("type", Value::Str("A".into())),
                    ],
                    Some(Content::text("x")),
                ),
            )
            .unwrap(),
        );
        let attrs = ol.attrs.as_ref().unwrap();
        assert_eq!(
            attrs
                .iter()
                .map(|(k, v)| (k.as_str(), v.as_str()))
                .collect::<Vec<_>>(),
            vec![("id", "o"), ("start", "-2"), ("reversed", ""), ("type", "A")]
        );
        assert_eq!(ol.body.content(), Some(&Content::text("x")));

        let li = elem(
            native_typed_html(
                "li",
                LI_ATTRS,
                &typed_args(&[("value", Value::Int(-3))], None),
            )
            .unwrap(),
        );
        assert_eq!(
            li.attrs.as_ref().unwrap().get("value").map(|v| v.as_str()),
            Some("-3")
        );
        assert!(matches!(li.body, HtmlBody::None));
    }

    #[test]
    fn p1169_1_type_aceita_cinco_tokens_e_especificos_nao_vazam() {
        for value in OL_TYPES {
            assert!(native_typed_html(
                "ol",
                OL_ATTRS,
                &typed_args(&[("type", Value::Str((*value).into()))], None),
            )
            .is_ok());
        }
        for (tag, specs, name, value) in [
            ("ol", OL_ATTRS, "type", Value::Str("x".into())),
            ("ol", OL_ATTRS, "reversed", Value::None),
            ("ol", OL_ATTRS, "start", Value::Float(1.5)),
            ("ol", OL_ATTRS, "value", Value::Int(1)),
            ("li", LI_ATTRS, "start", Value::Int(1)),
        ] {
            assert!(native_typed_html(tag, specs, &typed_args(&[(name, value)], None))
                .is_err());
        }
        let error = native_typed_html(
            "ol",
            OL_ATTRS,
            &typed_args(&[("type", Value::Str("x".into()))], None),
        )
        .unwrap_err();
        assert!(error[0]
            .message
            .contains("expected \"1\", \"a\", \"A\", \"i\", or \"I\""));
    }

    fn typed_args(named: &[(&str, Value)], body: Option<Content>) -> Args {
        let mut args = Args::positional(body.into_iter().map(Value::Content).collect());
        args.named = named.iter().map(|(k, v)| ((*k).into(), v.clone())).collect();
        args
    }

    fn elem(value: Value) -> Arc<HtmlElem> {
        match value {
            Value::Content(Content::HtmlElem(elem)) => elem,
            other => panic!("esperava HtmlElem, recebeu {other:?}"),
        }
    }

    #[test]
    fn p1168_tabela_tem_76_globais_unicos() {
        assert_eq!(GLOBAL_ATTRS.len(), 76);
        let unique: std::collections::BTreeSet<_> =
            GLOBAL_ATTRS.iter().map(|a| a.name).collect();
        assert_eq!(unique.len(), 76);
        assert_eq!(ROLES.len(), 80);
    }

    #[test]
    fn p1168_casts_globais_preservam_ordem_omissao_e_body() {
        let args = typed_args(
            &[
                ("id", Value::Str("d".into())),
                (
                    "class",
                    Value::Array(vec![Value::Str("a".into()), Value::Str("b".into())]),
                ),
                ("hidden", Value::Bool(true)),
                ("autofocus", Value::Bool(false)),
                ("tabindex", Value::Int(-2)),
                ("aria-valuenow", Value::Float(1.5)),
                ("aria-checked", Value::Str("mixed".into())),
                ("aria-expanded", Value::None),
                ("dir", Value::Dir(Dir::RTL)),
                ("lang", Value::None),
                ("popover", Value::Auto),
                ("role", Value::Str("window".into())),
                ("translate", Value::Bool(false)),
            ],
            Some(Content::text("x")),
        );
        let elem = elem(native_typed_html("div", &[], &args).unwrap());
        assert_eq!(elem.tag, "div");
        assert_eq!(elem.body.content(), Some(&Content::text("x")));
        let attrs = elem.attrs.as_ref().unwrap();
        assert_eq!(
            attrs.keys().map(|s| s.as_str()).collect::<Vec<_>>(),
            vec![
                "id",
                "class",
                "hidden",
                "tabindex",
                "aria-valuenow",
                "aria-checked",
                "aria-expanded",
                "dir",
                "lang",
                "popover",
                "role",
                "translate"
            ]
        );
        assert_eq!(attrs.get("class").map(|s| s.as_str()), Some("a b"));
        assert_eq!(attrs.get("hidden").map(|s| s.as_str()), Some(""));
        assert!(!attrs.contains_key("autofocus"));
        assert_eq!(attrs.get("aria-expanded").map(|s| s.as_str()), Some("undefined"));
        assert_eq!(attrs.get("translate").map(|s| s.as_str()), Some("no"));
    }

    #[test]
    fn p1168_casts_rejeitam_named_tipo_enum_e_data_attr_invalidos() {
        for args in [
            typed_args(&[("unknown", Value::Str("x".into()))], None),
            typed_args(&[("id", Value::Int(1))], None),
            typed_args(&[("role", Value::Str("bogus".into()))], None),
            typed_args(&[("data-x", Value::Str("x".into()))], None),
            typed_args(&[("accesskey", Value::Str("ab".into()))], None),
        ] {
            assert!(native_typed_html("div", &[], &args).is_err());
        }
    }

    #[test]
    fn p1293_c_expoe_sete_bindings_com_body_normal_e_void() {
        let module = make_html_module();
        let Value::Func(module) = &module else { panic!("html must be a function") };
        for tag in ["button", "col", "iframe", "select", "template", "video", "wbr"] {
            assert!(
                module.namespace().unwrap().get(tag).is_some(),
                "binding ausente: html.{tag}"
            );
        }

        for (tag, specs) in [
            ("button", BUTTON_ATTRS),
            ("iframe", IFRAME_ATTRS),
            ("select", SELECT_ATTRS),
            ("template", TEMPLATE_ATTRS),
            ("video", VIDEO_ATTRS),
        ] {
            let omitted =
                elem(native_typed_html(tag, specs, &typed_args(&[], None)).unwrap());
            assert!(matches!(omitted.body, HtmlBody::None), "{tag}");
            let present = elem(
                native_typed_html(tag, specs, &typed_args(&[], Some(Content::text("X"))))
                    .unwrap(),
            );
            assert_eq!(present.body.content(), Some(&Content::text("X")), "{tag}");
        }

        for (tag, specs) in [("col", COL_ATTRS), ("wbr", WBR_ATTRS)] {
            let void =
                elem(native_typed_html_void(tag, specs, &typed_args(&[], None)).unwrap());
            assert!(matches!(void.body, HtmlBody::Unset), "{tag}");
            assert!(
                native_typed_html_void(
                    tag,
                    specs,
                    &typed_args(&[], Some(Content::text("X"))),
                )
                .is_err(),
                "{tag} aceitou body"
            );
        }
    }

    #[test]
    fn p1293_c_tabelas_tem_48_especificos_sem_duplicar_globais() {
        let tables = [
            BUTTON_ATTRS,
            COL_ATTRS,
            IFRAME_ATTRS,
            SELECT_ATTRS,
            TEMPLATE_ATTRS,
            VIDEO_ATTRS,
            WBR_ATTRS,
        ];
        assert_eq!(tables.iter().map(|table| table.len()).sum::<usize>(), 48);
        assert_eq!(
            tables.iter().map(|table| table.len()).collect::<Vec<_>>(),
            vec![14, 1, 10, 7, 5, 11, 0]
        );
        for table in tables {
            let unique: std::collections::BTreeSet<_> =
                table.iter().map(|spec| spec.name).collect();
            assert_eq!(unique.len(), table.len());
            assert!(table.iter().all(|specific| GLOBAL_ATTRS
                .iter()
                .all(|global| global.name != specific.name)));
        }
    }

    #[test]
    fn p1293_c_casts_especificos_preservam_ordem_presence_e_listas() {
        let button = elem(
            native_typed_html(
                "button",
                BUTTON_ATTRS,
                &typed_args(
                    &[
                        ("id", Value::Str("b".into())),
                        ("command", Value::Str("request-close".into())),
                        ("commandfor", Value::Str("dialog".into())),
                        ("disabled", Value::Bool(true)),
                        ("formnovalidate", Value::Bool(false)),
                        ("formenctype", Value::Str("multipart/form-data".into())),
                        ("formmethod", Value::Str("POST".into())),
                        ("formtarget", Value::Str("named".into())),
                        ("popovertargetaction", Value::Str("show".into())),
                        ("type", Value::Str("submit".into())),
                    ],
                    None,
                ),
            )
            .unwrap(),
        );
        assert_eq!(
            button
                .attrs
                .as_ref()
                .unwrap()
                .iter()
                .map(|(name, value)| (name.as_str(), value.as_str()))
                .collect::<Vec<_>>(),
            vec![
                ("id", "b"),
                ("command", "request-close"),
                ("commandfor", "dialog"),
                ("disabled", ""),
                ("formenctype", "multipart/form-data"),
                ("formmethod", "POST"),
                ("formtarget", "named"),
                ("popovertargetaction", "show"),
                ("type", "submit"),
            ]
        );

        let iframe = elem(
            native_typed_html(
                "iframe",
                IFRAME_ATTRS,
                &typed_args(
                    &[
                        ("height", Value::Int(0)),
                        (
                            "sandbox",
                            Value::Array(vec![
                                Value::Str("allow-scripts".into()),
                                Value::Str("allow-forms".into()),
                            ]),
                        ),
                        ("referrerpolicy", Value::None),
                        ("allowfullscreen", Value::Bool(true)),
                    ],
                    None,
                ),
            )
            .unwrap(),
        );
        let attrs = iframe.attrs.as_ref().unwrap();
        assert_eq!(attrs.get("height").map(EcoString::as_str), Some("0"));
        assert_eq!(
            attrs.get("sandbox").map(EcoString::as_str),
            Some("allow-scripts allow-forms")
        );
        assert_eq!(attrs.get("referrerpolicy").map(EcoString::as_str), Some(""));
        assert_eq!(attrs.get("allowfullscreen").map(EcoString::as_str), Some(""));

        let select = elem(
            native_typed_html(
                "select",
                SELECT_ATTRS,
                &typed_args(
                    &[
                        (
                            "autocomplete",
                            Value::Array(vec![
                                Value::Str("shipping".into()),
                                Value::Str("email".into()),
                            ]),
                        ),
                        ("size", Value::Int(1)),
                    ],
                    None,
                ),
            )
            .unwrap(),
        );
        assert_eq!(
            select
                .attrs
                .as_ref()
                .unwrap()
                .get("autocomplete")
                .map(EcoString::as_str),
            Some("shipping email")
        );
    }

    #[test]
    fn p1293_c_dominios_enums_cross_tag_e_span_fechados() {
        use std::num::NonZeroU16;

        let span =
            Span::from_range(FileId::from_raw(NonZeroU16::new(7).unwrap()), 11..19);
        let mut errors = Vec::new();
        for (tag, specs, name, value) in [
            ("col", COL_ATTRS, "span", Value::Int(0)),
            ("iframe", IFRAME_ATTRS, "width", Value::Int(-1)),
            ("select", SELECT_ATTRS, "size", Value::Int(0)),
            ("button", BUTTON_ATTRS, "type", Value::Str("menu".into())),
            ("iframe", IFRAME_ATTRS, "sandbox", Value::Str("scripts".into())),
            ("select", SELECT_ATTRS, "autocomplete", Value::Str("address".into())),
            ("template", TEMPLATE_ATTRS, "shadowrootmode", Value::Str("auto".into())),
            ("video", VIDEO_ATTRS, "preload", Value::Str("eager".into())),
            ("button", BUTTON_ATTRS, "sandbox", Value::Str("allow-scripts".into())),
            ("wbr", WBR_ATTRS, "data-x", Value::Str("x".into())),
        ] {
            let mut args = typed_args(&[(name, value)], None);
            args.span = span;
            let error = if matches!(tag, "col" | "wbr") {
                native_typed_html_void(tag, specs, &args).unwrap_err()
            } else {
                native_typed_html(tag, specs, &args).unwrap_err()
            };
            assert_eq!(error[0].span, span, "{tag}.{name}");
            errors.push(error[0].message.clone());
        }
        assert_eq!(errors[0], "number must be positive");
        assert_eq!(errors[1], "number must be at least zero");
        assert_eq!(errors[2], "number must be positive");
        assert!(errors[8].contains("unexpected argument: sandbox"));
        assert!(errors[9].contains("unexpected argument: data-x"));

        for value in SANDBOX {
            assert!(native_typed_html(
                "iframe",
                IFRAME_ATTRS,
                &typed_args(&[("sandbox", Value::Str((*value).into()))], None),
            )
            .is_ok());
        }
        for value in AUTOCOMPLETE {
            assert!(native_typed_html(
                "select",
                SELECT_ATTRS,
                &typed_args(&[("autocomplete", Value::Str((*value).into()))], None),
            )
            .is_ok());
        }
    }

    #[test]
    fn p1293_c_preload_distingue_valores_de_linguagem_e_strings() {
        for (value, expected) in [
            (Value::None, "none"),
            (Value::Auto, "auto"),
            (Value::Str("metadata".into()), "metadata"),
        ] {
            let video = elem(
                native_typed_html(
                    "video",
                    VIDEO_ATTRS,
                    &typed_args(&[("preload", value)], None),
                )
                .unwrap(),
            );
            assert_eq!(
                video.attrs.as_ref().unwrap().get("preload").map(EcoString::as_str),
                Some(expected),
            );
        }
        for value in [Value::Str("none".into()), Value::Str("auto".into())] {
            let error = native_typed_html(
                "video",
                VIDEO_ATTRS,
                &typed_args(&[("preload", value)], None),
            )
            .unwrap_err();
            assert_eq!(error[0].message, "expected none, auto, or \"metadata\"");
        }
    }

    #[test]
    fn p1293_c_referrerpolicy_nomeia_none_como_literal_de_linguagem() {
        use std::num::NonZeroU16;

        let span =
            Span::from_range(FileId::from_raw(NonZeroU16::new(11).unwrap()), 23..32);
        let invalid_string = cast_attr(
            AttrKind::NoneEmptyOrEnum(REFERRER_POLICIES),
            &Value::Str("invalid".into()),
            span,
        )
        .unwrap_err();
        assert_eq!(
            invalid_string[0].message,
            "expected \"no-referrer\", \"no-referrer-when-downgrade\", \"same-origin\", \"origin\", \"strict-origin\", \"origin-when-cross-origin\", \"strict-origin-when-cross-origin\", \"unsafe-url\", or none",
        );
        assert_eq!(invalid_string[0].span, span);

        let invalid_type = cast_attr(
            AttrKind::NoneEmptyOrEnum(REFERRER_POLICIES),
            &Value::Bool(true),
            span,
        )
        .unwrap_err();
        assert_eq!(
            invalid_type[0].message,
            "expected \"no-referrer\", \"no-referrer-when-downgrade\", \"same-origin\", \"origin\", \"strict-origin\", \"origin-when-cross-origin\", \"strict-origin-when-cross-origin\", \"unsafe-url\", or none, found boolean",
        );
        assert_eq!(invalid_type[0].span, span);

        assert_eq!(
            cast_attr(
                AttrKind::NoneEmptyOrEnum(REFERRER_POLICIES),
                &Value::None,
                Span::detached(),
            )
            .unwrap(),
            Some(EcoString::new()),
        );
    }

    #[test]
    fn p1293_c_dominios_numericos_separam_tipo_de_faixa() {
        for (kind, value, expected) in [
            (
                AttrKind::PositiveInt,
                Value::Str("1".into()),
                "expected integer, found string",
            ),
            (AttrKind::PositiveInt, Value::Int(0), "number must be positive"),
            (AttrKind::NonNegativeInt, Value::Int(-1), "number must be at least zero"),
        ] {
            let error = cast_attr(kind, &value, Span::detached()).unwrap_err();
            assert_eq!(error[0].message, expected);
        }
    }
}

/// P1471 — RED isolado (Agente B): `html` retificado de `Value::Module`
/// para `Value::Func` (`compiler/stdlib/html.md` P1471). Testes escritos a
/// partir do L0 apenas — nenhuma implementação foi consultada.
///
/// Deliberadamente **não** chama `make_html_module()` diretamente: o L0
/// declara que a sua assinatura muda de `-> Module` para `-> Value`
/// ("espelhando `make_pdf_module`"), o que tornaria qualquer teste
/// Rust-level dependente dessa assinatura frágil a um erro de compilação
/// assim que o Agente A materializar a mudança (quebrando GREEN em vez de
/// o confirmar). Os testes abaixo passam exclusivamente pelo observável de
/// língua (`eval_expression_with_features`), robusto à forma interna do
/// constructor.
///
/// - `p1471_html_type_and_repr_are_function_with_feature_enabled` — RED:
///   hoje `type(html) == module` e `repr(html) == "<module html>"`.
/// - `p1471_html_call_fails_cannot_be_constructed_manually_with_feature_enabled`
///   — RED: hoje `html()` não produz esse diagnóstico.
/// - `p1471_html_members_present_with_feature_enabled` /
///   `p1471_html_namespace_length_stable_across_a11y` — não são RED por si
///   (a presença dos membros e a independência de `A11yExtras` já são
///   verdadeiras na baseline); mantidos como cobertura explícita do
///   observável pedido pelo passo (item 4 e item 6, metade `html`).
/// - `p1471_html_feature_off_blocks_bare_and_field_access` — não-regressão
///   do gate de identificador já existente (`Expr::Ident`), inalterado por
///   este passo.
#[cfg(test)]
mod p1471_tests {
    use crate::compiler::eval::eval_expression_with_features;
    use crate::entities::compiler_features::{Feature, Features};
    use crate::entities::file_id::FileId;
    use crate::entities::font_book::FontBook;
    use crate::entities::source::Source;
    use crate::entities::value::Value;
    use crate::entities::world_types::{
        Bytes, Datetime, FileError, FileResult, Font, Library,
    };

    struct TestWorld {
        source: Source,
        library: Library,
        book: FontBook,
    }

    impl crate::contracts::world::World for TestWorld {
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

    fn world() -> TestWorld {
        TestWorld {
            source: Source::detached(""),
            library: Library::new(),
            book: FontBook::new(),
        }
    }

    /// Só os dois perfis em que `Feature::Html` está ativa — sem ela, o
    /// identificador `html` já é bloqueado antes de qualquer lookup (gate
    /// pré-existente em `eval_expr_inner`, inalterado por P1471).
    fn html_enabled_profiles() -> Vec<Features> {
        let mut html_only = Features::empty();
        html_only.enable(Feature::Html);
        let mut html_and_a11y = Features::empty();
        html_and_a11y.enable(Feature::Html);
        html_and_a11y.enable(Feature::A11yExtras);
        vec![html_only, html_and_a11y]
    }

    fn html_disabled_profiles() -> Vec<Features> {
        let default = Features::empty();
        let mut a11y_only = Features::empty();
        a11y_only.enable(Feature::A11yExtras);
        vec![default, a11y_only]
    }

    /// Item 1 + item 2 — `type(html) == function` e `repr(html) == "html"`
    /// quando `Feature::Html` está ativa (com ou sem `A11yExtras`).
    #[test]
    fn p1471_html_type_and_repr_are_function_with_feature_enabled() {
        for features in html_enabled_profiles() {
            let (result, warnings) = eval_expression_with_features(
                &world(),
                "type(html) == function",
                features,
            );
            assert!(warnings.is_empty(), "type(html) em {features:?}: {warnings:?}");
            assert_eq!(
                result.unwrap(),
                Value::Bool(true),
                "type(html) deve ser function em {features:?}"
            );

            let (result, warnings) =
                eval_expression_with_features(&world(), "repr(html)", features);
            assert!(warnings.is_empty(), "repr(html) em {features:?}: {warnings:?}");
            assert_eq!(
                result.unwrap(),
                Value::Str("html".into()),
                "repr(html) deve ser \"html\" em {features:?}"
            );
        }
    }

    /// Item 3 — `html()` falha sempre com exatamente
    /// `"cannot be constructed manually"`.
    #[test]
    fn p1471_html_call_fails_cannot_be_constructed_manually_with_feature_enabled() {
        for features in html_enabled_profiles() {
            let (result, warnings) =
                eval_expression_with_features(&world(), "html()", features);
            assert!(warnings.is_empty(), "html() em {features:?}: {warnings:?}");
            let errors = result.unwrap_err();
            assert_eq!(errors.len(), 1, "html() em {features:?}: {errors:?}");
            assert_eq!(
                errors[0].message, "cannot be constructed manually",
                "html() em {features:?}"
            );
        }
    }

    /// Item 4 — `html.elem`, `html.frame` e amostra de tags tipadas
    /// continuam presentes e chamáveis (tipo função) quando a feature está
    /// ativa, independentemente de `A11yExtras`.
    #[test]
    fn p1471_html_members_present_with_feature_enabled() {
        for features in html_enabled_profiles() {
            let expr = "(type(html.elem) == function, type(html.frame) == function, \
                type(html.div) == function, type(html.a) == function, \
                type(html.br) == function)";
            let (result, warnings) =
                eval_expression_with_features(&world(), expr, features);
            assert!(warnings.is_empty(), "membros de html em {features:?}: {warnings:?}");
            assert_eq!(
                result.unwrap(),
                Value::Array(vec![
                    Value::Bool(true),
                    Value::Bool(true),
                    Value::Bool(true),
                    Value::Bool(true),
                    Value::Bool(true),
                ]),
                "html.elem/frame/div/a/br devem ser function em {features:?}"
            );
        }
    }

    /// Item 6 (metade `html`) — o tamanho do namespace de `html` não muda
    /// com `A11yExtras` (feature ortogonal ao módulo html).
    #[test]
    fn p1471_html_namespace_length_stable_across_a11y() {
        let profiles = html_enabled_profiles();
        let mut lengths = Vec::new();
        for features in &profiles {
            let (result, warnings) =
                eval_expression_with_features(&world(), "html", *features);
            assert!(warnings.is_empty(), "html em {features:?}: {warnings:?}");
            let Value::Func(func) = result.unwrap() else {
                panic!("P1471: html deve ser Value::Func em {features:?}");
            };
            let namespace = func.namespace().expect("html deve ter namespace anexado");
            lengths.push(namespace.len());
        }
        assert_eq!(
            lengths[0], lengths[1],
            "contagem de membros de html não deve depender de a11y-extras: {lengths:?}"
        );
    }

    /// Não-regressão — sem `Feature::Html`, `html` (identificador bare) e
    /// `html.elem` (que avalia o mesmo identificador como target) continuam
    /// bloqueados pelo gate pré-existente, com a mesma mensagem e hints.
    #[test]
    fn p1471_html_feature_off_blocks_bare_and_field_access() {
        for features in html_disabled_profiles() {
            for expr in ["html", "html.elem"] {
                let (result, warnings) =
                    eval_expression_with_features(&world(), expr, features);
                assert!(warnings.is_empty(), "{expr} em {features:?}: {warnings:?}");
                let errors = result.unwrap_err();
                assert_eq!(errors.len(), 1, "{expr} em {features:?}: {errors:?}");
                assert_eq!(
                    errors[0].message,
                    "cannot access variable `html` because the `html` feature is not enabled",
                    "{expr} em {features:?}"
                );
                assert_eq!(
                    errors[0].hints,
                    vec![
                        "try enabling the `html` feature".to_string(),
                        "see https://typst.app/help/compiler-features for more details"
                            .to_string(),
                    ],
                    "{expr} em {features:?}"
                );
            }
        }
    }
}
