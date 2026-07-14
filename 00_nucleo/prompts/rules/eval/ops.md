# Prompt L0 — `rules/eval/operators`
Hash do Código: 1a2ab64e

**Camada**: L1
**Ficheiro alvo**: `01_core/src/rules/eval/operators.rs`
**Criado em**: 2026-06-25 (P469)
**Atualizado em**: 2026-06-25
**ADRs relevantes**: ADR-0029 (`Length`), ADR-0117 Cláusula 4

---

## Contexto

`operators.rs` implementa `eval_binary_op` e `eval_unary_op`, o dispatcher de
operadores do eval Typst. P469 adiciona semântica para comprimentos relativos
(`Rel<Length>` / `Value::Relative`).

---

## Decisão — `%` como relativo, operações aritméticas com `Relative`

### Literal percentual

No eval de `Expr::Numeric` com `Unit::Percent`, o valor produzido é
`Value::Relative(Rel::from_percent(value))` em vez de `Value::Ratio`.

```rust
Unit::Percent => Ok(Value::Relative(Rel::from_percent(value)))
```

Isto reflete que, no subset actual, percentuais são comprimentos relativos.
`Value::Ratio` continua existindo como tipo L1 mas deixa de ser produzido pelo
literal `50%`.

### Operações binárias

```rust
// Relative + Relative, Relative - Relative
(Value::Relative(a), Value::Relative(b)) => Value::Relative(a + b) // / - b

// Relative + Length / Length + Relative
(Value::Relative(r), Value::Length(l)) |
(Value::Length(l), Value::Relative(r)) => Value::Relative(r + l)

// Relative - Length
(Value::Relative(r), Value::Length(l)) => Value::Relative(r - l)

// Relative * Int / Int * Relative
(Value::Relative(r), Value::Int(n)) |
(Value::Int(n), Value::Relative(r)) => Value::Relative(r * n as f64)

// Relative * Float / Float * Relative
(Value::Relative(r), Value::Float(f)) |
(Value::Float(f), Value::Relative(r)) => Value::Relative(r * f)

// Relative / Int, Relative / Float
(Value::Relative(r), Value::Int(n)) => Value::Relative(r / n as f64)
(Value::Relative(r), Value::Float(f)) => Value::Relative(r / f)
```

### Operação unária

```rust
(UnOp::Neg, Value::Relative(r)) => Value::Relative(-r)
```

---

## Fronteiras

- `%` como operador binário de remainder (`5 % 2`) não existe no parser
  actual; se for introduzido, deve continuar a produzir `Int` quando ambos os
  operandos forem `Int`.
- Comparações de `Relative` (`<`, `>`) permanecem sem suporte — requerem
  contexto de layout.
- Cast implícito `Relative → Length` em consumers deve usar `cast_length` e
  propagar `CastError::NeedsContext` quando não houver contexto.

---

## P706 — Operador `in` / `not in`

`BinOp::In`/`BinOp::NotIn` **já existiam no parser** (`entities/ast/expr.rs`)
e chegavam a `eval_binary_op`, mas caíam sempre no braço de fronteira
genérico (`"cannot apply {op:?} to {a} e {b}"`) — nenhuma combinação de
valores estava implementada. Isolado por P705/P706 via `cetz`
(`mark.typ:75,174,200` — `"chave" in dict`; `mark.typ:81,118`,
`draw/projection.typ:187` — `str in (tuple de strings)`).

### Combinações confirmadas contra o vanilla (`file:line` = medição directa)

| `lhs in rhs` | Resultado | Notas |
|---|---|---|
| `Str` in `Dict` | `Bool` — testa se a chave existe | `"a" in (a:1,b:2)` → `true` |
| `Str` in `Str` | `Bool` — testa substring | `"ell" in "hello"` → `true` |
| `any` in `Array` | `Bool` — testa igualdade de elemento (`PartialEq` de `Value`, recursivo — cobre array-de-arrays, `(1,2) in ((1,2),(3,4))` → `true`, e tipos mistos, `none in (none, 0%)` → `true`) | `matrix.typ:252` (`out in _ident`, array de matrizes) |
| `Int` in `Str` | **Erro**: `"cannot apply 'in' to integer and string"` (vanilla) | tipos incompatíveis — medido, não assumido |
| `not in` | Negação lógica das combinações acima | `1 not in (1,2,3)` → `false` |

