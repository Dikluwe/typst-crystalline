# Pinned vanilla comparison after R1–R7 — 2026-10-24

> Historical snapshot: the [comparison after R8–R10](vanilla-comparison-after-r8-r10-2026-10-24.md) is now current. Original note-layout and script residuals below were addressed later; the fresh report replays their scopes and lists additional limits. The earlier measurements below retain their original source state.

The reported repair scopes of **R1–R7 remain valid** in a fresh fixed-oracle
comparison. The stale legacy-symbol gate now exits 0 with **162 MATCH and
36 EXTENSION** in both release and debug. Extensions remain distinct from
vanilla parity. The original mathematical fields/conversion, continuation,
marker-text and painting corrections are retained; additional layout, painting
and resource observations remain below. No universal language, rendering or
performance parity is claimed.

Nine of ten workloads meet the unchanged latency advantage criterion in both CPU
profiles. Mathematical PDF remains slower (168.39 ms versus 137.56 ms by default);
prose/math peak RSS remain 6.34%/3.55% above vanilla. Substantial gradient-paint
limits also remain, separately from the small native-PNG and tiling residuals.

This is the current summary, superseding the [after-A–H summary](vanilla-comparison-after-ah-2026-10-23.md).
Original and individual R1–R7 evidence remains unchanged. This diagnostic does
not authorize productive implementation or change L0.

## State and measurement provenance

- HEAD `cf21f47a3b63f0e9fc5308925e1d0b67925e7227`, branch `codex/tekt-mature-layout`, **uncommitted working tree**.
  The [received baseline](../../lab/results/tekt-vs-vanilla-after-r1-r7-2026-10-24/baseline.json) records exact `git diff HEAD --stat`,
  status and all 2162 productive/config/L0 hashes; the
  [received source archive](../../lab/results/tekt-vs-vanilla-after-r1-r7-2026-10-24/received-source.json.gz) retains actual bytes.
- The [fresh release build](../../lab/results/tekt-vs-vanilla-after-r1-r7-2026-10-24/candidate-build/receipt.json) ran `2026-10-24T12:51:30.559815+00:00` through
  `2026-10-24T12:54:12.818811+00:00`, Rust 1.92.0, offline/locked, one Cargo job, accepted
  opt-level `s`, ThinLTO and one codegen unit. Its [source archive](../../lab/results/tekt-vs-vanilla-after-r1-r7-2026-10-24/candidate-build/source-state.json.gz)
  binds every input. Release SHA-256 `6ccf6b10b1dfb7d4443dda2ce9dd4d82837264e8fec2fdd1aec7762ba5f61d86` reproduces
  received R7 exactly. Debug `0ff7d7dc689a71f5caea518f7fc6f8a676251fc1112f070f69ba4e6a60c9caaa` retains the R6/R7 artifact;
  fresh workspace compilation and debug probes validate the same source state.
- Oracle commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree
  `cac836c07e4dec2a89219370428292393a3216b5`, executable SHA-256
  `9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`. Identity follows the
  [ratified source/tree/recipe receipt](../../lab/results/construction/diagnostics/vanilla-resync-2026-10-17/relatorio.md).
  No moving-main substitution, oracle update or version-only identification occurred.
- [Serial execution](../../lab/results/tekt-vs-vanilla-after-r1-r7-2026-10-24/complete-stages.json) ran `2026-10-24T12:51:24.317309+00:00`
  through `2026-10-24T13:19:42.814811+00:00`. [Effective caps](../../lab/results/tekt-vs-vanilla-after-r1-r7-2026-10-24/effective-caps.txt) are
  MemoryMax 12 GiB, MemoryHigh 8 GiB, swap zero. Builds do not compete with timing/RSS.
  Commands, UTC, source/worker/log/fixture/binary hashes are retained. The
  [worker archive](../../lab/results/tekt-vs-vanilla-after-r1-r7-2026-10-24/worker-source.json) and [origins/adaptations](../../lab/results/tekt-vs-vanilla-after-r1-r7-2026-10-24/worker-origins.json)
  distinguish current adapters from original tools. Diagnostic tools stay in excluded
  lab; no productive source, L0, seal, warning policy or acceptance threshold changed.

## Adapter failure and preserved recovery

