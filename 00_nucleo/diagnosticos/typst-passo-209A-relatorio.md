# Relatório do passo P209A — Diagnóstico-primeiro Selector enum extension

**Data**: 2026-05-12.
**Spec**: `00_nucleo/materialization/typst-passo-209A.md`.
**Tipo**: diagnóstico-primeiro reduzido (zero código tocado).
**Magnitude planeada**: S-M (~45 min). **Magnitude real**: S.
**Marco**: M9c (Bloco VI — Selector extensions).

---

## §1 O que foi auditado

Mapeado empíricamente o gap entre `Selector` cristalino actual
(`Kind` only — P175 minimal) e o target M9c per Q-decisões
fixadas pelo humano em P207A C10: materializar `Label`, `And`,
`Or`, `Regex`, `Location` (5 variants novos); adiar `Where`
(Q2=γ); excluir `Before`/`After` do roadmap. Zero código tocado;
1 output (este).

---

## §2 Auditoria A1–A5

### A1 — `Selector` cristalino actual (CONFIRMADO)

`01_core/src/entities/selector.rs` (53L, P175):

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Selector {
    Kind(ElementKind),
}
```

Derives: `Hash` automático (todos os fields são Hash). Único
consumer production: `Introspector::query` arm. L0
`00_nucleo/prompts/entities/selector.md` confirma estado P175
minimal — "Refino futuro adiado para passos dedicados quando
consumers reais necessitarem". 3 tests existentes (`igualdade_estrutural`,
`kinds_distintos_sao_diferentes`, `hash_determinismo`).

### A2 — `Selector` vanilla (CONFIRMADO)

`lab/typst-original/.../foundations/selector.rs:75` define 10
variants:

```rust
pub enum Selector {
    Elem(Element, Option<SmallVec<[(u8, Value); 1]>>),
    Location(Location),
    Label(Label),
    Regex(Regex),
    Can(TypeId),
    Or(EcoVec<Self>),
    And(EcoVec<Self>),
    Before { selector: Arc<Self>, end: Arc<Self>, inclusive: bool },
    After  { selector: Arc<Self>, start: Arc<Self>, inclusive: bool },
    Within { selector: Arc<Self>, ancestor: Arc<Self> },
}
```

Derives: `Debug, Clone, PartialEq, Hash` (não `Eq`). `Regex`
não deriva `Hash` natural — vanilla provavelmente tem wrapper
`Regex` com Hash manual (a confirmar em P209-Regex).

`And`/`Or` usam `EcoVec<Self>` (composição N-ária). `Before`/
`After`/`Within` usam `Arc<Self>` (composição binária, com
flag inclusive).

### A3 — `Introspector::query` impl actual (CONFIRMADO)

`01_core/src/entities/introspector.rs:419`:

```rust
fn query(&self, selector: &Selector) -> Vec<Location> {
    match selector {
        Selector::Kind(kind) => self.query_by_kind(*kind),
    }
}
```

Match exhaustive sobre 1 variant. **Extensão mínima esperada**:
adicionar 5 arms:

- `Label(l)` → `query_by_label(l).into_iter().collect()`.
- `Location(loc)` → `vec![*loc]` (trivial).
- `And(sels)` → intersecção de `sels.iter().map(query)`.
- `Or(sels)` → união de `sels.iter().map(query)`.
- `Regex(r)` → **N/A** — Regex match-se contra Content text,
  não contra Locations. Cristalino single-pass não tem
  Content texto durante query phase; Regex é stub `vec![]`
  ou `unimplemented!()` para Locations. Documenta divergência.

### A4 — Regex dependência (DIVERGÊNCIA)

`01_core/Cargo.toml` deps actuais: `thiserror`, `comemo`,
`rustc-hash`, `unicode-*` (4), `time`, `indexmap`, `ecow`,
`hypher`. **Sem `regex` ou `regex-lite`**.

`crystalline.toml:64` `l1_allowed_external.rust` allowlist:

```
thiserror, comemo, unicode_ident, unicode_math_class,
unicode_script, unicode_segmentation, rustc_hash, time,
indexmap, ecow, hypher
```

11 entries; `regex`/`regex-lite` não estão. Adição requer
**ADR nova** (per pattern ADR-0007/0010-0013/0018/0021/0023/0024/0057).

Comparação:
- **`regex` 1.x**: full Unicode (UCD tables embebidos);
  binário ~1.5 MB; pattern Rust standard; sem subset
  surprises.
- **`regex-lite`**: subset ASCII + limited Unicode;
  binário ~200 KB; menos ergonómico para usuários Typst
  que esperam paridade vanilla.

Custo ADR adicional: ~30min documental.

### A5 — Stdlib args API (PARCIAL)

`native_query("heading")` e `native_locate("heading")` (P175 +
P208C) aceitam apenas `[Value::Str(kind_str)]` →
`ElementKind::from_name`. Para os 5 variants novos:

| Variant | Input proposto | Mecanismo |
|---------|---------------|-----------|
| `Label(Label)` | `Value::Str("<name>")` | parse `<label>` syntax ou string directa |
| `Location(Location)` | `Value::Location(loc)` | já existe (P179) — type dispatch |
| `Regex(Regex)` | `Value::Str("pattern")` | constructor `Regex::new(pattern)` |
| `And(EcoVec<Self>)` | **Rust API only** (Opção c) | sem expressão `.typ` em P209 |
| `Or(EcoVec<Self>)` | **Rust API only** (Opção c) | sem expressão `.typ` em P209 |

`native_query`/`native_locate` ficam com **dispatch por
`Value` variant**: `Str(s)` se parece kind → Kind; se
começa por `<` → Label; outros casos → Regex (se feature
activa). `Location(loc)` → Selector::Location. `And`/`Or`
ficam para tests Rust e consumers Rust internos
(`Selector::Before/After` futuro re-abertos beneficiariam,
mas estão fora M9c).

Vanilla tem constructor func `selector(target: Selector)
-> Selector` + métodos `.or()`/`.and()`. Cristalino adiar
estes para passo dedicado se consumer real emergir
(pattern P208B/D "Caminho 1 anti-inflação").

---

## §3 Decisões C1–C5

### C1 — Estrutura dos 5 variants

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Selector {
    Kind(ElementKind),                       // existente
    Label(Label),                            // P209B
    Location(Location),                      // P209B
    And(EcoVec<Selector>),                   // P209C
    Or(EcoVec<Selector>),                    // P209C
    Regex(crate::entities::regex::Regex),    // P209D
}
```

