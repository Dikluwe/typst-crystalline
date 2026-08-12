# Prompt L0 — `entities/value`
Hash do Código: 1df06e80

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/value.rs`
**Criado em**: 2026-03-22 (Passo 13)
**Atualizado em**: 2026-07-10 (P685 — `Value::Type(Type)`: nomes de tipo como valores de primeira classe; `type(x)` devolve `Value::Type`; bindings globais `int`/`length`/`ratio`/...)
**ADRs relevantes**: ADR-0017 (adiamento eval), ADR-0023 (indexmap em L1), ADR-0024 (EcoString em Value::Str), ADR-0025 (Int == Float), ADR-0028/ADR-0029 (tipos tipográficos), ADR-0107 (paridade linguagem), ADR-0117 Cláusula 4 (`repr()` função pura)

---

## Contexto

`Value` é o **tipo de runtime** do compilador Typst — o enum que representa
todos os valores possíveis durante a avaliação (`eval.rs`). O original tem
~35 variantes. A migração adiciona variantes incrementalmente, protegida pela
regra: **não adicionar variantes sem ADR e tipo migrado**.

### Estado actual (Passo 25 + P469 + P509)

19 variantes implementadas:

- **Passo 13** — 5 primitivos: `None`, `Bool`, `Int`, `Float`, `Str`
- **Passo 15** — 4 variantes compostas: `Array`, `Dict`, `Module`, `Datetime`
- **Passo 16** — `Func`
- **Passo 18** — `Content`
- **Passo 25** — 5 tipos tipográficos: `Auto`, `Length`, `Ratio`, `Angle`, `Color`
- **P469** — `Relative(Rel<Length>)` (comprimento relativo: `50%`, `100% - 1em`)
- **Passo 395** — `Tiling` (padrão de azulejos; abertura ADR-0017)
- **P509** — `Label(Label)` (etiqueta `<name>` como valor de primeira classe)
- **P576** — `Dir(Dir)` (direcção de texto `ltr` / `rtl` / `ttb` / `btt`; requer `Dir` migrado em L1, ADR-0033)
- **P685** — `Type(Type)` (nome de tipo como valor de primeira classe; `type(x)` e os bindings globais `int`/`length`/`...`)

~10 variantes futuras permanecem comentadas no código (não implementar sem ADR).

---

## Decisão sobre EcoString (ADR-0024)

`Value::Str` usa `EcoString` (crate `ecow`), não `String`. Durante `eval()`,
strings são passadas como argumentos, capturadas em closures e concatenadas
frequentemente. Clone com `EcoString` é O(1) para strings não mutadas
(copy-on-write). Com `String` seria O(n) em cada passagem — degradação
de performance no hot path.

Contraste com ADR-0015 (que removeu `ecow` do parser): o parser constrói strings
uma vez; `Value::Str` é clonado no hot path de `eval()`. Contextos opostos.

## Decisão sobre Dict (ADR-0023)

`Value::Dict` usa `IndexMap<EcoString, Value, FxBuildHasher>` para preservar
ordem de inserção e manter ergonomia com o Typst original. `indexmap` e
`rustc_hash` são autorizados em L1.

## Decisão sobre igualdade Int/Float (ADR-0025)

`Value` usa `#[derive(PartialEq)]`, portanto `Value::Int(1) == Value::Float(1.0)`
é `false` em Rust. Para manter paridade com a semântica do Typst original,
`eval_binary_op` em `eval.rs` trata `Eq`/`Neq` com tipos mistos explicitamente
(coerção `i as f64`). **Dois sistemas de igualdade coexistem**:
igualdade Rust (para testes e estruturas de dados) vs igualdade Typst (em eval).

---

## Decisão sobre `Value::Type` (P685)

Em Typst 0.15, os nomes de tipo (`int`, `float`, `length`, `ratio`, `str`,
`array`, `dictionary`, `function`, `content`, `type`, ...) são **valores de
primeira classe** no scope global, e `type(x)` devolve **esse valor** — não
uma string. Medido na fonte (`lab/typst-original`, sonda P685):

- `type(1)` renderiza `int`; `type(1) == int` → `true`.
- `repr(int)` → `int` (sem aspas — logo **não** é `Value::Str`).
- `type(int)` → `type`; `type(type)` → `type`; `type(rgb)` → `function`.
- `length == ratio` → `false` (tipos distintos).
- Construtores (`int`, `float`, `str`, `type`) são **chamáveis**; os restantes
  tipos (`bool`, `length`, `array`, `dictionary`, ...) **não** são chamáveis
  (`bool(1)` → `type boolean does not have a constructor`).

