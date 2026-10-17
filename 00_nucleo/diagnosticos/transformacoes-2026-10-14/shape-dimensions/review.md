# Dimensões relativas de formas — correção interna

Estado: correção de Length/Ratio/Relative materializada e validada. None/Auto,
identidade circle/square, casts, altura automática de página e RTL continuam
pendentes; este resultado não certifica todas as formas nem produção/release.

## Proveniência e decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado.
Cada resultado citado inclui UTC, status/diffstat exatos e hashes das fontes e
binários. Pin vanilla 9f2b6e8715237cb086899a42873660fe744622e8, receita/árvore
P1489, binário SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea.
Os bytes anteriores dos owners estão em before-sources.json.gz; focused.patch
isola esta etapa do restante da árvore suja.

shape-dim-before-complete/results.json.gz registra 20 documentos bilaterais,
UTC 2026-10-14T19:38:59.904545+00:00 → 2026-10-14T19:39:15.839480+00:00.
helpers.rs:427–438 descartava Ratio/Relative e a parte em de Length. O L0
herdado não especificava sua resolução completa. Shape usa agora um resolvedor
privado no próprio owner, mantendo unidades até layout. A parcela percentual
usa o eixo regional; a parcela absoluta usa a fonte efetiva. Produto relativo
não finito contribui zero, como medido, sem perder a parcela absoluta.

Block instala/restaura relative_region_size com largura interna e altura de
referência após inset. Subframe herda a região real em vez do marcador infinito
de altura Auto. Transform nativo também usa a largura interna de Block para
medir/compor, evitando que a margem física da raiz inclua parte do inset.
Nenhuma API pública, campo público ou fase nova; L0 atualizado primeiro,
resselo restrito nos cinco consumers e seus owners.

## Testes e refutadores

Quatro regressões falharam antes da implementação (implementation/shape-dim-red).
A primeira tentativa passou duas, mas Block media 37.5 em vez de 35; a seguinte
passou três, deixando o mesmo erro em Rotate dentro de Block. A largura interna
foi aplicada nesses dois consumidores. implementation/shape-dim-green-third
passou as quatro regressões, incluindo estilo do uso, Box explícito/auto/aninhado,
Block explícito/auto com inset, restauração do pai e região ilimitada.

shape-dim-after e shape-dim-pdf-complete recompilam os 20 controles. Os retângulos
com dimensões explícitas e as regiões aninhadas coincidem nos três formatos.
A imagem de Block→Rotate→Rect foi inspecionada sem realinhamento.
A primeira tentativa before abortou ao rasterizar SVG inválido de unbounded;
a primeira rodada PDF after abortou por tamanho de imagem diferente. Ambas
são tentativas incompletas; as rodadas complete registram esses resultados
como falhas/diferenças, sem interromper nem tratá-los como aprovação.

O controle unbounded passou de SVG inválido para forma 10×10, mas a página
continua maior que a referência: o fluxo inclui margem vertical final indevida.
shape-dim-controls compara forma absoluta 10pt com relativa 50%+10pt: cada binário
produz os mesmos pixels dentro do próprio par, embora Tekt e pin difiram na
página. Os controles de elipse 102×51 também são idênticos dentro de cada binário,
mas ainda divergem entre eles; a resolução dimensional não é a causa dessa
diferença. equivalent-controls.json conserva hashes e comparação exata dos
pares. Não presumir que toda diferença de contorno seja defeito do Tekt.
Move percentual em Block Auto com inset coincide em SVG/PNG após a correção.

Defaults de rect/ellipse/circle e dimensões Auto continuam incorretos e foram
preservados como refutadores abertos. O passo de continuidade está em
00_nucleo/materialization/shape-dimensoes-automaticas.md; não é L0. Dar o mesmo
default a circle e ellipse perderia a identidade, que o carrier atual não separa.
O helper legado de estimativa continua com seus limites; não foi ampliado por
inferência nem usado para certificar os resultados do layout real.

A rodada after-shape-dim conserva 74 consultas e diagnósticos, 24 renders SVG
coincidentes e o erro esperado de dois eixos auto. refuters-shape-dim conserva
as mesmas métricas do corpus anterior. rtl-mixed-shape-dim passa os controles
percentuais LTR; os defeitos de alinhamento RTL permanecem, agora sem a
compensação acidental de uma forma com largura errada.

## Validação final e limites

Suíte completa: 8568 testes aprovados, 0 falhas,
7 ignorados; UTC 2026-10-14T19:46:40.433739+00:00 → 2026-10-14T19:54:04.752060+00:00.
workspace/workspace-pre.json.gz, workspace-result.json e workspace.log
preservam comandos, ambiente isolado e guard das fontes inalteradas.
Build, rustdoc com -D warnings, check de todos os alvos, fmt, linhagem estrita
V1/V5/V15/V26, linhagem reversa e diff check passaram; evidência em validation/.
Clippy registra 181 avisos no inventário com cap-lints warn,
não um CI estrito verde. Tekt registra 380 warnings,
1662 notes e 0 errors.
A meta geral continua aberta; não houve commit, push, publicação ou release.
