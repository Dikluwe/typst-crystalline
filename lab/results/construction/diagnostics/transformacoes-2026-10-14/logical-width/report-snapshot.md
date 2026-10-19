# Transformações — implementação e pendências

Estado: contrato tipado aprovado e materializado; validação X1–X5 em andamento. A frente permanece aberta.
A confirmação humana está em ../materialization/transformacoes-tipadas-2026-10-14/approval-receipt.json;
manifesto `960f20a8593874da3f3588281dbefa535e4e0ade8109ae117acbd4f5a038281d`. Os candidatos são a revisão aprovada; não são prova de implementação.

## Proveniência

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado. before.json.gz e results.json.gz
registram lista exata de alterações/diffstat, hashes das fontes/L0 e dos binários.
Sondas principais UTC 2026-10-14T17:35:13.352950+00:00 → 2026-10-14T17:35:31.695096+00:00: 47 queries e 25 renders bilaterais.
Pin vanilla `9f2b6e8715237cb086899a42873660fe744622e8`, árvore
`6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4`, receita P1489; binário SHA-256
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
SVG rasterizado a 2x com rsvg-convert, sem alinhar imagens. render-metrics.json
registra caixas de tinta e diferenças; são diagnóstico, não tolerância de aprovação.

## Medição e decisão anteriores à implementação

`01_core/src/entities/elements/transform.rs:19` armazena somente matrix/body.
`01_core/src/compiler/stdlib/transforms.rs:19/51/91/191` elimina identidade,
presença, unidades e parte dos argumentos. `compiler/layout/transform.rs:28`
recebe matriz já reduzida e calcula pivô por heurísticas, sem origin/reflow explícitos.
Os L0 antigos também especificam esses limites; a falha envolve código e contrato.

fields de rotate retorna vazio no Tekt e body/argumentos explícitos no pin;
func não corresponde à função nativa. Rotate zero e Scale 100% com o mesmo corpo
se comparam iguais no Tekt, distintos no pin. Omissão e defaults explícitos colapsam.
Named x de Scale altera y indevidamente; Move perde percentual/em; origin/reflow
são ignorados ou rejeitados. Bindings aceitam números sem unidade e ignoram sobras.
Os controles suplementares fixam casts, ordem, fase do erro de dois auto e set/getters.
Contextos com label no wrapper eram fixtures inválidas para extrair value:
context-controls/results.json corrige a label no metadata. O primeiro scale zero
sem body também é fixture inválida; os controles seguintes incluem body.

Contrato aprovado: variantes tipadas com Option para presença, resolução em layout,
Matrix Rust separado e preservação de observação, seleção, mapas e encoders. Cada
owner continua 1:1; núcleo transform-semantics pinado. V15/V26 passou no espelho
completo de revisão. Aprovação não permite contratos públicos adicionais inferidos.

## Divergência a conservar

Em quatro controles HTML, o pin avisa que ignorou a transformação e elimina TEXT.
O Tekt rejeita explicitamente conteúdo não suportado, conforme owner HTML vigente.
Não copiar perda silenciosa de conteúdo. Ilhas html.frame mantêm seu contrato.

## Evidência e continuidade

Arquivo evidence.json.gz: 222 entradas com bytes/base64 e SHA-256,
reabertas e comparadas aos arquivos. SHA-256 `2c20fc2d7df812ae60841a2927df09b3d680672c54ae203c37c766c2319ecfae`.
Inclui proposta aprovada, recibo, scripts, fontes, queries e imagens. A implementação
precisa RED→GREEN e revalidação X1–X5; este diagnóstico não encerra transformações,
paridade geral, CI ou release. Nenhuma fonte produtiva mudou nesta medição.


## Implementação e regressões — 2026-10-14

A aprovação foi aplicada aos construtores, carrier tipado, observação, fields/func,
repr, mapas, seletores, estilos/getters e resolução de layout. Matrix Rust conserva
seu caminho próprio. A lista exata de consumers está em
`transformacoes-2026-10-14/implementation/changed-sources.json`.
Os fontes anteriores à correção de reflow e ao limiar local estão preservados em
`implementation/reflow-before.json.gz` e `implementation/line-threshold-before.json.gz`.
Os resultados numéricos abaixo correspondem a HEAD
`06848cdcb5450f2c555626bf116bc7acd82bd82b` com working tree não commitado;
cada arquivo de resultados citado inclui UTC, lista exata/diffstat e hashes.

