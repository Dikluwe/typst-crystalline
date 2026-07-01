---

# P504 — Materialização de Novas Funcionalidades Typst 0.15.0

> **Passo:** 504
> **Data:** 2026-06-29
> **Foco:** Fechar empiricamente os gaps de funcionalidades novas do Typst 0.15.0 que o cristalino ainda não implementa. Não declarar conclusão — medir antes e depois de cada sub-tarefa.
> **Tipo:** Implementação M-size com validação via bateria P500 + novos testes.
> **Tamanho:** M (~60 min de implementação + 20 min de validação).
> **ADR-0075 ACEITE** — mecanismo de comparação via `typst query --format json` vs `Introspector::query_*`.
> **ADR-0054 ACEITE** — graded parity: MATCH / DIFF / ERRO_DESCRITIVO / PANIC / AUSENTE.
> **ADR-0109 ACEITE** — atomização de código.
> **Dependências:** P503 (re-baseline 0.15.0 confirmado 20/20 MATCH), P502 (audit P500 fechado), P501 (P1/P2 fechados).

---

## 1. Contexto

O P503 confirmou que a bateria P490 (20 ficheiros) é **estável** contra o Typst 0.15.0 — nenhum breaking change afeta os testes existentes. No entanto, o 0.15.0 introduziu **novas funcionalidades** que o cristalino ainda não implementa (ou implementou parcialmente).

Este passo materializa essas funcionalidades, priorizadas por **impacto de usabilidade** e **tamanho de implementação**.

---

## 2. Inventário de Novas Funcionalidades 0.15.0

### 2.1 — Prioridade Alta (S-size, impacto alto)

| # | Funcionalidade | Descrição | Tamanho | Arquivo alvo |
|---|----------------|-----------|---------|--------------|
| 1 | `within` selector | Seleciona elementos dentro de um ancestor | S | `entities/selector.rs`, `introspect/query.rs` |
| 2 | `dict.map()` | Aplica função a cada par chave-valor | S | `eval/bindings.rs`, `stdlib/collections.rs` |
| 3 | `dict.filter()` | Filtra pares por predicado | S | `eval/bindings.rs`, `stdlib/collections.rs` |
| 4 | `arguments` field access | Acessar named args via `.field` | S | `eval/bindings.rs`, `entities/args.rs` |
| 5 | `int(base:)` | Parsear string com base numérica | S | `eval/constructors.rs` |

### 2.2 — Prioridade Média (XS-size, impacto médio)

| # | Funcionalidade | Descrição | Tamanho | Arquivo alvo |
|---|----------------|-----------|---------|--------------|
| 6 | `calc.asinh` | Arco-seno hiperbólico | XS | `stdlib/calc.rs` |
| 7 | `calc.acosh` | Arco-cosseno hiperbólico | XS | `stdlib/calc.rs` |
| 8 | `calc.atanh` | Arco-tangente hiperbólica | XS | `stdlib/calc.rs` |
| 9 | `calc.erf` | Função erro de Gauss | XS | `stdlib/calc.rs` |
| 10 | `int.min` | Menor inteiro representável | XS | `eval/scope.rs` |
| 11 | `int.max` | Maior inteiro representável | XS | `eval/scope.rs` |
| 12 | `range(inclusive:)` | Parâmetro inclusive em range | XS | `stdlib/foundations.rs` |
| 13 | `counter.display(at:)` | Parâmetro at em counter.display | XS | `stdlib/introspect.rs` |
| 14 | `page.bleed` | Margens de bleed para impressão | XS | `stdlib/layout.rs` |

### 2.3 — Prioridade Baixa (S/M-size, impacto baixo)

| # | Funcionalidade | Descrição | Tamanho | Arquivo alvo |
|---|----------------|-----------|---------|--------------|
| 15 | `list.marker-align` | Alinhamento de list markers | S | `stdlib/layout.rs`, `layout/lists.rs` |
| 16 | `divider` element | Thematic break (linha horizontal) | S | `entities/content.rs`, `stdlib/structural.rs` |
| 17 | `text.variations` | Variable font axes (ital, slnt, wght, etc.) | M | `stdlib/text.rs`, `font subsystem` |
| 18 | File `path` type | Novo tipo para paths project-relative | M | `eval/types.rs`, `entities/value.rs` |

---

## 3. Metodologia

Para cada sub-tarefa, implementar a funcionalidade, criar ficheiro `.typ` de teste, correr contra vanilla 0.15.0 e cristalino, e classificar o resultado.

```bash
# Baseline P503 (antes de tocar código):
cargo test --test structural_parity p503_rebaseline_0150

# Após cada sub-tarefa:
cargo test --test structural_parity p504_<subtarefa>
```

Classificar cada resultado: `MATCH` | `DIFF` | `ERRO_DESCRITIVO` | `PANIC` | `AUSENTE`.

