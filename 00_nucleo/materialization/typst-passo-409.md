# Passo 409 — Aritmética `Duration`: `+`, `-`, `*`, `/` e comparações (S/M)

> **Reclassificação retroativa:** a sonda A.0 confirmou que o trabalho descrito já existia (P405). Esta spec passa a documento de verificação retroativa, não de materialização. Nenhum código novo foi produzido por este passo além do que a verificação exigiu.

**Tipo**: Materialização (L1 eval + stdlib; zero tipo novo; zero I/O).  
**Data**: 2026-06-22.  
**Padrão**: diagnóstico-primeiro; medir-antes-de-decidir (ADR-0108).  
**ADRs relevantes**: ADR-0107 (paridade linguagem), ADR-0033 (paridade vanilla).  
**Sonda fonte**: Inventário 148 — `Duration` constructor ativado em P403; operadores aritméticos e comparações ainda ausentes para `Value::Duration`.

> **Nota de numeração.** Um passo só. Não numerar à frente.

---

## 1. Contexto

P400 modelou `Duration` em L1; P403 ativou o constructor `duration("1h30m")`. O tipo existe, mas **não participa em operações aritméticas nem comparações**. Este passo ativa `+`, `-`, `*`, `/` e os operadores de comparação (`==`, `!=`, `<`, `<=`, `>`, `>=`) para `Value::Duration`, consumindo o tipo que agora vive na stdlib.

| Operação | Vanilla Typst | Paridade cristalino |
|----------|---------------|---------------------|
| `+` | `duration + duration` | `Value::Duration + Value::Duration` |
| `-` | `duration - duration` | `Value::Duration - Value::Duration` |
| `*` | `duration * int`, `int * duration` | `Value::Duration * Value::Int` e `Value::Int * Value::Duration` |
| `/` | `duration / int`, `duration / duration` | `Value::Duration / Value::Int` → `Value::Duration`; `Value::Duration / Value::Duration` → `Value::Float` |
| `==`, `!=` | `duration == duration` | bit-equivalente via nanos |
| `<`, `<=`, `>`, `>=` | `duration < duration` | ordenação total via nanos |

**Nota sobre coerção**: `Duration * Float` e `Float * Duration` existem no vanilla, mas são scope-out deste passo se a sonda A.0 revelar que o eval ainda não tem infra para dispatch binário com 3+ ramos de tipo. O foco é `Int` como escalar e `Duration` como operando homogéneo.

---

## 2. Decisão de engenharia

A aritmética `Duration` é **parcialmente heterogénea**: `*` e `/` aceitam `Int` como co-operando. No cristalino:

- `+`, `-`: ambos os operandos `Value::Duration` → resultado `Value::Duration`.
- `*`: `Value::Duration * Value::Int` ou `Value::Int * Value::Duration` → `Value::Duration`.
- `/`: `Value::Duration / Value::Int` → `Value::Duration` (divisão por zero → erro); `Value::Duration / Value::Duration` → `Value::Float` (ratio).
- Comparações: `Value::Duration` vs `Value::Duration` → `Value::Bool`.

O eval de expressões binárias (`BinaryExpr`) faz dispatch por tipo dos operandos. Se P404 (Decimal) já adicionou o padrão de "ramo Decimal ao match arm", Duration segue o mesmo padrão — mas com a complexidade adicional de `*` e `/` com `Int`.

Se o eval não tiver infraestrutura de operadores binários centralizada (ex.: cada op é um `native_` separado), a sonda A.0 descobrirá e o plano ajusta.

---

## 3. FASE A — L0 (redação; checkpoint obrigatório)

### A.0 — Sonda do substrato (obrigatória; 5 min)

Antes de redigir o L0, executar e documentar:

```bash
# 1. Confirmar que Value::Duration existe no enum
grep -n "Value::Duration" entities/value.rs
# Esperado: 1+ linha confirmando variant

# 2. Confirmar que Duration tipo L1 existe e tem acesso ao valor interno (nanos)
grep -n "pub struct Duration\|pub fn as_nanos\|pub fn from_nanos\|inner\|std::time::Duration" entities/duration.rs
# Esperado: struct com campo acessível ou wrapper com método de extração

# 3. Confirmar infra de eval binário existente para Int/Float/Decimal
grep -rn "BinaryOp::Add\|BinaryOp::Sub\|BinaryOp::Mul\|BinaryOp::Div\|BinaryOp::Eq\|BinaryOp::Lt" rules/eval/
# Esperado: match arms ou dispatch centralizado que trata Int/Float (e Decimal se P404 fechou)

# 4. Confirmar que Duration ainda NÃO está nos operadores
grep -rn "Value::Duration" rules/eval/ | grep -i "add\|sub\|mul\|div\|cmp\|eq\|lt\|gt"
# Esperado: zero hits (confirma ausência no dispatch)

# 5. Confirmar que a crate de Duration suporta as ops necessárias
# Se for std::time::Duration: verificar se Add, Sub, Mul<u32>, Div<u32> são derivados
grep -n "std::time::Duration\|chrono::Duration\|Duration" entities/duration.rs | head -10
# Esperado: tipo base que suporta Add, Sub, Mul, Div
```

