# Tekt mature repository layout

Status: revised execution plan, 2026-10-18. Not L0 or an architectural decision.
Replaces the earlier directory-migration proposal at this path. Its original bytes
are preserved in the [superseded proposal](../../diagnosticos/prompt-organization-2026-10-18/superseded-proposal.json.gz).

User decision retained: **English for active content; historical records preserved
in their original language.** Organization must follow responsibility and the
causal role of each artifact.

Latest user direction, 2026-10-18: perform the cleanup on a **new branch**, organize
ADRs, prompts, and diagnostics, retire the construction-stage `materialization/`
directory, and place benchmarks and test-build artifacts inside `lab/`. Keep the
repository root focused on L0, L1–L4, and lab. This direction supersedes the earlier
recommendation to continue cleanup on the existing branch. The causal safeguards
below support this cleanup; they do not require auditing all product behavior
before starting a mechanical migration.

## Mature tree

```text
00_nucleo/
  README.md
  adr/                         # current decisions and authority-ordered index
  prompts/                     # current causal owners
    _nuclei/                   # shared, pinned declarative obligations
  diagnostics/                 # concise findings, receipts, evidence references
01_core/
02_shell/
03_infra/
04_wiring/
lab/
  benches/                     # benchmark crate, corpus, runners
  parity/                      # language-oracle comparisons
  typst-original/              # pinned reference source
  experiments/                 # prototypes and exploratory work
  tools/                       # analysis, verification, build/release executors
  results/                     # retained raw experimental evidence
  target/                      # ignored build outputs, separated by purpose
  scratch/                     # ignored temporary compilations and outputs
  history/                     # recovery index and required historical bundles
```

Root manifests, lockfile, toolchain/configuration files, README, contributor/agent
entry points, license notices, and required hidden Git/CI/Cargo directories remain
at their tool-required locations. There is no new root-level documentation tree,
`tools/`, `benches/`, or `target/` in the intended final layout. Preserve provenance
and licensing files; a short root is not a reason to remove them.

`diagnosticos/` → `diagnostics/` is the explicit English-name migration in L0.
Update the name-based linter exclusion and all active references together before
removing the old path. The owner prefix `00_nucleo/prompts/` stays unchanged.
There is no mature `materialization/`, `plans/`, or equivalent permanent directory
of numbered execution steps. This plan is itself a transitional construction
artifact and leaves the active tree when the cleanup is complete.

## Concrete relocation and retirement rules

| Current material | Destination or disposition |
|---|---|
| Active ADRs | `00_nucleo/adr/`; retain IDs and the existing authority-ordered index; review English titles/prose and canonical filename mappings |
| Current owner specifications and nuclei | `00_nucleo/prompts/`; consolidate active obligations and concise revision history; preserve bijection and pins |
| Current diagnostic summaries | `00_nucleo/diagnostics/`; organize by subject with dated evidence, one entry point per subject |
| Raw measurements, fixtures, and executors in documentation directories | `lab/results/`, suitable lab corpora, and `lab/tools/`; L0 links to evidence without containing executable utilities |
| `benches/` | `lab/benches/`; reconcile Cargo workspace membership, dependency paths, commands, and ADR-0115's old root-location clause |
| `tools/` | `lab/tools/`; update CI/release entry points and script root discovery; do not exclude these utilities from their own tests |
| Root `target/` and new test-build artifacts | `lab/target/` with separate product/oracle/benchmark destinations; configure future builds first, retain unique receipts before cache disposal |
| Root scratch output and `.typ/` temporary compilation inputs | Classify unique fixtures versus disposable outputs; retain necessary inputs under lab and use `lab/scratch/` for new temporary runs |
| Construction plans, handoffs, completed steps | Recoverable construction history; remove from the active tree only after requirement/evidence extraction |
| Open proposals or unfinished work in construction records | Preserve the proposal or debt status in the appropriate ADR/diagnostic; do not promote it to an accepted contract or silently discard it |

