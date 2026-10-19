# L0 — infra/fontdb/generation
Hash do Código: 0cd789b1

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/watch-font-generation.toml sha256:0f2985807cc3c7b4aeed9a5a26b77ad66dbbdb67ccfb43ba8336dbc8e8eea1a5

**Layer:** L3
**Exclusive consumer:** `03_infra/src/fontdb/generation.rs`


## Font retention evidence and approval

`lab/results/construction/diagnostics/watch-retencao-2026-10-07.md` retains
HEAD/tree/binaries/UTC, actual edits/recovery and dynamic per-cycle acquisition
counts. `watch-validacao-2026-10-07/measurement.json` compares discovery validation
and complete acquisition at the same state; controls.json covers additions/removals/
replacement/restored mtime/links/configuration. These are isolated costs, no CLI
gain or upstream-intent proof. FontBook/slots/coverage must preserve language
selection/content/diagnostics/render; Arc/duration are mechanics. Cost, stale
resources, regression or unbounded retention refute possible benefit. Fixtures/
provenance/limits remain in the reports.

Owner `Confirmo` ratified
`lab/history/construction/records/00_nucleo/materialization/watch-catalogo-2026-10-07/manifest.json`
under ADR-0127 §2.1/§2.2; receipt
`lab/results/construction/diagnostics/watch-catalogo-execucao-2026-10-07/ratification.json`.
Each owner remains 1:1.

## Prepared discovery and validity key

No public API. Private-to-L3 PreparedDiscovery holds fontdb Database, current
project slots and comparable key. Prepare uses unchanged Database::load_system_fonts
and fonts::discover_fonts on supplied paths with exact order/duplicates. Key
contains path configuration and ordered origin/index/identity, never transient
Database ids.

For every Unix Source::File/project slot capture regular-file path/index/dev/ino/
size/mode/mtime-sec-nsec/ctime-sec-nsec. Recheck after prepare and before generation
reuse/publication. Metadata errors/nonregular/changing samples/SharedFile refuse
prepare/reuse. Fresh SharedFile keeps retained original bytes; Binary stays ignored.
Non-Unix declines, no weaker identity. Full discovery repeats per new World demand
unless the strict proof below succeeds. Old file/directory list alone is no proof;
new/nested/config changes use the same loaders. Equal resulting list/identity need
not rebuild immutable book. No fc-list/new watcher/directory-only snapshot/disk cache/
mtime-only fingerprint. Recompile demand is observation boundary, not atomic whole-
filesystem snapshot or later-change detection; font-only changes add no event.

## Strict generation proof

### Evidence

`lab/results/construction/diagnostics/watch-incremental-item5-2026-10-08/measurements.json`
records Tekt median CPU 54.38/54.75/54.73ms for same-bytes/word/structural edits
with vanilla-equivalent text/bboxes. correction-probe.json has three hot cycles:
no-text 1.51ms total/simple text 50.45ms/set-text-font 48.43ms with 47.48ms eval.
First catalog demand shifts its measured phase; generation.rs:27–29 calls
load_system_fonts and font_session.rs:23–52 prepares each demand. Rediscovery is
an inferred primary cause, refuted by direct measurement or absent paired gain.
These medians are not additive components of one profile.

### Contract

Fast proof is only for eligible standard Linux fontconfig without explicit project
fonts. Original fresh preparer remains full fallback/oracle. Proof materializes no
book/slot/face and never replaces fresh discovery after change. Gather effective
configuration using the pinned fontdb parser/path policy, merged config bytes and
relevant environment/cwd. At generation establishment recursively traverse all
effective-directory entries, including nonfonts, in observed order under one global
budget. On hot verification reparse config, compare files/environment/cwd and Unix
identities for roots/all directories/font-extension candidates/symlinks including
target/absence. Directory ctime catches create/remove/rename; candidate invalid/
unreadable fonts may become valid without directory change. Observe symlinks even
without font extension since canonical target extension may qualify. Record absent
roots for creation. Ordinary nonfont-extension file byte changes without rename
cannot affect discovery and need no key entry. This is discovery-content/association
proof, not directory/mtime-only snapshot; any unverifiable case requires fresh path.

Custom/unreadable config/traversal or metadata errors/instability/unsupported entries/
limits/other platform decline fast path. Bound retained descriptors at 16, 384 and
paths at 1 MiB, checked arithmetic/recoverable allocation failures decline. Limits
constrain cache eligibility only: never truncate fresh full catalog/book. Store
proof only with current paired generation. Identical complete new proof plus valid
prior generation stamps may reuse Arc without load_system_fonts/new font read_dir.
Different/refused proof uses prepare/prior validation and rebuild if needed.
Revalidate proof after prepare/catalog construction before storing, without another
full traversal if retained identities remain stable. Failure/instability loses fast
eligibility. Keep demand boundary and next-recompile recognition, no new event.

## Acceptance and maintenance

Inline RED→GREEN: unchanged/add/remove/negative→valid/atomic replace/equal-size+
restored mtime/link retarget/fontconfig included/custom config/project nesting/
limits/unstable identity/errors/concurrency. Separate deterministic discovery
backend from environmental dependencies. Compare full catalog/selection with fresh
preparer. Measure watch CPU/latency/RSS/startup in three rounds with original item-5
scenarios; remove fast connection if freshness proof incomplete or benefit fails
to repeat. Session handles refusals; this owner creates no metadata/FontBook.

`ci-simplificacoes-2026-10-14/before.json.gz` retains source/owner/spans/HEAD/
uncommitted tree/UTC/diffstat at generation.rs:356,571,574; red.log selected-lint
errors and types/uses checked under 1.92.0. In missing-root/nested-project fixtures
borrow PathBuf as unit slice via std::slice::from_ref, no path clone just for argument.
Preserve create/change/recheck/identity/assertion order. Internal continuous ADR-0127
adds no public/default/phase/unsafe/dependency/suppression. L0 first/scoped resealing,
existing fixtures/expectations/tolerances unchanged, selected-lint RED→GREEN then
full suite/build/all-targets/Rustdoc/fmt/lineage/full inventory. No syntax-mirror tests.
