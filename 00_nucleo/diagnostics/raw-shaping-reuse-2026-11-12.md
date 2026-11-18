# Reúso bruto de shaping integrado

Medição e emissão agora compartilham resultados brutos imutáveis somente dentro da compilação, por entrada completa e owner de fonte retido. Tracking, escala, posições, seleção e fallbacks mantêm seus contratos; None continua distinto de sucesso vazio. L0 primeiro, Nucleus compartilhado com owners 1:1 e regressões.

No lab de 100 parágrafos, sobre a mesma base experimental de sessão retida: CPU 22,39 → 19,36 ms, Δ pareado −12,75% nos seis pares; alocações −10,46%, tráfego −15,24%. Há custo: pico solicitado +64.501 bytes e RSS final no stress grande→pequeno aproximadamente +2,11 MiB. Esse ganho é incremental na base do lab, não uma comparação absoluta do produto entregue contra vanilla.

Somente o reúso bruto foi integrado; a sessão eval/fontes retidas continua experimental. A divergência semântica do outline permanece aberta, e o residual do watch não está encerrado.

Validação integrada: build release, 105 testes de shaping, 51 de métricas, 12 fixtures equivalentes e lint sem violações. Três mutantes refutados no lab.

Evidências, limites, proveniência e validação final: [relatório](../../lab/results/construction/diagnostics/raw-shaping-reuse-2026-11-12/relatorio.md).
