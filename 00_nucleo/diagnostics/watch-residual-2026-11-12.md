## Atualização 2026-11-13T09:08:17.178041+00:00 — buffers físicos preservados, laboratório

A projeção de links conserva vetores sem edição local e processa descendentes
com os mesmos escopos/destinos. Texto simples 2,341→2,395 ms; vanilla 1,661; 100 parágrafos 13,343→13,254 ms; vanilla 3,744; Sumário 22,445→21,593 ms; vanilla 4,209. Meta vanilla aberta;
sem integração. Passaram 10302 testes,
controles de saída/fontes/invalidação e perfis independentes. Tempos pareados,
contrapartidas e proveniência em
`lab/results/construction/diagnostics/watch-residual-2026-11-12/relatorio.md`
e `frame-preserve/conclusion.json`. O reúso de layout completo entre compilações
e de produtos de exportação ainda precisa preservar dependências e ocorrência.

## Atualização 2026-11-13T08:43:56.171216+00:00 — reúso de laços estruturados, laboratório

Corpos intactos com heading/Strong/Emph e lorem/pagebreak certificados agora
podem reter avaliação com guardas de dependências, efeitos, origem e orçamento.
Texto simples 2,386→2,354 ms; vanilla 1,709; 100 parágrafos 13,473→13,698 ms; vanilla 3,766; Sumário 24,147→22,404 ms; vanilla 4,252. A meta vanilla permanece aberta; nenhuma integração.
Passaram 10297 testes, controles existentes
e30comparações adicionais de15revisões com geometria, links e raster.
Tempos pareados, contrapartidas, perfis e proveniência estão em
`lab/results/construction/diagnostics/watch-residual-2026-11-12/relatorio.md`
e `pure-loop/conclusion.json`. A reconstrução física e a emissão de PDF
continuam como trabalho estrutural a atacar.

## Atualização 2026-11-13T08:07:02.642243+00:00 — grafo builtin, somente laboratório

Eliminadas1411definições de bindings por recompilação; matching root/import
compartilham grafo/stdlib com guardas completas e registry atual isolado.
Texto simples 2,596→2,347 ms; vanilla 1,612; 100 parágrafos 14,161→13,580 ms; vanilla 3,820; Sumário 24,005→24,126 ms; vanilla 4,266. Simples/layout melhoram12/12pares; sumário6/12,
sem ganho demonstrado, e abertura do sumário piora11/12. Meta vanilla aberta.
Passaram10285testes e controles de saída/fontes/invalidacão, sem integração.
Detalhes, contrapartidas e proveniência em
`lab/results/construction/diagnostics/watch-residual-2026-11-12/relatorio.md`
e `builtin-graph/conclusion.json`. Próxima auditoria: efeitos/leituras que
impedem reter o corpo intacto do laço com heading e chamadas de função.

## Atual: biblioteca builtin retida; meta de desempenho ainda aberta

2026-11-13T07:33:56.168942+00:00: somente laboratório. Texto simples 3,311→2,546 ms; vanilla 1,687; 100 parágrafos 14,766→13,725 ms; vanilla 3,735; Sumário 24,835→24,339 ms; vanilla 4,154. 10279 testes e252comparações de cache quente passaram, além dos gates e controles. Nenhuma integração. [Resultados/proveniência](../../lab/results/construction/diagnostics/watch-residual-2026-11-12/builtin-root/conclusion.json), [censo](../../lab/results/construction/diagnostics/watch-residual-2026-11-12/builtin-root/builtin-root-work-shift/causal-summary.json), [perfil](../../lab/results/construction/diagnostics/watch-residual-2026-11-12/builtin-root/builtin-root-deep-stacks/structural-summary.json), [relatório](../../lab/results/construction/diagnostics/watch-residual-2026-11-12/relatorio.md).

## Atual: produtos de linha medidos; meta ainda aberta

2026-11-13T06:55:43.809573+00:00: somente laboratório. Texto simples 3,432→3,381 ms; vanilla 1,706; 100 parágrafos 15,009→14,820 ms; vanilla 3,764; Sumário 24,810→24,697 ms; vanilla 4,208. 10268 testes e 252 comparações de cache quente passaram, além dos gates e controles. Nenhuma integração. [Resultados e proveniência](../../lab/results/construction/diagnostics/watch-residual-2026-11-12/line-products/conclusion.json), [censo](../../lab/results/construction/diagnostics/watch-residual-2026-11-12/line-products/line-products-work-shift/causal-summary.json), [perfil](../../lab/results/construction/diagnostics/watch-residual-2026-11-12/line-products/line-products-deep-stacks/structural-summary.json), [relatório](../../lab/results/construction/diagnostics/watch-residual-2026-11-12/relatorio.md).

## Atual: shaping repetido eliminado; reconstrução ainda custa mais

2026-11-13T06:15:44.686665+00:00: somente laboratório. 100 parágrafos 16,878→14,997 ms (vanilla 3,683); sumário 26,853→25,187 ms (vanilla 4,234), 12/12 pares melhores em ambos. Backend por edição 112→1 e 64→1; regravação idêntica chega a zero. Isso elimina trabalho real, mas ainda deixa 4,07×/5,95× o custo vanilla. 10256 testes e 252 comparações de cache quente passaram, além dos gates e controles anteriores. Investigar produtos físicos mais amplos e reconstrução residual; nenhuma integração, meta aberta. [Resultados e proveniência](../../lab/results/construction/diagnostics/watch-residual-2026-11-12/shaping-products/conclusion.json), [censo causal](../../lab/results/construction/diagnostics/watch-residual-2026-11-12/shaping-products/shaping-products-work-shift/causal-summary.json), [perfil](../../lab/results/construction/diagnostics/watch-residual-2026-11-12/shaping-products/shaping-products-deep-stacks/structural-summary.json), [relatório](../../lab/results/construction/diagnostics/watch-residual-2026-11-12/relatorio.md).

