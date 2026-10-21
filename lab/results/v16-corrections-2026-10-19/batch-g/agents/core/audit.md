# Batch G core audit — five selected decisions

Static evidence was captured before this plan in `source-before.json.gz`. Baseline HEAD is `4382891f0d5f0839669597cb5c90a98f24cfb8bd`; no productive or L0 changes were made and no Cargo/test/oracle command ran. Full SHA-256 inventories, the current complete Content enum and historical ledger entries are in audit.json. Historical ledger seals are not claimed to identify this baseline.

## Bounded decisions

### V16-049

Evidence: `01_core/src/compiler/realize/bare_math.rs:53-75`, `01_core/src/entities/content/traversal.rs:710-771`.

Replace only the false wildcard by every current complement variant explicitly. false means absence NOT PROVEN, therefore use the transforming path. Do not expand the proof-positive catalogue. Keep Equation/Located preserved; Table resolved cells instead of raw children/header/footer, always caption; TableCell uses visual; Realized uses visual only.

Gate: Continuous internal catalogue repair; no public/default/phase/compat change.

Tests to add after root release:

- Plain positive leaves and wrappers preserve original Arc where applicable.
- All raw/resolved Table cells/header/footer/caption and Cell visual paths; original-only versus visual-only Mathy under Realized.
- Non-proof containers containing MathFrac still transform; Metadata/Context/Dynamic are not optimistically certified absent.

### V16-050

Evidence: `01_core/src/compiler/realize/bare_math.rs:78-140`, `01_core/src/entities/content/traversal.rs:710-771`.

Close callback nominal complement explicitly; move Label Mathy guard into its arm with explicit non-Mathy Ok(None). Ok(None) delegates children to map_content_pre; Some stops traversal and replaces the complete subtree. Preserve absence/prepare_root early exits and MathStyled lifting.

Gate: Continuous internal catalogue/guard repair; no signature or phase change.

Tests to add after root release:

- MathFrac root wraps exactly once and does not wrap its denominator independently.
- Equation and Located terminal; Styled overlay and captured ShowRecipe styles; all ShowRecipe fields retained.
- Realized original untouched, visual transformed; Label Mathy wraps Label, non-Mathy Label descends; MathStyled bare versus text-only controls.

### V16-051

Evidence: `01_core/src/compiler/realize/mod.rs:62-113`, `00_nucleo/prompts/core/compiler/realize/mod.md:80-102,174-200,261-276`.

Close direct-child flatten trigger over Content. Only Sequence and unprotected Realized with Sequence visual trigger; every explicit complement is ineligible at this node, not subtree-normalization-free. Preserve protected textual product boundary, original-bearing Empty marker, encounter order, zero/one-child canonicalization and independent markup-space normalization. Also audit flatten match guarded Realized complement: existing other-binding clone covers protected variant without changing it.

Gate: Continuous same-owner same-phase structural correction.

Tests to add after root release:

- Unprotected Realized Sequence flattens with original-bearing Empty visual marker first.
- Protected textual product remains a boundary for both Sequence and non-Sequence visuals; unprotected non-Sequence also remains.
- Source/Styled wrappers retain their boundary; Unicode/markup spaces and canonical sequence Arc reuse remain.

### V16-052

Evidence: `01_core/src/compiler/realize/mod.rs:116-142`, `01_core/src/entities/content/traversal.rs:344-440`, `00_nucleo/prompts/core/compiler/realize/mod.md:80-102`.

Close node-local reservation inventory explicitly; no-op complements still allow descendant traversal. Retain Located Location observation and its direct ContextBlock-body handling because bottom-up mapper suppresses immediate Located body root callback. ContextBlock advances context-id floor saturating; ShowRecipe reserves context/rule floors and sequence marker; Realized explicitly scans original while ordinary mapper scans visual. No callback or closure execution.

Gate: Continuous internal identifier inventory repair, preserving allocator namespace and phase.

Tests to add after root release:

