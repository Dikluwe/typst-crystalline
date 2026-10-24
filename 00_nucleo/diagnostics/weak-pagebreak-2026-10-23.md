# Weak pagebreak correction — 2026-10-23

Status: batch A / D1 complete on the source-bound debug candidate. This diagnostic
covers batch A / D1 of the [handoff](vanilla-divergence-handoff-2026-10-23.md).
It does not close batches B–H or certify general pagination.

## Evidence and provenance

All measurements use HEAD `cf21f47a3b63f0e9fc5308925e1d0b67925e7227` on
`codex/tekt-mature-layout`, **uncommitted working tree**. Existing changes are
preserved. Exact UTC intervals, complete `git diff HEAD --stat`, source/L0/config
hashes, commands, observers and executable hashes are in the receipts below.
The source archives include untracked productive inputs; HEAD alone cannot
reproduce these binaries.

- [Initial baseline and build linkage](../../lab/results/weak-pagebreak-2026-10-23/provenance-index.json).
  Both frozen executable hashes and every build input matched the handoff receipt
  before changes. Before probes use those frozen executables; their launch-tree
  stat is not a replacement for the executable's archived build source.
- [Initial bilateral measurements](../../lab/results/weak-pagebreak-2026-10-23/before-boundaries/receipt.json).
- [First candidate build, source state and failed textual gate](../../lab/results/weak-pagebreak-2026-10-23/validation.json),
  with [source archive](../../lab/results/weak-pagebreak-2026-10-23/final-source.json.gz).
- [Refuting controls](../../lab/results/weak-pagebreak-2026-10-23/edge-after/receipt.json)
  and [unpainted-material controls](../../lab/results/weak-pagebreak-2026-10-23/unpainted-before/receipt.json).
- [Final candidate and checks](../../lab/results/weak-pagebreak-2026-10-23/final-validation.json),
  with [complete measured source archive](../../lab/results/weak-pagebreak-2026-10-23/source-final.json.gz).
  The final worker checks that its input hashes remain unchanged throughout.

