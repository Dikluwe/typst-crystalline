# A “mágica” do vanilla no caminho de fontes

O que a investigação comprova é uma combinação de **trabalho evitado, representação compacta e reaproveitamento com escopos específicos**. Isso explica mecanismos candidatos ao desempenho; ainda não atribui parcelas do gap end-to-end. Foram auditados os fontes do vanilla pinado `9f2b6e8715237cb086899a42873660fe744622e8` e o Tekt atual, e executada uma sonda estrutural nas mesmas fontes da comparação anterior. Nenhuma otimização foi feita.

## 1. A cmap já entrega a ordem que o vanilla precisa

Fonte: `lab/typst-original/crates/typst-library/src/text/font/info.rs:119`, `:346`, `:384`. Cada subtable Unicode emite codepoints em ordem crescente. CoverageBuilder imediatamente transforma pontos consecutivos em runs de gaps/contagens. A união percorre intervalos já ordenados, absorvendo sobreposições. O vanilla não materializa uma lista global de todos os callbacks nem ordena globalmente essa lista neste caminho.

**Ele ainda percorre todos os callbacks.** A economia demonstrada pela fonte é de materialização/ordenação e da representação de trabalho durante a união, não a eliminação da leitura da cmap. O construtor genérico from_vec em `:294` realmente ordena/deduplica pontos; não é esse o caminho interno de FontInfo. O comentário histórico sobre sort versus sort_unstable nessa função é evidência histórica do upstream, não medição nova nem explicação aplicável automaticamente ao caminho streaming.

Tekt: `03_infra/src/fonts.rs:403` coleta todos os callbacks num Vec. `01_core/src/entities/font_book.rs:172` já compacta sequências antes de ordenar intervalos. Portanto a descrição “Tekt ordena todos os pontos” está desatualizada para este estado: ele materializa todos os pontos, depois ordena os intervalos intermediários. A comparação anterior favoreceu o construtor genérico Tekt, mas favoreceu vanilla na composição metadata+cobertura; não há contradição, são caminhos diferentes.

A sonda [results.json](vanilla-font-mechanics/results.json) verificou ordem estritamente crescente **dentro** de cada subtable das fixtures e observou reinícios de ordem ao concatená-las. União usa um set independente para determinar os intervalos; não reproduz nem cronometra CoverageBuilder privado.

| Fonte | Callbacks de todas as subtables | Pontos distintos | Callbacks repetidos | Intervalos Tekt antes da ordenação | Intervalos finais |
| --- | ---: | ---: | ---: | ---: | ---: |
| Libertinus Serif | 9.522 | 2.382 | 7.140 | 708 | 178 |
| New Computer Modern Math | 16.768 | 4.913 | 11.855 | 474 | 144 |
| Noto Sans CJK | 174.060 | 44.810 | 129.250 | 4.432 | 2.032 |
| Noto Sans Math | 7.610 | 2.472 | 5.138 | 382 | 123 |

Na CJK, o payload lógico do Vec de codepoints é 696.240 bytes, sem contar capacidade excedente ou buffers de intervalos. Vanilla opera com runs por subtable e uniões de ranges. Não se trata de um valor de alocações ou RSS medido. A ordem das fixtures confirmou a premissa do algoritmo nesse conjunto; não prova a premissa para todo arquivo malformado ou todos os formatos possíveis.

## 2. A TTC continua sendo uma TTC até o consumer adequado

Fonte: vanilla `typst-kit/src/fonts.rs:112`, `typst-library/src/text/font/mod.rs:63`, `typst-pdf/src/text.rs:78` dentro de lab/typst-original/crates. O loader lê bytes e mantém o índice da face. Font guarda bytes e Face parseada. O PDF recebe o buffer original **e o índice** através de krilla::text::Font::new_variable; o adaptador PDF build_font também é memoizado.

Tekt: `03_infra/src/fonts.rs:153`, `:201` reconstrói uma sfnt independente no primeiro get de uma coleção. Seu caminho de export `03_infra/src/export/subset.rs:56` chama o subsetter com índice zero. Logo a reconstrução tem um consumer concreto que espera outra representação; removê-la cegamente não seria uma correção segura. Esta investigação não altera esse contrato.

