# Paridade produção — P694 — módulo builtin `sys` (`sys.version` / `sys.inputs`) + `--input`

**Commit deste passo:** `6c34f85ff6886509f85dd56083d478b21e76378d` (detached HEAD; ver §Proveniência).
**Passo:** `00_nucleo/materialization/typst-passo-694.md`.
**ADRs:** ADR-0107 (paridade com a linguagem), ADR-0108 (medir antes de decidir),
ADR-0023/ADR-0024/ADR-0018 (IndexMap/EcoString/rustc_hash em L1).

---

## Proveniência das medições (regra de proveniência)

- **Estado medido:** working tree **não commitado** sobre `HEAD = bcef3eda06f9291f5bc477f263911cda7dd3733b` (P693).
- **Hora das medições E2E/testes:** `2026-07-10T21:15:25-03:00` (data do ambiente).
- **Ficheiros alterados no momento da medição** (`git diff HEAD --stat`):

```
 00_nucleo/prompts/contracts/world.md    | 28 +++++++++++-
 00_nucleo/prompts/infra/system-world.md | 12 ++++-
 00_nucleo/prompts/rules/eval.md         | 17 +++++++-
 00_nucleo/prompts/shell/cli.md          | 16 +++++++-
 00_nucleo/prompts/wiring.md             |  9 ++--
 01_core/src/contracts/world.rs          | 24 +++++++++-
 01_core/src/rules/eval/bibliography.rs  |  2 +-   (hash)
 01_core/src/rules/eval/closures.rs      |  2 +-   (hash)
 01_core/src/rules/eval/control_flow.rs  |  2 +-   (hash)
 01_core/src/rules/eval/flow.rs          |  2 +-   (hash)
 01_core/src/rules/eval/markup.rs        |  2 +-   (hash)
 01_core/src/rules/eval/math.rs          |  2 +-   (hash)
 01_core/src/rules/eval/mod.rs           | 17 +++++++--
 01_core/src/rules/eval/modules.rs       |  6 ++-
 01_core/src/rules/eval/rules.rs         |  2 +-   (hash)
 01_core/src/rules/eval/tests.rs         |  4 +-
 01_core/src/rules/stdlib/mod.rs         |  4 ++
 02_shell/src/cli.rs                     | 81 ++++++++++++++++++++++++++++++++-
 03_infra/src/world.rs                   | 25 +++++++++-
 04_wiring/src/main.rs                   |  5 +-
 04_wiring/tests/cli.rs                  |  2 +-   (hash)
 04_wiring/tests/crystalline_lint.rs     |  2 +-   (hash)
 22 files changed, 232 insertions(+), 34 deletions(-)
```

Novos ficheiros (não no diff acima): `00_nucleo/prompts/rules/stdlib/sys.md`,
`01_core/src/rules/stdlib/sys.rs`, este relatório. As entradas marcadas `(hash)`
são propagação de `@prompt-hash` pelo `crystalline-lint --fix-hashes` (edição de
`eval.md`/`wiring.md`), sem mudança de lógica.

> Nota: a working tree tinha muitos ficheiros **untracked** pré-existentes
> (materialização P589–P694, dois ADRs, `perf.data`, `temp_p638/660/694`). Nenhum
> entrou no commit (commit explícito, nunca `git add -A`).

---

## Objectivo

Implementar o módulo builtin `sys` com os **dois únicos campos** do vanilla
0.15.0 (confirmado em `lab/typst-original/crates/typst-library/src/sys.rs:15-16`):

- `sys.version: version`;
- `sys.inputs: dict` (str→str; vazio por omissão), populável via
  `--input chave=valor` na CLI.

Objectivo de desbloqueio: `oxifmt` (`sys.version >= version(0, 11, 0)`) e, em
cascata, `cetz`.

## Decisões (com medição que as produz — ADR-0108)

1. **Versão de paridade, não versão do binário.** `sys.version = version(0, 15, 0)`.
   Medição: `oxifmt`/`cetz` comparam `sys.version` com `version(…)` literais da
   linguagem; a semântica observável é "que Typst este documento vê" (ADR-0107).
   Devolver a versão do binário cristalino (fora da numeração Typst) quebraria o
   gate sem ganho.
2. **Fio de `inputs` via trait `World`, não via novos parâmetros de `eval`/
   `pipeline`.** `World` ganha `fn inputs(&self) -> SysInputs` com **default
   vazio** (cobre todos os `MockWorld` sem os tocar — espelha `read_bytes`/
   `include_source`/`resolve_package`); `SystemWorld` (L3) ganha campo +
   `with_inputs`; `eval_with_full_error` lê `world.inputs()` → `make_stdlib(&inputs)`
   → `scope.define("sys", …)`. Razão: não parte a assinatura pública do eval nem
   os 8 callers de `eval_with_full_error` (medido por grep). **Limitação
   evitada:** módulos importados (`eval_imported_file`) e o helper de teste
   também lêem `engine.world.inputs()`/`world.inputs()`, logo `sys.inputs` é
   consistente em todo o documento (não só no top-level).
