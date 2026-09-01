# Relatório final do Passo 1285

## Resultado

**Veredito: `Not refined`.**

A implementação candidata preserva o fragmento black-box P1285 nos 42 casos e passa
todos os focais solicitados, mas não satisfaz os gates finais obrigatórios. A suíte
integral falha e a árvore não está formatada segundo `cargo fmt --check`. Há ainda uma
lacuna de pin no oracle receipt.

## Evidência decisiva

- Oráculos: `42 Preserved / 0 Violated / 0 Unknown`, exit 0.
- Testes P1285: L2 `5/5`, L1 `9/9`, L3 `2/2`, L4 `1/1`.
- Regressão math: `336/336`.
- Mutation testing registrado: `30/30` mortos, `0` survivors, `0` Unknown,
  score `1.0`; todos os nove hashes de restauração conferem.
- `cargo build`: exit 0.
- `cargo test`: exit 101; `5317 passed / 1 failed`. Falha:
  `compiler::eval::repr::tests::repr_value_complex_types`, observado
  `gradient.linear()` contra esperado `gradient(...)`.
- `cargo fmt --check`: exit 1 em `02_shell/src/cli.rs:924`.
- `git diff --check`: exit 0.
- `crystalline-lint .`: exit 0, com 0 resultados de nível error; 209 warnings e
  1054 notas.
- Cadeia: o oracle receipt final, embora descreva restart v4 e os nove L0, ainda
  referencia na tabela de entradas o contrato antigo `9897a6…`, em vez do contrato
  final `647176…`.

## Proveniência

- HEAD: `53d21c5a602f4045a769a0ab0c935baa5ecd3b88`.
- Estado: `working tree não commitado`.
- Medição final: `2026-08-30T16:38:40-03:00`.
- `git diff HEAD --stat`: `94 files changed, 645599 insertions(+), 1383 deletions(-)`;
  o inventário exato está preservado no receipt de verificação.
- Os sete artefatos protegidos, os nove pins normativos L0 e os nove consumers
  permaneceram estáveis durante a execução.

## Cadeia de evidência

1. Contrato v4+precision:
   `647176b3d0840a3c14a2ac837f401d43090fdb4ad253ce8e5f90ba13810899a2`.
2. Oracle receipt:
   `1f4e98c40ef44355e04134821d21c4d523dffad9ab6a6c11eae39067f2d6d32b`.
3. Runner:
   `08c75c14b86c74858964f1a17efd0f2b499d0b9cfe3c80f5b980d44dda988c16`.
4. Baseline:
   `5cf69aed9117b9b9fbb7880df751b56a44871b538be84ddc8f33ecf40a1cfec7`.
5. RED receipt:
   `3f093f748769ccbb128183ca73cfab9052f2a7d6b097503ab5e5ccff2532f25d`.
6. Plano adversarial v5:
   `9cf5b7304c9761c245ea8be544bb7dbdd8bf6c833822e3f268dbe0c0f23a743a`.
7. Adversarial receipt:
   `d2a278e1bc77e677715c1b033e7ae27d0c862c19a9ce68b413371b82b25e8a3b`.
8. Receipt final: `00_nucleo/diagnosticos/p1285-verification-receipt.md`.

O receipt final contém comandos, timestamps, exits, contagens, hashes, L0/consumers,
estatística exata da árvore e a justificação binária do veredito. Nenhum código, teste,
L0, contrato, oráculo, baseline, plano ou receipt preexistente foi corrigido por este
papel.
