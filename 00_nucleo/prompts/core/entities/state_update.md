# Prompt L0 — `entities/state_update`
Hash do Código: 28a44332

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/state_update.rs`

## Evidence before specification

Historical vanilla snapshot `e0e8ca4d`, `introspection/state.rs:336–349`, documents `state("key").update(f => …)`: the function receives the previous value and returns the next. It recommends this form for updates depending on prior state to reduce layout iterations. The old claim that literal Set covers 80% had no corpus measurement and is withdrawn.

`lab/results/construction/diagnostics/tekt-enums-2026-10-16/before.json.gz` records HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree/UTC/source/L0/config/status/diffstat for eleven V16 warnings and one note across seven owners, including this equality at `state_update.rs:58` and measured nonemitting fallbacks. Enum evolution is mechanics; observable results/fields/identity/errors must remain. No upstream intent or AI benefit is inferred.

`compiler/introspect/from_tags/state_updates.rs:9–47` has an Engine-backed Func postpass; `introspect/entrypoints.rs:29–35` distinguishes the legacy path. Later `lab/results/construction/diagnostics/lote-a1-callbacks-reflexao-seletores-2026-10-17/{state-counterproof,state-extended}.json.gz` records HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, tree/UTC/diffstat and current language pin `bd5ebec285505889e8ff1d4f68804ab147a699f3`. The approved state-reader-runtime review transfers automatic application to reading. The original silent Func stub is historical, not the current global carrier contract.

## Pure data contract

```rust
#[derive(Debug, Clone)]
pub enum StateUpdate {
    Set(Box<Value>),
    Func(Func),
}
impl PartialEq for StateUpdate { /* Set by Value; Func by Arc identity */ }
impl Eq for StateUpdate {}
impl std::hash::Hash for StateUpdate { /* hash Debug formatting */ }
```

Box breaks the Content→Value→Content sizing cycle. Set denotes replacement with a value; Func denotes a transformation of the previous value. This owner never executes callbacks. Runtime ownership belongs to `00_nucleo/prompts/core/compiler/introspect/from_tags/state_updates.md` and the approved reader runtime; the legacy path without Engine remains distinct. Consumers include Content::StateUpdate, ElementPayload::StateUpdate and StateRegistry::apply_update.

Preserve manual Eq marker despite Value's f64 limitations, Debug-based deterministic Hash and manual PartialEq: Set/Set compares Value, Func/Func uses Arc::ptr_eq, cross pairs explicitly return false. Do not replace Arc identity with Func::eq: distinct native Arcs with the same name can compare equal as Func and remain distinct as StateUpdate. Rust identity is not asserted to equal language equality. The old reexport integration request does not authorize a new facade change during this cleanup.

## Acceptance and explicit exhaustiveness

Require equal/different Set controls, deterministic repeated hashing, same-name distinct-native identity controls, and symmetric mixed pairs before implementation. Keep enum/fields/hash/marker/callback execution and all bodies/order/guards. No helper may conceal a catchall; nominal patterns must fail compilation when a variant is added.

For the recorded V16 repair, obtain functional local tests and scoped lint RED first. In a disposable workspace only, add sentinel variants to Color, ColorSpace, MathStyleKind, FuncRepr and StateUpdate. Run `cargo check --offline --locked -p typst-core` with one job and shared artifact cache; record sources/hashes/argv/UTC/diagnostics. Each of 21 selected matches must change from absence of its own E0004 to E0004 at that match, regardless of other failures. Unsentinelled compile and real behavior tests approve production; the negative probe does not prove parity. Never edit productive definitions during probe commands.

This is ADR-0127 continuous internal maintenance: no enum/field/trait/signature/default/dependency/compatibility/phase change or ownership move. Restrict direct/reverse resealing. Revalidate build, workspace/all-target Clippy with `-D warnings -D unfulfilled_lint_expectations`, fmt, full workspace tests, Rustdoc `-D warnings` and four HTML indices, strict V1/V5/V15/V26 and reverse lineage, no-new-warning SARIF and diff checks. Heavy jobs are sequential under MemoryMax 12 GiB, swap 0 and one job. Do not relax historical fixtures/tolerances. External V16 debts, including nonemitting T027/T158/T160, remain open.

## Historical record

`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/state_update.md` preserves the original evidence and chronology.
