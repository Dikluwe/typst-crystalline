# Estudo de reuso confiável: fontes, World e shaping

A fronteira segura a investigar é **conservar o estado de fontes e renovar o snapshot da compilação**. O Tekt já reutiliza catálogo/slots dentro de um World; watch descarta esse World entre ciclos. Porém conservar o World inteiro sem invalidar sources/arquivos seria incorreto. Este estudo examina ambos os lados: retrabalho e risco de resultado obsoleto. Somente diagnósticos foram adicionados; nenhuma otimização ou alteração de contrato foi implementada.

## Contratos consultados e evidência de ciclo de vida

Foram lidos os owners vigentes [system-world/fonts.md](../prompts/infra/system-world/fonts.md), [system-world/cache.md](../prompts/infra/system-world/cache.md), [infra/fonts.md](../prompts/infra/fonts.md), [shaper/cache.md](../prompts/infra/shaper/cache.md), [shaper/traversal.md](../prompts/infra/shaper/traversal.md), [shaper/shaping.md](../prompts/infra/shaper/shaping.md), [font_metrics/caches.md](../prompts/infra/font_metrics/caches.md), [pipeline/paged.md](../prompts/infra/pipeline/paged.md) e [wiring/watch.md](../prompts/wiring/watch.md). Seus hashes de arquivo constam no fingerprint, sem resselo. As decisões de paridade/mecânica, medição/proveniência e gates de contrato seguem ADR-0107/0108/0121/0127; a relação owner/consumer segue ADR-0129 e as referências ADR-0130.

- `04_wiring/src/watch.rs:63` executa run_compile_observed por ciclo; `04_wiring/src/compile.rs:246` cria SystemWorld nesse fluxo. O loader lazy do catálogo (`03_infra/src/world/fonts.rs:104`) inicializa uma vez por World, não uma vez por sessão watch.
- `03_infra/src/world/cache.rs:40` mantém o primeiro Source resultado, inclusive erro, em OnceLock. O owner especifica snapshot por World. Não há reset público desse cache no World atual. Ler novamente o mesmo World não significa reler o disco.
- `03_infra/src/fonts.rs:153` mantém Font/None em OnceLock; TTC é reconstruída uma vez por slot carregado. Outro World com slots novos pode repetir o trabalho. Retenção deve respeitar a identidade dos bytes/face, não apenas o nome do arquivo.
- `03_infra/src/world/fonts.rs:98` substitui catálogo e limpa cobertura vinculada aos índices anteriores. O owner também preserva coverage de sistema calculada em fontdb; não autoriza mapear todas as fontes para recomputá-la (P1410).
- `03_infra/src/shaper/traversal.rs:17` cria ShapeCache e FaceCache por documento; `shaper/shaping.rs:234` cria Face rustybuzz no miss de resultado. Repetir texto idêntico pode usar o cache de resultado; textos diferentes ainda podem compartilhar fonte e plano, sem compartilhar glifos.
- `03_infra/src/font_metrics.rs:68` cria caches próprios de face/largura. Clones compartilham os Arcs; outra chamada new cria outra instância. `pipeline/paged.rs:21`, `:63`, `:469`, `:562` mostram instâncias em fases/callbacks diferentes. Isso identifica limites de partilha, sem afirmar que todos esses caminhos são executados numa mesma compilação ou que seus misses dominam o tempo.

## Controles executados e resultados

Sonda diagnóstica chamando **APIs públicas produtivas existentes**, sem ampliar visibilidade. Todas as asserts passaram. O diretório [reuse-trust-study/](reuse-trust-study/) contém o crate, raw results, comandos, hashes e protocolo de reprodução. Os arquivos temporários ficaram nele; main.typ foi editado e restaurado, e uma fonte transitória criada/removida. Não se mudou uma fixture produtiva nem os inputs anteriores.

