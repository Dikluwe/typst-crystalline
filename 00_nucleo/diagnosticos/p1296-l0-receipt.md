# P1296 — recibo P1 de autoria L0

## Veredito

```text
P1296_L0_AUTHORED_READY_FOR_P2_CONTINUOUS_FLOW
```

Regime completo Tekt, papel P1, segregação por capacidades e artefatos no
workspace compartilhado. Não se alega isolamento técnico de leitura.

## Proveniência

- HEAD: `76fb7336311bdb6497456ab5fdc0a8ce355ff39b`
- passo: `00_nucleo/materialization/typst-passo-1296.md`
- SHA-256 do passo:
  `5a7f7c5d364fa6a373c1d411b8710131c68198bfeaaa4b19ec53fb17f7b8c00b`
- manifesto: `00_nucleo/diagnosticos/p1296-manifest.json`
- SHA-256 do manifesto:
  `7c9e2f75aab348ce434c293ea0b38259fc8c42b39360d6ca3b63a17491edd7d0`
- manifesto congelado em: `2026-09-03T00:26:36-03:00`
- medição pós-resselo: `2026-09-03T00:28:08-03:00`

## Medição e decisão

O receipt P1295, SHA-256
`06104b7d41f8e456e67df90c8036ba4362af67f3247880278bfa587e1d2c0573`,
registra uma suíte CLI integral `70/71`, exit `101`, com timeout exclusivo em
recuperação P1137. O log preservado `/tmp/p1295-p6-cli-1.log`, SHA-256
`919d16a5b6bb24050dcb6dcef20ef57ac8b9cd8f4fb82d018c31d130e6dd508b`,
confirma a fase.

O consumer usa `sleep(500 ms)` entre erro transitório e recuperação sem provar
que a iteração de erro capturou snapshot. O produto congelado fixa
`snapshot -> discard_output`; portanto um sentinel no staging removido por
`discard_output` é testemunha causal disponível ao harness.

Classificação ADR-0127: correção test-only em fluxo contínuo, sem contrato/API
pública, default, fase produtiva ou compatibilidade nova. Qualquer necessidade
de editar L3/L4 refuta a classificação e bloqueia P1296.

## L0 e resselo

| Artefato | SHA-256 antes | SHA-256 depois |
|---|---|---|
| `00_nucleo/prompts/wiring/tests/cli.md` | `5800231392fa0e8a5311a6a5d2664f0f06f85c80b273ffbe80c8fb5e04ed5d1c` | `ac81dcf4404d505cd10db05964351b35f5ba0e9b959d83ef903fd4b34241a49b` |
| `04_wiring/tests/cli.rs` | `cfd6ec5294f410c3625f7a6183db6ad5af776e4b3f8c5af25a96c284b7d5789f` | `e0dc7e8129729f1f15361fc76d1e82a908a63de2faeb1741fc540fd3bbc062e6` |

O único byte conceitualmente alterado no consumer por P1 foi o header
`@prompt-hash`, de `48bba6d8` para `58c349d6`. O corpo P1137 permanece o
baseline P1295 e só pode ser alterado por P2.

SHA-256 do diff acumulado do L0 contra HEAD:
`f1c4e1a09dc16c213c15f0063b65d99d5f1baab9ccee4ab51af15bb43f956753`.

## Gates P1

1. `crystalline-lint --fix-hashes --dry-run .` listou exclusivamente
   `04_wiring/tests/cli.rs`.
2. `crystalline-lint --fix-hashes .` atualizou exclusivamente esse header e
   terminou com zero drift warnings.
3. `crystalline-lint --checks v5,v15,v26 --fail-on warning .` terminou exit 0.
4. `crystalline-lint --fix-hashes --dry-run .` final imprimiu
   `Nothing to fix`.
5. `git diff --check` terminou exit 0.

P1 não escreveu corpo de teste, produto, mutante, selo, superfície ou veredito
final. Não criou nem alterou certificado/relatório P1295.

## Autoridade seguinte

P2 recebe somente passo, manifesto, L0 ressellado, baseline P1137, produto
congelado por hash e receipt/log vermelho P1295. Pode escrever apenas o corpo
P1137 e `p1296-test-receipt.json`; não recebe saídas privadas P3.

```text
P1295_BLOCKED_P1296_NOT_CERTIFIED
```
