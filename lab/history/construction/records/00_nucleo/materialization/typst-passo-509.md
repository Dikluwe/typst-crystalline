---

# P509 — Materialização de Stdlib Core: Lote A (Brechas Reais P508)

> **Passo:** 509
> **Data:** 2026-06-30
> **Foco:** Fechar empiricamente as 6 brechas reais identificadas no diagnóstico P508: `str` field access incompleto, `dict` métodos ausentes, `image(fit:)`, `raw(lang:, block:)`, e `query()` com selector. Não declarar conclusão — medir antes e depois de cada sub-tarefa.
> **Tipo:** Implementação S/M-size com validação via corpus P490+P500.
> **Tamanho:** M (~45 min de implementação + 15 min de validação).
> **ADR-0107 ACEITE** — paridade é de linguagem, não de mecânica.
> **ADR-0115 ACEITE** — infra de benchmark.
> **Dependências:** P508 (diagnóstico real de brechas), P506 (runtime state), P505 (list/enum indent), P504 (novas funcionalidades 0.15.0).

---

## 1. Contexto

O diagnóstico P508 corrigiu falsas premissas do P507/P508 e identificou **6 brechas reais** no cristalino:

| Brecha | Estado P508 | Impacto | Tamanho |
|--------|-------------|---------|---------|
| `str` field access (`.len`, `.first`, `.last`, `.at`, `.slice`, `.rev`, `.clusters`) | Parcial — `.split`/`.contains`/`.trim` existem; resto ausente | Alto | M |
| `dict` methods (`.len`, `.insert`, `.remove`) | Ausentes | Alto | S |
| `image(fit:)` | Ausente | Médio | S |
| `raw(lang:, block:)` | Ausente | Médio | S |
| `query()` com selector/location | Falha sem argumento selector | Médio | M |
| `fontdb` system discovery | Ausente — não bloqueia linguagem | Crítico para produção | M (Trilha 5) |

Este passo fecha as **5 primeiras** (paridade de linguagem). A sexta (`fontdb`) é Trilha 5 (produção).

---

## 2. Metodologia

Para cada sub-tarefa, implementar a brecha, correr o corpus P490+P500 (37 documentos), e verificar se a falha `test-metadata-query.typ` e as funcionalidades ausentes são resolvidas.

```bash
# Baseline P508 (antes de tocar código):
for f in lab/parity/corpus/p490/*.typ lab/parity/corpus/p500/*.typ; do
  target/release/typst "$f" /tmp/out.pdf >/dev/null 2>&1     && echo "OK: $(basename $f)"     || echo "FAIL: $(basename $f)"
done
# Esperado: 36/37 OK, 1 FAIL (test-metadata-query.typ)

# Após cada sub-tarefa:
# Re-executar o corpus e verificar regressões
```

---

## 3. Sub-tarefas

### 3.1 — `str` Field Access Completo (509a)

**Estado P508:** `.split`, `.contains`, `.starts-with`, `.ends-with`, `.trim`, `.replace` existem. `.len`, `.first`, `.last`, `.at`, `.slice`, `.rev`, `.clusters` ausentes.

**Implementação:**

Adicionar ao handler de field access para `Value::Str` em `eval/bindings.rs`:

```rust
Value::Str(s) => match field.as_str() {
    // Já existentes (P501/P502):
    "split" => Ok(Value::Func(str_split)),
    "contains" => Ok(Value::Func(str_contains)),
    "starts-with" => Ok(Value::Func(str_starts_with)),
    "ends-with" => Ok(Value::Func(str_ends_with)),
    "trim" => Ok(Value::Func(str_trim)),
    "replace" => Ok(Value::Func(str_replace)),
    // Novos (P509):
    "len" => Ok(Value::Int(s.len() as i64)),
    "first" => Ok(s.chars().next().map(|c| Value::Str(c.to_string().into())).unwrap_or(Value::None)),
    "last" => Ok(s.chars().last().map(|c| Value::Str(c.to_string().into())).unwrap_or(Value::None)),
    "at" => Ok(Value::Func(str_at)),
    "slice" => Ok(Value::Func(str_slice)),
    "rev" => Ok(Value::Func(str_rev)),
    "clusters" => Ok(Value::Func(str_clusters)),
    _ => Err("string does not have field '{}'"),
},
```

