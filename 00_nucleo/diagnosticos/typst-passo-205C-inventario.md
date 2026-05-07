# P205C — Inventário empírico

**Data**: 2026-05-07.
**Cláusula**: C1 do passo P205C.
**Pré-condição confirmada**: P205B concluído;
`SealedPositions` em L1 com `#[comemo::track]`;
`PagedDocument.extracted_positions` populated em
`Layouter::finish`; tests 1856 verdes; 0 violations.

---

## §1 C1 — Inventário (5 sub-secções)

### C1.1 — Consumers reais de `position_of`

**Status**: ⚠️ **AJUSTE NECESSÁRIO** — zero consumers
de produção.

`grep -rn "position_of" --include="*.rs" | grep -v lab/`:

| Local | Tipo de uso |
|-------|-------------|
| `01_core/src/entities/introspector.rs:70` | Trait declaration |
| `01_core/src/entities/introspector.rs:263` | Impl stub (retorna `None`) |
| `01_core/src/entities/introspector.rs:366,430` | Tests asserting `None` |
| `01_core/src/entities/sealed_positions.rs:63` | Método tracked em `SealedPositions` (P205B) |
| `01_core/src/entities/sealed_positions.rs:114,129-131` | Tests P205B |
| `01_core/src/entities/position.rs:45` | Doc comment |
| `03_infra/src/measurements.rs:50,189-191` | Wrapper `CountingIntrospector` (P204G — delegate sem consumer real) |

**Consumers de produção**: **0**.

**Tests assertando `position_of == None`**: 2
(introspector.rs:366, 430). Esses tests usam
`TagIntrospector::empty()` — sem positions injectadas.
Devem **continuar a retornar `None`** mesmo após P205C
(impl real preserva semântica empty → None).

**stdlib `here()`/`locate()` consumers**: ainda não
materializados (per P204F SKIP `here-locate.typ` —
`here()`/`locate()` não estão registadas em stdlib
cristalino).

**Outros call sites**: nenhum.

**Implicação**: ADR-0074 §"Decisão" + §"Consequências
positivas" fixaram explicitamente que F3 minimal **fecha
pendência ADR-0073 §C6a** materializando `Some(Position)`
real. Caminho C (adiar) seria contraditório com ADR
PROPOSTA — inflar é o erro oposto, mas adiar também é se
ADR fixou. Caminho A ou B prosseguem.

### C1.2 — Arquitectura `Introspector` actual

**Status**: ✅ **CONFIRMADO**.

- **Trait declaration**:
  `01_core/src/entities/introspector.rs:40` —
  `#[comemo::track] pub trait Introspector: Send + Sync`
  com 20 métodos (per P204B).
- **Impls existentes**: 1 produção (`TagIntrospector`)
  + wrappers de teste.
- **Layouter consume via**:
  `Tracked<'a, dyn Introspector + 'a>` (P204C).

### C1.3 — Pipeline pré vs pós-layout

**Status**: ✅ **CONFIRMADO**.

| Fase | Estado de `TagIntrospector` | Estado de `SealedPositions` |
|------|------------------------------|------------------------------|
| Pre-layout | Construído via `from_tags` (todas as sub-stores populated excepto `positions`) | Não existe ainda |
| Durante layout | Tracked imutavelmente; `runtime.positions` populated single-pass | Não existe ainda |
| `Layouter::finish` | Consumido (move) | **Construído** via `from_runtime(self.runtime.positions)` |
| Pós-layout | Disponível para queries (estático) | Em `PagedDocument.extracted_positions` |

**Quem precisa de `position_of` em qual fase?**
Pós-layout. Pre-layout não tem Position concreta para
oferecer (single-pass populates durante layout).

### C1.4 — Arquitectura `PagedTagIntrospector` candidata

**Status**: ⚠️ **AJUSTE NECESSÁRIO**.

- Tipo similar **não existe** em cristalino.
- Cristalino **não tem precedente** de wrappers que
  combinam introspector + dados pós-layout — o pattern
  é **ad-hoc** (P190C `LayouterRuntimeState` vive como
  field, não como wrapper sobre introspector).
