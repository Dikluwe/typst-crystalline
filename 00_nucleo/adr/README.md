# Typst Crystalline architecture decision records

This directory is the canonical home of Typst Crystalline architecture
decision records (ADRs). English is the normative language for new ADRs and
for maintained translations of existing ADRs.

The complete active corpus, ordered by authority and subsystem, is in
[ORDER.md](ORDER.md).

## Stable organization

ADRs remain in one flat directory. Their stable identity is the ADR-NNNN
identifier, not their status and not their filename. Status changes therefore
never move a record between directories.

Canonical filenames use this form:

    adr-NNNN-short-english-title.md

The numbered records now use canonical English filenames. Their old-path map is
being applied across the repository as part of the mature-layout migration;
complete that incoming-reference update before considering path migration
closed. Do not create new Portuguese filenames. Stable identity remains the
ADR number, including the distinct ADR-0026 revision record.

The following files are not numbered decisions:

- README.md: this reference and governance document;
- ORDER.md: the complete authority-ordered active catalog;
- template-adr.md: the required structure for new decisions.

The former Portuguese reference, which mixed a current index with cumulative
construction history, is preserved unchanged at
`lab/results/construction/diagnostics/adr-reference-legacy-portuguese.md`.

## Canonical status vocabulary

Every ADR must use one of these English values in its primary Status field:

| Status | Meaning |
|---|---|
| PROPOSED | The decision is awaiting adoption or implementation. |
| ACCEPTED | The policy or architectural rule is in force. |
| IMPLEMENTED | The concrete technical decision is materialized. |
| DEFERRED | The decision is retained but intentionally postponed. |
| SUPERSEDED BY ADR-NNNN | A later ADR replaced the decision. |
| REVOKED | The decision is no longer valid and has no direct replacement. |

Translations preserve the meaning of the original status. They do not promote,
revoke, or otherwise reinterpret a decision. The primary field uses
`**Status:** VALUE` with the canonical value alone. Approval dates, qualifiers,
and previous transitions are separate historical metadata, not new status values.

Legacy primary values map without reclassification: `EM VIGOR` (including
an adopted addendum), `ACEITE`/`aceite`, and `in force` become `ACCEPTED`;
`IMPLEMENTADO` becomes `IMPLEMENTED`; `PROPOSTO` becomes `PROPOSED`.
Quoted historical statuses and archived originals are evidence and remain literal.

## Authority-ordered index

Numeric order records history; it does not express authority. Read and apply
ADRs in this order:

| Order | Scope | Catalog section |
|---|---|---|
| 1 | Constitutional constraints, parity and architectural method | ORDER.md §1 |
| 2 | Evidence and decision process | ORDER.md §2 |
| 3 | Core domain and representation | ORDER.md §3 |
| 4 | Compiler runtime and pipeline | ORDER.md §4 |
| 5 | Layout, infrastructure, and export | ORDER.md §5 |
| 6 | Shell, CLI, and diagnostics | ORDER.md §6 |
| 7 | Parsing and dependency decisions | ORDER.md §7 |
| 8 | Proposed or pending decisions | ORDER.md §8 |
| Historical | Archived records, not current authority | Archive manifest |

When two decisions conflict, the more specific current decision governs unless
a higher-priority constitutional ADR explicitly forbids it. A revoked or
superseded ADR never governs current work.

The cleanup classification, consolidation candidates, and archival map are
preserved at
`lab/results/construction/diagnostics/adr-cleanup-classification.md`; archived
record redirects are at
`lab/results/construction/diagnostics/adr-archive/redirects.tsv`. Numbered
identifiers are not reassigned: reordering the index communicates authority
without breaking stable historical citations.

## Canonical L0 references

ADR-0130 defines one prose identity format: the full repository-relative
path including `00_nucleo/prompts/`, for example
`00_nucleo/prompts/core/compiler/layout/mod.md`. Neither `layout.md` nor
`compiler/layout.md` is a canonical prompt identity. Relative Markdown link
targets are allowed for navigation only; their visible prompt-path labels
retain the canonical identity. Rust `@prompt` paths remain repository-relative
with the same prefix; no Rust metadata syntax is changed.

## Required structure

New ADRs use template-adr.md and contain, at minimum:

1. status, date, and decision owner;
2. measured context before classification or decision;
3. the normative decision;
4. affected L0 prompts, using full repository-relative paths including
   `00_nucleo/prompts/`, as required by ADR-0130;
5. consequences and rejected alternatives;
6. compatibility or supersession relationships;
7. verification evidence and references.

Historical annotations must not be appended indefinitely to the normative
decision. Current evidence summaries belong in `00_nucleo/diagnostics/`;
construction records and detailed measurements remain in the preserved history
and evidence bundles. Existing cumulative ADRs are translated faithfully first
and may be split only by a separately reviewed cleanup map.

## Language and literal-preservation rules

Translate prose to English, including headings and status labels. Preserve these
items literally unless a separate tooling or product decision changes them:

- ADR identifiers, dates, hashes, pins, measurements, and approval history;
- repository paths until their mapped rename is applied atomically;
- code identifiers, signatures, code blocks, command lines, and error messages;
- quoted observable language output;
- linter metadata tokens that are parsed literally.

Machine-parsed Portuguese metadata remains unchanged until the parser supports
an English alias. This exception is syntax compatibility, not normative prose.

## Migration state

The English migration is staged. ADR-0130 is the reviewed pilot; ADR-0018,
ADR-0029, ADR-0030, ADR-0032, ADR-0107, ADR-0108, ADR-0109, ADR-0115,
ADR-0119, ADR-0121, ADR-0125, ADR-0127, and ADR-0129 have also received English
prose revisions. Other ADR bodies remain untranslated or await clause-by-clause
review. Canonical English filenames are in place, while the repository-wide
incoming-reference audit remains part of the active migration. Primary statuses
are normalized independently of prose translation; historical wording is not
thereby declared translated. Do not claim that the full corpus is translated
until every numbered record and its incoming references have been validated.

Missing numeric identifiers are not errors. ADR-0056 and ADR-0063 are currently
unused. ADR-0026 has two historical records under the same numeric identity; the
revision relationship must be preserved during migration.

## Validation gates

For each migration batch:

1. compare every translated obligation, prohibition, exception, number, and
   refutation condition against the preserved Portuguese baseline;
2. verify that paths and inline-code literals are unchanged unless included in
   the reviewed rename map; expansion of a prose prompt reference to its full
   repository-relative identity under ADR-0130 is not a file rename, but must
   resolve to the same existing owner;
3. update every incoming reference when a file is renamed;
4. run git diff --check and the repository's documentation/architecture checks;
5. record the exact commit or uncommitted file set used for validation.

The migration coordinator and pilot evidence are preserved under
`lab/history/construction/records/00_nucleo/materialization/` and
`lab/results/construction/diagnostics/`, respectively. Current progress and
active evidence entry points belong in `00_nucleo/diagnostics/`.
