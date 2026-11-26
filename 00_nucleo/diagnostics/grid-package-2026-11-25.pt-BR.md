# Proposta de pacote: grade e tabela

**25 de novembro de 2026 · aprovado pelo dono em 25/11/2026 (“Aprovo A–F”, no chat) sob a [ADR-0127](../adr/adr-0127-l0-gate-stop-vs-continuous-flow.md) (§2.1, §2.4 e pacotes de paridade)**

É o quarto pacote da série do [contrato de `set`](set-contract-package-2026-11-24.pt-BR.md). A [revisão dos L1](l1-review-2026-11-24.pt-BR.md) põe `grid` logo depois das listas. As decorações e o texto vêm depois, porque `layout/decorations.rs` tem uma alteração de outra autoria ainda sem commit.

## Os problemas principais

1. **Uma célula com mais de uma linha sai fora do lugar, em `grid` e em `table`.** A primeira linha sobe 2,6pt. Na primeira coluna, a segunda linha desce 11,8pt além do vanilla. Basta uma quebra de linha ou um texto que não cabe na largura.
2. **`grid` descarta células `str` em silêncio.** `#grid(columns: 2, "a", "b", "c")` não escreve nada. `#grid(1, 2.5)` é aceito, e o vanilla responde `expected content, found integer`. `table` faz certo nos dois casos.
3. **`set grid` não tem efeito em nenhuma propriedade.** Os 18 valores testados são ignorados com aviso, e na sonda da revisão também os de `grid.cell` e `grid.hline`.

## Medição

HEAD `39936c553`, árvore com a alteração de outra autoria em `layout/decorations` (`git diff HEAD --stat`: 5 arquivos, 359 inserções, 34 remoções, nenhum em `grid` ou `table`). Cristalino `148212223bbc5c854b74c13e592414e5a092944adbe48e7512af3ec7fe75577c` (debug), vanilla `e36333ee02be2066646ad632996d92468af9a60b834c6e518ce00ccf3221bb3b`. Sonda `set` da revisão refeita de 20:29:32 a 20:30:12 de Brasília. Raster da grade a 144 ppi de 20:31:17 a 20:31:29. Posições de texto em SVG e contagem de páginas depois disso, na mesma árvore. Scripts e saídas em `lab/results/grid-package-2026-11-25/` (`grid.json` SHA-256 `6c26341f70553e463d083f89e229c6ac021c40403417b7e217e781464b099f32`), local e não versionado.

Dos 30 documentos de grade, 12 coincidem em até 30 pixels de diferença forte. Para cada um dos 8 parâmetros de `grid` e `table`, o construtor e o `set` foram testados com até três formas de valor:

| | `grid` | `table` |
|---|---|---|
| construtor, mesmo raster que o vanilla | 6 de 18 | 7 de 18 |
| construtor recusado só aqui | 11 | 9 |
| construtor aceito e ignorado | 0 | 2 (`gutter`) |
| `set` com efeito igual | 0 | 8 |
| `set` ignorado | 18 | 9 |
| `set` recusado só aqui | 0 | 1 (`align` com função) |

O `gutter` de `grid` com um só comprimento é aceito, mas o raster difere em 360 pixels. A sonda conta isso como diferença, não como valor ignorado.

| Caso | Vanilla | Cristalino |
|---|---|---|
| `grid(columns: (auto, 1fr), [a], [b #linebreak() c])`, y das linhas da célula 2 | 17,2; 31,6 | 14,6; 29,0 |
| `grid(columns: 1, [b #linebreak() c])`, y das linhas | 17,2; 31,6 | 14,6; 43,4 |
| `table(…, inset: 0pt, stroke: none)` com a mesma célula | 17,2; 31,6 | 14,6; 29,0 |
| `block[b #linebreak() c]` e `box[…]` | 7,2; 21,6 | igual |
| `grid(columns: 2, "a", "b", "c")` | 3 células com texto | nada |
| `grid(columns: 2, ..range(60).map(str))`, páginas | 2 | 1, vazia |
| o mesmo com `[#x]` | 2 | 2 |
| `grid(…).children`, `table(…).children` | lista de células | erro; lista |
| `gutter: (5pt, 15pt)`, `column-gutter`, `row-gutter` | aceita | erro |
| `table(gutter: 10pt)` | espaça | ignora |
| `fill`, `stroke` e `align` com array ou função `(x, y) =>` | aceita | erro (`table.align` com array funciona) |
| `inset: (x: 10pt)` | aceita | erro |
| `set grid.cell(fill: red)`, `show grid.cell: set text(red)` | aplica | aviso; 46 pixels |
| `grid.hline` e `grid.vline` | traços | 124 pixels de diferença |
| `grid.cell(x: 1, y: 1)` | posição pedida | 41 pixels de diferença |
| `grid.cell(rowspan: 3, fill: red)` atravessando a página | preenche nas duas | 2.704 pixels de diferença |

