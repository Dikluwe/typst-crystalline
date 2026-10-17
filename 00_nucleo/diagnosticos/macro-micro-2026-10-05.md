# Tekt e vanilla: oportunidades de refinamento do macro ao micro

## Medições atuais e proveniência

Este estudo encontra oportunidades na materialização atual do Tekt, sem
alterar código produtivo ou L0 e sem declarar superioridade geral de um método.
As comparações usam o vanilla pinado
`9f2b6e8715237cb086899a42873660fe744622e8`, não um main móvel.

HEAD Tekt **`06848cdcb5450f2c555626bf116bc7acd82bd82b`**.
`git diff HEAD --stat` vazio antes/depois das coletas; os únicos não rastreados
eram os diagnósticos e sondas desta sessão, enumerados integralmente nos recibos.
O build release foi concluído antes da medição, com
`cargo build --release --locked --offline -p typst-wiring --bin typst`.
SHA-256 dos binários, idênticos antes/depois:

- Tekt: `eec9edd5ded17b0cd58c3297d313ff5796694899b39d5b87f6c5426f5dbc8e49`.
- Vanilla: `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`,
  build ratificado no P1489, árvore upstream
  `6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4`.

Coletas em **2026-10-05**, horário de São Paulo:

- Compilação isolada e GDB: **09:18:27–09:19:02** (`12:18:27–12:19:02 UTC`).
- Watch: **09:27:22–09:27:30** (`12:27:22–12:27:30 UTC`).
- Os controles de saída e traces vanilla têm seu horário no recibo separado.

[Sonda macro](macro-micro-2026-10-05/probe.py),
[sonda watch](macro-micro-2026-10-05/watch_probe.py) e
[controles/traces](macro-micro-2026-10-05/qualify.py) geram fixtures determinísticas
em `/tmp/tekt-macro-micro-2026-10-05/` e
`/tmp/tekt-macro-micro-watch-2026-10-05/`. Preservados no repositório:
[amostras macro](macro-micro-2026-10-05/results.json.gz),
[amostras watch](macro-micro-2026-10-05/watch-results.json.gz),
[qualificação](macro-micro-2026-10-05/qualification.json.gz) e
[hashes dos fontes/L0 inspecionados](macro-micro-2026-10-05/source-inventory.json.gz).
Os arquivos volumosos gerados permanecem locais conforme o `.gitignore` dos diagnósticos.

CPU 0, `SOURCE_DATE_EPOCH=0`, Rayon 1, vanilla `--jobs 1`, PDF sem tags e
`--pretty=false`. Por fixture macro: uma warmup por modo, seis pares de
processos normais e três sondas GDB intercaladas. A afinidade e opções são
iguais; a coleta curta em desktop ativo é exploratória, sem garantia de
precisão uniforme. Todos os processos próprios de watch foram terminados.

## Macro: vida dos recursos entre edições

O Tekt chama `run_compile_observed` em cada volta de watch
(`04_wiring/src/watch.rs:63`); essa função cria um novo `SystemWorld`
(`04_wiring/src/compile.rs:246`). O catálogo lazy só vive nesse world
(`03_infra/src/world/fonts.rs:104`). O vanilla cria o world antes do loop
(`lab/typst-original/crates/typst-cli/src/watch.rs:33`) e executa
`world.reset()` entre edições (`:77`), conservando fontes e biblioteca.
`world.rs:113–116` confirma que reset invalida arquivos e relógio, sem
reconstruir o catálogo de fontes. Há também memoização de eval/layout.

Dois casos de watch, cinco edições por motor no mesmo processo. Hello altera
texto visível; grid320 altera apenas um comentário final, mantendo o conteúdo.
Latência vai da escrita do fonte até observar um PDF completo publicado.
CPU é a diferença dos runtimes `schedstat` de todas as threads vivas do processo;
os conjuntos de threads permaneceram iguais entre as extremidades de cada amostra.
Espera/polling/debounce pertencem à latência, não à CPU.

