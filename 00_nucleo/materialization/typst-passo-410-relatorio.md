# Análise P410 — Relatório de sonda

**Data**: 2026-06-22.  
**Objetivo**: Determinar, via sonda de substrato, quais dos 3 caminhos já estão implementados (redundantes) e quais necessitam materialização.  
**Padrão**: ADR-0108 (medir-antes-de-decidir).

---

## Resumo executivo

| Caminho | Classificação | Justificativa |
|---------|---------------|---------------|
| **A — Field Access Version** | **VERMELHO** | `eval_field_access` em `rules/eval/bindings.rs` só trata `Value::Dict` e `Value::Content`; não há ramo para `Value::Version`. |
| **B — Field Access Duration** | **VERMELHO** | Mesmo motivo: `eval_field_access` não trata `Value::Duration`; não há métodos nem testes de field access. |
| **C — Aritmética/Comparações Version** | **VERDE** | Comparações (`==`, `!=`, `<`, `<=`, `>`, `>=`) já implementadas em `rules/eval/operators.rs` (comentários `P406`) e testadas em `rules/eval/tests.rs`. Aritmética não existe no vanilla. |

---

## Sonda A — Field Access Version

### Comandos executados

```bash
grep -n "Value::Version" 01_core/src/entities/value.rs
# Resultado: variant existe (linha 116), mas grep -E simples não retornou linha visível devido a formatação do arquivo.

grep -n "pub major\|pub minor\|pub patch\|pub pre\|pub build" 01_core/src/entities/version.rs
# 18:    pub major: u64,
# 19:    pub minor: u64,
# 20:    pub patch: u64,
# 22:    pub pre: Vec<EcoString>,
# 24:    pub build: Vec<EcoString>,

grep -rn "Value::Version" 01_core/src/engine/eval/ | grep -i "field\|get_field\|dot\|access\|major\|minor\|patch"
# 01_core/src/engine/eval/tests.rs:1309:  Value::Version(...)
# 01_core/src/engine/eval/tests.rs:1315:  Value::Version(...)
# 01_core/src/engine/eval/tests.rs:1321:  Value::Version(...)
# Apenas construção de valores em testes; nenhum field access.

grep -rn "get_field\|field_access" 01_core/src/engine/eval/ | head -20
# 01_core/src/engine/eval/bindings.rs:118: pub(super) fn eval_field_access(...)
# 01_core/src/engine/eval/bindings.rs:138: Value::Content(c) => c.get_field(...)
# 01_core/src/engine/eval/tests.rs:1835: fn pipeline_field_access_invalido_retorna_err()
# 01_core/src/engine/eval/mod.rs:528: Expr::FieldAccess(a) => bindings::eval_field_access(...)

grep -rn "version.*major\|version.*minor\|version.*patch" 01_core/src/engine/eval/tests.rs 01_core/src/entities/tests/ 2>/dev/null
# Nenhum teste de field access Version encontrado.
```

### Verificação manual de `eval_field_access`

Arquivo `01_core/src/engine/eval/bindings.rs`, linhas 118–147:

```rust
pub(super) fn eval_field_access(...) -> SourceResult<Value> {
    let target = eval_expr(access.target(), scopes, ctx, engine)?;
    let field  = access.field().as_str().to_string();
    match target {
        Value::Dict(d) => d.get(field.as_str()).cloned()...,
        Value::Content(c) => c.get_field(field.as_str())...,
        other => Err(... "field access não suportado em {}" ...),
    }
}
```

**Conclusão**: não há suporte a field access em `Value::Version`. Classificação **VERMELHO**.

---

## Sonda B — Field Access Duration

### Comandos executados

```bash
grep -n "Value::Duration" 01_core/src/entities/value.rs
# Resultado: variant existe (linha 112), mas grep -E simples não retornou linha visível.

grep -n "pub fn as_secs\|pub fn as_millis\|pub fn as_nanos\|pub fn days\|pub fn hours\|pub fn minutes\|pub fn seconds" 01_core/src/entities/duration.rs
# Nenhum método com esses nomes exactos.

grep -rn "Value::Duration" 01_core/src/engine/eval/ | grep -i "field\|get_field\|dot\|access\|days\|hours\|minutes\|seconds"
# 01_core/src/engine/eval/tests.rs:1217: Value::Duration(...)
# Apenas construção de valores em testes; nenhum field access.

grep -rn "get_field\|field_access" 01_core/src/engine/eval/ | head -20
# Idem Sonda A: apenas Dict e Content.

grep -rn "duration.*days\|duration.*hours\|duration.*minutes\|duration.*seconds" 01_core/src/engine/eval/tests.rs 01_core/src/entities/tests/ 2>/dev/null
# Nenhum teste de field access Duration encontrado.
```