**Critério de passagem**: 
- (1) `Value::Duration` existe.
- (2) `Duration` tipo L1 acessível (nanos extraíveis).
- (3) Infra de eval binário existe para pelo menos um tipo (Int ou Float).
- (4) Duration ainda ausente nos operadores.
- (5) Tipo base de Duration suporta Add/Sub/Mul/Div.

Se (3) falhar (não há infra de eval binário), **parar imediatamente** — o passo é maior que S/M e requer infraestrutura de operadores primeiro. Se (1) ou (5) falhar, reclassificar.

Documentar resultado no commit: `Sonda P409: Duration variant OK; eval binário infra OK (Int/Float/Decimal); Duration ausente em ops; tipo base Add/Sub/Mul/Div OK.`

### A.1 — Prompt L0 `duration-arithmetic.md`

Novo em `00_nucleo/prompts/rules/eval/duration-arithmetic.md`:

- **Paridade**: `duration("1h") + duration("30m")` → `duration("1h30m")`; `duration("1h") * 2` → `duration("2h")`; `duration("1h") / duration("30m")` → `2.0` (Float).
- **Substrato**: ramos adicionais no dispatch binário de `BinaryExpr` eval para `Value::Duration`.
- **Sem tipo novo**: reutiliza `Value::Duration` existente.
- **Operadores homogéneos**: `+`, `-`, `==`, `!=`, `<`, `<=`, `>`, `>=` — ambos operandos `Duration`.
- **Operadores heterogéneos (controlados)**: `*` com `Int` (ambas as ordens); `/` com `Int` (divisor) e `/` com `Duration` (ratio → Float).
- **Divisão por zero**: `duration / 0` → erro eval.
- **Coerção cruzada**: não implementar `Duration * Float` nem `Duration + Int` neste passo; misturas não cobertas → erro de tipo existente.
- **Testes**: 
  - Aritmética homogénea: `+`, `-` × 3 casos = 6.
  - Aritmética heterogénea: `*` (2 ordens) × 3 casos + `/ int` × 3 casos + `/ duration` × 3 casos = 15.
  - Comparações: 6 ops × 3 casos = 18.
  - Divisão por zero: 1.
  - Tipo errado (Duration + Int): 1.
  - Total mínimo: ~25 testes.

### A.2 — CHECKPOINT

Parar. Apresentar `duration-arithmetic.md` ao dono. **Só prosseguir para Fase B quando confirmar que guardou e computou hash.**

---

## 4. FASE B — Código (após confirmação humana)

1. **Localizar o dispatch binário** em `rules/eval/` (provavelmente `eval_expr` ou `eval_binary` no módulo de expressões).
2. **Adicionar ramo `Value::Duration` aos match arms** de:
   - `BinaryOp::Add` → `Duration + Duration` → `Value::Duration`.
   - `BinaryOp::Sub` → `Duration - Duration` → `Value::Duration`.
   - `BinaryOp::Mul` → `Duration * Int` e `Int * Duration` → `Value::Duration`.
   - `BinaryOp::Div` → `Duration / Int` → `Value::Duration` (checked_div; zero → erro); `Duration / Duration` → `Value::Float` (ratio de nanos).
   - `BinaryOp::Eq` / `BinaryOp::Neq` → comparação de nanos.
   - `BinaryOp::Lt` / `BinaryOp::Leq` / `BinaryOp::Gt` / `BinaryOp::Geq` → ordenação de nanos.
3. **Garantir que o resultado de comparação é `Value::Bool`**.
4. **Garantir que o resultado de aritmética homogénea é `Value::Duration`**.
5. **Garantir que `Duration / Duration` é `Value::Float`** (não `Value::Duration`).
6. **Testes** em `rules/eval/` ou módulo de testes dedicado:
   - `duration("1h") + duration("30m")` → `Duration(5400s)`.
   - `duration("1h") * 2` → `Duration(7200s)`.
   - `2 * duration("1h")` → `Duration(7200s)`.
   - `duration("1h") / 2` → `Duration(1800s)`.
   - `duration("1h") / duration("30m")` → `Float(2.0)`.
   - `duration("1h") / 0` → erro eval.
   - `duration("1h") == duration("60m")` → `Bool(true)`.
   - `duration("1h") < duration("90m")` → `Bool(true)`.
   - `duration("1h") + int(30)` → erro de tipo (sem coerção).
