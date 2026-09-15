# Passo 1406 — realizar `outline.entry` sob `show` antes do layout

**Estado:** implementado e verificado no recorte; diferença visual de fill
encaminhada ao passo seguinte.

**Gate ADR-0127:** confirmação humana explícita recebida em 2026-09-15.

## Objetivo

Fazer cada entry gerada por `outline()` atravessar as regras lexicais
`#show outline.entry`, inclusive filtros `where`, e substituir o layout textual
legado de headings pelo conteúdo realizado. Preservar `entry.indented`, o parent
privado, o default da linguagem e a convergência limitada sem executar closures
dentro do layouter.

## Proveniência congelada

- HEAD: `fe3209458abaa9177f41bbf4795755916c277ae8`.
- Referência vanila: `586e1bd43fae6c9a973218163d3165c53ab8d16d`.
- Binário cristalino: `target/debug/typst`, SHA-256
  `5b2bb9d2492228f9ea77893dcf911aec799b5895e8f9970e829921f8a6f1148b`.
- Binário vanila: `lab/typst-original/target/release/typst`, SHA-256
  `eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f`.
- Medição iniciada em `2026-09-15T08:24:45-03:00`.
- Working tree não commitado durante a medição: `git diff HEAD --stat` vazio e
  estes cinco ficheiros novos, ainda não rastreados:
  - `lab/parity/corpus/p1406/default.typ` —
    `e023645aa816a7db12ed0e99b9d2ea073aef4ddc7f03cec90a85543775aaf1e2`;
  - `lab/parity/corpus/p1406/custom.typ` —
    `987a24f28c4994bec5784a12050663922844066b5f889d9f10918ffe409da9d6`;
  - `lab/parity/corpus/p1406/where.typ` —
    `91463a4578106ce31bed89ed0d23f1852e0d908ea1ff74a5804f147c7640ada9`;
  - `lab/parity/corpus/p1406/parent.typ` —
    `d2c05c32d78831225afac1e941b2c7bb70ade564158a961e14890f2b57c0f86d`;
  - `lab/parity/corpus/p1406/lexical.typ` —
    `2590056ba0320d56b45399b1e47dea57715d9a0648c6542730412e681b4182b0`.
- Cada compilação bilateral foi limitada a 512 MiB, um thread e 15 segundos;
  stdout/stderr guardado abaixo de 64 KiB.

## Medição antes da decisão

| Fixture | Cristalino | Vanila ratificado |
|---|---|---|
| `default` | aceita, mas emite o título `Contents` apesar de `title: none` e usa linhas textuais legadas | omite o título e realiza entries com prefixo, corpo, fill e página |
| `custom` | rejeita `outline.entry` como seletor direto | aplica a receita a cada entry: `CUSTOM 1: Alpha`, `CUSTOM 2: Beta` |
| `where` | aceita a declaração, mas ela não produz efeito | transforma apenas a entry de nível 2 |
| `parent` | rejeita o seletor direto | `it.indented(it.prefix(), it.body())` funciona dentro da regra gerada |
| `lexical` | rejeita o seletor direto | a regra entre dois outlines altera apenas o segundo |

Fonte medida antes da classificação:

- `lab/typst-original/crates/typst-library/src/model/outline.rs:272-318`
  gera as entries depois da introspecção e preserva ordem, depth e `outlined`;
- `:397-407` injeta o parent privado e estilos relevantes na entry;
- `:481-521` separa campos públicos da referência privada ao parent;
- `lab/typst-original/crates/typst-layout/src/rules.rs:413-419` realiza o
  outline default como título seguido da sequência de entries;
- `:421-455` realiza a entry default por `prefix`, `inner`, `indented` e link;
- no cristalino, `01_core/src/compiler/layout/outline.rs` ainda constrói strings
  diretamente no layout;
- `01_core/src/compiler/introspect.rs:812` sintetiza entries consultáveis, mas
  não as insere no conteúdo a renderizar;
- `outline.entry.where(...)` já chega a `NativeElement`, enquanto o seletor
  direto fica como `Value::Func` sem a conversão equivalente;
- as pilhas de show rules são clonadas por escopo durante eval; o `Module` final
  não conserva uma única pilha que represente corretamente cada ocorrência.

## Classificação e inferências

- É paridade de linguagem: identidade do seletor, filtragem por campo, escopo
  lexical, morfologia do conteúdo e destino do link são observáveis Typst.
