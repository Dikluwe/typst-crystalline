# Prompt L0 — `compiler/eval/tests`
Hash do Código: 2e820a22

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/core.toml sha256:e7642a709c937928333439b2a78cdb3a6dbd6b56d67fcc67728efd2a26796e58

**Camada:** L1 test-only
**Ficheiro alvo:** `01_core/src/compiler/eval/tests.rs`

## Medição e contrato

Fornece Worlds puros, helpers test-only e regressões linguísticas do eval.
Fixtures não usam filesystem real. Asserções observam semântica, sintaxe,
morfologia e mensagens, não mecânica Rust incidental.

## Aceitação

Regressões têm controles e proveniência do vanilla quando decidem paridade.

P1250B retifica o oracle P744 depois de P1253: as constantes públicas
nomeadas preservam literais `f32`, portanto `red.mix(blue, space: rgb)` expõe
`#805b87` no vanilla ratificado. O valor `#805a88` pertence ao caso distinto
construído por canais inteiros `rgb(255,65,54).mix(rgb(0,116,217), ...)`.

P1252 protege pela superfície pública alpha em `luma(l, alpha: alpha)`,
`luma(color)` e na normalização de stops Linear/Radial/Conic com
`space: luma`. O teste exige alpha preservado nos três variants e não afirma
paridade de luminância nem promoção SVG.

P1239 fecha a luminância Luma na superfície pública: `luma(red)` e os
endpoints vermelhos de `gradient.linear(..., space: luma)` e
`gradient.radial(..., space: luma)` devem expor `54.02%`, como o vanilla
ratificado. O teste continua a exigir alpha preservado segundo P1252 e não
autoriza promoção SVG.

P1269-owner fixa a fronteira estrutural de stops Linear coincidentes em
offsets não diádicos: em Oklab e Linear RGB, `sample` e `samples` recebem a
razão pública devolvida por `stops()`; a coincidência exata escolhe o primeiro
stop daquele offset e um epsilon positivo escolhe o ramo à direita. O controle
usa `sharp(3)` para exercer `1/3` e `2/3` sem depender da representação textual
decimal.

P1271-C2 protege a morfologia pública de uma cor Luma normalizada para Oklab.
O stop `white` de um gradient Oklab deve expor os mesmos quatro componentes do
vanilla ratificado — inclusive os pequenos canais `a` e `b` não nulos — e o
controle sRGB vermelho continua no caminho P1253. O teste não observa a
estrutura das matrizes nem autoriza qualquer alteração de budget SVG.

P1271-C3 exige que `repeat(2)` preserve a resolução `f64` dos offsets
automáticos. Linear, Radial e Conic com quatro stops devem expor exatamente
`0, 1/6, 1/3, 1/2, 1/2, 2/3, 5/6, 1`; `sharp` é scope-out deste teste.

P1271-C4 fixa o sampling Radial em `37.123456789%`: Oklab `black/white` e
Linear RGB `red/blue` devem expor os componentes medidos no vanilla. O
controle nos endpoints continua idêntico; Conic e outros espaços são
scope-out, assim como qualquer observável SVG/PDF.

## P1215

Os testes de `eval_expression` reconstroem a `Source` code com o mesmo
`FileId` e exigem ranges exatos para chamada inteira, positional, named e
deslocamento por linhas. Mensagem sem range resolvível não satisfaz a prova.
