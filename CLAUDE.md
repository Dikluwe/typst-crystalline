# CLAUDE.md — typst-crystalline

This file is the governing assistant guide for this repository. Read the applicable ADRs in `00_nucleo/adr/` before making architecture decisions.

---

## Critical terminology: execution steps, L0 prompts, and Tekt Nuclei

Distinguish these three artifacts to prevent code being written without a specification:

- **Execution step:** temporary tactical coordination for immediate work, debugging, or planning a session between a human and an AI. It is not L0 and has no permanent numbered-step directory in the active tree. Historical construction records remain recoverable under `lab/history/construction/records/00_nucleo/materialization/`.
- **L0 prompt:** the enduring architectural specification for a productive component. It is the causal source that authorizes and defines the code. L0 prompts live in `00_nucleo/prompts/`.
- **Tekt Nucleus:** declarative invariants shared by two or more L0 prompts. A Nucleus does not directly authorize code, has no `Hash do Código`, and is never a direct `@prompt` target. Nuclei live in `00_nucleo/prompts/_nuclei/`.

**Required cardinality (ADR-0129):** each materializable L0 prompt has exactly one productive consumer, and each productive consumer has exactly one owning L0 prompt. Sharing 1:N occurs only through Tekt Nuclei consumed and pinned by the owning prompts. V15/V26 block repairs while ownership or the shared graph is invalid.

**Golden rule (architectural lock):** never instruct writing L1/L2/L3 code unless its corresponding L0 prompt exists under `00_nucleo/prompts/` and is current.

The workflow is:

```text
1. Temporary task coordination → 2. AI drafts or updates the L0 prompt → 3. Human saves L0 and calculates its hash → 4. AI writes code (L1)
```

**When to stop for confirmation (ADR-0127):** stop at step 3 only for (1) a public contract change (entity field, trait method, public signature), (2) a change to default product behavior (new mode/flag/default path), (3) a pipeline phase change (eval ↔ layout), or (4) a compatibility break. Internal formula corrections, mapping-table entries, and vanilla-parity corrections follow continuous flow: update L0 first, reseal its hash, then pass the RED→GREEN and revalidation gates without stopping for a separate decision. If the change class is uncertain, stop and classify it. See [ADR-0127](00_nucleo/adr/adr-0127-l0-gate-stop-vs-continuous-flow.md).

---

## Crystalline architecture (Tekt)

The original compiler source is quarantined in `lab/typst-original/`. Migration proceeds into the crystalline layers. The primary structural success criterion is `crystalline-lint .` with zero violations.

### Layers and import topology

| Layer | Directory | Import rules and purpose |
|---|---|---|
| L0 | `00_nucleo/prompts/` | Specifications; not code; source of lineage. |
| L1 | `01_core/` | Pure domain; zero I/O; imports only pure standard-library items and the whitelist. |
| L2 | `02_shell/` | CLI and formatters; knows only L1. |
| L3 | `03_infra/` | I/O, filesystem, fonts; knows only L1. |
| L4 | `04_wiring/` | Composition; knows L1, L2, L3; no business logic. |
| lab | `lab/` | Quarantine; never imported by L1–L4. |

---

## Organization under `00_nucleo`

`00_nucleo/` holds current architectural decisions, L0 prompts, and concise diagnostic summaries. Construction steps are temporary coordination, not a permanent directory hierarchy.

| Area | Purpose | Example |
|---|---|---|
| `adr/` | Formal architectural decisions. | `adr/adr-0109-atomization.md` |
| `diagnostics/` | Current analyses, inventories, measurements, receipts, and state reports. | `lab/results/construction/diagnostics/typst-cobertura-vanilla-vs-cristalino.md` |
| `prompts/` | L0 prompts directly paired with L1–L4 productive code. | `prompts/core/compiler/layout/mod.md` |
| `prompts/_nuclei/` | Shared declarative Tekt obligations in TOML. | `prompts/_nuclei/path-identity.toml` |

Organization rules:

- `prompts/` is reserved for L0 specifications with direct correspondence to code produced in L1–L4.
- L0 prompt ↔ productive consumer correspondence is strictly 1:1.
- ADR-0131 makes owner paths deterministic: `prompts/{core,shell,infra,wiring}/`
  plus the consumer-relative path, omitting only leading `src/` and replacing
  `.rs` with `.md`. Preserve `tests/`, `mod`, `lib`, `main`, and `build`; source
  and owner basenames describe responsibility, not execution-step numbers.