Na fixture Noto CJK há dez faces na coleção de 19.484.784 bytes. Para a face zero, o diretório determina 16 tabelas e 16.467.427 bytes de payload. A fórmula da reconstrução Tekt gera 16.467.712 bytes com padding/header e exige uma passagem de checksum sobre 4.116.928 palavras u32. Além da cópia das tabelas, o Vec começa reservado apenas para header/diretório e pode crescer durante a montagem. Contagens de bytes/palavras são fórmulas verificadas sobre o arquivo, **não** medições de RSS, tráfego efetivo, reallocations ou tempo de checksum.

Vanilla ainda paga leitura/alocação dos bytes, parsing e metadata+cobertura; não tem “I/O grátis”. Mas não realiza essa reconstrução sfnt no carregamento. A [comparação anterior](font-microbenchmarks-vanilla.md) observou primeiro acesso TTC mais lento no Tekt. Esse caminho comprova a existência de trabalho adicional, não que todo o diferencial observado pertença a ele. Primeiro acesso de fontes simples favoreceu o Tekt, cujo handle não incorpora toda a metadata/cobertura do Font vanilla.

## 3. O catálogo tem duração de World, não de chamada

Fonte vanilla: `typst-cli/src/world.rs:31`, `:85`, `:113`; `typst-kit/src/fonts.rs:24`, `:87`. O catálogo é LazyLock, os slots são OnceLock<Option<Font>> e reset limpa arquivos/tempo, preservando fontes. Em `typst-cli/src/watch.rs:76`, o mesmo World volta a compilar. Até falhas de carregamento ficam retidas no slot enquanto esse store existir.

**Não encontrei persistência de um catálogo próprio em disco nesses caminhos.** FontInfo/Coverage suportam serde, mas o discovery observado cria fontdb::Database, calcula metadata e devolve entradas em memória; não lê/escreve um catálogo serializado do CLI. Cache do SO/fontconfig não equivale a esse catálogo. Um processo novo tem outro World e outra descoberta. Não se extrapola a retenção de watch para invocações independentes do executável.

O Tekt atual já adotou catálogo lazy em `03_infra/src/world/fonts.rs:104` e slots OnceLock. Assim, “vanilla é lazy e Tekt não” não explica o estado atual. A diferença restante de duração aparece em watch: `04_wiring/src/watch.rs:63` chama run_compile_observed a cada ciclo; `04_wiring/src/compile.rs:246` cria um World naquele caminho. O catálogo retido dentro de um World não sobrevive à criação de outro. Este é um achado do fluxo, sem tempos novos de watch ou alteração da política de invalidação.

Vanilla também calcula FontInfo durante discovery em `typst-kit/src/fonts.rs:182` e novamente ao carregar um Font de arquivo. Não se deve descrevê-lo como “parseia cada fonte uma única vez em todo o pipeline”. Fontes embedded são construídas como Fonts no discovery e fornecem metadata já disponível; isso também difere dos slots de arquivo.

## 4. Instância, plano de shaping e layout são reutilizados em níveis diferentes

Fonte vanilla: `typst-library/src/text/font/mod.rs:108`, `:123`, `:136`. instantiate/instantiate_impl são memoizados, mantêm a Face rustybuzz e métricas na FontInstance Arc. `typst-layout/src/inline/shaping.rs:1218` memoiza ShapePlan por font instance, direção, script, língua e features; o shaping usa shape_with_plan em `:1026`. Isso permite reaproveitar o plano mesmo entre textos distintos com configuração compatível. Layout de parágrafo também tem implementação memoizada em `typst-layout/src/inline/mod.rs:70`.

Tekt possui caches reais: `03_infra/src/shaper/traversal.rs:22` cria ShapeCache e FaceCache para o documento; `shaper/shaping.rs:229` reutiliza resultados de sub-runs. No miss, `:234` recria Face rustybuzz e `:274` chama shape, sem o cache explícito de planos observado no vanilla. Um cache de resultado para texto idêntico e um cache de plano para textos diferentes não evitam o mesmo trabalho. A duração por documento também limita reaproveitamento entre recompilações. Nenhum desses custos/hit rates foi medido nesta sonda.

