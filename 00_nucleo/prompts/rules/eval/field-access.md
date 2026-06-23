# Prompt L0 — `rules/eval` — Field Access em tipos primitivos
Hash do Código: 6316e1ef

**Camada**: L1
**Ficheiro alvo**: `01_core/src/rules/eval/bindings.rs` (`eval_field_access`)
**Origem**: Passos 411 e 412 (`typst-passo-411.md`, `typst-passo-412.md`) — materializar field access em `Value::Version` e `Value::Duration`.
**ADRs relevantes**: ADR-0107 (paridade linguagem), ADR-0033 (paridade vanilla), ADR-0108 (medir-antes-de-decidir).
**Prompt pai**: `00_nucleo/prompts/rules/eval.md`

---

## 1. Contexto

`eval_field_access` em `rules/eval/bindings.rs` faz match no `target` de um `Expr::FieldAccess`. Inicialmente suportava apenas `Value::Dict` e `Value::Content`. Este prompt consolida o field access para tipos primitivos L1: `Version` (P411) e `Duration` (P412).

---

## 2. Field Access `Version` (P411)

Campos suportados para `Value::Version(v)`:

| Campo | Tipo L1 | Retorno eval |
|---|---|---|
| `.major` | `u64` | `Value::Int` |
| `.minor` | `u64` | `Value::Int` |
| `.patch` | `u64` | `Value::Int` |
| `.pre` | `Vec<EcoString>` | `Value::Array(Str)` |
| `.build` | `Vec<EcoString>` | `Value::Array(Str)` |

```rust
Value::Version(v) => {
    match field.as_str() {
        "major" => Ok(Value::Int(v.major as i64)),
        "minor" => Ok(Value::Int(v.minor as i64)),
        "patch" => Ok(Value::Int(v.patch as i64)),
        "pre" => Ok(Value::Array(v.pre.iter().map(|s| Value::Str(s.clone())).collect())),
        "build" => Ok(Value::Array(v.build.iter().map(|s| Value::Str(s.clone())).collect())),
        _ => Err(... "campo desconhecido em version: {field}" ...),
    }
}
```

Campos `.pre` e `.build` vazios retornam `Value::Array([])`.

---

## 3. Field Access `Duration` (P412)

Campos suportados para `Value::Duration(d)`:

| Field | Semântica | Retorno eval |
|---|---|---|
| `.seconds` | Total de segundos (inclui fração) | `Value::Float` |
| `.minutes` | Total de minutos (inclui fração) | `Value::Float` |
| `.hours` | Total de horas (inclui fração) | `Value::Float` |
| `.days` | Total de dias (inclui fração) | `Value::Float` |

```rust
Value::Duration(d) => {
    const NANOS_PER_SECOND: f64 = 1_000_000_000.0;
    const NANOS_PER_MINUTE: f64 = 60_000_000_000.0;
    const NANOS_PER_HOUR: f64 = 3_600_000_000_000.0;
    const NANOS_PER_DAY: f64 = 86_400_000_000_000.0;
    match field.as_str() {
        "seconds" => Ok(Value::Float(d.nanos as f64 / NANOS_PER_SECOND)),
        "minutes" => Ok(Value::Float(d.nanos as f64 / NANOS_PER_MINUTE)),
        "hours"   => Ok(Value::Float(d.nanos as f64 / NANOS_PER_HOUR)),
        "days"    => Ok(Value::Float(d.nanos as f64 / NANOS_PER_DAY)),
        _ => Err(... "campo desconhecido em duration: {field}" ...),
    }
}
```

**Nota de forma**: o vanilla usa métodos (`.seconds()`); o cristalino usa fields (`.seconds`) por simplicidade de infra. Semântica idêntica.

---

## 4. Erros

- Campo desconhecido → erro eval com mensagem clara indicando o tipo (`version` ou `duration`).
- Field access em tipo não suportado → erro existente.

---

## 5. Scope-out

- Não adicionar `.raw` ou `.string` em Version.
- Não implementar field access mutável (set).
- Não adicionar `get_field` em `entities/version.rs` nem `entities/duration.rs`.
- Não implementar `.milliseconds` nem `.nanoseconds` em Duration.
- Não tocar em `Decimal`.

---

## 6. Critérios de Verificação

```
Dado version("1.2.3").major
Então Value::Int(1)

Dado version("1.2.3-alpha.1+build.2").pre
Então Value::Array([Str("alpha"), Str("1")])

Dado duration("1h30m").seconds
Então Value::Float(5400.0)

Dado duration("1h30m").hours
Então Value::Float(1.5)

Dado version("1.2.3").foo
Então erro eval

Dado duration("1h").foo
Então erro eval
```

---

## 7. Testes

- Version: `version_field_major`, `version_field_minor`, `version_field_patch`, `version_field_pre`, `version_field_pre_empty`, `version_field_build`, `version_field_build_empty`, `version_field_unknown`.
- Duration: `duration_field_seconds_zero`, `duration_field_seconds_simple`, `duration_field_seconds_compound`, `duration_field_seconds_fraction`, `duration_field_minutes`, `duration_field_minutes_compound`, `duration_field_hours`, `duration_field_days`, `duration_field_days_zero`, `duration_field_unknown`.