**Implementações:**

```rust
fn str_at(args: Args) -> SourceResult<Value> {
    let s = args.expect::<Str>("self")?;
    let index = args.expect::<i64>("index")?;
    let chars: Vec<char> = s.chars().collect();
    let idx = if index < 0 { chars.len() as i64 + index } else { index } as usize;
    if idx >= chars.len() {
        return Err("index out of bounds");
    }
    Ok(Value::Str(chars[idx].to_string().into()))
}

fn str_slice(args: Args) -> SourceResult<Value> {
    let s = args.expect::<Str>("self")?;
    let start = args.expect::<i64>("start")?;
    let end = args.named.get("end").and_then(|v| v.cast::<i64>().ok());
    let count = args.named.get("count").and_then(|v| v.cast::<i64>().ok());

    // Validação 0.15.0: end + count = erro
    if end.is_some() && count.is_some() {
        return Err("cannot specify both end and count");
    }

    let chars: Vec<char> = s.chars().collect();
    let start_idx = if start < 0 { chars.len() as i64 + start } else { start } as usize;
    let end_idx = match (end, count) {
        (Some(e), None) => {
            let e = if e < 0 { chars.len() as i64 + e } else { e } as usize;
            e.min(chars.len())
        }
        (None, Some(c)) => {
            (start_idx + c as usize).min(chars.len())
        }
        (None, None) => chars.len(),
    };

    let result: String = chars[start_idx..end_idx].iter().collect();
    Ok(Value::Str(result.into()))
}

fn str_rev(args: Args) -> SourceResult<Value> {
    let s = args.expect::<Str>("self")?;
    let reversed: String = s.chars().rev().collect();
    Ok(Value::Str(reversed.into()))
}

fn str_clusters(args: Args) -> SourceResult<Value> {
    let s = args.expect::<Str>("self")?;
    let clusters: Vec<Value> = s.graphemes(true).map(|g| Value::Str(g.into())).collect();
    Ok(Value::Array(clusters))
}
```

**Nota:** `str_clusters` requer `unicode-segmentation` crate para `graphemes()`. Verificar se já está no `Cargo.toml`.

---

### 3.2 — `dict` Methods (509b)

**Estado P508:** `.keys`, `.values`, `.pairs` existem (colateral P495). `.len`, `.insert`, `.remove` ausentes.

**Implementação:**

```rust
Value::Dict(dict) => match field.as_str() {
    // Já existentes:
    "keys" => Ok(Value::Func(dict_keys)),
    "values" => Ok(Value::Func(dict_values)),
    "pairs" => Ok(Value::Func(dict_pairs)),
    "at" => Ok(Value::Func(dict_at)),
    // Novos (P509):
    "len" => Ok(Value::Func(dict_len)),
    "insert" => Ok(Value::Func(dict_insert)),
    "remove" => Ok(Value::Func(dict_remove)),
    _ => Err("dictionary does not have field '{}'"),
},
```

**Implementações:**

```rust
fn dict_len(args: Args) -> SourceResult<Value> {
    let dict = args.expect::<Dict>("self")?;
    Ok(Value::Int(dict.len() as i64))
}

fn dict_insert(args: Args) -> SourceResult<Value> {
    let mut dict = args.expect::<Dict>("self")?;
    let key = args.expect::<Str>("key")?;
    let value = args.expect::<Value>("value")?;
    dict.insert(key, value);
    Ok(Value::Dict(dict))
}

fn dict_remove(args: Args) -> SourceResult<Value> {
    let mut dict = args.expect::<Dict>("self")?;
    let key = args.expect::<Str>("key")?;
    match dict.remove(&key) {
        Some(value) => Ok(value),
        None => Err("dictionary does not contain key"),
    }
}
```

