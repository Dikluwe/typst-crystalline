//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/html.md
//! @prompt-hash 19720544
//! @layer L1

use std::sync::Arc;

use ecow::EcoString;

use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::dir::Dir;
use crate::entities::file_id::FileId;
use crate::entities::func::Func;
use crate::entities::html::{HtmlAttrs, HtmlBody, HtmlElem};
use crate::entities::module::Module;
use crate::entities::scope::Scope;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;

pub fn make_html_module() -> Module {
    let mut scope = Scope::new();
    scope.define("elem", Value::Func(Func::native("elem", native_html_elem)));
    for (name, native) in TYPED_TAGS {
        scope.define(*name, Value::Func(Func::native(name, *native)));
    }
    Module::new("html", scope)
}

type NativeHtmlFunc = fn(
    &mut EvalContext,
    &Args,
    &dyn crate::contracts::world::World,
    FileId,
) -> SourceResult<Value>;

macro_rules! typed_tag {
    ($fn_name:ident, $tag:literal) => {
        fn $fn_name(
            _ctx: &mut EvalContext,
            args: &Args,
            _world: &dyn crate::contracts::world::World,
            _current_file: FileId,
        ) -> SourceResult<Value> {
            native_typed_html($tag, &[], args)
        }
    };
}

typed_tag!(native_html_div, "div");
typed_tag!(native_html_span, "span");
typed_tag!(native_html_p, "p");
typed_tag!(native_html_h1, "h1");
typed_tag!(native_html_h2, "h2");
typed_tag!(native_html_h3, "h3");
typed_tag!(native_html_h4, "h4");
typed_tag!(native_html_h5, "h5");
typed_tag!(native_html_h6, "h6");
typed_tag!(native_html_strong, "strong");
typed_tag!(native_html_em, "em");
typed_tag!(native_html_ul, "ul");
typed_tag!(native_html_abbr, "abbr");
typed_tag!(native_html_address, "address");
typed_tag!(native_html_article, "article");
typed_tag!(native_html_aside, "aside");
typed_tag!(native_html_b, "b");
typed_tag!(native_html_bdi, "bdi");
typed_tag!(native_html_bdo, "bdo");
typed_tag!(native_html_cite, "cite");
typed_tag!(native_html_code, "code");
typed_tag!(native_html_dfn, "dfn");
typed_tag!(native_html_i, "i");
typed_tag!(native_html_kbd, "kbd");
typed_tag!(native_html_dd, "dd");
typed_tag!(native_html_dl, "dl");
typed_tag!(native_html_dt, "dt");
typed_tag!(native_html_figcaption, "figcaption");
typed_tag!(native_html_figure, "figure");
typed_tag!(native_html_footer, "footer");
typed_tag!(native_html_header, "header");
typed_tag!(native_html_hgroup, "hgroup");
typed_tag!(native_html_legend, "legend");
typed_tag!(native_html_main, "main");
typed_tag!(native_html_mark, "mark");
typed_tag!(native_html_menu, "menu");
typed_tag!(native_html_nav, "nav");
typed_tag!(native_html_picture, "picture");
typed_tag!(native_html_pre, "pre");
typed_tag!(native_html_s, "s");
typed_tag!(native_html_samp, "samp");
typed_tag!(native_html_search, "search");
typed_tag!(native_html_section, "section");
typed_tag!(native_html_small, "small");
typed_tag!(native_html_sub, "sub");
typed_tag!(native_html_sup, "sup");
typed_tag!(native_html_u, "u");
typed_tag!(native_html_var, "var");
typed_tag!(native_html_datalist, "datalist");
typed_tag!(native_html_noscript, "noscript");
typed_tag!(native_html_summary, "summary");
typed_tag!(native_html_ruby, "ruby");
typed_tag!(native_html_rp, "rp");
typed_tag!(native_html_rt, "rt");
typed_tag!(native_html_html, "html");
typed_tag!(native_html_head, "head");
typed_tag!(native_html_body, "body");
typed_tag!(native_html_title, "title");

