"""Batch-one authorized declaration generation; no oracle/product/runtime logic."""
import datetime
import hashlib
import json
import pathlib
import re
import subprocess

ROOT = pathlib.Path('/repos/Antigravity/typst-crystalline')
REG = pathlib.Path('/home/dikluwe/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f')
CORE = pathlib.Path('/home/dikluwe/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src')


def pin(path):
    return {'path': str(path), 'sha256': hashlib.sha256(path.read_bytes()).hexdigest()}


def declaration(path, name, kind=None):
    """Strip comments, retain actual declaration and source locations, never impls."""
    lines = path.read_text().splitlines()
    pattern = r'^(?:pub(?:\([^)]*\))?\s+)?(struct|enum|type|trait)\s+' + re.escape(name) + r'\b'
    found = [(i, re.match(pattern, line)) for i, line in enumerate(lines)]
    found = [(i, m) for i, m in found if m and (kind is None or m[1] == kind)]
    if len(found) != 1:
        raise ValueError((str(path), name, 'ambiguous/missing declaration', len(found)))
    first, match = found[0]
    depth = 0
    opened = False
    out = []
    for i in range(first, len(lines)):
        line = lines[i].split('//', 1)[0].rstrip()
        if line.strip():
            out.append({'line': i + 1, 'text': line})
        depth += line.count('{') - line.count('}')
        opened |= '{' in line
        if (opened and depth == 0) or (not opened and ';' in line):
            break
    else:
        raise ValueError(('unclosed', path, name))
    if match[1] == 'trait':
        # Retain only immediate associated declarations/signatures. Default method
        # bodies are not transmitted to the oracle author.
        signatures = [out[0]]
        level = 1
        pending = []
        for row in out[1:-1]:
            text = row['text']
            if level == 1 and (pending or re.match(r'^    (?:unsafe )?(?:fn|type|const)\b', text)):
                pending.append(row)
                if '{' in text or ';' in text:
                    if '{' in text:
                        pending[-1] = {**row, 'text': text.split('{', 1)[0].rstrip() + ';'}
                    signatures.extend(pending)
                    pending = []
            level += text.count('{') - text.count('}')
        if pending:
            raise ValueError(('unterminated signature', path, name))
        out = signatures + [out[-1]]
    return {**pin(path), 'qualified_name': None, 'name': name, 'kind': match[1],
            'first_line': first + 1, 'last_line': i + 1, 'declaration': out}


ROOTS = [
    ('crate::entities::layout_types::Ratio', '01_core/src/entities/layout_types.rs', 'Ratio'),
    ('crate::entities::layout_types::Angle', '01_core/src/entities/layout_types.rs', 'Angle'),
    ('crate::entities::location::Location', '01_core/src/entities/location.rs', 'Location'),
    ('crate::entities::bytes::Bytes', '01_core/src/entities/bytes.rs', 'Bytes'),
    ('crate::entities::world_types::Bytes', '01_core/src/entities/world_types.rs', 'Bytes'),
    ('crate::entities::element_payload::ElementPayload', '01_core/src/entities/element_payload.rs', 'ElementPayload'),
    ('crate::entities::counter_update::CounterUpdate', '01_core/src/entities/counter_update.rs', 'CounterUpdate'),
    ('crate::entities::elements::dynamic::DynElement', '01_core/src/entities/elements/dynamic.rs', 'DynElement'),
    ('crate::contracts::plugin_host::PluginHost', '01_core/src/contracts/plugin_host.rs', 'PluginHost'),
    ('crate::contracts::plugin_host::PluginModuleId', '01_core/src/contracts/plugin_host.rs', 'PluginModuleId'),
    ('crate::contracts::plugin_host::PluginError', '01_core/src/contracts/plugin_host.rs', 'PluginError'),
    ('crate::contracts::world::World', '01_core/src/contracts/world.rs', 'World'),
    ('crate::compiler::layout::metrics::FontMetrics', '01_core/src/compiler/layout/metrics.rs', 'FontMetrics'),
    ('crate::compiler::eval::EvalContext', '01_core/src/compiler/eval/mod.rs', 'EvalContext'),
    ('crate::compiler::scopes::Scopes', '01_core/src/compiler/scopes.rs', 'Scopes'),
    ('crate::entities::engine::Engine', '01_core/src/entities/engine.rs', 'Engine'),
    ('crate::entities::world_types::Engine', '01_core/src/entities/world_types.rs', 'Engine'),
    ('crate::entities::selector::Selector', '01_core/src/entities/selector.rs', 'Selector'),
    ('crate::entities::show::Selector', '01_core/src/entities/show.rs', 'Selector'),
    ('crate::entities::show::NodeKind', '01_core/src/entities/show.rs', 'NodeKind'),
    ('crate::entities::syntax_node::NodeKind', '01_core/src/entities/syntax_node.rs', 'NodeKind'),
    ('crate::entities::world_types::Datetime', '01_core/src/entities/world_types.rs', 'Datetime'),
    ('crate::entities::world_types::Route', '01_core/src/entities/world_types.rs', 'Route'),
]
EXTERNAL_ROOTS = [
    ('time::Date', 'time-0.3.47/src/date.rs', 'Date'),
    ('time::Time', 'time-0.3.47/src/time.rs', 'Time'),
    ('time::time::Padding', 'time-0.3.47/src/time.rs', 'Padding'),
    ('time::time::Hours', 'time-0.3.47/src/time.rs', 'Hours'),
    ('time::time::Minutes', 'time-0.3.47/src/time.rs', 'Minutes'),
    ('time::time::Seconds', 'time-0.3.47/src/time.rs', 'Seconds'),
    ('time::time::Nanoseconds', 'time-0.3.47/src/time.rs', 'Nanoseconds'),
    ('citationberg::IndependentStyle', 'citationberg-0.7.0/src/lib.rs', 'IndependentStyle'),
    ('comemo::Validate', 'comemo-0.4.0/src/track.rs', 'Validate'),
    ('comemo::constraint::ImmutableConstraint', 'comemo-0.4.0/src/constraint.rs', 'ImmutableConstraint'),
    ('comemo::constraint::ConstraintEntry', 'comemo-0.4.0/src/constraint.rs', 'ConstraintEntry'),
    ('comemo::constraint::EntryMap', 'comemo-0.4.0/src/constraint.rs', 'EntryMap'),
    ('rustc_hash::FxHashSet', 'rustc-hash-2.1.1/src/lib.rs', 'FxHashSet'),
]


