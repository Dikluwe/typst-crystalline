# ADR cleanup classification

## Provenance

Measured on 2026-09-25 in an uncommitted working tree based on HEAD
6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5. The tree already contained the
English migration pilot. This classification does not change product behavior,
public contracts, pipeline phases, or L0 ownership.

The audit found:

- 129 numbered ADR files;
- 128 unique numeric identities because ADR-0026 has a revision record;
- 36,800 lines in numbered ADRs;
- 394 files outside 00_nucleo/adr/ containing an ADR filename or path;
- two unused identifiers: ADR-0056 and ADR-0063.

Importance is represented by the canonical index order, not by renumbering.
Existing identifiers remain stable citations.

## Priority 1 — constitutional decisions

These decisions constrain most future work and must appear first in the index:

1. ADR-0129 — one L0 owner per productive consumer and shared Tekt nuclei;
2. ADR-0130 — canonical full-path L0 references;
3. ADR-0127 — mandatory L0 stop versus continuous-flow changes;
4. ADR-0029 — physical purity of L1;
5. ADR-0030 — memory performance belongs to L1 domain design;
6. ADR-0018 — external dependency authorization under L1 purity;
7. ADR-0032 — unsafe policy in L1;
8. ADR-0107 — parity with the language rather than Rust mechanics;
9. ADR-0108 — measure before deciding;
10. ADR-0109 — atomization and its boundary from decoupling;
11. ADR-0037 and ADR-0100 — cohesion by domain across L1 and L3;
12. ADR-0104 — agent atomicity as an architectural force;
13. ADR-0119 — verification rather than trust;
14. ADR-0121 — measurement provenance;
15. ADR-0125 — implementation difference versus language difference.

## Priority 2 — evidence and decision process

These remain normative but are subordinate to the constitutional decisions:

- ADR-0034 — diagnosis before materializing a vanilla type;
- ADR-0065 — inventory before non-trivial decisions;
- ADR-0084 — conditional audit before B1/B2/B3 classification;
- ADR-0085 — immutable diagnostics;
- ADR-0114 and ADR-0117 — probe before specification and its mechanism;
- ADR-0122 — parity defaults in comparison tests;
- ADR-0124 — propagation checks across sub-layouts;
- ADR-0093 and ADR-0094 — ADR evolution and specification operations;
- ADR-0098 — single source of truth;
- ADR-0113 — transparent stub versus software fallback.

ADR-0114 and ADR-0117 are a strong consolidation candidate: the latter is an
operational expansion of the former, not an independent architectural choice.

## Priority 3 — current system architecture

This tier contains current domain, pipeline, representation, CLI, export, and
runtime choices. It is indexed by subsystem rather than chronology:

- domain and values: ADR-0024 through ADR-0026-R1, ADR-0052, ADR-0053,
  ADR-0058, ADR-0059, ADR-0101 through ADR-0106, ADR-0112;
- evaluation and introspection: ADR-0038 through ADR-0044, ADR-0067 through
  ADR-0074, ADR-0099, ADR-0118;
- layout and typography: ADR-0054, ADR-0055, ADR-0057, ADR-0078, ADR-0120,
  ADR-0123;
- shell and diagnostics: ADR-0045 through ADR-0051;
- infrastructure and export: ADR-0019, ADR-0020, ADR-0022, ADR-0027,
  ADR-0075, ADR-0083 through ADR-0092, ADR-0095, ADR-0115, ADR-0116,
  ADR-0126, ADR-0128;
- parsing and external crates: ADR-0005, ADR-0006, ADR-0008 through ADR-0017,
  ADR-0021, ADR-0023, ADR-0031, ADR-0035, ADR-0062, ADR-0077, ADR-0111.

## Remove from the canonical ADR set

### Proven duplicate — moved now

The unnumbered adr-stub-vs-fallback.md was the drafting input for ADR-0113.
It has been removed from 00_nucleo/adr/ and preserved at
00_nucleo/diagnosticos/adr-archive/adr-0113-draft-stub-vs-fallback-portuguese.md.

### Revoked or superseded — archive after reference audit

These records are useful history but no longer normative:

- ADR-0002;
- ADR-0003;
- ADR-0007, superseded by ADR-0018;
- ADR-0028, superseded by ADR-0029;
- ADR-0066, superseded by ADR-0073;
- ADR-0090, superseded by ADR-0092.

They should move to 00_nucleo/diagnosticos/adr-archive/superseded/ with a
machine-readable redirect map. They must not remain in the active index.

### Misclassified process or roadmap documents — relocate

These files mainly coordinate work or record cumulative completion and should
not remain normative ADRs:

- ADR-0004 — Step 1 discoveries;
- ADR-0060 — model roadmap;
- ADR-0061 — layout phase roadmap;
- ADR-0079 — layout phase 5 roadmap;
- ADR-0081 — M7+ restructuring scope.

Their still-valid decisions must first be extracted into the owning L0 prompt
or a concise ADR. The cumulative bodies then move to materialization or
diagnostics. No source file is deleted until that extraction is verified.

### Likely process guidance — review for relocation

These are not automatically removed because they may still contain binding
cross-cutting rules:

- ADR-0095 — reusable resource deduplication pattern;
- ADR-0096 — consumer-pending field pattern;
- ADR-0097 — legitimate no-go result from Phase A.

ADR-0095 may remain a technical ADR. ADR-0096 and ADR-0097 are stronger
candidates for process guidance under materialization.

## Consolidation candidates

Consolidation creates one current English ADR and archives the source records:

1. ADR-0026 plus ADR-0026-R1 — one current Content representation decision;
2. ADR-0037 plus ADR-0100 — one cohesion-by-domain policy for all layers;
3. ADR-0046 through ADR-0051 — one current CLI ownership and behavior ADR;
4. ADR-0087, ADR-0088, ADR-0089, ADR-0091, and ADR-0092 — one current
   gradient/export strategy ADR;
5. ADR-0114 plus ADR-0117 — one probe-before-specification policy;
6. ADR-0107 plus ADR-0125, with ADR-0123 kept proposed — one language parity
   boundary, without silently accepting the proposed typography extension;
7. crate-authorization records — one current external-dependency policy plus a
   registry, while preserving decision-specific rationale in the archive.

Consolidation must preserve supersession history and must not change the
substance of an accepted decision.

## Execution order

1. Publish the authority-ordered English index.
2. Build a complete old-path to archive/current-path map.
3. Extract normative clauses from roadmap and cumulative records.
4. Consolidate only after clause-level comparison.
5. Update incoming references atomically.
6. Run git diff --check and crystalline-lint after every batch.
7. Delete bytes only for proven duplicates; archive all other historical
   evidence.

This classification intentionally avoids renumbering. Renumbering would make
the 394 incoming-reference sites ambiguous and would destroy stable historical
citations without improving authority ordering.

## Cleanup execution result

The first cleanup pass is complete:

- 116 numbered files remain in the canonical ADR directory;
- ORDER.md lists all 116 exactly once;
- 14 records are preserved in the diagnostic archive;
- the archive consists of one proven duplicate draft, six revoked or
  superseded decisions, five roadmap/process records, and two process-guidance
  records;
- historical materialization documents retain their original paths;
- redirects.tsv maps every removed canonical path to its preserved location.

ADR-0026 and ADR-0026-R1 remain active until a single consolidated decision is
written. ADR-0114 and ADR-0117 also remain active because the operational ADR
depends on the conceptual ADR rather than restating it completely. They will be
consolidated during the semantic rewrite, not removed prematurely.
