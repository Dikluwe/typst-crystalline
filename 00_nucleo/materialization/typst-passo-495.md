---

# P495 — Materialização de Argumentos Nomeados: Lote D2

> **Passo:** 495
> **Data:** 2026-06-29
> **Foco:** Fechar empiricamente os 4 gaps de argumentos nomeados identificados no diagnóstico P490 (Grupo D2), ainda persistentes após P494. Não declarar conclusão — medir antes e depois de cada sub-tarefa.
> **Tipo:** Implementação S-size com validação via bateria P490.
> **Tamanho:** S (~30 min de implementação + 15 min de validação).
> **ADR-0075 ACEITE** — mecanismo de comparação via `typst query --format json` vs `Introspector::query_*`.
> **ADR-0054 ACEITE** — graded parity: MATCH / DIFF / ERRO_DESCRITIVO / PANIC / AUSENTE.
> **ADR-0109 ACEITE** — atomização de código.
> **Dependências:** P490 (diagnóstico), P494 (baseline pós-selectors), P403/P405 (infraestrutura `Args::named`), P466 (array methods).

---

## Metodologia

Para cada sub-tarefa D2a–D2d, implementar o arg nomeado, correr a bateria P490 (20 ficheiros), e comparar o output estrutural contra o baseline do diagnóstico P494.

```bash
# Baseline P494 (antes de tocar código):
cargo test --test structural_parity p494_selectores_elementos_documento

# Após cada sub-tarefa:
cargo test --test structural_parity p495_<subtarefa>
```

Classificar cada resultado: `MATCH` | `DIFF` | `ERRO_DESCRITIVO` | `PANIC` | `AUSENTE`.

---

## Categoria 1 — `calc.log(base:)` (D2a)

### 1.1 — Estado baseline (P494)

```typst
// test-calc.typ
#calc.log(100, base: 10)
```

- **Vanilla:** `ok(2.0)`
- **Cristalino (pré-495):** `ERRO_DESCRITIVO` — arg nomeado `base:` não reconhecido; divergência consciente documentada em `rules/stdlib/calc.md:397-422`.
- **Classificação P494:** **DIFF**

### 1.2 — Diagnóstico de causa raiz

`entities/args.md:33-35` define `Args { named: IndexMap<EcoString, Value> }`. O parser já reconhece sintaxe `ident: expr` (lexer: `maybe_math_named_arg` em `rules/lexer/mod.md:87`). Dezenas de funções nativas já consomem `args.named.get("key")` (`native_duration`, `outline`, `bibliography`, `figure`, etc.).

O gap é **binding específico** em `calc.log`: o eval não verifica `args.named.get("base")` antes de `args.items.get(1)`.

**Verificação rápida:**
```bash
rg -n "calc_log\|native_log" src/stdlib/ --type rs
rg -n "args.named.get" src/stdlib/calc.rs --type rs
```

### 1.3 — Implementação

**Arquivo alvo:** `src/stdlib/calc.rs` (ou equivalente no cristalino)

**Mudança:** Verificar `args.named.get("base")` antes de `args.items.get(1)`. Se `base` presente em `named`, usá-lo. Se ausente, fallback para posicional (preserva compatibilidade com código cristalino existente).

**Tipo:** `Int | Float` (coerção para `f64` como já feito para `x`).

**Pseudo-código:**
```rust
let x = args.expect::<f64>("x")?;
let base = if let Some(val) = args.named.get("base") {
    val.cast::<f64>()?
} else if let Some(val) = args.items.get(1) {
    val.cast::<f64>()?
} else {
    return Err("missing argument: base");
};
```

### 1.4 — Validação

Rodar `test-calc.typ` contra vanilla e cristalino. Esperado:
- `calc.log(100, base: 10)` → **MATCH** (2.0)
- `calc.log(100, 10)` → **MATCH** (posicional preservado, não-regressão)

---

## Categoria 2 — `calc.round(digits:)` (D2b)

### 2.1 — Estado baseline (P494)

```typst
// test-calc.typ
#calc.round(3.567, digits: 2)
```

- **Vanilla:** `ok(3.57)`
- **Cristalino (pré-495):** `ERRO_DESCRITIVO` — arg `digits:` totalmente ausente.
- **Classificação P494:** **DIFF**

### 2.2 — Diagnóstico de causa raiz

`calc.round` provavelmente aceita apenas 1 argumento posicional. O eval não verifica `args.named.get("digits")`.

### 2.3 — Implementação

**Arquivo alvo:** `src/stdlib/calc.rs`

**Mudança:** Adicionar parâmetro `digits: Int` com default `0`.

**Algoritmo:**
```rust
let x = args.expect::<f64>("x")?;
let digits = args.named.get("digits")
    .and_then(|v| v.cast::<i64>().ok())
    .unwrap_or(0) as i32;
let factor = 10f64.powi(digits);
let result = (x * factor).round() / factor;
```

