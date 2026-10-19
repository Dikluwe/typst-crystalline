# L0 — compiler/stdlib/registry/contextual
Hash do Código: ca2c18a3

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/stdlib/registry/contextual.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Contract

Register metadata, state, counter, context, state/counter operations, query, selector, here, target and locate. State/counter/selector remain Type values; native ABIs remain unchanged. Do not decide targets or execute contextual reads.

Preserve the owned public names, Value kinds, native bindings, children and
aliases. Do not implement Args validation, casts, export or lookup behavior;
those operations are outside this registration consumer.

## Acceptance

Verify the owned behavior through local tests and its existing integrations.
Preserve public paths and observable results. Ownership, nucleus integrity and
lineage checks must remain valid; no unrelated consumer is authorized by this prompt.


## Ratified State reader registration

### Evidence before decision

HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, uncommitted working tree;
before.json.gz and final-freeze.json.gz under
`lab/results/construction/diagnostics/lote-a1-callbacks-reflexao-seletores-2026-10-17`
preserve UTC/status/diffstat/sources. state-counterproof.json.gz records bilateral
RED: readers initialized to 1/10 with the same key require 2/11 after +1; an
unread callback never executes; reused updates execute per occurrence. Pin:
`bd5ebec285505889e8ff1d4f68804ab147a699f3`; upstream
introspection/state.rs:226–245,419–455,477–520. The reader's init starts the
sequence; document order, not the Location integer, orders it. The measured
method treats Func as Set and eager postprocessing loses that semantics. These
are language values/errors, not Rust equality or a mandated algorithm.
The earlier State proposal was withdrawn; the owner ratified the object-reader
runtime gate in that session. Review:
`lab/history/construction/records/00_nucleo/materialization/state-reader-runtime-2026-10-17/review.md`,
SHA-256 `6d6a176b8e27cc53c38fe6650c03c217448059f4e7287fd8848fe54e80247fa4`.

### Exclusive responsibility and partial supersession

Register only native_state_final/native_state_at through Func::native_with_engine,
with the same public name and Value::Func. Other native ABIs/bindings remain.
Execute no reads in composition. NativeFuncWithEngine already exists; this
ratified clause supersedes ABI preservation only for these two registrations
and the historical State read phase.

### Acceptance and boundaries

The object-reader gate is ratified for its five signatures and read phase.
L0 first, functional RED before code; GREEN against the stated pin for language
and independent controls for Tekt extensions. Do not broaden other public
contracts. Validate build/workspace/doctests, strict Clippy/Rustdoc, fmt,
V1/V5/V15/V26 and reverse lineage; no increase in V16. Limits:
MemoryMax 12 GiB/High 8 GiB/swap0/jobs1; heavy tasks sequentially. Measure consumption
without inferring global benefit from representation alone.
