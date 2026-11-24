"""Materialize the measured laboratory delta on the reconstructed fixture tree."""
from pathlib import Path
import re
import sys

here = Path(__file__).resolve().parent
source = here / 'source'
core = source / '01_core/src/compiler/layout'
infra = source / '03_infra/src'

def edit(p, old, new):
    s = p.read_text()
    assert s.count(old) == 1, (p, old)
    p.write_text(s.replace(old, new, 1))

def owner(name, content):
    p = source / '00_nucleo/prompts' / f'{name}.md'
    with p.open('a') as f:
        f.write('\n\n## Adaptive physical suffix experiment\n\n' + content + '\n')

if sys.argv[1] == 'spec':
    for name, text in {
        'core/compiler/layout/paragraph_emit': 'The fixed-cut census had 506 avoided line closes. Within the isolated adaptive mode, suspend at any single page transition before the last line of an eligible paragraph. Preserve the historical suspension rule outside this mode.',
        'core/compiler/layout/sequence': 'Test candidate checkpoints at real page boundaries in ascending order. Keep existing root, route, pending item and paragraph guards. Record at most 64 checkpoints per complete attempt. Rebase only a certified suffix.',
        'core/compiler/layout/page_running_requests': 'Keep a bounded ordered list of checkpoints and find the one with the same number of committed pages. Require exact existing boundary equality and, in adaptive mode, compare admitted content values in addition to carrier metadata. The source admission admits only plain literal word edits in a fixed fixture skeleton.',
        'core/compiler/layout/physical_memo': 'Preserve the full-source admission, immutable epoch and both journal checks. In the adaptive laboratory arm, use the actual page boundary carried by the composer. Retain an authoritative full traversal only when it contains a checkpoint. Rerun without reuse if validation fails.',
        'core/compiler/layout/finish': 'Compose at the validated number of committed pages. Preserve prefix and suffix position, label, warning and layout checks. Refuse when the retained suffix is empty.',
        'infra/pipeline/context_stabilization/page_running': 'The prior cut was fixed at page 3. In the private adaptive lab arm allow the same bounded plain words in any literal block of the existing fixture skeleton. Select the first matching physical boundary by delegation to L1. An off arm must disable the suffix proof with the rest of the flow retained.',
        'infra/export/mod': 'Common historical build reconstruction: restore the archived laboratory multifont delegation recipe. Use the same implementation in every comparison arm.',
    }.items():
        owner(name, text)
    sys.exit(0)

# Reconstruct the common export owner from its preserved prototype recipe.
recipe = (here.parent / 'references-500-pdf-export-reuse-2026-11-20/prototype.py').read_text()
a = recipe.index('def replace_after(')
b = recipe.index('\nreplace(pdf,', a)
def replace(p, old, new):
    edit(p, old, new)
exec(recipe[a:b], {'export': infra / 'export', 'replace': replace})

p = core / 'paragraph_emit.rs'
edit(p, 'eligible: bool, before: usize, after: usize, index: usize, count: usize,','eligible: bool, before: usize, after: usize, index: usize, count: usize, adaptive: bool,')
edit(p, 'eligible && before == 1 && after == 2 && index.checked_add(1) == Some(count)',
     'eligible && before.checked_add(1) == Some(after)\n        && (adaptive || after == 2) && index.checked_add(1) == Some(count)')
edit(p,'lab_suspend_emitter(true, before, l.pages.len(), index, lines.len())',
     'lab_suspend_emitter(true, before, l.pages.len(), index, lines.len(),\n                l.lab_three_attempt.is_some_and(|attempt| attempt.adaptive))')
s=p.read_text()
s=re.sub(r'(lab_suspend_emitter\((?:true|false), \d, \d, \d, \d)\)\)',r'\1, false))',s)
p.write_text(s)

p = core / 'physical_memo.rs'
edit(p,'sealed: &super::SealedPageRunning, certify_three: bool,','sealed: &super::SealedPageRunning, certify_three: bool, adaptive: bool,')
edit(p,'LabThreeAttempt { previous, refuse }','LabThreeAttempt { previous, refuse, adaptive }')
edit(p,'product.trace.data.lab_three_cut.is_some()','!product.trace.data.lab_three_cuts.is_empty()')
edit(p,'pub(super) struct LabThreeBoundary {','pub(super) struct LabThreeBoundary {\n    pub(super) prefix_pages: usize,')
edit(p,'    pub(super) refuse: u64,','    pub(super) refuse: u64,\n    pub(super) adaptive: bool,')
edit(p,'pub(super) struct LabThreeReplay {','pub(super) struct LabThreeReplay {\n    pub(super) prefix_pages: usize,')

