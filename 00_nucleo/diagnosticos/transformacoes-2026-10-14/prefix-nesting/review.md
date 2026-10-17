# Prefixos multiline e aninhamento — encerramento da etapa

Esta etapa corrige os três casos pendentes do relatório anterior: o prefixo 40/8 com cor, o prefixo 21 com múltiplas palavras e o escopo parcial OUTER18/INNER8 contendo Block. A extensão de aninhamento também mostrou uma margem automática perdida no texto posterior ao bloco; esse caso foi incluído na correção, preservando below explícito.

## Mudanças

- Prefixos multiline anteriormente recusados são certificados antes da emissão e medidos como segmento inteiro. A altura causal das linhas e a margem entram uma vez na reserva anterior ao `1fr`.
- No novo caminho, o leading usa a cadeia lexical comum. Cada linha usa o top/bottom de seu próprio conteúdo; a baseline prevista é reconciliada também quando o próximo trecho tem ascent menor. Os estilos, incluindo fill vermelho, permanecem próprios de cada trecho.
- Uma Sequence que entra com linha exterior aberta herda seu dono lexical. O Block/Align interno fecha sob esse dono, preservando o espaçamento exterior do parágrafo. Wrappers irmãos com valores iguais continuam sem criar um ancestral comum.
- O contexto é privado, participa de checkpoint/rollback e é suspenso/restaurado no subframe isolado. O modo geométrico do novo prefixo termina antes do bloco. Os hooks dos certificados escalares anteriores continuam separados.
- Below automático de Block publica margem fraca pelo protocolo já existente. O próximo parágrafo pode colapsá-la com seu espaçamento exterior; below/spacing explícitos, inclusive zero, conservam sua prioridade.

Foram atualizados dez consumers e seus dez Prompts L0 proprietários: hub de layout, paragraph, sequence, fractional_preflight, inline_prefix, line_commit, checkpoint, sub_frame, block e tests. O novo Núcleo Tekt `00_nucleo/prompts/_nuclei/layout/text-segment-scope.toml` reúne as invariantes compartilhadas e está pinado por nove owners; não é alvo de header @prompt. O grafo permanece 1:1 por consumer. `stage.diff` é relativo ao snapshot inicial desta etapa, preservando alterações anteriores do working tree.

A classificação é correção interna de paridade ADR-0127, em fluxo contínuo: L0 primeiro, RED→GREEN e resselo restrito. Não houve mudança de entidade/trait/assinatura pública, flag do produto ou fase de pipeline.

## Verificação

Os dois testes negativos de prefixo foram substituídos por regressões positivas de paginação, posições e entrelinha. Foram adicionados testes de fechamento parcial, transporte de contexto por checkpoint/subframe e margem automática versus below 0/5 explícitos. Os três casos principais falharam antes da implementação; o teste adicional de saída de escopo reproduziu AFTER 86.4 contra 98.4 pt sob FixedMetrics antes do ajuste de margem. A rodada final de layout teve 1.215 aprovados, zero falhas e três ignorados, com fontes imutáveis (registro `implementation/prefix-nesting-layout-final-result.json.gz`). Uma construção inicial de fixture reutilizou um valor movido de SubLayoutRegion e foi corrigida sem alterar sua API; o erro de compilação está preservado nos logs.

A comparação bilateral final coincidiu em páginas, texto por página e coordenadas (tolerância de 0,0001 pt) em quatro conjuntos separados:

| Bateria | Resultado |
| --- | --- |
| Prefixos/aninhamento e extensões | 33/33 |
| Controles da etapa de escopo/cauda | 28/28 |
| Margens de Block | 24/24 |
| Bordas tipográficas e viúvas/órfãs | 11/11 |

Os três refutadores falhavam na bateria inicial; agora coincidem. Os cinco controles adicionais com AFTER que falhavam no primeiro candidato também coincidem após a classificação de margem automática. Fontes, UTC, comandos e guardas estão nos respectivos results.json.gz e em summary.json. Os conjuntos não são apresentados como necessariamente disjuntos.

SVG/PNG final (`2026-10-15T00:02:44.103253+00:00` a `2026-10-15T00:03:51.659032+00:00`): 33 documentos de página única, 132 compilações bilaterais; 33 SVGs rasterizados idênticos, 14 PNGs idênticos e 19 com diferença máxima de um nível por canal, sem diferença de dimensões. A rasterização é prova auxiliar de estilo/geometria, não contrato de igualdade de bytes (ADR-0107).

Suíte final (`2026-10-14T23:54:39.031072+00:00` a `2026-10-15T00:02:43.891938+00:00`): **8.593 aprovados, 0 falhas, 7 ignorados**, com guarda de fontes/L0 válida. Comando: `cargo test --offline --locked --workspace --no-fail-fast -- --test-threads=1`, oráculo pinado, linter CI no PATH, temporários em `/dev/shm`, sockets locais autorizados e Cargo com um job. Logs/resultados em workspace-delivery/.

