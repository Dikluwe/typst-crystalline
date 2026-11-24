"""Extract two evaluation responsibilities from the frozen laboratory source."""
from pathlib import Path

root = Path(__file__).resolve().parent / 'source/01_core/src/compiler/eval'
hub = root / 'mod.rs'
text = hub.read_text()
start = text.index('fn eval_with_subtree_session(')
end = text.index('// eval_args, apply_func', start)
driver = text[start:end].rstrip().replace(
    'fn eval_with_subtree_session(', 'pub(super) fn eval_with_subtree_session(', 1)
text = text[:start] + text[end:]
anchor = 'mod syntax_diagnostics;'
assert text.count(anchor) == 1
text = text.replace(anchor, anchor + '\nmod document_evaluation;\nuse self::document_evaluation::eval_with_subtree_session;', 1)
hub.write_text(text)
target = root / 'document_evaluation.rs'
assert not target.exists()
target.write_text('''//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/eval/document_evaluation.md
//! @prompt-hash 00000000
//! @layer L1
//! @updated 2026-11-22

use super::*;

''' + driver + '\n')

control = root / 'control_flow.rs'
text = control.read_text()
start = text.index('pub(super) fn eval_for(')
loop = text[start:].rstrip()
loop = loop.replace('pub(super) fn eval_for(', 'pub(in super::super) fn eval_for(', 1)
text = text[:start].rstrip() + '\n'
anchor = 'use super::{bindings::destructure_let, eval_expr, operators, EvalContext, FlowEvent};'
assert text.count(anchor) == 1
text = text.replace(anchor, anchor + '\n\n#[path = "control_flow/for_loop.rs"]\nmod for_loop;\npub(super) use self::for_loop::eval_for;', 1)
control.write_text(text)
target = root / 'control_flow/for_loop.rs'
target.parent.mkdir(exist_ok=True)
assert not target.exists()
loop = loop.replace('super::subtree_session::key(', 'super::super::subtree_session::key(', 1)
target.write_text('''//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/eval/control_flow/for_loop.md
//! @prompt-hash 00000000
//! @layer L1
//! @updated 2026-11-22

use super::*;

''' + loop + '\n')
print(hub, control, root / 'document_evaluation.rs', target, sep='\n')
