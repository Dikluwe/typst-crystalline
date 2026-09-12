# Prompt L0 — `stdlib/math_style` — 14 funções math style
Hash do Código: 2cfdff8a

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/stdlib/math_style.rs`
**Origem**: fatiado de `rules/stdlib.md` em **P314** (ADR-0104). Convenção e
helpers partilhados: ver `stdlib/_comum.md`. Mecanismo do variant:
ADR-0102/0103; layout: `rules/math/layout/_comum.md` (handler MathStyled).

---

## 14 funções math style — Passo 311b.3 + P765b

```rust
pub fn native_bb     (...) -> SourceResult<Value>;  // DoubleStruck
pub fn native_bold   (...) -> SourceResult<Value>;  // bold=Some(true)
pub fn native_cal    (...) -> SourceResult<Value>;  // Chancery
pub fn native_display(...) -> SourceResult<Value>;  // MathSize::Display (size variant)
pub fn native_frak   (...) -> SourceResult<Value>;  // Fraktur
pub fn native_inline (...) -> SourceResult<Value>;  // MathSize::Text (size variant)
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
fechar após `calc`/P308). **P765b** acrescenta `display`/`inline` e o named arg
`cramped` em `script`/`sscript`/`display`/`inline`.

### Mapping função → MathStyled fields

| Função | `kind` | `bold` | `italic` | `cramped` |
|---|---|---|---|---|
| `bb` | `Some(DoubleStruck)` | None | None | None |
| `bold` | None | `Some(true)` | None | None |
| `cal` | `Some(Chancery)` | None | None | None |
| `display` | `Some(Display)` | None | None | `Some(false)` / named |
| `frak` | `Some(Fraktur)` | None | None | None |
| `inline` | `Some(Inline)` | None | None | `Some(false)` / named |
| `italic` | None | None | `Some(true)` | None |
| `mono` | `Some(Monospace)` | None | None | None |
| `sans` | `Some(SansSerif)` | None | None | None |
| `scr` | `Some(Roundhand)` | None | None | None |
| `script` | `Some(Script)` | None | None | `Some(true)` / named |
| `serif` | `Some(Plain)` | None | None | None |
| `sscript` | `Some(SScript)` | None | None | `Some(true)` / named |
| `upright` | None | None | `Some(false)` | None |

`bold`/`italic`/`upright` têm `kind = None` (flags ortogonais sobre variant
herdado — `bold(bb(x))` preserva DoubleStruck). `serif` usa `Some(Plain)` para
forçar variant.

`display`/`inline` são modelados como variants de tamanho (`MathStyleKind::Display`,
`MathStyleKind::Inline`) com factor 1.0, preservando o campo `cramped` via named
arg. Isto é uma simplificação mecânica: no vanilla controlam `EquationElem::size`
(`MathSize::Display`/`Text`); no cristalino, enquanto o layout math completo não
está implementado, representam-se como `MathStyleKind` para evitar erro de
"unknown variable" e permitir forward compatibility.

### Registo

Cada função registada em `make_root_scope` (`eval/mod.rs`) como
`Func::native(<nome>, native_<nome>)`. User-facing:

```typst
$ bb(x) $       → MathStyled { kind:Some(DoubleStruck), body:MathIdent("x"), ... }
$ bold(x + y) $ → MathStyled { kind:None, bold:Some(true), ... }
$ bb(cal(x)) $  → inner Cal prevalece sobre Bb no mesmo eixo de glyph
$ display(x) $  → MathStyled { kind:Some(Display), cramped:Some(false), ... }
```

### Casos de Aceitação

