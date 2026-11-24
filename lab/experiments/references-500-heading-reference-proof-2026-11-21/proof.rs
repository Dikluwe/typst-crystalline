/// Laboratory admission proof for an evaluated document with static heading targets.
pub(in crate::compiler) fn only_numbered_heading_targets(content: &Content) -> bool {
    use crate::entities::elements::outline::{OutlineIndent, OutlineTarget};
    use crate::entities::style_chain::StyleDelta;
    use rustc_hash::FxHashSet;

    fn harmless_scope(delta: &StyleDelta) -> bool {
        delta.custom.iter().all(|(key, value)| {
            matches!((key.as_str(), value),
                ("eval.flow.scope" | "heading.numbering", Value::Bool(_))
                | ("heading.numbering.pattern", Value::Str(_))
                | ("page.width" | "page.height", Value::Float(_) | Value::Auto))
        })
    }

    fn plain_heading(content: &Content) -> bool {
        match content {
            Content::Source(e) => plain_heading(&e.body),
            Content::Sequence(parts) => parts.iter().all(plain_heading),
            Content::Text(_) | Content::Space | Content::SpaceFromNewline | Content::Empty => true,
            _ => false,
        }
    }

    fn scan<'a>(
        content: &'a Content,
        mut numbered: bool,
        labels: &mut FxHashSet<&'a str>,
        references: &mut Vec<&'a str>,
    ) -> bool {
        match content {
            Content::Source(e) => scan(&e.body, numbered, labels, references),
            Content::Sequence(parts) =>
                parts.iter().all(|part| scan(part, numbered, labels, references)),
            Content::Styled(e) => {
                let delta = e.styles.delta();
                if !harmless_scope(delta) {
                    return false;
                }
                for (key, value) in &delta.custom {
                    if let ("heading.numbering", Value::Bool(enabled)) = (key.as_str(), value) {
                        numbered = *enabled;
                    }
                }
                scan(&e.body, numbered, labels, references)
            }
            Content::Label(e) => numbered
                && matches!(e.body.source_body(), Content::Heading(heading)
                    if plain_heading(&heading.body))
                && labels.insert(e.name.as_str()),
            Content::Heading(e) => plain_heading(&e.body),
            Content::Ref(e) if e.form == RefForm::Normal && e.supplement.is_none() => {
                references.push(e.name.as_str());
                true
            }
            Content::Outline(e) => e.target == OutlineTarget::Headings
                && e.title.is_none()
                && !e.title_was_set()
                && matches!(e.indent, OutlineIndent::Auto)
                && e.show_rules().is_empty()
                && e.realized_body().is_none()
                && harmless_scope(e.captured_styles().delta()),
            Content::SetPage(e) => e.numbering.is_none()
                && e.supplement.is_none()
                && e.header.is_none()
                && e.footer.is_none()
                && e.background.is_none()
                && e.foreground.is_none()
                && e.columns.is_none(),
            Content::Text(_) | Content::Space | Content::SpaceFromNewline
            | Content::Parbreak | Content::Empty => true,
            _ => false,
        }
    }

    let mut labels = FxHashSet::default();
    let mut references = Vec::new();
    scan(content, false, &mut labels, &mut references)
        && !references.is_empty()
        && references.iter().all(|name| labels.contains(name))
}