Para reproduzir isto, `Value` ganha a variante `Type(Type)`, onde `Type` é um
enum `Copy` (sem payload) com uma variante por tipo. `type(x)` passa a devolver
`Value::Type(x.type_of())`; os nomes de tipo são registados no scope global como
`Value::Type(...)`. A igualdade é a de `#[derive(PartialEq)]` (já existente em
`Value`), logo `type(1) == int` funciona **sem** braço especial em `eval_binary_op`.

**Mapeamento** (`Value::type_of`):

| `Value` | `Type` | `Type::name()` |
|---|---|---|
| `None` | `None` | `"none"` |
| `Bool` | `Bool` | `"bool"` |
| `Int` | `Int` | `"int"` |
| `Float` | `Float` | `"float"` |
| `Str` | `Str` | `"str"` |
| `Array` | `Array` | `"array"` |
| `Dict` | `Dictionary` | `"dictionary"` |
| `Module` | `Module` | `"module"` |
| `Datetime` | `Datetime` | `"datetime"` |
| `Func` | `Function` | `"function"` |
| `Content` | `Content` | `"content"` |
| `Auto` | `Auto` | `"auto"` |
| `Length` | `Length` | `"length"` |
| `Relative` | `Relative` | `"relative"` |
| `Ratio` | `Ratio` | `"ratio"` |
| `Angle` | `Angle` | `"angle"` |
| `Color` | `Color` | `"color"` |
| `Stroke` | `Stroke` | `"stroke"` |
| `Fraction` | `Fraction` | `"fraction"` |
| `Align` | `Alignment` | `"alignment"` |
| `Location` | `Location` | `"location"` |
| `Gradient` | `Gradient` | `"gradient"` |
| `Regex` | `Regex` | `"regex"` |
| `Tiling` | `Tiling` | `"tiling"` |
| `Bytes` | `Bytes` | `"bytes"` |
| `Decimal` | `Decimal` | `"decimal"` |
| `Duration` | `Duration` | `"duration"` |
| `Version` | `Version` | `"version"` |
| `Selector` | `Selector` | `"selector"` |
| `Symbol` | `Symbol` | `"symbol"` |
| `Args` | `Arguments` | `"arguments"` |
| `State` | `State` | `"state"` |
| `Counter` | `Counter` | `"counter"` |
| `Label` | `Label` | `"label"` |
| `Dir` | `Direction` | `"direction"` |
| `Type(_)` | `Type` | `"type"` |

**P842 (achado #32 de P831)** — a nota anterior (`Value::Relative` →
`Type::Length`, "`type(50% + 1pt) == length` medido no vanilla") estava
**errada** — refutada por nova medição (`temp/p842/l1_type_*.typ`):
`type(50%)` → `ratio`, `type(50% + 0pt)` → `relative`, `type(30% + 1em)` →
`relative`. Desde P842 o literal percentual (`50%`) é `Value::Ratio`
(`eval/mod.rs`, `Unit::Percent`), `Type::Relative` existe no enum e o
binding global `relative` está registado. `Ratio + Length` constrói
`Value::Relative` nos operadores (ver `engine/eval/operators/arithmetic.md`).

**Chamabilidade** (despachada em `eval_func_call`, `rules/eval/closures.rs`):
`Type::Int`/`Float`/`Str`/`Type` invocam o construtor nativo correspondente
(`native_int`/`native_float`/`native_str`/`native_type`); qualquer outro `Type`
→ erro eval `"type {name} does not have a constructor"`.

**Field access** (despachado em `eval_field_access`, `rules/eval/bindings.rs`):
`Type::Int` expõe `.min` / `.max` (`i64::MIN`/`MAX`); `Type::Str` expõe
`.from-unicode` (função nativa). Estes campos existiam antes via
`Func::native_with_namespace` e passam a ser resolvidos directamente pelo
`Type`, preservando `int.min`, `int.max` e `str.from-unicode` sem regressão.

**Débito conhecido (não bloqueante para cetz):** nomes de tipo que **já**
estavam registados no scope como função/módulo (`color`, `gradient`, `stroke`,
`regex`, `tiling`, `decimal`, `duration`, `version`, `label`, `state`,
`counter`, `selector`) **não** são convertidos para `Value::Type` neste passo
(fazê-lo quebraria `color.rgb(...)`, `gradient.linear(...)`, `regex(...)`,
etc.). Para esses, `type(x) == color` permanece `false` até um passo futuro que
reconcilie módulo/função com `Value::Type`.

