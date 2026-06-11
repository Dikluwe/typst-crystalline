# Prompt L0 — `stdlib/math_style` — 12 funções math style
Hash do Código: 18e4ff6d

**Camada**: L1
**Ficheiro alvo**: `01_core/src/rules/stdlib/math_style.rs`
**Origem**: fatiado de `rules/stdlib.md` em **P314** (ADR-0104). Convenção e
helpers partilhados: ver `stdlib/_comum.md`. Mecanismo do variant:
ADR-0102/0103; layout: `rules/math/layout/_comum.md` (handler MathStyled).

---

## 12 funções math style — Passo 311b.3

```rust
pub fn native_bb     (...) -> SourceResult<Value>;  // DoubleStruck
pub fn native_bold   (...) -> SourceResult<Value>;  // bold=Some(true)
pub fn native_cal    (...) -> SourceResult<Value>;  // Chancery
pub fn native_frak   (...) -> SourceResult<Value>;  // Fraktur
pub fn native_italic (...) -> SourceResult<Value>;  // italic=Some(true)  [native_math_italic]
pub fn native_mono   (...) -> SourceResult<Value>;  // Monospace
pub fn native_sans   (...) -> SourceResult<Value>;  // SansSerif
pub fn native_scr    (...) -> SourceResult<Value>;  // Roundhand
pub fn native_script (...) -> SourceResult<Value>;  // Script + cramped
pub fn native_serif  (...) -> SourceResult<Value>;  // Plain (force serif)
pub fn native_sscript(...) -> SourceResult<Value>;  // SScript + cramped
pub fn native_upright(...) -> SourceResult<Value>;  // italic=Some(false)
```

Módulo dedicado per ADR-0037 (coesão por domínio; paralelo `calc.rs`/
`structural.rs`/`text.rs`). Cada função: (1) aceita 1 arg body (`Content` ou
`Str`); (2) wrap em `Content::MathStyled { kind, bold, italic, body, cramped }`;
(3) retorna `Value::Content(MathStyled)`.

**Marco P311b.3**: paridade "math style" **12/12 = 100%** (2ª categoria stdlib a
fechar após `calc`/P308). Cobertura A.7 Text features 57,1% → ~65%.

### Mapping função → MathStyled fields

| Função | `kind` | `bold` | `italic` | `cramped` |
|---|---|---|---|---|
| `bb` | `Some(DoubleStruck)` | None | None | None |
| `bold` | None | `Some(true)` | None | None |
| `cal` | `Some(Chancery)` | None | None | None |
| `frak` | `Some(Fraktur)` | None | None | None |
| `italic` | None | None | `Some(true)` | None |
| `mono` | `Some(Monospace)` | None | None | None |
| `sans` | `Some(SansSerif)` | None | None | None |
| `scr` | `Some(Roundhand)` | None | None | None |
| `script` | `Some(Script)` | None | None | `Some(true)` |
| `serif` | `Some(Plain)` | None | None | None |
| `sscript` | `Some(SScript)` | None | None | `Some(true)` |
| `upright` | None | None | `Some(false)` | None |

`bold`/`italic`/`upright` têm `kind = None` (flags ortogonais sobre variant
herdado — `bold(bb(x))` preserva DoubleStruck). `serif` usa `Some(Plain)` para
forçar variant.

### Registo

Cada função registada em `make_root_scope` (`eval/mod.rs`) como
`Func::native(<nome>, native_<nome>)`. User-facing:

```typst
$ bb(x) $       → MathStyled { kind:Some(DoubleStruck), body:MathIdent("x"), ... }
$ bold(x + y) $ → MathStyled { kind:None, bold:Some(true), ... }
$ bb(cal(x)) $  → outer Bb wraps inner Cal; P311b.4 outer-wins
```

### Casos de Aceitação

- `bb(Content)` → wrap directo · `bb(Str)` → `Content::text(s)` antes de wrap ·
  `bb()` sem arg → `Content::Empty` no body · `bb(x,y)` → Err (1 arg) ·
  `bb(x, key:v)` named → Err · `bb(1)` → Err (Int não-coercível).

### Helper interno

```rust
fn wrap_math_style(args, kind: Option<MathStyleKind>, bold: Option<bool>,
                   italic: Option<bool>, cramped: Option<bool>) -> SourceResult<Value>;
```
Helper único elimina ~10× duplicação nas 12 funções.

### Composição e Não-objectivos

Composição (outer-wins) resolve no `MathLayouter` (P311b.4 — ver
`math/layout/_comum.md`), não nas funções nativas (estas só wrap).
Não-objectivos P311b.3: `display`/`inline` vanilla; named args; Greek+dígitos
completos (Latin priorizado).
