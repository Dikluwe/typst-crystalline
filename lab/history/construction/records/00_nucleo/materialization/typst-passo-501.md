---

# P501 — Materialização de Gaps P1/P2 do Audit P500

> **Passo:** 501
> **Data:** 2026-06-29
> **Foco:** Fechar empiricamente os gaps de maior impacto e menor tamanho identificados no audit P500: `str` field access + métodos (P1), `dict.insert()`/`dict.len()` (P1), e `calc.log10`/`calc.deg`/`calc.rad` (P2). Não declarar conclusão — medir antes e depois de cada sub-tarefa.
> **Tipo:** Implementação M-size com validação via bateria P500.
> **Tamanho:** M (~45 min de implementação + 15 min de validação).
> **ADR-0075 ACEITE** — mecanismo de comparação via `typst query --format json` vs `Introspector::query_*`.
> **ADR-0054 ACEITE** — graded parity: MATCH / DIFF / ERRO_DESCRITIVO / PANIC / AUSENTE.
> **ADR-0109 ACEITE** — atomização de código.
> **Dependências:** P500 (audit de cobertura stdlib expandida), P495 (args nomeados), P496 (field access em coleções).

---

## Metodologia

Para cada sub-tarefa, implementar o gap, correr a bateria P500 (17 ficheiros), e comparar o output estrutural contra o baseline do diagnóstico P500.

```bash
# Baseline P500 (antes de tocar código):
cargo test -p typst-parity p500_audit

# Após cada sub-tarefa:
cargo test -p typst-parity p501_<subtarefa>
```

Classificar cada resultado: `MATCH` | `DIFF` | `ERRO_DESCRITIVO` | `PANIC` | `AUSENTE`.

---

## Categoria 1 — `str` Field Access + Métodos Avançados (P1)

### 1.1 — Estado baseline (P500)

```typst
// test-str-methods.typ
#"Hello".to-upper()
#"hello".contains("ell")
#"hello".find("l")
```

- **Vanilla:** `ok` — todos os métodos retornam valores.
- **Cristalino (pré-501):** `ERRO_DESCRITIVO` — `field access não suportado em str`.
- **Classificação P500:** **AUSENTE**

### 1.2 — Diagnóstico de causa raiz

O cristalino já suporta field access em `Array` (P496) e `Func` (P496), mas não em `Str`. O gap é:

1. **H1:** `eval_field_access` em `bindings.rs` não tem handler para `Value::Str`.
2. **H2:** Os métodos de `str` não existem como `Value::Func`.

**Verificação rápida:**
```bash
rg -n "eval_field_access" src/eval/bindings.rs --type rs
rg -n "Value::Str" src/eval/bindings.rs --type rs
```

### 1.3 — Implementação

**Arquivo alvo:** `src/eval/bindings.rs` e `src/stdlib/foundations.rs`

**Mudança 1 (eval):** Adicionar handler de field access para `Value::Str`:

```rust
Value::Str(s) => match field.as_str() {
    "len" => Ok(Value::Int(s.len() as i64)),
    "to-upper" => Ok(Value::Func(str_to_upper)),
    "to-lower" => Ok(Value::Func(str_to_lower)),
    "split" => Ok(Value::Func(str_split)),
    "trim" => Ok(Value::Func(str_trim)),
    "replace" => Ok(Value::Func(str_replace)),
    "to-unicode" => Ok(Value::Func(str_to_unicode)),
    "from-unicode" => Ok(Value::Func(str_from_unicode)),
    "contains" => Ok(Value::Func(str_contains)),
    "starts-with" => Ok(Value::Func(str_starts_with)),
    "ends-with" => Ok(Value::Func(str_ends_with)),
    "find" => Ok(Value::Func(str_find)),
    "rev" => Ok(Value::Func(str_rev)),
    "repeat" => Ok(Value::Func(str_repeat)),
    _ => Err("string does not have field '{}'"),
},
```

**Mudança 2 (stdlib):** Implementar cada método como `Value::Func`:

```rust
fn str_to_upper(args: Args) -> SourceResult<Value> {
    let s = args.expect::<Str>("self")?;
    Ok(Value::Str(s.to_uppercase().into()))
}

fn str_contains(args: Args) -> SourceResult<Value> {
    let s = args.expect::<Str>("self")?;
    let needle = args.expect::<Str>("needle")?;
    Ok(Value::Bool(s.contains(&needle[..])))
}

fn str_find(args: Args) -> SourceResult<Value> {
    let s = args.expect::<Str>("self")?;
    let needle = args.expect::<Str>("needle")?;
    match s.find(&needle[..]) {
        Some(pos) => Ok(Value::Int(pos as i64)),
        None => Ok(Value::Int(-1)), // ou None, conforme vanilla
    }
}

// ... implementar os restantes métodos analogamente
```

**Nota:** `str.from-unicode` é um método estático (não de instância). Verificar se o vanilla o expõe como `str.from-unicode(97)` ou `"a".to-unicode()`.

