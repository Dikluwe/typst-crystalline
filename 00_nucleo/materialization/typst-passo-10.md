# Passo 10 — FontBook real em L3

## Estado actual antes de começar

Ler antes de começar:
- `00_nucleo/adr/typst-adr-0022-fontbook.md`
- `00_nucleo/adr/typst-adr-0021-datetime.md` (time crate autorizado — executar também)
- `00_nucleo/adr/typst-adr-0019-ttf-rustybuzz.md`
- `03_infra/src/fonts.rs` — FontSlot já existente do Passo 8

Pré-condição: `cargo test` — 179 testes (159 L1 + 20 L3), zero violations.

Este passo tem duas tarefas sequencialmente dependentes:
1. **Datetime real** — rápida, executa ADR-0021 (time crate já decidido)
2. **FontBook real** — diagnóstico determina se vai para L1 ou L3

---

## Tarefa 0 — Executar ADR-0021 (Datetime + time crate)

Esta tarefa não tem diagnóstico — a decisão está tomada.

### 0a — crystalline.toml

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

### 0b — Workspace Cargo.toml

```toml
[workspace.dependencies]
# adicionar:
time = { version = "0.3", features = ["macros"] }
```

Verificar a versão actual usada pelo `lab/typst-original`:
```bash
grep "^time " lab/typst-original/Cargo.toml
# ou
grep "time" lab/typst-original/crates/typst-library/Cargo.toml | head -5
```

Usar a mesma versão major do original para evitar conflitos no workspace.

### 0c — Datetime real em `world_types.rs`

**Ficheiro**: `01_core/src/entities/world_types.rs`

Substituir o stub:
```rust
// Remover:
pub struct Datetime {
    pub year:  i32,
    pub month: u8,
    pub day:   u8,
}

// Substituir por:
/// Data e hora para o método today() de World.
/// Wrapper sobre time::Date + Option<time::Time> — ADR-0021.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Datetime {
    date: time::Date,
    time: Option<time::Time>,
}

impl Datetime {
    /// Cria Datetime a partir de componentes de data.
    /// Retorna None se a data não for válida no calendário gregoriano.
    pub fn new_date(year: i32, month: u8, day: u8) -> Option<Self> {
        let month = time::Month::try_from(month).ok()?;
        let date  = time::Date::from_calendar_date(year, month, day).ok()?;
        Some(Self { date, time: None })
    }

    /// Cria Datetime com componentes de hora.
    pub fn new_datetime(
        year: i32, month: u8, day: u8,
        hour: u8, minute: u8, second: u8,
    ) -> Option<Self> {
        let month = time::Month::try_from(month).ok()?;
        let date  = time::Date::from_calendar_date(year, month, day).ok()?;
        let time  = time::Time::from_hms(hour, minute, second).ok()?;
        Some(Self { date, time: Some(time) })
    }

    pub fn year(&self)    -> i32        { self.date.year() }
    pub fn month(&self)   -> u8         { self.date.month() as u8 }
    pub fn day(&self)     -> u8         { self.date.day() }
    pub fn hour(&self)    -> Option<u8> { self.time.map(|t| t.hour()) }
    pub fn minute(&self)  -> Option<u8> { self.time.map(|t| t.minute()) }
    pub fn second(&self)  -> Option<u8> { self.time.map(|t| t.second()) }
    /// Dia da semana: 1=Segunda … 7=Domingo (ISO 8601).
    pub fn weekday(&self) -> u8 {
        self.date.weekday().number_from_monday()
    }
}
```

### 0d — world.today() em SystemWorld

**Ficheiro**: `03_infra/src/world.rs`

```rust
fn today(&self, offset: Option<i64>) -> Option<Datetime> {
    use time::OffsetDateTime;

    let now = OffsetDateTime::now_utc();
    let now = match offset {
        Some(h) => now + time::Duration::hours(h),
        None    => now,
    };

    Datetime::new_date(now.year(), now.month() as u8, now.day())
}
```

