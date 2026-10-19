# ADR-0129 — L0 owners and shared Tekt nuclei

**Status:** ACCEPTED
**Status history:** IN FORCE — APPROVED AT THE ADR-0127 GATE
**Date:** 2026-08-25
**Owner approval:** 2026-08-25
**Origin:** P1179–P1181

## Authority and provenance

This ADR locally adopts the normative decision:

```text
/repos/Antigravity/Tekt/00_nucleo/adr/0004-nucleos-tekt-compartilhados.md
SHA-256: 2ae6e859396442a3e46405987429c16393539e0c0357c968d023ced1503405c2
Observed Tekt HEAD: fc8449f748c66adf084fdcad3a969343807f7bbe
```

The pin above identifies the bytes that were actually audited. Incorporating this ADR into `typst-crystalline` does not mechanically import documents from the Tekt repository or transfer authority to future unpinned changes.

## Measured context

P1179 demonstrated that the scalar `Hash do Código` metadata cannot represent a prompt used by multiple consumers. The old repair tool successively wrote the same prompt, kept the hash of the last consumer, and could leave source and prompt partially updated while the second pass ended with zero remaining repairs.

The P1179 inventory measured, on historical HEAD `ce49041de76eb64c011e990e9774a0339f979211`, 22 shared prompts used by 107 consumers and 23 prompts without canonical metadata, affecting 78 consumers.

At `2026-08-25T21:02:41-03:00`, on HEAD `00f402e875956304aa435f749a251f359287e2ba`, the corrected Tekt Linter binary measured 24 global ownership collisions. The increase from the 22 historical cases reflects the current corpus and the corrected global analysis; the P1179 inventory remains historical evidence, not a closed list.

The corrected executable had SHA-256 `b1521dbdc85d0218dee60a9710bb1b1de011c6039fa2c6adf525b1f30e78d3cf`, built from `/repos/Antigravity/tekt-linter` source at commit `cc357d9c3bfa26a4ce1bd71c72bc9cba5b3b027c`. The complete suite for that source finished with exit 0. In the product:

- `--checks v15,v26 --fail-on warning` finished with exit 1 and listed the 24 V15 collisions; it found no V26 because the corpus did not yet contain Tekt nuclei;
- `--fix-hashes --dry-run .` finished with exit 2, blocked by the 24 collisions before any write;
- the working tree remained unchanged by both commands.

## Decision

### 1. Productive ownership is bijective

For all productive L1–L4 code:

```text
1 materializable L0 prompt ↔ 1 productive consumer
```

- each consumer declares exactly one owning `@prompt`;
- each materializable L0 prompt legitimizes exactly one consumer;
- Prompt→Code `1:N` is an ownership violation, not an alternative format;
- zero consumers means an orphan prompt, except for an explicit, non-materializable exception already declared in configuration;
- V15 blocks hash repair while the bijection is invalid.

A productive test file, build script, or entry point also needs its own owner when it is in a strict directory; it cannot share a module’s prompt merely because of functional proximity.

### 2. Normative sharing uses Tekt nuclei

Adopt **Tekt nucleus** as a declarative L0 artifact for sharing obligations, never ownership:

- location: `00_nucleo/prompts/_nuclei/**/*.toml`;
- format: strict TOML 1.0;
- required discriminators: `tekt = 1` and `kind = "nucleus"`;
- content: atomic `must`, `must-not`, or `may` claims;
- composition: dependencies among nuclei form a DAG only;
- consumption: one or more owning prompts reference the nucleus by logical path and complete effective SHA-256;
- cardinality: `1:N` prompts; zero consumers makes a nucleus orphaned and produces V26;
- code never uses a nucleus as `@prompt` or points directly to it;
- a nucleus contains no `Hash do Código`, productive owner, or consumer list;
- changing bytes or dependencies deterministically invalidates the pins and effective hashes of transitively consuming prompts.

A prompt may consume zero or more nuclei, but it must remain complete and readable within its owner scope. Identity, specific contract, consumer algorithm, and its own criteria cannot be delegated to a nucleus.

### 3. Document boundaries

- **L0 prompt:** ex-ante cause and owner of exactly one consumer;
- **Tekt nucleus:** declarative obligation shared by prompts;
- **ADR:** architectural decision and its historical rationale;
- **diagnostic:** evidence and ex-post state;
- **step:** temporary operational coordination;
- **report:** minimal record of what was executed.

ADRs, diagnostics, steps, and reports cannot be referenced as nuclei. TOML outside `_nuclei` does not acquire nucleus identity. The `.tekt` extension is unsupported.

### 4. Tool gates

- V15 validates global ownership before any repair;
- V26 validates schema, inventory, pins, DAG, dependencies, orphans, and the prohibition on code→nucleus references;
- V15 and V26 block `--fix-hashes`;
- preflight must finish completely before the first mutation;
- an application failure requires verifiable rollback and a nonzero exit code;
- `Nothing to fix` requires complete bidirectional consistency, not merely absence of direct header drift.

Corpus repair proceeds in manifested batches. Global repair is prohibited while ownership collisions or nucleus errors remain.

## Migration of the existing corpus

Each currently shared prompt must be measured together with all its consumers and classified before changes:

1. Mixed specific contracts → split into owning prompts;
2. Real owner with common claims → individualize owners and extract only the common claims into a nucleus;
3. Entirely transversal document → convert the normative core into a nucleus and create owning prompts for every consumer;
4. Incorrect documentary association → create or locate the correct owner and move the old document to its proper category.

There is no automatic Markdown→TOML conversion. Create a nucleus only when at least two owning prompts consume genuinely shared claims.

Missing canonical metadata is repaired only after ownership is resolved. Resealing before then would legitimize an invalid relationship.

## Consequences

### Positive

- `Hash do Código` regains a unique meaning;
- a change in one consumer does not choose or invalidate unrelated owners;
- shared invariants are no longer copied or hidden in monolithic prompts;
- V15 proves ownership and V26 proves the shared declarative graph;
- hash repair can be transactional and reproducible.

### Costs

- existing shared prompts require semantic audits;
- the corpus will gain additional owning prompts;
- nuclei require a schema, complete pins, a DAG, and transitive updates;
- changes to a nucleus deliberately affect every consuming prompt.

## Rejected alternatives

- multiple `Hash do Código` values in one prompt;
- an aggregate hash for multiple consumers;
- representative consumer or “last one wins” behavior;
- full duplication of the same contract across prompts;
- code pointing directly to a nucleus;
- an ADR as an executable normative dependency;
- automatic conversion of every shared prompt into a nucleus.

## Adoption rollback

If the pinned binary no longer proves V15/V26, complete preflight, or rollback, corpus repair must stop before further writes. Prompts already individualized must not be shared again: preserve the bijection and temporarily remove only the affected nucleus consumption, restoring affected prompts/pins from the last valid commit. A new tool version requires provenance, two dry runs, and an explicit decision before work resumes.

## ADR-0127 gate

This decision changes the project’s public lineage contract. The owner explicitly approved it on 2026-08-25. Following approval:

- this ADR is `IN FORCE`;
- V15 and V26 are explicitly included in project configuration;
- new owners and nuclei must follow the decided cardinalities;
- legacy-corpus repair proceeds only in the audited P1181 batches;
- approval of this ADR does not by itself authorize global repair, automatic prompt splitting, or functional changes.
