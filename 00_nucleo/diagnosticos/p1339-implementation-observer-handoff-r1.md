# P1339 — observer implementation handoff, paused

Role: implementer of `01_core/src/compiler/eval/mod.rs`, not independent verifier. No verdict, commit, final phase-F credit, or technical-isolation attestation. Paused on parent instruction after the independent verifier identified an Array-constructor scope conflict in sealed public cases. No protected expectations were changed to accommodate product behavior.

## Current bounded implementation

- Per-EvalContext RefCell recording starts before demand selection; selection is restricted to filtered Element demand. Typed requests preserve complete counter folds separately from resolution/projection, modern versus legacy state/counter policies, query, locate, here and location accessors.
- Replay uses the actual read owners with a fresh isolated EvalContext and transactional Engine/Sink. Each retained request captures context, location, target/features, source file, styles, show rules and active guards. No ContextBlock body replay was added here.
- `replace_context_read_rules(&self, rules: Arc<[ShowRule]>, guards: Vec<RuleId>) -> (Arc<[ShowRule]>, Vec<RuleId>)` is integrated around eval_expr; parent reported matching apply_func integration. Read replay now takes rules/guards from that retained request, not the final ambient Engine.
- Closed private Same/Different/Unproven relation includes exhaustive Value/Content dispatch and recursive fields, IEEE bit distinctions, ordered dictionaries, external leaves and opaque Dynamic rejection. No global Eq change, generic Debug/hash proof or Arc shortcut for recursive containers.
- Three approved public observation APIs and cfg-only passive operation/binding telemetry are present. No pipeline productive integration or guessed external telemetry API was added by this owner.

## Evidence and limitations

The ordinary local test run `p1339-implementation-observer-local-r4.json` (SHA-256 `d2479932ce50e78da344886dac018e09a36ababe27374104d877d414b97b7d36`) passed 3 tests before the final rules/guards capture. The parent separately reports a later ordinary core/infra check passing. This is not evidence that the full protected matrix passes.

The bounded instrumented compile after capture and accepted style supersession is preserved losslessly in `p1339-implementation-observer-instrumented-compile-r2.json`, SHA-256 `3c4b1e39a5f9ee3328fc06b04f44fd1c419eb8b6b3e85297086df2d334dc44e0`. Exact argv, environment delta, dirty-tree HEAD/stat, productive source pins, UTC and compressed stderr are in that receipt. It ran 2026-09-10T06:26:31.903Z to 06:26:50.445Z, exited 101, and productive source pins were unchanged across the command.

Its only five errors are E0599 in protected `p1339-mutant-closed-state-style-harness-r2.rs`, lines 170, 194, 196, 265 and 267: `TrackedMut::reborrow_mut` is an associated function, not a method. The implementer did not patch this protected file. Further mechanical succession must be owned and accepted separately. No protected runtime matrix was executed after this compile failure.

The original wrapper and both original and successor style harnesses remain untouched. The candidate-only thin wrapper is `p1339-implementation-observer-core-wrapper-r2.rs`, SHA-256 `ec88b9cbcc48bd9a2e7a7e6f7ff27933323cd732022f9769f047f965cdea4b90`; its exact delta from the original wrapper is solely the style include basename `style-harness.rs` to `style-harness-r2.rs`. A superfluous trailing blank line from initial copying was removed while the compile command was active; it has no semantic effect, but the runner's productive-source pin set does not include this diagnostic wrapper, so it must not be represented as an immutable-all-inputs runtime receipt.

## Open implementation/audit items, not completion claims

- Independent audit must decide whether same-Arc Module/Closure identity requires transitive screening of captured opaque interior-mutable leaves. Current relation treats retained Module/Closure identity as Same; direct Dynamic remains Unproven. This concern was identified during implementation and sent to the parent; no unsupported completeness claim is made.
- Separate full-fold and projected requests can replay the same fold through both actual owner paths. Callback events record actual executions; there is no fake event suppression. Replay work/count semantics need the frozen matrix, not an assumed count.
- Instrumented runtime bindings and complete private fields coverage have not received final independent verification. Pipeline APIs are implemented here but parent pipeline integration remains pending.
- Final lineage metadata resealing remains the parent responsibility after the productive state is frozen. No L0 semantics were edited by this owner.

Productive file SHA-256 at handoff: `564eeac79a3f97eefc04740f441785034f7c08a27a477e7f51c7a0dd2e2cf6fc` (`01_core/src/compiler/eval/mod.rs`). No further product edits are planned during the scope pause.