Adicionar `time` a `03_infra/Cargo.toml`:
```toml
time = { workspace = true }
```

### 0e — Testes de Datetime

```rust
// Em 01_core/src/entities/world_types.rs, secção #[cfg(test)]

#[test]
fn datetime_date_valida() {
    let d = Datetime::new_date(2026, 3, 27).unwrap();
    assert_eq!(d.year(), 2026);
    assert_eq!(d.month(), 3);
    assert_eq!(d.day(), 27);
    assert!(d.hour().is_none());
}

#[test]
fn datetime_month_invalido() {
    assert!(Datetime::new_date(2026, 0, 1).is_none());
    assert!(Datetime::new_date(2026, 13, 1).is_none());
}

#[test]
fn datetime_day_invalido_para_mes() {
    // Fevereiro 2026 tem 28 dias
    assert!(Datetime::new_date(2026, 2, 29).is_none());
    // Mas 2024 foi bissexto
    assert!(Datetime::new_date(2024, 2, 29).is_some());
}

#[test]
fn datetime_com_hora() {
    let d = Datetime::new_datetime(2026, 3, 27, 14, 30, 0).unwrap();
    assert_eq!(d.hour(), Some(14));
    assert_eq!(d.minute(), Some(30));
    assert_eq!(d.second(), Some(0));
}

#[test]
fn datetime_weekday_segunda() {
    // 2026-03-23 foi segunda-feira
    let d = Datetime::new_date(2026, 3, 23).unwrap();
    assert_eq!(d.weekday(), 1); // ISO 8601: Segunda = 1
}

#[test]
fn datetime_roundtrip() {
    let d = Datetime::new_date(2026, 12, 31).unwrap();
    assert_eq!(d.year(), 2026);
    assert_eq!(d.month(), 12);
    assert_eq!(d.day(), 31);
}
```

### 0f — Verificação intermédia

```bash
cargo test -p typst-core
cargo build
crystalline-lint .
# ✓ Zero violations — time autorizado via ADR-0021
```

Se V14 disparar para `time`: verificar que `"time"` está no
`crystalline.toml` com o mesmo nome que o crate Rust usa
(`time`, não `time_rs`).

---

## Tarefa 1 — Diagnósticos de FontBook

**Parar aqui. Reportar output antes de continuar para a Tarefa 2.**

```bash
# Estrutura de FontBook e FontInfo
grep -n "^pub struct\|^pub enum\|^pub fn\|^impl " \
  lab/typst-original/crates/typst-library/src/text/font/book.rs \
  | head -50

# Campos de FontInfo — são primitivos?
grep -A 30 "pub struct FontInfo" \
  lab/typst-original/crates/typst-library/src/text/font/book.rs

# Dependências externas de book.rs
grep "^use\|^extern" \
  lab/typst-original/crates/typst-library/src/text/font/book.rs \
  | grep -v "crate::\|super::\|std::" | head -20

# Como FontBook é construído a partir de uma face ttf_parser
grep -n "push\|insert\|from_face\|ttf_parser\|Face" \
  lab/typst-original/crates/typst-library/src/text/font/book.rs \
  | head -20

# Métodos de pesquisa — select_font, fallback, etc.
grep -n "^pub fn\|^    pub fn" \
  lab/typst-original/crates/typst-library/src/text/font/book.rs \
  | head -30

# FontVariant, FontStyle, FontWeight, FontStretch — onde vivem?
grep -rn "^pub struct FontVariant\|^pub enum FontStyle\|^pub struct FontWeight" \
  lab/typst-original/crates/typst-library/src/ | head -10

# Como book() é usado pelo engine
grep -rn "world\.book()\|\.book()\." \
  lab/typst-original/crates/typst-library/src/ | head -20
```

---

## Tarefa 2 — Decisão e implementação

### Se Opção A (campos primitivos → L1)

**Criar**: `00_nucleo/prompts/entities/font-book.md`

**Criar**: `01_core/src/entities/font_book.rs`