fn native_html_ol(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    native_typed_html("ol", OL_ATTRS, args)
}

fn native_html_li(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    native_typed_html("li", LI_ATTRS, args)
}

fn native_html_a(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    native_typed_html("a", A_ATTRS, args)
}

fn native_html_br(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    native_typed_html_void("br", &[], args)
}

const TYPED_TAGS: &[(&str, NativeHtmlFunc)] = &[
    ("div", native_html_div),
    ("span", native_html_span),
    ("p", native_html_p),
    ("h1", native_html_h1),
    ("h2", native_html_h2),
    ("h3", native_html_h3),
    ("h4", native_html_h4),
    ("h5", native_html_h5),
    ("h6", native_html_h6),
    ("strong", native_html_strong),
    ("em", native_html_em),
    ("ul", native_html_ul),
    ("ol", native_html_ol),
    ("li", native_html_li),
    ("a", native_html_a),
    ("br", native_html_br),
    ("abbr", native_html_abbr),
    ("address", native_html_address),
    ("article", native_html_article),
    ("aside", native_html_aside),
    ("b", native_html_b),
    ("bdi", native_html_bdi),
    ("bdo", native_html_bdo),
    ("cite", native_html_cite),
    ("code", native_html_code),
    ("dfn", native_html_dfn),
    ("i", native_html_i),
    ("kbd", native_html_kbd),
    ("dd", native_html_dd),
    ("dl", native_html_dl),
    ("dt", native_html_dt),
    ("figcaption", native_html_figcaption),
    ("figure", native_html_figure),
    ("footer", native_html_footer),
    ("header", native_html_header),
    ("hgroup", native_html_hgroup),
    ("legend", native_html_legend),
    ("main", native_html_main),
    ("mark", native_html_mark),
    ("menu", native_html_menu),
    ("nav", native_html_nav),
    ("picture", native_html_picture),
    ("pre", native_html_pre),
    ("s", native_html_s),
    ("samp", native_html_samp),
    ("search", native_html_search),
    ("section", native_html_section),
    ("small", native_html_small),
    ("sub", native_html_sub),
    ("sup", native_html_sup),
    ("u", native_html_u),
    ("var", native_html_var),
    ("datalist", native_html_datalist),
    ("noscript", native_html_noscript),
    ("summary", native_html_summary),
    ("ruby", native_html_ruby),
    ("rp", native_html_rp),
    ("rt", native_html_rt),
    ("html", native_html_html),
    ("head", native_html_head),
    ("body", native_html_body),
    ("title", native_html_title),
];

#[derive(Clone, Copy)]
enum AttrKind {
    Str,
    Int,
    Float,
    TrueFalse,
    Presence,
    ListStr,
    ListChar,
    Enum(&'static [&'static str]),
    EnumList(&'static [&'static str]),
    EnumOrStr(&'static [&'static str]),
    NoneEmptyOrEnum(&'static [&'static str]),
    BoolOr(&'static [&'static str]),
    BoolOrUndefined,
    NoneOrEnum(&'static [&'static str]),
    OnOffOrNoneOrEnum(&'static [&'static str]),
    OnOff,
    ContentEditable,
    Direction,
    PresenceOr(&'static [&'static str]),
    NoneOrStr,
    AutoOr(&'static [&'static str]),
    NoneOrRole,
    YesNo,
}

struct AttrSpec {
    name: &'static str,
    kind: AttrKind,
}

const OL_TYPES: &[&str] = &["1", "a", "A", "i", "I"];
const OL_ATTRS: &[AttrSpec] = &[
    AttrSpec { name: "reversed", kind: AttrKind::Presence },
    AttrSpec { name: "start", kind: AttrKind::Int },
    AttrSpec { name: "type", kind: AttrKind::Enum(OL_TYPES) },
];
const LI_ATTRS: &[AttrSpec] = &[AttrSpec { name: "value", kind: AttrKind::Int }];