**Nota:** `digits` negativo arredonda para dezenas/centenas (comportamento vanilla).

### 2.4 — Validação

Rodar `test-calc.typ` contra vanilla e cristalino. Esperado:
- `calc.round(3.567, digits: 2)` → **MATCH** (3.57)
- `calc.round(3.567)` → **MATCH** (4.0, default 0 preservado)

---

## Categoria 3 — `str(base:)` (D2c)

### 3.1 — Estado baseline (P494)

```typst
// test-str.typ
#str(255, base: 16)
```

- **Vanilla:** `ok("ff")`
- **Cristalino (pré-495):** `ERRO_DESCRITIVO` — arg `base:` totalmente ausente.
- **Classificação P494:** **DIFF**

### 3.2 — Diagnóstico de causa raiz

Construtor `str()` provavelmente aceita apenas string posicional ou nada (empty string). O eval não verifica `args.named.get("base")` quando o primeiro argumento é `Int`.

### 3.3 — Implementação

**Arquivo alvo:** `src/stdlib/foundations.rs` (ou `src/eval/constructors.rs`, onde `str` é definido)

**Mudança:** Se primeiro argumento posicional é `Int` e `args.named.get("base")` está presente, converter inteiro para string na base indicada. `base` deve ser `Int` entre 2 e 36.

**Algoritmo:**
```rust
if let Some(Value::Int(n)) = args.items.first() {
    let base = args.named.get("base")
        .and_then(|v| v.cast::<i64>().ok())
        .unwrap_or(10) as u32;
    if !(2..=36).contains(&base) {
        return Err("base must be between 2 and 36");
    }
    let result = format_radix(*n, base);
    return Ok(Value::Str(result.into()));
}
```

**Implementação `format_radix`:**
```rust
fn format_radix(mut n: i64, base: u32) -> String {
    if n == 0 { return "0".to_string(); }
    let mut result = String::new();
    let negative = n < 0;
    let mut n = n.abs();
    while n > 0 {
        let digit = (n % base as i64) as u32;
        let c = std::char::from_digit(digit, base).unwrap();
        result.push(c);
        n /= base as i64;
    }
    if negative { result.push('-'); }
    result.chars().rev().collect()
}
```

### 3.4 — Validação

Rodar `test-str.typ` contra vanilla e cristalino. Esperado:
- `str(255, base: 16)` → **MATCH** ("ff")
- `str(255)` → **MATCH** ("255", default 10 preservado)
- `str(255, base: 37)` → **ERRO_DESCRITIVO** (base fora do range)

---

## Categoria 4 — `dict.at(default:)` (D2d)

### 4.1 — Estado baseline (P494)

```typst
// test-dict.typ
#let d = (a: 1, b: 2, c: 3)
#d.at("z", default: 99)
```

- **Vanilla:** `ok(99)`
- **Cristalino (pré-495):** `ERRO_DESCRITIVO` — arg `default:` totalmente ausente.
- **Classificação P494:** **DIFF**

### 4.2 — Diagnóstico de causa raiz

`dict.at(key)` aceita apenas `key` posicional. `entities/func.md:35` mostra `default: Option<Value>` na definição de parâmetro, mas o eval do call site não verifica `args.named.get("default")`.

### 4.3 — Implementação

**Arquivo alvo:** `src/stdlib/foundations.rs` (ou onde `dict` methods são definidas)

**Mudança:** `dict.at(key, default: value)` retorna `value` se `key` não existe. Se `default` ausente, comportamento atual preservado.

**Pseudo-código:**
```rust
let key = args.expect::<Str>("key")?;
let default = args.named.get("default");

match dict.get(&key) {
    Some(value) => Ok(value.clone()),
    None => match default {
        Some(val) => Ok(val.clone()),
        None => Err("dictionary does not contain key"),
    },
}
```

### 4.4 — Validação

Rodar `test-dict.typ` contra vanilla e cristalino. Esperado:
- `d.at("z", default: 99)` → **MATCH** (99)
- `d.at("a")` → **MATCH** (1, key existente)
- `d.at("z")` → **ERRO_DESCRITIVO** (key ausente, sem default)

---

## Formato do relatório de resultados

Para cada sub-tarefa, produzir uma linha:

```
| 495a calc.log(base:) | Vanilla: ok(2.0) | Cristalino: ok(2.0) | MATCH | posicional preservado |
| 495b calc.round(digits:) | Vanilla: ok(3.57) | Cristalino: ok(3.57) | MATCH | default 0 preservado |
| 495c str(base:) | Vanilla: ok("ff") | Cristalino: ok("ff") | MATCH | base 2-36 validada |
| 495d dict.at(default:) | Vanilla: ok(99) | Cristalino: ok(99) | MATCH | erro sem default preservado |
```

Colunas: `sub-tarefa | vanilla | cristalino | classificação | notas`

