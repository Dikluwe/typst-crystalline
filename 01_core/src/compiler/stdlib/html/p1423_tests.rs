//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/html/p1423-tests.md
//! @prompt-hash c86a1e90
//! @layer L1
#![cfg(test)]
//!
//! Teste A/B independente do P1423.
//!
//! Este módulo é incluído pelo implementador somente depois de materializar o
//! candidato. Ele testa a superfície e a morfologia L1; DOM/export é L3 e não
//! é falsamente atestado aqui.

use super::*;

use std::collections::BTreeSet;
use std::num::NonZeroU16;

const P1423_BINDINGS: [&str; 43] = [
    "area",
    "audio",
    "base",
    "blockquote",
    "canvas",
    "caption",
    "colgroup",
    "data",
    "del",
    "details",
    "dialog",
    "embed",
    "fieldset",
    "form",
    "hr",
    "img",
    "input",
    "ins",
    "label",
    "link",
    "map",
    "meta",
    "meter",
    "object",
    "optgroup",
    "option",
    "output",
    "progress",
    "q",
    "script",
    "slot",
    "source",
    "style",
    "table",
    "tbody",
    "td",
    "textarea",
    "tfoot",
    "th",
    "thead",
    "time",
    "tr",
    "track",
];

const P1423_VOID: [&str; 10] =
    ["area", "base", "embed", "hr", "img", "input", "link", "meta", "source", "track"];

const P1423_RAW: [&str; 2] = ["script", "style"];

const P1423_CONTENT: [&str; 31] = [
    "audio",
    "blockquote",
    "canvas",
    "caption",
    "colgroup",
    "data",
    "del",
    "details",
    "dialog",
    "fieldset",
    "form",
    "ins",
    "label",
    "map",
    "meter",
    "object",
    "optgroup",
    "option",
    "output",
    "progress",
    "q",
    "slot",
    "table",
    "tbody",
    "td",
    "textarea",
    "tfoot",
    "th",
    "thead",
    "time",
    "tr",
];

const PRE_P1423_BINDINGS: [&str; 69] = [
    "div",
    "span",
    "p",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "strong",
    "em",
    "ul",
    "ol",
    "li",
    "a",
    "br",
    "abbr",
    "address",
    "article",
    "aside",
    "b",
    "bdi",
    "bdo",
    "cite",
    "code",
    "dfn",
    "i",
    "kbd",
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
    "nav",
    "picture",
    "pre",
    "s",
    "samp",
    "search",
    "section",
    "small",
    "sub",
    "sup",
    "u",
    "var",
    "datalist",
    "noscript",
    "summary",
    "ruby",
    "rp",
    "rt",
    "html",
    "head",
    "body",
    "title",
    "button",
    "col",
    "iframe",
    "select",
    "template",
    "video",
    "wbr",
];

#[derive(Default)]
struct P1423World {
    library: crate::entities::world_types::Library,
    book: crate::entities::font_book::FontBook,
}

impl crate::contracts::world::World for P1423World {
    fn library(&self) -> &crate::entities::world_types::Library {
        &self.library
    }

    fn book(&self) -> &crate::entities::font_book::FontBook {
        &self.book
    }

    fn main(&self) -> FileId {
        test_file_id()
    }

    fn source(
        &self,
        _: FileId,
    ) -> crate::entities::world_types::FileResult<crate::entities::source::Source> {
        Err(crate::entities::world_types::FileError::NotFound)
    }

    fn file(
        &self,
        _: FileId,
    ) -> crate::entities::world_types::FileResult<crate::entities::world_types::Bytes>
    {
        Err(crate::entities::world_types::FileError::NotFound)
    }

    fn font(&self, _: usize) -> Option<crate::entities::world_types::Font> {
        None
    }

    fn today(
        &self,
        _: Option<crate::entities::duration::Duration>,
    ) -> Option<crate::entities::world_types::Datetime> {
        None
    }
}

fn test_file_id() -> FileId {
    FileId::from_raw(NonZeroU16::new(1423).unwrap())
}

fn args(named: &[(&str, Value)], positional: Vec<Value>) -> Args {
    let mut args = Args::positional(positional);
    args.named = named
        .iter()
        .map(|(name, value)| ((*name).into(), value.clone()))
        .collect();
    args
}