Estrutura base (ajustar após diagnóstico):

```rust
//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/font-book.md
//! @prompt-hash <hash>
//! @layer L1
//! @updated 2026-03-27

/// Estilo de fonte.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum FontStyle { #[default] Normal, Italic, Oblique }

/// Peso de fonte: 100 (Thin) … 900 (Black). 400 = Regular, 700 = Bold.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct FontWeight(pub u16);

impl FontWeight {
    pub const THIN:        Self = Self(100);
    pub const REGULAR:     Self = Self(400);
    pub const MEDIUM:      Self = Self(500);
    pub const BOLD:        Self = Self(700);
    pub const BLACK:       Self = Self(900);

    /// Distância entre dois pesos — para selecção da fonte mais próxima.
    pub fn distance(self, other: Self) -> u16 {
        self.0.abs_diff(other.0)
    }
}

impl Default for FontWeight {
    fn default() -> Self { Self::REGULAR }
}

/// Largura de fonte: 50% (ultra-condensed) … 200% (ultra-expanded). 100% = Normal.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub struct FontStretch(pub u16);

impl FontStretch {
    pub const NORMAL: Self = Self(1000); // representado como 1/10 de %
    // Ajustar unidade conforme o que o original usa
}

/// Variante completa de fonte (estilo + peso + largura).
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub struct FontVariant {
    pub style:   FontStyle,
    pub weight:  FontWeight,
    pub stretch: FontStretch,
}

/// Flags binárias de características de uma face de fonte.
#[derive(Debug, Clone, Copy, Default)]
pub struct FontFlags {
    pub monospace: bool,
    pub serif:     bool,
}

/// Metadados de uma face de fonte.
#[derive(Debug, Clone)]
pub struct FontInfo {
    pub family:  String,
    pub variant: FontVariant,
    pub flags:   FontFlags,
}

/// Catálogo de metadados de fontes disponíveis.
/// Populado em L3 a partir de FontSlot; consultado em L1 para selecção.
pub struct FontBook {
    infos: Vec<FontInfo>,
}

impl FontBook {
    pub fn new() -> Self {
        Self { infos: Vec::new() }
    }

    pub fn push(&mut self, info: FontInfo) {
        self.infos.push(info);
    }

    pub fn infos(&self) -> &[FontInfo] {
        &self.infos
    }

    pub fn len(&self) -> usize {
        self.infos.len()
    }

    pub fn is_empty(&self) -> bool {
        self.infos.is_empty()
    }

    /// Selecciona o índice da fonte mais próxima de (family, variant).
    /// Retorna None se nenhuma fonte da família existir.
    pub fn select(&self, family: &str, variant: &FontVariant) -> Option<usize> {
        // Primeiro: família exacta
        // Depois: peso mais próximo dentro da família
        // Depois: estilo mais próximo
        // Critério de desempate: primeiro encontrado
        let candidates: Vec<usize> = self.infos.iter()
            .enumerate()
            .filter(|(_, info)| info.family.eq_ignore_ascii_case(family))
            .map(|(i, _)| i)
            .collect();

        if candidates.is_empty() {
            return None;
        }

        // Melhor peso
        candidates.into_iter().min_by_key(|&i| {
            let info = &self.infos[i];
            let weight_dist = info.variant.weight.distance(variant.weight);
            let style_match = if info.variant.style == variant.style { 0u32 } else { 1 };
            (weight_dist as u32, style_match)
        })
    }

    /// Itera sobre índices de todas as faces de uma família.
    pub fn select_family<'a>(
        &'a self,
        family: &'a str,
    ) -> impl Iterator<Item = usize> + 'a {
        self.infos.iter()
            .enumerate()
            .filter(move |(_, info)| info.family.eq_ignore_ascii_case(family))
            .map(|(i, _)| i)
    }
}

impl Default for FontBook {
    fn default() -> Self { Self::new() }
}
```

Adicionar a `entities/mod.rs`:
```rust
pub mod font_book;
```

