# L0 — compiler/realize/mod
Hash do Código: 78d1574e

**Layer:** L1.
**Exclusive productive consumer:** `01_core/src/compiler/realize/mod.rs`.
**Governance:** ADR-0026, ADR-0029, ADR-0030, ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, and ADR-0130.


Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/map-content-change.toml sha256:6343f553bd5f842f83190bd6533a9a9642118a34214550d8289c9daaba219043
- 00_nucleo/prompts/_nuclei/content/sequence-realization.toml sha256:14ece1bcb7901da8d7e5048a82cd02ef9eafa258a328900a99636ff4ef74a843
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:059eb35863a413593c98fb7195a521e3d019dadf461433e9c42422218be89065
- 00_nucleo/prompts/_nuclei/content/symbol-identity.toml sha256:5060f0989415e92d54eab2a82afd84ba11472aef635d15830f9fb6ee3469aea9
- 00_nucleo/prompts/_nuclei/eval/textual-recipe-frontier.toml sha256:983b118684c0ddab1fbb3c619744957c2e1f93af7708b82c97f665e47410fbf3
- 00_nucleo/prompts/_nuclei/layout/physical-page-occurrences.toml sha256:b33f2e3c6238da01aa48783aecee6a8b6153b4f36ddcdc23edfe78434bae5b42
- 00_nucleo/prompts/_nuclei/math/bare-insertion.toml sha256:c54d66589196680f0db26d0b6328d20c5d28d474458f0661275e871139d07d9b
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:2b9e828f5e0d958f4328d98a35320ba2ea321997e541e6ab90520d2e8811e7bf


## Measured basis and domain decision

The measurement preceding this decision was taken at HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`. `01_core/src/compiler/eval/mod.rs:872–880` evaluates the program without show and then with show. `03_infra/src/query_helpers.rs:511–520` chooses between original and rendered indexes by cardinality. The audit `lab/results/construction/diagnostics/dupla-avaliacao-auditoria-2026-10-05.md` preserves controls and provenance: a recipe producing one heading makes Crystalline lose the generated item; a recipe producing two headings makes it lose the original. Full cases and commands are in `lab/results/construction/diagnostics/dupla-avaliacao-auditoria-2026-10-05/results.json.gz`.

Historical pinned vanilla `9f2b6e8715237cb086899a42873660fe744622e8` evaluates the main program before stabilization (`lab/typst-original/crates/typst/src/lib.rs:124–132`). Its realization prepares identity before applying a recipe and preserves tags from the original while visiting the product (`lab/typst-original/crates/typst-realize/src/lib.rs:353–430`). This establishes a reference mechanism; it does not require copying vtables, unsafe code, Rust equality, or the exact algorithmic steps.

Separate program execution from recipe realization. Evaluation executes common code once and transports content and recipes with an immutable lexical environment. Pure realization applies recipes, preserves original and generated occurrences, and returns visual content associated with the semantic index. Layout and export are outside this consumer. Do not recover the original by re-evaluating the file, comparing tree sizes, or rebuilding records from final text. Rust/morphological equality cannot identify an occurrence: two elements with equal fields and bodies can still be distinct occurrences.

## Public boundary and ownership

This consumer owns the L1 realization facade and its result:

```rust
pub struct RealizedDocument {
    pub visual: Content,
    pub introspector: TagIntrospector,
}

pub fn realize_document(
    content: &Content,
    world: &dyn World,
    source: &Source,
    registry: &ElementRegistry,
    previous: &TagIntrospector,
    full_error: bool,
    target: EvalTarget,
    features: Features,
) -> (SourceResult<RealizedDocument>, Vec<SourceDiagnostic>);
```

Use existing types at their canonical paths: `Content` in entities/content, `TagIntrospector` in entities/introspector, `World` in contracts/world, `Source` in entities/source, `ElementRegistry` in entities/element_registry, `EvalTarget` in compiler/eval, and `Features` in entities/compiler_features. Do not change these contracts for implementation convenience. Receive resources and configuration explicitly; use no filesystem, clock, environment, network, or mutable global state.

`previous` is the prior attempt's contextual result. Recipes and contributions depending on it may need another attempt; that does not permit executing the common program again. An invalid prior result must not accumulate in the new index.

The closed enum and immutable payloads transporting content, recipes, and provenance belong exclusively to `00_nucleo/prompts/core/entities/content.md`. Evaluation contexts, scopes, and lexical capture belong exclusively to `00_nucleo/prompts/core/compiler/eval/mod.md` and the owners of markup, rules, and closures. This consumer does not redefine entities, their constructors, equality, hashing, projections, or traversals.

Transport preserves body, lexical rule order, effective style, guards, causal file, function-captured environment, and presence distinctions. Do not use PropMap, vtables, dynamic native dispatch, or render values as substitutes for morphology. Internal transport must not become a new language-observable element or change `repr`/casts/query of affected elements.

## Identity, realization, and coordination

Before visually consuming a locatable element, register its observable snapshot and associate its identity with the occurrence. Identity does not depend on layout coordinates, text, span alone, or memory address. Geometry is associated later by the layout/pipeline consumer. Repeated realization preserves correspondence for stable occurrences, without promising stability across edits beyond the existing Locator contract.

Recipes receive the original element with its occurrence provenance. If a recipe returns that element without replacing it, do not register an extra phantom occurrence. Each produced element receives its own identity. Content not yet prepared receives distinct occurrences when inserted more than once. Additional references to the same already prepared `it` retain their original identity, including multiple visual emissions. The `query double_alias` probe on pin `9f2b6e871` returns one occurrence; `cloned_content` returns the original and two produced occurrences. Preserving identity in transport does not permit collapsing two insertions by equality.

Register originals and generated elements in causal realization order. The index retains observable fields, labels, relations, and locations; do not choose one index by item count. Do not emit old visual content for a replaced element merely to keep it queryable. Preserve lexical styles, original context, and guards when applying a recipe. Matching and termination remain with their respective owners; do not duplicate those algorithms. Preserve already ratified language divergences, including the termination contract, until a new measured decision. Errors and warnings retain their spans, causal order, channels, and existing limits.

L3 coordinates evaluation, realization, layout, and page/position snapshot updates. L1 owns transformation and identity decisions; L4 only composes services. The same semantic realization serves PDF, HTML, bundle, and query while respecting target/features. Queries use the realized index and carry captured fields; they do not evaluate a second time or count approximate trees to decide composition.

A compilation without recipes or contextual dependencies should preserve its content without unnecessary reconstruction. Demonstrate this with execution counts and validation; absence of `#show` in the main file is not proof.

## Observable acceptance and performance

The language and execution acceptance conditions are:

- A heading replaced by text remains queryable with its original body.
- An original producing one or two headings preserves original and generated items in order corresponding to the pinned oracle.
- Equal headings, two insertions of one content value, and elements sharing a span remain distinguishable.
- A recipe returning `it`, nested rules, labels, `where` selectors, show-set, local styles, imports/includes, matrices, and contextual content preserve behavior and effect order.
- An ordinary document, with and without show, executes non-contextual code once. A test World and call counters prove actual execution; equal final scopes do not suffice.
- Contextual repetition does not accumulate stale records, lose originals, or duplicate counters merely by passing through the compiler.
- PDF, HTML, bundle, and query meet applicable semantic, syntactic, and morphological cases; validate geometry where it is observable.

Measure performance with paired before/after runs that preserve HEAD, working tree, binary hashes, sources, commands, times, elapsed time, and memory. Do not invent a target gain. Report no gain and regressions. A fixture with a language failure cannot qualify as an equivalent performance comparison.

## Occurrence normalization and index construction

After applying recipes, complete identities for produced locatables and preserve the identity of every already prepared `Located`, including extra visual insertions of the same alias. Each insertion of new, unprepared content receives a new Location. The origin inside `Realized` is an immutable snapshot and does not count as a visual insertion; the first insertion of alias `it` retains the original identity. A cloned carrier is not a language occurrence.

For `textual=true`, recipes delegate to `realize_captured_text` after realizing the group's elements; for `textual=false`, they reuse `apply_show_rules`.

Finalize visual insertions after recipes: flatten produced sequences while retaining the origin in a `Realized` marker with empty visual content, and reuse `normalize_markup_spaces` from the textual owner. Visually empty provenance is not a textual boundary. Do not normalize raw `ContentBlock` spaces before insertion.

When realizing a partially realized tree again, preserve its Locations and observe the used range in the input graph, including consumed origins, before allocating new occurrences. Do not reserve the range of invalid attempts in `previous`. Finalization keeps no insertion counter for prepared aliases: `Located` already carries identity. Visiting unprepared content assigns a new Location; existing carriers retain their IDs without a second reconstruction in the callback.

Before recipes run, also reserve IDs of `ContextBlock`s already present or covered by `context_id_floor` in lexical snapshots. Contextual allocation is compilation-local and must not restart in the range occupied by captured content.

### Reuse the realized index — 2026-10-05

The measured sequence in `01_core/src/compiler/realize/mod.rs:197–200` indexes visual content before `eval/mod.rs:888–896` applies `wrap_page_columns`. To make the result contain the same content published by `Module`, call the existing `Content::wrap_page_columns` helper after `finish_occurrences`/`normalize_insertions` and before constructing `RealizedDocument` and its index. The helper remains owned by `00_nucleo/prompts/core/entities/content/page_structure.md`; do not duplicate it or apply it again on the realized eval path. Revalidate originals/products, columns, and contexts against fresh reconstruction.

### Captured rule IDs — 2026-10-06

Measurement: `observe_occurrences` in `mod.rs:89–107` reserves Locations and `context_id_floor` but omits rule IDs; `show.rs:382–383` allocates rules from the `EvalContext` counter. `controls-first.json` refutes a local rule being applied to an alias when the counter restarts. Before executing any recipe, advance `next_rule_id` past all `RuleId`s in captured `ShowRecipe`s, preserving the range of later snapshots not yet visited. The `apply` owner provides per-occurrence protection, and the termination revision confirmed in `apply-l0-approved.md.txt` supersedes preservation of the historical policy only for productive realization of locatables. This consumer only reserves IDs and delegates application; signatures and phase do not change.

## Automatic math insertion — ratified 2026-10-06

The approval receipt `lab/history/construction/records/00_nucleo/materialization/math-bare-aprovacao-2026-10-06.json` records human confirmation `confirmo` of manifest `149d6535b442a9275aa6612647ad79eac460351808ea1858175d09d1cf07e1f6`, promoted before tests/code. This satisfies the original ADR-0127 gate for the package below. It does not ratify general parity, D07, math labels, or other debt.

### Historical insertion measurement

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted working tree, UTC/state/diff/source/binaries are in canonical collections `lab/results/construction/diagnostics/math-bare-2026-10-06/{before,additional,refuters}.json.gz`. Vanilla pin: `9f2b6e8715237cb086899a42873660fe744622e8`; provenance report: `lab/results/construction/diagnostics/math-bare-medicao-2026-10-06.md`.

Before: bare `frac`, `mat`, `sqrt`, `lr`, and `op` produce one `Equation` in vanilla and zero in Tekt; `math.bold([x])` and ordinary text produce zero in both. `nested` and `nested-equation` produce one in both; `reuse` produces two in vanilla. `block-style`/`local-block`/`number-style` resolve insertion scope. `show-generated` and `context-generated` expose the same gaps. Additional probes: explicit `Equation` preserves fraction rendering; vanilla rejects `it.body` with `maximum show rule depth exceeded` while `it` is valid; a later show affects only the second insertion. Refuters measure raw `frac` value and original label versus explicit `Equation`.

`lab/typst-original/crates/typst-realize/src/lib.rs:245–265,302–334` transforms `Mathy` before show outside math. `01_core/src/compiler/layout/equation.rs:680–689` projects math `nu` to text; `compiler/realize/mod.rs:155–199` prepares show/identity without math normalization. `entities/content/traversal.rs:14–108` is bottom-up and recurses into math bodies; `rules/apply.rs:564–607` handles only existing locatables. These are language semantic/morphology/render observables, not requirements for vtables, buffers, Rust equality, or PDF bytes.

### Ratified exclusive responsibility and facade

Install private descendant module `bare_math`, keeping this facade as coordinator for recipes, identity, and index. Before preparing occurrences or applying `ShowRecipe`, normalize math insertions with the `bare_math` owner's helper, respecting styles and rules from the recipe snapshot. Do not apply the final snapshot to all content or modify the consumed origin of `Realized`.

The proposal adds exactly this pure public facade, using existing types:

```rust
pub fn realize_math_insertions(
    content: Content,
    ctx: &mut crate::compiler::eval::EvalContext,
    engine: &mut crate::entities::engine::Engine<'_>,
) -> crate::entities::source_result::SourceResult<Content>;
```

It realizes the already evaluated contribution in the causal `Engine` it receives: normalization, productive show, and occurrence preparation with existing allocators. It creates no new `World`, global `EvalContext`, `Source`, index/layout, or second execution. Restore productive mode and transient options on success and error; keep guards, sink, IDs, and provenance in the supplied context. Reserve existing carriers before allocating, without colliding with prior occurrences. `realize_document` and `RealizedDocument` signatures remain exactly unchanged. The prior unchanged-phase clause is superseded only for math preparation semantically before show/index, after this package is ratified.

### Math gate, contextual snapshot, and no-math guard

The human-confirmation gate for `math-bare-2026-10-06.md` was satisfied by the receipt above; RED→GREEN and bilateral scope revalidation remain required. The ratified package supersedes earlier phase restrictions only at the named boundaries.

