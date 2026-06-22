# Passo 400 — Modelagem de Tipos: `Value::Duration` (S)

**Tipo**: Modelagem de tipos primitivos (L1 — pureza; zero I/O; expande enum `Value` fechado per ADR-0017).
**Data**: 2026-06-22.
**Padrão**: diagnóstico-primeiro (sonda 389); medir-antes-de-decidir (ADR-0108).
**ADRs relevantes**: ADR-0017 (portão aberto P395), ADR-0107 (paridade linguagem), ADR-0029 (pureza L1), ADR-0054 (graded scope-out — operações temporais ricas).
**Sonda fonte**: `typst-sonda-ausentes-ordem-passo-389.md` §2D — `Value::Duration` ausente, bloqueia literais duration e operações temporais.

> **Nota de numeração.** Um passo só. Não numerar à frente.
> **Nota de marco.** Segundo da série de tipos S puros (P399 Decimal → P400 Duration → P401 Version). Ritmo rápido, zero consumer complexo.

---

## 1. Contexto

P395 abriu o portão ADR-0017. P399 modelou `Value::Decimal` (S puro). Agora `Value::Duration` segue o mesmo padrão: tipo primitivo L1, zero consumer, zero I/O.

No vanilla:
```typ
#let d = duration(days: 3, hours: 2, minutes: 30)
#let seconds = d.in(seconds)
```

`Duration` representa um **intervalo de tempo** (não um ponto no tempo — isso é `Datetime`). No cristalino, usamos `std::time::Duration` como base, mas com resolução maior (nanossegundos u64) e campos nomeados no constructor.

Este passo é **S puro** — apenas modela o tipo e o integra no pipeline `Value`. Operações (`+`, `-`, `in()`, etc.) e constructor stdlib são passos futuros.

---

## 2. Decisão de engenharia

### 2.1 — `Duration` tipo L1

```rust
// entities/duration.rs — tipo L1 puro
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Duration {
    pub nanos: u64,  // total de nanossegundos; zero = duration vazio
}
```

**Decisão interna**: `u64` de nanossegundos é suficiente para ~584 anos — mais que o vanilla precisa. Alternativa: campos separados (days, hours, minutes, seconds, millis, micros, nanos) — mais complexo, não necessário para L1.

**Decisão ADR-0107 (língua vs mecânica)**: a paridade é com o **valor do intervalo** (3d2h30m = 3×86400 + 2×3600 + 30×60 segundos), não com a representação interna. `u64` nanos é mecânica; o valor é linguagem.

**Decisão ADR-0029 (pureza L1)**: `Duration` é puro — nenhum I/O, nenhum alloc dinâmico (u64 stack value).

### 2.2 — Enum `Value::Duration`

```rust
// entities/value.rs — novo variant
Duration(Duration),
```

**Por que não Arc**: `Duration` é `Copy` (8 bytes). Sem overhead de heap.

### 2.3 — Parsing de literal duration

No vanilla, `duration` é constructor stdlib (não literal suffixo):
```typ
#duration(days: 3, hours: 2, minutes: 30, seconds: 0)
```

No cristalino, este passo **não implementa o constructor** — apenas modela o tipo. O constructor `native_duration` é passo futuro (S).

Casts implementados neste passo:
- `Duration → Duration` (identity)
- `Int → Duration` (nanossegundos — preciso)
- `Float → Duration` (segundos → nanos, com perda de sub-nano)

**Não implementar cast de `Str` → `Duration`** — parsing de string "3d2h30m" é scope-out ADR-0054 graded (futuro S).

### 2.4 — Impacto cross-module (match exhaustivo)

| Módulo | O que muda | Como |
|--------|-----------|------|
| `entities/value.rs` | +1 variant | `Duration(Duration)` |
| `eval/repr.rs` | +1 arm | `"3d2h30m"` ou `"270180s"` (formato canónico) |
| `eval/cast.rs` | +1 arm | `Duration → Duration`; `Int → Duration` (nanos); `Float → Duration` (segundos→nanos, perda) |
| `eval/ops.rs` | +1 arm | `==` por `u64` equality; `+`/`-`/`*`/`/` — scope-out ADR-0054 graded |
| `layout/types.rs` | Nenhum | Duration não é Paint/Fill/Style/Length |
| `export.rs` | Nenhum | Duration não emite direto |
| `stdlib/` | Nenhum | `native_duration` é passo futuro |