## Atual: passagem removida deslocava shaping; ganho limitado

2026-11-13T05:36:57.958504+00:00: somente laboratório. 100 parágrafos 17,285→16,921 ms (vanilla 3,760); sumário 27,725→26,943 ms (vanilla 4,209), com deltas pareados mistos. A sonda causal mostra que eliminar 2004 consultas não elimina o backend: 112 execuções por compilação continuam nos 100 parágrafos, apenas mudam de passagem. No sumário continuam 64. Precisamos ampliar o produto retido entre compilações além de preparação/quebras e buffers brutos. 10244 testes e controles passaram, incluindo frações aninhadas e atualização de fontes. Nenhuma integração; meta aberta. [Resultados e proveniência](../../lab/results/construction/diagnostics/watch-residual-2026-11-12/fraction-demand/conclusion.json), [sonda causal](../../lab/results/construction/diagnostics/watch-residual-2026-11-12/fraction-demand/fraction-demand-work-shift/causal-summary.json), [perfil](../../lab/results/construction/diagnostics/watch-residual-2026-11-12/fraction-demand/fraction-demand-deep-stacks/structural-summary.json), [relatório](../../lab/results/construction/diagnostics/watch-residual-2026-11-12/relatorio.md).

## Atual: certificado imutável reduz 21% da CPU do sumário

2026-11-13T05:10:44.087096+00:00: somente laboratório. CPU do sumário 34,918→27,584 ms; vanilla 4,277 ms, melhora nos 12 pares, ainda 6,45× o oracle. Nos 100 parágrafos, 17,779 contra 3,680 ms, sem ganho consistente. 10240 testes e gates/controles passaram; fontes tardias e substituídas invalidam corretamente. O perfil restante destaca uma análise de frações que mede sequências sem produzir alocação, além de shaping e exportação. Próximo passo é refutar essa redundância preservando as capacidades de escopo e geometria aninhada. Nenhuma integração; meta aberta. [Evidência e proveniência](../../lab/results/construction/diagnostics/watch-residual-2026-11-12/metric-epoch/conclusion.json), [perfil](../../lab/results/construction/diagnostics/watch-residual-2026-11-12/metric-epoch/metric-epoch-deep-stacks/structural-summary.json), [relatório](../../lab/results/construction/diagnostics/watch-residual-2026-11-12/relatorio.md).

## Atual: retenção física elimina três passagens, ganho ainda insuficiente

2026-11-13T04:34:57.591904+00:00: somente no laboratório, sumário passa de 6 para 3 execuções de layout físico. CPU 36,780→34,887 ms; vanilla 4,134 ms, 12/12 pares melhores. Cerca de 5% de ganho, ainda 8,4× vanilla. Texto simples e 100 parágrafos sem ganho consistente. Validação das dependências ocupa 11,5% das amostras inclusivas da candidata, motivando investigar um certificado mais barato sem enfraquecer a invalidação. Passaram 10231 testes e gates/controles completos. Nenhuma integração; meta aberta. [Evidências e proveniência](../../lab/results/construction/diagnostics/watch-residual-2026-11-12/physical-retain/conclusion.json), [perfil](../../lab/results/construction/diagnostics/watch-residual-2026-11-12/physical-retain/physical-retain-deep-stacks/structural-summary.json), [relatório](../../lab/results/construction/diagnostics/watch-residual-2026-11-12/relatorio.md).

## Atual: três passagens do sumário potencialmente reaproveitáveis

2026-11-13T03:29:54.152041+00:00: sonda no laboratório encontra 3 de 6 passagens com mesmo plano, consultas compatíveis e PageStore igual pela comparação existente, sem canais dinâmicos; reproduzido com largura e numeração diferentes. **Ainda não é prova completa de reúso nem ganho de CPU.** 10218 testes passam; 14 PDFs preservam palavras, páginas e coordenadas. Nenhuma integração; meta aberta. O caso de 100 parágrafos requer retenção entre compilações. [Evidência e limites](../../lab/results/construction/diagnostics/watch-residual-2026-11-12/layout-read-audit/conclusion.json), [relatório](../../lab/results/construction/diagnostics/watch-residual-2026-11-12/relatorio.md).

## Atual: fallback isolado por parágrafo medido no laboratório

2026-11-13T03:10:28.143071+00:00:10214testes passaram. 100parágrafos com deltas mistos; ganho consistente não estabelecido. Meta aberta, nenhuma integração. [Relatório e proveniência](../../lab/results/construction/diagnostics/watch-residual-2026-11-12/relatorio.md), `paragraph-fallback-validation/` e `evidence/freeze-paragraph-fallback.json`.

## Atual: compartilhamento imutável da introspecção medido no laboratório

2026-11-13T02:34:59.910037+00:00:10211 testes passaram; compartilhamento e isolamento confirmados. Sumário com deltas mistos; ganho consistente não estabelecido. Meta aberta, nenhuma integração. [Relatório e proveniência](../../lab/results/construction/diagnostics/watch-residual-2026-11-12/relatorio.md), `snapshot-share-validation/` e `evidence/freeze-snapshot-share.json`.

## Atual: retenção causal de Outline medida no laboratório