---

## Restrições Estruturais

- Camada **L1**: zero I/O. `Arc` em `Module`, `Func` é gestão de RAM (ADR-0029).
- Fronteira deliberada: `_ => Ok(Value::None)` em `eval.rs` para variantes não
  implementadas — não adicionar variantes sem ADR.
- `serde` nunca entra em L1 — serialização é responsabilidade de L3 via DTO.
- `Value::Array` usa `Vec<Value>` — clone O(n). Documentado como dívida técnica.
  `Value::Dict` usa `IndexMap` com `FxBuildHasher`.

---

## Interface pública

### `repr()` (P465)

Todos os variants existentes de `Value` têm representação via `repr_value`
(`rules/eval/repr.rs`). A saída é **reconhecível**, não garante round-trip.
Variants opacos (`Location`, `Gradient`, `Tiling`) usam placeholder
(`location(...)`, `gradient(...)`, `tiling(...)`). `Value::Type(t)` tem
`repr()` igual ao nome do tipo (`int`, `length`, `type`, ...) — ver
"Decisão sobre `Value::Type`" abaixo. `Symbol` ainda não é variante de
`Value` em L1; quando migrado terá `repr()` mapeado.

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    // Passo 13 — primitivos
    None,
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(EcoString),              // ADR-0024

    // Passo 15 — compostos
    Array(Vec<Value>),
    Dict(IndexMap<EcoString, Value, FxBuildHasher>),  // ADR-0023
    Module(Module),              // Arc<ModuleInner> internamente — clone O(1)
    Datetime(Datetime),

    // Passo 16
    Func(Func),                  // Arc<FuncRepr> internamente — clone O(1)

    // Passo 18
    Content(Content),

    // Passo 25 — tipos tipográficos (ADR-0028/0029)
    Auto,
    Length(Length),
    Relative(Rel<Length>),     // P469 — comprimento relativo
    Ratio(Ratio),
    Angle(Angle),
    Color(Color),

    // Passo 395 — Tiling (padrão de azulejos; abertura ADR-0017)
    Tiling(Arc<Tiling>),

    // P509 — Label como valor de primeira classe (query/locate com <label>)
    Label(Label),

    // P685 — nome de tipo como valor de primeira classe (int, length, type, …)
    Type(Type),

    // ~11 variantes futuras comentadas — NÃO implementar sem ADR e tipo migrado
}

/// P685 — enumerador fechado dos tipos Typst visíveis como valor.
/// `Copy` (sem payload); `Type::name()` coincide com `Value::type_name()`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Type {
    None, Auto, Bool, Int, Float, Str, Array, Dictionary, Module, Datetime,
    Function, Content, Length, Ratio, Angle, Color, Stroke, Fraction,
    Alignment, Location, Gradient, Regex, Tiling, Bytes, Decimal, Duration,
    Version, Selector, Symbol, Arguments, State, Counter, Label, Direction,
    Type,
}

impl Type {
    pub fn name(&self) -> &'static str  // "none", "bool", …, "type"
}

impl Value {
    pub fn type_name(&self) -> &'static str  // idem anterior + Self::Type(_) => "type"
    pub fn type_of(&self) -> Type            // P685 — mapeamento da tabela acima
    pub fn is_none(&self) -> bool
    pub fn truthy(&self) -> bool             // P466: semântica Typst minimal
    pub fn cast_bool(&self)  -> Option<bool>
    pub fn cast_int(&self)   -> Option<i64>
    pub fn cast_float(&self) -> Option<f64>  // aceita Int (coerção implícita)
    pub fn cast_str(&self)   -> Option<&str>
    pub fn cast_array(&self) -> Option<&[Value]>
    pub fn cast_dict(&self)  -> Option<&IndexMap<EcoString, Value, FxBuildHasher>>
}

// P466 — métodos de coleção são despachados por `try_dispatch_collection_method`
// em `rules/stdlib/collections.rs`, invocado desde `eval_func_call` para sintaxe
// de método de instância (ex.: `(1, 2, 3).first()`, `"ab".repeat(3)`).
// O eval de dict literal `(a: 1, b: 2)` foi materializado no mesmo passo.