### 1.4 — Validação

Rodar `test-str-methods.typ` contra vanilla e cristalino. Esperado:
- `"Hello".to-upper()` → **MATCH** ("HELLO")
- `"hello".contains("ell")` → **MATCH** (true)
- `"hello".find("l")` → **MATCH** (2)
- Todos os métodos listados no P500 → **MATCH**

---

## Categoria 2 — `dict.insert()` / `dict.len()` (P1)

### 2.1 — Estado baseline (P500)

```typst
// test-dict-methods.typ
#let d = (a: 1, b: 2, c: 3)
#d.insert("d", 4)
#d.len()
```

- **Vanilla:** `ok` — `d` torna-se `(a: 1, b: 2, c: 3, d: 4)` e `len()` retorna 4.
- **Cristalino (pré-501):** `ERRO_DESCRITIVO` — `campo 'insert' não existe` / `campo 'len' não existe`.
- **Classificação P500:** **AUSENTE**

### 2.2 — Diagnóstico de causa raiz

`dict` já tem `keys()`, `values()`, `pairs()`, `remove()` (colateral P495). `insert()` e `len()` faltam. O gap é:

1. **H1:** `eval_field_access` para `Value::Dict` não tem `insert`/`len`.
2. **H2:** Os métodos não existem como `Value::Func`.

### 2.3 — Implementação

**Arquivo alvo:** `src/eval/bindings.rs` e `src/stdlib/collections.rs`

**Mudança 1 (eval):** Adicionar `insert` e `len` ao handler de `Value::Dict`:

```rust
Value::Dict(dict) => match field.as_str() {
    "keys" => Ok(Value::Func(dict_keys)),
    "values" => Ok(Value::Func(dict_values)),
    "pairs" => Ok(Value::Func(dict_pairs)),
    "remove" => Ok(Value::Func(dict_remove)),
    "insert" => Ok(Value::Func(dict_insert)), // novo
    "len" => Ok(Value::Func(dict_len)),      // novo
    "at" => Ok(Value::Func(dict_at)),        // já existe (P495)
    _ => Err("dictionary does not have field '{}'"),
},
```

**Mudança 2 (stdlib):** Implementar `dict_insert` e `dict_len`:

```rust
fn dict_insert(args: Args) -> SourceResult<Value> {
    let mut dict = args.expect::<Dict>("self")?;
    let key = args.expect::<Str>("key")?;
    let value = args.expect::<Value>("value")?;
    dict.insert(key, value);
    Ok(Value::Dict(dict))
}

fn dict_len(args: Args) -> SourceResult<Value> {
    let dict = args.expect::<Dict>("self")?;
    Ok(Value::Int(dict.len() as i64))
}
```

**Nota:** `dict_insert` requer `Dict` mutável. Verificar se o modelo de `Value::Dict` permite mutação ou se precisa de `Rc<RefCell<Dict>>`.

### 2.4 — Validação

Rodar `test-dict-methods.typ` contra vanilla e cristalino. Esperado:
- `d.insert("d", 4)` → **MATCH** (dict com nova chave)
- `d.len()` → **MATCH** (4)

---

## Categoria 3 — `calc.log10`, `calc.deg`, `calc.rad` (P2)

### 3.1 — Estado baseline (P500)

```typst
// test-calc-rest.typ
#calc.log10(100)
#calc.deg(180)
#calc.rad(180)
```

- **Vanilla:** `ok` — `log10(100)` = 2.0, `deg(180)` = 180.0, `rad(180)` = π.
- **Cristalino (pré-501):** `ERRO_DESCRITIVO` — `campo 'log10' não existe`.
- **Classificação P500:** **AUSENTE**

### 3.2 — Diagnóstico de causa raiz

As funções `calc.log10`, `calc.deg`, `calc.rad` não estão registradas no módulo `calc`. O gap é trivial — adicionar bindings.

### 3.3 — Implementação

**Arquivo alvo:** `src/stdlib/calc.rs`

**Mudança:** Adicionar bindings no registro do módulo `calc`:

```rust
calc_namespace.define("log10", Value::Func(native_log10));
calc_namespace.define("deg", Value::Func(native_deg));
calc_namespace.define("rad", Value::Func(native_rad));
```

**Implementações:**

```rust
fn native_log10(args: Args) -> SourceResult<Value> {
    let x = args.expect::<f64>("x")?;
    Ok(Value::Float(x.log10()))
}

fn native_deg(args: Args) -> SourceResult<Value> {
    let rad = args.expect::<f64>("rad")?;
    Ok(Value::Float(rad.to_degrees()))
}

fn native_rad(args: Args) -> SourceResult<Value> {
    let deg = args.expect::<f64>("deg")?;
    Ok(Value::Float(deg.to_radians()))
}
```

### 3.4 — Validação