3. **Tipo `SysInputs = IndexMap<EcoString, EcoString, FxBuildHasher>`** no
   contrato L1. Os três crates estão em `[l1_allowed_external]` → sem V14.
   Conversão `String → EcoString` fica em L3 (`with_inputs`), mantendo L2
   (CLI) livre de `ecow`/`indexmap` (L2 carrega `Vec<(String, String)>` cru).
4. **`sys` como `Value::Dict`** (padrão `calc`/`sym`/`color`), não `Value::Module`.
   Divergência de **repr/mecânica** aceite (ADR-0107): a semântica de acesso
   (`sys.version`, `sys.inputs`) é idêntica; só a forma impressa de `#sys`
   difere.
5. **Valores de `--input` são sempre strings** (paridade vanilla): `--input n=42`
   → `sys.inputs.n == "42"`. Validado em L2 (split no **primeiro** `=`, chave não
   vazia; `k=a=b` → valor `a=b`; `k=` válido; sem `=` ou `=v` → exit 2).

## Medições E2E (vanilla vs cristalino; texto via `pdftotext`)

| Prova | vanilla | cristalino | Paridade |
|-------|---------|------------|----------|
| `#repr(sys.version)` | `version(0, 15, 0)` | `version(0, 15, 0)` | ✅ igual |
| `#repr(sys.inputs)` (sem `--input`) | `(:)` | `()` | ⚠ repr empty-dict (ver §Limitações) |
| `#repr(sys.inputs)` `--input chave=valor --input n=42` | `(chave: "valor", n: "42")` | `(chave: "valor", n: "42")` | ✅ igual (valores strings) |
| `#repr(sys)` | `<module sys>` | `(version: version(0, 15, 0), inputs: ())` | ⚠ repr module vs dict (ADR-0107) |

`#repr((:))` isolado no cristalino = `()` e `#repr((a: 1))` = `(a: 1)` — confirma
que `()` é o repr **geral** do dict vazio no cristalino (não específico de
`sys`); o dict não-vazio coincide com o vanilla.

## Desbloqueio de pacotes

- **`oxifmt:1.0.0`** — vanilla exit 0 (`versao 1 ok`). Cristalino: o gate
  `sys.version >= version(0, 11, 0)` **passa** (o import deixa de falhar em
  `sys`/campo ausente) e a execução avança até um erro **posterior e não
  relacionado**: `error: cannot apply Assign to bool and bool` (`<detached>`).
  **Inferência (marcada, ADR-0108):** como o erro já não menciona `sys`, o
  acesso `sys.version` resolveu e a comparação avaliou sem erro — caso
  contrário oxifmt falharia no field access de `sys`. **Refutaria:** um erro
  que voltasse a citar `sys`/`version`. O débito `Assign to bool` é de eval,
  fora do escopo de P694.
- **`cetz:0.5.2`** — vanilla exit 0. Cristalino exit 1 com
  `error: unknown variable: plugin` (`<detached>`). Bloqueio por ausência do
  builtin `plugin` (host WASM), **não** por `sys` — débito maior e separado,
  fora do escopo de P694 (previsto na planificação).

## Testes e lint

- `cargo test --workspace`: **4385 passed / 0 failed** (3711 + 610 + 33 + 2 + 27
  + 2; os restantes binários de teste reportam 0/0). P693 era 4378; o delta +7
  são os novos testes deste passo (2 em `stdlib::sys` + 5 em `cli::parse_input`).
- `crystalline-lint .`: **0 violations** (após `--fix-hashes`; sem V3/V4/V14 —
  `SysInputs` usa tipos whitelisted; L1 não toca em I/O/CLI).

## Limitações conhecidas (honestidade)

- `repr` de dict vazio no cristalino é `()` (vanilla `(:)`) — traço **geral**
  pré-existente do repr de `Value::Dict`, não introduzido por P694; afecta só a
  forma impressa de `sys.inputs` vazio.
- `#sys` imprime como dicionário, não `<module sys>` (ADR-0107: repr/mecânica).
- `oxifmt` ainda falha mais à frente (`Assign to bool`); `cetz` ainda falha em
  `plugin`/WASM. Ambos são débitos separados de eval/WASM, não de `sys`.

## Língua vs mecânica (ADR-0107)

Paridade (linguagem): `sys.version` é `version` e compara com `version(…)`;
`sys.inputs` é `dict` str→str (vazio por omissão); `--input chave=valor` popula
`sys.inputs.chave == "valor"`; valores sempre strings. Diverge (mecânica/repr):
`#sys` como dict vs `<module sys>`; `Value::Dict` vs `Value::Module`; repr de
dict vazio.
