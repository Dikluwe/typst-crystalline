# Certificado P1279 — identidade de offsets do harness polar

## Veredito

`POLAR-GENERALIZATION-PRESERVED-DIAGNOSTIC-ONLY`.

O transporte lossless de offsets removeu os 10 falsos negativos de P1278. Os
seis pares Linear/Radial × Oklch/Hsl/Hsv fecharam 144/144 fixtures gerais sem
mudar algoritmo produtivo, L0, budget ou fronteira SVG.

| Par | P1278 válido | P1279 válido | Classificação P1279 |
|---|---:|---:|---|
| linear/oklch | 22/24 | 24/24 | Generalization-Preserved |
| radial/oklch | 22/24 | 24/24 | Generalization-Preserved |
| linear/hsl | 23/24 | 24/24 | Generalization-Preserved |
| radial/hsl | 23/24 | 24/24 | Generalization-Preserved |
| linear/hsv | 22/24 | 24/24 | Generalization-Preserved |
| radial/hsv | 22/24 | 24/24 | Generalization-Preserved |

## Causa medida

- stopsets lossless idênticos ao vanilla: 5/5;
- mutante `.15g`: 30 offsets divergentes;
- mutante pré-avaliação: 1 offset divergente;
- mutante via `f32`: 33 offsets divergentes;
- ataques P1279: 30/30 rejeitados.

Luma permaneceu controle negativo em 14/48. O focal público P1278 foi herdado
imutavelmente em 336/336 porque P1279 não
alterou código do produto.

## Fronteira produtiva

Os 192/192 grupos conservaram fallback e
`productive_promotions_applied = 0`. O certificado cobre somente o fragmento
executado; promover os seis pares muda comportamento por defeito e exige uma
etapa ADR-0127 explícita.

## Proveniência

- baseline: upstream/main `a51e02804` ratificado;
- HEAD medido: `a0fedd5a396fae995772ffb8bea8baf893842410` com working tree não commitado;
- medição: `2026-08-29T14:32:43-03:00`;
- resultados reproduzidos: `524faea0210ae18eb46178d77230f13ee0d5428fb414cff98e96dfdde17f87ef`;
- testemunhos de offset reproduzidos: `4fe5f2d221add5bfc9eba29879d229b8b88cccd578b0c443c93a2cdd625a0313`.

## Atestação

`EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO`. Houve separação lógica de entradas,
ordem e artefatos, mas os papéis compartilharam checkout e leitura.
