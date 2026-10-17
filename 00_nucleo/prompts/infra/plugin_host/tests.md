# L0 — infra/plugin_host/tests
Hash do Código: 207daaf4

**Layer:** L3
**Exclusive consumer:** `03_infra/src/plugin_host/tests.rs`

## Current evidence

- `03_infra/src/plugin_host/tests.rs:17` — `uleb`.
- `03_infra/src/plugin_host/tests.rs:32` — `sleb`.
- `03_infra/src/plugin_host/tests.rs:46` — `vbytes`.
- `03_infra/src/plugin_host/tests.rs:52` — `name`.
- `03_infra/src/plugin_host/tests.rs:56` — `section`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/plugin_host/tests.rs`.

## Owned contract

- Own only the unit suite and deterministic binary WASM encoder, with no external fixture generator or WASM toolchain requirement.
- Keep Send+Sync, hello/output, function exports, missing-memory/binary failure, signature/arity, traps, bounds read/write, UTF-8 and invalid protocol cases. Invalid-binary runtime detail is checked only to the existing stable prefix.
- Test transition memory persistence, original-entry isolation, chaining and failure without derived state. Exact host messages are observables; fixtures/encoder are not production host logic or evidence of current upstream parity.

## Verification and boundary

Run `cargo test -p typst-infra --lib plugin_host::tests -- --test-threads=1` when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.


## Manutenção local e observação de testes — 2026-10-16

### Medição anterior à decisão

`03_infra/src/plugin_host/tests.rs` nas linhas 14 contém os avisos registrados em `00_nucleo/diagnosticos/clippy-locais-2026-10-16/before.json.gz`. O snapshot conserva fontes/L0, hashes, HEAD, working tree não commitado, UTC e git diff HEAD --stat. A inspeção dos consumidores e usos precedeu esta decisão; o gate Clippy selecionado deve falhar antes do código. Manutenção de representação/observação Rust, sem nova medição de intenção upstream ou paridade.

### Responsabilidade e aceitação

Retirar somente a constante privada I64 sem referências do encoder de fixtures WASM. Preservar bytes/opcodes das fixtures e todos os testes, inclusive recusas de assinatura; não alegar que I64 não seja suportado pelo produto.

Fluxo contínuo ADR-0127: sem contrato público, default de produto ou fase novos. L0 precede código e resselo restrito. Nenhuma supressão de lint nova. Conferir redução exata do inventário e ausência de avisos novos, gate Clippy RED→GREEN e unfulfilled_lint_expectations como erro; executar suíte workspace, build, formato, Rustdoc e linhagem sob 12 GiB, swap zero e um job. Não criar testes que apenas espelhem derive, Entry ou posicionamento de módulos. O caso de bbox exige observação real conforme especificado; os demais controles existentes mantêm fixtures e expectativas.
