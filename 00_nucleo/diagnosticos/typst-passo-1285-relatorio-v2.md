# Relatório final refinado v2 do Passo 1285

## Resultado

**Veredito: `Not refined`.**

O ciclo refinado resolveu as três causas da primeira rodada histórica, mas a
suíte integral revelou um novo conflito legado em `typst-shell`. Como o
critério é binário, essa única falha impede o fechamento mesmo com todos os
oráculos, mutantes, focais e gates arquiteturais restantes verdes.

## Evidência decisiva

- artefatos refinados e nove consumers: todos os SHA-256 conferem;
- nove pins normativos L0: todos conferem; `eval/math.md` mudou somente o selo;
- oráculos: `42 Preserved / 0 Violated / 0 Unknown`, exit 0;
- focais P1285: L2 `5/5`, L1 `9/9`, L3 `2/2`, L4 `1/1`;
- regressão math: `336/336`;
- `repr_value_complex_types`: `1/1`;
- adversarial v2: 15 reruns mortos + 15 transferências por identidade =
  `30/30`, `0` survivors, `0` `Unknown`, score `1.0`; baseline antes/depois
  `18/18` e nove restaurações conferidas;
- `cargo build`: exit 0;
- `cargo test`: exit `101`; antes da interrupção, `6290 passed / 1 failed`;
- falha: `p1225_eval_json_stroke_e_nominal_e_raw_permanece_proibido` ainda
  espera que JSON de `Value::Color` seja erro, enquanto P1285 no L0/contrato
  vigente exige fallback público por `repr` para color;
- `cargo fmt --all -- --check`: exit 0;
- `git diff --check`: exit 0;
- `crystalline-lint .`: exit 0, `0` errors, `209` warnings e `1054` notes.

O verificador não corrigiu o teste nem a produção. A presença do vermelho
integral é suficiente para `Not refined`.

## Histórico preservado

A primeira verificação `Not refined` permanece nos caminhos originais, sem
sobrescrita. Suas três causas foram revalidadas como resolvidas neste ciclo:

1. o focal legado de repr agora passa;
2. a formatação agora passa;
3. o oracle receipt SHA `998a997…` agora pina o contrato final `647176…`.

Essa resolução histórica não supera a nova falha integral P1225.

## Proveniência

- HEAD: `53d21c5a602f4045a769a0ab0c935baa5ecd3b88`;
- estado: `working tree não commitado`;
- janela dos gates: `2026-08-30T17:01:22-03:00` a
  `2026-08-30T17:05:06-03:00`;
- medição do estado: `2026-08-30T17:04:25-03:00`;
- `git diff HEAD --stat`: `94 files changed, 645599 insertions(+), 1384 deletions(-)`;
- inventário exato, comandos, timestamps, exits, contagens, hashes, pins L0 e
  consumers constam em `p1285-verification-receipt-v2.md`.

Linguagem de atestação: segregado por capacidades e artefatos, executado sem
atestação de isolamento ambiental forte. Nenhum código, teste, L0, contrato,
oráculo, baseline, runner, plano ou receipt preexistente foi editado por este
papel.
