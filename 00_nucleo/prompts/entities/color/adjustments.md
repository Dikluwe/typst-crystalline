# L0 — entities/color/adjustments
Hash do Código: 11e5d8ef

## Medição anterior à decisão

No HEAD ea7ae25ab (árvore limpa, 2026-09-30), o comportamento está em `entities/color.rs:325-390,530-608,619-666`. Trata-se de delimitação de código vigente; não há medição de desempenho nova.

## Owner e contrato

Camada L1. Consumer produtivo único: `01_core/src/entities/color/adjustments.rs`. Aplica clareamento, escurecimento, negação, rotação, saturação e ajuste de alpha no espaço da cor. Preserva fórmulas, semântica de spot e as falhas representadas por Option. Mantém as assinaturas públicas, defaults, fase do pipeline, compatibilidade e pureza L1.

## Aceitação

Testes L1, build e crystalline-lint passam. Sem inferência de ganho de desempenho.
