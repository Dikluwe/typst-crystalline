# Prompt organization audit

Status: evidence for a migration proposal, not an L0 specification.

Revision, 2026-10-18: the original census remains historical evidence. The proposed
`00_spec/` destinations are withdrawn following a direct reading of the Tekt
architecture. Original report, plan, script, maps, and validation receipt are
preserved in [superseded-proposal.json.gz](superseded-proposal.json.gz).

Latest direction: the user explicitly requests cleanup in a new branch, retirement
of `materialization/` from the mature tree, and benchmarks/test-build artifacts in
lab. The active [plan](../../materialization/prompt-organization-2026-10-18/plan.md)
now defines that target while preserving the Tekt layer roots and owner prefix.
Branch `codex/tekt-mature-layout` was created without discarding working changes.
No layout migration or build-configuration change has been performed yet.
The previous plan and relevant configuration evidence are preserved in
[mature-layout-baseline.json.gz](mature-layout-baseline.json.gz).

The repository needs a consistent distinction between current contracts, shared
invariants, working procedures, and historical evidence. Translation and renaming
alone would leave the main source of disorder in place: current obligations and
session history accumulate in the same documents.

## Measurement and provenance

The inventory ran from `2026-10-18T20:41:33.124787+00:00` to
`2026-10-18T20:41:33.551061+00:00`, on branch `Tekt`, HEAD
`b7e7c83223e5daa67bbdf124c36b5b7f93200731`, with uncommitted changes.
The exact working-tree status and `git diff HEAD --stat`, individual source and
prompt SHA-256 values, configuration hash, and inventory-script hash are embedded
in [inventory.json.gz](inventory.json.gz). The existing Context implementation
and its specifications were inputs to this audit, not changes made by it.

The original `inventory.py`, preserved as data in the superseded-proposal archive,
scans Rust `@prompt` headers in L1–L4 and all files
under the current prompt root. The compressed
[ownership map](ownership-map.csv.gz) contains current owners, consumers, source
prompt hashes, and proposed destinations. Its destination column belongs to the
withdrawn proposal and must not be used as a migration instruction. The executable
script was removed from L0, where executable tools do not belong.

| Observation | Count | Interpretation |
|---|---:|---|
| Non-nucleus documents under the prompt root | 999 | Includes explicit non-materializing exceptions |
| Documents referenced by consumer headers | 991 | Candidate set for owner-by-owner consolidation |
| Documents without a direct consumer header | 8 | Requires classification, not automatic deletion |
| Nucleus files | 109 | Translation can invalidate dependent pins |
| Owned documents with Portuguese heading markers | 638 | Editorial review candidates |
| Owned documents with both English and Portuguese heading markers | 309 | Mixed-heading review candidates |
| Owned documents with dated headings | 475 | Review current obligations versus historical narrative |
| Owned documents with step identifiers in filenames | 84 | Naming review candidates |

Heading counts overlap. The keyword method is not a full language detector, a
percentage of translated content, a semantic audit, or a V15/V26 validation.
Dates and step names do not prove that a specification is obsolete. The proposed
owner destinations are unique for the measured owner set.

## Findings before recommendations

### Current contracts accumulate historical layers

`00_nucleo/prompts/compiler/realize/mod.md:113` and `:200` contain proposal/gate
headings, `:264` records ratification, `:272` explicitly marks a superseded section,
and `:292` states the replacement decision. Later sections contain current
obligations. This demonstrates a need to resolve the status of individual clauses;
it does not authorize removing all dated sections or treating every proposal
heading as an unresolved approval.

`00_nucleo/prompts/passo-537b-set-page-columns.md` still owns
`01_core/src/compiler/layout/set_page.rs`. A filename containing a step number
cannot be used as the archival criterion.

### Procedures and historical redirects share the owner namespace

The documents with no direct consumer header are:

- `00_nucleo/prompts/auditar-fatiamento.md`
- `00_nucleo/prompts/auditar-spec.md`
- `00_nucleo/prompts/compiler/show-regex.md`
- `00_nucleo/prompts/compiler/stdlib/text/regex.md`
- `00_nucleo/prompts/compiler/style/font-dict.md`
- `00_nucleo/prompts/infra/package_version_resolution.md`
- `00_nucleo/prompts/package-spec-dto.md`
- `00_nucleo/prompts/template-prompts.md`

These include workflows, a template, a historical redirect, and specifications
whose requirements need an ownership/status review. Configured exceptions exist
in `crystalline.toml:191`; absence of a header is not by itself a new violation.
For example, `compiler/stdlib/text/regex.md:1` identifies itself as a historical
redirect, whereas the package documents require a substantive review before
classification. Exceptions must be reconciled with current files and consumers.

