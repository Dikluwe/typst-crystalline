# Materialização — Passo 1114: Diagnóstico de Tamanho de Fonte Embutido e Métricas Verticais na Secção 35

## Objetivo
Mapear e diagnosticar as divergências de layout e métricas verticais em equações com tamanho de fonte alterado na Secção 35.

## Achados Técnicos
1. `layout_external` herda leading de parágrafo que infla `descent` em nós de texto de tamanho grande (e.g. 20pt).
2. Resolução de avanço em `item_width` para sub-frames matemáticos requer preservação da tabela de métricas de `New Computer Modern Math`.
