"""Exact ELF symbol inputs and call sites for the residual evaluation census."""
from pathlib import Path

D = Path(__file__).resolve().parent
RESULT = D.parents[1]/"results/construction/diagnostics"
PRIOR = RESULT/"references-500-visible-textual-style-collapse-2026-11-21"
TWIN = RESULT/"references-500-visible-page-operator-cost-2026-11-21"
assert (TWIN/"vanilla-symbols-verified").is_file()
ELF_SHA = dict(line.split()[::-1] for line in (PRIOR/"binaries.sha256").read_text().splitlines())
TABLES = {
    "vanilla":TWIN/"vanilla-symbols-twin.txt",
    "candidate":PRIOR/"candidate.symbols",
    "control":PRIOR/"control.symbols",
}
TEKT = {
    "watch_wait": ("::watch::wait_for_change_since",),
    "eval": ("::eval_to_module_in_session",),
    "collapse": ("::style_chain::StyleChain::collapse",),
    "find": ("::representation::linked::LinkedNode::find",),
    "concat": ("::eval::operators::join::concat_content",),
    "join": ("::eval::operators::join::join",),
}
NATIVE = {
    "watch_wait": ("typst_kit::watcher::Watcher::wait",),
    "eval": ("typst_eval::eval",),
    "find": ("typst_syntax::node::LinkedNode::find_number", "typst_syntax::node::LinkedNode::find_range"),
    "concat": ("<typst_library::foundations::content::Content as core::ops::arith::Add>::add",
               "<typst_library::foundations::content::Content as core::ops::arith::Add<&typst_library::foundations::content::Content>>::add"),
    "join": ("typst_library::foundations::ops::join",),
}
# Return PCs after verified call instructions (addresses in .text).
COLLAPSES = {
    "candidate": {0x1fe32f8:"markup_entry",0x1fe46e6:"markup_boundary",0x2504542:"group"},
    "control": {0x1fe223a:"markup_entry",0x1fe3515:"markup_boundary",0x2034910:"group",0x2034923:"group"},
}
MARKUP_FIND = {"candidate":0x1fe3421,"control":0x1fe36ea}