Retiring `materialization/` means classifying its contents, not moving the whole
directory into diagnostics under a new name. Current requirements belong in their
owners/nuclei, architectural decisions in ADRs, and approvals/results in compact
diagnostic receipts. Historical narrative stays byte-preserved in the construction
checkpoint or recovery bundle. Active references must reach retained evidence or
an explicit historical locator, including commit and original path. Audit the
Context approval receipt and every still-referenced contract before retirement.

New work still follows investigate → update L0 → materialize → verify. “Materialize”
remains an operation; it no longer requires a permanent step-directory hierarchy.
Use the task for temporary coordination, recording lasting intent and evidence in
their established categories. Preserve a short revision record in each owner.

Product unit/integration test sources keep their legitimate owners in the layers;
their compiled outputs go into lab. Lab code may exercise product APIs as a test
harness, but L1–L4 must not import lab source. Moving a build artifact under lab
does not move the implementation's architectural responsibility there.

## Measured build dependencies

[mature-layout-baseline.json.gz](../../diagnosticos/prompt-organization-2026-10-18/mature-layout-baseline.json.gz)
captures exact configuration sources, root entries, previous plan, HEAD, working
changes, and UTC for this revision. The following findings determine the migration:

- `Cargo.toml` includes `benches` in the product workspace; `lab/Cargo.toml` is
  already a separate parity workspace. Do not let a moved benchmark accidentally
  inherit parity package versions or dependencies. Select and validate explicit
  membership using Cargo metadata before changing the command entry points.
- ADR-0115 explicitly chose root `benches/`. Record the user's new location
  decision as a scoped amendment; retain benchmark methods and historical results.
- `.cargo/config.toml` has no target-directory setting. The migration must configure
  output placement, not merely move today's cache. Keep the pinned oracle's source
  identity and existing receipt valid; a rebuilt oracle needs a new build receipt.
- CI, Docker, the release packager, and comparison scripts refer to `target/`,
  `tools/`, or `.ci-oracle` paths. Update active callers, cache configuration,
  packaging/artifact paths, and Docker copy rules together. The existing
  `.dockerignore` excludes lab; a moved benchmark or required executor cannot be
  silently omitted from its build context. The CI oracle checkout also moves under lab.
- Historical command strings are evidence and are not mass-rewritten. Provide
  restoration/path mapping separately. A mechanical rename does not establish new
  parity or performance results.

The default product build destination proposed for validation is
`lab/target/product`; benchmark and oracle targets remain distinct. Preserve the
existing oracle under `lab/typst-original/target` until a controlled replacement.
Release packaging outputs also stay under lab, with CI upload paths adjusted.
Tests that currently rely on tmpfs retain that runtime requirement; lab owns their
fixtures, recipes, and retained evidence without silently replacing `/dev/shm` with
a slower filesystem. Transient OS temporary directories are not root clutter.

## Evidence before direction

The architecture was read at the user-specified
`/home/dikluwe/Documentos/Antigravity/Tekt`, resolving to `/repos/Antigravity/Tekt`.
The inspected tree has local changes over HEAD
`35b9000631ddf118bade7223d8dc2877d9123fba`. Exact texts, hashes, status, diffstat,
and UTC are recorded in [architecture-reading.json.gz](../../diagnosticos/prompt-organization-2026-10-18/architecture-reading.json.gz).
That commit alone does not identify the architecture text read.

Decisive sources, relative to the Tekt repository:

- `MANIFESTO.md:33`: L0 is the causal origin; code is its materialization.
- `MANIFESTO.md:109`: invariants take precedence over local convenience.
- `MANIFESTO.md:219`: context required for a change is an architectural force.
- `00_nucleo/README.md:64–68`: no executable code in L0, bijective ownership,
  and mandatory revision history.
- `00_nucleo/README.md:80–84`: detailed evidence must not inflate owner contracts;
  resealing dependencies does not prove implementation conformance.
