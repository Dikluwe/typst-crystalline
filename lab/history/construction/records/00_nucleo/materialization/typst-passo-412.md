# Passo 412 — Field Access `Duration`: `.seconds`, `.minutes`, `.hours`, `.days` (S)

**Tipo**: Materialização (L1 eval; zero tipo novo; zero I/O).  
**Data**: 2026-06-22.  
**Padrão**: diagnóstico-primeiro; medir-antes-de-decidir (ADR-0108).  
**ADRs relevantes**: ADR-0107 (paridade linguagem), ADR-0033 (paridade vanilla).  
**Sonda fonte**: Análise P410 — `eval_field_access` em `rules/eval/bindings.rs` não trata `Value::Duration`; não há métodos nem testes de field access para Duration.

> **Nota de numeração.** Um passo só. Não numerar à frente.

---

## 1. Contexto

P400 modelou `Duration` em L1; P403 ativou o constructor `duration("1h30m")`; P405/P409 ativaram aritmética e comparações. O tipo participa em operações, mas **não tem field access** — `duration("1h30m").seconds` falha com "field access não suportado em duration".

No vanilla Typst, estas quantidades são expostas via **métodos** (`.seconds()`, `.minutes()`, `.hours()`, `.days()`), não fields. No cristalino, por simplicidade de infraestrutura e paridade de funcionalidade, implementamos como **field access** (`.seconds`, `.minutes`, `.hours`, `.days`) com a mesma semântica de retorno: valor total na unidade como `Float`.

| Field | Semântica | Tipo retornado |
|-------|-----------|----------------|
| `.seconds` | Total de segundos (inclui fração) | `Value::Float` |
| `.minutes` | Total de minutos (inclui fração) | `Value::Float` |
| `.hours` | Total de horas (inclui fração) | `Value::Float` |
| `.days` | Total de dias (inclui fração) | `Value::Float` |

Exemplo: `duration("1h30m").seconds` → `5400.0`; `.minutes` → `90.0`; `.hours` → `1.5`; `.days` → `0.0625`.

**Nota sobre paridade**: o vanilla usa métodos; o cristalino usa fields. A diferença é sintática (`.x` vs `.x()`), não semântica. O retorno é o mesmo. Documentar como dívida de forma (ADR-0107) se métodos forem exigidos no futuro.

---

## 2. Decisão de engenharia

Field access em L1 é lookup por nome de campo no eval de `Expr::FieldAccess`. No cristalino:

- `eval_field_access` (em `rules/eval/bindings.rs`) faz match no `target`.
- P411 adicionou `Value::Version`; este passo adiciona `Value::Duration`.
- A `Duration` L1 armazena nanos (`u64`). Os fields são **computados** a partir de nanos, não campos struct.

Conversões:
- `.seconds` → `nanos as f64 / 1_000_000_000.0`
- `.minutes` → `nanos as f64 / 60_000_000_000.0`
- `.hours` → `nanos as f64 / 3_600_000_000_000.0`
- `.days` → `nanos as f64 / 86_400_000_000_000.0`

Não tocar em `entities/duration.rs` — os métodos `as_seconds`/`as_minutes`/`as_hours`/`as_days` já existem (sonda P410 confirmou). Reutilizá-los se retornarem `f64`; se retornarem `u64` truncado, computar via nanos diretamente para preservar fração.

---

## 3. FASE A — L0 (redação; checkpoint obrigatório)

### A.0 — Sonda do substrato (obrigatória; 5 min)

Antes de redigir o L0, executar e documentar:

```bash
# 1. Confirmar que Value::Duration existe no enum
grep -n "Value::Duration" entities/value.rs
# Esperado: 1+ linha confirmando variant

# 2. Confirmar que Duration tipo L1 tem acesso ao valor interno (nanos)
grep -n "pub struct Duration\|pub nanos\|pub fn as_seconds\|pub fn as_minutes\|pub fn as_hours\|pub fn as_days" entities/duration.rs
# Esperado: struct com campo nanos ou métodos de conversão

# 3. Verificar eval_field_access atual (post-P411)
grep -n "eval_field_access" rules/eval/bindings.rs
# Esperado: função existe; match em Dict, Content, Version

# 4. Verificar que Value::Duration ainda NÃO está em eval_field_access
grep -A 30 "fn eval_field_access" rules/eval/bindings.rs | grep -i "duration"
# Esperado: zero hits (confirma ausência)

# 5. Verificar que Value::Float existe para retorno
grep -n "Value::Float" entities/value.rs | head -3
# Esperado: variant Float existe
```