- `prompts/_nuclei/**/*.toml` is a distinct L0 namespace using strict TOML 1.0, `tekt = 1`, `kind = "nucleus"`, `must`/`must-not`/`may` claims, a DAG, and full SHA-256 pins in consuming prompts.
- Every Nucleus has one or more consuming prompts; an orphan Nucleus is V26.
- Code never points directly to a Nucleus. A Nucleus never contains `Hash do Código`, a productive owner, or a consumer list.
- `prompts/` does not contain process documents, scan reports, state analysis, health metrics, or diagnostics. Current summaries belong under `diagnostics/`.
- A document that specifies what must be implemented and authorizes code is an L0 prompt. A document describing what was done, repository state, metrics, or current findings is a diagnostic.
- State analyses and mechanical scans are diagnostics, not prompts.
- Do not create a permanent `materialization/`, `context/`, or numbered execution-step hierarchy in the active tree. Retain historical construction records under `lab/history/construction/records/00_nucleo/materialization/`. Preserve diagnostic reports and raw measurements under `lab/results/construction/diagnostics/`; concise current summaries belong under `00_nucleo/diagnostics/` when present.

### Absolute L1 restrictions

**Never:**

- Read or write files (`std::fs`), make network calls, access the clock (`SystemTime`), or read environment variables (`std::env`).
- Use mutable global state (`static mut`, `static Mutex<T>`, `static OnceLock<T>`).
- Import external crates not declared in `[l1_allowed_external]`.

**Allowed** (RAM performance work is not I/O):

- `Arc<T>` in struct fields (ADR-0029), `Vec`, `Box`, `String`, and `HashMap`.
- `EcoString` (ADR-0024), `rustc-hash` / `FxHashMap` (ADR-0018).

---

## Parity is with the language, not mechanics (ADR-0107)

Parity is with the Typst **language**—semantics, syntax, and morphology—never with execution mechanics or Rust's restricted equality. Implementation structure, Rust `PartialEq`, output bytes, and algorithm steps intentionally diverge (P329). Measure and define acceptance at the three language levels, not by mechanics. **Morphology** is the form of content as a language object (text, markup, semantic styling such as `*bold*`), distinct from baked/derived **render** style. See [ADR-0107](00_nucleo/adr/adr-0107-language-parity-not-mechanics.md).

---

## Parity reference: which binary and source (pinned 2026-10-26)

The current vanilla upstream/main parity target is pinned to commit **`64c5e8c02fe0394d913ead36e42ef8c1334d2107`**, tree **`483bd6f1d8ac8810db47b65336d402647955fd17`**. The explicit resync authorized on 2026-10-26 replaces pin `bd5ebec285505889e8ff1d4f68804ab147a699f3`; the prior P1489 pin is `9f2b6e8715237cb086899a42873660fe744622e8`. A future update requires an explicit new step/decision. Do not use moving `main` or a textual tag/version as the target. Measurements, prompts, and reports against `bd5ebec285`, `9f2b6e871`, `586e1bd43`, or earlier pins remain historical for those snapshots; they do not certify the current target without a new run. Pin `a51e02804` remains historical in the P990–P992 correction. Parity follows ADR-0107/0108: measure semantics, syntax, and morphology before deciding; do not copy vanilla defects as requirements for Crystalline.

Four provenance traps:

1. **Version string is supporting evidence only.** `lab/typst-original` has no separate `.git`. The ratified build explicitly received `TYPST_COMMIT_SHA=64c5e8c02fe0394d913ead36e42ef8c1334d2107`, Rust 1.92.0, the lockfile, one job, a 12 GiB ceiling, and zero swap. Its source was blob-compared with the tree above, keeping only the lockfile and local extractor as overlays. Executable `lab/typst-original/target/release/typst` has SHA-256 `22897f93c7c2c8589cf5913b0ce735f1c01ba711c70f55c7a350f7c4b96c0b34`. Recipe, source/tree verification, resources, and full receipt are in `lab/results/construction/diagnostics/vanilla-resync-2026-10-26/relatorio.md` and its evidence under that directory. The previous bd5ebec285 oracle is preserved as `lab/typst-original/target/release/typst-bd5ebec285-2026-10-17` (SHA-256 `9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`). The P1489 binary is preserved locally as `lab/typst-original/target/release/typst-9f2b6e871-p1489`; its reports retain the original SHA. Identity requires commit, tree, recipe, and SHA-256, never only `--version`.
2. **`lab/target/product/release/typst` is the Crystalline product binary.** `04_wiring` declares `[[bin]] name = "typst"`; its public version comes from `PARITY_VERSION` (`entities/version.rs`) and our HEAD. Confusing product and oracle was the P1024 error. The oracle is the verified build in the lab or an independently checked-out CI build with a verified receipt. `/usr/local/bin/typst` can replace it only after its bytes are proven identical to the current ratified build.
3. **`sys.version` is a language surface.** Direct probe of new vanilla `64c5e8c0` returned `version(0, 15, 1)`. Crystalline retains this triplet; `PARITY_VERSION` did not change in the resync. A future change requires the ADR-0127 gate in `00_nucleo/prompts/core/entities/version.md`.
4. **Crystalline `--version` can be stuck on an old HEAD.** Before P1033, `02_shell/build.rs` only declared `rerun-if-env-changed=TYPST_COMMIT_SHA`. P1033 fixed this with `cargo:rerun-if-changed=../.git/HEAD` and `../.git/refs`. Provenance still records HEAD and tree state; a version string alone does not prove the source used for a measurement.

