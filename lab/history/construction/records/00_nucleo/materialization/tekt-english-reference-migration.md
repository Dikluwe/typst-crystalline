# Tekt English reference migration

## Baseline and branches

The owner authorized this migration on 2026-09-25. The Portuguese baseline
is commit `6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5`, which closes P1503/P1504.
The local branch `tekt-historico` preserves that commit. The new working
branch retains the name `Tekt` and starts at the same commit. No remote
branches have been changed or pushed.

This document coordinates the migration; it is not an L0 prompt or an ADR.

## Authorized scope

- Translate architectural prose in `00_nucleo/prompts/` and ADRs to English.
- Organize the ADR reference without changing decision IDs, approval,
  status, supersession relationships, or historical provenance.
- Translate source comments and refresh lineage headers in L1–L4.
- Do not change executable code, identifiers, signatures, literals, tests,
  dependency configuration, product defaults, or pipeline ordering.
- Keep historical measurements as historical evidence, not current claims.

Translation and reference cleanup are separate operations. The pilot uses
faithful translation first. Moving evidence or changing document paths
requires an explicit source-to-destination map and a reference audit.
No historical document is deleted merely because it is in Portuguese.

## Local drafting and semantic review

The installed LM Studio model `hunyuan-mt-7b-8k-latest` provides local first
drafts. A reviewer compares every accepted clause with the Portuguese
baseline, checking obligations, prohibitions, exceptions, units, numbers,
ownership, acceptance criteria, and refutation conditions.

The first word-segmentation samples mistranslated `consumer` as “customer”,
CJK quotation marks as characters, and “static and exhaustive” as “static
and deterministic”. Those drafts are not authoritative. Review must restore
the original meaning; fluency alone is not an acceptance criterion.

Paths, code identifiers, hashes, pins, code examples, and observable error
messages remain literal. The current linter recognizes the metadata labels
`Hash do Código:` and `Núcleos Tekt:` literally; these are retained as machine
syntax until a separate tooling change supports English aliases. This
migration does not modify the linter.

## Pilot

The initial review batch is:

1. `00_nucleo/prompts/compiler/layout/word_segment.md`;
2. comments and lineage of `01_core/src/compiler/layout/word_segment.rs`;
3. `00_nucleo/adr/typst-adr-0130-referencias-canonicas-l0.md`.

The ADR ID and file path remain unchanged in this batch. The corpus-wide
ADR index and any path reorganization remain pending.

## Gates for each batch

1. Compare the translation with the preserved Portuguese baseline.
2. Inspect every source diff outside `00_nucleo`: only comments and lineage
   metadata may change. Do not translate literals inside tests or code.
3. Validate ownership and nucleus integrity before hash repair (V15/V26).
4. Refresh hashes and validate V5/V6/V15/V26; a hash dry-run must be clean.
5. Run the full linter and proportionate build/tests.
6. Record the exact baseline, modified files, checks, and remaining scope.

This is a staged migration, not a claim that the full corpus is already
translated or that the architecture or runtime performance has changed.

## ADR reference cleanup foundation

The first reference-only cleanup batch establishes the target organization
without renaming numbered ADRs:

1. ADRs remain in a flat directory and retain their numeric identity;
2. new canonical paths use adr-NNNN-short-english-title.md;
3. status changes do not move files;
4. README.md is a concise English governance reference;
5. template-adr.md is the English template for new decisions;
6. the former Portuguese README and template are preserved in
   00_nucleo/diagnosticos/ as historical evidence.

The corpus measured for this batch contains 129 numbered files and 128 unique
numeric identifiers. ADR-0026 has two historical records. ADR-0056 and
ADR-0063 are unused. The numbered ADR records contain 36,800 lines. The former
directory-wide total was 41,166 lines because it also included the 4,232-line
legacy README, the template, and the unnumbered helper record.

A repository-wide scan found 394 files outside 00_nucleo/adr/ containing ADR
path or filename references. For that reason, no numbered ADR path is renamed
until a complete reviewed source-to-destination map and atomic incoming-
reference update are available. A local model draft for all titles was
rejected: it ignored the TSV contract, retained Portuguese, introduced
Italian and Spanish words, and truncated at ADR-0116.

## Importance order and elimination policy

The owner authorized reordering ADRs by importance and eliminating records
that no longer belong in the canonical set. The implementation keeps numeric
identifiers stable and expresses importance through the canonical index.

The audited classification is recorded in
00_nucleo/diagnosticos/adr-cleanup-classification.md. It distinguishes
constitutional decisions, evidence/process rules, current subsystem
architecture, proposed work, and historical archive material.

The first proven duplicate was removed from the canonical directory:
adr-stub-vs-fallback.md was the drafting input for ADR-0113. It is preserved
recoverably at
00_nucleo/diagnosticos/adr-archive/adr-0113-draft-stub-vs-fallback-portuguese.md.

Revoked, superseded, roadmap, and misclassified records are not deleted merely
because they leave the active index. They move only after their incoming
references and any still-normative clauses have been audited.

## Cleanup completed before translation

The structural cleanup now precedes corpus translation:

- the canonical directory contains 116 numbered files;
- ORDER.md lists every active file exactly once;
- 14 records were moved to the diagnostic archive;
- redirects.tsv preserves the former canonical path of every moved record;
- six revoked or superseded decisions are historical only;
- five cumulative roadmap/process records and two process-guidance records are
  no longer presented as architectural authority;
- the ADR-0113 drafting duplicate is no longer canonical.

The archive is recoverable and no historical evidence was physically deleted.
The next stage may consolidate current overlapping decisions before translating
their final normative form.