`EcoVec<Selector>` para composição N-ária (paridade vanilla).
`Label` e `Location` reusam tipos L1 existentes. `Regex` é
**wrapper L1** sobre `regex::Regex` crate, com `Hash` +
`PartialEq` + `Eq` manual (pattern string como key). Wrapper
isola dep em 1 ponto + permite divergência arquitectónica
se preciso (e.g. parser cristalino se Regex pattern for
limitado).

### C2 — Dep regex L1: **Caminho A — `regex` full**

Justificação: paridade Rust standard; usuários Typst esperam
Unicode regex. Binário maior (~1.3 MB extra) é aceitável
per ADR-0010-0013 (Unicode tables já presentes para outros
usos). `regex-lite` adicionaria divergência sem benefício
proporcional.

**ADR-0077 nova proposta**: "regex 1.x autorizado em L1
para `Selector::Regex`". Estrutura paralela a ADR-0023
(indexmap), ADR-0024 (ecow). Magnitude: S (~30min documental).

C2 fixa **Caminho A** + **ADR-0077 propor em P209D**.

### C3 — Stdlib args API

- `Label`: `Value::Str("<name>")` — parse leading `<` +
  trailing `>`. Falha se ausente.
- `Location`: `Value::Location(loc)` — dispatch por type.
- `Regex`: API exposta como `regex(pattern)` constructor
  func stdlib (paralelo a vanilla `regex(...)`); produz
  novo `Value::Selector(...)` ou similar. **Decisão fixada
  em P209D** quando Regex variant materializar.
- `And`/`Or`: **Opção (c) — Rust API only**. Sem expressão
  `.typ`. Tests unit reusam construção directa
  (`Selector::And(EcoVec::from_iter(...))`). Consumer
  stdlib futuro (se emergir) adiciona constructor func
  em sub-passo dedicado pós-P209.

