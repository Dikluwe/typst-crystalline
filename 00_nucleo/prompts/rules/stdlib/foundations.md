# Prompt L0 — `stdlib/foundations` — utilitários gerais, cores, state/counter display
Hash do Código: aa9e1531

**Camada**: L1
**Ficheiro alvo**: `01_core/src/rules/stdlib/foundations.rs`
**Origem**: fatiado de `rules/stdlib.md` em **P314** (ADR-0104). Convenção e
helpers partilhados: ver `stdlib/_comum.md`.
**Nota de deriva (F4)**: `foundations.rs` implementa mais funções do que
`stdlib.md` especificava (ex.: `oklab`/`oklch`/`cmyk`/`hsl`/`hsv`, `state*`,
`counter*`, `query`, `here`, `locate`). Este prompt preserva **só** o que estava
no prompt velho; as funções não-specadas ficam registadas como **candidatas a
spec dedicada** (não inventadas aqui).

---

## Utilitários gerais

| Função | Assinatura Typst | Implementação |
|--------|-----------------|---------------|
| `native_type` | `type(v)` | nome do tipo como `Value::Str` |
| `native_len` | `len(v)` | `Str` (chars), `Array` (items), `Dict` (entries) |
| `native_range` | `range(n)` ou `range(start, end)` | `Array` de `Int` |
| `native_str` | `str(v)` | conversão (`None`→`"none"`, `Bool`→`"true"/"false"`) |
| `native_int` | `int(v)` | `Int`, `Bool`, `Str(decimal)` → `Int`; `Float` → **Err** (ADR Typst) |
| `native_float` | `float(v)` | `Float`, `Int`, `Str` → `Float` |

**Nota** `native_int(Float)` retorna `Err` (semântica vanilla). Para converter
float a inteiro: `int(calc.round(x))`.

## Cores

| Função | Args | Retorno |
|--------|------|---------|
| `native_rgb` | `(r,g,b)` ou `(r,g,b,a)` — Int 0–255 | `Value::Color` |
| `native_luma` | `(l)` — Int 0–255 | `Value::Color(Color::rgb(l,l,l))` |

Fora de 0–255 → `Err`.

## `state_display(key, [callback])` — Passo 240 (M9d/M7+1; ADR-0081 PROPOSTO P239 Opção γ)

Render-mediated state display real walk-time. Constroi
`Content::StateDisplay { key, callback }` que walk emite como
`Tag::Start(loc, ElementInfo::new(ElementPayload::StateDisplay { key, callback }))`
via `extract_payload`. Pós-fixpoint, `apply_state_displays` (em
`rules/introspect/from_tags.rs`) chama `apply_func(callback, [state.value_at(key,
loc)], ctx, engine)` e armazena Content em `intr.state_displays[(key, loc)]`.
Layout arm consome via `Introspector::state_display_value(key, loc)` — Layouter
permanece puro. Assinatura `native_state_display(_ctx, args, _world,
_current_file, _figure_numbering)`.

**Formas**: 1-arg `state_display(key: Str)` (callback ausente; value directo) ·
2-arg `state_display(key: Str, callback: Func)` (callback aplicada). Two-pass
real via convergência fixpoint. Paridade vanilla `state.display(fn)`.

```
state_display([Str("k")]) → Ok(Content::StateDisplay { key:"k", callback:None })
state_display([Str("k"),Func(fn)]) → Ok(Content::StateDisplay { key:"k", callback:Some(fn) })
state_display([Int(1)]) → Err;  state_display([Str("k"),Int(1)]) → Err;  state_display([]) → Err;  state_display([Str("k"),Func(f),Int(1)]) → Err
```

## `counter_display(key, [callback])` — Passo 241 (M9d/M7+2; paralelo absoluto P240)

Constroi `Content::CounterDisplayCallback { key, callback }`; walk emite
`Tag::Start(loc, ElementInfo::new(ElementPayload::CounterDisplay { key, callback }))`.
Pós-fixpoint, `apply_counter_displays` converte `intr.counters.value_at(key,
loc)` para `Value::Array(Vec<Int>)` e chama `apply_func`. Resultado em
`intr.counter_displays[(key, loc)]`; layout via
`Introspector::counter_display_value`. **Distinto de `Content::CounterDisplay
{ kind }` legacy single-pass**.

**Formas**: 1-arg (snapshot default "1.2.3" via join "."; inexistente →
`Content::Empty`) · 2-arg (callback recebe `Value::Array(Vec<Int>)`). Paridade
`counter("heading").display(fn)`.

```
counter_display([Str("heading")]) → Ok(Content::CounterDisplayCallback { key:"heading", callback:None })
counter_display([Str("figure"),Func(fn)]) → Ok(Content::CounterDisplayCallback { key:"figure", callback:Some(fn) })
counter_display([Int(1)]) → Err;  counter_display([Str("k"),Int(1)]) → Err;  counter_display([]) → Err;  counter_display([Str("k"),Func(f),Int(1)]) → Err
```

---

## Critérios de Verificação (utilitários + cores)

```
native_type([Int(1)]) → Ok(Str("int"));  native_type([Bool(true)]) → Ok(Str("bool"));  native_type([None]) → Ok(Str("none"));  native_type([]) → Err;  native_type([Int,Int]) → Err
native_len([Str("abc")]) → Ok(Int(3));  native_len([Array([Int(1),Int(2)])]) → Ok(Int(2));  native_len([Int(1)]) → Err
native_rgb([Int(255),Int(0),Int(128)]) → Ok(Color::rgb(255,0,128));  native_rgb([Int(300),Int(0),Int(0)]) → Err;  native_rgb([Int(255),Int(0),Int(0),Int(200)]) → Ok(Color::rgba(255,0,0,200))
native_luma([Int(128)]) → Ok(Color::rgb(128,128,128));  native_luma([Int(256)]) → Err
native_str([Int(42)]) → Ok(Str("42"));  native_str([Float(3.14)]) → Ok(Str("3.14"));  native_str([Bool(true)]) → Ok(Str("true"));  native_str([None]) → Ok(Str("none"));  native_str([Str("hi")]) → Ok(Str("hi"))
native_int([Int(42)]) → Ok(Int(42));  native_int([Bool(true)]) → Ok(Int(1));  native_int([Str("42")]) → Ok(Int(42));  native_int([Str("abc")]) → Err;  native_int([Float(3.7)]) → Err;  native_int([]) → Err
native_range([Int(3)]) → Ok(Array([0,1,2]));  native_range([Int(2),Int(5)]) → Ok(Array([2,3,4]));  native_range([Int(3),Int(3)]) → Ok(Array([]));  native_range([Int(-1)]) → Err
```
