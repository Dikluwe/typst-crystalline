# L0 — Layout attempt convergence
Hash do Código: 8480ad0c

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/bibliography-plan.toml sha256:5c68aee638a285b125399ae9198883a90ae27a0c3a89f68e0765b86fa84b8197
- 00_nucleo/prompts/_nuclei/layout/physical-page-occurrences.toml sha256:b33f2e3c6238da01aa48783aecee6a8b6153b4f36ddcdc23edfe78434bae5b42

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/convergence.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

`01_core/src/compiler/layout/mod.rs:1859-2035` prepares bibliography cache/callbacks, rebuilds Layouter, injects known pages, compares maps and returns Complete/Pending. The hub L0 already requires five attempts and convergence even without outline. Structural measurement at `320c2149b`, without cost or vanilla-intent measurement.

C010 run at `convergence.rs:9` has 8 Clippy arguments against 7. `lab/results/construction/diagnostics/too-many-arguments-2026-10-15/before.json.gz` preserves full source/prior L0/signatures/hashes/HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`/uncommitted tree/UTC2026-10-15T16:39:18.873477+00:00/status/diffstat; classification.json records individual clauses and reconfirmed spans. The coordinator takes exact seeded-facade inputs; separate protocols and repeated seed are L0 obligations. This is Rust-maintenance evidence, not a functional defect inferred from arity.

The validated `lab/results/construction/diagnostics/clippy-locais-2026-10-16/clippy-inventory.json` preserves 79 warnings; current source/spans/reasons/signatures/hashes/HEAD/uncommitted state/UTC/diffstat are in `lab/results/construction/diagnostics/clippy-excecoes-2026-10-16/before.json.gz`. Concrete uses were read before decision. No new parity/cognitive-benefit proof.

`lab/results/construction/diagnostics/bibliography-a2-2026-10-18/before.json.gz` records 16 source/PDF pairs, argv/text/positions/UTC/HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`/uncommitted tree; before-sources.json.gz preserves sources/L0/status/full diffstat. Oracle `bd5ebec285505889e8ff1d4f68804ab147a699f3`, binary SHA-256 `9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`; Tekt `57154005875bca986a1c4eb44038c8bd9ad2b3580e5534059574895c94e62f82`. Simple MLA/IEEE preserve text; bibliography.where show-set red, bare rules, identity/Block/hanging show-set lose CSL in Tekt, changing (Silva)/MLA list to[1]/arrow fallback. Replaced/empty bibliography retains vanilla(Silva), so visual-only search fails. Block-hidden Bibliography rejects @audit in both: never resurrect unemitted children. Multiple bibliography duplicates one Tekt MLA list versus vanilla attribution and remains a separate defect.

`convergence.rs:197–249` partial catalogue omits Source/Located/Realized/ShowRecipe;`:57–91` builds cache only if declaration found. `bibliography.rs:112–123` falls back without cache. TagIntrospector already indexes Bibliography occurrences by kind_index/elements; BibStore holds resolved IndependentStyle by style_key. Pinned model/bibliography.rs:885–909 uses introspected bibliography occurrences;`:361–369` sets indent. These are language/effect facts, not required upstream structures. Carrier/hidden-declaration/order/cache tests can refute the partial-search causal inference.

