//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/structural/title.md
//! @prompt-hash a6498fd8
//! @layer L1
//! @updated 2026-08-13
//!
//! `title` — título do documento (lê `document_info`).
//!
//! Separado de `structural/sectioning.rs` em 2026-08-13: a fronteira anterior
//! estava sustentada por um cluster de co-mudança que não existia (artefacto
//! de atribuição da ferramenta). Fronteira nova medida no L0 deste nó.

use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::file_id::FileId;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;


/// `title(body?)` — emite `Content::Title`.
///
/// **P765a**: paridade com vanilla CLI 0.15.0. Aceita body posicional
/// (content ou string) ou named `body`. Se omitido, usa o metadado
/// `document.title` definido por `#set document(title: ...)`. Se este
/// também estiver ausente, produz erro.
pub fn native_title(
    ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    // Named `body` tem prioridade sobre posicional (paridade vanilla).
    let body_named = args.named.get("body").and_then(|v| match v {
        Value::Content(c) => Some(Ok(c.clone())),
        Value::Str(s) => Some(Ok(Content::text(s.as_str()))),
        Value::None => None,
        other => Some(Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!(
                "title(body:): espera content ou string, recebeu {}",
                other.type_name()
            ),
        )])),
    });

    let body_positional = if args.items.is_empty() {
        None
    } else {
        match args.items.first() {
            Some(Value::Content(c)) => Some(Ok(c.clone())),
            Some(Value::Str(s)) => Some(Ok(Content::text(s.as_str()))),
            Some(other) => Some(Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "title(): body espera content ou string, recebeu {}",
                    other.type_name()
                ),
            )])),
            None => None,
        }
    };

    let body =
        match (body_named, body_positional) {
            (Some(Ok(_)), Some(Ok(_))) => return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "title(): não pode usar body posicional e named `body` simultaneamente"
                    .to_string(),
            )]),
            (Some(Ok(b)), _) => b,
            (Some(Err(e)), _) => return Err(e),
            (None, Some(Ok(b))) => b,
            (None, Some(Err(e))) => return Err(e),
            (None, None) => match &ctx.document_info.title {
                Some(t) => Content::text(t.as_str()),
                None => {
                    return Err(vec![SourceDiagnostic::error(
                        Span::detached(),
                        "title() exige body ou metadado document.title".to_string(),
                    )])
                }
            },
        };

    Ok(Value::Content(Content::Title(std::sync::Arc::new(
        crate::entities::elements::title::TitleElem::new(body),
    ))))
}
