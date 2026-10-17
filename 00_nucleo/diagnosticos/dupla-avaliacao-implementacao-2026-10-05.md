# Avaliação única e conservação da identidade — implementação e ensaio

A avaliação do programa comum passa a ocorrer uma vez. O controle de 30 mil iterações reduziu o tempo total de 45,25 para 26,20 ms (42,1%). A tabela simples de 320 células não mostrou ganho; com show textual, ficou 1,7% mais lenta. Receitas `show` capturam o ambiente lexical e são aplicadas pela realização pura em L1. O grafo realizado conserva o original para introspecção e o produto para emissão, sem executar novamente todo o programa para fabricar uma segunda árvore.

A arquitetura pública e a transferência de fase foram autorizadas pelo usuário no gate ADR-0127, com SHA-256 inicial do L0 `5738955f3265e8062ba864e65e61b3999a1260b695969df5676c6cc9c9da10ab`. O L0 vigente inclui refinamentos internos posteriores, cada um anterior ao código e ressellado por owner. O [passo de execução](../materialization/dupla-avaliacao-ensaio-2026-10-05.md) registra a autorização; não substitui os prompts proprietários.

## O que mudou e como a identidade sobrevive

`ShowRecipe` conserva corpo, regras, estilos, guards, arquivo lexical e piso dos identificadores contextuais. `Located` conserva uma identidade preparada. `Realized` associa original e visual. São transportes explícitos no enum fechado, com `Arc` e sem introduzir `unsafe` ou estado global. A fachada está em `01_core/src/compiler/realize/mod.rs`, com owner exclusivo `00_nucleo/prompts/compiler/realize/mod.md`.

O layout e HTML emitem o visual. A introspecção visita original e gerados, deduplicando aliases pela identidade preparada. `query` usa esse índice conjunto, retirando a escolha anterior por quantidade de elementos. Os métodos de linguagem projetam o corpo ou o visual e permitem à receita observar a localização e os campos do original.

A preparação da identidade não determina a ordem documental. Foi preciso registrar a ordem da travessia para consultas temporais, contadores e estados. Essa invariante compartilhada está no Núcleo Tekt `00_nucleo/prompts/_nuclei/realization-order.toml`, pinado nos owners; a relação consumer→prompt continua 1:1.

A sonda refutou a hipótese de renumerar toda inserção: devolver `it` duas vezes conserva **uma ocorrência semântica e duas emissões visuais** no vanilla pinado. Construir dois headings iguais conserva **duas ocorrências**, além do original. A implementação respeita essa diferença. Identidade, igualdade de corpo e ordem documental são conceitos separados.

Dois ajustes causais também foram necessários: reservar o piso de IDs dos contextos já capturados antes de executar receitas que geram contextos novos; e projetar os transportes no medidor restrito e no medidor legado. Sem este último ajuste, a tabela com substituição textual emitia os textos, mas suas colunas Auto eram medidas menores.

## Validação

As sentinelas RED→GREEN demonstram leitura do programa comum uma vez, com e sem show; conservação do original e dos dois gerados; alias de `it`; distinção de gerados iguais; campos/localização disponíveis à receita; ordem causal dos contadores; contextos capturados e gerados sem colisão; e dimensões dos transportes de realização.

O adapter dos testes históricos de avaliação projeta o visual para preservar suas asserções estruturais antigas. Os testes novos de identidade consultam `introspection_content`; as sondas da CLI e os testes do pipeline exercitam o grafo produtivo completo.

As suítes completas passaram: **6.504 testes de núcleo**, com 5 ignorados, e **1.143 testes de infraestrutura**, sem falhas. O build otimizado também passou. Só houve atualização de metadados de linhagem após essas suítes, sem nova alteração de algoritmo.

Os recibos da CLI comparam baseline, candidato e vanilla pinado em query, PDF, HTML e bundle misto PDF/HTML. As nove consultas coincidiram com o oráculo; as três saídas HTML e os dois documentos do bundle também coincidiram em seus observáveis conferidos. Dois exemplos expõem a correção: a consulta contextual deixa de mostrar somente `Generated` e passa a mostrar `Original | Generated`; a numeração de dois headings transformados deixa de ser `0.1.`/`0.2.` e passa a `1.1.`/`2.1.`, como no vanilla. `table.cell` como seletor continua não suportado no baseline e no candidato; a sonda foi conservada como limite conhecido, não passou a ser critério desta mudança. Os dois controles de matriz mantêm o texto `a` do baseline, enquanto o vanilla emite `(a)`; essa divergência prévia e as compensações internas de matriz permanecem fora do escopo.

## Medição

Uma rodada de aquecimento e nove amostras por fixture/motor, pares intercalados, um CPU fixado, `RAYON_NUM_THREADS=1`, `SOURCE_DATE_EPOCH=0`. Cada execução usa `/usr/bin/time` envolvendo `typst compile`, `--pdf-tagged=false`, `--pretty=false` e `--timings-json`. O tempo de parede medido inclui a CLI inteira e o pequeno invólucro; RSS é o pico reportado pelo SO. As fontes exatas, opções, amostras, timestamps e fases estão no recibo pareado.

