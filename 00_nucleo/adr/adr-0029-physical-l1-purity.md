# ADR-0029 — Layer ownership, physical purity and safe memory use

**Status:** ACCEPTED
**Date:** 2026-03-29
**Decision owners:** Typst Crystalline maintainers
**Consolidated:** 2026-11-24
**Supersedes:** ADR-0028

## Evidence

The original decision corrected a typographic representation chosen during migration. A length can retain both its absolute and font-relative components in RAM even before a style chain is available to resolve them. Later work showed a separate distinction. Sharing an immutable metric handle in one compilation and retaining a completed product across edits require different validity proofs. Reference counting alone establishes neither. [Review of the owning contracts](../diagnostics/contract-scope-review-2026-11-13.md)

The dependency whitelist records permitted items per crate. Its entry for `time` includes `now_utc`, while the actual clock read belongs to the L3 World implementation. The entry is a configuration discrepancy to test and reconcile. It confers no permission to read the clock inside L1. [Whitelist](../../crystalline.toml) · [World adapter](../../03_infra/src/world/world_impl.rs)

## Layer responsibilities

| Layer | Responsibility and permitted dependencies |
|---|---|
| L0 | An owning prompt specifies one productive consumer. Shared invariants and lineage follow ADR-0129. |
| L1 | Own pure domain values, algorithms and contracts for injected capabilities. Use pure standard-library operations and specifically authorized external items. |
| L2 | Own CLI syntax, typed intents and human presentation. Depend on L1. The shell resolves its supported argument, environment and terminal options. |
| L3 | Implement L1 contracts for filesystem, packages, fonts, clocks and other external resources. Pure helpers belonging to an adapter may stay with it. Depend on L1. |
| L4 | Compose L1, L2 and L3, supply concrete adapters, and deliver output. Keep compiler and presentation rules in their owning layers. |
| Laboratory | Keep the pinned upstream reference, experiments and benchmark infrastructure in quarantine. L1–L4 never import laboratory code. |

In L1, prohibit disk and network access, system clocks, process environment, OS entropy, production console I/O and mutable global state, including a mutable global hidden behind a lock or an atomic. L1 can consume an observation through an explicit contract supplied by the caller. A calendar operation on a supplied date belongs in the pure domain. Obtaining today's date belongs to the World implementation. [00_nucleo/prompts/core/contracts/world.md](../prompts/core/contracts/world.md)

## Domain representation and RAM

Retain the language capabilities of typographic values, including compound dimensions, relative quantities, angles, axes, alignment and paint. Inspect the reference type, effective values and operators before choosing a representation. An unresolved contextual operation is insufficient reason to drop a component of its data. Record any deliberate simplification, its observable cost and its concrete review condition in an explicit ADR. The original typographic inventory excluded generic containers, AST elements, DSL values and internal layout types (`Frame`, `FrameItem`, `Region`, `Fragment`, `Paper`, `Margin`). Use each type's own contract and ADR-0107 when evaluating its representation.

Owned allocation, shared immutable storage, copy-on-write and processing already supplied bytes are permitted in L1. Monitor repeated deep copies on measured hot paths. Select `Arc`, `EcoString`, `EcoVec`, or independent owned storage according to lifetime, mutation isolation and full-path cost, including retained memory. A cache has a separate obligation to establish complete inputs, validity, invalidation and scope. The default hasher for performance-sensitive internal lookups is `rustc_hash`; ordinary standard maps remain suitable for small or clearer cases. Preserve ordered iteration wherever the language exposes it. Source hashing and the cost of copying content sequences remain with their owners. [00_nucleo/prompts/core/entities/source.md](../prompts/core/entities/source.md) · [00_nucleo/prompts/core/entities/content.md](../prompts/core/entities/content.md)

## External items

`[l1_allowed_external.<crate>]` in `crystalline.toml` is the item inventory. Review each prospective use against physical purity, its owning contract and imports. A Cargo declaration alone grants no L1 use. An allowed type does not expose every operation in its crate or authorize an external type in an unrelated public interface. Keep the admission rationale and observable constraints recoverable.