- A estrutura da store, a chave privada e o número de passes são mecânica livre,
  desde que terminem de forma limitada e preservem esses observáveis.
- **Inferência:** cada ocorrência de outline precisa transportar causalmente o
  snapshot lexical necessário à realização. É refutada se uma medição limitada
  demonstrar outra fonte já existente capaz de reconstruir exatamente o mesmo
  escopo sem tornar regras posteriores retroativas.
- **Inferência:** a realização deve acontecer depois de existirem entries
  introspectadas e antes do layout. É refutada se uma solução L1 demonstrar o
  mesmo comportamento, incluindo callbacks, parent e `prefix-info`, sem executar
  eval no layouter e sem antecipar dados contextuais.

## Contrato proposto

1. A função nativa `outline.entry` é seletor direto do mesmo elemento já
   reconhecido por `outline.entry.where(...)`; identidade é pelo ponteiro nativo,
   nunca pelo nome textual.
2. Cada `outline()` de headings preserva um carrier privado do snapshot lexical
   de show rules válido no seu ponto de construção. Uma regra posterior não age
   retroativamente e uma regra anterior age apenas enquanto estiver no escopo.
3. Depois da síntese introspectiva das entries e antes do layout, L1 realiza a
   sequência renderizável daquele outline em ordem documental e dentro de
   `depth`. L3 apenas orquestra a nova fronteira e não contém regra de negócio.
4. Durante cada transformação, a cadeia privada fornece o indent do parent e uma
   chave opaca estável daquele outline. O parent não é armazenado em
   `OutlineEntry`: entry direta ou recuperada por query continua sem parent.
5. `where(level: n)` e os demais campos públicos medidos filtram a entry gerada
   antes de aplicar a receita. Cada regra aplicável age uma vez segundo a ordem,
   revogação, guardas e teto já pertencentes a `eval/rules`.
6. Sem receita transformacional que substitua a entry, o show default compõe
   `link(element.location, entry.indented(entry.prefix(), entry.inner()))`.
   Prefixo, corpo, fill e página permanecem Content; não se assam strings.
7. Título omitido usa o default localizado vigente. `title: none` omite o título.
   A entidade deve preservar essa distinção sem expor novo campo Typst.
8. O layout de outline de headings recebe e dispõe apenas o corpo já realizado;
   não chama função Typst, não recompõe entries e não volta ao caminho textual.
9. A síntese consultável permanece uma vez por entry e por outline. Realizar o
   corpo não adiciona ocorrências de query nem muda Locations causais.
10. Página e máximos de `prefix-info` participam do fixpoint limitado vigente.
    Uma mudança invalida e refaz o candidato; o teto continua finito e excedê-lo
    é erro de não convergência, nunca sucesso silencioso.

## Fora do escopo e Unknown

- Entries de equação e targets de figuras/tabelas, pois o cristalino ainda
  sintetiza apenas entries de headings neste caminho.
- Texto exato/localização do `alt` PDF; o link causal é obrigação, mas o wording
  permanece Unknown até medição própria.
- Igualdade de bytes, escolha de containers Rust e algoritmo do fixpoint.
- Paridade geral de `show`, `query`, `#show heading: none` ou render de todas as
  propriedades tipográficas.

## Testes RED → GREEN

1. seletor direto transforma duas entries e entrega níveis corretos;
2. `where(level: 2)` transforma só a segunda entry;
3. regra entre dois outlines não altera o primeiro;
4. `indented` funciona somente na realização com parent e continua falhando em
   entry obtida diretamente/query;
5. default usa `prefix`, `inner`, `indented` e link para a Location do heading;
6. `title: none` difere de título omitido;
7. dois outlines preservam multiplicidade consultável sem duplicação adicional;
8. depth, `outlined:false`, ordem e fill/página permanecem corretos;
9. mudança de máximo de prefixo ou página converge dentro do teto;
10. teto excedido e callback inválido falham de modo limitado.

## Ataques obrigatórios

- reconhecer a função pelo nome em vez da identidade;
- fazer `where` aceitar a declaração mas não tocar a entry gerada;
- usar a pilha final/global e tornar regra posterior retroativa;
- armazenar parent na entry consultável;
- aplicar default em strings sem passar por helpers;
- tratar `title: none` como argumento omitido;
- executar closure dentro do layout;
- aplicar a mesma regra duas vezes ao reprocessar o fixpoint;
- duplicar entries consultáveis durante realização;
- perder link, ordem, depth ou filtro `outlined`;
- aceitar documento parcial ao atingir o teto.

