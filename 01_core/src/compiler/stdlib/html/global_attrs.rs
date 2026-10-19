//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/html/global_attrs.md
//! @prompt-hash 6811f0cd
//! @layer L1
//! @updated 2026-09-30

use super::*;

pub(super) const MIXED: &[&str] = &["mixed"];
pub(super) const AUTOCAPITALIZE: &[&str] = &["sentences", "words", "characters"];
pub(super) const ENTERKEYHINT: &[&str] =
    &["enter", "done", "go", "next", "previous", "search", "send"];
pub(super) const INPUTMODE: &[&str] =
    &["text", "tel", "email", "url", "numeric", "decimal", "search"];
pub(super) const ARIA_AUTOCOMPLETE: &[&str] = &["inline", "list", "both"];
pub(super) const ARIA_CURRENT: &[&str] = &["page", "step", "location", "date", "time"];
pub(super) const ARIA_HASPOPUP: &[&str] = &["menu", "listbox", "tree", "grid", "dialog"];
pub(super) const ARIA_INVALID: &[&str] = &["grammar", "spelling"];
pub(super) const ARIA_LIVE: &[&str] = &["assertive", "off", "polite"];
pub(super) const ARIA_ORIENTATION: &[&str] = &["horizontal", "undefined", "vertical"];
pub(super) const ARIA_RELEVANT: &[&str] =
    &["additions", "additions text", "all", "removals", "text"];
pub(super) const ARIA_SORT: &[&str] = &["ascending", "descending", "other"];
pub(super) const ROLES: &[&str] = &[
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

pub(super) const GLOBAL_ATTRS: &[AttrSpec] = &[
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