def prepare():
    records = []
    for base, entries in [(ROOT, ROOTS), (REG, EXTERNAL_ROOTS)]:
        for qualified, path, name in entries:
            item = declaration(base / path, name)
            item['qualified_name'] = qualified
            records.append(item)
    return {
        'status': 'Nominal declaration supplement, batch-one preparation; no runtime execution or coverage credit',
        'ancestor': {'path': str(ROOT / '00_nucleo/diagnosticos/p1339-mutant-closed-state-inventory.json'),
                     'sha256': 'd0ab8a2a2297a8f8fcff36ac372a8b418368b814c92cf25184fbe17f7efd101f'},
        'declarations': records,
        'aliases': [
            {'token': 'CounterAction', 'target': 'crate::entities::counter_update::CounterUpdate',
             'use_site': '01_core/src/entities/content.rs:21'},
            {'token': 'Constraint', 'target': '<crate::entities::world_types::Route<\'static> as comemo::Validate>::Constraint',
             'kind': 'associated type, not a missing nominal struct',
             'use_site': '01_core/src/entities/world_types.rs:291',
             'resolved_nominal': 'comemo::internal::ImmutableConstraint<macro-local __ComemoCall>',
             'macro_source': pin(REG / 'comemo-macros-0.4.0/src/track.rs'),
             'macro_lines': [203,213,273,288,298,368,371],
             'tracked_methods': ['contains(&self, id: FileId) -> bool', 'within(&self, depth: usize) -> bool'],
             'derived_declaration': 'pub struct __ComemoCall(__ComemoVariant); enum __ComemoVariant { contains(FileId), within(usize) }',
             'derivation_status': 'Source-macro derivation from the two actual immutable tracked methods, not cargo-expanded output; verifier must audit edges.'},
            {'token': 'IndependentStyle', 'target': 'hayagriva::citationberg::IndependentStyle (citationberg 0.7.0)',
             'use_site': '01_core/src/entities/module.rs:19'},
            {'token': 'NonZeroU64', 'target': 'core::num::NonZero<u64>',
             'source': pin(CORE / 'num/nonzero.rs'),
             'declaration_macro_lines': [565, 2257, 2258, 2259],
             'fields': 'NonZero<T: ZeroablePrimitive>(T::NonZeroInner); u64 specialization stores a nonzero u64.'},
            {'token': 'AtomicUsize', 'target': 'core::sync::atomic::AtomicUsize',
             'source': pin(CORE / 'sync/atomic.rs'),
             'declaration_macro_lines': [2605, 3872, 3886],
             'fields': 'v: UnsafeCell<usize>',
             'classification': 'Mutable control carrier in Route.upper, not an immutable language leaf.'},
        ],
        'qualified_ambiguities': {
            'Engine': {'eval': 'crate::entities::engine::Engine',
                       'legacy_stub': 'crate::entities::world_types::Engine'},
            'Selector': {'Value': 'crate::entities::selector::Selector',
                         'ShowRule': 'crate::entities::show::Selector'},
            'NodeKind': {'show_selector': 'crate::entities::show::NodeKind',
                         'SyntaxNode': 'crate::entities::syntax_node::NodeKind'},
            'Bytes': {'Value': 'crate::entities::bytes::Bytes',
                      'World::file': 'crate::entities::world_types::Bytes'},
            'Date': {'Datetime': 'time::Date', 'CSL_formatting': 'citationberg::Date'},
        },
        'boundary_witness_inputs': [
            {'subject': 'DynElement', 'carrier': 'Content::Dynamic(Arc<dyn DynElement>)',
             'rule': 'Trait signatures do not prove structural sameness. dyn_eq, Debug, dynamic address, and shared Arc alone are not immutable causal provenance.',
             'real_fixture_constructor': 'Content::dynamic(CalloutElem::new(body, title, tone)) from existing cfg(test) element.'},
            {'subject': 'PluginHost/PluginModuleId',
             'rule': 'The u64 module token is closed data; it does not identify its host. Host methods can consult/mutate external state through &self. No unconditional identity proof from token equality or Arc address.'},
            {'subject': 'IndependentStyle',
             'rule': 'Actual CSL declaration fields retained. No inferred scalar/opaque leaf from its name or Eq derive. Module immutable-causal retention must be witnessed on the productive owner path; fresh external style structural equivalence is not asserted by this inventory.'},
            {'subject': 'time::Date', 'source': pin(REG / 'time-0.3.47/src/date.rs'),
             'field_witness': 'Single value: NonZero<i32>; derive PartialEq/Eq covers that packed integer, no hidden Value/Func/f64.',
             'source_lines': [47, 48, 54, 55]},
            {'subject': 'time::Time', 'source': pin(REG / 'time-0.3.47/src/time.rs'),
             'field_witness': 'Both endian declarations contain only bounded integer Hours/Minutes/Seconds/Nanoseconds plus one-variant Padding. Actual equality delegates to packed as_u64.',
             'source_lines': [30, 43, 51, 78, 91, 96, 116, 121],
             'limit': 'Field and equality witness inputs, not an executed comparator test or eligibility verdict.'},
        ],
        'limits': [
            'Fixes known token/name omissions by explicit paths; still not a Rust-compiler-resolved transitive graph.',
            'Type-names matching variants must not be removed from reachability in any future closure extraction.',
            'Every newly exposed nested field remains subject to the independent concrete-test or normative-structural-witness map.',
            'No candidate, no future execution credit, no changes to independently authored expectations.',
        ],
    }


