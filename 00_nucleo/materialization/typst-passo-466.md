---

# P466 — Métodos restantes de `array`, `dict`, `str`

> **Passo:** 466  
> **Data:** 2026-06-25  
> **Foco:** Materializar os métodos de `array`, `dict` e `str` que estão parcialmente implementados ou ausentes, alcançando paridade funcional com o Typst vanilla nos tipos de coleção.  
> **Trilha:** 8 — Refinos de stdlib e tipos.  
> **Tipo:** Materialização / Refacto mecânico.  
> **Tamanho:** S (~20 min).  
> **ADR-0117 Cláusula 4:** Métodos de tipos existentes; não propõe estrutura em elementos.

---

## Contexto

O Typst vanilla tem métodos extensivos para `array`, `dict` e `str`. O cristalino tem o subset básico (`.len()`, `.at()`, `.join()`, `.keys()`, `.values()`), mas **falta**:

### `array` — métodos ausentes

| Método | Vanilla | Cristalino | Status |
|--------|---------|------------|--------|
| `.first()` | Retorna primeiro elemento | Ausente | ❌ |
| `.last()` | Retorna último elemento | Ausente | ❌ |
| `.rev()` | Inverte ordem | Ausente | ❌ |
| `.sum()` | Soma numérica | Ausente | ❌ |
| `.product()` | Produto numérico | Ausente | ❌ |
| `.min()` | Mínimo | Ausente | ❌ |
| `.max()` | Máximo | Ausente | ❌ |
| `.sorted()` | Ordena (com key opcional) | Ausente | ❌ |
| `.dedup()` | Remove duplicados adjacentes | Ausente | ❌ |
| `.filter(func)` | Filtra por predicado | Ausente | ❌ |
| `.map(func)` | Mapeia por função | Ausente | ❌ |
| `.find(func)` | Primeiro que satisfaz | Ausente | ❌ |
| `.position(func)` | Índice do primeiro que satisfaz | Ausente | ❌ |
| `.any(func)` | Algum satisfaz? | Ausente | ❌ |
| `.all(func)` | Todos satisfazem? | Ausente | ❌ |
| `.zip(other)` | Combina com outro array | Ausente | ❌ |
| `.enumerate()` | (index, value) pairs | Ausente | ❌ |
| `.flatten()` | Achata array de arrays | Ausente | ❌ |

### `dict` — métodos ausentes

| Método | Vanilla | Cristalino | Status |
|--------|---------|------------|--------|
| `.pairs()` | Retorna array de (key, value) | Ausente | ❌ |
| `.remove(key)` | Remove e retorna valor | Ausente | ❌ |
| `.update(other)` | Mescla com outro dict | Ausente | ❌ |

### `str` — métodos ausentes

| Método | Vanilla | Cristalino | Status |
|--------|---------|------------|--------|
| `.contains(substr)` | Contém substring? | Ausente | ❌ |
| `.starts-with(prefix)` | Começa com? | Ausente | ❌ |
| `.ends-with(suffix)` | Termina com? | Ausente | ❌ |
| `.find(substr)` | Índice da primeira ocorrência | Ausente | ❌ |
| `.replace(old, new)` | Substitui substring | Ausente | ❌ |
| `.trim()` | Remove whitespace dos extremos | Ausente | ❌ |
| `.split(sep)` | Divide por separador | Ausente | ❌ |
| `.rev()` | Inverte string | Ausente | ❌ |
| `.repeat(n)` | Repete n vezes | Ausente | ❌ |

Este passo materializa o **subset crítico** que é usado em 90% dos documentos reais, deixando o resto para passos futuros de Trilha 8.

---

## ADR-0108 — Medir antes de decidir

| Pergunta | Resultado | Status |
|----------|-----------|--------|
| `Value::Array` existe com métodos básicos? | Sim — `.len()`, `.at()`, `.join()` | ✅ |
| `Value::Dict` existe com métodos básicos? | Sim — `.len()`, `.at()`, `.keys()`, `.values()` | ✅ |
| `Value::Str` existe com métodos básicos? | Sim — `.len()`, `.at()`, `.slice()` | ✅ |
| `Func` como predicado existe? | Sim — closures nativas (P388+) | ✅ |
| `Value::None` como default existe? | Sim | ✅ |
| Infra de método nativo em `Value`? | Sim — `Value::method(name, args)` | ✅ |
| Bloqueadores? | Nenhum — métodos de tipo puro | ✅ |