---

## 4. Implementação por Categoria

### 4.1 — `within` selector (S)

**Estado baseline:** Não implementado.

**Sintaxe vanilla 0.15.0:**
```typst
#query(selector(heading).within(figure))
```

**Implementação:**
- Adicionar `Selector::Within { base: Box<Selector>, ancestor: Box<Selector> }` ao enum `Selector`.
- No `Introspector::query`, implementar ancestor traversal: para cada match de `base`, verificar se existe um ancestor que match `ancestor`.

```rust
Selector::Within { base, ancestor } => {
    let base_matches = self.query_selector(base);
    base_matches.into_iter().filter(|item| {
        self.has_ancestor(item, ancestor)
    }).collect()
}
```

**Teste:**
```typst
#figure[
  = Heading dentro de figure
]
= Heading fora de figure
#context query(selector(heading).within(figure))
```

---

### 4.2 — `dict.map()` / `dict.filter()` (S)

**Estado baseline:** Não implementados.

**Sintaxe vanilla 0.15.0:**
```typst
#let d = (a: 1, b: 2, c: 3)
#d.map((k, v) => v * 2)     // (a: 2, b: 4, c: 6)
#d.filter((k, v) => v > 1)  // (b: 2, c: 3)
```

**Implementação:**
- Adicionar `map` e `filter` ao field access de `Value::Dict`.
- `dict.map` recebe uma closure `(key, value) => new_value` e retorna novo dict com valores transformados.
- `dict.filter` recebe uma closure `(key, value) => bool` e retorna novo dict com pares que satisfazem o predicado.

```rust
fn dict_map(args: Args) -> SourceResult<Value> {
    let dict = args.expect::<Dict>("self")?;
    let func = args.expect::<Func>("mapper")?;
    let mut result = Dict::new();
    for (key, value) in dict.iter() {
        let mapped = func.call(vec![Value::Str(key.clone()), value.clone()])?;
        result.insert(key.clone(), mapped);
    }
    Ok(Value::Dict(result))
}

fn dict_filter(args: Args) -> SourceResult<Value> {
    let dict = args.expect::<Dict>("self")?;
    let func = args.expect::<Func>("predicate")?;
    let mut result = Dict::new();
    for (key, value) in dict.iter() {
        let keep = func.call(vec![Value::Str(key.clone()), value.clone()])?;
        if keep.cast::<bool>()? {
            result.insert(key.clone(), value.clone());
        }
    }
    Ok(Value::Dict(result))
}
```

---

### 4.3 — `arguments` field access (S)

**Estado baseline:** Não implementado.

**Sintaxe vanilla 0.15.0:**
```typst
#let f(..args) = args.named
#f(x: 1, y: 2)  // (x: 1, y: 2)
```

**Implementação:**
- Adicionar field access para `Value::Args`.
- Expor `named` e `positional` como campos acessíveis.

```rust
Value::Args(args) => match field.as_str() {
    "named" => Ok(Value::Dict(args.named.clone())),
    "positional" => Ok(Value::Array(args.items.clone())),
    _ => Err("arguments does not have field '{}'"),
},
```

---

### 4.4 — `int(base:)` (S)

**Estado baseline:** Não implementado.

**Sintaxe vanilla 0.15.0:**
```typst
#int("ff", base: 16)  // 255
#int("1010", base: 2)  // 10
```

**Implementação:**
- Adicionar arg nomeado `base` ao construtor `int()`.
- Se `base` presente, parsear string como inteiro na base indicada.

```rust
fn native_int(args: Args) -> SourceResult<Value> {
    let value = args.expect::<Value>("value")?;
    let base = args.named.get("base")
        .and_then(|v| v.cast::<i64>().ok())
        .unwrap_or(10) as u32;

    match value {
        Value::Str(s) => {
            let n = i64::from_str_radix(&s, base)
                .map_err(|_| "invalid integer for base")?;
            Ok(Value::Int(n))
        }
        Value::Int(n) => Ok(Value::Int(n)),
        Value::Float(f) => Ok(Value::Int(f as i64)),
        _ => Err("cannot convert to int"),
    }
}
```

---

### 4.5 — `calc.asinh` / `acosh` / `atanh` / `erf` (XS)

**Estado baseline:** Não implementados.

**Implementação:** Trivial — bindings para funções `f64`.

```rust
fn native_asinh(args: Args) -> SourceResult<Value> {
    let x = args.expect::<f64>("x")?;
    Ok(Value::Float(x.asinh()))
}

fn native_acosh(args: Args) -> SourceResult<Value> {
    let x = args.expect::<f64>("x")?;
    Ok(Value::Float(x.acosh()))
}

fn native_atanh(args: Args) -> SourceResult<Value> {
    let x = args.expect::<f64>("x")?;
    Ok(Value::Float(x.atanh()))
}

fn native_erf(args: Args) -> SourceResult<Value> {
    let x = args.expect::<f64>("x")?;
    // erf não está na std de Rust; usar approximação ou crate statrs
    Ok(Value::Float(erf_approx(x)))
}
```

