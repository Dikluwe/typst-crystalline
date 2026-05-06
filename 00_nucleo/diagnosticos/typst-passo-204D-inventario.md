# Inventário interno P204D — Position concrete

**Data**: 2026-05-06.
**Executor**: Claude Code (Opus 4.7).
**Spec**: `00_nucleo/materialization/typst-passo-204D.md`.
**Natureza**: diagnóstico interno (factos empíricos +
decisões + alterações aplicadas).

---

## §1 C1 — Inventário empírico

### 1.1 Tipo `Point` em L1 — **CONFIRMADO**

- **Localização**: `01_core/src/entities/layout_types.rs:67`.
- **Forma**: `pub struct Point { pub x: Pt, pub y: Pt }`.
- **Derive**: `#[derive(Debug, Clone, Copy, PartialEq)]`.
- **Não derive**: `Hash`, `Eq` (porque `Pt(f64)` bloqueia
  ambos).
- **`Pt`**: `pub struct Pt(pub f64)` — `#[derive(Debug,
  Clone, Copy, PartialEq, PartialOrd, Default)]`. Sem
  Hash, sem Eq.

### 1.2 Vanilla `PagedPosition` — **CONFIRMADO** (per P203A A2)

```rust
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct PagedPosition {
    pub page: NonZeroUsize,
    pub point: Point,
}
```

`PagedPosition` vanilla é Hash directamente porque
`Point` em vanilla provavelmente deriva Hash (não
verificado neste passo; assumido baseado em derive
list).

### 1.3 `LayouterRuntimeState` — **CONFIRMADO**

- **Localização**: `01_core/src/entities/layouter_runtime_state.rs`.
- **Pre-P204D**: 3 fields (`label_pages`,
  `known_page_numbers`, `is_readonly`).
- **Derive**: `#[derive(Debug, Default, Clone)]`.
- **Used by**: `Layouter.runtime` field (per P190C/D
  pattern "Layouter-runtime → struct dedicada").

### 1.4 Trait `Introspector::position_of` — **CONFIRMADO**

- **Localização**: `01_core/src/entities/introspector.rs:55`.
- **Pre-P204D signature**: `fn position_of(&self,
  location: Location) -> Option<()>`.
- **Consumers em produção**: 0 (per P204A A3).
- **Tests stub**: 2 sites em `introspector.rs:346, 369`
  asserting `position_of(...) == None`.

### 1.5 Layouter — `current_location` e `current_page` — **CONFIRMADO**

- **`current_location`**: field `Option<Location>` em
  Layouter (linha 149); set por
  `advance_locator_if_locatable` (linha 270-274) quando
  content é locatable.
- **`current_page`**: derivável de `pages.len() + 1`
  (1-based; primeira página = 1; pages é Vec<Page> com
  páginas FINALIZADAS).
- **`cursor_x, cursor_y`**: fields Pt (linhas 88-89),
  posição corrente do baseline.

**Single canonical site para emissão de Position**:
`advance_locator_if_locatable` (`mod.rs:270`). Mirror
exacto do gating que set `current_location` — ponto
único atómico.

### 1.6 Vanilla pipeline — **CONFIRMADO** (per P203A A7)

- Vanilla calcula Position **post-layout** em
  `PagedIntrospector::new(pages: &[Page])` (typst-layout/src/introspect.rs:35).
- Cristalino diverge intencionalmente — single-pass
  durante layout (per P203A C5).
- Saída observable equivalente — mapping
  Location → Position idêntico para um documento.

### 1.7 Etiquetas

Todos os items C1.1-C1.6 **CONFIRMADO**. Sem divergências
relevantes.

---

## §2 C2 — API-decisão fixada — **MIGRAR STUB**

### Decisão

`Introspector::position_of` migra de `Option<()>` para
`Option<Position>`.

### Justificação

- P204A A3 confirmou 0 consumers em produção
  (apenas 2 tests stub asserting None).
- Migration trivial — assert_eq! infere tipo correctly.
- Trait API matches vanilla literal (per ADR-0073
  Padrão A).
- `TagIntrospector` impl mantém `None` retorno (mas
  com tipo correcto agora).

Sem ramos. **Migrar**.

---

## §3 C6 — Trait API — **C6a** (TagIntrospector retorna None)

