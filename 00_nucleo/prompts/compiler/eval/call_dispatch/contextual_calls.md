# L0 — compiler/eval/call_dispatch/contextual_calls
Hash do Código: b511d597

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/eval/call_dispatch/contextual_calls.rs`. Extração interna de `call.rs` medida no HEAD `ce27af795` e árvore limpa em 2026-09-30 23:12:44 UTC. Não é nova medição de paridade ou desempenho.

A lógica extraída estava em `01_core/src/compiler/eval/call_dispatch/call.rs:317-477` no HEAD indicado. A classificação é extração interna de comportamento, não nova decisão de paridade.

## Contrato

Interceptar, na ordem vigente, `measure` e `layout` apenas após reconhecimento sintático do callee e da identidade nativa tipada. Preservar avaliação única do callee e argumentos em cada rota, gate contextual, estilos, geometria disponível, spans, erros e fallback. Não alterar contrato público, fase do pipeline, comportamento por defeito ou compatibilidade.

## Aceitação

Testes L1, `cargo build` e `crystalline-lint .` passam. Não inferir ganho de desempenho.
