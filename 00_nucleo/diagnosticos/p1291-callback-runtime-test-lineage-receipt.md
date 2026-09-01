# Recibo de linhagem dos testes — P1291.callback-runtime

## Papel e escopo

- Papel: Testador A, autor da verificação independente.
- Regime: A/B, `executed_without_technical_isolation_attestation` por filesystem partilhado.
- Escrita exercida somente em:
  - header Crystalline Lineage de `04_wiring/tests/p1291_callback_runtime.rs`, sem mudar nenhum teste;
  - bloco `#[cfg(test)]` local de `01_core/src/compiler/math/layout/callbacks.rs`, sem mudar produção existente;
  - este recibo.
- Nenhum L0 ou outro ficheiro foi editado.

## Proveniência

- HEAD: `53d21c5a602f4045a769a0ab0c935baa5ecd3b88`.
- Snapshot: `2026-08-31T17:29:59-03:00`.
- Working tree partilhada: 354 entradas em `git status --short`.
- SHA-256 da saída literal de `git status --short`: `954c5338340fb045afbd68ee90afe46c9e2049bee92878db81191a92b504f2ea`.
- Prompt L0 1:1 recebido: `00_nucleo/prompts/wiring/tests/p1291_callback_runtime.md`.
- SHA-256 do prompt: `e7f184370677072c85d74da1a52f879173a9e63659bda4f5b15a845c725c4a61`.
- Header aplicado ao consumer L4: `@prompt-hash e7f18437`, `@layer L4`, `@updated 2026-08-31`.

## Artefatos e hashes

- Bloco local desde `#[cfg(test)]` até EOF em `callbacks.rs`: `1ee542bde60ae273b2bdd57b9c54c3cd4b251aa779afc624ec5609869ebc786a`.
- `01_core/src/compiler/math/layout/callbacks.rs` completo: `7a71e4e11ff3d57ac54acd9e7e2b9553922529dec2bc41a4b24914a5d93f4985`.
- Black-box com novo header, `04_wiring/tests/p1291_callback_runtime.rs`: `613997cb501fe3d8ce7e547101a08032a3ad6d9eacc236dd976f34ee04d40d69`.

O teste local mínimo `p1291_passagem_vazia_devolve_documento_complete` prova que uma passagem sem store e sem requests devolve `Complete` com o documento vazio, sem duplicar os ataques semânticos do black-box.

## Gates focais

```text
$ cargo test -p typst-core p1291_passagem_vazia_devolve_documento_complete -- --nocapture
running 1 test
test compiler::math::layout::callbacks::tests::p1291_passagem_vazia_devolve_documento_complete ... ok
test result: ok. 1 passed; 0 failed; 5349 filtered out
exit status: 0

$ cargo test -p typst-wiring --test p1291_callback_runtime p1291_callback_runtime_erro_preserva_span_da_chamada -- --exact --nocapture
running 1 test
test p1291_callback_runtime_erro_preserva_span_da_chamada ... ok
test result: ok. 1 passed; 0 failed; 4 filtered out
exit status: 0

$ crystalline-lint . --checks v2 --fail-on warning
✓ No violations found
exit status: 0

$ git diff --check -- 01_core/src/compiler/math/layout/callbacks.rs 04_wiring/tests/p1291_callback_runtime.rs
(sem saída)
exit status: 0
```

## Gate V1 residual

O gate combinado foi executado e não está integralmente verde:

```text
$ crystalline-lint . --checks v1,v2 --fail-on warning
error: Arquivo Cristalino sem linhagem causal @prompt encontrada [V1]
   --> ./01_core/src/compiler/math/layout/callbacks.rs:1
exit status: 1
```

O novo consumer black-box L4 já possui seu header 1:1. O residual é o header do consumer **produtivo** `callbacks.rs`, que não existia e não podia ser adicionado neste papel porque a allowlist autorizou nesse ficheiro somente o bloco de testes. O implementador B v2 deve adicionar a linhagem produtiva legitimada por `compiler/math/layout/callbacks.md`; isso não exige que escreva sua própria verificação.

## Veredito

Autoria e ownership dos testes foram fechados dentro da allowlist; V2 está verde e o consumer black-box possui linhagem L4. O gate final V1 permanece bloqueado exclusivamente pelo header produtivo ausente em `callbacks.rs`, explicitamente fora desta autoridade de escrita.
