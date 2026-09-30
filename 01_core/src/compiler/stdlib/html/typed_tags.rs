//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/html/typed_tags.md
//! @prompt-hash 300c6566
//! @layer L1
//! @updated 2026-09-30

use super::*;

pub(super) type NativeHtmlFunc = fn(
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

pub(super) fn native_html_ol(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    native_typed_html("ol", OL_ATTRS, args)
}

pub(super) fn native_html_li(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    native_typed_html("li", LI_ATTRS, args)
}

pub(super) fn native_html_a(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    native_typed_html("a", A_ATTRS, args)
}

pub(super) fn native_html_br(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    native_typed_html_void("br", &[], args)
}

pub(super) fn native_html_col(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    native_typed_html_void("col", COL_ATTRS, args)
}

pub(super) fn native_html_wbr(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    native_typed_html_void("wbr", WBR_ATTRS, args)
}

pub(super) const TYPED_TAGS: &[(&str, NativeHtmlFunc)] = &[
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
