# L0 — compiler/eval/call_dispatch/trace_plugin
Hash do Código: 1ff5196a

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:059eb35863a413593c98fb7195a521e3d019dadf461433e9c42422218be89065

## Ownership and evidence

Layer L1. Exclusive productive consumer: `01_core/src/compiler/eval/call_dispatch/trace_plugin.rs`. At HEAD `d5c563dfab39f789e8e2f72c9a9364f5a761e3ca` with a clean tree, `01_core/src/compiler/eval/call_dispatch.rs:1215-1304` contains the extracted behavior. SHA-256, time and initial state are recorded in `lab/results/construction/diagnostics/atomizacao-call-dispatch-2026-09-30-before.json.gz`.

## Contract

Add causal traces and merge with arguments; validate and execute plugin calls. Preserve public signatures, diagnostics, interception priorities, traces and argument semantics from `00_nucleo/prompts/core/compiler/eval/call_dispatch.md`. Extraction remains in L1 with static dispatch, without I/O or mutable global state.

## Acceptance

Preserve the L1 suite, call_dispatch tests and `crystalline-lint .` with correct ownership and hashes. Do not infer performance gains.


## Ratified transparent source-origin trace boundary

Prior evidence is
`lab/results/construction/diagnostics/table-origens-d08b3c2-2026-10-07/baseline.json.gz`,
with HEAD/working tree/UTC/sources/argv/binaries at historical pin
`9f2b6e8715237cb086899a42873660fe744622e8`. This unit has the corresponding
transparent operation in existing ShowRecipe/Located/Realized branches; Source
must not leak as an observable element. Human approval `confirmo` was given on
2026-10-07. The final approval/execution record is
`lab/history/construction/records/00_nucleo/materialization/table-origens-d08b3c2-2026-10-07.md`;
the nested contract.md retained there is the pre-approval candidate.

Late Table realization traces use the FuncCall parent of Args identified by
Span in the causal Source CST, never names/text/equality search. Reuse trace_call's
containment rule; do not fabricate traces without a resolvable anchor. The
helper is crate-private and adds no external API or phase move.

Preserve exclusive ownership and static dispatch without I/O, new
signature/trait/flag or eval↔layout phase change. C2/C/macro tests and
collections, builds and lineage are required before certification.

## Selectorless lexical-call trace reuse (2026-10-29)

Before deciding, the pinned `selectorless-capture-before` receipt under
`lab/results/construction/diagnostics/readiness-fixes-2026-10-28/` measures
alias native/closure errors at their real definition origins, with an additional
call trace at the actual show declaration. The current helper is at
trace_plugin.rs:24–63 and already owns containment-based trace addition.
Allow this existing helper within compiler/eval ancestry for the private whole
operator, retaining its exact signature/body/name and range policy. No new public
API or duplicated trace algorithm. The caller supplies actual function/error and
declaration spans; detached/unresolvable origins keep the current behavior.
Existing inferred closure-name debt is not certified by reusing this helper.
Require real source/function-parameter/call-trace controls, existing call tests,
valid lineage and unchanged final gates. Internal ADR-0127 delegation only.