**Reclassificação:** S (~20 min; 12 métodos + tests).

**Subset crítico selecionado:**
- `array`: `.first()`, `.last()`, `.rev()`, `.sum()`, `.sorted()`, `.filter()`, `.map()`, `.find()`, `.any()`, `.all()`, `.zip()`, `.enumerate()`
- `dict`: `.pairs()`, `.remove()`, `.update()`
- `str`: `.contains()`, `.starts-with()`, `.ends-with()`, `.find()`, `.replace()`, `.trim()`, `.split()`, `.repeat()`

Total: 23 métodos. Média de <1 min por método (match arm + test).

---

## Toques pontuais

### 1. `array` — métodos em `rules/stdlib/arrays.rs` (ou `rules/stdlib/foundations.rs`)

```rust
// array.first() -> Value
fn array_first(arr: Vec<Value>) -> Value {
    arr.first().cloned().unwrap_or(Value::None)
}

// array.last() -> Value
fn array_last(arr: Vec<Value>) -> Value {
    arr.last().cloned().unwrap_or(Value::None)
}

// array.rev() -> Array
fn array_rev(arr: Vec<Value>) -> Value {
    Value::Array(arr.into_iter().rev().collect())
}

// array.sum() -> Value (Int or Float)
fn array_sum(arr: Vec<Value>) -> Value {
    let mut sum_int = 0i64;
    let mut sum_float = 0.0f64;
    let mut has_float = false;
    for v in arr {
        match v {
            Value::Int(i) => if has_float { sum_float += i as f64 } else { sum_int += i },
            Value::Float(f) => { sum_float += f + sum_int as f64; sum_int = 0; has_float = true; },
            _ => continue,
        }
    }
    if has_float { Value::Float(sum_float) } else { Value::Int(sum_int) }
}

// array.sorted(key: none) -> Array
fn array_sorted(arr: Vec<Value>, key: Option<Func>) -> Value {
    let mut sorted = arr;
    if let Some(key_fn) = key {
        // key-based sort: compute key for each, sort by key
        // Simplification: stable sort with key extraction
        sorted.sort_by(|a, b| {
            let ka = key_fn.call(vec![a.clone()]).unwrap_or(Value::None);
            let kb = key_fn.call(vec![b.clone()]).unwrap_or(Value::None);
            ka.partial_cmp(&kb).unwrap_or(Ordering::Equal)
        });
    } else {
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
    }
    Value::Array(sorted)
}

// array.filter(func) -> Array
fn array_filter(arr: Vec<Value>, pred: Func) -> Value {
    Value::Array(arr.into_iter().filter(|v| {
        pred.call(vec![v.clone()]).map(|r| r.truthy()).unwrap_or(false)
    }).collect())
}

// array.map(func) -> Array
fn array_map(arr: Vec<Value>, func: Func) -> Value {
    Value::Array(arr.into_iter().filter_map(|v| func.call(vec![v]).ok()).collect())
}

// array.find(func) -> Value
fn array_find(arr: Vec<Value>, pred: Func) -> Value {
    arr.into_iter().find(|v| {
        pred.call(vec![v.clone()]).map(|r| r.truthy()).unwrap_or(false)
    }).unwrap_or(Value::None)
}

// array.any(func) -> Bool
fn array_any(arr: Vec<Value>, pred: Func) -> Value {
    Value::Bool(arr.iter().any(|v| {
        pred.call(vec![v.clone()]).map(|r| r.truthy()).unwrap_or(false)
    }))
}

// array.all(func) -> Bool
fn array_all(arr: Vec<Value>, pred: Func) -> Value {
    Value::Bool(arr.iter().all(|v| {
        pred.call(vec![v.clone()]).map(|r| r.truthy()).unwrap_or(false)
    }))
}

// array.zip(other) -> Array of pairs
fn array_zip(arr: Vec<Value>, other: Vec<Value>) -> Value {
    Value::Array(arr.into_iter().zip(other).map(|(a, b)| {
        Value::Array(vec![a, b])
    }).collect())
}

// array.enumerate() -> Array of (index, value)
fn array_enumerate(arr: Vec<Value>) -> Value {
    Value::Array(arr.into_iter().enumerate().map(|(i, v)| {
        Value::Array(vec![Value::Int(i as i64), v])
    }).collect())
}
```

### 2. `dict` — métodos em `rules/stdlib/dicts.rs`