const REFERRER_POLICIES: &[&str] = &[
    "no-referrer",
    "no-referrer-when-downgrade",
    "same-origin",
    "origin",
    "strict-origin",
    "origin-when-cross-origin",
    "strict-origin-when-cross-origin",
    "unsafe-url",
];
const REL_TYPES: &[&str] = &[
    "alternate",
    "canonical",
    "author",
    "bookmark",
    "dns-prefetch",
    "expect",
    "external",
    "help",
    "icon",
    "manifest",
    "modulepreload",
    "license",
    "next",
    "nofollow",
    "noopener",
    "noreferrer",
    "opener",
    "pingback",
    "preconnect",
    "prefetch",
    "preload",
    "prev",
    "privacy-policy",
    "search",
    "stylesheet",
    "tag",
    "terms-of-service",
];
const TARGET_TYPES: &[&str] = &["_blank", "_self", "_parent", "_top"];
const A_ATTRS: &[AttrSpec] = &[
    AttrSpec { name: "download", kind: AttrKind::Str },
    AttrSpec { name: "href", kind: AttrKind::Str },
    AttrSpec { name: "hreflang", kind: AttrKind::Str },
    AttrSpec { name: "ping", kind: AttrKind::ListStr },
    AttrSpec {
        name: "referrerpolicy",
        kind: AttrKind::NoneEmptyOrEnum(REFERRER_POLICIES),
    },
    AttrSpec { name: "rel", kind: AttrKind::EnumList(REL_TYPES) },
    AttrSpec {
        name: "target",
        kind: AttrKind::EnumOrStr(TARGET_TYPES),
    },
    AttrSpec { name: "type", kind: AttrKind::Str },
];

const MIXED: &[&str] = &["mixed"];
const AUTOCAPITALIZE: &[&str] = &["sentences", "words", "characters"];
const ENTERKEYHINT: &[&str] =
    &["enter", "done", "go", "next", "previous", "search", "send"];
const INPUTMODE: &[&str] =
    &["text", "tel", "email", "url", "numeric", "decimal", "search"];
const ARIA_AUTOCOMPLETE: &[&str] = &["inline", "list", "both"];
const ARIA_CURRENT: &[&str] = &["page", "step", "location", "date", "time"];
const ARIA_HASPOPUP: &[&str] = &["menu", "listbox", "tree", "grid", "dialog"];
const ARIA_INVALID: &[&str] = &["grammar", "spelling"];
const ARIA_LIVE: &[&str] = &["assertive", "off", "polite"];
const ARIA_ORIENTATION: &[&str] = &["horizontal", "undefined", "vertical"];
const ARIA_RELEVANT: &[&str] =
    &["additions", "additions text", "all", "removals", "text"];
const ARIA_SORT: &[&str] = &["ascending", "descending", "other"];
const ROLES: &[&str] = &[
    "alert",
    "alertdialog",
    "application",
    "article",
    "banner",
    "button",
    "cell",
    "checkbox",
    "columnheader",
    "combobox",
    "command",
    "complementary",
    "composite",
    "contentinfo",
    "definition",
    "dialog",
    "directory",
    "document",
    "feed",
    "figure",
    "form",
    "grid",
    "gridcell",
    "group",
    "heading",
    "img",
    "input",
    "landmark",
    "link",
    "list",
    "listbox",
    "listitem",
    "log",
    "main",
    "marquee",
    "math",
    "menu",
    "menubar",
    "menuitem",
    "menuitemcheckbox",
    "menuitemradio",
    "navigation",
    "note",
    "option",
    "presentation",
    "progressbar",
    "radio",
    "radiogroup",
    "range",
    "region",
    "roletype",
    "row",
    "rowgroup",
    "rowheader",
    "scrollbar",
    "search",
    "searchbox",
    "section",
    "sectionhead",
    "select",
    "separator",
    "slider",
    "spinbutton",
    "status",
    "structure",
    "switch",
    "tab",
    "table",
    "tablist",
    "tabpanel",
    "term",
    "textbox",
    "timer",
    "toolbar",
    "tooltip",
    "tree",
    "treegrid",
    "treeitem",
    "widget",
    "window",
];