if __name__ == '__main__':
    started = datetime.datetime.now(datetime.timezone.utc).isoformat()
    d = ROOT/'00_nucleo/diagnosticos'
    required = {
        'p1339-budget-redesign-r1.json':'4bdd984d7b9d482d68ebb787e98eeed8333a1d4447c637615d4ceb9c78ed6c19',
        'p1339-verifier-budget-redesign-acceptance-r1.md':'0552cdbfb02e18496ac3c07121e0640fad2a772aa32e678a0b7062344286bafd',
        'p1339-mutant-closed-state-inventory.json':'d0ab8a2a2297a8f8fcff36ac372a8b418368b814c92cf25184fbe17f7efd101f',
        'p1339-authority-manifest-r2.json':'842d6526739014022c073800148a47b3c886831e2198ab65bbfdbe73ce59411b',
        'p1339-l0-freeze.json':'397c136fc8710d44b2f7537193fe5b9c44ab89d40a99bf6b994296e05e7c4d04',
        'p1339-closed-harness-authority.json':'08d0be7f19894c111130d0c791df6804cd1e7d89d2b8d6f418702be88af2125d',
    }
    for name, expected in required.items():
        assert pin(d/name)['sha256'] == expected, name
    record = prepare()
    record.update({
        'schema':'p1339-qualified-baseline-inventory-supplement-v1',
        'executor':'/root/p1336_tests', 'role':'baseline declarative adapter preparation only',
        'regime':'executado sem atestação de isolamento',
        'inherited_context':'P1336; P1339 baseline permitted; no P1339 candidate exists or was read',
        'batch':1, 'started_at':started,
        'inputs':{name:pin(d/name) for name in required},
        'cargo_lock':pin(ROOT/'Cargo.lock'),
        'generator':pin(pathlib.Path(__file__)),
        'provenance':{},
        'generation_history':'One preparation-only in-memory extraction succeeded before publication; no focal/product process. This generator is the first publication of the qualified supplement.',
    })
    for key, argv in {
        'head':['git','rev-parse','HEAD'],
        'diff_stat':['git','diff','HEAD','--stat','--','.',':(exclude)00_nucleo/materialization',':(exclude)00_nucleo/context'],
        'working_tree':['git','status','--porcelain','--untracked-files=all','--','.',':(exclude)00_nucleo/materialization',':(exclude)00_nucleo/context'],
        'rustc':['rustc','-Vv'],
    }.items():
        p=subprocess.run(argv,cwd=ROOT,capture_output=True,text=True)
        assert p.returncode == 0
        record['provenance'][key]={'argv':argv,'exit':p.returncode,'stdout':p.stdout,'stderr':p.stderr}
    record['finished_at']=datetime.datetime.now(datetime.timezone.utc).isoformat()
    output=d/'p1339-mutant-closed-state-inventory-supplement.json'
    with output.open('x') as f:
        json.dump(record,f,indent=2,ensure_ascii=False)
        f.write('\n')
    print(json.dumps(pin(output)))
