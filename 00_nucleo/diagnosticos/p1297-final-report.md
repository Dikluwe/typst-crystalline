# P1297 — relatório final do verificador P6

## Veredito

`P1297_CERTIFIED` para o fragmento R1 pinado. O protocolo foi executado em
regime Tekt full, segregado por entradas, ordem, capacidades e artefatos, sem
alegação de isolamento técnico de leitura no filesystem compartilhado.

Claim máxima, literalmente como autorizada pelo Passo 1297:

```text
Armamento e finalização causal do ciclo watch verificados pelo redesenho
vencedor registrado, para os artefatos, versões e observáveis pinados, sem
isolamento técnico de leitura e sem alegação de equivalência funcional geral.
```

```text
PROCESS_VIOLATION_CONFIRMED_NOT_ABSOLVED
```

P1294, P1295 e P1296 continuam `BLOCKED`, byte-preservados e não absolvidos.
As duas execuções P6 anteriores também continuam terminalmente `BLOCKED`:

- tentativa 1, receipt SHA-256
  `f226cc3a86d138cf2b5b8d402109ccfae2bb54e9fd6447ba89118c0823177049`:
  segunda execução obrigatória R1 falhou com ENOSPC;
- tentativa 2, receipt SHA-256
  `a36470ab02c0cb337eb93d85e75de1086351142ff6c0d895e4d8ec826a8bfef2`:
  o workspace falhou porque `TMPDIR` em RAM contaminou o observável P1286.

O receipt final incorpora os 34.255 bytes exatos da tentativa 2 em base64. O
decode foi executado e reproduziu o SHA-256 `a36470ab...`; como esse payload já
incorpora integralmente a tentativa 1, a cadeia permanece no único path
canónico permitido.

## Entradas congeladas

- Passo P1297: `5116ffc535bb8cbd33ca8fb449b492ddfcf9e133246d7143c27a97960748c682`;
- manifesto: `5bf3f29e353403006b90971e05f786ad2548648c053b9db59f879841aecdf322`;
- ledger: `bfc87da5047004f1cce67df0a14a7f6b9e1fb341a77169fc6ac1af8faaa91db0`;
- gate humano R1: `11777b20d0fc62256077339c35b86c559ba90cce167da2aee2ca081a9bd446e2`;
- selo R1: `2710fbbbe8748176764d37be40b40c30d93d39f1442e02064f32c7ff31348e92`;
- candidato `watch.rs`: `34188c0a3bdc129abc5c3eb9636a5fc5a701fc68f61c530c7a7f08a80a961b3c`;
- candidato `main.rs`: `40a7885a2d4aa6c9a02921461be44928f14e75f072bcda203433c55d1771a6f7`;
- consumer CLI: `56d989a4f79112d59558a71b0dbf609aa4022f9df0290e56afebc1b2b218a42b`;
- receipt de implementação: `7b23e0c2de48343debfa9ae684202729db450f5adbfe15a000dcb24495e228cb`.

Owner e consumer do contrato, RED, plano adversarial e receipt discriminatório
também foram revalidados contra o selo. R2 e R3 permaneceram ausentes; nenhum
mutante foi executado por P6; o índice permaneceu vazio e não houve staging.

## Gates executados do zero

Todos os comandos cargo usaram exclusivamente
`CARGO_TARGET_DIR=/dev/shm/typst-crystalline-p1297-p6.7lbVHi/target`. P6 não
definiu, exportou nem alterou `TMPDIR`, `TMP` ou `TEMP`; temporários funcionais
usaram o default normal `/tmp`. O target RAM existente foi reutilizado sem
cleanup e sem criar target adicional.

| Gate | Resultado |
|---|---|
| Contrato R1 | `7/7` duas vezes; 35,68 s e 35,11 s |
| Contrato P1295 | `6/6` duas vezes; 35,32 s e 35,47 s |
| P1292/P1293 direto | `11/11` e `11/11` |
| P1293/P1292 inverso | `11/11` e `11/11` |
| P1137 focal | `1/1` duas vezes |
| P1137 stress fixo | `20/20`, sem retry ou laço adaptativo |
| CLI integral | `71/71` duas vezes; 28,81 s e 29,02 s |
| Workspace | exit `0` duas vezes; 114,11 s e 130,77 s |
| `cargo fmt --all -- --check` | PASS |
| `crystalline-lint .` | PASS |
| V3, V4, V5, V7, V13, V14, V15, V26 individuais | todos PASS com `--fail-on warning` |
| `crystalline-lint --fix-hashes --dry-run .` | `Nothing to fix` |
| `git diff --check` e cached checks | PASS; índice vazio |

Nenhuma falha real de gate ocorreu nesta terceira execução. Portanto não houve
retry nem cleanup após falha.

## Build e superfícies

Medição de build sobre HEAD
`76fb7336311bdb6497456ab5fdc0a8ce355ff39b`, working tree não commitida:

- `Cargo.lock`: `6ab63c71323c0c8b3c1dd1f4036f0eecaf79c7b929456b1acd13b36e93343db7`;
- `rustc 1.92.0 (ded5c06cf 2025-12-08)`;
- `cargo 1.92.0 (344c4567c 2025-10-21)`;
- inventário Rust: 473 paths, SHA-256
  `3d556c34f6e5425e55e0ef8bcf08faca1f4d188cb1ac77c07de716adb2454cd1`;
- inventário de fontes: 1.323 entradas `fc-list`, SHA-256
  `7f3a56b9c5bba9921d10d8cac9c8f9078a614a41f704ccf3d94ae014433432af`.

`cargo build --release` usou
`TYPST_COMMIT_SHA=76fb7336311bdb6497456ab5fdc0a8ce355ff39b`, terminou em
96,73 s e produziu SHA-256
`9b9ac843317c766ddd2aa664caf6cbf8f6a8adee5810b0ebf696bbc4b12bebd8`.
A versão observada foi exatamente `typst 0.15.1 (76fb7336)`.

As superfícies bilaterais foram regeneradas com o binário RAM real:

- default: `111/99/12`, zero missing/unverified/Unknown, SHA-256
  `f8e36e6a38e97aa570da3905021a029321aaffa214e5b3fa78b489e9504312eb`;
- HTML: `115/104/11`, zero missing/unverified/Unknown, SHA-256
  `407570734adae8e7920f7780f9cdd5c9083bf3d4307577efd2d44e508ae7fa2e`.

Todos os objetos `results` são idênticos aos P1293. A única diferença é
`$.binaries.crystalline.sha256`, e o valor interno coincide com o binário real
usado. A grafia serializada do executável foi normalizada ao alias estável
P1293 `target/release/typst`; stdout, stderr, exits e resultados não foram
alterados.

## Artefatos terminais

- superfícies: `p1297-surface-default.json` e `p1297-surface-html.json`;
- receipt final: SHA-256
  `c9dd6046ebea9c90397212c1fce2ec3e00d5e473df0211f3cecef9c0c50ba111`;
- certificado: SHA-256
  `b7d429d76fd691238eef6ad2a67c8bf7631a5e91fa48b042d2d4f947f6ddac2b`;
- este relatório.

Estado terminal:

```text
P1294_BLOCKED_P1295_BLOCKED_P1296_BLOCKED_P1297_CERTIFIED
```