| Edição | Latência Tekt | Latência vanilla | CPU Tekt | CPU vanilla |
| --- | ---: | ---: | ---: | ---: |
| Hello, mudança de texto | 205,11 ms | 101,77 ms | 165,17 ms | 1,27 ms |
| Grid 320, mudança de comentário | 259,12 ms | 102,89 ms | 217,91 ms | 2,17 ms |

CPU por edição variou de 164,28–167,69 ms no Hello Tekt e 1,19–1,47 ms no
vanilla. Grid Tekt: 215,74–223,06 ms; vanilla: 1,98–2,31 ms. O timer público
`eval_ms` do Tekt continua em 164,18/167,66 ms de mediana nesses dois casos,
coerente com a reconstrução do catálogo dentro da avaliação.
Texto de cada revisão coincide bilateralmente. A posição vertical do Hello
ainda tem o desvio descrito adiante; não é uma prova completa de render equivalente.

**Decisão:** esta é a primeira oportunidade de refinamento do fluxo de edição.
O lifetime atual desperdiça recursos que já foram adquiridos. Não atribuir toda
a diferença exclusivamente ao catálogo: retenção de biblioteca, fontes e
memoização também diferem. O ensaio causal seguinte deve conservar recursos
com invalidação explícita e testar mudanças em imports, assets, fontes,
inputs, features e relógio, além da sobrevivência da última saída após erro.
Reduzir o intervalo de polling sozinho não resolve o custo de CPU observado.

## Macro: primeira compilação e perfil do documento

Medianas de CLI normal, sem GDB. Incluem setup, recursos, compilação e escrita.
Delta é a mediana dos pares, não a diferença entre as medianas das colunas.

| Fixture | Tekt | Vanilla | Delta pareado |
| --- | ---: | ---: | ---: |
| Retângulo, sem texto | 10,99 ms | 11,81 ms | −1,08 ms |
| Hello, fonte explícita | 193,08 ms | 137,33 ms | +56,62 ms |
| 60 parágrafos, 2160 palavras repetidas | 276,34 ms | 140,29 ms | +136,53 ms |
| Grid 320 células | 251,54 ms | 164,32 ms | +85,82 ms |
| Cálculo, 1000 iterações | 11,25 ms | 11,10 ms | +0,10 ms |
| Cálculo, 10000 iterações | 22,67 ms | 13,35 ms | +9,24 ms |
| Cálculo, 30000 iterações | 47,57 ms | 18,38 ms | +29,34 ms |

O retângulo e os cálculos têm a mesma saída visível no controle raster e
os cálculos verificam o mesmo resultado por `assert` na própria linguagem.
Não afirmar vantagem estável do Tekt a partir do pequeno delta do retângulo.
Esses controles mostram que o custo não é uniforme entre workloads.

As fixtures textuais preservam texto, ordem e uma página nos dois motores.
Grid: máximo desvio vertical de 0,0002 pt e horizontal zero. Hello e
parágrafos: máximo desvio vertical de 0,658 pt e horizontal zero. Seus tempos
são comparações descritivas de cargas semelhantes, não uma declaração de
paridade geométrica completa. Nenhum resultado de matemática foi extrapolado.

## Do macro às unidades de trabalho

A sonda GDB mede entradas/retornos e CPU da thread inferior por `schedstat`,
excluindo CPU do debugger e tempo sem execução. Os tempos são **inclusivos**:
catálogo está dentro do primeiro eval; stdlib está dentro dos evals. Não somar
essas colunas a um total. Breakpoints podem afetar execução/caches; não se
deduz uma correção fixa do timing público. O breakpoint chamado `pdf` na sonda
cobre somente o export multifont: zero hits nos demais caminhos não significa
export gratuito. Para esses casos, usar os timers públicos e os traces.