A primeira suíte completa passou com 8554 testes, 0 falhas e 7 ignorados,
UTC 18:15:36.596408 → 18:22:31.733732Z (`implementation/workspace-result.json`,
proveniência em workspace-pre.json.gz). Esse resultado precede a correção seguinte.

As sondas adicionais refutaram a região original de medição de reflow.
Scale precisa medir na região inversamente escalada; Rotate usa a região com
rotação inversa. O marcador de altura auto de Block não substitui a região
externa finita. Medições especulativas restauram o checkpoint antes da composição.
Outro defeito, compartilhado com caixas comuns, usava margem física como limiar
para quebra em coordenadas locais; o L0 preservava explicitamente essa regra.
A comparação bilateral margin-before confirmou overflow. Word/chunk agora passam
a origem real da linha ao seletor; palavra inicial longa continua progredindo.

As duas regressões de reflow falharam antes da implementação e na primeira
tentativa (`implementation/reflow-red.log`, `reflow-green-first.log`). Depois das
correções, a suíte do núcleo passou: 6853 testes, 0 falhas e 5 ignorados
(`implementation/reflow-core-result.json.gz`, comando/log/proveniência homônimos).
O teste da origem local verifica também ausência de linha vazia espúria.
Build do CLI, fmt global, linhagem V1/V5/V15/V26 estrita e linhagem reversa passaram
(`implementation/reflow-build-second-result.json.gz`, `reflow-fmt-result.json.gz`,
`reflow-lineage-result.json.gz`, `reflow-reverse-result.json.gz`). Linhagem reversa:
970 consumers e 0 erros. Isso não elimina os demais avisos arquiteturais/Clippy.

`after-fourth/results.json.gz` (18:37:51.984497 → 18:37:58.216716Z) conserva
74 queries com stdout/primeira linha de diagnóstico/exit coincidentes e 24 controles
SVG rasterizados coincidentes; o outro render rejeita corretamente dois eixos auto.
`refuters-third/results.json.gz` e `margin-after/results.json.gz` mostram a correção
de reflow scale/rotate em SVG e da quebra de caixas com margem grande. Todos
preservam texto sem realinhamento de imagens. `refuters-pdf-first/results.json.gz`
estende a investigação a PDF via pdftocairo; diferenças de pixels são inventário,
não tolerância ampliada nem prova de equivalência semântica.

A nova suíte completa terminou com 8557 testes passados, 0 falhas e 7 ignorados,
UTC 18:41:23.018139 → 18:47:21.298731Z. Resultado inspecionado em
`implementation/reflow-workspace/workspace-result.json`; fontes inalteradas durante
a execução, conforme guard e workspace-pre.json.gz. Este resultado corresponde
à correção de reflow/origem local, antes de qualquer correção seguinte no PNG.

## Refutadores ainda abertos

- Baseline lógica de texto dentro de Box com Rotate/Scale/Skew e avanço posterior
  a skew vertical (`refuters-third` e `refuters-pdf-first`).
- Posicionamento horizontal RTL de transformações nativas.
- Largura lógica, overflow e espaços foram corrigidos nos controles da etapa
  `logical-width` abaixo. Escala absoluta, flip de texto/shape/Box excedente e
  espaço terminal fraco/literal/fracionário agora têm regressões causais.
  Isso não certifica dimensões vazias/não finitas nem toda a baseline de Box.
- O defeito de rasterização afim de TextShaped foi corrigido na etapa abaixo.
  Casos com Box ainda divergem também em SVG/PDF; não confundir sua baseline ou
  largura lógica com falha remanescente de rotação dos glifos. Glyph direto,
  fontes coloridas/bitmap e suporte integral de matemática continuam não certificados.
- Gradiente Parent sob rotação diverge em SVG/PDF e coincide em PNG; investigar
  cada exporter antes de decidir que lado está correto.
- Empty/absolute-scale precisa de auditoria de finitude e transporte PDF; sucesso
  de compilação não prova matriz válida nem preservação do texto posterior.
- `measurement-controls-corrected` expõe measure(width:) ainda recusado e medida
  de espaço literal final divergente. `effects-controls` confirma que callback de
  state e query de label em texto já divergem sem transformação; esses controles
  não certificam ausência de efeitos duplicados da medição. X3 continua aberto.
  A primeira rodada measurement-controls tem sintaxe inválida em ambos os lados
  e foi preservada somente como tentativa rejeitada, nunca como prova funcional.

