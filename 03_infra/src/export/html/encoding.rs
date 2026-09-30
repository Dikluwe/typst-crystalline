//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/html/encoding.md
//! @prompt-hash 9fd762f7
//! @layer L3
//! @updated 2026-09-26

use super::HtmlSerializationMode;
use typst_core::entities::content::Content;
use typst_core::entities::source_result::SourceDiagnostic;
use typst_core::entities::span::Span;

pub(super) fn raw_text(content: &Content) -> Result<String, SourceDiagnostic> {
    Ok(match content {
        Content::Empty => String::new(),
        Content::Text(text) => text.to_string(),
        Content::Space | Content::SpaceFromNewline => " ".into(),
        Content::Linebreak(_) => "\n".into(),
        Content::Sequence(items) => {
            items.iter().map(raw_text).collect::<Result<Vec<_>, _>>()?.concat()
        }
        Content::Styled(styled) => raw_text(&styled.body)?,
        _ => {
            return Err(SourceDiagnostic::error(
                Span::detached(),
                "HTML raw text element cannot have non-text children",
            ))
        }
    })
}

pub(super) fn title_text(content: &Content) -> Result<String, SourceDiagnostic> {
    Ok(match content {
        Content::Empty => String::new(),
        Content::Text(text) => text.as_str().replace('&', "&amp;").replace('<', "&lt;"),
        Content::Space | Content::SpaceFromNewline => " ".into(),
        Content::Linebreak(_) => "\n".into(),
        Content::Sequence(items) => {
            let mut out = String::new();
            let mut after_linebreak = false;
            for item in items.iter() {
                if after_linebreak
                    && matches!(item, Content::Space | Content::SpaceFromNewline)
                {
                    after_linebreak = false;
                    continue;
                }
                out.push_str(&title_text(item)?);
                after_linebreak = matches!(item, Content::Linebreak(_));
            }
            out
        }
        Content::Styled(styled) => title_text(&styled.body)?,
        _ => {
            return Err(SourceDiagnostic::error(
                Span::detached(),
                "HTML raw text element cannot have non-text children",
            ))
        }
    })
}

pub(super) fn escape_text(text: &str, mode: HtmlSerializationMode) -> String {
    let escaped = text.replace('&', "&amp;").replace('<', "&lt;");
    match mode {
        HtmlSerializationMode::Crystalline => {
            escaped.replace('>', "&gt;").replace('"', "&quot;")
        }
        HtmlSerializationMode::Vanilla => escaped,
    }
}

pub(super) fn escape_attribute(text: &str, mode: HtmlSerializationMode) -> String {
    let escaped = text.replace('&', "&amp;").replace('"', "&quot;");
    match mode {
        HtmlSerializationMode::Crystalline => {
            escaped.replace('<', "&lt;").replace('>', "&gt;")
        }
        HtmlSerializationMode::Vanilla => escaped,
    }
}
