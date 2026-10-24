# Pinned vanilla comparison after A–H — 2026-10-23

## Complete recomparison after R1–R7

The [new fixed-oracle comparison](vanilla-comparison-after-r1-r7-2026-10-24.md) is the current summary. It freshly
rebuilds the accepted release, repeats R1–R7 and A–H controls, full validation,
broader language/profile/operational probes, paired latency and RSS measurements.
The corrected symbol gate exits 0 with 162 MATCH and 36 EXTENSION in release/debug.
Remaining note/prose/math/layout/painting/resource observations are quantified
there. Productive sources, L0, the pinned oracle and historical evidence remain
unchanged. Earlier sections below retain their original measurement states.

## Subsequent R7 update

[R7 legacy symbol gate correction](legacy-symbol-gate-r7-2026-10-24.md) aligns
the diagnostic classifier with the already approved native-Symbol L0. Fresh
release and debug runs each exit 0: 162 MATCH and 36 EXTENSION across 198 cases.
Only the 12 stale bare-extension verdicts change; all compiler observations,
glyphs, warnings and oracle absence remain identical. Extensions remain
extensions. The earlier exit-1 receipts below remain historical evidence.
Productive sources, L0 and R6 executable bytes are unchanged.

## Subsequent R6 update

[R6 resource reduction](pdf-resources-r6-2026-10-24.md) accepts release opt-level
`s` after L0-first resource RED→GREEN and complete A–H/R1–R5 preservation.
Normalized executable bytes fall 14.65% and the local seven-workload
geometric mean peak RSS falls 5.22% versus received R5.
Immutable coverage clones share storage; symbols remain and the rejected PDF transfer is not shipped. Per-workload
RSS/oracle differences and timing tradeoffs remain explicit; universal resource
superiority is not certified. Earlier measurements below retain their snapshots.

**Subsequent R3 update:** [R3 geometry correction](math-geometry-r3-2026-10-23.md) resolves all seven original mathematical/variation-selector geometry differences. All 19 original render probes now qualify in full PDF bounds and SVG coordinates at 0.001 pt. Additional ordinary-text fallback-face and inline-after-pagebreak baseline limits remain documented; A–H/R1/R2 are preserved. Earlier measurements below retain their snapshots.

**Subsequent R2 update:** [explicit-hyphenation prose continuation](hyphenation-continuation-r2-2026-10-23.md) corrects both reported inputs, including zero tracking, at the unchanged 0.001 pt bbox limit. Justified punctuation overhang and the explicitly refused word/tracking contexts retain measured limits. The original A–H snapshot below remains preserved.

**Subsequent R1 update:** [footnote marker identity and formatting](footnote-markers-r1-2026-10-23.md) are corrected in a new source-bound run. All 60 A controls now match complete PDF page/word sequences. Note geometry and broader layout limits remain open. The A–H measurements below retain their original frozen snapshot.

The original A–H corrections remain valid within their recorded acceptance
scopes. All seven original PDF workloads now pass the unchanged 0.001 pt gate;
the original 16 public symbol carrier differences are gone. Remaining language,
rendering, resource and measurement-policy differences are listed below.
**The frozen pre-R7 snapshot was not an all-green raw run:** its symbol checker exits 1 for
12 legacy extensions because it still requires the pre-E `text` carrier.
The raw failure is retained and separately explained from the approved L0.

This diagnostic supersedes the pre-A–H comparison as the current summary;
it does not authorize productive code or replace any owning L0. The
[original comparison](../../lab/results/tekt-vs-vanilla-2026-10-23/README.md)
and individual batch receipts remain historical evidence, unchanged.

## Subsequent R5 update

The [R5 correction](painting-r5-2026-10-24.md) closes the reported Table painting
difference in PDF and SVG, both pages at72/600dpi. It also corrects signed/local
line coordinates and clipped paths, with four actual RED→GREEN regressions,
full final-source validation and preserved A–H/R1–R4 scopes. Additional differing-
stroke/gutter/filled-SVG controls, the two PUBLIC_FORMAT observations, R6/R7 and
prior R1–R4 layout limits remain explicit. Earlier sections retain their frozen
measurement states.

## State, provenance and method