Ainda faltam concluir estes refutadores e a integração completa X1–X5.
Os avisos registrados abaixo continuam impedindo considerar o CI estrito limpo. A frente não certifica
produção, paridade geral, CI remoto ou release; nenhuma publicação foi realizada.


## Rasterização afim de texto PNG — etapa verificada

Em `03_infra/src/export/render.rs`, TextShaped agora usa contorno da face preparada
sob a transformação completa para rotação, shear, escala não uniforme e reflexão;
o caminho pixglyph uniforme permanece. Origem e escala de unidades da fonte são
compostas uma vez, com inversão do eixo da fonte, máscara do dispositivo e alpha.
Nenhuma API pública ou fase mudou. L0 foi atualizado antes dos testes e código.

Dois testes novos falharam funcionalmente antes da correção: a rotação de 90 graus
perdia a tinta, e o clip não encontrava o glifo transformado. Após a correção,
21 testes do renderizador passaram (`implementation/png-affine-{red,green}-result.json.gz`,
logs correspondentes e `png-afim/render-red.rs.txt`). A integração PNG passou com
7 testes, UTC 2026-10-14T18:56:45.344294+00:00 → 2026-10-14T18:56:56.800626+00:00, em
`png-afim/integration/workspace-result.json`, com guard das fontes e proveniência.
A suíte completa de 8557 testes descrita acima antecede esta mudança PNG; não
foi apresentada como uma nova suíte completa do estado posterior.

`refuters-fourth/results.json.gz` mostra reflow-wrap-rotate e matemática rotacionada
coincidentes em PNG. `margin-png-after` conserva a correção com margem grande e
passa a coincidir também no controle rotacionado. `glyph-affine-final/results.json.gz`
mede cinco controles adicionais coincidentes em SVG e PNG: quarter-turn, shear,
anisotropic, flip-x e flip-y. O sexto, glyph-clip, ainda diverge nos dois formatos:
a geometria do frame de Box precisa ser corrigida; o teste unitário de máscara
confirma somente o recorte do contorno já posicionado. Não realinhar imagens para
ocultar posição nem usar soma de pixels como prova universal de paridade.

A primeira validação expôs dois avisos novos de sin/cos nos testes; foram trocados
por libm pinado. Os auxiliares privados novos enumeram os casos sem baseline e
não nativos, exigindo revisão de futuras variantes em vez de defaults silenciosos.
As regressões de reflow foram reexecutadas após essa mudança, passando
(`implementation/hygiene-core-result.json.gz`).

Validação final (2026-10-14T18:58:38.511473+00:00): build, rustdoc com -D warnings, check de todos
os alvos, fmt, linhagem estrita V1/V5/V15/V26, linhagem reversa e diff check passaram.
Comandos, saídas completas, UTC, HEAD/diffstat e bytes/hashes atuais estão em
`png-afim/validation-final/`. O inventário Clippy permanece com 181 avisos,
com mesmos códigos/mensagens/arquivos do inventário anterior; não é execução
verde de -D warnings. O linter Tekt registra 380 warnings,
1662 notes e nenhum error. As duas categorias de
avisos permanecem trabalho aberto, distinto da linhagem estrita já validada.

Arquivo da etapa anterior: `implementation-evidence-1.json.gz`, 1440 entradas,
SHA-256 `46edca1da3dfc5724418a8cbe68b78b3360da997503b8f8d665611c1f51050c8`.
Ele foi reaberto e comparado antes desta etapa PNG. Nenhum commit, push ou release.

Arquivo recuperável desta etapa: `png-afim/evidence.json.gz`, 476 entradas
com path/SHA-256/base64, reabertas e comparadas. SHA-256 `d51ade885ea52025a55c2dbca7baab771718cdb72a9b3586fd24735d9ac67976`.


## Largura lógica de transformações — correção verificada

Medição anterior em `logical-before-complete/results.json.gz`, UTC
2026-10-14T19:04:35.804370+00:00 → 2026-10-14T19:05:02.359649+00:00,
com HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado,
status/diffstat exatos, hashes de fontes e binários. A primeira tentativa
`logical-before/` foi interrompida e não tem resultado final: não serve como
prova de fechamento. `logical-observation` confirma a morfologia do texto e
preserva o diagnóstico ainda aberto de measure(width:).

