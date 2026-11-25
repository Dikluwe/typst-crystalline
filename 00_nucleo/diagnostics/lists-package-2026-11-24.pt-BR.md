# Proposta de pacote: listas, enumerações e termos

**24 de novembro de 2026 · aprovado pelo dono em 24/11/2026 (“Aprovo A–G”, no chat) sob a [ADR-0127](../adr/adr-0127-l0-gate-stop-vs-continuous-flow.md) (§2.1, §2.4 e pacotes de paridade)**

A aprovação cobre o escopo abaixo. É o terceiro pacote da série do [contrato de `set`](set-contract-package-2026-11-24.pt-BR.md).

## Os problemas principais

1. **Listas aninhadas saem achatadas.** Todo item é desenhado na margem da página, qualquer que seja o nível, e o marcador não muda. `- a` com `- b` dentro põe os dois na mesma coluna.
2. **Dentro de contêineres a posição é errada.** A origem usada é a margem da página: em `block(inset: 20pt)` o item fica 20pt à esquerda do lugar, fora do bloco; em `rect`, `box` e tabela, sobra espaço.
3. **`enum.item(-1)` escreve `4294967295.`**, sem erro.

## Medição

HEAD `f2c737125`, árvore não commitada dos quatro pacotes anteriores e da alteração de outra autoria em `layout/decorations` (`git diff HEAD --stat`: 184 arquivos). Cristalino `b5b38cc53d605a5dfb68e2408be10b5f1d8f6e14d8393a08f52f53aa1c2769a7`, vanilla `e36333ee02be2066646ad632996d92468af9a60b834c6e518ce00ccf3221bb3b`. Raster a 144 ppi, de 23:46:18 a 23:46:31 de Brasília; posições de texto a partir de 23:46:51. Script e saída em `lab/results/lists-package-2026-11-24/` (`lst.json` SHA-256 `ad11fc3dc000e43fe020af7c3c59a9fc55f2c7a8fcc4162acb6c739793f307e9`), local e não versionado.

De 57 documentos, 21 coincidem em até 30 pixels de diferença forte. Dos 27 valores por `set`, 22 são ignorados com aviso e os outros cinco não têm efeito próprio no documento de teste.

| Caso | Vanilla | Cristalino |
|---|---|---|
| três níveis de `-`, x do marcador | 10; 19,3; 28,9 · `•` `‣` `–` | 10; 10; 10 · `•` `•` `•` |
| três níveis de `+`, x do número | 10; 23; 36 | 10; 10; 10 |
| 11 itens de `+`, x de `1.` a `9.` | 15,1 (alinhado à direita) | 10 |
| `block(inset: 20pt)[- a]` | 30 | 10 |
| `- = H`, título dentro do item | mesma linha do marcador | uma linha abaixo |
| `- #rect[a]` | mesma linha | uma linha abaixo |
| parágrafo depois de lista justa | 20,5pt | 14,4pt |
| `/ aa: texto longo`, segunda linha | x 32 | x 10 |
| `enum(start: 0)` | aceita | erro |
| `enum.item(-1)` | erro | `4294967295.` |
| `[- a].fields()`, `list([a],[b]).func()` | `body`; `list` | vazio; sequência sem `children` |
| `show list.item: it => it.body` | aplica | erro |

## Causa

- `layout/list_item.rs`, `enum_item.rs`, `terms.rs` e `term_item.rs` usam `page_config.margin.left` como origem, e não o início da linha da região.
- Não há elemento de lista: o construtor devolve uma sequência de itens, e a marcação produz itens soltos. Sem o conjunto não há largura comum de rótulo, nível, `show list`, `spacing` nem `reversed`.
- O marcador é texto simples (`ListMarker::Custom`), então conteúdo e função não cabem.
- O corpo do item começa na linha já ocupada pelo marcador, e um bloco no começo quebra essa linha.
- `TermItem` escreve `": "` fixo e não recua a continuação.

## Escopo proposto

| Parte | Mudança | Efeito |
|---|---|---|
| A | origem na região, e não na margem da página, em itens, números e termos | corrige aninhamento e contêineres |
| B | o conjunto: itens consecutivos de mesma espécie formam um elemento, na marcação, no construtor e em laços, com nível, largura comum de rótulo e espaçamento próprio | alinhamento dos números, marcador por nível, espaço antes e depois |
| C | corpo composto ao lado do rótulo, com a primeira linha alinhada à linha de base | título, forma, equação e bloco ficam na linha do marcador |
| D | construtores completos: `marker` com conteúdo, array e função; `spacing`, `marker-align`, `number-align`, `full`, `reversed`, `start: 0`, numeração por função; número negativo é erro | aceita mais; `enum.item(-1)` vira erro |
| E | identidade e campos: `show list`, `enum`, `terms`, `fields`, `func`, `children`, `start`, `number`, `body` | regras e consultas hoje inválidas funcionam |
| F | `set` dos três, pelo mecanismo existente | 22 valores deixam de ser ignorados |
| G | `terms`: separador como conteúdo, `hanging-indent` de 2em na continuação, `indent`, `spacing` | muda listas de termos com mais de uma linha |

**Representação.** `Content::Terms` já é um contêiner de itens percorrido por tudo. Proponho que carregue também a espécie (lista, enumeração ou termos) e as propriedades, do mesmo modo que `Block` passou a carregar o contorno das formas. Não há variante nova em `Content`.

## Compatibilidade e contrato