Font/Bytes vanilla usam Arc, e Bytes contém LazyHash em `typst-library/src/foundations/bytes.rs:46`; `typst-utils/src/hash.rs:102` calcula o hash ao precisar e o conserva. Mutação através de DerefMut reseta o hash em `:143`. O FontBook também usa LazyHash. Isso evita revarrer dados imutáveis para cada chave de memoização depois da primeira computação. Não significa que o primeiro hash seja gratuito ou que o Tekt não use partilha/cache em outras unidades.

## 5. O número exibido por watch tem uma fronteira diferente

Em `typst-cli/src/watch.rs:51`, o próprio comentário explica a varredura antecipada para não incluí-la no tempo exibido. `world.scan_fonts()` ocorre antes de timer.record. Já a compilação CLI comum chama timer.record depois de construir World (`typst-cli/src/compile.rs:40`), com discovery lazy potencialmente dentro da compilação.

Portanto tempo de watch exibido, recompilação quente e duração de um processo CLI completo são observáveis distintos. Isso pode produzir uma impressão de rapidez maior, mas **não invalida nem explica** a baseline anterior, que cronometrou processos end-to-end. Não foram misturados esses cronômetros nesta análise.

## Experiências que separariam as explicações

| Hipótese | Intervenção controlada | Evidência que a enfraquece/refuta |
| --- | --- | --- |
| Materializar cmap pesa no catálogo | Mesma Face parseada, metadata fora; comparar composição de coleta+Coverage atual com reprodução diagnóstica streaming, destruir outputs fora e verificar os mesmos sets | Diferença desaparece com fronteiras equivalentes e estabilidade |
| Reconstrução TTC explica boa parte do primeiro get | Mesma face TTC versus sfnt previamente reconstruída, nos dois engines; mesmos glifos/coverage, armazenamento aquecido | Gap persiste na sfnt; ressalvar que container/tamanho mudam juntos |
| Vida curta do World pesa em watch | Mesmas edições, mesmo catálogo: World novo versus retido com invalidação explícita, processos/ordem controlados | Discovery não se repete no trajeto medido ou ganho não aparece |
| Plano/instância são repetidos em misses | Muitos textos distintos na mesma fonte/configuração, medir misses e criação de planos/faces em harness diagnóstico; variar apenas retenção | Planos não são reconstruídos nesse caminho ou custo é irrelevante |
| Tempo exibido cria impressão enganosa | Capturar simultaneamente duração externa e cronômetro exibido em cold compile/watch | As fronteiras comparadas eram iguais ou a diferença não explica a observação |

## Evidência e limites

Sonda em [vanilla-font-mechanics/](vanilla-font-mechanics/), exclusivamente diagnóstico. [Provenance](vanilla-font-mechanics/provenance.json) registra HEAD `6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5`, working tree não commitado com status/diff-stat/patch exatos, comandos, lockfile, Rustc, perfil, hashes e horários. Execução em 2026-09-27, UTC 14:15:53.636645–14:15:53.701353. Esses timestamps identificam a sonda, não são uma medição de desempenho publicada.

O fingerprint de consumidores/inputs/pin antes/depois permaneceu `9d29d3f8b1a0223b55eff92b955b4cd0735d205cd4c49970825ae2047bc44fe2`, igual ao dataset anterior. Inputs/índice zero são os mesmos arquivos hashados. Probe verifica seus bytes, ordem das subtables e usa BTreeSet independente para contagens. A classificação é de mecânica de execução (ADR-0107/0108), não prova de paridade da linguagem. ADR-0030 e ADR-0121 foram consultadas. A sonda é compilada em release, mas não coleta tempos nem reivindica isolamento de CPU/cache.

Só este relatório e o novo diretório diagnóstico foram adicionados. Produção, L0s, ADRs, benchmarks e datasets selados anteriores permaneceram intactos. O arquivo bin preserva o executável; MANIFEST.sha256 sela o dataset. Não houve resselo global ou formatação global.

A principal conclusão é que o vanilla **mantém informação na representação útil aos próximos consumidores e reaproveita trabalho com chaves/duração adequadas**. Cmap e TTC têm trabalho evitável identificado; cache/planos/watch têm escopos diferentes demonstrados. A parcela de cada mecanismo no gap total ainda precisa das intervenções acima; os microbenchmarks anteriores continuam exploratórios, com suas falhas de estabilidade preservadas.