`native_query`/`native_locate` estendidos com type
dispatch em P209B+:

```text
match &args.items[..] {
    [Value::Str(s)] if s.starts_with('<') => Selector::Label(...),
    [Value::Str(s)] => Selector::Kind(ElementKind::from_name(s)?),
    [Value::Location(l)] => Selector::Location(*l),
    [...] => err,
}
```

### C4 — Plano P209B+: **Caminho 2 — 5 sub-passos**

Critério: magnitude balanceada + ADR como passo formal.

- **P209B** (S ~45min) — `Label` + `Location` materialização
  + query arms + native_query/locate dispatch + tests.
- **P209C** (M ~1-1.5h) — `And` + `Or` materialização +
  query arms (intersecção/união) + tests Rust API.
- **P209D** (M ~1-1.5h) — `Regex` wrapper L1 + ADR-0077
  PROPOSTO + dep `regex` adicionada + arm `Regex` em
  query (provavelmente stub `vec![]` ou `unimplemented!`
  documentado) + native `regex(pattern)` constructor + tests.
- **P209E** (S ~30min) — encerramento série + blueprint
  §3.0sexies + ADR-0077 anotada (PROPOSTO mantém-se até
  P212).

### C5 — Magnitude agregada P209: **M (~4-5h)**

Distribuição:
- P209A: S (~45min) — concluído aqui.
- P209B: S (~45min).
- P209C: M (~1-1.5h).
- P209D: M (~1-1.5h) — Regex + ADR-0077 nova.
- P209E: S (~30min) encerramento documental.

Total: **~4-5h** série inteira (5 sub-passos com A já
concluído).

---

## §4 Magnitude agregada P209

**M (~4-5h estimado)**. Hipótese provável confirmada per
spec §7.

Caveat: P209D pode revelar custo Regex superior ao
estimado se ADR-0077 fricções com allowlist (Unicode
features default vs gated). Sub-passo `P209D.div-N` ou
`P209D-2` pode emergir.

---

## §5 Plano P209B-E (resumo executável)

| Sub-passo | Tipo | Magnitude | Output principal |
|-----------|------|-----------|------------------|
| P209B | Variants triviais | S | +`Label(Label)` + `Location(Location)` variants; query arms; stdlib dispatch; 4-6 tests. |
| P209C | Composição N-ária | M | +`And(EcoVec<Self>)` + `Or(EcoVec<Self>)`; query arms (intersecção/união); 4-6 tests Rust API. |
| P209D | Regex + dep + ADR | M | +`Regex` wrapper L1 (`Hash`+`Eq` manual); +`regex` em allowlist + `01_core/Cargo.toml`; +`native_regex(pattern)`; ADR-0077 PROPOSTO; arm Regex em query (stub doc); 4-6 tests. |
| P209E | Encerramento série | S | ADR-0076 série anotada; blueprint §3.0sexies; relatório resumo. Decisão Caminho 1 (puro) vs 2 fixada em P209E próprio C1. |

**Pré-condições mantidas**:
- Trait `Introspector` mantém 26 métodos (P209 não estende
  trait — apenas variants + query arm internal).
- Regra empírica P207B §5 **não acionada** (zero novos
  trait methods).
- Selector enum fica com 6 variants pós-P209D (1 existente
  + 5 novos).

---

## §6 Próximo sub-passo

**P209B** — `Selector::Label` + `Selector::Location`
materialização. Pré-condição cumprida: P209A diagnóstico
fechado; C1-C5 fixados sem condicionais.

Trabalho concreto P209B (preview):

- L0 `entities/selector.md` — Hash impl preservado
  (derive funciona porque Label/Location são Hash);
  histórico actualizado.
- L1 `entities/selector.rs` — 2 variants novos.
- L1 `entities/introspector.rs` query arm — 2 arms novos.
- L1 stdlib `foundations.rs` — type dispatch em
  `native_query` + `native_locate` para `Value::Location`
  e `Value::Str("<label>")`.
- 4-6 tests dedicados.

ADR-0076 mantém `PROPOSTO` até P212. Estado M9c: 2 séries
fechadas + diagnóstico P209A concluído.
