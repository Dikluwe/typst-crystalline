# P1340 — passive L1 bindings, engineering handoff R1

Implementer `/root/p1312_review`; not contract author, adversary or verifier. Full protocol, executed without attestation of technical isolation. Implementation began only after GO `p1340-verifier-seal-r3.json` (`82ceda880adef9a6ba2f0330f71badcfea084af64168510501cb4c4a939ea9f4`), under `p1340-test-binding-authority.json` and `p1340-test-binding-delegation.json`. No protected oracle, semantic L0, registry source or productive API was edited.

## Available only with cfg(p1339_observation)

At `compiler::eval`:

- `EvalContext::p1340_observation_requests(&self) -> serde_json::Value`: every actual registered request in original order.
- `EvalContext::p1340_observation_replays(&self) -> serde_json::Value`: append-only actual replay transcript; no filtering of full folds or early nonSame witness. L3 takes the suffix since its actual call boundary and selects the recorded phase.
- `p1340_observation_func(&Func) -> serde_json::Value`: actual Func allocation, variant, closure capture allocation, diagnostic and body spans; With retains its own identity and additionally projects target/arguments.
- `p1340_observation_value(&Value) -> serde_json::Value`: typed actual values, including Location(u128 decimal), Float(IEEE bits), Func, Length, selector/counter/state/args and integrated content carriers. Unintegrated values explicitly report `binding_gap`; not repr or a made-up value.
- `p1340_observation_diagnostics(&[SourceDiagnostic]) -> serde_json::Value`: complete ordered severity/span/message/hints/trace including nullable Call payload.
- `p1340_observation_counter_events(&CounterRegistry) -> serde_json::Value`: all actual ordered actions, including automatic key:null, complete Set/Step/Func action and Location. This registry does not store span or producer generation; L3 joins those through actual tag provenance.

`StyleChain::p1339_observation_identity` is cross-crate public only under the same cfg. The ordinary API is unchanged.

Request/replay rows contain `request_id`, `operation`, `typed_arguments`, `span:{raw_span,file_id}`, `chain_id`, `chain_size_pt_bits`, `context_location`, `in_context`, `target`, `features`, `current_file`, and complete `result`. Raw spans and file IDs are decimal strings (file optional); L3 resolves offsets and source hashes against its retained real Source. It also supplies actual producer/capture/World/metrics resources from its real boundary, not from fixture expectations.

Replay additionally contains `phase` and `relation`. The validation relation is the **single actual private comparison used by the branch**, recorded before testing its discriminant. Ordinary cfg uses the exact original expression. Diagnostic-history replays have relation:null: no comparison is invented for operations that did not make that per-request validation comparison. Effective replay context and chain are captured directly before real dispatch and retained; recording never substitutes original capture data for an effective resource.

Identity lifetime is a binding precondition: L3 agreed to retain prior execution boxes/Arcs and real Func/chain resources until transcript completion so allocation addresses cannot be reused between retired generations.

## Current evidence

`p1340-implementation-l1-bindings-focal-r1.json`, SHA-256 `9e7d0e72a2713e0bd2516ce74e901b7bc131b92b649d907d0a3198e173c628d9`, records the actual command, environment delta, full compressed streams, dirty-tree HEAD/stat and 433 core/include/gate input pins before/after. UTC 2026-09-10T14:12:00.682Z–14:14:51.408Z; exit 0, one Rust test passed, no failures; inputs unchanged. The test uses actual state reads and actual validation to establish Same full replay and Different fail-fast prefix, retained original results/request identities, no sink publication, Location u128 and raw span/trace u64 preservation.

Actual executable SHA collected **after** cargo test: `77690addd3d577a4fc3373bbf25598ccad542f9ed5352e920eca6529952e7de7`. This is not a before-runtime binary pin. External toolchain/cache contents are not attested. A preliminary automatic-approval timeout did not execute the command; one allowed retry started the recorded run. No fabricated timestamp or result is attached to the unexecuted attempt.

`p1340-implementation-l1-bindings-inverse-check.cjs` removes exactly the documented cfg-only additions and comparison wrapper. It reconstructs byte-identical sealed source hashes: eval `8cac57e3d1d1f1401a8e0d63ede40d4fc0772d1df60134478a73f37d37b47e6a`, StyleChain `3f77f12991d4038b3c695606aec5d18934869c3a17a23bbe45e3c60f594287ed`. This static restriction witness passed alongside the focal run; it is not a substitute for the independent ordinary/instrumented path audit.

## Explicit open items

- FunctionInvoked identity is absent from the older callback log (only phase/category existed). A real callsite hook with Func and origin needs separately authorized owner integration; these ports do not fabricate it from requests. Root and L3 implementer were notified.
- R1 generic Value::Dict projection is an ordered JSON object, potentially ambiguous with tagged scalar objects. A tagged ordered-pair Dict projection was proposed to root after the focal run; not yet applied in this receipt. This prevents claiming complete lossless generic projection until resolved.
- Unsupported content/value variants remain explicit binding gaps. Any mandatory actual case reaching one blocks acceptance rather than being excluded.
- Full lifecycle, terminal/public matrices, invocation completeness and final F remain with the independent integration/verifier. No overall PASS, commit, or independent verdict is claimed here.

Sources of this measured R1: eval `b4146639cfc5f34f8f0cbf158b13dd4c79e1922bc1c385e3cff6e6dffb11ce5b`; StyleChain `0ded456e4065c28536de9dd0e29b800ec88043f2f610df3ab19c81aa7dd77026`.