p = core / 'page_running_requests.rs'
edit(p, 'pub(super) lab_three_cut: Option<LabThreeCut>,','pub(super) lab_three_cuts: Vec<LabThreeCut>,')
edit(p, 'lab_three_cut: None,','lab_three_cuts: Vec::new(),')
edit(p, 'current: &LabThreeCut,','current: &LabThreeCut,\n        adaptive: bool,')
edit(p, 'let previous = old.data.lab_three_cut.as_ref().ok_or(5_u64)?;',
     'let previous = old.data.lab_three_cuts.iter().find(|cut|\n            cut.boundary.prefix_pages == current.boundary.prefix_pages).ok_or(5_u64)?;')
edit(p, '|| std::mem::discriminant(&a.content) != std::mem::discriminant(&b.content)\n            {\n                if old_event.page',
     '|| std::mem::discriminant(&a.content) != std::mem::discriminant(&b.content)\n                || (adaptive && a.content != b.content)\n            {\n                if old_event.page')

p = core / 'sequence.rs'
edit(p, 'let fail = |reason| {\n        lab_three_cut_receipt(reason, 2, 0);','let prefix_pages = l.pages.len();\n    let fail = |reason| {\n        lab_three_cut_receipt(reason, prefix_pages, 0);')
edit(p, 'l.is_sub_frame || l.pages.len() != 2 || !lab_sequence_boundary(parts, start, end)',
     'l.is_sub_frame || prefix_pages == 0 || (!attempt.adaptive && prefix_pages != 2)\n        || !lab_sequence_boundary(parts, start, end)')
edit(p,'let boundary = LabThreeBoundary {\n        start,','let boundary = LabThreeBoundary {\n        prefix_pages, start,')
edit(p,'pass.lab_prepare_three_splice(&previous.product.trace, &cut)','pass.lab_prepare_three_splice(&previous.product.trace, &cut, attempt.adaptive)')
edit(p,'previous: std::sync::Arc::clone(previous), splice,','previous: std::sync::Arc::clone(previous), splice, prefix_pages,')
edit(p,'l.page_running_pass.as_mut().unwrap().trace.data.lab_three_cut = Some(cut);',
     'let cuts = &mut l.page_running_pass.as_mut().unwrap().trace.data.lab_three_cuts;\n    if cuts.len() < 64 && cuts.last().is_none_or(|old| old.prefix_pages() < prefix_pages) {\n        cuts.push(cut);\n    }')

p = core / 'page_running_requests.rs'
edit(p, 'pub(super) struct LabThreeSplice {','impl LabThreeCut {\n    pub(super) fn prefix_pages(&self) -> usize { self.boundary.prefix_pages }\n}\n\npub(super) struct LabThreeSplice {')

p = core / 'finish.rs'
edit(p,'const PREFIX: usize = 2;', 'let prefix = replay.prefix_pages;')
p.write_text(p.read_text().replace('PREFIX', 'prefix'))

p = infra / 'pipeline/context_stabilization/page_running.rs'
edit(p, 'let (mut retained, mut witness) = physical_memo::lab_retain(&body, input.as_ref().clone(),',
     'let suffix_mode = std::env::var("TEKT_LAB_SUFFIX_MODE");\n        let adaptive = matches!(suffix_mode.as_deref(), Ok("adaptive"));\n        let (mut retained, mut witness) = physical_memo::lab_retain(&body, input.as_ref().clone(),')
edit(p, 'lab_page_three_fixture(self.source.text())).map_err', '!matches!(suffix_mode.as_deref(), Ok("off"))\n                && lab_page_three_fixture(self.source.text(), adaptive), adaptive).map_err')
edit(p, 'fn lab_page_three_fixture(mut text: &str) -> bool {','fn lab_page_three_fixture(mut text: &str, adaptive: bool) -> bool {')
edit(p, 'let Some(body) = text.strip_prefix(HEADER) else { return false };\n    text = body;',
     'let Some(body) = text.strip_prefix(HEADER) else { return false };\n    text = body;')
edit(p, 'let Some((words, rest)) = body.split_once(PARAGRAPH_TAIL) else { return false };\n        if if i == 0 {',
     'let Some((words, rest)) = body.split_once(PARAGRAPH_TAIL) else { return false };\n        if if i == 0 || adaptive {')
