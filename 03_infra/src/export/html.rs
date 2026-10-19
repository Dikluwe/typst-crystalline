//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/html.md
//! @prompt-hash 8460c58e
//! @layer L3
//! @updated 2026-09-26
//!
//! Fachada pública da exportação semântica HTML.

use typst_core::entities::content::Content;
use typst_core::entities::source_result::SourceDiagnostic;

mod document;
mod encoding;
mod inline;
mod pretty;
#[cfg(test)]
mod pretty_serialization_tests;
mod table;
mod tags;
#[cfg(test)]
mod tests;
mod whitespace;

use document::export_html_with_frame_resolver;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HtmlSerializationMode {
    Crystalline,
    Vanilla,
}

pub fn export_html(content: &Content) -> Result<String, SourceDiagnostic> {
    export_html_with_serialization_and_pretty(
        content,
        HtmlSerializationMode::Crystalline,
        false,
    )
}

pub fn export_html_with_serialization(
    content: &Content,
    mode: HtmlSerializationMode,
) -> Result<String, SourceDiagnostic> {
    export_html_with_serialization_and_pretty(content, mode, false)
}

pub fn export_html_with_serialization_and_pretty(
    content: &Content,
    mode: HtmlSerializationMode,
    pretty: bool,
) -> Result<String, SourceDiagnostic> {
    export_html_with_frame_resolver(content, mode, pretty, None)
}

pub type HtmlFrameResolver<'a> = dyn Fn(
        &typst_core::entities::html::HtmlFrameElem,
        &typst_core::entities::style_chain::StyleChain,
    ) -> Result<String, SourceDiagnostic>
    + 'a;

pub fn export_html_with_resolved_frames(
    content: &Content,
    mode: HtmlSerializationMode,
    pretty: bool,
    resolver: &HtmlFrameResolver<'_>,
) -> Result<String, SourceDiagnostic> {
    export_html_with_frame_resolver(content, mode, pretty, Some(resolver))
}

impl HtmlSerializationMode {
    pub fn export_with_resolved_frames(
        self,
        content: &Content,
        pretty: bool,
        resolver: &HtmlFrameResolver<'_>,
    ) -> Result<String, SourceDiagnostic> {
        export_html_with_resolved_frames(content, self, pretty, resolver)
    }
}