### Semântica de implementação

- `Str in Dict` → `dict.contains_key(s.as_str())` (`IndexMap`, já com
  `Borrow<str>` via `EcoString`).
- `Str in Str` → `haystack.as_str().contains(needle.as_str())`.
- `any in Array` → `arr.contains(&needle)` — usa `PartialEq` já derivado em
  `Value`, cobre recursivamente arrays aninhados sem código extra.
- Combinação sem braço específico → cai no fronteira genérico já existente
  (mensagem `"cannot apply {op:?} to {a} e {b}"`) — **divergência de
  mecânica aceite** (mensagem, não o "é erro"): o texto exacto diverge do
  vanilla (`"cannot apply 'in' to integer and string"`), mas a
  observável "isto é um erro de tipo" é preservada. Mesmo padrão já
  aceite noutros operadores deste ficheiro (ADR-0107, mecânica diverge de
  propósito).
- `NotIn` implementado via o mesmo cálculo de `In`, negado — evita
  duplicar as 3 combinações.

---

## P713 — `Length / Length` (e `Length / Int|Float`)

`(BinOp::Div, Value::Length(_), Value::Length(_))` não tinha braço —
caía no fronteira genérico (`"cannot apply Div to length and length"`).
Isolado por P710/P711/P712 via `cetz` (`canvas.typ:37-38`:
`assert(length / 1cm != 0, ...)`, e `resolve-number`: `x / length`,
ambos `Length / Length` — `length` já resolvido por `.to-absolute()`,
`em: 0.0`).

### Mecanismo do vanilla (`layout/length.rs:64-72`, `foundations/ops.rs:289-342`)

```rust
pub fn try_div(self, other: Self) -> Option<f64> {
    if self.abs.is_zero() && other.abs.is_zero() {
        Some(self.em / other.em)
    } else if self.em.is_zero() && other.em.is_zero() {
        Some(self.abs / other.abs)
    } else {
        None  // incomensurável — erro no caller, não silencioso
    }
}
```

`div()` (vanilla) verifica `is_zero(&rhs)` **antes** do match, para
**todos** os tipos numéricos, incluindo `Length` — `x / 0cm` erra
"cannot divide by zero" antes de chegar a `try_div`. `Length(a) /
Int(b)` e `Length(a) / Float(b)` são o mesmo agrupamento no vanilla
(`ops.rs:312-314`) — escala uniforme, já implementado no cristalino
via `Length: Div<f64>` (`entities/layout_types.rs:808-813`), só faltava
o braço em `eval_binary_op` para os alcançar a partir do eval.

### Scope-out medido, não assumido — `Value::Ratio`/`Value::Relative` mistos

Vanilla também define `Length/Relative`, `Relative/Length`,
`Ratio/Relative`, `Relative/Ratio` (`ops.rs:315,324,328-330`). **Não
implementados aqui** — `Value::Ratio` não é actualmente produzível por
sintaxe de utilizador no cristalino (`50%` produz `Value::Relative`,
não `Value::Ratio`; `grep` confirma que os únicos construtores de
`Value::Ratio` em `eval`/`stdlib` são `Ratio * Int`/`Int * Ratio`, que
exigem já ter um `Ratio` — circular, sem ponto de entrada). Sem
consumidor medido em `cetz` para estas combinações; um bug por passo
(mesma disciplina de P711/P712).

### Semântica de implementação

```rust
// Div por zero — Length(l) se l.is_zero(), mesmo gate genérico já
// usado por Int(0)/Float(0.0)/Decimal(0).
Value::Length(l) if l.is_zero() => Err("cannot divide by zero")

(BinOp::Div, Value::Length(a), Value::Int(b))    => Ok(Value::Length(a / b as f64))
(BinOp::Div, Value::Length(a), Value::Float(b))  => Ok(Value::Length(a / b))
(BinOp::Div, Value::Length(a), Value::Length(b)) => {
    if a.abs.is_zero() && b.abs.is_zero() { Ok(Value::Float(a.em / b.em)) }
    else if a.em == 0.0 && b.em == 0.0 { Ok(Value::Float(a.abs.to_pt() / b.abs.to_pt())) }
    else { Err("cannot divide these two lengths") }
}
```

