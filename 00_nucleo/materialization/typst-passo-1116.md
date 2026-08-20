# Materialização — Passo 1116: Diagnóstico da Secção 36 (Transformações Geométricas)

## Objetivo
Mapear as divergências de fluxo inline e matrizes afins de transformação na Secção 36 (`.typ/sec_36.typ`).

## Achados Técnicos
1. `transform::layout` força quebra de linha em elementos que deveriam permanecer inline.
2. Emissão de `FrameItem::Group` no exportador PDF requer paridade de orientação vertical na matriz `cm`.
