# Testes do catálogo de desempenho

> **Estado da fila em 10/11/2026:** este documento é histórico. A [decisão consolidada](performance-integration-decisions-2026-11-10.md) rege a fila atual: manter as melhorias incorporadas, integrar somente C28 e arquivar os demais candidatos atuais. As recomendações e falhas abaixo descrevem seu snapshot; não reabrem campanhas nem substituem o código atual. Arquivado/inconclusivo não significa refutado.

Foram exercitados os candidatos aplicáveis do [catálogo de 28 investigações](code-performance-candidates-2026-11-05.md), com testes públicos, ciclos reais de CLI/watch, perfil de CPU, contagem de alocações e testes internos. **Duas falhas de correção foram reproduzidas: identidade do cache de imagens e namespace do cache de plugins.** Não foram integradas otimizações nem correções ao produto. Cobertura de uma hipótese não significa comparação de uma implementação alternativa: arenas, índices u32, World genérico e persistência continuam sem candidato autorizado ou ganho demonstrado.

## Estado e método

Estado medido: HEAD `3c6b0ce7b7632cc74b004cea6681e4746e63d28d`, **working tree não commitada** recebida em `2026-11-05T17:14:36.221369+00:00`. A lista exata de alterações é o apêndice `git diff HEAD --stat` do catálogo: 182 arquivos, 7719 inserções e 443 remoções. Os arquivos recebidos foram congelados em RAM antes dos builds. Código produtivo e experimentos anteriores não foram alterados durante os lotes. Somente fontes de laboratório novas e este diagnóstico foram acrescentados. A comparação de integridade final consta abaixo.

O laboratório está em [lab/experiments/catalog-tests](../../lab/experiments/catalog-tests/). A escreveu instrumentação; B escreveu testes independentemente; o agente principal integrou e executou. Os contextos foram separados, mas compartilharam o checkout original imutável; não se afirma isolamento de filesystem dos agentes. As instrumentações de fontes foram aplicadas apenas a cópias em RAM. O candidato conjunto recebido também foi preservado.

Rust utilizado: `1.92.0 (ded5c06cf 2025-12-08)`. Builds Cargo offline/locked, um job, `TMPDIR` e `CARGO_TARGET_DIR` dentro do diretório exclusivo `/dev/shm/tekt-catalog-tests-20261105`; limite de memória 12 GiB, MemoryHigh 8 GiB e swap zero via systemd. Release usa `opt-level=s`, thin LTO e uma unidade de codegen, com a exceção já existente de zune-jpeg. O binário baseline recompilado tem SHA-256 `902cfde0215fe0a4c5e4e9713820a1eae1b339e7aaa0d8005439849c2fdaa84c`. Testes públicos adicionais ligados diretamente aos rlibs reais e testes internos em dev são verificações funcionais, sem alegação de desempenho desses executáveis. Não houve nova comparação com vanilla.

Dados de domínio e fixtures estão em [lab/results/construction/diagnostics/code-performance-tests-2026-11-05](../../lab/results/construction/diagnostics/code-performance-tests-2026-11-05/). Tempos de lotes diferentes não são comparáveis entre si. Fontes utilizadas nas microbancadas constam dos próprios testes; os CSVs conservam parâmetros, unidades e repetições. Não se conservaram logs, mutantes ou diretórios de build.

## Correção antes de ampliar caches

**C01 — imagem:** dois controles passam; o teste adversário com o exportador PDF público falha. Após liberar o primeiro documento, `Arc::get_mut` permite substituir legalmente o PNG na mesma alocação. A segunda exportação devolve os pixels RGB `[17,89,201]` da primeira imagem em vez dos atuais `[203,41,7]`. O cache TLS usa endereço numérico e sobrevive ao documento. Não há unsafe nem simulação do exportador no teste. O caso está em `tests/c01_image_identity.rs`. Oito ciclos reais alternando arquivos de imagem produziram pixels corretos: a reciclagem de endereço nesse watch não foi reproduzida, o que não refuta o caso determinístico da API.

