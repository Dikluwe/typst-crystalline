# ADR-0032 — `unsafe` policy in L1

**Status:** ACCEPTED
**Date:** 2026-04-22

---

## Context

The Crystalline Architecture requires purity in L1 — no system I/O, no mutable global state (linter V13), and no external dependencies outside `[l1_allowed_external]`. The rule “no `unsafe` in L1” was implicitly applied throughout the Step 84.x series as a crystalline convention, consistent with the spirit of ADRs 0004, 0005, 0019, and 0029.

The Step 84.7 audit (Section 6.1 of the report `lab/results/construction/diagnostics/relatorio-auditoria-adrs-passo-84.7.md`) found that the rule was **not literally followed** — the code had 14 actual `unsafe` occurrences in L1 (plus one ineffective comment):

- `01_core/src/engine/lexer/scanner.rs` — **13 occurrences**:
  - 6 `unsafe impl Sealed<T>` (sealed-trait pattern — an encapsulation mechanism, not memory manipulation).
  - 7 `unsafe { get_unchecked(...) }` (unchecked bounds access in code inherited from `unscanny`; see ADR-0014).
- `01_core/src/engine/eval.rs:235` — **1 occurrence**: raw-pointer dereference in `Drop for ImportGuard` (invariant documented in a SAFETY comment).
- `01_core/src/entities/content.rs:20` — one comment documenting vanilla’s `unsafe trait NativeElement`; it is not actual `unsafe` code.

The rule had never been formalized in an ADR. Step 84.7 identified this gap as Input 4 for this decision.

## Analysis of the three `unsafe` classes

### Class A — Sealed traits (6 occurrences)

`unsafe impl Sealed<T> for ...` is a Rust pattern for a private trait with restricted implementors. The word `unsafe` here is arbitrary language semantics — it does not indicate memory manipulation, and there is no actual safety invariant to preserve.

**Safe alternative:** the “sealed via private module” pattern — put the trait in a private module and export only its name.

```rust
mod sealed {
    pub trait Sealed<T> {}
}

pub trait Pattern: sealed::Sealed<char> {}
```

**Cost of removal:** zero. Mechanical refactor.

### Class B — `get_unchecked` in the scanner (7 occurrences)

These are unchecked substring accesses where the scanner maintains its own invariants to guarantee valid indices. Inherited from `unscanny` (ADR-0014 inlined the crate as an L1 module).

**Safe alternative:** replace `get_unchecked(start..end)` with `&self.string[start..end]`. Rust adds a dynamic check — two comparisons and one branch per access.

**Cost of removal:** unknown. Potentially small in lexing (which does not dominate total compilation time — layout and shape do). A reproducible benchmark is required for an informed decision. **It is not possible to decide without benchmarking infrastructure, which the project did not yet have.**

### Class C — `ImportGuard::drop` (1 occurrence)

Raw-pointer dereference in `Drop for ImportGuard` to call `retain` on `EvalContext`’s `import_stack`. This is an RAII pattern using a raw pointer because the lifetime of `EvalContext` cannot be expressed as a lifetime parameter (the guard is created inside a function receiving `&mut ctx` but persists until the function scope ends).

**Safe alternatives:**

1. Use `Rc<RefCell<Vec<FileId>>>` in `EvalContext` and clone it into the guard. Cost: one `Rc` allocation plus runtime borrow checking.
2. Remove the guard. Push/pop manually around the recursive call. This loses RAII — if an error occurs between push and pop, `pop` does not run. Acceptable if `EvalContext` is discarded on error.
3. Replace the raw pointer with an index and check `len()` during drop. No performance cost and no `unsafe`, but loses the structural guarantee that the guard matches its entry.

**Cost of removal:** zero to low. A refactor with a design choice.

## Decision

### Rule in force

**Zero `unsafe` in L1 is the goal. Exceptions require measured cost evidence.**

Interpretation:

- **Ideal to reach:** no `unsafe` anywhere in L1.
- **Temporarily permitted:** `unsafe` in inlined code from a reviewed external crate (e.g. scanner code inherited from `unscanny`) or where a refactor has been chosen as DEBT with a documented plan.
- **Permanently permitted:** only if a reproducible benchmark demonstrates an unacceptable regression from removing `unsafe`, and a case-specific ADR records the concrete number.

