# Passo 405 — Activação de `Duration`: constructor + operações básicas (S-M)

**Tipo**: Activação de tipo modelado (L1 — stdlib constructor + eval ops; zero I/O; zero tipo novo; reusa `Value::Duration` P400).
**Data**: 2026-06-22.
**Padrão**: diagnóstico-primeiro (sonda 389 + P400); medir-antes-de-decidir (ADR-0108).
**ADRs relevantes**: ADR-0017 (portão aberto P395), ADR-0107 (paridade linguagem), ADR-0054 (graded scope-out — operações temporais avançadas: `.in()`, `.display()`, etc.).
**Sonda fonte**: `typst-sonda-ausentes-ordem-passo-389.md` §2D — `Value::Duration` modelado em P400; `duration()` stdlib + operações básicas ausentes.

> **Nota de numeração.** Um passo só. Não numerar à frente.
> **Nota de marco.** Paralelo ao P404 (Decimal aritmética). Série de activação de tipos S modelados: P404 Decimal → P405 Duration → P406 Version.

---

## 1. Contexto

P400 modelou `Value::Duration` (tipo L1 puro `Duration { nanos: u64 }`). Este passo **activa o tipo** para uso user-facing: constructor stdlib + operações básicas eval + comparações.

No vanilla:
```typ
#let d = duration(days: 3, hours: 2, minutes: 30)
#let d2 = duration(seconds: 90)
#d + d2
#d > d2
```

Operações básicas: `+`, `-`, `*`, `/` (com Int/Float), `==`, `!=`, `<`, `>`, `<=`, `>=`.
Operações avançadas (scope-out): `.in(seconds)`, `.display()`, `.hours()`, etc.

---

## 2. Decisão de engenharia

### 2.1 — `native_duration` stdlib

```rust
// signature paralela a vanilla: duration(days, hours, minutes, seconds, milliseconds, microseconds, nanoseconds)
// Todos opcionais, default 0. Pelo menos um deve ser positivo (ou zero total = zero duration).
native_duration(
    days: Option<Int>,        // default 0
    hours: Option<Int>,       // default 0
    minutes: Option<Int>,      // default 0
    seconds: Option<Int>,      // default 0
    milliseconds: Option<Int>, // default 0
    microseconds: Option<Int>, // default 0
    nanoseconds: Option<Int>,  // default 0
) -> Value::Duration
```

**Decisão ADR-0107**: paridade com a **forma** do vanilla — `duration(days: 3, hours: 2)` constrói um intervalo. A mecânica interna (nanos u64) é invisível.

**Validação**: todos os argumentos devem ser `Int` não-negativos. Negativo → `Err`. Overflow de u64 → `Err` (saturar ou rejeitar? Decisão: rejeitar com mensagem clara).

**Cálculo**:
```rust
let total_nanos = days as u128 * DAY_NANOS
                + hours as u128 * HOUR_NANOS
                + minutes as u128 * MINUTE_NANOS
                + seconds as u128 * SECOND_NANOS
                + milliseconds as u128 * 1_000_000
                + microseconds as u128 * 1_000
                + nanoseconds as u128;

if total_nanos > u64::MAX as u128 {
    bail!("duration excede o máximo suportado");
}
```

### 2.2 — Operações eval básicas

| Operador | Operandos | Resultado | Nota |
|----------|-----------|-----------|------|
| `+` | Duration + Duration | Duration | saturar ou overflow? → rejeitar |
| `-` | Duration - Duration | Duration | underflow (negativo) → rejeitar |
| `*` | Duration * Int | Duration | overflow → rejeitar |
| `*` | Duration * Float | Duration | truncar nanos (perda sub-nano) |
| `/` | Duration / Int | Duration | divisão por zero → rejeitar; truncar |
| `/` | Duration / Float | Duration | truncar nanos |
| `/` | Duration / Duration | Float | razão como f64 |
| `==`, `!=` | Duration vs Duration | Bool | via PartialEq (nanos) |
| `<`, `>`, `<=`, `>=` | Duration vs Duration | Bool | via Ord (nanos) |

