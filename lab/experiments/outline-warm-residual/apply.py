#!/usr/bin/env python3
"""Reuse the joint witness overlay, adding only optional outline L3 probes.

All mutations are restricted to a separate prepared child of this task's RAM.
Prior modules are imported explicitly; their RAM constants are changed only in
this Python process, never in old files. No Cargo/build/test command is run.
"""
import argparse
import importlib.util
from pathlib import Path
import sys

HERE = Path(__file__).resolve().parent
PREVIOUS = HERE.parent / 'joint-font-witness'
RAM = Path('/dev/shm/tekt-outline-warm-20261106')


def load(name, path):
    specification = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(specification)
    saved_bytecode = sys.dont_write_bytecode
    sys.dont_write_bytecode = True
    try:
        specification.loader.exec_module(module)
    finally:
        sys.dont_write_bytecode = saved_bytecode
    return module


def previous_modules():
    guard = load('outline_previous_guard', PREVIOUS / '_overlay.py')
    guard.RAM = RAM
    saved = sys.modules.get('_overlay')
    sys.modules['_overlay'] = guard
    try:
        overlay = load('outline_previous_overlay', PREVIOUS / 'apply_overlay.py')
        probes = load('outline_previous_probes', PREVIOUS / 'apply_probes.py')
    finally:
        if saved is None:
            del sys.modules['_overlay']
        else:
            sys.modules['_overlay'] = saved
    return guard, overlay, probes


def stage_collector(guard):
    # Definitive standalone source for independent collector tests. Only its
    # RAM-installed copy is compiled by the pipeline; previous files stay fixed.
    return (HERE / 'phase_probe.rs').read_text()


def outline_probes(edits, guard):
    paged = '03_infra/src/pipeline/paged.rs'
    probe = 'crate::lab_watch_phase_probe'
    # These RAII scopes follow the existing t1/t2/t3/t4 phase boundaries;
    # early error returns still drop their currently active guard.
    edits.replace(paged, '    let t1 = Instant::now();',
        '    let t1 = Instant::now();\n'
        f'    let lab_introspect = {probe}::enter("pipeline.introspect");')
    edits.replace(paged, '    let t2 = Instant::now();',
        '    let t2 = Instant::now();\n    drop(lab_introspect);\n'
        f'    let lab_context = {probe}::enter("pipeline.context");')
    edits.replace(paged, '    let t3 = Instant::now();',
        '    let t3 = Instant::now();\n    drop(lab_context);\n'
        f'    let lab_layout = {probe}::enter("pipeline.layout");')
    edits.replace(paged, '    let t4 = Instant::now();',
        '    let t4 = Instant::now();\n    drop(lab_layout);\n'
        f'    let _lab_finish = {probe}::enter("pipeline.finish");')
    anchor = '    shared_metrics: Option<&FallbackFontMetrics<\'_>>,\n) -> SourceResult<(PagedDocument, Vec<SourceDiagnostic>)> {'
    edits.replace(paged, anchor,
        anchor + f'\n    let _lab_round = {probe}::enter("layout.round");')
    edits.replace(paged, '    for _ in 0..8 {\n        match layout_with_introspector_and_metrics_and_callbacks_from(',
        '    for lab_pass in 0..8 {\n'
        '        let lab_outcome = {\n'
        f'            let _lab_pass = {probe}::enter([\n'
        '                "layout.pass.1", "layout.pass.2", "layout.pass.3", "layout.pass.4",\n'
        '                "layout.pass.5", "layout.pass.6", "layout.pass.7", "layout.pass.8",\n'
        '            ][lab_pass]);\n'
        '            layout_with_introspector_and_metrics_and_callbacks_from(')
    edits.replace(paged,
        '            location_start,\n        ) {\n            LayoutPassOutcome::Complete(document) => return Ok((document, warnings)),',
        '            location_start,\n            )\n        };\n'
        '        match lab_outcome {\n'
        '            LayoutPassOutcome::Complete(document) => {\n'
        f'                {probe}::mark("event.layout.complete");\n'
        '                return Ok((document, warnings));\n            }')
    edits.replace(paged, '            LayoutPassOutcome::Pending(requests) => {',
        '            LayoutPassOutcome::Pending(requests) => {\n'
        f'                {probe}::mark("event.layout.pending");')
    context = '03_infra/src/pipeline/context_stabilization.rs'
    edits.replace(context, '        for attempt in 1..=5 {',
        '        for attempt in 1..=5 {\n'
        f'            let _lab_attempt = {probe}::enter([\n'
        '                "context.attempt.1", "context.attempt.2", "context.attempt.3",\n'
        '                "context.attempt.4", "context.attempt.5",\n'
        '            ][attempt - 1]);')
    # Productive constructor calls before cfg(test) only. Measure the actual
    # constructor expression, leaving I/O, cache hits and error conversion out.
    for relative, replacements in (
        ('03_infra/src/world/cache.rs', [
            ('Source::new(id, text)', f'{probe}::measure("source.construct", || Source::new(id, text))', 1),
            ('Source::new(self.id, text)', f'{probe}::measure("source.construct", || Source::new(self.id, text))', 1),
            ('Source::new_with_parser(\n            id,\n            text,\n            typst_core::compiler::parse::parse_code,\n        )',
             f'{probe}::measure("source.construct", || Source::new_with_parser(\n            id,\n            text,\n            typst_core::compiler::parse::parse_code,\n        ))', 1),
        ]),
        ('03_infra/src/world/source_session.rs', [
            ('Source::new(id, text)', f'{probe}::measure("source.construct", || Source::new(id, text))', 2),
        ]),
    ):
        path = guard.checked(edits.workspace, relative)
        source = path.read_text()
        before, separator, after = source.partition('#[cfg(test)]')
        for old, new, expected in replacements:
            if before.count(old) != expected:
                raise ValueError(f'Constructor anchor mismatch in {relative}: {old!r}')
            before = before.replace(old, new)
        edits.contents[path] = before + separator + after


