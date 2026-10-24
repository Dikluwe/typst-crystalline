# Footnote marker identity — R1 — 2026-10-23

The generated decimal bracket prefix is corrected in both actual capture and
legacy/column flush. The entry now retains the same formatted marker as its
inline call. R1's reported `note` and `note-successor` text differences are gone;
all 60 distinct A controls match complete PDF page/word sequences in release,
debug and pinned vanilla. This certifies marker identity and formatting, not
complete note typography or placement parity.

## State and evidence

- HEAD `cf21f47a3b63f0e9fc5308925e1d0b67925e7227`, **uncommitted working tree**, preserving all incoming work.
  [Baseline](../../lab/results/footnote-markers-r1-2026-10-23/baseline.json)
  binds exact prior changed-file list, UTC, hashes and
  [actual source](../../lab/results/footnote-markers-r1-2026-10-23/baseline-source.json.gz).
- Final source-bound release build: `2026-10-24T00:34:10.380232+00:00` through `2026-10-24T00:40:14.491228+00:00`,
  SHA-256 `0edb076c23666c661df3792bc6bd28809f7b55ef853da21db5f93fd8d1075c2c`. Its
  [receipt](../../lab/results/footnote-markers-r1-final-2026-10-23/final-build/receipt.json)
  and [source archive](../../lab/results/footnote-markers-r1-final-2026-10-23/final-build/source.json.gz)
  record the exact full dirty stat/status, inputs, toolchain, recipe and timestamps.
  Measurements after that build use the same frozen source map.
- Fixed oracle commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree
  `cac836c07e4dec2a89219370428292393a3216b5`, executable SHA-256
  `9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`. No resync or version-string identity substitution.
- Only seven existing consumers and their seven owning L0s differ from the
  received productive/config/L0 state. The
  [audit](../../lab/results/footnote-markers-r1-final-2026-10-23/audit.json)
  lists every changed input. All 1853 font/reference assets remain byte-identical.
  Cargo ran sequentially with one job, 12 GiB maximum, 8 GiB high watermark and
  zero swap; commands and logs remain under the evidence root.

## Measurement, decision and implementation

The [before probes](../../lab/results/footnote-markers-r1-2026-10-23/before/receipt.json)
show `[1]` in the entry where vanilla exposes `1`; custom calls already render
`a` and `(ii)` while their entries incorrectly reconstruct `[1]` and `[2]`.
`footnote.rs` formats the causal counter correctly. The former reconstruction
sites were `footnote_fragments.rs:286` and `footnote_flush.rs:156` in the archived
baseline. Pinned upstream `typst-library/src/model/footnote.rs:305–338` realizes
the original note numbering for its entry. Visible number identity/format is a
language obligation; the private queue representation is mechanics.

Gate: ADR-0127 continuous parity correction. The L0 owners were amended and
sealed before code. No public entity, signature, flag, default path, compatibility
or pipeline phase changed. The call owner builds a generated prefix from its
already formatted native superscript Content and the existing word separator.
The private note tuple retains this prefix with its body and diagnostic number.
Capture consumes it directly; flush and all remainder branches carry it without
reformatting. Checkpoints clone/restore the whole tuple. Source cursors, capture
eligibility, certified cuts, warning strings and termination remain unchanged.
No exporter text substitution or normalization strips user brackets.

## Tests and validation

The [actual RED log](../../lab/results/footnote-markers-r1-2026-10-23/red-tests/test.log)
has four behavioral failures and one passing intentional-bracket control.
The [adjudication](../../lab/results/footnote-markers-r1-2026-10-23/red-adjudication.json)
retains the original worker's mistaken all-five-fail expectation separately.
An earlier missing test argument failed compilation and is not credited as RED.
The first GREEN attempt exposed a reversed coordinate assertion in the new test;
its correction follows existing native superscript baseline conventions, without
changing marker existence assertions. Both attempts and source archives remain.
The targeted [GREEN](../../lab/results/footnote-markers-r1-2026-10-23/green-tests/test.log)
passed all five tests. The final full suite also reruns these tests on the final state.

The [first full run](../../lab/results/footnote-markers-r1-2026-10-23/validation-first/tests-domain-infra.log)
exposed nine stale assertions: eight demanded the defective bracket prefix; the
column test's supposed inline filter actually admitted footer/body digits.
After their L0 amendments, marker checks require both call and entry and reject
generated brackets; the column test separately checks causal 1,2,3 sequences in
both regions, excluding numeric body text by semantic style. Existing body,
page, overflow, warning, overlap and no-note controls remain. No fixture was refreshed.

