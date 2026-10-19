# L0 — Context block carrier
Hash do Código: 7673023b

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/context_block.rs`

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/context/block-semantics.toml sha256:22f48ae879819be41fc5490d426bd4611fe7ffecbdd9bad20cb0d9ddcea00717

## Contract


`ContextBlockElem` preserves public `id: u64` and `closure: Func`. Clone
preserves both; equality and hash use the id. It returns ContextBlock kind
and a payload carrying the id, preserves itself through child/text maps and
has no own plain text. Native construction and expansion are separate owners.


The trait API/defaults belong to
`00_nucleo/prompts/core/entities/elements/mod.md`. This owner specifies only
this element consumer; language constructors and layout algorithms remain
with their respective owners. Historical inline-arm evidence is retained in
`lab/results/construction/diagnostics/sanitization-entities-contracts-history.md`.