def physical_probes(edits, guard):
    relative = '03_infra/src/pipeline/context_stabilization/page_running.rs'
    original = guard.checked(edits.workspace, relative).read_text()
    if '"physical.candidate"' in original or '"physical.step.discovery"' in original:
        raise ValueError('Physical probe complement is already installed')
    probe = 'crate::lab_watch_phase_probe'
    candidate = ('    pub(super) fn physical_candidate(\n'
                 '        &mut self,\n'
                 '        content: &Content,\n'
                 '        input: &Arc<TagIntrospector>,\n'
                 '    ) -> SourceResult<PhysicalCandidate> {')
    edits.replace(relative, candidate,
        candidate + f'\n        let _lab_candidate = {probe}::enter("physical.candidate");')
    step = ('        replay: bool,\n'
            '        numbering_range: Option<(NonZeroUsize, NonZeroUsize)>,\n'
            '    ) -> SourceResult<PhysicalStep> {')
    edits.replace(relative, step, step + '\n'
        f'        let _lab_step = {probe}::enter(if replay {{\n'
        '            "physical.step.replay"\n'
        '        } else { "physical.step.discovery" });')
    edits.replace(relative, '            let outcome = layout_with_page_running(',
        '            let outcome = {\n'
        f'                let _lab_layout = {probe}::enter(if replay {{\n'
        '                    "physical.layout.replay"\n'
        '                } else { "physical.layout.discovery" });\n'
        '                layout_with_page_running(')
    edits.replace(relative, '            .into_result();\n            match &outcome {',
        '                .into_result()\n'
        '            };\n'
        '            match &outcome {\n'
        f'                Ok(_) => {probe}::mark("event.physical.complete"),\n'
        f'                Err(_) => {probe}::mark("event.physical.pending"),\n'
        '            }\n'
        '            match &outcome {')


