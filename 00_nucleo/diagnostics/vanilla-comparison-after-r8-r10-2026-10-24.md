# Comparação com o vanilla fixado após R8–R10 — 2026-10-24

> Snapshot histórico: a [comparação após R11–R13](vanilla-comparison-after-r11-r13-2026-10-25.md) reconfirma as correções, qualifica 15 dos 32 casos adicionais e mantém 17 abertos. As três fontes percentuais de R12 também qualificam. Os números abaixo conservam a rodada anterior.

> Atualização R11: as omissões das notas literal e aninhada foram corrigidas; restam 20 dos 32 casos adicionais. Consulte o [diagnóstico R11](long-footnotes-r11-2026-10-24.md). Os resultados abaixo conservam o snapshot anterior.

**Os resultados declarados de R8–R10 foram reconfirmados nos corpora originais, em debug e release.** Os dez casos originais de notas, os controles de numeração/fontes, as 75 notas longas e os 69 controles de `super` continuam qualificados a **0,001 pt**, com palavras completas por página e origens independentes no SVG. Os seis limites de `super` fechados por R10 e a nota longa fechada por R9 continuam fechados nos seus reprodutores.

A expansão independente contém **32 fontes aceitas pelo vanilla**: **dez qualificam em geometria PDF/SVG e 22 não qualificam**, com observações idênticas em debug e release. São 22 casos, não 22 defeitos independentes. Há novas omissões de texto extraído em notas, configurações de entrada não respeitadas, argumentos públicos de `super` rejeitados e limites de composição de scripts. A pintura foi medida separadamente; dois dos dez controles geometricamente qualificados ainda têm resíduos mínimos no SVG.

Este é o diagnóstico atual, sucedendo o [snapshot após R1–R7](vanilla-comparison-after-r1-r7-2026-10-24.md). Os relatórios individuais [R8](footnote-layout-r8-2026-10-24.md), [R9](long-footnotes-r9-2026-10-24.md) e [R10](superscript-r10-2026-10-24.md) e suas evidências permanecem intactos. Esta rodada não modificou código produtivo, L0, configurações, fontes de referência, classificadores ou tolerâncias.

## Estado e proveniência

