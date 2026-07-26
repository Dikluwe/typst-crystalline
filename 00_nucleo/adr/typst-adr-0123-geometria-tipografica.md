# ADR-0123 — Geometria tipográfica como categoria própria: fidelidade literal ao vanilla, não mecânica divergente

**Estado:** `PROPOSTO`
**Data:** 2026-07-25
**Histórico de numeração:** proposta inicialmente como `0112`, que **colidia de facto** com
`ADR-0112` já existente (`rust_decimal em L1`, P399) — renumerada para `0123`, sequencial depois de
`ADR-0122` nesta mesma remessa de organização. `0119` ficou vazio entre `ADR-0118` e `ADR-0120`;
`0121`/`0122` foram atribuídos nesta remessa a duas ADRs que não tinham número. Confirmar por
varredura real de `00_nucleo/adr/` antes de commitar — esta ADR é exatamente o tipo de erro que
`ADR-0109`/`ADR-0110` já avisam para não cometer ("nunca assumir número sem varrer"), e foi
cometido por mim ao propor `0112` sem ter visto o diretório real.
**ADRs relacionadas:** ADR-0107 (paridade é com a língua, não a mecânica — esta ADR **não a
revoga**, **estreita** o que conta como "mecânica livre"), ADR-0108 (medir antes de decidir —
esta ADR generaliza a disciplina de medição para o momento de **escrita original**, não só de
correcção), ADR-0109 (atomização — esta ADR é ortogonal, não conflita).

---

## Contexto

`ADR-0107` estabeleceu que a paridade do cristalino com o vanilla é com a **linguagem**
(semântica/sintaxe/morfologia) — a estrutura de dados, o algoritmo, os bytes de saída **divergem
de propósito**. Essa regra foi escrita tarde no projeto (já com muito código escrito), por uma
razão deliberada e correcta na altura: sem a distinção nomeada entre "copiar geometria" e "copiar
acoplamento", copiar qualquer coisa do vanilla arriscava trazer também a estrutura monolítica que o
projeto existe para evitar. Evitar copiar, por inteiro, era a decisão prudente disponível.

Essa prudência teve um custo que só apareceu depois, numa frente de trabalho específica — layout
matemático (`math/layout/`). Três módulos distintos, escritos em passos diferentes (`root.rs`
P37/P40, `frac.rs` P37, `underover.rs` P297), **cada um inventou independentemente a própria
convenção geométrica** em vez de portar a fórmula do vanilla, e **os três erraram do mesmo jeito**:

- `root.rs` (corrigido P901): assumiu `y=0` = topo da caixa; a convenção real (confirmada por
  leitura de `hconcat_spaced`/`layout_equation`) é `y=0` = baseline própria de cada `MathBox`.
- `frac.rs` (corrigido P905): **o mesmo erro exacto**, na mesma família de convenção — nunca
  auditado contra a correcção que P901 já tinha estabelecido, apesar de serem módulos irmãos no
  mesmo subsistema.
- `underover.rs` (corrigido P906): empilha por `height()` sem nenhuma constante de gap da tabela
  MATH; o vanilla usa `underbar_vertical_gap`/`overbar_vertical_gap` explícitos da fonte.

Nenhum destes três é uma "divergência arquitectural com ganho". Não existe um jeito cristalino
diferente, e igualmente válido, de posicionar uma barra de fracção — é aritmética que codifica
convenção tipográfica estabelecida (a mesma que o OpenType MATH spec formaliza, e que o vanilla
também só está a implementar, não a inventar). Divergir aqui não compra nada; só produz bug,
descoberto tarde, um módulo de cada vez.

## Decisão

Nomeia-se uma terceira categoria, distinta das duas que `ADR-0107` já nomeia:

| Categoria | Regra | Exemplo |
|---|---|---|
| **Língua** (semântica/sintaxe/morfologia) | Paridade obrigatória — o observável ao utilizador | `center + bottom` combina os dois eixos |
| **Mecânica** (estrutura de dados, algoritmo, bytes) | Divergência livre, de propósito | `Align2D` struct em vez de `Alignment` enum |
| **Geometria tipográfica** (posições, offsets, gaps, constantes de layout) | **Portar a fórmula literal do vanilla antes de escrever código próprio** — não é mecânica livre, é parte do contrato observável | posição da barra de fracção, gap de `underover`, constantes de `MathConstants` |