The initial orchestrator retains its nonzero exit: the H worker expected
`final-build/source.json.gz`, while the fresh build archived sources under
`candidate-build/source-state.json.gz`. It stopped before any H case ran, so
later dependent replays and their prepared-paint audit had no outputs.
The [original attempt](../../lab/results/tekt-vs-vanilla-after-r1-r7-2026-10-24/replays-first-attempt.json) and raw stage logs
remain untouched. The [correction](../../lab/results/tekt-vs-vanilla-after-r1-r7-2026-10-24/archive-alias-correction.json) creates
a byte-identical alias to the actual fresh archive. Only pending stages resume
after timed measurements, avoiding competing work; the [recovery receipt](../../lab/results/tekt-vs-vanilla-after-r1-r7-2026-10-24/recovery.json)
records their actual timestamps. Final replay acceptance is read from the
recovered 21-stage receipt, while the original orchestration status remains visible.
No product, fixture, threshold or classifier changed during recovery.

## R1–R7 revalidation

| Scope | Fresh result | Retained boundary |
|---|---|---|
| R1 marker text | All 60 A controls match complete PDF page/word sequences; all ten note controls preserve their recorded output; custom markers remain in call and entry | Marker/entry geometry and broader note layout are separate below |
| R2 explicit-hyphenation continuation | Both original inputs qualify at 0.001 pt; 24 controls retain 17 full-geometry and 21 continuation qualifications; 14 boundary probes retain their partitions | Justified overhang, unsupported colon contexts and nonzero tracking remain |
| R3 math/variation selectors | All 19 originals qualify in PDF and SVG at 0.001 pt; all 47 original/additional outputs preserve their scope | Three additional fallback/baseline limits remain |
| R4 public fields/casts | 170 successful JSON matches and 40 bilateral rejections in the 210-case corpus, release/debug; fourteen converted forms equal explicit counterparts | Expected rejections are not successful language MATCH; three layout families and ordering observations remain |
| R5 PDF/SVG painting | Original Table difference stays closed; original seven workloads and seven qualified added controls retain zero changed pixels at observed resolutions, in release/debug/compact | Other stroke/gutter/fill, prepared-paint and PUBLIC_FORMAT observations remain |
| R6 resources | Accepted binary/recipe reproduced; fresh same-window RSS, latency, size and historical stress controls recorded below | Per-workload costs and timing tradeoffs remain explicit |
| R7 legacy-symbol gate | 198 cases in each binary: 162 MATCH, 36 EXTENSION, exit 0; eight classifier regression methods pass in CI | Oracle absence, native identity, exact glyphs and warning policy remain enforced |

The [preservation replay](../../lab/results/tekt-vs-vanilla-after-r1-r7-2026-10-24/replays.json) passes all 21 stages:
A 60, B 28 init observations, C/D/F 40 complete-word/bounds controls, E 210 raw
outcomes, H 98 stdout controls, font inventories, normalized size and R1–R5
render/reflection/refusal controls. These corpora overlap and are not summed as
unique language features. The [acceptance audit](../../lab/results/tekt-vs-vanilla-after-r1-r7-2026-10-24/audit.json) checks the
actual receipts, including known limits, rather than inferring success from dispatch.

The [fresh complete validation](../../lab/results/tekt-vs-vanilla-after-r1-r7-2026-10-24/validation-complete/validation-commands.json)
passes all 12 gates: 42 Python tests; layout and strict/reverse lineage; workspace
build; domain/infra and wiring tests; doctests; all-target build; Clippy with
`-D warnings -D unfulfilled_lint_expectations`; fmt; Rustdoc with warnings denied
and all four indexes. Rust: **9878 passed, 7 ignored, zero failed**.
Full crystalline lint has zero violations; diff check passes. The
[source freeze](../../lab/results/tekt-vs-vanilla-after-r1-r7-2026-10-24/validation-complete/validation-freeze.json) records zero input changes.

## Broader pinned comparison

The unchanged [qualification](../../lab/results/tekt-vs-vanilla-after-r1-r7-2026-10-24/qualification/receipt.json) passes its
21 logical stages, retaining the original 55-fixture inventory, negative-input
policy and HTML/Length/repeat/link/phase controls. Independent language probes
retain 9 MATCH, 3 disabled and zero VIOLATED. Six matrix profiles repeat the same
43-case inventory; disabled cases and shared errors do not establish language parity.

