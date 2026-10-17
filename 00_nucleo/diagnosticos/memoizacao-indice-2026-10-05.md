# Memoização do índice semântico — primeira implementação

O Module conserva o índice já produzido pela realização. PDF, HTML e consultas aproveitam esse resultado inicial; mudanças de conteúdo invalidam o snapshot. Consultas de 500 e 2.000 headings transformados ficaram 17,2% e 22,0% mais rápidas nesta rodada. A compilação completa não mostrou ganho convincente na tabela.

## Medição estrutural antes da decisão

Na árvore da avaliação única, `01_core/src/compiler/realize/mod.rs:197–200` produzia o índice, e `01_core/src/compiler/eval/mod.rs:888–896` descartava-o. Paged, HTML e query reconstruíam o índice inicial. Essa evidência de fonte legitimou a investigação; o ganho de tempo só foi decidido pelo ensaio final abaixo.

O Hash/PartialEq genérico de Content projeta Realized no visual. Logo, uma chave que usasse apenas essa igualdade/hash poderia confundir origens ou Locations distintas com visual igual. Isso é insuficiente para o índice semântico. Não foi alterada a igualdade da linguagem para acomodar o cache.

O gate ADR-0127 para o getter público foi confirmado pelo usuário em 2026-10-05: L0 `00_nucleo/prompts/entities/module.md`, SHA-256 inicial `b1499e45b2c79e61ceb0b4e6f079caf639ae610ffa09a37bab53c057edd89e54`. Os owners de eval, realização e seus consumers foram atualizados antes de código; o [passo](../materialization/memoizacao-indice-2026-10-05.md) registra o escopo. Refinamentos e hashes finais estão nos recibos.

## Implementação e validade

`ModuleInner` recebe um campo privado `Option<Arc<TagIntrospector>>` e expõe `Module::realization_introspector()`. É memoização do resultado associada à avaliação dona do Module: a identidade do módulo é a chave implícita e o Arc conserva lifetime e snapshot. Não é um cache global, entre avaliações ou entre Worlds; não usa endereço cru, relógio, unsafe ou igualdade visual como chave.

A fachada aplica o helper vigente de colunas antes de construir o índice, de modo que índice e conteúdo sejam publicados a partir da mesma árvore final. Eval conserva visual e introspector, publica os carriers e só depois instala o índice. Sem receitas, o campo permanece ausente e os callers usam o caminho anterior.

Os setters efetivos de content/introspection_content invalidam o snapshot. Query o utiliza por empréstimo, sem clonar stores. PDF e HTML clonam antes de acrescentar resultados da tentativa. Bibliografia e metadata mantêm suas injeções vigentes; páginas, posições e callbacks posteriores não são armazenados no índice inicial. Bundle conserva suas sementes e resets por documento e não usa esse snapshot do root como substituto do seu algoritmo.

Foram alterados sete consumers nesta etapa: module, eval, testes de eval, fachada de realização, paged, HTML e query_helpers. A linhagem do conjunto pendente anterior também foi revalidada, sem alteração de algoritmo em owners externos a esta etapa.

## RED→GREEN e compatibilidade

Duas falhas RED reproduziram o descarte do resultado na publicação e a ausência de invalidação. Depois da correção, passaram os quatro testes novos: índice de originais/gerados após colunas, fallback sem receitas, invalidação por ambos os carriers e isolamento do snapshot em clones/tentativas.

Passaram **6.508 testes de núcleo** (5 ignorados), **1.143 de infraestrutura**, build release e `git diff --check`. O adapter histórico que projeta visual invalida o índice por seu setter; as novas sentinelas usam eval produtivo sem essa projeção.

As nove consultas CLI coincidiram com o vanilla pinado. Os controles de colunas, contadores, contextos capturados/gerados e include/import passaram. As três sondas HTML e o bundle misto preservaram seus observáveis conferidos. As limitações prévias de seletor table.cell e dos parênteses de matriz permanecem iguais ao baseline e não qualificam paridade geral.

Nos sete PDFs do ensaio pareado, dimensões de página, caixas de palavras e rasters a 36 dpi coincidiram integralmente com o baseline; cada fixture produziu uma página. Foi inspecionado visualmente o recorte do conteúdo transformado a 144 dpi. Isso prova preservação destes controles, não igualdade de bytes PDF ou paridade universal.

## Contagem de execução

GDB contou entradas da função produtiva `typst_core::compiler::introspect::entrypoints::introspect_with_introspector`, sem instrumentar fontes e separadamente dos tempos. Fonte da sonda: `#show heading: it => upper(it.body)` seguida de `= Original`.

| Caminho | Antes | Depois |
| --- | ---: | ---: |
| PDF | 6 | 5 |
| HTML | 2 | 1 |
| query | 2 | 1 |

Isso comprova uma reconstrução removida por caminho nesta sonda. Não mede todos os walkers internos nem autoriza dizer que PDF faz uma só introspecção: cinco entradas puras continuam acontecendo em etapas posteriores. A primeira tentativa do debugger no sandbox falhou por ptrace; o ensaio completo foi reexecutado com permissão de depurar somente processos locais filhos, e o recibo final conserva comandos e saídas.

## Consultas: desempenho e memória

Uma rodada de aquecimento e 15 amostras por fixture/motor, pares intercalados, afinidade a um CPU, `RAYON_NUM_THREADS=1`, `SOURCE_DATE_EPOCH=0`. Tempo da CLI query inteira via `/usr/bin/time`, incluindo impressão/captura do JSON e o invólucro. Todas as saídas antes/depois coincidiram.

