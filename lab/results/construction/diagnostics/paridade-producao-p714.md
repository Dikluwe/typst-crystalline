# Paridade Produção — P714 — `array.at(index, default:)`

**Data:** 2026-07-11
**Passo:** sem materialização prévia — instrução directa do utilizador
("avança para o array.at()"), sonda feita directamente contra
`lab/typst-original` per protocolo (ADR-0108).
**Hash do commit (implementação):** `3ca764ab9`.
**HEAD base:** `1c10a0b2b` (fim de P713).
**Estado:** FECHADO — `array.at(index, default:)` implementado com
paridade exacta ao `Array::at`/`locate_opt` do vanilla.

---

## 1. Sonda

### 1.1 Causa exacta do erro reportado

O erro `"campo desconhecido em array: 'at'"` **não vinha** de
`try_dispatch_collection_method` (o dispatcher de método,
`stdlib/collections.rs`) — vinha do fallback de **field access**
genérico em `eval/bindings.rs:596-607`:

```rust
// P493a — Field access em Value::Array: len, first, last.
// dedup/chunks/windows são despachados via try_dispatch_collection_method
// em closures.rs (method call), não como field access.
Value::Array(arr) => match field.as_str() {
    "len" => ..., "first" => ..., "last" => ...,
    _ => Err(... format!("campo desconhecido em array: '{}'", field) ...),
},
```

`try_dispatch_collection_method` (`collections.rs:40-59`) **não tinha
braço** para `(Value::Array(arr), "at")` — devolvia `None`; o
interceptor de método em `eval_func_call` (P466,
`closures.rs:391-401`), ao receber `None`, caía no fallback de field
access genérico acima, que trata `arr.at(...)` como um campo
desconhecido (não reconhece que é uma chamada de método com
argumentos). Confirmado: bastava adicionar o braço em
`try_dispatch_collection_method` — nenhuma outra mudança de fiação
necessária.

### 1.2 Mecanismo vanilla confirmado (`foundations/array.rs:203-221,127-137`)

```rust
pub fn at(&self, index: i64, default: Option<Value>) -> StrResult<Value> {
    self.locate_opt(index, false)
        .and_then(|i| self.0.get(i).cloned())
        .or(default)
        .ok_or_else(|| out_of_bounds_no_default(index, self.len()))
}

fn locate_opt(&self, index: i64, end_ok: bool) -> Option<usize> {
    let wrapped = if index >= 0 { Some(index) } else { (self.len() as i64).checked_add(index) };
    wrapped.and_then(|v| usize::try_from(v).ok()).filter(|&v| v < self.0.len() + end_ok as usize)
}
```

- Índice negativo conta a partir do fim (`len + index`, via
  `checked_add` — sem overflow para índices patológicos).
- `end_ok = false` para `.at()` — só `0 <= resolved < len` é válido.
- Fora de limites: usa `default` se fornecido; senão erro
  `out_of_bounds_no_default`: `"array index out of bounds (index: {i},
  len: {n}) and no default value was specified"` — mensagem exacta
  reproduzida no cristalino (string literal simples, sem hints a
  divergir).

### 1.3 Confirmação de uso real em `cetz`

```
aabb.typ:43:  bounds.high.at(2, default: 0)
aabb.typ:45:  bounds.low.at(2, default: 0)
aabb.typ:75:  bounds.low.at(0)  -= padding.at("left", default: 0)
aabb.typ:76:  bounds.low.at(1)  -= padding.at("top", default: 0)
aabb.typ:77:  bounds.high.at(0) += padding.at("right", default: 0)
```

`padding.at(...)` nestas linhas é `dict.at` (já implementado desde
P495); `bounds.high.at(2, ...)`/`bounds.low.at(...)` são `array.at` —
exactamente a lacuna fechada por este passo.

### Critério de fecho da sonda

- [x] Localização exacta da causa (`file:line`, dois pontos: braço em
      falta em `try_dispatch_collection_method` + fallback de field
      access que produz a mensagem observada).
- [x] Mecanismo do vanilla confirmado (`locate_opt`, gate `end_ok`,
      mensagem de erro exacta).
