"""Atomize the substitution in the disposable reconstruction of the control."""
from pathlib import Path
import re
import sys

root, mode = Path(sys.argv[1]), sys.argv[2]
assert mode in ("atomized", "retained")
pipeline = root / "03_infra/src/pipeline.rs"
source = pipeline.read_text()
start = source.index("fn substitute_context_blocks(")
end = source.index("/// Tempos das fases", start)
original = source[start:end]
heading_start = source.index("fn stable_heading_substitution_body(")
heading_start = source.rfind("\n\n", 0, heading_start) + 2
heading = source[heading_start:start]
page_run_start = source.index("fn source_content_root(")
page_run_end = source.index("fn collect_context_blocks(", page_run_start)
page_run = source[page_run_start:page_run_end]
directory = pipeline.parent / "pipeline/substitution"
directory.mkdir(exist_ok=True)
marker = "        // These nodes delegate descendant substitution"
catalogue = original[original.index(marker):original.index("    let mut operational_templates")]
catalogue = catalogue.replace("=> {}\n", "=> fallback::map(self, content, chain),\n")
fallback_start = original.index("    let mut operational_templates")
callback_start = original.index("            Ok(match node", fallback_start)
callback_end = original.index("\n            })", callback_start)
callback = original[callback_start:callback_end].removeprefix("            Ok(")
callback += "\n            }"
callback = re.sub(r"resolved.get", "pass.resolved.get", callback)

def write(name, source):
    (directory / f"{name}.rs").write_text(source.strip() + "\n")

kinds = re.findall(r"Content::(\w+)\(e\) => body!\(", original)
simple = "".join(f"        Content::{kind}(e) => wrappers::plain!(self, {kind}, e, chain),\n"
                 for kind in kinds)
write("mod", ("""
use super::figure_context_supplement;
use std::{collections::HashMap, sync::Arc};
use typst_core::entities::{content::Content, style_chain::StyleChain};

mod fallback;
mod figure;
mod heading;
mod page_run;
mod sequence;
mod wrappers;
#[cfg(test)]
mod tests;

const PRESERVE: bool = MODE;

pub(super) fn substitute_context_blocks(content: Content, resolved: &HashMap<u64, Content>) -> Content {
    substitute_context_blocks_in_chain(content, resolved, &StyleChain::default_chain())
}

pub(super) fn substitute_context_blocks_in_chain(
    content: Content,
    resolved: &HashMap<u64, Content>,
    chain: &StyleChain,
) -> Content {
    Rewrite { resolved }.rewrite(&content, chain).unwrap_or(content)
}

struct Rewrite<'a> {
    resolved: &'a HashMap<u64, Content>,
}

impl Rewrite<'_> {
    // Atom-only control deliberately rebuilds exactly the same wrappers as the
    // original path. The second arm propagates an unchanged child instead.
    fn child(&self, content: &Content, chain: &StyleChain) -> Option<Content> {
        self.rewrite(content, chain).or_else(|| (!PRESERVE).then(|| content.clone()))
    }

    fn rewrite(&self, content: &Content, chain: &StyleChain) -> Option<Content> {
        match content {
        Content::Source(e) => wrappers::plain!(self, Source, e, chain),
        Content::Located(e) if !matches!(e.body, Content::ContextBlock(_)) =>
            wrappers::plain!(self, Located, e, chain),
        Content::Realized(_) => wrappers::realized(self, content, chain),
        Content::ShowRecipe(_) => wrappers::show_recipe(self, content),
        Content::Styled(_) => wrappers::styled(self, content, chain),
        Content::Sequence(parts) => sequence::map(self, parts, chain).map(Content::Sequence),
        Content::MathSequence(parts) => sequence::map(self, parts, chain).map(Content::MathSequence),
        Content::Figure(_) => figure::map(self, content, chain),
        Content::Heading(_) => heading::map(self, content, chain),
SIMPLE        Content::Par { body } => wrappers::par(self, body, chain),
        Content::SmallCaps { body } => wrappers::small_caps(self, body, chain),
CATALOGUE    }
}
""").replace("MODE", str(mode == "retained").lower()).replace("SIMPLE", simple)
   .replace("CATALOGUE", catalogue))

write("wrappers", """
use super::{Arc, Content, Rewrite, StyleChain};
use typst_core::entities::source_result::SourceResult;
use typst_core::compiler::realize::reference_supplements::{
    map_prepared_reference_supplement, prepared_reference_supplement,
};

macro_rules! plain {
    ($pass:expr, $variant:ident, $element:expr, $chain:expr) => {{
        $pass.child(&$element.body, $chain).map(|body| {
            let mut rebuilt = (**$element).clone();
            rebuilt.body = body;
            Content::$variant(Arc::new(rebuilt))
        })
    }};
}
pub(super) use plain;

pub(super) fn realized(pass: &Rewrite<'_>, content: &Content, chain: &StyleChain) -> Option<Content> {
    let Content::Realized(element) = content else { unreachable!() };
    let visual = pass.child(&element.visual, chain)?;
    let mut rebuilt = (**element).clone();
    rebuilt.visual = visual;
    Some(Content::Realized(Arc::new(rebuilt)))
}

pub(super) fn show_recipe(pass: &Rewrite<'_>, content: &Content) -> Option<Content> {
    let Content::ShowRecipe(element) = content else { unreachable!() };
    plain!(pass, ShowRecipe, element, &element.styles)
}

// Outer Option says the capsule was recognized. Inner Option says its
// selected child changed. A recognized unchanged capsule stays opaque.
pub(super) fn reference(
    pass: &Rewrite<'_>, content: &Content, chain: &StyleChain,
) -> SourceResult<Option<Option<Content>>> {
    let Some(child) = prepared_reference_supplement(content) else { return Ok(None) };
    let Some(replacement) = pass.child(child, chain) else { return Ok(Some(None)) };
    map_prepared_reference_supplement(content, |_| Ok(replacement)).map(Some)
}

pub(super) fn styled(pass: &Rewrite<'_>, content: &Content, chain: &StyleChain) -> Option<Content> {
    if let Ok(Some(rebuilt)) = reference(pass, content, chain) {
        return rebuilt;
    }
    let Content::Styled(element) = content else { unreachable!() };
    plain!(pass, Styled, element, &chain.push_styles(&element.styles))
}

pub(super) fn par(pass: &Rewrite<'_>, body: &Content, chain: &StyleChain) -> Option<Content> {
    pass.child(body, chain).map(|body| Content::Par { body: Box::new(body) })
}

pub(super) fn small_caps(pass: &Rewrite<'_>, body: &Content, chain: &StyleChain) -> Option<Content> {
    pass.child(body, chain).map(|body| Content::SmallCaps { body: Box::new(body) })
}
""")