const GLOBAL_ATTRS: &[AttrSpec] = &[
    AttrSpec { name: "accesskey", kind: AttrKind::ListChar },
    AttrSpec { name: "aria-activedescendant", kind: AttrKind::Str },
    AttrSpec { name: "aria-atomic", kind: AttrKind::TrueFalse },
    AttrSpec {
        name: "aria-autocomplete",
        kind: AttrKind::NoneOrEnum(ARIA_AUTOCOMPLETE),
    },
    AttrSpec { name: "aria-busy", kind: AttrKind::TrueFalse },
    AttrSpec {
        name: "aria-checked",
        kind: AttrKind::BoolOr(MIXED),
    },
    AttrSpec { name: "aria-colcount", kind: AttrKind::Int },
    AttrSpec { name: "aria-colindex", kind: AttrKind::Int },
    AttrSpec { name: "aria-colspan", kind: AttrKind::Int },
    AttrSpec { name: "aria-controls", kind: AttrKind::ListStr },
    AttrSpec {
        name: "aria-current",
        kind: AttrKind::BoolOr(ARIA_CURRENT),
    },
    AttrSpec { name: "aria-describedby", kind: AttrKind::ListStr },
    AttrSpec { name: "aria-details", kind: AttrKind::Str },
    AttrSpec { name: "aria-disabled", kind: AttrKind::TrueFalse },
    AttrSpec { name: "aria-errormessage", kind: AttrKind::Str },
    AttrSpec {
        name: "aria-expanded",
        kind: AttrKind::BoolOrUndefined,
    },
    AttrSpec { name: "aria-flowto", kind: AttrKind::ListStr },
    AttrSpec {
        name: "aria-haspopup",
        kind: AttrKind::BoolOr(ARIA_HASPOPUP),
    },
    AttrSpec {
        name: "aria-hidden",
        kind: AttrKind::BoolOrUndefined,
    },
    AttrSpec {
        name: "aria-invalid",
        kind: AttrKind::BoolOr(ARIA_INVALID),
    },
    AttrSpec { name: "aria-keyshortcuts", kind: AttrKind::Str },
    AttrSpec { name: "aria-label", kind: AttrKind::Str },
    AttrSpec { name: "aria-labelledby", kind: AttrKind::ListStr },
    AttrSpec { name: "aria-level", kind: AttrKind::Int },
    AttrSpec { name: "aria-live", kind: AttrKind::Enum(ARIA_LIVE) },
    AttrSpec { name: "aria-modal", kind: AttrKind::TrueFalse },
    AttrSpec { name: "aria-multiline", kind: AttrKind::TrueFalse },
    AttrSpec {
        name: "aria-multiselectable",
        kind: AttrKind::TrueFalse,
    },
    AttrSpec {
        name: "aria-orientation",
        kind: AttrKind::Enum(ARIA_ORIENTATION),
    },
    AttrSpec { name: "aria-owns", kind: AttrKind::ListStr },
    AttrSpec { name: "aria-placeholder", kind: AttrKind::Str },
    AttrSpec { name: "aria-posinset", kind: AttrKind::Int },
    AttrSpec {
        name: "aria-pressed",
        kind: AttrKind::BoolOr(MIXED),
    },
    AttrSpec { name: "aria-readonly", kind: AttrKind::TrueFalse },
    AttrSpec {
        name: "aria-relevant",
        kind: AttrKind::EnumList(ARIA_RELEVANT),
    },
    AttrSpec { name: "aria-required", kind: AttrKind::TrueFalse },
    AttrSpec { name: "aria-roledescription", kind: AttrKind::Str },
    AttrSpec { name: "aria-rowcount", kind: AttrKind::Int },
    AttrSpec { name: "aria-rowindex", kind: AttrKind::Int },
    AttrSpec { name: "aria-rowspan", kind: AttrKind::Int },
    AttrSpec {
        name: "aria-selected",
        kind: AttrKind::BoolOrUndefined,
    },
    AttrSpec { name: "aria-setsize", kind: AttrKind::Int },
    AttrSpec {
        name: "aria-sort",
        kind: AttrKind::NoneOrEnum(ARIA_SORT),
    },
    AttrSpec { name: "aria-valuemax", kind: AttrKind::Float },
    AttrSpec { name: "aria-valuemin", kind: AttrKind::Float },
    AttrSpec { name: "aria-valuenow", kind: AttrKind::Float },
    AttrSpec { name: "aria-valuetext", kind: AttrKind::Str },
    AttrSpec {
        name: "autocapitalize",
        kind: AttrKind::OnOffOrNoneOrEnum(AUTOCAPITALIZE),
    },
    AttrSpec { name: "autocorrect", kind: AttrKind::OnOff },
    AttrSpec { name: "autofocus", kind: AttrKind::Presence },
    AttrSpec { name: "class", kind: AttrKind::ListStr },
    AttrSpec {
        name: "contenteditable",
        kind: AttrKind::ContentEditable,
    },
    AttrSpec { name: "dir", kind: AttrKind::Direction },
    AttrSpec { name: "draggable", kind: AttrKind::TrueFalse },
    AttrSpec {
        name: "enterkeyhint",
        kind: AttrKind::Enum(ENTERKEYHINT),
    },
    AttrSpec {
        name: "hidden",
        kind: AttrKind::PresenceOr(&["until-found"]),
    },
    AttrSpec { name: "id", kind: AttrKind::Str },
    AttrSpec { name: "inert", kind: AttrKind::Presence },
    AttrSpec {
        name: "inputmode",
        kind: AttrKind::NoneOrEnum(INPUTMODE),
    },
    AttrSpec { name: "is", kind: AttrKind::Str },
    AttrSpec { name: "itemid", kind: AttrKind::Str },
    AttrSpec { name: "itemprop", kind: AttrKind::ListStr },
    AttrSpec { name: "itemref", kind: AttrKind::ListStr },
    AttrSpec { name: "itemscope", kind: AttrKind::Presence },
    AttrSpec { name: "itemtype", kind: AttrKind::ListStr },
    AttrSpec { name: "lang", kind: AttrKind::NoneOrStr },
    AttrSpec { name: "nonce", kind: AttrKind::Str },
    AttrSpec {
        name: "popover",
        kind: AttrKind::AutoOr(&["manual"]),
    },
    AttrSpec { name: "role", kind: AttrKind::NoneOrRole },
    AttrSpec { name: "slot", kind: AttrKind::Str },
    AttrSpec { name: "spellcheck", kind: AttrKind::TrueFalse },
    AttrSpec { name: "style", kind: AttrKind::Str },
    AttrSpec { name: "tabindex", kind: AttrKind::Int },
    AttrSpec { name: "title", kind: AttrKind::Str },
    AttrSpec { name: "translate", kind: AttrKind::YesNo },
    AttrSpec {
        name: "writingsuggestions",
        kind: AttrKind::TrueFalse,
    },
];

