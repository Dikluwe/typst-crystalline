# L0 — StrongElem
Hash do Código: 2500101d

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/strong.rs`

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/entities/element-boundary.toml sha256:cafcd80a58c3e84a9b44a49eb92a93cd2422ddbcf530d295c3645ae54bf1e41b

## Evidence before specification

The consumer represents `*bold*` as its own semantic element with one body;
render style is owned by layout. Historical upstream `a51e02804` at
`model/strong.rs:21` declares Locatable and Tagged. The prior baseline
`2f42d64253547734564513a1159ee6b584c1c4b4` lacked the payload override; that
chronology is diagnostic history, not a current gate or fresh parity measurement.

## Owned contract

`StrongElem::new(body)` preserves Content. `plain_text` and `is_empty` delegate
to body. `map_content` and `map_text` recurse and reconstruct `Content::strong`,
preserving this semantic occurrence rather than creating additional wrappers.
`get_field("body")` returns Content; other fields return None.
`to_payload` returns `Some(ElementPayload::NativeElement)` for this occurrence,
even with empty body, not for a descendant's visual style. Preserve kind, fields,
constructor and existing trait signatures. Functional identity and occurrence
snapshots belong to compiler owners, not this entity.
The trait boundary is owned by
`00_nucleo/prompts/entities/elements/_comum.md` and shared invariants by the
pinned nucleus. Verify morphology, text/body transparency, recursive maps and
the own-occurrence payload. Language query/counter integration distinguishes
Emph, Strong and mere rendering styles; a unit constructor alone is not proof.
