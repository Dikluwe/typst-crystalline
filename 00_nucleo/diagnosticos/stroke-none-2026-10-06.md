# Stroke:none nas formas fechadas

Corrigida a aceitação de ausência explícita de contorno em rect/square/ellipse/circle.
**16/16 controles bilaterais passam**: fill preto + None, sem fill + None,
omissão sem fill e fill preto com stroke omitido. O núcleo compartilhado
optional-stroke distingue explicitamente None de omissão; fallback preto não
é reativado quando o autor escolheu None. O parser de Stroke obrigatório e os
scope-outs de stroke por lado não foram alterados.

## Proveniência

Working tree não commitado, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, coleta UTC `2026-10-06T13:31:29.756899+00:00`.
Binário da coleta de formas SHA-256 `31a94ffac09b4f382162c3d9a60bb5f0d2d4705d1304c12f9435d255b6ed5591`;
vanilla pinado 9f2b6e8715237cb086899a42873660fe744622e8, SHA-256
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea` conforme P1489. before.json conserva fontes/L0
anteriores, stat/status e hash; execution.json.gz conserva os dois consumers/
owners, núcleo, hashes e logs finais. Macro e regressão de layout posteriores
usam binário `d448be2e5831441836dba19333c0d74989a64c16c6cb72bcdd17075fe7ac80b8`, com o mesmo código de construtores e
as correções HTML subsequentes. Fonte/comando/retorno/PDF/XML/raster por caso
em measurement.json.gz/measure.py; RAYON_NUM_THREADS=1, SOURCE_DATE_EPOCH=0,
NO_COLOR=1, tags false e pretty false. Não houve benchmark.

## Medição antes da decisão

A baseline macro rejeitava visual-rect/visual-circle por cast de none para
Stroke obrigatório. No pin atual, typst-layout/src/shapes.rs:125-129,335-339
separa Auto condicionado a fill de Custom(None). O Tekt confundia ausência
explícita com ausência de argumento, inclusive na decisão de fallback.
Dois testes de módulo (quatro construtores) ficaram RED antes da implementação,
conservando controles de omissão; red.log registra a falha comportamental.

L0 primeiro, claims compartilhadas no Núcleo Tekt pinado pelos dois owners,
leitura de None antes do cast e fallback condicionado à ausência de None
explícito. Nenhum campo/assinatura pública, fase, modo, dependência ou unsafe novo.

## Validação

**6.545 testes L1 passaram, 5 ignorados**; integração/doc-tests do pacote,
builds debug/release e diff-check passaram. Lint após esse pacote: 0 erros, 531 avisos, 1.579 notas, sem novo erro/aviso;
V15/V26 íntegros. Houve uma nova nota V19 de cobertura de alternativas e
atualização do texto de uma nota V20 de profundidade do padrão. HTML removeu
duas notas mecânicas; total posterior 1.577 notas. O L0 de show proposto mais tarde é pacote
não materializado, aguardando gate; seu drift não pertence ao reparo de formas.

Raster a 600dpi, limiar 180, correspondência bilateral ±1px, texto e caixas
<0,05pt, dimensões/páginas. Formas sem tinta têm máscaras vazias nos dois; não
se infere geometria interna de uma máscara vazia — ausência de Stroke também
é certificada no teste tipado. Todas as 16 sondas passam nos observáveis
explicitamente coletados. Regressões de layout após ambos reparos: **228/228**,
sem regressões, em layout-verification.json.gz/layout-summary.json.

A macro passou a aceitar e renderizar os dois controles anteriormente rejeitados.
Ver [matriz atual](paridade-dominios-2026-10-06.md). Auto explícito, strokes por
lado e superfície completa das formas não foram certificados por esta rodada.
