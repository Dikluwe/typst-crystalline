# L0 — par line callbacks
Hash do Código: 50f3503f

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:3921c80230cfcc66a807429fbf8bc07ee090bae4988c351b6524023038bab84b

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/par_line_callbacks.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/par_line_callbacks.rs:18`, `01_core/src/compiler/layout/par_line_callbacks.rs:27`, `01_core/src/compiler/layout/par_line_callbacks.rs:34`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. The former reference `9f2b6e8715237cb086899a42873660fe744622e8` and older vanilla pins are historical snapshots. They do not certify the current ratified reference `bd5ebec285505889e8ff1d4f68804ab147a699f3` without a new measurement.

Representation evidence is preserved in `lab/results/construction/diagnostics/clippy-enums-2026-10-16/before.json.gz`: source/L0, global hashes, HEAD, uncommitted tree, UTC and diffstat; `callers.txt` records workspace uses. `probe-details.json` and `measurements.json` preserve rustc 1.92/x86_64, commands/binary/rlib, and sizes/allocations of isolated candidates before implementation. The private enum faithfully mirrors source; public types are measured directly from the rlib. The unsafe observer exists only in the diagnostic executable, never productive layers. This measures Rust representation, not language equality, upstream intent or global gain.

Productive-use search shows public LayoutPassOutcome returned and consumed through match in the coordinator, not collected. The Complete-only probe with an empty document and black_box measures inline without allocation versus one per Complete for the Box candidate: 20,000 additional allocations per sample. The enum is 440 bytes versus the candidate's 48; the 440-byte document still exists separately in the box. Reducing the discriminated container therefore does not reduce total complete-result memory. This probe is neither a real layout benchmark nor evidence about every workload.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/core/compiler/layout/mod.md`; the contract below defines this unit's operations, not ownership of that hub.

Own the public request/resolution types, sealed par.line callback store, attempt transcript and observational matching. Match by function, line number and collapsed style-chain observation; consume each matching resolution at most once. Missing resolution records the request and remains Pending rather than fabricating Content. Preserve existing reexports and signatures through the hub, without introducing I/O, Engine or Func execution.

Own the final region-local maximum-number-width fixup over the existing hub buffer, keeping item indices and left/right horizontal alignment consistent. Physical baseline/page/counter selection belongs to `00_nucleo/prompts/core/compiler/layout/line_commit.md`; segmented candidate consolidation belongs to the column owner. Store/transcript lifetime is coordinated by `00_nucleo/prompts/core/compiler/layout/mod.md`; this module does not decide rollback, regional progress or the document fixpoint.

Preserve public `LayoutPassOutcome::Pending(LayoutCallbackRequests)` and `Complete(PagedDocument)` directly by value. This single-result boundary remains direct, without a new Complete allocation or indirection. Use only `#[expect(clippy::large_enum_variant, reason="E002: resultado único de layout por valor; Box adiciona alocação ao Complete, sem reduzir o documento")]` on the enum. Do not raise thresholds or suppress a module/impl/crate. Do not extend expectations to Tag/Transformation or the private request.

The representation change follows continuous flow under ADR-0127: only crate-private representation/constructors and nominal metadata may change; external outcome/entity contracts remain. L0 precedes code and resealing is restricted. Tag/Transformation remain measured; this package authorizes no public change to them.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

The private-size RED→GREEN gate and Clippy large_enum_variant must establish three selected spans before the change, then resolve one through code and two through satisfied expectations. Preserve fixtures/expected outputs and existing outline controls (including the eight historical P1405 cases), context/replay, callbacks and pipeline. Validate the full workspace/build/Clippy/expectations/Rustdoc/fmt/lineage with no new Tekt warnings, under 12 GiB, zero swap and one job.
