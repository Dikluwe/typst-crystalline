# P681 — Pacotes `@preview` só offline (P-β de P678)

**Estado:** fechado (escopo de resolução; ver classificação cetz abaixo).
**Commit do trabalho:** `02a434dea1f59d7f48dbf158dd1b03b9e511ef83`.
**Data/hora da medição:** 2026-07-10T13:18:25-03:00.
**Commit base (HEAD antes do trabalho):** `fd60fc62ca4865821be2f8ae6c9457b6fe570427` — "P680: adiciona hash do commit ao relatório".

## Proveniência da medição

- `cargo test --workspace` → **4327 passed, 0 failed** (soma de todos os `test result:`).
- `crystalline-lint .` → `✓ No violations found` (após `crystalline-lint --fix-hashes .`: "Fixed 12 files … 0 drift").
- `git diff HEAD --stat` (estado exacto que gerou os números):

```
 00_nucleo/prompts/contracts/world.md    | 15 +++++++-
 00_nucleo/prompts/infra/system-world.md | 34 ++++++++++++++++-
 00_nucleo/prompts/rules/eval.md         | 14 +++++--
 01_core/src/contracts/world.rs          | 16 +++++++-
 01_core/src/entities/package_spec.rs    | 24 ++++++++++++
 01_core/src/rules/eval/modules.rs       | 35 ++++++++++-------
 03_infra/Cargo.toml                     |  1 +
 03_infra/src/world.rs                   | 66 ++++++++++++++++++++++++++++++++-
 Cargo.lock                              |  1 +
 ... (8 ficheiros eval/* só com @prompt-hash, 2 linhas cada)
 18 files changed, 192 insertions(+), 32 deletions(-)
```

Binário usado: `target/debug/typst` (debug build do commit de trabalho).

## O que o passo ligou

P678 (formato da cache `~/.cache/typst/packages/{ns}/{name}/{ver}/` + `typst.toml`/`entrypoint`)
a P679/P680 (`#import` local). A resolução `PackageSpec → Source` vive em **L3** (I/O) e
atravessa a fronteira por um método novo do trait `World` (L1):

- L1 `contracts/world.rs`: `fn resolve_package(&self, spec: &PackageSpec) -> Result<Source, String>`
  com **default `Err`** — não quebra `MockWorld`s existentes.
- L3 `SystemWorld::resolve_package`: procura data dir → cache dir
  (`$XDG_DATA_HOME|~/.local/share` depois `$XDG_CACHE_HOME|~/.cache`, sufixo `typst/packages`),
  lê `typst.toml` (`toml::Value`), extrai `[package].entrypoint`, `register_file(entrypoint)` +
  `source(id)`. Imports internos do pacote resolvem sozinhos porque `directory_of(entrypoint_id)`
  aponta para dentro da cache.
- L1 `eval/modules.rs`: braço `@` de `eval_module_import` agora faz
  `PackageSpec::from_str(&path)?` → `engine.world.resolve_package(&spec)?` e reaproveita o mesmo
  fluxo (ciclo + `eval_imported_file` + bindings) do braço local.
- L0 actualizados: `contracts/world.md`, `infra/system-world.md`, `rules/eval.md` (hashes
  recalculados via `--fix-hashes`). `entities/package-spec.md` inalterado (só ganhou testes).

## Medição (antes de decidir — ADR-0108)

| # | Caso | Comando essencial | Resultado medido |
|---|------|-------------------|------------------|
| 1 | Pacote mini isolado (cache falsa `XDG_CACHE_HOME`) | `#import "@preview/p681-mini:0.1.0": saudacao` → `#saudacao("Mundo")` | exit 0; PDF; `pdftotext` = **`Olá, Mundo!`** |
| 2 | Pacote ausente | `#import "@preview/nao-existe:9.9.9": foo` | exit 1; `pacote '@preview/nao-existe:9.9.9' não encontrado na cache local; download ainda não implementado (ver P-γ de P678)` |
| 3 | cetz (cache real `~/.cache`) | `#import "@preview/cetz:0.2.2": canvas, draw` … | exit 1; `version(): requer 3 argumentos posicionais (major, minor, patch) [+ pre opcional], recebeu 1` |
| 4 | Regressão import local | `#import "u.typ": saudacao` → `#saudacao("local")` | exit 0; PDF; `pdftotext` = **`Oi, local`** |

## Classificação (língua vs mecânica — ADR-0107/0108)

- **Resolução (escopo de P681) — fecha.** Os casos 1 e 2 provam as duas pontas da fronteira:
  pacote presente → `Source` registado e avaliado em módulo isolado com binding exposto
  (`Olá, Mundo!`); pacote ausente → erro claro e distinto, com remissão para P-γ de P678
  (download). O braço `@` já não é "não suportado".
- **cetz (caso 3) — NÃO é falha de resolução.** A resolução funcionou: o pacote foi encontrado
  na cache real, o manifesto lido, o entrypoint `src/lib.typ` registado e a avaliação começou.
  O erro ocorre **dentro** do pacote, na linha 1 de `src/lib.typ`:

  ```
  #let version = version((0,2,2))
  ```

  cetz chama `version` com **um** argumento (o array `(0,2,2)`); o `version()` cristalino exige
  **3 posicionais**. Isto é **cobertura de linguagem** (semântica de `version()`), fora do
  escopo de P681. O que refutaria esta classificação: se o erro viesse de "pacote não
  encontrado" ou de um caminho errado — não veio; o span e a mensagem são de aridade de
  `version()`. O débito é de linguagem, não de resolução/download.
- **Regressão (caso 4) — intacta.** Import local continua a produzir o resultado esperado.

## Débitos que permanecem (não são P681)

- **P-γ de P678** — download de pacotes `@preview` quando ausentes da cache. P681 só resolve o
  que já está em disco; a mensagem de erro aponta explicitamente para P-γ.
- **`version()` com array** — aridade/semântica revelada por cetz `src/lib.typ:1`. Item de
  linguagem separado; não bloqueia P681 (a resolução está correcta).

## Ficheiros tocados (commit)

- `00_nucleo/prompts/contracts/world.md`, `00_nucleo/prompts/infra/system-world.md`,
  `00_nucleo/prompts/rules/eval.md` (L0 + hashes)
- `01_core/src/contracts/world.rs` (`resolve_package` default), `01_core/src/entities/package_spec.rs`
  (+3 testes), `01_core/src/rules/eval/modules.rs` (braço `@`)
- `03_infra/Cargo.toml` (`toml`), `03_infra/src/world.rs` (`resolve_package` +
  `package_candidate_dirs` + `load_package_entrypoint`)
- 8 ficheiros `01_core/src/rules/eval/*.rs` — só `@prompt-hash` (fix-hashes)
- `Cargo.lock`
