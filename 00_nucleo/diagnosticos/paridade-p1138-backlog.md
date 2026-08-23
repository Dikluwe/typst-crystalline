# Backlog derivado da baseline P1138

## Ordem recomendada

1. **P1139 — paridade de diagnósticos públicos.** Fechar `P1138-S-001..003`,
   incluindo trace cross-file. É contrato público e exigirá L0 + gate
   ADR-0127 antes de código.
2. **P1140 — inventário de superfície da linguagem.** Extrair metadados de
   globais/módulos/callables/args/defaults e validá-los com probes observáveis.
   A ausência de reflexão `params()` impede fingir um inventário puramente
   runtime.
3. **P1141 — decomposição da divergência SVG.** Separar agrupamento e
   morfologia observável de whitespace numérico, IDs e serialização de paths.
4. **P1142 — auditoria de unidades e arredondamento.** Investigar a largura
   595,276 vs 595,280 pt e decidir, pela fonte da fórmula, língua versus
   mecânica. O que refuta bug de língua: ambos representarem exatamente A4 e a
   diferença surgir apenas na precisão textual de exportação.
5. **P1143 — raster diferencial.** Localizar os 168 pixels de delta 1 e
   relacioná-los a geometria, antialiasing ou arredondamento. Não elevar a
   tolerância antes dessa medição.
6. **P1144 — observáveis PDF.** Separar diferença real de caixa da diferença
   mecânica no nome subset/encoding da fonte; preservar igualdade de texto.
7. **P1145 — expansão HTML.** Depois dos itens de linguagem, ampliar a árvore
   semântica para heading, strong, emph, linebreak, escape e erros explícitos
   das variantes ainda fora do subconjunto.

Cada passo começa pela leitura do L0 correspondente. Correções internas de
paridade seguem fluxo contínuo; contrato/default/pipeline/compatibilidade para
no gate humano da ADR-0127.
