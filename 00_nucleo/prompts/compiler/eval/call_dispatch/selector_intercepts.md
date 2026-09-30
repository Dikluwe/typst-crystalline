# L0 — compiler/eval/call_dispatch/selector_intercepts
Hash do Código: a6bb9392

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/eval/call_dispatch/selector_intercepts.rs`. No HEAD `1492976302212e007282875aef066f84a3cf13aa` e árvore limpa, `01_core/src/compiler/eval/call_dispatch/call.rs:43-93` contém o comportamento extraído. O SHA-256, horário e estado inicial estão em `00_nucleo/diagnosticos/atomizacao-call-core-2026-09-30-before.json.gz`.

## Contrato

Interceptar somente selector.or/and/within/before/after na ordem atual; manter a resolução contextual e a distinção entre método lógico e outros nomes. Preservar `00_nucleo/prompts/compiler/eval/call_dispatch.md` e o owner `call.md`, sem mudar assinatura pública, defaults, fase do pipeline ou compatibilidade. A extração fica em L1, com despacho estático.

## Aceitação

Preservar a suíte L1 e os testes de call_dispatch; `crystalline-lint .` valida ownership/hashes. Não inferir ganho de desempenho.