**Nota:** `erf` não está na `std` de Rust. Opções:
1. Implementar aproximação (Abramowitz & Stegun, formula 7.1.26).
2. Adicionar dependência `statrs` (ou similar).
3. Scope-out `erf` se a dependência for indesejada.

---

### 4.6 — `int.min` / `int.max` (XS)

**Estado baseline:** Não implementados.

**Implementação:** Constantes no scope global.

```rust
global.define("int.min", Value::Int(i64::MIN));
global.define("int.max", Value::Int(i64::MAX));
```

---

### 4.7 — `range(inclusive:)` (XS)

**Estado baseline:** `range` não aceita `inclusive`.

**Sintaxe vanilla 0.15.0:**
```typst
#range(1, 5, inclusive: true)  // (1, 2, 3, 4, 5)
```

**Implementação:**
- Adicionar arg nomeado `inclusive: Bool` ao `range`.
- Se `inclusive: true`, o range inclui o end.

```rust
fn native_range(args: Args) -> SourceResult<Value> {
    let start = args.expect::<i64>("start")?;
    let end = args.expect::<i64>("end")?;
    let inclusive = args.named.get("inclusive")
        .and_then(|v| v.cast::<bool>().ok())
        .unwrap_or(false);

    let range = if inclusive {
        start..=end
    } else {
        start..end
    };

    let values: Vec<Value> = range.map(Value::Int).collect();
    Ok(Value::Array(values))
}
```

---

### 4.8 — `counter.display(at:)` (XS)

**Estado baseline:** `counter.display` não aceita `at`.

**Sintaxe vanilla 0.15.0:**
```typst
#counter(heading).display("1.", at: <label>)
```

**Implementação:**
- Adicionar arg nomeado `at: Label` ao `counter.display`.
- Se `at` presente, buscar o valor do counter no ponto do label, não no contexto atual.

```rust
fn counter_display(args: Args) -> SourceResult<Value> {
    let counter = args.expect::<Counter>("self")?;
    let pattern = args.expect::<Str>("pattern")?;
    let at = args.named.get("at")
        .and_then(|v| v.cast::<Label>().ok());

    let value = match at {
        Some(label) => counter.at_label(label),
        None => counter.current(),
    };

    Ok(Value::Str(value.display(&pattern)))
}
```

---

### 4.9 — `page.bleed` (XS)

**Estado baseline:** Não implementado.

**Sintaxe vanilla 0.15.0:**
```typst
#set page(bleed: 3mm)
```

**Implementação:**
- Adicionar campo `bleed: Length` a `PageElem`.
- No layout, expandir a página pelo bleed (scope-out de renderização específica se necessário).

```rust
fn native_page(args: Args) -> SourceResult<Value> {
    // ... campos existentes
    let bleed = args.named.get("bleed")
        .and_then(|v| v.cast::<Length>().ok())
        .unwrap_or(Length::zero());

    Ok(Value::Content(Content::Page {
        // ... campos existentes
        bleed,
    }))
}
```

---

### 4.10 — `list.marker-align` (S)

**Estado baseline:** Não implementado.

**Sintaxe vanilla 0.15.0:**
```typst
#list(marker-align: start, [Item A], [Item B])
#list(marker-align: center, [Item A], [Item B])
#list(marker-align: end, [Item A], [Item B])
```

**Implementação:**
- Adicionar campo `marker-align: Alignment` a `ListElem`.
- No layout de listas, alinhar o marker conforme o parâmetro.

```rust
fn native_list(args: Args) -> SourceResult<Value> {
    // ... campos existentes
    let marker_align = args.named.get("marker-align")
        .and_then(|v| v.cast::<Alignment>().ok())
        .unwrap_or(Alignment::Start);

    Ok(Value::Content(Content::List {
        // ... campos existentes
        marker_align,
    }))
}
```

---

### 4.11 — `divider` element (S)

**Estado baseline:** Não implementado.

**Sintaxe vanilla 0.15.0:**
```typst
#divider
```

**Implementação:**
- Adicionar `Content::Divider` ao enum `Content`.
- Adicionar `native_divider` ao stdlib.
- Default show-rule: linha horizontal (stroke) com largura total.

```rust
// entities/content.rs
pub enum Content {
    // ... variants existentes
    Divider(DividerElem),
}

// stdlib/structural.rs
fn native_divider(_args: Args) -> SourceResult<Value> {
    Ok(Value::Content(Content::Divider(DividerElem::default())))
}
```

---

