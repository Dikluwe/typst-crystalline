//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/html.md
//! @prompt-hash 7d452ced
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

#[cfg(test)]
#[path = "html/p1423_tests.rs"]
mod p1423_tests;

#[cfg(test)]
#[path = "html/p1424_tests.rs"]
mod p1424_tests;

/// Constrói o valor `html` como `Value::Func` com namespace fechado
/// (`elem`, `frame`, as tags tipadas) — paridade com o vanilla ratificado,
/// onde `type(html) == function` (ver `compiler/stdlib/html.md` P1471).
/// Chamar `html(...)` diretamente é sempre erro; só os membros do
/// namespace são chamáveis.
pub fn make_html_module() -> Value {
    let mut scope = Scope::new();
    scope.define("elem", Value::Func(Func::native("elem", native_html_elem)));
    scope.define("frame", Value::Func(Func::native("frame", native_html_frame)));
    for (name, native) in TYPED_TAGS {
        scope.define(*name, Value::Func(Func::native(name, *native)));
    }
    Value::Func(Func::native_with_namespace("html", native_html_root, Arc::new(scope)))
}

/// `html(...)` não pode ser construído manualmente — mesmo diagnóstico
/// medido no vanilla ratificado `586e1bd43fae6c9a973218163d3165c53ab8d16d`
/// para `pdf()`/`html()` (`compiler/stdlib/html.md` P1471).
fn native_html_root(
    _ctx: &mut EvalContext,
    _args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    Err(vec![SourceDiagnostic::error(Span::detached(), "cannot be constructed manually")])
}

fn native_html_frame(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    if args.named.contains_key("body") {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "the argument `body` is positional",
        )]);
    }
    if let Some((name, _)) = args.named.first() {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!("unexpected argument: {name}"),
        )]);
    }
    let body = match args.items.as_slice() {
        [] => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                "missing argument: body",
            )])
        }
        [Value::Content(body)] => body.clone(),
        [other] => return type_error(args.span, "content", other),
        [_, ..] => {
            return Err(vec![SourceDiagnostic::error(args.span, "unexpected argument")])
        }
    };
    Ok(Value::Content(Content::HtmlFrame(Arc::new(HtmlFrameElem { body }))))
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

macro_rules! typed_tag_with_attrs {
    ($fn_name:ident, $tag:literal, $attrs:ident) => {
        fn $fn_name(
            _ctx: &mut EvalContext,
            args: &Args,
            _world: &dyn crate::contracts::world::World,
            _current_file: FileId,
        ) -> SourceResult<Value> {
            native_typed_html($tag, $attrs, args)
        }
    };
}

macro_rules! typed_void_tag_with_attrs {
    ($fn_name:ident, $tag:literal, $attrs:ident) => {
        fn $fn_name(
            _ctx: &mut EvalContext,
            args: &Args,
            _world: &dyn crate::contracts::world::World,
            _current_file: FileId,
        ) -> SourceResult<Value> {
            native_typed_html_void($tag, $attrs, args)
        }
    };
}

