# P1184 — saneamento dos owners do hub e testes L3

**Execução:** 2026-08-25T21:47:04-03:00 a 2026-08-25T21:50:08-03:00
**HEAD:** `00f402e875956304aa435f749a251f359287e2ba`
**Estado:** working tree não commitado; índice vazio
**Linter:** `/home/dikluwe/.cargo/bin/crystalline-lint`, SHA-256
`eb7494979040e70feb6ac3b738c86979488b8c26927480126746aae2ff707c9d`

## RED estrutural

`crystalline-lint --checks v15,v26 --fail-on warning .` foi executado duas
vezes. Os outputs foram byte-idênticos, SHA-256
`a43ad6de4aecf0f9289e739af30147d3656fc9f67ec13ad019cab5151ffee32f`,
com V15=23, V26=0 e exatamente dois consumers de `infra.md`:
`03_infra/src/lib.rs` e `03_infra/src/integration_tests.rs`.

A árvore já continha as alterações não commitadas P1181–P1183 e ADR-0129,
registradas por `git status --short` e `git diff HEAD --stat`. O lote as
preservou e manteve o índice vazio.

## Owners finais

| Prompt L0 | Consumer único | Hash do Código | `@prompt-hash` final |
|---|---|---:|---:|
| `00_nucleo/prompts/infra.md` | `03_infra/src/lib.rs` | `4cc717a7` | `e07b48fb` |
| `00_nucleo/prompts/infra/integration_tests.md` | `03_infra/src/integration_tests.rs` | `841aa746` | `f647624f` |

Os hashes de código foram calculados removendo somente a linha
`@prompt-hash`. Os hashes dos prompts foram calculados sobre os L0s completos,
incluindo `Hash do Código`, depois do saneamento final de whitespace.

`infra.md` agora especifica somente a raiz estática da crate: 19 módulos
produtivos públicos e dois módulos privados sob `cfg(test)`. Ele não reivindica
o comportamento dos módulos filhos.

`infra/integration_tests.md` especifica somente o harness E2E L3, incluindo a
fronteira com `SystemWorld`, filesystem, eval, introspecção, layout e export. A
história P844 foi transferida para esse owner sem transformar as 226 funções de
teste em enumeração contratual perene.

Nenhum Núcleo Tekt foi criado porque não existe claim comum entre topologia do
hub e comportamento da suíte.

## GREEN estrutural

- V15: 23 → 22;
- V26: 0 → 0;
- V5: 419 → 417;
- os dois sources focais: ausentes de V5;
- o grupo compartilhado `infra.md`: ausente de V15;
- `--fix-hashes --dry-run .`: exit 2, bloqueado pelas 22 colisões restantes;
- status antes/depois do dry-run: byte-idêntico, zero writes.

Os dois sources foram comparados contra HEAD removendo somente `@prompt` e
`@prompt-hash`; o restante é byte-idêntico. Em `lib.rs`, o path `@prompt`
permaneceu `00_nucleo/prompts/infra.md`.

## Testes e build

- `cargo test -p typst-infra integration_tests`: 226 passed, 0 failed,
  632 filtered out;
- `cargo build`: GREEN;
- `git diff --check`: limpo após saneamento de whitespace;
- `git diff --cached --quiet`: GREEN, índice vazio.

Warnings preexistentes do workspace permanecem fora do escopo.

## Conclusão

P1184 fecha uma colisão de ownership sem alterar corpo Rust, contrato público,
default, fase ou compatibilidade. Restam 22 colisões V15. O candidato previsto
para P1185 é separar `testing/math_oracle.md` entre o módulo de implementação e
o hub `testing/mod.rs`, condicionado a nova auditoria focal.
