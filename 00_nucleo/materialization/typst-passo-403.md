# Passo 403 — Constructors stdlib: `decimal()`, `duration()`, `version()` (M)

**Tipo**: Materialização (L1 stdlib + enum Value; zero tipo novo; zero I/O).  
**Data**: 2026-06-22.  
**Padrão**: diagnóstico-primeiro; medir-antes-de-decidir (ADR-0108).  
**ADRs relevantes**: ADR-0017 (portão aberto), ADR-0107 (paridade linguagem), ADR-0033 (paridade vanilla).  
**Sonda fonte**: Inventário 148 — `native_decimal`, `native_duration`, `native_version` ausentes; tipos Decimal/Duration/Version modelados em P399–P401.

> **Nota de numeração.** Um passo só. Não numerar à frente.

---

## 1. Contexto

P399 (Decimal), P400 (Duration), P401 (Version) modelaram os tipos L1 e os variantes em `Value`. Os tipos existem mas **não têm constructor stdlib** — o utilizador não pode criá-los em código Typst. Este passo consome o portão ADR-0017 (já aberto) e ativa os 3 tipos para uso real.

| Tipo | Modelado em | Constructor vanilla | Paridade |
|------|-------------|---------------------|----------|
| `Decimal` | P399 | `decimal("1.23")` | `Str → Decimal` fallible |
| `Duration` | P400 | `duration("3d2h")` | `Str → Duration` fallible |
| `Version` | P401 | `version("1.2.3")` | `Str → Version` fallible |

---

## 2. Decisão de engenharia

Os 3 constructors são **morfologicamente idênticos**: `Str → T fallible`. A paridade (ADR-0107) é com a **forma da linguagem** (função que converte string em tipo), não com a mecânica interna. No cristalino:

- `native_decimal(s)` → parse `rust_decimal::Decimal::from_str` → `Value::Decimal`.
- `native_duration(s)` → parse formato canónico `NdNhNmNs` → `Value::Duration`.
- `native_version(s)` → parse semver 2.0.0 → `Value::Version`.

Não criar `Value::Decimal`/`Duration`/`Version` novos — reutilizar os existentes do P399–P401. Não expandir o enum.

---

## 3. FASE A — L0 (redação; checkpoint obrigatório)

### A.0 — Sonda do substrato (obrigatória; 5 min)

Antes de redigir o L0, executar e documentar:

```bash
# 1. Confirmar que Value::Decimal, Value::Duration, Value::Variant existem no enum
grep -n "Decimal\|Duration\|Version" entities/value.rs | head -20
# Esperado: 3 linhas com "Value::Decimal", "Value::Duration", "Value::Version"

# 2. Confirmar que os tipos L1 existem em entities/
grep -rn "pub struct Decimal\|pub struct Duration\|pub struct Version" entities/
# Esperado: 3 hits em entities/decimal.rs, entities/duration.rs, entities/version.rs

# 3. Confirmar padrão de registro de natives em make_stdlib
grep -n "native_" rules/eval/mod.rs | tail -10
# Esperado: lista com native_str, native_int, native_float, etc.

# 4. Confirmar que os 3 constructors não existem ainda
grep -rn "native_decimal\|native_duration\|native_version" rules/
# Esperado: zero hits (confirmam ausência no Inventário 148)
```

**Critério de passagem**: todos os 4 grep produzem output conforme esperado. Se qualquer um falhar (ex.: `Value::Duration` não existe no enum), **parar imediatamente** e reclassificar o passo — o substrato não está pronto.

Documentar o resultado da sonda no commit message: `Sonda P403: substrato verificado; 3 variants + 3 structs + padrão make_stdlib OK.`

### A.1 — Prompt L0 `primitives-constructors.md`

Novo em `00_nucleo/prompts/engine/stdlib/primitives-constructors.md` (ou integrar em `stdlib.md` existente se o linter sinalizar órfão):

- **Paridade**: `decimal("1.5")` ≡ `1.5` (decimal literal); `duration("1h30m")` ≡ `5400s`; `version("1.2.3")` compara via semver.
- **Substrato**: 3 helpers stdlib puros que constroem `Value` existentes via `From`/`TryFrom`.
- **Sem tipo novo**: reutiliza `Value::Decimal`, `Value::Duration`, `Value::Version`.
- **Parâmetros**: 1 posicional `Str` cada; named args rejeitados.
- **Erros**: string inválida → erro eval (mensagem clara; paridade vanilla).
- **Testes**: parse válido + inválido para cada; round-trip `repr`.