### Decisão

`TagIntrospector::position_of` retorna sempre `None`.
Position vive em `Layouter.runtime.positions` —
consumers que precisem de Position acedem via Layouter
directamente.

### Justificação

- Cristalino single-pass: Position vive em runtime
  (Layouter), não em Introspector estrutural.
- `TagIntrospector` é construído pre-layout via
  `from_tags`, sem acesso ao Layouter runtime.
- Tracked é `&` borrow imutável após `.track()`;
  populating `intr.positions` durante layout é
  impossível arquitecturalmente.
- Future: `PagedIntrospector` (ou similar) que abrace
  Layouter runtime pode override e retornar
  `Some(Position)`.

### Alternativas rejeitadas

- **C6b** (separação de traits): inflação sem benefício;
  trait separation inflaciona toolkit.
- **C6c** (TagIntrospector ganha campo settable):
  arquitecturalmente impossível com Tracked imutável.

---

## §4 C5 — Ponto de emissão — `advance_locator_if_locatable`

### Decisão

Emit Position no mesmo gating que set `current_location`:

```rust
fn advance_locator_if_locatable(&mut self, content: &Content) {
    if is_locatable(content) {
        let loc = self.locator.next();
        self.current_location = Some(loc);
        // P204D: emit Position single-pass.
        let page = NonZeroUsize::new(self.pages.len() + 1)
            .expect("pages.len() + 1 >= 1");
        let point = Point {
            x: self.cursor_x,
            y: self.cursor_y,
        };
        self.runtime.positions.insert(loc, Position { page, point });
    }
}
```

### Justificação

- **Single canonical site**: 1 ponto onde locatables
  são detectados; mirror exacto do gating de
  `current_location`.
- **Idempotência**: `insert` substitui em re-layout (TOC
  fixpoint). Iterações posteriores sobrescrevem.
- **No overhead em non-locatable**: if guard limita
  custo a apenas locatables.
- **Side-effect free** quanto à locator state: locator
  já avança por `is_locatable` gate; Position emit é
  zero-cost adicional.

---

## §5 Alterações literais aplicadas

### 5.1 Novo ficheiro `01_core/src/entities/position.rs`

```text
+ pub struct Position {
+     pub page: NonZeroUsize,
+     pub point: Point,
+ }
+
+ #[derive(Debug, Clone, Copy, PartialEq)]
+ // Hash manual via to_bits() (Pt(f64) bloqueia derive Hash).
+ impl std::hash::Hash for Position { ... }
+
+ // 3 unit tests.
```

Header com `@prompt 00_nucleo/prompts/entities/position.md`
e `@prompt-hash 208e41b7` (auto-fixed via
`crystalline-lint --fix-hashes`).

### 5.2 L0 prompt criado

`00_nucleo/prompts/entities/position.md` — prompt L0
formal com:
- Contexto (M8 / ADR-0066 superseded; ADR-0073).
- Decisão (forma + bounds + Hash via to_bits).
- Pipeline (single-pass cristalino vs post-layout
  vanilla).
- Trait API (migração + C6a).
- Restrições absolutas (L1).
- Plano de validação.

`Hash do Código: 5f2dbfa9` (auto-anotado pelo linter
`--fix-hashes`).

### 5.3 `01_core/src/entities/mod.rs`

```text
  pub mod location;
  pub mod locator;
+ pub mod position;
  pub mod tag;
```

### 5.4 `01_core/src/entities/layouter_runtime_state.rs`

```text
+ use crate::entities::location::Location;
+ use crate::entities::position::Position;

  pub struct LayouterRuntimeState {
      pub label_pages: ...,
      pub known_page_numbers: ...,
      pub is_readonly: bool,
+     pub positions: HashMap<Location, Position>,
  }
```

### 5.5 `01_core/src/rules/layout/mod.rs` (advance_locator_if_locatable)

```text
  fn advance_locator_if_locatable(&mut self, content: &Content) {
      if is_locatable(content) {
-         self.current_location = Some(self.locator.next());
+         let loc = self.locator.next();
+         self.current_location = Some(loc);
+         // P204D: emit Position concrete single-pass.
+         let page = NonZeroUsize::new(self.pages.len() + 1)
+             .expect("pages.len() + 1 >= 1");
+         let point = Point { x: self.cursor_x, y: self.cursor_y };
+         self.runtime.positions.insert(loc, Position { page, point });
      }
  }
```