| Fixture | Antes, ms | Depois, ms | Variação | Pico RSS antes → depois, MiB |
| --- | ---: | ---: | ---: | ---: |
| 500 headings, sem show | 13.37 | 13.26 | -0.76% | 17.83 → 17.83 |
| 1 heading, com show | 7.00 | 7.05 | +0.67% | 14.19 → 14.18 |
| 500 headings, com show | 22.07 | 18.27 | -17.21% | 21.18 → 20.36 |
| 2.000 headings, com show | 70.84 | 55.27 | -21.98% | 42.27 → 41.44 |

Medianas; variação negativa indica redução. Para 500 headings com show, as faixas foram 21,42–30,59 ms antes e 17,18–19,26 ms depois; para 2.000, 68,50–73,43 ms e 53,39–57,42 ms. São ganhos claros nestes controles, sem extrapolação nem teste estatístico formal. O controle sem show e a consulta pequena tiveram faixas sobrepostas e não demonstram ganho. O pico RSS caiu cerca de 0,83 MiB nos dois controles grandes, compatível com evitar o segundo índice de consulta; não é uma medição isolada das alocações do índice.

## Compilação completa

Uma rodada de aquecimento e nove amostras por fixture/motor, intercaladas, na mesma afinidade/configuração. `compile` usa `--pdf-tagged=false --pretty=false --timings-json`; fontes, realização, layout e exportação continuam no tempo. Não houve suíte/build concorrente durante as duas rodadas pareadas.

| Fixture | Antes, ms | Depois, ms | Variação | Pico RSS antes → depois, MiB |
| --- | ---: | ---: | ---: | ---: |
| hello | 186.86 | 183.91 | -1.58% | 41.26 → 41.32 |
| paragraphs | 278.39 | 278.91 | +0.19% | 52.90 → 52.80 |
| grid320 | 226.29 | 227.88 | +0.70% | 42.54 → 42.32 |
| eval30000 | 26.08 | 26.31 | +0.89% | 20.91 → 20.61 |
| show_headings500 | 259.60 | 257.65 | -0.75% | 55.94 → 56.50 |
| show_eval30000 | 193.82 | 192.27 | -0.80% | 41.15 → 41.13 |
| show_grid320 | 252.10 | 253.21 | +0.44% | 43.01 → 43.12 |

As diferenças são pequenas e há sobreposição das faixas; não há ganho convincente para compilação completa nesta rodada. O controle sem show Hello também oscilou, lembrando que a mediana da CLI sozinha não atribui causalidade ao cache. Paged/HTML ainda precisam clonar o snapshot mutável da tentativa, e PDF conserva trabalho posterior. O custo de manter o snapshot existe: em 500 headings compilados, o pico RSS mediano aumentou cerca de 0,55 MiB. O getter evita a reconstrução, mas não torna gratuito manter e clonar stores.

## Proveniência

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, com todas as alterações anteriores da avaliação única presentes. `git status --short`, `git diff HEAD --stat` e hashes do código/L0 estão nos recibos. O baseline desta etapa já contém avaliação única; não é o binário anterior à correção global.

- Baseline preservado: SHA-256 `f03000b982aa07e4dd5ddad5042099757cb48e5f5ec674edc69d658e7b4bc20e`.
- Candidato: SHA-256 `70bb22e90f7b0fa1a660de1c8f0df0a89f8ebb6ddd82a77fd4794f753ebb07f5`.
- Oráculo: vanilla pin `9f2b6e8715237cb086899a42873660fe744622e8`, SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
- Compilação pareada: `2026-10-05T16:10:24.099810+00:00` até `2026-10-05T16:10:55.369211+00:00`, UTC.
- Consulta pareada: `2026-10-05T16:12:11.982051+00:00` até `2026-10-05T16:12:15.713457+00:00`, UTC.

Recibos e receitas:

- [Compilação pareada](memoizacao-indice-2026-10-05/bench-paired.json.gz), [harness](memoizacao-indice-2026-10-05/bench.py).
- [Consultas pareadas](memoizacao-indice-2026-10-05/query-paired.json.gz), [harness](memoizacao-indice-2026-10-05/query_bench.py).
- [Contagens](memoizacao-indice-2026-10-05/count.json.gz), [harness](memoizacao-indice-2026-10-05/count.py).
- [Geometria](memoizacao-indice-2026-10-05/geometry.json.gz), [harness](memoizacao-indice-2026-10-05/geometry.py).
- [Sondas CLI](memoizacao-indice-2026-10-05/verification.json.gz), [harness](memoizacao-indice-2026-10-05/verify.py).
- [Bundle misto](memoizacao-indice-2026-10-05/bundle.json.gz), [harness](memoizacao-indice-2026-10-05/bundle.py).
- [Build, suítes, lint, RED→GREEN](memoizacao-indice-2026-10-05/execution.json.gz), [coletor](memoizacao-indice-2026-10-05/receipts.py), [fontes exatas do RED](memoizacao-indice-2026-10-05/red-state.json.gz).

Linter global sem erros, mas ainda com avisos: 172 V5 em owners externos ao conjunto alterado, além de V12/V16/V17/V18 e notas V19/V20. Não há achados V5/V6/V15/V26 nos 54 consumers do conjunto pendente; o gate dedicado de ownership/núcleos está limpo. Não alegamos zero violations globais nem ressellamos owners externos sem auditoria.

## Próxima frente

Localizar as cinco introspecções puras posteriores do caminho PDF e distinguir quais veem uma árvore/contexto novos. Reaproveitamento só é válido quando conteúdo, proveniência e estado pertinente coincidem. A memoização de layout de células ainda exige largura, região, estilos, contexto, identidade e replay correto dos efeitos produzidos; não foi implementada nesta etapa. O próximo passo deve medir essas dependências antes de escolher a chave e o lifetime do cache.
