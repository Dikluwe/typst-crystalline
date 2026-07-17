# Prompt L0 — `stdlib/counter` — objeto `counter` e métodos
Hash do Código: badba3c3

**Camada**: L1
**Ficheiro alvo**: `01_core/src/engine/stdlib/counter.rs` (novo; funções exportadas para `rules/stdlib/mod.rs` e registadas em `rules/eval/mod.rs::make_stdlib`).
**Origem**: Passo 506 — fecho do gap P500 (runtime state mutável).
**ADRs**: ADR-0033 (paridade vanilla), ADR-0054 (graded parity), ADR-0107 (paridade linguagem), ADR-0118 (runtime state via context).
**Convenções partilhadas**: ver `00_nucleo/prompts/engine/stdlib/_comum.md`.

---

## 1. Visão geral

Este módulo implementa o construtor `counter(selector)` e os seus métodos `.update()`, `.step()`, `.get()`, `.display()` e `.at()`. A representação subjacente é `Value::Counter { key }` (ver `entities/value.md`).

**P737 — o binding `counter` no scope global passou de `Value::Func` a
`Value::Type(Type::Counter)`** (paridade vanilla — medido: `type(counter)` →
`type`; `type(counter("x")) == counter` → `true`; `repr(counter)` →
`"counter"`). A chamabilidade mantém-se via o despacho de tipos chamáveis
de P685 (`eval/closures.rs`: `Type::Counter` → `native_counter`).

A sintaxe vanilla suportada:

```typst
#counter(heading).update(1)
#context counter(heading).get()
#context counter(heading).display("1.")
#counter(heading).step()
= Heading <my-label>
#context counter(heading).at(<my-label>)
```

Paridade com Typst 0.15.0 para o subset identificado em P500/P506.

---

## 2. Construtor

### `native_counter` — `counter(selector)`

**Assinatura**: `counter(selector: str | selector) -> counter`

**Argumentos**:
- `selector`: string de kind (ex: `"heading"`, `"figure"`, `"equation"`) ou `Value::Selector` que designa um kind.

**Semântica**:
- Rejeita argumentos nomeados.
- Aceita `Value::Str` ou `Value::Selector`.
- Converte o selector para uma chave string:
  - `"heading"` → `"heading"`
  - `Selector::Kind(Heading)` → `"heading"`
  - outras forms de selector → string correspondente ou erro se não mapeável.
- Retorna `Value::Counter { key }`.

**Testes canônicos**:
```
counter("heading") -> Value::Counter { key: "heading" }
counter(heading)   -> Value::Counter { key: "heading" } (via Selector::Kind)
counter(1)         -> Err "counter() requer string ou selector"
```

---

## 3. Métodos (field access)

`Value::Counter` expõe cinco campos funcionais. O dispatch é feito em `rules/eval/bindings.rs::eval_field_access` (braço `Value::Counter`).

### `.update(value)`

**Assinatura**: `counter.update(value: int) -> content`

**Semântica**:
- Retorna `Content::CounterUpdate { key, action: CounterUpdate::Update(value) }`.
- Define o counter para `[value]` (reseta hierarquia).

**Testes canônicos**:
```
counter("heading").update(5) -> Content::CounterUpdate("heading", Update(5))
```

### `.step()`

**Assinatura**: `counter.step() -> content`

**Semântica**:
- Retorna `Content::CounterUpdate { key, action: CounterUpdate::Step }`.
- Equivalente a incrementar o counter na posição onde é inserido.

**Testes canônicos**:
```
counter("heading").step() -> Content::CounterUpdate("heading", Step)
```

### `.get()`

**Assinatura**: `counter.get() -> array`

**Semântica**:
- **Só pode ser chamado dentro de `context`.**
- Fora de context: erro descritivo `"counter.get() can only be used inside context"`.
- Dentro de context: consulta `ctx.introspector.counters.value_at(key, location)`.
  - Se houver valor, retorna `Value::Array(Vec<Value::Int>)` com o slice.
  - Se não houver valor registado, retorna `Value::Array(vec![Value::Int(0)])`.

**Testes canônicos**:
```
counter("heading").update(1)
context counter("heading").get() -> (1,)
```

### `.display(pattern?)`

**Assinatura**: `counter.display() -> content` / `counter.display(pattern: str) -> content` / `counter.display(callback: function) -> content`

**Semântica**:
- Sem argumento: formata o slice com join `"."` (ex: `[1, 2]` → `"1.2"`).
- Com string `pattern`: aplica o pattern de numbering (paridade vanilla `counter.display("1.")`).
- Com callback: passa `Value::Array(Vec<Value::Int>)` ao callback e converte o resultado para `Content`.
- Fora de context: erro descritivo.

**Testes canônicos**:
```
counter("heading").update(1)
context counter("heading").display()     -> "1"
context counter("heading").display("1.") -> "1."
```

### `.at(label)`

**Assinatura**: `counter.at(label: label | str) -> array`

**Semântica**:
- Aceita `Value::Label` ou `Value::Str` com nome do label.
- Resolve a `Location` do label via `ctx.introspector.query_by_label`.
- Retorna o valor do counter nessa location como `Value::Array(Vec<Value::Int>)`.
- Pode ser usado dentro ou fora de context (paridade vanilla 0.15.0).
- Se label/counter inexistente → `Value::Array(vec![])`.

**Testes canônicos**:
```
counter("heading").step()
= Heading <my-label>
context counter("heading").at(<my-label>) -> (1,)
```

---

## 4. Interação com `CounterRegistry` existente

O `CounterRegistry` continua populado pelo walk a partir de `Content::CounterUpdate` e do arm `Content::Heading` (counter hierárquico automático). Os métodos `.update()` e `.step()` apenas emitem `Content::CounterUpdate`; o runtime state real vive no `CounterRegistry`/`TagIntrospector`.

---

## 5. Counter automático para headings

Mantém-se o comportamento existente: o walk arm `Content::Heading` chama `intr.counters.apply_hierarchical_at("heading", level, loc)`. Não é alterado por este prompt.

---

## 6. Paridade vanilla

- `counter(selector)` cria counter identificado pela chave do selector.
- `.update(n)` define o valor do counter.
- `.step()` incrementa o counter.
- `.get()` retorna array de valores dentro de `context`.
- `.display(pattern)` formata o counter dentro de `context`.
- `.at(label)` retorna valor do counter no ponto do label.

---

## 7. Scope-outs

- Counters custom com nome string fora dos kinds conhecidos (`counter("meu")`) — scope-out deste prompt; a arquitetura `Value::Counter { key }` suporta qualquer string, mas consumers de layout podem não reconhecer.
- Counters sobre tipos de elemento sem kind string mapeável.

---

## 8. Testes obrigatórios

- `counter("heading")` retorna `Value::Counter`.
- `counter(heading)` retorna `Value::Counter { key: "heading" }`.
- `c.update(5)` retorna `Content::CounterUpdate` correto.
- `c.step()` retorna `Content::CounterUpdate` com `Step`.
- `context c.get()` retorna array correto após update.
- `context c.display("1.")` retorna content textual correto.
- `c.at(<label>)` retorna array correto.
- `c.get()` fora de context retorna erro descritivo.
