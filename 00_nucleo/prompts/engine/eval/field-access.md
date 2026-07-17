# Prompt L0 — `rules/eval` — Field Access em tipos primitivos e funções com namespace
Hash do Código: 5b54f43b

**Camada**: L1
**Ficheiro alvo**: `01_core/src/engine/eval/bindings.rs` (`eval_field_access`, `eval_element_where`)
**Origem**: Passos 411 e 412 (`Version`/`Duration`), P493 (`Array`, `Func` namespace, `Where` multi-field).
**ADRs relevantes**: ADR-0107 (paridade linguagem), ADR-0033 (paridade vanilla), ADR-0108 (medir-antes-de-decidir), ADR-0109 (atomização).

---

## 1. Contexto

`eval_field_access` em `rules/eval/bindings.rs` faz match no `target` de um `Expr::FieldAccess`. Inicialmente suportava apenas `Value::Dict` e `Value::Content`. Este prompt consolida o field access para:

- Tipos primitivos L1: `Version` (P411), `Duration` (P412), `Array` (P493a);
- Funções com namespace anexado: `Func` (P493b) — `table.header`, `table.footer`, `table.cell`;
- Selectores `Where` multi-field: `eval_element_where` (P493c) — `heading.where(level: 1, outlined: true)`.

---

## 2. Field Access `Version` (P411)

Campos suportados para `Value::Version(v)`:

| Campo | Tipo L1 | Retorno eval |
|---|---|---|
| `.major` | `u64` | `Value::Int` |
| `.minor` | `u64` | `Value::Int` |
| `.patch` | `u64` | `Value::Int` |
| `.pre` | `Vec<EcoString>` | `Value::Array(Str)` |
| `.build` | `Vec<EcoString>` | `Value::Array(Str)` |

Campos `.pre` e `.build` vazios retornam `Value::Array([])`.

---

## 3. Field Access `Duration` (P412)

Campos suportados para `Value::Duration(d)`:

| Field | Semântica | Retorno eval |
|---|---|---|
| `.seconds` | Total de segundos (inclui fração) | `Value::Float` |
| `.minutes` | Total de minutos (inclui fração) | `Value::Float` |
| `.hours` | Total de horas (inclui fração) | `Value::Float` |
| `.days` | Total de dias (inclui fração) | `Value::Float` |

**Nota de forma**: o vanilla usa métodos (`.seconds()`); o cristalino usa fields (`.seconds`) por simplicidade de infra. Semântica idêntica.

---

## 4. Field Access `Array` (P493a)

Campos suportados para `Value::Array(arr)`:

| Campo | Tipo | Retorno eval | Notas |
|---|---|---|---|
| `.len` | `int` | `Value::Int` | Número de elementos. |
| `.first` | `any` | `Value` | Primeiro elemento ou `none`. |
| `.last` | `any` | `Value` | Último elemento ou `none`. |

Os métodos `.dedup()`, `.chunks(n)` e `.windows(n)` são despachados em `try_dispatch_collection_method` (`rules/eval/closures.rs`) como chamadas de método, não como field access. A forma fixada separa **propriedades** (`len`, `first`, `last`) de **métodos estruturais** (`dedup`, `chunks`, `windows`), mantendo a implementação no módulo de coleções.

### Semântica

```rust
Value::Array(arr) => match field.as_str() {
    "len"    => Ok(Value::Int(arr.len() as i64)),
    "first"  => Ok(arr.first().cloned().unwrap_or(Value::None)),
    "last"   => Ok(arr.last().cloned().unwrap_or(Value::None)),
    _ => Err(... "campo desconhecido em array: {field}" ...),
}
```

### `array.dedup()`

Remove elementos adjacentes duplicados (paridade vanilla). Implementado em `stdlib/collections.rs`.

```rust
fn array_dedup(arr: Vec<Value>) -> Vec<Value> {
    let mut result = Vec::new();
    for item in arr {
        if result.last() != Some(&item) {
            result.push(item);
        }
    }
    result
}
```

### `array.chunks(n)`

Divide o array em sub-arrays de tamanho `n`. O último chunk pode ser menor.

- `n <= 0` → erro eval.
- `n >= len` → array com um único chunk (ou vazio se array vazio).

### `array.windows(n)`

Retorna janelas deslizantes de tamanho `n`.

- `n <= 0` → erro eval.
- `n > len` → array vazio.

---

