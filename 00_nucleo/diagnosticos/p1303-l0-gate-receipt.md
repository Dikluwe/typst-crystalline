# P1303 — recibo do gate L0-first e resselo

## Veredito

`P1303_L0_GATE_PASS`.

Os dois owners L0 foram atualizados antes de qualquer candidato RED ou de
implementação. O resselo terminou sem drift, o dry-run imprimiu `Nothing to
fix` e o diff tracked ficou contido exatamente nos quatro paths da allowlist.
Nos dois consumers Rust, a única alteração é o valor do header
`@prompt-hash`; não houve mudança semântica Rust.

O regime foi o protocolo completo de materialização segregada, **executado
sem atestação de isolamento técnico**. O executor ocupou somente o papel
`AUTOR_DA_OBRIGAÇÃO_L0`; hashes, ordem, capacidades e diff documentam a cadeia,
mas o filesystem e o contexto de coordenação compartilhados não provam
isolamento.

## Estado e entradas congeladas

- capturado em: `2026-09-04T00:48:20,405214585-03:00`;
- timezone: `America/Sao_Paulo (-03:00)`;
- HEAD: `5b4a0d0438a535c54fdb5e74b28903c1313f5bc2`;
- P0: `00_nucleo/diagnosticos/p1303-baseline-status.txt`, SHA-256
  `8e06d7deba171604986f5ae3eca4d9c2c964a2aaf6d2a0f1971f7aa27574d9a8`;
- medição fresca: `00_nucleo/diagnosticos/p1303-pre-measurement.json`,
  SHA-256
  `ef3a4eb0b6fcb3fb0e9b1d8ec54bfbfa8f1a4f7b58dd7c675cbf677bada573d4`;
- recibo do oracle: `00_nucleo/diagnosticos/p1303-oracle-receipt.md`,
  SHA-256
  `f23960b7c5a59f0586fda5378588d52de7546ff8551a5c6787c322af6f652289`;
- contrato `C-P1303-v1`: `00_nucleo/diagnosticos/p1303-contract.md`, SHA-256
  `837cb4bd1f38428d93cdadf9216c0182c69e3d8bfa8eb489ed52a710e2bbdf8a`;
- plano adversarial: `00_nucleo/diagnosticos/p1303-adversarial-plan.md`,
  SHA-256
  `ebaaad08d475a53fc2dc6fe41ea69c901069f302633b7e41ba0f5293031057b8`.

Todos esses hashes foram conferidos antes da escrita L0. A medição registrou
`48` runs, `24` comparações, `12` `DIFFERENT_DIAGNOSTIC`, `12` `MATCH_VALUE`,
`0` Unknown e `0` divergências de repetição nas duas ordens. Ela precedeu a
decisão normativa e confirmou a classificação
`ADR-0127_CONTINUOUS_DIAGNOSTIC_PARITY`.

## Capacidades exercidas

Leitura: `AGENTS.md`, skill `tekt-materializacao-segregada` e suas duas
referências, passo P1303 pelo path exato, ADR-0107/0108/0127, os dois owners L0
e os handoffs congelados acima. Não foram lidos candidatos de implementação ou
de testes.

Escrita manual: somente os dois owners L0 e este recibo. Escrita mecânica pelo
resselo: somente os headers dos dois consumers autorizados; o linter também
retificou o metadado `Hash do Código` já pertencente ao owner `tests.md`. Não
houve staging, commit, edição de contrato/oracle/plano adversarial nem mudança
semântica no Rust.

## Hashes antes e depois

| Path | SHA-256 antes (P0) | SHA-256 depois do resselo |
|---|---|---|
| `00_nucleo/prompts/compiler/eval/bindings/field_access.md` | `de81ef3bf6572a1777f9057655e8433c6f2b393e8846f1fbdb66bcd1c8f81df9` | `92aa897909abfb6095ab59191614b0fd87c0d648e8e593ed8f0702176c2a542e` |
| `00_nucleo/prompts/compiler/eval/tests.md` | `5ca2ad1e4bcf3f6fe30909be21f42bb2a2a5939a6efcbdfc136dba5b44e7a8d1` | `f50afc2f609a738f3fff5e7c511cdc26d8e6c8c3878107862ef0881c0a45a479` |
| `01_core/src/compiler/eval/bindings/field_access.rs` | `29abd27cc01b347da9fbc12a93f05265882c043d36cd9a03cff5e6487a96e024` | `6ce0009a9a25c786b935128dd3e5cc40689ef8a0942b0bd089ee627ec14cee58` |
| `01_core/src/compiler/eval/tests.rs` | `5437bd761f48bef79b2eedd5c2e920bcba310e41c0e85346afaf6db6a2e68af9` | `6fd1e0087ef29f8b2bb52082d08a61f87a1deabeb6689016721c046ee9c90c07` |

Os hashes “depois” foram capturados imediatamente após o resselo e antes da
criação deste recibo. Os headers mudaram literalmente assim:

```diff
--- 01_core/src/compiler/eval/bindings/field_access.rs
-//! @prompt-hash adcb180b
+//! @prompt-hash 73f15bd4
--- 01_core/src/compiler/eval/tests.rs
-//! @prompt-hash 4afb0873
+//! @prompt-hash ffc80c85
```

No L0 test-only, o resselo também corrigiu literalmente o metadado de linhagem:

```diff
-Hash do Código: 343763db
+Hash do Código: 44131ec1
```

O metadado equivalente de `field_access.md` permaneceu `2d102890`.

## Comandos e outputs

Comando:

```bash
crystalline-lint --fix-hashes .
```

Exit `0`; output integral:

```text
Applied ./01_core/src/compiler/eval/bindings/field_access.rs prompt=00_nucleo/prompts/compiler/eval/bindings/field_access.md hash-a=73f15bd4 hash-b=2d102890
Applied ./01_core/src/compiler/eval/tests.rs prompt=00_nucleo/prompts/compiler/eval/tests.md hash-a=ffc80c85 hash-b=44131ec1

Re-running analysis... ✅ 0 drift warnings remaining
```

Comando:

```bash
crystalline-lint --fix-hashes --dry-run .
```

Exit `0`; output integral:

```text
Nothing to fix
```

Comando adicional de integridade:

```bash
git diff --check
```

Exit `0`; output vazio.

## Inspeção literal e containment

`git diff --name-only` após o resselo listou exatamente:

```text
00_nucleo/prompts/compiler/eval/bindings/field_access.md
00_nucleo/prompts/compiler/eval/tests.md
01_core/src/compiler/eval/bindings/field_access.rs
01_core/src/compiler/eval/tests.rs
```

`git diff --numstat` registrou:

```text
80  0  00_nucleo/prompts/compiler/eval/bindings/field_access.md
99  1  00_nucleo/prompts/compiler/eval/tests.md
1   1  01_core/src/compiler/eval/bindings/field_access.rs
1   1  01_core/src/compiler/eval/tests.rs
```

O diff literal dos owners acrescenta medição antes da decisão,
classificação língua/mecânica, inferência e refutação, fluxo contínuo
ADR-0127, regra field-only exclusiva, proibições, regressões
negativas/positivas, sentinelas, `Unknown` estrito, poder discriminatório e
limites da alegação. Nos consumers, a inspeção literal encontrou somente as
duas substituições de `@prompt-hash` reproduzidas acima.

Não apareceu nenhum path tracked fora da allowlist. Os artefatos `p1303-*` e o
passo P1303 permanecem adições untracked autorizadas; este recibo junta-se a
elas. O gate L0-first está apto para handoff aos papéis posteriores, sem
antecipar RED, GREEN, score adversarial, verificação ou certificado final.