- HEAD `cf21f47a3b63f0e9fc5308925e1d0b67925e7227`, branch `codex/tekt-mature-layout`,
  **uncommitted working tree**. Exact changed-file list (`git diff HEAD --stat`),
  status and 2162 source/config/L0 hashes are in [baseline](../../lab/results/tekt-vs-vanilla-after-ah-2026-10-23/baseline.json)
  and [build](../../lab/results/tekt-vs-vanilla-after-ah-2026-10-23/candidate-build/receipt.json); the
  [complete source archive](../../lab/results/tekt-vs-vanilla-after-ah-2026-10-23/candidate-build/source-state.json.gz)
  preserves actual bytes including untracked productive files.
- Release SHA-256 `a18ecc19a76863f6c07f6574c606b3c5c7d00ab99b4480a8763a2bc704c8aa32`. The fresh release build ran
  2026-10-23T23:04:51.212320+00:00 through 2026-10-23T23:08:28.960841+00:00, with no changed inputs, and reproduced
  the exact H artifact. The whole measurement window is
  2026-10-23T23:04:50.997497+00:00 through 2026-10-23T23:19:03.067902+00:00;
  the corrected HTML trace ran later, with its own UTC/binary receipt.
- Oracle commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree
  `cac836c07e4dec2a89219370428292393a3216b5`, SHA-256
  `9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`. Its ratified source/recipe receipt remains
  [the 2026-10-17 resync](../../lab/results/construction/diagnostics/vanilla-resync-2026-10-17/relatorio.md).
  No target update or version-string substitution occurred.
- Both use Rust 1.92.0 and their accepted ThinLTO/one-codegen-unit release
  recipes. Product symbols remain; size normalization strips disposable copies
  of both executables only. These compare actual products, not a causal
  architectural ablation. Performance uses 20 randomized paired fresh-process
  samples after two warmups per binary, both default affinity and one-CPU
  controls, with 4000 paired-bootstrap resamples. Credit requires the original
  output gate and lower 95% speed-ratio bound above 1.05.
- PDF acceptance retains full per-page words and all page/word bounds at
  0.001 pt. SVG and public-object comparisons retain their original criteria;
  pixels, font encoding, output bytes, cost and operational failures are
  separate observations (ADR-0107/0108). Identical errors, disabled cases and
  extensions are not promoted to language MATCH.
- Heavy commands ran sequentially with one Cargo job, MemoryMax 12 GiB,
  MemoryHigh 8 GiB and zero swap. Commands, worker/fixture/binary hashes, CPU,
  affinity, environment and timestamps remain in the receipts. All 1853
  font/reference assets and every productive/config/L0 input are unchanged.
  This update changes diagnostic documents and adds lab evidence only.

## Revalidation of the original batches

| Batch | Fresh observation | Current acceptance and retained boundary |
|---|---|---|
| A | 60 distinct weak/strong/parity/PageRun/unpainted/terminal-position cases in release and source-bound H debug; all page counts match vanilla | Original extra final pages remain fixed: multipage 20, context 10, direct/loop/newline 1. Full extracted text matches in 58; the two note-marker differences remain R1. |
| B | 28 init observations over both binaries: healthy, full and closed channels, valid/invalid templates | Checked Tekt notification failures exit 2; created projects survive; no panic captured. Healthy channels/order stay. Different success channels versus vanilla remain deliberate. |
| C | All 16 original prose controls pass full line/page words and 0.001 pt PDF bounds | Original prose: 12 pages, 1800 words, correct membership. Explicit hyphenation remains R2. |
| D | All 16 original math controls pass full line/page words and 0.001 pt PDF bounds | Original math: 6 pages, 525 words, continuation membership restored. Additional inline-math geometry remains R3. |
| E | 146 prior MATCH plus all 16 former carrier differences are now MATCH; 210 additional before/current raw outcomes are preserved | Original identity scope passes. This does not turn shared errors or extensions into parity. Additional public fields/constructors remain R4; the old extension checker is R7. |
| F | Original Hello/Table plus all 32 C/D controls pass every bounding-box coordinate at the original tolerance | Seven original PDFs have maximum delta 0.000020 pt. Table painting remains R5. |
| G | Seven fresh 3-round RSS replays improve over the frozen pre-G artifact; normalized executable passes its original oracle cap | Local RSS reduction 8.59–12.62%; higher RSS medians than vanilla on six controls and original symbol-bearing size remain R6. |
| H | 98 stdout controls plus actual short/long HTML syscall traces | Write/flush failures exit 2; warning order, healthy channels and timing boundaries preserved. Oracle still exits 0 for short late ENOSPC and 1 for long errors; this deliberate checked-I/O difference is not a failed parity claim. |