Mensagem de erro (`"cannot divide these two lengths"`) segue a
convenção já usada em `try_div_length` do vanilla (mesmo texto),
disponível porque o vanilla o expõe como string literal simples (não
há `hint`/formatação especial a replicar aqui, ao contrário de outros
casos já divergentes por design neste ficheiro).

---

## P720 — `Array + Array` e `Dict + Dict` (concatenação/merge)

`(BinOp::Add, Value::Array(_), Value::Array(_))` e o par `Dict` não
tinham braço — caíam no fronteira genérico (`"cannot apply Add to array
and array"`). Isolado por P719 via `cetz` (`path-util.typ:423,430`,
`bezier.typ:413,522,524`, `hobby.typ:77,78,126` — todos `Array + Array`,
concatenação de coordenadas/segmentos de path).

### Mecanismo do vanilla (`foundations/array.rs:1203-1216`, `dict.rs:388-404`)

```rust
impl Add for Array { fn add(mut self, rhs) -> Self { self += rhs; self } }
impl AddAssign for Array { fn add_assign(&mut self, rhs) { self.0.extend(rhs.0); } }
// idêntico para Dict, sobre IndexMap::extend
```

`Array + Array` = concatenação (`extend`, ordem preservada, sem dedup).
`Dict + Dict` = merge — chave do lado **direito** vence em colisão, mas
**mantém a posição original** da primeira ocorrência (semântica de
`IndexMap::extend`/`insert`: actualiza o valor in-place, não move para o
fim). Medido: `(a: 1, b: 2) + (b: 99, c: 3)` → `(a: 1, b: 99, c: 3)` —
`b` fica na posição 1, valor 99; `c` é acrescentado no fim.

### `Dict + Dict` — sem consumidor confirmado em `cetz`, implementado por ser a mesma função

Grep a `+ (` em `path-util.typ`/`bezier.typ`/`hobby.typ` (os três ficheiros
apontados por P719) não encontra nenhum `Dict + Dict` — só `Array +
Array`. Ainda assim, `Dict + Dict` é implementado no mesmo passo: no
vanilla partilha a mesma posição estrutural (`ops.rs:39-40,140-141`, par
de braços lado a lado) e o custo de o adicionar é uma linha idêntica à
de `Array` — não há scope-out real a fazer sem duplicar artificialmente
a decisão (mesmo raciocínio de P718 para `Value::Args` em `eval_args`).

### Semântica de implementação

```rust
(BinOp::Add, Value::Array(mut a), Value::Array(b)) => { a.extend(b); Ok(Value::Array(a)) }
(BinOp::Add, Value::Dict(mut a), Value::Dict(b))    => { a.extend(b); Ok(Value::Dict(a)) }
```

`IndexMap::extend`/`Vec::extend` já implementam exactamente a semântica
medida (concatenação ordenada para array; override-in-place + append
para dict) — sem lógica adicional.

---

## P722 — `Array * Int` / `Int * Array` (repetição)

`(BinOp::Mul, Value::Array(_), Value::Int(_))` e o par inverso não
tinham braço — caíam no fronteira genérico (`"cannot apply Mul to array
and int"`). Encontrado por P720 durante a sonda de `+`, nas mesmas
linhas de `hobby.typ` (77, 78) que motivaram P720
(`(0,) * (n - 1)`); adiado conscientemente nesse passo por ser operador
distinto.

### Comportamento do vanilla (medido + fonte)

`ops.rs:274-275`:

```rust
(Array(a), Int(b)) => Array(a.repeat(Value::Int(b).cast()?)?),
(Int(a), Array(b)) => Array(b.repeat(Value::Int(a).cast()?)?),
```

- **Ambas as ordens** funcionam (`(0,) * 3` e `3 * (0,)` → `(0, 0, 0)`).
- O `Int` é convertido a `usize` (`cast`): negativo → erro
  `"number must be at least zero"` (`foundations/int.rs:507`).