- Vanilla `PagedIntrospector` é struct concreta nova
  (per A9 P205A) porque vanilla tem **múltiplos tipos de
  Introspector** (paged, html, html-paged, etc.).
  Cristalino tem **único tipo** `TagIntrospector`.
- Wrapper `PagedTagIntrospector { inner: TagIntrospector,
  positions: SealedPositions }` exigiria delegar **19
  métodos do trait** ao `inner` só para o 1 método
  especial (`position_of`). Inflação alta sem benefício
  arquitectural — vanilla precisa porque tem múltiplas
  impls; cristalino não.

### C1.5 — Arquitectura "TagIntrospector enriquecido" candidata

**Status**: ✅ **CONFIRMADO**.

- `TagIntrospector` pode ganhar field
  `pub positions: SealedPositions` sem invadir
  invariantes:
  - `Default::default()` é `SealedPositions::empty()`
    (vazio); construção pré-layout via `from_tags`
    continua a funcionar.
  - Sub-stores existentes são `pub` (per linhas 184-186);
    `positions` segue mesmo padrão.
  - `from_tags` não toca `positions` — fica com default.
- Sealing pós-layout: caller invoca
  `intr.inject_positions(doc.extracted_positions.clone())`
  (`&mut self`). Caller pós-layout não usa Tracked, então
  re-tracking invalidation não é problema.
- `SealedPositions` é `Clone+Send+Sync` — derives
  satisfeitos em `TagIntrospector` (`derive(Debug, Clone,
  Default)` mantém-se).
- `#[comemo::track]` no trait `Introspector` exige
  bounds dos params/returns dos métodos (não dos fields).
  `Option<Position>` retorno preservado — Hash impl manual
  via `to_bits()` (P204D).

---

## §2 C2 — Caminho fixado

**Decisão**: **Caminho A — `TagIntrospector` enriquecido**.

Justificação:

1. **C1.4** confirmou que cristalino tem **único impl**
   `Introspector`. Wrapper Caminho B exigiria 19 métodos
   delegate só para 1 especial — inflação sem benefício.
   Vanilla precisa porque tem múltiplas impls (paged/html/...);
   cristalino não.
2. **C1.5** confirmou que enriquecimento é cirúrgico — 1
   field opcional + 1 método de injecção + 1 line muda em
   `position_of` impl.
3. **C1.1** mostrou zero consumers de produção; tests
   E2E novos exercitam o caminho (3 unit + 1 E2E
   pipeline). Caminho C (adiar) contradiria ADR-0074 que
   fixou explicitamente que F3 minimal fecha pendência
   §C6a.
4. **Coerência arquitectural cristalina**: P190C
   estabeleceu pattern "Layouter-runtime → struct
   dedicada (`LayouterRuntimeState`)" sem criar wrappers
   sobre `TagIntrospector`. Caminho A respeita esse
   pattern adicionando `positions` ao `TagIntrospector`
   directamente.

C2 fixa **uma**: **Caminho A**.

---

## §3 C3 — Implementação literal

```rust
// 01_core/src/entities/introspector.rs

#[derive(Debug, Clone, Default)]
pub struct TagIntrospector {
    // ... 9 sub-stores existentes ...

    /// **P205C (F3)** — sub-store sealed Location → Position
    /// injectado pós-layout via `inject_positions`.
    pub positions: SealedPositions,
}

impl TagIntrospector {
    pub fn empty() -> Self { Self::default() }

    /// **P205C (F3)** — Injecta sub-store sealed produzido por
    /// Layouter::finish (PagedDocument.extracted_positions).
    pub fn inject_positions(&mut self, sealed: SealedPositions) {
        self.positions = sealed;
    }
}

impl Introspector for TagIntrospector {
    fn position_of(&self, location: Location) -> Option<Position> {
        // P205C (F3): impl real per ADR-0074. Pre-injecção
        // (default empty), devolve None — comportamento
        // P204D §C6a preservado.
        self.positions.position_of(location)
    }
    // ... outros 19 métodos inalterados ...
}
```

