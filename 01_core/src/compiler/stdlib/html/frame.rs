//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/html/frame.md
//! @prompt-hash 5bbc2392
//! @layer L1
//! @updated 2026-09-30

use super::*;

pub(in crate::compiler) fn native_html_frame(
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
