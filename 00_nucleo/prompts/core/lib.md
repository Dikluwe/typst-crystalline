# L0 — Pure domain crate
Hash do Código: 2878f5de

**Layer:** L1
**Exclusive consumer:** `01_core/src/lib.rs`

## Evidence and classification

`01_core/src/lib.rs:7-17` declares compiler, contracts, entities, the test-only
verification module and internal utilities. The archived original describes an
initial migration subset rather than the current crate. This is structural
inspection, not a new language parity measurement.

## Contract

Own the pure Typst domain crate's module composition. Expose compiler, contracts
and entities with their existing public paths. Keep testing behind `#[cfg(test)]`
and utilities internal; their algorithms belong to their exclusive owners.

The original foundation inventory identifies FileId, SyntaxKind and Span/Spanned
as migrated syntax-domain entities. Their current contracts remain with their
entity owners, not this crate root. FileId remains an opaque handle; global
interning is infrastructure, and mutable global state is prohibited in L1.

External dependencies follow the current `[l1_allowed_external]` configuration
and applicable ADRs, including comemo and thiserror where authorized. The
archived exclusion of ecow and deferred SyntaxNode/PackageSpec/VirtualPath and
Source migrations described the initial construction state; it does not revoke
ADR-0024 or create current blockers. Those types and parse dependencies remain
under their own current owners. No dependency approval is inferred from this
module declaration contract.

## Acceptance and historical disposition

Preserve module visibility and test-only compilation boundaries, L1 purity,
exclusive ownership and lineage. Do not move logic into this root or change
public contracts, defaults, compatibility or phase ownership. Initial migration
sequencing and its dependency questions are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/lib.md`.