Uma bancada separada constatou um payload comprimido de 13 bytes retido em uma entrada após liberar documento e fonte. É evidência da retenção prevista pelo mecanismo atual, **não prova de vazamento ilimitado nem uma segunda falha de linguagem**. O owner atual manda preservar esse lifetime; uma correção deve revisar primeiro `prompts/infra/export/images/scan.md`.

**C25 — plugin:** três controles passam; 16 casos adversários de identidade falham, incluindo módulos WASM reais, call/transition, ordens AB/BA, erros e função ausente. Cada CLI fresca retorna seu próprio módulo A ou B. No mesmo processo watch, trocar atomicamente o WASM e editar o texto publica **`A Epoch 2` quando deveria publicar `B Epoch 2`**. `tests/watch_plugin_identity.py` preserva a reprodução independente. A identidade usada na memoização não distingue hosts com IDs locais coincidentes. O risco já constava do L0 de PluginFunc; agora existe reprodução observável. Não se deve ampliar memoização de imports/plugins antes de corrigir essa fronteira.

Os testes RED permanecem RED deliberadamente: esta tarefa mede o produto atual. Não foram enfraquecidos nem ocultados para obter uma suíte verde.

## Fontes: consultas explicadas e ganho local

Lote pareado com fixture `plain`, CPU 0, Rayon 1, stack de 64 MiB, UTC e epoch fixos: 12 rodadas × 3 repetições × CLI/startup × cinco variantes = **360 processos medidos**, além de aquecimento. Original, controle nativo e descoberta conjunta; instrumentação Off/On. Sem builds, strace ou contagem de alocações durante os relógios. Watch medido até publicação completa do PDF; o tempo externo inclui publicação e instrumentação. Conferidos 360 PDFs por texto/geometria/fontes e oito pares rasterizados.

|Mediana das médias por rodada, ms|Original|Nativo Off|Nativo On|Conjunto Off|Conjunto On|
|---|---:|---:|---:|---:|---:|
|CLI|93,04|93,22|93,01|93,07|93,25|
|Startup watch|138,23|138,72|138,56|134,85|135,02|

Startup conjunto/original: mediana pareada **−2,525%**, intervalo bootstrap 95% `[−2,942%; −2,104%]` (10 mil reamostragens, seed 0); CPU −2,533%. Conjunto/nativo Off: −2,648%. A perturbação On/Off no startup foi pequena e seu intervalo inclui zero. CLI não mostrou ganho, coerente com a rota não alterada. **O ganho é local a esta fixture e ambiente**: não substitui o lote anterior com várias fixtures, que não sustentou ganho global.

Strace foi executado em bancada separada. Startup nativo→conjunto: `getdents64` **290→148**, `statx` **19342→19779**, `read` **673→783**, `pread64` **11182→11182**. O delta de **437 statx** fecha integralmente:

- 183 destinos canônicos adicionais × metadata/symlink_metadata: +366;
- seis stamps adicionais na autoverificação e na prova final: +24;
- observação lexical/raízes: −8;
- 55 leituras lógicas adicionais de arquivos de configuração: +55 statx por descritor e +110 read.

Os wrappers não cobrem todos os detalhes de bibliotecas, DirEntry ou mmap; o fechamento usa também o trace completo. O dump acrescenta um open. Contagens detalhadas não são medidas de tempo.

Escopos de tempo são **inclusivos e sobrepostos**. G1/G2/G3 continuam aproximadamente 2,8/2,8/3,0 ms, sem supressão de prova. Preparação nativa compreende fast_key (~25,2 ms) e prepare (~41,4 ms); preparação conjunta ~67,4 ms. Não se demonstrou economia nessa preparação. Aquisição/materialização posterior caiu de ~46,8 para ~42,2 ms. Portanto, não se pode atribuir todo o ganho final à retirada de uma caminhada: a causa da diferença posterior ainda precisa de atribuição mais fina. Reparse inicial/final permanece, agregado em duas chamadas.

Os **29 testes adversários** da descoberta conjunta passaram novamente. Próximo candidato restrito: reduzir a observação canônica adicional aproveitando observações da descoberta na mesma fronteira, mantendo campos, follow/nofollow, ordem e provas. Não retirar reparse, G1/G2/G3 ou leitura de conteúdo por inferência deste lote.