| Profile | MATCH | DIFFERENCE | Disabled | VIOLATED / UNKNOWN |
|---|---:|---:|---:|---:|
| [default](../../lab/results/tekt-vs-vanilla-after-r1-r7-2026-10-24/matrix-default.json) | 27 | 2 | 14 | 0 / 0 |
| [html](../../lab/results/tekt-vs-vanilla-after-r1-r7-2026-10-24/matrix-html.json) | 34 | 2 | 7 | 0 / 0 |
| [a11y-extras](../../lab/results/tekt-vs-vanilla-after-r1-r7-2026-10-24/matrix-a11y-extras.json) | 28 | 2 | 13 | 0 / 0 |
| [html+a11y](../../lab/results/tekt-vs-vanilla-after-r1-r7-2026-10-24/matrix-html+a11y.json) | 35 | 2 | 6 | 0 / 0 |
| [bundle](../../lab/results/tekt-vs-vanilla-after-r1-r7-2026-10-24/matrix-bundle.json) | 31 | 2 | 10 | 0 / 0 |
| [html+bundle](../../lab/results/tekt-vs-vanilla-after-r1-r7-2026-10-24/matrix-html+bundle.json) | 40 | 2 | 1 | 0 / 0 |

Forward/reverse matrix order agrees. The two PUBLIC_FORMAT differences remain
DIFFERENCE; the matrix is not represented as 100% language parity. Fresh
[CLI boundaries](../../lab/results/tekt-vs-vanilla-after-r1-r7-2026-10-24/robustness-cli/receipt.json),
[stdio/watch](../../lab/results/tekt-vs-vanilla-after-r1-r7-2026-10-24/robustness-stdio-watch/receipt.json) and
[late watch](../../lab/results/tekt-vs-vanilla-after-r1-r7-2026-10-24/robustness-watch-late/receipt.json) preserve measured
operational behavior. Init success channels differ; checked failing streams
use the accepted Tekt exit 2. Oracle success/panic/signal/error outcomes and
healthy-watch harness timeouts retain their individual classification. This is
not a global panic-freedom claim.

## Latency and resource costs

The [paired benchmark](../../lab/results/tekt-vs-vanilla-after-r1-r7-2026-10-24/benchmark/receipt.json) uses 20 randomized paired
fresh processes after two warmups per binary/workload/profile, 4000 paired-bootstrap
resamples, both default affinity and one-CPU controls. Credit requires the original
output gate and lower 95% speed-ratio bound above 1.05. Ratio is median within-pair
vanilla seconds / Tekt seconds, not a ratio of marginal medians. Below is default
affinity; every sample and the one-CPU results remain in the receipt.

| Workload | Tekt ms | Vanilla ms | Paired ratio (95% CI) | Measured direction |
|---|---:|---:|---|---|
| empty | 99.05 | 126.80 | 1.285 (1.269–1.287) | tekt_faster |
| hello | 101.78 | 128.54 | 1.263 (1.254–1.268) | tekt_faster |
| prose | 128.99 | 135.64 | 1.055 (1.053–1.067) | tekt_faster |
| math | 168.39 | 137.56 | 0.819 (0.815–0.823) | tekt_slower |
| table | 108.90 | 133.54 | 1.226 (1.211–1.232) | tekt_faster |
| multipage | 104.85 | 129.94 | 1.238 (1.229–1.248) | tekt_faster |
| context | 105.04 | 129.67 | 1.235 (1.227–1.241) | tekt_faster |
| eval-startup | 4.26 | 5.74 | 1.368 (1.326–1.388) | tekt_faster |
| eval-fold | 11.40 | 19.02 | 1.671 (1.610–1.722) | tekt_faster |
| eval-map | 5.22 | 7.25 | 1.393 (1.347–1.413) | tekt_faster |

default-affinity: faster `empty, hello, prose, table, multipage, context, eval-startup, eval-fold, eval-map`; slower `math`; inconclusive `none`.

single-cpu: faster `empty, hello, prose, table, multipage, context, eval-startup, eval-fold, eval-map`; slower `math`; inconclusive `none`.

These are local actual-product measurements, not architectural causal attribution,
watch/incremental results or arbitrary-document speed guarantees. All ten original
output gates remain qualified.