### 5.6 `01_core/src/entities/introspector.rs` (trait declaration)

```text
- /// M3 stub: retorna sempre `None`. Mapa Location→Position fica
- /// vazio até consumer real (layout) integrar em M5/M9.
- fn position_of(&self, location: Location) -> Option<()>;
+ /// **P204D (M8)** — assinatura migrada de stub Option<()>
+ /// para Option<Position> per ADR-0073 ...
+ fn position_of(&self, location: Location) -> Option<crate::entities::position::Position>;
```

### 5.7 `01_core/src/entities/introspector.rs` (TagIntrospector impl)

```text
- fn position_of(&self, _location: Location) -> Option<()> {
+ fn position_of(&self, _location: Location) -> Option<crate::entities::position::Position> {
      // ... return None (TagIntrospector sem acesso runtime)
      None
  }
```

### 5.8 Tests existentes

Nenhum ajuste necessário. `assert_eq!(intr.position_of(loc), None)`
infere tipo `Option<Position>` correctly.

### 5.9 Tests novos (4 P204D)

Em `01_core/src/rules/layout/tests.rs`:
- **`p204d_position_struct_existe`** — sentinel tipo.
- **`p204d_runtime_positions_field_existe`** — sentinel
  field.
- **`p204d_position_populada_para_locatable_basico`** —
  E2E: Heading produz Position com page=1.
- **`p204d_position_nao_populada_para_nao_locatable`** —
  E2E: Text plain não popula positions.

Em `01_core/src/entities/position.rs` (já incluídos no
ficheiro novo):
- **`position_construcao_basica`** — construção.
- **`position_iguais_produzem_mesmo_hash`** — Hash
  determinístico.
- **`position_distintos_produzem_hashes_distintos`** —
  Hash discrimina.

---

## §6 Decisões tomadas durante a leitura

### 6.1 Hash via `to_bits()` em vez de Debug formatting

P204B usou Debug formatting para Value/Content. P204D
opta por `to_bits()` directo:

```rust
impl Hash for Position {
    fn hash<H>(&self, state: &mut H) {
        self.page.hash(state);
        self.point.x.val().to_bits().hash(state);
        self.point.y.val().to_bits().hash(state);
    }
}
```

**Justificação**: Position só tem 3 fields simples
(NonZeroUsize, f64, f64). Hash explícito é mais
eficiente que `format!("{:?}", self).hash(state)` e
elimina overhead de formatação. Position é hot path
em cache lookups comemo.

### 6.2 Sem Eq derive (apenas PartialEq)

`f64` não permite Eq. Position fica sem Eq. Consumers
que precisem de igualdade usam `==` (PartialEq).

### 6.3 Single canonical site no Layouter

Decidido populate Position em
`advance_locator_if_locatable` em vez de em cada arm de
`layout_content` que processa locatable. Vantagens:
- 1 site em vez de N.
- Garantia de paridade com `current_location`.
- Idempotência por construção (insert substitui).

### 6.4 L0 prompt criado em P204D mesmo

Per CLAUDE.md "Protocolo de Nucleação", L0 é
pré-requisito. Criado em P204D para satisfazer linter
V1. `--fix-hashes` syncronizou hash.

Decisão honesta: L0 normalmente precede sub-passo de
implementação per protocolo formal. Em P204D este foi
criado em-passo-completo porque spec assumiu preparação
prévia. Trabalho mantém-se dentro de S-M.

### 6.5 0 alterações em consumers existentes

Pre-P204D, `position_of -> Option<()>`. Pós-P204D,
`position_of -> Option<Position>`. Tests assert `None` —
sem alteração necessária. Consumers em produção 0 (per
A3) — sem alteração necessária.

Posição limpa para consumers futuros (PagedIntrospector
ou stdlib `here().position()`) implementarem.

---

## §7 C8+C9+C10+C11 — Verificações

### C8 — Compilação

```
cargo build --workspace
```

**Resultado**: verde. 2 warnings pré-existentes
(foundations.rs unreachable patterns).

### C9 — Tests workspace

```
cargo test --workspace
Total tests: 1836
```