`lab/results/construction/diagnostics/bibliography-structure-2026-10-18/before.json.gz` records 12 pairs, sources/argv/UTC/HEAD/status/diff HEAD --stat, text/boxes/binaries; before-sources.json.gz preserves prior sources/L0s. HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, uncommitted tree; Tekt SHA-256 `c2b1b2e93cf34f3c597a0a0fbbd51c8f8ac96a3c6bc0a4d9bd72373599acf899`; oracle `bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree `cac836c07e4dec2a89219370428292393a3216b5`, binary SHA-256 `9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`. IEEE12 places A. at x25.124 versus 29.924 and second entry y89.936 versus 112.232. IEEE18 uses x33.686 versus 40.886. In prefix-free CSL, second entry starts at y184.424 versus 191.024. par.spacing0/2em probes separate margins from within-line spacing. Adjacent-citation grouping also differs (MLA Silva/Doe), outside list composition: preserve that RED explicitly and do not claim general parity from list equality alone.

`bib_csl.rs:342–355` concatenates first_field, literal space, content and Linebreak, losing entry boundaries/hanging_indent. `convergence.rs:58–82` carries only public BibRenderCache; `bibliography.rs:107–151` emits one Block. Pinned `typst-layout/src/rules.rs:463–531` composes two Auto columns with 0.65em gutter and par.spacing row-gutter, or one Block per prefix-free entry; authored Rust form is not required. Observables are position, order, content/style and scope. Hypothesis: real boundaries/prefix transport correct columns/separation without reformatting; font, order, empty fields, no-prefix, zero/2em, show-grid and style controls can refute it.

## Exclusive ownership and current contract

Layer L1. Sole consumer: `01_core/src/compiler/layout/convergence.rs`. Coordinate private layout attempts using identical tracked introspection, Locator seed, CSL cache and initial diagnostics for every candidate. Transport known label/figure/table/PageStore in the same order, with at most five attempts. Accept only stable maps with math/par.line transcripts consumed; a rejected attempt discards transcript. Exhaustion returns Pending with empty requests. Public facades retain signatures; exhaustive Content dispatch/Layouter state stay in `00_nucleo/prompts/core/compiler/layout/mod.md`.

Bibliography discovery stays local to coordination; CSL interpretation belongs to `00_nucleo/prompts/core/compiler/layout/bib_csl.md`. First choose Bibliography from current attempt's semantic index in kind_index order, reading elements snapshot. A realized/replaced declaration still supplies citation CSL; never traverse arbitrary hidden originals to invent an occurrence. Preserve that declaration's style/locale and resolved_style side-table value from its own style_key. A first declaration with style None remains first; do not skip to a later explicit style.

Without an indexed Bibliography snapshot, retain raw-layout compatibility by borrowing first navigable occurrence through visit_content_static (`00_nucleo/prompts/core/entities/content/traversal.md`). Source/Located/ShowRecipe follow body, Realized only visual; Table/Cell use existing active view. Never execute recipe/Func/ContextBlock/Dynamic. Host prunes Document/Asset and stops descending after first match; no extra partial container catalogue, Content navigation clone or visitor change. The closure's if-let observes an event already reached by exhaustive domain dispatcher, not partial child enumeration. Document/Asset boundary is explicit; Content is not open/dynamic and V16/V19/V20 must not be suppressed.

Prepare cache/Option<Arc<plan>> pair once from the same selected declaration/locale/resolved_style/order. Propagate diagnostics as before; each attempt gets cache and Arc clone of plan, without rebuilding discovery or another bibliography driver. Same cache-before-layout phase/facades and explicit no-style fallback remain; no mode/flag/public field/trait/I/O/Layouter-state or CSL-interpretation change.

Preserve run signature/order/semantics with its injected metrics/sizer, introspector, font_size and two sealed transcripts plus seed. The authorized materialized nominal exception is only `#[expect(clippy::too_many_arguments, reason="C010: Coordenador privado recebe exatamente os inputs da fachada seeded: métricas/sizer injetados, introspector, font_size e dois transcripts selados com seed")]` on run. No module/impl/crate/global suppression, threshold increase or other lint coverage. If arity falls below threshold, remove/review expectation; never hide unfulfilled_lint_expectations. This later nominal annotation supersedes earlier audit-only pending materialization only for C010, retaining body/calls/cfg/unrelated attributes and all other corrections, including conditional C047/C066, as pending.

## Acceptance and explicit limits

Preserve pages/positions/content/diagnostics/callback requests for convergence/nonconvergence/invalid cache/injected seed. No phase/default change. Build/focal tests/lint pass.

Local RED before code covers omitted carriers/containers, semantic-index priority/order, registered replaced/hidden declarations, style_key/resolved_style/locale and raw fallback without Realized.original/Document traversal. Check no recipe effect. Bilateral GREEN restores CSL under show-set/identity/Block and citations after replacement; IEEE remains IEEE, outer paragraph style preserved. Validate build/suites/strict Clippy/Rustdoc/fmt/V1/V5/V15/V26/reverse/differential SARIF no new error/warning; remove search V16 by exhaustive visitor delegation, never suppression. Heavy commands sequential under 12GiB/High8GiB/swap0/jobs1/debug0/incremental0; no commit/push/release.

Discovery is incomplete for overall bibliography scope: generic/default MLA hanging/lexical overrides/actual geometry/OUTSIDE isolation and per-Bibliography attribution/cache remain separate work. Returning MLA format does not approve existing coordinates or close E05/all bibliography. Symbol, public Styles and proposed R3-T1 order remain unratified by this correction.

This internal parity correction follows ADR-0127 continuous flow within layout, without a new public API/type, flag/default, compatibility or phase. L0 precedes tests/code. Structural plans preserve hanging in their discriminant; physical hanging, per-occurrence overrides and simple/optimized/RTL composition belong to the separately specified transport/paragraph-indent work. Do not force greedy fallback to bypass missing initial-advance preparation. Title margins, complete CSL, multiple-bibliography attribution/cache and adjacent-citation grouping are not certified by structural-list work.