[Fresh R6 resource replay](../../lab/results/tekt-vs-vanilla-after-r1-r7-2026-10-24/final-resources/receipt.json) retains the exact
R6 method: three randomized rounds × three fresh processes per role/workload,
two warmups, GNU time peak RSS/elapsed, same normalized-size/RSS/repeatability gates.
The [historical control provenance](../../lab/results/tekt-vs-vanilla-after-r1-r7-2026-10-24/resource-control-provenance.json)
binds received R5 and recipe/coverage variants to their source-bound R6 evidence.
Geometric mean current/received-R5 peak RSS ratio: **0.949223**;
normalized-size ratio: **0.853467**. Raw tests:
`{"all_repeatable_and_preserved": true, "math_elapsed_below_z_shared": true, "oracle_normalized_cap": true, "rss_reduction": true, "size_reduction": true}`.

| PDF | Received R5 KiB | Current KiB | Vanilla KiB | Current / vanilla |
|---|---:|---:|---:|---:|
| empty | 27888 | 26828 | 27264 | 0.984 |
| hello | 32136 | 30432 | 31356 | 0.971 |
| prose | 36780 | 37228 | 35008 | 1.063 |
| math | 38472 | 35360 | 34148 | 1.035 |
| table | 34848 | 31820 | 33796 | 0.942 |
| multipage | 32624 | 30972 | 35384 | 0.875 |
| context | 33312 | 31484 | 34940 | 0.901 |

| Executable | Current bytes | Vanilla bytes |
|---|---:|---:|
| Original, symbols retained | 50613888 | 52271128 |
| Disposable copies stripped | 42700720 | 52271120 |

[Historical stress replays](../../lab/results/tekt-vs-vanilla-after-r1-r7-2026-10-24/legacy-resources/receipt.json) retain exact
word/page bounds and record all three local-set/variant controls. Fresh results
are not substituted into or attributed to the historical Oct10/R6 measurements.
Stress controls passing every original geometry/RSS/repeatability gate: `local-set-400, local-set-1600, variants-1600`. Raw gate booleans and ratios remain in the acceptance audit.
Identical current product bytes do not promise identical RSS or latency in every run.

## Remaining measured divergences

1. **Note geometry and broader layout (R1 residual).** The [ten-note replay](../../lab/results/tekt-vs-vanilla-after-r1-r7-2026-10-24/R1-preservation/receipt.json)
   retains native superscript/entry bounds, typography, indentation and placement differences.
   In `note`, maximum word-box delta is 9.611034 pt; inline marker y-min is
   25.032108 pt versus vanilla 17.229893 pt; entry marker x-min is 20 pt versus 29.35 pt.
   Raw extracted page/word sequences differ in `two, custom, columns, page-columns, nested, deferred, split`.
   Multiple-entry `two`/`custom` extraction order is retained as a mechanical
   observation rather than inferred content loss. Nested INNER body,
   deferred call/body grouping, split END and page-column placement
   retain their measured failures; corrected marker text does not certify layout.
2. **Prose boundaries (R2 residual).** The [24 controls](../../lab/results/tekt-vs-vanilla-after-r1-r7-2026-10-24/after/receipt.json)
   retain nonqualifying geometry in `simple-zeroFalse-justifytrue, simple-zeroTrue-justifytrue, optimized-zeroFalse-justifytrue, optimized-zeroTrue-justifytrue, internal, numeric, standalone`. Justified variants match
   continuation but retain up to 2.797267 pt of punctuation-overhang difference; internal/numeric/standalone
   colon and [nonzero-tracking boundary cases](../../lab/results/tekt-vs-vanilla-after-r1-r7-2026-10-24/boundary/receipt.json) retain
   the refused fallback and continuation limits. `overhang:false` refuters remain qualified.
3. **Additional math geometry (R3 residual).** In the [47-case corpus](../../lab/results/tekt-vs-vanilla-after-r1-r7-2026-10-24/R3/receipt.json),
   `control-markup-base, control-markup-vs, control-math-explicit-break` retain ordinary-text fallback-face and inline-after-pagebreak
   baseline differences. All nineteen originally reported controls remain qualified;
   selector fixes do not establish arbitrary fallback/shaping parity.
4. **Native-body/Equation layout (R4 residual).** The [render audit](../../lab/results/tekt-vs-vanilla-after-r1-r7-2026-10-24/render-audit.json)
   retains `accent, class, equation` as preexisting
   geometry limits. Root SVG paint order and Scripts PDF extraction line order remain
   separate mechanical observations despite equal physical geometry. Physical comparisons
   do not replace the strict ordered gate. Arbitrary display values and universal
   invalid-call span/diagnostic parity remain uncertified.
