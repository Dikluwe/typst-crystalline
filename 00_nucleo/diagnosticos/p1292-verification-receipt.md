# P1292 — recibo de verificação integrada v14

**Resultado:** `PASS`
**Verificador:** `/root/verificador_final_p1292`
**Escopo do veredito:** somente os quatro fragmentos observáveis P1292; não é
alegação de equivalência geral com Typst.

## Gates finais

| Gate | Resultado reproduzido sobre v14 |
|---|---|
| `cargo build --release` | exit 0 |
| `cargo test --workspace -q` | exit 0: 5366 + 912 + 1 + 61 + 2 + 71 + 2 + 6 + 1 + 5 + 11; 3 ignorados, zero falhas |
| `cargo test -q -p typst-core p1292 -- --nocapture` | 17/17 |
| `cargo test -q -p typst-wiring --test p1292_contract -- --nocapture` | 11/11 |
| `cargo fmt --all -- --check` | exit 0 |
| `crystalline-lint --quiet .` | exit 0 |
| V5 `--checks v5 --fail-on warning` | exit 0 |
| V7 `--checks v7 --fail-on warning` | exit 0 |
| V15 `--checks v15 --fail-on warning` | exit 0 |
| V26 `--checks v26 --fail-on warning` | exit 0 |
| `git diff --check` | exit 0 |

A sintaxe prescrita originalmente, `--fail-on-warning=V5`, não existe nesta
versão do linter. `crystalline-lint --help` declara `--checks <vN>` e
`--fail-on warning`; foi usado o equivalente suportado acima, sem relaxar a
severidade.

O lint global em modo não quieto emite avisos V16+ históricos, mas sai 0. Os
gates explicitamente exigidos V5/V7/V15/V26 ficaram sem warnings. Warnings do
compilador Rust não foram promovidos pelo passo a `-D warnings` e não ocultaram
falhas de teste.

## Regressões P1288–P1291

- P1288: 46 `Preserved`, 0 `Violated`, 4 `Unknown` deliberadamente opacos,
  ordem forward/reverse idêntica; testes do contrato/oráculo 20/20 e
  `SELF_TEST_OK`. Os quatro opacos históricos permanecem fora do sucesso e não
  são `Unknown` do P1292.
- P1289: 11/11 `Preserved`, 0 `Violated`, `Unknown=0`, determinístico; campanha
  histórica 5/5 e teste wiring 1/1.
- P1290: testes focais 7/7. O gate selado histórico interrompe corretamente
  porque P1292 alterou intencionalmente o L0 `compiler/eval/repr.md`; ele não
  foi falsificado. Um replay prospectivo com o mesmo contrato/oráculo,
  omitindo somente essa asserção histórica de hash, produziu igualdade
  bilateral e repetida para 21/21 casos, 84 `Preserved`, 0 `Violated`,
  `Unknown=0`.
- P1291: core 22/22 e wiring callback runtime 5/5.

## Superfícies e Unknown

- default: 111/99/12, quatro ganhos nominais, zero regressões dos 95;
- HTML: 115/89/26 conforme inventário HTML canônico, zero `Unknown`;
- contrato P1292: 11/11, `Unknown=0`;
- testes independentes P1292: 17/17;
- mutações P1292: 23/23, score 1.0, `Unknown=0`.

O único conjunto `Unknown` observado pertence ao fragmento histórico P1288 e
permanece explicitamente opaco; não é contado para aprovar P1292.

## Julgamento

Os lotes A (`math.cancel`), B (`math.underline`), C (`math.vec`) e D
(`place.flush`) satisfazem seus contratos, a superfície e a arquitetura. A
cadeia causal registra confirmação humana, RED anterior ao produto, campanha
adversarial, reabertura test-only, invalidação honesta do v12, resselo v13 e
resselo v14 da única LF extra removida no pre-commit. A prova `current + LF =
pin v13`, os 26/27 L0s restantes idênticos e o header do consumer sincronizado
demonstram ausência de delta normativo ou produtivo.
Resultado do verificador: **APPROVED**, `whole_step_closed=true`.
