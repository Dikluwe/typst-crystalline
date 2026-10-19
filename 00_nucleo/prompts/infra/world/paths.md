# L0 — infra/system-world/paths
Hash do Código: dfebefdf

**Layer:** L3
**Exclusive consumer:** `03_infra/src/world/paths.rs`

## Current evidence

- `03_infra/src/world/paths.rs:19` — `root`.
- `03_infra/src/world/paths.rs:25` — `directory_of`.
- `03_infra/src/world/paths.rs:37` — `path_of`.
- `03_infra/src/world/paths.rs:41` — `realize_rooted`.
- `03_infra/src/world/paths.rs:56` — `resolve_physical_path`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/world/paths.rs`.

## Owned contract

- Own root/directory/path access, rooted realization, physical-path resolution, bibliography and read/include path helpers. Use the cache/package owners, not parallel state.
- Legacy absolute paths use the current package root or project root; relative paths use the current file's directory with existing root fallback for an unregistered id.
- Rooted paths preserve VirtualRoot::Project or full Package identity; lexical normalization stays in L1 DTOs. Unknown ids, bases outside root, escape and backslash keep their existing errors.
- Join the selected physical root and vpath with the existing prefix check. Do not add canonicalization or incidental sandbox hardening.
- Register canonical read/include ids and use shared caches. Preserve diagnostics including `searched at`, access denied and `include: ficheiro não encontrado`.
- Read bibliography via World, validate UTF-8 and delegate BibTeX parsing to L1, preserving error formatting and current-file relative base.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.