Require functional column RED with relative gutter/interentry margin before implementation. Controls cover public cache (prior Content/signatures), prefix-free CSL, hanging/plain mode preservation, empty input, order/citations and rollback. Bilateral PDF compares list boxes/text while preserving indent/citation-grouping divergences in the report. Validate build/workspace/doctests, strict Clippy/Rustdoc, fmt, V1/V5/V15/V26 and reverse lineage; SARIF has no new error/warning. Reseal only after valid ownership/graph. Use MemoryMax12GiB/High8GiB/swap0, jobs1/debug0/incremental0, sequential heavy commands; no commit/push/release.

C010 continuous-flow ADR-0127 adds only lint metadata, changing no executable body/cfg/public contract/default/phase/dependency/architecture. L0 before code and restricted resealing; removing new attributes/lineage header must recover rustfmt-normalized prior source. Establish RED before, then all-targets inventory exactly 46 arities+five test warnings fulfilled by 49 expectations, no new warnings and unfulfilled_lint_expectations as error;11 correction arities stay visible. Confirm workspace/all-targets C010 warning and unchanged calls/function controls, error-free build/fmt/strict-reverse lineage/Tekt no new findings/existing Counter tests/coverage under 12GiB/zero swap/one job. Prior full suite remains evidence for unchanged executable code; no attribute-mirror tests or new metadata performance claim. Private signature/caller changes require their L0 migration first; newly necessary public/compatibility change needs concrete ADR-0127 approval, not this exception.


## Approved physical page occurrence protocol

### Evidence before decision

At baseline HEAD `63b7b96b99109b61655b373dc0d248beb7838f86`, the complete source/owner snapshot and applicable ADRs are retained in `lab/results/set-page-header-2026-10-21/agents/core/source-evidence.json.gz` and its supplement. `baseline-parity/receipt.json` there binds current-pin physical query/state/counter/page observations and inert overwritten templates. `ambient-baseline/receipt.json` refutes freezing creation10pt styles: insertion uses actual page20/30pt ambient style. These are language observations; the source plan, packet transcript and rollback representation are mechanics. The exact five facades, opaque types and Paged discovery/realization/source-construction phase were explicitly approved in `lab/results/set-page-header-2026-10-21/review.md` (SHA-256 `cb4a684b483e7f9f6ac3f7eef52277f7ada405033a24e4e66e55811e93c12039`), recorded in `lab/results/set-page-header-implementation-2026-10-21/approval.json`. Only conflicting no-new-phase clauses for this exact protocol are superseded; legacy signatures/defaults and unrelated limits remain.

### Exclusive protocol responsibility

Own a separate private planned-attempt path for the approved facade, retaining the existing legacy run signature, five-attempt limit, injected metrics/sizer/introspector, bibliography preparation and same Locator seed behavior. Each planned candidate creates fresh transactional running state alongside math/par.line state. A candidate may expose Complete only after stable maps, complete accepted source/body/slot coverage, valid retained source/environment authority, and successful consumption of every supplied channel. Pending returns running requests and source-owned trace alongside the unchanged callback request type; it never exports the tentative document. Discard rejected attempt transcripts as a unit. No second Context or arena is created here.

### Acceptance and preserved boundaries

Require semantic RED→GREEN for repeated physical header/footer/four-slot order, zero-paint updates, source-derived parents, overwritten inert templates, aliases and physical pages, effective20/30pt ambient plus explicit inner styles, nested PageRun restoration, Context reads, and exact duplicate-label/source diagnostics. Cover speculative suffix rollback, deferred floats/notes, subframes, measurement isolation, complete body/slot coverage, stale/mixed authority and math/par.line coexistence. Do not weaken existing tests, callback budgets, per-store sealing, source morphology, HTML isolation or old public compatibility. No generic painted-frame semantic reconstruction, source re-evaluation, second arena/Context solver, direct Func marginal mode, new default/flag or extra public API is authorized. Every changed descendant retains its own current 1:1 owner and explicit typed route; finite source inventories are not blanket traversal permission. Exhaustive new decisions need actual future-variant barriers, and the complete lineage/Nucleus, build/test/lint/Rustdoc/fmt/Clippy gates remain required without suppressions.


## Opaque physical trace storage

The source-bound Clippy RED in
`lab/results/set-page-deferred-corrections-2026-10-21/candidate-2/validation-remainder/clippy.log:83–113`
identifies oversized private pass variants and the public result's opaque trace
storage. Exact before texts, SHA-256, UTC and working-tree state are in
`lab/results/set-page-report-blockers-2026-10-21/agents/layout/trace-callers-before.json.gz`
and `agents/layout/before.json.gz` under that evidence root.

The trace owner retains every existing source, admission, request, sealed geometry
and completeness value behind one compiler-private owning payload Box. Access
those same fields through `trace.data`; preserve exact mutation/read order,
cloning, authority, geometry, verification and acceptance. This is private storage
indirection only: the public opaque PageRunningTrace and all public fields and
function signatures stay unchanged. No new semantic fact, source traversal, phase,
callback, diagnostic suppression or relaxed gate is authorized.