**Critério de passagem**: 
- (1) `Value::Duration` existe.
- (2) `Duration` tipo L1 acessível (nanos ou métodos de conversão).
- (3) `eval_field_access` existe e tem padrão extensível (Dict/Content/Version).
- (4) `Duration` ainda ausente em `eval_field_access`.
- (5) `Value::Float` existe para retorno.

Se (3) falhar (não há `eval_field_access`), **parar imediatamente**. Se (2) falhar (não há acesso ao valor), reclassificar para M (refatoração de `Duration`).

Documentar resultado no commit: `Sonda P412: Duration variant OK; nanos/conversão OK; eval_field_access extensível (Dict/Content/Version); Duration ausente; Float variant OK.`

### A.1 — Prompt L0 `duration-field-access.md`

Novo em `00_nucleo/prompts/engine/eval/duration-field-access.md`:

- **Paridade funcional**: `duration("1h30m").seconds` → `5400.0` (Float); `.minutes` → `90.0`; `.hours` → `1.5`; `.days` → `0.0625`.
- **Nota de forma**: o vanilla usa métodos (`.seconds()`); o cristalino usa fields (`.seconds`) por simplicidade de infra. Semântica idêntica.
- **Substrato**: ramo `Value::Duration(d)` em `eval_field_access` (`rules/eval/bindings.rs`).
- **Sem tipo novo**: reutiliza `Value::Duration`, `Value::Float`.
- **Campos**: 4 fields fixos (`seconds`, `minutes`, `hours`, `days`); campo desconhecido → erro eval.
- **Conversão**: nanos `u64` → `f64` via divisão por constante de unidade; preserva fração.
- **Testes**: 
  - 4 fields × 3 casos (zero, valor simples, valor composto com fração) = 12.
  - Campo desconhecido: 1.
  - Total mínimo: 13 testes.

### A.2 — CHECKPOINT

Parar. Apresentar `duration-field-access.md` ao dono. **Só prosseguir para Fase B quando confirmar que guardou e computou hash.**

---

## 4. FASE B — Código (após confirmação humana)

1. **Localizar `eval_field_access`** em `rules/eval/bindings.rs`.
2. **Adicionar ramo `Value::Duration(d)`** ao match de `target`:
   ```rust
   Value::Duration(d) => {
       match field.as_str() {
           "seconds" => Ok(Value::Float(d.nanos as f64 / 1_000_000_000.0)),
           "minutes" => Ok(Value::Float(d.nanos as f64 / 60_000_000_000.0)),
           "hours" => Ok(Value::Float(d.nanos as f64 / 3_600_000_000_000.0)),
           "days" => Ok(Value::Float(d.nanos as f64 / 86_400_000_000_000.0)),
           _ => Err(... "campo desconhecido em duration: {}" ...),
       }
   }
   ```
   *(Se `Duration` tiver métodos `as_seconds_f64` etc., usá-los; senão, computar via nanos.)*
3. **Testes** em `rules/eval/tests.rs` (bloco `P412 — Field Access Duration`):
   - `duration_field_seconds_zero` — `duration("0s").seconds` → `Value::Float(0.0)`.
   - `duration_field_seconds_simple` — `duration("5s").seconds` → `Value::Float(5.0)`.
   - `duration_field_seconds_compound` — `duration("1h30m").seconds` → `Value::Float(5400.0)`.
   - `duration_field_seconds_fraction` — `duration("1.5s").seconds` → `Value::Float(1.5)`.
   - `duration_field_minutes` — `duration("90m").minutes` → `Value::Float(90.0)`.
   - `duration_field_minutes_fraction` — `duration("1h30m").minutes` → `Value::Float(90.0)`.
   - `duration_field_hours` — `duration("1h30m").hours` → `Value::Float(1.5)`.
   - `duration_field_days` — `duration("36h").days` → `Value::Float(1.5)`.
   - `duration_field_days_zero` — `duration("0s").days` → `Value::Float(0.0)`.
   - `duration_field_unknown` — `duration("1h").foo` → erro eval.
