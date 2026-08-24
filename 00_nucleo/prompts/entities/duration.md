# Prompt L0 — `Duration` — intervalo de tempo com sinal
Hash do Código: 5b70ecaa

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/duration.rs`, `01_core/src/entities/value.rs`
**Origem**: Passo 400 — modelagem de `Value::Duration` (S); Passo 850 — migração para representação com sinal (paridade vanilla `duration(seconds: -3)` e `-duration(seconds: 3)`).
**ADRs**: ADR-0017 (portão aberto P395), ADR-0107 (paridade linguagem), ADR-0029 (pureza L1), ADR-0054 (operações temporais scope-out).

---

## 1. Contexto

O Typst vanilla expõe `duration` como tipo de intervalo de tempo (não ponto no tempo — isso é `datetime`). Em cristalino, o variant `Value::Duration` estava ausente por causa do enum fechado (ADR-0017). Com o portão aberto em P395 e os tipos `Decimal` (P399) modelados, segue-se `Duration` como tipo S puro. A representação é **com sinal**, de modo a suportar durações negativas (`duration(seconds: -3)`, `-duration(seconds: 3)`, subtracção que produz resultado negativo) em paridade com o vanilla.

> **Fonte de paridade (P1031)** — doc comments do vanilla ratificado (`e0e8ca4d`),
> `crates/typst-library/src/foundations/duration.rs`, publicados em
> `typst.app/docs/reference/foundations/duration/`:
>
> - **Sinal** — `duration.rs:9-12`, doc comment `#[ty]` do tipo: *"Represents a **positive or
>   negative** span of time."* sobre `pub struct Duration(time::Duration)`. **Citação
>   literal** da afirmação-chave deste contexto: a linguagem admite durações negativas, logo
>   a representação com sinal é paridade e não escolha do cristalino.
> - **Intervalo, não ponto no tempo** — `duration.rs:38-42`, doc do construtor: *"Creates a
>   new duration. You can specify the duration using weeks, days, hours, minutes and
>   seconds. You can also get a duration by **subtracting two datetimes**."* Confirma a
>   distinção `duration` vs `datetime` e as cinco unidades nomeadas.
>
> **Natureza**: literal para o sinal e para a distinção intervalo/instante. A representação
> `nanos: i128` é mecânica (o vanilla usa `time::Duration`) e diverge de propósito
> (ADR-0107); o que tem de coincidir é o observável — que uma duração negativa seja
> representável e imprimível.

---

## 2. Tipo L1

```rust
// 01_core/src/entities/duration.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Duration {
    pub nanos: i128,
}
```

- `i128` de nanossegundos — resolução suficiente (~1.7e17 segundos em cada sinal); zero alloc; `Copy`.
- Semântica de intervalo, não de ponto no tempo.
- Durações negativas são representáveis e ordenáveis.

---

## 3. Construtores e acesso

```rust
impl Duration {
    pub const ZERO: Self = Self { nanos: 0 };
    pub const SECOND: Self = Self { nanos: 1_000_000_000 };
    pub const MINUTE: Self = Self { nanos: 60_000_000_000 };
    pub const HOUR: Self = Self { nanos: 3_600_000_000_000 };
    pub const DAY: Self = Self { nanos: 86_400_000_000_000 };

    pub fn from_nanos(nanos: i128) -> Self;
    pub fn from_seconds(seconds: i64) -> Self;
    pub fn from_minutes(minutes: i64) -> Self;
    pub fn from_hours(hours: i64) -> Self;
    pub fn from_days(days: i64) -> Self;

    pub fn as_seconds(&self) -> i64;
    pub fn as_minutes(&self) -> i64;
    pub fn as_hours(&self) -> i64;
    pub fn as_days(&self) -> i64;
    pub fn is_zero(&self) -> bool;

    /// Formato canónico: "3d2h30m15.500s", "-3d2h30m15.500s" ou "0s".
    pub fn to_string(&self) -> String;
}

impl Default for Duration {
    fn default() -> Self { Self::ZERO }
}

impl Neg for Duration {
    type Output = Self;
    fn neg(self) -> Self::Output;
}
```

---

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

---

## 5. Cast

- `Value::Duration(d) -> Duration`: identidade.
- `Value::Int(i) -> Duration`: trata `i` como nanossegundos; aceita negativos.
- `Value::Float(f) -> Duration`: trata `f` como segundos, multiplica por `1e9`; aceita negativos; rejeita overflow (`None`).
- `Value::Str` → `Duration`: scope-out ADR-0054 graded (parsing futuro).
- Outros tipos → `None`.

---

## 6. Repr

`native_repr` devolve o formato nomeado do vanilla: `duration(weeks: 1, days: 2, hours: 3, minutes: 4, seconds: 5)`. Componentes zero são omitidos. Durações negativas representam-se com componentes negativos (ex.: `duration(seconds: -3)`). Zero → `duration()`.

---

## 7. Aritmética

Operadores binários e unários em `Value` (não na entidade pura):
- `UnOp::Neg` sobre `Value::Duration` devolve `Value::Duration(-d)`.
- `BinOp::Add` / `Sub` entre durações: adição/subtracção com sinal; overflow → erro.
- `BinOp::Mul` / `Div` de duração por `Int` / `Float`: produto/quociente com sinal; divisão por zero → erro.
- `BinOp::Div` entre durações: devolve `Float` (pode ser negativo); divisão por zero → erro.
- Comparações (`<`, `<=`, `>`, `>=`): ordenação total com sinal.

---

## 8. Field access

`Value::Duration` expõe campos de leitura (P412):
- `.seconds`, `.minutes`, `.hours`, `.days` devolvem `Float` com sinal.

---

## 9. Scope-out

- Cast de `Str` → `Duration` — scope-out ADR-0054 graded.
- Checks de overflow em constructors — scope-out para valores fora de escopo real.


## P1140.1-B — medição anterior à decisão (2026-08-23)

No vanilla ratificado `a51e02804`, `repr(type(PATH))` devolve `"type"`; no
cristalino anterior a esta mudança devolve `"function"`. O catálogo P1140 e
os probes públicos em `00_nucleo/diagnosticos/superficie-linguagem-p1140*`
medem a divergência para `decimal`, `duration`, `regex`, `selector`, `stroke`,
`tiling` e `version`. Os construtores atuais foram novamente executados após
a atomização P1140.1-A: catálogo byte-idêntico e 22 probes byte-idênticos ao
baseline estrutural. Esta é divergência de semântica pública da linguagem,
não de mecânica Rust (ADR-0107).

## P1140.1-B — identidade pública do tipo `duration`

O nome global `duration` é o valor `Value::Type(Type::Duration)`, chamável
pelo dispatcher estático e delegado ao construtor L1 já existente. Assim,
`type(duration) == type` e `repr(type(duration)) == "type"`, enquanto valores
construídos continuam com `type_name() == "duration"`. A representação interna
da entidade e suas operações não mudam.
