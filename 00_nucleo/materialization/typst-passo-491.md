---

# P491 — Materialização de Argumentos Nomeados: Lote D2

> **Passo:** 491
> **Data:** 2026-06-29
> **Foco:** Fechar empiricamente os 4 gaps de args nomeados identificados no diagnóstico P490 (Grupo D2). Não declarar conclusão — medir antes e depois de cada sub-tarefa.
> **Tipo:** Implementação S-size com validação via bateria P490.
> **Tamanho:** S (~30 min de implementação + 15 min de validação).
> **ADR-0075 ACEITE** — mecanismo de comparação via `typst query --format json` vs `Introspector::query_*`.
> **ADR-0054 ACEITE** — graded parity: MATCH / DIFF / ERRO_DESCRITIVO / PANIC / AUSENTE.

---

## Metodologia

Para cada sub-tarefa D2a–D2d, implementar o arg nomeado, correr a bateria P490 (20 ficheiros), e comparar o output estrutural contra o baseline do diagnóstico.

```bash
# Baseline P490 (antes de tocar código):
cargo test --test structural_parity p490_bateria_paridade_funcional_20_ficheiros

# Após cada sub-tarefa:
cargo test --test structural_parity p491_<subtarefa>
```

Classificar cada resultado: `MATCH` | `DIFF` | `ERRO_DESCRITIVO` | `PANIC` | `AUSENTE`.

---

## Categoria 1 — `calc.log(base:)` (D2a)

### 1.1 — Estado baseline (P490)

```typst
// test-calc.typ
#calc.log(100, base: 10)
```

- **Vanilla:** `ok(2.0)`
- **Cristalino (pré-491):** `ERRO_DESCRITIVO` — arg nomeado `base:` não reconhecido; divergência consciente documentada em `rules/stdlib/calc.md:397-422`.
- **Classificação P490:** **DIFF**

### 1.2 — Implementação

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

### 1.3 — Validação

Rodar `test-calc.typ` contra vanilla e cristalino. Esperado:
- `calc.log(100, base: 10)` → **MATCH** (2.0)
- `calc.log(100, 10)` → **MATCH** (posicional preservado, não-regressão)

---

## Categoria 2 — `calc.round(digits:)` (D2b)

### 2.1 — Estado baseline (P490)

```typst
// test-calc.typ
#calc.round(3.567, digits: 2)
```

- **Vanilla:** `ok(3.57)`
- **Cristalino (pré-491):** `ERRO_DESCRITIVO` — arg `digits:` totalmente ausente.
- **Classificação P490:** **DIFF**

### 2.2 — Implementação

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

### 2.3 — Validação

Rodar `test-calc.typ` contra vanilla e cristalino. Esperado:
- `calc.round(3.567, digits: 2)` → **MATCH** (3.57)
- `calc.round(3.567)` → **MATCH** (4.0, default 0 preservado)

---

## Categoria 3 — `str(base:)` (D2c)

### 3.1 — Estado baseline (P490)

```typst
// test-str.typ
#str(255, base: 16)
```

- **Vanilla:** `ok("ff")`
- **Cristalino (pré-491):** `ERRO_DESCRITIVO` — arg `base:` totalmente ausente.
- **Classificação P490:** **DIFF**

### 3.2 — Implementação

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

### 3.3 — Validação

Rodar `test-str.typ` contra vanilla e cristalino. Esperado:
- `str(255, base: 16)` → **MATCH** ("ff")
- `str(255)` → **MATCH** ("255", default 10 preservado)
- `str(255, base: 37)` → **ERRO_DESCRITIVO** (base fora do range)

---

## Categoria 4 — `dict.at(default:)` (D2d)

### 4.1 — Estado baseline (P490)

```typst
// test-dict.typ
#let d = (a: 1, b: 2, c: 3)
#d.at("z", default: 99)
```

- **Vanilla:** `ok(99)`
- **Cristalino (pré-491):** `ERRO_DESCRITIVO` — arg `default:` totalmente ausente.
- **Classificação P490:** **DIFF**

### 4.2 — Implementação

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

### 4.3 — Validação

Rodar `test-dict.typ` contra vanilla e cristalino. Esperado:
- `d.at("z", default: 99)` → **MATCH** (99)
- `d.at("a")` → **MATCH** (1, key existente)
- `d.at("z")` → **ERRO_DESCRITIVO** (key ausente, sem default)