2026-11-13T02:08:18.225373+00:00:1766infra,3GREENs e2mutantes refutados; core8442 anterior com identidade de fontes/L0 verificada. Sumário39.383→39.563ms, vanilla3.208ms.100 parágrafos15.007→14.920ms, vanilla2.741ms. Resultado do sumário misto entre pares; não há ganho consistente estabelecido. Execuções Outline3→2, layouts físicos6→6 por edição. Apenas transação single-run comprovada, mesma captura, mesma compilação; descoberta/replay preservados. Sem integração; meta aberta. [Relatório/proveniência](../../lab/results/construction/diagnostics/watch-residual-2026-11-12/relatorio.md), `outline-retain-validation/` e `evidence/freeze-outline-retain.json`.

## Atual: reúso contextual validado; seis layouts ainda repetidos

2026-11-13T01:47:58.604353+00:00: somente no laboratório,10205 testes e gates/controles passaram. Sumário41,693→40,152ms, vanilla4,026ms; delta pareado−1,391ms,12/12 pares melhores.100 parágrafos17,709→17,952ms, vanilla3,376ms: sem ganho. Censo confirma6 layouts completos por compilação no sumário (3 candidatos × descoberta/replay), apesar de preparação direta78→6; no documento simples de100 parágrafos há1 layout. Próximo alvo é ampliar o produto puro reutilizado além da preparação, preservando leituras/efeitos atuais. Nenhuma integração; meta aberta. [Relatório e proveniência](../../lab/results/construction/diagnostics/watch-residual-2026-11-12/relatorio.md), `paragraph-context-validation/` e `evidence/freeze-paragraph-context.json`.

## Continuação: identidade corrigida; reúso contextual sob teste

2026-11-13T01:27:44.866532+00:00: dois REDs confirmaram a lacuna de proprietário de fonte e passaram após a correção somente no laboratório. 1762 testes infra e gates/controles passaram; core idêntico ao freeze anterior. A correção não demonstrou ganho de CPU:100 parágrafos18,224→18,886ms, vanilla3,564ms; delta pareado+0,422ms. Sumário teve delta adverso e permanece fora do reúso. Próxima candidata testa corpo direto/contexto sem reter execução contextual. Nenhuma integração; meta aberta. [Relatório e proveniência](../../lab/results/construction/diagnostics/watch-residual-2026-11-12/relatorio.md), `paragraph-proof-validation/` e `evidence/freeze-paragraph-proof.json`.

## Continuação: produtos de parágrafo reutilizados, ganho ainda insuficiente

Atualização 2026-11-13T01:13:47.661882+00:00. Candidata paragraph somente no laboratório, sem integração. O observador registra 99 hits após mudança de um parágrafo; o custo nominal continua 16,724 ms contra 2,929 ms do vanilla nos 100 parágrafos. Delta pareado −0,202 ms [−2,942; +1,815], sem ganho consistente e com aumento de RSS. 10.200 testes passaram, mais gates e controles; dois mutantes refutados. A rota de contexto do sumário ainda prepara 78 corpos por compilação e concentra cerca de 29,5 ms instrumentados. Há uma hipótese adicional de troca de proprietário da fonte dentro da vida de métricas, ainda sob teste. Meta aberta. Fontes, UTC, dirty-stat e limites em [relatório](../../lab/results/construction/diagnostics/watch-residual-2026-11-12/relatorio.md), `evidence/freeze-paragraph.json` e `paragraph-validation/`.

## Continuação: avaliação de trechos reaproveitada somente no laboratório

Em 2026-11-13T00:11:17.504230+00:00, a candidata subtree passou os gates e permanece sem integração. CPU dos 100 parágrafos: base 19,950 ms → candidata 18,871 ms; vanilla 3,641 ms na mesma campanha de doze pares. Delta pareado −1,021 ms [−2,775; +0,174]: ganho limitado, meta aberta. O observador confirma hits e invalidação de bindings importados. Layout ainda não é retido. Relatório e proveniência: [laboratório](../../lab/results/construction/diagnostics/watch-residual-2026-11-12/relatorio.md), `subtree-validation/conclusion.json` e patches `subtree*.patch`.

# Residual de watch — investigação aberta

Atualização 2026-11-12T23:33:17.748851+00:00: **Source::reparse + SourceSession implementados e testados
somente no laboratório**. O usuário dispensou novas autorizações para laboratório;
integração continua excluída. Inserir um heading fora do loop preserva agora sua
identidade e 50 registros exatos de nós, ante zero antes. Parse completo continua;
isto ainda não reutiliza resultados de avaliação/layout. 8423 testes core e 1755
infra passaram, além dos gates estritos, controles de watch/imports/Unicode/fontes
e dois refutadores de resultado obsoleto. Em 12 pares, 100 parágrafos:
19.901 → 19.798 ms,
vanilla 3.342 ms; deltas pareados mistos, **sem ganho robusto**.
Estado exato/UTC/ELFs/patch/limitações no relatório bruto, seção “Revisões imutáveis
de Source”, e `incremental-source-validation/`. Nenhum novo código integrado;
2499 hashes do checkout preservados. Próxima frente: reúso causal de produtos
de avaliação e layout, não mais redução isolada do custo do parse.

Estado atual de laboratório, atualizado após a campanha22:17:43.414652Z–
22:20:09.637323Z: World::select_font/font_info implementados e validados; pilha
fontdemandv2+loremlocal+linkscope+controlprimary+primaryproof+streammemo preservada,
sem integração. Na última campanha de doze pares,100 parágrafos20,913→19,924ms
de CPU (vanilla3,776ms), sumário41,840→39,800ms (vanilla4,264ms); os doze deltas
pareados foram favoráveis em ambos. Simples3,468→3,465ms, vanilla1,625ms, misto.
Meta ainda aberta. Não somar ganhos de campanhas diferentes. Os detalhes abaixo
conservam a sequência de resultados e refutações, não medições intercambiáveis.

