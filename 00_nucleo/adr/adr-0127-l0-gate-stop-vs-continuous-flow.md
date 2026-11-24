# ADR-0127 — L0 gate: mandatory stop for contract or behavior changes; continuous flow for internal fixes

**Status:** ACCEPTED
**Date:** 2026-08-04
**Decision owner:** Repository owner (in response to the question about P962–P964 — the distinction was already practiced; the owner accepted it as valid and requested that it be written as a rule).
**Recorded in:** Step 965.
**Consolidated:** 2026-11-24
**Amended:** 2026-11-24 — protected contracts, supported-use compatibility and parity packages. Approved by the owner on 2026-11-24 (“Aprovo”, in chat, after review of the wording).

---

## 1. Context

The “Golden Rule” (`CLAUDE.md`, Nucleation Protocol) says: the AI drafts L0 and **STOPS**, proceeding only after human confirmation. In practice (P893–P964), the actual stop occurred only in two kinds of situations, while internal corrections followed continuous flow (edit L0 first, without stopping). The original wording did not distinguish the two modes, and that ambiguity allowed the P952 deviation: new fields in `MathConstants` and a new method in `FontMetrics` were added **without** a stop, then corrected in a retrospective review. This ADR formalizes the criterion so it does not depend on case-by-case judgment.

## 2. Decision — the criterion

### MANDATORY STOP before writing code (confirm with the owner)

The test is whether the change alters a contract someone can use, or an architectural boundary the project has decided to preserve. Rust visibility is a signal to investigate use. It does not settle the classification: the layers are separate crates, so `pub` often exists only to connect them.

1. **Protected contract:** change any of the following.
   - Typst syntax, semantics or observable diagnostics, or the CLI, except a qualifying convergence under continuous item 3.
   - Output or interchange formats and persisted data, including documented serialized field names, under the same exception.
   - Integration traits, meaning traits whose implementations are supplied across a layer boundary (e.g. `World`, `FontMetrics`).
   - Entities declared as contract: those whose public shape is fixed by the owning L0, a pinned nucleus or a recorded owner approval (e.g. `Value`, `Selector`, `MathConstants`).

   A public type, field or function outside these groups is presumed internal after its actual consumers have been inspected and recorded. An entity in `entities/` with no declaration is classified with the owner the first time its public shape changes, and the result is recorded in its L0. A specific L0 or nucleus clause that preserves a signature stays in force.
2. **Product default behavior:** introduce a new default mode or flag, change the compiler’s default path (e.g. P956 — verbose emission as the default), or make any deliberate change to the user-facing surface. Qualifying convergence under continuous item 3 is excluded.
3. **Architectural change to pipeline phase/order:** move logic between phases (e.g. eval ↔ layout, the type of decision in P966), even if the final implementation is small.
4. **Compatibility:** break a use supported by the current version, such as accepted input, a documented command, readable persisted data or a declared integration. Judge the change itself. The possibility that a future reversal would break code written against an addition does not trigger this item.

### CONTINUOUS FLOW (edit L0 first + reseal its hash; NO stop)

1. Correct a formula/value inside an existing function without changing a protected contract (P957, P959-formula, P963).
2. Add or correct entries in mapping tables — symbols, names, shorthand (P958, P962, P964).
3. **Parity corrections that preserve protected contracts**, even when they change rendered output or a diagnostic (P958: `Gamma(z)` → Γ(𝑧); P961: reduced legend). Convergence of language behavior with the pinned reference restores the contract defined by ADR-0107. The exception does not cover reversing a recorded deliberate divergence, changing a protected Rust, CLI or format carrier, moving a phase, changing a default outside the reference behavior or ceasing to accept a supported use. The gate is RED→GREEN tests plus revalidation (visual/`compare.py`/benchmark).
4. Any internal change that meets none of the mandatory-stop items, including changes to visibility or signatures used only to connect modules or layers.

### Parity packages