### Application to the 14 actual occurrences

| Class | Location | Plan |
|--------|----------|------|
| A (`Sealed`) | `scanner.rs`, 6 occurrences | Remove. DEBT-41 opened in Step 84.8a. Refactor in a dedicated step. |
| B (`get_unchecked`) | `scanner.rs`, 7 occurrences | DEBT-42 opened in Step 84.8a. Blocked by benchmark infrastructure. Decide after measurement: remove if regression < X%; otherwise retain under a specific ADR. |
| C (`ImportGuard`) | `eval.rs:235`, 1 occurrence | Remove. DEBT-40 opened in Step 84.8a. Refactor in a dedicated step choosing among 3 options. |

### Absolute prohibitions (including L3)

- `Box::leak` — ADR-0004 established the precedent.
- Arbitrary `mem::transmute` between unrelated types.
- `std::mem::zeroed` on non-`Copy` types.
- Raw-pointer casts via `as` between pointer types.

### Clause on inlined code

ADR-0014 authorized inlining `unscanny` in `01_core/src/engine/lexer/scanner.rs`. At the time of its original inclusion, `unsafe` in this file was **inherited** from external code reviewed by the Rust community.

This clause does **not** exempt the scanner from the “zero `unsafe`” rule — it only records that the current `unsafe` was not introduced by the crystalline project and has an external review history. Its removal remains pending work recorded in DEBT-41 and DEBT-42.

## Affected prompts

| Prompt | Change |
|--------|--------|
| `00_nucleo/prompts/engine/scanner.md` (if it exists) | Note DEBT-41 and DEBT-42 — current `unsafe` is temporary. |
| `00_nucleo/prompts/engine/eval.md` (if it exists) | Note DEBT-40 — `ImportGuard` will be refactored. |

The actual prompt updates are deferred to the steps that materialize the refactors (DEBT-40, 41, 42), not this decision step.

## Consequences

**Positive:**

- Formalizes a rule that was applied implicitly and violated without visibility.
- Gives a clear criterion for future decisions: zero is the goal; exceptions require concrete evidence.
- Establishes a precedent against empty ritual (SAFETY comments without a demonstrated invariant).

**Negative:**

- This ADR creates a dependency between future refactors (DEBT-41, DEBT-42) and benchmarking infrastructure that did not yet exist.
- “Permanently permitted with a specific ADR” could be abused if the benchmark is not reproducible; “unacceptable regression” is ambiguous. Resolution: every future exception ADR must state a concrete number and methodology.

**Neutral:** `01_core/src/entities/content.rs:20` contains a comment about vanilla’s `unsafe trait NativeElement`. It is not actual `unsafe` code and is unaffected.

## Alternatives considered

| Alternative | Advantages | Disadvantages |
|-------------|------------|---------------|
| “Zero `unsafe` immediately” | Strongest rule | Requires an immediate refactor in 14 places, including 7 with unknown cost |
| “Keep it permanently with SAFETY comments” | Accepts current reality | Normalizes `unsafe` without an exit plan; empty ritual |
| “Zero `unsafe` with predeclared permanent exceptions” | Simple | Requires listing exceptions before measuring cost — a blind decision |
| **Adopted: “Aim for zero, with measured DEBTs”** | **Honors both constraints (purism + realism)** | **Depends on benchmark infrastructure to close DEBT-42** |

## References

- ADR-0004 — precedent on `Box::leak` as a “silent exception.”
- ADR-0005 — confirmed removal of `Box::leak`.
- ADR-0014 — `unscanny` inlining (source of scanner `unsafe`).
- ADR-0019 — `unsafe` authorized in L3.
- ADR-0029 — physical definition of L1 purity.
- `lab/results/construction/diagnostics/relatorio-auditoria-adrs-passo-84.7.md`, Section 6.1 — empirical motivation for this ADR.
- DEBT-40, DEBT-41, DEBT-42 — pending work resulting from this decision.