Remover stub de `world_types.rs`:
```rust
// Remover:
pub struct FontBook(());
```

Actualizar `World` trait — `book()` passa a retornar o tipo real.
Se a assinatura mudar, `SystemWorld` e testes precisam de actualização.

**Criar**: `00_nucleo/prompts/entities/font-book.md` com interface
pública, critérios de verificação e decisão de arquitectura.

### Se Opção B (fica em L3)

Documentar em `01_core/DEBT.md`:
```markdown
## FontBook real — bloqueado por dependências em L3
FontInfo em `typst-library/src/text/font/book.rs` usa tipos de
`ttf_parser` nos campos — impede migração para L1.
Decisão: stub `FontBook(())` mantido.
Desbloqueia quando: FontInfo for redesenhado com campos primitivos.
Registado: 2026-03-27
```

---

## Tarefa 3 — Extracção de FontInfo em L3 (se Opção A)

**Ficheiro**: `03_infra/src/fonts.rs`

Adicionar após `FontSlot`:

```rust
use typst_core::entities::font_book::{
    FontBook, FontFlags, FontInfo, FontStretch, FontStyle, FontVariant, FontWeight,
};

/// Extrai FontInfo de bytes de fonte OpenType/TrueType.
/// Retorna None se os bytes não forem uma fonte válida.
///
/// Nota: `ttf_parser` não expõe directamente se uma fonte é serif.
/// O campo `flags.serif` fica false — pode ser melhorado com heurísticas
/// de nome de família, se necessário.
pub fn font_info_from_bytes(data: &[u8], index: u32) -> Option<FontInfo> {
    let face = ttf_parser::Face::parse(data, index).ok()?;

    // Preferir nome em inglês; fallback para qualquer idioma
    let family = face.families()
        .find(|(_, lang)| *lang == ttf_parser::Language::English_UnitedStates)
        .or_else(|| face.families().next())
        .map(|(name, _)| name.to_string())?;

    let style = if face.is_italic() {
        FontStyle::Italic
    } else if face.is_oblique() {
        FontStyle::Oblique
    } else {
        FontStyle::Normal
    };

    // ttf_parser retorna FontWeight como newtype sobre u16
    let weight = FontWeight(face.weight().to_number());

    // ttf_parser retorna Width como enum; converter para u16
    let stretch = FontStretch(face.width().to_number());

    Some(FontInfo {
        family,
        variant: FontVariant { style, weight, stretch },
        flags: FontFlags {
            monospace: face.is_monospaced(),
            serif: false,
        },
    })
}
```

Actualizar `SystemWorld::new()` para popular `FontBook`:

```rust
// Em SystemWorld::new(), após discover_fonts
let mut font_book = FontBook::new();
for slot in &font_slots {
    if let Ok(data) = std::fs::read(&slot.path) {
        if let Some(info) = font_info_from_bytes(&data, slot.index) {
            font_book.push(info);
        }
    }
}
// self.font_book = font_book;
```

**Nota**: a leitura do ficheiro aqui duplica o I/O com `FontSlot::get()`.
Para este passo, é aceitável — optimizar com cache partilhado de bytes
é trabalho futuro (provavelmente Passo 11 ou quando `Font` real migrar).

---

## Tarefa 4 — Testes de FontBook (se Opção A)

