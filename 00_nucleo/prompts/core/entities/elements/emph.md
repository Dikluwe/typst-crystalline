# L0 — EmphElem
Hash do Código: fdb9d779

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/emph.rs`

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/map-content-change.toml sha256:ec32b8f89dc95f1de7376ca91e6252eb61a5da2ac0326abb7474fe56fd27fd7a
- 00_nucleo/prompts/_nuclei/entities/element-boundary.toml sha256:cafcd80a58c3e84a9b44a49eb92a93cd2422ddbcf530d295c3645ae54bf1e41b

## Evidence before specification

The consumer represents `_italic_` as its own semantic element with one body;
render style is owned by layout. Historical upstream `a51e02804` at
`model/emph.rs:26` declares Locatable and Tagged. The prior baseline
`2f42d64253547734564513a1159ee6b584c1c4b4` lacked the payload override; that
chronology is diagnostic history, not a current gate or fresh parity measurement.

## Owned contract

`EmphElem::new(body)` preserves Content. `plain_text` and `is_empty` delegate
to body. `map_content` and `map_text` recurse and reconstruct `Content::emph`,
preserving this semantic occurrence rather than creating additional wrappers.
`get_field("body")` returns Content; other fields return None.
`to_payload` returns `Some(ElementPayload::NativeElement)` for this occurrence,
even with empty body, not for a descendant's visual style. Preserve kind, fields,
constructor and existing trait signatures. Functional identity and occurrence
snapshots belong to compiler owners, not this entity.
The trait boundary is owned by
`00_nucleo/prompts/core/entities/elements/mod.md` and shared invariants by the
pinned nucleus. Verify morphology, text/body transparency, recursive maps and
the own-occurrence payload. Language query/counter integration distinguishes
Emph, Strong and mere rendering styles; a unit constructor alone is not proof.

## Reuso de contêineres sem substituição — 2026-10-10

Medição anterior à decisão: `lab/results/construction/diagnostics/content-container-reuse-2026-10-10/before.json.gz` captura HEAD/working tree/UTC/fontes completas/owners/release. `01_core/src/entities/elements/emph.rs:45` conserva a travessia vigente e materializa o elemento mesmo quando os descendentes não mudam. `01_core/src/entities/content/traversal.rs:111–192` embala os resultados destes owners em Some incondicionalmente. Isso é custo mecânico; a linguagem e a identidade semântica dos elementos são preservadas. Inferência refutável: propagar a ausência explícita de mudança reduz alocações e retenção temporária; as sondas e os controles de tempo podem refutar o ganho.

Refinamento interno ADR-0127: disponibilizar somente helper crate-private map_content_if_changed nos elementos já owned por este consumer. Retornar None apenas se nenhum filho efetivamente visitado foi substituído; terminais sem filhos retornam None. Transformar filhos na mesma ordem do map_content vigente, uma única vez, e interromper no primeiro erro. Some do callback, inclusive conteúdo igual, sempre é mudança. Preservar todos os campos, valores, presença explícita, slots omitidos/none/presentes, flags, spans e metadados; não usar igualdade/hash/endereço como certificado. Vetores/linhas só alocam após o primeiro filho/linha alterado; os irmãos conservam seu conteúdo compartilhado. Assinaturas públicas e métodos de trait ficam iguais, com adaptação interna à nova prova; map_content_pre e map_text mantêm seu contrato anterior. Nenhuma ratificação de contratos históricos propostos é ampliada por este refinamento.

A lógica específica permanece neste owner; o match exaustivo em Content apenas delega estaticamente. Sem vtable, enum novo, campos novos, estado global, I/O ou fase nova. Aceitação: RED→GREEN de reuso, ordem, erro, substituição igual, opcionais/slots e metadados; suite core, builds e lint, qualificação de texto/morfologia/render/query contra release anterior e vanilla pinado; alocações e tempos pareados com proveniência. Não alegar ganho universal a partir de casos isolados.


A prova compartilhada de ausência de mudança obedece às claims desse núcleo. O algoritmo, os campos transformados, sua ordem e a preservação específica permanecem owned por este prompt e seu único consumer.
