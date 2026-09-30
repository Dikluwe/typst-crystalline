//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/html/whitespace.md
//! @prompt-hash c19b7d6a
//! @layer L3
//! @updated 2026-09-26

use super::inline::inline;
use super::tags::{should_group_into_paragraph, BLOCK_TAGS, REPLACED_TAGS};
use super::{HtmlFrameResolver, HtmlSerializationMode};
use typst_core::entities::content::Content;
use typst_core::entities::source_result::SourceDiagnostic;

const PROTECTED_SPACE: &str = "<span style=\"white-space: pre-wrap\">&#x20;</span>";

#[derive(Clone, Copy)]
enum SpaceSupport {
    Boundary,
    EmptyInline,
    Supportive,
}

pub(super) fn inline_html_body(
    parent_tag: &str,
    content: &Content,
    mode: HtmlSerializationMode,
    resolver: Option<&HtmlFrameResolver<'_>>,
    chain: &typst_core::entities::style_chain::StyleChain,
) -> Result<String, SourceDiagnostic> {
    let Content::Sequence(items) = content else {
        return inline(content, mode, resolver, chain);
    };
    let first = items
        .iter()
        .position(|item| !matches!(item, Content::Space | Content::SpaceFromNewline));
    let Some(first) = first else { return Ok(String::new()) };
    let last = items
        .iter()
        .rposition(|item| !matches!(item, Content::Space | Content::SpaceFromNewline))
        .unwrap();
    let mut out = String::new();
    for index in first..=last {
        if matches!(items[index], Content::Space | Content::SpaceFromNewline) {
            if parent_tag == "ruby"
                && index > first
                && index < last
                && is_ruby_annotation(&items[index - 1])
                && is_ruby_annotation(&items[index + 1])
            {
                continue;
            }
            if index > first && collapses_adjacent_whitespace(&items[index - 1])
                || index < last && collapses_adjacent_whitespace(&items[index + 1])
            {
                continue;
            }
            out.push_str(space_between(items, index));
        } else {
            out.push_str(&inline(&items[index], mode, resolver, chain)?);
        }
    }
    Ok(out)
}

fn is_ruby_annotation(content: &Content) -> bool {
    matches!(content, Content::HtmlElem(elem) if matches!(elem.tag.as_str(), "rp" | "rt"))
}

fn space_between(items: &[Content], index: usize) -> &'static str {
    match (
        space_support(items[..index].iter().rev()),
        space_support(items[index + 1..].iter()),
    ) {
        (SpaceSupport::Boundary, _) | (_, SpaceSupport::Boundary) => "",
        (SpaceSupport::Supportive, SpaceSupport::Supportive) => " ",
        _ => PROTECTED_SPACE,
    }
}

pub(super) fn top_level_space_between(items: &[Content], index: usize) -> &'static str {
    match (
        paragraph_space_support(items[..index].iter().rev()),
        paragraph_space_support(items[index + 1..].iter()),
    ) {
        (SpaceSupport::Boundary, _) | (_, SpaceSupport::Boundary) => "",
        (SpaceSupport::Supportive, SpaceSupport::Supportive) => " ",
        _ => PROTECTED_SPACE,
    }
}

fn paragraph_space_support<'a>(items: impl Iterator<Item = &'a Content>) -> SpaceSupport {
    let mut found_inline = false;
    for item in items {
        match item {
            Content::Parbreak | Content::Heading(_) | Content::Par { .. } => {
                return if found_inline {
                    SpaceSupport::EmptyInline
                } else {
                    SpaceSupport::Boundary
                };
            }
            Content::HtmlElem(elem)
                if !should_group_into_paragraph(elem.tag.as_str()) =>
            {
                return if found_inline {
                    SpaceSupport::EmptyInline
                } else {
                    SpaceSupport::Boundary
                };
            }
            _ => {}
        }
        if supports_adjacent_space(item) {
            return SpaceSupport::Supportive;
        }
        if !matches!(item, Content::Space | Content::SpaceFromNewline | Content::Empty) {
            found_inline = true;
        }
    }
    if found_inline {
        SpaceSupport::EmptyInline
    } else {
        SpaceSupport::Boundary
    }
}

fn space_support<'a>(items: impl Iterator<Item = &'a Content>) -> SpaceSupport {
    let mut found_inline = false;
    for item in items {
        if collapses_adjacent_whitespace(item) {
            return if found_inline {
                SpaceSupport::EmptyInline
            } else {
                SpaceSupport::Boundary
            };
        }
        if supports_adjacent_space(item) {
            return SpaceSupport::Supportive;
        }
        if !matches!(item, Content::Space | Content::SpaceFromNewline | Content::Empty) {
            found_inline = true;
        }
    }
    if found_inline {
        SpaceSupport::EmptyInline
    } else {
        SpaceSupport::Boundary
    }
}

fn supports_adjacent_space(content: &Content) -> bool {
    match content {
        Content::Text(text) => !text.is_empty(),
        Content::Sequence(items) => items.iter().any(supports_adjacent_space),
        Content::Strong(elem) => supports_adjacent_space(&elem.body),
        Content::Emph(elem) => supports_adjacent_space(&elem.body),
        Content::Styled(styled) => supports_adjacent_space(&styled.body),
        Content::HtmlElem(elem) => {
            if collapses_adjacent_whitespace(content) {
                false
            } else if REPLACED_TAGS.contains(&elem.tag.as_str()) {
                true
            } else {
                elem.body.content().is_some_and(supports_adjacent_space)
            }
        }
        Content::Empty
        | Content::Space
        | Content::SpaceFromNewline
        | Content::Parbreak => false,
        _ => true,
    }
}

fn collapses_adjacent_whitespace(content: &Content) -> bool {
    matches!(content, Content::HtmlElem(elem) if elem.tag == "br" || BLOCK_TAGS.contains(&elem.tag.as_str()))
}
