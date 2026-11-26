# Proposta de pacote: cabeçalhos e rodapés de grade e tabela

**26 de novembro de 2026 · aprovado pelo dono em 26/11/2026 (“Aprovo A–D”, no chat) sob a [ADR-0127](../adr/adr-0127-l0-gate-stop-vs-continuous-flow.md) (§2.1, §2.4 e pacotes de paridade)**

É o sexto pacote da série do [contrato de `set`](set-contract-package-2026-11-24.pt-BR.md) e o primeiro escolhido pela [suíte oficial](upstream-suite-panics-2026-11-26.pt-BR.md). As decorações e o texto continuam adiados enquanto `layout/decorations.rs` tiver a alteração de outra autoria sem commit.

## Os problemas principais

1. **`grid.header` e `table.header` não aceitam vários cabeçalhos.** `grid.header(level:)` é recusado, e um segundo cabeçalho dá `não pode haver mais do que um header`. A linguagem aceita cabeçalhos com nível, no começo e no meio da grade, e um cabeçalho novo substitui os de nível igual ou maior.
2. **O cabeçalho de `grid` e os rodapés não se repetem nas páginas seguintes.** O cabeçalho de `table` já se repete.
3. **Erros da linguagem faltam.** Cabeçalho dentro de cabeçalho ou rodapé, `table.header` dentro de `grid` e o contrário, e célula que expandiria o rodapé são aceitos sem erro.
4. **`[a] * 3` dá `cannot multiply content with integer`.** A linguagem repete o conteúdo.

## Medição

A suíte oficial rodou de 09:46:17 a 09:48:49 de Brasília no cristalino debug `c7a24391acc37f554748422c74cbacd540eaf62593c1dbe92e9b9d4dcdb71adf`. Esse binário foi compilado de HEAD `721bd15cf` com as correções de panics, mais tarde commitadas como `69ddba38c`, e da alteração de outra autoria em `layout/decorations`. As sondas de 10:05 a 10:09 usaram o mesmo binário, com HEAD já em `69ddba38c` e a mesma alteração de outra autoria (`git diff HEAD --stat`: 5 arquivos). Vanilla `lab/typst-original/target/release/typst`. Contagens por `cnt.py` sobre `up-after2.json` (SHA-256 `4bb1625a7ae6cd18d34ffb71dae3edd2879f005a6171f81e6ebf9dff32ba8421`); script, saída e sondas em `lab/results/grid-headers-package-2026-11-26/`, local e não versionado.

A família de grade e tabela (`layout/grid/*` e `layout/table.typ`) tem 354 testes. Só 26 saem iguais ao vanilla.

| Sintoma | Testes |
|---|---:|
| `level` recusado ou segundo cabeçalho recusado | 49 |
| erro de aninhamento ou de tipo de cabeçalho e rodapé que falta | 26 |
| célula que expandiria o rodapé, aceita | 2 |
| `table.header()` ou `grid.footer()` sem células recusado | 5 |
| `repeating table headers leave no room` ou `does not fit` só aqui | 4 |
| número de páginas diferente em `headers.typ` e `footers.typ` | 31 |
| `cannot multiply content with integer`, em todas as áreas | 29 |

Sondas com página de 120 × 80pt e 12 linhas:

| Caso | Vanilla | Cristalino |
|---|---|---|
| `table(table.header([H]), …)`: páginas, ocorrências de `H` | 6, 6 | 6, 6 |
| `table(…, table.footer([H]))` | 6, 6 | 5, 1 |
| `grid(grid.header([H]), …)` | 2, 2 | 2, 1 |
| `[a] * 3` | `sequence([a], [a], [a])` | erro |

## Causa

- `GridHeaderElem` não tem campo `level`. O construtor de `grid` recusa um segundo cabeçalho, e o de `table` recusa níveis repetidos (`stdlib/layout/grid.rs:570`, `stdlib/structural/table_grid.rs:203-221`). A mensagem cita a ADR-0054 como escopo adiado; essa ADR foi substituída pela [ADR-0107](../adr/adr-0107-language-parity-not-mechanics.md).
- Só os grupos de cabeçalho de `table` entram na repetição (`layout/grid/cell_emission.rs:205-302`). Não há repetição de rodapé.
- Os construtores não verificam onde cabeçalhos e rodapés aparecem (vanilla `typst-library/src/layout/grid/mod.rs:490-540`).
- `content * int` não existe em `eval/operators/arithmetic.rs` (vanilla `foundations/ops.rs:276`).

## Escopo proposto

| Parte | Mudança | Efeito |
|---|---|---|
| A | `level` em `grid.header` e `table.header`; vários cabeçalhos, no começo e no meio; um cabeçalho novo substitui os de nível igual ou maior; regras da linguagem para cabeçalho órfão e para cabeçalho que não cabe (`typst-layout/src/grid/repeated.rs`) | 49 testes hoje recusados e os 4 erros só daqui |
| B | o cabeçalho de `grid` e os rodapés com `repeat: true` se repetem em cada página | muda o número de páginas de 31 testes e todo documento com rodapé de tabela em várias páginas |
| C | erros da linguagem para aninhamento, tipo trocado e célula que expandiria o rodapé; cabeçalho e rodapé vazios aceitos | 33 testes |
| D | `content * int` e `int * content` repetem o conteúdo | 29 testes |

## Compatibilidade e contrato

- B muda a aparência de documentos com rodapé de tabela em várias páginas: o rodapé passa a aparecer em todas.
- C transforma em erro documentos que hoje compilam: só os que a linguagem recusa.
- `GridHeaderElem` ganha o campo `level`, como `TableHeaderElem`. Não há variante nova em `Content`, `Value` ou `Style`, flag, fase ou dependência.

## Comportamentos do vanilla a decidir pela prova

