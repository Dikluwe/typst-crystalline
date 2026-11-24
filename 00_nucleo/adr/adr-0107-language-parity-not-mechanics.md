# ADR-0107 — Language parity and deliberate extensions

**Status:** ACCEPTED
**Date:** 2026-06-17
**Consolidated:** 2026-11-24

## Evidence

The original content-equality investigation used a heading whose rendered text was bold. Its body compared differently from plain text when the comparison inspected resolved rendering properties. The language can distinguish semantic markup, such as explicit strong emphasis, while treating the body of that heading as the text it contains. The equality and content owners define the current cases. [00_nucleo/prompts/core/compiler/eval/operators/equality.md](../prompts/core/compiler/eval/operators/equality.md) · [00_nucleo/prompts/core/entities/content.md](../prompts/core/entities/content.md)

The repository pins an upstream commit, tree and local executable hash. CI builds the pinned source and records its own executable identity. A version string alone can identify multiple source states. Earlier measurements retain their original pins. [Oracle manifest](../../lab/parity/vanilla.json) · [CI build](../../.github/workflows/ci.yml) · [Reference resynchronization](../diagnostics/vanilla-reference-2026-11-19.md)

## Decision

Parity covers Typst syntax, semantics and morphology. Morphology includes the text, markup and semantic styling of a language object. Observable evaluation, errors, order, public fields and rendered geometry must be tested at the boundary the change affects. Typst numeric equality compares an integer and a float by language rules, so `1 == 1.0` is true. Rust representation, structural equality, cache design, pass count and binary serialization can differ. Each implementation still satisfies its owning L0, purity and dependency contracts.

Before introducing user syntax, an argument or a language feature, run a minimal example against the pinned reference and inspect its semantics. If upstream rejects the proposed feature, retain it only under a deliberate product decision documenting the user benefit and portability consequence. Classify accidental extensions for correction and apply ADR-0127 to their public impact. Use the pinned source and executable together when intent or behavior is ambiguous.

Distinguish controlled comparisons from production-default tests. For a controlled geometry comparison, explicitly match the font and other sensitive settings such as page size, margins and language in both inputs. For a production-default comparison, use each compiler's actual defaults and measure which setting explains an observed difference. Record which question the test answers.

A feature counts as supported only where a real consumer produces the claimed observable result. Acceptance by a constructor, storage of a field, and painting or publication are distinct claims. Identify partial support, transparent stubs and approximations in their owners. A transparent stub can preserve an input or produce empty output while a capability is missing. A software fallback requires proven equivalence or an explicitly bounded approximation with its limits and completion condition. Exercise the working and affected paths when activating deferred behavior. The language contract does not impose a general requirement to preserve incidental artifact bytes. The PDF validation contract is specific to its output modes. [ADR-0126](adr-0126-verbose-pdf-default-compact-option.md)

## Boundaries and verification

Use observed coordinates, baselines and font metrics to evaluate typographic geometry. The general proposal in ADR-0123 to copy formulas literally has not been adopted. The proposed upstream correspondence marker in ADR-0110 adds no required field to current L0s. The deliberate finite-result restrictions of ADR-0101 remain subject to a separate decision about compatibility and per-function behavior. [00_nucleo/prompts/_nuclei/math/layout-observables.toml](../prompts/_nuclei/math/layout-observables.toml) · [00_nucleo/prompts/core/compiler/stdlib/calc/logarithms.md](../prompts/core/compiler/stdlib/calc/logarithms.md)

This decision carries forward the language boundary in ADR-0025 and ADR-0033, the oracle role in ADR-0075, comparison controls in ADR-0122, portability in ADR-0125, and the observable-consumption criteria in ADR-0054, ADR-0082, ADR-0099 and ADR-0113. ADR-0132 governs evidence, provenance and how to verify a causal explanation.