- `Array::repeat` (`array.rs:140-147`): `len.checked_mul(n)`; overflow
  → `"cannot repeat this array {n} times"`; caso contrário
  `iter().cloned().cycle().take(count)`.
- `n = 0` → array vazio `()`. Array vazio repetido → `()`.
- **`Dict * Int` não existe** no vanilla — medido: `(:) * 2` → erro
  ("cannot multiply dictionary with integer", fronteira genérica do
  vanilla). Scope-out: o cristalino mantém o seu erro genérico de
  fronteira, coerente com os demais pares inválidos de `Mul`.

### Semântica de implementação

Um braço com guarda compartilhada para as duas ordens, espelhando
`Array::repeat` do vanilla:

```rust
(BinOp::Mul, Value::Array(a), Value::Int(n)) | (BinOp::Mul, Value::Int(n), Value::Array(a)) => {
    if n < 0 { return Err("number must be at least zero".into()); }
    let count = a.len().checked_mul(n as usize)
        .ok_or_else(|| format!("cannot repeat this array {n} times"))?;
    Ok(Value::Array(a.iter().cloned().cycle().take(count).collect()))
}
```

---

## P725 — `Length * Int|Float` (as quatro combinações)

Nenhum braço `Mul` com `Length` existia — as quatro combinações
(`Length*Int`, `Int*Length`, `Length*Float`, `Float*Length`) caíam no
fronteira genérico (`"cannot apply Mul to float and length"`). Isolado
por P724 via `cetz` (`canvas.typ:146-147,182-186`: `(x - offset) * length`,
escala de coordenadas).

### Comportamento do vanilla (medido + fonte)

`foundations/ops.rs:238-243`:

```rust
(Length(a), Int(b))   => Length(a * b as f64),
(Length(a), Float(b)) => Length(a * b),
(Int(a), Length(b))   => Length(b * a as f64),
(Float(a), Length(b)) => Length(b * a),
```

Medições da sonda (binário vanilla release):

| Expressão | `repr` vanilla |
|---|---|
| `2.0 * 1pt`, `1pt * 2.0`, `2 * 1pt`, `1pt * 2` | `2pt` |
| `0 * 1pt` | `0pt` |
| `-1 * 1pt` | `-1pt` |
| `3 * 2em` | `6em` |
| `0.5 * (1pt + 1em)` | `0.5pt + 0.5em` |
| `1pt * -0.0` | `-0pt` |
| `1e308 * 1pt`, `1pt * float.inf` | `float.inf * 1pt` — **inf propaga-se, sem erro** |
| `1em * float.inf` | `float.inf * 1em` |
| `1pt * float.nan`, `float.nan * 1pt` | `0pt` — **NaN → 0** |
| `1em * float.nan`, `(1pt + 1em) * float.nan` | `0pt` |

Mecanismo do NaN → 0: `Abs` e `Em` do vanilla embrulham `Scalar`
(`layout/abs.rs:13`, `layout/em.rs:16`), e `Scalar::new` saneia
`if x.is_nan() { 0.0 } else { x }` (`typst-utils/src/scalar.rs:30-32`) —
toda a multiplicação de componentes passa por ele (`scalar.rs:203-209`).
Inf **não** é saneado. Isto é observável ao nível da língua (via
`repr`), logo é paridade (ADR-0107), não mecânica.

### Scope-out medido — `Length * Ratio` / `Ratio * Length`

Vanilla define também `Length * Ratio` e `Ratio * Length`
(`ops.rs:240,243`). **Não implementados** — mesmo raciocínio de P713:
`Value::Ratio` não é produzível por sintaxe de utilizador no cristalino
(`50%` produz `Value::Relative`); sem consumidor medido em `cetz`.

### Semântica de implementação

Dois braços com ordens em guarda partilhada, sobre `Length: Mul<f64>`
já existente (`entities/layout_types.rs:801-806`), com saneamento
NaN → 0 por componente no resultado (paridade do efeito observável de
`Scalar::new`):

