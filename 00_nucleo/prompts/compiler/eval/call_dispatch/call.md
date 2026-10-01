# L0 — compiler/eval/call_dispatch/call
Hash do Código: 112d63bf

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/eval/call_dispatch/call.rs`. No HEAD `d5c563dfab39f789e8e2f72c9a9364f5a761e3ca` e árvore limpa, `01_core/src/compiler/eval/call_dispatch.rs:1305-2125` contém o comportamento extraído. O SHA-256, horário e estado inicial estão em `00_nucleo/diagnosticos/atomizacao-call-dispatch-2026-09-30-before.json.gz`.

## Contrato

Resolver e interceptar chamadas de funções e métodos na prioridade existente; delegar métodos de instância sobre Value a `instance_methods.rs` após mutação e antes de coleções; preservar avaliação única de receiver e argumentos. A unidade delega a sequência de métodos de Selector para `selector_intercepts.rs` e as chamadas contextuais de medida/layout para `contextual_calls.rs`, métodos de coleção/`with` para `collection_methods.rs` e métodos de Location/Content/OutlineEntry para `content_methods.rs` e a invocação final do callee para `invoke.rs`, cujos Prompts L0 são proprietários individuais. Preservar assinaturas públicas, diagnósticos, prioridades de intercepção, rastros e semântica de argumentos especificados em `00_nucleo/prompts/compiler/eval/call_dispatch.md`. A extração permanece em L1 com despacho estático, sem I/O ou estado global mutável.

## Aceitação

Preservar a suíte L1, os testes de call_dispatch e `crystalline-lint .` com ownership e hashes corretos. Não inferir ganho de desempenho.
