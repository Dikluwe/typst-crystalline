# P1249 — dados de glifo na fronteira SVG

**Classificação saneada:** `L3 TRANSPORT-GAP / FONTLESS PRESERVED-SCOPE-OUT`.

O carrier já contém posição, id relativo, advance, tamanho, `base_char` e
`style.fill`. A pipeline já resolve a fonte exata e inclui os bytes; somente a
associação per-glyph não atravessa para o exporter SVG. Texto sem fontes é
scope-out explícito, não parte deste gap.

A arquitetura aprovada adiciona contexto L3 imutável derivado pela pipeline e
consumido pelo SVG, sem alterar `FrameItem`. A engenharia foi refinada para
mapear chave normalizada ao `FontKey` completo, ficando independente da ordem
da coleção. Sondagem por `glyph_id`, render do `base_char` e fallback textual
inventado são rejeitados. Nenhum candidato foi executado. Veredito:
`L0_WRITTEN_ARCHITECTURE_APPROVED_AWAITING_SEGREGATED_PRESEAL`.
