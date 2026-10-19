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

Arquivo recuperável: `logical-width/evidence.json.gz`, 1465 entradas
com path/SHA-256/base64, reabertas e comparadas. SHA-256 `8bd73f4e4c9cc346f7994ddc17008c9aa0739fc680c731fa4acf8dcdc2684379`.


## Alinhamento RTL — investigação posterior

`rtl-study/review.md` reúne 59 controles de direção, alinhamento, irmãos,
região automática, porcentagens e texto multilinha. A medição refutou a
correção por deslocamento global: Start/End depende da direção efetiva de
cada filho, e cada frame deve ser alinhado por sua largura lógica.
Também identificou Ratio/Relative descartados na dimensão de Shape e um
erro vertical independente após transformações em página automática.
Código e L0 permanecem iguais à etapa validada de 8564 testes, comprovado
por rtl-study/source-guard.json. A correção RTL ainda não está implementada.

Evidência RTL recuperável: `rtl-study/evidence.json.gz`, SHA-256 `84f6cee5b206c8e4b94fb3afd49f0798f7df1d3158ee30d9cef86a38c5c6c318`, 470 entradas reabertas e comparadas.


## Dimensões relativas de formas — correção posterior

`shape-dimensions/review.md` registra a correção de porcentagens, em e
comprimentos mistos em Shape, a região real de Block/Subframe e a largura
interna usada pelas transformações. Quatro regressões RED→GREEN passaram.
A suíte completa terminou com 8568 testes passados, 0 falhas
e 7 ignorados; UTC 2026-10-14T19:46:40.433739+00:00 → 2026-10-14T19:54:04.752060+00:00.
Build/doc/check/fmt/linhagem passaram. Dimensões automáticas, identidade de
círculo/quadrado, altura Auto de página, contorno de elipse e RTL continuam
pendentes. Não confundir esta correção com conclusão de X1–X5 ou CI/release.

Evidência recuperável: `shape-dimensions/evidence.json.gz`, SHA-256 `0261924e3959628722428bdea7b15bcf3772f08dba06344034168d66a63184e7`, 878 entradas reabertas e comparadas.


## Fronteira vertical de Shape — avanço com pendências explícitas

`shape-flow/review.md` registra quatro regressões RED→GREEN, posição/fim
causal das formas e manutenção de espaço forte. Suíte: 8570
aprovados, duas falhas de referências deliberadamente alteradas, 7
ignorados. Somente 04/07 foram regeneradas após comparação independente;
os nove snapshots passaram depois sem UPDATE. Verificações de linhagem,
build/doc/check/fmt passaram. Block/inset/altura e espaços fracos continuam
refutados; a meta geral e CI/release não estão concluídos.

Arquivo recuperável: `shape-flow/evidence.json.gz`, SHA-256
`0895100c1b4572ffa84c45ed138c0a8d86f97f953ecfe9ba0c874e68b5c3d14e`, 1178 entradas reabertas e comparadas.


## Continuação Block — fronteiras — 2026-10-14T20:29Z

O [relatório de Block](transformacoes-2026-10-14/block-height/review.md)
registra alocação, inset/outset, origem após pagebreak, distribuição
fracionária e preservação dos floats. No estado HEAD06848cdcb com árvore
não commitada, a rodada de layout20:27:27Z–20:28:05Z tem1198 aprovados,
2 falhas e3 ignorados. Build/fmt/linhagem estrita e reversa passaram.
As falhas restantes são Align vertical e prefixo18pt isolado; a suíte
completa e CI/release não estão certificados. A comparação PDF final
preserva todos os marcadores do checkpoint; os deltas pagebreak SVG/PNG
anteriores eram comparação de páginas distintas e não contam como prova.
Arquivo de evidência SHA-256
`0ed0d0364c8b70b9d1f97bdae216317fc9f7da072d711e0de3908da80ac841ba`,2321 entradas verificadas.


## Continuação Align — 2026-10-14T20:41Z

[Relatório de consumo vertical de Align](transformacoes-2026-10-14/align-flow/review.md):
no HEAD06848cdcb, working tree não commitado, a rodada de layout
20:37:38Z–20:38:22Z tem 1.204 aprovados, uma falha P1381 e três ignorados.
Build/fmt/linhagem estrita e reversa passaram; linter amplo conserva zero
erros, 380 avisos e 1.662 notas. Cinco regressões novas passaram após RED.
A medição final contém 48 documentos PDF (43 coincidem integralmente em
páginas/bboxes) e 42 documentos SVG/PNG de página única (39 SVGs coincidem).
Continuam abertos Align Place-only, fechamento textual antes de pagebreak,
escopo de par.spacing e Block ignorando valores explícitos. Suíte completa,
Clippy e CI/release não estão aprovados. Evidência com 1.144 entradas,
SHA-256 `f7d708cf57c7fa6988fb8e45f972efba052bc3963c06fe9322f5898e71440ba0`.


## Continuação margens de Block — 2026-10-14T20:55Z