- Nested ordinary containers still reserve descendant Locations/context IDs/rule IDs.
- Located(ContextBlock) direct-body reservation; Realized original-only and visual-only IDs both reserve.
- ShowRecipe context floor/rule floor/deferred sequence marker; max saturation; next allocation above observed slots without evaluating closure.

### V16-074

Evidence: `01_core/src/entities/content/equality.rs:11-211`, `01_core/src/entities/html.rs:13-47`, `00_nucleo/prompts/core/entities/html.md:20-37,53-61`, `01_core/src/entities/elements/title.rs:18-22`, `01_core/src/entities/elements/context_block.rs:39-49`, `00_nucleo/prompts/core/entities/elements/context_block.md:15-19`, `00_nucleo/prompts/core/entities/elements/metadata.md:33-38`, `00_nucleo/prompts/core/entities/elements/state.md:17-21`, `00_nucleo/prompts/core/entities/elements/state_update.md:39-45`, `01_core/src/compiler/eval/operators/equality.rs:149-173`, `01_core/src/entities/content/rust_hash.rs:15-147`, `01_core/src/entities/content_hash.rs:25-29`.

Separate three justified mechanical always-false same-kind cases Metadata/State/StateUpdate, four unexplained newer omissions HtmlElem/HtmlFrame/Title/ContextBlock, and unlike-kind false. Preserve all current transparent wrappers, Space equivalence, existing payload delegations and Dynamic dyn_eq. Do not preserve the four unexplained omissions just to close V16; propose delegation to the audited entity equality only after language probes and hash check. Both axes of the residual unlike-kind partition must explicitly name the closed Content domain so sentinel addition fails here. A moved wildcard or discriminant-only guard is not accepted.

Gate: Purely explicit preservation portions continuous. Newer four rules remain decision-pending: current operator projects then uses Rust PartialEq, so language effect is real. Proven parity correction can flow L0-first under ADR-0127; deliberate public equality/compatibility change or unresolved semantics requires explicit classification/approval. No new API/default/phase is proposed.

Tests to add after root release:

- Mechanical: explicit historical false self/clone cases for Metadata/State/StateUpdate, separately identified as compatibility mechanics.
- Mechanical: Html tag/attrs/body presence/content, HtmlFrame body, Title body, Context same/different id and same-id/different closure; all seven same-kind cases tested.
- Unlike-kind symmetry; nested Source/Located/Realized/ShowRecipe transparency both directions; Space/SpaceFromNewline same hash.
- Language: actual raw title, HTML elem/frame, Context alias and two distinct Context values; ==, !=, array/dict containment and fields/morphology, using pinned vanilla and product frozen provenance. These are independent of Rust reflexivity assertions.
- Hash: HTML attributes inserted in opposite orders must respect equal-values/equal-Hash if structural dispatch is adopted. Record baseline then scope any repair separately; do not modify hash owner without root assignment.

## Additional causal dependency

HTML attributes use IndexMap 2.13.0 (Cargo.lock:1098–1101). Its PartialEq checks key/value membership, independent of insertion order (registry primary source map.rs:1866–1881), while Debug prints self.iter order (:121–128). Content Hash uses Debug serialization (content_hash.rs:25–29). Therefore adding structural HtmlElem dispatch requires an equal-values/equal-hash permutation control. This is not authorization for broad hashing or public changes. ContextBlock equality and Debug both use id; no closure-name hash defect is asserted.

## Execution gate

Update each exact owning L0 before tests/source. First establish baseline: semantic RED only for independently measured omissions; behavior-preserving controls are not falsely labeled RED. Root owns the copied-workspace enum extension baseline and exact-site E0004 witnesses for all five decisions, including both equality axes. Preserve source/origin, rule/guard/style restoration, IDs, sequence boundaries and callback nonexecution. No generic visitor, public signature, entity, pipeline phase, configuration, suppression, Nucleus or performance change is proposed. Any expansion requires root classification and current owner review. No code/test/spec changes have been performed.