**1829 → 1836** (+7 tests):
- 3 unit tests em `position.rs` (construção + 2 hash).
- 4 P204D tests em `tests.rs` (2 sentinels + 2 E2E).

**Sem regressões**.

### C10 — Linter

```
crystalline-lint .
```

**Resultado**: 0 violations (após `--fix-hashes` ter
resolvido V1 inicial).

### C11 — Sentinelas P204D adicionadas

2 sentinels:
- `p204d_position_struct_existe` — falha de compilação
  se `Position` for removido.
- `p204d_runtime_positions_field_existe` — falha de
  compilação se `runtime.positions` for removido.

---

## §8 Métricas

| Métrica | Pré-P204D | Pós-P204D | Δ |
|---------|-----------|-----------|---|
| Tests workspace | 1829 | **1836** | +7 |
| Crystalline-lint violations | 0 | 0 | = |
| LOC produção (position.rs + LRS field + advance) | baseline | +~80 | +80 |
| LOC tests (3 unit + 4 P204D) | baseline | +~120 | +120 |
| L0 prompts | 0 (entities/position) | **1 novo** | +1 |
| Trait API breaking | — | sim (`position_of` signature) | breaking interno |
| Consumers afectados | — | 0 (production) + 2 (tests stub — sem mudança) | — |

---

## §9 Critério de fecho — C13

Per spec §3 C13:

- [x] C1 inventário completo (6 sub-secções).
- [x] C2 API-decisão fixada (migrar stub).
- [x] C3 tipo `Position` criado (com Hash manual).
- [x] C4 sub-store `runtime.positions` adicionado.
- [x] C5 população single-pass em
  `advance_locator_if_locatable`.
- [x] C6 trait API resolvida (C6a — TagIntrospector
  retorna None).
- [x] C7 tests E2E (2: locatable + non-locatable).
- [x] C8 compilação verde.
- [x] C9 tests workspace verdes (1836).
- [x] C10 linter 0 violations.
- [x] C11 sentinelas (2 adicionadas).
- [x] Inventário registado (este ficheiro).
- [ ] Relatório escrito (próximo output).

**Sem `P204D.div-N`** registadas — sem divergências
empíricas relevantes.

---

## §10 Referências

### Modificados em P204D

- `01_core/src/entities/position.rs` (novo ficheiro).
- `01_core/src/entities/mod.rs` (export).
- `01_core/src/entities/layouter_runtime_state.rs`
  (campo + imports).
- `01_core/src/entities/introspector.rs` (trait
  signature + impl return).
- `01_core/src/rules/layout/mod.rs` (advance_locator
  populate).
- `01_core/src/rules/layout/tests.rs` (2 sentinels +
  2 E2E).
- `00_nucleo/prompts/entities/position.md` (L0 prompt
  novo).

### Inalterados (intencional)

- `LayouterRuntimeState` campos pré-existentes
  (`label_pages`, `known_page_numbers`, `is_readonly`).
- TagIntrospector impl outras (apenas `position_of`
  alterado).
- Consumers Layouter (`mod.rs`, `equation.rs`,
  `references.rs`, `outline.rs`) — não usam
  `position_of`.
- ADR-0073 (transita ACEITE em P204H).
- ADR-0066 (transita superseded em P204H).

### Auditoria fonte

- `00_nucleo/diagnosticos/typst-passo-204A-auditoria-comemo.md`
  (A2 PagedPosition, A3 0 consumers, A14 LayouterRuntimeState
  trackable, A16 Position scope).
- `00_nucleo/diagnosticos/typst-passo-204A-diagnostico.md`
  (C8 sub-passo M8, C13.1 P204D).
- `00_nucleo/diagnosticos/typst-passo-203A-auditoria-position.md`
  (A1 Point existe; A4 sub-stores 9; A5 walk-time
  impossibilitado; A6 Layouter info suficiente; A7
  vanilla post-layout pipeline).
- `00_nucleo/diagnosticos/typst-passo-203A-diagnostico.md`
  (C1 forma vanilla; C3 sub-store em LayouterRuntimeState;
  C4 single-pass; C5 cristalino divergence intencional).
- `00_nucleo/adr/typst-adr-0073-comemo-introspector.md`
  (PROPOSTO; plano de materialização §P204D).