---

## Formato do relatório de resultados

Para cada sub-tarefa, produzir uma linha:

```
| 491a calc.log(base:) | Vanilla: ok(2.0) | Cristalino: ok(2.0) | MATCH | posicional preservado |
| 491b calc.round(digits:) | Vanilla: ok(3.57) | Cristalino: ok(3.57) | MATCH | default 0 preservado |
| 491c str(base:) | Vanilla: ok("ff") | Cristalino: ok("ff") | MATCH | base 2-36 validada |
| 491d dict.at(default:) | Vanilla: ok(99) | Cristalino: ok(99) | MATCH | erro sem default preservado |
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

| Ficheiro | Selector | Vanilla | Cristalino (pré-491) | Esperado pós-491 |
|----------|----------|---------|----------------------|------------------|
| test-calc.typ | `metadata` | ok(0) | ERRO_DESCRITIVO | **MATCH** (D2a+D2b resolvidos) |
| test-str.typ | `metadata` | ok(0) | ERRO_DESCRITIVO | **MATCH** (D2c resolvido) |
| test-dict.typ | `metadata` | ok(0) | ERRO_DESCRITIVO | **MATCH** (D2d resolvido) |
| (outros 17) | — | — | — | **preservados** (não-regressão) |

**DIFFs restantes esperados:** 9 - 4 = **5** (D1: selectors, D3: field access, D4/D5: cores/show-regex).

**PANICs esperados:** **0** (preservado).

---

## Critério de fecho

- [ ] 491a implementado: `calc.log(base:)` funciona, posicional preservado.
- [ ] 491b implementado: `calc.round(digits:)` funciona, default 0 preservado.
- [ ] 491c implementado: `str(value, base:)` funciona, base 2-36 validada.
- [ ] 491d implementado: `dict.at(key, default:)` funciona, erro sem default preservado.
- [ ] 4 testes unitários novos passam (`p491_calc_log_base_named`, `p491_calc_round_digits_named`, `p491_str_base_named`, `p491_dict_at_default_named`).
- [ ] Bateria P490 completa: 4 DIFFs de D2 viraram MATCH.
- [ ] DIFFs restantes: 5 (confirmado não-regressão).
- [ ] PANICs: 0 (preservado).
- [ ] Documentação atualizada (`rules/stdlib/calc.md` reclassificado, `foundations.md` ou `primitives-constructors.md` atualizado).
- [ ] ADR-0107 checklist atualizado (4 itens marcados implementado).
- [ ] Sentinela `p491_args_nomeados_lote_d2` adicionada em `lab/parity/tests/structural_parity.rs`.
- [ ] `00_nucleo/diagnosticos/paridade-funcional-p491.md` produzido com tabela de resultados.

---

## Próximo passo (P492)

Com P491 fechado, os gaps restantes do P490 são:

| Grupo | Gap | Tamanho | Recomendação |
|-------|-----|---------|--------------|
| D1 | Selectors ausentes (`list`, `enum`, `par`, `link`, `raw`, `quote`, `footnote`) | M-size | P492 = expansão `parse_selector` |
| D3 | Field access (`arr.dedup()`, `table.header`) | M-size | P493 = field access em coleções |
| D4/D5 | Variáveis de cor (`red`, `blue`, `green`) | S-size | P494 = scope de variáveis predefinidas |

**Recomendação:** P492 = **D4/D5 (variáveis de cor)** — S-size rápido, desbloqueia `test-stroke-sides.typ` e `test-show-regex.typ`, reduzindo DIFFs de 5 para 3 antes de atacar os M-size.

Alternativa: P492 = **D1 (selectors ausentes)** — M-size, maior impacto de usabilidade (7 elementos comuns não introspectáveis via `typst query`).

---

## A. Apêndice — Referência rápida dos gaps D2

```typst
// 491a: calc.log(base:)
#assert(calc.log(100, base: 10) == 2.0)

// 491b: calc.round(digits:)
#assert(calc.round(3.567, digits: 2) == 3.57)

// 491c: str(base:)
#assert(str(255, base: 16) == "ff")

// 491d: dict.at(default:)
#assert((a: 1).at("z", default: 99) == 99)
```
