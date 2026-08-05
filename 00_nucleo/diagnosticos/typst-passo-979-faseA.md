# Passo 979 — Fase A: agrupamento de runs de texto num único `BT…ET` (PARADO no gate, ADR-0127)

**Data:** 2026-08-05 · **Estado da árvore:** HEAD = `b31913ff7` (P978),
working tree limpa à entrada. Nenhum código de produção alterado. L0
editado e resselado (`stream.md` §P979, `stream.rs` → `6308467e` — só
hash), linter limpo (0 violations; só o V7 órfão pré-existente).

## A regra do vanilla (lida e confirmada)

- O vanilla emite **um `BT…ET` por `TextItem`**
  (`lab/typst-original/crates/typst-pdf/src/text.rs:50-57` — um
  `draw_glyphs` por item) e o krilla abre um `begin_text()` por chamada
  (`lab/krilla-reference/crates/krilla/src/content.rs:626-700`).
- Os TextItems vêm do shaping de linha do typst-layout: **uma linha de
  prosa com estilo uniforme = um TextItem** (muitos glifos, espaços
  inter-palavra como ajustes `TJ`). Partes: mudança de fonte/estilo/cor/
  transformação ou fronteira de shaping.
- Resposta à Fase A.2: a fusão não atravessa mudanças de estado gráfico;
  dentro do bloco, ajustes finos vão no array `TJ` (kerning/espaços).

## Cristalino actual

Um `BT…ET` por item (`stream.rs`, envelope verbose P956): em prosa, um
por palavra+espaço; em math, um por run/glifo.

## Medição do ganho (Fase A.4)

| documento | BT cristalino | BT vanilla | stream cris. | stream van. |
|---|---|---|---|---|
| 02-lorem (prosa) | **503** | **36** | 82 486 B | 30 816 B |
| 30 secções (math) | 2074 | 1919 | 259 267 B | 246 018 B |

Em prosa: 14× menos blocos e stream 2.7× menor; em math puro o ganho é
modesto (~8%) porque o vanilla também fragmenta equações. O ganho real é
em documentos de prosa — e na legibilidade do diff directo de operadores
contra o vanilla (o objectivo do dono: posição + sequência de operador
idênticas, não bytes de container — decidido em P976).

## Desenho proposto (no L0, `stream.md` §P979)

Agrupar no emissor verbose itens `Text`/`TextShaped` consecutivos com
envelope idêntico (fonte, tamanho, fill, Tr, tracking, direcção, upem) e
mesma baseline: um prefixo só, um array `TJ` só, gaps inter-itens como
ajustes computados das **posições** (bit-exactos por construção). Nunca
fundir através de mudança de estado, não-texto, quebra de linha ou
fronteiras de Group/Link. Aceitação: render pixel-idêntico + compare.py
identidade + contagem BT ≈ vanilla no corpus de prosa.

**Custo/risco estimado**: médio — a aritmética de deltas `TJ` já existe
(P485/P520/P548); o risco está em condições de fronteira incorrectas
(fundir o que não devia). Coberto pelos testes de não-fusão do Agente A.

**PARADO no gate** — à confirmação, executo a Fase B com o protocolo de
dois agentes (testes de fusão/não-fusão primeiro, implementação depois,
render pixel-idêntico como revisão).