**Classificações:**
- `MATCH` — output estruturalmente equivalente.
- `DIFF` — ambos produzem resultado mas diferente (investigar).
- `ERRO_DESCRITIVO` — cristalino retorna erro com mensagem clara (aceitável se scope-out).
- `PANIC` — cristalino crasha (bug prioritário, deve ser zero).
- `AUSENTE` — funcionalidade não reconhecida.

---

## Testes de não-regressão

Após as 4 sub-tarefas, rodar a bateria completa P490 (20 ficheiros):

| Ficheiro | Selector | Vanilla | Cristalino pré-495 | Esperado pós-495 | Δ |
|---|---|---|---|---|---|
| test-calc.typ | `metadata` | ok(0) | ERRO_DESCRITIVO | ok(0) | **DIFF → MATCH** |
| test-str.typ | `metadata` | ok(0) | ERRO_DESCRITIVO | ok(0) | **DIFF → MATCH** |
| test-dict.typ | `metadata` | ok(0) | ERRO_DESCRITIVO | ok(0) | **DIFF → MATCH** |
| test-table.typ | `table` | ok(1) | ERRO_DESCRITIVO | ERRO_DESCRITIVO | DIFF (D3) |
| test-array.typ | `metadata` | ok(0) | ERRO_DESCRITIVO | ERRO_DESCRITIVO | DIFF (D3) |
| test-show-regex.typ | `heading` | ok(0) | ERRO_DESCRITIVO | ERRO_DESCRITIVO | DIFF (D4/D5) |
| test-show-where-multi.typ | `heading` | ok(1) | ERRO_DESCRITIVO | ERRO_DESCRITIVO | DIFF (D3) |
| test-stroke-sides.typ | `heading` | ok(0) | ERRO_DESCRITIVO | ERRO_DESCRITIVO | DIFF (D4/D5) |
| (outros 12) | — | — | MATCH | MATCH | preservados |

**DIFFs restantes esperados:** 7 - 4 = **3** (D3: table/array/show-where-multi; D4/D5: stroke-sides/show-regex).  
**PANICs esperados:** **0** (preservado).  
**AUSENTEs esperados:** 2 (test-set-local, test-columns — count=0, não regressão).

---

## Critério de fecho

- [ ] 495a implementado: `calc.log(base:)` funciona, posicional preservado.
- [ ] 495b implementado: `calc.round(digits:)` funciona, default 0 preservado.
- [ ] 495c implementado: `str(value, base:)` funciona, base 2-36 validada.
- [ ] 495d implementado: `dict.at(key, default:)` funciona, erro sem default preservado.
- [ ] 4 testes unitários novos passam (`p495_calc_log_base_named`, `p495_calc_round_digits_named`, `p495_str_base_named`, `p495_dict_at_default_named`).
- [ ] Bateria P490 completa: 4 DIFFs de D2 viraram MATCH.
- [ ] DIFFs restantes: 3 (confirmado não-regressão).
- [ ] PANICs: 0 (preservado).
- [ ] Documentação atualizada (`rules/stdlib/calc.md` reclassificado; `rules/stdlib/foundations.md` ou `primitives-constructors.md` atualizado).
- [ ] ADR-0107 checklist atualizado (4 itens marcados implementado).
- [ ] Sentinela `p495_args_nomeados_lote_d2` adicionada em `lab/parity/tests/structural_parity.rs`.
- [ ] `00_nucleo/diagnosticos/paridade-funcional-p495.md` produzido com tabela de resultados.

---

## Próximo passo (P496)

Com P495 fechado, os gaps restantes do P490 são:

| Grupo | Gap | Tamanho | Recomendação |
|---|---|---|---|
| D3 | Field access em coleções (`arr.dedup()`, `table.header/footer`, `show.where(multi)`) | M-size | P496 = field access em arrays + table namespace + Where encadeado |
| D4/D5 | Variáveis de cor predefinidas (`red`, `blue`, `green`) + `text()` em show-regex | S-size | P497 = scope de variáveis predefinidas |

**Recomendação:** P496 = **D3 (field access em coleções)** — M-size, mas fecha 3 DIFFs de uma vez (test-table, test-array, test-show-where-multi), reduzindo o total de DIFFs de 3 para 0 antes do S-size D4/D5.

Alternativa: P496 = **D4/D5 (variáveis de cor)** — S-size rápido, desbloqueia `test-stroke-sides.typ` e `test-show-regex.typ`, mas deixa D3 como M-size para P497.

---

## A. Apêndice — Referência rápida dos gaps D2

```typst
// 495a: calc.log(base:)
#assert(calc.log(100, base: 10) == 2.0)

// 495b: calc.round(digits:)
#assert(calc.round(3.567, digits: 2) == 3.57)

// 495c: str(base:)
#assert(str(255, base: 16) == "ff")

// 495d: dict.at(default:)
#assert((a: 1).at("z", default: 99) == 99)
```