5. **Additional painting and public formats (R5 residual).** The [painting replay](../../lab/results/tekt-vs-vanilla-after-r1-r7-2026-10-24/after-release/receipt.json)
   retains differences for `different, fill, gutter`; exact per-page pixel counts and channel
   deltas remain recorded. [Prepared gradients/tiling](../../lab/results/tekt-vs-vanilla-after-r1-r7-2026-10-24/prepared-paint-audit.json)
   retain substantial gradient color/paint differences; `gradient-table` reaches
   107172 changed SVG pixels at 600 dpi, maximum channel delta 255.
   Tiling residuals are smaller: SVG at 600 dpi differs by 105/38 pixels
   for `tiling-line`/`tiling-local`, each maximum channel delta 1. Native PNG retains
   `11` changed pixels, maximum channel delta
   `1`. Matrix P1138-X-002 and P1138-X-003 retain
   PNG and Libertinus font-name/encoding observations under PUBLIC_FORMAT, not new
   semantic MATCH or automatic language defects. All seven original workloads are
   raster-tested at 72 dpi; Hello/Table and the added painting controls also at 600 dpi,
   in both PDF and SVG. No unmeasured resolution is certified.
6. **Resource/timing tradeoffs (R6 residual).** The fresh workload table and paired
   confidence intervals above retain every higher-RSS or slower/inconclusive observation.
   Prose/math current/oracle RSS ratios are 1.06341 and 1.03549. Math paired speed
   ratio is 0.8191 (95% CI 0.8147–0.8234) by default and 0.8302
   (0.8266–0.8342) on one CPU, a stable slowdown in both profiles.
   Lower normalized executable size and a lower geometric mean versus R5 do not imply
   universal memory or speed superiority over vanilla. See raw R6 test booleans before
   treating this local replay as reproducing every original performance gate.

## Exact remaining painting observations

Changed pixels by target/resolution; all listed fixtures have equal raster sizes.
These independent render observations retain their raw channel deltas in the receipts.

| Fixture | PDF 72 dpi | SVG 72 dpi | PDF 600 dpi | SVG 600 dpi | Maximum channel delta |
|---|---:|---:|---:|---:|---:|
| different | 26 | 172 | 1704 | 2911 | 255 |
| fill | 0 | 0 | 0 | 12 | 1 |
| gutter | 562 | 1006 | 33664 | 37241 | 255 |
| gradient-line | 556 | 473 | 32490 | 28178 | 148 |
| gradient-table | 1482 | 1968 | 102675 | 107172 | 255 |
| tiling-line | 0 | 0 | 0 | 105 | 1 |
| tiling-local | 0 | 0 | 0 | 38 | 1 |

**R7 has no remaining failure in its stated gate scope.** Legacy extensions are
intentionally absent from the pinned oracle and still classified EXTENSION. Text
with an equal glyph, wrong payloads/warnings and unexpected oracle outcomes remain
negative controls. The earlier raw exit-1 receipts are untouched; the new exit-0
result is from the corrected approved-contract gate, not a rewritten historical run.

The [fresh residual inventory](../../lab/results/tekt-vs-vanilla-after-r1-r7-2026-10-24/remaining.json) records exact observations
and [raw original residual probes](../../lab/results/tekt-vs-vanilla-after-r1-r7-2026-10-24/residuals/receipt.json) separately replay
former R2/R3/R4/R5 witnesses. This finite inventory can be refuted by the same exact
inputs qualifying under the unchanged observers in a future source-bound run. It
neither proves exhaustive absence of other divergences nor authorizes copying
vanilla implementation mechanics or accidental failures (ADR-0107/0108).

## Final preservation receipt

The [completion seal](../../lab/results/tekt-vs-vanilla-after-r1-r7-2026-10-24/completion.json)
records final document/evidence hashes, exact dirty-tree status and diff, unchanged
2162 productive/config/L0 inputs, original R7 lab-contract hashes and oracle/product
identity. The acceptance audit verifies all 1853 font/reference assets. All 13303
historical after-A–H/R6/R7 evidence files retain their sealed hashes (and sizes
where supplied). Current indexes and the handoff point here; earlier measurements
and their failed attempts remain recoverable. The
[report review receipt](../../lab/results/tekt-vs-vanilla-after-r1-r7-2026-10-24/report-review.json)
records the numerical/editorial review before this preservation footer.