Rodar `test-calc-rest.typ` contra vanilla e cristalino. Esperado:
- `calc.log10(100)` → **MATCH** (2.0)
- `calc.deg(180)` → **MATCH** (180.0) — verificar se vanilla retorna graus ou radianos
- `calc.rad(180)` → **MATCH** (π) — verificar se vanilla retorna radianos ou graus

**Nota:** Verificar a semântica exata do vanilla para `deg`/`rad` — se `deg` converte radianos para graus ou vice-versa.

---

## Formato do relatório de resultados

Para cada sub-tarefa, produzir uma linha:

```
| 501a str methods | Vanilla: ok | Cristalino: ok | MATCH | 13 métodos implementados |
| 501b dict insert/len | Vanilla: ok | Cristalino: ok | MATCH | mutação e introspecção |
| 501c calc log10/deg/rad | Vanilla: ok | Cristalino: ok | MATCH | bindings triviais |
```

Colunas: `sub-tarefa | vanilla | cristalino | classificação | notas`

---

## Testes de não-regressão

Após as 3 sub-tarefas, rodar a bateria completa P500 (17 ficheiros):

| Ficheiro | Vanilla | Cristalino pré-501 | Esperado pós-501 | Δ |
|---|---|---|---|---|
| test-str-methods.typ | ok | AUSENTE | ok | **AUSENTE → MATCH** |
| test-dict-methods.typ | ok | AUSENTE | ok | **AUSENTE → MATCH** |
| test-calc-rest.typ | ok | AUSENTE | ok | **AUSENTE → MATCH** |
| (outros 14) | — | MATCH/AUSENTE | preservados | sem regressão |

**AUSENTEs restantes esperados:** 9 - 3 = **6** (image.fit, list/enum indent, raw lang/block, footnote numbering, state/counter/context, outline.indent API).

**PANICs esperados:** **0** (preservado).

---

## Critério de fecho

- [ ] 501a implementado: field access em `str` + 13 métodos.
- [ ] 501b implementado: `dict.insert()` e `dict.len()`.
- [ ] 501c implementado: `calc.log10()`, `calc.deg()`, `calc.rad()`.
- [ ] 3 testes unitários novos passam (`p501_str_methods`, `p501_dict_insert_len`, `p501_calc_log10_deg_rad`).
- [ ] Bateria P500: 3 AUSENTEs viraram MATCH.
- [ ] AUSENTEs restantes: 6 (confirmado não-regressão).
- [ ] PANICs: 0 (preservado).
- [ ] Documentação atualizada (`rules/stdlib/foundations.md` para str/dict; `rules/stdlib/calc.md` para log10/deg/rad).
- [ ] ADR-0107 checklist atualizado (3 itens marcados implementado).
- [ ] Sentinela `p501_gaps_p1_p2` adicionada em `lab/parity/tests/structural_parity.rs`.
- [ ] `00_nucleo/diagnosticos/paridade-funcional-p501.md` produzido com tabela de resultados.

---

## Próximo passo (P502)

Com P501 fechado, os gaps restantes do P500 são:

| Prioridade | Gap | Tamanho | Recomendação |
|---|---|---|---|
| P2 | `image.fit` | S | P502a |
| P2 | `list`/`enum` `indent`, `body-indent`, `tight` | M | P502b |
| P2 | `#raw(lang:, block:)` | S | P502c |
| P3 | `footnote(numbering:)` | XS | P502d |
| P3 | `state.update/get` + `context` + `counter` | L | P503 (trilha separada) |
| P3 | Alinhar API `outline.indent` com vanilla | S | P502e |

**Recomendação:** P502 = **image.fit + raw(lang:, block:) + footnote(numbering:) + outline.indent** — 4 gaps S/XS que podem ser fechados rapidamente, reduzindo AUSENTEs de 6 para 2 antes de atacar o M-size `list`/`enum` indent e o L-size `state`/`counter`/`context`.

---

## A. Apêndice — Referência rápida dos gaps P1/P2

```typst
// 501a: str methods
#assert("Hello".to-upper() == "HELLO")
#assert("hello".contains("ell") == true)
#assert("hello".find("l") == 2)
#assert("hello".split("l") == ("he", "", "o"))
#assert("  hello  ".trim() == "hello")
#assert("hello".replace("l", "r") == "herro")
#assert("abc".to-unicode() == (97, 98, 99))
#assert(str.from-unicode(97) == "a")
#assert("hello".starts-with("he") == true)
#assert("hello".ends-with("lo") == true)
#assert("hello".rev() == "olleh")
#assert("hello".repeat(3) == "hellohellohello")

// 501b: dict insert/len
#let d = (a: 1, b: 2)
#d.insert("c", 3)
#assert(d.len() == 3)

// 501c: calc log10/deg/rad
#assert(calc.log10(100) == 2.0)
#assert(calc.deg(calc.pi) == 180.0)
#assert(calc.rad(180.0) == calc.pi)
```
