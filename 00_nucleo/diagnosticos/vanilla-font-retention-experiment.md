# Lab experiment: vanilla-style font-store retention in Tekt

The lab adapter reproduces one observed upstream mechanism: retain the lazy font
catalog and loaded FontSlots across source revisions while invalidating file state.
All 324 output comparisons passed against the direct Tekt pipeline. Five accepted
rounds show lower six-compilation session cost with retention in every paired case;
the observed lazy-retention reductions span 68.76–83.67%. Absolute timing admission
is weaker: only 11 of 27 cohorts meet the predeclared stability rule. These results
support eliminating repeated acquisition for the tested sessions, not a universal
speedup estimate or a claim that the Tekt–vanilla cold CLI gap is solved.

## Reference and intervention

Source evidence precedes the experimental choice:

- `lab/typst-original/crates/typst-cli/src/world.rs:31`: session font field is LazyLock;
  constructor configures it lazily; `:113` reset clears file/time state and preserves
  fonts. `typst-kit/src/fonts.rs:86` FontSlot holds a OnceLock loaded font.
- `lab/typst-original/crates/typst-cli/src/watch.rs:76`: compilation reuses the World.
- `03_infra/src/world/fonts.rs:65`: Tekt already has lazy catalog acquisition and
  per-World retention. `04_wiring/src/watch.rs:63` calls run_compile_observed each
  cycle, whose compile path constructs a new World. The lifetime differs.