// Conversões From para ergonomia em eval() e testes
impl From<bool>       for Value
impl From<i64>        for Value
impl From<i32>        for Value
impl From<f64>        for Value
impl From<EcoString>  for Value
impl From<&str>       for Value
impl From<String>     for Value
impl From<Vec<Value>> for Value
impl From<IndexMap<EcoString, Value, FxBuildHasher>> for Value
impl From<Module>     for Value
impl From<Datetime>   for Value
impl From<Func>       for Value
impl From<Content>    for Value
impl From<Length>     for Value
impl From<Rel<Length>> for Value  // P469
impl From<Ratio>      for Value
impl From<Angle>      for Value
impl From<Color>      for Value
impl From<Tiling>     for Value   // Passo 395
impl From<Type>       for Value   // P685
```

---

## Critérios de Verificação

```
Value::None.type_name()         = "none"
Value::None.is_none()           = true
Value::Int(0).is_none()         = false

Value::Int(3).cast_float()      = Some(3.0)     // coerção implícita
Value::Float(3.14).cast_float() = Some(3.14)
Value::Bool(true).cast_float()  = None

Value::from("hello")            → Value::Str(EcoString)
Value::from(42i64)              → Value::Int(42)
Value::from(3.14f64)            → Value::Float(3.14)
Value::from(true)               → Value::Bool(true)

// EcoString — clone O(1) e igualdade
Value::Str("a".into()) == Value::Str("a".into())  = true
Value::Str("a".into()) != Value::Str("b".into())

// Array — clone O(n), independência garantida
let v1 = Value::Array(vec![Value::Int(1)]);
let mut v2 = v1.clone();
// mutar v2 não afecta v1

// Dict
Value::Dict(m).type_name() = "dictionary"
cast_dict().unwrap().get("k") = Some(&Value::Int(1))

// Module — clone O(1) via Arc
Value::Module(m1) clone → type_name() ainda "module"

// Tipos tipográficos (Passo 25 + P469)
Value::Length(Length::pt(12.0)).type_name()     = "length"
Value::Relative(Rel::from_percent(50.0)).type_name() = "relative length"
Value::Ratio(Ratio(0.5)).type_name()            = "ratio"
Value::Angle(Angle::deg(90.0)).type_name()      = "angle"
Value::Color(Color::rgb(0,0,0)).type_name()     = "color"
Value::Auto.type_name()                         = "auto"
Value::Tiling(...).type_name()                  = "tiling"

// Scope integration
Scope::define("x", Value::Int(42))
scope.get("x") = Some(&Value::Int(42))
```

---

## Variantes futuras (comentadas no código — NÃO implementar sem ADR)

`Fraction`, `Gradient`, `Symbol`, `Version`,
`Bytes`, `Decimal`, `Duration`, `Styles`, `Args`, `Dyn` (~10 restantes)

---

## Resultado Esperado

- `01_core/src/entities/value.rs` com todas as variantes até ao Passo 25
- Testes co-localizados em `#[cfg(test)]` cobrindo cada variante e os critérios acima

---

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-03-22 | Criação — Passo 13: 5 primitivos | `value.rs` |
| 2026-03-24 | Passo 15: Array, Dict, Module, Datetime | `value.rs` |
| 2026-03-25 | Passo 16: Func | `value.rs` |
| 2026-03-26 | Passo 18: Content | `value.rs` |
| 2026-03-28 | Passo 25: Auto, Length, Ratio, Angle, Color (ADR-0028) | `value.rs` |
| 2026-06-22 | Passo 395: Tiling (abertura ADR-0017) | `value.rs`, `tiling.rs` |
| 2026-04-12 | Restauro — prompt expandido para refletir Passos 15–25; sem mudanças no código | `value.md` |
| 2026-06-25 | P469: `Value::Relative(Rel<Length>)`, `repr`, cast `NeedsContext` | `value.rs`, `rel.rs`, `repr.rs`, `cast.rs` |
| 2026-07-10 | P685: `Value::Type(Type)` + enum `Type` + `type_of`; `type(x)` e nomes de tipo como valores | `value.rs`, `repr.rs`, `stdlib/foundations.rs`, `eval/mod.rs`, `eval/closures.rs`, `eval/bindings.rs` |
| 2026-07-22 | P842 (#32): `Type::Relative` novo; `type_of` Relative→Relative; literal percentual → `Value::Ratio`; comentário P685 de paridade errada corrigido | `value.rs`, `eval/mod.rs`, `eval/operators.rs` |