Streammemo retém somente compressões de inputs byte a byte iguais, limitada a
1MiB/32 entradas. RED1passou/6falharam→GREEN7; infra1754/2ignorados, demais gates,
55 controles estendidos,648 comparações nominais e fontes atualizadas passaram.
Mantêm-se as108 diferenças nominais conhecidas de outline e três dívidas lexicais.
No release, compressores por edição8→1 nos100 parágrafos e18→1 no sumário;
regravação idêntica faz zero. RSS aumentou0,198/0,600MiB respectivamente.
Tamanho normalizado47265640bytes abaixo do oracle52288080, sem alegar que essa
checagem cubra toda a bateria histórica. Patch incremental de quatro caminhos,
freeze/dirty-stat/UTC/receitas em streammemo* no relatório bruto vinculado abaixo.

A auditoria de retenção confirma que watch conserva fontes e Source, mas não
produtos de parágrafos/layout entre edições. Os dois comemo::memoize produtivos
são de plugins; comentários sobre TrackedWorld não constituem implementação.
Há caches manuais locais, portanto isso não significa ausência de todo reúso.
Os contrastes ainda mostram custo maior mesmo quando todos os parágrafos mudam:
falta de retenção e custo da passagem são duas frentes. Nenhuma nova API de
layout foi proposta como aprovada, nem a meta foi marcada concluída.

A meta de igualar o vanilla em CPU ainda não foi atingida. **Somente faceproduct foi integrado, com autorização explícita do usuário.** Ele mantém uma única face Rustybuzz configurada por compilação, com identidade exata da fonte e dos eixos. Na medição do checkout integrado, os 100 parágrafos passaram de 33,600 para 31,796 ms de CPU, contra 3,184 ms do vanilla; o delta pareado mediano foi −2,189 ms, mínimo/máximo [−2,880;−1,287], favorável nos seis pares. Plain continuou misto. A campanha ocorreu em 2026-11-12T10:24:10.752516Z–10:25:08.064445Z, sobre a árvore não commitada em HEAD `48e5a2570159c6fc325babe883bac7d1c1fbd4b2`; lista exata de alterações, hashes e binário estão em `evidence/freeze-faceintegrated.json` no relatório vinculado abaixo.

A candidata reduziu alocações por compilação dos 100 parágrafos de 106228 para 95814 e tráfego de 51,46 para 43,33 MB. O pico vivo aumentou 67704 bytes; RSS também aumentou nas duas campanhas diretas. Não há alegação de redução geral de memória. Na base coldopt, um ensaio separado confirmou redução de CPU e 112→1 construções reais de face, sem reduzir as 222 requisições de raw shaping. Deltas de campanhas diferentes não são somados.

Passaram 160 testes de shaping/métricas após limpeza dos artefatos internos e recompilação do checkout, além de build release e lint estrito. A repetição anterior de testes em RAM reutilizou um executável em cache e não serve como prova independente dessa base; a auditoria e a revalidação estão registradas. Uma primeira mutação sem identidade de fonte revelou um teste insuficiente; um novo refutador direto corrigiu a lacuna, e as três mutações passaram a ser rejeitadas. No binário integrado, os 12 documentos adicionais preservaram texto, páginas e geometria; os 216 controles de PDF passaram e a troca atômica de fonte no watch publicou resultado igual ao de uma compilação nova. As demais dívidas de paridade, incluindo outline, continuam abertas.

O patch altera 17 arquivos: nova lógica e testes apenas em shaper/cache, seu L0 e os pins/selos compartilhados. Foi comprovado que se aplica diretamente ao produto original, sem depender de rootsopt, coldopt ou avaliação retida. A retenção de avaliação investigada antes continua em laboratório: beneficia imports, mas não resolveu a edição dos 100 parágrafos e aumenta retenção de memória.

**Integração concluída.** A resposta “Autorizar somente faceproduct” superou a rejeição anterior exclusivamente para esse patch. A aplicação ocorreu em 2026-11-12T10:16:19.094706Z–10:16:22.623019Z, L0 primeiro, com validação do grafo e lint estrito. A conferência dos 2499 hashes confirma exatamente os 17 caminhos aprovados como diferenças em relação ao estado recebido; as alterações preexistentes foram preservadas. As demais candidatas continuam em laboratório. Nenhum commit/push foi feito. Recibos: `evidence/face-product-approval.json`, `evidence/integrated-faceproduct.json` e `integrated-preservation.json` no diretório do relatório.

[Patch revisável](../../lab/results/construction/diagnostics/watch-residual-2026-11-12/faceproduct.patch) · [Relatório, estados exatos, hashes, UTC e limitações](../../lab/results/construction/diagnostics/watch-residual-2026-11-12/relatorio.md). Builds, testes e saídas medidas permanecem em `/dev/shm/tekt-watch-residual-20261112`.


Na retomada, a base composta de laboratório com `handlespath` mediu 22,956 ms de CPU nos 100 parágrafos, ante 24,673 ms da sua base e 2,858 ms do vanilla (repetição de seis pares; um par desfavorável). Isso não é medição do checkout integrado nem ganho atribuível apenas a faceproduct. A versão anterior de handles foi rejeitada por bloquear numa troca por FIFO; a correção O_PATH foi revalidada. Nenhuma nova candidata foi integrada.