### A.2 — CHECKPOINT

Parar. Apresentar `primitives-constructors.md` ao dono. **Só prosseguir para Fase B quando confirmar que guardou e computou hash.**

---

## 4. FASE B — Código (após confirmação humana)

1. **Implementar `native_decimal`** em `rules/stdlib/` (novo ou existente):
   ```rust
   // pseudo: native_decimal(s: Str) -> Result<Value, EvalError>
   // Decimal::from_str(&s).map(Value::Decimal).map_err(|e| ...)
   ```
2. **Implementar `native_duration`** — parse canónico `NdNhNmNs` (reutilizar lógica de `Repr` de Duration se existir; se não, implementar parser minimal).
3. **Implementar `native_version`** — parse semver 2.0.0 (reutilizar `Version::parse` de P401).
4. **Registar os 3 em `make_stdlib`** em `rules/eval/mod.rs`.
5. **Testes**:
   - `decimal("1.23")` → `Value::Decimal(1.23)`.
   - `decimal("abc")` → erro.
   - `duration("2h30m")` → `Value::Duration(9000_000_000_000)` nanos.
   - `duration("abc")` → erro.
   - `version("1.2.3-alpha")` → `Value::Version(1,2,3,"alpha","")`.
   - `version("invalid")` → erro.
   - Round-trip `repr(decimal("1.5"))` → `"1.5"`.
6. **Linhagem**: `@prompt` aponta para `primitives-constructors.md`; `@prompt-hash` via `--fix-hashes`.
7. **Validação**:
   - `cargo test --workspace` — verde.
   - `crystalline-lint .` — zero violations novas.
   - `git diff --stat` dos `.rs`: apenas 3 natives + registro + testes.

---

## 5. O que NÃO fazer (scope-out)

- **Não** criar `Value` variant novo — reutiliza P399–P401.
- **Não** tocar em `entities/` — os tipos já existem.
- **Não** adicionar operações aritméticas (`+`, `-`, `*`, `/`) — scope-out futuro S/M por tipo.
- **Não** adicionar field access (`.major`, `.minor`, etc.) — scope-out futuro S.
- **Não** tocar em outro ausente — um passo de cada vez.

---

## 6. Critérios de aceitação

1. `decimal("1.5")`, `duration("1h")`, `version("1.0.0")` compilam e produzem `Value` correto.
2. Zero tipo novo; zero variant novo; zero I/O.
3. Testes verdes (≥ 12 novos); lint zero; hashes propagados.
4. Inventário 148: `decimal`/`duration`/`version` transitem `ausente` → `implementado`.
5. L0 salvo e hashado antes do código (protocolo de nucleação).
6. **Sonda A.0 executada e documentada no commit** — não pode ser pulada.

---

## 7. O que pode sair errado

- **`Duration` parser não existe ainda.** Mitigação: implementar parser minimal inline (regex ou hand-rolled) no próprio `native_duration`; refinar para módulo dedicado em passo futuro se crescer.
- **`Decimal` from_str rejeita formatos que o vanilla aceita.** Mitigação: testar com exemplos vanilla; ajustar parser se necessário (dívida documentada).
- **Tentação de fazer operações aritméticas junto.** Mitigação: scope-out claro; constructors primeiro, operações depois.
- **Sonda A.0 falha (substrato inesperado).** Mitigação: parar; reclassificar passo; investigar por que o tipo sumiu ou nunca existiu.

---

## 8. Referências

- `entities/decimal.rs` (P399) — `Decimal` tipo L1.
- `entities/duration.rs` (P400) — `Duration` tipo L1.
- `entities/version.rs` (P401) — `Version` tipo L1.
- `entities/value.rs` — variants `Value::Decimal`, `Value::Duration`, `Value::Version`.
- ADR-0017 — portão aberto para tipos primitivos.
- ADR-0107 — paridade linguagem (forma, não mecânica).
- ADR-0108 — medir-antes-de-decidir (sonda A.0 é a medição).

---

## 9. Nota sobre o Tekt

Este passo é o **consumo do portão ADR-0017**: tipos modelados em P399–P401 ganham vida via constructors. Ritmo esperado: M (3 constructors + testes), mas com baixo risco porque os tipos já estão validados. Se o passo demorar >2× o tempo do P401, investigar complexidade oculta no parser de Duration.
