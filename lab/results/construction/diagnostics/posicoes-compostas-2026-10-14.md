# Posições acompanham a composição dos frames

A posição observável de conteúdo dentro de Box agora acompanha origem da
página, inset, prefixo e aninhamento. A identidade continua sendo registrada
durante o layout; pontos privados seguem as transformações da geometria e
são projetados antes do sealing dos mapas existentes. Os registros privados
são retirados do documento público. Nenhuma closure é reexecutada nessa etapa.

## Evidência anterior e proveniência

O diagnóstico box-contexto-2026-10-14.md demonstrou que here().position()
ficava em (0,0) mesmo quando inset e prefixo deslocavam o conteúdo. Essa
invariância indevida constitui evidência independente do vanilla. A
comparação isolada de baselines do contexto direto não fundamentou reparo.

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado.
Snapshot inicial em 2026-10-14T03:46:47.345512+00:00, com fontes, L0, núcleos, status e diffstat
em before.json.gz. raw-before.json.gz registra a inclusão posterior do
owner de Raw. pre-validation.json.gz conserva o candidato final;
workspace-pre.json.gz registra todos os fontes Rust, HEAD, árvore suja,
ambiente, receita e UTC antes da suíte. scope.json separa corpos alterados
de headers apenas ressellados. Não atribuir a este lote a árvore suja inteira.

## Contrato e implementação

O núcleo coordinates foi qualificado para admitir frames locais explícitos
com transformação até a página, além dos percursos absolutos existentes.
Seus consumers foram repinados. O novo núcleo position-frame-carriers fixa
captura única, neutralidade geométrica, ordem lógica e rollback.

O owner position_marker possui o transporte privado de Location/Label e a
projeção por Group.pos/matrix. A última escrita lógica prevalece, mesmo
quando a ordem de pintura muda. Duplicar visualmente um registro não cria
uma nova observação. Se a última captura não sobreviver na árvore, mantém-se
seu valor runtime; uma captura anterior não o sobrescreve. Checkpoint trunca
o registro junto com a restauração de runtime/frames.

Mod/Realized preservam alocação, observe e política insert/or_insert;
References mantém a captura de labels. Finish projeta as camadas de cada
página antes do sealing. O caminho HTML detalhado preserva seu protocolo;
o retorno visual HTML remove ambos os formatos privados. Image, Shape,
Table e Raw deixam de confundir um registro direto sem tinta com conteúdo.
Pagebreak fecha também uma linha que só contenha registros posicionais
antes da transição explícita, sem alterar as transições de overflow.
Box e ContextBlock receberam testes, sem nova implementação produtiva nesses
owners neste lote. Não se adicionou API pública, fase, default ou variante
pública de FrameItem. ADR-0127: correção interna contínua, L0 primeiro.

## RED→GREEN e revisão do candidato

Os seis testes causais de Box falharam no estado anterior e passaram após
o reparo: inset, prefixo/identidade do pai e sufixo, aninhamento, shape,
contexto vazio e labels. red-state.json.gz, red.log e green-first.log
preservam as execuções.

O helper testa matrizes/página, ordem lógica versus pintura, cópias visuais,
rollback e reconhecimento estrito de registros privados. A revisão adicionou
um teste de última captura descartada: o primeiro candidato aplicava a
projeção antiga sobre o fallback mais recente. discard-red.log confirma a
falha, e marker-final.log confirma os seis testes do helper passando depois
do reparo. first-candidate-validation.json.gz conserva o candidato anterior;
nenhum esperado de regressão foi relaxado para aprová-lo.

A primeira suíte completa encontrou uma regressão P1159: referência de
página com rodapé explícito emitia R8 em vez de R7. O registro do label
após uma linha fechada não tinha avanço horizontal; Pagebreak deixava-o
pendente para a página seguinte. page-boundary-candidate.json.gz conserva
a suíte falha e seus fontes. O novo teste causal confirmou página 2 em vez
de 1 (pagebreak-red.log); o reparo fecha esses registros na página correta.
marker-pagebreak-green.log e a suíte final verificam o reparo mantendo o
esperado R7 do teste de integração. pagebreak-before.json.gz registra o
owner antes dessa extensão; L0 atualizado primeiro.

