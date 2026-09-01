# P1292 — amendment-11: flow ordinário sem Flush, owner Block/Place

**Papel:** autor segregado do contrato + auditor de ownership

**Manifest autorizado:**
`d72497168ba64350377e705dec213be3027a2f0a8147f5e790a5c8a63be2b84e`

**Estado:** correção interna de paridade em owners existentes; sem novo gate
ADR-0127. Contrato pronto para resselo v12 e entrega causal.

## Proveniência

- `HEAD`: `0eb39f8ecb48930515f2cadb6a378450855b5a72`;
- working tree não commitada;
- medição fresca: `2026-09-01T10:52:16-03:00`;
- `git diff HEAD --stat`: 45 arquivos, 3.053 inserções e 497 remoções;
- vanilla ratificado `/usr/local/bin/typst` SHA-256
  `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`;
- candidato `target/debug/typst` SHA-256
  `e15ee67575bd87d6770927086ab979b3fe30d8addf348eb90b2710a2a28966df`.

As fixtures foram compiladas bilateralmente por CLI e lidas com
`pdftotext -bbox`. O autor não leu nem executou o oracle protegido e não leu
ou editou código produtivo/testes.

## Medição antes da decisão

Fixtures independentes:

```text
/tmp/p1292-v11-no-flush.typ
d0ef9c0534aec9c9fd23104bbb5b8ca9dc82f99b63b9f92a1e0b7c51ec838be0

/tmp/p1292-v11-block-control.typ
31b6fed8403c7138f4eebc093d6b00ac3cdf1f0eb1f5df53caf92795a33df7a2
```

O primeiro conserva o bottom float anterior e remove `place.flush`; o segundo
remove float e Flush. Os resultados coincidem dentro de cada renderer:

| Controle | Token | Vanilla ratificado | Candidato refutado |
|---|---|---:|---:|
| sem Flush | `AFTER_FLOW` | p1, `40.604pt` | p1, `27.404pt` |
| sem Flush | `AFTER_MARKER` | p1, `47.842pt` | p1, `-9.834pt` |
| sem float/Flush | `AFTER_FLOW` | p1, `40.604pt` | p1, `27.404pt` |
| sem float/Flush | `AFTER_MARKER` | p1, `47.842pt` | p1, `-9.834pt` |

O vetor transacional principal já estava bilateral: p3
`AFTER_FLOW=-2.596pt`, `AFTER_MARKER=4.642pt`, prefix float p2
`17.404pt` e suffix float p3 `87.404pt`. Portanto o controle novo não refuta a
atomicidade v11; refuta a atribuição das fórmulas ordinárias ao replay.

## Fonte vanilla e classificação

Medições de fonte no pin ratificado:

- `typst-library/src/layout/container.rs:347-360` define o default de
  `block.spacing` como `Em::new(1.2)` e o projeta em `above`/`below`;
- `typst-layout/src/flow/collect.rs:232-278` resolve o espaçamento e insere
  `below` depois do frame Block;
- `typst-layout/src/flow/distribute.rs:504-510` conserva Place não-float como
  item out-of-flow;
- `typst-layout/src/flow/distribute.rs:630-662` avança o offset pela altura do
  frame in-flow e ancora Place sem alinhamento vertical nesse offset corrente.

Hashes das fontes:

```text
56f673258670f56ba73807d47db316bb14d48b82114c1d9b4ce50aa62a8c220a  container.rs
0baa7e20f1b2e1bfb5d7901ce24f90073fd53648281f4b41c25b090d43318b11  flow/collect.rs
36f0732a6004d189024da6dfade8767f9f53ad0cacc346211d0b2d3cf3c9b09b  flow/distribute.rs
```

**Medido:** a diferença de `AFTER_FLOW` é exatamente `13.2pt`, ou `1.2em` a
11pt; ela existe sem marker e sem float. **Inferido:** no modelo cristalino, a
transição Block perde a origem/gap ordinária e Place não-float com linha
pendente usa o topo do sub-layout. A inferência seria refutada se uma fixture
sem Block conservasse o mesmo delta de 13.2pt, ou se variar
`block.spacing`/`below` não variasse `AFTER_FLOW`; esses casos entram no teste
independente v12.

Classificação ADR-0107/0108: página, bbox, gap e ordem são geometria observável
da linguagem. `Child`, `Item`, offset interno e forma do checkpoint são
mecânica e não são exigidos como cópia.

## Decisão owner-correct

### Block entra no grafo

`compiler/layout/block.md` é o owner 1:1 de
`01_core/src/compiler/layout/block.rs` e entra como 27º L0 canônico. Ele deve
preservar o fim físico do Block e o espaçamento `below` efetivo como origem da
unidade seguinte, sem apagar o estado que a próxima linha precisa. O default
continua relativo e valores explícitos/colapso permanecem soberanos.

Esta obrigação não pertence a Cursor: somar `1.2em`, altura de Block ou uma
constante de fixture durante replay seria hack cross-owner. Descendentes
out-of-flow do Block devem ser rebased com o frame/container pelo owner Block,
sem absorver a fórmula específica de Place.

### Place conserva sua fórmula não-float

`compiler/layout/place.md`, já no grafo, passa a fixar que Place não-float sem
alinhamento vertical ancora na origem corrente do flow. Com linha pendente,
isso combina baseline corrente e aresta superior semântica; nunca escolhe o
topo físico só porque a linha ainda não foi commitada. `dx`, `dy`, alinhamento
horizontal e rebase permanecem.

### Layouter/Cursor têm limite negativo

`compiler/layout.md` continua dono da capacidade privada de checkpoint e
`compiler/layout/cursor.md` de commit/rollback/progresso/replay. Ambos recebem
uma proibição explícita: nenhuma translação genérica por crescimento de
`current_items`, nenhum gap de Block e nenhuma fórmula de Place fora de uma
transação de marker realmente rejeitada. O replay apenas reproduz a semântica
ordinária dos owners Block/Place.

`compiler/layout/flush.md` permanece inalterado: os controles demonstram que a
sentinela não é causa do resíduo. Alterá-la para compensar seria deriva.

## Aceitação v12

1. sem float/Flush: `AFTER_FLOW=40.604±0.002pt` e
   `AFTER_MARKER=47.842±0.002pt`, p1, uma vez;
2. com float anterior e sem Flush: os mesmos valores e página;
3. variação de `block.spacing`/`below` prova resolução relativa e mata
   hardcode `13.2pt`;
4. vetor transacional preservado: p3 `AFTER_FLOW=-2.596±0.002pt`,
   `AFTER_MARKER=4.642±0.002pt`, uma vez;
5. prefix/suffix floats conservam `17.404±0.002pt` e `87.404±0.002pt`;
6. nenhum ajuste em Flush, contrato público, default de produto, fase ou
   compatibilidade.

## ADR-0127 e parada causal

O refino corrige fórmulas internas de Block/Place e restringe uma compensação
interna de Cursor. Não adiciona/alterar campo público, método de trait,
assinatura, comportamento deliberadamente padrão, fase do pipeline ou
compatibilidade. É correção de paridade e segue fluxo contínuo ADR-0127, sem
novo gate humano.

Produto permanece suspenso até o seal v12, receipt e testes independentes RED.
