# Passo 412 — Relatório de execução: Field Access `Duration`

**Tipo**: Materialização (L1 eval).  
**Data**: 2026-06-22.  
**Status**: concluído.

---

## 1. Sonda do substrato (FASE A.0)

Executados os comandos de sonda definidos em `typst-passo-412.md`:

- `Value::Duration` existe em `entities/value.rs` ✅
- `Duration` tipo L1 tem campo `nanos: u64` público e métodos `as_seconds`/`as_minutes`/`as_hours`/`as_days` ✅
- `eval_field_access` existe em `rules/eval/bindings.rs` (após P411, com ramo `Value::Version`) ✅
- `Value::Duration` ainda não estava em `eval_field_access` ✅
- `Value::Float` existe para retorno ✅

**Resultado da sonda**: critérios todos passados; passo é S viável.

---

## 2. Prompt L0 (FASE A)

- Prompt inicial criado em `00_nucleo/prompts/rules/eval/duration-field-access.md`.
- Durante a fase de lint, consolidou-se o prompt em `00_nucleo/prompts/rules/eval/field-access.md` (cobrindo Version P411 + Duration P412) para evitar warnings de prompt órfão no linter quando há múltiplos `@prompt` no mesmo arquivo.
- O prompt consolidado descreve field access para `Value::Version` e `Value::Duration`.

---

## 3. Implementação (FASE B)

Arquivo alterado: `01_core/src/rules/eval/bindings.rs`.

Adicionado ramo `Value::Duration(d)` em `eval_field_access`:

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
        _ => Err(vec![SourceDiagnostic::error(
            access.span(),
            format!("campo desconhecido em duration: '{}'", field),
        )]),
    }
}
```

- Retorno `Value::Float` para todos os campos.
- Fração preservada (ex.: `duration("1.5s").seconds` → `1.5`).
- Zero preservado corretamente.
- Campo desconhecido retorna erro eval.

Também foi adicionado `@prompt` para `field-access.md` no header de `bindings.rs`.

---

## 4. Testes

Adicionados 10 testes em `01_core/src/rules/eval/tests.rs` (bloco `P412 — Field Access Duration`):

- `duration_field_seconds_zero` → `Value::Float(0.0)`
- `duration_field_seconds_simple` → `Value::Float(5.0)`
- `duration_field_seconds_compound` → `Value::Float(5400.0)`
- `duration_field_seconds_fraction` → `Value::Float(1.5)`
- `duration_field_minutes` → `Value::Float(90.0)`
- `duration_field_minutes_compound` → `Value::Float(90.0)`
- `duration_field_hours` → `Value::Float(1.5)`
- `duration_field_days` → `Value::Float(1.5)`
- `duration_field_days_zero` → `Value::Float(0.0)`
- `duration_field_unknown` → erro eval

---

## 5. Validações

- `cargo test -p typst-core duration_field` — 10 testes passaram; 0 falhas.
- `cargo test --workspace -- --skip p350c_flag_on_nao_convergente_classifica` — verde.
- `crystalline-lint` — 0 erros / 0 drift; resta apenas warning pré-existente de `show-regex.md` órfão.
- Hashes propagados manualmente: `eval.md @prompt-hash 7a92cc2d`, `field-access.md @prompt-hash 6316e1ef`.

---

## 6. Notas

- O vanilla usa métodos (`.seconds()`); o cristalino usa fields (`.seconds`) por simplicidade de infra. A semântica é idêntica.
- Não houve alteração em `entities/duration.rs` — o campo `nanos` já era público.
- Prompt L0 consolidado com o P411 para evitar fragmentação e warnings do linter.

---

## 7. Conclusão

Field access para `Value::Duration` implementado e testado. Nenhum tipo novo, nenhuma mudança em `entities/duration.rs`, nenhum I/O. O passo cumpre todos os critérios de aceitação.
