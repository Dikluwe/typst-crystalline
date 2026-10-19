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
- Largura lógica versus overflow/tinta e espaços finais: escala absoluta e flip
  horizontal de texto longo ainda divergem (`margin-after`). Fonte pinada
  flow/distribute.rs:650–651 limita o frame lógico à região; o conteúdo pode
  exceder a caixa sem redefinir seu tamanho. Não corrigir isso alterando só a tinta.
- Raster PNG de texto afim: render.rs:402–405 usa apenas sx/sy e ignora kx/ky;
  o pin typst-render/src/text.rs:53–91 usa contorno sob transformação completa.
  Reflow-wrap-rotate coincide em SVG e quase integralmente em PDF, mas PNG não.
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

Ainda faltam concluir estes refutadores, a integração completa X1–X5, inventário
final de avisos e checks amplos após a última correção. A frente não certifica
produção, paridade geral, CI remoto ou release; nenhuma publicação foi realizada.
