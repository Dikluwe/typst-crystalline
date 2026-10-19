# P1233 — retificar a autoridade e o orçamento P1229

**Estado:** EXECUTADO — RETIFICAÇÃO DIAGNÓSTICA ACEITA; P1229 REABERTO  
**Predecessor:** P1232  
**Saída:** contrato numérico único e predecessor P1229 ratificado ou reaberto.

## Objetivo

Resolver a contradição documental entre a aceitação P1229 e a reexecução R01.
Reconstruir o orçamento a partir do vanilla ratificado `a51e02804`, sem observar
um candidato novo, e definir explicitamente se fixtures adicionais são regressão
normativa ou expansão de escopo.

## Execução

Congelar um corpus mínimo comum contendo sRGB e as três promoções P1229, com
stops opacos, alpha em cada posição, coincidência e caixa larga. O oráculo deve
comparar `Gradient.sample(t)` vanilla com a função linear definida pelo SVG
emitido, em malhas 2048/4096, usando sRGB codificado premultiplicado e alpha
separado. Raster só pode ser gate após normalização geométrica local provada.

## Gate

Se P1229 não passar o novo contrato, marcar suas promoções `Unknown` antes de
qualquer tentativa de ampliar paridade. Se passar, publicar o witness exato que
refuta R01. Produzir contrato, budgets, oráculos e recibo Tekt segregados. Não
alterar código produtivo neste passo.

## Resultado

A reexecução saneada executou F01/F02 e o oráculo numérico a partir de uma
allowlist persistida contendo somente IDs; nenhum veredito anterior foi lido.
F01 encontrou zero tuples concretos idênticos entre os 10
registos reduzidos P1229 e os 30 controles R01. F02 mostrou que os 30 são
extensão de cobertura concreta **dentro do escopo normativo já exigido por
P1229**, não expansão de linguagem: cada tuple foi validado em
variante/espaço, papel, quantidade e ordem de stops, offsets, alpha e
geometria; caixa/aspect ratio foi classificada explicitamente como mecânica do
harness. Nenhuma das 30 linhas possui dimensão de escopo desconhecida.

As malhas 2048/4096 foram executadas duas vezes com outputs byte-idênticos,
sRGB codificado premultiplicado e alpha separado por `stop-opacity`. Houve 15
passes e 15 falhas: Linear/Oklab 6/6 falhas, Linear/LinearRgb 4/6 e Radial/Hsv
5/6; os 12 controles sRGB passaram. A retificação recebe certificado
`ACCEPTED_DIAGNOSTIC`, mantém P1229 reaberto e as três promoções `Unknown`.
O regime é reprodução diagnóstica, não protocolo Tekt completo atestado.
Nenhuma mudança de whitelist ou código produtivo foi autorizada.
