# Passo 413 — Aritmética `Decimal`: `+`, `-`, `*`, `/` e comparações (S/M)

**Tipo**: Materialização (L1 eval + stdlib; zero tipo novo; zero I/O).  
**Data**: 2026-06-22.  
**Padrão**: diagnóstico-primeiro; medir-antes-de-decidir (ADR-0108).  
**ADRs relevantes**: ADR-0107 (paridade linguagem), ADR-0033 (paridade vanilla).  
**Sonda fonte**: Inventário 148 — `Decimal` constructor ativado em P403; operadores aritméticos e comparações ainda ausentes para `Value::Decimal`.

> **Nota de numeração.** Um passo só. Não numerar à frente.

---

## 1. Contexto

P399 modelou `Decimal` em L1; P403 ativou o constructor `decimal("1.23")`. O tipo existe, mas **não participa em operações aritméticas nem comparações** — `decimal("1.5") + decimal("2.5")` falha ou produz erro de tipo. Este passo ativa `+`, `-`, `*`, `/` e os operadores de comparação (`==`, `!=`, `<`, `<=`, `>`, `>=`) para `Value::Decimal`, consumindo o tipo que agora vive na stdlib.

| Operação | Vanilla Typst | Paridade cristalino |
|----------|---------------|---------------------|
| `+` | `decimal + decimal` | `Value::Decimal + Value::Decimal` |
| `-` | `decimal - decimal` | `Value::Decimal - Value::Decimal` |
| `*` | `decimal * decimal` | `Value::Decimal * Value::Decimal` |
| `/` | `decimal / decimal` | `Value::Decimal / Value::Decimal` (fallible por div/0) |
| `==`, `!=` | `decimal == decimal` | bit-equivalente via `rust_decimal` |
| `<`, `<=`, `>`, `>=` | `decimal < decimal` | ordenação total via `rust_decimal` |

---

## 2. Decisão de engenharia

A aritmética `Decimal` é **homogénea**: ambos os operandos são `Value::Decimal`; o resultado é `Value::Decimal`. Não há coerção com `Int` ou `Float` neste passo (coerção cruzada é scope-out futuro).

No cristalino:
- O eval de expressões binárias (`BinaryExpr`) faz dispatch por tipo dos operandos.
- Para `Int` e `Float` já existe infraestrutura de operadores; este passo **adiciona o ramo `Decimal` aos match arms existentes**.
- `+`, `-`, `*` usam `rust_decimal::Decimal` diretamente (ops da crate).
- `/` usa `checked_div` ou equivalente; divisão por zero → erro eval (paridade vanilla).
- Comparações usam `PartialOrd`/`Eq` de `rust_decimal::Decimal`.

Se o eval não tiver infraestrutura de operadores binários centralizada (ex.: cada op é um `native_` separado), a sonda A.0 descobrirá e o plano ajusta para criar o dispatch.

---

## 3. FASE A — L0 (redação; checkpoint obrigatório)

### A.0 — Sonda do substrato (obrigatória; 5 min)

Antes de redigir o L0, executar e documentar:

```bash
# 1. Confirmar que Value::Decimal existe no enum
grep -n "Value::Decimal" entities/value.rs
# Esperado: 1+ linha confirmando variant

# 2. Confirmar que Decimal tipo L1 existe e tem ops da crate
grep -n "impl.*Add\|impl.*Sub\|impl.*Mul\|impl.*Div\|impl.*PartialOrd\|impl.*Eq" entities/decimal.rs
# Esperado: zero (ops são da crate rust_decimal, não nossos impls) OU
#           linhas mostrando que Decimal é wrapper com acesso ao inner

# 3. Confirmar infra de eval binário existente para Int/Float/Duration/Version
grep -rn "BinaryOp::Add\|BinaryOp::Sub\|BinaryOp::Mul\|BinaryOp::Div\|BinaryOp::Eq\|BinaryOp::Lt" rules/eval/
# Esperado: match arms ou dispatch centralizado que trata Int/Float/Duration/Version

# 4. Confirmar que Decimal ainda NÃO está nos operadores
grep -rn "Value::Decimal" rules/eval/ | grep -i "add\|sub\|mul\|div\|cmp\|eq\|lt\|gt"
# Esperado: zero hits (confirma ausência no dispatch)

# 5. Confirmar que rust_decimal suporta as ops necessárias
grep "rust_decimal" Cargo.toml
# Esperado: dependência presente
```

