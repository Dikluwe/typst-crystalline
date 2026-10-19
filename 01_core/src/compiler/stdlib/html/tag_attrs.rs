//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/html/tag_attrs.md
//! @prompt-hash bca8f37c
//! @layer L1
//! @updated 2026-09-30

use super::*;

pub(super) const OL_TYPES: &[&str] = &["1", "a", "A", "i", "I"];
pub(super) const OL_ATTRS: &[AttrSpec] = &[
    AttrSpec { name: "reversed", kind: AttrKind::Presence },
    AttrSpec { name: "start", kind: AttrKind::Int },
    AttrSpec { name: "type", kind: AttrKind::Enum(OL_TYPES) },
];
pub(super) const LI_ATTRS: &[AttrSpec] =
    &[AttrSpec { name: "value", kind: AttrKind::Int }];

pub(super) const REFERRER_POLICIES: &[&str] = &[
    "no-referrer",
    "no-referrer-when-downgrade",
    "same-origin",
    "origin",
    "strict-origin",
    "origin-when-cross-origin",
    "strict-origin-when-cross-origin",
    "unsafe-url",
];
pub(super) const REL_TYPES: &[&str] = &[
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
pub(super) const TARGET_TYPES: &[&str] = &["_blank", "_self", "_parent", "_top"];
pub(super) const A_ATTRS: &[AttrSpec] = &[
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

pub(super) const BUTTON_COMMANDS: &[&str] = &[
    "toggle-popover",
    "show-popover",
    "hide-popover",
    "close",
    "request-close",
    "show-modal",
];
pub(super) const BUTTON_ENCTYPES: &[&str] =
    &["application/x-www-form-urlencoded", "multipart/form-data", "text/plain"];
pub(super) const BUTTON_METHODS: &[&str] = &["GET", "POST", "dialog"];
pub(super) const POPOVER_TARGET_ACTIONS: &[&str] = &["toggle", "show", "hide"];
pub(super) const BUTTON_TYPES: &[&str] = &["submit", "reset", "button"];
pub(super) const BUTTON_ATTRS: &[AttrSpec] = &[
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

pub(super) const COL_ATTRS: &[AttrSpec] =
    &[AttrSpec { name: "span", kind: AttrKind::PositiveInt }];

pub(super) const IFRAME_LOADING: &[&str] = &["lazy", "eager"];
pub(super) const SANDBOX: &[&str] = &[
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
pub(super) const IFRAME_ATTRS: &[AttrSpec] = &[
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

pub(super) const AUTOCOMPLETE: &[&str] = &[
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
pub(super) const SELECT_ATTRS: &[AttrSpec] = &[
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

pub(super) const TEMPLATE_ATTRS: &[AttrSpec] = &[
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

pub(super) const VIDEO_ATTRS: &[AttrSpec] = &[
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

pub(super) const WBR_ATTRS: &[AttrSpec] = &[];

pub(super) const NO_ATTRS: &[AttrSpec] = &[];
pub(super) const CROSS_ORIGIN: &[&str] = &["anonymous", "use-credentials"];
pub(super) const FETCH_PRIORITY: &[&str] = &["auto", "high", "low"];
pub(super) const FORM_ENCTYPES: &[&str] =
    &["application/x-www-form-urlencoded", "multipart/form-data", "text/plain"];
pub(super) const FORM_METHODS: &[&str] = &["GET", "POST", "dialog"];
pub(super) const BLOCKING: &[&str] = &["blocking"];

pub(super) const AREA_ATTRS: &[AttrSpec] = &[
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
pub(super) const AUDIO_ATTRS: &[AttrSpec] = &[
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
pub(super) const BASE_ATTRS: &[AttrSpec] = &[
    AttrSpec { name: "href", kind: AttrKind::Str },
    AttrSpec {
        name: "target",
        kind: AttrKind::EnumOrStr(TARGET_TYPES),
    },
];
pub(super) const CITE_ATTRS: &[AttrSpec] =
    &[AttrSpec { name: "cite", kind: AttrKind::Str }];
pub(super) const CANVAS_ATTRS: &[AttrSpec] = &[
    AttrSpec { name: "height", kind: AttrKind::NonNegativeInt },
    AttrSpec { name: "width", kind: AttrKind::NonNegativeInt },
];
pub(super) const COLGROUP_ATTRS: &[AttrSpec] =
    &[AttrSpec { name: "span", kind: AttrKind::PositiveInt }];
pub(super) const DATA_ATTRS: &[AttrSpec] =
    &[AttrSpec { name: "value", kind: AttrKind::Str }];
pub(super) const DEL_INS_ATTRS: &[AttrSpec] = &[
    AttrSpec { name: "cite", kind: AttrKind::Str },
    AttrSpec { name: "datetime", kind: AttrKind::Datetime },
];
pub(super) const DETAILS_ATTRS: &[AttrSpec] = &[
    AttrSpec { name: "name", kind: AttrKind::Str },
    AttrSpec { name: "open", kind: AttrKind::Presence },
];
pub(super) const DIALOG_ATTRS: &[AttrSpec] =
    &[AttrSpec { name: "open", kind: AttrKind::Presence }];
pub(super) const EMBED_ATTRS: &[AttrSpec] = &[
    AttrSpec { name: "height", kind: AttrKind::NonNegativeInt },
    AttrSpec { name: "src", kind: AttrKind::Str },
    AttrSpec { name: "type", kind: AttrKind::Str },
    AttrSpec { name: "width", kind: AttrKind::NonNegativeInt },
];
pub(super) const FIELDSET_ATTRS: &[AttrSpec] = &[
    AttrSpec { name: "disabled", kind: AttrKind::Presence },
    AttrSpec { name: "form", kind: AttrKind::Str },
    AttrSpec { name: "name", kind: AttrKind::Str },
];
pub(super) const FORM_ATTRS: &[AttrSpec] = &[
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
pub(super) const IMG_ATTRS: &[AttrSpec] = &[
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
pub(super) const INPUT_TYPES: &[&str] = &[
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
pub(super) const INPUT_ATTRS: &[AttrSpec] = &[
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
pub(super) const LABEL_ATTRS: &[AttrSpec] =
    &[AttrSpec { name: "for", kind: AttrKind::Str }];
pub(super) const LINK_AS: &[&str] = &[
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
pub(super) const LINK_ATTRS: &[AttrSpec] = &[
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
pub(super) const NAME_ATTRS: &[AttrSpec] =
    &[AttrSpec { name: "name", kind: AttrKind::Str }];
pub(super) const META_ATTRS: &[AttrSpec] = &[
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
pub(super) const METER_ATTRS: &[AttrSpec] = &[
    AttrSpec { name: "high", kind: AttrKind::Float },
    AttrSpec { name: "low", kind: AttrKind::Float },
    AttrSpec { name: "max", kind: AttrKind::Float },
    AttrSpec { name: "min", kind: AttrKind::Float },
    AttrSpec { name: "optimum", kind: AttrKind::Float },
    AttrSpec { name: "value", kind: AttrKind::Float },
];
pub(super) const OBJECT_ATTRS: &[AttrSpec] = &[
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
pub(super) const OPTGROUP_ATTRS: &[AttrSpec] = &[
    AttrSpec { name: "disabled", kind: AttrKind::Presence },
    AttrSpec { name: "label", kind: AttrKind::Str },
];
pub(super) const OPTION_ATTRS: &[AttrSpec] = &[
    AttrSpec { name: "disabled", kind: AttrKind::Presence },
    AttrSpec { name: "label", kind: AttrKind::Str },
    AttrSpec { name: "selected", kind: AttrKind::Presence },
    AttrSpec { name: "value", kind: AttrKind::Str },
];
pub(super) const OUTPUT_ATTRS: &[AttrSpec] = &[
    AttrSpec { name: "for", kind: AttrKind::ListStr },
    AttrSpec { name: "form", kind: AttrKind::Str },
    AttrSpec { name: "name", kind: AttrKind::Str },
];
pub(super) const PROGRESS_ATTRS: &[AttrSpec] = &[
    AttrSpec { name: "max", kind: AttrKind::Float },
    AttrSpec { name: "value", kind: AttrKind::Float },
];
pub(super) const SCRIPT_ATTRS: &[AttrSpec] = &[
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
pub(super) const SOURCE_ATTRS: &[AttrSpec] = &[
    AttrSpec { name: "height", kind: AttrKind::NonNegativeInt },
    AttrSpec { name: "media", kind: AttrKind::Str },
    AttrSpec { name: "sizes", kind: AttrKind::SourceSizeList },
    AttrSpec { name: "src", kind: AttrKind::Str },
    AttrSpec { name: "srcset", kind: AttrKind::ImageCandidateList },
    AttrSpec { name: "type", kind: AttrKind::Str },
    AttrSpec { name: "width", kind: AttrKind::NonNegativeInt },
];
pub(super) const STYLE_ATTRS: &[AttrSpec] = &[
    AttrSpec {
        name: "blocking",
        kind: AttrKind::EnumList(BLOCKING),
    },
    AttrSpec { name: "media", kind: AttrKind::Str },
];
pub(super) const TD_ATTRS: &[AttrSpec] = &[
    AttrSpec { name: "colspan", kind: AttrKind::PositiveInt },
    AttrSpec { name: "headers", kind: AttrKind::ListStr },
    AttrSpec { name: "rowspan", kind: AttrKind::NonNegativeInt },
];
pub(super) const TEXTAREA_ATTRS: &[AttrSpec] = &[
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
pub(super) const TH_ATTRS: &[AttrSpec] = &[
    AttrSpec { name: "abbr", kind: AttrKind::Str },
    AttrSpec { name: "colspan", kind: AttrKind::PositiveInt },
    AttrSpec { name: "headers", kind: AttrKind::ListStr },
    AttrSpec { name: "rowspan", kind: AttrKind::NonNegativeInt },
    AttrSpec {
        name: "scope",
        kind: AttrKind::Enum(&["row", "col", "rowgroup", "colgroup"]),
    },
];
pub(super) const TIME_ATTRS: &[AttrSpec] = &[AttrSpec {
    name: "datetime",
    kind: AttrKind::DurationOrDatetime,
}];
pub(super) const TRACK_ATTRS: &[AttrSpec] = &[
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