fn native_typed_html(
    tag: &str,
    specific_attrs: &[AttrSpec],
    args: &Args,
) -> SourceResult<Value> {
    native_typed_html_impl(tag, specific_attrs, args, true)
}

fn native_typed_html_void(
    tag: &str,
    specific_attrs: &[AttrSpec],
    args: &Args,
) -> SourceResult<Value> {
    native_typed_html_impl(tag, specific_attrs, args, false)
}

fn native_typed_html_impl(
    tag: &str,
    specific_attrs: &[AttrSpec],
    args: &Args,
    accepts_body: bool,
) -> SourceResult<Value> {
    if !accepts_body && !args.items.is_empty() {
        return Err(vec![SourceDiagnostic::error(args.span, "unexpected argument")]);
    }
    let body = match args.items.as_slice() {
        [] => None,
        [Value::Content(body)] => Some(body.clone()),
        [other] => return type_error(args.span, "content", other),
        [_, ..] => {
            return Err(vec![SourceDiagnostic::error(args.span, "unexpected argument")])
        }
    };

    let mut attrs = HtmlAttrs::default();
    for (name, value) in &args.named {
        let Some(spec) = specific_attrs
            .iter()
            .chain(GLOBAL_ATTRS)
            .find(|spec| spec.name == name.as_str())
        else {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("unexpected argument: {name}"),
            )]);
        };
        if let Some(value) = cast_attr(spec.kind, value, args.span)? {
            attrs.insert(name.clone(), value);
        }
    }

    Ok(Value::Content(Content::HtmlElem(Arc::new(HtmlElem::new(
        tag.into(),
        (!attrs.is_empty()).then_some(attrs),
        if accepts_body {
            body.map_or(HtmlBody::None, |body| HtmlBody::Content(Box::new(body)))
        } else {
            HtmlBody::Unset
        },
    )))))
}