## RAM, imports e exportação

Contagens reais por breakpoint em entradas de funções, separadas dos relógios, mostram:

|Watch, contagens cumulativas em startup / mesmos bytes / edição|Source::new|Construção da stdlib|
|---|---|---|
|Documento sem imports|1 / 1 / 2|1 / 2 / 3|
|Um import|2 / 2 / 3|2 / 4 / 6|
|Quatro imports|5 / 5 / 6|5 / 10 / 15|
|Sete imports|8 / 8 / 9|8 / 16 / 24|
|Oito imports|9 / 18 / 27|9 / 18 / 27|
|16 imports|17 / 34 / 51|17 / 34 / 51|

O limite de oito entradas da SourceSession explica a descontinuidade de oito imports mais o arquivo principal. O limite de bytes não é vinculante nestas fixtures pequenas. Isso é thrashing da política atual, não falha de correção. A biblioteca padrão é reconstruída mesmo quando o CST é reutilizado. Na fixture de 16 imports, 37 das 53 amostras de CPU incluem especificamente `make_stdlib_with_features`. É uma frente mais delimitada que memoizar Module completo. Breakpoints contam entradas observadas; funções inline e caminhos não exercitados continuam fora dessa medida.

Outline entrou seis vezes em `run_planned` e criou seis Layouters; notas tiveram duas entradas em convergência e duas criações adicionais para preflight de footnote. Não são seis tentativas intercambiáveis nem autorização para remover convergência. Faltam cardinalidades/cópias por campo de cada tentativa.

Microbancadas release, sete processos por caso, medianas:

|Operação|Parâmetros|Tempo|
|---|---|---:|
|Clone de Dict com Arrays|100 clones, 16 / 256 / 4096 entradas|0,130 / 1,939 / 35,051 ms|
|Clone + into_text CST|100 operações, 8192 bytes, profundidade 0 / 16 / 64 / 256|0,945 / 1,259 / 2,301 / 6,498 ms|
|Formatter de diagnóstico|100 erros, fontes de 1280 / 81920 / 1310720 bytes|0,897 / 40,330 / 964,300 ms|
|Shaping|256 runs, 3584 glifos, repetidos / únicos|1,189 / 7,225 ms|
|Métricas|8192 chamadas, repetidas / únicas|1,329 / 2,483 ms|

São custos compostos atuais, sem speedup de candidato nem atribuição isolada de chave, lock, buffer ou cópia. No assembly release inspecionado de `select_shaped`, três callsites estáticos chegam ao panic de bounds após resolver GOT/relocações; não são três checks executados por parágrafo. Não se usou unchecked.

Tamanhos x86-64: Value e Binding 144 bytes; Content 120; FrameItem 448; SyntaxNode 32; TextStyle 336; StyleChain 8. Tamanho não revela frequência nem bytes vivos e não justifica encaixotar variantes globalmente.

O contador do alocador encaminha para System, sem reset e com snapshots antes/depois. Não mede malloc estrangeiro, mmap, RSS ou bytes efetivamente copiados; o pico é cumulativo do processo. Não se usam seus tempos como benchmark. Em bancada distinta, 100 clones de Dict com 4096 entradas solicitaram 251.496.000 bytes em 409.800 alocações, com saldo vivo zero. CST profundo e formatter também apresentam pressão crescente; suas fixtures Unicode de alocação diferem das microbancadas de tempo, portanto os números não devem ser divididos entre si.

Foram executadas 139 cargas do profiler: 129 terminam com sucesso e dez são recusas HTML existentes (cinco fixtures × modos de World), também confirmadas na CLI. Recusas não entram em comparações de exportação bem-sucedida. Os snapshots de World/documento não atribuem alocações a campos privados, nem demonstram vazamento. Nos casos repetidos há saldo inicial de caches e depois zero em várias fixtures.

Query: nove controles públicos exatos de JSON/YAML passaram; uma consulta real de 200 metadados produz valores equivalentes nos dois formatos. Em 18 cargas de alocação, projetar um campo de Dict com 4096 entradas ainda solicita ~160,4 MB por 100 chamadas, versus ~329,5 MB para JSON compacto completo. Isso sustenta testar projeção antes da conversão total; nenhum serializer alternativo foi implementado. PNG composto: quatro controles públicos passam, incluindo dimensões e 11 coordenadas RGBA decodificadas; não há caller CLI identificado desse helper, cuja rota difere da exportação PNG da CLI.