**Critério de passagem**: 
- (1) `Value::Decimal` existe.
- (2) `Decimal` tipo L1 acessível.
- (3) Infra de eval binário existe para pelo menos um tipo (Int, Float, Duration ou Version).
- (4) Decimal ainda ausente nos operadores.
- (5) `rust_decimal` é dependência.

Se (3) falhar (não há infra de eval binário), **parar imediatamente** — o passo é maior que S/M e requer infraestrutura de operadores primeiro. Se (1) ou (5) falhar, reclassificar.

Documentar resultado no commit: `Sonda P413: Decimal variant OK; eval binário infra OK (Int/Float/Duration/Version); Decimal ausente em ops; rust_decimal OK.`

### A.1 — Prompt L0 `decimal-arithmetic.md`

Novo em `00_nucleo/prompts/rules/eval/decimal-arithmetic.md`:

- **Paridade**: `decimal("1.5") + decimal("2.5")` → `4.0` (Decimal); comparações retornam `Bool`.
- **Substrato**: ramos adicionais no dispatch binário de `BinaryExpr` eval (ou equivalente) para `Value::Decimal`.
- **Sem tipo novo**: reutiliza `Value::Decimal` existente.
- **Operadores**: `+`, `-`, `*`, `/` (4 aritméticos); `==`, `!=`, `<`, `<=`, `>`, `>=` (6 comparativos).
- **Divisão por zero**: `/` com divisor zero → erro eval com mensagem clara.
- **Coerção**: não implementar coerção Decimal↔Int/Float neste passo; mistura de tipos → erro de tipo existente.
- **Testes**: 
  - Aritmética: 4 ops × 3 casos (positivo, negativo, zero) = 12 mínimo.
  - Comparações: 6 ops × 3 casos (menor, igual, maior) = 18 mínimo.
  - Divisão por zero: 1 caso.
  - Tipo errado (Decimal + Int): 1 caso.
  - Total mínimo: ~20 testes.

### A.2 — CHECKPOINT

Parar. Apresentar `decimal-arithmetic.md` ao dono. **Só prosseguir para Fase B quando confirmar que guardou e computou hash.**

---

## 4. FASE B — Código (após confirmação humana)

1. **Localizar o dispatch binário** em `rules/eval/` (provavelmente `eval_expr` ou `eval_binary` no módulo de expressões).
2. **Adicionar ramo `Value::Decimal` aos match arms** de:
   - `BinaryOp::Add` → `a + b` via `rust_decimal::Decimal`.
   - `BinaryOp::Sub` → `a - b`.
   - `BinaryOp::Mul` → `a * b`.
   - `BinaryOp::Div` → `a.checked_div(b).ok_or(div_zero_error)`.
   - `BinaryOp::Eq` / `BinaryOp::Neq` → `==` / `!=`.
   - `BinaryOp::Lt` / `BinaryOp::Leq` / `BinaryOp::Gt` / `BinaryOp::Geq` → comparações.
3. **Garantir que o resultado de comparação é `Value::Bool`** (não `Value::Decimal`).
4. **Garantir que o resultado de aritmética é `Value::Decimal`**.
5. **Testes** em `rules/eval/` ou módulo de testes dedicado:
   - `decimal("1.5") + decimal("2.5")` → `Decimal(4.0)`.
   - `decimal("10") / decimal("3")` → `Decimal(3.333...)` (precisão da crate).
   - `decimal("1") / decimal("0")` → erro eval.
   - `decimal("1.0") == decimal("1.00")` → `Bool(true)` (paridade: trailing zeros não alteram valor).
   - `decimal("1.0") < decimal("2.0")` → `Bool(true)`.
   - `decimal("1") + int(2)` → erro de tipo (sem coerção).
