# Proposta de pacote: família de layout

**26 de novembro de 2026 · aprovado pelo dono em 26/11/2026 (“Aprovo A–E”, no chat) sob a [ADR-0127](../adr/adr-0127-l0-gate-stop-vs-continuous-flow.md) (§2.1, §2.4 e pacotes de paridade)**

É o quinto pacote da série do [contrato de `set`](set-contract-package-2026-11-24.pt-BR.md). As decorações e o texto continuam adiados enquanto `layout/decorations.rs` tiver a alteração de outra autoria sem commit.

## Os problemas principais

1. **`v` entre linhas de um parágrafo usa a entrelinha, não o espaço entre parágrafos.** `a`, `#v(10pt)`, `b` em linhas seguidas põe `b` 6,1pt acima do vanilla. `v(…, weak: true)` não absorve o espaço entre parágrafos, e `v(1fr)` dá erro.
2. **`place` dentro de um parágrafo não o interrompe.** Em `x #place[P] y`, o vanilla começa `y` num parágrafo novo; aqui `y` continua na linha de `x`.
3. **22 propriedades de `set` são ignoradas** em elementos cujo construtor já as aplica: `pad`, `place`, `stack`, `repeat`, `outline`, `quote`, `raw`, `figure.caption`, `pagebreak` e `linebreak`.

## Medição

HEAD `c1b4a1ebd`, árvore com a alteração de outra autoria em `layout/decorations` (`git diff HEAD --stat`: 5 arquivos, nenhum desta família). Cristalino `b891d6cf011e61f11eea0799a657ec74aed733ec624a3e9684e1003c18923ea8` (debug), vanilla `e36333ee02be2066646ad632996d92468af9a60b834c6e518ce00ccf3221bb3b`. Sonda de 01:02:34 a 01:02:39 de Brasília, raster de PDF a 144 ppi; posições de palavras pelo PDF logo depois, na mesma árvore. Script e saída em `lab/results/layout-package-2026-11-26/` (`lay.txt` SHA-256 `303d80b1f49c21b3b1b06cc30a111179c1651e8168ce5bbc679be33cbc528cec`), local e não versionado.

Dos 45 valores testados em 15 elementos, por `set` e por construtor:

| Resultado | `set` | construtor |
|---|---:|---:|
| igual ao vanilla | 15 | 25 |
| ignorado | 22 | — |
| erro só aqui | 1 | 6 |
| diferença visual | 2 | 6 |
| erro nos dois, por argumento mal escrito na sonda | 5 | 5 |

Construtor recusado só aqui: `raw(tab-size:)`, `raw(align:)`, `raw(theme:)`, `figure(gap:)`, `figure(outlined:)` e `outline(target:)` com seletor.

| Caso | Vanilla | Cristalino |
|---|---|---|
| `a`, `#v(10pt)`, `b` em linhas seguidas, y de `b` | 50,4 | 44,3 |
| o mesmo com `weak: true` | 37,2 | 44,3 |
| `a`, linha vazia, `#v(10pt, weak: true)`, linha vazia, `b` | 37,2 | 40,4 |
| `#v(1fr)` | ocupa o resto da página | erro |
| `x #place[P] y`, posição de `y` | linha nova, 40,4 | mesma linha |
| `raw("a\tb", block: true)` | tabulação expandida | sem espaço |
| `quote(block: true)` longo, x da segunda linha | 21 | 10 |
| `quote(attribution: [A])` em linha | sem atribuição | `— A` no fim |

`#v(10pt)` entre parágrafos separados por linha vazia e entre blocos já coincide.

## Causa

- `v` dentro de um parágrafo não fecha o parágrafo; o avanço seguinte usa a entrelinha. A forma fraca não colapsa com o espaço de parágrafo, e `v` só aceita comprimento.
- `place` é tratado como item de linha quando aparece no meio de texto.
- `rules/set.rs` não tem destino para as propriedades destes elementos, embora os donos dos construtores já tenham os campos.
- `raw` não expande tabulação nem aceita `tab-size`, `align` e `theme`; `figure` não aceita `gap` e `outlined`; `outline(target:)` só aceita texto.
- `quote` em bloco não recua as linhas seguintes, e a atribuição é escrita também em linha.

## Escopo proposto

| Parte | Mudança | Efeito |
|---|---|---|
| A | `v` fecha o parágrafo e soma ao espaço entre blocos; a forma fraca absorve espaços vizinhos; `v(1fr)` reparte o espaço restante como `h(1fr)` | corrige `v` em todo documento que o usa entre linhas |
| B | `place` sem `float` no meio de um parágrafo o interrompe e é posto no fluxo, como bloco | muda documentos com `place` dentro de texto |
| C | `set` de `pad`, `place`, `stack`, `repeat`, `outline`, `quote`, `raw`, `figure.caption`, `pagebreak` e `linebreak`, pelo mecanismo existente | 22 valores deixam de ser ignorados |
| D | construtores: `raw(tab-size, align, theme)` com tabulação expandida, `figure(gap, outlined)`, `outline(target:)` com seletor | 6 argumentos hoje recusados |
| E | `quote`: recuo do bloco em todas as linhas e atribuição só em bloco | muda citações em bloco longas e citações em linha com atribuição |

## Compatibilidade e contrato

A e B mudam a aparência de documentos que usam `v` entre linhas ou `place` dentro de texto. E muda citações. C e D só aplicam o que hoje é ignorado ou recusado; `v(1fr)` passa a valer. Nenhuma variante nova em `Content`, `Value` ou `Style`, flag, fase ou dependência; `raw(theme:)` aceita só `none` e `auto` se carregar temas exigir leitura de arquivo, e o resto fica declarado.

