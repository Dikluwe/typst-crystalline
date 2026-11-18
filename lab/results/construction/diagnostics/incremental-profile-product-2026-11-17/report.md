# Próximo alvo após os ganhos integrados

Data local: 17/11/2026, America/Sao_Paulo. Recibos guardam UTC. Diagnóstico do pacote já aplicado, sem alterar código produtivo/L0, integrar outra otimização ou fazer commits.

## Evidência atual

Árvore de trabalho sem commit sobre `48e5a2570159c6fc325babe883bac7d1c1fbd4b2`; os 2.560 hashes de fonte coincidem com o estado qualificado da integração anterior. Produto instalado SHA-256 `61be8cf9e6b32bb4df09e2efbd98f4c0b3ce4c6aa6d8109b6b5384bee614cbb4`. Observador separado SHA-256 `0bded9e34c0bdc0484083cb2b437112e8acac7373eb01feed2ca93e23b61fe29`, com as duas diferenças de marcação já qualificadas na comparação anterior; nunca instalado como produto. [initial.json](/repos/Antigravity/typst-crystalline/lab/results/construction/diagnostics/incremental-profile-product-2026-11-17/initial.json) · [final.json](/repos/Antigravity/typst-crystalline/lab/results/construction/diagnostics/incremental-profile-product-2026-11-17/final.json) · [watch-frozen.json](/repos/Antigravity/typst-crystalline/lab/results/construction/diagnostics/gains-product-vanilla-2026-11-18/watch-frozen.json). Antes de decidir, foram relidos os L0 dos owners: [owner-audit.json](/repos/Antigravity/typst-crystalline/lab/results/construction/diagnostics/incremental-profile-product-2026-11-17/owner-audit.json).

CPU nominal: seis blocos, dois caminhos para o mesmo ELF byte idêntico, cinco entradas e três estados (início, comentário e cauda visível), total de 180 ciclos. Sem sampler nem JSON de fases no nominal. CPU END→END inclui detecção, reset, compile, exportação, drop e eviction; startup separado. Não é uma comparação nova de ganho nem de vanilla. Valores de outra rodada ou outro aquecimento não são evidência de regressão de código.

| Caso | Comentário, CPU ms | Detecção, CPU ms | HWM mediano MiB | A/A pareado Δ% |
|---|---:|---:|---:|---:|
| references_500 | 372.84 | 0.087 | 69.9 | -1.76 |
| outline_100 | 43.21 | 0.085 | 33.3 | -2.04 |
| mixed_report | 157.11 | 0.081 | 46.5 | +2.59 |
| math_supported | 37.01 | 0.069 | 34.8 | +0.28 |
| unicode_repo | 88.04 | 0.113 | 110.6 | +1.42 |

`references_500`: 20 seções/referências e 500 parágrafos. `outline_100`: um heading e 100 parágrafos, não 100 entradas de sumário.

Perfil separado: 35,968 amostras, duas frequências (997.000 e 1.237.000 ns), dez coletas, pelo menos quatro segundos de CPU e dez ciclos por caso/frequência. Sampler da thread principal ligado antes de liberar o END anterior e desligado após o próximo END; inclui detecção e descarte. Cobertura thread/processo aproximadamente 100%; os valores brutos têm excesso numérico de até 0.00058% pela ordem de leitura dos relógios. Até 1.59% das pilhas truncadas; zero PCs próprios desconhecidos no topo. PCs de libc são mantidos como biblioteca e atribuídos apenas ao chamador observável, sem supor que todo custo é alocação.

As faixas abaixo são os dois resultados observados, não intervalos de confiança. Grupos inclusivos podem se sobrepor e não devem ser somados. As etapas do gráfico são exclusivas por atribuição ao primeiro owner relevante na pilha. Fração de custo não é ganho recuperável prometido.

![Perfil do fluxo atual](/repos/Antigravity/typst-crystalline/lab/results/construction/diagnostics/incremental-profile-product-2026-11-17/profile.png)

| Caso | Layout físico | Indexação semântica | Shaping+Bidi final | Exportação PDF |
|---|---:|---:|---:|---:|
| references_500 | 65.50–65.71% | 9.78–10.05% | 1.74–1.75% | 8.04–8.15% |
| outline_100 | 70.57–71.61% | 7.84–8.34% | 2.36–2.38% | 8.41–8.62% |
| mixed_report | 73.98–75.04% | 9.85–10.94% | 2.04–2.11% | 3.68–3.98% |
| math_supported | 73.25–73.35% | 4.07–4.46% | 4.46–4.63% | 5.03–5.14% |
| unicode_repo | 42.40–43.03% | 0.03–0.10% | 10.03–10.05% | 41.56–41.85% |

Em referências, a passagem pelo corpo (`layout_content_retaining_plan`) ocupa 47,53–47,60% das amostras; texto 14,97–15,16%; fechamento de linha 10,03–10,11%. São medidas inclusivas, sobrepostas. No sumário, corpo 59,36–60,18%, texto 22,06–22,11% e fechamento de linha 11,98–13,04%.