## Sondas do CLI final

Execução de 2026-10-14T04:15:20.452879+00:00 a 2026-10-14T04:15:26.614948+00:00. Binário Tekt SHA-256
cc8ad709c02ae3a64b23e08f90f06ed03204b81b46057b92db45bb13789ee198; vanilla
7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea, pin
9f2b6e8715237cb086899a42873660fe744622e8, árvore ratificada
6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4. A receita do oráculo é a de P1489.
Os hashes foram verificados antes e depois. after-probes/probe.json.gz
registra fonte, comando, diagnóstico, texto, HEAD e diffstat; PDFs preservados.

| Caso | Tekt antes | Tekt depois | Vanilla pinado |
|---|---|---|---|
| Box | (0,0) | (10,10) | (10,10) |
| Inset 2pt | (0,0) | (12,12) | (12,12) |
| Prefixo P | (0,0) | (15.95,10) | (15.95,10) |
| Aninhamento com inset 3pt | (0,0) | (13,13) | (13,13) |
| Contexto direto | (10,10) | (10,10) | (10,17.24) |

Valores em pt, na precisão impressa pelo produto. O prefixo apresenta
quebra textual diferente no controle vanilla: coincidência das coordenadas
não é prova de paridade da composição textual inteira. A âncora do contexto
direto permanece uma questão separada, sem copiar a baseline do oráculo.

Os cinco pares de neutral-probes/results.json.gz comparam conteúdo direto e
contextual: vazio, caixa vazia, texto, forma e prefixo/sufixo, em página auto.
Dimensões e caixas textuais extraídas são iguais em todos. Essa verificação
não equivale a comparação raster geral; os testes de geometria existentes
integram a suíte completa.

## Validação final

Suíte workspace serial em Linux, TMPDIR em /dev/shm, oráculo pinado e linter
publicado no PATH: 8394 passaram, 0 falharam,
7 ignorados. Execução 2026-10-14T04:13:22.220991+00:00 a 2026-10-14T04:19:44.136877+00:00; fontes
inalterados durante a medição. workspace-result.json e workspace.log contêm
comando, ambiente e resultados por alvo.

Build, check de todos os alvos, Rustdoc com -D warnings, formatação,
linhagem estrita V1/V5/V15/V26, verificação reversa e git diff --check passaram.
Clippy em modo de inventário manteve 228 avisos, sem variação por
regra; não é aprovação do gate CI -D warnings. Testes infra continuam com
os mesmos 1289 nomes. O relatório arquitetural tem
0 erros, 382 warnings e
1624 notas; portanto o gate global zero violations permanece
aberto. O selo foi reparado em staging isolado e copiado somente aos owners
explicitamente listados em seal-sources.json.

## Limites e continuidade

Este lote corrige os casos de Box e quebra explícita listados, sem
certificar todas as formas de paginação/observação. Uma sonda adicional
de altura 40pt confirmou panic de coleta de links/referências em quebra
automática, ausente da suíte: links-paginacao-2026-10-14.md registra três
caminhos que abortam e um controle textual que passa. Esse reparo tem
prioridade imediata; não se declara esse caminho corrigido. Box fracionária segue com as lacunas de
pintura, recorte e isolamento descritas em box-fracionaria-2026-10-14.md.
O objetivo geral continua aberto, incluindo grupos multipágina, marginais
contextuais, notas longas, Clippy e validação remota de CI/release.
Não houve commit, push, publicação ou execução remota nesta rodada.

Logs, fontes de sonda, PDFs, snapshots e receitas estão em evidence.json.gz,
com SHA-256 de cada entrada conferido após descompressão. Os logs avulsos
podem ser ignorados pelo Git; o arquivo comprimido conserva a evidência.
