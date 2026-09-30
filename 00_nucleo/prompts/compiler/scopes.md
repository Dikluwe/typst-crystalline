# L0 — scopes

Hash do Código: 85a36a45

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/scopes.rs`

## Consumer evidence

`01_core/src/compiler/scopes.rs:19` locates `pub struct Scopes<'a>`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`00_nucleo/diagnosticos/sanitization-remaining-compiler-history.md`.

## Scope stack and capture

Preserve `Scopes<'a>` with public `top: Scope`, `scopes: Vec<Scope>`,
`captured: Option<Arc<Scope>>`, `captured_by: Option<Capturer>` and
`base: Option<&'a Library>`. Capture reason is present with the captured
scope and affects mutation diagnostics, not reading. Arc is local RAM ownership,
not I/O or mutable global state.

`new(base)` creates an empty top and no capture.
`with_parent(parent, capturer) -> Scopes<'static>` shares the captured scope,
starts with empty top/stack and has no base reference.
`get` searches top, previous scopes newest first, captured, then base.global.
`get_local` uses the same order but stops before base; `has_global` reports
membership in base.global for caller diagnostics, not mathematical resolution.

`snapshot` includes base.global, then captured, previous scopes oldest first
and top; nearer bindings overwrite earlier values. This preserves standard
library access when a capture is later called without a base reference.
`iter_all` traverses captured, previous scopes oldest first, then top; it
does not include base, and is not a deduplicated replacement for snapshot.

Enter pushes the old top and starts an empty scope. Exit returns the departed
scope and restores the previous one (or an empty scope if none remains).
Push_scope installs a populated top while retaining the old one.
Define writes only top. `get_mut` searches only top and previous scopes;
captured and base are read-only through this interface. A local shadow remains
mutable. Preserve these methods and their signatures.

`captured_by(name)` reports the definition-time capturer only when a nearer
mutable binding does not shadow the capture. `is_constant(name)` is true
only for names reachable solely through base. These classifications allow the
assignment owner `00_nucleo/prompts/compiler/eval/bindings/access.md` to
distinguish captured-variable, constant and unknown-variable diagnostics,
in that priority order. This owner does not duplicate assignment execution
or error construction. Missing names are not constants.

Acceptance covers lexical shadowing, enter/exit, captured sharing, base access,
snapshot precedence, local-only lookup and mutation classifications without
requiring the upstream Binding representation.