7. **Linhagem**: `@prompt` aponta para `duration-arithmetic.md`; `@prompt-hash` via `--fix-hashes`.
8. **Validação**:
   - `cargo test --workspace -- --skip p350c_flag_on_nao_convergente_classifica` — verde.
   - `crystalline-lint .` — zero violations novas.
   - `git diff --stat` dos `.rs`: apenas match arms no eval + testes.

---

## 5. O que NÃO fazer (scope-out)

- **Não** implementar `Duration * Float` ou `Float * Duration` — scope-out futuro; requer decisão de precisão (float → int para nanos).
- **Não** implementar `Duration + Int` ou `Duration - Int` — o vanilla não coage Int para Duration em adição; é erro de tipo.
- **Não** implementar `%` (modulo) para Duration — verificar paridade vanilla; se não tem, scope-out.
- **Não** implementar `//` (floor division) para Duration — verificar paridade vanilla.
- **Não** tocar em `entities/duration.rs` — o tipo L1 já está correto; ops vêm do tipo base.
- **Não** tocar em `decimal` ou `version` — um passo de cada vez.
- **Não** criar funções stdlib (`duration_add`, etc.) — operações são via operadores infixos, não calls.

---

## 6. Critérios de aceitação

1. `duration("1h") + duration("30m")` → `Value::Duration(5400s)`; `-`, `* int`, `/ int` funcionam analogamente.
2. `duration("1h") / duration("30m")` → `Value::Float(2.0)` (ratio, não Duration).
3. Comparações (`==`, `<`, etc.) entre dois `Value::Duration` retornam `Value::Bool` correto.
4. Divisão por zero (`duration / 0`) produz erro eval (não panic).
5. Mistura Duration + Int (sem coerção definida) → erro de tipo existente (não regressão).
6. Zero tipo novo; zero variant novo; zero I/O.
7. Testes verdes (≥ 25 novos); lint zero; hashes propagados.
8. L0 salvo e hashado antes do código; sonda A.0 documentada no commit.

---

## 7. O que pode sair errado

- **Infra de eval binário não suporta operandos de tipos diferentes (`Duration * Int`).** Mitigação: sonda A.0 detecta; se o dispatch for por `(lhs_type, rhs_type)` e já trata `Int * Float`, `Duration * Int` é apenas mais um ramo. Se o dispatch for por tipo único, o passo cresce para M.
- **`Duration / Duration` retornando Float quebra invariante de "op homogénea retorna mesmo tipo".** Mitigação: é comportamento vanilla correto; documentar no L0 que `/` é especial.
- **Overflow em `Duration * Int` (nanos × int muito grande).** Mitigação: usar `checked_mul` ou equivalente; overflow → erro eval (paridade com comportamento de Int no vanilla).
- **Tentação de implementar `Duration * Float` junto.** Mitigação: scope-out claro; float como escalar requer conversão de precisão que é decisão de design separada.

---

## 8. Referências

- `entities/duration.rs` (P400) — `Duration` tipo L1.
- `entities/value.rs` — variant `Value::Duration`.
- `rules/eval/` — dispatch de expressões binárias (infra a sondar em A.0).
- `rules/stdlib/primitives_constructors.rs` (P403) — `native_duration`.
- P404 — `decimal-arithmetic.md` (padrão de operadores binários para tipos primitivos).
- ADR-0107 — paridade linguagem (forma dos operadores).
- ADR-0108 — medir-antes-de-decidir (sonda A.0).
- rust std::time::Duration docs — `Add`, `Sub`, `Mul<u32>`, `Div<u32>` impls.

---

## 9. Nota sobre o Tekt

Este passo é **S/M** se o eval binário já tem dispatch por par `(lhs_type, rhs_type)` (padrão estabelecido em P404). O risco principal é a heterogeneidade de `*` e `/` com `Int` — se o dispatch for monomórfico (um tipo por operador), o passo cresce para M. A sonda A.0 determina o tamanho real antes do código.

Se P404 ainda não fechou (Decimal aritmética), este passo pode ser **bloqueado** — a infra de eval binário pode não existir ainda. Verificar estado do P404 antes de iniciar A.0.
