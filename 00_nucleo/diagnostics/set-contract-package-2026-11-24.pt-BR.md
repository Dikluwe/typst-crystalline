# Proposta de pacote: contrato de `set`

**24 de novembro de 2026 · aprovado pelo dono em 24/11/2026 (“Aprovo A–F”, no chat) sob a [ADR-0127](../adr/adr-0127-l0-gate-stop-vs-continuous-flow.md) (§2.1, §2.4 e pacotes de paridade)**

A aprovação cobre o escopo abaixo. Este é o primeiro de uma série; as famílias seguintes reutilizam o mecanismo e terão proposta própria, menor.

## Medição

A superfície está na [revisão de L1](l1-review-2026-11-24.pt-BR.md): 136 de 182 propriedades sem efeito, 13 em silêncio, 228 de 298 sem validação. A semântica foi sondada a partir das 22:31:47 de Brasília, sobre HEAD `f2c737125` com a árvore não commitada, cristalino `9c29cabd3c12b5c6a07763ae577b5c77b5cce1e73e256c1cd184a7b4ea33aea4` e vanilla `e36333ee02be2066646ad632996d92468af9a60b834c6e518ce00ccf3221bb3b`, saída SVG.

| Caso | Vanilla | Cristalino |
|---|---|---|
| valor criado antes do `set` e inserido depois | recebe o estilo | não |
| `set` dentro de bloco, função, `show: template` | vale só no escopo | sem efeito |
| construtor com `none` explícito | vence o `set` | sem efeito |
| `set stroke: red` e depois `set stroke: 3pt` | combina cor e espessura | sem efeito |
| construtor `stroke: 3pt` sobre `set stroke: red` | combina | perde a cor |
| `show table: set table(…)` | aplica | sem efeito |
| apelido `#let t = table`, `set table(..d)` | aplica | sem efeito |
| `show heading: set block(above: 60pt)` | muda o espaço | sem efeito |
| condicional falsa | nada | nada |
| nome desconhecido, posicional, argumento do corpo | erro | `table` aceita em silêncio; `block` só avisa |
| `set f()` com função do usuário, `set calc.abs(1)`, `set x` com inteiro | erro | avisa e segue |
| argumento repetido | erro | erro |

## Causa

- `eval/rules/set.rs` compara o texto do alvo e trata cada elemento em um braço próprio. O resto cai em um aviso sem avaliar os argumentos.
- `table` só transporta `align`, e descarta os outros argumentos sem aviso.
- Os elementos gravam o padrão já resolvido (`breakable: bool`, `inset: Sides`). Não distinguem omissão de valor explícito, condição para o estilo entrar.
- `block.spacing`, `above` e `below` chegam à `StyleChain`, e o consumidor de `block` comum e o de título não os leem; `stack`, `image` e `raw` leem.
- O vanilla resolve campos omitidos na inserção (`foundations/styles.rs`). Isso é semântica: muda o resultado do valor criado antes.

## Escopo proposto

| Parte | Mudança | Efeito |
|---|---|---|
| A | o alvo é resolvido pela identidade da função | apelidos funcionam; função comum e não função viram erro |
| B | os nomes são conferidos contra o esquema do elemento; posicional e corpo são recusados | nome inválido vira erro em todos os elementos |
| C | quando o construtor já sabe converter o valor, o `set` usa a mesma conversão | valor inválido vira erro |
| D | nome válido sem efeito avisa sempre, com elemento e propriedade | nada mais é ignorado em silêncio |
| E | transporte comum: `elemento.propriedade` na `StyleChain`, campo explícito vence, `stroke` e `inset` combinam por componente, resolução na inserção | mecanismo para as famílias |
| F | `table`: `columns`, `rows`, `fill`, `stroke`, `inset`; `block`: as 13 propriedades do construtor; `above`, `below` e `spacing` de `set block` em títulos; `heading.offset`, `figure.gap`, `footnote.numbering` | passam a ter efeito |

**Regra de segurança de C.** Só é erro o que o vanilla também recusa. Valor válido na linguagem que o construtor cristalino ainda não aceita (por exemplo, `fill` com gradiente, função ou array em `table`) avisa e não falha.

**Compatibilidade.** A, B e C fazem documentos que hoje compilam com entrada inválida errarem. F muda a aparência de documentos que hoje têm a regra ignorada. D acrescenta avisos.

**Contrato público.** `TableElem`, `BlockElem`, `HeadingElem`, `FigureElem` e `FootnoteElem` precisam lembrar quais campos foram explícitos. Proponho um campo de presença por elemento, de visibilidade `pub(crate)`, com construtores públicos que o preenchem. Isso impede literais dessas structs fora de `typst_core`; dentro do repositório há 112 usos de `BlockElem {` e 32 de `TableElem {`, contagem textual que inclui a definição, implementações e testes. Não há variante nova em `Content`, `Value` ou `Style`, flag, fase ou dependência.