## 5. Field Access `Func` com namespace anexado (P493b)

Algumas funções nativas expõem sub-funções via field access:

| Função | Campos do namespace |
|---|---|
| `table` | `header`, `footer`, `cell` |
| `grid` | `header`, `footer`, `cell` |
| `list` | `item` |
| `enum` | `item` |

### Semântica

```rust
Value::Func(f) => {
    match f.namespace() {
        Some(ns) => ns.get(field.as_str())
            .cloned()
            .ok_or_else(|| err(... "função não tem campo '{field}' ...)),
        None => Err(... "função não tem campos" ...),
    }
}
```

### Construção do namespace

As nativas `table`, `grid`, `list`, `enum` são registadas no scope global com namespace anexado:

```rust
let mut table_ns = Scope::new();
table_ns.define("header", Value::Func(native_table_header));
table_ns.define("footer", Value::Func(native_table_footer));
table_ns.define("cell", Value::Func(native_table_cell));

let table = Value::Func(Func::native_with_namespace(
    "table",
    native_table,
    Arc::new(table_ns),
));
```

**Nota**: `Func::native_with_namespace` é a interface pública para criar funções com namespace (ver `entities/func.md`).

---

## 6. `eval_element_where` multi-field (P493c)

`eval_element_where` avalia `<elemento>.where(field: value, ...)` para funções nativas de elemento suportadas (`heading`, `figure`).

### Semântica

Para cada argumento nomeado, construir um `Selector::Where` encadeado:

```rust
let mut selector = Selector::Kind(kind);
for (field, value) in args_named {
    selector = Selector::Where {
        base: Box::new(selector),
        field,
        value,
    };
}
Ok(Some(selector))
```

- Zero argumentos nomeados → erro eval (`where() requer pelo menos um argumento nomeado`).
- Argumentos posicionais → erro eval (`where() requer argumentos nomeados`).
- Múltiplos argumentos nomeados → encadeamento de `Selector::Where`.

### Elementos suportados

- `heading` → `ElementKind::Heading`
- `figure` → `ElementKind::Figure`
- `strong`, `emph`, `raw` → erro explícito (`selector where não suportado para X`) ou scope-out.

### Show-rule matching

O matching de `Selector::Where` em show rules já deve funcionar (P417/P467). Se o matching de `Where` encadeado não estiver implementado, implementá-lo em `rules/eval/rules.rs::apply_show_rules` ou no consumer de `Selector` adequado.

---

## 7. Erros

- Campo desconhecido → erro eval com mensagem clara indicando o tipo (`version`, `duration`, `array`, `function`).
- Field access em tipo não suportado → erro existente.
- `array.chunks(n)` / `array.windows(n)` com `n <= 0` → erro eval.

---

## 8. Scope-out

- Não adicionar `.raw` ou `.string` em Version.
- Não implementar field access mutável (set).
- Não adicionar `get_field` em `entities/version.rs` nem `entities/duration.rs`.
- Não implementar `.milliseconds` nem `.nanoseconds` em Duration.
- Não tocar em `Decimal`.
- `array.dedup`/`chunks`/`windows` não suportam closure/key neste passo.
- Namespace anexado não se aplica a closures nem a elementos de utilizador neste passo.

---

## 9. Critérios de Verificação

```
Dado (3, 1, 4, 1, 5).dedup()
Então (3, 1, 4, 1, 5)

Dado (3, 1, 4, 1, 5, 9, 2, 6).dedup()
Então (3, 1, 4, 5, 9, 2, 6)

Dado (1, 2, 3, 4).chunks(2)
Então ((1, 2), (3, 4))

Dado (1, 2, 3).windows(2)
Então ((1, 2), (2, 3))

Dado table.header
Então Value::Func(native_table_header)

Dado heading.where(level: 1, outlined: true)
Então Selector::Where { base: Where { base: Kind(Heading), field: "level", value: Int(1) }, field: "outlined", value: Bool(true) }

Dado version("1.2.3").major
Então Value::Int(1)

Dado duration("1h30m").seconds
Então Value::Float(5400.0)
```

---

## 10. Testes