Em referências, os passes finais de links ocupam 6,49–6,66% e posições 4,92–4,94%. A construção do SourcePlan aparece em 0,77–0,80%, e child em cerca de 0,41%. No sumário, construção 1,08–1,15% e child 0,49–0,70%. Não há sinal amostral que justifique escolher novamente essas consultas como o maior alvo; funções inline e erro amostral limitam essa conclusão. O shaping final não representa todo custo de métricas/shaping executado durante layout físico.

[summary.json](/repos/Antigravity/typst-crystalline/lab/results/construction/diagnostics/incremental-profile-product-2026-11-17/summary.json) · [decoded.json](/repos/Antigravity/typst-crystalline/lab/results/construction/diagnostics/incremental-profile-product-2026-11-17/decoded.json) · [profile.json](/repos/Antigravity/typst-crystalline/lab/results/construction/diagnostics/incremental-profile-product-2026-11-17/profile.json) · [phases.json](/repos/Antigravity/typst-crystalline/lab/results/construction/diagnostics/incremental-profile-product-2026-11-17/phases.json) (fases em elapsed, com instrumentação separada, não CPU exclusiva).

## Verificação do cache de parágrafos

O cache já existe. O L0 contém evidência histórica de recusas e também a correção de uma conclusão antiga demasiado ampla: alguns try_layout já eram aceitos. Portanto ausência de símbolos no topo das amostras não provaria um cache quebrado. Usamos a API pública existente ParagraphSession.stats através de um executável auxiliar ligado às rlibs atuais, mantendo SourceSession, FontCatalogSession e PdfCompileSession; os valores por ciclo são descartados antes de comemo::evict(10). Nenhum contador foi acrescentado ao produto.

| Caso nominal | Compilações | Hits acumulados | Misses acumulados | Entradas finais |
|---|---:|---:|---:|---:|
| references_500 | 4 | 0 | 0 | 0 |
| outline_100 | 4 | 0 | 0 | 0 |
| mixed_report | 4 | 0 | 0 | 0 |
| math_supported | 4 | 0 | 0 | 0 |
| unicode_repo | 4 | 0 | 0 | 0 |

Nos cinco inputs há zero lookup contabilizado e zero produto armazenado, inclusive após três comentários. Isso localiza uma não participação do memo nesses casos, não um problema de capacidade ou prova de que o cache nunca é chamado em outros documentos. As recusas anteriores a Request::lookup não incrementam misses. O motivo exato ainda pode ser a seleção do caminho, a estrutura admitida, a identidade de métricas ou outro guard; não foi isolado nesta rodada. Matemática e Unicode podem legitimamente ficar fora do subconjunto permitido.

Controles positivos independentes com texto latino estático confirmam que o contador funciona e que o memo recebe hits:

| Controle | Hits finais | Misses finais | Entradas finais |
|---|---:|---:|---:|
| static_long_text | 2 | 2 | 2 |
| explicit_simple | 2 | 2 | 2 |
| explicit_par | 3 | 1 | 1 |
| ten_static_paragraphs | 37 | 3 | 3 |
| optimized_static | 2 | 2 | 2 |

Os nomes de slots fixos do executável auxiliar não identificam os casos nominais nos controles positivos: a correspondência e os conteúdos exatos estão registrados. O auxiliar foi relinkado com LTO-O0 para coletar contagens; seus tempos não são comparáveis aos tempos do produto. Os PDFs foram conferidos contra o produto release real.

[paragraph-census.json](/repos/Antigravity/typst-crystalline/lab/results/construction/diagnostics/incremental-profile-product-2026-11-17/paragraph-census.json) · [paragraph-positive.json](/repos/Antigravity/typst-crystalline/lab/results/construction/diagnostics/incremental-profile-product-2026-11-17/paragraph-positive.json) · [census-build.json](/repos/Antigravity/typst-crystalline/lab/results/construction/diagnostics/incremental-profile-product-2026-11-17/census-build.json) · [census-verification.json](/repos/Antigravity/typst-crystalline/lab/results/construction/diagnostics/incremental-profile-product-2026-11-17/census-verification.json).

Âncoras do código atual: [lookup e contadores](/repos/Antigravity/typst-crystalline/01_core/src/compiler/layout/paragraph_session.rs:146) · [certificação do request](/repos/Antigravity/typst-crystalline/01_core/src/compiler/layout/paragraph_session.rs:485) · [admissão e emissão por partes](/repos/Antigravity/typst-crystalline/01_core/src/compiler/layout/paragraph_plan.rs:201) · [subconjunto estrutural](/repos/Antigravity/typst-crystalline/01_core/src/compiler/layout/paragraph_prepare.rs:211) · [entrada da sequência](/repos/Antigravity/typst-crystalline/01_core/src/compiler/layout/sequence.rs:73) · [entrada do texto](/repos/Antigravity/typst-crystalline/01_core/src/compiler/layout/text.rs:84) · [fechamento da linha](/repos/Antigravity/typst-crystalline/01_core/src/compiler/layout/line_commit.rs:416).