**Donos.** Núcleo novo `set/declared-properties.toml`, fixado por `eval/rules/set`, um dono novo `eval/rules/set/schema` e os donos de construtor, entidade, realização e layout dos cinco elementos. Cada dono de elemento declara a sua superfície de `set`. Os donos alterados passam ao formato curto.

## Erros do vanilla

Nenhum caso sondado mostrou comportamento comprovadamente errado do vanilla. Se a execução achar algum, ele não será copiado, e a prova vai para o dono da propriedade.

## Fora

- `gutter`, `column-gutter` e `row-gutter`: o layout de `grid` e `table` não os aplica nem no construtor. Pacote da família `grid`.
- `fill` de `set block` em títulos, figuras e parágrafos. Eles não são blocos no cristalino.
- Getters (`it.fill`) e `context table.fill`, hoje erro.
- Formas, `box`, listas, decorações, texto e os demais elementos: recebem A a D, e continuam sem efeito, com aviso.
- Argumentos que o construtor recusa, `datetime`, idioma das mensagens.

## Aceitação

1. L0 e Núcleo primeiro, selos e linhagem limpos.
2. RED por parte no CLI: as doze linhas da tabela, cada propriedade de F em SVG e PNG, e erro com posição para cada classe de entrada inválida.
3. Os 298 parâmetros sondados de novo: nenhum nome inválido aceito, nenhum valor válido no vanilla recusado por `set`, nenhum sem efeito sem aviso.
4. Propriedades de F: saída comparada por pixel com o construtor que tem o mesmo valor explícito, e com o vanilla onde o construtor já coincide.
5. Valor reutilizado em dois escopos, consulta, contadores e rótulos não mudam; cada corpo é avaliado uma vez.
6. Suíte completa, lint e linhagem reversa.

## Decisão pedida

Aprovar A–F, com o campo de presença. A alternativa é aprovar agora A–D, que só validam e avisam, e deixar E e F, que tocam entidades e layout, para depois.

## Execução

Árvore não commitada sobre `f2c737125`, com os pacotes de `calc` e de comparação: `git diff HEAD --stat` com 130 arquivos, mais os não rastreados. Binário `847f1c1eb28c33800e1fff913411c8aec4907d78818513102ee85a6ab7519f98`, 23:02 de Brasília. A árvore inclui também uma alteração de outra autoria em `layout/decorations.rs`, no seu L0 e no diagnóstico de gradiente de texto. A validação abaixo foi feita com ela presente.

- **RED:** cinco testes novos de CLI falharam antes do código e passam agora.
- **Suíte:** 10.942 passaram, zero falhas, sete ignorados, com `TMPDIR` em `/dev/shm` e `TYPST_VANILLA`. Lint sem erros nem avisos na cópia dos arquivos do repositório. Linhagem reversa: 1.031, zero erros.
- **Admissão:** 1.449 combinações de propriedade e valor, medidas antes dos últimos ajustes: 33 inválidas aceitas e 8 válidas recusadas. `document.title`, `outline.entry.fill` e cinco de `table.cell` foram corrigidas em seguida; a função em `table.align` segue recusada por contrato ratificado. As aceitas dependem do valor, não do tipo: padrão de numeração, nome de estilo, arquivo, posição, delimitador e tamanho de array.
- **Efeito visual**, com os mesmos casos da revisão: 60 propriedades com efeito nos dois (eram 46), 124 só com aviso e uma em silêncio.

### Ajustes em relação à proposta

- **C** usa os tipos que o próprio vanilla lista na mensagem de erro de cada parâmetro, em vez da conversão do construtor. É mais preciso e não depende do que o construtor aceita.
- **Presença:** só três elementos ganharam campo. `BlockElem` e `TableElem` guardam os omitidos; `HeadingElem`, a profundidade lógica. Figura e nota não precisaram.
- **`footnote.numbering`** já funcionava. O caso em silêncio é o padrão `"*"`, que a numeração do cristalino não reconhece em lugar nenhum.
- **Templates:** `#show: w` capturava a tabela com os estilos antigos. A inserção agora sobrepõe as declarações de tabela.
- **Títulos** leem `above`, `below` e `spacing` só do próprio `show heading: set block`. Um `set block` ambiente não os altera, como no vanilla.
- **`heading(offset:)` e `heading(depth:)`** passaram a valer, porque o offset declarado precisava deles.

### Limites

- O nível de título é resolvido na avaliação, não na inserção: um título guardado em `let` antes do `set heading(offset:)` conserva o nível. `include` recebe o offset. Nível limitado a 6, como antes.
- `height` fracionária em `set block` avisa. `radius` não combina por canto.
- Linha vertical sem `x` em tabela cujas colunas vêm de `set` fica na coluna antiga.
- Prévia de alinhamento de texto e medições internas não veem o bloco projetado atrás de um invólucro de estilo.
- `set block(fill:)` não alcança títulos, figuras nem citações.
- Um traço dado só por espessura ou só por cor fica meio traço fora da posição do `espessura + cor` em `block`. A diferença já existia no construtor.
- `math.cancel`, `math.accent`, `columns` e `image` seguem sem sonda visual válida.