- `00_nucleo/adr/0005-evolucao-verificavel-e-contexto-local.md:35–52`:
  distinguish lineage from correctness; measure context across comparable changes.

The current Typst decisions remain binding. Reading this external working tree
does not silently adopt its changes or override local ADR-0107/0108/0109/0127/0129/0130.
Tekt ADR-0003 is still proposed. Tekt ADR-0001's differential `@base` mechanism
does not authorize additional owners or an unsupported local lineage format.

The [local audit](../../diagnosticos/prompt-organization-2026-10-18/report.md)
provides organizational signals, not semantic certification. Reconcile the existing
`00_nucleo/materialization/tekt-english-reference-migration.md`, its completed pilot,
and pending scope rather than creating another parallel migration program.

## Assessment and withdrawn recommendations

The evidence supports investigating drift in how Tekt is applied. It does not
establish that the architecture is fundamentally wrong or every consumer violates
its contract. The critical question is whether L0 determines the intended change
or merely records a convenient implementation afterward.

The earlier `00_spec/` tree, source-path mirror, and new documentation hierarchy
are withdrawn: they were selected before measuring responsibility and context cost.
Source paths are useful ownership lookup keys, but cannot determine the architecture
of L0. Bijection establishes ownership, not the correctness of unit boundaries.

Keep `00_nucleo`, the current layer roots, and canonical owner paths. A future
rename must solve a demonstrated ambiguity, preserve references, and respect the
applicable decision gate. English prose does not require a root rename.

Keep a concise revision record in each prompt: date, reason, affected consumer,
and evidence link. Move detailed execution narratives only after auditing their
clauses. Preserving history does not require reloading every execution transcript.

## Hypothesis and refutation

Hypothesis: accumulated execution narratives, unresolved obligation status, and
responsibilities spread across unrelated contexts increase the effort needed for
a correct change. Mixed language may compound this; it is not itself a structural
violation. The earlier filename census does not establish this hypothesis.

Evaluate comparable changes using the actual owner, complete pinned nucleus
closure, relevant interfaces, and governing decisions. Record what must be read to
explain the intended behavior and design acceptance cases, and why additional
files are needed. Compare before and after consolidation. If the same unrelated
context is still needed, or obligations become harder to recover, the claimed
improvement is not demonstrated. Fewer lines, English prose, or clean hashes alone
cannot close this claim.

## Execution sequence

| Stage | Work | Completion evidence |
|---|---|---|
| Preserve and branch | Create `codex/tekt-mature-layout` from the current state without discarding changes; before removals, capture uncommitted Context sources, L0, fixtures, and receipts in a recoverable checkpoint and immutable historical reference | New working branch, verified restoration, hashes, and baseline limitations; a branch pointer alone does not preserve uncommitted or ignored files |
| Reconcile authority | Reuse the English migration; distinguish accepted decisions, superseded clauses, proposals, and exceptions | One active direction; no proposal promoted by translation or omission |
| Audit causality and responsibility | Map intention → owner/nuclei → interface → consumer/tests → evidence for representative units | Obligation origins and observables; missing chronology or scope remains unknown |
| Consolidate a pilot | Restore contract readability and translate to English without changing meaning | Clause mapping, compact revision history, repeatable context-reading task |
| Consolidate L0 and retire construction records | Organize ADRs/prompts/diagnostics, extract live obligations and evidence, remove `materialization/` from the active tree | No unresolved loss of requirements or approvals; active links and historical restoration verified |
| Consolidate lab and root | Move benchmarks/tools/experimental material and configure build outputs; update callers, workspace boundaries, Docker, and CI | Cargo metadata and affected tooling tests pass; root matches the intended layout; no production source dependency on lab |
| Correct proven structural drift | Update L0 first where responsibility is misplaced, then materialize in the owning layer | Appropriate gate, RED→GREEN, preserved static guarantees and language observables |
| Expand and maintain | Repeat successful treatment by responsibility; normalize justified internal names and active guidance | Per-batch evidence and explicit outstanding scope, not blanket conformity |