- `bb(Content)` → wrap directo · `bb(Str)` → `Content::MathText(s)` antes de
  wrap (**P899** — corrige `Content::text(s)`, que produzia `Content::Text`,
  fora do alcance de `apply_math_style`/`layout/mod.rs`, cujos braços cobrem
  `MathIdent`/`MathText`/`MathSequence`/`MathMatrix` mas não `Text`; sintoma
  medido: `$ bb("R") $` compilava sem erro mas devolvia `"R"` sem estilo
  nenhum, enquanto `$ bb(R) $` — argumento identificador, já avaliado como
  `Content::MathIdent` por `eval_math_arg_value` — devolvia `ℝ` correctamente.
  `Content::MathText` é o mesmo tipo que `Expr::MathText`/símbolos bare já
  produzem em modo math, dá braço a `apply_math_style` sem mudança de layout) ·
  `bb()` sem arg → **Err `missing argument: body`** (P811 — paridade vanilla
  medida: `$ frak() $` → `error: missing argument: body`; o comportamento
  anterior, `Content::Empty` silencioso, deixava uma equação vazia chegar ao
  export e produzia um PDF inválido) · `bb(x,y)` → Err (1 arg) ·
  `bb(x, key:v)` named → Err · `bb(1)` → Err (Int não-coercível).
- `script(Content)` → `cramped: Some(true)` (default) ·
  `script(Content, cramped: false)` → `cramped: Some(false)`.
- `sscript(Content)` → `cramped: Some(true)` (default) ·
  `sscript(Content, cramped: false)` → `cramped: Some(false)`.
- `display(Content)` → `cramped: Some(false)` (default) ·
  `display(Content, cramped: true)` → `cramped: Some(true)`.
- `inline(Content)` → `cramped: Some(false)` (default) ·
  `inline(Content, cramped: true)` → `cramped: Some(true)`.

### P1291 — diagnósticos públicos vanilla

**Medição bilateral anterior à decisão (2026-08-31, vanilla ratificado
`a51e02804`):** o comportamento anterior ainda emitia mensagens históricas em
português para excesso, named desconhecido e `cramped` inválido. Mensagens de
erro são observáveis da língua (ADR-0107/0108), logo o helper comum deve emitir:

| caso | mensagem |
|---|---|
| body ausente | `missing argument: body` |
| mais de um posicional | `unexpected argument` |
| body não convertível | `expected content, found <tipo vanilla>` |
| named desconhecido | `unexpected argument: <nome>` |
| `cramped` não booleano | `expected boolean, found <tipo vanilla>` |

Os nomes longos incluem pelo menos `int → integer`, `str → string` e `bool →
boolean`; os restantes usam o nome público vigente. `Str` continua convertível
para conteúdo matemático, como já especificado. A correção pertence ao helper
único e vale igualmente para a função global e o mesmo function pointer exposto
sob `math`; não criar wrappers por namespace.

### Helper interno

```rust
fn wrap_math_style(
    args,
    name: &str,
    kind: Option<MathStyleKind>,
    bold: Option<bool>,
    italic: Option<bool>,
    default_cramped: Option<bool>,
    allow_cramped_named: bool,
) -> SourceResult<Value>;
```
Helper único elimina duplicação. Só `script`/`sscript`/`display`/`inline` aceitam
o named arg `cramped`; as restantes rejeitam named args.

### Composição e Não-objectivos

Composição resolve no `MathLayouter` (P311b.4 — ver
`math/layout/_comum.md`), não nas funções nativas (estas só wrap). **P1291
corrige a precedência:** medição bilateral no vanilla ratificado em
2026-08-31 mostrou que o setter mais interno vence no mesmo eixo:
`serif(bb(ABC))` é byte-idêntico a `bb(ABC)`, enquanto
`bb(serif(ABC))` é byte-idêntico a `serif(ABC)`. Propriedades ortogonais
continuam a compor; um wrapper de tamanho como `inline` não apaga a variante
de glyph exterior/interior e o seu `cramped` continua local ao subtree.
Não-objectivos: Greek+dígitos completos (Latin priorizado).

## Diagnósticos de `mono` e `script`

`wrap_math_style` usa o span da chamada recebido em `Args` para os
diagnósticos de `mono` e `script`: body ausente ou inválido, positional
extra, named desconhecido e `cramped` não booleano. As demais funções
preservam sua política de span.

Quando a identidade é `mono` ou `script` e o named é exatamente `body`,
o diagnóstico é:

```text
the argument body is positional
hint: try removing body:
```

O hint é campo separado. Outro named continua `unexpected argument: <nome>`;
`script.cramped` é o único named válido dessa dupla.

Kind, body, cramped, defaults, ordem de avaliação, morfologia e layout não
mudam. O helper continua comum; não duplicar wrappers nem estender a exceção a
outra identidade.