def physical_incremental(args, parser, guard, pipeline):
    if args.probes:
        parser.error('--physical-probes is a separate incremental step; omit --probes')
    helper = guard.checked(pipeline, '03_infra/src/lab_watch_phase_probe.rs')
    if not helper.is_file():
        parser.error('--physical-probes requires the existing general probes')
    manifest = guard.checked(pipeline, '03_infra/Cargo.toml').read_text()
    if args.variant == 'base':
        if args.engine or 'joint_font_witness' in manifest:
            parser.error('Base physical probes require the standalone base collector')
        if helper.read_bytes() != (HERE / 'phase_probe.rs').read_bytes():
            parser.error('Standalone collector differs from the definitive tested source')
    else:
        if not args.engine:
            parser.error('Joint physical probes require the existing --engine')
        engine = guard.prepared_workspace(args.engine)
        dependency = 'joint_font_witness = { path = "' + str(engine) + '" }'
        if dependency not in manifest:
            parser.error('Existing joint dependency does not match --engine')
        if guard.checked(engine, 'src/lab_phase_probe.rs').read_bytes() != (HERE / 'phase_probe.rs').read_bytes():
            parser.error('Shared collector differs from the definitive tested source')
    edits = guard.Edits(pipeline)
    physical_probes(edits, guard)
    edits.publish()
    print(f'Applied only physical L3 probe complement to {pipeline}')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('pipeline')
    parser.add_argument('--variant', required=True, choices=('base', 'joint'))
    parser.add_argument('--engine')
    parser.add_argument('--probes', action='store_true')
    parser.add_argument('--physical-probes', action='store_true')
    args = parser.parse_args()
    guard, overlay, previous_probes = previous_modules()
    pipeline = guard.prepared_workspace(args.pipeline)
    if not guard.checked(pipeline, '.cargo/config.toml').is_file():
        parser.error('Copy the complete received .cargo/config.toml first')
    if pipeline.relative_to(RAM.resolve()).parts[0].lower().startswith('reference'):
        parser.error('Reference workspaces are immutable')
    if args.physical_probes:
        physical_incremental(args, parser, guard, pipeline)
        return
    for helper in ('03_infra/src/lab_watch_phase_probe.rs', '01_core/src/compiler/eval/lab_stdlib_snapshot.rs'):
        if guard.checked(pipeline, helper).exists():
            parser.error('Pipeline already contains an experimental overlay')
    manifest = guard.checked(pipeline, '03_infra/Cargo.toml').read_text()
    has_joint = 'joint_font_witness' in manifest
    if 'joint_font_discovery' in manifest:
        parser.error('Pipeline contains an incompatible older joint overlay')
    if has_joint and not (args.variant == 'joint' and args.probes):
        parser.error('Joint overlay already installed; only its separate --probes step is accepted')
    first = pipeline.relative_to(RAM.resolve()).parts[0].lower()
    if first.startswith('reference'):
        parser.error('Reference workspaces are immutable')
    if args.variant == 'base' and args.engine:
        parser.error('--engine is only used by variant joint')
    if args.variant == 'joint':
        if not args.engine:
            parser.error('variant joint requires a separate --engine RAM path')
        engine = Path(args.engine).absolute()
        resolved_engine = engine.resolve()
        if (engine != resolved_engine or not resolved_engine.is_relative_to(RAM.resolve())
                or resolved_engine == RAM.resolve()
                or resolved_engine.relative_to(RAM.resolve()).parts[0].lower().startswith(('frozen', 'baseline', 'reference'))
                or resolved_engine.relative_to(RAM.resolve()).parts[0] == 'R'):
            parser.error('Engine must be a separate non-reference RAM child without symlinks')
        if has_joint:
            engine = guard.prepared_workspace(resolved_engine)
            dependency = 'joint_font_witness = { path = "' + str(engine) + '" }'
            patch = 'fontdb = { path = "' + str(engine / 'vendor/fontdb') + '" }'
            if dependency not in manifest or patch not in guard.checked(pipeline, 'Cargo.toml').read_text():
                parser.error('Installed joint dependency/patch does not match --engine')
            for relative in ('03_infra/src/fontdb/generation.rs', '03_infra/src/world/font_session.rs'):
                expected = (PREVIOUS / 'overlays' / relative).read_bytes()
                if guard.checked(pipeline, relative).read_bytes() != expected:
                    parser.error('Installed joint source differs from the nominal previous overlay')
            for relative in ('Cargo.toml', 'src/lib.rs', 'vendor/fontdb/Cargo.toml', 'vendor/fontdb/src/lib.rs'):
                if guard.checked(engine, relative).read_bytes() != (PREVIOUS / relative).read_bytes():
                    parser.error('Installed joint engine differs from the nominal previous source')
            if guard.checked(engine, 'src/lab_phase_probe.rs').exists():
                parser.error('Engine already contains probes; use an unprobed engine')
        else:
            saved = sys.argv
            try:
                sys.argv = [str(PREVIOUS / 'apply_overlay.py'), str(pipeline), args.engine]
                overlay.main()
            finally:
                sys.argv = saved
    if args.probes:
        edits = guard.Edits(pipeline)
        previous_probes.instrument(edits, 'none')
        engine_edits = None
        if args.variant == 'joint':
            engine_edits = guard.Edits(guard.prepared_workspace(args.engine))
            previous_probes.instrument_joint(edits, engine_edits, 'none')
            engine_edits.contents[guard.checked(engine_edits.workspace, 'src/lab_phase_probe.rs')] = stage_collector(guard)
        else:
            # Same original routes, no lab dependency or patched vendor.
            helper = guard.checked(pipeline, '03_infra/src/lab_watch_phase_probe.rs')
            edits.contents[helper] = stage_collector(guard)
        outline_probes(edits, guard)
        if engine_edits is not None:
            engine_edits.publish()
        edits.publish()
    print(f'Applied outline laboratory variant={args.variant} probes={args.probes} to {pipeline}')
    print('No build, lock reconciliation or configuration changes performed.')


if __name__ == '__main__':
    main()