```rust
// dict.pairs() -> Array of (str, value)
fn dict_pairs(dict: HashMap<EcoString, Value>) -> Value {
    Value::Array(dict.into_iter().map(|(k, v)| {
        Value::Array(vec![Value::Str(k), v])
    }).collect())
}

// dict.remove(key) -> Value (removed or none)
fn dict_remove(dict: &mut HashMap<EcoString, Value>, key: EcoString) -> Value {
    dict.remove(&key).unwrap_or(Value::None)
}

// dict.update(other) -> Dict
fn dict_update(dict: &mut HashMap<EcoString, Value>, other: HashMap<EcoString, Value>) {
    dict.extend(other);
}
```

### 3. `str` — métodos em `rules/stdlib/strings.rs`

```rust
// str.contains(substr) -> Bool
fn str_contains(s: EcoString, substr: EcoString) -> Value {
    Value::Bool(s.contains(substr.as_str()))
}

// str.starts-with(prefix) -> Bool
fn str_starts_with(s: EcoString, prefix: EcoString) -> Value {
    Value::Bool(s.starts_with(prefix.as_str()))
}

// str.ends-with(suffix) -> Bool
fn str_ends_with(s: EcoString, suffix: EcoString) -> Value {
    Value::Bool(s.ends_with(suffix.as_str()))
}

// str.find(substr) -> Int (or none)
fn str_find(s: EcoString, substr: EcoString) -> Value {
    s.find(substr.as_str()).map(|i| Value::Int(i as i64)).unwrap_or(Value::None)
}

// str.replace(old, new) -> Str
fn str_replace(s: EcoString, old: EcoString, new: EcoString) -> Value {
    Value::Str(s.replace(old.as_str(), new.as_str()).into())
}

// str.trim() -> Str
fn str_trim(s: EcoString) -> Value {
    Value::Str(s.trim().into())
}

// str.split(sep) -> Array of Str
fn str_split(s: EcoString, sep: EcoString) -> Value {
    Value::Array(s.split(sep.as_str()).map(|part| Value::Str(part.into())).collect())
}

// str.repeat(n) -> Str
fn str_repeat(s: EcoString, n: usize) -> Value {
    Value::Str(s.repeat(n).into())
}
```

### 4. Registro no dispatcher de métodos

**Ficheiro:** `entities/value.rs` (ou `rules/stdlib/mod.rs`)

Adicionar braços no `Value::method()`:

```rust
impl Value {
    pub fn method(&self, name: &str, args: Vec<Value>) -> Result<Value, Error> {
        match (self, name) {
            (Value::Array(arr), "first") => Ok(array_first(arr.clone())),
            (Value::Array(arr), "last") => Ok(array_last(arr.clone())),
            (Value::Array(arr), "rev") => Ok(array_rev(arr.clone())),
            (Value::Array(arr), "sum") => Ok(array_sum(arr.clone())),
            (Value::Array(arr), "sorted") => {
                let key = args.get(0).and_then(|v| v.as_func());
                Ok(array_sorted(arr.clone(), key))
            }
            (Value::Array(arr), "filter") => {
                let pred = args.get(0).and_then(|v| v.as_func()).ok_or(Error::MissingArgument)?;
                Ok(array_filter(arr.clone(), pred))
            }
            (Value::Array(arr), "map") => {
                let func = args.get(0).and_then(|v| v.as_func()).ok_or(Error::MissingArgument)?;
                Ok(array_map(arr.clone(), func))
            }
            (Value::Array(arr), "find") => {
                let pred = args.get(0).and_then(|v| v.as_func()).ok_or(Error::MissingArgument)?;
                Ok(array_find(arr.clone(), pred))
            }
            (Value::Array(arr), "any") => {
                let pred = args.get(0).and_then(|v| v.as_func()).ok_or(Error::MissingArgument)?;
                Ok(array_any(arr.clone(), pred))
            }
            (Value::Array(arr), "all") => {
                let pred = args.get(0).and_then(|v| v.as_func()).ok_or(Error::MissingArgument)?;
                Ok(array_all(arr.clone(), pred))
            }
            (Value::Array(arr), "zip") => {
                let other = args.get(0).and_then(|v| v.as_array()).ok_or(Error::TypeMismatch)?;
                Ok(array_zip(arr.clone(), other.clone()))
            }
            (Value::Array(arr), "enumerate") => Ok(array_enumerate(arr.clone())),

            (Value::Dict(dict), "pairs") => Ok(dict_pairs(dict.clone())),
            (Value::Dict(dict), "remove") => {
                let key = args.get(0).and_then(|v| v.as_str()).ok_or(Error::TypeMismatch)?;
                let mut d = dict.clone();
                Ok(dict_remove(&mut d, key))
            }
            (Value::Dict(dict), "update") => {
                let other = args.get(0).and_then(|v| v.as_dict()).ok_or(Error::TypeMismatch)?;
                let mut d = dict.clone();
                dict_update(&mut d, other.clone());
                Ok(Value::Dict(d))
            }

            (Value::Str(s), "contains") => {
                let substr = args.get(0).and_then(|v| v.as_str()).ok_or(Error::TypeMismatch)?;
                Ok(str_contains(s.clone(), substr))
            }
            (Value::Str(s), "starts-with") => {
                let prefix = args.get(0).and_then(|v| v.as_str()).ok_or(Error::TypeMismatch)?;
                Ok(str_starts_with(s.clone(), prefix))
            }
            (Value::Str(s), "ends-with") => {
                let suffix = args.get(0).and_then(|v| v.as_str()).ok_or(Error::TypeMismatch)?;
                Ok(str_ends_with(s.clone(), suffix))
            }
            (Value::Str(s), "find") => {
                let substr = args.get(0).and_then(|v| v.as_str()).ok_or(Error::TypeMismatch)?;
                Ok(str_find(s.clone(), substr))
            }
            (Value::Str(s), "replace") => {
                let old = args.get(0).and_then(|v| v.as_str()).ok_or(Error::TypeMismatch)?;
                let new = args.get(1).and_then(|v| v.as_str()).ok_or(Error::TypeMismatch)?;
                Ok(str_replace(s.clone(), old, new))
            }
            (Value::Str(s), "trim") => Ok(str_trim(s.clone())),
            (Value::Str(s), "split") => {
                let sep = args.get(0).and_then(|v| v.as_str()).ok_or(Error::TypeMismatch)?;
                Ok(str_split(s.clone(), sep))
            }
            (Value::Str(s), "repeat") => {
                let n = args.get(0).and_then(|v| v.as_int()).ok_or(Error::TypeMismatch)?;
                Ok(str_repeat(s.clone(), n as usize))
            }

            _ => Err(Error::UnknownMethod(name.into())),
        }
    }
}
```

### 5. Tests — 23 testes (1 por método)

- `array.first()` — `(1, 2, 3).first()` → `1`
- `array.last()` — `(1, 2, 3).last()` → `3`
- `array.rev()` — `(1, 2, 3).rev()` → `(3, 2, 1)`
- `array.sum()` — `(1, 2, 3).sum()` → `6`
- `array.sorted()` — `(3, 1, 2).sorted()` → `(1, 2, 3)`
- `array.filter()` — `(1, 2, 3).filter(x => x > 1)` → `(2, 3)`
- `array.map()` — `(1, 2, 3).map(x => x * 2)` → `(2, 4, 6)`
- `array.find()` — `(1, 2, 3).find(x => x > 1)` → `2`
- `array.any()` — `(1, 2, 3).any(x => x > 2)` → `true`
- `array.all()` — `(1, 2, 3).all(x => x > 0)` → `true`
- `array.zip()` — `(1, 2).zip((a, b))` → `((1, a), (2, b))`
- `array.enumerate()` — `(a, b).enumerate()` → `((0, a), (1, b))`
- `dict.pairs()` — `(a: 1, b: 2).pairs()` → `((a, 1), (b, 2))`
- `dict.remove()` — `(a: 1, b: 2).remove(a)` → `1` (e dict fica `(b: 2)`)
- `dict.update()` — `(a: 1).update((b: 2))` → `(a: 1, b: 2)`
- `str.contains()` — `"hello".contains("ell")` → `true`
- `str.starts-with()` — `"hello".starts-with("he")` → `true`
- `str.ends-with()` — `"hello".ends-with("lo")` → `true`
- `str.find()` — `"hello".find("ll")` → `2`
- `str.replace()` — `"hello".replace("l", "x")` → `"hexxo"`
- `str.trim()` — `"  hello  ".trim()` → `"hello"`
- `str.split()` — `"a,b,c".split(",")` → `(a, b, c)`
- `str.repeat()` — `"ab".repeat(3)` → `"ababab"`

### 6. Spec L0

- `rules/stdlib/foundations.md` — tabela de métodos de `array`, `dict`, `str` com assinaturas.
- `entities/value.md` — nota: "métodos de coleção completos (P466)".

