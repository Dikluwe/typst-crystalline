# L0 — text_measure
Hash do Código: 8983ce7b

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/text_measure.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidência anterior à decisão

A inspeção de `01_core/src/compiler/layout/text_measure.rs:85`, `word_layout.rs:89` e `space.rs:162` encontrou respectivamente fronteira textual, admissão de baseline e teste de integração de espaços iniciais em owners de outras responsabilidades. O estado exato, fontes e prompts anteriores estão em `00_nucleo/diagnosticos/separacao-tracking-2026-09-29/initial.json` e `before/`. A correção já medida encontra-se em `00_nucleo/diagnosticos/correcao-tracking-2026-09-29.md`; esta separação conserva seus resultados e limites. Posições, texto e estilos são observáveis da língua; localização das funções é mecânica. A inspeção não demonstra intenção histórica nem nova paridade bilateral.

## Contrato vigente

Possuir apenas default_text_width, word e space. default_text_width usa advance_shaped quando disponível, senão advance, e conserva a ordem da fórmula existente: largura base mais tracking resolvido vezes número de pares de codepoints Unicode. word delega a FontMetrics.text_width e respeita overrides injetados; space mede advance(" ") com tamanho/estilo ativos. Não consultar Layouter, decidir fronteiras, selecionar quebras, emitir itens ou solicitar métricas verticais. A fórmula de codepoints é fallback, não prova de seleção completa por clusters.

## Aceitação e limites

Preservar os testes causais existentes e o texto, posições, estilos, páginas e diagnósticos dos controles anteriores à separação. Ownership 1:1, pins válidos, build e lint sem erros. Movimentação interna ADR-0109/0127: funções estáticas descendentes, sem estado novo. As limitações de tracking em estilos mistos, RTL, seleção global e contagem de clusters permanecem as registradas no diagnóstico da correção.
