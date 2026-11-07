# Consolidação dos seis experimentos e plano para os gargalos

A prioridade para documentos pequenos é reduzir o custo de aquisição de fontes na CLI e de validação da geração no watch, preservando a garantia de frescor. Para outline, a prioridade específica é entender o trabalho necessário entre candidatos de layout. Memoização de imports é a opção com resultado positivo mais claro em bancada, mas atende outra parcela do custo e ainda não demonstra ganho na CLI/watch.

**Nenhuma das seis rodadas validou uma otimização integrada para o gargalo principal.** Os resultados restringem as hipóteses: várias formas de eliminar observações perderam correção, enquanto copiar snapshots completos aumentou custo. O plano abaixo prioriza o trabalho restante com esse limite explícito.

## Evidência consolidada

Os números abaixo são resultados anteriores, não uma nova rodada de medição. Cada relatório vincula o estado exato, método, ambiente e limitações. Tempos de sondas distintas não são somados nem subtraídos para inferir economia.

|Experimento|Resultado sustentado|Consequência para o plano|
|---|---|---|
|[1 — medição](bottleneck-experiment1-2026-11-04.md)|Texto pequeno: CPU com sonda desligada de 94,43 ms na CLI e 18,92 ms no watch com mesmos bytes. Watch quente: 3.803 consultas wrappers de metadados por ciclo; apenas 26 repetições da mesma operação/caminho na mesma invocação. Outline teve seis candidatos com durações distintas.|Fontes primeiro nos documentos pequenos; layout em frente própria. As 26 repetições não explicam o conjunto do custo.|
|[2 — deduplicação TTC](bottleneck-experiment2-2026-11-04.md)|Reúso da primeira identidade entre faces perdeu detecção de substituição, remoção, permissões e retarget que o baseline observou.|Descartar essa variante como equivalente à prova atual. A economia teórica de 26 consultas não é ganho seguro nem porcentagem de CPU.|
|[3 — catálogo e recursos](bottleneck-experiment3-2026-11-04.md)|Catálogo fresco mais payload imutável reutilizável preservou os controles; parses de recursos caíram de 27 para cinco. Continuaram cerca de 46 MB de leituras explícitas por rodada. Parse fresco de recursos totalizou apenas 0,038 ms de mediana nessa bancada.|Reúso entre gerações é possível sob condições fortes, mas não resolveu aquisição/validação. Só priorizar se reconstrução de recursos realmente usados aparecer material no produto.|
|[4 — imports](bottleneck-experiment4-2026-11-04.md)|Entrada pura isolada com imports e World congelado: fresh 6,843 ms; hit quente 0,561 µs, com zero execução do avaliador e zero resolução de import. Replay da projeção escalar/diagnósticos passou.|Melhor candidato para uma bancada mais próxima do import real. Não equivale a cache de Module/closures, nem a ganho end-to-end.|
|[5 — snapshots](bottleneck-experiment5-2026-11-04.md)|Cópia completa por compilação manteve o número de verificações; para três Worlds ativos, leitura passou de 556.384 para 7.895.784 bytes e retenção de fontes de 556.216 para 3.947.808 bytes. RSS mediano: 3.188 contra 7.412 KiB.|Não adotar snapshot eager para performance. O compartilhamento intracompliação já é parcialmente atendido por OnceLock/Arc. Snapshot lazy continua hipótese sem ganho demonstrado.|
|[6 — eventos e Fontconfig](bottleneck-experiment6-2026-11-04.md)|Houve mudanças com fila vazia, perda simulada e overflow real. Cache Fontconfig 2.15 manteve família antiga e omitiu fonte reparada com tamanho/mtime preservados; rescan corrigiu esses dois casos. Diferenças de ordem e symlinks permaneceram.|Eventos podem ser dicas, mantendo prova completa. Fontconfig nativo não demonstrou ser substituto do catálogo/prova atuais ou acelerador do Tekt.|

A [primeira rodada](bottleneck-first-round-2026-11-04.md) e o experimento 1 apontam fontes por amostragem, fases e syscalls. A sonda reduzida do experimento 1 mediu aproximadamente 85,79 ms exclusivos em fresh_catalog na CLI pequena e 16,24 ms somando spans exclusivos de fontes no watch pequeno. Esses relógios são instrumentados; os valores de CPU acima são da sonda desligada. Não transformar sua razão em uma fração exata do tempo original.

O perfil depende do documento. No outline do experimento 1, CPU sem sonda foi 213,87 ms CLI e 122,71 ms watch com mesmos bytes. Os seis candidatos watch tiveram medianas inclusivas de 16,62 / 2,09 / 10,69 / 11,22 / 8,95 / 10,85 ms. O primeiro inclui fontes demandadas; não são seis iterações de um único loop, nem prova de seis resultados intercambiáveis. Exportação também pesa nessa fixture: cerca de 19–20 ms nos envelopes medidos, sem evidência de que seja o gargalo principal dos documentos pequenos.

## Plano em ordem de prioridade