## Decisão e próximo experimento

**Prioridade: tornar efetivo o reúso já existente de produtos textuais no caminho físico de referências e sumário, começando por isolar a recusa anterior ao memo.** O alvo é o corpo textual e seus parágrafos, não mais um cache genérico de SourcePlan. A hipótese de benefício deriva da concentração de CPU no corpo e do censo vazio, com um controle positivo que gera hits. Não há prova de que toda essa fração seja repetição desnecessária, nem promessa de 65–75% de ganho.

Experimento delimitado: L0 primeiro nos owners correspondentes; contabilizar entrada, recusa estrutural, opções de layout, prova de métricas, lookup e emissão em try_parts/try_layout/request. Identificar um subconjunto textual real destes fixtures que conserve origem/SourcePlan, estilos, largura, fontes, efeitos e invalidação. Se houver uma recusa corrigível sem alargar os contratos de linguagem, testar o reúso desse subconjunto contra o caminho frio. Caso o trabalho seja necessário ou as recusas protejam efeitos/posição, rejeitar a hipótese em vez de limpar os guards. Nenhum novo algoritmo ou alteração de L0 foi aplicado nesta rodada.

Critérios que refutam a hipótese: não obter consultas/hits nos parágrafos alvo; o ganho desaparecer no ciclo completo; não reduzir medições/produção de produtos; apenas deslocar custo; divergência em origem, estilos, query/counter/state, callbacks, paginação, links, erros ou invalidação; ou crescimento de retenção não justificado. Aumentar limites do cache com zero entradas não é um experimento fundamentado.

## Frentes específicas posteriores

Matemática: 38.67–38.71% inclusivo em validação/configuração da geração de fontes; só scan_stamp representa 22.29–22.55%. Isso é observado na primeira aquisição do catálogo completo em cada World: font_catalog usa OnceLock, portanto não se conclui revalidação a cada consulta. O L0 exige essa prova na fronteira de demanda. Eventual redução de custo precisa manter a descoberta de adição/remoção/substituição/symlink/configuração; simplesmente suprimir validação não é autorizado. Essa frente não explica referências, onde o perfil não capturou esse custo.

Unicode: exportação 41.56–41.85%, processamento PNG de bitmap 18.69–18.85% e compressão zlib 15.82–17.07%, grupos sobrepostos. Já há um cache de compressão de 32 entradas/1MiB; seria necessário medir bypass/miss/eviction e o conjunto de dados antes de alterar limites ou introduzir outro cache. Cobertura/fallback é outra parcela relevante, sem inferir que todo custo pode ser removido.

## Validação, RAM e preservação

195 pares de PDFs das medições/perfis/fases e 40 do censo e controles positivos passaram contra compilações frescas do ELF produtivo não instrumentado. Comparam caracteres, estilo/posição por glifo, geometria, links, bookmarks, presença de tagging e mapas completos de destinos. Perfis/fases/censo passaram também por rasterização72dpi. Preservar o produto não certifica paridade global com vanilla; as dívidas já documentadas na integração anterior permanecem. [verification.json](/repos/Antigravity/typst-crystalline/lab/results/construction/diagnostics/incremental-profile-product-2026-11-17/verification.json) · [census-verification.json](/repos/Antigravity/typst-crystalline/lab/results/construction/diagnostics/incremental-profile-product-2026-11-17/census-verification.json).

Execução sequencial, CPU1 compilador/CPU2 controlador, AS1GiB por processo de execução/análise, pelo menos6GiB disponíveis antes dos lançamentos. O maior HWM nominal foi 120.4MiB e no sampler 122.8MiB. O auxiliar diagnóstico foi compilado com limite AS2GiB, um CPU/um thread de linker, pico de filho 1179.0MiB. As tentativas anteriores de link tiveram criação de threads recusada/timeout e um erro corrigido no próprio auxiliar; os recibos estão preservados e não são regressões do produto. Não houve nova compilação produtiva nem execução de testes unitários, pois L0 e código produtivo permaneceram idênticos ao build já qualificado.

[measurement-plan-before-census.json](/repos/Antigravity/typst-crystalline/lab/results/construction/diagnostics/incremental-profile-product-2026-11-17/measurement-plan-before-census.json) preserva o plano original vinculado pelo hash do nominal; [measurement-plan.json](/repos/Antigravity/typst-crystalline/lab/results/construction/diagnostics/incremental-profile-product-2026-11-17/measurement-plan.json) contém o adendo datado antes do censo. [initial.json](/repos/Antigravity/typst-crystalline/lab/results/construction/diagnostics/incremental-profile-product-2026-11-17/initial.json) e [final.json](/repos/Antigravity/typst-crystalline/lab/results/construction/diagnostics/incremental-profile-product-2026-11-17/final.json) guardam HEAD, diffstat, status e hashes exatos. Todos os dados são de árvore de trabalho sem commit. Nenhum staging, commit ou alteração ao executável instalado.