A investigação de repetição agora identifica uma diferença estrutural: nesse fixture há uma única passagem de layout em ambas as versões, mas Tekt repete 102 preparações de trechos e 113 chamadas de shaping em cada edição; vanilla prepara/quebra apenas o parágrafo alterado e reaproveita os demais. Com bytes idênticos, vanilla não prepara parágrafos, enquanto Tekt repete o trabalho. São contagens, com granularidades distintas, não uma atribuição completa de CPU. A investigação posterior mostrou que as 102 preparações eram tentativas especulativas descartadas, eliminadas por planreject com ganho pequeno; não equivalem diretamente às preparações efetivas contadas no vanilla. Isso aponta para insuficiência de reúso entre edições, sem evidência de voltas excessivas da máquina de estados nesse caso. Estado/UTC/ELFs e dados: `phase-repetition/receipt.json` no relatório. A retenção causal segura de layout é uma hipótese relevante; os contrastes posteriores abaixo delimitam quanto ela explica.


O contraste posterior delimitou a hipótese: mudando todos os 100 parágrafos, vanilla subiu de 3,836 para 6,529 ms; handlespath ficou perto de 27 ms. Assim, falta de reúso entre edições é real, mas não explica toda a diferença. O contador também confirmou uma única passagem no ELF integrado. Layout/shaping são o maior bloco; avaliação e PDF também custam mais, com tempos instrumentados e limites registrados em `native-phases/`.

`presenceopt` não foi promovida: no ensaio original, 23,876→24,905 ms (delta pareado mediano +0,517 ms), apesar do microensaio favorável. Testes/build/lint e controles funcionais passaram. `localstyle` também ficou sem ganho consistente: delta pareado mediano −0,170 ms, extremos [−0,593;+0,921], com aumento de 201 alocações nos 100 parágrafos. Permanece sem promoção. A meta de CPU permanece aberta.

O observador aplicado diretamente aos ELFs de laboratório contou 102086 solicitações de alocação/26,70 MB nos 100 parágrafos contra 2446/1,83 MB no vanilla, em três repetições. Trata-se de tráfego, não RSS nem atribuição de CPU. Os dados, horários e hashes estão em `allocation-current/`. `framepass` passou nos testes e controles funcionais e reduziu cerca de 2,90 MB de tráfego por ciclo, mas o delta de CPU foi pequeno/misto (−0,274 ms pareado; extremos −0,923/+1,122). Não foi promovida. Nenhuma nova integração.

A amostragem direta de CPU localizou custo em validação de metadados de fontes, hashing e métricas. As pilhas apontam uma preparação fracionária que mede textos mesmo quando não há Box fracionário no escopo. `fracguard` preservou as rotas e teve ganho pequeno na campanha de 12 pares dos 100 parágrafos: delta mediano −0,957 ms, extremos [−1,575;+0,039], com 11 pares favoráveis. Uma auditoria corrigiu o delta anteriormente apresentado: o agregador usava somente os seis primeiros pares. CSV original e correção estão em `fracguard-repeat/twelve-pair-audit.json`. A base laboratorial mediu 23,018 ms, a candidata 22,347 ms e o vanilla 3,122 ms. Passaram 309 testes distintos e controles de documentos/fontes, build/lint/Clippy. O pico vivo subiu 11422 bytes. Continua somente no laboratório; a meta não foi atingida. Perfil, limitações e hashes estão em `cpu-samples/` e `cpu-stacks/` no relatório.

`witnessidx`, índice privado dos registros de fontes, não foi promovida: eliminou uma alocação por validação estável, mas piorou o ensaio original. CPU dos 100 parágrafos: fracguard 21,339 ms, witnessidx 22,093 ms, vanilla 2,860 ms; delta pareado +1,304 ms, extremos [−1,618;+5,738]. Passaram 216 testes, controles de fontes/PDF, build e lint; recibos de 2026-11-12T13:33:39.130860Z–13:36:39.375767Z em `witnessidx-comparison/`, com fontes vinculadas a `evidence/freeze-witnessidx.json`.

O contraste `font-floor2/` usa Noto Sans explícita, mesmos bytes da fonte e nenhum `--font-path`. Na base fracguard, catálogo normal/mínimo mediram 24,097/18,555 ms; vanilla 3,509/3,188 ms. Os 324 controles de texto/página/geometria passaram. Isso muda o ambiente e a rota de aquisição; não é ganho do produto nem decomposição exata do ensaio original. O primeiro contraste, `font-floor/`, usou `--font-path` e revelou outra condição: esse argumento desativa a prova rápida, elevando a descoberta completa. Esse custo adicional não foi atribuído aos 10× originais.

`plan-admission/receipt.json`, 2026-11-12T13:47:19.336817Z–13:47:21.898138Z, confirmou no ELF fracguard: a cada ciclo dos 100 parágrafos, 102 preparações e duas tentativas do planejador, ambas recusadas. Portanto, há trabalho de preparação descartado dentro da única passagem. A candidata `lineguard` tentou provar antecipadamente a recusa por linha única, mas não foi promovida: 21,350→21,742 ms nos 100 parágrafos, delta pareado +0,714 ms. Os contadores continuaram em 102 preparações. O marcador contém underscore, recusado pelo predicado Latin atual, e uma recusa estrutural tardia também invalida o planejamento inteiro. A candidata `planreject` antecipou essa recusa geral e zerou as preparações descartadas nesse cenário. Passaram 1729 testes de layout, build/lint e controles de PDF/SVG/fonte. Na repetição de 12 pares (2026-11-12T14:32:57.364461Z–14:34:37.370430Z), fracguard mediu 23,606 ms, planreject 22,450 ms e vanilla 3,334 ms; delta pareado −0,883 ms [−1,845;−0,040], favorável nos 12 pares. É um ganho pequeno e condicionado: com marcador alfanumérico, o planejador ainda prepara 101 segmentos e o delta foi misto, −0,157 ms [−2,015;+2,056]. Não explica a ordem de grandeza. Recibos e fontes em planreject-{comparison,repeat,alnum,counts-normal,counts-alnum}/ e evidence/freeze-planreject.json. O checkout continua exatamente após a integração autorizada de faceproduct.