## Causa

- `stdlib/layout/grid.rs` (`native_grid`) aceita só `columns`, `rows`, `gutter`, `align`, `inset`, `stroke` e `fill`, cada um em uma forma única. Valores que não são conteúdo caem fora da lista de células sem erro.
- `GridElem` guarda um `gutter` único, `align` único e `inset` uniforme. `TableElem` tem resolução própria (`TableGridResolved`), alinhamento por coluna e declarações de `set`, que o [pacote de `set`](set-contract-package-2026-11-24.pt-BR.md) ligou só para `table`.
- `layout_grid` aplica o espaçamento só entre colunas (o comentário diz “graded”) e não recebe espaçamento entre linhas.
- O deslocamento das linhas está no caminho de célula comum aos dois elementos, em `layout/grid/cell_body.rs` ou `cell_emission.rs`, e não aparece em `block` nem em `box`. **Inferência:** a primeira linha usa a altura de linha e não o topo do texto, e a primeira coluna soma a entrelinha duas vezes. A execução confirma isso com RED antes da correção. Se a causa for outra, ela será registrada aqui.

## Escopo proposto

| Parte | Mudança | Efeito |
|---|---|---|
| A | posição das linhas dentro da célula igual à de `block` | corrige toda grade e tabela com célula de mais de uma linha |
| B | células de `grid` como em `table`: `str` vira texto, outros valores dão `expected content, found …` com posição; `children` e `fields` | o texto volta a aparecer; entrada inválida vira erro |
| C | `gutter`, `column-gutter` e `row-gutter` com comprimento, relativo ou array por trilha, nos dois eixos, em `grid` e `table` | aceita mais; `table(gutter:)` passa a espaçar |
| D | `fill`, `stroke`, `align` e `inset` em todas as formas da linguagem: array por coluna, função `(x, y) =>`, dicionário de lados, em `grid` e `table` | aceita mais |
| E | `set` de `grid`, `grid.cell`, `grid.hline`, `grid.vline`, `grid.header` e `grid.footer`, e os espaçamentos e funções de `table`, pelo mecanismo existente; `show grid.cell` | 18 valores de `grid` e 10 de `table` deixam de ser ignorados |
| F | posição explícita de célula, traços de `hline` e `vline` e `rowspan` atravessando a página em `grid`, como já funcionam em `table` | corrige os três casos medidos |

**Representação.** `grid` e `table` são o mesmo modelo na linguagem, com padrões diferentes: `table` tem contorno de 1pt e recuo de 5pt, e `grid` não tem nenhum dos dois. Proponho que `GridElem` passe a carregar as mesmas formas de valor e a mesma resolução que `TableElem`, compartilhando os donos de resolução e de declaração em vez de duplicá-los. Não há variante nova em `Content`.

## Compatibilidade e contrato

A muda a aparência de toda grade e tabela com célula de mais de uma linha. B faz `grid(1, 2)` errar onde hoje compila. C muda tabelas que hoje passam `gutter` e o veem ignorado. As outras partes só aceitam o que hoje é erro, ou aplicam o que hoje é ignorado com aviso.

Contrato público: `GridElem` ganha campos e muda o tipo de `gutter`, `align`, `inset`, `stroke` e `fill`, o que quebra literais fora de `typst_core`. Os campos ficam `pub(crate)`, como fiz com `TermsElem`. Nenhuma variante nova em `Content`, `Value` ou `Style`, flag, fase ou dependência.

## Comportamentos do vanilla a decidir pela prova

| Caso | Vanilla | Tratamento proposto |
|---|---|---|
| `grid(1, 2)` | erro, mas `table("a")` e `grid("a")` aceitam | **não comprovado:** só conteúdo e texto viram célula. Seguir |
| `fill` com função que devolve `none` ou cor | sem preenchimento na célula | seguir |
| `gutter` negativo | aceita e sobrepõe | hoje é erro aqui. **Não comprovado:** é pedido explícito do autor, como `list(spacing: -5pt)`. Seguir o vanilla |

Nenhum caso medido mostrou comportamento comprovadamente errado. Se a execução achar algum, ele não será copiado.

## Fora

