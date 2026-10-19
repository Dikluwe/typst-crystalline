# Workflow — auditing a proposed module split

**Purpose:** Decide whether and how to split a large file/module (the hub) into smaller units (nodes). This method was extracted from five real applications: `operators.rs` (P1002), `layout::metrics` (P1006, rejected in full), `eval::bindings` (P1013), `stdlib::structural` (P1014), and `stdlib::text` (2026-08-13). It was not written in advance; each rule has provenance.

**This is not:** an automatic approval gate. It is an investigation guide. “Do not split” is a valid result (P1006), not a failure of the method.

## Required order

### 0. First criterion: aggregate or interface?

Before anything else, inspect public and restricted traits:

```sh
grep -nE '^(pub(\([a-z:) ]+\))? )?trait ' <file>
```

If the file defines a public trait with multiple real implementers and most of its content is that trait’s methods, it is an **interface**. Splitting it would change the public contract (ADR-0127) to move little actual logic and is usually not worthwhile (P1006: `layout::metrics`, 13 implementers, about 90 lines of logic, rejected).

If it is an aggregate of free functions (`fn` / `pub(...) fn`, at any visibility), continue. This distinction was decisive in both directions: P1006 rejected a split, while P1013 and P1014 identified aggregates without friction. Always make this determination first.

### 1. Build the complete inventory with a visibility-generic pattern

Three repeated failures exposed incomplete inventories: P1000 omitted four of 13 `operators.rs` functions by missing `pub(crate)`; P1013 omitted 18 of 43 `bindings.rs` functions because the pattern included `pub(crate)` but not `pub(super)`. Fixing a single case only moved the gap.

Use a pattern that covers restricted visibility:

```sh
grep -nE '^(pub(\([a-z:) ]+\))? )?fn ' <file>
```

Also count top-level `enum` and `struct` declarations when relevant to the domain. In P1013, `enum ContentField` belonged in the inventory and was nearly missed.

### 2. Apply the four criteria in the order supported by evidence

#### Criterion 3 — historical co-change (the practical decision signal)

Across four applications this was the only criterion that always produced actionable signal: it corrected the initial hypothesis in P1002 (`coercion.md` was discarded), confirmed it in P1013 (commit-by-commit alignment with vanilla clusters), and corrected over-splitting in P1014 (it rejected splitting `table_grid` / `table_lines` even though vanilla separates them).

Run `lab/tools/analysis/cochange_metrics.py <file> <historical-path-regex>`, including known renames in the path pattern (for example `rules/` → `engine/` → `compiler/`). Exclude commits that only change `@prompt-hash` / `@updated` metadata (proposed in P1006 and implemented in the tool by P1013); mass reseals, renames, and global formatting otherwise mask the signal.

A commit that touches many functions from different clusters at once (P1013: `b806f562d`, 16 functions across three clusters) is structural noise, such as introducing a cross-cutting mechanism, not boundary evidence. Identify and discount it.

**Boundary-attribution artifact:** In `stdlib/text.rs` (2026-08-13), when a change adds a new function, the comment banner before it (`// ── … ──` or a doc comment) falls after the preceding function’s `}` and before the new `fn`. The tool assigned those lines to the preceding function, making it appear to co-change with an unchanged body. This fabricated three of four apparent clusters: `overline` + `smallcaps` (`0b97d2d78`), `smallcaps` + `sub`/`super` (`f28fba77d`), and `highlight` + `super` (`fa5bda1d4`). It would have made two wrong decisions: join `smallcaps` to `sub`/`super`, and connect `highlight` to the wrong pair.

The correction has two necessary parts; the first alone failed:

1. An item’s boundary extends through the **next item’s banner**, not merely to the next declaration.
2. Only lines **starting at the declaration line** count as body changes. Banner lines are reported separately (“banner only; does not count”). The blank line left by the preceding item belongs to the next banner; counting it recreates the artifact.

Without the corrected tool, suspect an artifact when one function is new in a commit and the other is immediately above it in the file. Check the diff hunk:

```sh
git show <commit> -U0 -- <path> | grep '^@@'
```

A hunk `-N,0 +M,K` with **zero removed lines**, whose context names the preceding function, is a pure insertion: the context function did not change. This distinguished all six known cases (three in `stdlib/text.rs`, two in `stdlib/structural.rs`, one in the audit script) from real co-change.

