#!/usr/bin/env python3
"""Apply aggregate opt-in probes after uninstrumented joint witness runs.

Clocks are aggregate, never per path/face. Does not select a timings/oracle
pipeline. All rewrites preserve expression order, short-circuiting and the
existing lock/fallback/postcheck boundaries. See README for overlapping scopes.
"""
import argparse
from _overlay import Edits, HERE, prepared_workspace, checked

GEN = '03_infra/src/fontdb/generation.rs'
SESSION = '03_infra/src/world/font_session.rs'
FONTS = '03_infra/src/world/fonts.rs'
DB = '03_infra/src/fontdb.rs'
PROBE = 'crate::lab_watch_phase_probe'


def instrument(edits, mutation):
    def replace(path, old, new):
        edits.replace(path, old, new)
    def entry(path, anchor, label):
        replace(path, anchor, anchor + f'\n    let _lab_phase = {PROBE}::enter("{label}");')
    def measured(label, expression):
        return f'{PROBE}::measure("{label}", || {expression})'
    def event(label):
        return f'{PROBE}::mark("event.session.{label}");'

    implementation = 'pub use joint_font_witness::lab_phase_probe::{enter, measure, mark, flush};\n'
    edits.new('03_infra/src/lab_watch_phase_probe.rs', implementation)
    replace('03_infra/src/lib.rs', 'mod bidi_context;\n',
            'mod bidi_context;\npub mod lab_watch_phase_probe;\n')

    # Session control flow: each wrapper evaluates its original expression
    # once. || and && remain outside wrappers, preserving short-circuiting.
    replace(SESSION, '    ) -> Arc<Catalog> {\n        let Ok(mut cached) = self.cached.lock() else {',
        '    ) -> Arc<Catalog> {\n'
        f'        let _lab_session = {PROBE}::enter("session.total");\n'
        f'        let Ok(mut cached) = {measured("session.lock_wait", "self.cached.lock()")} else {{\n'
        f'            {event("fallback_poisoned")}')
    replace(SESSION, '        if let Some(old) = cached.as_ref() {\n',
        f'        if cached.is_some() {{ {event("warm")} }} else {{ {event("cold")} }}\n'
        '        if let Some(old) = cached.as_ref() {\n')
    replace(SESSION, '            if old.fast.as_ref().is_some_and(&verify) && old.key.unchanged() {\n',
        f'            if {measured("session.initial_fast_verify", "old.fast.as_ref().is_some_and(&verify)")}\n'
        f'                && {measured("session.initial_key_check", "old.key.unchanged()")} {{\n'
        f'                {event("reuse_fast")}\n')
    replace(SESSION, '        let before = probe();',
        f'        let before = {measured("session.probe_before", "probe()")};')
    replace(SESSION, '        let Some(prepared) = prepare() else {',
        f'        let Some(prepared) = {measured("session.prepare", "prepare()")} else {{\n'
        f'            {event("fallback_prepare")}')
    replace(SESSION, '        if !prepared.unchanged() {',
        f'        if !{measured("session.prepared_check", "prepared.unchanged()")} {{\n'
        f'            {event("fallback_prepared_changed")}')
    replace(SESSION, '            if before.as_ref().is_none_or(&verify) {',
        f'            if {measured("session.reuse_verify", "before.as_ref().is_none_or(&verify)")} {{\n'
        f'                {event("reuse_prepared")}')
    replace(SESSION, '        let catalog = Arc::new(build(prepared));',
        f'        let catalog = Arc::new({measured("session.build", "build(prepared)")});')
    replace(SESSION, '        if !key.unchanged() || !before.as_ref().is_none_or(verify) {',
        f'        if !{measured("session.final_key_check", "key.unchanged()")}\n'
        f'            || !{measured("session.final_fast_verify", "before.as_ref().is_none_or(verify)")} {{\n'
        f'            {event("fallback_final_changed")}')
    replace(SESSION, '        *cached = Some(Cached { key, fast: before, catalog: catalog.clone() });',
        f'        {event("published_new")}\n'
        '        *cached = Some(Cached { key, fast: before, catalog: catalog.clone() });')

    entry(GEN, 'pub(crate) fn fast_key(paths: &[PathBuf]) -> Option<FastKey> {', 'fast.key')
    entry(GEN, 'pub(crate) fn fast_key_unchanged(old: &FastKey, paths: &[PathBuf]) -> bool {', 'fast.verify')
    entry(GEN, 'fn config_key(paths: &[PathBuf]) -> Option<ConfigKey> {', 'fast.config')
    entry(GEN, 'fn probe_roots(roots: &[PathBuf]) -> Option<ProbeKey> {', 'fast.walk_roots')
    replace(GEN, 'impl ProbeKey {\n    fn unchanged(&self) -> bool {',
        'impl ProbeKey {\n    fn unchanged(&self) -> bool {\n'
        f'        let _lab_phase = {PROBE}::enter("fast.roots_recheck");')
    replace(GEN, '    if config.files.iter().any(|(path, content)| {\n'
        '        std::fs::read(path).ok().as_deref() != Some(content.as_slice())\n'
        '    }) {',
        f'    if {PROBE}::measure("fast.config_bytes_recheck", || config.files.iter().any(|(path, content)| {{\n'
        '        std::fs::read(path).ok().as_deref() != Some(content.as_slice())\n'
        '    })) {')
    entry(GEN, 'pub(crate) fn prepare(paths: &[PathBuf]) -> Option<PreparedDiscovery> {', 'prepare.total')
    replace(GEN, '    db.load_system_fonts();',
        f'    {PROBE}::measure("discovery.fontdb_load_system", || db.load_system_fonts());')
    entry(GEN, '    bytes: usize,\n) -> Option<PreparedDiscovery> {', 'prepare.from_database')
    replace(GEN, '    let mut stamps = Vec::new();\n    let mut push =',
        f'    let lab_stamps = {PROBE}::enter("prepare.collect_stamps");\n'
        '    let mut stamps = Vec::new();\n    let mut push =')
    replace(GEN, '    let key = GenerationKey { paths: paths.to_vec(), stamps };',
        '    let key = GenerationKey { paths: paths.to_vec(), stamps };\n    drop(lab_stamps);')
    replace(GEN, 'impl GenerationKey {\n    pub(crate) fn unchanged(&self) -> bool {',
        'impl GenerationKey {\n    pub(crate) fn unchanged(&self) -> bool {\n'
        f'        let _lab_phase = {PROBE}::enter("generation.unchanged");')

    entry(FONTS, '    pub(super) fn font_catalog(&self) -> &(Vec<FontSlot>, FontBook) {', 'catalog.access')
    replace(FONTS, 'self.font_catalog.get_or_init(|| (self.font_loader)()).as_ref()',
        'self.font_catalog.get_or_init(|| {\n'
        f'            let _lab_phase = {PROBE}::enter("catalog.initialize");\n'
        '            (self.font_loader)()\n        }).as_ref()')
    entry(FONTS, 'pub(super) fn fresh_catalog(paths: &[PathBuf]) -> (Vec<FontSlot>, FontBook) {', 'catalog.fresh_build')
    entry(FONTS, '    prepared: crate::fontdb::generation::PreparedDiscovery,\n) -> (Vec<FontSlot>, FontBook) {', 'catalog.prepared_build')
    entry(FONTS, '    project: Vec<FontSlot>,\n) -> (Vec<FontSlot>, FontBook) {', 'catalog.compose')
    replace(FONTS, '    let (project_slots, project_book) = crate::fonts::pair_slots_with_book(project);',
        f'    let (project_slots, project_book) = {measured("catalog.project_book", "crate::fonts::pair_slots_with_book(project)")};')
    entry('03_infra/src/embedded_fonts.rs', 'pub fn load_embedded_fonts() -> EmbeddedFontSets {', 'catalog.embedded')
    entry('03_infra/src/fonts.rs', 'pub fn discover_fonts(font_paths: &[PathBuf]) -> Vec<FontSlot> {', 'project.discovery')
    replace(DB, '    db.load_system_fonts();',
        f'    {PROBE}::measure("discovery.fontdb_load_system", || db.load_system_fonts());')
    entry(DB, 'pub(crate) fn catalog_from_database(db: &fontdb::Database) -> (Vec<FontSlot>, FontBook) {',
          'catalog.acquire_selective_coverage')

    entry('03_infra/src/world/world_impl.rs', '    fn source(&self, id: FileId) -> FileResult<Source> {', 'source.access')
    replace('03_infra/src/pipeline/paged.rs', '    let t0 = Instant::now();',
        f'    let _lab_paged = {PROBE}::enter("pipeline.paged");\n'
        f'    let lab_eval = {PROBE}::enter("pipeline.eval");\n'
        '    let t0 = Instant::now();')
    replace('03_infra/src/pipeline/paged.rs', '    let t1 = Instant::now();',
        '    drop(lab_eval);\n    let t1 = Instant::now();')
    replace('03_infra/src/pipeline/pdf.rs', '    let t_render = Instant::now();',
        f'    let lab_export = {PROBE}::enter("pipeline.pdf_export");\n'
        '    let t_render = Instant::now();')
    replace('03_infra/src/pipeline/pdf.rs', '    let (pdf, subset_ms) = export.output;',
        '    let (pdf, subset_ms) = export.output;\n    drop(lab_export);')
    replace('04_wiring/src/compile.rs', '    Ok((exit_code, dependencies, published))',
        '    typst_infra::lab_watch_phase_probe::flush();\n'
        '    Ok((exit_code, dependencies, published))')