Detailed replay counts and accepted exceptions are bound by the
[audit](../../lab/results/tekt-vs-vanilla-after-ah-2026-10-23/audit.json), [batch driver](../../lab/results/tekt-vs-vanilla-after-ah-2026-10-23/batches.json),
[40 C/D/F replays](../../lab/results/tekt-vs-vanilla-after-ah-2026-10-23/preservation-final/receipt.json),
[init controls](../../lab/results/tekt-vs-vanilla-after-ah-2026-10-23/B/receipt.json) and
[HTML controls](../../lab/results/tekt-vs-vanilla-after-ah-2026-10-23/release-controls.json).

The unchanged qualification driver passed all **21 logical stages**, including
55 original fixtures, their negative-diagnostic policy, HTML/Length, underlines,
repeat, SVG/link and approved phase controls. The
[qualification receipt](../../lab/results/tekt-vs-vanilla-after-ah-2026-10-23/qualification/receipt.json) retains every stage.
The six broader profiles repeat 43 cases; they are not 258 distinct language
features. Forward/reverse order agrees; no VIOLATED or UNKNOWN occurred:

| Profile | MATCH | DIFFERENCE | Disabled | VIOLATED / UNKNOWN |
|---|---:|---:|---:|---:|
| default | 27 | 2 | 14 | 0 / 0 |
| html | 34 | 2 | 7 | 0 / 0 |
| a11y-extras | 28 | 2 | 13 | 0 / 0 |
| html+a11y | 35 | 2 | 6 | 0 / 0 |
| bundle | 31 | 2 | 10 | 0 / 0 |
| html+bundle | 40 | 2 | 1 | 0 / 0 |

The independent language probes retain 9 MATCH and 3 disabled. The raw symbol
probe reports **162 MATCH, 24 EXTENSION, 12 EXTENSION_DRIFT**, exit 1. No result
has been silently relabeled. The original 55/21 and matrix results coexist with
this explicit raw failure; there is no global parity percentage.

The full 9853-passing/7-existing-ignored Rust validation from H is **inherited
by exact source/config/L0 equivalence and byte-identical release**, not described
as a new complete Rust run here. This run freshly rebuilt the release, checked
strict/reverse lineage, repeated comparative and A–H controls, then performed
full crystalline lint and diff checks. Final outcomes and hashes are in
[completion](../../lab/results/tekt-vs-vanilla-after-ah-2026-10-23/completion.json).

## Latency and resources

All ten workloads pass the original output gate and the predeclared speed-gain
criterion in both CPU profiles. Default-affinity latency is below; speed ratio
is vanilla seconds / Tekt seconds, using the median within-pair ratio rather
than the ratio of marginal medians. This is bounded local evidence and gives no
claim about watch, incremental execution or arbitrary documents. Table still
has the separate pixel difference in R5.

| Workload | Tekt median ms | Vanilla median ms | Paired ratio (95% CI) |
|---|---:|---:|---|
| empty | 96.82 | 126.48 | 1.307 (1.300–1.310) |
| hello | 101.14 | 128.51 | 1.269 (1.263–1.283) |
| prose | 124.65 | 135.19 | 1.084 (1.075–1.090) |
| math | 124.78 | 133.14 | 1.071 (1.065–1.081) |
| table | 109.38 | 135.63 | 1.242 (1.229–1.258) |
| multipage | 104.38 | 131.31 | 1.252 (1.242–1.267) |
| context | 105.11 | 130.83 | 1.245 (1.233–1.253) |
| eval-startup | 3.82 | 5.90 | 1.561 (1.540–1.576) |
| eval-fold | 10.25 | 19.21 | 1.875 (1.824–1.911) |
| eval-map | 4.68 | 7.18 | 1.530 (1.494–1.627) |

PDF RSS below is the median of nine uninstrumented fresh processes per role,
three rounds × three replicates, two warmups, balanced randomized order and no
competing build. All before/current round-median variations meet the unchanged
3% repeatability bound. The frozen pre-G F artifact has its own source/recipe
receipt; this is a retention test of accepted G/H, not a new causal ablation.