Measured `after-additional context-show-render` proves loss of external show. The contextual collection uses a new `Engine` with empty rules, and `ContextBlock` has no rule field. A `pub(crate) capture_context_recipe` helper transports a lexical snapshot through `Styled`/custom `eval.context.recipe: Value::Content(ShowRecipe with Empty body)`, only when an active Equation rule exists, via the static helper in `bare_math`. In the contribution Engine, extract this transport and apply captured rules/guards/file, restoring previous values on every return. Do not turn private metadata into a queryable element or Location.

Deferred capture of `Mathy` stores the raw `Span` in custom `eval.math.origin`. If realization of that recipe fails, only a detached error receives the captured origin; normalize formatting and `Source` through existing formatters. Do not invent spans without evidence or overwrite a known span on an internal product.

Measurement `delivered-infra.log` (`p1493_descendente_unico_leitor` and `fase_html_latched`) fails after `finish_occurrences` allocates extra identities in an HTML contribution without `Mathy`. `realize_math_insertions` is not general realization. If the helper finds no external `Mathy` and no lexical Equation snapshot, return the existing `Content` before observing/preparing/allocating or applying show. If an Equation snapshot exists but there is no math target, also preserve the product. Do not change budgets, execution, IDs, or context selection for content without math.

## Ratified Table insertion and incomplete Cell scope

The approval receipt `lab/history/construction/records/00_nucleo/materialization/table-cell-realizacao-d08b3-aprovacao-2026-10-07.json` records user confirmation `comfirmo`, manifest `5fec42835e38e9a28ee2ea2714b30135f069f111c140bb5e918a0902a06cc82b`, and materialization with documented metric residuals. This satisfies the original D08B3-A gate; it does not complete the Cell domain or promote the excluded scope. Retain the receipt’s limitations on raw default typography, D08B3-B isolated Cell/fresh reconstruction, and generic Table equality/rich properties.

### Measurement before the proposal

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted working tree. `lab/results/construction/diagnostics/table-cell-realizacao-d08b3-2026-10-07/{before,additional,order,aliases}.json.gz` records pin `9f2b6e8715237cb086899a42873660fe744622e8`, UTC, diffstat/status, sources/binaries, arguments, and results. Oracle binary SHA-256: `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`; Tekt binary: `7c0957f45fe40290aa809e3358c9a87979aa1e28058511ad0ee978feea6713a2`.

Vanilla `lab/typst-original/crates/typst-library/src/layout/grid/resolve.rs:204–278` resolves fields, `:1500–1530` rejects conflicts in order, `:1665–1703` creates empty cells, and `:1945–1988` derives breakability. Tekt `grid_placement.rs:91–112` prioritizes explicit placement and `:262–270` knows only `GridCell`; `realize/mod.rs:160–200` visits recipes bottom-up; `table.rs:127–152` discards wrappers without attributes. `order` measures zero cell effects when a table is replaced and zero heading effects when a cell hides it; `aliases` measures `LOCAL a`, `OUTER LOCAL a`, and `NEW a`. These are language observables, not requirements to copy mechanics, bytes, or Rust equality. Unknown fields, hidden effects, or wrong order after a change would refute the hypothesis; cache/pagination equivalence is not inferred.

### Ratified exclusive responsibility and facade

Register private descendant `compiler/realize/table_cells` with its own owner. Add exactly this pure public facade:

```rust
pub fn realize_table_insertions(
    content: Content,
    ctx: &mut crate::compiler::eval::EvalContext,
    engine: &mut crate::entities::engine::Engine<'_>,
) -> crate::entities::source_result::SourceResult<Content>;
```

Use the received causal `Engine`/context and restore modes/styles/rules/guards/file on success and error. Do not re-evaluate source or create a new `World`/index/layout. Realize Table insertions at effective points (including show/context products) before final indexing, coordinating with `bare_math` without replacing its guard. Existing private contextual capture may retain Table/Cell rules as well as Equation rules; store no resolved state in Styles/custom. Build the plan only after the table rule leaves a Table to emit; child/cell is a terminal boundary until its turn, preventing premature bottom-up traversal.

### Ratified gate, acceptance, and explicit scope split

The ratified package adds five public fields, two public carriers, and one new public facade, and changes the logical phase before Cell show. The receipt above establishes ratification for this scope. Continue to require RED→GREEN, suites/build/lint/restricted preservation, and D08A/B2, D05/D06/D07/query collections. Acceptance is language, effect, and actual geometry behavior; byte equality or whitespace normalization cannot close render acceptance.

This first package D08B3-A is incomplete in the Cell domain: D08B3-B is to complete isolated Cell defaults (`raw-cell-no-placement`/`outside-fields`) and the separate decision on fresh generation by `show-cell-reconstruct`. Generic Table defaults/fields (`table-root-equality`), rich styles/functions/paint/strokes/insets, rich rows/gutters, multi-page spans, and PDF/UA conformance remain outside certification. Do not change additional legacy D08B2 casts, syntax, flags, or global non-locatable policy in this package.

### Measured internal refinement — 2026-10-07

Measurement before decision: `lab/results/construction/diagnostics/table-cell-realizacao-d08b3-2026-10-07/{after-second,refuters-second}.json.gz`, with pin/binaries/HEAD/state/sources/UTC/argv/results. `after-second/context-table` and `nested`; `refuters-second/reused-prepared-table` show preparation with rules absent. `capture_table_insertion` in `realize/mod.rs` ignores `ctx.apply_show_rules`, unlike `eval/rules/intercept.rs:22–25`. The refutable hypothesis is that the original capture pass baked the plan without rules. Capturing a value with `apply_show_rules=false` neither prepares nor captures Table; the effective insertion and causal facade do that work. The facade also restores the `eval.context.recipe` snapshot when the product is a raw Table without a Styled wrapper. Acceptance is bilateral behavior, effects once, and geometry; do not normalize whitespace or alter ratified contracts.

## Ratified transparent source-origin boundary

The prior measurement `lab/results/construction/diagnostics/table-origens-d08b3c2-2026-10-07/baseline.json.gz` records HEAD/working tree/UTC/sources/argv and binaries at pin `9f2b6e8715237cb086899a42873660fe744622e8`. This unit has the corresponding transparent operation in existing `ShowRecipe`/`Located`/`Realized` branches; `Source` must not leak as a language-observable value. Human approval was `confirmo`, 2026-10-07; final approval and execution record: `lab/history/construction/records/00_nucleo/materialization/table-origens-d08b3c2-2026-10-07.md`. The nested `table-origens-d08b3c2-2026-10-07/contract.md` is the pre-approval candidate retained in that history directory.

A crate-private helper exposes only the existing Table/Cell predicate to eval for causal block capture; it adds no public contract. Retain exclusive ownership and static dispatch, with no I/O, new signature/trait/flag, or eval↔layout phase change. C2/C/macro tests and collections, builds, and lineage remain necessary before certification.

## Internal content traversal and allocation decisions

### Finalizing occurrences without a change — 2026-10-09, superseded below

