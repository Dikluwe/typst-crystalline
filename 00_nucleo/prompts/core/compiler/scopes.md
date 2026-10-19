# L0 — scopes
Hash do Código: 49b45193


**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/scopes.rs`

## Consumer evidence

`01_core/src/compiler/scopes.rs:19` locates `pub struct Scopes<'a>`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`lab/results/construction/diagnostics/sanitization-remaining-compiler-history.md`.

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
assignment owner `00_nucleo/prompts/core/compiler/eval/bindings/access.md` to
distinguish captured-variable, constant and unknown-variable diagnostics,
in that priority order. This owner does not duplicate assignment execution
or error construction. Missing names are not constants.

Acceptance covers lexical shadowing, enter/exit, captured sharing, base access,
snapshot precedence, local-only lookup and mutation classifications without
requiring the upstream Binding representation.


## Snapshot prefix copy

### Evidence before decision

`lab/results/construction/diagnostics/snapshot-scopes-prioridade7-2026-10-08/before.json.gz`
retains HEAD/tree/UTC/texts and release 17e72149. scopes.rs:84–103 reinserts every
prefix/overlay binding by name. closures.rs:196 and expression_dispatch.rs:505
capture at definition, not each call. probes.json.gz/probe-summary.json measure
100 captures in closures100: 20,300 input bindings, 20,100 from the base, always
size 201. A closure called 1,000 times captures once. context100 captures 100
times; nested has base and captured in different calls. Debug counts and release
calls agree for common controls. GDB timing is intrusive and does not establish
normal CLI time share.

prototype.json uses real Scope/Scopes and synthetic integer data; its external
helper starts by cloning the whole base (or captured without base). Five pairs
per configuration, 2,000 snapshots per sample: base201 34.09→18.94 ms,
captured201 37.47→20.79 ms, both201 37.50→20.93 ms. Order/values were compared
before timing. Do not treat these integers as the full stdlib. Copy/reinsertion
is mechanics; values, precedence, order and isolation are existing
semantics/morphology (ADR-0107/0108). These are historical measurements with
the recorded snapshot qualifications, not current-pin parity.

### Exclusive contract

Start snapshot from a clone of base.global when base exists, otherwise a clone
of captured when captured exists, otherwise Scope::new. Overlay captured only
when it was not already the prefix. Then apply previous scopes oldest→newest
and top. Preserve the first position of each name, nearer replacement, stdlib
availability when called and capture isolation. Do not filter used/free names,
share later mutations, omit stdlib or turn capture into lookup of post-definition
state.

Do not change Scope representation or new/define/get/get_mut/clone, Capturer,
public Scopes fields/signatures, phases, defaults or diagnostics. Loops without
capture gain no extra Arc/allocation. A private helper may take a static FnOnce
to clone the prefix at most once: pure RAM construction, never a language
callback. No I/O, unsafe code, globals or new crate. This reversible internal
correction follows ADR-0127 continuous flow and exclusive 1:1 ownership.

### Acceptance and refutation

RED→GREEN: a private witness counts one initial prefix copy instead of individual
reinsertion, covering base+captured and captured without base. Functional
controls preserve overlays, first position, isolated mutation and no-prefix
fallback. The witness measures internal mechanics, not language pointer equality.
Compare frozen and candidate release CLI on repeated creation, repeated calls,
contexts, loops without capture and the minimal case; preserve text/boxes/
diagnostics and qualify pinned vanilla before declaring parity. Retain only with
repeatable capture-workload benefit and no meaningful control regression.
Validate dev/release builds, pertinent core/infra suites, lint and this owner's
restricted resealing. The original numbered priority is historical provenance,
not a current organization criterion.
