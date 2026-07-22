# Prompt L0 — `stdlib/state` — objeto `state` e métodos
Hash do Código: 7ec66d58

**Camada**: L1
**Ficheiro alvo**: `01_core/src/engine/stdlib/state.rs` (novo; funções exportadas para `rules/stdlib/mod.rs` e registadas em `rules/eval/mod.rs::make_stdlib`).
**Origem**: Passo 506 — fecho do gap P500 (runtime state mutável).
**ADRs**: ADR-0033 (paridade vanilla), ADR-0054 (graded parity), ADR-0107 (paridade linguagem), ADR-0118 (runtime state via context).
**Convenções partilhadas**: ver `00_nucleo/prompts/engine/stdlib/_comum.md`.

---

## 1. Visão geral

Este módulo implementa o tipo/valor `state(key, init)` e os seus métodos `.update()`, `.get()` e `.display()`. A representação subjacente é `Value::State { key, init }` (ver `entities/value.md`).

**P737 — o binding `state` no scope global passou de `Value::Func` a
`Value::Type(Type::State)`** (paridade vanilla — medido: `type(state)` →
`type`; `type(state("y", 0)) == state` → `true`; `repr(state)` →
`"state"`). A chamabilidade mantém-se via o despacho de tipos chamáveis
de P685 (`eval/closures.rs`: `Type::State` → `native_state`).

A sintaxe vanilla suportada:

```typst
#let s = state("key", 0)
#s.update(5)
#context s.get()
#context s.display()
```

Paridade com Typst 0.15.0 para o subset identificado em P500/P506.

---

## 2. Construtor

### `native_state` — `state(key, init)`

**Assinatura**: `state(key: str, init: any) -> state`

**Argumentos**:
- `key`: string identificadora do estado.
- `init`: valor inicial.

**Semântica**:
- Rejeita argumentos nomeados.
- Primeiro argumento deve ser `Value::Str`.
- Retorna `Value::State { key, init }`.
- Quando esse valor aparece em posição de markup (não dentro de `#let` nem como argumento), o eval de markup converte-o para `Content::State { key, init }`, garantindo registo no `StateRegistry` durante o walk.

**Testes canônicos**:
```
state("total", 0) -> Value::State
state(1, 0)       -> Err "state() requer string como primeiro argumento"
state("total")    -> Err "state() requer 2 argumentos"
```

---

## 3. Métodos (field access)

`Value::State` expõe três campos funcionais. O dispatch é feito em `rules/eval/bindings.rs::eval_field_access` (braço `Value::State`).

### `.update(value)`

**Assinatura**: `state.update(value: any) -> content`

**Semântica**:
- Retorna `Content::StateUpdate { key, update: StateUpdate::Set(Box<Value>) }`.
- Não avalia o estado; apenas emite o content de update.

**Testes canônicos**:
```
let s = state("x", 0)
s.update(5) -> Content::StateUpdate("x", Set(5))
```

### `.get()`

**Assinatura**: `state.get() -> any`

**Semântica**:
- **Só pode ser chamado dentro de `context`.**
- Fora de context (`ctx.in_context == false`): erro descritivo `"state.get() can only be used inside context"`.
- Dentro de context: consulta `ctx.introspector.state.value_at(key, location)`.
  - Se houver valor, retorna-o.
  - Se não houver valor registado, retorna `init`.
- A `location` é a do `Content::ContextBlock` onde o `.get()` está contido, fornecida pelo mecanismo de expansão pós-introspecção (ADR-0118 §4).

**Testes canônicos**:
```
let s = state("x", 0)
context s.get() -> 0 (antes de update) / 5 (após s.update(5))
s.get() -> ERRO_DESCRITIVO
```

### `.display([callback])`

**Assinatura**: `state.display() -> content` / `state.display(callback: function) -> content`

**Semântica**:
- Sem callback: retorna `Content::text(value.to_string())` quando resolvido dentro de context.
- Com callback: aplica `callback(value)` e converte o resultado para `Content` (paridade `apply_state_displays`).
- Fora de context: erro descritivo.

**`value_to_content` (P821/P842)** — cobertura de tipos do display direto:
`Content`, `Str`, `Int`, `Float`, `Bool`, `Type` (nome curto, P821) e,
desde **P842 (achado #35 de P831)**, `Length`, `Ratio`, `Relative`,
`Angle` e `Fraction` com o display de `repr` (medido no vanilla:
`#context (10pt)` → "10pt", `(50%)` → "50%", `(30% + 1em)` →
"30% + 1em", `(45deg)` → "45deg", `(2fr)` → "2fr"; pré-P842 caíam no
braço `_ => Content::Empty` — página vazia).

**Testes canônicos**:
```
let s = state("x", 10)
context s.display() -> "10"
context s.display(v => [Valor: #v]) -> content
```

---

## 4. Interação com `Content::State` existente

O `Content::State` legacy (P171) continua a existir como representação locatável no content tree. A transição `Value::State → Content::State` acontece no eval de markup quando o valor é usado como content isolado. O `StateRegistry` continua populado pelo walk a partir de `Content::State` e `Content::StateUpdate`.

---

## 5. Paridade vanilla

- `state(key, init)` cria estado identificado por `key`.
- `.update(value)` marca update no documento.
- `.get()` retorna valor acumulado até o ponto do `context`.
- `.display()` formata o valor dentro de `context`.
- `.get()` fora de `context` é erro (semântica, não mecânica).

---

## 6. Scope-outs

- `state.update(key, fn)` com callback funcional (typst vanilla) — continua a ser suportado via `state_update_with` existente; não faz parte deste prompt.
- Estados locais a um scope (vanilla permite state local em show-rules) — scope-out; este prompt cobre apenas estado documental global por key.

---

## 7. Testes obrigatórios

- `state("x", 0)` retorna `Value::State`.
- `s.update(5)` retorna `Content::StateUpdate` com key e valor corretos.
- `context s.get()` dentro de documento com `s.update(5)` retorna `5`.
- `s.get()` fora de context retorna erro descritivo.
- `context s.display()` retorna content textual.
- Dois estados com keys distintas não interferem.

---

## P844 (achados #49/#50/#51 de P831) — métodos `at`/`final`, `counter.at(Location)`, repr de array

- `state.at(selector)` ligado no dispatch de métodos (`bindings.rs::state_at_dispatch` → `state.rs::state_at_location`). Aceita `Location` directa (ex.: `here()`) ou `<label>` resolvida via introspector. Sem update prévio à Location, devolve o init (medido no vanilla 0.15.0). Mensagens verbatim medidas: `missing argument: selector`, `unexpected argument`, `expected label, function, location, or selector, found {type}`, `text is not locatable` (string), ``label `<x>` does not exist in the document``. Validação de argumentos precede o gate de contexto (`can only be used when context is known`) — ordem medida no vanilla.
- `state.final()` ligado no dispatch (`state.rs::state_final`). Devolve o valor final pós-walk via `Introspector::state_final_value` (P171/P240, two-pass real); sem updates, o init (medido: `state("s", 7).final()` → `7`).
- `value_to_content` usa o repr para `Value::Array` (#51): `#context ((3,))` → `(3,)` (medido; o join próprio com `.` foi removido). Reusa `eval/repr::repr_value`, a mesma rotina corrigida em P801 para o caminho directo.