fn call(tag: &str, args: Args) -> SourceResult<Value> {
    let native = TYPED_TAGS
        .iter()
        .find_map(|(name, native)| (*name == tag).then_some(*native))
        .unwrap_or_else(|| panic!("binding ausente no dispatcher: html.{tag}"));
    native(&mut EvalContext::new(), &args, &P1423World::default(), test_file_id())
}

fn elem(value: Value) -> Arc<HtmlElem> {
    match value {
        Value::Content(Content::HtmlElem(elem)) => elem,
        other => panic!("esperava HtmlElem, recebeu {other:?}"),
    }
}

fn attr<'a>(elem: &'a HtmlElem, name: &str) -> Option<&'a str> {
    elem.attrs
        .as_ref()
        .and_then(|attrs| attrs.get(name))
        .map(EcoString::as_str)
}

#[test]
fn p1423_ab_superficie_e_particao_sao_exatamente_43_sem_frame() {
    let p1423: BTreeSet<_> = P1423_BINDINGS.into_iter().collect();
    let void: BTreeSet<_> = P1423_VOID.into_iter().collect();
    let raw: BTreeSet<_> = P1423_RAW.into_iter().collect();
    let content: BTreeSet<_> = P1423_CONTENT.into_iter().collect();
    assert_eq!(p1423.len(), 43);
    assert_eq!(void.len(), 10);
    assert_eq!(raw.len(), 2);
    assert_eq!(content.len(), 31);
    assert!(void.is_disjoint(&raw));
    assert!(void.is_disjoint(&content));
    assert!(raw.is_disjoint(&content));
    let mut partition = void.clone();
    partition.extend(raw);
    partition.extend(content);
    assert_eq!(partition, p1423);
    assert!(!p1423.contains("frame"));

    let legacy: BTreeSet<_> = PRE_P1423_BINDINGS.into_iter().collect();
    assert_eq!(legacy.len(), 69);
    assert!(legacy.is_disjoint(&p1423));

    let dispatched: BTreeSet<_> = TYPED_TAGS.iter().map(|(name, _)| *name).collect();
    assert_eq!(TYPED_TAGS.len(), 112, "dispatcher deve conter 69 + 43 tags");
    assert_eq!(dispatched.len(), 112, "binding duplicado no dispatcher");
    assert_eq!(dispatched.difference(&legacy).copied().collect::<BTreeSet<_>>(), p1423);

    let module = make_html_module();
    let Value::Func(module) = &module else { panic!("html must be a function") };
    let module = module.namespace().unwrap();
    assert_eq!(module.len(), 114, "elem + frame + 112 constructors tipados");
    for name in P1423_BINDINGS {
        assert!(module.get(name).is_some(), "binding ausente: html.{name}");
    }
    assert!(module.get("frame").is_some(), "html.frame pertence ao P1424");
}

#[test]
fn p1423_ab_body_void_raw_e_content_preservam_as_tres_morfologias() {
    for tag in P1423_VOID {
        let omitted = elem(call(tag, args(&[], vec![])).unwrap());
        assert!(matches!(omitted.body, HtmlBody::Unset), "{tag}");
        assert!(
            call(tag, args(&[], vec![Value::Content(Content::text("X"))])).is_err(),
            "void aceitou body: {tag}"
        );
    }

    for tag in P1423_CONTENT {
        let omitted = elem(call(tag, args(&[], vec![])).unwrap());
        assert!(matches!(omitted.body, HtmlBody::None), "{tag}");
        let supplied = elem(
            call(tag, args(&[], vec![Value::Content(Content::text("conteudo"))]))
                .unwrap(),
        );
        assert_eq!(supplied.body.content(), Some(&Content::text("conteudo")), "{tag}");
        assert!(
            call(tag, args(&[], vec![Value::Str("texto cru".into())])).is_err(),
            "{tag}"
        );
    }

    for tag in P1423_RAW {
        let omitted = elem(call(tag, args(&[], vec![])).unwrap());
        assert!(matches!(omitted.body, HtmlBody::None), "{tag}");
        let supplied =
            elem(call(tag, args(&[], vec![Value::Str("a < b".into())])).unwrap());
        assert_eq!(supplied.body.content(), Some(&Content::text("a < b")), "{tag}");
        assert!(
            call(tag, args(&[], vec![Value::Content(Content::text("não raw"))])).is_err(),
            "raw aceitou Content: {tag}"
        );
    }
}