fn cast_attr(
    kind: AttrKind,
    value: &Value,
    span: Span,
) -> SourceResult<Option<EcoString>> {
    let string = match kind {
        AttrKind::Str => require_str(value, span)?.into(),
        AttrKind::Int => match value {
            Value::Int(value) => value.to_string().into(),
            other => return type_error(span, "integer", other),
        },
        AttrKind::Float => match value {
            Value::Float(value) => value.to_string().into(),
            Value::Int(value) => value.to_string().into(),
            other => return type_error(span, "float", other),
        },
        AttrKind::TrueFalse => bool_string(value, span, "true", "false")?.into(),
        AttrKind::Presence => match value {
            Value::Bool(true) => "".into(),
            Value::Bool(false) => return Ok(None),
            other => return type_error(span, "boolean", other),
        },
        AttrKind::ListStr => cast_list(value, span, None, false)?.into(),
        AttrKind::ListChar => cast_list(value, span, None, true)?.into(),
        AttrKind::Enum(values) => require_enum(value, span, values)?.into(),
        AttrKind::EnumList(values) => cast_list(value, span, Some(values), false)?.into(),
        AttrKind::EnumOrStr(values) => match value {
            Value::Str(value) => value.clone(),
            other => {
                let values = values
                    .iter()
                    .map(|value| format!("{value:?}"))
                    .collect::<Vec<_>>()
                    .join(", ");
                return Err(vec![SourceDiagnostic::error(
                    span,
                    format!("expected {values}, or string, found {}", other.type_name()),
                )]);
            }
        },
        AttrKind::NoneEmptyOrEnum(values) => match value {
            Value::None => "".into(),
            _ => require_enum(value, span, values)?.into(),
        },
        AttrKind::BoolOr(values) => match value {
            Value::Bool(value) => value.to_string().into(),
            _ => require_enum(value, span, values)?.into(),
        },
        AttrKind::BoolOrUndefined => match value {
            Value::Bool(value) => value.to_string().into(),
            Value::None => "undefined".into(),
            other => return union_error(span, "boolean or none", other),
        },
        AttrKind::NoneOrEnum(values) => match value {
            Value::None => "none".into(),
            _ => require_enum(value, span, values)?.into(),
        },
        AttrKind::OnOffOrNoneOrEnum(values) => match value {
            Value::Bool(value) => if *value { "on" } else { "off" }.into(),
            Value::None => "none".into(),
            _ => require_enum(value, span, values)?.into(),
        },
        AttrKind::OnOff => bool_string(value, span, "on", "off")?.into(),
        AttrKind::ContentEditable => match value {
            Value::Bool(value) => value.to_string().into(),
            _ => require_enum(value, span, &["plaintext-only"])?.into(),
        },
        AttrKind::Direction => match value {
            Value::Dir(Dir::LTR) => "ltr".into(),
            Value::Dir(Dir::RTL) => "rtl".into(),
            Value::Auto => "auto".into(),
            other => return union_error(span, "ltr, rtl, or auto", other),
        },
        AttrKind::PresenceOr(values) => match value {
            Value::Bool(true) => "".into(),
            Value::Bool(false) => return Ok(None),
            _ => require_enum(value, span, values)?.into(),
        },
        AttrKind::NoneOrStr => match value {
            Value::None => "".into(),
            _ => require_str(value, span)?.into(),
        },
        AttrKind::AutoOr(values) => match value {
            Value::Auto => "auto".into(),
            _ => require_enum(value, span, values)?.into(),
        },
        AttrKind::NoneOrRole => match value {
            Value::None => "none".into(),
            _ => require_enum(value, span, ROLES)?.into(),
        },
        AttrKind::YesNo => bool_string(value, span, "yes", "no")?.into(),
    };
    Ok(Some(string))
}

