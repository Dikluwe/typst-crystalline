# L0 — compiler/eval/call_dispatch/invoke
Hash do Código: 7032f051

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/eval/call_dispatch/invoke.rs`. No HEAD `1492976302212e007282875aef066f84a3cf13aa` e árvore limpa, `01_core/src/compiler/eval/call_dispatch/call.rs:638-827` contém o comportamento extraído. O SHA-256, horário e estado inicial estão em `00_nucleo/diagnosticos/atomizacao-call-core-2026-09-30-before.json.gz`.

## Contrato

Resolver o callee preservado, avaliar argumentos e executar o match fechado de Func/Type/Plugin/Content com spans e traces atuais. Preservar `00_nucleo/prompts/compiler/eval/call_dispatch.md` e o owner `call.md`, sem mudar assinatura pública, defaults, fase do pipeline ou compatibilidade. A extração fica em L1, com despacho estático.

## Aceitação

Preservar a suíte L1 e os testes de call_dispatch; `crystalline-lint .` valida ownership/hashes. Não inferir ganho de desempenho.