**Decisão**: este passo é **modelagem pura** — não implementa `native_duration` nem operações. Apenas modela o tipo e o integra no enum `Value`.

---

## 3. FASE A — L0 (redação; checkpoint obrigatório)

### A.1 — Prompt L0 `duration.md`

Novo em `00_nucleo/prompts/entities/duration.md`:

- **Paridade**: `Duration` ≡ vanilla `Duration` tipo (intervalo de tempo).
- **Substrato**: tipo L1 puro `Duration { nanos: u64 }`; `Copy` (8 bytes); zero I/O; zero alloc.
- **Semântica**: intervalo de tempo — não ponto no tempo (Datetime). Resolução nanossegundos.
- **Variant `Value`**: `Duration(Duration)` — não Arc (Copy).
- **Cast**: `Duration → Duration` (identity); `Int → Duration` (nanossegundos); `Float → Duration` (segundos→nanos, perda documentada).
- **Repr**: formato canónico `"NdNhNmNs"` (ex.: `"3d2h30m"`) ou `"Ns"` (segundos totais). Escolher canónico vanilla se conhecido; senão, `"270180s"` (segundos totais como fallback).
- **Literal**: não existe literal suffixo — constructor via `duration(...)` (stdlib futuro).
- **Teste**: construção `Duration::from_seconds(270180)` → `Value::Duration` → repr → cast.

### A.2 — CHECKPOINT

Parar. Apresentar `duration.md` ao dono. **Só prosseguir para Fase B quando confirmar que guardou e computou hash.**

---

## 4. FASE B — Código (após confirmação humana)

### B.1 — Tipo entity `Duration`

Em `01_core/src/entities/duration.rs`:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Duration {
    pub nanos: u64,
}

impl Duration {
    pub const ZERO: Self = Self { nanos: 0 };
    pub const SECOND: Self = Self { nanos: 1_000_000_000 };
    pub const MINUTE: Self = Self { nanos: 60_000_000_000 };
    pub const HOUR: Self = Self { nanos: 3_600_000_000_000 };
    pub const DAY: Self = Self { nanos: 86_400_000_000_000 };

    pub fn from_nanos(nanos: u64) -> Self {
        Self { nanos }
    }

    pub fn from_seconds(seconds: u64) -> Self {
        Self { nanos: seconds * 1_000_000_000 }
    }

    pub fn from_minutes(minutes: u64) -> Self {
        Self { nanos: minutes * 60_000_000_000 }
    }

    pub fn from_hours(hours: u64) -> Self {
        Self { nanos: hours * 3_600_000_000_000 }
    }

    pub fn from_days(days: u64) -> Self {
        Self { nanos: days * 86_400_000_000_000 }
    }

    pub fn as_seconds(&self) -> u64 {
        self.nanos / 1_000_000_000
    }

    pub fn as_minutes(&self) -> u64 {
        self.nanos / 60_000_000_000
    }

    pub fn as_hours(&self) -> u64 {
        self.nanos / 3_600_000_000_000
    }

    pub fn as_days(&self) -> u64 {
        self.nanos / 86_400_000_000_000
    }

    pub fn is_zero(&self) -> bool {
        self.nanos == 0
    }

    /// Formato canónico: "3d2h30m15s" ou "0s" se zero.
    pub fn to_string(&self) -> String {
        if self.nanos == 0 {
            return "0s".to_string();
        }
        let mut rem = self.nanos;
        let days = rem / Self::DAY.nanos;
        rem %= Self::DAY.nanos;
        let hours = rem / Self::HOUR.nanos;
        rem %= Self::HOUR.nanos;
        let minutes = rem / Self::MINUTE.nanos;
        rem %= Self::MINUTE.nanos;
        let seconds = rem / Self::SECOND.nanos;
        rem %= Self::SECOND.nanos;
        let millis = rem / 1_000_000;

        let mut parts = Vec::new();
        if days > 0 { parts.push(format!("{}d", days)); }
        if hours > 0 { parts.push(format!("{}h", hours)); }
        if minutes > 0 { parts.push(format!("{}m", minutes)); }
        if seconds > 0 || millis > 0 || parts.is_empty() {
            if millis > 0 {
                parts.push(format!("{}.{:03}s", seconds, millis));
            } else {
                parts.push(format!("{}s", seconds));
            }
        }
        parts.join("")
    }
}

