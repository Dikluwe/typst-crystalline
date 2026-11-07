#!/usr/bin/env python3
"""Apply the stdlib experiment only to C's separate RAM workspace copy.

No builds, active productive edits, L0 resealing or environment-dependent core
behavior. Mutations are compile-time laboratory choices for adversarial tests.
"""

import argparse
from pathlib import Path


RAM = Path('/dev/shm/tekt-stdlib-reuse-20261105')
HERE = Path(__file__).resolve().parent


def replace_once(text, old, new):
    if text.count(old) != 1:
        raise ValueError(f'Expected exactly one unmodified anchor: {old[:100]!r}')
    return text.replace(old, new, 1)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('workspace', type=Path)
    parser.add_argument('--mutation', default='none', choices=(
        'none', 'share-identities', 'ignore-inputs', 'ignore-features',
    ))
    args = parser.parse_args()
    workspace = args.workspace.resolve()
    if not RAM.is_dir() or not workspace.is_dir():
        parser.error('Task RAM area and prepared workspace must already exist')
    if (not workspace.is_relative_to(RAM.resolve())
            or workspace == RAM.resolve()
            or workspace.is_relative_to((RAM / 'baseline').resolve())):
        parser.error('Only a separate child workspace under the exclusive task RAM area is allowed')
    hub = workspace / '01_core/src/compiler/eval/mod.rs'
    modules = workspace / '01_core/src/compiler/eval/modules.rs'
    helper = hub.parent / 'lab_stdlib_snapshot.rs'
    if any(not path.resolve().is_relative_to(workspace) for path in (hub, modules, helper)):
        parser.error('Source/helper paths may not escape the prepared RAM workspace')
    if helper.exists() or helper.is_symlink():
        parser.error('Workspace already contains this experiment')

    text = hub.read_text()
    text = replace_once(text, 'mod syntax_diagnostics;\n',
        'mod syntax_diagnostics;\nmod lab_stdlib_snapshot;\n')
    text = replace_once(text, 'pub struct EvalContext {\n',
        'pub struct EvalContext {\n'
        '    lab_stdlib_snapshot: Option<Arc<lab_stdlib_snapshot::StdlibSnapshot>>,\n')
    text = replace_once(text, '            context_reads: RefCell::new(ContextReads::new()),\n',
        '            lab_stdlib_snapshot: None,\n'
        '            context_reads: RefCell::new(ContextReads::new()),\n')

    # The Source/expression facade still queries inputs and composes at the same
    # point before its existing syntax/argument evaluation.
    expression_start = text.index('pub fn eval_source_with_features_and_introspector(')
    expression_end = text.index('pub fn eval_with_full_error(', expression_start)
    expression = text[expression_start:expression_end]
    expression = replace_once(expression,
        '    let stdlib = make_stdlib_with_features(&inputs, features);',
        '    let lab_stdlib_snapshot = Arc::new(lab_stdlib_snapshot::StdlibSnapshot::new(\n'
        '        inputs.clone(), features, make_stdlib_with_features(&inputs, features),\n'
        '    ));\n'
        '    let stdlib = lab_stdlib_snapshot.scope();')
    expression = replace_once(expression, '    ctx.features = features;\n',
        '    ctx.features = features;\n'
        '    ctx.lab_stdlib_snapshot = Some(Arc::clone(&lab_stdlib_snapshot));\n')
    text = text[:expression_start] + expression + text[expression_end:]

    # One fresh template in the document's existing run_pass, not a cache of
    # completed Modules or a move into World/layout.
    text = replace_once(text,
        '        let stdlib = make_stdlib_with_features(&inputs, features);',
        '        let lab_stdlib_snapshot = Arc::new(lab_stdlib_snapshot::StdlibSnapshot::new(\n'
        '            inputs.clone(), features, make_stdlib_with_features(&inputs, features),\n'
        '        ));\n'
        '        ctx.lab_stdlib_snapshot = Some(Arc::clone(&lab_stdlib_snapshot));\n'
        '        let stdlib = lab_stdlib_snapshot.scope();')

    text += '''

/// Laboratory diagnostic only: exercises the same guarded helper as imports.
pub fn __lab_resolve_stdlib(
    seed_inputs: SysInputs,
    seed_features: crate::entities::compiler_features::Features,
    requested_inputs: SysInputs,
    requested_features: crate::entities::compiler_features::Features,
) -> Scope {
    let seed = Arc::new(lab_stdlib_snapshot::StdlibSnapshot::new(
        seed_inputs.clone(),
        seed_features,
        make_stdlib_with_features(&seed_inputs, seed_features),
    ));
    lab_stdlib_snapshot::resolve(Some(&seed), &requested_inputs, requested_features).0
}
'''

    imported = modules.read_text()
    imported = replace_once(imported,
        '    let stdlib = super::make_stdlib_with_features(&engine.world.inputs(), ctx.features);',
        '    let lab_inputs = engine.world.inputs();\n'
        '    let (stdlib, lab_stdlib_snapshot) = super::lab_stdlib_snapshot::resolve(\n'
        '        ctx.lab_stdlib_snapshot.as_ref(), &lab_inputs, ctx.features,\n'
        '    );')
    imported = replace_once(imported, '    local_ctx.features = ctx.features;\n',
        '    local_ctx.features = ctx.features;\n'
        '    local_ctx.lab_stdlib_snapshot = Some(lab_stdlib_snapshot);\n')

    implementation = (HERE / 'snapshot.rs').read_text()
    flag = {
        'share-identities': 'SHARE_IDENTITIES',
        'ignore-inputs': 'IGNORE_INPUTS',
        'ignore-features': 'IGNORE_FEATURES',
    }.get(args.mutation)
    if flag:
        implementation = replace_once(implementation,
            f'const {flag}: bool = false;', f'const {flag}: bool = true;')
    # Stage every validated edit before publishing; the active checkout never
    # appears among these paths and the received baseline is explicitly denied.
    helper.write_text(implementation)
    hub.write_text(text)
    modules.write_text(imported)
    print(f'Applied laboratory stdlib snapshot: mutation={args.mutation}')


if __name__ == '__main__':
    main()
