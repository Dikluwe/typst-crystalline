# Panics na suíte oficial

**26 de novembro de 2026 · pedido do dono no chat (“Corrigir os panics”), fluxo contínuo pela [ADR-0127](../adr/adr-0127-l0-gate-stop-vs-continuous-flow.md): correção de falha e de paridade sem mudança de contrato protegido**

## Medição

HEAD `721bd15cf`, árvore com a alteração de outra autoria em `layout/decorations` (`git diff HEAD --stat`: 5 arquivos). Cristalino debug `b4b414bacfa10d49cd54c87e1e6fa36928bc9b6aeac98bb4446fcf2bf91e6a58`, compilado em RAM. A suíte oficial (`lab/typst-original/tests/suite`, 3.829 testes) rodou de 09:20:24 a 09:22:14 de Brasília, com PNG a 72 ppi. Script e saída em `lab/results/upstream-suite-2026-11-26/` (`up.json` SHA-256 `04d7cb298d6e5430e34a79bf9d88127b34eef710466ddce461fae54f7e5dcb33`), local e não versionado.

25 testes terminam com panic (código 101) no cristalino e nenhum no vanilla.

| Testes | Local | Mensagem |
|---:|---|---|
| 1 | `stdlib/foundations/cast.rs:219` | `attempt to add with overflow` |
| 8 | `layout/block.rs:719` | `insertion index (is N) should be <= len (is M)` |
| 16 | `03_infra/src/export/render.rs:117` | `tamanho validado acima`: a página tem largura infinita |

## Causa

- `range` soma o passo sem verificar estouro. `range(2, 3, step: int.max)` estoura em debug; em release o valor dá a volta. A linguagem para antes de estourar (`tests/suite/foundations/array.typ:149-153`).
- O construtor de `curve` mede a caixa como máximo menos mínimo dos pontos e deixa a largura indefinida quando há um só ponto; o layout então usa a largura da região, infinita em `page(width: auto)`. A linguagem mede a curva a partir da origem: o tamanho é o maior ponto, nunca negativo (`crates/typst-layout/src/shapes.rs:186-271`). Medido: `curve(curve.move((20pt,20pt)), curve.line((40pt,40pt)))` seguido de `X` põe `X` em y 83,4 no vanilla e 43,4 aqui.
- A largura de coluna `auto` de células de tabela ganha uma folga de arredondamento proporcional à largura disponível; com largura infinita, `f64::EPSILON * ∞` é infinito, e qualquer `table` em `page(width: auto)` fica infinita. `grid` não passa por esse termo.
- Um bloco com fundo, borda ou `outset` cujo corpo passa para a região seguinte insere a moldura no índice e na altura da região anterior.

## Correção

| Parte | Mudança |
|---|---|
| A | `range` para quando o próximo passo estouraria |
| B | `curve` tem o tamanho do maior ponto a partir da origem, inclusive com um só ponto |
| C | a folga de arredondamento ignora larguras disponíveis infinitas |
| D | a moldura de um bloco cujo corpo mudou de região cobre só o trecho na região final, a partir do topo do conteúdo; o tamanho fixo não leva o cursor de volta à região anterior |

### Segunda medição

Depois de A–D, com o cristalino `a0f288eb49df2e78fd71f5b87e889b4291589f8c585f1c882402842ac8cd44ed` (árvore não commitada: os arquivos de A–D mais a alteração de outra autoria, `git diff HEAD --stat`: 15 arquivos), a suíte oficial rodou de 09:36:25 a 09:38:41 (`up-after.json`). A primeira contagem considerou só testes sem erro esperado; contando todos, eram 29 panics antes e restam 5:

| Testes | Causa | Linguagem |
|---:|---|---|
| 3 | `calc.gcd` e `calc.lcm` tiram o valor absoluto de `int.min` sem verificar | `the result is too large` (`foundations/calc.rs:748-784`) |
| 1 | `polygon` com vértice infinito chega ao layout | `cannot create polygon with infinite size` (`typst-layout/src/shapes.rs:321-323`) |
| 1 | colunas `fr` dividem um espaço restante infinito | uma fração de espaço infinito é zero (`layout/fr.rs:56-63`) |

`polygon` também mede a caixa como máximo menos mínimo; a linguagem mede a partir da origem, como em `curve`.

| Parte | Mudança |
|---|---|
| E | `calc.gcd` e `calc.lcm` seguem o algoritmo e o erro da linguagem |
| F | `polygon` e `curve` medem a partir da origem e recusam tamanho infinito com o erro da linguagem |
| G | colunas `fr` sobre espaço restante infinito têm largura zero |

Fica de fora: a moldura nos fragmentos anteriores de um bloco quebrado e o `{p}` literal no nome do PNG de página única.

## Execução

**26 de novembro de 2026.** HEAD `721bd15cf`, árvore não commitada com A–G e a alteração de outra autoria (`git diff HEAD --stat`: 19 arquivos). Cristalino debug `c7a24391acc37f554748422c74cbacd540eaf62593c1dbe92e9b9d4dcdb71adf`, compilado em RAM. Suíte oficial de 09:46:17 a 09:48:49 (`up-after2.json`). Suíte do projeto 10.982 aprovados, 0 falhas, 7 ignorados; doc-tests 5; lint sem violações; linhagem reversa 1.034 consumidores, 0 erros.

| Suíte oficial | Antes | Depois |
|---|---:|---:|
| panics | 29 | 0 |
| imagem idêntica | 883 | 888 |
| erro só no cristalino | 451 | 426 |
| erro esperado nos dois | 877 | 878 |
| mesma mensagem de erro | 539 | 546 |

Nenhum teste que era idêntico ou quase idêntico piorou. Os 25 que deixaram de falhar renderizam; 23 ainda diferem na imagem.

`issue-1918-layout-infinite-length-polygon` deixou de dar panic, mas agora é aceito: `layout(size => …)` numa página `width: auto` entrega uma largura finita aqui, e o vanilla entrega infinita. Fica como divergência de `layout`, fora deste pacote.
