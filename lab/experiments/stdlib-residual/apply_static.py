#!/usr/bin/env python3
"""Install the nominal lazy candidate into a separate prepared RAM workspace.

Laboratory only. Root base-scope construction stays literally unchanged. No
runtime selector, World method, environment access, phase probe or public portal
is installed. No builds, tests, active-product edits or L0 resealing are done.
"""

import argparse
from pathlib import Path


RAM = Path('/dev/shm/tekt-stdlib-residual-20261105')
HERE = Path(__file__).resolve().parent


def replace_once(text, old, new):
    if text.count(old) != 1:
        raise ValueError(f'Expected exactly one unmodified anchor: {old[:100]!r}')
    return text.replace(old, new, 1)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('workspace', type=Path)
    args = parser.parse_args()
    workspace = args.workspace.resolve()
    if not RAM.is_dir() or not workspace.is_dir():
        parser.error('Task RAM area and prepared workspace must already exist')
    ram = RAM.resolve()
    if (not workspace.is_relative_to(ram)
            or workspace == ram
            or workspace.is_relative_to((RAM / 'baseline').resolve())
            or workspace.is_relative_to((RAM / 'R').resolve())):
        parser.error('Only a separate child workspace under the exclusive task RAM area is allowed')
    hub = workspace / '01_core/src/compiler/eval/mod.rs'
    modules = workspace / '01_core/src/compiler/eval/modules.rs'
    helper = hub.parent / 'lab_stdlib_snapshot.rs'
    if any(not path.resolve().is_relative_to(workspace) for path in (hub, modules, helper)):
        parser.error('Source/helper paths may not escape the prepared RAM workspace')
    if helper.exists() or helper.is_symlink():
        parser.error('Workspace already contains a stdlib experiment')

    text = hub.read_text()
    text = replace_once(text, 'mod syntax_diagnostics;\n',
                        'mod syntax_diagnostics;\nmod lab_stdlib_snapshot;\n')
    text = replace_once(text, 'pub struct EvalContext {\n',
        'pub struct EvalContext {\n'
        '    lab_stdlib_snapshot: RefCell<Option<Arc<lab_stdlib_snapshot::StdlibSnapshot>>>,\n')
    text = replace_once(text, '            context_reads: RefCell::new(ContextReads::new()),\n',
        '            lab_stdlib_snapshot: RefCell::new(None),\n'
        '            context_reads: RefCell::new(ContextReads::new()),\n')
    # In particular, neither document nor expression root construction is
    # replaced: both retain the original builder and owned Scope local.

    imported = modules.read_text()
    imported = replace_once(imported,
        '    let stdlib = super::make_stdlib_with_features(&engine.world.inputs(), ctx.features);',
        '    let lab_inputs = engine.world.inputs();\n'
        '    let (stdlib, lab_stdlib_snapshot) = super::lab_stdlib_snapshot::resolve(\n'
        '        &ctx.lab_stdlib_snapshot, &lab_inputs, ctx.features, 3,\n'
        '    );')
    imported = replace_once(imported, '    local_ctx.features = ctx.features;\n',
        '    local_ctx.features = ctx.features;\n'
        '    *local_ctx.lab_stdlib_snapshot.borrow_mut() = lab_stdlib_snapshot;\n')
    implementation = (HERE / 'snapshot.rs').read_text()
    for flag in ('SHARE_IDENTITIES', 'IGNORE_INPUTS', 'IGNORE_FEATURES', 'FORCE_EAGER_LAZY'):
        if implementation.count(f'const {flag}: bool = false;') != 1:
            raise ValueError(f'Nominal helper must disable mutation {flag}')
    # Validate all anchors before any publication. The copied helper retains
    # unused private diagnostic machinery, but installs no public facade and
    # has only this constant-policy call site. Release DCE may remove it.
    edits = {helper: implementation, hub: text, modules: imported}
    for path, content in edits.items():
        path.write_text(content)
    print('Applied nominal laboratory lazy stdlib candidate (constant policy 3)')


if __name__ == '__main__':
    main()