### 1 Fontes no caminho real da CLI e do watch

**Próxima execução recomendada:** decompor o custo restante dos dois caminhos reais com uma sonda agregada de baixo overhead. A contagem já basta para rejeitar a busca por grandes ganhos na deduplicação TTC; falta localizar o custo que pode ser reduzido sem suprimir observações necessárias.

No watch, separar configuração, prova de raízes/candidatos/symlinks e GenerationKey: leitura/parse de configuração, resolução de caminhos, consultas de metadados, alocação/cópia e comparação em RAM. Preservar sequência, quantidade e semântica follow/nofollow das observações. Acumular contadores por fase e descarregar uma vez por compilação, evitando JSON por stat. Medir overhead off/on; strace continua controle de contagem, não relógio do produto sem instrumentação.

Na CLI, separar descoberta fontdb, caminho selective/acquire, fallback de aquisição, extração de FontInfo/coverage, composição de fontes embutidas/sistema/projeto e cargas de recursos demandados. Contar aberturas, leituras e trabalho por arquivo e por face de coleção. A leitura seletiva já existe: a hipótese não é simplesmente introduzi-la de novo.

**Hipóteses para avaliar somente após essa atribuição:** reduzir reconstrução/alocação de estruturas de comparação mantendo todas as observações; reduzir trabalho puro repetido de extração de metadados/cobertura sob a mesma identidade validada e com as mesmas barreiras. Por exemplo, scan_stamp constrói PathBuf por consulta; isso é um ponto concreto a medir, não uma economia demonstrada. Os owners e fontes atuais são [generation.rs](../../03_infra/src/fontdb/generation.rs), [selective.rs](../../03_infra/src/fontdb/selective.rs), [fontdb.rs](../../03_infra/src/fontdb.rs) e [world/fonts.rs](../../03_infra/src/world/fonts.rs).

**Saída necessária:** distribuição de custo e limite plausível do ganho de cada hipótese. Se o custo restante for predominantemente o das próprias observações obrigatórias, registrar esse limite e não prometer removê-lo com otimização de RAM ou com eventos. Nenhuma das seis rodadas forneceu uma prova alternativa equivalente que permita dispensá-las.

**Gate:** candidato futuro precisa manter observações/barreiras e passar nos adversários de alteração, recusa e concorrência; benefício só existe se reduzir CPU e/ou latência final de modo repetível sem deslocar custo para startup ou memória.

### 2 Imports como primeiro candidato positivo de escopo restrito

Avançar o experimento 4 até a fronteira real de arquivo importado, inicialmente para imports puros caros e em cópia de bancada. O objetivo é preservar o trabalho do import quando o main muda, sem conservar binding, closure, rota ou diagnóstico incorreto.

A chave do experimento 4 contém todo o snapshot, inclusive o main. Logo, uma edição normal do main muda a chave mesmo quando o import não mudou. A chave também contém namespace World: usar a identidade de um novo objeto World por ciclo pode impedir hits entre compilações. **O hit isolado não prova uma oportunidade de reúso útil no watch atual.**

A próxima bancada deve demonstrar hit genuíno após editar somente o main; miss após alterar uma dependência transitiva, inputs, features/target ou qualquer consulta externa pertinente; reconhecimento de arquivo ausente que aparece; ciclo/erro com trace correto; replay de SourceDiagnostic no Sink real, na ordem e multiplicidade corretas. Definir o que constitui a identidade estável do ambiente e o que deve ser rastreado, sem retirar main/contexto da chave por conveniência quando forem dependências reais.

Conservar a fronteira de isolamento de [eval/modules.rs](../../01_core/src/compiler/eval/modules.rs) e seu [L0](../prompts/core/compiler/eval/modules.md). Validar Module, bindings e closures que referenciam Source/World, não apenas um escalar formatado. Capacidade não capturada, como plugin/relógio/recurso externo, deve recusar o caso elegível. Não presumir que Engine.world esteja integralmente tracked.

**Gate:** execuções efetivamente evitadas no caller real, saídas e diagnósticos corretos, aquisição/validação/replay/memória incluídos. Medir CLI nova separadamente: cache em RAM não cria reúso entre processos CLI independentes. No watch, comparar startup, hits, misses e edições nas cinco fixtures. Sem ganho final, não integrar mesmo com hits corretos.

Essa é a opção com melhor evidência positiva de computação evitável; permanece secundária ao gargalo de fontes dos documentos pequenos.

### 3 Layout para documentos com outline

Instrumentar cada candidato por coordenador e motivo: mudanças em introspecção/mapas, contexto/regiões, estilos/métricas, callbacks, running-content e rollback. Registrar quais entradas e produtos realmente diferem, além do número e tempo de candidatos já conhecidos.

O primeiro alvo é identificar trabalho puro repetido dentro de candidatos necessários, ou uma repetição cuja remoção possa ser justificada pelas dependências. Não eliminar candidatos apenas porque o conteúdo parece igual ou porque há seis. O controle do experimento 4 preservou as mesmas palavras e produziu sete ou 22 páginas ao variar métricas efetivamente consumidas: uma chave adequada para eval não é automaticamente adequada para layout.

