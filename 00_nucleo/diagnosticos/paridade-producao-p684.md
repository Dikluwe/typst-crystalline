# P684 — Remover `pre`/`build` de `version()`; componentes arbitrários + zero-pad

**Estado:** fechado.
**Commit do trabalho:** `__P684_COMMIT__` (preenchido no commit seguinte).
**Data/hora da medição:** 2026-07-10T15:14:35-03:00.
**Commit base (HEAD antes do trabalho):** `e3a38e351838518a6f9b799a0de248533d4a63c3` — "P683: adiciona hash do commit ao relatório".
**Vanilla de referência:** `lab/typst-original/target/release/typst` = `typst 0.15.0 (969087ec)`.

## Proveniência da medição

- `cargo test --workspace` → **4325 passed, 0 failed** (soma de todos os `test result:`). O total desceu face a P683 (4337) porque os testes de `pre`/`build` (que deixaram de existir) foram removidos e substituídos por menos testes de componentes arbitrários/zero-pad; **0 falhas**.
- `crystalline-lint .` → `✓ No violations found`. `crystalline-lint --fix-hashes .` actualizou `entities/version.rs` → `@prompt-hash b85c305b` e `rules/stdlib/primitives_constructors.rs` → `@prompt-hash 1292005a` (0 drift).
- `git diff HEAD --stat` (estado exacto que gerou os números):

```
 00_nucleo/prompts/entities/version.md              |  95 +++---
 00_nucleo/prompts/rules/stdlib/primitives-constructors.md | 108 +++---
 01_core/src/entities/value.rs                      |  39 +-
 01_core/src/entities/version.rs                    | 364 +++++++++------------
 01_core/src/rules/eval/bindings.rs                 |  16 +-
 01_core/src/rules/eval/operators.rs                |  14 +-
 01_core/src/rules/eval/repr.rs                     |  10 +-
 01_core/src/rules/eval/tests.rs                    |  99 +++---
 01_core/src/rules/stdlib/primitives_constructors.rs    | 207 +++++-------
 9 files changed, 413 insertions(+), 539 deletions(-)
```

Binário usado: `target/debug/typst` (debug build do commit de trabalho).

## Sonda — verdade do vanilla 0.15.0 (medida)

| Forma | Vanilla 0.15.0 | Resultado |
|-------|----------------|-----------|
| `version()` / `version(1)` / `version(1, 2)` / `version(1, 2, 3, 4, 5)` | ✅ | sequência arbitrária de componentes inteiros |
| `version(1, 2, 3, pre: "alpha")` | ❌ | `unexpected argument: pre` |
| `version(1, 2, 3, "alpha.1")` | ❌ | `expected integer or array, found string` |
| `version(1, 2, 3) == version(1, 2, 3, 0)` | ✅ `true` | componente em falta = `0` (zero-pad) |
| `version(1, 2, 3) < version(1, 2, 3, 0)` | ✅ `false` | trailing zero não altera a ordem |
| `version(1, 2, 3) < version(1, 2, 3, 4)` | ✅ `true` | prefixo < mais longo |
| `repr(version(0, 11, 0))` / `repr(version(1, 2, 3, 0))` | ✅ | `"version(0, 11, 0)"` / `"version(1, 2, 3, 0)"` — **repr preserva zeros à direita** |
| `version(1, 2, 3).major` / `.patch` | ✅ | `1` / `3` (só os três primeiros têm nome) |

Conclusão (ADR-0108): o modelo correcto guarda os componentes **como dados** (para `repr`
fiel) e compara/ordena com **zero-pad** (componente em falta = `0`). A confirmação de que a
repr preserva zeros à direita foi decisiva — inviabiliza a alternativa de normalizar no
armazenamento.

## Implementação

- `entities/version.rs` — `Version { components: Vec<u64> }` (guardados como dados).
  `PartialEq`/`Eq`/`Hash`/`Ord` implementados à mão com zero-pad (`Hash` ignora zeros à
  direita para ser consistente com `eq`). `new`/`from_components`/`component(i)`/`major()`/
  `minor()`/`patch()`/`to_string`/`from_str` (int-only). Removidos `pre`/`build`,
  `with_pre`/`with_build`, `cmp_pre`/`cmp_pre_id`.
