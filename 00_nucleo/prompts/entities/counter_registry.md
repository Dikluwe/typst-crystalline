# Prompt L0 — `entities/counter_registry`
Hash do Código: 0cecccd2

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/counter_registry.rs`
**Criado em**: 2026-04-30 (P165 sub-passo .C — sub-store de M3)
**ADRs relevantes**: ADR-0033 (paridade vanilla), ADR-0066 (Introspection runtime)

---

## Contexto

`CounterRegistry` é o sub-store de counters indexados por kind (string). Aplica `CounterUpdate`s em ordem de chegada via `from_tags` (P165 .E).

Vanilla agrega no `ElementIntrospector` indirectamente — counters são queriáveis via `Selector` mas não há um "registry" dedicado. Cristalino isola por simetria com `LabelRegistry` e composição visível.

`CounterKey` enum vanilla (`Page | Selector(Selector) | Str(Str)`) **não** é replicado em M3. Em vez disso, usar `String` directo como chave (nomes: "heading", "figure", etc.). `CounterKey` rico fica para M9 quando counters custom forem adicionados via `counter("name")` em Typst markup.

---

## Restrições Estruturais

- Camada **L1**: struct puro, sem I/O.
- Read-only após construção (mutação só via `pub(crate) fn apply` durante fase de construção).
- `Clone` derivado.
- Forma interna: `HashMap<String, Vec<usize>>` — vector de números por kind, semelhante a `CounterStateLegacy.figure_numbers`.

---

## Interface pública

```rust
use crate::entities::counter_update::CounterUpdate;

#[derive(Debug, Clone, Default)]
pub struct CounterRegistry { /* HashMap<String, Vec<usize>> interno */ }

impl CounterRegistry {
    pub fn empty() -> Self;
    pub fn value(&self, key: &str) -> Option<&[usize]>;
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;

    /// **P170 (M9 sub-passo 2)** — formato hierárquico "1.2.3".
    pub fn format(&self, key: &str) -> Option<String>;

    pub(crate) fn apply(&mut self, key: String, update: CounterUpdate);

    /// **P170 (M9 sub-passo 2)** — step hierárquico ao nível indicado.
    /// Paridade com `CounterStateLegacy::step_hierarchical`. Resolve
    /// lacuna #5 (`m1-lacunas-captura.md`).
    pub(crate) fn apply_hierarchical(&mut self, key: String, level: usize);
}
```

---

## Semântica

- `empty()`: cria registry vazio.
- `value(key)`: retorna o slice actual do counter. `None` se nunca foi tocado.
- `apply(key, update)` (pub(crate)): aplica update.
  - `CounterUpdate::Step`: incrementa o último elemento. Se vector vazio, inicializa com `[1]`. Semelhante a `CounterStateLegacy::step_flat`.
  - `CounterUpdate::Update(value)`: define para `[value]`. Reseta hierarquia.
- **P170**: `apply_hierarchical(key, level)` (pub(crate)): paridade com `CounterStateLegacy::step_hierarchical`. Comportamento:
  - `[]` + `1` → `[1]`.
  - `[1]` + `2` → `[1, 1]`.
  - `[1, 1]` + `1` → `[2]`.
  - Level clamped a mínimo 1.
- **P170**: `format(key)` retorna `Option<String>` com Vec<usize> joined com "." ("1.2.1"). Forma equivalente a `CounterStateLegacy::format_hierarchical`. Resolve lacuna #5 — CounterRegistry deixa de ser flat em M9.

Hierarquia em M9: counters podem ter múltiplos níveis para Headings (paridade com walk arm `Content::Heading` em `introspect.rs::279`). Counters flat (figure, equation) continuam a usar `apply(_, Step)` que mantém Vec de tamanho 1.

---

## Invariantes

- `apply` é a única mutação — caller externo (não-test) só lê.
- Counters começam vazios; primeiro `Step` inicializa em 1.
- Igualdade entre instâncias é por conteúdo do HashMap interno.
- Sem ordering determinístico em iteração — caller que precise de ordem deve usar a chave conhecida.

---

## Tests obrigatórios (sub-passo .C P165)

- `CounterRegistry::empty().value("heading")` retorna `None`.
- Após `apply("heading", Step)`, `value("heading")` retorna `Some([1])`.
- 3 `Step` consecutivos produzem `[3]`.
- `apply("heading", Update(5))` → `value("heading")` retorna `Some([5])`.
- Counters distintos isolados — apply em "heading" não afecta "figure".

---

## Consumers actuais

Nenhum no momento da criação.

## Consumers planeados

- `rules/introspect/from_tags.rs` (P165 .E) — populador.
- `entities/introspector.rs` `TagIntrospector` (P165 .D) — leitor via getter.

---

## Sobre paridade

Vanilla não tem `CounterRegistry` separado — counters são parte do estado fixpoint via `comemo` + `ElementIntrospector` queries. Cristalino isola para coerência com `LabelRegistry`. Forma simplificada (flat counter, 1 nível) é suficiente para M3 minimal viable.

Forma rica (hierarquia, função counters via `Func`) fica para M9+ paralelamente à introdução de `CounterKey` enum vanilla.

Ver `00_nucleo/diagnosticos/inventario-tipos-introspection-vanilla.md` (2026-04-30) §3 para classificação de `Counter`/`CounterKey`/`CounterUpdate` vanilla.

---

## Resultado Esperado

- `01_core/src/entities/counter_registry.rs` — struct + 5 métodos + tests.
- Re-export em `01_core/src/entities/mod.rs`.

---

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-04-30 | P165 sub-passo .C: sub-store de counters por kind para Introspector M3 | `counter_registry.rs`, `counter_registry.md` |
