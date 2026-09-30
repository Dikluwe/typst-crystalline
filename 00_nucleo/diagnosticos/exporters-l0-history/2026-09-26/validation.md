# Exporter L0 sanitization validation

## Current status — authorized reseal completed

The user subsequently authorized metadata-only Rust header updates and correction
of the wiring/pipeline nucleus declarations. At execution, those declarations were
already valid, so this task made no wiring/pipeline edits. All exporter owners were
resealed after a green global V15/V26 preflight. Only @prompt-hash source annotations
and reciprocal Hash do Código metadata changed; all 38 Rust bodies are byte-identical.
Global V5/V15/V26 now passes with no violations. No global formatting ran.

authorized-reseal.json records the before-state, official isolated seal, exact
metadata changes and body hashes. authorized-final-checks.json records the final
workspace validation and its exact provenance. The original blocked results below
are retained as historical observations, not current blockers.

Measured at 2026-09-26T23:46:51.584Z on HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, uncommitted working tree.
The exact dirty stat/inventory, commands and outputs are in seal-plan.json;
manifest.json records the before-state and byte hashes.

- Scope: 38 owner prompts across builder, stream, html, svg and images.
- Global V15/V26 preflight: exit 1; see exact findings in seal-plan.json.
- Isolated official reseal and V5/V15/V26 validation: all exit 0.
- Real scoped Rust sources retain their captured SHA-256 exactly; no implementation
  or header changes were applied. No global formatting or global hash repair ran.
- Required prompt code hashes were calculated by the official linter against copied
  consumers and are recorded in the plan. Real prompt metadata repair remains pending
  while global ownership/nucleus preflight is blocked; no existing seal is claimed
  valid for the translated prompt until both directions are updated together.

## Remaining authority boundary

V5 cannot be closed in the real workspace while all .rs files must remain unchanged:
translated prompt content changes effective prompt hashes, whose reciprocal values
are Rust @prompt-hash annotations. The 38 required annotation replacements
are recorded in seal-plan.json, not applied. The isolated green result proves that
these metadata-only replacements suffice for this scope; it is not a green result
for the real workspace. Ask for explicit permission before changing those headers.

The initial global V5 baseline also reported unrelated layout/mod.rs, layout/parbreak.rs
and layout/tests.rs drift. Later global V26 runs reported orphan format-option-precedence
and cli-observables nuclei introduced outside this batch. Those files/prompts are
outside this task; their exact outputs and timestamps are retained. Do not repair
them here or claim global V5/V26 is closed.

## Evidence and ownership limits

The complete pre-sanitization archive is byte-identical to the original prompts,
including trailing blank lines. Historical results are preserved, not rerun or
promoted to proof against the current vanilla pin. No Rust behavior, tests, defaults,
signatures or pipeline phases were changed. Parent prompts delegate to full canonical
child-owner paths; child contracts no longer rely on monolithic parent behavior.
Existing cross-consumer nuclei/pins remain unchanged except moving the builder bitmap
pin to its font owner; their shared non-English bytes are outside this owner-only
translation batch because external consumers also pin them.