| Intervenção | Resultado observado | Implicação limitada |
| --- | --- | --- |
| Editar main.typ depois de carregar Source | World retido: `First snapshot`; World fresco: `Edited snapshot` | Não reutilizar esse World inteiro entre edições sem invalidar arquivos; o watch atual evita isso criando outro World |
| FontSlot inicialmente ausente; criar arquivo válido depois | Mesmo slot continua None; slot novo carrega | Cache de falha precisa de renovação quando sua dependência reaparece |
| Repetir get da mesma TTC no mesmo World | Handles compartilham endereço de buffer com ambos vivos | O slot já evita repetir reconstrução dentro desse World |
| Carregar a mesma TTC num World novo | Buffers simultaneamente vivos distintos e bytes iguais, tamanho 16.467.712 | Reuso não atravessa Worlds independentes; controle explícito de slots, não medição de discovery/CLI |
| Popular coverage CJK e substituir slot zero por Libertinus | Candidato para `中` muda de `[0]` para `[]`, igual ao catálogo fresco | A invalidação por builder funciona neste caso, mesmo reutilizando o índice zero |
| Shaping normal/math isolado e em documento misto, duas ordens | `ffi`, `office`, `ab`, `Hello`, `123`, `sin`: ambos os modos frescos coincidem e os resultados mistos coincidem com eles | Não reproduziu erro de alias nesse conjunto; como os modos frescos nem diferiram, a amostra não discrimina o risco de script na chave |
| Encoding de eixos f32 distintos | Bits 1137180672 e 1137180673 viram `400.0000` | Colisão representacional demonstrada; impacto de glifos/normalização não demonstrado |

[results.json](reuse-trust-study/results.json) conserva os glifos/advances/clusters de todas as configurações e controles. O SHA-256 da sfnt CJK adquirida é `787b85a3c6a5ce274ffde92ff64f47eea34989af7474f883e334dc0341f0aad6`, compatível com o resultado do dataset anterior. Identidade de buffer é controle de ownership, não prova de paridade nem contagem exata de alocações/tempo.

Os casos de falha/source retido são consequências dos contratos atuais de snapshot/OnceLock, não defeitos do watch fresco. Eles refutam a solução ingênua de conservar tudo indefinidamente. A substituição de catálogo passou, mas não cobre adição/remoção concorrente, geração compartilhada entre threads, arquivos substituídos ou persistência em disco.

## Matriz de ownership e invalidação a validar

Esta matriz é **proposta de estudo**, não novo contrato L0 nem autorização de código. Uma mudança materializada precisa atualizar os owners afetados primeiro, respeitando os gates ADR-0127. L3 possui identidade/I/O/invalidação; L4 somente compõe componentes e preserva observáveis/publicação. L1 não ganha I/O, cache global ou dependência de lab.

| Estado | Owner atual | Dependências relevantes para reuso | O que o invalida | O que não deveria invalidá-lo sozinho |
| --- | --- | --- | --- | --- |
| Universo/catálogo de fontes | infra/system-world/fonts; discovery em infra/fontdb e infra/fonts | Ordem/prioridade de providers, paths/raiz, fontes disponíveis, bytes/índice, embedded snapshot e algoritmo de metadata | Fonte adicionada/removida/substituída; mudança de configuração/universo | Edição de texto do documento sem mudar universo |
| Bytes originais/FontSlot | infra/fonts | Identidade de arquivo/bytes e índice de face | Bytes/face trocados, reaparência após falha, nova geração | Nova posição/layout do texto |
| Sfnt derivada da TTC | infra/fonts; consumers de export dependem de índice zero | Bytes originais, índice, versão da transformação | Mudança de qualquer dependência | Recompilação usando a mesma face |
| Coverage | infra/fontdb; lazy em infra/system-world/fonts | Bytes e face; geração do catálogo para índices | Troca de fonte ou reindexação | Consulta de outro caractere |
| Face/instância/planos | infra/shaper/cache e shaping; font_metrics/caches | Fonte estável, eixos efetivos; para plano: direção, script, língua e features efetivos | Qualquer dependência que altere resultado/planos | Outro texto que tenha a mesma configuração de plano |
| Resultado de shaping | infra/shaper/cache | Texto, identidade da fonte/face, configuração efetiva completa | Texto/configuração/geração modificados | Posição de pintura se resultado permanece em unidades de fonte |
| Sources e bytes de assets | infra/system-world/cache | Snapshot por arquivo/compilação, caminhos e dependências inclusas/ausentes | Edição, criação, remoção/substituição de dependência | Nada que justifique conservar bytes velhos após evento relevante |
| Contexto, callback stores e candidatos de layout | infra/pipeline/paged e filhos | Source/inputs/features/contexto da compilação e transcripts | Nova compilação/relayout conforme contrato | Não generalizar reuso de fontes para esses stores |