The prior measurement `lab/results/construction/diagnostics/table-local-scaling-followup-2026-10-09/table-map-passes-before.json.gz` attributes one Table reconstruction to `finish_occurrences` in each 100/1,600-cell fixture without any `locate` call in that pass. `finish-occurrences-red-gate.json.gz` records HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted working-tree status/diffstat, UTC, release SHA-256 `7a37e3178feb4bd25b58998f1ff99b10e899c935b7d5a2d767b00f0845120ba0`, scripts/interposer/fixtures/commands, and **498.251** glibc requests for `local-set-1600`, above the pre-fixed gate of 490.000. `realize/mod.rs:44-54` calls `map_content` even when no node receives a Location. In `entities/content/traversal.rs:25-55`, `Source` is transparent, immediate body of `Located` receives no callback, `Realized` traverses only visual content, and `ShowRecipe` traverses body. `compiler/introspect/extract_payload.rs:18-98` determines effective candidates; `is_locatable` is not a proven substitute by variant (for example, its `OutlineEntry` arm does not appear in the extractor). Refutable inference: a conservative borrowed preflight could remove reconstruction if it proves there is no effectively visited candidate; unknown variants keep the old pass.

The internal continuous-flow decision was: before `finish_occurrences`, prove by reference there is no unlocated locatable in known containers using the same `Source`/`Located`/`Realized`/`ShowRecipe` boundaries as traversal; if there is any candidate or unclassified variant, run `map_content` fully. `entities/content/source.rs:23-26` shows `with_source_span` discards an outer wrapper when the body already has an origin, or the wrapper when its span is detached; therefore only a canonical `Source` with its own non-detached span and a body without a span could be preserved directly. Other shapes keep the prior traversal. Do not change location, order, payload, enum, generic traversal, public contract, or phase. Acceptance was RED→GREEN counts/allocations, trees with and without needed locations, paired time against prior release and pinned vanilla, visual/query qualification, and `cargo build && crystalline-lint .`; remove if divergent or robustly slower.

### Reuse during insertion normalization — 2026-10-10

Prior measurement: `snapshot-normalization-before.json.gz` in `lab/results/construction/diagnostics/table-local-scaling-followup-2026-10-09` preserves complete sources and UTC; `realize/mod.rs:107–143` flattens and reconstructs every Sequence even without internal Sequence or changed space. `entities/content/traversal.rs` already propagates explicit absence of substitution. Inference: signaling `None` for canonical sequences may remove allocations; counts and time against the captured release can refute it.

Internal decision: after children, return `None` for a Sequence of at least two children when neither flattening nor space normalization is needed. Borrowed normalization belongs to the textual/space owner and shares policy with the in-place variant. Keep canonicalization of empty/single-child sequences and flattening of Sequence and `Realized` with visual Sequence, including the marker with empty original visual. Preserve child changes, provenance, order, inferred/explicit spaces, and boundaries. Do not use Content equality, hashes, or pointers to infer change. Public contracts and phases remain.

Acceptance: RED→GREEN reuse-without-change and flatten/canonicalization controls; Unicode spaces and carriers; paired allocations/time, text/geometry/raster preservation against prior release and pinned-vanilla controls; core suite, cargo build, and lint. Remove if divergent or robustly slower.

### One traversal after change-proof propagation — 2026-10-10

`lab/results/construction/diagnostics/content-preflight-removal-2026-10-10/{before,measurements,confirmation,allocation-red,allocation-green,validation}.json.gz` records HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted working tree, status/diffstat, sources, UTC, commands, fixtures, and binaries. Prior release: `2d7f5dc93f6a7f15c81ff93b74270c80a14077d6753c8f79c73c9575d113d5f1`; candidate: `9ae0a3d9285b8d0b4bae0da1db058483e7f5543003fabff46bb7d93b17e474f2`; pinned vanilla `9f2b6e8715237cb086899a42873660fe744622e8`/binary `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`. The before snapshot preserves borrowed guards; `entities/content/traversal.rs:24–82` provides explicit proof in mapping. Removing the two guards reduced paired `blocks-400` time 1.21% in both collections, six rounds with negative medians; six cases retained allocation counts. 34 text/geometry/raster/query controls were preserved; core suite remained green. Math/repeated-block cases remain outside equivalent-work comparisons with vanilla.

Current internal decision superseding the preflights above: use `map_content` directly in this unit, without a duplicated borrowed preflight. The explicit no-change proof protocol belongs to the pinned `content/map-content-change` Nucleus. Keep the existing callback, boundaries, order, error behavior, origin, location, and transformation. Do not infer change by equality, hash, or pointer; expand no variants or replace the enum with dynamic dispatch. `Source` remains transparent and preserves first origin per the current Nucleus. No public contract, default behavior, phase, or compatibility changes.

Acceptance: retain controls with/without columns, already located/pending occurrences, origin, effects, and query; core tests, build, and lint. Paired-time external gate must move from baseline with no gain to repeatable candidate benefit and no robust regression, without increased allocations. Arc reuse is auxiliary engineering evidence; language acceptance uses text, geometry, morphology, and effects, without requiring equal PDF bytes. This is a small improvement and does not certify general parity or lower resident memory.

## Boundary arity — `clippy::too_many_arguments`

### Evidence before the decision

The structural reading uses HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted working tree, captured `2026-10-15T16:39:18.873477+00:00`. Full sources, prior L0s, signatures, hashes, UTC, status, and `git diff HEAD --stat` are in `lab/results/construction/diagnostics/too-many-arguments-2026-10-15/before.json.gz`. Individual classifications and clauses read before the decision are in `lab/results/construction/diagnostics/too-many-arguments-2026-10-15/classification.json`. Clippy spans were reconfirmed against the prior inventory sources. This is Rust structure measurement, not new proof of parity or functional failure.

- `C020`: `01_core/src/compiler/realize/mod.rs:127`, `realize_document`, 8 arguments counted by Clippy (threshold 7).

### Exclusive prescription and exception boundary

The corrections below were maintenance targets not yet materialized in that audit. Preserve current public signatures and language decisions. The need for structural correction is maintenance judgment based on the stated data; the arity warning alone does not prove a functional defect.

For `C020` (`realize_document`), retain the exact public realization signature: content, resources, source, registry, previous snapshot, and configuration are explicit. The registry must not be removed from the API because this lint currently finds it unread. Preserve signature, order, and semantics. At materialization, apply exactly `#[expect(clippy::too_many_arguments, reason="C020: Contrato público de realização reproduzido literalmente no L0: conteúdo, recursos, source, registry, snapshot anterior e configuração são explícitos")]` to this function only. Confirm the warning in Clippy workspace/all-targets and no `unfulfilled_lint_expectations`; preserve call sites and functional controls. Do not raise the global threshold or suppress other lints.

Authorize only `#[expect(clippy::too_many_arguments, reason = "...")]` on the named item with the stated architectural reason. Do not apply it to module/impl/crate/configuration; do not raise the threshold or cover another lint. For cases removing dead arguments, any residual exception depends on that correction. If the function no longer exceeds the threshold, remove/review the expectation; do not silence `unfulfilled_lint_expectations`. This specific permission overrides generic suppression prohibitions only for the named lint/items; all other correction duties remain.

