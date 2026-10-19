# ADR-0130 — Canonical L0 prompt references

**Status:** ACCEPTED
**Layer:** L0 — Governance and Document Traceability
**Created on:** 2026-08-17 (Step 1062)
**Decision owner:** Repository owner
**Approval:** ADR-0127 — owner approval of Option (b), explicit full paths, in Step 1062.

## 1. Context

Different components can legitimately share a leaf filename. A basename
therefore does not uniquely identify an L0 owner. Preserve the existing
prompt-to-consumer ownership rules and directory structure; disambiguate
references rather than renaming prompts or introducing duplicate owners.

## 2. Canonical references in Markdown prose

Every reference or citation to an L0 prompt in a Markdown document must use
its full **repository-relative path**, including the literal prefix
`00_nucleo/prompts/`. Use forward slashes and preserve the actual path and
filename. This is the single canonical prose-reference format.

Existing examples:

- `00_nucleo/prompts/core/compiler/layout/mod.md`
- `00_nucleo/prompts/core/compiler/eval/mod.md`
- `00_nucleo/prompts/core/entities/content.md`
- `00_nucleo/prompts/infra/export/stream.md`

Do not use `layout.md`, `compiler/layout.md`, an absolute filesystem path,
or a document-relative abbreviation as the prompt's identity. A Markdown
link may use a relative target for navigation, but its visible prompt-path
label must retain the full repository-relative identity.

This reconciles prose with the explicit repository-root convention already
used in source headers. It does not rename or relocate a prompt, alter its
hash, change its ownership, or authorize changes to its contract.

The subsequent [ADR-0131](adr-0131-owner-aligned-prompt-paths.md) authorizes the
layer/module directory convention and its path migration. The canonical
repository-relative reference format defined here continues to apply.

## 3. Rust provenance annotations (L1–L4)

Every Rust source file generated or maintained from an L0 prompt declares
its owner at the top of the module. The existing `@prompt` syntax remains
unchanged: the path is relative to the repository root and includes
`00_nucleo/prompts/`. It is not relative to the Rust file or the prompts root.

```rust
//! @prompt 00_nucleo/prompts/<subpasta>/<nome_do_prompt>.md
//! @prompt-hash <hash_crc32_ou_sha>
```

Existing source-to-owner examples:

- `01_core/src/contracts/world.rs` → `//! @prompt 00_nucleo/prompts/core/contracts/world.md`
- `01_core/src/compiler/layout/mod.rs` → `//! @prompt 00_nucleo/prompts/core/compiler/layout/mod.md`
- `01_core/src/entities/ast/code.rs` → `//! @prompt 00_nucleo/prompts/core/entities/ast/code.md`

The annotation points to the consumer's owning prompt, not a shared Tekt
nucleus; ADR-0129 governs ownership and nucleus pins.

## 4. Verification and historical evidence

Check that each example resolves to an existing file and each source example
names its actual `@prompt` owner. Check that the governance reference and ADR
template use the same prose convention. No source-header migration is needed.

The Step 1060 inventory and the historical explanation formerly embedded
here are preserved in
[the ADR-0130 historical diagnostic](../../lab/results/construction/diagnostics/adr-0130-step-1060-history.md).
They are retrospective evidence, not a normative inventory of the current tree.
