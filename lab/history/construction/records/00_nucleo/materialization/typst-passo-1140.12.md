# Passo 1140.12 — reconciliar posições shaped RTL sem destruir gaps justificados

**Estado:** executado  
**Data:** 2026-08-24  
**Predecessor:** P1140.11  
**ADRs:** ADR-0107, ADR-0108, ADR-0120, ADR-0127

## 1. Objetivo

Fechar a divergência RTL revelada no P1140.11 corrigindo a passagem que
reconcilia larguras estimadas do layout com advances reais do shaper. A
correção deve preservar os gaps que o layout já decidiu, inclusive os gaps
expandidos por `linebreak(justify: true)`, sem constantes extraídas de PDFs.

## 2. Proveniência e medição

Baseline: vanilla pinado `a51e02804`; cristalino da working tree sobre
`ca28f4ab74ae66985cdc66805c16c2ddc8f08366`.

Na sonda hebraica de página 160pt/margem 10pt:

- vanilla true: xMin `130,99 / 70,00 / 10,00` em ordem lógica RTL;
- cristalino true: `130,99 / 86,51 / 43,02`;
- o controle false já diverge na agregação/redistribuição pós-shaping.

Fonte medida:

- pipeline `03_infra/src/pipeline.rs:596-597`: `shape_document` e depois
  `fix_line_positions`;
- `03_infra/src/shaper.rs:2296+`: P582 afirma preservar espaçamentos;
- braço RTL `:2421+` ancora por x mas acumula apenas `w_real - w_est`, sem
  reconstruir explicitamente os gaps originais;
- o teste P582 RTL atual fixa uma posição artificial, não prova preservação de
  gap nem de borda direita.

## 3. Decisão

Para cada linha RTL, ordenar fisicamente por x e medir, antes de mutar:

```text
gap[i] = x[i+1] - (x[i] + largura_estimada[i])
right_edge = x[last] + largura_estimada[last]
```

Depois reconstruir da direita para a esquerda com larguras reais:

```text
x_real[last] = right_edge - largura_real[last]
x_real[i] = x_real[i+1] - gap[i] - largura_real[i]
```

Assim a borda direita e cada gap decidido pelo layout são invariantes. Os
números das sondas são somente oracle; nenhum entra na fórmula.

LTR permanece no algoritmo vigente. Math continua excluído como em P975.
Itens não textuais usam largura estimada = real e não criam correção espúria.

### 3.1 Achado durante o RED real

A fórmula de reconciliação passou nos testes unitários, mas não alterou a
sonda PDF. A causa dominante ocorre antes: `layout_bidi::reflow_rtl_paragraphs`
funde as duas linhas separadas por `linebreak`. Z aparece na mesma baseline da
primeira linha no cristalino, mas permanece na segunda no vanilla; o controle
false também é fundido.

O Frame não transporta a causa da quebra. Depois do flush é impossível
distinguir wrapping automático de quebra explícita apenas por distância y.
Heurísticas geométricas ou a constante `30.0` existente em `same_paragraph`
não podem substituir essa informação semântica.

Contrato público mínimo proposto:

```rust
SemanticKind::ExplicitLinebreakBoundary
```

`layout/linebreak.rs` acrescenta, somente ao fechar linha não vazia, um
`FrameItem::Semantic` transparente dessa kind com filho Text vazio na
baseline. Ele não desenha nem altera plain text/dimensões. `layout_bidi` trata
a kind como barreira absoluta de reflow. A variante muda o enum público L1→L3
e aciona ADR-0127; a execução para até confirmação do dono.

## 4. L0 e ADR-0127

Atualizar primeiro `00_nucleo/prompts/infra/shaper.md` com a fórmula acima e a
medição que a produz. É correção interna de paridade, sem contrato público,
default ou fase nova; fluxo contínuo ADR-0127, sem parada adicional.

O achado §3.1 exige também atualizar `entities/layout_types.md`,
`compiler/layout.md` e `infra/layout_bidi.md`. A variante pública exige
confirmação antes do código correspondente.

## 5. RED → GREEN

1. substituir o teste RTL artificial P582 por invariante de gaps + borda;
2. testar três itens com duas diferenças independentes de largura;
3. testar largura real igual à estimada como identidade;
4. recompilar sondas RTL false/true e comparar bboxes ao vanilla;
5. preservar os cinco testes P1140.11 e as sondas LTR exatas.

## 6. Validação

```sh
cargo test -p typst-infra p1140_12
cargo test -p typst-infra p582
cargo test -p typst-core p1140_11
cargo test -p typst-core
cargo test -p typst-infra
cargo build
cargo fmt --all -- --check
crystalline-lint .
git diff --check
```

Relatório final:
`00_nucleo/diagnosticos/typst-p1140.12-rtl-shaped-gaps.md`.

## 7. Gate L0 executado

Os quatro L0 foram atualizados antes do marcador público. Hashes calculados em
dry-run, ainda não escritos nos headers:

- `compiler/layout.md`: `69e30d1c`;
- `entities/layout_types.md`: `9758776a`;
- `infra/layout_bidi.md`: `f0c3f045`;
- `infra/shaper.md`: `0728ce28`.

A parte interna de reconciliação shaped já possui RED→GREEN unitário, mas a
sonda real provou que ela não resolve sozinha a fusão. Nenhum código da nova
variante `ExplicitLinebreakBoundary` foi escrito. O dono deve confirmar este
contrato e os hashes para a execução continuar.

## 8. Execução após confirmação

O dono confirmou o gate. Foi adicionada
`SemanticKind::ExplicitLinebreakBoundary`; `layout/linebreak.rs` emite o
envelope transparente apenas para linha não vazia, e `layout_bidi` impede que
`same_paragraph` atravesse essa fronteira. Os coletores de tags/MCID filtram
explicitamente `SemanticKind::Formula`, portanto o marcador não cria Formula,
tag, texto nem desenho.

A reconciliação shaped RTL também passou a reconstruir posições preservando
gaps e borda direita, em vez de validar coordenada artificial.

Sondas PDF pós-GREEN:

- true, vanilla = cristalino: Alpha `130,99–150,00`, Beta `70,00–84,35`,
  Gamma `10,00–23,36`, Z `147,11–150,00` na segunda baseline;
- false, vanilla = cristalino: Gamma `98,28–111,64`, Beta `114,14–128,49`,
  Alpha `130,99–150,00`, Z `147,11–150,00` na segunda baseline.

Nenhum desses valores aparece na implementação; são somente oracle.

Validação em `2026-08-24T12:12:30-03:00`, commit base
`ca28f4ab74ae66985cdc66805c16c2ddc8f08366`, working tree não commitado:

- diff tracked: **56 ficheiros, 1242 inserções, 211 remoções**;
- status: **65 entradas**;
- L1: **5155 passed, 0 failed**;
- L3: **830 passed, 0 failed**;
- P1140.12: **2 passed**; P1140.11: **5 passed**;
- build, fmt, lint e diff-check validados no fecho.