- [x] Uso real em `cetz` confirmado, distinto de `dict.at` (já
      implementado).

---

## 2. Implementação

**`01_core/src/engine/stdlib/collections.rs`**:

- Novo braço `(Value::Array(arr), "at") => Some(array_at(arr, args))`
  em `try_dispatch_collection_method` — nenhuma outra fiação
  necessária (o dispatcher já era alcançado correctamente pela sintaxe
  de método; só faltava o braço).
- Nova `array_at(arr: Vec<Value>, args: Args) -> SourceResult<Value>`
  — mesmo padrão de `dict_at` (validação de named args primeiro,
  `default` extraído por borrow antes de consumir `args` por valor),
  com a lógica de `locate_opt` (índice negativo, `checked_add`,
  filtro `0 <= v < len`) e a mensagem de erro exacta do vanilla.

### Testes (11 novos)

- `collections.rs` (6): índice positivo, negativo (conta do fim, dois
  casos incluindo o primeiro elemento via `-3`), fora de limites com
  `default`, fora de limites sem `default` (mensagem exacta), named
  arg desconhecido rejeitado, array vazio sem `default`.
- `eval/tests.rs` (5, via sintaxe de método real): índice positivo,
  negativo, com `default`, fora de limites sem `default` (erro),
  reprodução exacta do padrão `cetz` (`(1, 2).at(2, default: 0)`).

---

## 3. Validação

Reprodução manual (release build), contra o vanilla:

```
#let arr = (10, 20, 30)
#(arr.at(1))              → 20   (vanilla: 20)
#(arr.at(-1))              → 30   (vanilla: 30)
#(arr.at(5, default: 0))  → 0    (vanilla: 0)

#((10, 20).at(5))  → Err "array index out of bounds (index: 5, len: 2)
                      and no default value was specified" (idêntico
                      ao vanilla)
```

- `cargo test --workspace` → **3820** (typst-core: 3809 + 11 novos) +
  **630** (typst-infra, inalterado) passed, 0 failed.
- `crystalline-lint .` → 0 violations. `collections.rs` não tem
  `@prompt-hash` no header (só `@prompt`, sem par de hash) —
  `--fix-hashes .` correu e devolveu "Nothing to fix", consistente com
  a ausência do par.

### Reprodução `cetz`

```
#import "@preview/cetz:0.5.2"
#cetz.canvas({ import cetz.draw: *; line((0,0),(2,1)); circle((0,0)) })
```

**Bloqueio anterior resolvido** — `array.at()` deixou de ser a causa.
**Novo bloqueio**: `error: destructuring assignment is not yet
implemented` — atribuição por desestruturação a variáveis já
existentes (não `let`), confirmado em uso real:
`coordinate.typ:259,264` (`(ctx, p) = resolve(ctx, p)`),
`bezier.typ:282`, `path-util.typ:404,408`. Tempo de compilação
**~52.7s**, mesma ordem de grandeza, sem regressão.

### Próximo passo sugerido (não iniciado)

Atribuição por desestruturação (`(a, b) = expr`, distinta de
`let (a, b) = expr`) — confirmar primeiro no vanilla o mecanismo exacto
(provavelmente `Expr::Destructuring` já existe no parser para `let`,
mas o caminho de **atribuição simples** pode ainda não estar
implementado no eval do cristalino) antes de decidir a spec.

---

## 4. Critério de fecho do passo

- [x] Sonda completa, causa exacta confirmada (dois pontos: braço em
      falta + fallback que produz a mensagem observada).
- [x] `array.at()` implementado, testado com múltiplos casos (índice
      positivo/negativo, limites, `default`, sem `default`, vazio).
- [x] Sem regressão em `cargo test --workspace` (3820 vs 3809 antes).
- [x] `crystalline-lint .` limpo.
- [x] `cetz` re-testado — bloqueio de `array.at()` resolvido, novo
      bloqueio identificado (desestruturação em atribuição), sem
      regressão de tempo.
- [x] Relatório com resultado exacto (este ficheiro).
