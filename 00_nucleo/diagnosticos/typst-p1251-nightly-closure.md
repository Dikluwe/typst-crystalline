# P1251 — fechamento saneado da sequência P1232–P1250

## Retificação causal

A versão anterior deste relatório congelou estados intermediários e tornou-se
obsoleta quando P1245–P1250 foram materializados e certificados. Os recibos de
falha permanecem históricos, mas não prevalecem sobre certificados aditivos
posteriores. Esta retificação não promove nenhum fragmento além do escopo dos
respectivos certificados.

## Estado terminal por passo

- P1232: diagnóstico aceito com lacunas de evidência; P1229 permaneceu reaberto.
- P1233: retificação diagnóstica aceita; nenhuma promoção automática.
- P1234: diagnóstico saneado; causalidade continuou desconhecida.
- P1235: rollback seguro dos pares sem evidência suficiente.
- P1236: diagnóstico do fragmento Luma/alpha aceito; divergências de luminância
  não foram perdoadas pela adjudicação de alpha.
- P1237: nenhum par adicional promovido; fronteiras numéricas preservadas.
- P1238: polar L1 preservado 6/6; SVG não promovido.
- P1239: luminância L1 preservada no fragmento certificado.
- P1240: harness raster retangular aceito apenas como auxiliar.
- P1241: consolidação diagnóstica aceita com classificações por par.
- P1242: inventário diagnóstico fechado; a fotografia antiga foi superada pelas
  materializações posteriores.
- P1243: preseal negativo retido; não autorizou promoção de paint multi-space.
- P1244: encerrado sem código e sem promoção.
- P1245: contrato público de tiling aprovado e materialização certificada via
  P1254, limitada aos targets e observáveis declarados.
- P1246: clip geométrico representável materializado; alpha mask e carrier
  even-odd ausente permanecem `Unknown`.
- P1247: destinos e links internos same-page materializados; cross-page/bundle e
  expansão pública de `link()` permanecem fora do escopo.
- P1248: raster e SVG aninhado autossuficiente materializados; formato opaco,
  dependência externa e SVGZ permanecem `Unknown`.
- P1249: glifo direto com identidade de fonte materializado; fonte ausente,
  conflito e wrapper sem fontes permanecem `Unknown`.
- P1250: campanha integrada fechada por P1250A + P1250B. Os 202 V16 restantes
  são exceções exatas ratificadas; V17, V18 e V21 estão zerados.

## Fronteira após P1250

O cluster SVG da fila P1213 continua `PARTIAL`, mas mudou de forma. Tiling,
clip geométrico, links same-page, imagens suportadas e glifos com contexto já
possuem certificados de fragmento e deixam de ser descritos como ausências
produtivas gerais. Continuam abertos paint multi-space não provado, links
cross-page, imagens opacas, wrappers sem fontes e a comparação bilateral ampla
contra o vanilla. PNG/PDF e os demais clusters P1213 mantêm os seus estados.

## Regime e isolamento

O P1251 é consolidação diagnóstica com verificação segregada de entradas e
veredito. Os certificados consumidos foram produzidos por papéis separados,
mas o filesystem e o contexto permanecem compartilhados. Portanto o resultado
correto é **EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO**, nunca equivalência geral.

## Execução

Os comandos, hashes, estado exato da árvore e veredito final estão registrados
em `00_nucleo/diagnosticos/p1251-final-certificate.tsv`.

Todos os gates exigidos passaram em 2026-08-28. A lente repetiu saída
byte-idêntica; o lint normal passou com warnings vigentes e o gate arquitetural
V1/V5/V15/V26 terminou sem violações. O veredito é
`PASS_DIAGNOSTIC_CLOSURE`, limitado à consolidação P1232–P1250 e aos fragmentos
já certificados.