---

## §4 C4–C5 — Sealing point + migração

### C4 — Sealing point + injecção

**Sealing point**: já existe em `Layouter::finish` (P205B
construiu `doc.extracted_positions`).

**Injecção**: caller pós-layout invoca
`intr.inject_positions(doc.extracted_positions.clone())`
manualmente. **Não há wiring automático** em
`pub fn layout` — `pub fn layout` consome o introspector
binding original e não tem acesso ao caller; injecção
fica responsabilidade do caller que precisa do impl
real. Tests E2E demonstram o pattern.

Decisão: não inflar `pub fn layout` para devolver
`(PagedDocument, TagIntrospector enriquecido)` — caller
controla. Pattern análogo ao do `extracted_label_pages`
field (P63 + P190C): consumers que precisam fazem
extract manual.

### C5 — Migração de consumers

**Lista**:

- 2 tests stub asserting `position_of == None` em
  `introspector.rs::tests` (linhas 408, 472):
  - **Mantidos** — usam `TagIntrospector::empty()`
    sem injecção. Comportamento `None` preservado per
    semântica empty.
  - Comentário actualizado para clarificar a semântica
    P205C (default empty → None).
- 4 tests novos:
  - 3 unit (introspector.rs): `position_of` pre-injecção,
    inject + lookup real, re-inject (idempotência).
  - 1 E2E (layout/tests.rs): pipeline completo
    layout → seal → inject → query.
- Stdlib `here()`/`locate()`: out of scope (não
  materializados; per P204F SKIP).

---

## §5 Decisões durante a leitura

### D1 — Caminho A sobre B (pragmatismo cristalino)

C1.4 mostrou que cristalino tem único impl `Introspector`;
wrapper vanilla-style seria inflação (19 delegates só
para 1 especial). Caminho A é cirúrgico e respeita o
pattern P190C de "field directo em vez de wrapper
externo".

### D2 — Field directo (sem `Option`)

`pub positions: SealedPositions` em vez de
`pub positions: Option<SealedPositions>`. `Default::default()`
é `SealedPositions::empty()` — semanticamente equivalente
a `None` para lookup (devolve None para qualquer
location). Mais leve sintacticamente; sem ramo
`as_ref()?` no impl.

### D3 — Caller controla injecção (sem wiring automático)

`pub fn layout` mantém assinatura
`fn(&Content) -> PagedDocument` (per P190I).
Caller pós-layout invoca `intr.inject_positions(...)`
manualmente. Pattern análogo ao de
`PagedDocument.extracted_label_pages` (Passo 63):
infraestrutura disponível, caller decide se consume.

Vantagem: zero impacto em consumers actuais que não
precisam de Position; flexibilidade para futuros stdlib
`here()`/`locate()` consumir.

### D4 — Tests existentes preservam semântica

2 tests existentes (`empty_devolve_vazio_em_todos_os_queries`
e `populado_responde_correctamente`) asseram `position_of(loc)
== None`. Esses tests usam introspectors empty/non-injected;
P205C preserva o comportamento `None` para esses casos.
Apenas comentário actualizado para clarificar a nova
semântica.

### D5 — 4 tests novos (3 unit + 1 E2E)

Spec C6 pediu 2-4 tests E2E. Implementei 3 unit
substantivos (cobertura granular dos novos branches
inject_positions + lookup real + re-inject) + 1 E2E que
exercita pipeline completo (layout → finish → inject →
query). Cobertura mais densa que o mínimo; trivial em
volume (~70 LOC).

### D6 — `position_of` chama método raw (não tracked)

`self.positions.position_of(location)` chama o método
`&self -> Option<Position>` directamente, **não** via
`.track()`. Razão: este impl roda dentro de `Introspector::position_of`
que já é tracked a nível do trait (P204B); re-tracking
interno seria recursivo e desnecessário. O método raw
existe (gerado pelo macro `#[comemo::track] impl` que
preserva ambos `&self` e Tracked variants).
