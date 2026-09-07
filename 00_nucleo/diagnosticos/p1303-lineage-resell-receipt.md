# P1303 — recibo do resselo mecânico de linhagem pós-candidato

## Resultado do papel

`P1303_LINEAGE_RESELL_NOOP_PASS`.

O papel `SELADOR MECÂNICO DE LINHAGEM` recebeu o candidato P1303 já
materializado e executou o resselo exigido. O comando terminou com exit `0` e
imprimiu `Nothing to fix`: os metadados já estavam coerentes, portanto o
resselo foi idempotente e não alterou bytes em nenhum dos quatro paths.

Este recibo não faz nova alegação semântica, não executa ataques, não emite o
veredito P7/P8 e não atesta isolamento técnico. O filesystem e o contexto de
coordenação eram compartilhados. Não houve staging nem commit.

## Identidade e handoff conferido

- workspace: `/repos/Antigravity/typst-crystalline`;
- HEAD: `5b4a0d0438a535c54fdb5e74b28903c1313f5bc2`;
- captura pós-gates: `2026-09-04T01:09:16,078652803-03:00`;
- recibo do gate L0:
  `00_nucleo/diagnosticos/p1303-l0-gate-receipt.md`, SHA-256
  `98ae55dcc62a8b1d29e2ab7133997e06c3b6587354b5851d21cca70e657dbe15`;
- recibo RED:
  `00_nucleo/diagnosticos/p1303-red-tests-receipt.md`, SHA-256
  `d1d20e0510f328b1ee505c3344287d70e8d28f56ed2e4608943118f871806e4a`;
- recibo de implementação:
  `00_nucleo/diagnosticos/p1303-implementation-receipt.md`, SHA-256
  `ccbfa2ea7644d6a6f4a7f779ceef15b116828077d277dc6a7388c014bbc37ef2`.

Os três hashes coincidiram exatamente com o handoff. Antes do resselo, os
quatro hashes candidatos também coincidiram exatamente com os valores
recebidos.

## Hashes antes e depois

| Path | SHA-256 antes | SHA-256 depois |
|---|---|---|
| `00_nucleo/prompts/compiler/eval/bindings/field_access.md` | `92aa897909abfb6095ab59191614b0fd87c0d648e8e593ed8f0702176c2a542e` | `92aa897909abfb6095ab59191614b0fd87c0d648e8e593ed8f0702176c2a542e` |
| `00_nucleo/prompts/compiler/eval/tests.md` | `f50afc2f609a738f3fff5e7c511cdc26d8e6c8c3878107862ef0881c0a45a479` | `f50afc2f609a738f3fff5e7c511cdc26d8e6c8c3878107862ef0881c0a45a479` |
| `01_core/src/compiler/eval/bindings/field_access.rs` | `6b82a38b7c953ebe9dcbe96857dd9cb689aaaf69e95df153afa5de668ef89486` | `6b82a38b7c953ebe9dcbe96857dd9cb689aaaf69e95df153afa5de668ef89486` |
| `01_core/src/compiler/eval/tests.rs` | `ab57d8c30dd54e1f8d638fcad4ab52e3c2c4bf9c4b3f674816806fe5ba8824b0` | `ab57d8c30dd54e1f8d638fcad4ab52e3c2c4bf9c4b3f674816806fe5ba8824b0` |

## Metadados antes e depois

| Path | Metadado antes | Metadado depois |
|---|---|---|
| `00_nucleo/prompts/compiler/eval/bindings/field_access.md:2` | `Hash do Código: 2d102890` | `Hash do Código: 2d102890` |
| `00_nucleo/prompts/compiler/eval/tests.md:2` | `Hash do Código: 44131ec1` | `Hash do Código: 44131ec1` |
| `01_core/src/compiler/eval/bindings/field_access.rs:3` | `//! @prompt-hash 73f15bd4` | `//! @prompt-hash 73f15bd4` |
| `01_core/src/compiler/eval/tests.rs:3` | `//! @prompt-hash ffc80c85` | `//! @prompt-hash ffc80c85` |

Diff literal de metadados antes→depois (`diff -u`):

```diff
```

O output literal é vazio: não houve substituição de `Hash do Código` nem de
`@prompt-hash` no resselo pós-candidato.

## Comandos e outputs integrais

Comando:

```bash
crystalline-lint --fix-hashes .
```

Exit `0`; output integral:

```text
Nothing to fix
```

Comando:

```bash
crystalline-lint --fix-hashes --dry-run .
```

Exit `0`; output integral:

```text
Nothing to fix
```

Comando:

```bash
git diff --check
```

Exit `0`; output integral vazio.

## Containment e inspeção do diff Rust

Antes e depois do resselo, `git diff --name-status HEAD` listou exatamente os
mesmos quatro paths tracked autorizados:

```text
M  00_nucleo/prompts/compiler/eval/bindings/field_access.md
M  00_nucleo/prompts/compiler/eval/tests.md
M  01_core/src/compiler/eval/bindings/field_access.rs
M  01_core/src/compiler/eval/tests.rs
```

`git diff --cached --name-status` teve output vazio. Nenhum path tracked fora
da allowlist foi tocado pelo resselo.

O diff de `field_access.rs` contra o HEAD contém somente o header de linhagem
preexistente e a troca semântica P1303 recebida:

```diff
-//! @prompt-hash adcb180b
+//! @prompt-hash 73f15bd4
@@
-                access.span(),
+                access.field().span(),
```

O diff de `tests.rs` contra o HEAD contém somente o header de linhagem
preexistente e o bloco de testes `p1303_*` pinado pelo SHA-256 candidato:

```diff
-//! @prompt-hash 4afb0873
+//! @prompt-hash ffc80c85
```

Como os hashes candidatos dos dois arquivos Rust permaneceram idênticos antes
e depois do comando, o selador não introduziu mudança Rust semântica, de teste
ou de header. A inspeção de containment limita-se ao resselo mecânico e não é
uma atestação de correção do candidato.