fn require_str<'a>(value: &'a Value, span: Span) -> SourceResult<&'a str> {
    match value {
        Value::Str(value) => Ok(value),
        other => type_error(span, "string", other),
    }
}

fn require_enum<'a>(
    value: &'a Value,
    span: Span,
    values: &'static [&str],
) -> SourceResult<&'a str> {
    let Value::Str(value) = value else {
        return Err(vec![SourceDiagnostic::error(
            span,
            format!("{}, found {}", expected_enum(values), value.type_name()),
        )]);
    };
    let value = value.as_str();
    if values.contains(&value) {
        Ok(value)
    } else {
        Err(vec![SourceDiagnostic::error(span, expected_enum(values))])
    }
}

fn expected_enum(values: &[&str]) -> String {
    format!("expected {}", expected_enum_body(values))
}

fn expected_enum_body(values: &[&str]) -> String {
    let quoted = values.iter().map(|value| format!("{value:?}")).collect::<Vec<_>>();
    match quoted.as_slice() {
        [] => "valid string".into(),
        [one] => one.clone(),
        [first, second] => format!("{first} or {second}"),
        [head @ .., last] => format!("{}, or {last}", head.join(", ")),
    }
}

fn bool_string<'a>(
    value: &Value,
    span: Span,
    yes: &'a str,
    no: &'a str,
) -> SourceResult<&'a str> {
    match value {
        Value::Bool(true) => Ok(yes),
        Value::Bool(false) => Ok(no),
        other => type_error(span, "boolean", other),
    }
}

fn cast_list(
    value: &Value,
    span: Span,
    allowed: Option<&'static [&'static str]>,
    chars: bool,
) -> SourceResult<String> {
    let items: &[Value] = match value {
        Value::Array(items) => items,
        value => std::slice::from_ref(value),
    };
    let mut out = Vec::with_capacity(items.len());
    for item in items {
        let item = require_str(item, span)?;
        if chars && item.chars().count() != 1 {
            return Err(vec![SourceDiagnostic::error(
                span,
                "expected a single character",
            )]);
        }
        if let Some(allowed) = allowed {
            if !allowed.contains(&item) {
                return Err(vec![SourceDiagnostic::error(span, expected_enum(allowed))]);
            }
        }
        out.push(item);
    }
    Ok(out.join(" "))
}

fn type_error<T>(span: Span, expected: &str, found: &Value) -> SourceResult<T> {
    Err(vec![SourceDiagnostic::error(
        span,
        format!("expected {expected}, found {}", found.type_name()),
    )])
}

fn union_error<T>(span: Span, expected: &str, found: &Value) -> SourceResult<T> {
    type_error(span, expected, found)
}

