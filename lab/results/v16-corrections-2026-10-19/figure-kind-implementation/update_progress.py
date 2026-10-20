"""Consolidate the current diagnostic while preserving its full prior checkpoint."""
from pathlib import Path
import datetime
import hashlib
import json
import subprocess

ROOT=Path.cwd();P=Path(__file__).resolve().parent
ledger=json.loads((P/'closure-ledger.json').read_text())
assert ledger['closed_selected_ids']==['V16-055'] and ledger['remaining_v16']==18
path=ROOT/'00_nucleo/diagnostics/v16-correction-progress-2026-10-19.md'
old=path.read_bytes();sha=hashlib.sha256(old).hexdigest()
archive=P/'diagnostic-history'
assert not archive.exists();archive.mkdir()
(archive/'v16-correction-progress-before-figure-closure.md').write_bytes(old)
(archive/'provenance.json').write_text(json.dumps({'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'original_path':str(path.relative_to(ROOT)),'sha256':sha,'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'reason':'Consolidate current diagnostic after approved Figure closure; retain all earlier detailed status, acceptance and outstanding obligations.'},indent=2)+'\n')
path.write_text('''# V16 correction progress

Status: batches A–F are complete within their recorded acceptance scopes.
The full pinned linter reports **18 remaining V16 errors**, assigned to the
remaining batch G work. This is neither a zero-violation result nor complete
Typst language parity. No alerts, severities or contracts were weakened.

## Measurements and acceptance

Each linked report retains its baseline commit, exact dirty-state input hashes,
changed-file list, UTC, argv, binary identities, unsuccessful attempts and scoped
closure ledger. Counts below are linter findings, not a measure of full language
coverage. The linter remains pinned to commit
`12797073972e375d9ab078cf6750c8b30b71e2b9`, executable SHA-256
`421d0797e7ae1a5102e9c5bcd87b488fea70a1997394c38765a03239fe579121`.

| Accepted scope | V16 before → after | Evidence and limits |
|---|---|---|
| A: enum projections and adapters | 105 → 91 | [Report](../../lab/results/v16-corrections-2026-10-19/batch-a/README.md) |
| B: test observers and storage | 91 → 83 | [Report](../../lab/results/v16-corrections-2026-10-19/batch-b/README.md) |
| C: selectors, rich display and discarded effects | 83 → 64 | [Report](../../lab/results/v16-corrections-2026-10-19/batch-c/README.md), [open obligations](../../lab/results/v16-corrections-2026-10-19/batch-c/open-obligations.json) |
| D: grid, layout and bounded measurement | 64 → 44 | [Report](../../lab/results/v16-corrections-2026-10-19/batch-d/README.md) |
| E: reflection and serialization | 44 → 37 | [Report](../../lab/results/v16-corrections-2026-10-19/batch-e/README.md), [open obligations](../../lab/results/v16-corrections-2026-10-19/batch-e/open-obligations.json) |
| F: initial conversions and exhaustive decisions | 37 → 21 | [Partial checkpoint](../../lab/results/v16-corrections-2026-10-19/batch-f/README.md), [open obligations](../../lab/results/v16-corrections-2026-10-19/batch-f/open-obligations.json) |
| F: approved Circle origin and explicit radius | 21 → 20 | [Report](../../lab/results/v16-corrections-2026-10-19/circle-origin/README.md) |
| F: approved Figure/Grid and contextual Ref supplements | 20 → 18 | [Final report](../../lab/results/v16-corrections-2026-10-19/figure-kind-implementation/README.md), [closure ledger](../../lab/results/v16-corrections-2026-10-19/figure-kind-implementation/closure-ledger.json) |

The current Figure measurement starts at HEAD
`731f0ffdf4c7c152f590551e72ff63f7bd85ef85` on
`codex/tekt-mature-layout`, with the exact uncommitted implementation identified
by the final receipts. V16-055 is repaired; the associated substitution correction
also removes V16-100. The separate V16-099 collector is still open. All 1,606
outside-scope diagnostic objects match the baseline exactly. V19=1,315 and
V20=905 remain visible with a complete message/multiplicity/location review.

Figure's final source state passes all twelve integrated gates: canonical paths,
strict forward/reverse lineage, ownership/Nuclei, builds, domain/infra/wiring
tests, doctests, all targets, strict Clippy, formatting and Rustdoc. Actual enum
extensions produce all eight required E0004 witnesses. The frozen CLI policies
pass 84/84, including 77 required cases; five declared debts and two
negative controls remain distinct. All 40 required contextual-reference cases
pass across six matrices. Additional target controls pass 10/11; the failing
SetPage header remains open. Exact UTC, changed files, source/L0 and binary hashes
are linked from the report; policy acceptance is not complete oracle equality.

The first full-suite attempt exposed three obsolete phase fixtures. Their exact
sources and expectations were preserved; they now check the approved contextual
phase and canonical Ref identity instead of bypassing it. Original failures,
independent review, focused GREEN and the final complete run are all retained.

## Remaining obligations and organizational result

[The Figure gap ledger](../../lab/results/v16-corrections-2026-10-19/figure-kind-implementation/open-obligations.json)
retains configured SetPage headers, Block own-label projection, unrestricted Ref
show termination diagnostics, ambiguity diagnostics and the declared caption/style/HTML/Grid,
preview, Dynamic-field and physical-page boundaries. Native PageRun has separate
passing evidence; it does not certify repeated physical-page behavior. The
Circle report retains Auto/default and reflection/selector/show/set limits.
The math-cancel repair restores known style context locally; its existing script14pt versus pinned20pt and returned-Context conflict remains recorded for a separate decision. Earlier per-batch obligations remain binding and are not closed by this summary.
The private modern counter.final adapter now requires a Location before reading
or folding history, preserving demand tracking and measured context hints.
This repairs a styles-only false-zero result without changing public helpers;
math callback error anchors, here/get diagnostics and the ordinary outside-context
style hint difference remain open.
The parser comment for `a^b'_c^d` still requires an intent decision, and three
callback font formats still lack independent fixtures.

The reorganization helped trace findings to exclusive canonical L0 owners and
separate current specifications from historical evidence. No runtime or
development-speed improvement was measured. The same branch and Git ancestry
are preserved, with builds, probes, tests and detailed receipts under `lab/`.
The [complete previous diagnostic](../../lab/results/v16-corrections-2026-10-19/figure-kind-implementation/diagnostic-history/v16-correction-progress-before-figure-closure.md)
and its [hash/provenance](../../lab/results/v16-corrections-2026-10-19/figure-kind-implementation/diagnostic-history/provenance.json)
retain all earlier checkpoints. The original [cause-correction plan](v16-cause-correction-plan-2026-10-19.md)
remains the historical inventory and scheduling baseline, not authorization to
skip current L0 or outstanding contracts.
''')
print('Current diagnostic consolidated; previous SHA-256',sha)
