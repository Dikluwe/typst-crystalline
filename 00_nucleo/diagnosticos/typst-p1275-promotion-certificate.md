# P1275 — certificado da promoção produtiva SVG

**Veredito:** `PROMOTED-PRESERVED`
**Atestação:** `EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO`
**Baseline commitado:** `b989c95a5ceaf7300904594b690f70f9d986f4e0`
**Estado medido:** working tree não commitado, pinado pelo preseal
`9d7e791769d303c364a5474e71eb24b09b38b7e279c854551b9d88ac8595f96a`

## Veredito por par

Os quatro pares autorizados fecharam todos os gates e recebem, individualmente,
o veredito `PROMOTED-PRESERVED`:

- Linear/Oklab: 24/24;
- Radial/Oklab: 24/24;
- Linear/LinearRgb: 24/24;
- Radial/LinearRgb: 24/24.

Nos 96 SVGs produzidos por `target/debug/typst`, cada fixture continha servidor
Linear/Radial nativo, referência local resolvida e zero marcador de fallback.
O binário testado tem SHA-256
`522c0bde4dec1a8056a5aa5c2875af349b5e6fc249b2b8e054e0504294f776c8`.

## Fronteira não aprovada

Linear/Radial × Hsv/Oklch/Hsl/Luma/CMYK foi executado como 10 fixtures
produtivas adicionais. Os 10/10 casos conservaram exatamente um marcador
`gradient-color-space` e zero servidores Linear/Radial. Nenhum par foi
promovido por similaridade.

## Gates repetidos

- fixtures e gates G01–G13: 96/96;
- métricas numéricas: 384/384;
- intervalos/custo: 1.224/1.224;
- grafo: 96/96;
- raster adjudicado por G06/G07A/G07B: 96/96;
- domínio inválido: 28/28 rejeitado;
- determinismo direto/inverso/repetição: 192/192;
- mutantes válidos: 24/24 rejeitados (`mutation_score=1.0`);
- opacos: 10/10 excluídos do sucesso;
- controles focais atuais: 14/14;
- workspace, build, formato, sintaxe, diff e gates Tekt: PASS.

O raster bruto conserva quatro linhas S20 com o status histórico `Violated`
por esperar área zero. P1275 não apaga esse dado: `p1275-raster.tsv` registra
`raw_status=Violated` e `status=Preserved` separadamente, pois o contrato
corrigido G07A exige e encontrou suporte positivo nos quatro casos.

## Proveniência e integridade

L0 `6a8c1ad3…`, consumer `3ce7bd9a…`, manifesto P1274 `b53db8b3…`, manifesto
P1272 `0c711546…` e certificado P1272 `d07786a8…` permaneceram byte a byte.
Ferramentas: `rustc 1.92.0`, `cargo 1.92.0`; candidato `typst 0.15.1
(b989c95a)`. A versão é acompanhada por HEAD, estado da árvore e hash do
binário, não usada isoladamente como proveniência.

A primeira tentativa foi descartada antes do certificado porque o runner
sobrescreveu o arquivo incorretamente classificado como preseal. A cadeia
válida separou o preseal imutável do snapshot gravável e repetiu integralmente
o envelope, os ataques, a fronteira negativa e os gates.

## Limitação

Esta autoridade compartilhou contexto e filesystem com a implementação P1274;
portanto, o certificado não alega segregação atestada. O veredito cobre somente
o envelope de 96 fixtures e a fronteira de 10 pares não aprovados. Não prova
equivalência SVG geral, não promove Conic/Tiling e não converte `Unknown` em
sucesso fora do fragmento explicitamente certificado.
