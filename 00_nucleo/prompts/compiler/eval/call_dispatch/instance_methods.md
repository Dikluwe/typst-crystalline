# L0 — compiler/eval/call_dispatch/instance_methods
Hash do Código: 4e2ab85d

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/eval/call_dispatch/instance_methods.rs`. O comportamento original estava em `01_core/src/compiler/eval/call_dispatch/call.rs:70-263` no HEAD `6fc546566`, árvore limpa, 2026-10-01 01:04:22 UTC. Esta é extração interna, não nova medição de paridade ou desempenho.

## Contrato

Mantém o match estático ordenado dos métodos de instância de Value para State, Counter, SpotColorant, Color, Gradient, valores numéricos, geometria e Version. Preserva a avaliação do alvo nesta fase, os argumentos avaliados apenas quando necessários, spans, traces e a passagem para o próximo despacho quando o método não é reconhecido. Preservar contrato público, fase do pipeline, defaults, compatibilidade e pureza L1.

## Aceitação

Suíte L1, `cargo build` e `crystalline-lint .` passam. Não inferir ganho de desempenho.