| Unidade Tekt | Hello | Parágrafos | Grid 320 | Cálculo 30000 |
| --- | ---: | ---: | ---: | ---: |
| Catálogo completo, CPU | 171,94 ms | 168,69 ms | 180,54 ms | 0 ms |
| Descoberta de fontes de sistema, CPU | 162,81 ms | 159,74 ms | 171,54 ms | 0 ms |
| Duas montagens da stdlib, CPU total | 2,32 ms | 2,30 ms | 2,55 ms | 2,43 ms |
| Duas chamadas de layout, CPU total | 0,57 ms | 53,18 ms | 40,63 ms | 0,23 ms |
| Shaping final, CPU | 0,12 ms | 24,86 ms | 9,77 ms | 0,02 ms |
| Reconciliação, CPU | 0,02 ms | 2,10 ms | 0,36 ms | 0,02 ms |

Uma coleta separada de três traces vanilla por caso observou descoberta de
fontes em 117,44/119,54/122,63 ms de parede para Hello/parágrafos/grid.
Layout document nesses casos: 0,46/6,48/21,31 ms, com uma chamada em cada
amostra; o layout vanilla inclui shaping. Não subtrair diretamente o span
vanilla de uma fase Tekt com escopo ou instrumento diferente.

### Fontes: aquisição e cobertura

`03_infra/src/fontdb.rs:16–23` chama `font_info_from_bytes` e depois parseia
a mesma face outra vez para cobertura. `fonts.rs:339–340` confirma a primeira
parse. `fonts.rs:404–415` coleta codepoints em Vec; `font_book.rs:172–200`
já compacta intervalos antes de ordenar. Portanto a auditoria antiga de
sort/dedup não descreve integralmente o código atual.

O vanilla `FontInfo::new` usa uma face e constrói cobertura por subtable com
união (`lab/typst-original/crates/typst-library/src/text/font/info.rs:48–51`,
`:119–133`). Ambos usam fontdb 0.23 no catálogo; não há prova de que a escolha
do crate explique o delta. Candidatos micro: evitar a segunda parse,
diminuir os temporários de cobertura e medir composição/merge do catálogo.
O tempo exclusivo de cada candidato ainda não está separado; não prometer
que uma dessas mudanças removerá dezenas de ms.

### Avaliação: duplicação e custo dentro de cada passagem

`01_core/src/compiler/eval/mod.rs:872–880` executa a árvore primeiro sem show
e depois com show. `:806` monta stdlib dentro de cada passagem. A sonda
confirma duas chamadas de eval-pass e duas montagens da stdlib em cada fixture.
O L0 atual **exige** os carriers original/renderizado e a independência das
passagens; uma consolidação necessita atualização de L0 e classificação do gate.

No cálculo de 30000 iterações sem fontes, as passagens consomem aproximadamente
20,26 e 19,82 ms de CPU (medianas por posição). O trace vanilla registra um eval
de 8,65 ms de parede. Assim, a duplicação sozinha não explica tudo: cada
passagem Tekt também merece perfil de execução. O loop atual abre/fecha escopo,
liga o padrão, avalia o corpo e faz join por iteração
(`01_core/src/compiler/eval/control_flow.rs:150–180`). Estes são candidatos
para contagem de alocações e lookup, sem atribuição causal ainda.

Composição da biblioteca base uma vez por compilação é uma experiência mais
estreita que eliminar a avaliação dupla. Sua escala medida é de poucos ms;
não deve tomar precedência sobre retenção de recursos no watch.

### Layout e shaping: repetir e depois reconstruir

O vanilla valida as dependências introspectadas para decidir a repetição
(`lab/typst-original/crates/typst/src/lib.rs:145–161`), e memoiza eval e
unidades de layout. No Tekt, o loop L3 paginado é obrigatório no caminho
comum (`03_infra/src/pipeline/paged.rs:336–413`). A
[sonda anterior desta sessão](grid-pagination-cost-2026-10-05.md) mediu
15,15 ms de CPU no segundo layout de grid320, já com caches aquecidos.
Essa é uma parte do custo, não a explicação de todo o delta.

