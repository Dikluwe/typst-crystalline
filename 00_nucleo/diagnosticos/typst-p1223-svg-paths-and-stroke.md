# P1223 — paths SVG gerais e primeiro gap de stroke complexo

Execução iniciada em `2026-08-26T17:16:47-03:00`, HEAD
`0a0fabe05cd001802c6dfcfffb183f7651ab17d1`, working tree não commitado com
P1222 acumulado. O `git diff HEAD --stat` inicial tinha 6 ficheiros rastreados,
275 inserções e 39 remoções; os artefatos P1222 e os passos 1222/1223 estavam
untracked. Vanilla ratificado: `a51e02804`, binário SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.

O controle watch foi repetido isoladamente três vezes e passou em 2.18s,
2.29s e 2.12s. Nesta execução o controle é 3/3 GREEN; isso não apaga a
instabilidade observada dentro da suíte integral em P1222.

A lente DSM comparou `lab/typst-original` com o cristalino usando o mapa
`typst-correspondencias-v1.toml` e terminou sem erro de mapa ou cardinalidade.
O JSON bruto foi preservado durante a execução em `/tmp/p1223-lente.json`.

## Medição que decide o gate

O vanilla ratificado representa `Stroke` com `paint`, `thickness`, `cap`,
`join`, `dash` e `miter_limit` em
`lab/typst-original/crates/typst-library/src/visualize/stroke.rs:54-66`; seu
constructor lê os quatro campos complexos em `:236-243`. O cristalino contém
somente `paint`, `thickness` e `overhang` em
`01_core/src/entities/geometry.rs:32-44`, e a allow-list de
`native_stroke` rejeita qualquer outro nome em
`01_core/src/compiler/stdlib/layout.rs:2333-2339`.

As sondas públicas confirmaram a consequência: `cap: "round"`,
`join: "bevel"`, `dash: "dashed"` e `miter-limit: 2` têm exit 0 e valor
representável no vanilla; as quatro têm exit 1 no cristalino como argumento
inesperado. Portanto o primeiro gap causal não está no serializador SVG: a
informação é rejeitada antes de existir na entidade ou no `FrameItem`.

## Veredito

O Prompt L0 proprietário `00_nucleo/prompts/entities/geometry.md` foi ampliado
com o contrato proposto, defaults e critérios RED. Isso adiciona campos a uma
entidade pública interna compartilhada e cai exatamente no gate ADR-0127
previsto pelo próprio passo. A execução para aqui: nenhum campo, parser,
layout, exportador ou mapa DSM foi materializado/readjudicado como fechado.

Consequentemente, paths `A/S/Q/T`, normalização normativa de arcos e o corpus
de 25 mutantes continuam abertos. Selá-los antes de resolver ou autorizar a
fronteira de `Stroke` não mudaria o primeiro gap produtivo localizado e não
autoriza alegação de paridade adicional.

Resultado: `SVG GENERAL PATHS NOT SEALED — FIRST COMPLEX-STROKE GAP GATED`.

EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO.