The unchanged vanilla oracle is commit
`bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree
`cac836c07e4dec2a89219370428292393a3216b5`, executable SHA-256
`9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`.
Candidate measurements use a newly built debug executable, bound to its receipt;
the frozen old release remains original evidence, not a corrected executable.

## Measured cause and correction

At the baseline, `compiler/layout/pagebreak.rs` ignores `weak` and opens an
unconditionally required successor. `finish.rs` only knows how to omit the
empty successor created by PageRun. Repeated and initial weak breaks therefore
also differ. PageRun's own terminal transition must cooperate with an already
opened optional successor.

The first candidate corrected visible examples but was refuted by further
controls: hidden text and explicit spacing, including zero spacing, can require
an unpainted page. An initially required page or a strong break's required
successor also survives weak parity adjustment. The pinned source documents
weak collapse at `crates/typst-library/src/layout/page.rs:554–557` and distinguishes
staged required pages and real flow runs at
`crates/typst-layout/src/pages/collect.rs:29–119`. These language observations
justify the contract; the upstream collection algorithm is not required.

The correction tracks optional weak successors separately from PageRun
boundaries. Feature adapters record actually admitted unpainted layout material
without inventing paint or changing spacing geometry. The witness stays isolated
from subframes and resets only after physical page commitment. Full checkpoints
restore both private facts. Terminal composition admits pending notes/floats
before deciding whether an optional successor still needs a page. Strong pages,
parity intermediates and deliberately empty PageRun bodies remain required.
A PageRun boundary can only consume a following break while it remains empty.

A further actual-source query refuted page counts alone: terminal metadata after
a weak break still reported provisional page2 in a one-page document. The
[position receipt](../../lab/results/weak-pagebreak-2026-10-23/positions-final/receipt.json)
and [terminal RED](../../lab/results/weak-pagebreak-2026-10-23/terminal-red.log)
retain that failure. The position owner now projects only the actual pending
private captures of an omitted terminal successor to the final effective page's
end, preserving identity/latest-write policy and sealing. It does not clamp maps
or reinterpret arbitrary discarded frames. Pinned `pages/collect.rs:11–14` and
`pages/mod.rs:236–243` document the terminal-tag destination. PageRun boundaries
use the same terminal projection.

Each affected consumer retains its exclusive L0 owner. L0 edits preceded code
and resealing; this is continuous flow under ADR-0127. Public entities, traits,
signatures, default modes and pipeline phases are unchanged.

## Validation and preserved failures

The original new tests failed semantically before implementation
([RED](../../lab/results/weak-pagebreak-2026-10-23/red.log)). The unpainted/parity
refuters were also made into failing tests before their correction
([second RED](../../lab/results/weak-pagebreak-2026-10-23/unpainted-red2.log)).
An intervening Rust fixture quoting error is retained separately as a compilation
failure and is not counted as semantic RED.

The first candidate's strict full-text gate failed on `note` and
`note-successor`: Tekt's existing footnote body marker is `[1]`, vanilla's is `1`.
Both before and after receipts preserve that difference. `footnote_flush.rs:156`
constructs the bracketed marker. This batch preserves the exact Tekt note text,
page membership and positions; it does not claim bilateral marker morphology
parity or normalize that failure away. The final receipt lists full-text
exceptions explicitly while checking every other page's text against vanilla.

The final candidate matches page count and per-page membership in 60 distinct
cases across [original controls](../../lab/results/weak-pagebreak-2026-10-23/after-sealed/receipt.json),
[boundary controls](../../lab/results/weak-pagebreak-2026-10-23/edge-sealed/receipt.json)
[unpainted controls](../../lab/results/weak-pagebreak-2026-10-23/unpainted-sealed/receipt.json)
and [terminal position controls](../../lab/results/weak-pagebreak-2026-10-23/positions-sealed/receipt.json).
Repeated Context/multipage controls across workers are counted once. Full
per-page extracted text matches in 58 cases; the two note-marker exceptions
above remain explicit. The multipage document changes from 21 to 20 pages,
Context from 11 to 10, and direct/loop/newline controls from 2 to 1, matching the
oracle. Strong breaks, parity intermediates, explicit empty pages and admitted
unpainted content remain present.

[Geometry observations](../../lab/results/weak-pagebreak-2026-10-23/geometry.json)
compare baseline/candidate PDF boxes only on prefix pages with identical text.
The maximum observed coordinate delta is 0 pt, including every retained page of
the multipage/Context/note fixtures. This is preservation evidence for this
correction, not arbitrary-resolution painting equality or closure of D6's
existing strict PDF gate.

The final workspace and doctest run reports **9,825 passed, 0 failed and seven
existing ignored tests**: 9,574 domain/infra, 246 wiring and five doctests. The
[command receipts](../../lab/results/weak-pagebreak-2026-10-23/validation/validation-commands.json)
and per-command logs bind these results to the same source state and intervals.
Nine new semantic regressions pass. Full workspace/all-targets builds, strict
Clippy (`-D warnings -D unfulfilled_lint_expectations`), rustdoc (`-D warnings`),
formatting, Python CI tests, path/layout checks, strict forward/Nucleus ownership
and reverse lineage all pass. Full crystalline-lint reports zero errors and
warnings; existing informational observations are retained. Source inputs were
unchanged across validation.

[Completion receipt](../../lab/results/weak-pagebreak-2026-10-23/completion.json)
binds all outcomes. The frozen corrected executable is
[`tekt-debug`](../../lab/results/weak-pagebreak-2026-10-23/tekt-debug), SHA-256
`a05d2aeb2eb09a23efc08c6fbd216112e8fdd8cdda9164b67bfc06c5a058d45c`.
The [patch against the original dirty baseline](../../lab/results/weak-pagebreak-2026-10-23/change-vs-dirty-baseline.patch)
is limited to eleven productive consumers and their eleven L0 owners. No commit
or push was made. Batch B / D2 (checked init notification writes) is next. No benchmark
threshold, oracle, assertion, warning or original receipt is weakened. An initial
validation preflight lacked `rsvg-convert` in the systemd service PATH; its failure
is retained, and the rerun supplies the installed observer PATH. Validation of
the refuted candidate was stopped and retained as superseded, not reported as
passing. All heavy commands run sequentially with one job, MemoryMax 12 GiB,
MemoryHigh 8 GiB and zero swap.

A final optional systemd/journal metadata read exceeded the automatic approval
review deadline. This did not affect any build, test or validation command; the
existing receipts preserve their exact intervals, commands and source states.
The timeout is retained in the completion receipt and is not a safety verdict.