#[test]
fn p1423_ab_casts_representativos_fecham_attrs_enum_lista_uniao_e_cross_tag() {
    let details = elem(
        call(
            "details",
            args(
                &[
                    ("id", Value::Str("d".into())),
                    ("name", Value::Str("grupo".into())),
                    ("open", Value::Bool(true)),
                ],
                vec![],
            ),
        )
        .unwrap(),
    );
    assert_eq!(
        details
            .attrs
            .as_ref()
            .unwrap()
            .iter()
            .map(|(name, value)| (name.as_str(), value.as_str()))
            .collect::<Vec<_>>(),
        vec![("id", "d"), ("name", "grupo"), ("open", "")],
    );
    let closed =
        elem(call("details", args(&[("open", Value::Bool(false))], vec![])).unwrap());
    assert!(closed.attrs.is_none());

    assert_eq!(
        attr(
            &elem(call("canvas", args(&[("width", Value::Int(0))], vec![])).unwrap()),
            "width",
        ),
        Some("0")
    );
    assert!(call("canvas", args(&[("height", Value::Int(-1))], vec![])).is_err());
    assert!(call("colgroup", args(&[("span", Value::Int(1))], vec![])).is_ok());
    assert!(call("colgroup", args(&[("span", Value::Int(0))], vec![])).is_err());

    let th = elem(
        call(
            "th",
            args(
                &[
                    (
                        "headers",
                        Value::Array(vec![
                            Value::Str("a".into()),
                            Value::Str("b".into()),
                        ]),
                    ),
                    ("scope", Value::Str("rowgroup".into())),
                ],
                vec![],
            ),
        )
        .unwrap(),
    );
    assert_eq!(attr(&th, "headers"), Some("a b"));
    assert_eq!(attr(&th, "scope"), Some("rowgroup"));
    assert!(call("th", args(&[("scope", Value::Str("cell".into()))], vec![])).is_err());

    let area = elem(
        call(
            "area",
            args(
                &[("coords", Value::Array(vec![Value::Float(1.5), Value::Float(2.0)]))],
                vec![],
            ),
        )
        .unwrap(),
    );
    assert!(attr(&area, "coords").is_some());
    assert!(call("area", args(&[("coords", Value::Float(1.5))], vec![])).is_err());

    assert_eq!(
        attr(
            &elem(call("audio", args(&[("preload", Value::None)], vec![])).unwrap()),
            "preload",
        ),
        Some("none")
    );
    assert!(
        call("audio", args(&[("preload", Value::Str("none".into()))], vec![])).is_err()
    );
    assert_eq!(
        attr(
            &elem(
                call("form", args(&[("autocomplete", Value::Bool(false))], vec![]))
                    .unwrap()
            ),
            "autocomplete",
        ),
        Some("off")
    );

    assert!(call("blockquote", args(&[("open", Value::Bool(true))], vec![])).is_err());
    assert!(call("details", args(&[("cite", Value::Str("/x".into()))], vec![])).is_err());
    assert!(call("caption", args(&[("data-x", Value::Str("x".into()))], vec![])).is_err());
}

#[test]
fn p1423_ab_raw_attrs_nesting_e_feature_continuam_puros_em_l1() {
    let script = elem(
        call(
            "script",
            args(
                &[("async", Value::Bool(true)), ("type", Value::Str("module".into()))],
                vec![Value::Str("if (a < b) c();".into())],
            ),
        )
        .unwrap(),
    );
    assert_eq!(attr(&script, "async"), Some(""));
    assert_eq!(attr(&script, "type"), Some("module"));
    assert_eq!(script.body.content(), Some(&Content::text("if (a < b) c();")));

    let caption = elem(
        call("caption", args(&[], vec![Value::Content(Content::text("Legenda"))]))
            .unwrap(),
    );
    let table = elem(
        call(
            "table",
            args(&[], vec![Value::Content(Content::HtmlElem(caption.clone()))]),
        )
        .unwrap(),
    );
    assert_eq!(table.tag, "table");
    assert!(
        matches!(table.body.content(), Some(Content::HtmlElem(child)) if child.tag == "caption")
    );

    // O gate da feature acontece no acesso ao binding, não dentro do
    // constructor. Chamar o native com EvalContext default (feature off)
    // continua válido e não cria dependência de target/export em L1.
    assert!(!Features::default().contains(Feature::Html));
    let direct = elem(call("caption", args(&[], vec![])).unwrap());
    assert_eq!(direct.tag, "caption");
    let mut enabled = Features::empty();
    enabled.enable(Feature::Html);
    assert!(enabled.contains(Feature::Html));
}