[Relatório de margens de Block](transformacoes-2026-10-14/block-par-spacing/review.md)
registra correção de par.spacing ignorado, reserva de dois Blocks1fr e
baseline de texto após margem zero. HEAD06848cdcb, working tree não commitado;
proveniência completa no arquivo. Três testes RED→GREEN; layout20:51:56Z–20:52:31Z
com1.207 aprovados, uma falha P1381 preexistente e três ignorados. Build/fmt,
linhagem estrita e reversa passaram; linter amplo zero erros,380 avisos,1.664 notas.
PDF final23/24 documentos coincidentes, incluindo todos os18 controles de margem;
três controles adicionais de dois Blocks1fr corrigidos. SVG23 idênticos; PNG17
idênticos e seis com diferença de um nível em cinco pixels, sem mudança geométrica.
O wrapper18 único mantém delta9.6pt; cauda1fr, margens negativas e demais frentes
continuam abertas. Suíte completa, Clippy e CI/release não certificados nesta etapa.
Evidência537 entradas reabertas e conferidas, SHA-256
`714e0867a3006275321380f83ff9e7093124a6b4b9fd063d1cf97511f31676dd`.


## Encerramento — escopo textual e cauda após 1fr — 2026-10-14T21:38Z

[Relatório desta etapa](transformacoes-2026-10-14/scope-tail/review.md):
fechamento sob escopo lexical compartilhado, altura causal do prefixo e
reserva da cauda após 1fr, incluindo limite exato e primeiro fragmento com
viúvas/órfãs. Seis owners L0/L1 atualizados; quatro regressões adicionadas
e duas fortalecidas. HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree
não commitado, com stat/status/hashes e horas UTC nos snapshots.

Suíte final 21:31:46Z–21:37:41Z: **8.590 aprovados, zero falhas, sete ignorados**,
fontes/L0 estáveis. Build, rustdoc estrito, check, fmt, linhagem V1/V5/V15/V26 e
970 hashes reversos passaram. Tekt: zero erros, 380 avisos, 1.668 notas;
inventário Clippy: 181 avisos existentes, sem delta por classe. Os dois
avisos novos da implementação foram removidos. O gate Clippy `-D warnings`
e a certificação global de CI/release não estão aprovados.

Replay PDF final 21:30:14Z–21:30:43Z: 28/28 documentos coincidentes; controles
anteriores à limpeza equivalente: 24/24 margens e 11/11 bordas/viúvas/órfãs.
Raster da mesma implementação: 26 SVGs idênticos; 17 PNGs idênticos e nove
com delta máximo de um nível por canal. As medições são conjuntos separados.
P1381 e P1383 agora passam; permanecem dois prefixos multiline opacos e um
aninhamento parcial, documentados com refutadores. Não houve release/push.

Evidência reaberta e conferida: 1379 entradas, SHA-256
`69232b98b9105449e4afcc8e0e5a94d87b259d84abc35f127b28564852247619`. Etapa encerrada; demais frentes permanecem pendentes.


## Encerramento — três casos de prefixo/aninhamento — 2026-10-14T21:08-03:00

[Relatório desta etapa](transformacoes-2026-10-14/prefix-nesting/review.md)
registra os três refutadores corrigidos e a margem automática do texto após
Block aninhado. Dez consumers/owners e um Núcleo compartilhado foram atualizados,
sem contrato público ou fase nova. HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`,
working tree não commitado, com snapshots/status/stat/UTC/hashes no dossier.

Suíte final UTC 23:54:39 de 14/10–00:02:43 de 15/10: **8.593 aprovados, zero falhas,
sete ignorados**, fontes/L0 estáveis, um job Cargo e cgroup de 12 GiB. As três
falhas da primeira rodada eram ausência do rsvg-convert no PATH do serviço;
a rodada final preserva o PATH. Build/check/fmt/rustdoc estrito, linhagem
V1/V5/V15/V26 e 970 hashes reversos passaram. Tekt conserva zero erros,
380 avisos e 1.668 notas; Clippy 181 avisos anteriores, sem delta por classe.
O gate Clippy -D warnings e a certificação global de CI/release seguem pendentes.

PDF final 33/33 casos/extensões, 28/28 controles de escopo/cauda, 24/24 margens,
11/11 bordas/viúvas/órfãs; conjuntos separados, não declarados disjuntos.
Raster final 33 SVGs idênticos; 14 PNGs idênticos e 19 com delta máximo de um
nível por canal. Fontes, argv, horários e binários estão nos resultados.

O kernel confirmou três OOM globais de rustdoc com 43–44 GiB. A reprodução
limitada isolou o crescimento no índice antigo target/doc/search.index:
mesma fonte em saída vazia passou; copiar só o índice reproduziu a falha.
Artefatos/fingerprints anteriores foram preservados em quarentena. A regeneração
integral passou com pico observado 0,76 GiB; a revalidação após a suíte passou
com 1,06 GiB e quatro páginas HTML recompostas. Nenhuma fonte ou API foi alterada
por esse reparo de artefatos. MemoryMax de 12 GiB/MemorySwapMax de zero foram confirmados;
o pico total amostrado de 11,72 GiB incluía principalmente cache de arquivos,
e a amostra detalhada do job tinha zero OOM. O diagnóstico está em memory-incident.md.

Evidência reaberta e conferida: 3068 entradas, SHA-256
`09d077992b9bb7d3409c221d5d56fe62a7ab161e88f1576a1b752caf1612c8d9`. Etapa encerrada; não houve commit/push/release.