`clusterbound` localizou uma busca quadrática de fronteiras de clusters na exportação e trocou somente essa busca por upper-bound binário. O recurso real passou de 8394752 para 57344 comparações em 4096 clusters; 673 testes e controles PDF/SVG/Unicode/fontes passaram. Porém, a repetição de 12 pares (2026-11-12T15:03:46.586818Z–15:05:27.257088Z) não confirmou ganho consistente: layout 21,022→20,892 ms, delta −0,135 [−1,536;+2,881], vanilla 2,949 ms; plain piorou no delta mediano. Sem promoção. Fonte/ELF/UTC e estado não commitado em evidence/freeze-clusterbound.json, clusterbound-repeat/ e resumed-laboratory-preservation.json no relatório. Nenhuma nova integração.

A resposta sobre a máquina de estados está documentada em state-machine-audit/: uma passagem no cenário medido, preparação especulativa descartada e escopos movidos sem cópia integral da stdlib por parágrafo. Os perfis apontam blocos de fontes, métricas/shaping e exportação; não demonstram um único responsável pelos 10×. A falta de reúso não explica toda a diferença, e as correções locais ainda não atingiram a meta.


A investigação estrutural voltou à **base integrada**, sem candidatas empilhadas.
Na repetição de 12 pares (2026-11-12T15:36:05.970355Z–15:36:27.729128Z), o harness
real mediu 30,841 ms por compilação completa e 17,803 ms ao omitir experimentalmente
a validação posterior de fontes; delta pareado −13,066 ms, todos os pares favoráveis.
Antecipar a validação preservando-a manteve o total em 30,894 ms: mover o trabalho
não o elimina. A ablação não é candidata segura. A prova isolada do catálogo
conta 55 configurações, 1345 stamps de raízes/links/candidatos e 1112 de fontes;
reparse e metadados custam cerca de 14 ms. O cenário usa Libertinus embutida,
mas as consultas atuais exigem validar o catálogo mutável inteiro.

Mesmo alterando os 100 parágrafos, o ELF integrado pede 119134 alocações/realocações
e 54,36 MB, contra 8813 e 5,62 MB no vanilla. Em ciclos com bytes idênticos repete
113 shapings/planos e reconstrói as cinco páginas e o subset de fonte, com uma
única passagem de layout. O PDF real tem 1904 blocos de texto contra 101;
TextSeparator interrompe o agrupador de TextShaped. Quatro das cinco páginas e
todos os recursos/fontes permanecem idênticos entre edições. São evidências de
dependências amplas, trabalho repetido e representação fragmentada; não somas
de CPU nem autorização para enfraquecer fontes/ActualText/semântica.

Fontes/ELFs/UTC/lista exata da árvore não commitada em HEAD
`48e5a2570159c6fc325babe883bac7d1c1fbd4b2` estão no bloco estrutural do relatório,
`structural/{analysis,checkout-verification}.json`, `repeat2/receipt.json` e no
freeze integrado. Os 2499 hashes produtivos/L0/config permanecem preservados.
Somente faceproduct está integrado. A meta de vanilla continua aberta.

A próxima candidata proposta adiciona consultas seletivas `World::select_font`
e `World::font_info`, com defaults compatíveis, mantendo book completo e fontes
mutáveis frescas quando necessárias. O pacote L0 revisável e os pins estão em
`structural/font-demand-proposal/` no relatório; V15/V26 passaram. Implementação
aguarda o gate de contrato público da ADR-0127, exclusivamente para laboratório.

Há uma diferença de obrigação relevante na comparação: no controle já registrado
em `face-product-font-control/vanilla/receipt.json`, a substituição atômica da
fonte mudou a compilação nova, mas o watch vanilla manteve a geometria antiga.
O seu reset não reconstrói o FontStore. O Tekt preserva a atualização de fontes;
a ablação de 13 ms não autoriza copiar essa retenção obsoleta. A consulta
seletiva pretende evitar a dependência do catálogo mutável somente quando ela
comprovadamente não pode afetar a fonte escolhida.

O agrupamento experimental de PDF `separatorruns` permanece em laboratório,
com alteração privada de runs.rs e seu L0. Passaram 674 testes de exportação,
build/lint/Clippy e os controles de texto/posições/anotações/SVG. Entretanto,
dois documentos diferiram na comparação exata de raster: 136 e 92 pixels a
144 dpi. Os clocks dessa candidata são exploratórios; ela não está validada
para integração. O relatório preserva as falhas e o refutador de sobreposição
com fonte sintética. Nenhuma nova alteração produtiva foi aplicada.