Todos os mutantes materialmente representáveis devem ser mortos; score exigido
`1.0`. Sem isolamento entre autor e adversário, o resultado deve ser declarado
“executado sem atestação de isolamento”.

## Limites de recursos

- Probes: `ulimit -v 524288`, `RAYON_NUM_THREADS=1`, timeout 15 s e captura
  máxima de 64 KiB.
- Builds/testes focados: `ulimit -v 2097152`, `CARGO_BUILD_JOBS=1`, incremental
  e debug info desligados, 512 codegen units e linker gold.
- Proibido neste passo: `cargo test -p typst-core --lib` monolítico e
  `cargo check --tests`.

## Gate ADR-0127

Este contrato amplia o seletor público, corrige o comportamento default e cria
uma fase de realização pós-introspecção/pré-layout. Confirmação humana explícita
recebida em 2026-09-15; implementação produtiva e resselo estão autorizados.

## Execução e evidência final

- Estado medido ao fechar: HEAD ainda
  `fe3209458abaa9177f41bbf4795755916c277ae8`, working tree não commitado com
  os ficheiros listados por `git diff HEAD --stat`; hora
  `2026-09-15T10:42:11-03:00`.
- Binário cristalino final usado nas provas: SHA-256
  `c64958da6386a012dcde83678f44917ac7af7b03b10db7b2223a4a2abc249d28`.
- O seletor direto passou a reconhecer `outline.entry` pela identidade da
  função nativa, incluindo aliases; `where(level: 2)` atua sobre a entry
  gerada.
- Cada `Outline` captura regras e estilos no seu ponto lexical. A realização
  ocorre em L1 depois da introspecção e antes do layout; L3 somente instancia o
  engine e repete no máximo uma vez quando o máximo de prefixo muda.
- O corpo realizado permanece privado dentro do próprio nó `Outline`. Esta
  preservação de identidade foi uma correção produzida pelo ataque bilateral:
  substituir o nó por uma `Sequence` deslocava Locations e retirava a
  numeração dos headings-fonte.
- `title: none` é distinto de argumento omitido; parent só existe durante a
  realização; entries consultáveis não são duplicadas; `depth` e
  `outlined: false` filtram antes da receita; o caminho default emite destino
  interno.
- Testes focais finais: **8 aprovados, 0 falharam**, com 928 testes filtrados na
  primeira execução de 7 casos e 935 filtrados na execução isolada do oitavo.
- Provas bilaterais `default`, `custom`, `where`, `parent` e `lexical`: **5/5**
  terminaram com código zero e stderr vazio em ambos os binários, sob 512 MiB,
  um thread e timeout de 15 s. Regras direta, filtrada, lexical e parent têm a
  mesma morfologia textual observada no vanila; headings-fonte conservaram a
  numeração.
- O vanila expande visualmente o fill pontilhado até à página; o cristalino
  ainda dispõe apenas uma marca de fill seguida do número. O carrier continua
  sendo `Content::Repeat`, portanto esta diferença não reabre a realização das
  regras, mas permanece dívida verificável de layout e não é declarada em
  paridade por este passo.
- `cargo fmt --check`: código zero.
- `cargo build --bin typst`: código zero sob teto rígido de 2 GiB, um job,
  incremental/debug desligados e 512 codegen units.
- `crystalline-lint --fix-hashes .`: nove owners ressellados e zero drift.
- `crystalline-lint .`: código zero, sem violations; os avisos informativos
  preexistentes de complexidade/exaustividade permanecem fora do recorte.

## Ataques e veredito

Os oito mutantes materializados no recorte foram mortos: identidade por nome,
`where` inerte, snapshot global, parent persistido/indisponível na realização,
confusão entre título ausente e `none`, substituição destrutiva da identidade do
nó, duplicação consultável e perda de link/filtros. **Score: 8/8 = 1.0.** Os
tetos do pipeline permaneceram finitos; não houve aumento do limite de memória.

Uma tentativa acidental do teste monolítico proibido de `typst-core` atingiu o
teto rígido de 2 GiB no linker e foi abortada. Ela não foi repetida, não gerou
core e não foi usada como evidência. Todas as verificações aceites acima são
focais e limitadas.

**Veredito:** `PASS` para realização lexical de `outline.entry`, com a expansão
visual do fill explicitamente `Unknown`/aberta para o próximo passo. Protocolo
Tekt executado sem atestação de isolamento.
