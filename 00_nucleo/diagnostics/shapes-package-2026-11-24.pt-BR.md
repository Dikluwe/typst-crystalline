# Proposta de pacote: formas e `box`

**24 de novembro de 2026 · aprovado pelo dono em 24/11/2026 (“Aprovo A–F”, no chat) sob a [ADR-0127](../adr/adr-0127-l0-gate-stop-vs-continuous-flow.md) (§2.1, §2.4 e pacotes de paridade)**

A aprovação cobre o escopo abaixo. É o segundo pacote da série do [contrato de `set`](set-contract-package-2026-11-24.pt-BR.md).

## O problema principal

**O corpo das formas é descartado em silêncio.** `#rect[TEXTO]`, `square`, `circle` e `ellipse` desenham o contorno e perdem o texto, as notas, os contadores, os títulos e os rótulos que estão dentro. Nada é avisado. `box` e `block` conservam o corpo.

## Medição

HEAD `f2c737125`, árvore não commitada dos três pacotes anteriores e de uma alteração de outra autoria em `layout/decorations`. Cristalino `847f1c1eb28c33800e1fff913411c8aec4907d78818513102ee85a6ab7519f98`, vanilla `e36333ee02be2066646ad632996d92468af9a60b834c6e518ce00ccf3221bb3b`. 167 combinações de propriedade e valor em SVG, de 23:03:58 a 23:04:09 de Brasília; sondas de texto e de medida a partir de 23:04:37. Script e saídas em `lab/results/shapes-package-2026-11-24/` (`shp.json` SHA-256 `fa6c4cf90a8b49d4eb700109abca1979aa6a493080d34a87263efe95b349ae1e`), local e não versionado.

| Resultado no construtor | Casos |
|---|---:|
| Mesmas cores e espessuras no SVG | 50 |
| Cores ou espessuras diferentes; nas formas com corpo, a diferença inclui o texto perdido | 62 |
| Só o cristalino recusa | 46 |
| Só o cristalino ignora | 6 |
| O vanilla também recusa | 3 |

Por `set`, 129 combinações têm efeito no vanilla e nenhuma tem no cristalino.

| Medida | Vanilla | Cristalino |
|---|---|---|
| `rect[abc]` | 25,27 × 17,24 | 45 × 30 |
| `square[abc]`, `square()` | 25,27 × 25,27; 30 × 30 | largura infinita; erro |
| `circle[abc]`, `circle()` | 35,73 × 35,73; 30 × 30 | largura infinita × 0 |
| `ellipse[abc]` | 35,73 × 24,38 | largura infinita × 0 |
| `rect(width: 60pt)[abc]` | 60 × 17,24 | 60 × 30 |
| `line()` | 30 × 0 | 0 × 0 |
| `line(length: 50%)` sem região | 0 × 0 | largura infinita |
| `box(baseline: 5pt)[a]` | 5,03 × 7,24 | 5,03 × 12,24 |
| `rect([a], [b])`, `rect(1)` | erro | aceito |

## Causa

- `ShapeElem` não tem corpo, e os construtores leem só argumentos nomeados. Argumento posicional é ignorado. Os L0s registram o corpo como pendência, sem exigir aviso.
- `ShapeOrigin` só distingue `Geometry` e `Circle`; `set rect` não tem como não alcançar `square`.
- `intrinsic_defaults` só conhece retângulo vazio; as outras formas usam a largura disponível.

## Escopo proposto

| Parte | Mudança | Efeito |
|---|---|---|
| A | corpo em `rect`, `square`, `circle` e `ellipse`: conservado, medido, pintado sobre o fundo, percorrido por introspecção, `show`, rótulos e notas | conteúdo hoje perdido aparece; posição do que vem depois muda |
| B | tamanho intrínseco: `auto` segue o corpo com `inset` de 5pt; sem corpo, 45 × 30, ou 30 × 30 nas quadradas; região infinita não vira largura infinita | medidas mudam |
| C | construtores: `inset`, `outset`, `square.size`, `circle.width` e `height`, `stroke: auto`, `none` e por lado; posicional excedente ou de tipo errado é erro | aceita mais; entrada inválida vira erro |
| D | identidade de cada forma, `show` por ela (hoje só `circle` e `box` casam; `show rect: it => …` não tem efeito) e `set` das sete formas, com o mecanismo já existente; `stroke: auto` segue o `fill` efetivo | 129 combinações deixam de ser ignoradas, menos as de E e F |
| E | `box`: `set`, larguras `auto` e relativa, `inset` relativo, `baseline` sem aumentar a altura e por alinhamento, traço por lado e dicionário | aceita mais; linhas com `baseline` mudam |
| F | `line` sem argumentos mede 30pt e, com comprimento relativo sem região, mede 0 | hoje 0 e infinito. O L0 de `ShapeElem` registra o 0 como deliberado; esta parte o substitui |

**Precedência que o pacote segue.** Campo explícito vence. `radius` de `circle` e `size` de `square` equivalem a `width` e `height`. `stroke` combina por lado e componente, `inset` e `outset` por lado, `stroke: auto` explícito volta ao padrão.

## Compatibilidade e contrato

A e B mudam a aparência de todo documento que usa formas com corpo. C faz `rect(1)` e afins errarem. D a F mudam documentos com regras hoje ignoradas.

Contrato público: `ShapeElem` ganha corpo, `inset`, `outset` e registro de omitidos, e `BoxedElem` ganha o registro; os campos são `pub(crate)`, o que impede literais fora de `typst_core`. `ShapeOrigin` ganha cinco variantes públicas. `Content::Shape` deixa de ser folha: há 203 ocorrências textuais em L1 e dez em L3 a revisar, porque um percurso que a trate como folha perderia conteúdo de novo. Nenhuma variante nova em `Content`, `Value` ou `Style`, flag, fase ou dependência.