This audit updates only L0 and its lineage seal; suppression attributes and signature/body changes remain pending materialization. Corrections require owner language/integration tests and before/after call controls without relaxing fixtures or tolerances. Preserve layers, phase, defaults, static dispatch, and absence of mutable global state. Private signature corrections also migrate callers and their L0 before code. Any new public contract or compatibility need requires a concrete proposal and ADR-0127 confirmation; this section does not pre-authorize it.

### Materialized named exceptions — 2026-10-16

The validated inventory in `lab/results/construction/diagnostics/clippy-locais-2026-10-16/clippy-inventory.json` contains 79 warnings. Reconfirmation of sources/spans, current reasons, signatures, and hashes for this batch is in `lab/results/construction/diagnostics/clippy-excecoes-2026-10-16/before.json.gz`, with HEAD, uncommitted working tree, UTC, and `git diff HEAD --stat`. The functions and concrete data were read before this decision. Arity and type heuristics are Rust mechanics; they are not new parity or measured cognitive-benefit evidence.

Materialize only direct named expectations already justified in this owner: C020, reconfirmed against current spans and individual clauses. Use exactly `#[expect(clippy::too_many_arguments, reason="ID: motivo prescrito")]` on the function, never module/impl/crate; preserve signature, body, calls, cfg, and unrelated attributes. Items classified `fix`, including conditional C047/C066, remain pending and receive no annotation in this batch.

Continuous ADR-0127 flow: this batch adds lint metadata only; it changes no public contract, executable body, cfg, defaults, phase, dependencies, or architecture. It materializes only expressed named exceptions and supersedes generic suppression prohibitions only for those items/lints; it does not authorize silencing pending fixes. L0 comes first, followed by restricted resealing and proof that removing the added attributes and lineage header recovers the prior source after rustfmt. Measure RED first; then check all-targets inventory, exactly 46 arity warnings and five test warnings satisfied by 49 expectations, no new warning, and `unfulfilled_lint_expectations` treated as error. The 11 arities targeted for correction remain visible. Validate build, format, strict/reverse lineage, Tekt with no new errors/warnings, existing Counter tests and coverage under 12 GiB, zero swap, one job. The prior full suite remains evidence for the same executable code; do not create tests that mirror lint attributes or require a new performance comparison for metadata.

## Ratified nominal Sequence realization

### Measurement before the decision

`before/expanded/controls.json.gz` and `candidate.log` in `lab/results/construction/diagnostics/sequence-constructor-2026-10-17` fix HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, uncommitted working tree, UTC/diffstat/sources/binaries, and vanilla pin `bd5ebec285505889e8ff1d4f68804ab147a699f3`. Eight initial REDs; five passed in the candidate, while show and recursion did not. `markup_stream.rs:240–260` flattens before selecting the sequence; `realize/mod.rs:170–210` visits children before recipe; `apply.rs:140–170` delegates non-locatables to fixed point. Vanilla accepts alias and rejects a new Sequence with equal children. These are language effects, scope, values, and diagnostics.

The owner approved `lab/history/construction/records/00_nucleo/materialization/sequence-realization-2026-10-17/review.md`, manifest `46132cb53898f2b126a08bf3b0495ed4c05b531c380846bccbd3d0906477c327`, in that session. ADR-0127 §2.3 covers this new boundary's order before children/Table/math; it does not cover a new public contract.

### Exclusive responsibility and acceptance

Compose private descendant `sequences` and call its pre-order realization of marked recipes before `realize_table_insertions`/`bare_math` and generic `ShowRecipe` traversal. A selectable Sequence boundary can hide its children. The descendant resolves its products/guards; the hub retains existing routes for other recipes, layouts, indexes, and targets. Expose only crate-internal paths needed by markup/invoke/apply, with no external `pub`.

Ratified S1–S20, RED before code, bilateral GREEN, and regressions are required; do not reduce expectations for mechanical limits. All owners must remain 1:1 and pinned nuclei valid before resealing. Validate separated build/workspace/doctests, strict Clippy/Rustdoc, fmt/lineage/reverse lineage, and SARIF with no new errors/warnings. Limits: 12 GiB/High8 GiB/swap0/jobs1; run heavy commands sequentially. No publish/release.

The existing `observe_occurrences` pass also sets private flag `has_deferred_show_rules` when it finds a marked Sequence recipe. The new realization context starts without it; set it before composition to avoid skipping empty/local bodies. With no Sequence marker/rule, the descendant returns a shallow clone without traversing/reconstructing the document. Do not create a public product field/flag or global state for this shortcut.

## Ratified causal realization

### Measurement before the decision

`lab/results/construction/diagnostics/textual-realization-2026-10-18/{before,scope,refuters}.json.gz` preserves HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, uncommitted working tree, UTC, status/diffstat, source/argv/binary hashes, and vanilla pin `bd5ebec285505889e8ff1d4f68804ab147a699f3`. `before-sources.json.gz` contains full source/current L0. No candidate code was implemented in this measurement. `matching.rs:118–150` applies reverse regex and then declared literals; `:194–207` exposes products to later passes. `textual.rs:102–123` captures the full snapshot. `realize/mod.rs:177–217` visits recipes bottom-up. `apply.rs:620–648` and `matching.rs:218–225` call functions without capturing children. `expression_dispatch.rs:125–129` already distinguishes `insertion_rules` from local state.

Measured examples: identical literals C/B; literal/regex tie R/L; overlap X/YB; first effect LATER/FIRST; inverse chained function BBBB/BB; separate product AB/X; product plus neighbor X/AB; Strong hiding a child panics HIDDEN/Q; captured global followed by new global gives L/R, while captured local gives L/L. Local CJK input `水\n果` gives `水果/X`. Pinned `foundations/selector.rs:110–125` unifies escaped literal with regex; `typst-realize/src/lib.rs:1328–1405` chooses the start and carries revocation on the product. These data measure language effects; they do not ratify Rust mechanics, buffer equality, or PDF-byte equality. It is an inference that scope separation and visiting before children explain the results; `acceptance.json` controls for equality/fields/style/query/errors can refute it.

### Ratified decision, exclusive responsibility, and acceptance

Apply the pinned `textual-recipe-frontier` nucleus in this L0. Human confirmation of manifest `47e714dfbb813eae9f2be20b8f57de5c197cda13db1871c8bb3da49a08430302` ratified this order. Within this scope it supersedes historical clauses for provisional show execution in `ContentBlock` and bottom-up visits of pending recipes, without changing expression evaluation order. Do not re-evaluate the program, advance State/Counter, settle render into morphology, or create a public Rust contract.