Na repetição de doze pares, 2026-11-12T18:53:28.616014Z–18:55:12.195504Z,
separatorruns mediu 31,432 ms contra 32,901 ms da base e 3,413 ms do vanilla;
delta pareado −1,324 ms [−2,285;+0,380]. Plain foi misto, mediana +0,266 ms.
O PDF caiu de 1904 para 102 blocos de texto, ante 101 do vanilla, mas conservou
1904 matrizes de posição e comandos separados por folha. O ganho total é
limitado. Um segundo renderizador produziu imagens idênticas nos dois casos
problemáticos; a diferença original em Poppler permanece registrada e aberta.
Sem promoção. Fontes/ELFs/lista exata da árvore não commitada estão em
evidence/freeze-separatorruns.json, separatorruns-repeat/freeze.json e no recibo
de preservação do relatório. O maior alvo medido continua sendo a dependência
desnecessária do catálogo inteiro; a proposta pública segue sem implementação.

A prova posterior de seleção (`structural/selection-proof/`,
2026-11-12T19:00:41.232633Z–19:00:47.580897Z) confirmou metadata/bytes/índices
do prefixo embutido de dez fontes. Das 2142 consultas de diagnóstico, 60 tinham
pontuação mínima no prefixo e todas coincidiram com o catálogo completo.
Contraexemplos refutam usar somente nome da família ou ausência no prefixo.
A auditoria também mostrou que acelerar só a seleção não basta: buscar bytes,
preparar fallbacks e resolver recursos do PDF ainda dispara o catálogo inteiro.
Esses caminhos devem ser tratados juntos, preservando o fallback fresco.
É evidência para a candidata, sem novo ganho de CPU, mudança pública ou
integração; o pacote aguardando o gate permanece com o mesmo hash.


Atualização após autorização explícita de 2026-11-12: o gate dos dois métodos
`World::select_font` e `World::font_info` foi satisfeito exclusivamente para o
laboratório. Recibo: `fontdemand-validation/approval.json`; pacote aprovado
SHA-256 `281b8289424b9c24fe61b8890cd30a12e8d9331165afc4b81590a6e80e85cc30`.
A candidata `fontdemand` parte somente de faceproduct integrado; nenhuma outra
candidata foi empilhada ou integrada. O estado exato está em
`fontdemand-preservation.json` e `fontdemand.patch` no diretório do relatório.

O teste até o PDF inicialmente continuou adquirindo o catálogo: a pilha em
`fontdemand-validation/trace.log` localizou `eval_set_text`, que enumerava todas
as famílias para validar um nome já conhecido. A consulta de existência agora
usa a seleção autorizada, preservando avisos e a fase de avaliação. O teste
passou em 2026-11-12T19:26:43.797481Z–19:27:35.955021Z, vinculado ao estado
`consumer-green-source.json`; casos que não têm prova no prefixo continuam
exigindo catálogo completo. Validação ampliada e desempenho ainda pendentes:
a ablação histórica de catálogo não é um ganho medido desta candidata.


A v1 foi medida e preservada: doze pares confirmaram CPU 18,688→3,300 ms
no simples e 33,603→22,761 ms nos 100 parágrafos; vanilla 1,451/3,469 ms.
`fontdemand-repeat/freeze.json`, summary.csv e paired.csv preservam intervalo,
ELFs e agregação. Apesar de controles funcionais e de atualização de fontes
aprovados, a v1 não é promovível: fontes do sistema e variantes não mínimas
regrediram por seleção repetida sem reutilização própria. O RED da v2
confirmou nove buscas para nove consultas idênticas; a correção privada
passou em quatro testes novos e na suíte de infraestrutura. Desempenho v2
ainda pendente, sem integração.

O perfil mostrou outra causa estrutural: outline faz seis construções de
layout, e seu bloco de expansão/contexto domina o custo. Cada tentativa
executa discovery e replay físicos; esse replay é uma obrigação semântica
explícita do L0 atual. A investigação continua com perfil de pilhas para
separar validação necessária de trabalho reconstruído sem necessidade.


Atualização v2 concluída: 12 pares em 20:07:44.185693Z–20:09:22.224098Z
confirmaram 18,511→3,302 ms no simples e 33,683→21,870 ms nos 100 parágrafos,
contra vanilla 1,491/3,456 ms. Ainda ~2,2×/~6,3×: meta não atingida.
Todos os gates e 47 controles estendidos passaram, incluindo raster e fontes
tardias/substituídas. As grandes regressões v1 foram corrigidas; fonte do
sistema ainda teve delta pareado mediano +0,872 ms em seis pares.
Perfil posterior aponta ~21% das amostras diretas de outline em lorem, que
reconstrói treinamento constante por chamada, além de remontagem de vetores
em projeção de posições e resolução de links. Candidatas internas em
investigação somente no laboratório; replay físico obrigatório preservado.
Fontes/UTC/ELF/receita e lista exata estão em evidence/freeze-fontdemand-v2.json,
fontdemand-v2-preservation.json e fontdemand-v2-cpu-stacks/freeze.json.

Atualização estrutural: loremlocal concluiu os gates e 55 controles estendidos.
Doze pares de 20:33:53.309566Z a 20:37:07.744709Z confirmaram melhora de outline:
v2 158,898 ms → 120,364 ms; vanilla 3,704 ms. Delta pareado mediano −29,132 ms,
favorável nos doze pares. Sem ganho geral demonstrado nos casos sem lorem.
Proveniência em `evidence/freeze-loremlocal.json`, `loremlocal-preservation.json`
e `loremlocal-repeat/` no diagnóstico bruto; não combinar campanhas distintas.

