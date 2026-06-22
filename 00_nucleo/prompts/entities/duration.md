# Prompt L0 — `Duration` — intervalo de tempo
Hash do Código: ebe6ddde

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/duration.rs`, `01_core/src/entities/value.rs`
**Origem**: Passo 400 — modelagem de `Value::Duration` (S); segundo tipo S puro após abertura do portão ADR-0017.
**ADRs**: ADR-0017 (portão aberto P395), ADR-0107 (paridade linguagem), ADR-0029 (pureza L1), ADR-0054 (operações temporais scope-out).

---

## 1. Contexto

O Typst vanilla expõe `duration` como tipo de intervalo de tempo (não ponto no tempo — isso é `datetime`). Em cristalino, o variant `Value::Duration` estava ausente por causa do enum fechado (ADR-0017). Com o portão aberto em P395 e os tipos `Decimal` (P399) modelados, segue-se `Duration` como tipo S puro.

## 2. Tipo L1

```rust
// 01_core/src/entities/duration.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Duration {
    pub nanos: u64,
}
```

- `u64` de nanossegundos — resolução suficiente (~584 anos); zero alloc; `Copy`.
- Semântica de intervalo, não de ponto no tempo.

## 3. Construtores e acesso

```rust
impl Duration {
    pub const ZERO: Self = Self { nanos: 0 };
    pub const SECOND: Self = Self { nanos: 1_000_000_000 };
    pub const MINUTE: Self = Self { nanos: 60_000_000_000 };
    pub const HOUR: Self = Self { nanos: 3_600_000_000_000 };
    pub const DAY: Self = Self { nanos: 86_400_000_000_000 };

    pub fn from_nanos(nanos: u64) -> Self;
    pub fn from_seconds(seconds: u64) -> Self;
    pub fn from_minutes(minutes: u64) -> Self;
    pub fn from_hours(hours: u64) -> Self;
    pub fn from_days(days: u64) -> Self;

    pub fn as_seconds(&self) -> u64;
    pub fn as_minutes(&self) -> u64;
    pub fn as_hours(&self) -> u64;
    pub fn as_days(&self) -> u64;
    pub fn is_zero(&self) -> bool;

    /// Formato canónico: "3d2h30m15.500s" ou "0s".
    pub fn to_string(&self) -> String;
}

impl Default for Duration {
    fn default() -> Self { Self::ZERO }
}
```

## 4. Variant `Value::Duration`

```rust
// 01_core/src/entities/value.rs
Duration(crate::entities::duration::Duration),
```

Atualizações necessárias:
- `type_name()` → `"duration"`.
- `PartialEq` / `Hash` via derive (a struct já implementa).
- `cast_duration()` para extrair / converter `Duration`.
- `From<Duration> for Value`.

## 5. Cast

- `Value::Duration(d) -> Duration`: identidade.
- `Value::Int(i) -> Duration`: trata `i` como nanossegundos; rejeita negativos (`None`).
- `Value::Float(f) -> Duration`: trata `f` como segundos, multiplica por `1e9`; rejeita negativos e overflow (`None`).
- `Value::Str` → `Duration`: scope-out ADR-0054 graded (parsing futuro).
- Outros tipos → `None`.

## 6. Repr

Quando existir `native_repr`, deve devolver o formato canónico `"NdNhNmN.NNNs"` (ex.: `"3d2h30m15.500s"`) ou `"0s"`. O repr actual não é user-facing crítico neste passo S puro.

## 7. Scope-out

- `native_duration(...)` stdlib — passo futuro (S).
- Operações aritméticas (`+`, `-`, `*`, `/`) e conversões (`.in(seconds)`) — scope-out ADR-0054 graded.
- Cast de `Str` → `Duration` — scope-out ADR-0054 graded.
- Checks de overflow em constructors — scope-out (valores fora de escopo real).
