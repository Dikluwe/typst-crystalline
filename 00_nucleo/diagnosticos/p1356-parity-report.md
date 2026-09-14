# P1356 — medição executada, revisão focal 2

## Veredito

**EXPERIMENTO EXECUTADO; ALEGAÇÃO INTEGRAL AINDA NÃO DEMONSTRADA.** A revisão
2 corrigiu o delta, e o gate abstrato permanece `mutation_score = 1.0`
(16/16). O único vanilla é o pin ratificado `586e1bd43`; `a51e02804` aparece
somente como snapshot histórico não autoritativo para atribuição temporal.
S11 continua `HARNESS_GAP`, os onze sentinelas tentados não cobrem todos os
perfis congelados e alguns não materializam todos os subcasos/observáveis.
Isso limita a conclusão que pode ser extraída, mas não bloqueia a continuação
do trabalho nem exige decisão metodológica do dono. Execução **executado sem
atestação de isolamento**.

## Proveniência e custo

Executor `/root/verificador_p1356`; HEAD `8ceea20f92dff06b6ca5a34828135c0fd3e35248`; tree `c3c2fa834e6ec318690ddb88e181277ff2e25b27`; UTC `2026-09-12T18:49:25Z`. Harness medido `485385d29b47551b85a459175c4088ef612060d6b0c805d5fc37ab0c53297ee3`; gate R2 `1a7b09cb8b9c1c0d8118470320dacd12e1013368d9d91c1209b7c9b73f974e1b`; delta R2 `0a675f4a32dd1f11972c30f4d37c0da80dc42523687c8d36950442357ce4eb5c`; sentinelas R2 `eff91e88eadaa20fcb01d1edbaca047bec8eae95c4d1a440c40a0939f92c64a2`.

Após a medição, o dono corrigiu a autoridade: há um único vanilla, o pin
`586e1bd43`; `a51e02804` é somente snapshot histórico. Essa correção invalida
intencionalmente o certificado anterior, mas não apaga nem altera os payloads
medidos. Os hashes acima identificam a execução anterior à retificação; a
próxima execução parte da terminologia e política corrigidas.

A revisão foi focal: as três matrizes não foram reexecutadas e conservam os hashes anteriores. O gate permaneceu 16/16. O delta caiu de 19.368 atribuições para as 986 células realmente não-MATCH; falsos `RESOLVED_BY_PRODUCT` caíram de 18.382 para zero; entradas de catálogo passaram de zero para 124. A campanha sentinela produziu 38 observações laterais, duas incompletas de S11. Não há duração registrada nos recibos, portanto nenhum custo temporal é inferido.

## Delta corrigido

O delta R2 tem 492 `RETAINED_GAP`, 494 `TARGET_DRIFT`, 269 paths, catálogo 4.718→4.842, 124 entradas e zero saídas. A recomputação independente confirmou todas as premissas. Trinta candidatos históricos a resolução continuam inconclusivos porque P1353 não preserva classes por profile.

## Matrizes e determinismo

Cada par conserva 4.842 probes/19.368 células. Contagens e SHA canônico foram
recomputados; os quatro hashes por par continuam idênticos: cristalino–snapshot
histórico `a36ef4a399cbc6cd2348eac62b8ea682f3613fbbe550e77258b99ea801445092`,
cristalino–vanilla `8ff2aa84da07fedb3bb7bd0568fbe58160faad9ddb80cb9a7d81e284830d081a`,
snapshot histórico–vanilla
`f5771760dead8906b204481f7abdcf838a8d42c44200c907dc5f26c44f012544`.
A comparação normativa cristalino–vanilla conserva 986 células não-MATCH em
269 paths e zero `UNKNOWN`.

## Auditoria dos 12 sentinelas

O runner resume 10 `DIFFERENCE`, 1 `MATCH` e 1 `HARNESS_GAP`. A auditoria contra os oráculos congelados encontra cobertura parcial adicional:

| ID | Runner | Perfis exigidos ausentes | Lacuna |
|---|---|---|---|
| S01 | DIFFERENCE | default, html, a11y-extras | single tuple ended DIAGNOSTIC on both sides, so individual kind/alias tuple was not materialized |
| S02 | DIFFERENCE | a11y-extras | document-versus-CLI precedence was not executed |
| S03 | DIFFERENCE | a11y-extras | named args and incompatibility executed; required a11y-extras profile absent |
| S04 | DIFFERENCE | html, a11y-extras, html+a11y | combined tuple ended DIAGNOSTIC, so alpha boundary subexpressions after luma() are not independently observed |
| S05 | DIFFERENCE | html, a11y-extras, html+a11y | invalid-key diagnostic observed; three required profiles absent |
| S06 | DIFFERENCE | a11y-extras | composite LTR/RTL/empty/custom SVG witness executed; a11y-extras profile absent |
| S07 | DIFFERENCE | html+a11y | PDF observer hashes bbox and Tagged/Pages but does not expose tag morphology; html+a11y profile absent |
| S08 | DIFFERENCE | html, a11y-extras, html+a11y | one narrow SVG executed, not all pair × start/middle/boundary placements; three profiles absent |
| S09 | DIFFERENCE | html+a11y | author meta array observed; html+a11y profile absent |
| S10 | DIFFERENCE | html, a11y-extras, html+a11y | bare --pretty and bare --pdf-tagged forms absent; three profiles absent |
| S11 | HARNESS_GAP | default | explicit PARSER_UNSUPPORTED gap; no boundary vector executed |
| S12 | MATCH | html, a11y-extras, html+a11y | exact value/equality observed; three profiles absent |

Logo, S11 é o único `HARNESS_GAP` declarado pelo runner, mas não a única
limitação da alegação integral. Em particular, S02 não executa precedência
documento–CLI; S04 aborta a tupla no diagnóstico de `luma()` antes de provar
isoladamente os limites; S07 não observa morfologia de tags; S10 omite as
formas bare; e nenhuma família executada cobre todos os profiles exigidos.

## Inventário quatro-perfis

Existem inventários somente para vanilla/cristalino em `default` e `html`;
faltam `a11y-extras` e `html+a11y` em ambos. Agendamento das matrizes nesses
perfis não equivale à descoberta pública exigida pelo contrato. Esta é uma
tarefa concreta do próximo ensaio, não um pedido de direção ao dono.

## Política de continuidade

O gate controla **o que se pode afirmar**, não **se o trabalho pode continuar**.
O saldo de 4.842 rotas e as 986 células não-MATCH são medições válidas do
fragmento observado. A alegação de rebaseline integral fica pendente. O próximo
ensaio operacional é determinado: completar o inventário nos quatro perfis e
executar probes focais separados para S02, S04, S07, S10 e S11. Nenhuma escolha
de produto é necessária para iniciar esse trabalho.

## Lista integral dos 269 paths não-MATCH

