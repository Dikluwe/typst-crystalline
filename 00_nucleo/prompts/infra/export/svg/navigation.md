# L0 — svg/navigation
Hash do Código: ddf76185

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/svg/navigation.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/export/svg-destination-context.toml sha256:13cad5ab1322bad4c569eec2aaf544452530cb972c3421130d0b9cb127170ef1

## Consumer evidence

`03_infra/src/export/svg/navigation.rs:17` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Page-local destination context

Own SvgDestinationContext/SvgDestination, label normalization, lookup, destination
nodes and render_link. Consume the pinned destination-context nucleus rather than
duplicating its shared page-local registry/routing obligations in the façade.
The immutable context performs no layout/filesystem lookup and carries no filenames.

Emit one local node per present destination at its supplied position and bind the
link edge using the same collision-free identity. Missing/cross-page destinations
without an explicit route preserve visual children and remain Unknown, not dangling
fragments. External URLs retain their target. Preserve hit area, escaping, nested
children and transforms. Caller-owned bundle/cross-page filenames are not inferred.
SvgDestinationContext constructors and normalization belong here. SvgOptions and
export wrapper defaults remain in `00_nucleo/prompts/infra/export/svg.md`.

## Verification

Check same-page closure, deterministic IDs, missing/cross-page fallback, URL targets,
escaped labels, nesting and hit-area transforms.