**Decisão overflow/underflow**: rejeitar com `Err` claro. Não saturar silenciosamente (surpreendente). Não wrap (bug). O vanilla provavelmente usa bigint internamente; o cristalino usa u64 com checks.

**Decisão ADR-0054 (graded)**: operações avançadas (`.in(unit)`, `.display()`, `.hours()`, `.minutes()`, etc.) são scope-out. Este passo cobre só as 8 operações básicas acima + constructor.

### 2.3 — Cast adicional

P400 implementou cast: `Duration → Duration`, `Int → Duration`, `Float → Duration`.

Este passo adiciona:
- `Duration → Int` (nanossegundos totais — truncamento)
- `Duration → Float` (segundos totais como f64 — perda sub-nano)

**Decisão**: `Duration → Int` retorna nanos totais. `Duration → Float` retorna segundos (nanos / 1e9). Paridade vanilla? Verificar se o vanilla tem cast implícito. Se não, scope-out.

**Simplificação**: se o vanilla não tem cast Duration→Int/Float explícito, não adicionar. Focar nas operações básicas. O cast pode ser adicionado em passo futuro (XS) se necessário.

---

## 3. FASE A — L0 (redação; checkpoint obrigatório)

### A.1 — Prompt L0 `duration-stdlib.md`

Extensão de `00_nucleo/prompts/entities/duration.md` (ou novo `00_nucleo/prompts/engine/stdlib/duration-stdlib.md`):

- **Paridade**: `duration(days: 3, hours: 2)` ≡ vanilla constructor; `+`/`-`/`*`/`/`/`==`/`<` ≡ vanilla ops.
- **Substrato**: reusa `Duration { nanos: u64 }` P400; stdlib constructor + eval ops.
- **Sem tipo novo**: reusa `Value::Duration` P400.
- **Constructor**: `duration(days:?, hours:?, minutes:?, seconds:?, milliseconds:?, microseconds:?, nanoseconds:?)` — todos `Int`, ≥0, default 0.
- **Ops básicas**: `+` (Duration+Duration), `-` (Duration-Duration, ≥0), `*` (Duration*Int/Float), `/` (Duration/Int/Float, Duration/Duration→Float).
- **Comparações**: `==`, `!=`, `<`, `>`, `<=`, `>=` (Duration vs Duration).
- **Erros**: argumento negativo → Err; overflow u64 → Err; underflow `-` → Err; div by zero → Err.
- **Scope-out**: `.in(unit)`, `.display()`, `.hours()`, `.minutes()`, `.seconds()`, `.milliseconds()`, `.microseconds()`, `.nanoseconds()` — operações avançadas ADR-0054 graded.
- **Teste**: `duration(seconds: 90) + duration(seconds: 30)` → `duration(seconds: 120)`; `duration(minutes: 2) > duration(seconds: 119)` → true.

### A.2 — CHECKPOINT

Parar. Apresentar extensão `duration.md` (ou `duration-stdlib.md`) ao dono. **Só prosseguir para Fase B quando confirmar que guardou e computou hash.**

---

## 4. FASE B — Código (após confirmação humana)

### B.1 — `native_duration` stdlib

Em `rules/stdlib/foundations.rs` (ou módulo temporal/duration apropriado; confirmar path):

