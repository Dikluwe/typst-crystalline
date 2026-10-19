from pathlib import Path
import json,gzip,hashlib,datetime,subprocess
b=Path('lab/results/v16-corrections-2026-10-19/batch-f');o=b/'agents/adapters';out=o/'version-ordering-repair';sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
paths=[Path('01_core/src/compiler/stdlib/collections/array_predicate.rs'),Path('00_nucleo/prompts/core/compiler/stdlib/collections/array_predicate.md'),Path('01_core/src/compiler/stdlib/collections/validation.rs'),Path('00_nucleo/prompts/core/compiler/stdlib/collections/validation.md')]
assert all(p.read_text()==json.loads(gzip.decompress((out/'before-sources.json.gz').read_bytes()))[str(p)] for p in paths)
failed=b/'version-ordering-failure/agents/adapters/accepted-replay/receipt.json';(out/'failed-replay.json').write_bytes(failed.read_bytes())
r={'utc_before':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'working_tree':'uncommitted working tree','diff_HEAD_stat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'before':{str(p):sha(p) for p in paths},'evidence':{str(p):sha(p) for p in [out/'before-sources.json.gz',out/'failed-replay.json',failed,o/'replay-manifest.json',o/'baseline.json',Path('01_core/src/entities/version.rs'),Path('00_nucleo/prompts/core/entities/version.md'),Path('01_core/src/compiler/eval/operators/ordering.rs'),Path('00_nucleo/prompts/core/compiler/eval/operators/ordering.md'),Path('lab/typst-original/crates/typst-library/src/foundations/ops.rs')]},'classification':'Private homogeneous Version comparison missing from collection comparator; existing domain Version::Ord provides exact zero-padded ordering; no entity/public/default/phase changes.','frozen_sorted_cases':['baseline.json::decimal-key','baseline.json::version-key','baseline.json::decimal-by','baseline-supplement.json::by-error','baseline-supplement.json::arguments-by-error'],'other_new_constructor_ordering_audit':{'Decimal':'Homogeneous comparison repaired; frozen decimal-key passes.','Version':'Frozen version-key sole failure; focused RED test now required.','Duration':'Domain ordering exists; native invocation residual prevents frozen map case from reaching ordering. No sorted Duration case in frozen corpus; general collection Duration ordering remains separate debt.','Array':'Domain lexicographic ordering exists but collection comparator lacks it. No sorted Array case in frozen corpus; constructor remains Bytes-only and collection Array ordering remains separate debt.','Regex,Selector,Stroke,Tiling,Label,Arguments,SpotColorant':'No homogeneous comparison branch in pinned ops::compare; admission does not imply sortable values.'},'stage':'L0 and test only; implementation paused for parent focused RED'}
paths[1].write_text(paths[1].read_text()+'''

## Version key ordering after constructor admission

### Evidence before decision

At HEAD `91ea6ee91105b97b1aedf172e34780dddde14c94`, uncommitted working tree,
`lab/results/v16-corrections-2026-10-19/batch-f/agents/adapters/version-ordering-repair/before.json`
records UTC, changed-file stat, source/owner snapshots and the failed accepted
replay. Frozen `baseline.json::version-key` requires `(10, 2).sorted(key: version)`
to return `(2, 10)`, matching the pinned oracle. The candidate instead reports
`cannot compare version and version`; the frozen expectation is unchanged.
`entities/version.rs:161-178` already defines total component-wise zero-padded
ordering, and `compiler/eval/operators/ordering.rs:125-127` uses that relation.
Pinned `foundations/ops.rs:482` also compares homogeneous Version values.
This missing collection branch is language ordering, not sorting mechanics.

### Bounded correction and acceptance

Add only homogeneous Version comparison using the existing domain ordering.
Preserve trailing-zero semantics, existing Decimal/Int/Float/Str branches, stable
key sorting, all validation/callback behavior and unsupported-pair diagnostics.
The existing higher-order sorted-key test must retain Decimal and additionally
assert the frozen Version result; parent must observe that Version assertion
fail before implementation. No expected-result relaxation or sorting by repr.
Duration, Array and other omitted comparison families are outside this correction
and remain separate obligations. No public contract, default, phase or domain
Version change is authorized. Reseal and revalidate this exact owner only through
the coordinated parent gates.
''')
paths[3].write_text(paths[3].read_text()+'''

### Sorted-key constructor integration regression

The failed accepted replay archived in
`lab/results/v16-corrections-2026-10-19/batch-f/agents/adapters/version-ordering-repair/before.json`
pins the source/owner state and shows that Version admission reaches a missing
comparison branch. Keep the existing Decimal sorted-key assertion and add
`(10, 2).sorted(key: version) == (2, 10)` to that same test function. The frozen
oracle expectation remains unchanged. Comparison belongs exclusively to
`00_nucleo/prompts/core/compiler/stdlib/collections/array_predicate.md`; this
owner adds integration acceptance without duplicating sorting or conversion.
Require focused semantic RED before the comparator correction and exact replay
afterward. This does not change constructor identity, admission or diagnostics.
''')
r['utc_L0_before_test']=datetime.datetime.now(datetime.timezone.utc).isoformat();r['L0_before_test']={str(p):sha(p) for p in [paths[1],paths[3]]}
s=paths[2].read_text();start=s.index('    fn batch_f_higher_order_sorted_key() {');end=s.index('\n    }',start);s=s[:end]+'''
        assert_eq!(
            evaluate("(10, 2).sorted(key: version) == (2, 10)"),
            Value::Bool(true)
        );'''+s[end:];paths[2].write_text(s)
assert sha(paths[0])==r['before'][str(paths[0])]
r['after_test_only']={str(p):sha(p) for p in paths};r['utc_after_test']=datetime.datetime.now(datetime.timezone.utc).isoformat();(out/'before.json').write_text(json.dumps(r,indent=2)+'\n');print('Tests ready; implementation unchanged')