The reference remains pin `9f2b6e8715237cb086899a42873660fe744622e8`, ratified tree
`6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4`; the reference executable was rehashed to
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`. Its provenance is
linked in provenance.json. No vanilla runtime timing was collected here. This is a
mechanical experiment under ADR-0107/0108, not a new language requirement or an
assertion about upstream design intent. ADR-0030/0121 and current font/watch/compile
L0s were read before implementing the quarantined adapter.

[lab/vanilla-font-retention/src/main.rs](../../lab/vanilla-font-retention/src/main.rs)
implements a World adapter using existing public APIs. Every edited revision receives
fresh SystemWorld file/library/source state; only book/font/candidate operations
can delegate to a session-retained font donor. Font discovery, ordering, coverage,
TTC handling, metrics, shaping, layout and export remain current Tekt implementations.
The donor is an existing synthetic SystemWorld, so retention also preserves its unused library/file scaffolding; those extra construction/destruction costs are not isolated. Observed later-cycle setup means differ by only about 0.002–0.010ms between policies, while donor/catalog destruction is included rather than hidden. Metric instances and shaping results do not acquire a new session cache here. This
conservatively models file invalidation; it does not port vanilla FileStore reset,
ShapePlan caching, watch events, or library retention. It introduces no production
API and is not imported by L1–L4.

Three policies are compared within one executable: fresh donor per compilation,
lazy retained donor per session, and retained donor with eager scan. The last policy
is a control for shifting acquisition outside the first compile timer: its preload
cost remains inside the reported session cost. There is no on-disk catalog, new
font-demand suppression or SoA/data-layout rewrite. This tests lifetime/reuse, one
part of the broader data-oriented approach discussed with the user.

## Measured state and protocol

HEAD `17f13b1df79b87ea065ee62e78164b18e477da3d`, dirty working tree; exact inventory and
tracked diff stat are retained in [provenance.json](vanilla-font-retention-experiment/provenance.json).
The provenance inventory was captured during scaffold preparation, before the complete lab helper existed; initial-fingerprints.json and the source archive identify the executed helper, and completion-audit.json records the final exact inventory. The tracked diff stat remained identical throughout. The source archive captures the actual measured consumers; results are not for a
clean HEAD alone. Machine: Dell-G15-Linuxmint, Ryzen 7 5800H (16 logical CPUs), Linux
6.17.0-42-generic, Rust/Cargo 1.92.0, x86_64-unknown-linux-gnu. Release opt-level 3,
no debug assertions/overflow checks, empty Rust flags, offline locked dependencies,
one Cargo job; runtime also refuses debug assertions. Executable SHA-256:
`7b959e9cd9ab0150f39229975914e217bc5a67a94d519e0b33100e45d32dfc0e`. Raw compiler artifact profile/recipe are in artifact.json.
The initial compile failed a lab borrow check; its log is retained. The corrected
build and final locally formatted build succeeded before any accepted measurement.

Collection: 2026-09-28 UTC 00:43:18.004937+00:00 through
00:44:37.220589+00:00 (local date 2026-09-27). Five rounds,
45 fresh-process sessions, six edited inputs each: 270 compiled revisions. Three
fixtures have six distinct source hashes each. Latin/dense render changed revision
numbers; placed rectangle widths change 80..85pt. Page 160×180pt, margin 8pt; Latin
and dense use Libertinus Serif 11pt. Default embedded/system fonts, no project paths;
1129 catalog faces on this host. Inventoried font files, /etc/fonts plus the fc-list targets, consumers, inputs, lab source,
locks and executables retained identical before/after fingerprints in every round.
A 100ms monitor found no known competing build/test/benchmark; it is not a complete
process classifier. Controls warm OS caches, then a five-second quiet gate precedes
measurement. Round ordering alternates/reverses policies and fixtures; no selective
retry or outlier removal.

The host load average was approximately 13.9–15.4 at round completion. The configuration inventory does not guarantee coverage of every custom environment/per-user fontconfig file. No CPU affinity,
frequency/thermal control or exclusive desktop session was imposed. This substantial
background-load indicator is a limitation, not proof of a specific source of noise
or of CPU saturation. A post-collection process snapshot is retained separately;
it cannot reconstruct the workload during every sample. No historical timing was
substituted for a slow current measurement.

## Results

All values below are **milliseconds**, medians. Cold-cycle column is cycle zero:
prewarmed already paid acquisition in the separate preload interval. Subsequent
cycles are revisions 1..5 (25 observations per policy/fixture); first/session have
five observations. Session includes all six compile cycles, donor setup/preload and
final donor destruction. † = exploratory, fails CV ≤20% or a round mean/session
value within 10% of pooled mean. Passing this descriptive rule is not a confidence
interval or a precision guarantee with five session samples.

| Fixture | Policy | First cycle | Subsequent cycles | Six-cycle session |
|---|---|---:|---:|---:|
| latin | fresh | 301.559 † | 300.228 † | 1768.467 † |
| latin | retained | 300.235 | 5.907 | 330.109 |
| latin | prewarmed | 6.673 † | 5.830 † | 316.989 † |
| dense | fresh | 321.912 † | 293.579 | 1787.934 |
| dense | retained | 309.371 † | 27.845 † | 454.156 † |
| dense | prewarmed | 29.537 | 27.971 † | 453.448 |
| placed-shape | fresh | 265.458 † | 261.693 † | 1568.602 † |
| placed-shape | retained | 263.730 | 3.697 † | 282.240 |
| placed-shape | prewarmed | 4.381 | 3.690 | 286.612 † |

Paired lazy-retention session reductions, pairing policies by fixture and round:

| Fixture | Median observed reduction | Observed range across five pairs |
|---|---:|---:|
| latin | 82.62% | 77.62–83.67% |
| dense | 74.60% | 68.76–76.95% |
| placed-shape | 82.51% | 80.90–83.20% |

These percentages describe this collected block, including unstable absolute cohorts.
They are not promoted to reproducible production speedup estimates. Every pair is
positive; exact absolute durations and dense/lazy variance require a quieter repeat.
[summary.json](vanilla-font-retention-experiment/summary.json) preserves samples,
mean/median/SD/CV/min/max/round means and all 27 admission flags;
[paired-session-effects.json](vanilla-font-retention-experiment/paired-session-effects.json)
retains every paired difference. Prewarm versus lazy session totals overlap; no
additional speedup is established by moving the scan before cycle zero.

## Attribution and output checks

The first book accessor in later revisions provides a directly nested observation
of acquisition/reuse. Descriptive means from the same collected cohorts:

| Fixture | Fresh store: first book on each later revision | Retained store: first book on each later revision |
|---|---:|---:|
| latin | 292.732 ms | 0.1978 µs |
| dense | 267.327 ms | 0.2028 µs |
| placed-shape | 258.987 ms | 0.1511 µs |

The initial lazy retained compile still pays acquisition: its first-book means are
289.694ms (Latin), 290.283ms (dense), and 256.778ms (placed shape). Subsequent book
access means are below a microsecond; pointers remain constant for each retained
session, catalog metadata signatures and selected font bytes agree. The catalog
signature hashes Debug metadata and does **not** independently encode complete
coverage sets; immutable source/font fingerprints and output controls complement it.
Fresh store addresses may be reused by the allocator and are not used as identity
proof. This controlled lifetime intervention and observed repeated book acquisition
support the explanation that recurring discovery is a major cost here. Other effects
of retaining a book on memoization/selection are not individually separated.

Acquisition is inside eval for the explicit-font text fixtures and inside layout for
the default-font placed shape. Book duration is already included in pipeline time:
never add it again. The lab retains existing eval/layout/render fields as descriptive
phase observations in phase-summary.json. **render_ms is post-subsetting export**, not
complete PDF export: shape/subset/introspection/context fields were not retained by
the lab row schema. Do not sum these partial fields to reconstruct total or attribute
export to stream. The outer pipeline_ms interval includes the complete productive
pipeline and full PDF export. No timings from differing fixtures are combined.

All 324 direct-versus-adapter/measured output comparisons match every page dimension,
word text/geometry, and PDF font list via Poppler. All-page raster pixels at 36dpi
match for all controls and round zero; later rounds receive bbox/page/font checks,
not raster checks. Used font byte hashes and warnings match among policies within each
measured revision. Text output changes with all six revision numbers, so the fast
path did not simply return stale source output. This validates equivalence to current
Tekt for these inputs, not complete language parity or arbitrary visual equivalence.
The placed shape's known unnecessary font demand remains: retention makes it cheaper
on later compiles without repairing that defect. Existing Place geometry regressions
are unchanged, and vanilla output equality is not claimed.

## Practical conclusion and limits

Session-retained font state is worth integrating experimentally into the actual
watch lifecycle next, with explicit source/time invalidation and font-policy replacement.
This lab test supports the benefit for changing-source recompilations; it does not
save first-process discovery or allow separate CLI invocations to share RAM. Before
production adoption, verify font file/config changes, errors, dependency invalidation,
concurrent use and memory/resource bounds; then measure the real watch path with
matching outputs and a quieter host. Keep nontext demand correction as a separate
intervention. No production repair or architectural contract change is made here.

Evidence and reproduction: [README.md](vanilla-font-retention-experiment/README.md),
[protocol.md](vanilla-font-retention-experiment/protocol.md), source/executable archives,
raw stdout/stderr/PDFs/commands, distributions, controls and completion-audit.json.
MANIFEST.sha256 seals this dataset; the separate report digest is in the completion
audit. Only the new lab directory and diagnostics were changed. Older sealed studies,
production, L0s, ADRs and benchmark sources are preserved. No global formatting,
hash reseal, commit or push was performed.