4. **Linhagem**: `@prompt` aponta para `duration-field-access.md`; `@prompt-hash` via `--fix-hashes`.
5. **Validação**:
   - `cargo test --workspace -- --skip p350c_flag_on_nao_convergente_classifica` — verde.
   - `crystalline-lint .` — zero violations novas.
   - `git diff --stat` dos `.rs`: apenas `bindings.rs` + testes.

---

## 5. O que NÃO fazer (scope-out)

- **Não** implementar `.milliseconds` ou `.nanoseconds` — não existem no vanilla; scope-out futuro se solicitado.
- **Não** implementar field access mutável (set) — o vanilla não tem; read-only.
- **Não** adicionar método `get_field` em `entities/duration.rs` — lookup é no eval.
- **Não** tocar em `entities/duration.rs` — reutilizar métodos existentes ou computar via nanos.
- **Não** tocar em `Version` ou `Decimal` — um passo de cada vez.
- **Não** criar funções stdlib (`duration_seconds`, etc.) — field access é via `.`, não call.

---

## 6. Critérios de aceitação

1. `duration("1h30m").seconds` → `Value::Float(5400.0)`.
2. `duration("1h30m").minutes` → `Value::Float(90.0)`.
3. `duration("1h30m").hours` → `Value::Float(1.5)`.
4. `duration("36h").days` → `Value::Float(1.5)`.
5. `duration("0s").seconds` → `Value::Float(0.0)` (zero correto).
6. Fração preservada: `duration("1.5s").seconds` → `Value::Float(1.5)`.
7. Campo desconhecido (`.foo`) → erro eval com mensagem clara.
8. Zero tipo novo; zero variant novo; zero I/O.
9. Testes verdes (≥ 13 novos); lint zero; hashes propagados.
10. L0 salvo e hashado antes do código; sonda A.0 documentada no commit.

---

## 7. O que pode sair errado

- **`Duration` não expõe `nanos` como campo público.** Mitigação: sonda A.0 detecta; se `nanos` é privado, usar métodos `as_seconds`/`as_minutes`/etc. se retornarem `f64`; se retornarem `u64` truncado, adicionar métodos `as_seconds_f64` em `Duration` (scope-out documentado).
- **Fração perdida em conversão `u64 → f64`.** Mitigação: `u64 as f64` tem precisão suficiente para nanos até ~2^53 (~104 dias). Para durações maiores, documentar limitação. O vanilla usa `f64` para duration fields.
- **Tentação de implementar como métodos (`.seconds()`).** Mitigação: scope-out de forma; fields são funcionalmente equivalentes e mais simples de implementar no eval atual. Se métodos forem exigidos, é passo separado de infra de método em tipos primitivos.
- **`eval_field_access` cresce muito com 4+ tipos.** Mitigação: se o match ficar grande, considerar refatorar para dispatch table em passo futuro — não neste passo.

---

## 8. Referências

- `entities/duration.rs` (P400) — `Duration` tipo L1 com nanos e métodos de conversão.
- `entities/value.rs` — variants `Value::Duration`, `Value::Float`.
- `rules/eval/bindings.rs` — `eval_field_access` (infra a sondar em A.0).
- P410 — análise de sonda que confirmou ausência de field access para Duration.
- P411 — field access Version (padrão de implementação em `eval_field_access`).
- ADR-0107 — paridade linguagem (funcionalidade idêntica, forma pode diferir).
- ADR-0108 — medir-antes-de-decidir (sonda A.0).

---

## 9. Nota sobre o Tekt

Este passo é **S** porque o tipo já existe, o valor é acessível, e o field access é apenas mais um ramo em `eval_field_access` (padrão estabelecido em P411). O risco principal é acessar `nanos` — se for privado, o passo cresce para M (adição de getters em `Duration`). A sonda A.0 detecta isso antes do código.

Se P411 ainda não fechou (Version field access), este passo pode ser **bloqueado** — a infra de `eval_field_access` pode não ter o padrão extensível ainda. Verificar estado do P411 antes de iniciar A.0.