**Tests remain attributed:** In a file with its own test suite, functions inside `mod tests` are labeled `test:<name>` in tool output. They are noise for boundary decisions, but do not exclude them from attribution. Otherwise their lines are assigned to the last top-level item, recreating the original artifact one layer up. The retrospective audit script made this mistake before correction: `heading` appeared to co-change with `native_table_vline`, the last function before the test suite.

#### Corollary — re-audit earlier decisions when the measurement tool changes

A correction to the measurement tool calls every earlier boundary it decided into question. This was done for the three earlier families on 2026-08-13; results are in `lab/results/construction/diagnostics/typst-passo-1022-auditoria-retroactiva.md`.

| Family | Artifacts | Boundaries decided from artifacts |
|---|---:|---|
| `operators` | 0 | none |
| `eval::bindings` | 10 (inflated real clusters) | none |
| `stdlib::structural` | 15 | 2: `flow` (`par` + `quote`) and `sectioning` (`outline` + `title`) |

For both `structural` cases, the cited cluster did not exist. The false statement was corrected in the node L0 prompts, while code remained unchanged and the split decision was recorded as open with an owner. Therefore:

- Compare the report’s written claims, cluster by cluster, with the new measurement. Rerunning the tool and looking only at the total is insufficient.
- A co-change pair with no plausible mechanism must be checked, not explained away. The implausible `heading` + `table_vline` pair exposed the second artifact.
- A committed boundary is not immune to review after the tool changes. The audit diagnoses; a later task closes the decision. On 2026-08-13, the two false `stdlib/structural` boundaries (`flow`, `sectioning`) were undone into seven nodes after corrected measurement showed one genuine group (`outline` + `lof` + `lot`) and six standalone native functions.
- When undoing a refuted grouping, remove its name as well. Keeping a directory called `flow/` or `sectioning/` and placing the nodes beneath it restates the disproved group in the topology and adds a logic-free re-export sub-hub. Flatten the nodes into the existing hub.
- A node containing one native function is legitimate. Six of seven reviewed `stdlib/structural` natives stood alone. Criterion 4 can decide when Criterion 3 is silent, but not when it contradicts the split. Record whether Criterion 3 was unmeasured, silent, or refuting; “not measured” and “measured and refuted” differ.

#### Criterion 4 — vanilla correspondence (a hypothesis to test, never sufficient alone)

Both default and over-splitting errors have occurred:

- P1006: name-based mapping pointed to the wrong file (`layout::metrics` was matched by “metrics” rather than actual content).
- P1014: vanilla splits more finely than co-change supports (21 vanilla files for 50 crystalline functions; history supported only nine groups).

“Follow vanilla” is never the final decision. It is a candidate that Criterion 3 accepts, corrects, or rejects. Use only the currently pinned vanilla target documented in `CLAUDE.md`; do not compare to a moving branch or a stale pin. A vanilla match does not make a Rust implementation detail a language-parity requirement.

#### Criterion 2 — purity versus state (three classes, not two)

Refinement from P1012/P1013: the presence of `Engine` / `EvalContext` in a signature does not decide purity. Measure uses after the context is received and cite `file:line`. The three observed classes are:

1. **No context:** the signature receives no `Engine` / `ctx`.
2. **Pass-through:** it receives context but has no real uses afterward, only propagating it to evaluate arguments without introducing an effect. Classify as functionally pure despite the signature.
3. **Real context use:** cite measured reads/writes with effects such as I/O, sinks, introspector, or counters.

A node may be called declarative for a mechanical reason (pass-through), never because of its “purpose” or what the code is presumed to intend. P1011 misclassified `font_dict` by intent; P1012 corrected it using evidence.

This criterion was decisive when it separated three real classes (P1013). When the whole file was pure or impure (P1002, P1006, P1014), it was often vacuous, but must still be measured: that result shows the state boundary is not the relevant axis.

#### Criterion 1 — test isolation (proposal: remove or replace; not ratified)

This criterion was vacuous in five of five applications and pointed against the correct split in P1014: 56 tests share one harness, and splitting them would create nine copies. The project tests end-to-end through Typst files, one file per module, so test granularity does not align with function granularity. In `stdlib/text` (2026-08-13), the split file had zero `#[test]`; the entire suite lived in `stdlib/mod.rs`.

