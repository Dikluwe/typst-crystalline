# Architecture decisions after the construction period

The review started at `d6f402b26815a6eaafbb9235e4a271d6cdd75771`.
The numbered corpus had 117 records, with 69 accepted, 44 implemented and
four proposed. Status reflects the decision's recorded history. The
reading order now identifies selected construction decisions separately.

| Finding | Action |
|---|---|
| ADR-0114 §2 and ADR-0117 §§1–3 require the same separate A.0 artifact. ADR-0065 adds a numbered inventory and ADR-0084/0085 add a phase sequence and filenames. | ADR-0132 retains concrete pre-decision evidence and provenance. The owner or a diagnostic can carry it according to the scope. |
| ADR-0080 §Scope allows a new module or public field without updating L0 in the construction workflow. | ADR-0132 supersedes the exception. ADR-0127 and ADR-0129 supply the present approval and ownership rules. |
| ADR-0017 §Decision defers evaluator migration in that construction step. The compiler now exposes `eval` in `01_core/src/compiler/mod.rs` and implements it in `01_core/src/compiler/eval/`. | The index treats the deferral as historical. The record and filename remain available. |
| ADR-0094 §§1 and 3 encode LOC limits and a research recipe from the gradient series. | ADR-0132 makes their scope explicit. A current limit or research need determines their use. |
| ADR-0124 prescribed a fixed four-wrapper checklist. | The [sublayout review](sublayout-coverage-2026-11-19.md) traces the current call paths. The revised ADR credits shared tests for common contracts and requires assertions at affected caller boundaries. |

See [ADR-0132](../adr/adr-0132-evidence-for-maintained-changes.md) and
the [decision index](../adr/ORDER.md).
