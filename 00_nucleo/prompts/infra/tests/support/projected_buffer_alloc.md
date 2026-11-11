# L0 — Projected-buffer test allocation recorder
Hash do Código: 18aae65e

**Layer:** L3 (test support only)
**Exclusive consumer:** `03_infra/tests/support/projected_buffer_alloc.rs`

## Evidence before the support contract

The independent C23 lab tests recover the output Vec's realloc lineage and inject
reservation failure with fixed records. Their executable owner is
`00_nucleo/prompts/infra/tests/projected_buffer_reserve.md`; this support is never
imported into the productive library or its ordinary test allocator.

## Contract

Forward GlobalAlloc arguments/results unchanged to System except an explicitly
selected allocation failure for the tracing test thread. Recording itself must
not allocate. Use bounded fixed storage and report overflow; scope failure
injection to a TLS owner so libtest/other workers continue using System normally.
Track alloc/realloc/dealloc addresses and requested sizes. Recover the final
buffer's realloc chain backwards, stopping at the initial allocation of that
lifetime, even when old freed addresses were reused. Expose requested-size maximum
and whether failure/overflow occurred. Do not interpret requests as copies or RSS.

Trace windows must be serialized by the caller or a support guard, with cleanup
on normal completion and no unsafe memory inspection. SAFETY documentation must
explain System delegation and no allocated-memory access. No new dependencies,
product globals, process receipts, benchmarks or scores belong in this module.
