# L0 — paragraph_select_justice_overflow
Hash do Código: 3f45af6e

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/paragraph_select_bounded/justice_overflow.rs`
**Architecture:** ADR-0107/0108/0109/0127/0129/0130.

## Medição anterior à decisão

O código de `kitlangton/justice` no commit `86dbc6da8cc2059cabb2a67c0aa42d773d5023e2`, `src/engine.ts:340-400`, percorre predecessores em ordem inversa e interrompe após transbordamento sob guarda de monotonicidade. No protótipo conjunto anterior, `00_nucleo/diagnosticos/paragraph-select-podas-vanilla-justice-2026-10-02.md` mediu 1.000 palavras/240pt: 210,360 µs com limite superior Simple e 196,900 µs sem semente, contra 120,770 e 122,730 µs das respectivas DPs exatas. A função de custo e os estados do Justice diferem; isto é um decalque da regra de busca, não um porte integral.

## Decisão experimental

Este consumer privado e compilado só em testes possui o percurso decrescente dos inícios para cada fim. Recebe do módulo pai medidas prefixadas, custo exato, fronteira ativa e vetores da DP. Mede separadamente a opção com limite superior Simple e sem semente. Quando palavras, gaps e tracking têm contribuições finitas não negativas, uma linha overfull prova que as linhas com início ainda anterior permanecem overfull; interromper somente se o custo mínimo dessas linhas não puder melhorar o melhor custo deste fim. Na última linha, excluir a penalidade `runt` da estimativa de linhas anteriores ao início de uma palavra. Em empates preservar o início posterior pela comparação estrita `<`. Depois da busca, atualizar a fronteira ativa como no seletor produtivo. Em caso de dúvida não interromper. Sem alterar custo, seleção produtiva, default, API ou fase. A bancada do pai compara ranges completos e mede visitas, podas e tempo.