- Version: `version_field_major`, `version_field_minor`, `version_field_patch`, `version_field_pre`, `version_field_pre_empty`, `version_field_build`, `version_field_build_empty`, `version_field_unknown`.
- Duration: `duration_field_seconds_zero`, `duration_field_seconds_simple`, `duration_field_seconds_compound`, `duration_field_seconds_fraction`, `duration_field_minutes`, `duration_field_minutes_compound`, `duration_field_hours`, `duration_field_days`, `duration_field_days_zero`, `duration_field_unknown`.
- Array field access: `p493_array_dedup`, `p493_array_chunks`, `p493_array_windows`, `p493_array_len_first_last`.
- Func namespace: `p493_table_header_field`, `p493_table_footer_field`, `p493_table_cell_field`.
- Where multi-field: `p493_heading_where_multi`, `p493_figure_where_multi`, `p493_heading_where_single_regression`.


---

## 11. Field Access `Module` (P679)

Medição na fonte vanilla 0.15.0 (`lab/typst-original/crates/typst-eval/src/import.rs`):

- `import.rs:77` — `let scope = source.scope().unwrap();`: um `Value::Module` expõe o seu
  `Scope` via `Value::scope()`.
- `import.rs:109` / `import.rs:120` — `scope.iter()` / `scope.get(component)`: a resolução
  de um nome num módulo é lookup no `Scope` do módulo. O field access `#mod.campo` usa o
  mesmo `Scope` (typst-eval resolve `FieldAccess` em `Value::Module` por lookup no scope).

Classificação (ADR-0108): o acesso `#mod.campo` é **morfologia** da linguagem (paridade);
a mensagem de erro para campo ausente é **observável mecânico**.

Semântica a adicionar em `eval_field_access` (`01_core/src/engine/eval/bindings.rs`), novo
armo no `match target`:

```rust
Value::Module(m) => m.scope().get(field.as_str()).cloned().ok_or_else(|| {
    vec![SourceDiagnostic::error(
        access.span(),
        format!("módulo '{}' não tem campo '{}'", m.name(), field),
    )]
}),
```

Isto habilita as formas `#import "u.typ"` (bare) e `#import "u.typ" as u` de P679: depois
de ligar o módulo no escopo, `#u.saudacao("Mundo")` resolve `saudacao` no scope do módulo
e chama a função resultante. A chamada (`#u.saudacao(...)`) já funciona porque o valor
obtido é `Value::Func` e o dispatcher de chamada (`apply_func`) trata `Value::Func`.

Critérios de verificação:

- `#import "u.typ"` + `#u.saudacao("Mundo")` → `Olá, Mundo!` (onde `u` é o file_stem).
- `#import "u.typ" as u` + `#u.saudacao("Mundo")` → `Olá, Mundo!`.
- `#u.inexistente` → erro `módulo 'u' não tem campo 'inexistente'`.
- Field access nos tipos já suportados (Version/Duration/Array/Func/Dict/Content) sem
  regressão.

---

## 12. Field Access `Type` (P685)

Com `int`, `float`, `str`, `type` registados como `Value::Type` (P685), os
campos antes expostos via `Func::native_with_namespace` passam a ser resolvidos
directamente pelo `Type` em `eval_field_access`.

| `Type` | Campo | Retorno eval |
|---|---|---|
| `Int` | `min` | `Value::Int(i64::MIN)` |
| `Int` | `max` | `Value::Int(i64::MAX)` |
| `Str` | `from-unicode` | `Value::Func(native_str_from_unicode)` |

Qualquer outro `Type` (ou campo desconhecido) → erro eval
`"type {name} não tem campo '{field}'"` / `"type {name} não tem campos"`.

Semântica:

```rust
Value::Type(t) => match (t, field.as_str()) {
    (Type::Int, "min") => Ok(Value::Int(i64::MIN)),
    (Type::Int, "max") => Ok(Value::Int(i64::MAX)),
    (Type::Str, "from-unicode") => Ok(Value::Func(Func::native(
        "str.from-unicode", native_str_from_unicode))),
    (Type::Int | Type::Str, _) => Err(... "type {name} não tem campo '{field}'" ...),
    _ => Err(... "type {name} não tem campos" ...),
}
```

Critérios de verificação:

- `int.min` → `Value::Int(i64::MIN)`; `int.max` → `Value::Int(i64::MAX)`.
- `str.from-unicode(97)` → `"a"` (a função obtida é chamável via `apply_func`).
- `int.x` → erro; `length.min` → erro (`length` não tem campos).
- Sem regressão em `table.header`, `grid.cell`, `list.item`, `enum.item`
  (continuam via `Value::Func` namespace).
