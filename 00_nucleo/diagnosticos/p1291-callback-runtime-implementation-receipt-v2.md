# P1291 — recibo de implementação v2 do runtime de callback de `math.cancel`

## Papel, regime e proveniência

- Papel: Implementador B v2 de `P1291.cancel-angle-runtime`.
- Regime: materialização segregada completa, restrita à fase de implementação após amendment e reseal dos testes.
- Este recibo não constitui veredito independente nem alega isolamento técnico.
- Estado medido: working tree não commitado sobre `HEAD 53d21c5a602f4045a769a0ab0c935baa5ecd3b88`, em `2026-08-31T17:44:57-03:00`.
- A árvore era compartilhada e já continha a implementação v1 e alterações alheias; nenhuma delas foi revertida.

Entradas causais desta rodada:

- `00_nucleo/diagnosticos/p1291-callback-runtime-seal-amendment-3.json`, SHA-256 `30ad068e8c002fad7b00a6fc8ef245b899c403c44ad3b9c1693b8c26d0853b5e`.
- `00_nucleo/diagnosticos/p1291-callback-runtime-test-reseal-v3.json`, SHA-256 `5ba8a1a6a19a9025558684eb7f44c4fe1cb0a518f8209320d8e88029da90a948`.
- L0 `compiler/math/layout/callbacks.md`, SHA-256 `59f636f030677058218a23b45396cf146ab2c6f8445d4d8889f05a612cf0365d`.
- L0 `compiler/math/layout/cancel.md`, SHA-256 `213fbf21fbea1d23f3923838fbf6b0e9aad47c89e0b1b96378464a5e06e7d3bf`.
- L0 `compiler/eval/bindings/field_access.md`, SHA-256 `076a98ab6ed2ddbddb5322d59f9fdbd9e76975ef2a5d28c9d5c9e4a1467b070d`.
- L0 `compiler/eval/math.md`, SHA-256 `6be2fbb147f07a460fc438486169ff2c2c00518a09176bb9fb57e3982b89eda6`.
- Owners relidos para a correção de TOC/API: `compiler/layout.md`, SHA-256 `f4bd917d3c8256392359a44ba462f48c84ad2a976acebabbad9aa57a36ac865c`, e `infra/pipeline.md`, SHA-256 `28fb5f18d4f6d454de32cb6120c0ac2919d802a5d1aa85261b466872d632df3b`.

## Entradas protegidas preservadas

| Entrada | SHA-256 antes | SHA-256 depois |
|---|---|---|
| `01_core/src/compiler/math/layout/tests.rs` | `54aa5b5017803bb829e56d0ff632f359c6824574d5eb84e9f2f17351ac4e991a` | `54aa5b5017803bb829e56d0ff632f359c6824574d5eb84e9f2f17351ac4e991a` |
| `04_wiring/tests/p1291_callback_runtime.rs` | `613997cb501fe3d8ce7e547101a08032a3ad6d9eacc236dd976f34ee04d40d69` | `613997cb501fe3d8ce7e547101a08032a3ad6d9eacc236dd976f34ee04d40d69` |
| `01_core/src/compiler/math/layout/callbacks.rs`, de `#[cfg(test)]` até EOF | `1ee542bde60ae273b2bdd57b9c54c3cd4b251aa779afc624ec5609869ebc786a` | `1ee542bde60ae273b2bdd57b9c54c3cd4b251aa779afc624ec5609869ebc786a` |

Nenhum L0, selo ou teste protegido foi editado.

## RED confirmado antes da correção

```text
cargo test -p typst-core p1291_ -- --nocapture
FAILED: p1291_cross_reusa_default_positivo_e_ignora_inverted_na_chamada
test result: FAILED. 20 passed; 1 failed; 0 ignored; 0 measured; 5329 filtered out
```

```text
cargo test -p typst-wiring --test p1291_callback_runtime -- --nocapture
FAILED: p1291_callback_aninhada_em_script_observa_size_efetivo_reduzido
test result: FAILED. 4 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

## Correções v2

- `01_core/src/compiler/math/layout/cancel.rs`: as duas requests de `cross` recebem o mesmo `auto` positivo; a reflexão ocorre somente após Auto/Angle/Func resolverem, por `invert = if cross { line == 1 } else { inverted }`.
- `01_core/src/compiler/eval/bindings/field_access.rs`: `text.size` devolve `Length::pt(engine.styles.size())`, observando o slot tipado derivado para scripts.
- `01_core/src/compiler/layout/mod.rs`: cada tentativa de TOC cria pass state próprio; documento e transcript de tentativa rejeitada são descartados; `finish(doc)` é chamado somente na tentativa convergida. O teto sem convergência devolve `Pending(vec![])`, nunca documento best-effort. A entrypoint compatível com métricas agora devolve `SourceResult<PagedDocument>`; `Complete` vira `Ok` e `Pending` vira diagnóstico em `Err`, sem panic nem documento provisório. O wrapper legacy que não expõe callbacks preserva `PagedDocument` via `.expect(...)`.
- `03_infra/src/pipeline.rs`: `Pending` vazio produz diagnóstico imediato e não entra num ciclo de realização fictício.
- `01_core/src/compiler/math/layout/callbacks.rs`: header produtivo completo com owner `00_nucleo/prompts/compiler/math/layout/callbacks.md`; o bloco protegido não foi tocado.
- `01_core/src/compiler/eval/math.rs`: apenas atualização de lineage para o L0 refinado e formatação local, sem mudança adicional de classificação.
- Callsites mecânicos autorizados em `01_core/src/compiler/layout/tests.rs` e `03_infra/src/integration_tests.rs`: consumo explícito do `SourceResult` com `.expect(...)`.

Os headers produtivos dos consumers alterados nesta rodada foram alinhados manualmente aos L0s vigentes. Não foi executado `--fix-hashes`.

## GREEN e gates executados

```text
cargo test -p typst-core p1291_ -- --nocapture
test result: ok. 21 passed; 0 failed; 0 ignored; 0 measured; 5329 filtered out
```

```text
cargo test -p typst-wiring --test p1291_callback_runtime -- --nocapture
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

```text
cargo test -p typst-infra --no-run
exit 0; callsites de infra compilados
```

```text
cargo test -p typst-core layout_toc_com_readonly_nao_duplica_contadores -- --nocapture
test result: ok. 1 passed; 0 failed
```

```text
cargo test -p typst-infra pipeline_toc_ -- --nocapture
test result: ok. 3 passed; 0 failed
```

```text
crystalline-lint . --checks v1,v2
✓ No violations found
```

```text
git diff --check
exit 0; sem saída
```

`rustfmt --edition 2021 --check` passou para os ficheiros produtivos e callsites modificados, excluindo `callbacks.rs`. A checagem prévia desse arquivo mostrou diferença exclusivamente dentro do bloco `cfg(test)` protegido; por isso não se executou formatação destrutiva sobre o arquivo inteiro e o seu hash selado foi preservado.

Os builds emitiram warnings preexistentes/não fatais; nenhum alterou os resultados acima.

## Limitações e parada

- Não foi executado `crystalline-lint --fix-hashes`.
- Não foram executados os gates finais nem emitido veredito de verificador/adversário.
- `vec` continua **Unknown**, fora desta rodada e sem crédito.
- A execução continua sob `executed_without_technical_isolation_attestation` devido ao thread/filesystem compartilhado.

## Resultado do implementador

Contrato focal v2 de `math.cancel(angle: callback)`: **GREEN**. O veredito final permanece reservado ao papel independente.