```rust
(BinOp::Mul, Value::Length(a), Value::Int(b)) | (BinOp::Mul, Value::Int(b), Value::Length(a)) =>
    Ok(Value::Length(sanitize_length_nan(a * b as f64))),
(BinOp::Mul, Value::Length(a), Value::Float(b)) | (BinOp::Mul, Value::Float(b), Value::Length(a)) =>
    Ok(Value::Length(sanitize_length_nan(a * b))),

// helper local: if x.is_nan() { 0.0 } else { x } em abs e em
```

O saneamento fica **no braço do eval** (não em `Length::mul` de
`entities/layout_types.rs`) por disciplina um-bug-por-passo:
`Length / Float` com NaN (P713) fica com o comportamento actual
(NaN propaga-se) — divergência latente registada no relatório de P725
como candidata a passo futuro. **P739D — FECHADO**: o saneamento foi
estendido a `Length / Int|Float` (`sanitize_length_nan` nos dois braços
Div), após a sonda confirmar alcançabilidade via `calc.inf - calc.inf`
→ NaN (medido: `repr(1pt / NaN)` → `0pt` no vanilla; o cristalino
propagava `float.nan * 1pt + ...`). Zero-divisor mantém o erro
"cannot divide by zero" (paridade exata, medida em ambos).

---

## P728 — Short-circuit de `and`/`or` + `join` em code block

Dois bugs de mecanismo central encontrados na validação final do `cetz`
em P727 (`line((0,0),(2,1))` ausente do PDF), isolados com casos mínimos
independentes de `cetz`. ADR-0114 aplicado: sonda antes desta spec.

### Bug 1 — `and`/`or` sem short-circuit (braço `Expr::Binary` de `eval/mod.rs`)

O dispatch genérico avaliava **os dois operandos** antes de despachar
(`eval/mod.rs:687-692`), violando a semântica da linguagem:
`type(a) == str and a.contains(".")` com `a` array errava
"campo desconhecido em array: 'contains'" em vez de dar `false`.

Mecanismo do vanilla (`typst-eval/src/ops.rs:52-66`, medido):

```rust
let lhs = binary.lhs().eval(vm)?;
// Short-circuit boolean operations.
if (binary.op() == ast::BinOp::And && lhs == false.into_value())
    || (binary.op() == ast::BinOp::Or && lhs == true.into_value())
{
    return Ok(lhs);
}
let rhs = binary.rhs().eval(vm)?;
```

Medições vanilla (binário release): `type(a) == str and a.contains(".")`
(a array) → `false`; `type(a) == array or a.contains(".")` → `true`;
`false and (1/0 == 0)` → `false`; `true or (1/0 == 0)` → `true`
(nenhum erro — o segundo operando não é avaliado). Caso inverso:
`1 and 2` → erro (ambos avaliados; `and` exige Bool — comportamento de
`eval_binary_op` inalterado).

### Implementação (bug 1)

Braço dedicado em `eval_expr` **antes** do dispatch genérico de
`Expr::Binary`:

```rust
Expr::Binary(b) if matches!(b.op(), BinOp::And | BinOp::Or) => {
    let lhs = eval_expr(b.lhs(), ...)?;
    let decided = matches!((b.op(), &lhs),
        (BinOp::And, Value::Bool(false)) | (BinOp::Or, Value::Bool(true)));
    if decided { Ok(lhs) }
    else {
        let rhs = eval_expr(b.rhs(), ...)?;
        operators::eval_binary_op(b.op(), lhs, rhs)...
    }
}
```

### Bug 2 — code block sem `join` (a "anomalia de ordem" de P727)

O braço `Expr::CodeBlock` devolvia só o valor da **última** expressão,
descartando as anteriores. O vanilla acumula com `ops::join`
(`typst-eval/src/code.rs:57`: `output = ops::join(output, value)`).

É este o mecanismo da "anomalia de ordem" notada em P727 — **não
memoização** (refutado: não há `#[comemo::memoize]` no eval de closures;
casos puros de closure reproduzem o erro consistentemente): o body do
canvas cetz `{ line(...); circle(...) }` vale `(closure_line,
closure_circle)` no vanilla mas só `(closure_circle)` no cristalino — a
primeira expressão perdia-se. Medido: `{ (1,); (2,) }` → vanilla `(1, 2)`,
cristalino `(2)`.