6. **Linhagem**: `@prompt` aponta para `decimal-arithmetic.md`; `@prompt-hash` via `--fix-hashes`.
7. **Validação**:
   - `cargo test --workspace -- --skip p350c_flag_on_nao_convergente_classifica` — verde.
   - `crystalline-lint .` — zero violations novas.
   - `git diff --stat` dos `.rs`: apenas match arms no eval + testes.

---

## 5. O que NÃO fazer (scope-out)

- **Não** implementar coerção Decimal↔Int/Float — scope-out futuro; mistura de tipos continua erro.
- **Não** implementar `//` (floor division) se não existir para Decimal no vanilla — verificar; se vanilla não tem, não adicionar.
- **Não** implementar `%` (modulo) para Decimal — verificar paridade vanilla; se não tem, scope-out.
- **Não** tocar em `entities/decimal.rs` — o tipo L1 já está correto; ops vêm da crate.
- **Não** tocar em `duration` ou `version` — um passo de cada vez.
- **Não** criar funções stdlib (`decimal_add`, etc.) — operações são via operadores infixos, não calls.

---

## 6. Critérios de aceitação

1. `decimal("1.5") + decimal("2.5")` → `Value::Decimal(4.0)`; `-`, `*`, `/` funcionam analogamente.
2. Comparações (`==`, `<`, etc.) entre dois `Value::Decimal` retornam `Value::Bool` correto.
3. Divisão por zero produz erro eval (não panic).
4. Mistura Decimal + Int/Float sem coerção → erro de tipo existente (não regressão).
5. Zero tipo novo; zero variant novo; zero I/O.
6. Testes verdes (≥ 20 novos); lint zero; hashes propagados.
7. L0 salvo e hashado antes do código; sonda A.0 documentada no commit.

---

## 7. O que pode sair errado

- **Infra de eval binário não existe ou é fragmentada.** Mitigação: sonda A.0 detecta; se não houver dispatch centralizado, o passo vira M/L e requer refatoração prévia do eval.
- **`rust_decimal` não implementa `PartialOrd` de forma total.** Mitigação: verificar; `Decimal` da crate implementa `PartialOrd` e `Eq` corretamente para valores finitos (não-NaN).
- **Precisão de `/` difere do vanilla.** Mitigação: testar com exemplos vanilla; documentar divergência se houver (dívida aceitável se mínima).
- **Tentação de implementar coerção cruzada.** Mitigação: scope-out claro; coerção é passo separado e requer decisão de design (precisão vs. performance).

---

## 8. Referências

- `entities/decimal.rs` (P399) — `Decimal` tipo L1.
- `entities/value.rs` — variant `Value::Decimal`.
- `rules/eval/` — dispatch de expressões binárias (infra a sondar em A.0).
- `rules/stdlib/primitives_constructors.rs` (P403) — `native_decimal`.
- P405/P409 — aritmética Duration (padrão de operadores binários para tipos primitivos).
- ADR-0107 — paridade linguagem (forma dos operadores).
- ADR-0108 — medir-antes-de-decidir (sonda A.0).
- rust_decimal crate docs — `Add`, `Sub`, `Mul`, `Div`, `PartialOrd`, `Eq` impls.

---

## 9. Nota sobre o Tekt

Este passo é **S/M** porque o tipo já existe e a operação é homogénea. O risco principal é a infra de eval binário: se o dispatch for centralizado, são ~10 linhas por operador; se for fragmentado, o passo cresce. A sonda A.0 determina o tamanho real antes do código. Se o eval já trata `Int`, `Float`, `Duration` e `Version` de forma elegante, `Decimal` segue o mesmo padrão em minutos.
