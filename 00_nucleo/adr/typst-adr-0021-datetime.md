# ⚖️ ADR-0021: `time` crate → `[l1_allowed_external]` e `Datetime` real

**Status**: `IMPLEMENTADO`
**Data**: 2026-03-27

---

## Contexto

O stub actual em `01_core/src/entities/world_types.rs` é:

```rust
pub struct Datetime {
    pub year:  i32,
    pub month: u8,
    pub day:   u8,
}
```

`world.today()` retorna `None` em `SystemWorld`. `Datetime` no original
(`typst-library/foundations/datetime.rs`) expõe aritmética de datas
ao utilizador Typst — adição e subtracção de durações, cálculo de dia
da semana, formatação. É lógica de domínio pura que pertence a L1.

A crate `time` implementa esta aritmética correctamente para o
calendário gregoriano, incluindo anos bissextos e meses com número
variável de dias.

---

## Análise de pureza

| Propriedade | Estado |
|-------------|--------|
| Zero I/O | ✓ — aritmética pura sobre valores |
| Zero estado global mutável | ✓ |
| Determinismo total | ✓ |
| `time::OffsetDateTime::now_utc()` | ✗ — I/O do relógio; **não entra em L1** |

A obtenção da data actual fica em L3. L1 usa apenas os tipos de valor:
`time::Date`, `time::Time`, `time::Duration`, `time::Month`, `time::Weekday`.

---

## Decisão

`time` é adicionado a `[l1_allowed_external]`:

```toml
[l1_allowed_external]
rust = [
    "thiserror",
    "comemo",
    "unicode_ident",
    "unicode_math_class",
    "unicode_script",
    "unicode_segmentation",
    "rustc_hash",
    "time",  # ADR-0021 — aritmética de datas; time::now() não entra em L1
]
```

`Datetime` em L1 substitui o stub por um wrapper sobre `time::Date`
e `time::Time` (opcional):

```rust
/// Data e hora para o método today() de World e para uso em Typst.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Datetime {
    date: time::Date,
    time: Option<time::Time>,
}

impl Datetime {
    pub fn new_date(year: i32, month: u8, day: u8) -> Option<Self>;
    pub fn new_datetime(year: i32, month: u8, day: u8,
                        hour: u8, minute: u8, second: u8) -> Option<Self>;
    pub fn year(&self)    -> i32;
    pub fn month(&self)   -> u8;
    pub fn day(&self)     -> u8;
    pub fn hour(&self)    -> Option<u8>;
    pub fn minute(&self)  -> Option<u8>;
    pub fn second(&self)  -> Option<u8>;
    pub fn weekday(&self) -> u8;  // 1=Mon … 7=Sun (ISO 8601)
}
```

`world.today()` em L3 usa `time::OffsetDateTime::now_utc()`:

```rust
fn today(&self, offset: Option<i64>) -> Option<Datetime> {
    use time::OffsetDateTime;
    let now = OffsetDateTime::now_utc();
    let now = match offset {
        Some(h) => now + time::Duration::hours(h),
        None    => now,
    };
    Datetime::new_date(
        now.year(),
        now.month() as u8,
        now.day(),
    )
}
```

---

## O que esta ADR não decide

- Aritmética `Datetime + Duration` exposta ao utilizador Typst —
  adiado para quando `Value` real migrar.

---

## Prompts afectados

| Prompt | Natureza da mudança |
|--------|---------------------|
| `00_nucleo/prompts/world-types.md` | Actualizar — Datetime real com `time` |

---

## Consequências

**Positivas**: `world.today()` passa a retornar data real; aritmética
de datas disponível em L1 quando `Value` migrar.

**Negativas**: Nova crate em `[l1_allowed_external]`.

**Neutras**: `time::now()` explicitamente excluído de L1.

---

## Referências

- ADR-0001 — critérios para `[l1_allowed_external]`
- `time` crate: https://github.com/time-rs/time