```rust
pub fn native_duration(args: &Args) -> SourceResult<Value> {
    let days = args.find::<Int>("days").unwrap_or(0).max(0) as u64;
    let hours = args.find::<Int>("hours").unwrap_or(0).max(0) as u64;
    let minutes = args.find::<Int>("minutes").unwrap_or(0).max(0) as u64;
    let seconds = args.find::<Int>("seconds").unwrap_or(0).max(0) as u64;
    let milliseconds = args.find::<Int>("milliseconds").unwrap_or(0).max(0) as u64;
    let microseconds = args.find::<Int>("microseconds").unwrap_or(0).max(0) as u64;
    let nanoseconds = args.find::<Int>("nanoseconds").unwrap_or(0).max(0) as u64;

    // Validar argumentos negativos (find retorna Int, mas se o usuário passar -1, max(0) clampa)
    // Decisão: clampar a 0 ou rejeitar? Vanilla rejeita negativo? Decisão: rejeitar com Err.
    // Reimplementar com validação explicita:
    let extract_nonneg = |name: &str| -> SourceResult<u64> {
        match args.find::<Int>(name) {
            Some(v) if v < 0 => bail!("{name} não pode ser negativo"),
            Some(v) => Ok(v as u64),
            None => Ok(0),
        }
    };

    let days = extract_nonneg("days")?;
    let hours = extract_nonneg("hours")?;
    let minutes = extract_nonneg("minutes")?;
    let seconds = extract_nonneg("seconds")?;
    let milliseconds = extract_nonneg("milliseconds")?;
    let microseconds = extract_nonneg("microseconds")?;
    let nanoseconds = extract_nonneg("nanoseconds")?;

    let total = days as u128 * DAY_NANOS
        + hours as u128 * HOUR_NANOS
        + minutes as u128 * MINUTE_NANOS
        + seconds as u128 * SECOND_NANOS
        + milliseconds as u128 * 1_000_000
        + microseconds as u128 * 1_000
        + nanoseconds as u128;

    if total > u64::MAX as u128 {
        bail!("duration excede o máximo suportado (~584 anos)");
    }

    Ok(Value::Duration(Duration::from_nanos(total as u64)))
}
```

**Constantes** (já existem em P400 `duration.rs`):
```rust
const SECOND_NANOS: u128 = 1_000_000_000;
const MINUTE_NANOS: u128 = 60 * SECOND_NANOS;
const HOUR_NANOS: u128 = 60 * MINUTE_NANOS;
const DAY_NANOS: u128 = 24 * HOUR_NANOS;
```

### B.2 — Operações eval

Em `eval/ops.rs` (ou onde as operações binárias são definidas):

```rust
// Adicionar arms para Value::Duration nas operações existentes

// Add
(Value::Duration(a), Value::Duration(b)) => {
    let sum = a.nanos as u128 + b.nanos as u128;
    if sum > u64::MAX as u128 {
        bail!("overflow em duration + duration");
    }
    Ok(Value::Duration(Duration::from_nanos(sum as u64)))
}

// Sub
(Value::Duration(a), Value::Duration(b)) => {
    if a.nanos < b.nanos {
        bail!("underflow em duration - duration (resultado negativo)");
    }
    Ok(Value::Duration(Duration::from_nanos(a.nanos - b.nanos)))
}

// Mul: Duration * Int
(Value::Duration(d), Value::Int(n)) | (Value::Int(n), Value::Duration(d)) => {
    if n < 0 {
        bail!("duration * int negativo não suportado");
    }
    let prod = d.nanos as u128 * n as u128;
    if prod > u64::MAX as u128 {
        bail!("overflow em duration * int");
    }
    Ok(Value::Duration(Duration::from_nanos(prod as u64)))
}

// Mul: Duration * Float
(Value::Duration(d), Value::Float(f)) | (Value::Float(f), Value::Duration(d)) => {
    if f < 0.0 {
        bail!("duration * float negativo não suportado");
    }
    let prod = (d.nanos as f64 * f) as u64;
    Ok(Value::Duration(Duration::from_nanos(prod)))
}

// Div: Duration / Int
(Value::Duration(d), Value::Int(n)) => {
    if n == 0 {
        bail!("divisão por zero");
    }
    if n < 0 {
        bail!("duration / int negativo não suportado");
    }
    Ok(Value::Duration(Duration::from_nanos(d.nanos / n as u64)))
}

// Div: Duration / Float
(Value::Duration(d), Value::Float(f)) => {
    if f == 0.0 {
        bail!("divisão por zero");
    }
    if f < 0.0 {
        bail!("duration / float negativo não suportado");
    }
    Ok(Value::Duration(Duration::from_nanos((d.nanos as f64 / f) as u64)))
}

// Div: Duration / Duration → Float
(Value::Duration(a), Value::Duration(b)) => {
    if b.nanos == 0 {
        bail!("divisão por zero");
    }
    Ok(Value::Float(a.nanos as f64 / b.nanos as f64))
}

// Comparisons: reusa PartialOrd/Ord de Duration (P400)
// Se ops.rs usa match em Value, adicionar arms:
(Value::Duration(a), Value::Duration(b)) => Ok(Value::Bool(a.cmp(b) == Ordering::Equal)),  // ==
(Value::Duration(a), Value::Duration(b)) => Ok(Value::Bool(a.cmp(b) != Ordering::Equal)),  // !=
(Value::Duration(a), Value::Duration(b)) => Ok(Value::Bool(a.cmp(b) == Ordering::Less)),    // <
(Value::Duration(a), Value::Duration(b)) => Ok(Value::Bool(a.cmp(b) == Ordering::Greater)), // >
// etc.
```