## Comportamentos do vanilla a decidir pela prova

| Caso | Vanilla | Tratamento proposto |
|---|---|---|
| `circle(radius: -5pt)` | mede 0 × −10pt | erro comprovado: medida negativa. Não copiar; dimensão negativa é tratada como 0 |
| `rect(width: -5pt)` | 0 × 5pt | igual ao tratamento acima |
| `set circle(radius: 20pt)` e `circle(width: 10pt)` | 10 × 40, um círculo não redondo | **não comprovado:** decorre de `radius` definir as duas dimensões. Seguir o vanilla |
| forma em HTML | perde o texto | perda de conteúdo. Não copiar; manter o erro |
| `square(width: 40pt, height: 20pt)` | 40 × 20 | idem |

## Fora

- Gradiente e tiling em `box.fill` e `box.stroke`: o portador de `box` só leva cor sólida. Avisam em `set`, e o construtor continua recusando. O mesmo vale para `block`.
- Quebra do corpo de uma forma entre páginas: o vanilla também trata formas como indivisíveis.
- Getters de formas além de `body`, `show … where` por campos, e as famílias restantes.
- Formas em HTML: o cristalino recusa com erro e o vanilla compila sem o texto. Conservar o erro, que não perde conteúdo em silêncio.
- Idioma das mensagens fora das que o pacote cria.

## Aceitação

1. L0s e Núcleo primeiro; selos e linhagem limpos. Donos alterados no formato curto.
2. RED no CLI por parte: texto extraído, medida, SVG e PNG contra o vanilla, e erro com posição.
3. Corpo com nota, contador, estado, título com rótulo, referência, link, imagem, tabela, forma aninhada, RTL, `show`, `context` e repetição em cabeçalho: cada efeito uma vez, em PDF, SVG e PNG.
4. Todos os percursos que hoje tratam `Content::Shape` como folha revistos um a um, com teste do que percorrem.
5. Propriedades por `set` iguais ao construtor equivalente; controles sólidos e formas sem corpo iguais ao estado anterior, pixel a pixel.
6. As 167 combinações e as medidas de novo; toda diferença restante declarada.
7. Suíte completa, lint e linhagem reversa.

## Decisão pedida

Aprovar A–F. A alternativa é aprovar agora A–C, que corrigem a perda de conteúdo, e deixar D, E e F para depois.

## Execução

Árvore não commitada sobre `f2c737125`, com os pacotes anteriores e a alteração de outra autoria em `layout/decorations`: `git diff HEAD --stat` com 184 arquivos, mais os não rastreados. Binário `b5b38cc53d605a5dfb68e2408be10b5f1d8f6e14d8393a08f52f53aa1c2769a7`, 23:41 de Brasília. A saída bruta da medição inicial (`shp.json`) foi substituída por `shp-after.json`; a tabela acima conserva os números.

- **RED:** quatro testes novos de CLI falharam antes do código e passam agora. O quinto, de HTML, já passava e protege o erro.
- **Suíte:** 10.950 passaram, zero falhas, sete ignorados, com `TMPDIR` em `/dev/shm` e `TYPST_VANILLA`. Lint sem erros nem avisos na cópia dos arquivos. Linhagem reversa: 1.032, zero erros.
- **167 combinações:** 159 com a mesma tinta do vanilla (eram 50), três recusadas só aqui (eram 46), nenhuma ignorada. Duas diferem só na forma do traço no SVG (`radius` em um canto) e três o vanilla também recusa. Por `set`, 126 das 129 têm efeito.
- **Pixel, PNG a 144 ppi contra o vanilla:** 213 casos, com 198 a até 30 pixels de diferença forte, medidos antes dos últimos ajustes de lint e repetidos depois com o mesmo total.

### Ajustes em relação à proposta

- **Representação.** O corpo vive em um `Block` com contorno, e não em `ShapeElem`. Todo percurso que já desce em `Block` passou a ver o corpo, sem revisar os 213 pontos. `Content::Shape` continua folha, e só existe sem corpo.
- **Medida de `circle(width: 10pt)` sob `set circle(radius: 20pt)`:** o vanilla ocupa 10 × 40 e pinta um círculo redondo de 10. A proposta dizia que o círculo saía não redondo, o que estava errado. O cristalino faz o mesmo.
- **`curve`:** qualquer regra `show` ativa quebrava toda `curve` com `expected content, found content`. Corrigido.
- **Retângulos em RTL** sem corpo também passaram para o lado direito, como no vanilla.
- **Testes trocados:** tamanho posicional de `square`, `circle(radius: auto)` igual a zero e `line()` de comprimento zero fixavam o contrato que a aprovação substituiu.

### Divergências e limites

- Dimensão negativa ou infinita vale 0. Forma com corpo em HTML continua erro. `#par(justify: true)[#rect[a]]` some no vanilla; o cristalino conserva o retângulo.
- `box` com gradiente ou tiling continua recusada no construtor e avisa em `set`. `radius` não combina por canto.
- Lista, título e equação de bloco dentro de forma saem maiores que no vanilla. `box` com o mesmo corpo tem o mesmo defeito: é da composição em região isolada.
- `set align` não tem efeito, então não alinha dentro de formas. Um círculo menor que o seu espaço interno posiciona o corpo um pouco diferente.
- `square(fill: red)[a]` e `align(right, circle[a])` diferem em 68 e 59 pixels, sem causa isolada.
- Corpo de forma não quebra entre páginas, como no vanilla.
