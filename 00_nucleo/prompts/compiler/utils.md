# L0 — utils
Hash do Código: b6143520


**Layer:** L1
**Exclusive consumer:** `01_core/src/utils.rs`

## Consumer evidence

`01_core/src/utils.rs:24` locates `pub(crate) struct DeferHandle<'a, T, F: FnOnce(&mut T)>`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`00_nucleo/diagnosticos/sanitization-remaining-compiler-history.md`.

## Internal utilities

Own only `01_core/src/utils.rs`, not compiler phase behavior.
`timing_scope!($name:expr)` is a no-op instrumentation marker expanding to
unit without evaluating its name expression or performing I/O.
It is module-local; do not promise an exported telemetry API or a scheduled
reactivation.

`pub(crate) defer<T, F: FnOnce(&mut T)>(thing: &mut T, deferred: F)`
returns `DeferHandle<'_, T, F>`. Its guard borrows the value, implements
`Deref<Target=T>` and `DerefMut`, and invokes the closure exactly once on
drop by taking the optional closure. Mutations through the guard are visible
to that closure. Preserve normal RAII lifetime/unwind behavior; do not add
global state or external dependencies.

The inlined defer implementation retains its Apache-2.0 provenance from
`typst_utils::defer` (ADR-0008); the no-op marker follows ADR-0006.
Acceptance covers drop effects, mutable access and unit expansion, not timing.