Foi localizado um defeito release: `link_capture.rs` encerra escopos com
`active.pop()` dentro de `debug_assert_eq!`. O release elimina também o pop;
links do sumário vazam para páginas de corpo. O censo vinculado a PDFs/ELFs
mostra 5258 anotações na primeira página e 60 por página de corpo, contra
12/0 no vanilla. A correção laboratorial `linkscope` teve RED real com debug
assertions desativadas (dois testes de fechamento falhos; três controles
aprovados). Validação ampliada e medição estão em andamento. Ainda não há
prova de paridade de desempenho nem integração das novas candidatas.

Linkscope concluído somente no laboratório. Doze pares de
20:57:01.352771Z–20:59:43.876974Z mediram sumário 90,987→56,129 ms;
vanilla 3,257 ms. Delta pareado −35,078 ms [−39,572;−32,949], todos favoráveis.
Sem melhora demonstrada no simples/100 parágrafos. Gates completos passaram
(core 8414/5 ignorados; infra 1739/2 ignorados); 55 controles de pintura/texto/
geometria e 648 comparações nominais preservaram as diferenças já registradas.

O PDF release passou de 5258 para 12 links na primeira página do sumário e
eliminou os links indevidos no corpo. A comparação lexical estrita revelou três
dívidas preexistentes (caixa, aninhamento em caixa, rodapé), conservadas exatamente
pela candidata e ainda abertas. Seis demais controles e ausência de BODY clicável
passaram; não se declara paridade lexical completa. `linkscope-lexical/audit.json`
retém essa distinção; freeze, patch, checks e dirty-stat estão no diagnóstico bruto.
O experimento controlprimary está iniciando seu RED, sem integração.

Controlprimary teve RED real (duas falhas esperadas), mas o primeiro GREEN foi
parcial: três passaram e o sumário ainda pediu catálogo. A pilha localizou uma
segunda medição em link_bbox via advance. Cinco probes bilaterais confirmaram
áreas clicáveis que ignoram kerning/tracking e atribuem largura ao controle
invisível. O L0 do link foi atualizado antes de dois testes RED, e a candidata
passou a consumir text_width, contrato já existente. Validação final em andamento;
nenhum ganho novo é alegado. Provas e fontes em controlprimary-validation/,
controlprimary-catalog-trace/, controlprimary-linkwidth-before/ e
controlprimary-linkwidth-validation/ no diagnóstico bruto. Nada foi integrado.


Controlprimary final concluiu gates e controles, somente no laboratório.
Doze pares de 21:19:47.295977Z–21:22:18.599364Z: sumário 58,532→44,334ms,
vanilla3,969ms; delta pareado −14,446ms [−21,428;−12,244], todos favoráveis.
Sem melhora consistente nos 100 parágrafos (21,778ms versus vanilla3,250ms).
O sumário agora dispensa o catálogo integral e a largura clicável usa a medição
já autoritativa. Core8416/5ignorados, infra1743/2ignorados, demais gates e55
controles estendidos passaram, incluindo fontes tardias/substituídas.
Cinco probes horizontais coincidem com o oracle na precisão de serialização;
as três dívidas lexicais anteriores seguem abertas. Nas campanhas de doze
pares há108 diferenças conhecidas de outline em648 comparações, corrigindo
a contagem textual54 atribuída anteriormente a linkscope; CSV preservado.
Freeze/patch/dirty-stat/horários no diagnóstico bruto sob controlprimary*.
Meta ainda aberta; perfil e contraste de invalidação em execução.


Contraste atual de invalidação concluído: alterando todos os100 corpos,
controlprimary25,176ms versus vanilla7,890ms; apenas marcador25,564/4,763ms.
216 controles iguais. Receita diagnóstica3 em quatro crates reduziu os100
parágrafos23,809→20,308ms na campanha própria, mas excedeu o tamanho normalizado
do oracle (54.392.544>52.288.080bytes); rejeitada como receita, sem integração.

Censo release identificou2230 análises bidi/compilação, metade na prova de seleção
e metade nos consumidores, além de repetição entre ocorrências iguais. A candidata
interna primaryproof passou RED(1aprovado/3falhos)→GREEN4, core e infra1747/2ignorados.
Retém só prova positiva/partição dentro da FaceCache, com chave exata e fonte
retida, preservando guardas/cmap/fonte atual e cache nova vazia. Três L0s+nucleo
precederam código. Desempenho e controles release ainda pendentes; receita s
mantida, sem integração. Evidências primaryproof* no diagnóstico bruto.

Primaryproof final concluiu validações: representação de direção corrigida após
V16, infra1747/2ignorados novamente, core8416/5ignorados reutilizado com prova de
identidade, demais gates completos. Seis mutantes de perda de identidade foram
refutados;55 controles estendidos,648 comparações nominais, fontes tardias/
substituídas e controles lexicais passaram com as dívidas anteriores explícitas.
Doze pares22:02:33.374587Z–22:05:01.546958Z:100 parágrafos21,896→20,398ms,
vanilla3,372ms; delta pareado−1,346ms [−4,254;+1,249], sem ganho uniforme.
Sumário44,638→43,261ms, vanilla4,020ms. Meta ainda aberta, nada integrado.

Censo release reduziu bidi2230→224, split2230→1339 e pedidos raw1339→448 por
compilação, mas4209 clones de estilo e1307 construções de família permanecem.
Nova decomposição decorrida localiza layout7,989/shape3,027/render4,418/eval3,871ms
na carga original; não são parcelas exatas de CPU. Streams decodificados mostram
8/10 iguais entre edições dos100 parágrafos. A próxima hipótese, streammemo,
retém só compressões puras de bytes exatos sob limites estritos, no laboratório.
Os recibos primaryproof* preservam fontes/horários/observadores e a lista exata
do checkout; primaryproof-only.patch tem sete caminhos sobre controlprimary.