Mecanismo do vanilla (`foundations/ops.rs:24-45`):

| Combinação | Resultado |
|---|---|
| `(a, None)` / `(None, b)` | `a` / `b` — None é identidade |
| `Str+Str`, `Symbol±Str`, `Symbol+Symbol` | concatenação de texto |
| `Bytes+Bytes` | concatenação |
| `Content+Content`, `Content±Str/Symbol` | sequência de content |
| `Array+Array` | concatenação (ordem preservada) |
| `Dict+Dict` | merge (direita vence, posição preservada — como P720) |
| `Args+Args` | merge |
| resto | **erro** — medido: `{ 1; 2 }` → vanilla "cannot join integer with integer" |

Medições vanilla adicionais: `{ "a"; "b" }` → `"ab"`;
`{ none; (1,) }` → `(1,)`; `{ (1,); none }` → `(1,)`;
`{ (:); (a: 1) }` → `(a: 1)`; `{ 1; none }` → `1`.

### Implementação (bug 2)

`pub(crate) fn join(lhs: Value, rhs: Value) -> Result<Value, String>` em
`operators.rs` (mirror da tabela acima; `Content::sequence` e
`Content::text` para content; `Symbol` vira texto via `ch`); o braço
`Expr::CodeBlock` de `eval_expr` acumula
`output = operators::join(output, value)?` por expressão (span da
expressão no erro, paridade `.at(span)` do vanilla).

Mensagem de erro: formato do vanilla (`cannot join {a} with {b}`) com os
`type_name()` do cristalino (`int`, `str`, … em vez de `integer`,
`string`) — divergência de texto aceite, mesmo padrão já aceite na
fronteira genérica de `eval_binary_op` (o observável "é erro de tipo"
preservado; ADR-0107 — mecânica diverge de propósito).

### P729 — `join` também entre iterações de `for`/`while`

P728 corrigiu o join **intra-bloco** (`Expr::CodeBlock`). A auditoria P729
(medição construto a construto, ADR-0114) confirmou que o mesmo `join`
acumula **entre iterações** nos corpos de `for` e `while` (vanilla
`typst-eval/src/flow.rs:86` e `:132` — `output = ops::join(output, value)`
por iteração, antes do match de flow). O cristalino descartava o valor do
corpo no `while` (`control_flow.rs:62`) e rejeitava não-`Content` no `for`
(`control_flow.rs:143-154`, erro "corpo do for deve ser content").

Medições vanilla (binário release): `#if true { (1,); (2,) }`,
`#if false { (9,) } else { (1,); (2,) }`, `#let f() = { (1,); (2,) }` +
`#f()`, `#for i in (1,) { (1,); (2,) }`, `#while i < 1 { i += 1; (1,); (2,) }`
→ todos `(1, 2)`. Cristalino pré-P729: `if`/`else`/closure correctos
(delegam em `Expr::CodeBlock`, já corrigido por P728); `for` → erro;
`while` → output vazio.

Reaproveitamento directo de `operators::join` (P728) — sem lógica própria:
`let mut output = Value::None` antes do ciclo, `output = join(output, value)?`
por iteração (span do corpo no erro), `Ok(output)` no fim. No `for`, o
acumulador substitui `parts: Vec<Content>` + `Content::sequence`; os casos
antigos (`Content`/`Str`/`None` no corpo) comportam-se igual pela tabela
de join.

Nota registada (**fora do scope P729** — mecanismo distinto de join): o
vanilla marca `FlowEvent::Return` como condicional no fim de `while`/`for`
(`typst-eval/src/flow.rs:105-108,183-185`); o cristalino só o faz em
`eval_conditional` (P635). Registado em `achados-adiados-cetz.md`.

---

## Critérios de Verificação

