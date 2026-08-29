# P1262 — corrigir os oito máximos interiores saturados

**Estado:** EXECUTADO — OITO WITNESSES FECHADOS, PROMOÇÃO NÃO APLICADA  
**Predecessores:** P1257 e P1260  
**Owners candidatos:** `entities/color`, `export/gradients/adaptive`,
`infra/export/svg`.

## Escopo congelado

Oito `color_max` interiores cujo sample público vanilla possui ao menos um
canal `00/ff`. Saturação observada não prova que a cor fonte estava fora do
gamut; a causa deve ser localizada antes de código.

## Matriz causal RED

Para cada witness, registrar no mesmo `t`:

1. componentes nativos do espaço de interpolação;
2. conversão para sRGB codificado em float, antes de clamp;
3. RGB premultiplicado usado pelo critério adaptativo;
4. clamp/ties-to-even para u8;
5. `#RRGGBB` + `stop-opacity` e reparse;
6. comparação com as mesmas fronteiras no vanilla ratificado.

Cada fronteira recebe `Measured`, `Violated` ou `Unknown`. Não atribuir owner
atravessando uma fronteira `Unknown`.

## Decisão por owner

- primeira divergência em componentes/conversão → atualizar primeiro
  `00_nucleo/prompts/entities/color.md` e corrigir L1;
- primeira divergência na escolha de stops → atualizar `adaptive.md` e corrigir
  L3;
- primeira divergência em clamp, quantização ou XML → atualizar
  `infra/export/svg.md` e corrigir o exporter;
- se a equivalência exigir uma nova política pública de gamut, parar no gate
  ADR-0127 para aprovação humana; não inventar gamut mapping por default.

## Ataques obrigatórios

- clamp precoce usado para reduzir artificialmente erro;
- comparação apenas depois de u8;
- canal saturado tratado automaticamente como sucesso;
- Oklab corrigido por coerção silenciosa a LinearRgb/sRGB;
- alteração global de `Color::to_srgb` sem regressão dos demais consumers;
- budget alargado ou fixture saturada descartada.

## Fechamento

Os oito witnesses passam máximos e p95 de cor, alpha não regride, e qualquer
mudança em `Color` passa todos os consumers L1/L3. P1234 deve continuar com
zero divergência pública nos probes já fechados. Sem causa completa, o passo
fecha apenas como diagnóstico e não autoriza promoção.

`EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO` se aplicável.

## Resultado

Implementação e recibos: `00_nucleo/diagnosticos/typst-p1262-saturated-interior.md`.
Veredito: **EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO**.
