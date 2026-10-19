//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/html/typed_element.md
//! @prompt-hash ec8fcb8b
//! @layer L1
//! @updated 2026-09-30

use super::*;

pub(super) fn native_typed_html(
    tag: &str,
    specific_attrs: &[AttrSpec],
    args: &Args,
) -> SourceResult<Value> {
    native_typed_html_impl(tag, specific_attrs, args, true)
}

pub(super) fn native_typed_html_void(
    tag: &str,
    specific_attrs: &[AttrSpec],
    args: &Args,
) -> SourceResult<Value> {
    native_typed_html_impl(tag, specific_attrs, args, false)
}

pub(super) fn native_typed_html_raw(
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

pub(super) fn native_typed_html_impl(
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

pub(super) fn collect_typed_attrs(
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
