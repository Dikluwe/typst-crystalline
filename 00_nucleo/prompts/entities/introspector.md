# Prompt L0 — `entities/introspector`
Hash do Código: ee0371c4

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/introspector.rs`
**Criado em**: 2026-04-30 (P165 sub-passo .D — núcleo de M3)
**ADRs relevantes**: ADR-0033 (paridade vanilla), ADR-0066 (Introspection runtime)

---

## Contexto

`Introspector` é o trait que expõe queries sobre elementos indexados durante introspecção. `TagIntrospector` é a implementação concreta construída em `from_tags` (P165 .E) a partir de `Vec<Tag>`.

Vanilla equivalente: `lab/typst-original/.../introspection/introspector.rs::Introspector` trait + `ElementIntrospector<P>` impl genérica em P (target paged vs html). Cristalino simplifica:

- **Plain trait** (sem `#[comemo::track]` em M3) — tracking fica para M7+ quando fixpoint exigir memoização cross-iteration. Decisão consciente registada em P165.A.
- **Concrete struct `TagIntrospector`** — sem genérico em P (cristalino é paged-only); composta de sub-stores explícitos (`LabelRegistry`, `CounterRegistry`, índice por `ElementKind`, mapa Location→Position futuro).

Position vazio em M3 — `position_of` retorna sempre `None`. Mecanismo de população só virá quando layout integrar (M5+ ou M9).

---

## Restrições Estruturais

- Camada **L1**: sem I/O, sem estado global.
- Trait com 5 métodos (todos read-only sobre `&self`).
- Struct concreta read-only após construção. Sub-stores como fields privados.
- Ordering determinístico: `query_by_kind` retorna `Vec<Location>` em ordem de inserção (preservada pelo construtor `from_tags`).
- Sem `Clone` no trait (object-safe não é requisito M3); `Clone` derivado na struct.

---

## Interface pública

```rust
use crate::entities::element_kind::ElementKind;
use crate::entities::label::Label;
use crate::entities::location::Location;

pub trait Introspector {
    fn query_by_kind(&self, kind: ElementKind) -> Vec<Location>;
    fn query_by_label(&self, label: &Label) -> Option<Location>;
    fn query_first(&self, kind: ElementKind) -> Option<Location>;
    fn query_unique(&self, kind: ElementKind) -> Option<Location>;
    fn position_of(&self, location: Location) -> Option<()>;
    // Position é () em M3 — mapa Location→Position fica vazio.
    // Tipo Position concreto será introduzido em M5/M9.

    /// **P168 (M5 sub-passo 2)**: número 1-based de figura associada a label.
    /// Apenas figuras numeradas+captioned (`is_counted=true`) são indexadas.
    /// Equivalente a `state.figure_label_numbers.get(label).copied()` legacy.
    fn figure_number_for_label(&self, label: &Label) -> Option<usize>;

    /// **P169 (M9 sub-passo 1)**: todos os values embebidos via
    /// `metadata(value)` vanilla, na ordem de aparecimento no walk.
    fn query_metadata(&self) -> &[Value];

    /// **P170 (M9 sub-passo 2)**: formato hierárquico do counter
    /// como string. Equivalente a `state.format_hierarchical(key)`.
    /// Resolve lacuna #5.
    fn formatted_counter(&self, key: &str) -> Option<String>;
}

#[derive(Debug, Clone, Default)]
pub struct TagIntrospector {
    pub labels:                LabelRegistry,
    pub counters:              CounterRegistry,
    pub kind_index:            HashMap<ElementKind, Vec<Location>>,
    /// **P168**: indexação Label→1-based para figuras numeradas+captioned.
    /// Populado por `from_tags` quando `ElementPayload::Figure.is_counted == true`
    /// E há label associada. Suporta `references.rs::layout_ref` figure-arm.
    pub figure_label_numbers:  HashMap<Label, usize>,
    /// **P169 (M9 sub-passo 1)** — values embebidos via `metadata()`.
    pub metadata:              MetadataStore,
    // positions: HashMap<Location, Position> — adiado para M5/M9
}

impl TagIntrospector {
    pub fn empty() -> Self;
}

impl Introspector for TagIntrospector { /* ... */ }
```

