# L0 — compiler/math/layout/node_wrappers
Hash do Código: 233b16b4

## Medição anterior à decisão

No HEAD ea7ae25ab (árvore limpa, 2026-09-30), `compiler/math/layout/mod.rs:1070-1104,1119-1193` contém os dois braços não triviais de wrappers de nó. A evidência delimita a extração, sem medir desempenho.

## Owner e contrato

Camada L1. Consumer único: `01_core/src/compiler/math/layout/node_wrappers.rs`. O wrapper de classe preserva o avanço do glifo e a correção itálica; o wrapper de estilo resolve os eixos internos, aplica a transformação existente e dispõe o corpo resultante. O match central permanece exaustivo e estático. Preservar assinaturas públicas, defaults, fase do pipeline e pureza.

## Aceitação

Testes L1, build e crystalline-lint passam. Nenhum ganho de desempenho é inferido.