macro_rules! typed_raw_tag_with_attrs {
    ($fn_name:ident, $tag:literal, $attrs:ident) => {
        fn $fn_name(
            _ctx: &mut EvalContext,
            args: &Args,
            _world: &dyn crate::contracts::world::World,
            _current_file: FileId,
        ) -> SourceResult<Value> {
            native_typed_html_raw($tag, $attrs, args)
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
typed_tag_with_attrs!(native_html_button, "button", BUTTON_ATTRS);
typed_tag_with_attrs!(native_html_iframe, "iframe", IFRAME_ATTRS);
typed_tag_with_attrs!(native_html_select, "select", SELECT_ATTRS);
typed_tag_with_attrs!(native_html_template, "template", TEMPLATE_ATTRS);
typed_tag_with_attrs!(native_html_video, "video", VIDEO_ATTRS);
typed_void_tag_with_attrs!(native_html_area, "area", AREA_ATTRS);
typed_tag_with_attrs!(native_html_audio, "audio", AUDIO_ATTRS);
typed_void_tag_with_attrs!(native_html_base, "base", BASE_ATTRS);
typed_tag_with_attrs!(native_html_blockquote, "blockquote", CITE_ATTRS);
typed_tag_with_attrs!(native_html_canvas, "canvas", CANVAS_ATTRS);
typed_tag!(native_html_caption, "caption");
typed_tag_with_attrs!(native_html_colgroup, "colgroup", COLGROUP_ATTRS);
typed_tag_with_attrs!(native_html_data, "data", DATA_ATTRS);
typed_tag_with_attrs!(native_html_del, "del", DEL_INS_ATTRS);
typed_tag_with_attrs!(native_html_details, "details", DETAILS_ATTRS);
typed_tag_with_attrs!(native_html_dialog, "dialog", DIALOG_ATTRS);
typed_void_tag_with_attrs!(native_html_embed, "embed", EMBED_ATTRS);
typed_tag_with_attrs!(native_html_fieldset, "fieldset", FIELDSET_ATTRS);
typed_tag_with_attrs!(native_html_form, "form", FORM_ATTRS);
typed_void_tag_with_attrs!(native_html_hr, "hr", NO_ATTRS);
typed_void_tag_with_attrs!(native_html_img, "img", IMG_ATTRS);
typed_void_tag_with_attrs!(native_html_input, "input", INPUT_ATTRS);
typed_tag_with_attrs!(native_html_ins, "ins", DEL_INS_ATTRS);
typed_tag_with_attrs!(native_html_label, "label", LABEL_ATTRS);
typed_void_tag_with_attrs!(native_html_link, "link", LINK_ATTRS);
typed_tag_with_attrs!(native_html_map, "map", NAME_ATTRS);
typed_void_tag_with_attrs!(native_html_meta, "meta", META_ATTRS);
typed_tag_with_attrs!(native_html_meter, "meter", METER_ATTRS);
typed_tag_with_attrs!(native_html_object, "object", OBJECT_ATTRS);
typed_tag_with_attrs!(native_html_optgroup, "optgroup", OPTGROUP_ATTRS);
typed_tag_with_attrs!(native_html_option, "option", OPTION_ATTRS);
typed_tag_with_attrs!(native_html_output, "output", OUTPUT_ATTRS);
typed_tag_with_attrs!(native_html_progress, "progress", PROGRESS_ATTRS);
typed_tag_with_attrs!(native_html_q, "q", CITE_ATTRS);
typed_raw_tag_with_attrs!(native_html_script, "script", SCRIPT_ATTRS);
typed_tag_with_attrs!(native_html_slot, "slot", NAME_ATTRS);
typed_void_tag_with_attrs!(native_html_source, "source", SOURCE_ATTRS);
typed_raw_tag_with_attrs!(native_html_style, "style", STYLE_ATTRS);
typed_tag!(native_html_table, "table");
typed_tag!(native_html_tbody, "tbody");
typed_tag_with_attrs!(native_html_td, "td", TD_ATTRS);
typed_tag_with_attrs!(native_html_textarea, "textarea", TEXTAREA_ATTRS);
typed_tag!(native_html_tfoot, "tfoot");
typed_tag_with_attrs!(native_html_th, "th", TH_ATTRS);
typed_tag!(native_html_thead, "thead");
typed_tag_with_attrs!(native_html_time, "time", TIME_ATTRS);
typed_tag!(native_html_tr, "tr");
typed_void_tag_with_attrs!(native_html_track, "track", TRACK_ATTRS);

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

fn native_html_col(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    native_typed_html_void("col", COL_ATTRS, args)
}

fn native_html_wbr(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    native_typed_html_void("wbr", WBR_ATTRS, args)
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
    ("button", native_html_button),
    ("col", native_html_col),
    ("iframe", native_html_iframe),
    ("select", native_html_select),
    ("template", native_html_template),
    ("video", native_html_video),
    ("wbr", native_html_wbr),
    ("area", native_html_area),
    ("audio", native_html_audio),
    ("base", native_html_base),
    ("blockquote", native_html_blockquote),
    ("canvas", native_html_canvas),
    ("caption", native_html_caption),
    ("colgroup", native_html_colgroup),
    ("data", native_html_data),
    ("del", native_html_del),
    ("details", native_html_details),
    ("dialog", native_html_dialog),
    ("embed", native_html_embed),
    ("fieldset", native_html_fieldset),
    ("form", native_html_form),
    ("hr", native_html_hr),
    ("img", native_html_img),
    ("input", native_html_input),
    ("ins", native_html_ins),
    ("label", native_html_label),
    ("link", native_html_link),
    ("map", native_html_map),
    ("meta", native_html_meta),
    ("meter", native_html_meter),
    ("object", native_html_object),
    ("optgroup", native_html_optgroup),
    ("option", native_html_option),
    ("output", native_html_output),
    ("progress", native_html_progress),
    ("q", native_html_q),
    ("script", native_html_script),
    ("slot", native_html_slot),
    ("source", native_html_source),
    ("style", native_html_style),
    ("table", native_html_table),
    ("tbody", native_html_tbody),
    ("td", native_html_td),
    ("textarea", native_html_textarea),
    ("tfoot", native_html_tfoot),
    ("th", native_html_th),
    ("thead", native_html_thead),
    ("time", native_html_time),
    ("tr", native_html_tr),
    ("track", native_html_track),
];

#[derive(Clone, Copy)]
enum AttrKind {
    Str,
    Int,
    PositiveInt,
    NonNegativeInt,
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
    Preload,
    NoneOrRole,
    YesNo,
    Datetime,
    DurationOrDatetime,
    Color,
    FloatListComma,
    StrListComma,
    StrFloatDatetime,
    PositiveFloatOr(&'static [&'static str]),
    InputValue,
    SourceSizeList,
    ImageCandidateList,
    IconSizeList,
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

const BUTTON_COMMANDS: &[&str] = &[
    "toggle-popover",
    "show-popover",
    "hide-popover",
    "close",
    "request-close",
    "show-modal",
];
const BUTTON_ENCTYPES: &[&str] =
    &["application/x-www-form-urlencoded", "multipart/form-data", "text/plain"];
const BUTTON_METHODS: &[&str] = &["GET", "POST", "dialog"];
const POPOVER_TARGET_ACTIONS: &[&str] = &["toggle", "show", "hide"];
const BUTTON_TYPES: &[&str] = &["submit", "reset", "button"];
const BUTTON_ATTRS: &[AttrSpec] = &[
    AttrSpec {
        name: "command",
        kind: AttrKind::EnumOrStr(BUTTON_COMMANDS),
    },
    AttrSpec { name: "commandfor", kind: AttrKind::Str },
    AttrSpec { name: "disabled", kind: AttrKind::Presence },
    AttrSpec { name: "form", kind: AttrKind::Str },
    AttrSpec { name: "formaction", kind: AttrKind::Str },
    AttrSpec {
        name: "formenctype",
        kind: AttrKind::Enum(BUTTON_ENCTYPES),
    },
    AttrSpec {
        name: "formmethod",
        kind: AttrKind::Enum(BUTTON_METHODS),
    },
    AttrSpec { name: "formnovalidate", kind: AttrKind::Presence },
    AttrSpec {
        name: "formtarget",
        kind: AttrKind::EnumOrStr(TARGET_TYPES),
    },
    AttrSpec { name: "name", kind: AttrKind::Str },
    AttrSpec { name: "popovertarget", kind: AttrKind::Str },
    AttrSpec {
        name: "popovertargetaction",
        kind: AttrKind::Enum(POPOVER_TARGET_ACTIONS),
    },
    AttrSpec { name: "type", kind: AttrKind::Enum(BUTTON_TYPES) },
    AttrSpec { name: "value", kind: AttrKind::Str },
];

const COL_ATTRS: &[AttrSpec] = &[AttrSpec { name: "span", kind: AttrKind::PositiveInt }];

const IFRAME_LOADING: &[&str] = &["lazy", "eager"];
const SANDBOX: &[&str] = &[
    "allow-downloads",
    "allow-forms",
    "allow-modals",
    "allow-orientation-lock",
    "allow-pointer-lock",
    "allow-popups",
    "allow-popups-to-escape-sandbox",
    "allow-presentation",
    "allow-same-origin",
    "allow-scripts",
    "allow-top-navigation",
    "allow-top-navigation-by-user-activation",
    "allow-top-navigation-to-custom-protocols",
];
const IFRAME_ATTRS: &[AttrSpec] = &[
    AttrSpec { name: "allow", kind: AttrKind::Str },
    AttrSpec { name: "allowfullscreen", kind: AttrKind::Presence },
    AttrSpec { name: "height", kind: AttrKind::NonNegativeInt },
    AttrSpec {
        name: "loading",
        kind: AttrKind::Enum(IFRAME_LOADING),
    },
    AttrSpec {
        name: "name",
        kind: AttrKind::EnumOrStr(TARGET_TYPES),
    },
    AttrSpec {
        name: "referrerpolicy",
        kind: AttrKind::NoneEmptyOrEnum(REFERRER_POLICIES),
    },
    AttrSpec { name: "sandbox", kind: AttrKind::EnumList(SANDBOX) },
    AttrSpec { name: "src", kind: AttrKind::Str },
    AttrSpec { name: "srcdoc", kind: AttrKind::Str },
    AttrSpec { name: "width", kind: AttrKind::NonNegativeInt },
];

const AUTOCOMPLETE: &[&str] = &[
    "shipping",
    "billing",
    "name",
    "honorific-prefix",
    "given-name",
    "additional-name",
    "family-name",
    "honorific-suffix",
    "nickname",
    "username",
    "new-password",
    "current-password",
    "one-time-code",
    "organization-title",
    "organization",
    "street-address",
    "address-line1",
    "address-line2",
    "address-line3",
    "address-level4",
    "address-level3",
    "address-level2",
    "address-level1",
    "country",
    "country-name",
    "postal-code",
    "cc-name",
    "cc-given-name",
    "cc-additional-name",
    "cc-family-name",
    "cc-number",
    "cc-exp",
    "cc-exp-month",
    "cc-exp-year",
    "cc-csc",
    "cc-type",
    "transaction-currency",
    "transaction-amount",
    "language",
    "bday",
    "bday-day",
    "bday-month",
    "bday-year",
    "sex",
    "url",
    "photo",
    "home",
    "work",
    "mobile",
    "fax",
    "pager",
    "tel",
    "tel-country-code",
    "tel-national",
    "tel-area-code",
    "tel-local",
    "tel-local-prefix",
    "tel-local-suffix",
    "tel-extension",
    "email",
    "impp",
];
const SELECT_ATTRS: &[AttrSpec] = &[
    AttrSpec {
        name: "autocomplete",
        kind: AttrKind::EnumList(AUTOCOMPLETE),
    },
    AttrSpec { name: "disabled", kind: AttrKind::Presence },
    AttrSpec { name: "form", kind: AttrKind::Str },
    AttrSpec { name: "multiple", kind: AttrKind::Presence },
    AttrSpec { name: "name", kind: AttrKind::Str },
    AttrSpec { name: "required", kind: AttrKind::Presence },
    AttrSpec { name: "size", kind: AttrKind::PositiveInt },
];

const TEMPLATE_ATTRS: &[AttrSpec] = &[
    AttrSpec {
        name: "shadowrootclonable",
        kind: AttrKind::Presence,
    },
    AttrSpec {
        name: "shadowrootcustomelementregistry",
        kind: AttrKind::Presence,
    },
    AttrSpec {
        name: "shadowrootdelegatesfocus",
        kind: AttrKind::Presence,
    },
    AttrSpec {
        name: "shadowrootmode",
        kind: AttrKind::Enum(&["open", "closed"]),
    },
    AttrSpec {
        name: "shadowrootserializable",
        kind: AttrKind::Presence,
    },
];

const VIDEO_ATTRS: &[AttrSpec] = &[
    AttrSpec { name: "autoplay", kind: AttrKind::Presence },
    AttrSpec { name: "controls", kind: AttrKind::Presence },
    AttrSpec {
        name: "crossorigin",
        kind: AttrKind::Enum(&["anonymous", "use-credentials"]),
    },
    AttrSpec { name: "height", kind: AttrKind::NonNegativeInt },
    AttrSpec { name: "loop", kind: AttrKind::Presence },
    AttrSpec { name: "muted", kind: AttrKind::Presence },
    AttrSpec { name: "playsinline", kind: AttrKind::Presence },
    AttrSpec { name: "poster", kind: AttrKind::Str },
    AttrSpec { name: "preload", kind: AttrKind::Preload },
    AttrSpec { name: "src", kind: AttrKind::Str },
    AttrSpec { name: "width", kind: AttrKind::NonNegativeInt },
];

const WBR_ATTRS: &[AttrSpec] = &[];

const NO_ATTRS: &[AttrSpec] = &[];
const CROSS_ORIGIN: &[&str] = &["anonymous", "use-credentials"];
const FETCH_PRIORITY: &[&str] = &["auto", "high", "low"];
const FORM_ENCTYPES: &[&str] =
    &["application/x-www-form-urlencoded", "multipart/form-data", "text/plain"];
const FORM_METHODS: &[&str] = &["GET", "POST", "dialog"];
const BLOCKING: &[&str] = &["blocking"];

const AREA_ATTRS: &[AttrSpec] = &[
    AttrSpec { name: "alt", kind: AttrKind::Str },
    AttrSpec { name: "coords", kind: AttrKind::FloatListComma },
    AttrSpec { name: "download", kind: AttrKind::Str },
    AttrSpec { name: "href", kind: AttrKind::Str },
    AttrSpec { name: "ping", kind: AttrKind::ListStr },
    AttrSpec {
        name: "referrerpolicy",
        kind: AttrKind::NoneEmptyOrEnum(REFERRER_POLICIES),
    },
    AttrSpec { name: "rel", kind: AttrKind::EnumList(REL_TYPES) },
    AttrSpec {
        name: "shape",
        kind: AttrKind::Enum(&["circle", "default", "poly", "rect"]),
    },
    AttrSpec {
        name: "target",
        kind: AttrKind::EnumOrStr(TARGET_TYPES),
    },
];
const AUDIO_ATTRS: &[AttrSpec] = &[
    AttrSpec { name: "autoplay", kind: AttrKind::Presence },
    AttrSpec { name: "controls", kind: AttrKind::Presence },
    AttrSpec {
        name: "crossorigin",
        kind: AttrKind::Enum(CROSS_ORIGIN),
    },
    AttrSpec { name: "loop", kind: AttrKind::Presence },
    AttrSpec { name: "muted", kind: AttrKind::Presence },
    AttrSpec { name: "preload", kind: AttrKind::Preload },
    AttrSpec { name: "src", kind: AttrKind::Str },
];
const BASE_ATTRS: &[AttrSpec] = &[
    AttrSpec { name: "href", kind: AttrKind::Str },
    AttrSpec {
        name: "target",
        kind: AttrKind::EnumOrStr(TARGET_TYPES),
    },
];
const CITE_ATTRS: &[AttrSpec] = &[AttrSpec { name: "cite", kind: AttrKind::Str }];
const CANVAS_ATTRS: &[AttrSpec] = &[
    AttrSpec { name: "height", kind: AttrKind::NonNegativeInt },
    AttrSpec { name: "width", kind: AttrKind::NonNegativeInt },
];
const COLGROUP_ATTRS: &[AttrSpec] =
    &[AttrSpec { name: "span", kind: AttrKind::PositiveInt }];
const DATA_ATTRS: &[AttrSpec] = &[AttrSpec { name: "value", kind: AttrKind::Str }];
const DEL_INS_ATTRS: &[AttrSpec] = &[
    AttrSpec { name: "cite", kind: AttrKind::Str },
    AttrSpec { name: "datetime", kind: AttrKind::Datetime },
];
const DETAILS_ATTRS: &[AttrSpec] = &[
    AttrSpec { name: "name", kind: AttrKind::Str },
    AttrSpec { name: "open", kind: AttrKind::Presence },
];
const DIALOG_ATTRS: &[AttrSpec] = &[AttrSpec { name: "open", kind: AttrKind::Presence }];
const EMBED_ATTRS: &[AttrSpec] = &[
    AttrSpec { name: "height", kind: AttrKind::NonNegativeInt },
    AttrSpec { name: "src", kind: AttrKind::Str },
    AttrSpec { name: "type", kind: AttrKind::Str },
    AttrSpec { name: "width", kind: AttrKind::NonNegativeInt },
];
const FIELDSET_ATTRS: &[AttrSpec] = &[
    AttrSpec { name: "disabled", kind: AttrKind::Presence },
    AttrSpec { name: "form", kind: AttrKind::Str },
    AttrSpec { name: "name", kind: AttrKind::Str },
];
const FORM_ATTRS: &[AttrSpec] = &[
    AttrSpec {
        name: "accept-charset",
        kind: AttrKind::Enum(&["utf-8"]),
    },
    AttrSpec { name: "action", kind: AttrKind::Str },
    AttrSpec { name: "autocomplete", kind: AttrKind::OnOff },
    AttrSpec {
        name: "enctype",
        kind: AttrKind::Enum(FORM_ENCTYPES),
    },
    AttrSpec { name: "method", kind: AttrKind::Enum(FORM_METHODS) },
    AttrSpec { name: "name", kind: AttrKind::Str },
    AttrSpec { name: "novalidate", kind: AttrKind::Presence },
    AttrSpec { name: "rel", kind: AttrKind::EnumList(REL_TYPES) },
    AttrSpec {
        name: "target",
        kind: AttrKind::EnumOrStr(TARGET_TYPES),
    },
];
const IMG_ATTRS: &[AttrSpec] = &[
    AttrSpec { name: "alt", kind: AttrKind::Str },
    AttrSpec {
        name: "crossorigin",
        kind: AttrKind::Enum(CROSS_ORIGIN),
    },
    AttrSpec {
        name: "decoding",
        kind: AttrKind::Enum(&["auto", "sync", "async"]),
    },
    AttrSpec {
        name: "fetchpriority",
        kind: AttrKind::Enum(FETCH_PRIORITY),
    },
    AttrSpec { name: "height", kind: AttrKind::NonNegativeInt },
    AttrSpec { name: "ismap", kind: AttrKind::Presence },
    AttrSpec {
        name: "loading",
        kind: AttrKind::Enum(&["lazy", "eager"]),
    },
    AttrSpec {
        name: "referrerpolicy",
        kind: AttrKind::NoneEmptyOrEnum(REFERRER_POLICIES),
    },
    AttrSpec { name: "sizes", kind: AttrKind::SourceSizeList },
    AttrSpec { name: "src", kind: AttrKind::Str },
    AttrSpec { name: "srcset", kind: AttrKind::ImageCandidateList },
    AttrSpec { name: "usemap", kind: AttrKind::Str },
    AttrSpec { name: "width", kind: AttrKind::NonNegativeInt },
];
const INPUT_TYPES: &[&str] = &[
    "hidden",
    "text",
    "search",
    "tel",
    "url",
    "email",
    "password",
    "date",
    "month",
    "week",
    "time",
    "datetime-local",
    "number",
    "range",
    "color",
    "checkbox",
    "radio",
    "file",
    "submit",
    "image",
    "reset",
    "button",
];
const INPUT_ATTRS: &[AttrSpec] = &[
    AttrSpec { name: "accept", kind: AttrKind::StrListComma },
    AttrSpec { name: "alpha", kind: AttrKind::Presence },
    AttrSpec { name: "alt", kind: AttrKind::Str },
    AttrSpec {
        name: "autocomplete",
        kind: AttrKind::EnumList(AUTOCOMPLETE),
    },
    AttrSpec { name: "checked", kind: AttrKind::Presence },
    AttrSpec {
        name: "colorspace",
        kind: AttrKind::Enum(&["limited-srgb", "display-p3"]),
    },
    AttrSpec { name: "dirname", kind: AttrKind::Str },
    AttrSpec { name: "disabled", kind: AttrKind::Presence },
    AttrSpec { name: "form", kind: AttrKind::Str },
    AttrSpec { name: "formaction", kind: AttrKind::Str },
    AttrSpec {
        name: "formenctype",
        kind: AttrKind::Enum(FORM_ENCTYPES),
    },
    AttrSpec {
        name: "formmethod",
        kind: AttrKind::Enum(FORM_METHODS),
    },
    AttrSpec { name: "formnovalidate", kind: AttrKind::Presence },
    AttrSpec {
        name: "formtarget",
        kind: AttrKind::EnumOrStr(TARGET_TYPES),
    },
    AttrSpec { name: "height", kind: AttrKind::NonNegativeInt },
    AttrSpec { name: "list", kind: AttrKind::Str },
    AttrSpec { name: "max", kind: AttrKind::StrFloatDatetime },
    AttrSpec { name: "maxlength", kind: AttrKind::NonNegativeInt },
    AttrSpec { name: "min", kind: AttrKind::StrFloatDatetime },
    AttrSpec { name: "minlength", kind: AttrKind::NonNegativeInt },
    AttrSpec { name: "multiple", kind: AttrKind::Presence },
    AttrSpec { name: "name", kind: AttrKind::Str },
    AttrSpec { name: "pattern", kind: AttrKind::Str },
    AttrSpec { name: "placeholder", kind: AttrKind::Str },
    AttrSpec { name: "popovertarget", kind: AttrKind::Str },
    AttrSpec {
        name: "popovertargetaction",
        kind: AttrKind::Enum(POPOVER_TARGET_ACTIONS),
    },
    AttrSpec { name: "readonly", kind: AttrKind::Presence },
    AttrSpec { name: "required", kind: AttrKind::Presence },
    AttrSpec { name: "size", kind: AttrKind::PositiveInt },
    AttrSpec { name: "src", kind: AttrKind::Str },
    AttrSpec {
        name: "step",
        kind: AttrKind::PositiveFloatOr(&["any"]),
    },
    AttrSpec { name: "type", kind: AttrKind::Enum(INPUT_TYPES) },
    AttrSpec { name: "value", kind: AttrKind::InputValue },
    AttrSpec { name: "width", kind: AttrKind::NonNegativeInt },
];
const LABEL_ATTRS: &[AttrSpec] = &[AttrSpec { name: "for", kind: AttrKind::Str }];
const LINK_AS: &[&str] = &[
    "audio",
    "audioworklet",
    "document",
    "embed",
    "font",
    "frame",
    "iframe",
    "image",
    "json",
    "manifest",
    "object",
    "paintworklet",
    "report",
    "script",
    "serviceworker",
    "sharedworker",
    "style",
    "track",
    "video",
    "webidentity",
    "worker",
    "xslt",
];
const LINK_ATTRS: &[AttrSpec] = &[
    AttrSpec { name: "as", kind: AttrKind::Enum(LINK_AS) },
    AttrSpec {
        name: "blocking",
        kind: AttrKind::EnumList(BLOCKING),
    },
    AttrSpec { name: "color", kind: AttrKind::Color },
    AttrSpec {
        name: "crossorigin",
        kind: AttrKind::Enum(CROSS_ORIGIN),
    },
    AttrSpec { name: "disabled", kind: AttrKind::Presence },
    AttrSpec {
        name: "fetchpriority",
        kind: AttrKind::Enum(FETCH_PRIORITY),
    },
    AttrSpec { name: "href", kind: AttrKind::Str },
    AttrSpec { name: "hreflang", kind: AttrKind::Str },
    AttrSpec { name: "imagesizes", kind: AttrKind::SourceSizeList },
    AttrSpec {
        name: "imagesrcset",
        kind: AttrKind::ImageCandidateList,
    },
    AttrSpec { name: "integrity", kind: AttrKind::Str },
    AttrSpec { name: "media", kind: AttrKind::Str },
    AttrSpec {
        name: "referrerpolicy",
        kind: AttrKind::NoneEmptyOrEnum(REFERRER_POLICIES),
    },
    AttrSpec { name: "rel", kind: AttrKind::EnumList(REL_TYPES) },
    AttrSpec { name: "sizes", kind: AttrKind::IconSizeList },
    AttrSpec { name: "type", kind: AttrKind::Str },
];
const NAME_ATTRS: &[AttrSpec] = &[AttrSpec { name: "name", kind: AttrKind::Str }];
const META_ATTRS: &[AttrSpec] = &[
    AttrSpec { name: "charset", kind: AttrKind::Enum(&["utf-8"]) },
    AttrSpec { name: "content", kind: AttrKind::Str },
    AttrSpec {
        name: "http-equiv",
        kind: AttrKind::Enum(&[
            "content-type",
            "default-style",
            "refresh",
            "x-ua-compatible",
            "content-security-policy",
        ]),
    },
    AttrSpec { name: "media", kind: AttrKind::Str },
    AttrSpec { name: "name", kind: AttrKind::Str },
];
const METER_ATTRS: &[AttrSpec] = &[
    AttrSpec { name: "high", kind: AttrKind::Float },
    AttrSpec { name: "low", kind: AttrKind::Float },
    AttrSpec { name: "max", kind: AttrKind::Float },
    AttrSpec { name: "min", kind: AttrKind::Float },
    AttrSpec { name: "optimum", kind: AttrKind::Float },
    AttrSpec { name: "value", kind: AttrKind::Float },
];
const OBJECT_ATTRS: &[AttrSpec] = &[
    AttrSpec { name: "data", kind: AttrKind::Str },
    AttrSpec { name: "form", kind: AttrKind::Str },
    AttrSpec { name: "height", kind: AttrKind::NonNegativeInt },
    AttrSpec {
        name: "name",
        kind: AttrKind::EnumOrStr(TARGET_TYPES),
    },
    AttrSpec { name: "type", kind: AttrKind::Str },
    AttrSpec { name: "width", kind: AttrKind::NonNegativeInt },
];
const OPTGROUP_ATTRS: &[AttrSpec] = &[
    AttrSpec { name: "disabled", kind: AttrKind::Presence },
    AttrSpec { name: "label", kind: AttrKind::Str },
];
const OPTION_ATTRS: &[AttrSpec] = &[
    AttrSpec { name: "disabled", kind: AttrKind::Presence },
    AttrSpec { name: "label", kind: AttrKind::Str },
    AttrSpec { name: "selected", kind: AttrKind::Presence },
    AttrSpec { name: "value", kind: AttrKind::Str },
];
const OUTPUT_ATTRS: &[AttrSpec] = &[
    AttrSpec { name: "for", kind: AttrKind::ListStr },
    AttrSpec { name: "form", kind: AttrKind::Str },
    AttrSpec { name: "name", kind: AttrKind::Str },
];
const PROGRESS_ATTRS: &[AttrSpec] = &[
    AttrSpec { name: "max", kind: AttrKind::Float },
    AttrSpec { name: "value", kind: AttrKind::Float },
];
const SCRIPT_ATTRS: &[AttrSpec] = &[
    AttrSpec { name: "async", kind: AttrKind::Presence },
    AttrSpec {
        name: "blocking",
        kind: AttrKind::EnumList(BLOCKING),
    },
    AttrSpec {
        name: "crossorigin",
        kind: AttrKind::Enum(CROSS_ORIGIN),
    },
    AttrSpec { name: "defer", kind: AttrKind::Presence },
    AttrSpec {
        name: "fetchpriority",
        kind: AttrKind::Enum(FETCH_PRIORITY),
    },
    AttrSpec { name: "integrity", kind: AttrKind::Str },
    AttrSpec { name: "nomodule", kind: AttrKind::Presence },
    AttrSpec {
        name: "referrerpolicy",
        kind: AttrKind::NoneEmptyOrEnum(REFERRER_POLICIES),
    },
    AttrSpec { name: "src", kind: AttrKind::Str },
    AttrSpec {
        name: "type",
        kind: AttrKind::EnumOrStr(&["module"]),
    },
];
const SOURCE_ATTRS: &[AttrSpec] = &[
    AttrSpec { name: "height", kind: AttrKind::NonNegativeInt },
    AttrSpec { name: "media", kind: AttrKind::Str },
    AttrSpec { name: "sizes", kind: AttrKind::SourceSizeList },
    AttrSpec { name: "src", kind: AttrKind::Str },
    AttrSpec { name: "srcset", kind: AttrKind::ImageCandidateList },
    AttrSpec { name: "type", kind: AttrKind::Str },
    AttrSpec { name: "width", kind: AttrKind::NonNegativeInt },
];
const STYLE_ATTRS: &[AttrSpec] = &[
    AttrSpec {
        name: "blocking",
        kind: AttrKind::EnumList(BLOCKING),
    },
    AttrSpec { name: "media", kind: AttrKind::Str },
];
const TD_ATTRS: &[AttrSpec] = &[
    AttrSpec { name: "colspan", kind: AttrKind::PositiveInt },
    AttrSpec { name: "headers", kind: AttrKind::ListStr },
    AttrSpec { name: "rowspan", kind: AttrKind::NonNegativeInt },
];
const TEXTAREA_ATTRS: &[AttrSpec] = &[
    AttrSpec {
        name: "autocomplete",
        kind: AttrKind::EnumList(AUTOCOMPLETE),
    },
    AttrSpec { name: "cols", kind: AttrKind::PositiveInt },
    AttrSpec { name: "dirname", kind: AttrKind::Str },
    AttrSpec { name: "disabled", kind: AttrKind::Presence },
    AttrSpec { name: "form", kind: AttrKind::Str },
    AttrSpec { name: "maxlength", kind: AttrKind::NonNegativeInt },
    AttrSpec { name: "minlength", kind: AttrKind::NonNegativeInt },
    AttrSpec { name: "name", kind: AttrKind::Str },
    AttrSpec { name: "placeholder", kind: AttrKind::Str },
    AttrSpec { name: "readonly", kind: AttrKind::Presence },
    AttrSpec { name: "required", kind: AttrKind::Presence },
    AttrSpec { name: "rows", kind: AttrKind::PositiveInt },
    AttrSpec {
        name: "wrap",
        kind: AttrKind::Enum(&["soft", "hard"]),
    },
];
const TH_ATTRS: &[AttrSpec] = &[
    AttrSpec { name: "abbr", kind: AttrKind::Str },
    AttrSpec { name: "colspan", kind: AttrKind::PositiveInt },
    AttrSpec { name: "headers", kind: AttrKind::ListStr },
    AttrSpec { name: "rowspan", kind: AttrKind::NonNegativeInt },
    AttrSpec {
        name: "scope",
        kind: AttrKind::Enum(&["row", "col", "rowgroup", "colgroup"]),
    },
];
const TIME_ATTRS: &[AttrSpec] = &[AttrSpec {
    name: "datetime",
    kind: AttrKind::DurationOrDatetime,
}];
const TRACK_ATTRS: &[AttrSpec] = &[
    AttrSpec { name: "default", kind: AttrKind::Presence },
    AttrSpec {
        name: "kind",
        kind: AttrKind::Enum(&[
            "subtitles",
            "captions",
            "descriptions",
            "chapters",
            "metadata",
        ]),
    },
    AttrSpec { name: "label", kind: AttrKind::Str },
    AttrSpec { name: "src", kind: AttrKind::Str },
    AttrSpec { name: "srclang", kind: AttrKind::Str },
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

fn native_typed_html_raw(
    tag: &str,
    specific_attrs: &[AttrSpec],
    args: &Args,
) -> SourceResult<Value> {
    let body = match args.items.as_slice() {
        [] => None,
        [Value::Str(body)] => Some(Content::text(body.clone())),
        [other] => return type_error(args.span, "string", other),
        [_, ..] => {
            return Err(vec![SourceDiagnostic::error(args.span, "unexpected argument")])
        }
    };
    let attrs = collect_typed_attrs(specific_attrs, args)?;
    Ok(Value::Content(Content::HtmlElem(Arc::new(HtmlElem::new(
        tag.into(),
        (!attrs.is_empty()).then_some(attrs),
        body.map_or(HtmlBody::None, |body| HtmlBody::Content(Box::new(body))),
    )))))
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

    let attrs = collect_typed_attrs(specific_attrs, args)?;

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

fn collect_typed_attrs(
    specific_attrs: &[AttrSpec],
    args: &Args,
) -> SourceResult<HtmlAttrs> {
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
    Ok(attrs)
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
        AttrKind::PositiveInt => match value {
            Value::Int(value) if *value > 0 => value.to_string().into(),
            Value::Int(_) => {
                return Err(vec![SourceDiagnostic::error(
                    span,
                    "number must be positive",
                )])
            }
            other => return type_error(span, "integer", other),
        },
        AttrKind::NonNegativeInt => match value {
            Value::Int(value) if *value >= 0 => value.to_string().into(),
            Value::Int(_) => {
                return Err(vec![SourceDiagnostic::error(
                    span,
                    "number must be at least zero",
                )])
            }
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
                    format!(
                        "expected {values}, or string, found {}",
                        vanilla_type_name(other)
                    ),
                )]);
            }
        },
        AttrKind::NoneEmptyOrEnum(values) => match value {
            Value::None => "".into(),
            Value::Str(value) if values.contains(&value.as_str()) => value.clone(),
            Value::Str(_) => {
                return Err(vec![SourceDiagnostic::error(
                    span,
                    expected_none_or_enum(values),
                )])
            }
            other => {
                return Err(vec![SourceDiagnostic::error(
                    span,
                    format!(
                        "{}, found {}",
                        expected_none_or_enum(values),
                        vanilla_type_name(other)
                    ),
                )])
            }
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
        AttrKind::Preload => match value {
            Value::None => "none".into(),
            Value::Auto => "auto".into(),
            Value::Str(value) if value == "metadata" => value.clone(),
            Value::Str(_) => {
                return Err(vec![SourceDiagnostic::error(
                    span,
                    "expected none, auto, or \"metadata\"",
                )])
            }
            other => {
                return Err(vec![SourceDiagnostic::error(
                    span,
                    format!(
                        "expected none, auto, or \"metadata\", found {}",
                        other.type_name()
                    ),
                )])
            }
        },
        AttrKind::NoneOrRole => match value {
            Value::None => "none".into(),
            _ => require_enum(value, span, ROLES)?.into(),
        },
        AttrKind::YesNo => bool_string(value, span, "yes", "no")?.into(),
        AttrKind::Datetime => cast_datetime(value, span)?,
        AttrKind::DurationOrDatetime => match value {
            Value::Duration(value) => cast_duration(*value)?.into(),
            Value::Datetime(_) => cast_datetime(value, span)?,
            other => return union_error(span, "duration or datetime", other),
        },
        AttrKind::Color => match value {
            Value::Color(value) => value.to_hex().into(),
            other => return type_error(span, "color", other),
        },
        AttrKind::FloatListComma => cast_float_list(value, span, false)?.into(),
        AttrKind::StrListComma => cast_string_list(value, span, ",", true)?.into(),
        AttrKind::StrFloatDatetime => match value {
            Value::Str(value) => value.clone(),
            Value::Float(value) => value.to_string().into(),
            Value::Int(value) => value.to_string().into(),
            Value::Datetime(_) => cast_datetime(value, span)?,
            other => return union_error(span, "string, float, or datetime", other),
        },
        AttrKind::PositiveFloatOr(values) => match value {
            Value::Float(_) | Value::Int(_) => {
                cast_positive_float(value, span)?.to_string().into()
            }
            _ => require_enum(value, span, values)?.into(),
        },
        AttrKind::InputValue => match value {
            Value::Str(value) => value.clone(),
            Value::Float(value) => value.to_string().into(),
            Value::Int(value) => value.to_string().into(),
            Value::Datetime(_) => cast_datetime(value, span)?,
            Value::Color(value) => value.to_hex().into(),
            Value::Array(_) => cast_string_list(value, span, ",", false)?.into(),
            other => {
                return union_error(
                    span,
                    "string, float, datetime, color, or array",
                    other,
                )
            }
        },
        AttrKind::SourceSizeList => {
            cast_complex_list(value, span, cast_source_size, ",")?.into()
        }
        AttrKind::ImageCandidateList => {
            cast_complex_list(value, span, cast_image_candidate, ",")?.into()
        }
        AttrKind::IconSizeList => {
            cast_complex_list(value, span, cast_icon_size, " ")?.into()
        }
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
            format!("{}, found {}", expected_enum(values), vanilla_type_name(value)),
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

fn expected_none_or_enum(values: &[&str]) -> String {
    let quoted = values.iter().map(|value| format!("{value:?}")).collect::<Vec<_>>();
    if quoted.is_empty() {
        "expected none".into()
    } else {
        format!("expected {}, or none", quoted.join(", "))
    }
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
    let is_array = matches!(value, Value::Array(_));
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
        if is_array && item.contains(' ') {
            return Err(vec![SourceDiagnostic::error(
                span,
                "array item may not contain a space",
            )]);
        }
        out.push(item);
    }
    Ok(out.join(" "))
}

fn cast_positive_float(value: &Value, span: Span) -> SourceResult<f64> {
    let number = match value {
        Value::Float(value) => *value,
        Value::Int(value) => *value as f64,
        other => return type_error(span, "float", other),
    };
    if number > 0.0 {
        Ok(number)
    } else {
        Err(vec![SourceDiagnostic::error(span, "number must be positive")])
    }
}

fn cast_datetime(value: &Value, span: Span) -> SourceResult<EcoString> {
    let Value::Datetime(value) = value else {
        return type_error(span, "datetime", value);
    };
    let date = match (value.year(), value.month(), value.day()) {
        (Some(year), Some(month), Some(day)) => {
            Some(format!("{year:04}-{month:02}-{day:02}"))
        }
        _ => None,
    };
    let time = match (value.hour(), value.minute(), value.second()) {
        (Some(hour), Some(minute), Some(0)) => Some(format!("{hour:02}:{minute:02}")),
        (Some(hour), Some(minute), Some(second)) => {
            Some(format!("{hour:02}:{minute:02}:{second:02}"))
        }
        _ => None,
    };
    match (date, time) {
        (Some(date), Some(time)) => Ok(format!("{date}T{time}").into()),
        (Some(date), None) => Ok(date.into()),
        (None, Some(time)) => Ok(time.into()),
        (None, None) => Err(vec![SourceDiagnostic::error(span, "invalid datetime")]),
    }
}

fn cast_duration(value: crate::entities::duration::Duration) -> SourceResult<String> {
    const SECOND: i128 = 1_000_000_000;
    const MINUTE: i128 = 60 * SECOND;
    const HOUR: i128 = 60 * MINUTE;
    const DAY: i128 = 24 * HOUR;
    const WEEK: i128 = 7 * DAY;

    let negative = value.nanos < 0;
    let mut remaining = value.nanos.abs();
    let weeks = remaining / WEEK;
    remaining %= WEEK;
    let days = remaining / DAY;
    remaining %= DAY;
    let hours = remaining / HOUR;
    remaining %= HOUR;
    let minutes = remaining / MINUTE;
    remaining %= MINUTE;
    let seconds = remaining as f64 / SECOND as f64;
    let mut parts = Vec::new();
    if weeks > 0 {
        parts.push(format!("{weeks}w"));
    }
    if days > 0 {
        parts.push(format!("{days}d"));
    }
    if hours > 0 {
        parts.push(format!("{hours}h"));
    }
    if minutes > 0 {
        parts.push(format!("{minutes}m"));
    }
    if seconds > 0.0 || parts.is_empty() {
        parts.push(format!("{seconds}s"));
    }
    let rendered = parts.join(" ");
    Ok(if negative { format!("-{rendered}") } else { rendered })
}

fn cast_float_list(value: &Value, span: Span, shorthand: bool) -> SourceResult<String> {
    let items: &[Value] = match value {
        Value::Array(items) => items,
        value if shorthand => std::slice::from_ref(value),
        other => return type_error(span, "array", other),
    };
    items
        .iter()
        .map(|item| match item {
            Value::Float(value) => Ok(value.to_string()),
            Value::Int(value) => Ok(value.to_string()),
            other => type_error(span, "float", other),
        })
        .collect::<SourceResult<Vec<_>>>()
        .map(|items| items.join(", "))
}

fn cast_string_list(
    value: &Value,
    span: Span,
    separator: &str,
    shorthand: bool,
) -> SourceResult<String> {
    let items: &[Value] = match value {
        Value::Array(items) => items,
        value if shorthand => std::slice::from_ref(value),
        other => return type_error(span, "array", other),
    };
    let items = items
        .iter()
        .map(|item| require_str(item, span).map(str::to_owned))
        .collect::<SourceResult<Vec<_>>>()?;
    if matches!(value, Value::Array(_))
        && items.iter().any(|item| item.contains(separator))
    {
        return Err(vec![SourceDiagnostic::error(
            span,
            if separator == "," {
                "array item may not contain a comma"
            } else {
                "array item may not contain a space"
            },
        )]);
    }
    Ok(items.join(if separator == "," { ", " } else { separator }))
}

fn cast_complex_list(
    value: &Value,
    span: Span,
    cast: fn(&Value, Span) -> SourceResult<String>,
    separator: &str,
) -> SourceResult<String> {
    let Value::Array(items) = value else {
        return type_error(span, "array", value);
    };
    let items = items
        .iter()
        .map(|item| cast(item, span))
        .collect::<SourceResult<Vec<_>>>()?;
    if items.iter().any(|item| item.contains(separator)) {
        return Err(vec![SourceDiagnostic::error(
            span,
            if separator == "," {
                "array item may not contain a comma"
            } else {
                "array item may not contain a space"
            },
        )]);
    }
    Ok(items.join(if separator == "," { ", " } else { separator }))
}

fn cast_icon_size(value: &Value, span: Span) -> SourceResult<String> {
    let Value::Array(items) = value else {
        return type_error(span, "array", value);
    };
    let [Value::Int(width), Value::Int(height)] = items.as_slice() else {
        return Err(vec![SourceDiagnostic::error(
            span,
            "expected an array of two integers",
        )]);
    };
    if *width < 0 || *height < 0 {
        return Err(vec![SourceDiagnostic::error(span, "number must be at least zero")]);
    }
    Ok(format!("{width}x{height}"))
}

fn cast_image_candidate(value: &Value, span: Span) -> SourceResult<String> {
    let Value::Dict(dict) = value else {
        return type_error(span, "dictionary", value);
    };
    if dict
        .keys()
        .any(|key| !matches!(key.as_str(), "src" | "width" | "density"))
    {
        return Err(vec![SourceDiagnostic::error(span, "unexpected key in dictionary")]);
    }
    let Some(Value::Str(src)) = dict.get("src") else {
        return Err(vec![SourceDiagnostic::error(span, "expected string at key `src`")]);
    };
    if src.is_empty() {
        return Err(vec![SourceDiagnostic::error(span, "`src` must not be empty")]);
    }
    if src.starts_with(',') || src.ends_with(',') {
        return Err(vec![SourceDiagnostic::error(
            span,
            "`src` must not start or end with a comma",
        )]);
    }
    let width = match dict.get("width") {
        Some(Value::Int(value)) if *value > 0 => Some(*value),
        Some(Value::Int(_)) => {
            return Err(vec![SourceDiagnostic::error(span, "number must be positive")])
        }
        Some(other) => return type_error(span, "integer", other),
        None => None,
    };
    let density = match dict.get("density") {
        Some(value) => Some(cast_positive_float(value, span)?),
        None => None,
    };
    match (width, density) {
        (Some(_), Some(_)) => Err(vec![SourceDiagnostic::error(
            span,
            "cannot specify both `width` and `density`",
        )]),
        (Some(width), None) => Ok(format!("{src} {width}w")),
        (None, Some(density)) => Ok(format!("{src} {density}d")),
        (None, None) => Ok(src.to_string()),
    }
}

fn cast_source_size(value: &Value, span: Span) -> SourceResult<String> {
    let Value::Dict(dict) = value else {
        return type_error(span, "dictionary", value);
    };
    if dict.keys().any(|key| !matches!(key.as_str(), "condition" | "size")) {
        return Err(vec![SourceDiagnostic::error(span, "unexpected key in dictionary")]);
    }
    let Some(Value::Str(condition)) = dict.get("condition") else {
        return Err(vec![SourceDiagnostic::error(
            span,
            "expected string at key `condition`",
        )]);
    };
    let Some(Value::Length(size)) = dict.get("size") else {
        return Err(vec![SourceDiagnostic::error(span, "expected length at key `size`")]);
    };
    Ok(format!(
        "({condition}) {}",
        crate::compiler::stdlib::foundations::str::format_length(size)
    ))
}

fn type_error<T>(span: Span, expected: &str, found: &Value) -> SourceResult<T> {
    Err(vec![SourceDiagnostic::error(
        span,
        format!("expected {expected}, found {}", vanilla_type_name(found)),
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