| Fixture | Antes, ms | Depois, ms | Variação total | eval_ms antes → depois | Pico RSS antes → depois, MiB |
| --- | ---: | ---: | ---: | ---: | ---: |
| Hello World | 177.63 | 177.54 | -0.05% | 168.54 → 167.94 | 41.09 → 41.43 |
| 60 parágrafos | 266.49 | 267.60 | +0.42% | 169.83 → 168.07 | 52.61 → 52.78 |
| 320 células | 228.69 | 229.09 | +0.18% | 170.26 → 169.37 | 42.20 → 42.42 |
| 30 mil iterações | 45.25 | 26.20 | -42.10% | 38.20 → 19.74 | 25.02 → 20.93 |
| 30 mil iterações + show heading | 209.02 | 194.65 | -6.88% | 38.00 → 19.97 | 40.93 → 41.23 |
| 320 células + show textual | 250.95 | 255.14 | +1.67% | 171.19 → 170.61 | 42.29 → 42.90 |

Valores: medianas. Uma variação negativa indica redução do tempo. Não foi feito teste de significância nem extrapolação para outros documentos. Hello, parágrafos e tabela simples variaram menos de 0,5%, com intervalos de amostras sobrepostos; não demonstram ganho ou regressão robustos. No cálculo pesado, a faixa antes foi 44,43–47,17 ms e depois 25,71–27,47 ms: o ganho de 42,1% é claro neste controle. Com heading transformado, a redução total foi 6,9%.

A tabela com show textual ficou 1,7% mais lenta, com faixas 249,76–253,15 ms antes e 254,17–258,07 ms depois. Seu `eval_ms` quase não mudou. Isso indica custo adicional posterior à fronteira cronometrada da avaliação, mas não isola uma travessia específica. O ganho aparente da primeira rodada desta fixture foi invalidado pela geometria incorreta: os recibos RED dessa rodada foram preservados e não fundamentam o resultado final.

Os seis PDFs finais tiveram caixas de palavras e dimensões de página idênticas ao baseline, além de rasters idênticos a 36 dpi, em uma página por fixture. O recorte da tabela com e sem show foi inspecionado visualmente a 144 dpi. É prova de preservação destes controles, não de paridade universal nem de igualdade dos bytes PDF.

## Proveniência e recibos

Estado: HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, **working tree não commitado**. A lista exata de arquivos modificados (`git status --short`), `git diff HEAD --stat` e hashes de fonte/L0 estão nos recibos. O benchmark final começou em `2026-10-05T15:50:33.353855+00:00` e terminou em `2026-10-05T15:50:59.025958+00:00` (UTC). Não houve compilação ou suíte concorrente durante a rodada.

Binários:

- Baseline preservado: SHA-256 `eec9edd5ded17b0cd58c3297d313ff5796694899b39d5b87f6c5426f5dbc8e49`.
- Candidato otimizado: SHA-256 `f03000b982aa07e4dd5ddad5042099757cb48e5f5ec674edc69d658e7b4bc20e`.
- Oráculo vanilla: pin `9f2b6e8715237cb086899a42873660fe744622e8`, SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.

Recibos e receitas reproduzíveis:

- [Benchmark pareado](dupla-avaliacao-auditoria-2026-10-05/bench-paired.json.gz), [harness](dupla-avaliacao-auditoria-2026-10-05/bench.py).
- [Geometria final](dupla-avaliacao-auditoria-2026-10-05/geometry.json.gz), [harness](dupla-avaliacao-auditoria-2026-10-05/geometry.py).
- [Sondas query/PDF/HTML](dupla-avaliacao-auditoria-2026-10-05/verification.json.gz), [harness](dupla-avaliacao-auditoria-2026-10-05/verify.py).
- [Bundle misto](dupla-avaliacao-auditoria-2026-10-05/bundle.json.gz), [harness](dupla-avaliacao-auditoria-2026-10-05/bundle.py).
- [Build, suítes, lint e falhas RED](dupla-avaliacao-auditoria-2026-10-05/execution.json.gz), [coletor](dupla-avaliacao-auditoria-2026-10-05/receipts.py). Os hashes desse coletor descrevem a árvore final; os logs RED descrevem etapas intermediárias e não são medições de desempenho da árvore final.
- [Benchmark intermediário invalidado](dupla-avaliacao-auditoria-2026-10-05/bench-paired-measurement-red.json.gz), [regressão geométrica correspondente](dupla-avaliacao-auditoria-2026-10-05/geometry-measurement-red.json.gz).

O linter global terminou sem erros, mas **não está com zero violations**: há 172 avisos V5 em owners externos a esta mudança, além dos avisos globais V12/V16/V17/V18 e notas V19/V20. Nos 52 consumers alterados, não há achados V5/V6/V15/V26. O gate dedicado de ownership/núcleos passou sem violations. Os hashes de owners externos não foram corrigidos cegamente. `git diff --check` passou.


## Limites e próximas frentes

Esta mudança retira a dupla avaliação global; não retira a segunda passagem completa de layout paginado, as iterações contextuais necessárias nem a medição/emissão separadas das células. Imports continuam seguindo o mecanismo anterior, sem nova memoização.

A fachada ainda constrói um introspector cujo resultado não é reaproveitado pelo coordenador: o pipeline reconstrói o índice. Documentos com show também percorrem normalização e preparação de ocorrências. Essas travessias são candidatas a refinamento, mas a sua economia não foi isolada por ablação neste ensaio. Não confundir `eval_ms` instrumentado da infraestrutura, que inclui a chamada à realização e trabalho dos serviços acessados, com custo puro de uma função de L1.

O estudo mede o Tekt antes/depois na mesma máquina. A diferença histórica de aproximadamente 79 ms contra vanilla não é referência numérica para este ensaio. O vanilla pinado aqui serve de oráculo das sondas de linguagem.