Delegate recipe execution to `compiler/realize/recipes.rs`. Replace the global Sequence prepass plus bottom-up `ShowRecipe` realization with causal coordination of root and product; do not execute a hidden Sequence merely because it exists in the graph. Observe/reserve identities and contexts before allocation; preserve source/introspector, `finish_occurrences`, and final normalization. Prepared Table/Cell follow their owner and order Table→placement→Cell→body; Mathy must be prepared before selecting its own insertion. Do not move these decisions into layout/common eval or call the program again. `realize_document`/`realize_math_insertions`/`realize_table_insertions` facades remain unchanged.

Require RED→GREEN for `acceptance.json` families, R2-S/R3 corpus, and one-evaluation controls. Revalidate PDF and HTML separately; build/workspace/doctests, strict Clippy/Rustdoc, fmt, V1/V5/V15/V26 and reverse lineage; differential SARIF with no new errors/warnings. Limit: 12 GiB/High8 GiB/swap0/jobs1, heavy commands sequentially. Do not claim RAM gain before paired trials or clear gates with suppression. No commit/push/release.

### Causal realization refinements

**Preserve final-normalization provenance.** Final normalization does not extract visual Sequence from a `Realized` whose original carries the private textual-product marker. This boundary preserves guards and provenance for later insertions. Ordinary sequences and legacy `Realized` continue through existing normalization. R3-T1's separate-product AB/X measurement supports the restriction; do not discard transport after causal coordination.

**Install contextual math snapshot before its insertion.** `workspace-rest.log` (candidate7) refuted the contextual math facade: the captured Equation rule was installed only after `recipes::realize` had prepared the fraction; the later `needs_preparation` returned false and skipped it. In `realize_math_insertions`, install the `eval.context.recipe` snapshot (rules, file, floor, and guards joined with current ones) and realization flags before causal coordination. Run `recipes` once and finish occurrences; restore everything on success/error. Coordination already prepares Mathy before selection. Do not repeat the global prepass or apply show in a second traversal. Keep deferred return while `defer_show_rules` is active, current public signature, and lazy State/Counter realization. The existing integration test requires EQ in contextual PDF and is the RED; do not change its expectation.

**Leave raw contribution identity canonical.** `workspace-rest-candidate8.log` refuted finalizing every occurrence in an isolated contextual contribution: two HTML controls lost stability and query for shown Equation lost the original. The facade has only the contribution, not the canonical range of the full document. Leave raw nodes without recipes for contextual owner's later indexing/positioning; do not call `finish_occurrences` indiscriminately in this facade. Identities needed to execute an actually matched recipe remain in the coordinator. Installing the captured snapshot before Mathy preparation remains mandatory. The causal collision hypothesis is supported by `Locator::new` and Context/Located markers; the three existing tests and future multiple-producer measurement can refute it. Do not invent a Location/namespace or expand API.

**Keep nominal guards and V17.** Candidate9 SARIF found V17 in `flatten:66`. Expand the compound `Realized` guard: first exclude protected products in the simple guard; in the body test visual Sequence, retaining emission of the empty original marker and flattening only in that case. For non-Sequence visual content, emit a clone of the original content, as in the prior fallback. Preserve test order and effects; remove no product guards and normalize no other boundary. V17 must pass without suppression. This refinement changes no behavior, public contract, or ratified phase; revalidate existing tests and gates.

## Context — lexical inheritance, 2026-10-18

### Measurement before the decision

HEAD `b7e7c83223e5daa67bbdf124c36b5b7f93200731`, clean productive source; `lab/results/construction/diagnostics/context-paridade-2026-10-18/before.json.gz` contains UTC, argv, diffstat, and executable SHA. New build recipe is in `baseline-build-resources.json`; full freeze in `before-sources.json.gz`. Vanilla pin: `bd5ebec285505889e8ff1d4f68804ab147a699f3`. Global/local probes return A/A in Tekt versus R/L; creation before rule, nesting, regex, and product also lose the rule. Local control inside Context gives L in both. `compiler/realize/mod.rs:240–269` captures only Equation/Table; `compiler/realize/recipes.rs:343` treats Context as a generic leaf; `textual.rs:46–103` already preserves baseline and composes local delta by RuleId. Pinned vanilla `foundations/context.rs:78–86` evaluates with styles at the contextual point; `typst-realize/src/lib.rs:353–418` prepares root, applies the rule, and visits product. This measures language scope/effects and does not require copying upstream structure. The cause of rule loss is inferred from these paths and is refuted if scope, nesting, product, or parent-hiding-child controls fail.

### Decision, boundary, and responsibility

This is an internal parity correction within already ratified R3-T1 contextual realization: retain deferred execution, API, and phases. Do not address Location collisions using offsets/hashes, remove `Located`, or change contracts in this scope. Ordinary `Styled` remains opaque to textual matching; do not alter the mapper.

Lexical capture of Context retains all relevant rules, including textual rules, using `capture_styles` to distinguish baseline from local delta. With no rules, it may preserve raw content; its later insertion still inherits current rules. A private helper prepares the Context emitted during realization: compose the existing lexical snapshot with inherited rules using `compose_rules`, then carry the effective result in `eval.context.recipe`, with joined guards, file, and floor preserved. Metadata has `Empty` body and never executes callbacks. `realize_math_insertions` continues restoring the effective snapshot before realizing the contribution; it does not advance the closure or change its signature.

The existing `eval.context.recipe` lookup is shared by a private helper borrowed by `realize_math_insertions` and `prepare_context_recipe`. Keep the same shallow projection of `Value::Content(ShowRecipe)` and absence for other values. Do not duplicate dispatch for the same metadata or change local Table precedence.

## Context — occurrence identity, ratified 2026-10-18

### Measurement and gate

`before.json.gz` in `lab/results/construction/diagnostics/context-paridade-2026-10-18` preserves HEAD `b7e7c83223e5daa67bbdf124c36b5b7f93200731`, sources, UTC, diffstat, argv, and binary SHA; `baseline-build-resources.json` contains the new recipe. Vanilla pin: `bd5ebec285505889e8ff1d4f68804ab147a699f3`. Two equations give EQ EQ Q1/Q2; Heading prefix H EQ Q0/Q1; query of `it.location` gives 00/01. `context_stabilization.rs:257` restarts `EvalContext` per producer; `eval/mod.rs:438` restarts Locator; `realize/mod.rs:41` allocates without a shared domain; `introspect/walk.rs:375–390` deduplicates by Location. This is language identity. Upstream `typst-realize/src/lib.rs:353–418` prepares identity before the recipe but does not require copying its representation, vtables, or algorithm. The collision hypothesis is refuted by failing count, alias, callback, or replay controls.

The human approved `review.md` under `lab/history/construction/records/00_nucleo/materialization/context-occurrences-2026-10-18`, SHA-256 `500e01cf0754a166deb7f493ff734f040ef38e0374cbf5aa0ba879a37d19f388`; `approval-receipt.json` is in the same directory. ADR-0127 §2.1 is satisfied for the specified additive API. This does not authorize unrelated phase/API changes.

### Exclusive responsibility

