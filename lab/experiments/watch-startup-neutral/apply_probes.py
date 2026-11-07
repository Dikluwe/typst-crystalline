#!/usr/bin/env python3
"""Apply identical opt-in L3 probes after uninstrumented bas/neu/laz runs.

Clocks are aggregate, never per path/face. Does not select a timings/oracle
pipeline. All rewrites preserve expression order, short-circuiting and the
existing lock/fallback/postcheck boundaries. See README for overlapping scopes.
"""
import argparse
from _overlay import Edits, HERE, prepared_workspace

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

    implementation = (HERE / 'phase_probe.rs').read_text()
    if mutation == 'probe-invalid-label':
        implementation = implementation.replace('mark("event.cycle");', 'mark("event.INVALID_cycle");')
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


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('workspace')
    parser.add_argument('--mutation', default='none', choices=('none', 'probe-invalid-label'))
    args = parser.parse_args()
    edits = Edits(prepared_workspace(args.workspace))
    instrument(edits, args.mutation)
    edits.publish()
    print(f'Applied LAB L3 watch probes mutation={args.mutation}')


if __name__ == '__main__':
    main()