| PDF | Pre-G MiB | Current MiB | Vanilla MiB | Pre-G→current reduction |
|---|---:|---:|---:|---:|
| empty | 31.01 | 27.09 | 26.67 | 12.62% |
| hello | 37.73 | 33.45 | 30.32 | 11.36% |
| prose | 42.24 | 37.80 | 34.02 | 10.52% |
| math | 41.02 | 37.50 | 33.47 | 8.59% |
| table | 39.35 | 35.81 | 32.65 | 8.99% |
| multipage | 37.88 | 33.85 | 35.22 | 10.62% |
| context | 38.75 | 34.64 | 34.46 | 10.59% |

The [paired latency receipt](../../lab/results/tekt-vs-vanilla-after-ah-2026-10-23/benchmark/receipt.json) retains every
sample and both affinity profiles; the [RSS receipt](../../lab/results/tekt-vs-vanilla-after-ah-2026-10-23/memory/receipt.json)
retains all rounds and source bindings. Executable bytes are separate from RSS:

| Size | Tekt MiB | Vanilla MiB |
|---|---:|---:|
| Original executable bytes | 52.13 | 49.85 |
| Both disposable copies stripped | 47.69 | 49.85 |

The [normalized artifact test](../../lab/results/tekt-vs-vanilla-after-ah-2026-10-23/final-artifact-test.json) is GREEN.
This does not imply smaller original bytes or lower PDF memory than vanilla.

## Measured remaining divergences

**R1 — Footnote marker identity: subsequently corrected.** The frozen A–H
run observed `[1]` versus `1` in `note` and `note-successor`; those original
[A observations](../../lab/results/tekt-vs-vanilla-after-ah-2026-10-23/A/receipt.json)
remain unchanged. The [R1 correction](footnote-markers-r1-2026-10-23.md) retains the
formatted marker through capture, flush and rollback, and all 60 A controls now
match complete text/pages without normalization. Native marker geometry, entry
typography and the documented broader note-layout limits remain uncertified.

**R2 — Original colon-bearing hyphenation continuation: subsequently corrected.**
The frozen [residual probes](../../lab/results/tekt-vs-vanilla-after-ah-2026-10-23/residuals/receipt.json)
record the original `hyphenation` and `zero-tracking-hyphenation` mismatch and
passing bold refuter. The [R2 correction](hyphenation-continuation-r2-2026-10-23.md)
admits terminal colon in the uniform hyphenated-word certificate. Both original
inputs now match complete line/page words and all bbox coordinates at 0.001 pt.
Justified punctuation overhang, internal/numeric/standalone colons and nonzero
tracking retain separately measured boundaries; they are not declared full parity.

**R3 — Original additional mathematical/variation-selector geometry: subsequently corrected.**
The frozen [19-case replay](../../lab/results/tekt-vs-vanilla-after-ah-2026-10-23/residuals/receipt.json)
retains the original seven failures: unicode at 2.136 pt, five standalone
inline-math controls at 0.9211875 pt and selected-multichar at 0.492 pt.
The [R3 correction](math-geometry-r3-2026-10-23.md) restores all 19 originals
at the unchanged 0.001 pt gate for complete PDF words/line/page/bounds and
independent SVG glyph/rule coordinates. It corrects normalized initial admission,
retained math-line text demands, native Symbol grapheme composition and text-mode
fallback activation; supplementary selectors no longer gain missing-glyph advance.
The source-bound added controls retain the separate isolated ordinary-text
fallback-face and inline-after-pagebreak baseline boundaries rather than claiming global rendering parity.

**R4 — Public mathematical fields and qualified constructor conversion — corrected.**
The [R4 correction](math-fields-r4-2026-10-23.md) restores Vec/Cases children
and explicit options, Class class/body and numeric MathText text, including
fields/has/at/direct lookup. Qualified content casts retain native Symbol and
accept their measured String/None domain; Attach slots retain Symbol, and Binom
lower uses display conversion. The expanded 210-case R4 corpus goes from 80
differences to zero in debug/release under its documented JSON/error criteria.
L0, actual RED→GREEN, full validation and A–H/R1–R3 preservation bind this state.
Matrix-symbol fields remain a passing control. Fourteen converted render forms
match explicit Content counterparts, whose fourteen layouts remain unchanged;
preexisting Accent/Class Symbol-body and nested Equation layout limits and
SVG/PDF ordering observations are recorded in the report. Unmeasured math reflection,
arbitrary display values and universal diagnostic-span parity are not certified.