---

## Scope-out explícito

- `array.dedup()` — scope-out; raro na prática.
- `array.min()` / `array.max()` — scope-out; implementáveis via `.sorted().first()` / `.sorted().last()`.
- `array.flatten()` — scope-out; raro.
- `dict` com ordenação — scope-out; `HashMap` não é ordenado.
- `str.rev()` — scope-out; raro na prática.
- `str` com Unicode avançado (grapheme clusters) — scope-out; usa byte/char indices simples.
- `str.replace` com regex — scope-out; apenas substring literal.
- Métodos com `default` argumento — scope-out; usa `Option` simples.
- `array.sorted` com `key` como closure complexa — scope-out; `key` é `Func` simples.

---

## Critério de fecho

- [ ] 12 métodos de `array` implementados e registados no dispatcher.
- [ ] 3 métodos de `dict` implementados e registados no dispatcher.
- [ ] 8 métodos de `str` implementados e registados no dispatcher.
- [ ] 23 tests verdes (1 por método).
- [ ] Spec L0 atualizada (`rules/stdlib/foundations.md`, `entities/value.md`).
- [ ] `cargo test --workspace` verde; `crystalline-lint` zero violations.
- [ ] **Trilha 8: 2/8 completo** (repr P465 + métodos P466).

---

## Próximo passo

- **P467** — Tipos `Value`: `Relative` (Rel<Length>), `Symbol` refinado (Trilha 8, S, ~15 min)
- **P467** — `pad`/`corners`/`sides` inset modeling (Trilha 8, S, ~15 min)
- **P467** — Sonda `Selector::Where` (Trilha 3, S, ~15 min)
- **P467** — Bibliografia Fase 2: estilos numéricos (Trilha 6, M, ~40 min)

**Aguardando sua indicação:**

1. **Executar o P466** (métodos array/dict/str, ~20 min)?
2. **Escrever o P467** (próximo passo: tipos Value, Selector::Where, ou bibliografia)?
3. **Ajustar o escopo** do P466?

---

## Estado pós-P465 (para referência)

| Passo | Descrição | Estado | Trilha |
|-------|-----------|--------|--------|
| **P444** | `underline` / `overline` / `strike` | ✅ FECHADO | 8 |
| **P445** | Smart quotes | ✅ FECHADO | 8 |
| **P446** | Smallcaps | ✅ FECHADO | 8 |
| **P447** | Cobertura vanilla + DSM audit | ✅ FECHADO | — |
| **P448** | Subscript / Superscript | ✅ FECHADO | 8 |
| **P449** | Highlight | ✅ FECHADO | 8 |
| **P450** | Bibliography `.bib` de disco | ✅ FECHADO | 6 (Fase 1) |
| **P451** | Heading numbering | ✅ FECHADO | 1 |
| **P452** | Links / Hyperlinks | ✅ FECHADO | 2 |
| **P453** | Compilado de correções documentais | ✅ FECHADO | — |
| **P454** | Figure numbering | ✅ FECHADO | 1 |
| **P455** | Fecho correções retroativas + Cláusula 4 ADR-0117 | ✅ FECHADO | — |
| **P456** | Equation numbering | ✅ FECHADO | 1 |
| **P457** | Table of Contents (`outline()`) | ✅ FECHADO | 1 |
| **P458** | Sonda DEBT-2: premissa refutada | ✅ FECHADO | — |
| **P459** | Table numbering | ✅ FECHADO | 1 |
| **P460** | `label<x>`: Destinos nomeados | ✅ FECHADO | 2 |
| **P461** | Correção `table_counter` → `CounterRegistry` | ✅ FECHADO | 1 |
| **P462** | `ref<x>` / `@x`: Resolução de destino | ✅ FECHADO | 2 |
| **P463** | PDF `/GoTo` links internos | ✅ FECHADO | 2 |
| **P464** | Cleanup `Content::Label` vs `Content::Labelled` | ✅ FECHADO | — |
| **P465** | `repr()` completo | ✅ FECHADO | 8 |
| **DEBT-2** | Closures eager | ✅ FECHADO | — |
| **DEBT-9** | Tracking contínuo | ℹ️ Processo | — |

**Inventário de débitos: LIMPO.**  
**Trilha 1: COMPLETA E COERENTE.**  
**Trilha 2: COMPLETA.**  
**Trilha 8: 1/8 completo.**

