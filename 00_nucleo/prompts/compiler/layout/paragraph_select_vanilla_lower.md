# L0 — paragraph_select_vanilla_lower
Hash do Código: d98bbc00

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/paragraph_select_bounded/vanilla_lower.rs`
**Architecture:** ADR-0107/0108/0109/0127/0129/0130.

## Medição anterior à decisão

`lab/typst-original/crates/typst-layout/src/inline/linebreak.rs:264-344` usa um limite inferior para linhas subcheias após obter limite superior válido. No protótipo conjunto anterior, `lab/results/construction/diagnostics/paragraph-select-podas-vanilla-justice-2026-10-02.md` mediu 1.000 palavras/240pt: 9.695 avaliações e 195,470 µs, contra 120,770 µs da DP exata. A medição é sintética e não atribui intenção ao vanilla.

## Decisão experimental

Este consumer privado e compilado só em testes possui o percurso crescente dos inícios para cada fim. Recebe do módulo pai as medidas prefixadas, custo exato, limite superior Simple, fronteira ativa e vetores da DP. Após a primeira linha subcheia não final, seu custo pode servir de limite inferior para inícios posteriores quando palavras, gaps e tracking têm contribuições finitas não negativas. Descartar um predecessor somente se seu custo acumulado mais esse limite exceder estritamente o limite superior. Preservar atualização da fronteira ativa, empate `<=` a favor do início posterior e fallback do pai. Sem alterar função de custo, seleção produtiva, default, API ou fase. A bancada proprietária do pai compara ranges com a DP exata e mede visitas, podas e tempo.
