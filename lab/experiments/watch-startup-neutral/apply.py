#!/usr/bin/env python3
"""Install bas/neu/laz static laboratory controls in separate RAM copies."""
import argparse
from _overlay import Edits, HERE, prepared_workspace

HUB = '01_core/src/compiler/eval/mod.rs'
MODULES = '01_core/src/compiler/eval/modules.rs'
SESSION = '03_infra/src/world/font_session.rs'


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('workspace')
    parser.add_argument('--variant', required=True, choices=('bas', 'neu', 'laz'))
    parser.add_argument('--mutation', default='none',
                        choices=('none', 'neutral-allocate', 'session-always-reuse'))
    args = parser.parse_args()
    workspace = prepared_workspace(args.workspace)
    if args.mutation == 'neutral-allocate' and args.variant != 'neu':
        parser.error('neutral-allocate requires variant neu')
    edits = Edits(workspace)
    if args.variant != 'bas':
        helper = (HERE / 'snapshot.rs').read_text()
        for flag in ('SHARE_IDENTITIES', 'IGNORE_INPUTS', 'IGNORE_FEATURES', 'FORCE_EAGER_LAZY'):
            if helper.count(f'const {flag}: bool = false;') != 1:
                raise ValueError(f'Nominal helper must disable mutation {flag}')
        edits.new('01_core/src/compiler/eval/lab_stdlib_snapshot.rs', helper)
        edits.replace(HUB, 'mod syntax_diagnostics;\n',
                      'mod syntax_diagnostics;\nmod lab_stdlib_snapshot;\n')
        edits.replace(HUB, 'pub struct EvalContext {\n',
            'pub struct EvalContext {\n'
            '    lab_stdlib_snapshot: RefCell<Option<Arc<lab_stdlib_snapshot::StdlibSnapshot>>>,\n')
        initial = 'None'
        if args.mutation == 'neutral-allocate':
            initial = ('Some(Arc::new(lab_stdlib_snapshot::StdlibSnapshot::new('
                       'SysInputs::default(), crate::entities::compiler_features::Features::default(), '
                       'Scope::new())))')
        edits.replace(HUB, '            context_reads: RefCell::new(ContextReads::new()),\n',
            f'            lab_stdlib_snapshot: RefCell::new({initial}),\n'
            '            context_reads: RefCell::new(ContextReads::new()),\n')
    if args.variant == 'laz':
        edits.replace(MODULES,
            '    let stdlib = super::make_stdlib_with_features(&engine.world.inputs(), ctx.features);',
            '    let lab_inputs = engine.world.inputs();\n'
            '    let (stdlib, lab_stdlib_snapshot) = super::lab_stdlib_snapshot::resolve(\n'
            '        &ctx.lab_stdlib_snapshot, &lab_inputs, ctx.features, 3,\n'
            '    );')
        edits.replace(MODULES, '    local_ctx.features = ctx.features;\n',
            '    local_ctx.features = ctx.features;\n'
            '    *local_ctx.lab_stdlib_snapshot.borrow_mut() = lab_stdlib_snapshot;\n')
    if args.mutation == 'session-always-reuse':
        edits.replace(SESSION, '        if let Some(old) = cached.as_ref() {\n',
            '        if let Some(old) = cached.as_ref() { return old.catalog.clone(); }\n'
            '        if let Some(old) = cached.as_ref() {\n')
    edits.publish()
    print(f'Applied LAB static variant={args.variant} mutation={args.mutation}')


if __name__ == '__main__':
    main()
