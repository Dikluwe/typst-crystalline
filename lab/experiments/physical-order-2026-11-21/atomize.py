#!/usr/bin/env python3
"""Extract physical candidate orchestration without changing its execution order."""
from pathlib import Path
import shutil
import sys

ROOT = Path(__file__).resolve().parent
source = ROOT / (sys.argv[1] if len(sys.argv) > 1 else 'control-source')
target = ROOT / (sys.argv[2] if len(sys.argv) > 2 else 'atomized-source')
if target.exists():
    raise SystemExit(f'already exists: {target}')
shutil.copytree(source, target, copy_function=shutil.copy2)
parent = target / '03_infra/src/pipeline/context_stabilization/page_running.rs'
text = parent.read_text()
start = text.index('    fn index_physical_steps(')
end = text.index('\n}\n\n#[cfg(test)]\nmod outline_retention_tests', start)
methods = text[start:end]
text = text[:start] + '}\n\n#[path = "page_running/physical_candidate.rs"]\nmod physical_candidate;\n' + text[end + 2:]
parent.write_text(text)
child = target / '03_infra/src/pipeline/context_stabilization/page_running/physical_candidate.rs'
child.parent.mkdir(exist_ok=True)
child.write_text('''//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/pipeline/context_stabilization/page_running/physical_candidate.md
//! @prompt-hash 00000000
//! @layer L3
//! @updated 2026-11-21

use super::*;

impl Session<'_> {
''' + methods.replace('    pub(super) fn physical_candidate(', '    pub(in super::super) fn physical_candidate(')
    .replace('super::super::runtime::', 'super::super::super::runtime::')
    .replace('super::super::paged::', 'super::super::super::paged::')
    .replace('super::bundle_page_views(', 'super::super::bundle_page_views(') + '\n}\n')
owner = target / '00_nucleo/prompts/infra/pipeline/context_stabilization/page_running/physical_candidate.md'
owner.parent.mkdir(exist_ok=True)
owner.write_text((ROOT / 'physical-candidate-l0-control.md').read_text())
print(parent, child, sep='\n')