O erro era usar cursor e extensão dos descendentes como dimensão lógica.
`transform.rs` agora usa a largura causal do subframe, limitada pela região,
sem recortar os filhos. Isso corrige pivôs de reflexão, escala absoluta e o
controle de texto dentro de Box recortado. `line_commit.rs` exclui o avanço
literal terminal na quebra automática, preservando o carrier para extração;
o fim do parágrafo conserva avanço literal. `h_space.rs` publica o avanço
explícito na largura causal. O refutador adicional terminal-fraction encontrou
uma contribuição perdida de h(fr); o fechamento agora publica também a
ocupação já distribuída, sem alterar sua distribuição ou contar duas vezes.

L0s foram atualizados antes dos respectivos testes e código. Cinco regressões
falharam funcionalmente antes da correção e passaram depois; logs e proveniência
em `implementation/logical-width-*-red*`, `logical-width-*-green*` e
`logical-fraction-{red,green}*`. Os dois primeiros resultados usam os nomes
`logical-width-red`/`logical-width-green`. Os testes verificam posições
observáveis, espaços fortes/fracos, overflow e frações 1/2/0. O primeiro resselo
foi recusado por ordem de referências de Núcleo no L0 de h_space; a lista foi
ordenada antes do resselo final. Não houve novo contrato público nem fase nova.

A comparação final `logical-final` cobre 12 documentos em SVG/PNG;
`logical-spacing-final` acrescenta 8 controles. Os casos corrigidos coincidem.
Spacing-only/fraction-only ainda têm 19 pixels PNG com diferença máxima de
um nível de canal, também presentes antes: não declarar igualdade absoluta.
O refutador h(-20pt) continua recusado pelo Tekt em P156D e aceito no pin;
é uma pendência anterior do constructor, não certificada por esta correção.
Overlay-only e overlay com espaço conservam a geometria esperada.

`logical-pdf-final` executa os 20 controles em PDF. O texto extraído coincide
nos casos com pequenas diferenças rasterizadas; `text-geometry.json.gz` e os
XMLs de pdftotext preservam a medição sobre os PDFs identificados por hash.
A maior diferença de caixa de palavra nesses casos é 0.000021 pt. Isso dá
suporte à geometria medida, sem afirmar identidade de bytes ou prova universal
de que toda diferença de rasterização seja inócua. As imagens de escala
absoluta e overflow foram inspecionadas visualmente, sem realinhamento.

`after-logical-final` conserva 74 consultas e seus diagnósticos, 24 renders SVG
coincidentes e o erro esperado de dois eixos auto. `refuters-logical-final`
e `pdf-logical-final` conservam as divergências anteriores de baseline de Box,
RTL, Parent e empty-size/PDF. `margin-logical-width` confirma também a margem
grande, escala absoluta e reflexão; esta medição precede o ajuste de h(fr).

A primeira suíte desta etapa passou com 8563 testes, antes do ajuste fracionário
(`logical-width/workspace`). A suíte **final** passou com 8564 testes,
0 falhas e 7 ignorados, UTC 2026-10-14T19:20:26.811468+00:00 → 2026-10-14T19:27:29.817262+00:00.
`logical-width/final-workspace/workspace-{pre,result}` e workspace.log
registram comandos, ambiente isolado, HEAD, alterações exatas e hashes;
as fontes permaneceram inalteradas durante a execução.

Validação final em 2026-10-14T19:23:08.263189+00:00: build, rustdoc com -D warnings,
check de todos os alvos, fmt, linhagem estrita V1/V5/V15/V26, linhagem reversa
(970 consumers, zero erros) e diff check passaram. Proveniência e saídas em
`logical-width/validation-final`. Clippy mantém 181 avisos com os mesmos
códigos, mensagens e arquivos do inventário anterior; o comando foi inventário
com cap-lints warn, não CI estrito aprovado. Tekt mantém 380 warnings e
1662 notes, sem errors. `logical-width/warning-map.json` separa os avisos:
327 V16, 28 V17, 19 V12 e 6 V18; Clippy é um inventário distinto.

`logical-width/focused.patch` mostra somente esta etapa contra os bytes
arquivados anteriores em before-sources.json.gz. A frente X1–X5 e a meta geral
continuam abertas: baseline, RTL, Parent, finitude/PDF, efeitos da medição,
measure(width:), espaçamento negativo e os avisos ainda exigem correção.
As divergências justificadas do Tekt continuam preservadas. Não houve commit,
push, publicação ou release.
