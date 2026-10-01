# L0 — entities/color/mixing
Hash do Código: 817707c2

## Medição anterior à decisão

No HEAD ea7ae25ab (árvore limpa, 2026-09-30), o comportamento está em `entities/color.rs:391-529`. Trata-se de delimitação de código vigente; não há medição de desempenho nova.

## Owner e contrato

Camada L1. Consumer produtivo único: `01_core/src/entities/color/mixing.rs`. Converte componentes para vetores no espaço pedido e mistura duas ou N cores, preservando o caminho circular de hue, os pesos e a identidade de spot. Mantém as assinaturas públicas, defaults, fase do pipeline, compatibilidade e pureza L1.

## Aceitação

Testes L1, build e crystalline-lint passam. Sem inferência de ganho de desempenho.
