# ADR-0131 — Owner aligned prompt paths

**Status:** ACCEPTED
**Date:** 2026-10-18
**Approval:** The repository owner explicitly authorized this organization and
execution on the existing branch, with semantic filenames, preserved history,
delegated review, and path, lineage and test validation.

## Evidence

The audit at commit `d330c717eb06c86a560c4cf3d8d0ba40224609b9` identified 991
exclusive owner/consumer pairs, 84 step-named L0 files, and 85 step-named Rust
files. Layer and functional categories did not always agree: shell-named
prompts owned infrastructure files, while an infra-named prompt owned pure
BibTeX evaluation. A deterministic layer/module mapping, without a leading
`src/`, had no duplicate destinations. The inventory and content review are
preserved under `lab/results/prompt-organization-audit-2026-10-18/`.

## Decision

Derive each materializable L0 path from its sole consumer:

1. Replace `01_core`, `02_shell`, `03_infra`, or `04_wiring` with
   `00_nucleo/prompts/core`, `00_nucleo/prompts/shell`,
   `00_nucleo/prompts/infra`, or `00_nucleo/prompts/wiring` respectively.
2. Remove only the immediately following `src/` component, if present.
3. Preserve the remaining module path and basename, replacing `.rs` with `.md`.

Examples:

- `01_core/src/compiler/layout/mod.rs` owns
  `00_nucleo/prompts/core/compiler/layout/mod.md`.
- `01_core/tests/outline_entry_indented.rs` owns
  `00_nucleo/prompts/core/tests/outline_entry_indented.md`.
- `02_shell/build.rs` owns `00_nucleo/prompts/shell/build.md`.
- `03_infra/tests/support/wiring/format_pdf.rs` owns
  `00_nucleo/prompts/infra/tests/support/wiring/format_pdf.md`.

Keep `tests/`, internal test module paths, and the basenames `mod`, `lib`,
`main`, and `build` when they describe actual consumers. Use the source's
snake_case basename for its Markdown owner. Filenames of active owning Rust
files and prompts describe responsibility, not numbered execution steps.
Explicitly excluded source files keep their classification; a rename does not
invent an owner or silently remove an exclusion.

Shared invariants remain in `00_nucleo/prompts/_nuclei/`, outside this mapping.
They have no direct source owner. Preserve the ADR-0129 bijection and pinned
nucleus graph, and the ADR-0130 canonical repository-relative reference format.
This decision supplies the subsequent relocation authorization; ADR-0130's
earlier reference-only scope is not a permanent ban on relocation.

## Migration and preservation

Update L0 before source references. Move files according to an explicit
old/new manifest, then update `@prompt`, module declarations, `#[path]`, links,
canonical citations and operational test targets. Reseal only after ownership
and nucleus validation. Check both directions of lineage and resolve the
final paths in an isolated versioned snapshot.

Preserve construction records, historical evidence and fixture bytes.
Numbered test function names, inline test-module identifiers and provenance
asset names are not file-layout decisions; this migration does not rewrite
them. No public signature, default behavior, pipeline phase or compatibility
contract changes as a consequence of this directory convention.

Do not merge owners or split productive modules to fit a visual taxonomy.
Semantic consolidation of cumulative or outdated L0 clauses is separate from
path migration and must retain current obligations, partial supersessions,
fallbacks, approvals and unresolved scope. Moving a prompt does not certify
that its entire prose has been consolidated or translated.

## Enforcement

The layout gate derives the expected owner from each non-excluded consumer,
checks naming and path uniqueness, and verifies references. V1/V5/V15/V26 and
reverse seals validate lineage. Compile all targets and run the product and
tool tests; compare full lint with the recorded pre-migration baseline rather
than hiding existing failures behind a successful path check.