When a parity correction alters a protected contract or otherwise meets a mandatory-stop item, approval prevails over continuous item 3. One approval covers the package. The proposal names the language behavior, affected owners and carriers, phases and compatibility effects, the validation and any exclusions. Work within that scope then follows continuous flow, including internal adjustments it requires. Reopen approval only if the scope changes: another protected contract, owner contract or phase, a new compatibility effect, or reduced validation.

Example: a gradient text-fill package can cover `Paint` transport through styles, the affected public text carriers, nested style scope and restoration, shaping, and validation in PDF and SVG. The example does not approve that package.

### Where the approval lives

Record the scope, date and source of the owner's decision against the specific L0 change. An approval for one public type or capability covers only the approved contract. If the change introduces a new productive owner, confirm its canonical path and hash before writing code. When the classification is uncertain, pause and classify it with the owner.

An extension within an existing architectural decision, including a new public `Value` variant whose type has been migrated, can be specified in its owning L0. Obtain public-contract approval before implementation. A new ADR is warranted when the choice establishes a durable cross-component or public product policy and no existing decision settles the alternatives. Apply this distinction to new changes without rewriting the record of earlier approvals. [00_nucleo/prompts/core/entities/value.md](../prompts/core/entities/value.md) · [00_nucleo/prompts/infra/watch.md](../prompts/infra/watch.md)

## 3. Verification against executed steps (P965 Phase A)

| Step | Type | Applied mode | Correct? |
|---|---|---|---|
| P893/896/906/909/915/918/922/937 | contract (fields/methods) | stop | ✓ |
| P956 | default behavior + public signatures | stop | ✓ |
| P959 | contract (2 `MathConstants` fields) | stop | ✓ |
| P952 | contract (field + trait method) | **no stop — deviation** | ✗ (corrected by retrospective review) |
| P957 | internal formula | flow | ✓ |
| P958 | table + fallback arm (parity) | flow | ✓ |
| P961 | annotation style (parity) | flow | ✓ |
| P962 | wrapper registration (parity) | flow | ✓ |
| P963 | internal formula (`is_text_like`) | flow | ✓ |
| P964 | table (Greek code points) | flow | ✓ |

The table records the original criterion. Item numbering is preserved so existing L0 citations keep their meaning, and earlier approvals keep their recorded scope.

The criterion cleanly separated all executed cases; no boundary case remained uncovered. One boundary case was discussed and resolved in the wording: a parity correction that changes visible output (P958) appeared to conflict with “default behavior.” The distinction is **product intent vs. convergence with vanilla**: the former stops; the latter flows.

## 4. Collision check (actual scan of `00_nucleo/adr/`)

A scan for `paragem|confirma|gate|PARAR` found no existing ADR that decides when to stop after editing L0. ADR-0114 and ADR-0117 were investigation procedures at the time of that scan. ADR-0132 now governs evidence for maintained changes. The slot was confirmed by listing: the last file was `typst-adr-0126-*` → **free slot = 0127**.

## 5. Consequences

- **Positive:** the Golden Rule is no longer ambiguous; the criterion is verifiable (protected contract? default behavior? pipeline phase? supported use broken? — if none apply, flow continues).
- **Positive:** the P952 deviation is classified as a violation of item 1 and can be detected by inspecting protected contracts in the diff (a declared entity or integration trait changed without a recorded stop).
- **Cost of the amendment:** internal classification needs recorded consumer evidence. Entities without a declaration are classified progressively.
- **Neutral/cost:** when a change class is uncertain, the rule of thumb is to **stop** — an unnecessary stop costs a message; a missed stop costs retrospective review (the P952 lesson).
- **Operational:** record the stop in the step report (“gate: confirmed by the owner on <date>”), as already done in P956/P959.

## 6. References

- `lab/history/construction/records/00_nucleo/materialization/typst-passo-965.md` (statement + owner’s proposed criterion).
- `lab/results/construction/diagnostics/typst-passo-952-relatorio.md` §6 (the deviation that motivated formalization).
- [ADR-0132](adr-0132-evidence-for-maintained-changes.md) governs measurement, provenance and validation. [ADR-0129](adr-0129-l0-owners-and-shared-tekt-nuclei.md) governs ownership and lineage.