**Gate:** preservar convergência, transações, seleção de candidatos e resultados observáveis; reduzir CPU/latência de outline sem regressão em texto simples, callbacks, notas, running-content ou memória. Reúso de layout exige especificação própria; não será carona da memoização de imports.

### 4 Reúso de recursos apenas quando o custo marginal justificar

Depois da atribuição de fontes, verificar quanto custa reconstruir os recursos realmente utilizados quando outra parte do catálogo muda. O experimento 3 permite investigar essa separação, mas o ganho de parse de recursos foi pequeno e a validação permaneceu.

Não transportar índices antigos nem manter a seleção antiga. Catálogo atual continua autoridade. Exigir payload realmente imutável, identidade completa, face/contexto de parser, invalidação de negativos e orçamento total incluindo chaves e Worlds ativos. Arc sobre mmap atual não demonstra isolamento diante de escrita externa. Para TTC, compartilhar bytes da coleção sem multiplicá-los por face é uma hipótese específica de representação; a cópia ingênua da bancada reteve cerca de 195 MB só em payloads, mais cópias nas chaves.

**Gate:** reconstrução material medida no produto e redução líquida de tempo/memória com a prova completa mantida. Caso contrário, encerrar essa frente como baixa prioridade.

## Opções fora da primeira fila de implementação

- Deduplicar stats TTC como no experimento 2: perdeu oportunidades de observação do baseline e tinha alcance pequeno.
- Snapshot eager por compilação: custo adicional demonstrado na fixture, sem redução das verificações.
- Dispensar validação porque não chegaram eventos, ou porque uma epoch/TTL não mudou: não fornece prova equivalente de frescor.
- Substituir a descoberta pelo cache Fontconfig sem prova adicional: metadados obsoletos e representações distintas foram observados. Como pista sob validação, ainda precisaria ganho líquido medido.
- Memoização ampla de eval/layout ou novo banco de consultas como primeiro patch: não há fronteira completa validada para isso nas seis rodadas.

Esses resultados não provam defeitos universais de inotify/Fontconfig nem rejeitam todo snapshot ou cache. Delimitam o que não é uma otimização transparente sustentada pelo contrato e pelas medições atuais.

## Critérios comuns para avançar até o produto

Primeiro uma bancada concreta, depois um candidato especificado por seu L0. O baseline é o estado atual com sua retenção real, não uma implementação deliberadamente mais cara. Builds descartáveis na RAM, um job e limites de recursos aplicáveis; medições sem compilação concorrente. Rodadas pareadas/intercaladas, sonda off/on quando necessária, estado de fontes/configuração/binário registrado.

Repetir texto pequeno, import sintático, import computacional, parágrafos e outline; CLI nova e watch com startup, mesmos bytes, edição pequena e mudança estrutural. Para fontes, somar adição/remoção/substituição, tamanho/mtime restaurados, symlink/configuração, candidata inválida reparada, falha e mudanças concorrentes. Para imports, somar dependências transitivas, rota, inputs, host, target/features, erros e efeitos. Medir CPU, latência até publicação, RSS pico/retido, execuções realmente evitadas e custo da validação/chave/replay. Não subtrair medianas de fixtures diferentes ou de estados distintos para inventar uma fase.

Aceitação funcional por semântica, sintaxe e morfologia, com seleção/diagnósticos/render pertinentes; hashes, Arc e igualdade mecânica são instrumentos, não paridade por si sós. Contraexemplo de correção bloqueia o candidato mesmo se houver ganho. Ganho em microssegundos de uma função não substitui ganho repetível no cenário alvo, e não se exige uma porcentagem arbitrária antes de estimar ruído e teto plausível.

Nenhum percentual de aceleração do produto é previsto por esta consolidação. Aplicar ADR-0127 à mudança concreta: contrato público, comportamento padrão, fase ou compatibilidade exigem seu gate; correção interna elegível segue L0 primeiro e fluxo contínuo. Não pedir aprovação genérica agora nem tratar pesquisa como autorização de código.

## Proveniência desta consolidação

Síntese de resultados de 2026-11-04, sem novas medições ou builds. Os seis relatórios vinculados preservam seus estados e limites; as bancadas temporárias foram removidas conforme o skill de execução. Reexecutar requer reconstruir as sondas, não há executáveis experimentais promovidos a ferramentas do produto.

HEAD observado nesta revisão: `3c6b0ce7b7632cc74b004cea6681e4746e63d28d`, working tree não commitado. O diffstat atual de arquivos rastreados é o mesmo registrado no experimento 6: 180 arquivos, 7.181 inserções e 437 remoções; a lista exata está no apêndice daquele diagnóstico. Esta tarefa adiciona somente esta consolidação; não altera relatórios anteriores, fontes ou L0.

Conferência final UTC 2026-11-04T19:05:53.993722+00:00: arquivos registrados antes desta síntese permaneceram byte a byte iguais.