```rust
eval("#let x = 50%")        → Value::Relative(Rel::from_percent(50.0))
eval("#let x = 100% - 1em") → Value::Relative(Rel::from_percent(100.0) - Length::em(1.0))
eval("#let x = 50% + 2cm")  → Value::Relative(Rel::from_percent(50.0) + Length::cm(2.0))
eval("#let x = 50% * 2")    → Value::Relative(Rel::from_percent(100.0))

eval_binary_op(Add, Relative(50%), Length(2cm))
    == Relative(50% + 2cm)

eval_unary_op(Neg, Relative(50%)) == Relative(-50%)

// P706 — in / not in
eval_binary_op(In, Str("a"), Dict{a:1,b:2})     == Bool(true)
eval_binary_op(In, Str("z"), Dict{a:1,b:2})     == Bool(false)
eval_binary_op(In, Str("ell"), Str("hello"))    == Bool(true)
eval_binary_op(In, Int(1), Array[1,2,3])        == Bool(true)
eval_binary_op(In, Array[1,2], Array[Array[1,2],Array[3,4]]) == Bool(true)
eval_binary_op(NotIn, Int(5), Array[1,2,3])     == Bool(true)
eval_binary_op(In, Int(1), Str("hello"))        == Err (tipos incompatíveis)

// P713 — Length / Length, Length / Int|Float
eval_binary_op(Div, Length(2cm), Length(1cm))   == Float(2.0)
eval_binary_op(Div, Length(0em+4em-part), Length(0em+2em-part)) == Float(2.0)  // rácio de em
eval_binary_op(Div, Length(10pt+1em), Length(5pt)) == Err (incomensurável)
eval_binary_op(Div, Length(10pt), Length(0pt))  == Err ("cannot divide by zero")
eval_binary_op(Div, Length(10pt+4em), Int(2))   == Length(5pt+2em)
eval_binary_op(Div, Length(10pt), Float(4.0))   == Length(2.5pt)

// P720 — Array + Array, Dict + Dict
eval_binary_op(Add, Array[1,2], Array[3,4])     == Array[1,2,3,4]
eval_binary_op(Add, Array[], Array[1,2])        == Array[1,2]
eval_binary_op(Add, Array[1,2], Array[])        == Array[1,2]
eval_binary_op(Add, Dict{a:1}, Dict{b:2})       == Dict{a:1,b:2}
eval_binary_op(Add, Dict{a:1,b:2}, Dict{b:99,c:3}) == Dict{a:1,b:99,c:3}  // ordem preservada

// P722 — Array * Int / Int * Array (repetição)
eval_binary_op(Mul, Array[0], Int(3))           == Array[0,0,0]
eval_binary_op(Mul, Int(3), Array[0])           == Array[0,0,0]   // ordem inversa
eval_binary_op(Mul, Array[1,2], Int(0))         == Array[]
eval_binary_op(Mul, Array[], Int(5))            == Array[]
eval_binary_op(Mul, Array[1,2], Int(-1))        == Err ("number must be at least zero")
eval_binary_op(Mul, Dict{:}, Int(2))            == Err (fronteira genérica — Dict * Int não existe no vanilla)

// P725 — Length * Int|Float (quatro combinações)
eval_binary_op(Mul, Length(1pt), Int(2))         == Length(2pt)
eval_binary_op(Mul, Int(2), Length(1pt))         == Length(2pt)
eval_binary_op(Mul, Length(1pt), Float(2.5))     == Length(2.5pt)
eval_binary_op(Mul, Float(2.5), Length(1pt))     == Length(2.5pt)
eval_binary_op(Mul, Int(0), Length(1pt))         == Length(0pt)
eval_binary_op(Mul, Int(-1), Length(1pt))        == Length(-1pt)
eval_binary_op(Mul, Int(3), Length(2em))         == Length(6em)
eval_binary_op(Mul, Float(0.5), Length(1pt+1em)) == Length(0.5pt+0.5em)
eval_binary_op(Mul, Length(1pt), Float(NaN))     == Length(0pt)   // NaN → 0 (paridade Scalar::new)
eval_binary_op(Mul, Length(1em), Float(NaN))     == Length(0pt)
eval_binary_op(Mul, Length(1pt), Float(inf))     == Length(inf pt) // inf propaga-se

// P728 — short-circuit and/or (via eval de markup)
eval("#(false and (1/0 == 0))")                  == Content("false")
eval("#(true or (1/0 == 0))")                    == Content("true")
eval("#(type((1,2)) == str and (1,2).at(9))")    == Content("false") // rhs não avaliado
eval("#(true and 2)")                            == Content("2")     // caso comum
eval("#(false or 3)")                            == Content("3")

// P728 — join em code block (via eval de markup)
eval("#let x = { (1,); (2,) }; #repr(x)")        == "(1, 2)"
eval("#let x = { \"a\"; \"b\" }; #repr(x)")      == "\"ab\""
eval("#let x = { none; (1,) }; #repr(x)")        == "(1,)"
eval("#let x = { (1,); none }; #repr(x)")        == "(1,)"
eval("#let x = { (:); (a: 1) }; #repr(x)")       == "(a: 1)"
eval("#let x = { 1; none }; #repr(x)")           == "1"
eval("#let x = { 1; 2 }")                        == Err (cannot join)

// P728 — join unitário (operators::join)
join(Str("a"), Str("b"))                         == Str("ab")
join(Array[1], Array[2])                         == Array[1, 2]
join(Dict{a:1}, Dict{b:2})                       == Dict{a:1, b:2}
join(Int(1), Int(2))                             == Err (cannot join)
join(Content([a]), Content([b]))                 == Content([a b])

// P729 — join entre iterações de for/while (via eval de markup)
eval("#let x = for i in (1,) { (1,); (2,) } #repr(x)")         == "(1, 2)"
eval("#let i = 0 #let x = while i < 1 { i += 1; (1,); (2,) } #repr(x)") == "(1, 2)"
eval("#let x = for i in (1, 2) { (i,) } #repr(x)")             == "(1, 2)"  // join entre iterações
eval("#let x = for i in (1,) { 1; 2 }")                        == Err (cannot join)
eval("#for i in (1, 2) [x]")                                   == Content("xx")  // sem regressão
```