`pub` nos sub-stores: caller (testes ou consumers em M4+) pode aceder
directamente. Equivalente a getters por delegação mas sem boilerplate.
Imutabilidade garantida pela ausência de `pub(crate)` nos métodos
mutadores dos sub-stores expostos a clientes externos.

---

## Semântica dos métodos

- `query_by_kind(kind)`: vector de todas as `Location`s do kind, em ordem de inserção (= ordem de aparecimento no walk). Vazio se nenhum.
- `query_by_label(label)`: `Some(location)` se label existir; `None` caso contrário. Delegação para `LabelRegistry::lookup`.
- `query_first(kind)`: `Some(primeira location)` ou `None`. Equivalente a `query_by_kind(kind).first().copied()`.
- `query_unique(kind)`: `Some(loc)` apenas se houver **exactamente** 1 location desse kind; `None` se 0 ou >1.
- `position_of(loc)`: M3 retorna sempre `None` — mapa de positions vazio.

---

## Invariantes

- Construção via `from_tags`: índice por kind preserva ordem de tags.
- `query_first`/`query_unique` são derivados de `query_by_kind` — comportamento consistente.
- `query_by_label` ↔ `LabelRegistry::lookup` 1:1.
- M3: `position_of` é stub.

---

## Tests obrigatórios (sub-passo .D P165)

- `TagIntrospector::empty()` retorna struct com sub-stores vazios; todos os queries retornam `None`/`Vec::new()`.
- Construir struct com `LabelRegistry` populado (1 par) + `CounterRegistry` populado (1 kind) + `kind_index` populado (1 kind, 2 locations) — verificar:
  - `query_by_kind(kind)` retorna 2 locations em ordem.
  - `query_first(kind)` retorna a primeira.
  - `query_unique(kind)` retorna `None` (porque há 2).
  - `query_by_label(label)` retorna a location associada.
  - `position_of(loc)` retorna `None`.
- Caso unique: 1 location → `query_unique` retorna `Some(loc)`.

---

## Consumers actuais

Nenhum no momento da criação. Construído em paralelo a `CounterStateLegacy` por `from_tags` (P165 .E) mas descartado em `pub fn introspect()` (M3 não expõe — M4 começará).

## Consumers planeados

- M4: `pub fn introspect_with_introspector(content) -> (CounterStateLegacy, TagIntrospector)` ou similar.
- M5: primeiro consumer real de `Introspector` migra de `CounterStateLegacy.resolved_labels` para `query_by_label`.
- M9: features novas (`metadata`, `state`, `query`) consultam Introspector como source-of-truth.

---

## Sobre paridade

Vanilla `Introspector` trait com `#[comemo::track]` + `ElementIntrospector<P>` genérica. Diferenças cristalinas:

- Sem `#[comemo::track]` em M3 (deferido para M7+).
- Sem genérico `P` (cristalino paged-only).
- Sub-stores explícitos como fields públicos (composição visível) vs vanilla com fields privados via getters.
- `Position` ainda não materializado — em M3 o método é stub; vanilla tem `DocumentPosition::Paged | Html`.

Ver `00_nucleo/diagnosticos/inventario-tipos-introspection-vanilla.md` (2026-04-30) para mapa completo.

Fan-in baixo: M3 não tem consumers externos ainda.

---

## Resultado Esperado

- `01_core/src/entities/introspector.rs` — trait + struct + impl + tests.
- Re-export em `01_core/src/entities/mod.rs` (trait + struct).

---

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-04-30 | P165 sub-passo .D: trait + impl concreta para queries sobre tags | `introspector.rs`, `introspector.md` |