- `accent`
- `asset`
- `bb`
- `bold`
- `cal`
- `calc.deg`
- `calc.log10`
- `calc.rad`
- `cancel`
- `color.spot`
- `color.spot.tint`
- `counter_at`
- `counter_display`
- `counter_final`
- `counter_step`
- `display`
- `enum.item`
- `figure.caption`
- `footnote.entry`
- `format`
- `format.bundle`
- `format.html`
- `format.html.a`
- `format.html.abbr`
- `format.html.address`
- `format.html.area`
- `format.html.article`
- `format.html.aside`
- `format.html.audio`
- `format.html.b`
- `format.html.base`
- `format.html.bdi`
- `format.html.bdo`
- `format.html.blockquote`
- `format.html.body`
- `format.html.br`
- `format.html.button`
- `format.html.canvas`
- `format.html.caption`
- `format.html.cite`
- `format.html.code`
- `format.html.col`
- `format.html.colgroup`
- `format.html.data`
- `format.html.datalist`
- `format.html.dd`
- `format.html.del`
- `format.html.details`
- `format.html.dfn`
- `format.html.dialog`
- `format.html.div`
- `format.html.dl`
- `format.html.dt`
- `format.html.elem`
- `format.html.em`
- `format.html.embed`
- `format.html.fieldset`
- `format.html.figcaption`
- `format.html.figure`
- `format.html.footer`
- `format.html.form`
- `format.html.frame`
- `format.html.h1`
- `format.html.h2`
- `format.html.h3`
- `format.html.h4`
- `format.html.h5`
- `format.html.h6`
- `format.html.head`
- `format.html.header`
- `format.html.hgroup`
- `format.html.hr`
- `format.html.html`
- `format.html.i`
- `format.html.iframe`
- `format.html.img`
- `format.html.input`
- `format.html.ins`
- `format.html.kbd`
- `format.html.label`
- `format.html.legend`
- `format.html.li`
- `format.html.link`
- `format.html.main`
- `format.html.map`
- `format.html.mark`
- `format.html.menu`
- `format.html.meta`
- `format.html.meter`
- `format.html.nav`
- `format.html.noscript`
- `format.html.object`
- `format.html.ol`
- `format.html.optgroup`
- `format.html.option`
- `format.html.output`
- `format.html.p`
- `format.html.picture`
- `format.html.pre`
- `format.html.progress`
- `format.html.q`
- `format.html.rp`
- `format.html.rt`
- `format.html.ruby`
- `format.html.s`
- `format.html.samp`
- `format.html.script`
- `format.html.search`
- `format.html.section`
- `format.html.select`
- `format.html.slot`
- `format.html.small`
- `format.html.source`
- `format.html.span`
- `format.html.strong`
- `format.html.style`
- `format.html.sub`
- `format.html.summary`
- `format.html.sup`
- `format.html.table`
- `format.html.tbody`
- `format.html.td`
- `format.html.template`
- `format.html.textarea`
- `format.html.tfoot`
- `format.html.th`
- `format.html.thead`
- `format.html.time`
- `format.html.title`
- `format.html.tr`
- `format.html.track`
- `format.html.u`
- `format.html.ul`
- `format.html.var`
- `format.html.video`
- `format.html.wbr`
- `format.pdf`
- `format.pdf.artifact`
- `format.pdf.attach`
- `format.png`
- `format.svg`
- `frak`
- `grid_cell`
- `grid_footer`
- `grid_header`
- `html`
- `html.area`
- `html.audio`
- `html.base`
- `html.blockquote`
- `html.canvas`
- `html.caption`
- `html.colgroup`
- `html.data`
- `html.del`
- `html.details`
- `html.dialog`
- `html.embed`
- `html.fieldset`
- `html.form`
- `html.frame`
- `html.hr`
- `html.img`
- `html.input`
- `html.ins`
- `html.label`
- `html.link`
- `html.map`
- `html.meta`
- `html.meter`
- `html.object`
- `html.optgroup`
- `html.option`
- `html.output`
- `html.progress`
- `html.q`
- `html.script`
- `html.slot`
- `html.source`
- `html.style`
- `html.table`
- `html.tbody`
- `html.td`
- `html.textarea`
- `html.tfoot`
- `html.th`
- `html.thead`
- `html.time`
- `html.tr`
- `html.track`
- `inline`
- `italic`
- `list.item`
- `lof`
- `lot`
- `math.Dif`
- `math.abs`
- `math.accent`
- `math.bold`
- `math.cal`
- `math.cases`
- `math.dif`
- `math.display`
- `math.frac`
- `math.italic`
- `math.liminf`
- `math.limits`
- `math.limsup`
- `math.lr`
- `math.mat`
- `math.mid`
- `math.norm`
- `math.overbrace`
- `math.overbracket`
- `math.overline`
- `math.overparen`
- `math.overshell`
- `math.primes`
- `math.quote`
- `math.quote.double`
- `math.quote.single`
- `math.registered`
- `math.root`
- `math.round`
- `math.sans`
- `math.scr`
- `math.sscript`
- `math.stretch`
- `math.text`
- `math.underbrace`
- `math.underbracket`
- `math.underparen`
- `math.undershell`
- `math.upright`
- `mono`
- `op`
- `outline.entry`
- `outline.entry.body`
- `outline.entry.indented`
- `outline.entry.inner`
- `outline.entry.page`
- `outline.entry.prefix`
- `par.line`
- `pdf`
- `polygon.regular`
- `raw.line`
- `replace`
- `sans`
- `scr`
- `script`
- `selector.after`
- `selector.before`
- `serif`
- `sscript`
- `state_at`
- `state_display`
- `state_final`
- `state_update`
- `state_update_with`
- `sym.quote`
- `sym.quote.double`
- `sym.quote.single`
- `sym.registered`
- `table_cell`
- `table_footer`
- `table_header`
- `terms.item`
- `underover`
- `upright`

## Fecho

Não selar nem fechar. Uma nova revisão focal precisa materializar S11, completar a matriz de profiles e os subcasos faltantes, e gerar os quatro inventários ausentes. Só então cabe repetir o recorte afetado antes de qualquer execução integral.
