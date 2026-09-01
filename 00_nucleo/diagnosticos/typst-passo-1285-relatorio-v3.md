# Relatório final refinado v3 do Passo 1285

## Resultado

**Veredito: `Refined`.**

O terceiro verificador independente reexecutou a cadeia final após os
refinamentos históricos de gradient, formatação, pin documental e Color RGB.
Todos os gates obrigatórios passaram e nenhum `Unknown` foi promovido a sucesso.

## Evidência decisiva

- sete entradas finais, nove pins normativos L0 e nove consumers: SHA-256
  conferidos antes e depois dos gates;
- oráculos: `42 Preserved / 0 Violated / 0 Unknown`, exit 0;
- focais P1285: shell `5/5`, core `9/9`, infra `2/2`, wiring `1/1`;
- regressão math: `336/336`;
- `repr_value_complex_types`: `1/1`;
- P1225 exato de Color/Stroke/Raw: `1/1`;
- adversarial v3: 15 reruns mortos + 15 transferências por identidade =
  `30/30`, `0` survivors, `0` `Unknown`, score `1.0`; baseline pré/pós
  `19/19` e restores/hashes conferidos;
- `cargo build`: exit 0;
- `cargo test` integral: exit 0, `6366 passed / 0 failed`; 3 doctests
  ignorados;
- `p1137_watch_dependencias_recuperacao_e_filtro` passou na primeira suíte
  integral; não houve flake nem repetição excepcional;
- `cargo fmt --all -- --check`: exit 0;
- `git diff --check`: exit 0;
- `crystalline-lint .`: exit 0, `0` errors, `209` warnings e `1054` notes.

## Histórico preservado

V1 e v2 permanecem nos seus caminhos originais, sem sobrescrita e com hashes
inalterados. V1 foi `Not refined` por gradient/formatação/pin; v2 foi
`Not refined` por Color RGB nominal. As quatro causas foram revalidadas como
resolvidas nesta rodada, inclusive pela suíte integral verde.

## Proveniência

- HEAD: `53d21c5a602f4045a769a0ab0c935baa5ecd3b88`;
- estado: `working tree não commitado`;
- janela total da auditoria: `2026-08-30T17:29:09-03:00` a
  `2026-08-30T17:33:08-03:00`;
- janela dos gates: `2026-08-30T17:30:32-03:00` a
  `2026-08-30T17:32:53-03:00`;
- `git diff HEAD --stat`: `94 files changed, 645608 insertions(+), 1390 deletions(-)`;
- inventário exato do stat, comandos, timestamps, exits, contagens, hashes,
  pins L0, consumers e histórico constam em
  `p1285-verification-receipt-v3.md`.

Linguagem de atestação: segregado por capacidades e artefatos, executado sem
atestação de isolamento ambiental forte. Nenhum código, teste, L0, contrato,
oráculo, baseline, runner, plano ou receipt preexistente foi editado por este
papel.