| Caso | Vanilla | Tratamento proposto |
|---|---|---|
| cabeçalho órfão no fim da página | **não comprovado:** os testes `*-orphan*` indicam que ele desce com a linha seguinte | medir na execução e seguir se confirmado |
| cabeçalho maior que a página | **não comprovado:** os testes `*-too-large-*` dependem de `[a] * N` (parte D) | medir depois de D |

Nenhum caso medido mostrou comportamento comprovadamente errado. Se a execução achar algum, ele não será copiado.

## Fora

- Linhas e bordas de grade (`grid/stroke.typ`), `rowspan` e `colspan` além do que A e B tocarem.
- Matemática, gradientes, colunas e notas, que têm grupos próprios na suíte oficial.
- Decorações e texto.

## Aceitação

1. L0s primeiro, com selos e linhagem limpos.
2. RED por parte, no CLI ou no layout, com páginas, posição de texto e erro com posição.
3. Os testes de `headers.typ`, `subheaders.typ` e `footers.typ` iguais ao vanilla a até 30 pixels, ou declarados com a causa.
4. A suíte oficial inteira sem nenhum teste que hoje é igual ao vanilla piorando.
5. Suíte completa, lint e linhagem reversa, com build em RAM.

## Decisão pedida

Aprovar A–D. A alternativa é aprovar só A e B, que são o layout dos cabeçalhos e rodapés, deixando C e D para depois.

## Execução

**26 de novembro de 2026.** HEAD `69ddba38c`, árvore não commitada com A–D e a alteração de outra autoria (`git diff HEAD --stat`: 39 arquivos). Cristalino debug `d963afd2606cfbb07f80c024874e5f2c174bea63f7f256c87796ec21439b5715`, compilado em RAM. Suíte do projeto 10.988 aprovados, 0 falhas, 7 ignorados; doc-tests 5; lint sem violações; linhagem reversa 1.034 consumidores, 0 erros.

**Correção de método.** A medição de antes comparava PNG gerado com `{p}` no nome. O cristalino não substitui `{p}` e grava uma única imagem, então todo teste com mais de uma página aparecia com uma página só, e a linha "número de páginas diferente" estava inflada. A comparação nova gera PDF nos dois e rasteriza com `pdftoppm` a 72 ppi (`up2.py`). O "antes" foi refeito com o mesmo método, num binário compilado de HEAD numa worktree em RAM (`d884e0dfbcf0e52e934f79329b6902a5f32449466a07f0c4ed1e4c9ae3e603ee`). Antes de 10:34:09 a 10:37:27 (`pdf-head.json`, SHA-256 `020d175c36d21480714f296f03a7639aa195630f40873051a8f0b98c56b7f411`); depois de 10:44:03 a 10:47:47 (`pdf-after2.json`, SHA-256 `4de67b7e9ba90837186b4b712dafb8e530151a9c2661a5ecc858b1f77cd936d7`). A falha do `{p}` é do CLI, um contrato protegido, e fica fora deste pacote.

| Suíte oficial | HEAD | pacote |
|---|---:|---:|
| imagem idêntica | 919 | 955 |
| até 30 pixels | 89 | 91 |
| erro só no cristalino | 412 | 338 |
| erro que falta no cristalino | 236 | 219 |
| mesma mensagem de erro | 546 | 568 |
| família de grade igual ao vanilla, de 354 | 25 | 61 |
| `headers.typ`, de 62 | 5 | 14 |
| `subheaders.typ`, de 49 | 2 | 28 |
| `footers.typ`, de 56 | 3 | 4 |

Nenhum teste que era idêntico ou quase idêntico piorou, e nenhum dá panic.

| Sintoma da proposta | Antes | Depois |
|---|---:|---:|
| `level` ou segundo cabeçalho recusado | 49 | 1 |
| erro de aninhamento ou tipo de cabeçalho e rodapé que falta | 26 | 6 |
| cabeçalho ou rodapé vazio recusado | 5 | 0 |
| `no room` ou `does not fit` só aqui | 4 | 0 |
| `cannot multiply content with integer` | 26 | 0 |

**Como ficou.**

- A: `grid.header` e `table.header` têm `level`. Sem linhas explícitas, a grade guarda os filhos em ordem de documento e o layout os coloca nessa ordem, com os grupos de cabeçalho e rodapé. Um cabeçalho novo substitui os que se repetiam com nível igual ou maior. Cabeçalho maior que a página é posto uma vez, sem erro e sem repetir; cabeçalhos que não deixariam espaço para a próxima linha não se repetem naquela página.
- B: o cabeçalho de `grid` se repete, e o rodapé de `grid` e `table` com `repeat: true` é reservado em cada página e posto logo depois da última linha dela, como medido no vanilla.
- C: os erros de aninhamento e de tipo trocado de cabeçalho e rodapé são os da linguagem; cabeçalho e rodapé vazios valem uma linha em branco.
- D: `content * int` e `int * content` repetem o conteúdo; contagem negativa dá `number must be at least zero`.

**Limites que ficam.**

- Vários cabeçalhos com linhas explícitas (`hline`, `vline`) ainda dão erro; um teste da suíte.
- Célula que expandiria o rodapé, rodapé levado para baixo dos `rowspan` e células com posição explícita depois do rodapé seguem a regra antiga (`footer must end at the last row`); quatro testes.
- `grid.cell` em `table`, `table.cell` em `grid` e linhas trocadas entre os dois ainda são aceitos; seis testes, fora do escopo de cabeçalho e rodapé.
- A maior parte das diferenças restantes em `footers.typ` vem de outras causas medidas: `page(height: 15em)` resolvido com o tamanho de texto do momento do `set`, e não com o final, e colunas `auto` mais largas que no vanilla.