**R5 — Original Table painting corrected; additional observations remain.**

The [R5 correction](painting-r5-2026-10-24.md) closes the reported two-page Table
paint in both targets at72/600dpi. The following numbers are the frozen pre-R5
snapshot, retained with their provenance. At 72 dpi all pages
of six original PDF workloads are pixel-identical; Table differs by 18 and 10
pixels on its two pages. At 600 dpi Hello's first page remains identical while
Table's first page differs by 1548 pixels, maximum channel delta 255. Table SVG
has the same glyph counts (332/198) but rule counts 42/26 versus 24/16. Text and
all word boxes pass; at that snapshot the precise painting cause/repair was unresolved. The six
feature profiles additionally retain `P1138-X-002` (11 pixels, maximum channel
delta 1) and `P1138-X-003` (Libertinus font naming/encoding suffix). Those two
recorded PUBLIC_FORMAT differences remain DIFFERENCE, not automatically
language defects or MATCH. See the
[painting replay](../../lab/results/tekt-vs-vanilla-after-ah-2026-10-23/residuals/receipt.json) and
[default matrix](../../lab/results/tekt-vs-vanilla-after-ah-2026-10-23/matrix-default.json).

**R6 — Resource reduction validated; bounded tradeoffs remain.**
The [R6 correction](pdf-resources-r6-2026-10-24.md) retains immutable coverage
sharing and release opt-level `s`. Normalized executable bytes fall
14.65% and the local seven-workload geometric peak-RSS median
falls 5.22% against received R5. Symbols remain;
the original and normalized executable are both below the pinned oracle in this
run. Per-workload RSS and elapsed-time costs remain explicit in the report;
no universal resource or speed superiority is established. The source-bound
full validation preserves every A–H/R1–R5 scope and residual observation.

**R7 — Closed: stale legacy-extension carrier gate.** The
[R7 correction](legacy-symbol-gate-r7-2026-10-24.md) replaces the obsolete
`func=text` extension expectation with the approved native `func=symbol`.
Eight RED→GREEN classifier tests retain rejection of incorrect identity,
glyphs, warnings and oracle failures. Fresh 198-case release/debug runs exit 0
with 162 MATCH and 36 EXTENSION; all raw compiler observations are unchanged.
The oracle still lacks these extensions. Historical exit-1 receipts and the
[earlier counterfactual classification](../../lab/results/tekt-vs-vanilla-after-ah-2026-10-23/symbols-contract-classification.json)
remain preserved. No productive or L0 change was needed.

## Operational differences and limitations

Init success remains stdout in Tekt and stderr in vanilla. Checked failing
streams, including HTML, continue using the existing Tekt operational exit 2
instead of reproducing oracle success, panic or signal termination. The fresh
[CLI](../../lab/results/tekt-vs-vanilla-after-ah-2026-10-23/robustness-cli/receipt.json),
[stdio/watch](../../lab/results/tekt-vs-vanilla-after-ah-2026-10-23/robustness-stdio-watch/receipt.json) and
[late-watch](../../lab/results/tekt-vs-vanilla-after-ah-2026-10-23/robustness-watch-late/receipt.json) observations preserve
these distinctions; healthy watch timeouts are harness termination, not program
failures. Failed-channel bytes cannot establish a captured panic. No global
panic-freedom or complete HTML/Typst coverage is claimed.

The first copied HTML trace adapter retained the old output directory. Only
newly created H-prefixed files were moved into a separate attempt archive; all
prior H evidence hashes were verified unchanged. The adapter was rebound and
only the trace rerun. The initial audit's missing-receipt error is not product
RED. Both attempts and the correction are retained in
[adapter correction](../../lab/results/tekt-vs-vanilla-after-ah-2026-10-23/trace-adapter-correction.json); the final
[actual HTML trace](../../lab/results/tekt-vs-vanilla-after-ah-2026-10-23/H-probe.json) uses the current receipt directory.
The standard runtime's cleanup retry after a detected failed flush remains
visible; the product exit is 2. No rollback/atomic-publication claim is added.