The first complete linter then rejected a silent wildcard in the new test text
collector (V16) and the column test's compound guard (V17). Those negative logs
remain in the original evidence root. After L0-first correction, all ignored
leaves are explicit and the same predicate executes inside the text branch.
The [source projection proof](../../lab/results/footnote-markers-r1-final-2026-10-23/test-form-projection.json)
confirms that only tests and two owning seals changed; non-test footnote code
is identical. Full lint passed before the final independent validation began.
The validation and runtime numbers below belong to that final state.

Final [validation](../../lab/results/footnote-markers-r1-final-2026-10-23/validation-complete/validation-commands.json):
**12/12 gates**, **9858 Rust tests passed, 7 ignored**;
34 Python CI tests passed. Build, strict/reverse lineage, workspace tests,
wiring tests, doctests, all targets, Clippy with warnings denied, formatting and
Rustdoc with warnings denied passed. The final full linter and diff check pass,
with no violations. Frozen inputs do not change during validation or measurements.

[Fresh replays](../../lab/results/footnote-markers-r1-final-2026-10-23/replays.json):
60 distinct A full-text/page matches; 28 B init observations; 40 C/D/F controls
preserving full line/page words and every bounding-box coordinate within the
unchanged 0.001 pt tolerance; 210 E raw outcomes preserved, including existing
errors/extensions; 98 H stdout controls. The legacy-extension comparator's
previously recorded raw exit 1 is unchanged policy debt, not newly declared green.

The [R1 PDF probes](../../lab/results/footnote-markers-r1-final-2026-10-23/after/receipt.json)
and [final debug probes](../../lab/results/footnote-markers-r1-final-2026-10-23/after-debug-final/receipt.json)
retain all ten exact fixtures, successful commands, words, bounds and binaries.
The original single-note and literal-bracket controls match full text/pages. Custom `a` and `(ii)`
each occur in both call and entry, preserving original body content; different
Poppler reading order for multiple superscript entries is retained rather than normalized.

## Resource preservation

Fresh [RSS](../../lab/results/footnote-markers-r1-final-2026-10-23/memory/receipt.json)
uses three randomized rounds of three fresh processes per role/workload, two
warmups, GNU time maxRSS in KiB and no competing build. Preservation requires
both round-median variations at most 3% and current/before-R1 median at most 1.03.
This checks retention of the accepted recipe, not a new causal G improvement or
latency claim. All seven original output geometries qualify at 0.001 pt.

| PDF | Before R1 KiB | Current KiB | Vanilla KiB | RSS preservation |
|---|---:|---:|---:|---|
| empty | 27820 | 27548 | 27328 | pass |
| hello | 34320 | 34072 | 30880 | pass |
| prose | 39104 | 39084 | 34744 | pass |
| math | 38200 | 38220 | 33812 | pass |
| table | 36780 | 36896 | 33668 | pass |
| multipage | 34696 | 34688 | 35980 | pass |
| context | 35484 | 35540 | 34808 | pass |

Normalized executable size: 50009456 bytes versus oracle 52271120,
passing the original G cap. Only disposable copies were stripped; the source-bound
product executable retains its original symbols.

## Explicit remaining limits

Marker geometry and entry typography remain divergent. For the exact `note`
fixture, the existing inline marker's PDF word y-min is about 25.032 pt versus
oracle 17.230 pt; the new entry reuses that native superscript mechanism, whose
raw FrameItem convention is not proof of PDF placement parity. Default body size,
entry indentation, spacing and marker/body bounds are not certified by matching
text. The exact pre/post/oracle bounds are retained in the R1 receipts. No global
superscript formula or note-entry style redesign was attempted under this text fix.

The nested probe still lacks INNER body, deferred still separates call/body,
split still omits END, and page-column placement differs. These were already
visible before R1 and remain open, with all raw sequences retained. Existing
hyphenation, supplementary math geometry/reflection, painting and resource gaps
R2–R7 remain as described in the [after-A–H diagnostic](vanilla-comparison-after-ah-2026-10-23.md).

Bateia was consulted read-only at the user's suggestion. No applicable footnote
marker mechanism was found; incidental historical LaTeX `footnotesize` evidence
is not a numbering design. No Bateia code was copied or changed. The
[consultation receipt](../../lab/results/footnote-markers-r1-2026-10-23/bateia-consultation.json)
records the sources and limits.