Um índice de slot só é identidade dentro de um catálogo congelado. Compartilhar cache por `slot=0` entre dois catálogos pode fazer glifos/coverage da fonte anterior valerem para outra. Uma identidade de fonte estável deve distinguir bytes e face, e seleção que depende do universo precisa da geração do catálogo. Path/mtime/tamanho isolados não demonstram identidade de bytes; hard links, substituição e mesmo tamanho precisam de controles.

É necessário separar invariantes de reuso e consistência: um compile deve enxergar pares book/slots de uma geração coerente; uma invalidação concorrente não deve misturar book antigo com font novo. Handles antigos podem continuar vivos para um resultado anterior, sem serem apresentados como parte da geração nova. Erros cacheados também precisam de política de recuperação, não apenas o caso feliz.

## Chaves: correção antes de duração maior

O owner shaper/cache especifica chaves textuais com eixos/tracking formatados a quatro casas e caches locais. `03_infra/src/shaper/cache.rs:34` implementa essa representação. Valores f32 distintos podem ter a mesma representação; a sonda demonstra essa colisão de encoding, mas não demonstra por si um erro de glifo para uma fonte variável. Normalização OpenType pode coincidir; um teste com fonte/eixos reais deve decidir. Não se alterou a precisão exigida pelo L0.

Há outro risco verificável: a chave de resultado contém ssty, mas não declara diretamente o script efetivo. Em `shaper/shaping.rs:256`, math pode forçar script `math`, enquanto ssty pode continuar None (`:21`). O estudo compara chamadas isoladas e documentos com ambas as configurações nas duas ordens. Isso verifica equivalência do cache local atual; ampliar sua duração sem antes validar entradas completas agravaria qualquer alias existente.

Não se conclui que um hash de conteúdo basta para todas as caches. Plano e resultado têm dependências diferentes; identidade da fonte não substitui texto/configuração nem geração da seleção. O cache de métricas possui chaves/locks/retry próprios e não deve ser fundido especulativamente ao cache de emissão.

## Critério de confiança e sequência recomendada

1. Fixar o contrato de identidade e geração das fontes e demonstrar reindexação/troca/recuperação. Auditar chaves antes de ampliar cache de shaping.
2. Explorar retenção de um componente L3 de fontes com snapshots novos de compilação. Comparar com execução limpa após cada edição; não conservar inadvertidamente source/asset/read_cache ou falhas velhas.
3. Validar faces/instâncias/planos por configuração efetiva, primeiro dentro de uma compilação. Verificar clone/share versus recriação entre fases de métricas/shaping. Não persistir resultados textuais indiscriminadamente.
4. Medir cold/warm e contagem de trabalho repetido no mesmo input/contexto. Confirmar no watch real, incluindo erro→sucesso e publicação; não deduzir ganho do tempo exibido vanilla nem somar fixtures de microbenchmarks diferentes.
5. Somente depois avaliar catálogo em disco. Ele exige schema/versão, validação de bytes/face/configuração, escrita atômica, recuperação de corrupção e bounds. O vanilla auditado conserva catálogo em RAM; não serve de evidência para que persistência em disco seja necessária ou segura aqui.