---

## Resultado Esperado

- `operators.rs` com braços para `Value::Relative` em `eval_binary_op` e
  `eval_unary_op`.
- `eval/mod.rs` mapeando `Unit::Percent` para `Value::Relative`.
- Testes E2E e unitários cobrindo as expressões acima.

---

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-06-25 | Criação — P469 operadores para `Rel<Length>` | `operators.rs`, `eval/mod.rs`, `tests.rs` |
| 2026-07-11 | P706 — `in`/`not in` para `Str`/`Dict`/`Array` (isolado via `cetz`) | `operators.rs`, `tests.rs` |
| 2026-07-11 | P713 — `Length / Length` (paridade `try_div`), `Length / Int\|Float`; scope-out `Ratio`/`Relative` mistos (não produzíveis por sintaxe de utilizador) | `operators.rs`, `tests.rs` |
| 2026-07-12 | P720 — `Array + Array` (concatenação) e `Dict + Dict` (merge, direita vence, posição preservada); isolado via `cetz` | `operators.rs`, `tests.rs` |
| 2026-07-13 | P722 — `Array * Int` e `Int * Array` (repetição, paridade `Array::repeat`); scope-out `Dict * Int` (inexistente no vanilla); isolado via `cetz` (`hobby.typ:77,78`) | `operators.rs`, `tests.rs` |
| 2026-07-13 | P725 — `Length * Int\|Float` (quatro combinações, paridade `ops.rs:238-243`); NaN → 0 por componente (paridade `Scalar::new`), inf propaga-se; scope-out `Length * Ratio` (não produzível); isolado via `cetz` (`canvas.typ:146-147,182-186`) | `operators.rs`, `tests.rs` |
| 2026-07-13 | P728 — short-circuit `and`/`or` (paridade `typst-eval/ops.rs:52-66`, braço dedicado em `eval_expr`); `join` em code block (paridade `typst-eval/code.rs:57` + `foundations/ops.rs:24-45`); a "anomalia de ordem" de P727 era o bug 2, não memoização; isolado via `cetz` (`canvas` body) | `operators.rs`, `eval/mod.rs`, `eval/tests.rs` |
| 2026-07-13 | P729 — `join` entre iterações de `for`/`while` (paridade `typst-eval/flow.rs:86,132`); reaproveita `operators::join` de P728; `while` deixava de descartar o corpo, `for` deixava de exigir `Content`; nota: marcação `Return` condicional em loops fica registada como achado fora de scope | `control_flow.rs`, `eval/tests.rs` |