---

### 3.3 — `image(fit:)` (509c)

**Estado P508:** `image("x.png", fit: "contain")` → `unknown variable: fit`.

**Implementação:**

Adicionar `fit` como arg nomeado ao construtor `image`:

```rust
fn native_image(args: Args) -> SourceResult<Value> {
    let path = args.expect::<Str>("path")?;
    let width = args.named.get("width").and_then(|v| v.cast::<Length>().ok());
    let height = args.named.get("height").and_then(|v| v.cast::<Length>().ok());
    let fit = args.named.get("fit")
        .and_then(|v| v.cast::<Str>().ok())
        .unwrap_or_else(|| "cover".into());

    if !matches!(fit.as_str(), "contain" | "cover" | "stretch") {
        return Err("fit must be 'contain', 'cover' or 'stretch'");
    }

    Ok(Value::Content(Content::Image {
        path: path.into(),
        width,
        height,
        fit: fit.into(),
    }))
}
```

**Nota:** Se `Content::Image` não tem campo `fit`, adicionar. O layout de `Image` precisa respeitar `fit` no cálculo de dimensões (scope-out de renderização específica se necessário, mas o arg deve ser aceito).

---

### 3.4 — `raw(lang:, block:)` (509d)

**Estado P508:** `raw(lang: "rust", block: true, "...")` → `unknown variable: lang`.

**Implementação:**

```rust
fn native_raw(args: Args) -> SourceResult<Value> {
    let content = args.expect::<Str>("content")?;
    let lang = args.named.get("lang").and_then(|v| v.cast::<Str>().ok());
    let block = args.named.get("block")
        .and_then(|v| v.cast::<bool>().ok())
        .unwrap_or(true);

    Ok(Value::Content(Content::Raw {
        text: content.into(),
        lang,
        block,
    }))
}
```

**Nota:** Se `Content::Raw` não tem campos `lang`/`block`, adicionar. O highlighting pode ser scope-out (fallback monocromo), mas os args devem ser aceitos.

---

### 3.5 — `query()` com Selector/Location (509e)

**Estado P508:** `query(<tag>)` sem argumento selector falha com `query() requer string ou location, recebeu none`.

**Diagnóstico:** O `query()` do cristalino exige um argumento posicional (string ou location). O vanilla permite `query(<label>)` — query por label sem selector explícito.

**Implementação:**

```rust
fn native_query(args: Args) -> SourceResult<Value> {
    let selector = if let Some(label) = args.items.first().and_then(|v| v.as_label()) {
        // query(<label>) → query tudo no label
        Selector::Any
    } else if let Some(loc) = args.items.first().and_then(|v| v.as_location()) {
        // query(location) → query no location
        Selector::Any
    } else if let Some(s) = args.items.first().and_then(|v| v.as_str()) {
        // query("selector") → parse selector
        parse_selector(s).ok_or("invalid selector")?
    } else {
        return Err("query() requires a selector string, location, or label");
    };

    // ... resto da implementação
}
```

**Nota:** A semântica exata de `query(<label>)` no vanilla precisa ser verificada. Pode ser: "retorna todos os elementos no ponto do label" ou "retorna o elemento etiquetado com o label".

---

## 4. Validação

### 4.1 Corpus P490+P500

Após as 5 sub-tarefas, re-executar o corpus:

```bash
for f in lab/parity/corpus/p490/*.typ lab/parity/corpus/p500/*.typ; do
  target/release/typst "$f" /tmp/out.pdf >/dev/null 2>&1     && echo "OK: $(basename $f)"     || echo "FAIL: $(basename $f)"
done
```