Never cite "0.15.0" or "0.15.1" as the target without confirming the pinned hash first.

---

## Anti-drift discipline: measure before deciding (ADR-0108)

Every prompt measures before deciding: the decision/classification section follows the measurement (`file:line`) that supports it, never the reverse. Classify source evidence as language or mechanics (semantics/syntax/morphology means parity; Rust equality/bytes/steps/structure means divergence under ADR-0107). Distinguish intended behavior from observed behavior; do not infer intent from behavior. Mark inferences and what would refute them. Be suspicious of flattering or convenient framing and check the source again before accepting it. Acceptance is at the language level, never mechanics, except where mechanics are themselves observable (for example, an error message). A malformed prompt still requires the owner to audit its substance; format checks reduce reliance on the brake but do not eliminate it. See [ADR-0108](00_nucleo/adr/adr-0108-anti-drift-measure-before-deciding.md).

---

## Record the provenance of every measurement

**Date:** 2026-07-05. **Applies to:** every project step that produces a number used to decide something.

P569 recorded "4 lines" for a test document. P574 and P575 later tried to reproduce the number from P569's commit and got "2 lines" both with and without the orphan L1 code thought to be responsible. The number was not reproduced and its cause remained unknown. The problem is missing provenance: the exact code state at measurement time, whether uncommitted changes existed, and what otherwise distinguished that state from the final commit.

Any number used in a report to decide whether something is open or closed—page, line, word, position, or runtime counts—must include:

1. The commit hash where the test ran, or "uncommitted working tree" plus the exact changed-file list from `git diff HEAD --stat`.
2. The exact time when the state may have changed between measurements within the same step.

This complements, and does not replace, the rules to make a new decision and measure before deciding. Future reports must answer "from exactly what code state did this number come?" without guesswork. If the answer is unknown, the number is only an indication to remeasure, not evidence that closes an item.

The three rules complete the cycle: decide again, support the decision with evidence, and preserve enough provenance for that evidence to be found and reproduced.

---

## Atomization: move logic to its owning unit and layer (ADR-0109)

**Atomization** means moving logic out of monolithic files into the owning unit's file in its proper layer, with each element/feature readable on its own. It does **not** mean reducing `content→elements` to zero, decoupling imports, using vtables/`dyn`/PropMap, or removing exhaustive `match` dispatch.

The required shape is B: keep the core `match` lean, with bodies delegating to a free function such as `<elem>::layout(self, e)` in `compiler/layout/<elem>.rs`. Move render logic to the feature file in the render layer; access `Layouter` through module descent, with no reverse `entities→compiler` import and no `pub(crate)`. Option A—render logic in the struct file—is rejected because it couples data to rendering. Keep exhaustive `match`, jump table, and imports. The `content→elements` metric is irrelevant as a gate. A proposal for dynamic dispatch, zeroing `content→elements`, or Option A in the name of atomization is prohibited drift; stop and separate these meanings. See [ADR-0109](00_nucleo/adr/adr-0109-atomization.md).

---

## Nucleation protocol (required before code)

1. **Audit L0:** Does a prompt exist under `00_nucleo/prompts/` for the affected module? Is it current against applicable ADRs?
2. **Validate L0:** If it is absent or outdated, draft the new L0 and stop. Continue only after the human confirms the file is saved and its hash is available.
3. **Tests first:** Add tests in the module (`#[cfg(test)]`) and confirm they fail.
4. **Implementation:** Write code to make the specified tests pass.
5. **Lineage:** Add the required header with correct `@prompt-hash` and canonical `@prompt`.
6. **Final validation:** `cargo build && crystalline-lint .` with zero violations.

---

## Linter gates and common errors

| Code | Name | Resolution |
|---|---|---|
| V3 | `ForbiddenImport` | Import violates layer topology; reorganize the dependency. |
| V4 | `ImpureCore` | I/O symbol detected in L1; move it to L3 and inject through a trait. |
| V5 | `PromptDrift` | Prompt hash diverges; run `crystalline-lint --fix-hashes .` after editing L0. |
| V13 | `MutableStateInCore` | Mutable global state detected in L1. (`Arc` constructed in a struct does not trigger this.) |
| V14 | `ExternalTypeInContract` | Undeclared external type imported in L1. Do not use `pub use self::X::Y` in L1. |
| V15 | `PromptOwnership` | Materializable prompt lacks 1:1 productive consumer ownership; individualize owners before resealing. |
| V26 | `NucleusIntegrity` | Tekt Nucleus/pin/DAG/orphan is invalid or code points directly to a Nucleus; repair the graph before `--fix-hashes`. |