## Cobertura dos 28 candidatos e limites

|Candidato|Resultado desta rodada; o que permanece sem demonstração|
|---|---|
|C01 imagens|RED determinístico no PDF; retenção medida separadamente; reciclagem no watch não reproduzida.|
|C02 fontes|Delta integral atribuído; ganho local pareado; causalidade da aquisição posterior ainda aberta.|
|C03 chave de shaping|Baseline público repetido/único e testes internos; sem alternativa de chave ou atribuição por lookup.|
|C04 métricas|Baseline e testes internos; sem separar chave/lock/compute em hits.|
|C05 buffers rustybuzz|Carga real e testes de shaping; nenhum reset/reúso alternativo comprovado.|
|C06 glifos em hits|Resultados públicos exercitados; frequência e bytes de cópias privadas ainda Unknown.|
|C07 walker|Pipeline e alocações globais; falta atribuição por camada/profundidade e scratch alternado.|
|C08 convergência|Entradas/callers e perfil de outline/notas; faltam cardinalidades e bytes por campo.|
|C09 checkpoints|Gates internos existentes; journal/deltas e custo por checkpoint não medidos isoladamente.|
|C10 seletor|Parágrafos, gates internos e assembly; sem candidato scratch ou ganho no pipeline.|
|C11 preparação|Carga, pressão global e gates internos; lifetime comum para arena não demonstrado.|
|C12 estilos|Fixture e gates internos; profundidades/propriedades reais ainda não atribuídas.|
|C13 stdlib|Reconstrução por arquivo/ciclo e concentração de amostras demonstradas; candidato de reúso ainda não implementado.|
|C14 imports/comemo|Reeval e controles de identidade exercitados; Module completo continua sem chave/contrato validado.|
|C15 coleções|Clone profundo e alocações medidos; sem troca de representação.|
|C16 carriers|Tamanhos medidos; falta distribuição de variantes vivas e custo da indireção.|
|C17 índices u32|Pré-condição de tamanho medida; arena/geração/identidade não especificadas, sem protótipo de produto.|
|C18 CST|Custo por profundidade medido; frequência produtiva não demonstrada (entradas não observadas nas fixtures depuradas).|
|C19 PDF|Pipeline e custos globais medidos; nenhum novo writer nem capacidade ótima demonstrada.|
|C20 HTML/SVG|Perfis dirigidos e gates de escaping; dez recusas HTML preservadas; sem serializer alternativo.|
|C21 PNG|Dimensões/pixels corretos no helper público; sem caller CLI ou ganho de composição demonstrado.|
|C22 recursos exportados|Cargas/controles existentes e alocações globais; sem nova política de caches ou cardinalidade atribuída.|
|C23 World/threads|Baseline não isola custo material de dyn; genericidade/parallel layout não justificados.|
|C24 disco|Owner atual proíbe cache nessa rota; não implementado para contornar o contrato.|
|C25 plugins|RED em mocks, WASM real e watch; corrigir identidade antes de ampliar memoização.|
|C26 SourceSession|Limiar de oito entradas reproduzido; alteração da política ainda exige experimento limitado.|
|C27 diagnóstico|Escalada com fonte grande demonstrada; candidato de empréstimo/recorte ainda não implementado.|
|C28 query|Controles e custo de projeção medidos; serialização dirigida pelo campo é próximo teste.|

Perfil de CPU separado: 42 processos PDF e nove HTML/SVG, amostragem SIGPROF de 1 ms. Para retirar descoberta de fontes da triagem de RAM usou-se configuração Fontconfig privada vazia com fontes embutidas, **não o ambiente padrão**. Frames são inclusivos, sobrepostos e poucas amostras não sustentam percentuais gerais. Raw concentra amostras no layout de raw; sua inicialização já é lazy. Não há evidência para acrescentar outro Lazy ou paralelizar páginas agora.

