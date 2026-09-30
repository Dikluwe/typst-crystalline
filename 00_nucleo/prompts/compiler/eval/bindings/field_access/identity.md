# L0 — compiler/eval/bindings/field_access/identity
Hash do Código: 2442a200

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/eval/bindings/field_access/identity.rs`. No HEAD `ba5a4c7d305ed1444660d9eb80862f51b417efb6` e árvore limpa, `01_core/src/compiler/eval/bindings/field_access.rs:434-556` contém o comportamento extraído. O SHA-256, horário e estado inicial estão em `00_nucleo/diagnosticos/atomizacao-field-access-2026-09-30-before.json.gz`.

## Contrato

Identificar funções nativas e módulos efetivos e emitir avisos de símbolos obsoletos com base no binding resolvido. Preservar as assinaturas e semântica de `00_nucleo/prompts/compiler/eval/bindings/field_access.md`; despacho estático na mesma camada L1, sem I/O nem estado global mutável.

## Aceitação

Preservar a suíte L1 e os testes de acesso a campos; `crystalline-lint .` passa com ownership e hashes corretos. Não inferir ganho de desempenho.
