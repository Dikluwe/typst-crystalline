#!/usr/bin/env python3
"""Apply the stdlib experiment only to C's separate RAM workspace copy.

No builds, active productive edits or L0 resealing. Runtime policy enters
through a laboratory-only World method, with environment access confined to L3. Mutations are compile-time laboratory choices for adversarial tests.
"""

import argparse
from pathlib import Path


RAM = Path('/dev/shm/tekt-stdlib-residual-20261105')
HERE = Path(__file__).resolve().parent


def replace_once(text, old, new):
    if text.count(old) != 1:
        raise ValueError(f'Expected exactly one unmodified anchor: {old[:100]!r}')
    return text.replace(old, new, 1)


def add_policy_and_probe(workspace, edits):
    def edit(relative, old, new):
        path = workspace / relative
        if not path.resolve().is_relative_to(workspace):
            raise ValueError('Overlay source path escapes RAM workspace')
        edits[path] = replace_once(edits.get(path, path.read_text()), old, new)
    edit('01_core/src/contracts/world.rs',
         'pub trait World: Send + Sync {',
         'pub trait World: Send + Sync {\n'
         '    /// LAB runtime control, not a proposed productive contract.\n'
         '    fn __lab_stdlib_policy(&self) -> u8 { 2 }\n')
    edit('03_infra/src/world/world_impl.rs',
         '    fn inputs(&self) -> SysInputs {',
         '    fn __lab_stdlib_policy(&self) -> u8 {\n'
         '        match std::env::var("TEKT_LAB_STDLIB_POLICY").as_deref() {\n'
         '            Ok("0") => 0, Ok("1") => 1, Ok("2") => 2, Ok("3") => 3,\n'
         '            Err(std::env::VarError::NotPresent) => 2,\n'
         '            _ => panic!("TEKT_LAB_STDLIB_POLICY must be 0, 1, 2 or 3"),\n'
         '        }\n'
         '    }\n\n    fn inputs(&self) -> SysInputs {')
    edit('03_infra/src/lib.rs', 'mod bidi_context;\n',
         'mod bidi_context;\npub mod lab_phase_probe;\n')
    probe = workspace / '03_infra/src/lab_phase_probe.rs'
    if not probe.resolve().is_relative_to(workspace):
        raise ValueError('Probe path escapes RAM workspace')
    if probe.exists() or probe.is_symlink():
        raise ValueError('Probe already exists')
    edits[probe] = (HERE / 'phase_probe.rs').read_text()
    edit('03_infra/src/world/fonts.rs',
         '    pub(super) fn font_catalog(&self) -> &(Vec<FontSlot>, FontBook) {',
         '    pub(super) fn font_catalog(&self) -> &(Vec<FontSlot>, FontBook) {\n'
         '        let _lab_phase = crate::lab_phase_probe::enter("font_catalog_access_inclusive");')
    # Includes fresh font discovery/acquisition and session freshness validation,
    # nested in catalog_access. Cache-hit access is counted separately above.
    edit('03_infra/src/world/fonts.rs',
         'self.font_catalog.get_or_init(|| (self.font_loader)()).as_ref()',
         'self.font_catalog.get_or_init(|| {\n'
         '            let _lab_phase = crate::lab_phase_probe::enter("font_catalog_initialize_inclusive");\n'
         '            (self.font_loader)()\n'
         '        }).as_ref()')
    edit('03_infra/src/world/world_impl.rs',
         '    fn source(&self, id: FileId) -> FileResult<Source> {',
         '    fn source(&self, id: FileId) -> FileResult<Source> {\n'
         '        let _lab_phase = crate::lab_phase_probe::enter("source_access_inclusive");')
    edit('03_infra/src/pipeline/paged.rs',
         '    let t0 = Instant::now();',
         '    let _lab_paged = crate::lab_phase_probe::enter("paged_pipeline_inclusive");\n'
         '    let lab_eval = crate::lab_phase_probe::enter("eval_to_module_inclusive");\n'
         '    let t0 = Instant::now();')
    edit('03_infra/src/pipeline/paged.rs',
         '    let t1 = Instant::now();',
         '    drop(lab_eval);\n    let t1 = Instant::now();')
    edit('03_infra/src/pipeline/pdf.rs',
         '    let t_render = Instant::now();',
         '    let lab_export = crate::lab_phase_probe::enter("pdf_export_dispatch_inclusive");\n'
         '    let t_render = Instant::now();')
    edit('03_infra/src/pipeline/pdf.rs',
         '    let (pdf, subset_ms) = export.output;',
         '    let (pdf, subset_ms) = export.output;\n    drop(lab_export);')
    edit('04_wiring/src/compile.rs',
         '    Ok((exit_code, dependencies, published))',
         '    typst_infra::lab_phase_probe::flush();\n'
         '    Ok((exit_code, dependencies, published))')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('workspace', type=Path)
    parser.add_argument('--mutation', default='none', choices=(
        'none', 'share-identities', 'ignore-inputs', 'ignore-features', 'force-eager-lazy',
    ))
    args = parser.parse_args()
    workspace = args.workspace.resolve()
    if not RAM.is_dir() or not workspace.is_dir():
        parser.error('Task RAM area and prepared workspace must already exist')
    if (not workspace.is_relative_to(RAM.resolve())
            or workspace == RAM.resolve()
            or workspace.is_relative_to((RAM / 'baseline').resolve())
            or workspace.is_relative_to((RAM / 'R').resolve())):
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
        '    lab_stdlib_snapshot: RefCell<Option<Arc<lab_stdlib_snapshot::StdlibSnapshot>>>,\n'
        '    lab_stdlib_policy: u8,\n')
    text = replace_once(text, '            context_reads: RefCell::new(ContextReads::new()),\n',
        '            lab_stdlib_snapshot: RefCell::new(None),\n'
        '            lab_stdlib_policy: 2,\n'
        '            context_reads: RefCell::new(ContextReads::new()),\n')

    # The Source/expression facade still queries inputs and composes at the same
    # point before its existing syntax/argument evaluation.
    expression_start = text.index('pub fn eval_source_with_features_and_introspector(')
    expression_end = text.index('pub fn eval_with_full_error(', expression_start)
    expression = text[expression_start:expression_end]
    expression = replace_once(expression,
        '    let stdlib = make_stdlib_with_features(&inputs, features);',
        '    let lab_policy = world.__lab_stdlib_policy();\n'
        '    let lab_root = lab_stdlib_snapshot::RootLibrary::new(&inputs, features, lab_policy);\n'
        '    let stdlib = lab_root.scope();')
    expression = replace_once(expression, '    ctx.features = features;\n',
        '    ctx.features = features;\n'
        '    ctx.lab_stdlib_policy = lab_policy;\n'
        '    *ctx.lab_stdlib_snapshot.borrow_mut() = lab_root.seed();\n')
    text = text[:expression_start] + expression + text[expression_end:]

    # One fresh template in the document's existing run_pass, not a cache of
    # completed Modules or a move into World/layout.
    text = replace_once(text,
        '        let stdlib = make_stdlib_with_features(&inputs, features);',
        '        let lab_root = lab_stdlib_snapshot::RootLibrary::new(&inputs, features, lab_policy);\n'
        '        ctx.lab_stdlib_policy = lab_policy;\n'
        '        *ctx.lab_stdlib_snapshot.borrow_mut() = lab_root.seed();\n'
        '        let stdlib = lab_root.scope();')
    # Read runtime policy once per root; never query inputs an extra time.
    text = replace_once(text, '    let run_pass = |apply_show_rules: bool,',
        '    let lab_policy = world.__lab_stdlib_policy();\n'
        '    let run_pass = |apply_show_rules: bool,')
    text = replace_once(text, '        self.inner.inputs()\n    }',
        '        self.inner.inputs()\n    }\n'
        '    fn __lab_stdlib_policy(&self) -> u8 { self.inner.__lab_stdlib_policy() }')
    text += '''

/// Diagnostic only; no candidate productive API is implied.
pub fn __lab_resolve_stdlib(
    seed_inputs: SysInputs, seed_features: crate::entities::compiler_features::Features,
    requested_inputs: SysInputs, requested_features: crate::entities::compiler_features::Features,
) -> Scope {
    __lab_resolve_stdlib_policy(seed_inputs, seed_features, requested_inputs, requested_features, 2)
}
pub fn __lab_resolve_stdlib_policy(
    seed_inputs: SysInputs, seed_features: crate::entities::compiler_features::Features,
    requested_inputs: SysInputs, requested_features: crate::entities::compiler_features::Features,
    policy: u8,
) -> Scope {
    let root = lab_stdlib_snapshot::RootLibrary::new(&seed_inputs, seed_features, policy);
    let slot = RefCell::new(root.seed());
    lab_stdlib_snapshot::resolve(&slot, &requested_inputs, requested_features, policy).0
}
'''

    imported = modules.read_text()
    imported = replace_once(imported,
        '    let stdlib = super::make_stdlib_with_features(&engine.world.inputs(), ctx.features);',
        '    let lab_inputs = engine.world.inputs();\n'
        '    let (stdlib, lab_stdlib_snapshot) = super::lab_stdlib_snapshot::resolve(\n'
        '        &ctx.lab_stdlib_snapshot, &lab_inputs, ctx.features, ctx.lab_stdlib_policy,\n'
        '    );')
    imported = replace_once(imported, '    local_ctx.features = ctx.features;\n',
        '    local_ctx.features = ctx.features;\n'
        '    local_ctx.lab_stdlib_policy = ctx.lab_stdlib_policy;\n'
        '    *local_ctx.lab_stdlib_snapshot.borrow_mut() = lab_stdlib_snapshot;\n')

    implementation = (HERE / 'snapshot.rs').read_text()
    flag = {
        'share-identities': 'SHARE_IDENTITIES',
        'ignore-inputs': 'IGNORE_INPUTS',
        'ignore-features': 'IGNORE_FEATURES',
        'force-eager-lazy': 'FORCE_EAGER_LAZY',
    }.get(args.mutation)
    if flag:
        implementation = replace_once(implementation,
            f'const {flag}: bool = false;', f'const {flag}: bool = true;')
    # Stage every validated edit before publishing; the active checkout never
    # appears among these paths and the received baseline is explicitly denied.
    edits = {hub: text, modules: imported, helper: implementation}
    add_policy_and_probe(workspace, edits)
    for path, content in edits.items():
        path.write_text(content)
    print(f'Applied laboratory stdlib snapshot: mutation={args.mutation}')


if __name__ == '__main__':
    main()