## Comportamentos do vanilla a decidir pela prova

| Caso | Vanilla | Tratamento proposto |
|---|---|---|
| `quote(attribution:)` em linha | descarta a atribuição sem aviso | **não comprovado:** a documentação diz que a atribuição só aparece em bloco. Seguir |
| `place` no meio do texto interrompe o parágrafo | quebra o parágrafo mesmo sem ocupar espaço | seguir: é a regra de bloco |

Nenhum caso medido mostrou comportamento comprovadamente errado. Se a execução achar algum, ele não será copiado.

## Fora

- Decorações e texto (`underline`, `overline`, `strike`, `highlight`, `sub`, `super`, `smallcaps`, `strong.delta` e as propriedades de `text`).
- Conteúdos de célula que ainda divergem (equação de bloco, `align`, `place`, `raw`, `figure`, bloco seguido de texto em sub-quadro).
- `columns` com contagem posicional e `columns.gutter`.
- `ref.supplement` e `figure.caption.separator`, que a sonda escreveu de forma inválida nos dois; serão medidos na execução.
- `typst eval` com contexto de estilos.

## Aceitação

1. L0s primeiro, com selos e linhagem limpos.
2. RED no CLI por parte, com posição de texto, SVG ou PNG e erro com posição.
3. Os 45 valores e os casos da tabela: cada um a até 30 pixels do vanilla, ou declarado com a causa.
4. `set` igual ao construtor, `show … set`, escopo e valor reutilizado.
5. `v` entre parágrafos separados por linha vazia e entre blocos, e `place` fora de parágrafo, iguais ao estado anterior.
6. Suíte completa, lint e linhagem reversa, com build em RAM.

## Decisão pedida

Aprovar A–E. A alternativa é aprovar só A e B, que corrigem posições erradas em documentos comuns, e C, que é mecânico, deixando D e E para depois.

## Execução

**26 de novembro de 2026.** HEAD `c1b4a1ebd`, árvore não commitada deste pacote e da alteração de outra autoria em `layout/decorations` (`git diff HEAD --stat`: 95 arquivos). Cristalino `b4b414bacfa10d49cd54c87e1e6fa36928bc9b6aeac98bb4446fcf2bf91e6a58` (debug, compilado em RAM), vanilla o mesmo da medição. Sondas de 01:50:55 a 01:51:07 de Brasília. Saídas em `lab/results/layout-package-2026-11-26/` (`lay-after.txt` SHA-256 `1c03477263082e5d4b601f47ddd88f99bf16a81d7984eac8fc8cbf815a5b7bbc`). Suíte completa 10.971 aprovados, 0 falhas, 7 ignorados; doc-tests à parte, 5 aprovados; lint sem violações; linhagem reversa 1.034 consumidores, 0 erros.

| Conjunto | Antes | Depois |
|---|---:|---:|
| `set` igual ao vanilla, dos 45 valores | 15 | 33 |
| construtor igual ao vanilla | 25 | 33 |
| `v` (18 casos) | — | 18 |
| `place` no texto (13) | — | 12 |
| `raw` (11) | — | 10 |
| posição da legenda (7) | — | 6 |
| `quote` (13) | — | 13 |

**Como ficou.**

- A: `v` fecha o parágrafo. A forma fraca fixa o espaço até o próximo bloco ou parágrafo e some no início da região. `v(1fr)` é um bloco fracionário sem espaço. `v` aceita valor negativo, como a linguagem.
- B: `place` interrompe o parágrafo. Como o vanilla só conta o espaço entre parágrafos se vier conteúdo depois, a interrupção é adiada até o próximo conteúdo, e o elemento desce junto. No fim do parágrafo, do bloco ou do sub-quadro, o espaço não existe.
- C: o `set` dos dez elementos é guardado na cadeia e lido no layout para o que a chamada não deu. Cada elemento marca no campo `given` o que veio na chamada. A posição `top` da legenda não existia nem pelo construtor; passou a existir.
- D: `raw` com `tab-size`, `align` e `theme: none`; `figure` com `gap` e `outlined`, e `outlined: false` tira a figura da lista de figuras; `outline(target:)` com seletor de títulos, figuras ou tabelas.
- E: `quote` em bloco com 2,4em acima, 1,8em abaixo, recuo de 1em nos dois lados em todas as linhas e atribuição alinhada ao fim; aspas automáticas; sem atribuição fora de bloco.

**Defeito do vanilla não copiado.** `raw(tab-size: 8)` no construtor não tem efeito no vanilla: a expansão lê só `set raw` (`styles.get(RawElem::tab_size)`, `text/raw.rs:989`). Aqui o valor do construtor vale.

**Limites que ficam.**

- `raw(theme:)` com arquivo de tema dá erro explícito: carregar arquivo é I/O.
- `outline(target:)` com outros seletores, como `heading.where(level: 2)`, dá erro explícito. A lista de figuras e de tabelas já renderizava diferente do vanilla (título, preenchimento, numeração por tipo) e continua assim.
- `set repeat` dentro de `box(width: 1fr)`: qualquer `set` que só gera propriedade personalizada desloca essa caixa. Já acontecia com `set par` antes do pacote.
- `place` dentro de célula de grade e tabela dentro de figura já divergiam e continuam.
- Dois `v` ou blocos fracionários com pesos diferentes (`1fr` e `2fr`) repartem o espaço errado; já era assim com blocos.