**Critério de pertença à categoria "geometria tipográfica"**: qualquer cálculo cujo resultado é
uma posição, distância, proporção ou dimensão que afecta directamente onde tinta aparece na
página — não a estrutura que representa o conceito, mas a aritmética que decide "quantos pontos
daqui para ali". Se a pergunta "existe um jeito diferente, mas igualmente correcto, de calcular
isto?" tem resposta não-óbvia ou negativa, é geometria tipográfica, não mecânica.

**Regra prática, aplicável na Fase A de qualquer passo que escreva ou altere geometria de
layout**: antes de escrever a fórmula em Rust, ler o código-fonte real do vanilla
(`lab/typst-original/`) que produz o mesmo resultado, e portar a aritmética — traduzida para a
convenção de tipos/injeção de dependência do cristalino (ex.: consumir `MathConstants` via trait,
não hardcode), mas **numericamente fiel**. Isto não é "confirmar depois que bate" (validação
empírica pós-hoc, que os três bugs acima já mostraram ser insuficiente sozinha) — é **ler antes de
derivar**, mesmo disciplina de `ADR-0108` aplicada ao momento de escrita original, não só de
correcção de bug.

## Porquê isto não conflita com atomização nem com a estrutura de dados livre

Atomização (`ADR-0109`) é sobre **onde** o código mora — cada módulo (`frac.rs`, `root.rs`,
`underover.rs`) continua a existir, continua legível sozinho, continua a consumir `MathConstants`
por injecção de trait, sem I/O, sem acoplamento novo. Geometria tipográfica fiel ao vanilla é
sobre **o que** esse código calcula. Os dois eixos são independentes: pode-se (deve-se) ter os
dois ao mesmo tempo — fórmula idêntica ao vanilla, dentro de um arquivo atomizado.

A estrutura de dados (`ADR-0107`, categoria "mecânica") continua livre para divergir — que
`Content` variant existe, que campos tem, como o dispatch funciona. O que esta ADR retira da
categoria "mecânica" é só a aritmética de posicionamento — que nunca devia lá ter estado, porque
não há benefício arquitectural em reinventá-la.

## Sobre a taxonomia mais ampla

Esta ADR nomeia uma categoria específica (geometria tipográfica) porque há evidência concreta de
três bugs reais causados pela ausência dela. **Não pretende fechar a taxonomia completa do
projeto.** O dono já observou que a distinção entre "língua", "mecânica" e outras categorias ainda
por nomear é um trabalho em curso, a refinar por experiência acumulada — do mesmo jeito que
`ADR-0107`/`ADR-0108`/`ADR-0109` surgiram cada uma de um padrão de erro real observado, não de
um desenho completo antecipado. Esta ADR segue o mesmo método: nomeia o que já dói, deixa o resto
para quando a próxima categoria se tornar visível pela mesma via — um padrão de erro repetido,
não especulação antecipada.

## Alternativas consideradas

| Alternativa | Razão rejeitada |
|---|---|
| Tratar isto como adendo a `ADR-0107` em vez de ADR nova | É uma decisão nova (estreita o que conta como mecânica), não uma clarificação do texto já escrito — merece registo próprio para citação precisa, mesmo precedente de `ADR-0065`/`ADR-0085` (separar por âmbito em vez de misturar). |
| Exigir portar literalmente TODA a geometria do vanilla, incluindo escolhas de performance internas | Rejeitado — o critério é sobre resultado observável (posição final), não sobre a estrutura de dados que carrega esse resultado. `MathConstants` continua injectado por trait, não precisa replicar a arquitectura de cache do vanilla. |
| Esperar por mais casos antes de nomear a categoria | Já há N=3 (root/frac/underover), mesmo tipo de erro, três passos separados — limiar de padrão repetido já ultrapassado, mesmo critério usado por outras ADRs deste projeto (ex.: `ADR-0065`, N=5). |

## Próximo passo natural (não implementado por esta ADR)

Auditar os módulos de `math/layout/` que ainda não passaram por esta checagem
(`attach.rs`/`matrix.rs`/`cases.rs`/`delimited.rs`/`stretchy.rs`/`assembly.rs`) contra a fórmula
real do vanilla, e considerar extrair um núcleo geométrico partilhado (empilhar com gap,
centrar por baseline, aplicar overline) dentro de `math/layout/_comum.md`, para que a fidelidade
seja auditada uma vez, não módulo a módulo.