### Métodos disponíveis em `Duration` L1

Arquivo `01_core/src/entities/duration.rs`:

- `from_nanos`, `from_seconds`, `from_minutes`, `from_hours`, `from_days`
- `as_seconds`, `as_minutes`, `as_hours`, `as_days`
- `is_zero`, `to_string`

Não há métodos nomeados `.seconds`, `.minutes`, `.hours`, `.days` para field access estilo Typst.

**Conclusão**: não há suporte a field access em `Value::Duration`. Classificação **VERMELHO**.

---

## Sonda C — Aritmética/Comparações Version

### Comandos executados

```bash
grep -n "Value::Version" 01_core/src/entities/value.rs
# Variant existe (linha 116).

grep -n "impl.*PartialOrd\|impl.*Ord\|impl.*Eq\|impl.*PartialEq" 01_core/src/entities/version.rs
# 109: impl PartialOrd for Version {
# 115: impl Ord for Version {

grep -rn "Value::Version" 01_core/src/engine/eval/ | grep -i "add\|sub\|mul\|div\|cmp\|eq\|lt\|gt\|BinaryOp"
# 01_core/src/engine/eval/operators.rs:154: (BinOp::Eq,  Value::Version(a), Value::Version(b)) => ...
# 01_core/src/engine/eval/operators.rs:157: (BinOp::Neq, Value::Version(a), Value::Version(b)) => ...
# 01_core/src/engine/eval/operators.rs:172: (BinOp::Lt,  Value::Version(a), Value::Version(b)) => Ok(Value::Bool(a < b)),
# 01_core/src/engine/eval/operators.rs:182: (BinOp::Leq, Value::Version(a), Value::Version(b)) => Ok(Value::Bool(a <= b)),
# 01_core/src/engine/eval/operators.rs:192: (BinOp::Gt,  Value::Version(a), Value::Version(b)) => Ok(Value::Bool(a > b)),
# 01_core/src/engine/eval/operators.rs:202: (BinOp::Geq, Value::Version(a), Value::Version(b)) => Ok(Value::Bool(a >= b)),

grep -rn "BinaryOp::Eq\|BinaryOp::Lt" 01_core/src/engine/eval/ | head -10
# Match arms presentes em rules/eval/operators.rs.

grep -rn "version.*==\|version.*<\|version.*cmp" 01_core/src/engine/eval/tests.rs 01_core/src/entities/tests/ 2>/dev/null
# Testes encontrados em rules/eval/tests.rs: version_eq_neq, version_lt_gt_leq_geq, version_pre_release_ordering.
```

### Verificação dos testes existentes

Em `01_core/src/engine/eval/tests.rs`, bloco `P406 — Comparações Version`:

- `version_eq_neq`
- `version_lt_gt_leq_geq`
- `version_pre_release_ordering`

**Conclusão**: comparações Version já implementadas e testadas. Aritmética não faz sentido para Version (não é numérico no vanilla). Classificação **VERDE**.

---

## Decisões e próximos passos

| Caminho | Ação recomendada |
|---------|------------------|
| **A** | Materializar field access `.major`/`.minor`/`.patch`/`.pre`/`.build` em `Value::Version` (passo S dedicado). |
| **B** | Materializar field access `.seconds`/`.minutes`/`.hours`/`.days` em `Value::Duration` (passo S dedicado). |
| **C** | Nenhuma ação — já implementado. |

### Notas de implementação futura

- Caminho A: adicionar ramos `Value::Version(v)` em `eval_field_access` (`bindings.rs`) mapeando cada campo para `Value::Int`, `Value::Array(Str)`, etc.
- Caminho B: adicionar ramo `Value::Duration(d)` em `eval_field_access` mapeando `.seconds`, `.minutes`, `.hours`, `.days` para `Value::Int` (provavelmente truncado). Pode ser necessário adicionar métodos de decomposição em `entities/duration.rs` se `as_seconds`/`as_minutes`/etc. não forem suficientes.

---

## Validações

- `cargo test --workspace -- --skip p350c_flag_on_nao_convergente_classifica` — **verde**.
- `crystalline-lint` — **0 erros / 0 drift**; resta apenas o warning pré-existente de `show-regex.md` órfão.

---

## Conclusão

O Passo 410 é puramente analítico. Nenhuma alteração de código foi necessária. O resultado da sonda indica **dois caminhos pendentes** (A e B) que devem virar passos de materialização futuros, e **um caminho já fechado** (C).