## 5. Formato do Relatório de Resultados

Para cada sub-tarefa, produzir uma linha:

```
| 504a within selector | Vanilla: ok | Cristalino: ok | MATCH | ancestor traversal |
| 504b dict.map/filter | Vanilla: ok | Cristalino: ok | MATCH | closures em coleções |
| 504c arguments field | Vanilla: ok | Cristalino: ok | MATCH | named/positional expostos |
| 504d int(base:) | Vanilla: ok | Cristalino: ok | MATCH | parse string com base |
| 504e calc asinh... | Vanilla: ok | Cristalino: ok | MATCH | bindings f64 |
| 504f int.min/max | Vanilla: ok | Cristalino: ok | MATCH | constantes globais |
| 504g range(inclusive) | Vanilla: ok | Cristalino: ok | MATCH | arg nomeado |
| 504h counter.display(at) | Vanilla: ok | Cristalino: ok | MATCH | arg nomeado |
| 504i page.bleed | Vanilla: ok | Cristalino: ok | MATCH | arg nomeado |
| 504j list.marker-align | Vanilla: ok | Cristalino: ok | MATCH | layout de listas |
| 504k divider | Vanilla: ok | Cristalino: ok | MATCH | novo elemento |
```

---

## 6. Critério de Fecho

- [ ] 504a implementado: `within` selector funciona.
- [ ] 504b implementado: `dict.map()` e `dict.filter()` funcionam.
- [ ] 504c implementado: `arguments.named` e `arguments.positional` acessíveis.
- [ ] 504d implementado: `int("ff", base: 16)` funciona.
- [ ] 504e implementado: `calc.asinh`, `calc.acosh`, `calc.atanh`, `calc.erf` funcionam.
- [ ] 504f implementado: `int.min` e `int.max` no scope global.
- [ ] 504g implementado: `range(inclusive:)` funciona.
- [ ] 504h implementado: `counter.display(at:)` funciona.
- [ ] 504i implementado: `page.bleed` aceito.
- [ ] 504j implementado: `list.marker-align` aceito.
- [ ] 504k implementado: `divider` element funciona.
- [ ] 11 testes unitários novos passam.
- [ ] Bateria P490: 20/20 MATCH preservado (não-regressão).
- [ ] PANICs: 0 (preservado).
- [ ] Documentação atualizada (L0 prompts).
- [ ] ADR-0107 checklist atualizado (11 itens marcados implementado).
- [ ] Sentinela `p504_novas_funcionalidades_0150` adicionada.
- [ ] `00_nucleo/diagnosticos/paridade-funcional-p504.md` produzido.

---

## 7. Próximo Passo (P505)

Com P504 fechado, o cristalino estará alinhado com as funcionalidades **S/XS** do 0.15.0. As funcionalidades **M/L** restantes são:

| Funcionalidade | Tamanho | Recomendação |
|----------------|---------|--------------|
| `text.variations` (variable fonts) | M | P505 = font subsystem upgrade |
| File `path` type | M | P506 = novo tipo de dados |
| Spot colors | M | Scope-out (offset printing niche) |
| HTML MathML export | L | Scope-out (HTML export não é core) |
| Bundle export | L | Scope-out (múltiplos outputs) |

**Recomendação:** P505 = **DEBT-42 Benchmark** — comparar tempo cristalino vs vanilla 0.15.0, avaliar impacto da passagem dupla do P498 e das novas funcionalidades do P504.

---

## A. Apêndice — Referência Rápida das Funcionalidades 0.15.0

```typst
// 504a: within selector
#query(selector(heading).within(figure))

// 504b: dict.map/filter
#let d = (a: 1, b: 2)
#assert(d.map((k, v) => v * 2) == (a: 2, b: 4))
#assert(d.filter((k, v) => v > 1) == (b: 2))

// 504c: arguments field access
#let f(..args) = args.named
#assert(f(x: 1, y: 2) == (x: 1, y: 2))

// 504d: int(base:)
#assert(int("ff", base: 16) == 255)
#assert(int("1010", base: 2) == 10)

// 504e: calc asinh/acosh/atanh/erf
#assert(calc.asinh(1.0) > 0.88)
#assert(calc.acosh(1.0) == 0.0)
#assert(calc.atanh(0.5) > 0.54)
#assert(calc.erf(1.0) > 0.84)

// 504f: int.min/max
#assert(int.min < 0)
#assert(int.max > 0)

// 504g: range(inclusive)
#assert(range(1, 5, inclusive: true) == (1, 2, 3, 4, 5))

// 504h: counter.display(at:)
#counter(heading).display("1.", at: <label>)

// 504i: page.bleed
#set page(bleed: 3mm)

// 504j: list.marker-align
#list(marker-align: center, [Item A], [Item B])

// 504k: divider
#divider
```