A, B, C e G mudam a aparência de praticamente todo documento com listas aninhadas, contêineres, dez ou mais números, ou termos longos. D faz número negativo errar. E muda o resultado de `repr` e igualdade: `list([a])` e `[- a]` deixam de ser iguais, como no vanilla.

Contrato público: `TermsElem` ganha campos `pub(crate)`, o que impede literais fora de `typst_core`. `ListMarker` permanece como entidade e deixa de ser o portador do construtor. Nenhuma variante nova em `Content`, `Value` ou `Style`, flag, fase ou dependência.

## Comportamentos do vanilla a decidir pela prova

| Caso | Vanilla | Tratamento proposto |
|---|---|---|
| `align(center)[- a …]` | centra o corpo e deixa o marcador na margem | **não comprovado:** o item ocupa a largura toda e só o corpo é alinhado. Seguir o vanilla |
| `list(spacing: -5pt)` | sobrepõe os itens | pedido explícito do autor. Seguir |
| `list(body-indent: -30pt)` | o corpo sai da página | idem |
| `enum(reversed: true, [a], enum.item(7)[b], [c])` | 3, 7, 6 | **não comprovado:** continua a contagem regressiva a partir do número dado. Seguir |

Nenhum caso medido mostrou comportamento comprovadamente errado. Se a execução achar algum, ele não será copiado.

## Fora

- `set align`, que não tem efeito em lugar nenhum.
- `numbering` com padrão sem símbolo de contagem, como `"*"`: é da numeração.
- Título e equação de bloco em região isolada fora de listas.
- `rect` e `box` com lista dentro devem passar a coincidir por causa de A; se sobrar diferença, será declarada.
- `query` por item, que o vanilla também recusa.

## Aceitação

1. L0s e Núcleo primeiro; selos e linhagem limpos. Donos alterados no formato curto.
2. RED no CLI por parte, com posição de texto, SVG, PNG e erro com posição.
3. Os 57 documentos: cada um a até 30 pixels do vanilla, ou declarado com a causa.
4. Listas com nota, contador, estado, referência, `context`, quebra de página, RTL, colunas e repetição em cabeçalho: cada efeito uma vez.
5. `set` igual ao construtor, `show … set`, escopo e valor reutilizado.
6. Listas simples de um nível, fora de contêiner, iguais ao estado anterior.
7. Suíte completa, lint e linhagem reversa.

## Decisão pedida

Aprovar A–G. A alternativa é aprovar agora A–C e a recusa de número negativo, que corrigem posição e erro silencioso, e deixar E, F, G e o resto de D para depois.

## Execução

**25 de novembro de 2026, 09:46 às 09:50 de Brasília.** Partes A a G implementadas. HEAD `f2c737125`, árvore não commitada:  222 files changed, 6539 insertions(+), 2952 deletions(-) (inclui os quatro pacotes anteriores e a alteração de outra autoria em `layout/decorations`), mais os arquivos não versionados listados por `git status`. Cristalino `1e0711123fe806ce83f8ee84206d2280d68a0ba0867e3710fed07c4dc8bb5982`; vanilla `e36333ee…`.

- **Suíte:** 10.960 passam, nenhuma falha, 7 ignorados. Lint com zero violações. Linhagem reversa: 1.033 consumidores, 0 erros.
- **RED→GREEN:** seis testes novos do CLI falharam antes do código e passam.
- **Sonda:** 54 dos 57 documentos a até 30 pixels (antes, 21). Nenhum valor por `set` é ignorado (antes, 22), e todos os de `list` e `enum`, por construtor e por `set`, têm diferença zero. `lab/results/lists-package-2026-11-24/lst-after.json`, SHA-256 `360c15ff22ff3a5aa945751f888254aa4b6488459107bcde756625261da6e6d4`, local e não versionado.

### Os três documentos restantes

| Documento | Causa |
|---|---|
| `block(stroke: 1pt)[- a…]` | a moldura mede o mesmo; o traço de `block` é desenhado maior que o do vanilla |
| `- a #footnote[n]` | 32 pixels, na chamada da nota; `a #footnote[n]` sem lista mostra a mesma separação |
| `#v(170pt)` e quatro itens | posições e páginas iguais no PDF; o PNG saiu num só arquivo com `{p}` no nome, em vez de um por página |

### Achado na execução

`/ termo: descrição` avaliava para nada: a lista de termos em marcação sumia do documento. A proposta registrou só a posição da continuação. Agora é um item.

### Desvios declarados

- O vanilla recusa `terms(([a], [b]))` com `expected content, found array`, embora a sua própria mensagem anuncie `term item or array`. A contradição está provada, e o par em array é aceito.
- Os pares nomeados de `terms` continuam aceitos como extensão.
- `[+ a].number` fora de regra devolve `auto`; o vanilla erra. Dentro da regra os dois coincidem.
- Cada campo de um conjunto numa regra vale a declaração ou o padrão. Em `repr(it)` só aparecem os declarados.
- Número maior que 4.294.967.295 é erro; o vanilla aceita 64 bits.
- A resolução do marcador que vem de contexto dentro de uma lista parte do nível 0.

### Limites já existentes, fora do pacote

`align` num bloco com recuo, `quote` e `figure` com bloco no corpo não deslocam o bloco, com ou sem lista. `set par(first-line-indent:)` ainda não tem efeito. O HTML não exporta listas, como antes. Rótulo `<l>` em item continua associado ao item.