Declare public module `context_occurrences`, owned by `00_nucleo/prompts/core/compiler/realize/context_occurrences.md`. `locate` keeps an existing `Located` intact and delegates only acquisition of a new Location to this module's private `next` helper; without an active frame it retains current `Locator::next`. Existing occurrence reservation continues observing Locator without emitting a node, running a callback, or inventing identity from offset/hash.

## Documentary provenance

The pre-consolidation owner, including superseded implementation proposals and
dated decision history, is preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/compiler/realize/mod.md`.
All approval receipts, scope splits, current refinements and pending fixes above
retain their original authority. Historical vanilla measurements do not certify
the current pin without remeasurement.


## Approved Figure/Grid kind repair (2026-10-19)

### Measurement and classification before decision

The frozen proposal at baseline `731f0ffdf4c7c152f590551e72ff63f7bd85ef85` in `lab/results/v16-corrections-2026-10-19/figure-kind/review.md`, its `agents/contracts/design.md`, `agents/contracts/semantic-partition.json`, and `agents/counter/design.md` bind measurements and full source/oracle provenance. Current-pin oracle is `bd5ebec285505889e8ff1d4f68804ab147a699f3`. Constructor inference loses typed identity and lexical occurrence styles; semantic inspection and final supplement need separate phases. These are language semantics and causal effects, not a requirement for upstream Rust mechanics. The human explicitly approved the exact public carriers, phase changes and compatibility migration in the current session under ADR-0127.

### Exclusive responsibility

Register the private feature owners figure and figure_kind. Preserve all existing public realization facades, causal Engine, allocation namespace and source/recipe restoration. No prewalk may prepare hidden descendants.

### Acceptance and boundaries

Require functional RED→GREEN, frozen kind/set/show-set/supplement/hidden-parent cases, alias/homonym and field snapshots, full build/tests, paths, strict forward/reverse lineage, V15/V26 and differential lint. No suppressions, opaque catchall, fake neutral child or weakened assertion. Caption omission/none, complete caption synthesis, supplement-dependent preview selection, complete function-number formatting, HTML implicit captions/references, Dynamic field enumeration, and mixed Table header/ordinary-cell supplemental target order remain explicit bounded debts.

## Approved reference supplement phase

Human approval is bound by reference-phase-approval.json and frozen Ref proposal
SHA1578cbea7378b739a7c1b0809d3437c0165c2de6d288e40b2db75c9ce7e38ff2
under figure-kind-implementation. Expose public reference_supplements module,
owned by its exact canonical L0, with only the approved three doc-hidden facade
functions. Coordinate existing context recipe capture and shared occurrence
allocation; no fourth public signature or new entity field. Preserve unrelated
facades and error/warning/target/features behavior. Internal selected-template
instantiation may reconstruct raw ContextBlock IDs only under approved stable
producer/frame insertion contract, preserving preexisting Located aliases.

## Closed normalization and occurrence reservation

### Evidence before classification

Baseline HEAD `4382891f0d5f0839669597cb5c90a98f24cfb8bd`, clean productive
tree, UTC, exact bytes/SHA-256 and the complete120-variant Content inventory are
archived in
`lab/results/v16-corrections-2026-10-19/batch-g/agents/core/{source-before.json.gz,audit.json}`.
The measured `mod.rs:90–97` tests direct-child flatten eligibility; false does
not prove a subtree needs no normalization. `:118–140` reserves node-local
identifiers; its empty branch does not suppress mapped child visits. These
readings are engineering evidence, not new language or performance claims.

### Owned closed decisions

Both decisions name their complete current nominal complements at the actual
productive sites, preserving all preconditions and surrounding passes.
Sequence and unprotected Realized with Sequence visual are eligible to flatten;
protected textual products and non-Sequence Realized visuals are not. Keep the
original-bearing empty marker, encounter order, semantic/style carrier
boundaries, zero/one-child canonicalization and separate markup-space policy.

Reservation observes Located location, including its direct ContextBlock body
because the bottom-up mapper omits that body's callback; ContextBlock advances
the context floor, ShowRecipe reserves context/rule floors and the Sequence
deferred marker, and Realized additionally scans original while the ordinary
mapper scans visual. Every other current carrier has no node-local reservation
in this pass but retains ordinary child traversal. Preserve saturating floor
arithmetic and no closure/callback execution. Do not create an identity, scan
previous invalid attempts or change allocation namespaces.

### Acceptance and limits

Require protected/unprotected Realized Sequence and non-Sequence cases,
provenance markers, canonical no-change reuse, and Locations/context/rule IDs in
ordinary containers, Located(ContextBlock), and each Realized side. Preserve
lexical fields and deferred Sequence markers. Enum extension must establish
exact-site compiler nonexhaustiveness at both decisions; preservation tests
alone are not semantic RED. This internal same-owner/phase classification
follows ADR-0127 continuous flow, with no public contract/default/compatibility
change. L0-first implementation and graph-valid restricted bilateral resealing
remain required.

## Physical page operational boundaries

### Measurement and authorization

At the implementation baseline, `mod.rs:50–60` completes occurrences through a
bottom-up generic map, which also descends PageRun static fields;
`mod.rs:234–262` reserves identifiers but the ordinary mapper cannot enter
SetPage operational fields. `recipes.rs:609–660` already restores captured
recipes on every Result path. These readings identify the owning boundaries;
reservation is not semantic admission.

The frozen bilateral `lab/results/set-page-header-2026-10-21/baseline-parity/receipt.json`
compares source-bound executables against vanilla commit
`bd5ebec285505889e8ff1d4f68804ab147a699f3`: raw SetPage headers/footers
render but their Figure/Context contributions are absent from productive
semantic stores; overwritten headers remain inert. Native PageRun already
prepares raw static fields before knowing physical repetitions, so repeating
that prepared raw object would incorrectly turn independent insertions into
aliases. These are language effects, morphology, order and Location/page
observations, not a requirement to copy upstream mechanics.

The separate `lab/results/set-page-header-2026-10-21/ambient-baseline/receipt.json`
SHA-256 6291efca219a3558ad2192f99f1208d64ad3e70b38ec96db2d417527549fcb05
refutes freezing creation-time 10pt as insertion styles: the measured header
uses page-effective 20pt, then inherited page-effective 30pt on the next page.
Explicit inner header18pt/footer12pt remain binding preservation controls.

The approved review is `lab/results/set-page-header-2026-10-21/review.md`,
SHA-256 cb4a684b483e7f9f6ac3f7eef52277f7ada405033a24e4e66e55811e93c12039.
The direct human decision is preserved in
`lab/results/set-page-header-implementation-2026-10-21/approval.json`.
It authorizes only the frozen five public facades, opaque carriers and physical
page phase; it does not authorize additional public contracts or generic
observer/traversal changes. The implementation baseline is HEAD
`cf21f47a3b63f0e9fc5308925e1d0b67925e7227`. Exact owning source/L0 bytes,
SHA-256, UTC and working-tree state before these amendments are archived in
`lab/results/set-page-header-implementation-2026-10-21/agents/realization/before.json`.

### Durable responsibility

Declare the public descendant `page_running`, owned exclusively by
`00_nucleo/prompts/core/compiler/realize/page_running.md`; its public surface is
exactly the two approved realization/binding functions. Preserve all existing
facades, flags, allocation and diagnostics outside this finite boundary.

Before productive allocation, a feature-owned reserve-only scan observes existing
Located identities, ContextBlock IDs, recipe context floors and rule IDs inside
all four SetPage/PageRun templates, including overridden and otherwise dormant
ones. It executes no closure, show, kind preview, callback or fold and publishes
no query/counter/Context membership. It preserves existing aliases and does not
reserve prior invalid candidates. Keep the original lexical Content intact.

Occurrence completion treats reached SetPage as an operational capture boundary;
PageRun retains and completes only its lexical body until physical slot requests
are admitted. Do not allocate into raw dormant templates through a bottom-up
mapper before this decision. Delegate template reconstruction to the page-running
owner, keep all fields/presence/style/source carriers, and preserve ordinary
completion for all nonoperational content. Generic entity mapper contracts do
not change. For the operational path, delegate provisional index construction to
the introspection owner's body-only source-derived helper: captured templates
reserve identity but do not enter the initial body index or Context discovery.
Preserve legacy unplanned indexing outside this selected path. Pure sequence
normalization alone is not evidence of an effect defect.

Acceptance requires dormant panic/hidden descendants to remain inert, reservation
of high preexisting Location/Context/rule floors without semantic membership,
fresh raw physical insertions and stable prepared aliases, source restoration on
success/error, PageRun body preparation, unchanged legacy realization and both
target controls. No synthetic operational root, fake Location/current context,
second arena, fallback allocator or global index reconstruction is authorized.

### Empty-rule operational Context environment

Current prepare_context_recipe returns the raw Context when effective rules are
empty, so no existing Styled/context-recipe carrier transports the supplied
request's 20/30pt Engine environment into later Session collection. The opaque
resolution plan retains that environment but L3's existing content-only Session
cannot read private request fields. Retain the existing style recipe transport
also for an exact private active-page-request token supplied by page_running;
empty rules remain empty and no Context closure executes here. Ordinary no-rule
content outside that scoped token keeps the existing bypass. Preserve styles,
guards, file/floors, Source and later real Location; do not collapse chains,
add a public style getter or grant contextual capability from the token.

## Operational Context chain transport and selected Cell completion

### Measurement before correction

The source-boundary-9 actual-evaluation controls certify request chains 20/30.
The unchanged three integrated style controls in focused-diagnostic-8 still
observe 11pt. prepare_context_recipe stores the complete chain in a metadata
snapshot but returns only a custom Styled delta; L3 collection starts from
11pt and therefore invokes the actual Context closure without the page chain.
Existing ShowRecipe collection already transports its full styles chain. Exact
pre-correction source/L0 bytes are retained in agents/realization/context-transport-before
under the physical-page implementation evidence.

Only under the exact operational environment token, pass the existing prepared
Context and snapshot to the page-running owner's private passive transport
constructor. It wraps that already prepared Styled Context in a non-textual
ShowRecipe carrying the unchanged persistent Engine chain, with no executable
rules or guards. Its body-local private authenticated marker and exact snapshot
chain binding certify the prepared transport. Ordinary Context capture stays
unchanged. Never collapse/reconstruct style values or introduce a public getter,
new phase, Location, language element or weaker observer.

The same source-boundary-9 run has an actual full-source Table failure during
eval, before any Pending layout: operational completion recurses through
finish_occurrences and thereby falls back to generic Cell mapping, which selects
standalone before resolved; the actual Cell visual selects resolved first.
The provisional source index correctly requires existing canonical identities.
Keep operational completion mode through all of its child recursion. Delegate
TableCell selected finishing to its private feature owner, following resolved.visual,
else standalone.visual, else lexical body. Preserve dormant snapshots/raw fields,
existing aliases and source origins. Do not change the generic mapper, legacy
non-operational entry, allocate in the provisional index or infer identities.

Require the exact integrated style and full-source Table RED sources unchanged,
plus explicit inner18/12, ordinary captures, prepared transport revisit and fresh
pending-recipe preservation. Operational Table body/default/header/footer controls
must be canonical before the existing-only provisional index.


## Source-owned Outline product completion

Measurement precedes this correction: `eval/rules/outline.rs:270–273` publishes
new realized_body children without the canonical recipe/finishing boundary.
`source_plan` existing-only indexing requires actual canonical Location transport,
not factory allocation invented afterwards. Route-probes11 preserves actual
source-owned Outline failure; round13 separately measures title defaults and
missing empty-input dependency. The round13 integration error specifically
exposes an auxiliary Heading without its canonical snapshot; it does not isolate
completion alone. Exact current before bytes are archived under
`lab/results/set-page-header-implementation-2026-10-21/agents/realization/outline-before-production/`.

Provide only a compiler-descendant insertion helper for the existing Outline
realization owner. With an actual installed allocation owner, process only its
newly synthesized body: reserve existing aliases, scope template instantiation in
the retained frame, run existing recipes with the captured Engine styles/rules,
finish actual selected visual roots and normalize insertions. Preserve original
snapshots, Locations and dormant operational fields. Never reevaluate the source,
reapply the original Outline root recipe, execute pending Context closures, or
allocate an independent context/arena. Scope and restore realization/defer flags
and Engine styles on every result. Reuse the existing authenticated full-style
Context transport so a generated Context retains its exact captured environment;
its ID comes from the retained template slot, not a monotonic global number.
Without an actual installed owner return the original generated body unchanged,
preserving the legacy completion boundary. No public signature or phase changes.


## Test identifier compatibility

The pinned tree-sitter grammar rejects references such as `&raw` in the newly added test code as raw-pointer syntax. A direct parse probe isolated the rejected tokens; the first tuple-pattern hypothesis was refuted and its source edits were restored. Name these test-local content values `template` instead, retaining exactly the same variant checks, assertions and failure boundaries. No productive behavior, criterion or public contract changes. Exact preflight failures and prior bytes remain under `lab/results/set-page-header-implementation-2026-10-21/seal-preflight-repair/` and `lab/results/set-page-header-implementation-2026-10-21/seal-preflight-repair-2/`.

## Approved batch E integration

Measurement: pre-E inputs and bilateral reflection in
`lab/results/symbol-identity-2026-10-23/{baseline.json,before/receipt.json,additional/receipt.json}`.
The owner approved the native Symbol declaration on 2026-10-23.
Handle Symbol explicitly as a native childless textual leaf in exhaustive complements and existing projections. Preserve the complete Unicode payload and reconstruct the same variant when mapping. Do not silently classify Symbol as a bare equation outside math.
Preserve the existing unit responsibility and all other variants/effects; no public contract beyond the approved Content addition or phase move.