### The active authoring instructions need reconciliation

`00_nucleo/prompts/template-prompts.md:5` asks for plural generated files, and
`:42` prescribes simultaneous code and test generation. The template must be
reviewed against current ownership and testing rules before it becomes the
English template. Translating it verbatim would perpetuate conflicting guidance.

`00_nucleo/prompts/auditar-spec.md:59` still instructs use of oracle pin
`9f2b6e8715237cb086899a42873660fe744622e8`. The current repository instruction
pins `bd5ebec285505889e8ff1d4f68804ab147a699f3`. This active procedural reference
needs correction during migration. Historical measurements against the older pin
must retain their original provenance and must not be relabeled as current tests.

### Naming is also a machine protocol

ADR-0130, `00_nucleo/adr/typst-adr-0130-referencias-canonicas-l0.md:20`, requires
the current canonical prefix `00_nucleo/prompts/`.
`tools/ci/check_reverse_lineage.py:29` enforces that prefix, and `:38–43` parses
the literal field `Hash do Código:`. `crystalline.toml:18` excludes diagnostics
by directory name. These are concrete dependencies, not a complete list of all
tools affected by a future root or metadata change.

Nucleus TOML includes English structural keys and Portuguese prose. Changing
prose changes bytes, so the migration must update pins and downstream lineage
as a complete dependency closure. An English heading is not automatically an
editorial-only change to the integrity graph.

### History preservation already has a foundation

The branch `tekt-historico` points to
`6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5`; its tip is also the merge base with
the audited `Tekt`. The recent organization inventory and cleanup reports in
`00_nucleo/diagnosticos/inventario-organizacao-2026-10-18/` describe indexing,
evidence preservation, and cache cleanup. They do not establish that the active
contracts were consolidated or that the previous branch split caused the present
disorder. A historical branch also cannot preserve current uncommitted files or
ignored evidence by itself.

## Recommendation and limits

Investigate causal integrity and context locality before choosing structural
changes. Keep current canonical paths, historical originals, and concise prompt
revision records. No new root or path-mirroring policy is recommended.

The [execution plan](../../materialization/prompt-organization-2026-10-18/plan.md)
records the user's language decision and the proposed sequence. This audit does
not establish full behavioral parity, global lint cleanliness, or a complete
classification of active ADRs, scripts, comments, and documents outside the prompt
root. Those are explicitly included in the plan's classification phase.

## Architecture reading and correction

The user-specified `/home/dikluwe/Documentos/Antigravity/Tekt` resolves to
`/repos/Antigravity/Tekt`. The inspected working tree is modified over HEAD
`35b9000631ddf118bade7223d8dc2877d9123fba`; [architecture-reading.json.gz](architecture-reading.json.gz)
records exact texts, hashes, status, diffstat, and UTC. This is reference evidence,
not automatic incorporation of an external working tree into Typst governance.

Observations preceding the revised direction:

- Tekt `MANIFESTO.md:33` makes L0 the causal origin of code. A source-path mirror
  may aid navigation but does not establish that causal relationship.
- Tekt `00_nucleo/adr/0005-evolucao-verificavel-e-contexto-local.md:39–52`
  requires evaluating responsibility and context across comparable changes. The
  first audit measured naming and headings, not that architectural property.
- Tekt `00_nucleo/README.md:56–67` requires prompt revision history;
  `:82` separates detailed evidence from the current contract. Removing history
  wholesale would contradict this distinction.
- Tekt `MANIFESTO.md:138` prohibits executable code in L0. The inventory script
  introduced by the previous response violated that boundary. Its removal corrects
  this planning work; it is not an audit of every existing executor under L0.
- `00_nucleo/materialization/tekt-english-reference-migration.md` already records
  the English migration, historical branch, accepted pilot, and remaining scope.
  The initial recommendation did not adequately reconcile this existing work.

These are confirmed deficiencies in the earlier proposal. The broader inference
that current Typst development has lost causal discipline requires per-unit
chronology, contract, and responsibility review. Dated appendices, mixed language,
and valid hashes cannot establish that alone.

The Tekt reference itself has points to reconcile: L2 and L4 README examples still
use `@spec 00_nucleo/specs/...` while the main protocol uses `@prompt`; ADR-0001's
differential `@base` chain needs explicit interpretation alongside ADR-0004's owner
bijection before local adoption. These observations do not justify replacing the
lattice or silently importing a template. ADR-0003 remains proposed, and LESSONS
contains candidates rather than a blanket set of accepted new requirements.
No external Tekt files were changed.

The architectural hypothesis remains testable: comparable changes must demonstrate
preserved intent and bounded relevant context, with costs and negative results
recorded. This review provides neither general superiority evidence nor a complete
semantic conformity verdict.