impl Default for Duration {
    fn default() -> Self {
        Self::ZERO
    }
}
```

**Nota**: `to_string()` usa formato canónico `NdNhNmNs` (ex.: `"3d2h30m15s"`). Se o vanilla usa formato diferente, adaptar. Se não se sabe, este formato é legível e reversível.

**Nota de overflow**: `from_days(u64::MAX)` overflowa — mas `u64::MAX` dias é ~50 trilhões de anos, fora de escopo real. Não adicionar checks de overflow neste passo (scope-out ADR-0054 graded).

### B.2 — Variant `Value::Duration`

Em `entities/value.rs`:

```rust
Duration(Duration),
```

Atualizar:
- `PartialEq` — match arm `Duration(a) => matches!(other, Duration(b) if a == b)`.
- `Repr` — `duration.to_string()` (formato canónico `NdNhNmNs`).
- `Cast` — `Duration(d) => Ok(d)`; `Int(i) => Ok(Duration::from_nanos(*i as u64))` (i64→u64, negativo → 0 ou Err? Decisão: negativo retorna `Err`); `Float(f) => Ok(Duration::from_nanos((*f * 1e9) as u64))` (perda de sub-nano).
- `type_name` — `"duration"`.

**Decisão cast `Int`**: `Int(i64)` → `Duration`. Se `i < 0`, retornar `Err` (duration negativa não é válida no vanilla). Se `i > u64::MAX`, retornar `Err` (overflow).

**Decisão cast `Float`**: `Float(f64)` → `Duration`. `f * 1e9` como `u64`. Se `f < 0`, `Err`. Se `f * 1e9 > u64::MAX`, `Err`.

### B.3 — Testes

1. **Unit `entities/duration.rs`** (6-8 tests):
   - `duration_zero` — `Duration::ZERO` → `0s`.
   - `duration_from_seconds` — `from_seconds(270180)` → `3d2h30m`.
   - `duration_from_days_hours_minutes` — `from_days(3) + from_hours(2) + from_minutes(30)` (se ops implementadas; senão, construir nanos direto).
   - `duration_to_string_canonical` — `3d2h30m15s500ms` → `"3d2h30m15.500s"`.
   - `duration_to_string_zero` — `Duration::ZERO` → `"0s"`.
   - `duration_equality` — `from_seconds(60) == from_minutes(1)`.
   - `duration_ordering` — `from_seconds(1) < from_seconds(2)`.
   - `duration_copy` — Copy trait (sem move).
   - `duration_constants` — `SECOND`, `MINUTE`, `HOUR`, `DAY` valores corretos.

2. **Unit `entities/value.rs`** (4-5 tests):
   - `value_duration_variant` — discriminação `Value::Duration`.
   - `value_duration_repr` — `"3d2h30m"` para `from_seconds(270180)`.
   - `value_duration_repr_zero` — `"0s"` para `Duration::ZERO`.
   - `value_duration_cast_identity` — `Duration → Duration`.
   - `value_duration_cast_from_int` — `Int(1_000_000_000) → Duration::from_nanos(1_000_000_000)` (1 segundo).
   - `value_duration_cast_from_int_negative` — `Int(-1) → Err`.
   - `value_duration_cast_from_float` — `Float(1.5) → Duration::from_nanos(1_500_000_000)` (1.5 segundos).
   - `value_duration_cast_from_float_negative` — `Float(-1.0) → Err`.
   - `value_duration_partial_eq` — equality com outro Duration.

3. **Integration** (1-2 tests, opcional):
   - `duration_no_ops_yet` — confirmar que `Value::Duration` não participa de `+`/`-`/`*`/`/` (se ops.rs tiver match, adicionar braço que retorna `Err` claro: `"operações em duration — scope-out futuro"`).

### B.4 — Linhagem

- `@prompt` aponta para `duration.md`.
- `@prompt-hash` via `--fix-hashes`.
- Referência cruzada: P395 (portão ADR-0017 aberto), P399 (Decimal — modelo de tipo S puro), ADR-0107 (paridade linguagem), ADR-0029 (pureza L1).

---

## 5. O que NÃO fazer (scope-out)

- **Não** implementar `native_duration` (stdlib constructor) — é passo futuro (S).
- **Não** implementar operações aritméticas (`+`, `-`, `*`, `/`, `%`) — scope-out ADR-0054 graded (futuro S).
- **Não** implementar conversões (`.in(seconds)`, `.in(minutes)`, etc.) — scope-out ADR-0054 graded (futuro S).
- **Não** implementar cast de `Str` → `Duration` — parsing de string "3d2h30m" é scope-out ADR-0054 graded (futuro S).
- **Não** implementar `Duration` como `Length`/`Angle`/`Ratio` — não é dimensional.
- **Não** adicionar checks de overflow em constructors — scope-out ADR-0054 graded.
- **Não** tocar em `Version` — é P401.
- **Não** quebrar invariantes de camada (L1 puro, zero I/O).

---

## 6. Critérios de aceitação

1. `Value::Duration(Duration)` compila e participa de `match` exhaustivo em eval.
2. `Duration` é `Copy`, puro-Rust, zero alloc.
3. Cast: `Duration → Duration` (identity); `Int → Duration` (nanos, negativo → Err); `Float → Duration` (segundos→nanos, negativo → Err).
4. Repr: formato canónico `NdNhNmNs` (ex.: `"3d2h30m"`) ou `"0s"`.
5. Zero consumer complexo; zero I/O; zero func stdlib nova.
6. Testes verdes (≥10 unit + 1-2 integration); lint zero; hashes propagados.
7. Inventário 148: `Value::Duration` transita `ausente` → `implementado` (tipo); `duration()` stdlib permanece `ausente` (futuro S).
8. L0 salvo e hashado antes do código (protocolo de nucleação).
9. Ritmo S: tempo de ciclo comparável a P399 (baseline de tipo S puro).

---

## 7. O que pode sair errado

- **Formato `NdNhNmNs` não é o canónico do vanilla.** Mitigação: se o vanilla usa formato diferente (ex.: `"270180s"` ou `"P3DT2H30M"` ISO 8601), adaptar `to_string()`. Mas como este passo é S puro e o repr não é user-facing crítico, o formato pode ser ajustado no passo do constructor stdlib (quando o vanilla format será conhecido via testes de paridade).
- **Cast `Float` para `Duration` perde precisão.** Mitigação: documentar no L0; `Float(1.23456789)` → nanos truncados. É aceitável para S puro.
- **Cast `Int` negativo retorna `Err` — mas o vanilla pode aceitar?** Mitigação: verificar; se o vanilla aceita duration negativa (improvável), ajustar. Se não, `Err` é correto.
- **Tentação de já implementar `native_duration` ou operações.** Mitigação: um passo de cada vez; este é S puro de modelagem.
- **Tentação de implementar `Str` → `Duration` parsing.** Mitigação: scope-out ADR-0054; parsing é S de funcionalidade, não modelagem.

---

## 8. Referências

- P395 — `Value::Tiling` (portão ADR-0017 aberto).
- P399 — `Value::Decimal` (modelo de tipo S puro — padrão paralelo).
- ADR-0017 — trava arquitetural enum fechado.
- ADR-0107 — paridade linguagem vs mecânica (intervalo de tempo).
- ADR-0029 — pureza L1.
- ADR-0054 — graded scope-out (operações temporais, constructor stdlib, parsing Str).

---

## 9. Nota sobre o Tekt

Este passo é o **segundo da série de tipos S puros** — P399 (Decimal) → P400 (Duration) → P401 (Version). O ritmo deve ser uniforme: cada um ~10-15 tests, zero surprises, zero consumers.

Se P400 demorar significativamente mais que P399 (>2×), investigar: `Duration` tem mais complexidade aparente (formato canónico, constants, múltiplos constructors) que pode mascarar dependências ocultas. Mas o tipo em si é tão simples quanto `Decimal` — um `u64` com métodos de conveniência.

Registar o tempo de ciclo de P400 como **comparação com P399** — validar que a série de tipos S puros mantém ritmo constante.
