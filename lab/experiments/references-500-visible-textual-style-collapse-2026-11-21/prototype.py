"""Add a comparison-local collapsed style cache to the disposable source."""
from pathlib import Path
import runpy
import sys

here = Path(__file__).resolve().parent
runpy.run_path(str(here.parent / "references-500-visible-eval-protected-memo-2026-11-21" / "prototype.py"))
core = Path(sys.argv[1]) / "01_core/src"

def edit(path, before, after):
    text = path.read_text()
    assert text.count(before) == 1, (path, before)
    path.write_text(text.replace(before, after))

chain = core / "entities/style_chain.rs"
edit(chain, "impl StyleChain {", """impl StyleChain {
    /// Proves identity of the immutable chain, including all ancestors.
    pub(crate) fn same_head(&self, other: &Self) -> bool {
        match (&self.0, &other.0) {
            (Some(a), Some(b)) => Arc::ptr_eq(a, b),
            (None, None) => true,
            _ => false,
        }
    }
""")
textual = core / "compiler/eval/rules/textual.rs"
edit(textual, "\nuse std::sync::Arc;\n", "\nuse std::{cell::OnceCell, rc::Rc, sync::Arc};\n")
edit(textual, "use crate::entities::style_chain::StyleChain;", "use crate::entities::style_chain::{StyleChain, StyleDelta};")
edit(textual, """    guards: Vec<crate::entities::show::RuleId>,
}

impl TextualSnapshot {
    pub(crate) fn capture(engine: &Engine<'_>, ctx: &EvalContext) -> Self {
        Self {""", """    guards: Vec<crate::entities::show::RuleId>,
    collapsed: Rc<OnceCell<StyleDelta>>,
}

impl TextualSnapshot {
    pub(crate) fn capture(engine: &Engine<'_>, ctx: &EvalContext) -> Self {
        Self::capture_following(engine, ctx, None)
    }

    pub(crate) fn capture_following(
        engine: &Engine<'_>,
        ctx: &EvalContext,
        previous: Option<&Self>,
    ) -> Self {
        let base = ctx.textual_frames.last().map_or(&*engine.show_rules, |frame| &frame.rules);
        // Both metadata nodes are constructed with the same immutable style
        // input, lexical rule list and file. Keep the actual transport intact.
        let collapsed = previous.filter(|prev| baseline(&prev.styles).is_some_and(|old| {
            old.styles.same_head(engine.styles) && Arc::ptr_eq(&old.rules, base)
                && old.file == engine.current_file
        })).map_or_else(Rc::default, |prev| prev.collapsed.clone());
        Self {
            collapsed,""")
edit(textual, """            && self.styles.collapse() == other.styles.collapse()
            && self.guards == other.guards""", """            && self.collapsed.get_or_init(|| self.styles.collapse())
                == other.collapsed.get_or_init(|| other.styles.collapse())
            && self.guards == other.guards""")
edit(textual, "            styles: StyleChain::empty(),\n            guards: vec![],\n        };",
     "            styles: StyleChain::empty(),\n            guards: vec![],\n            collapsed: Default::default(),\n        };")
edit(textual, """                .push(StyleDelta { bold: Some(true), ..StyleDelta::empty() }),
            ..baseline.clone()""", """                .push(StyleDelta { bold: Some(true), ..StyleDelta::empty() }),
            collapsed: Default::default(),
            ..baseline.clone()""")
markup = core / "compiler/eval/markup_stream.rs"
edit(markup, "        let snapshot = rules::textual::TextualSnapshot::capture(engine, ctx);",
     """        let snapshot = rules::textual::TextualSnapshot::capture_following(
            engine, ctx, textual_snapshots.last(),
        );""")