O texto denso também revelou shaping final relevante mesmo com palavras
repetidas. `03_infra/src/shaper/shaping.rs:135–233` resolve candidatas,
executa bidi e segmentação antes de consultar o cache de glifos em `:242`.
`shaper/traversal.rs:21–25` cria caches de runs/faces do documento separados
do cache de larguras usado no layout. Investigar custo de preparação e hits
por etapa antes de concluir que a rustybuzz é o gargalo. Unificar resultados
de medição/emissão exige provar identidade de fonte, idioma, script, direção,
eixos, tracking, contexto e região; os L0s atuais preservam fases distintas.

O agrupamento de linhas da reconciliação já teve protótipo sem ganho convincente
em `grid-line-group-and-bidi-cache-2026-10-03.md`. Com o pequeno custo observado
no grid atual, não é a primeira intervenção. As células ainda têm medição e
emissão próprias; reúso de frames continua hipótese dependente de região/contexto.

### Travessias de leitura que materializam árvores

`Content::map_content` reconstrói Sequence/Par e outros containers mesmo
quando o transform retorna None (`01_core/src/entities/content/traversal.rs:24–47`).
Há callers que só observam e descartam essa árvore:
`03_infra/src/pipeline.rs:275`, `pipeline/paged.rs:122`, `:129`, `:853` e
`pipeline/context_stabilization.rs:82`. Isto comprova construção desnecessária
para esses usos de leitura, mas ainda não sua importância no total.

Investigar visitor de leitura ou preservação de nós não alterados com contagem
de nós/alocações. O L0 de traversal atual governa transformação bottom-up:
qualquer proposta precisa preservar ordem, recursão exaustiva, primeira falha
e causalidade; adicionar uma API pública requer o gate ADR-0127.

## Correção encontrada durante a qualificação

A fixture `context` produz `Title / 0 Hello World` no Tekt e
`Title / 1 Hello World` no vanilla. Portanto **foi excluída da tabela de
comparações equivalentes**. Seus dados brutos foram preservados para reproduzir
o achado. Ambos fazem duas chamadas externas de layout nesta sonda; quantidade
de passagens não garante o snapshot contextual correto.

Essa divergência precisa de diagnóstico causal e teste RED antes de qualquer
atalho de contexto/paginação que atravesse esse comportamento. Esta tarefa
não corrigiu o contador nem atribuiu sua causa a uma unidade ainda não isolada.

## Ordem proposta de refinamento

1. **Watch e lifetime de recursos:** comparar uma sessão com invalidação correta
   e retenção de fontes/biblioteca. Medir CPU e latência separadamente, incluindo
   edição real e mudança de comentário. Preservar publicação após erros.
2. **Primeiro catálogo de fontes:** perfil interno de parsing, coverage, I/O e
   merge, seguido de ablações estreitas. Não reapresentar otimizações já incorporadas.
3. **Eval:** começar pela composição repetida da base; estudar a dupla avaliação
   e o custo por iteração com provas de show/introspecção e sem confundir os carriers.
4. **Texto/layout:** predicado causal de reutilização de candidato, preparação de
   shaping e frames de célula. Gate geométrico/linguístico antes do ganho de tempo.
5. **Travessias/alocações:** medir os observadores que hoje usam transformação;
   promover a prioridade apenas se o custo real justificar.

A divergência de contador é uma frente de correção separada e um requisito
antes de otimizar seu caminho contextual. Mudanças produtivas começam pelos
owners L0 e pelas ADRs; contratos públicos ou mudança de fase exigem aprovação.
Este diagnóstico não faz essas mudanças nem exige uma nova decisão arquitetural
para executar suas sondas externas.

O resultado atual sustenta uma agenda de refinamento concreta, mas não prova
que o Tekt possa superar o vanilla de forma geral. Recompilação com retenção
correta, compilação isolada, uso de memória e custo de manutenção são critérios
distintos; o próximo ganho precisa aparecer no workload correspondente e manter
os observáveis da linguagem.