```rust
// Em 01_core/src/entities/font_book.rs #[cfg(test)]

#[test]
fn fontbook_vazio() {
    let book = FontBook::new();
    assert!(book.is_empty());
    assert!(book.select("Any", &FontVariant::default()).is_none());
}

#[test]
fn fontbook_select_exacto() {
    let mut book = FontBook::new();
    book.push(FontInfo {
        family: "Test Family".into(),
        variant: FontVariant {
            style: FontStyle::Normal,
            weight: FontWeight::REGULAR,
            stretch: FontStretch(1000),
        },
        flags: FontFlags::default(),
    });
    let idx = book.select("Test Family", &FontVariant {
        style: FontStyle::Normal,
        weight: FontWeight::REGULAR,
        stretch: FontStretch(1000),
    });
    assert_eq!(idx, Some(0));
}

#[test]
fn fontbook_select_familia_case_insensitive() {
    let mut book = FontBook::new();
    book.push(FontInfo {
        family: "Liberation Sans".into(),
        variant: FontVariant::default(),
        flags: FontFlags::default(),
    });
    assert!(book.select("liberation sans", &FontVariant::default()).is_some());
    assert!(book.select("LIBERATION SANS", &FontVariant::default()).is_some());
}

#[test]
fn fontbook_select_peso_mais_proximo() {
    let mut book = FontBook::new();
    book.push(FontInfo {
        family: "Test".into(),
        variant: FontVariant { weight: FontWeight(300), ..Default::default() },
        flags: FontFlags::default(),
    });
    book.push(FontInfo {
        family: "Test".into(),
        variant: FontVariant { weight: FontWeight(700), ..Default::default() },
        flags: FontFlags::default(),
    });
    // Pedir 400 (Regular) — mais próximo é 300 (distância 100) vs 700 (distância 300)
    let idx = book.select("Test", &FontVariant {
        weight: FontWeight::REGULAR,
        ..Default::default()
    }).unwrap();
    assert_eq!(book.infos()[idx].variant.weight, FontWeight(300));
}

#[test]
fn fontbook_select_family_iterator() {
    let mut book = FontBook::new();
    book.push(FontInfo { family: "A".into(), variant: Default::default(), flags: Default::default() });
    book.push(FontInfo { family: "B".into(), variant: Default::default(), flags: Default::default() });
    book.push(FontInfo { family: "A".into(), variant: FontVariant { weight: FontWeight(700), ..Default::default() }, flags: Default::default() });

    let a_indices: Vec<usize> = book.select_family("A").collect();
    assert_eq!(a_indices, vec![0, 2]);
}

// Em 03_infra/src/fonts.rs #[cfg(test)]

#[test]
fn font_info_bytes_invalidos() {
    assert!(font_info_from_bytes(b"not a font", 0).is_none());
}

// Teste com fixture (requer ficheiro .ttf em tests/fixtures/)
// Se o Passo 8 já incluiu uma fixture, usar essa.
#[test]
#[ignore = "requer fixture de fonte em tests/fixtures/"]
fn font_info_de_fixture() {
    let data = std::fs::read("tests/fixtures/some-font.ttf").unwrap();
    let info = font_info_from_bytes(&data, 0).unwrap();
    assert!(!info.family.is_empty());
}
```

---

## Verificação final

```bash
cargo test -p typst-core
cargo test -p typst-infra
cargo build
crystalline-lint .
crystalline-lint --fix-hashes .
crystalline-lint .
# ✓ No violations found
```

Critérios de conclusão:
- `Datetime::new_date(2026, 3, 27)` cria valor válido
- `Datetime::new_date(2026, 2, 29)` retorna None (2026 não é bissexto)
- `world.today(None)` retorna `Some(Datetime)` com ano > 2020
- Se Opção A: `FontBook::select("Liberation Sans", ...)` retorna Some após popular com fontes reais
- Se Opção A: `font_info_from_bytes(bytes_invalidos, 0)` retorna None
- Se Opção B: DEBT.md actualizado
- Zero violations
- V11 warnings para `World`/`TrackedWorld` mantêm-se em zero (já resolvidos no Passo 7)

---

## Ao terminar, reportar

- Versão do `time` crate adicionada
- Se `Datetime::new_date(2026, 2, 29)` falhou correctamente (valida calendário real)
- Opção escolhida para FontBook (A ou B) e justificação com base no diagnóstico
- Se Opção A: campos reais de `FontInfo` encontrados no diagnóstico
- Se Opção A: `FontWeight` e `FontStretch` — unidades usadas pelo original
- Número total de testes
- Zero violations confirmado

Esta informação vai para o Passo 11
(indexmap + Scope em L1 → Module real → abre caminho para eval()).
