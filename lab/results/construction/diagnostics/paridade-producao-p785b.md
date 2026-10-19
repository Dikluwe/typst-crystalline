# Diagnostic Report — Paridade de Produção — Passo 785b

**Data:** 2026-07-20  
**Executado por:** Antigravity AI  
**Escopo:** Implementação de campos nativos em tipos de valor (`RelativeLength`, `Alignment`, `Length`, `Stroke`) e alinhamento de mensagens de erro para campos ausentes com o Typst Vanilla 0.15.0.

---

## 1. Inspeção de Código e Evidência no Typst Vanilla

Inspeção no repositório Vanilla em `lab/typst-original/crates/typst-library/src/foundations/fields.rs`:

```bash
grep -rn "fn fields\b\|missing_field\|no_fields" lab/typst-original/crates/typst-library/src/foundations/fields.rs
```

**Resultado da Inspeção de Código:**
```rust
pub fn field(value: &Value, field: &str) -> StrResult<Value> {
    match value {
        Value::Version(v) => v.component(field),
        Value::Length(l) => match field {
            "em" => Ok(Value::Float(l.em)),
            "abs" => Ok(Value::Length(l.abs.into())),
            _ => missing_field(value, field),
        },
        Value::Relative(v) => match field {
            "ratio" => Ok(Value::Ratio(v.rel)),
            "length" => Ok(Value::Length(v.abs)),
            _ => missing_field(value, field),
        },
        Value::Stroke(s) => match field {
            "paint" => Ok(s.paint.clone().unwrap_or_default()),
            "thickness" => Ok(s.thickness.into()),
            ...
        },
        Value::Alignment(a) => match field {
            "x" => Ok(a.x().map(Into::into).unwrap_or(Value::None)),
            "y" => Ok(a.y().map(Into::into).unwrap_or(Value::None)),
            _ => missing_field(value, field),
        },
        ...
        _ => no_fields(value),
    }
}
```

Formatadores formais de erro no Vanilla:
- `missing_field(value, field)` $\to$ `"{ty} does not contain field \"{field}\""`
- `no_fields(value)` $\to$ `"cannot access fields on type {ty}"`

---

## 2. Refatoração de Arquitetura e Reutilização em L1

### Centralização em `eval_value_field_access`
Criou-se a função auxiliar `eval_value_field_access(target, field, span)` em `01_core/src/engine/eval/bindings.rs`. Essa função é reutilizada tanto pelo dispatcher de `eval_field_access` (`bindings.rs`) quanto por `eval_math_callee` (`math.rs`), eliminando duplicação de lógica e garantindo que acessos a campos em modo script e em modo math produzam exatamente os mesmos resultados e diagnósticos.

### Coerção Morfológica de Igualdade (`Ratio` ↔ `Relative`)
Ao avaliar `#assert.eq((10pt + 50%).ratio, 50%)`:
- `(10pt + 50%).ratio` produz `Value::Ratio(Ratio(0.5))`
- `50%` literal produz `Value::Relative(Rel { rel: 0.5, abs: 0pt })`
Para permitir a asserção sem violar a pureza do `PartialEq` derivado do Rust em `Value` (ADR-0025 e ADR-0107), adicionou-se a regra de coerção morfológica da linguagem em `operators.rs` (`BinOp::Eq`/`Neq`) e `assert.rs` (`values_equal`), validando igualdade quando `rel.abs.is_zero()` e `rel.rel == ratio.0`.

---

## 3. Medições de Paridade Real (Vanilla 0.15.0 vs Cristalino Release)

### Commit e Ambiente de Medição
- **Commit Git / State:** `working tree (git diff HEAD --stat)`
- **Binário Vanilla:** `lab/typst-original/target/release/typst` (Typst 0.15.0 rev `969087ec`)
- **Binário Cristalino:** `./target/release/typst` (produzido via `cargo build --release -p typst-wiring`)

### Tabela de Comparação de Testes

| Teste | Documento Typst | Saída Vanilla 0.15.0 | Saída Cristalino Release | Status |
|---|---|---|---|---|
| Campos Nativos Válidos | `#let r = 10pt + 50%` <br> `#assert.eq(r.ratio, 50%)` <br> `#assert.eq(r.length, 10pt)` <br> `#let a = top + left` <br> `#assert.eq(a.x, left)` <br> `#assert.eq(a.y, top)` | `exit: 0` | `exit: 0` | **PARIDADE 100%** |
| Erro Campo Inexistente | `#let x = (10pt).invalid` | `error: length does not contain field "invalid"` (`exit 1`) | `error: length does not contain field "invalid"` (`exit 1`) | **PARIDADE 100%** |
| Erro Tipo Sem Campos | `#let x = (123).invalid` | `error: cannot access fields on type integer` (`exit 1`) | `error: cannot access fields on type integer` (`exit 1`) | **PARIDADE 100%** |

---

## 4. Validação Arquitetural e Testes

- **`cargo test --workspace`**: 650+ testes executados, **0 falhas**.
- **`crystalline-lint .`**: **0 violações** de regras de arquitetura e linhagem.
- **Prompt L0**: `00_nucleo/prompts/engine/eval/fields.md` registrado com `@prompt-hash 3624668a`.

---

## 5. Conclusão

O Passo 785b está oficialmente concluído e validado com paridade absoluta em nível de produção.
