# Testes independentes P1431 — transporte de `FrameItem::Line.cap`
Hash do Código: 46c0d5cf

## Dono test-only

Consumer único: `01_core/src/entities/p1431_line_cap_tests.rs`.

## Obrigação

Observar linhas produzidas por rotas públicas, sem construir manualmente a
representação nova. Augment automático de matriz chega ao frame como `Square`;
stroke customizado conserva `Butt`, `Round` ou `Square`; produtores legados,
como a linha de fração, continuam `Butt`. A observação atravessa o layout
completo para cobrir reconstruções mecânicas sem ler a implementação candidata.

Linha ausente, múltipla sem identidade atribuível ou observação truncada do cap
é `UNKNOWN_STROKE_OBSERVATION`, com aceitação e crédito falsos.