**Esperado:** 37/37 OK (incluindo `test-metadata-query.typ`).

### 4.2 Testes Individuais

```typst
// 509a: str field access
#assert("hello".len() == 5)
#assert("hello".first() == "h")
#assert("hello".last() == "o")
#assert("hello".at(1) == "e")
#assert("hello".slice(1, 4) == "ell")
#assert("hello".rev() == "olleh")
#assert("hello".clusters() == ("h", "e", "l", "l", "o"))

// 509b: dict methods
#let d = (a: 1, b: 2)
#assert(d.len() == 2)
#d.insert("c", 3)
#assert(d.len() == 3)
#d.remove("a")
#assert(d.len() == 2)

// 509c: image fit
#image("test.png", fit: "contain")

// 509d: raw lang/block
#raw("fn main() {}", lang: "rust", block: true)

// 509e: query com label
#metadata("info") <tag>
#context query(<tag>)
```

---

## 5. Critério de Fecho

- [ ] 509a implementado: `str.len`, `str.first`, `str.last`, `str.at`, `str.slice`, `str.rev`, `str.clusters`.
- [ ] 509b implementado: `dict.len`, `dict.insert`, `dict.remove`.
- [ ] 509c implementado: `image(fit:)` aceita `contain`/`cover`/`stretch`.
- [ ] 509d implementado: `raw(lang:, block:)` aceita args nomeados.
- [ ] 509e implementado: `query(<label>)` funciona sem selector explícito.
- [ ] Corpus P490+P500: 37/37 OK (zero falhas).
- [ ] 5 testes unitários novos passam (`p509_str_field_access`, `p509_dict_methods`, `p509_image_fit`, `p509_raw_lang_block`, `p509_query_label`).
- [ ] PANICs: 0 (preservado).
- [ ] Documentação atualizada (`rules/stdlib/foundations.md`, `rules/stdlib/structural.md`).
- [ ] Sentinela `p509_stdlib_core_lote_a` adicionada.
- [ ] `00_nucleo/diagnosticos/paridade-funcional-p509.md` produzido.

---

## 6. Próximo Passo (P510)

Com P509 fechado, o corpus P490+P500 atinge **37/37 OK**. As brechas restantes são:

| Brecha | Tamanho | Trilha |
|--------|---------|--------|
| `fontdb` system discovery | M | **Trilha 5** (produção) |
| Subsetting de fontes | L | **Trilha 5** (produção) |
| Math styles (12 funções) | M | P510+ (linguagem) |
| Math elements (7 granulares) | M | P510+ (linguagem) |
| Table/Grid HLine/VLine | M | P510+ (linguagem) |
| Curve elements | M | P510+ (linguagem) |

**Recomendação:** P510 = **Math Styles** (`bb`, `bold`, `cal`, `frak`, `italic`, `mono`, `sans`, `scr`, `script`, `serif`, `sscript`, `upright`) — 12 funções de mapeamento de nome para transformação de glifo, S-size coletivo.

Alternativa: Iniciar **Trilha 5 (fontdb)** em paralelo.

---

## A. Apêndice — Referência Rápida

```typst
// 509a: str field access completo
#assert("hello".len() == 5)
#assert("hello".first() == "h")
#assert("hello".last() == "o")
#assert("hello".at(1) == "e")
#assert("hello".slice(1, 4) == "ell")
#assert("hello".rev() == "olleh")
#assert("hello".clusters() == ("h", "e", "l", "l", "o"))

// 509b: dict methods
#let d = (a: 1, b: 2)
#assert(d.len() == 2)
#d.insert("c", 3)
#assert(d.len() == 3)
#d.remove("a")
#assert(d.len() == 2)

// 509c: image fit
#image("test.png", fit: "contain")

// 509d: raw lang/block
#raw("fn main() {}", lang: "rust", block: true)

// 509e: query com label
#metadata("info") <tag>
#context query(<tag>)
```