**Nota**: se `ops.rs` usa um trait genérico para operações (ex.: `ops::add(a, b)`), implementar o trait para `Value::Duration`. Se usa match explícito, adicionar arms.

### B.3 — Registar em `make_stdlib`

```rust
scope.define("duration", Func::native(native_duration));
```

### B.4 — Testes

1. **Unit stdlib** (8-10 tests em `stdlib/mod.rs` ou `foundations.rs`):
   - `native_duration_zero` — `duration()` → `Duration::ZERO`.
   - `native_duration_seconds` — `duration(seconds: 90)` → 90s.
   - `native_duration_mixed` — `duration(days: 1, hours: 2, minutes: 3)` → 1d2h3m.
   - `native_duration_negative` — `duration(seconds: -1)` → Err.
   - `native_duration_overflow` — `duration(nanoseconds: u64::MAX + 1)` → Err.
   - `native_duration_all_zeros` — `duration()` → 0.
   - `native_duration_nanos_only` — `duration(nanoseconds: 500)` → 500ns.

2. **Unit eval ops** (8-10 tests em `eval/ops.rs` ou `eval/tests.rs`):
   - `duration_add` — `90s + 30s` → `120s`.
   - `duration_add_overflow` — `max + 1ns` → Err.
   - `duration_sub` — `120s - 30s` → `90s`.
   - `duration_sub_underflow` — `30s - 120s` → Err.
   - `duration_mul_int` — `60s * 2` → `120s`.
   - `duration_mul_int_neg` — `60s * -1` → Err.
   - `duration_mul_float` — `60s * 1.5` → `90s`.
   - `duration_div_int` — `120s / 2` → `60s`.
   - `duration_div_int_zero` — `120s / 0` → Err.
   - `duration_div_float` — `120s / 2.0` → `60s`.
   - `duration_div_duration` — `120s / 60s` → `2.0` (Float).
   - `duration_eq` — `60s == 60s` → true.
   - `duration_lt` — `59s < 60s` → true.
   - `duration_gt` — `61s > 60s` → true.

3. **Integration / E2E** (2-3 tests):
   - `duration_pipeline` — parse + eval + `duration(seconds: 90) + duration(seconds: 30)` → `120s`.
   - `duration_comparison_pipeline` — `duration(minutes: 2) > duration(seconds: 119)` → true.
   - `duration_repr` — `repr(duration(seconds: 90))` → `"90s"` (ou formato canónico P400).

### B.5 — Linhagem

- `@prompt` aponta para extensão `duration.md` (ou `duration-stdlib.md`).
- `@prompt-hash` via `--fix-hashes`.
- Referência cruzada: P400 (Duration tipo modelado), P404 (Decimal aritmética — padrão paralelo), ADR-0107 (paridade linguagem), ADR-0054 (graded scope-out operações avançadas).

---

## 5. O que NÃO fazer (scope-out)

- **Não** implementar `.in(unit)` — scope-out ADR-0054 graded (converte Duration para Int/Float em unidade específica).
- **Não** implementar `.display()` — scope-out ADR-0054 graded (formatação localizada).
- **Não** implementar `.hours()`, `.minutes()`, `.seconds()`, etc. — scope-out ADR-0054 graded (extratores de componente).
- **Não** implementar `Duration → Int` cast — scope-out ADR-0054 graded (futuro XS).
- **Não** implementar `Duration → Float` cast — scope-out ADR-0054 graded (futuro XS).
- **Não** implementar `Int → Duration` cast melhorado (ex.: `90` → `duration(seconds: 90)`) — já existe P400 como nanos; scope-out se quiser semantic diferente.
- **Não** implementar `Duration` em `Paint`/`Fill`/`Style`/`Length` — não é visual.
- **Não** adicionar tipo novo — reusa P400.
- **Não** quebrar invariantes de camada (L1 puro, zero I/O).