def instrument_joint(pipeline_edits, engine_edits, mutation):
    implementation = (HERE / 'phase_probe.rs').read_text()
    if mutation == 'probe-invalid-label':
        implementation = implementation.replace('mark("event.cycle");', 'mark("event.INVALID_cycle");')
    engine_edits.new('src/lab_phase_probe.rs', implementation)
    engine_edits.replace('src/lib.rs', 'use std::collections::HashMap;',
        'pub mod lab_phase_probe;\nuse std::collections::HashMap;')
    def ep(old, new):
        engine_edits.replace('src/lib.rs', old, new)
    def ee(anchor, label):
        ep(anchor, anchor + f'\n    let _lab_phase = crate::lab_phase_probe::enter("{label}");')
    text = checked(engine_edits.workspace, 'src/lib.rs').read_text()
    for function in ('capture_dirs_with_mode', 'capture_system_with_mode', 'capture_config_file_with_mode'):
        start = text.index('pub fn ' + function + '(')
        end = text.index(') -> Attempt {', start) + len(') -> Attempt {')
        ee(text[start:end], 'joint.capture')
    ee("        mode: CaptureMode,\n    ) -> Self {", 'joint.config_context')
    for call in ('db.load_fonts_dirs_observed(roots, &mut capture)',
                 'db.load_system_fonts_observed(&mut capture)',
                 'db.load_config_file_observed(path, &mut capture)'):
        ep(call + ';', f'crate::lab_phase_probe::measure("joint.discovery", || {call});')
    ep('if OMIT_SELFCHECK || validate(&evidence) {',
       'if OMIT_SELFCHECK || crate::lab_phase_probe::measure("joint.evidence_selfcheck", || validate(&evidence)) {')
    ep('        Attempt { db, evidence, stats: self.stats }',
       '        crate::lab_phase_probe::count("event.joint.capture_lstat", self.stats.capture_lstat);\n'
       '        crate::lab_phase_probe::count("event.joint.capture_stat", self.stats.capture_stat);\n'
       '        Attempt { db, evidence, stats: self.stats }')
    ee(') -> Option<(F, K, C)> {', 'joint.establish')
    ep('    let (fast, prepared) = discover()?;',
       '    let (fast, prepared) = crate::lab_phase_probe::measure("joint.establish_discover", discover)?;')
    ep('!check_prepared(&prepared)', '!crate::lab_phase_probe::measure("joint.g2", || check_prepared(&prepared))')
    ep('    let catalog = build(prepared);',
       '    let catalog = crate::lab_phase_probe::measure("joint.build", || build(prepared));')
    ep('!check_key(&key)', '!crate::lab_phase_probe::measure("joint.g3", || check_key(&key))')
    ep('!check_policy(&fast)', '!crate::lab_phase_probe::measure("joint.policy", || check_policy(&fast))')
    pe = pipeline_edits.replace
    # G1 remains exactly prepare_from, including all descriptor bounds and
    # unchanged() calls. The extra scope measures the complete preparation.
    pe(GEN, '    let prepared = prepare_from(attempt.db, paths, MAX_DESCRIPTORS, MAX_PATH_BYTES)?;',
       '    let prepared = crate::lab_watch_phase_probe::measure("joint.g1_prepare", || '
       'prepare_from(attempt.db, paths, MAX_DESCRIPTORS, MAX_PATH_BYTES))?;')
    pe(SESSION,
       '    fn catalog_joint(&self, paths: &[PathBuf], mode: joint_font_witness::CaptureMode) -> Arc<Catalog> {',
       '    fn catalog_joint(&self, paths: &[PathBuf], mode: joint_font_witness::CaptureMode) -> Arc<Catalog> {\n'
       '        let _joint_session = crate::lab_watch_phase_probe::enter("joint.session");')
    pe(SESSION, '        let mut cached = match self.cached.lock() {',
       '        let mut cached = match crate::lab_watch_phase_probe::measure("joint.lock_wait", || self.cached.lock()) {')
    pe(SESSION, '                drop(poison);\n                return self.catalog_standard(paths);',
       '                drop(poison);\n'
       '                crate::lab_watch_phase_probe::mark("event.joint.poisoned");\n'
       '                crate::lab_watch_phase_probe::mark("event.joint.fallback_standard");\n'
       '                return self.catalog_standard(paths);')
    pe(SESSION, '        if cached.is_none() {\n            let established =',
       '        if cached.is_none() {\n'
       '            crate::lab_watch_phase_probe::mark("event.joint.cold");\n'
       '            let established =')
    pe(SESSION, '            if let Some((fast, key, catalog)) = established {',
       '            if let Some((fast, key, catalog)) = established {\n'
       '                crate::lab_watch_phase_probe::mark("event.joint.established");')
    pe(SESSION, '                return catalog;\n            }\n        }\n        // Warm/refusal',
       '                return catalog;\n            }\n'
       '            crate::lab_watch_phase_probe::mark("event.joint.decline");\n'
       '        } else {\n'
       '            crate::lab_watch_phase_probe::mark("event.joint.warm");\n'
       '        }\n        // Warm/refusal')
    pe(SESSION, '        drop(cached);\n        self.catalog_standard(paths)',
       '        drop(cached);\n'
       '        crate::lab_watch_phase_probe::mark("event.joint.fallback_standard");\n'
       '        self.catalog_standard(paths)')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('workspace')
    parser.add_argument('engine')
    parser.add_argument('--mutation', default='none', choices=('none', 'probe-invalid-label'))
    args = parser.parse_args()
    edits = Edits(prepared_workspace(args.workspace))
    engine_edits = Edits(prepared_workspace(args.engine))
    instrument(edits, args.mutation)
    instrument_joint(edits, engine_edits, args.mutation)
    engine_edits.publish()
    edits.publish()
    print(f'Applied LAB L3 watch probes mutation={args.mutation}')


if __name__ == '__main__':
    main()
