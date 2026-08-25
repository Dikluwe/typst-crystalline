//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/html.md
//! @prompt-hash 67bf7944
//! @layer L1

use std::sync::Arc;

use ecow::EcoString;

use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::file_id::FileId;
use crate::entities::func::Func;
use crate::entities::html::{HtmlAttrs, HtmlElem};
use crate::entities::module::Module;
use crate::entities::scope::Scope;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;

pub fn make_html_module() -> Module {
    let mut scope = Scope::new();
    scope.define("elem", Value::Func(Func::native("elem", native_html_elem)));
    Module::new("html", scope)
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

    Ok(Value::Content(Content::HtmlElem(Arc::new(HtmlElem::new(tag, attrs, body)))))
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
