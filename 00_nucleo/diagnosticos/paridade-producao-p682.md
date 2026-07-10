# P682 — `version()` aceita a forma de array `version((M, m, p))`

**Estado:** fechado.
**Commit do trabalho:** `adc82302ff3febbe07e0cc7b70e6ee36ac41d8c1`.
**Data/hora da medição:** 2026-07-10T14:04:47-03:00.
**Commit base (HEAD antes do trabalho):** `96841a62fe49923ca32e513dfd8c975ea4a67ac3` — "P681: adiciona hash do commit ao relatório".
**Vanilla de referência:** `lab/typst-original/target/release/typst` = `typst 0.15.0 (969087ec)`.

## Proveniência da medição

- `cargo test --workspace` → **4333 passed, 0 failed** (soma de todos os `test result:`). P681 tinha 4327; os **+6** são os novos testes da forma de array.
- `crystalline-lint .` → `✓ No violations found`. `crystalline-lint --fix-hashes .` actualizou o header de `primitives_constructors.rs` para `@prompt-hash fe287bb8`.
- `git diff HEAD --stat` (estado exacto que gerou os números):

```
 00_nucleo/prompts/rules/stdlib/primitives-constructors.md | 29 ++++++++++-
 01_core/src/rules/stdlib/primitives_constructors.rs        | 56 +++++++++++++++++++++-
 2 files changed, 83 insertions(+), 2 deletions(-)
```

Binário usado: `target/debug/typst` (debug build do commit de trabalho).

## Sonda — o que o vanilla 0.15.0 realmente aceita (medido, não assumido)

Testei cada forma **em ficheiro isolado** (o vanilla pára no primeiro erro). Resultado:

| Forma | Vanilla 0.15.0 | `repr` / resultado |
|-------|----------------|--------------------|
| `version(0, 2, 2)` | ✅ aceita | `version(0, 2, 2)` |
| `version((0, 2, 2))` | ✅ aceita | `version(0, 2, 2)` (idêntico) |
| `#(version(0,2,2) == version((0,2,2)))` | ✅ | `true` |
| `version(0, 2, 2, "beta")` | ❌ | `expected integer or array, found string` |
| `version((0, 2, 2, "beta"))` | ❌ | `expected integer, found string` |
| `version(0, 2, 2, pre: "beta")` | ❌ | `unexpected argument: pre` |
| `version((0, 2, 2), pre: "beta")` | ❌ | `unexpected argument: pre` |

**Conclusão da sonda (ADR-0108):** o vanilla 0.15.0 aceita **apenas** 3 inteiros
posicionais **ou** um único array de exactamente 3 inteiros. **Não** aceita `pre`/`build`
(nem posicional nem named), nem string dentro do array. O enunciado de P682 previa isto
("…`version((0,2,2,"beta"))` funciona, **se confirmado pela sonda**"); a sonda **não**
confirmou — logo essa forma **não** foi implementada. Também contradiz o L0 pré-existente
(que descrevia `pre`/`build` como "forma vanilla"); essa discrepância é pré-existente e fica
registada como débito (fora do scope de P682).

## Implementação

`01_core/src/rules/stdlib/primitives_constructors.rs::native_version` ganhou um braço de
**forma de array fiel ao vanilla**, avaliado antes da forma posicional:

- `args.items.len() == 1` e o único posicional é `Value::Array(_)` → forma de array.
- named args presentes → erro: `"version(): a forma de array não aceita argumentos nomeados"`.
- `arr.len() != 3` → erro: `"version(): a forma de array requer exactamente 3 inteiros (major, minor, patch), recebeu N"`.
- elemento não-`Int` ou negativo → erro de tipo/sinal (via `as_nonneg_int`, mesma validação da forma posicional).
- sucesso → `Value::Version(Arc::new(Version::new(major, minor, patch)))`.

A forma posicional (incluindo o suporte pré-existente a `pre`/`build`) ficou **intacta**
(sem regressão). L0 `primitives-constructors.md` §5 ganhou a sub-secção "Forma array (P682)"
+ linhas de teste; header recalculado para `fe287bb8`.

## Validação (cristalino, `target/debug/typst`)

| Forma | Esperado | Resultado medido |
|-------|----------|------------------|
| `version(0, 2, 2)` | OK | exit 0 ✅ |
| `version((0, 2, 2))` | OK | exit 0 ✅ |
| `version(0,2,2) == version((0,2,2))` | `true` | exit 0 ✅ (paridade com vanilla: saída idêntica) |
| `version(0, 2, 2, "beta")` (posicional, pré-existente) | OK (sem regressão) | exit 0 ✅ |
| `version(0, 2)` | erro claro | `version(): requer 3 argumentos posicionais …` ✅ |
| `version((0, 2))` | erro claro | `version(): a forma de array requer exactamente 3 inteiros … recebeu 2` ✅ |
| `version((0, 2, 2, 4))` | erro claro | `… recebeu 4` ✅ |
| `version((0, 2, "x"))` | erro claro | `version(): 'patch' espera Int, recebeu str` ✅ |
| `version((0, 2, 2, "beta"))` | erro (vanilla rejeita) | `… recebeu 4` ✅ |
| `version((0, 2, 2), pre: "beta")` | erro (vanilla rejeita) | `version(): a forma de array não aceita argumentos nomeados` ✅ |

Documento de paridade (`#repr(version(0,2,2))` / `#repr(version((0,2,2)))` / igualdade):
cristalino e vanilla produzem texto **idêntico** (`version(0, 2, 2)` / `version(0, 2, 2)` / `true`).

## cetz re-testado — avança para lá da linha 1 (próximo problema registado)

```typst
#import "@preview/cetz:0.2.2": canvas, draw
#canvas({ draw.line((0,0), (1,1)) })
```

**Antes de P682:** `version(): requer 3 argumentos posicionais …, recebeu 1` (linha 1 de
`src/lib.typ`: `#let version = version((0,2,2))`).
**Depois de P682:** a linha 1 passa; cetz avança e falha **mais adiante**, com:

```
error: import: caminho deve ser uma string literal (ex.: #import "ficheiro.typ")
```

Origem (medida no pacote): cetz usa **import a partir de módulo/field-access**, não de
string literal — ex.: `src/util.typ:2` → `#import deps.oxifmt: strfmt` e
`src/anchor.typ:5` → `#import util: typst-length`. O `#import` cristalino só aceita caminho
string literal. **É o próximo débito de linguagem**, fora do scope de P682 (não se assume
cetz resolvido).

## Débitos que permanecem (não são P682)

- **`#import <mod>.<campo>: item` / `#import <mod>: item`** — import a partir de valor de
  módulo (sem string literal). É o próximo bloqueio de cetz (`util.typ:2`, `anchor.typ:5`).
- **`pre`/`build` e 4º posicional em `version()`** — o cristalino aceita (pré-existente,
  na forma posicional) mas o vanilla 0.15.0 **rejeita**. Divergência de paridade a tratar
  noutro passo (ou L0 a corrigir). Não tocado por disciplina de scope.
- **Mensagem da forma de array** difere da do vanilla (`recebeu N` vs `expected integer`);
  é clara e distinta, mas não byte-idêntica (paridade é de linguagem, não de bytes — ADR-0107).

## Ficheiros tocados (commit)

- `00_nucleo/prompts/rules/stdlib/primitives-constructors.md` (L0: forma array + testes)
- `01_core/src/rules/stdlib/primitives_constructors.rs` (`native_version` + 6 testes)
