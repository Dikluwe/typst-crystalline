# Captura de links através de linhas e páginas

Links URL, referências e links internos do outline agora conservam seus
fragmentos durante transições de linha, página e coluna, sem coletar buffers
por índices antigos. As sondas que abortavam passaram a gerar PDF. Foram
corrigidos também delimitadores extras no endereço PDF e uma fórmula que
estendia a área clicável sobre a linha seguinte. A referência de página após
um título ainda pode usar a página seguinte; esse caso permanece aberto.

## Medição e proveniência

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado.
Os snapshots before.json.gz, red-state.json.gz e pre-validation.json.gz
conservam o início e os candidatos do reparo. followup-before.json.gz e
bbox-before.json.gz registram as extensões posteriores. O estado final
validado está em followup-validation-pre.json.gz, capturado em 2026-10-14T04:52:16.587524+00:00,
com fontes e L0 completos, SHA-256, status e git diff HEAD --stat.
Não atribuir a este lote todas as alterações preexistentes da árvore.

links-paginacao-2026-10-14.md registra o RED anterior no CLI: Link URL, Ref e
Ref Page abortavam após o layout esvaziar buffers e o coletor reutilizar
índices antigos. Texto sem link atravessava a mesma fronteira normalmente.
Esse fato não demonstra qual mudança histórica introduziu o defeito.

## Contrato e implementação

O novo owner link_capture e o núcleo link-frame-scopes especificam escopos
privados por destino. Limites sem área acompanham os frames; a resolução
terminal conserva a ordem e a região, remove os limites e produz links por
fragmento. O corpo é executado uma vez. Não há nova variante pública de
FrameItem, API ou fase; correção interna contínua conforme ADR-0127.

Fechamento de linha, página, coluna e subframe segmenta os escopos. Floats
conservam sua pertença lexical; marginais, camadas de página, notas e floats
alheios não herdam o link ambiente. Rollback restaura o registro e os
escopos. Posições são projetadas depois da resolução dos links. A linha
explícita e a origem dos floats ignoram os novos limites sem geometria.

Na exportação PDF, escape_pdf_literal já fornece o operando completo:
a anotação deixou de acrescentar outro par de parênteses. No cálculo de
bounds, vertical_metrics retorna ascender e altura total da linha, não
descender; Text e TextShaped deixaram de somar o ascender à altura total.
Os owners foram atualizados antes das alterações produtivas e ressellados
em staging, com cópia restrita aos arquivos autorizados nos logs de selo.

## Testes e revisão dos candidatos

O RED funcional inicial reproduz perda de segurança nas fronteiras, wrap,
quebras explícitas, corpo rico, colunas e destino interno. Os testes de fluxo
também cobrem corpo vazio, links aninhados, rollback, floats próprios e
alheios, background, foreground e footer. Os testes antigos que observavam
um Link imediato no buffer passaram a observar o documento final,
conservando as exigências de conteúdo e bounds.

O primeiro candidato revelou deslocamento por limite sem tinta na quebra
explícita; a revisão de float revelou origem alterada pelo mesmo tipo de
limite. Os respectivos logs e snapshots estão preservados. canvas-red.log
registra a captura indevida de background antes da separação de escopos.
O erro de import do primeiro fixture está em test-fixture-compile.log e não
é contado como RED comportamental.

O lint identificou V2 no helper novo, cujos testes de integração estavam
somente em link.rs. Foram acrescentados testes locais de preservação dos
vizinhos, bounds finais, escopo vazio e artefato legítimo semelhante a um
marcador. Não foi inserido um teste vazio para silenciar a regra.

uri-red.log confirma duas falhas de destino serializado; followup-uri-green.log confirma
os dois casos após o reparo. bbox-red.log confirma Text e TextShaped com
altura indevidamente aumentada; followup-core.log confirma os 21 testes
focados de links e captura após as correções.

## PDFs finais

Sondas de 2026-10-14T04:53:14.871879+00:00 a 2026-10-14T04:53:27.794962+00:00; binário Tekt
SHA-256 df329addda2adf58a81c9233f8fc81849015dba025af7b0433937b34d5d21efd.
Vanilla SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea, pin
9f2b6e8715237cb086899a42873660fe744622e8, árvore ratificada
6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4, receita P1489.
Os hashes dos binários foram verificados antes/depois. Fontes, comandos,
saídas, PDFs, XML de texto e objetos decodificados por qpdf estão em
followup-probes e no arquivo de evidências.

As entradas de Link URL, referência normal e referência Page terminam sem
panic. Wrap e linhas explícitas conservam exatamente as caixas textuais do
controle Tekt sem link. Centros das caixas das palavras do corpo pertencem
às anotações esperadas; P e Q externos ficam fora. Corpo vazio não gera
anotação. Os URLs simples e com parênteses internos são decodificados sem
delimitadores extras. SUFFIX na linha seguinte já não está na área clicável
do link anterior. probe-checks.json registra os predicados executados.

A contagem de linhas por página difere do vanilla também no controle sem
link; portanto essa diferença não é atribuída ao reparo da captura. Esta
medição usa texto, geometria e destinos; não exige identidade de bytes PDF.

## Validação e limites

A suíte completa do candidato de captura, antes dos reparos adicionais de
URI e altura, passou de 2026-10-14T04:41:29.858642+00:00 a 2026-10-14T04:48:44.746577+00:00: 8406 testes
passaram, 0 falharam e 7 foram ignorados, com fontes
inalterados. workspace-pre.json.gz e workspace-result.json identificam
exatamente esse estado; não representam a suíte completa do estado final.

No estado final, a suíte core passou com 6762 testes unitários, 8 de
integração e 3 doctests; 5 unitários ignorados. Os dois testes novos de URI
passaram. Build, check de todos os alvos, formatação, linhagem estrita
V1/V5/V15/V26, linhagem reversa e git diff --check passaram. A validação
final terminou em 2026-10-14T04:54:04.047618+00:00, com fontes inalterados. Rustdoc com
-D warnings passou no candidato anterior, conforme validation-results.json.

Clippy em modo de inventário mantém 228 avisos, sem variação
por regra; não equivale a passar -D warnings. O lint arquitetural final
registra 0 erros,
382 warnings e
1631 notas. O gate global zero violations e a
preparação completa para produção continuam abertos.

A sonda page-ref ainda escreve page 2 para o título da primeira página;
a sonda logical-ref escreve R8, embora a consulta do título indique página
física 1 e contador 7. Hipótese a testar: o carrier do label capturado após
um título já fechado fica pendente e migra com o próximo conteúdo na quebra
automática. O reparo anterior de Pagebreak explícito não cobre esse caminho.
Não fechar a correção da referência enquanto esse RED não for resolvido.

No link aninhado, o centro de INNER pertence aos dois retângulos, externo e
interno. Os destinos distintos são preservados, mas a prioridade de clique
entre anotações sobrepostas ainda não foi validada; não declarar equivalência
com os três fragmentos não sobrepostos do vanilla. Esses dois casos, Box
fracionária e os demais itens da auditoria de produção seguem pendentes.

Não houve commit, push ou publicação. Evidências e receitas estão em
evidence.json.gz com SHA-256 verificado após descompressão.

## Seguimento dos labels

labels-fronteira-2026-10-14.md registra o reparo posterior da captura do
label após linha fechada: as referências voltaram a page 1 e R7 nas sondas
de quebra automática. O leitor independente revelou ainda falha de
resolução da árvore de destinos PDF, distinta da captura/paginação.