- `rules/stdlib/primitives_constructors.rs::native_version` — formas: string (int-only),
  posicional (qualquer número ≥ 0 de `Int` ≥ 0) e array (P682, idem). Qualquer argumento
  nomeado → erro `version(): não aceita argumentos nomeados`; 4º posicional de texto → erro
  de tipo. Removido o import `ecow::EcoString` (ficou sem uso).
- `rules/eval/repr.rs` — `version({todos os componentes})` (preserva zeros à direita).
- `rules/eval/operators.rs` — `==`/`!=` via `PartialEq` (zero-pad); comentários de "build
  ignorado" removidos.
- `rules/eval/bindings.rs` — field access só `major`/`minor`/`patch` → `component(0|1|2)`;
  `.pre`/`.build` → `campo desconhecido em version`.
- Testes actualizados em `version.rs`, `primitives_constructors.rs`, `value.rs`, `eval/tests.rs`
  (removidos os de `pre`/`build`; adicionados componentes arbitrários, zero-pad, named
  rejeitado, field-access desconhecido).
- L0: `entities/version.md` reescrito (componentes arbitrários + zero-pad, nota de origem);
  `rules/stdlib/primitives-constructors.md` §5/§6b/§7/§8 actualizados.

## Validação (cristalino `target/debug/typst` vs vanilla 0.15.0)

| Caso | Cristalino | Vanilla | Paridade |
|------|-----------|---------|----------|
| `version(1,2,3,pre:"alpha")` | erro: `version(): não aceita argumentos nomeados` | erro: `unexpected argument: pre` | ✅ ao nível de "é erro" (ADR-0107) |
| `version(1,2,3,"alpha.1")` | erro: `version(): 'componente' espera Int, recebeu str` | erro: `expected integer or array, found string` | ✅ ao nível de "é erro" |
| `repr(version(1,2,3,4,5))` | `version(1, 2, 3, 4, 5)` | `version(1, 2, 3, 4, 5)` | ✅ idêntico |
| `version(1,2,3) == version(1,2,3,0)` | `true` | `true` | ✅ idêntico |
| `repr(version(0,11,0))` | `version(0, 11, 0)` | `version(0, 11, 0)` | ✅ idêntico |

## cetz — estado inalterado (registado)

`#import "@preview/cetz:0.2.2": canvas, draw` + `#canvas({ draw.line((0,0),(1,1)) })` continua
a falhar em **`error: unknown variable: length`** — o mesmo débito registado em P683 (tipos
como valores: `length`/`ratio` usados em `type(x) == length`). P684 corrigiu `version()`
(não toca nesse débito); cetz não avança neste passo. Não se assume progresso.

## Nota de origem (P404–P406) — registo exigido pelo passo

O passo P684 pede uma nota no histórico do passo original (série "P404 Decimal → P405
Duration → P406 Version") a explicar o erro de origem. Em vez de **mutar** o ficheiro de
materialização histórico (que a regra do repositório manda não tocar sem indicação
explícita, e que seria reescrever o passado), regista-se aqui, na trilha de diagnósticos:

> A forma `version(major, minor, patch, pre: "...", build: "...")` (e o 4º posicional de
> texto) introduzida na série P404–P406 **nunca existiu no Typst**. Veio de uma confusão
> entre o `version` do Typst ("sequência arbitrária de componentes inteiros; só os três
> primeiros têm nome") e o **SemVer 2.0.0** geral (`major.minor.patch[-pre][+build]`). A
> justificação original citava explicitamente "semver 2.0.0", não a especificação do Typst,
> apesar de a chamar "forma vanilla". Duas fontes independentes confirmam: (1) a
> documentação oficial do Typst; (2) o histórico do próprio projecto. Corrigido em P684.

## Débitos que permanecem (não são P684)

- **Tipos como valores (`length`, `ratio`, …)** — bloqueio de cetz (P683), inalterado.
- **`version().at(i)` / métodos de array sobre `version`** — o vanilla expõe componentes via
  `.at`; não implementado (scope-out, ver `entities/version.md` §9).

## Ficheiros tocados (commit)

- `00_nucleo/prompts/entities/version.md`, `00_nucleo/prompts/rules/stdlib/primitives-constructors.md` (L0)
- `01_core/src/entities/version.rs`, `01_core/src/entities/value.rs`
- `01_core/src/rules/stdlib/primitives_constructors.rs`
- `01_core/src/rules/eval/repr.rs`, `01_core/src/rules/eval/operators.rs`,
  `01_core/src/rules/eval/bindings.rs`, `01_core/src/rules/eval/tests.rs`