**Unresolved proposal, not an adopted rule:** At the next application, consider replacing this with “who calls it”—fan-in per symbol within the module/domain (number of distinct consumers) as a cohesion proxy. It was explicitly not applied to `stdlib/text`: its eight boundaries were decided by Criterion 3 after artifact discounting, and Criterion 4 where Criterion 3 was silent; no uncertain boundary needed fan-in to break a tie. Keep this proposal’s status unresolved until it is explicitly reviewed.

### 3. Check orphan prompts before writing a new L0

Twice confirmed (P1002 with `decimal-arithmetic.md`; P1014 with four `stdlib/grid_*.md` / `table_*.md` prompts): an existing ownerless prompt contains a previously written boundary hypothesis and is often useful. Read it before drafting, compare it with current code (it may have drifted), and retain only what remains correct.

Also inspect `crystalline.toml` for orphan exceptions that name files removed by earlier splits. P1014 found that the `decimal-arithmetic.md` exception remained after the file was deleted in P1002.

### 4. Materialize only under the current ownership rules

- V15: each materializable L0 has exactly one productive consumer, and each consumer has exactly one owning L0 (ADR-0129). Shared invariants belong in pinned Tekt nuclei; they do not create shared ownership. Do not create an L0 header, path, or code link that implies an unresolved many-to-one or one-to-many relationship.
- Preserve behavior by moving code without rewriting it. Compare each item (counts before/after and bodies), not only test results.
- Preserve original visibility when moving a function from a flat aggregate to a node. For example, old `pub(super)` may require `pub(in full::path)` in the node and `pub(super) use` in the hub; a wider hub re-export than the item allows produces E0364 (P1013).
- A hub is not always a dispatch table. If the original unit is a flat aggregate with direct calls and no dispatcher, the hub should only re-export. State this in the owning L0 so it is not mistaken for an incomplete dispatch hub (P1013).
- Do not duplicate shared-harness tests into nodes. Keep them in the hub and state in its L0 that it owns the suite (P1014).
- A node name that collides with an external crate may require `pub use self::<node>::…` (V14; observed in `stdlib/text`, 2026-08-13).

### 5. Validate serially

Write or adjust tests before implementation, run them and confirm the intended failure (RED), then implement and confirm they pass (GREEN). Keep these stages separate; do not require tests and code to be authored simultaneously.

Run the checks required by the repository and affected owners. For this historical workflow, the recorded checks were:

```sh
crystalline-lint .
cargo test --workspace
```

Compare the count of `#[test]` functions before and after; it must match exactly. Follow current repository guidance for build/resource limits and additional checks. A passed test suite alone does not prove code-preserving extraction.

### 6. Evaluate the method on every application

Even when the result is “do not split” (P1006), record which criterion decided, which was silent/vacuous, whether evidence corrected an initial hypothesis (vanilla candidate or inspection-based split), and whether an earlier guard clause (for example, “if this split touches X, confirm Y”) activated. State explicitly whether it activated; silence must not imply the answer. This was learned in P1014, when confirmation of `table_counter` had to be requested after the report omitted whether the guard fired.

## Provenance of rules

| Rule | Origin |
|---|---|
| Decide interface versus aggregate first | P1006 |
| Visibility-generic inventory pattern | P1000 → P1013 (repeated omission, closed here) |
| Exclude reseal noise in co-change | P1006 (proposed) → P1013 (in tool) |
| Two-part boundary-attribution correction | `stdlib/text`, 2026-08-13 (in tool) |
| External-crate node collision may require `pub use self::<node>::…` (V14) | `stdlib/text`, 2026-08-13 |
| Implausible co-change needs investigation | Retrospective audit after `stdlib/text`, 2026-08-13 |
| Previously materialized boundaries can be revisited after tool correction | Retrospective audit, 2026-08-13 |
| Remove refuted group names and flatten nodes | `stdlib/structural`, 2026-08-13 |
| Vanilla is never sufficient alone | P1006 (wrong match) + P1014 (over-splitting) |
| Three measured purity classes | P1012 (`font_dict`) + P1013 (refined) |
| Check orphan prompts and stale `crystalline.toml` exceptions | P1002 + P1014 |
| Hub without dispatcher only re-exports | P1013 |
| Shared-harness tests stay in the hub | P1014 |
| Explicitly record whether a guard activated | P1014 (this consolidation) |
| Criterion 1 replacement with fan-in | Unresolved proposal from P1002/P1006/P1013/P1014; not applied |