O lote amplo de 19 fixtures × três rodadas completou 57 CLIs e 57 watches com 171 publicações de startup/mesmos bytes/edição. Contudo, os tempos variaram muito: plain CLI CPU 124,402 / 158,591 / 103,694 ms; controle posterior com o mesmo SHA 91,568 / 91,312 / 88,851 ms. A causa não foi isolada. **Esse lote é exploratório e não sustenta diferenças pequenas ou ranking quantitativo.** O wall de recompilação inclui eventos/debounce; não é tempo puro do compilador.

## Próximos experimentos sustentados

1. Corrigir identidade de imagens e namespace de plugins, primeiro nos L0s e com os REDs como regressões. Classificar cada mudança pela ADR-0127; não ampliar lifetime de caches enquanto essas fronteiras falham.
2. Na frente de fontes, reduzir somente observação canônica redundante, com as provas atuais e bancada pareada em várias fixtures. Separar a queda posterior da aquisição do custo da preparação conjunta antes de atribuir causalidade.
3. Na frente de RAM, testar reúso da biblioteca padrão **dentro de uma compilação**, preservando escopos/features/imports e isolamento dos Worlds. Comparar com os 17 builds atuais na fixture de 16 imports; só depois reconsiderar memoização de Module.
4. Testar formatter emprestando/recortando a fonte, depois query projetando antes da conversão. São custos isoláveis com testes públicos existentes. Para CST, exigir primeiro caller produtivo quente antes de otimizar a microbancada.
5. Instrumentar campos de convergência, chave de shaping e buffers individualmente. Só os custos atribuídos devem escolher scratch, arena ou representação compartilhada. Índices globais, boxing amplo e threads permanecem sem suporte suficiente.

Esta ordem distingue correção, ganho local medido e hipóteses ainda abertas; não autoriza alteração produtiva por este diagnóstico.

## Validação e integridade

Os filtros internos selecionados passaram: **176 testes core e 285 infra**, quatro ignorados pelas marcações existentes. Incluem checkpoints, preparação/seleção de parágrafos, StyleChain, SyntaxNode, Value, measurement cache, convergência, shaping, métricas, SourceSession, FontSession, geração, HTML, SVG e builder PDF. Não se afirma execução integral dos 9986 testes listados nos dois binários.

A instrumentação tem oito funções de teste (sete cenários e uma entrada de subprocesso); todos passam. O parser tem 19 testes, incluindo dumps reais de timing/consultas e ausência de saída em Off. As mutações dirigidas foram rejeitadas: produtor 7/7, parser 7/7 e contador de alocações 6/6. Neste último, cada mutante executou normalmente e foi rejeitado pelo validador independente. Controles explícitos cobrem alloc_zeroed, crescimento, redução e dealloc, preservação de bytes e continuidade dos contadores; não equivalem a prova de todos os interleavings ou falhas de alocação.

As duas falhas produtivas permanecem abertas nos testes públicos. O alvo de retenção zero é registrado separadamente do contrato atual. O script independente de watch termina com código 1 ao detectar o plugin antigo, depois de aprovar as duas CLIs frescas e o startup.

Âncoras produtivas das decisões: imagens `03_infra/src/export/images/scan.rs:25`, `:481` e `:520`; identidade de plugins `01_core/src/entities/plugin_func.rs:83` e `:93`; construção da biblioteca `01_core/src/compiler/eval/mod.rs:640` e `:848`; limite de fontes de texto `03_infra/src/world/source_session.rs:15`. Os owners e hipóteses restantes estão ligados no catálogo anterior. Nenhum desses arquivos foi alterado nesta rodada.

Baseline release e binários instrumentados compilados com sucesso; `crystalline-lint .` reexecutado no checkout final, código de saída zero e sem violações bloqueantes. As notas informativas existentes não foram alteradas.

Integridade conferida em `2026-11-05T18:55:43.983764+00:00`: os **52911 arquivos recebidos** mantêm os mesmos bytes. O `git diff HEAD --stat=240` permanece idêntico ao inicial (a largura é necessária para comparação textual). As únicas adições não ignoradas são o laboratório novo e este diagnóstico; os dados de domínio ficam na área de resultados ignorada pelo Git.
