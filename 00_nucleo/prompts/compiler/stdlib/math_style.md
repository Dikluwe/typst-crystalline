# Prompt L0 — `stdlib/math_style` — 14 funções math style
Hash do Código: 56b9e300

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

## P1293.reopen-B-spans — diagnósticos de `mono` e `script`

### Medição anterior à decisão

O recibo de implementação B SHA-256
`7ebff1e223ceebe789224ed18f2c0518990aae787d7f3e48c3f96bbfb3d792dd`,
medido em `2026-09-01T16:38:33-03:00` sobre
`HEAD 7dd25ff0e222b6c7c640d6bc7957b98f94227507` e working tree não
commitada, registra `10/10` testes próprios GREEN para semântica, morfologia
e layout, mas bloqueia o lote pelos spans negativos. `math.mono(1)` conserva
a mensagem `expected content, found integer`, porém não emite source/range;
o vanilla ratificado ancora o valor em `1:10`. `math.script` reutiliza o mesmo
helper e apresenta a mesma causa para body, extra, named e `cramped` inválido.

Medição direta em `2026-09-01T16:44:26-03:00`: o consumer vigente e ainda
sem escrita P1293-B-spans, SHA-256
`439729b339dadd12cce1c7d7223c7b7aa6306ff2273cac4c645f21c378d651a4`,
usa `Span::detached()` para named desconhecido, `cramped` não bool, excesso,
cast de body e body ausente em
`01_core/src/compiler/stdlib/math_style.rs:32-101`. `native_mono` em
`:181-187` e `native_script` em `:211-217` já passam os nomes fechados
`"mono"`/`"script"` ao mesmo helper. O owner `call_dispatch` possui a AST
individual e pode selecionar privadamente `args.span` antes da delegação;
não é necessário campo novo em `Args`.

### Decisão estreita

No helper `wrap_math_style`, somente quando o parâmetro de identidade
existente é exatamente `"mono"` ou `"script"`, todos os diagnósticos usam
`args.span` recebido do call dispatch em vez de `Span::detached()`. A troca
cobre, sem mudar mensagem ou precedência:

- body ausente;
- segundo positional;
- body não convertível a Content;
- named desconhecido;
- para `script`, `cramped` não booleano.

As outras doze funções deste owner conservam byte-conceitualmente a política
vigente, inclusive `Span::detached()`; P1293 não as promove implicitamente a
paridade. A função global e o binding `math` continuam a reutilizar a mesma
function pointer, sem wrapper namespace-específico. O helper não seleciona a
âncora nem conhece AST: apenas consome o `args.span` privado já escolhido
pelo owner `call_dispatch` para a identidade resolvida.

Esta correção preserva kind/body/cramped, casts, defaults, ordem de validação,
mensagens, morfologia e layout. Spans diagnósticos são observáveis da língua
(ADR-0107) e a obrigação já pertence ao contrato P1293; logo é correção
interna em fluxo contínuo ADR-0127, sem API pública, campo, entidade, default,
compatibilidade ou fase nova. É proibido alterar `Args`, aplicar esta regra a
outra identidade, duplicar `native_mono`/`native_script` ou mudar a ordem de
avaliação. Este Prompt permanece owner 1:1 somente de `math_style.rs`.

Refutam a decisão: mensagem ou precedência diferente; span detached em
mono/script quando a chamada possui AST; mudança de span nas outras doze
funções; perda de reuso direto; ou regressão nos dez GREENs do lote. Qualquer
necessidade de outro owner ou de contrato público obriga parada antes do
código.

## P1293.reopen-B-independent-RED — `body:` posicional em mono/script

### Medição anterior à decisão

O julgamento independente posterior ao recibo final B SHA-256
`d677c0b1e6d8796c6680787d27b3409c100ff13653ab8ff89d7154813866720c`
rejeitou `math.mono(body: ...)` e `math.script(body: ...)`: o candidato emitia
`unexpected argument: body`, enquanto o vanilla/contrato exige a mensagem
`the argument body is positional` e o hint separado
`try removing body:`. O recibo vanilla independente SHA-256
`39f11f324677885ba093178fd5bc9cc40187a6dcceb67fa28fd55b247531c9a7`
mede em `p1293-vanilla-measurement-receipt.md:188-197` body posicional para
ambas as funções. Este owner, em `:107-126`, tratava todo named não aceito
como desconhecido; `:183-199` já restringe o comportamento P1293 às
identidades fechadas `mono`/`script`.

Medido: mensagem e hint são transcript diagnóstico observável. Inferência: o
helper comum possui identidade e argumento suficientes para distinguir o campo
posicional reconhecido de named realmente desconhecido. Refutam-na mudança no
vanilla para essas duas formas, perda do span já preservado, alteração de
precedência/avaliação ou necessidade de `Args`/API/payload novo.

### Decisão estreita

Somente quando a identidade existente é exatamente `mono` ou `script` e o named
é exatamente `body`, `wrap_math_style` emite:

```text
the argument body is positional
hint: try removing body:
```

O hint é campo diagnóstico separado; `hint:` acima identifica sua classe e não
faz parte do texto armazenado, que é exatamente `try removing body:`. Outro
named continua `unexpected argument: <nome>`; `script.cramped` continua o
único named válido. As outras doze funções permanecem fora do escopo e
byte-conceitualmente sob a política anterior, inclusive mensagens e spans.

Mensagem/hint são observáveis da linguagem (ADR-0107) e a correção é interna
e contínua (ADR-0127), sem novo gate humano. Kind/body/cramped, casts,
defaults, function pointer compartilhado, spans, precedência, ordem/quantidade
de avaliação, morfologia, layout e fase permanecem inalterados. É proibido
alterar `Args`, duplicar wrappers/nativas ou estender a exceção a outra
identidade.