write("heading", """
use super::{wrappers, Arc, Content, Rewrite, StyleChain};

pub(super) fn map(pass: &Rewrite<'_>, content: &Content, chain: &StyleChain) -> Option<Content> {
    let Content::Heading(element) = content else { unreachable!() };
    if stable_heading_substitution_body(&element.body) {
        return None;
    }
    wrappers::plain!(pass, Heading, element, chain)
}

""" + heading)

write("page_run", """
use super::{Arc, Content};
""" + page_run.replace("fn page_run_at_callback(", "pub(super) fn at_callback(")
    .replace("fn restore_page_run_templates(", "pub(super) fn restore(")
    .replace("restore_page_run_templates(", "restore("))

write("figure", """
use super::{figure_context_supplement, Arc, Content, Rewrite, StyleChain};

pub(super) fn map(pass: &Rewrite<'_>, content: &Content, chain: &StyleChain) -> Option<Content> {
    let Content::Figure(element) = content else { unreachable!() };
    let body = pass.child(&element.body, chain);
    let supplement = figure_context_supplement(element, chain)
        .and_then(|supplement| pass.child(supplement, chain));
    let caption = element.caption.as_ref().and_then(|caption| pass.child(caption, chain));
    if super::PRESERVE && body.is_none() && supplement.is_none() && caption.is_none() {
        return None;
    }
    let mut rebuilt = (**element).clone();
    if let Some(body) = body { rebuilt.body = body; }
    if let Some(supplement) = supplement { rebuilt.resolved_supplement = Some(Some(supplement)); }
    if let Some(caption) = caption { rebuilt.caption = Some(caption); }
    Some(Content::Figure(Arc::new(rebuilt)))
}
""")

write("sequence", """
use super::{Arc, Content, PRESERVE, Rewrite, StyleChain};

pub(super) fn map(
    pass: &Rewrite<'_>, parts: &[Content], chain: &StyleChain,
) -> Option<Arc<[Content]>> {
    if !PRESERVE {
        return Some(parts.iter().map(|part| pass.child(part, chain).unwrap())
            .collect::<Vec<_>>().into());
    }
    let mut output = None;
    for (index, child) in parts.iter().enumerate() {
        if let Some(replacement) = pass.rewrite(child, chain) {
            let output = output.get_or_insert_with(|| {
                let mut items = Vec::with_capacity(parts.len());
                items.extend_from_slice(&parts[..index]);
                items
            });
            output.push(replacement);
        } else if let Some(output) = &mut output {
            output.push(child.clone());
        }
    }
    output.map(Into::into)
}
""")

write("fallback", """
use super::{page_run, wrappers, Arc, Content, PRESERVE, Rewrite, StyleChain};

pub(super) fn map(pass: &Rewrite<'_>, content: &Content, chain: &StyleChain) -> Option<Content> {
    if PRESERVE && matches!(content, Content::Empty | Content::Text(_) | Content::Space
        | Content::SpaceFromNewline | Content::Parbreak | Content::Ref(_)) {
        // These leaves have no generic projection and cannot be a callback
        // replacement. All other variants keep the mapper's normalization,
        // including Source handling below a generic fallback root.
        return None;
    }
    let mut operational_templates = Vec::new();
    let _ = content.map_content(&mut |node| {
        if let Some(page) = page_run::at_callback(node) {
            operational_templates.push(page.clone());
        }
        Ok(None)
    });
    let mut operational_templates = operational_templates.into_iter();
    content.map_content(&mut |node| {
        let replacement = if let Some(page) = page_run::at_callback(node) {
            let original = operational_templates.next()
                .expect("paired structural traversals preserve order");
            (!PRESERVE || !Arc::ptr_eq(page, &original)).then(|| page_run::restore(node, &original))
        } else if let Some(rebuilt) = wrappers::reference(pass, node, chain)? {
            rebuilt
        } else {
            context_marker(pass, node)
        };
        Ok(replacement)
    }).ok()
}

fn context_marker(pass: &Rewrite<'_>, node: &Content) -> Option<Content> {
MARKER
}
""".replace("MARKER", callback))
write("tests", (Path(__file__).with_name("tests.rs")).read_text())
pipeline.write_text(source[:page_run_start] + source[page_run_end:heading_start] + """mod substitution;
use substitution::{substitute_context_blocks, substitute_context_blocks_in_chain};

""" + source[end:])
