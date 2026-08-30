# P1272 — certificar a conjunção completa dos quatro pares

**Estado:** EXECUTADO
**Predecessores:** P1269 sem dívida ou todas as materializações P1271
**Regime:** verificação segregada; sem escrita produtiva

## Objetivo

Reexecutar integralmente o envelope selado e decidir cada par de forma
independente.

## Gate pair-local

Cada um dos quatro pares exige simultaneamente:

- 24/24 fixtures válidas preservadas em todos os observáveis aplicáveis;
- stops originais, ordem, coincidência e right-continuity preservados;
- 24/24 dentro dos quatro budgets numéricos;
- grafo e raster local preservados;
- S20 stroke degenerado e S21 fill sem área conforme o contrato corrigido;
- todos os probes inválidos aplicáveis rejeitados;
- zero `Unknown` necessário;
- todos os mutantes válidos rejeitados, `mutation_score=1.0`;
- direto, inverso e repetido semanticamente idênticos;
- custo limitado pela fórmula por intervalo e integralmente publicado;
- zero regressão em sRGB, P1234, P1236, P1237 e P1264.

## Gates de repositório

`cargo build --workspace`, `cargo test --workspace`, formatos, verificadores
dos recibos, `git diff --check`, V1/V5/V15/V26 e `crystalline-lint .` com zero
violações.

## Veredito

Somente 24/24 produz `Generalization-Preserved` para o par. Um par não fecha
outro. O certificado limita a alegação ao envelope de 96 fixtures e não altera
o fallback nem autoriza promoção.

## Resultado

Os quatro pares fecharam independentemente em 24/24 como
`Generalization-Preserved`. A conjunção final registrou 96/96 fixtures,
384/384 métricas numéricas, 1.224/1.224 intervalos de custo, 28/28 probes
inválidos rejeitados, 192/192 recibos direto/inverso/repetido e 24/24 mutantes
rejeitados (`mutation_score=1.0`), com zero `Unknown` necessário.

S20 preservou o stroke degenerado e S21 preservou fill sem área. O fallback
produtivo permaneceu inalterado e a rota seguinte é P1273. Os gates de
repositório passaram com zero violações. Evidência completa em
`00_nucleo/diagnosticos/typst-p1272-generalization-certificate.md`.

**Atestação:** EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO.