A aceitação exige comparação quente versus limpa para alterações de source, includes e assets, sys.inputs/features, ordem de fontes, TTC face, eixos, math/TextItem/ssty, direção e configuração de shaping. Incluir falhas/removidos→reaparecidos, invalidação concorrente e budgets de memória. Outputs devem ser verificados na linguagem/geometria relevante; identidade de buffers e raw glyph signatures são controles locais, não substitutos de paridade global.

Callbacks/transcripts e loops de estabilização têm trabalho que pode ser semanticamente necessário: pipeline/paged proíbe cachear stores entre compilações. O estudo não classifica todo passe repetido como desperdício. Métricas novas só entram como prova com fingerprints, perfis, exclusividade e estabilidade — os datasets anteriores continuam exploratórios.

## Pendências

- Frequência/custo real de discovery, reconstrução, instâncias e planos nos documentos/watch representativos; não medidos nesta sonda.
- Validar diferenças de chave com fixtures variáveis e features/scripts reais, além da precisão textual e amostras atuais.
- Definir invalidação para fontes de sistema e paths de projeto; dependências de World incluem sources/assets/falhas, mas não sistema de fontes como universo monitorado completo.
- Memory/eviction do componente retido, respeitando P1410; não aquecer todos os FontSlots.
- Eventual discrepância entre contrato de reutilizar base face em shaper/shaping e criação observada de Face rustybuzz no miss: auditar a substância por tipo e configuração antes de alterar owner/consumer.

## Proveniência e conclusão do estudo

HEAD `6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5`, working tree não commitado. [provenance.json](reuse-trust-study/provenance.json) registra comandos de resolução/build release e build final offline locked, perfil/toolchain, identidade do executável e inputs do harness. source-before/after.json conservam consumidores, L0s, pin original/staged e fontes; status.txt/diff-stat.txt/incoming.patch registram a lista exata de alterações, incluindo untracked. O fingerprint antes/depois permaneceu `9d29d3f8b1a0223b55eff92b955b4cd0735d205cd4c49970825ae2047bc44fe2`, igual à coleta vanilla anterior.

Sonda UTC `2026-09-27T15:52:33.741288+00:00` a `2026-09-27T15:52:33.903020+00:00`; timestamps identificam a execução, sem usar a duração como resultado de desempenho. Executável SHA-256 `8dba94817f080c7dd61b3f6bff1f6a40ce9cbcd86607b380ca18a77c37691454`, preservado em bin/. Rustc 1.92.0; Cargo profile release opt-level 3, sem debug assertions/overflow checks, flags Rust ambientais neutralizadas. Uma falha inicial de preparação (Point não implementa Default) ficou arquivada em failed-preparation; corrigiu-se somente a construção do fixture diagnóstico e o build final passou. Nenhuma amostra veio daquela falha.

Não foram executados novos microbenchmarks de tempo nem feitos experimentos end-to-end/watch neste estudo; portanto não há novos fatores de ganho ou custos atribuídos. Os benchmarks existentes não mudaram e sua compilação otimizada anterior continua registrada no dataset selado. Formatação limitada ao main.rs diagnóstico; sem resselo global. Os datasets/relatórios anteriores e a árvore produtiva foram verificados intactos.

A conclusão útil é **preservar o componente de fontes por identidade/geração, sem conservar inadvertidamente o snapshot da compilação**. O reuso dentro do World já funciona, a troca de catálogo invalida corretamente no controle, e os caches de falha/source mostram por que retenção precisa de invalidação. Planos/faces e fragmentação de métricas continuam candidatos de retrabalho, sem parcela temporal demonstrada. Antes de ampliar a duração desses caches, é necessário auditar suas chaves e executar os controles de dependências descritos acima. Este relatório entrega o estudo e seus limites, não uma alteração de arquitetura materializada nem garantia global de cache correto.