| Use | Scope inherited from the decisions |
|---|---|
| Unicode and math classification | Use the listed identifier, script, segmentation and math-class items. Retain the language's class overrides and source boundaries. |
| Strings, dictionaries and memory | `Value::Str` uses `EcoString` for cheap cloning. `EcoVec<T>` requires an admissible element. Other ecow containers need their own admission. Ordered dictionaries preserve insertion order with the admitted `indexmap` and hasher. |
| Calendar, numbers and patterns | Admit pure calendar arithmetic, the fixed-precision `rust_decimal` substrate for Decimal, and the declared regex operations through the L1 domain wrapper. The wrapper preserves pattern identity without leaking the compiled external type. The input/output types and diagnostics belong to the corresponding owners. |
| Hyphenation and data loading | Use embedded hyphenation patterns in L1. Use the listed items of `serde_json`, `saphyr`, `toml`, `ciborium`, `roxmltree` and `csv` for their respective input formats. YAML maps its parser tree explicitly into language values. Keep each format’s conversion and limitations in its owner. The external reader supplies input. Bibliographic parsing may use the admitted Hayagriva items in L1. |
| Fonts and tracking | The font catalog in L1 holds domain metadata. System discovery, opening faces and Rustybuzz shaping belong to L3. Allowed comemo operations track explicit reads or writes under the owning contract. A tracking facility grants no mutable global domain store. |

Preserve the whitelist and the responsibilities in the owning specifications when altering an admission. [00_nucleo/prompts/core/entities/value.md](../prompts/core/entities/value.md) · [00_nucleo/prompts/core/compiler/stdlib/loading.md](../prompts/core/compiler/stdlib/loading.md) · [00_nucleo/prompts/infra/fontdb.md](../prompts/infra/fontdb.md)

## Unsafe in L1

The goal is zero unsafe code in L1. The temporary exceptions recorded by ADR-0032 apply to reviewed inlined code or an identified refactor with a documented exit plan. A permanent exception requires a site-specific architectural decision, a demonstrated safety invariant, the safe alternative and a reproducible benchmark quantifying the unacceptable regression. The prohibitions on `Box::leak`, arbitrary transmutation between unrelated types, zero-initializing non-`Copy` types and raw-pointer `as` casts between pointer types also apply in L3.

Document the safety invariant at each unsafe site in a `SAFETY` comment. The permanent scanner exception covers the five existing `str::get_unchecked` slices in `before`, `after`, `from`, `to` and `get`. The cursor stays inside the borrowed string at a UTF-8 scalar boundary. External endpoints are snapped in bounds and ordered against the cursor or each other. Sealed matching advances only by a valid in-bounds UTF-8 length. Changes to cursor movement, snapping, the pattern contract or any of the five slices require rechecking this proof. The exception grants no unchecked indexing for arbitrary input or other consumers. [00_nucleo/prompts/core/compiler/lexer/scanner.md](../prompts/core/compiler/lexer/scanner.md)

The historical ADR-0116 comparison used a safe-slicing candidate, Criterion 0.5 and five lexer inputs covering a short file, prose, math, code and multibyte text. The original table records elapsed-time regressions from 6.4% to 58.3% and per-byte regressions from 8.5% to 54.1%. These are historical measurements. For a new decision, use equivalent inputs and builds, measure lexer time per byte in three sequential runs, check the original reproducibility criterion of less than 3% variation between median and mean, and bind the result to the tested revisions. Keep the benchmark dependencies in the laboratory. [Scanner benchmark](../../lab/benches/scanner_bench.rs)

## Verification and history

Verify layer topology, I/O, global state, whitelist use and ownership through the applicable architecture checks. Exercise the actual language operation whenever a representation or dependency changes. For the scanner, exercise bounded and reversed ranges, multibyte navigation, matching and failure alongside the benchmark. The four approval boundaries in ADR-0127 apply to public contracts, defaults, phases and compatibility.

ADR-0001 supplies migration topology and quarantine. ADR-0030 supplies RAM ownership and cost, and ADR-0032/0115/0116 supply unsafe policy, methodology and the scanner exception. ADR-0010–0013, 0018, 0021, 0023, 0024, 0035, 0057, 0077, 0111 and 0112 provide the individual dependency rationales. The full original records, rejected alternatives and historical benchmark table are recoverable in Git at `958aa51a15501417b584c374a23c4b262e6f52ac`. The current whitelist and owners continue to define the concrete admissions. The separate review of the clock entry remains open.