- `columns(count)` com conteúdo e `columns.gutter`: são do elemento de colunas, não da grade.
- Cabeçalho e rodapé de `table` que já coincidem, e `header(repeat:)` da tabela.
- O traço de `block(stroke)` que passa da borda, das listas.
- Leitura de tabela em HTML.
- Os métodos de `datetime` e de cor chamados no valor, com `datetime + duration` e `datetime - datetime`. Na linguagem, um método é qualquer função do escopo do tipo, com o valor como primeiro argumento. As formas `datetime.year(d)` e `color.hsl(red)` já funcionam aqui. Falta essa ponte geral e a aritmética de datas. Cabem num pacote pequeno à parte.

## Aceitação

1. L0s primeiro. Selos e linhagem limpos. Donos alterados no formato curto.
2. RED no CLI por parte, com posição de texto, SVG, PNG e erro com posição.
3. Os 30 documentos e as 36 combinações de parâmetro: cada um a até 30 pixels do vanilla, ou declarado com a causa.
4. Grade e tabela com nota, contador, `context`, quebra de página, RTL, colunas, lista dentro de célula e cabeçalho repetido: cada efeito uma vez.
5. `set` igual ao construtor, `show … set`, escopo e valor reutilizado.
6. Grades e tabelas de células de uma linha, sem os novos argumentos, iguais ao estado anterior.
7. Suíte completa, lint e linhagem reversa.

## Decisão pedida

Aprovar A–F. A alternativa é aprovar agora A e B, que corrigem posição errada e texto perdido em silêncio, e deixar C–F para depois.

## Execução

**25 de novembro de 2026.** HEAD `39936c553`, árvore não commitada deste pacote e da alteração de outra autoria em `layout/decorations` (`git diff HEAD --stat`: 93 arquivos). Cristalino `90d9821c316e8764621ed789c1b66306972a2f15e9bf061f070a55859877d6c8` (debug), vanilla o mesmo da medição. Sondas de 22:00:25 a 22:00:30 de Brasília, raster de PDF a 144 ppi. Scripts e saídas em `lab/results/grid-package-2026-11-25/` (`x30-after.txt` SHA-256 `90c50c87fc5b4bbf25ec8e9059a8cdb44005f644fb24438d2bddb351ec007796`).

| Conjunto | Antes | Depois |
|---|---:|---:|
| 30 documentos da proposta | 12 | 26, mais 2 erros iguais nos dois |
| linhas em células (19) | 5 | 16 |
| espaçamentos (14) | — | 14 |
| arrays e funções em `grid` (14) | — | 12 |
| arrays e funções em `table` (10) | — | 9 |
| `hline`, `vline` e posições (8) | — | 8 |

Suíte completa 10.968 aprovados, 0 falhas, 7 ignorados; lint sem violações; linhagem reversa 1.034 consumidores, 0 erros.

**Causas encontradas.**

- A: o sub-quadro da célula põe a primeira linha de base no ascendente; `flush_line` puxava a primeira linha de um parágrafo quebrado para o topo do texto, e o fechamento final não. A célula passa a abrir um sub-quadro de célula, com a mesma regra nas duas pontas. Título e equação na célula usavam a margem da página como origem; o título foi corrigido.
- A, geral: um espaço que não cabe antes de `#linebreak()` quebrava a linha, e a quebra explícita abria outra. Valia também em `block` e `box`.
- B a F seguiram a proposta. A posição real das células passa a guiar a medição de trilhas, o que corrigiu células com `x`/`y` explícitos. `hline(y:)` e `vline(x:)` eram ignorados.

**Desvio de representação.** `GridElem.spec` é `pub`, não `pub(crate)`: testes de `03_infra` constroem grades.

**Limites que ficam.**

- Contorno por lado em dicionário (`stroke: (top: 3pt)`) em `grid` e `table`: a célula guarda um só contorno. Dá erro explícito.
- Contorno por função com espessuras diferentes em `table`: precedência entre células vizinhas (159 pixels).
- `show grid.cell` não alcança células escritas sem `grid.cell`, e `grid.cell.where(x:)` não conhece a posição. Envolver toda célula expôs um defeito anterior: nota de rodapé dentro de `grid.cell` quebra a linha.
- Na célula: equação de bloco, `box` em linha mais alto que o texto, `align`, `place`, `raw` em bloco, `figure`, e um bloco seguido de texto (este também em `box`). Já falhavam antes.
- Saída JSON de `typst eval` para grade e listas: o shell serializa à mão e mostra só `func`.
- `colspan` preenchido com espaçamento: 0,4pt na coluna `auto`.