---

## 6. Critérios de aceitação

1. `duration(seconds: 90)` compila e retorna `Value::Duration`.
2. `duration(seconds: 90) + duration(seconds: 30)` → `Value::Duration(120s)`.
3. `duration(minutes: 2) > duration(seconds: 119)` → `Value::Bool(true)`.
4. Overflow/underflow/div-by-zero rejeitados com `Err` claro.
5. Zero tipo novo; zero I/O; zero variant novo.
6. Testes verdes (≥18 unit + 2-3 integration); lint zero; hashes propagados.
7. Inventário 148: `duration()` stdlib transita `ausente` → `implementado`; `Value::Duration` permanece `implementado` (tipo já modelado P400).
8. L0 salvo e hashado antes do código (protocolo de nucleação).
9. Ritmo S-M: tempo de ciclo comparável a P404 (baseline de activação de tipo S modelado).

---

## 7. O que pode sair errado

- **`ops.rs` usa trait genérico para operações, não match explícito.** Mitigação: implementar o trait para `Value::Duration` (ex.: `Add`, `Sub`, etc.). Se o trait não suporta `Result` (só retorna `Value`), usar `panic` interno ou ajustar o trait. Preferir ajustar o trait para `SourceResult`.
- **Comparações em `ops.rs` usam `PartialEq`/`PartialOrd` de `Value`, não match explícito.** Mitigação: `Value::Duration` já tem `PartialEq`/`PartialOrd` (P400) — se `ops.rs` delega para estes traits, nenhuma mudança é necessária. Verificar se é o caso.
- **`Duration` não é `Copy` em `Value` (era `Copy` em P400?).** Mitigação: verificar P400. Se `Duration` é `Copy`, operações podem ser mais simples. Se não, usar clone. P400 definiu `Duration { nanos: u64 }` como `Copy` (u64 é Copy).
- **Testes de overflow precisam de `Duration::MAX` ou similar.** Mitigação: criar `Duration::MAX` constante se não existir, ou usar `u64::MAX` nanos diretamente.
- **Tentação de já implementar `.in()` ou `.display()`.** Mitigação: ADR-0054 graded; operações básicas são suficientes para S-M.
- **Tentação de implementar `Duration * Duration` ou `Duration + Int`.** Mitigação: não existe no vanilla; rejeitar com tipo inválido.

---

## 8. Referências

- P400 — `Value::Duration` tipo modelado (baseline).
- P404 — `Decimal` aritmética (padrão paralelo de activação de tipo S modelado).
- P395 — `Value::Tiling` (portão ADR-0017 aberto).
- ADR-0107 — paridade linguagem vs mecânica (constructor + ops básicas).
- ADR-0054 — graded scope-out (`.in()`, `.display()`, extractores).

---

## 9. Nota sobre o Tekt

Este passo é **activação de tipo S modelado** — paralelo ao P404 (Decimal). A série de activação: P404 Decimal → P405 Duration → P406 Version. Cada um reusa o tipo modelado no passo S puro anterior e adiciona constructor + ops básicas.

O ritmo deve ser uniforme: cada activação é S-M (não M), porque o tipo já existe e as operações são mecânicas (match arms + validação). Se P405 for significativamente mais lento que P404, investigar: `Duration` tem mais argumentos no constructor (7 vs 0 para Decimal) e mais operações (`/` com 3 variantes vs 2 para Decimal).

Registar o tempo de ciclo de P405 como **baseline de activação de tipo S modelado com constructor multi-arg** — comparar com P404 (Decimal, constructor zero-arg implícito) e P406 (Version, constructor multi-arg com prerelease/build).