pub fn native_html_elem(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    if let Some((name, _)) = args.named.iter().find(|(name, _)| name.as_str() != "attrs")
    {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!("unexpected argument: {name}"),
        )]);
    }
    let (tag, body) = match args.items.as_slice() {
        [Value::Str(tag)] => (tag.clone(), None),
        [Value::Str(tag), Value::Content(body)] => (tag.clone(), Some(body.clone())),
        [Value::Str(_), other] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("expected content, found {}", other.type_name()),
            )])
        }
        [other, ..] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("expected string, found {}", other.type_name()),
            )])
        }
        [] => {
            return Err(vec![SourceDiagnostic::error(args.span, "missing argument: tag")])
        }
    };
    validate_name(&tag, "tag", args.span)?;

    let attrs = match args.named.get("attrs") {
        None => None,
        Some(Value::Dict(dict)) => {
            let mut attrs = HtmlAttrs::default();
            for (name, value) in dict {
                validate_name(name, "attribute", args.span)?;
                let Value::Str(value) = value else {
                    return Err(vec![SourceDiagnostic::error(
                        args.span,
                        format!("expected string, found {}", value.type_name()),
                    )]);
                };
                attrs.insert(name.clone(), value.clone());
            }
            Some(attrs)
        }
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("expected dictionary, found {}", other.type_name()),
            )])
        }
    };

    Ok(Value::Content(Content::HtmlElem(Arc::new(HtmlElem::new(
        tag,
        attrs,
        body.map_or(HtmlBody::Unset, |body| HtmlBody::Content(Box::new(body))),
    )))))
}

fn validate_name(name: &EcoString, kind: &str, span: Span) -> SourceResult<()> {
    if name.is_empty() {
        return Err(vec![SourceDiagnostic::error(
            span,
            format!("{kind} name must not be empty"),
        )]);
    }
    if let Some(ch) = name.chars().find(|ch| {
        ch.is_ascii_whitespace() || matches!(ch, '"' | '\'' | '>' | '/' | '=' | '<')
    }) {
        return Err(vec![SourceDiagnostic::error(
            span,
            format!("the character {ch:?} is not valid in a {kind} name"),
        )]);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const P1168_TAGS: &[&str] =
        &["div", "span", "p", "h1", "h2", "h3", "h4", "h5", "h6", "strong", "em", "ul"];

    #[test]
    fn p1168_html_module_expoe_exatamente_o_primeiro_lote_tipado() {
        let module = make_html_module();
        for tag in P1168_TAGS {
            assert!(module.scope().get(tag).is_some(), "binding ausente: html.{tag}");
        }
        for excluded in ["frame"] {
            assert!(
                module.scope().get(excluded).is_none(),
                "binding fora do lote: {excluded}"
            );
        }
    }

    #[test]
    fn p1169_1_html_module_expoe_ol_e_li() {
        let module = make_html_module();
        assert!(module.scope().get("ol").is_some(), "binding ausente: html.ol");
        assert!(module.scope().get("li").is_some(), "binding ausente: html.li");
    }

    #[test]
    fn p1170_1_html_module_expoe_a() {
        let module = make_html_module();
        assert!(module.scope().get("a").is_some(), "binding ausente: html.a");
    }

    #[test]
    fn p1171_1_html_module_expoe_br_sem_body() {
        let module = make_html_module();
        assert!(module.scope().get("br").is_some(), "binding ausente: html.br");

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
        for tag in [
            "abbr", "address", "article", "aside", "b", "bdi", "bdo", "cite", "code",
            "dfn", "i", "kbd",
        ] {
            assert!(module.scope().get(tag).is_some(), "binding ausente: html.{tag}");
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
            assert!(module.scope().get(tag).is_some(), "binding ausente: html.{tag}");
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
        for tag in [
            "nav", "picture", "pre", "s", "samp", "search", "section", "small", "sub",
            "sup", "u", "var",
        ] {
            assert!(module.scope().get(tag).is_some(), "binding ausente: html.{tag}");
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
        for tag in ["datalist", "noscript", "summary"] {
            assert!(module.scope().get(tag).is_some(), "binding ausente: html.{tag}");
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
        for tag in ["ruby", "rp", "rt"] {
            assert!(module.scope().get(tag).is_some(), "binding ausente: html.{tag}");
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
        for tag in ["html", "head", "body", "title"] {
            assert!(module.scope().get(tag).is_some(), "binding ausente: html.{tag}");
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
}