Validação final concluída em `2026-10-14T23:54:38.840804+00:00`: build, rustdoc com `RUSTDOCFLAGS=-D warnings`, check de todos os targets, fmt e diff passaram. Linhagem V1/V5/V15/V26 com `--fail-on warning` passou; reversa conferiu 970 consumers, zero erros. Linter completo: zero erros, 380 avisos e 1668 notas. Clippy conserva 181 avisos, sem delta por classe. **Inventário `--cap-lints warn` não é aprovação do gate CI `-D warnings`.** Nenhuma supressão nova foi adicionada. Linter pinado em 49c6bb0e584104a0c89270431d8402f5c6c61150.

SHA-256 do Tekt na medição final: `6ca39d20377863440150b41b24709d17a03a419d7edf3c28cb951f33935a7f4c`. Os conjuntos completos usados no fechamento têm guarda de imutabilidade válida.

A primeira suíte sob serviço limitado teve três falhas de ambiente: rsvg-convert não estava no PATH do gerenciador. Sua execução e resultados estão preservados em workspace/. A rodada final preserva o PATH explicitamente, sem mudar os testes ou fontes.

## Investigação e correção do rustdoc

O kernel confirmou três OOM globais: rustdoc com cerca de 44,1/42,6/43,8 GiB foi encerrado. As tentativas incompletas não foram tomadas como checks aprovados (`memory-incident.json`). A execução protegida do workspace confirmou crescimento no binário L4 com `--document-private-items`, atingindo o limite próprio de 12 GiB; o teto funcionou e isolou o encerramento no job.

A investigação alterou apenas a saída de documentação, usando a mesma fonte e argumentos. Em diretório vazio, o binário real passou com cerca de 0,5 GiB. Copiar somente o antigo `search.index` reproduziu a falha com limite de 12 GiB. As reproduções e fontes temporárias estão em rustdoc/; o crescimento depende do índice acumulado, não de uma ampliação de RAM do layout. A identificação do arquivo individual ou da causa interna no algoritmo Rust 1.92 permanece além da prova desta etapa.

Foram preservados 1.708 arquivos gerados e oito fingerprints de documentação em `target/doc-prefix-nesting-quarantine-20261014`. O índice anterior foi também arquivado em `rustdoc/search-index-before.tar.gz`, com manifesto/hashes. Só artefatos gerados de documentação foram invalidados; nenhuma implementação ou API foi modificada por este reparo.

A regeneração completa de `2026-10-14T23:45:40.680374+00:00` a `2026-10-14T23:45:59.342564+00:00` passou, com pico observado de `0,76 GiB`, fonte/L0 estáveis e páginas HTML de typst, typst_core, typst_shell e typst_infra presentes. O job final executou dentro de MemoryMax de 12 GiB, MemorySwapMax de zero e um job Cargo; as amostras efetivas estão em memory-budget-samples.json. O índice antigo ocupava cerca de 43 MiB; o índice regenerado observado após os contratos JSON ocupava 3,2 MiB. Os contratos JSON substituem a saída HTML de typst_infra; a documentação foi revalidada após a suíte (`2026-10-15T00:04:18.969203+00:00` a `2026-10-15T00:05:31.002724+00:00`, exit 0 e guarda válida), confirmando a recomposição das quatro páginas HTML, com pico observado de 1.06 GiB. O diagnóstico e as medidas de proteção estão em memory-incident.md. Não se alterou a configuração global de RAM do computador.

## Proveniência

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, branch Tekt, working tree não commitado. Cada prova usada para fechamento preserva status, `git diff HEAD --stat`, hora UTC, argv e hashes de fontes/binários. `before-edit.json.gz` preserva o texto inicial dos fontes/L0; `final-files.json` identifica os 21 arquivos da entrega, incluindo o Núcleo. Os registros de validação preservam o texto das fontes e verificam imutabilidade.

O oráculo é vanilla pinado `9f2b6e8715237cb086899a42873660fe744622e8`, árvore `6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4`, receita P1489, binário `lab/typst-original/target/release/typst`, SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`. O Tekt medido é `target/debug/typst`, com SHA-256 em cada results.json.gz. A string de versão não é usada como prova suficiente de identidade.

Rodadas de validação/raster interrompidas estão preservadas e não são tratadas como conclusão. A retomada da validação reutiliza apenas comandos terminados com exit 0 depois de conferir novamente todos os hashes de fonte; os demais são executados e registrados. Os jobs finais têm PID, UTC, comando, log e resultado persistidos. Nenhuma fonte produtiva ou L0 foi editada durante os testes/sondas/validação.

`evidence.json.gz` reúne fontes, medições, saídas, L0, código e scripts; `archive-receipt.json` registra o SHA-256 do arquivo e a conferência byte a byte após reabertura. Todos os diretórios de medição são exclusivos ou retomadas documentadas, sem sobrescrever provas anteriores completas.

## Limites

Os três refutadores nomeados estão corrigidos no recorte verificado. O novo prefixo multiline exige também a certificação tipográfica de paragraph_prepare: Linebreak explícito, overrides não suportados e corpos opacos não entram nessa extensão. As rotas anteriores permanecem para entradas recusadas. Esta etapa não certifica paginação universal, bidi/floats, todos os containers nem desempenho.

Os avisos anteriores de Clippy/Tekt e as demais frentes de CI/release continuam como pendências gerais. Sucesso desta etapa não constitui fechamento do objetivo global de todas as correções. Não houve commit, push ou publicação de release.
