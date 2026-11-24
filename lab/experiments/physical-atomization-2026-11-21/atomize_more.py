#!/usr/bin/env python3
"""Move physical index, traversal and read validation without changing order."""
from pathlib import Path
import textwrap

root = Path(__file__).resolve().parent / 'source'
infra = root / '03_infra/src/pipeline/context_stabilization'
candidate = infra / 'page_running/physical_candidate.rs'
source = candidate.read_text()

index_start = source.index('    fn index_physical_steps(')
candidate_start = source.index('    pub(in super::super) fn physical_candidate(', index_start)
traversal_start = source.index('    fn current_running_store(', candidate_start)
impl_end = source.rindex('\n}')
index_methods = source[index_start:candidate_start].replace(
    '    fn index_physical_steps(', '    pub(super) fn index_physical_steps(', 1)
traversal_methods = source[traversal_start:impl_end]
for name in ('current_running_store', 'retire_stale_running', 'physical_step'):
    traversal_methods = traversal_methods.replace(
        f'    fn {name}(', f'    pub(super) fn {name}(', 1)
candidate.write_text(source[:index_start] + source[candidate_start:traversal_start] + '\n}\n')

def physical_file(name, methods):
    path = infra / f'page_running/{name}.rs'
    assert not path.exists(), path
    path.write_text(f'''//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/pipeline/context_stabilization/page_running/{name}.md
//! @prompt-hash 00000000
//! @layer L3
//! @updated 2026-11-21

use super::*;

impl Session<'_> {{
{methods}}}
''')

physical_file('physical_index', index_methods)
physical_file('physical_traversal', traversal_methods)

running = infra / 'page_running.rs'
running_text = running.read_text()
anchor = '#[path = "page_running/physical_candidate.rs"]\nmod physical_candidate;'
assert running_text.count(anchor) == 1
running.write_text(running_text.replace(anchor, anchor + '''
#[path = "page_running/physical_index.rs"]
mod physical_index;
#[path = "page_running/physical_traversal.rs"]
mod physical_traversal;'''))

parent = root / '03_infra/src/pipeline/context_stabilization.rs'
text = parent.read_text()
start = text.index('            // P1159 participates in this same budget')
end = text.index('            history.push(candidate.clone());', start)
body = text[start:end]
assert body.count('return (self.fail(errors), self.warnings());') == 1
assert body.count('return (self.fail(errors), self.warnings()),') == 1
body = body.replace('return (self.fail(errors), self.warnings());', 'return Err(errors);')
body = body.replace('return (self.fail(errors), self.warnings()),', 'return Err(errors),')
body = body.replace('module.is_some() || self.bundle_html_targets.is_some()',
                    'module_present || self.bundle_html_targets.is_some()', 1)
method = '''    pub(super) fn validate_candidate_reads(
        &mut self,
        input: &TagIntrospector,
        candidate: &TagIntrospector,
        module_present: bool,
        physical_incomplete: bool,
        html_positions_selected: bool,
    ) -> SourceResult<(bool, bool)> {
''' + textwrap.indent(textwrap.dedent(body), '        ') + '''        Ok((valid, closed))
    }
'''
replacement = '''            let (valid, closed) = match self.validate_candidate_reads(
                input,
                &candidate,
                module.is_some(),
                physical_incomplete,
                html_positions_selected,
            ) {
                Ok(result) => result,
                Err(errors) => return (self.fail(errors), self.warnings()),
            };
'''
text = text[:start] + replacement + text[end:]
helper_start = text.index('/// Conservative equality of the actual layout input views')
helper_end = text.index('\npub(super) fn expand(', helper_start)
helper = text[helper_start:helper_end]
text = text[:helper_start] + text[helper_end + 1:]
assert text.count('pagination_stability(&execution.input, candidate)') == 1
text = text.replace('pagination_stability(&execution.input, candidate)',
                    'read_validation::pagination_stability(&execution.input, candidate)', 1)
helper = helper.replace('fn pagination_stability(', 'pub(super) fn pagination_stability(', 1)
anchor = 'mod page_running;'
assert text.count(anchor) == 1
text = text.replace(anchor, anchor + '\nmod read_validation;', 1)
parent.write_text(text)

validation = infra / 'read_validation.rs'
assert not validation.exists(), validation
validation.write_text('''//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/pipeline/context_stabilization/read_validation.md
//! @prompt-hash 00000000
//! @layer L3
//! @updated 2026-11-21

use super::*;

impl Session<'_> {
''' + method + '}\n\n' + helper + '\n')
print(candidate, infra/'page_running/physical_index.rs',
      infra/'page_running/physical_traversal.rs', validation, sep='\n')