- HEAD `cf21f47a3b63f0e9fc5308925e1d0b67925e7227`, **working tree não commitado**. [baseline.json](../../lab/results/tekt-vs-vanilla-after-r8-r10-2026-10-24/baseline.json) contém a lista exata de alterações recebidas (`git diff HEAD --stat`), status e os 2.162 inputs produtivos/config/L0; [received-source.json.gz](../../lab/results/tekt-vs-vanilla-after-r8-r10-2026-10-24/received-source.json.gz) conserva o conteúdo. Os inputs são exatamente os selados no encerramento de R10 e permanecem iguais no fim desta rodada.
- Oracle: commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`, árvore `cac836c07e4dec2a89219370428292393a3216b5`, executável SHA-256 `9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`. A identidade foi conferida contra a [receita e verificação de fontes ratificadas](../../lab/results/construction/diagnostics/vanilla-resync-2026-10-17/relatorio.md); não houve resync nem substituição por versão textual.
- Release reconstruído de `2026-10-24T21:32:23.268438+00:00` a `2026-10-24T21:35:11.670449+00:00` UTC, Rust 1.92.0, offline/locked, um job, `opt-level=s`, ThinLTO e uma unidade de geração. SHA-256 `0d88239ebcd6e5a05b85f2bf1458f9da5a561ba9018688170cc41c30b75d72eb`. O build debug também reproduziu o artefato de R10 (`af269b123cb4b7dbeda9bca4e2282b4aa75fe83d670be7db06a0245eb2da2aee`). [candidate-build/receipt.json](../../lab/results/tekt-vs-vanilla-after-r8-r10-2026-10-24/candidate-build/receipt.json) e [candidate-build/source-state.json.gz](../../lab/results/tekt-vs-vanilla-after-r8-r10-2026-10-24/candidate-build/source-state.json.gz) vinculam o build às fontes.
- [driver.json](../../lab/results/tekt-vs-vanilla-after-r8-r10-2026-10-24/driver.json) registra comandos, horários e logs. As medições de desempenho/RSS ocorreram em série, após os builds e sem compilações concorrentes, sob o lançamento configurado com teto de 12 GiB e zero swap. O benchmark registra o cgroup e os limites observados; a receita não é uma alegação de causalidade arquitetural.

## Validação dos lotes corrigidos

| Lote | PDF por build | SVG por build | Maior delta PDF | Maior delta de origem SVG |
|---|---:|---:|---:|---:|
| R8 | 28 | 22 | 0.000016000 pt | 0.000000001000 pt |
| R9 | 75 | 75 | 0.000030000 pt | 0.000000001000 pt |
| R10 | 69 | 69 | 0.000021000 pt | 0.000000001000 pt |

As contagens são por corpus de lote, com sobreposição entre os reprodutores; não representam documentos únicos somados. O controle bitmap de R10 preserva PNG decodificado e geometria de imagem, com **zero pixels diferentes a 72/600 dpi** nas duas fontes afetadas, em ambos os builds. Os 16 refutadores herdados de R9 passam integralmente. Fontes e critérios dos corpora originais foram conferidos contra os recibos históricos, sem remoção de casos.

Os **21 replays A–H/R1–R7 e cinco suplementos passaram**. Permanecem os 198 símbolos públicos: **162 MATCH e 36 EXTENSION** em ambos os builds. A extensão continua distinta de paridade. As matrizes públicas em seis perfis mantêm zero UNKNOWN/VIOLATED e duas observações PUBLIC_FORMAT por perfil. [comparison-commands.json](../../lab/results/tekt-vs-vanilla-after-r8-r10-2026-10-24/comparison-commands.json), [revalidation/replays.json](../../lab/results/tekt-vs-vanilla-after-r8-r10-2026-10-24/revalidation/replays.json), [revalidation/supplement.json](../../lab/results/tekt-vs-vanilla-after-r8-r10-2026-10-24/revalidation/supplement.json) e [revalidation/audit.json](../../lab/results/tekt-vs-vanilla-after-r8-r10-2026-10-24/revalidation/audit.json) conservam os resultados completos.

Foram repetidos agora **43 testes de módulo de R8–R10**, com zero falhas. [module-tests/receipt.json](../../lab/results/tekt-vs-vanilla-after-r8-r10-2026-10-24/module-tests/receipt.json) contém cada filtro, contagem e horário. O build e o lint completo/linhagem foram repetidos. Os 12 gates completos do workspace de R10 — 9.921 testes Rust aprovados, sete ignorados preexistentes e 42 Python, Clippy/fmt/Rustdoc — são **evidência anterior reutilizada**, após igualdade exata dos inputs e verificação dos recibos; não foram apresentados como uma nova execução integral. A nova rodada de corpora, módulos, CLI, perfis e recursos está registrada separadamente em [audit.json](../../lab/results/tekt-vs-vanilla-after-r8-r10-2026-10-24/audit.json).

## Divergências adicionais em R8–R10

| Grupo | Casos não qualificados | Observação medida |
|---|---:|---|
| Notas longas fora dos controles originais | 4 | Quebras literais, quebra terminal, primeira linha vazia e nota aninhada longa |
| Configuração da entrada da nota | 3 | `indent: 2em`, `gap: 12pt` e `separator: none` |
| Argumentos públicos de `super` | 6 | Três fontes × duas formas com `typographic: false`; a forma combinada também contém baseline/tamanho |
| Composição de scripts | 9 | `super`/`sub` aninhados, corpo em `box` e quebra dentro de link em Nimbus Sans |

Os reprodutores e limites abaixo são observações novas, não uma reatribuição das falhas antigas já fechadas:

- **Notas longas:** [quebras literais](../../lab/results/tekt-vs-vanilla-after-r8-r10-2026-10-24/refuters-release/note-long-literal-breaks.typ) produzem uma página em Crystalline contra três no vanilla e omitem `L3`, `L4`, `L5` da extração; a pintura também diverge. A [nota aninhada longa](../../lab/results/tekt-vs-vanilla-after-r8-r10-2026-10-24/refuters-release/note-long-nested.typ) mantém duas páginas nos dois executáveis, mas omite `L6` em Crystalline. A [quebra terminal](../../lab/results/tekt-vs-vanilla-after-r8-r10-2026-10-24/refuters-release/note-long-trailing-break.typ) preserva os caracteres globais, porém distribui o conteúdo por quatro páginas contra três. A [linha inicial vazia](../../lab/results/tekt-vs-vanilla-after-r8-r10-2026-10-24/refuters-release/note-long-leading-blank.typ) mantém palavras/páginas, com delta máximo de 1,403865 pt. Os números e os termos omitidos vêm dos PDFs completos, não de ordem de extração ou comparação de bytes.
- **Entradas:** [indentação](../../lab/results/tekt-vs-vanilla-after-r8-r10-2026-10-24/refuters-release/note-entry-indent.typ) e [gap](../../lab/results/tekt-vs-vanilla-after-r8-r10-2026-10-24/refuters-release/note-entry-gap.typ) retêm deltas de 9,350001 pt e 6,500001 pt. Em [separator:none](../../lab/results/tekt-vs-vanilla-after-r8-r10-2026-10-24/refuters-release/note-entry-no-separator.typ), as caixas de texto qualificam, mas uma regra indevida permanece: a 600 dpi são 3.251 pixels diferentes no PDF e 3.255 no SVG. A ordem de extração diferente em `gap` é registrada separadamente; os caracteres globais estão preservados.
- **Sintaxe pública de `super`:** o [reprodutor](../../lab/results/tekt-vs-vanilla-after-r8-r10-2026-10-24/refuters-release/DejaVu%20Sans-forced-synthesis.typ) é aceito pelo vanilla e rejeitado em Crystalline com argumento nomeado inesperado `typographic`. O [contrato upstream](../../lab/results/tekt-vs-vanilla-after-r8-r10-2026-10-24/additional-source-evidence.json) documenta a opção; o proprietário atual `stdlib/text/shift.rs` aceita somente `size`, e seu L0 conserva explicitamente a limitação de offset configurável. Esses seis casos não isolam o comportamento de `baseline` depois de corrigida a rejeição de `typographic`.
- **Composição:** há diferenças visíveis em scripts aninhados e em `box`, até 3,097485 pt nos controles com palavras comparáveis. O [link com quebra em Nimbus Sans](../../lab/results/tekt-vs-vanilla-after-r8-r10-2026-10-24/refuters-release/Nimbus%20Sans-linked-break.typ) preserva os caracteres globais, mas difere em segmentação de palavras, origens SVG e pintura (47 pixels PDF/56 SVG a 72 dpi); não é classificado como perda de texto. As causas internas destes limites de layout não foram atribuídas por esta rodada.

[refuters-debug/receipt.json](../../lab/results/tekt-vs-vanilla-after-r8-r10-2026-10-24/refuters-debug/receipt.json), [refuters-release/receipt.json](../../lab/results/tekt-vs-vanilla-after-r8-r10-2026-10-24/refuters-release/receipt.json) e [refuter-paint/receipt.json](../../lab/results/tekt-vs-vanilla-after-r8-r10-2026-10-24/refuter-paint/receipt.json) guardam todos os 32 casos em ambos os builds. Todos os casos que compilam em ambos foram rasterizados em PDF/SVG a 72 dpi; quatro controles selecionados antes da observação também a 600 dpi. Entre os dez geometricamente qualificados, `Liberation Serif-nested-super` conserva dois pixels SVG diferentes e `note-entry-clearance` três, todos com delta máximo de canal 1 a 72 dpi. Esses resíduos não foram convertidos em falhas de geometria nem ocultados como pintura idêntica.

A primeira tentativa do observador adicional parou por import ausente de `re`; os artefatos permanecem separados. A exploração seguinte usou `size:75%` em três fontes, rejeitado também pelo vanilla; esses casos inválidos foram preservados e não contam como defeitos de paridade. A matriz final troca somente esse valor por `0.75em`. [refuter-harness-correction.json](../../lab/results/tekt-vs-vanilla-after-r8-r10-2026-10-24/refuter-harness-correction.json) e [refuter-ratio-correction.json](../../lab/results/tekt-vs-vanilla-after-r8-r10-2026-10-24/refuter-ratio-correction.json) documentam as correções de harness, sem alteração dos critérios. A auditoria final também precisou renomear uma variável local do observador de dimensões de página; a falha e a correção permanecem em [audit-correction.json](../../lab/results/tekt-vs-vanilla-after-r8-r10-2026-10-24/audit-correction.json), e os cinco estágios finais aprovados em [after-final.json](../../lab/results/tekt-vs-vanilla-after-r8-r10-2026-10-24/after-final.json). Nenhuma medição ou tolerância mudou com esse ajuste.

## Divergências anteriores ainda medidas

- **R2:** os 24 controles mantêm 17 geometrias qualificadas, quatro variantes justificadas com sobreposição óptica de pontuação e três contextos recusados (`internal`, `numeric`, `standalone`). Os 14 controles de fronteira preservam os limites com tracking não zero; `overhang:false` continua qualificado.
- **R3:** no corpus de 47 casos permanecem `control-markup-base`, `control-markup-vs` e `control-math-explicit-break`, relativos a fallback ordinário/baseline após quebra. Os 19 controles originalmente corrigidos continuam qualificados.
- **R4:** os 210 controles de campos/conversão continuam dentro dos critérios (170 valores JSON e 40 rejeições bilaterais). `accent`, `class` e `equation` continuam limites geométricos. Ordem SVG/extração e geometria física continuam observações distintas; layout não substitui o gate ordenado.
- **R5:** `different`, `fill`, `gutter`, gradientes, tiling e o PNG nativo conservam suas diferenças. As duas observações PUBLIC_FORMAT da matriz continuam relativas a PNG e nomes/encoding de fontes, sem conversão automática em defeitos de linguagem. As contagens exatas seguem abaixo e em [remaining.json](../../lab/results/tekt-vs-vanilla-after-r8-r10-2026-10-24/remaining.json).
- **R6:** os gates relativos à referência R5 foram mantidos, mas não implicam menor RSS ou maior velocidade em todos os casos diante do vanilla. As observações atuais estão nas tabelas de recursos e latência.
- **R7:** não há falha remanescente em seu gate declarado; as 36 extensões permanecem extensões.

| Pintura | PDF 72 dpi | SVG 72 dpi | PDF 600 dpi | SVG 600 dpi | Maior delta de canal |
|---|---:|---:|---:|---:|---:|
| different | 26 | 172 | 1704 | 2911 | 255 |
| fill | 0 | 0 | 0 | 12 | 1 |
| gutter | 562 | 1006 | 33664 | 37241 | 255 |
| gradient-line | 556 | 473 | 32490 | 28178 | 148 |
| gradient-table | 1482 | 1968 | 102675 | 107172 | 255 |
| tiling-line | 0 | 0 | 0 | 105 | 1 |
| tiling-local | 0 | 0 | 0 | 38 | 1 |

O PNG nativo continua com 11 pixels diferentes e delta máximo de canal 1. As diferenças de pintura de cada página, tamanhos e observações recusadas continuam nos recibos. Não há certificação para resoluções não medidas.

## Recursos e latência atuais

Os sete workloads mantiveram os gates originais de R6: tamanho normalizado ≤95% de R5 e ≤oracle; média geométrica RSS ≤97% de R5; cada caso ≤103%; variação entre medianas de rodadas ≤3%. A razão geométrica RSS atual/R5 foi **0.945781** e a razão de tamanho normalizado **0.855199**. Os três testes de estresse passaram. [resource-control-provenance.json](../../lab/results/tekt-vs-vanilla-after-r8-r10-2026-10-24/resource-control-provenance.json) autentica as referências históricas de R5/receita; os resultados atuais não são apresentados como ganhos causais de R10 sobre R9.

| PDF | Atual KiB | Vanilla KiB | Atual / vanilla |
|---|---:|---:|---:|
| empty | 26648 | 27288 | 0.976546 |
| hello | 30260 | 31088 | 0.973366 |
| prose | 37328 | 34592 | 1.079093 |
| math | 35332 | 34060 | 1.037346 |
| table | 31540 | 33644 | 0.937463 |
| multipage | 31000 | 36804 | 0.842300 |
| context | 31508 | 35072 | 0.898380 |

| Executável | Atual bytes | Vanilla bytes |
|---|---:|---:|
| Original, com símbolos | 50714352 | 52271128 |
| Cópias descartáveis com strip | 42787368 | 52271120 |

RSS acima do vanilla nesta medição: `prose` (+7.91%), `math` (+3.73%). Esses custos permanecem explícitos.

Vantagem de latência creditada nos dois perfis: 8/10 cargas. Lentidão estável nos dois perfis: `math`.

Em `prose`, o IC95% do perfil padrão não supera integralmente o gate de 1,05; a vantagem não foi creditada nesse perfil. Isso não foi atribuído causalmente a um lote.

O benchmark usa 20 pares de processos novos, duas execuções de aquecimento por papel, ordem balanceada/randomizada e IC95% por bootstrap pareado; ambos os perfis têm o mesmo gate independente de saída. Razão é tempo vanilla / atual; valores abaixo de 1 indicam Crystalline mais lento. Vantagem só é creditada quando todo o IC excede 1,05 e a saída qualifica.

| Carga | Atual ms, padrão | Vanilla ms, padrão | Razão padrão [IC95%] | Razão 1 CPU [IC95%] | Saída |
|---|---:|---:|---|---|---|
| empty | 96.16 | 121.89 | 1.2729 [1.2662; 1.2870] | 1.2829 [1.2742; 1.2924] | qualificada |
| hello | 98.27 | 123.84 | 1.2682 [1.2544; 1.2770] | 1.2802 [1.2707; 1.2862] | qualificada |
| prose | 130.35 | 135.50 | 1.0387 [1.0363; 1.0517] | 1.0658 [1.0565; 1.0793] | qualificada |
| math | 156.34 | 127.98 | 0.8147 [0.8066; 0.8243] | 0.8222 [0.8112; 0.8279] | qualificada |
| table | 105.88 | 128.80 | 1.2211 [1.2114; 1.2293] | 1.2382 [1.2292; 1.2497] | qualificada |
| multipage | 101.44 | 125.01 | 1.2343 [1.2170; 1.2472] | 1.2438 [1.2288; 1.2520] | qualificada |
| context | 101.72 | 124.74 | 1.2232 [1.2144; 1.2337] | 1.2476 [1.2362; 1.2639] | qualificada |
| eval-startup | 4.41 | 6.07 | 1.3886 [1.3521; 1.4212] | 1.4165 [1.3778; 1.4356] | qualificada |
| eval-fold | 11.66 | 19.54 | 1.6919 [1.6380; 1.7240] | 1.6769 [1.6509; 1.7971] | qualificada |
| eval-map | 5.14 | 7.06 | 1.3722 [1.3380; 1.4217] | 1.4024 [1.3660; 1.4307] | qualificada |

[benchmark/receipt.json](../../lab/results/tekt-vs-vanilla-after-r8-r10-2026-10-24/benchmark/receipt.json), [revalidation/final-resources/receipt.json](../../lab/results/tekt-vs-vanilla-after-r8-r10-2026-10-24/revalidation/final-resources/receipt.json) e [revalidation/legacy-resources/receipt.json](../../lab/results/tekt-vs-vanilla-after-r8-r10-2026-10-24/revalidation/legacy-resources/receipt.json) conservam amostras, limites, variantes, RSS e intervalos. Não há extrapolação para documentos arbitrários, caches em processo ou watch de longa duração.

## Preservação e alcance

[preservation.json](../../lab/results/tekt-vs-vanilla-after-r8-r10-2026-10-24/preservation.json) confirma **2.162 inputs recebidos intactos**, os 1.853 assets de referência, a fixture bitmap adicional e os **49102 arquivos históricos** de A–H e R1–R10 verificados por hash/tamanho. O encerramento [completion.json](../../lab/results/tekt-vs-vanilla-after-r8-r10-2026-10-24/completion.json) registra documentos, comandos, fontes, binários e inventário final. Só novos arquivos de diagnóstico e os ponteiros atuais foram alterados nesta rodada.

A via Type3 certificada em R10 continua limitada a conjuntos completos suportados de até 256 glifos PNG por fonte; outros formatos/conjuntos/caminhos mantêm seu escopo anterior. O inventário finito acima é refutável pela repetição das mesmas fontes com os mesmos observadores e tolerâncias. Não prova ausência exaustiva de divergências nem autoriza copiar mecanismos ou defeitos acidentais do vanilla (ADR-0107/0108).