First audit candidate: the realization area.
`00_nucleo/prompts/compiler/realize/mod.md` contains accumulated decisions and
supersession; `00_nucleo/prompts/compiler/realize/context_occurrences.md` is its
bounded companion for the approved identity responsibility. This selects an audit,
not a split, merge, or rewrite of either consumer. Read the actual pinned nuclei
and interfaces before editing. Do not select only the shortest prompt to make the
pilot appear successful.

For each obligation record original path/hash/lines, current meaning, authority,
dependencies, observable, and disposition: retained, explicitly superseded,
historical evidence, or unresolved. Existing code can expose a discrepancy; it
cannot turn that behavior into intended policy. Tests follow declared intent.
This review does not claim independent verification by another executor.

## Boundaries and naming

Preserve L1 purity, L2/L3 separation, L4 composition, and lab quarantine. Inspect
responsibility as well as imports: a legal graph can still hide an overloaded hub
or business decisions in an adapter. Any movement must be justified by its current
owner and layer contract. Atomization follows local ADR-0109: owner-layer delegation
with static exhaustive dispatch. File-count reduction is not the acceptance gate.

Names should express responsibility and distinguish phase, container, facade,
language content, and rendered representation. Preserve substrate names where they
identify parity counterparts; record deliberate divergences. English normalization
does not authorize renaming APIs, diagnostics, flags, or language fixtures. Keep
step IDs in process records and causal history; review step-based permanent names
with their callers individually instead of mass replacement.

Maintain a compact terminology reference through existing active governance. It
explains terms and links to owners without becoming a second contract store.
Genuinely shared obligations belong in pinned nuclei; owners retain their specific
contracts. Nucleus propagation is a review cost to measure, not hide by resealing.

## Language, history, and tools

Translate active prose after resolving its intended meaning. Preserve obligations,
prohibitions, exceptions, evidence pins, and acceptance conditions. Keep machine
tokens such as `Hash do Código:` and `Núcleos Tekt:` until a separately specified
tooling change supports another format. Historical Portuguese remains intact.
The previous English pilot records semantic mistranslations; fluency is insufficient.

During this transition, keep coordination outside owner prompts and move lasting
evidence to diagnostics. Retire this plan with the construction records. No
executable audit utility belongs in L0. The previous plan's `inventory.py` violated
that boundary: preserve its bytes as data in the superseded-proposal archive and
remove the executable file. Future executors belong outside L0.

Perform cleanup in the requested new branch while preserving existing Git lineage.
The branch isolates the reorganization; it does not substitute for contract and
reference validation. No history rewrite or forced update of an existing branch
is planned. The previous `tekt-historico` remains intact, but does not by itself
cover subsequent uncommitted work.

## Validation and stop conditions

Report lineage, structure, behavior, and judgment separately. Validate ownership
and the full nucleus graph before resealing; update a changed closure atomically
and reevaluate its obligations. Require bidirectional lineage and an unchanged
code body for purely editorial owner changes.

For materialization, run required build/lint and relevant tests; parity claims use
the pinned oracle and language observables under ADR-0107/0108. Record existing
debt explicitly without reporting global cleanliness. Translation cannot silently
change behavior, architectural invariants, or the acceptance criteria.

Completion requires a readable current causal contract, preserved concise history,
verified ownership and boundaries, evidence for claimed observables, and a comparison
of context needed for the selected change. It also requires no active
`materialization/` directory, no root benchmark/tool/build-output trees, working
build/test/CI entry points, and recoverable historical evidence. Validate the
affected tooling and the established product build/tests/lint without treating
pre-existing debt as newly clean. Unknown origin remains unknown. If a
Tekt premise fails under comparable changes, document that result and propose a
reasoned architectural revision instead of asserting compliance or renaming folders.
