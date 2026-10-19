# L0 — compiler/math/layout/line_decorations
Hash do Código: e5ed28df

## Medição anterior à decisão

No HEAD ea7ae25ab (árvore limpa, 2026-09-30), a implementação vigente está em `01_core/src/compiler/math/layout/mod.rs:1312-1401`. A medição identifica o limite do bloco a extrair; não mede desempenho nem altera a paridade de linguagem.

## Owner e contrato

Camada L1. Consumer produtivo único: `01_core/src/compiler/math/layout/line_decorations.rs`. Dispor as linhas de overline e underline matemáticos com as constantes de fonte, coordenadas relativas à baseline e estilos já vigentes. Esta extração mantém contrato público, defaults, fase do pipeline, compatibilidade e pureza L1.

## Aceitação

O despacho permanece estático. Testes L1, build e crystalline-lint devem passar. Não inferir ganho de desempenho da separação.