---

## Current ADRs: read before proposing architecture

| ADR | Decision |
|---|---|
| [ADR-0018](00_nucleo/adr/adr-0018-rustc-hash-l1-dependency.md) | `rustc_hash` authorized in L1 (revokes ADR-0007). |
| [ADR-0024](00_nucleo/adr/adr-0024-ecow-for-value-string.md) | `EcoString` in `Value::Str`; O(1) clone in `eval()`. |
| [ADR-0026](00_nucleo/adr/adr-0026-content-intentional-divergence.md) / [ADR-0026 R1](00_nucleo/adr/adr-0026-r1-content-arc-revision-1.md) | Closed `Content` enum; `Arc` in `Sequence`. |
| [ADR-0029](00_nucleo/adr/adr-0029-physical-l1-purity.md) | Physical purity; `Arc` in domain structs is permitted (revokes ADR-0028). |
| [ADR-0030](00_nucleo/adr/adr-0030-ram-performance-in-l1-domain.md) | RAM performance belongs to L1; corrects ADR-0004/0015. |
| [ADR-0031](00_nucleo/adr/adr-0031-early-source-hashing.md) | Early hashing in `Source`; complements ADR-0016. |
| [ADR-0107](00_nucleo/adr/adr-0107-language-parity-not-mechanics.md) | Parity is language semantics/syntax/morphology, not mechanics or Rust equality. |
| [ADR-0108](00_nucleo/adr/adr-0108-anti-drift-measure-before-deciding.md) | Anti-drift discipline: six verifiable rules; measure before deciding; owner audits substance. |
| [ADR-0109](00_nucleo/adr/adr-0109-atomization.md) | Atomization uses owner-layer render logic and free functions (shape B, Option A rejected); keep exhaustive static `match` and imports; it does not decouple `content→elements`. |
| [ADR-0126](00_nucleo/adr/adr-0126-verbose-pdf-default-compact-option.md) | Verbose vanilla-mirroring PDF is production default; compact is `--compact` validated by decalque; tagged PDF/accessibility is separate. |
| [ADR-0127](00_nucleo/adr/adr-0127-l0-gate-stop-vs-continuous-flow.md) | Stop only for public contract/default behavior/pipeline phase/compatibility changes; internal and parity corrections use continuous flow with L0 first and resealing. |
| [ADR-0128](00_nucleo/adr/adr-0128-html-as-semantic-target.md) | HTML is a separate semantic target from paginated layout; feature and target are separate axes. |
| [ADR-0129](00_nucleo/adr/adr-0129-l0-owners-and-shared-tekt-nuclei.md) | L0 prompt ↔ consumer is 1:1; shared invariants use pinned 1:N Tekt Nuclei validated by V26. |
| [ADR-0130](00_nucleo/adr/adr-0130-canonical-l0-prompt-references.md) | L0 references use full canonical paths; `@prompt` headers identify the owner unambiguously. |
| [ADR-0131](00_nucleo/adr/adr-0131-owner-aligned-prompt-paths.md) | Owner paths mirror the layer and source modules without leading `src/`; active filenames describe responsibility. |

Do not follow revoked ADRs that are absent from the current table.

For operational `#[comemo::track]` conventions in L1 (Clone, `Option<&str>`, and `TrackedMut::reborrow_mut` gotchas), see [`01_core/CLAUDE.md`](01_core/CLAUDE.md).

---

## Constructor work intentionally split across steps

When a constructor or feature is deliberately divided across multiple temporary tasks, the initial L0 must explicitly say it is incomplete and name the task that completes each subset. Never leave L0 in an undocumented intermediate state; that creates drift (for example, the `duration()` string form in P403 versus named arguments in P405). Record this in an L0 header note or a future scope-out clause that points to the task record. Historical step records are recoverable under `lab/history/construction/records/00_nucleo/materialization/`; they are not a permanent active step directory.

## Read current L0 before proposing architecture

Before proposing an architectural option in a specification—especially module subdivision/consolidation—read the current L0 for affected modules and confirm its hash. If L0 has already decided the issue (for example, `stream.md` says not to subdivide shape primitives), do not recommend the opposite. Resolve conflict by updating L0 and its hash or by scoping the proposal out; do not recommend option β against current L0. This gate prevents drift like P427 (`shape_emit.rs` versus `stream.md`, hash `9acca994`).

See [`01_core/CLAUDE.md`](01_core/CLAUDE.md) for the `TrackedMut::reborrow_mut` lifetime-descent convention.
