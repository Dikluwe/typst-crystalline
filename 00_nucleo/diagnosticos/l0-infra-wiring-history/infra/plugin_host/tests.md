# Archived pre-sanitation L0 — 00_nucleo/prompts/infra/plugin_host/tests.md

This is historical evidence, not a materializable prompt or an approval.
Original SHA-256: `b5992ef99b6df7d44566316dd0791a95025c76d0860539a8c370631be6c2e3c1`.
The original text below is preserved without rewriting; dates, line references,
working-tree inventories, proposals and old parity claims retain their original scope.
Dead or unpinned references are archival only and cannot prove current behavior.

---

---
Hash do Código: 047e42c9
prompt: infra/plugin_host/tests
layer: L3
created: 2026-09-26
adr: ADR-0107, ADR-0108, ADR-0109, ADR-0111, ADR-0127, ADR-0129, ADR-0130
---

# Prompt L0 — infra/plugin_host/tests

## Medição anterior à decisão

Consumer anterior `03_infra/src/plugin_host.rs:384-907`; HEAD `6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5`, working tree não commitado, observado em `2026-09-26T09:11:38-03:00`. Proveniência exata em `00_nucleo/diagnosticos/plugin-host-atomization.md`; contrato anterior arquivado em `00_nucleo/diagnosticos/plugin-host-l0-before-atomization.md`. Referências plugin.rs do texto histórico abaixo correspondem às medições antigas de P696/P819, não a nova prova contra o pin vanilla vigente.

## Decisão e contrato

Consumer exclusivo: `03_infra/src/plugin_host/tests.rs`. Possuir integralmente a suíte unitária e o encoder binário WASM determinista existentes, com nomes e cenários preservados, sem arquivos externos nem toolchain WASM. Manter teste Send + Sync, mensagens, hello, exports e transition. Não confundir fixture/encoder com código produtivo do host.

Atomização reversível dentro de L3 (ADR-0127, fluxo contínuo): nenhuma alteração de API pública, comportamento por defeito, fase de pipeline ou compatibilidade. Fronteiras Rust são mecânica (ADR-0107); bytes, efeitos de transition e mensagens permanecem os observáveis. Não atualizar runtime/features ou o contrato L1 nesta mudança.

## Critérios de verificação

Testes directos em Rust em `03_infra/src/plugin_host/tests.rs` (`#[cfg(test)]`),
**sem** sintaxe Typst:

- `hello.wasm` (192 B, regenerado pelo `gen_wasm.py` de P696 §A):
  `let id = host.load(&bytes).unwrap(); host.call(id, "hello", &[])` →
  `Ok(Bytes::new(b"hello".to_vec()))`.
- Cada item do catálogo coberto por um teste que constrói (em scratch, gerado
  por helper de teste) um wasm mínimo que provoca a mensagem e afirma
  `err.message == "<texto exacto>"`: sem `memory` (1); param não-`i32` (3);
  resultado ≠ `[i32]` (4); nº de args errado (5); trap via `unreachable` (6);
  out-of-bounds em write e em read (7); retorno `1` com UTF-8 (8) e com
  não-UTF-8 (9); retorno `2` (10). Item 2 (wasm inválido) coberto com bytes
  truncados — afirma só o prefixo `failed to load WebAssembly module (` (porque
  o `{err}` é inferência marcada).
- Header `@prompt 00_nucleo/prompts/infra/plugin_host/tests.md` e `@prompt-hash`
  correcto (via `crystalline-lint --fix-hashes .`).
- `cargo test -p typst-infra plugin_host` verde; `cargo test --workspace` sem
  regressão; `crystalline-lint .` zero violations (L3 pode importar `wasmi`;
  V3/V4/V14 não se aplicam a L3).
