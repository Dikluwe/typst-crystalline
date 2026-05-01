# Prompt L0 — `entities/label_registry`
Hash do Código: 630fada0

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/label_registry.rs`
**Criado em**: 2026-04-30 (P165 sub-passo .B — sub-store de M3)
**ADRs relevantes**: ADR-0033 (paridade vanilla), ADR-0066 (Introspection runtime)

---

## Contexto

`LabelRegistry` é o sub-store que mapeia `Label → Location`. Construído em `from_tags` (P165 .E) a partir de `Tag::Start(loc, info)` quando `info.label.is_some()`.

Vanilla agrega no `ElementIntrospector` field `labels: MultiMap<Label, usize>`. Cristalino isola num tipo próprio — decisão consciente "melhor que vanilla" via fan-in/fan-out reduzido e composição visível. Ver `00_nucleo/diagnosticos/auditoria-isolamento-vs-vanilla.md`.

---

## Restrições Estruturais

- Camada **L1**: struct puro, sem I/O.
- Read-only após construção (mutação só via `pub(crate) fn add` durante fase de construção em `from_tags`).
- `Clone` derivado para satisfazer eventual contrato `comemo::Track` futuro (M7+).
- Decisão sobre múltiplos labels iguais: **rejeitar duplicados** silenciosamente (primeiro `add` ganha). Vanilla usa MultiMap mas em paridade comportamental cristalino documenta que labels devem ser únicos. Caso de label duplicada é diagnosticável em M9+ via `query_unique`.

---

## Interface pública

```rust
use crate::entities::label::Label;
use crate::entities::location::Location;

#[derive(Debug, Clone, Default)]
pub struct LabelRegistry { /* HashMap<Label, Location> interno */ }

impl LabelRegistry {
    pub fn empty() -> Self;
    pub fn lookup(&self, label: &Label) -> Option<Location>;
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;

    pub(crate) fn add(&mut self, label: Label, location: Location);
}
```

---

## Semântica

- `empty()`: cria registry vazio. Equivalente a `Default::default()`.
- `lookup(&label)`: retorna `Some(location)` se label foi adicionada, `None` caso contrário.
- `add(label, location)` (pub(crate)): insere par. Se label já existir, **mantém** a anterior (primeira ganha) — sem panic, sem warning. M9 pode introduzir validação stricter.
- `len()`/`is_empty()`: contagem de entradas únicas.

---

## Invariantes

- Após construção, registry é read-only para callers externos (apenas `lookup`/`len`/`is_empty`).
- Igualdade entre instances é por conjunto de pares (`HashMap` Eq).
- Ordem de inserção não é preservada (HashMap interno não ordena).

---

## Tests obrigatórios (sub-passo .B P165)

- `LabelRegistry::empty().lookup(&label)` retorna `None`.
- Após `add(label, location)`, `lookup(&label)` retorna `Some(location)`.
- 5 labels distintos, todos resolvem correctamente.
- Adicionar label duplicada não corrompe estado — primeira location é preservada.

---

## Consumers actuais

Nenhum no momento da criação.

## Consumers planeados

- `rules/introspect/from_tags.rs` (P165 .E) — populador.
- `entities/introspector.rs` `TagIntrospector::query_by_label` (P165 .D) — leitor.

---

## Sobre paridade

Vanilla `ElementIntrospector.labels: MultiMap<Label, usize>` (índice de label para posição em `elems` array). Cristalino isola e simplifica (HashMap, primeira ganha). Diferenças:

- Cristalino: tipo dedicado (40 linhas) vs vanilla (campo num struct de 695 linhas).
- Cristalino: ausência de MultiMap (decisão consciente para M3).
- Cristalino: API pública minimalista (4 métodos vs vanilla múltiplos métodos via Introspector trait).

Refino candidato: M9+ pode introduzir variant `MultiLabelRegistry` se algum cliente precisar de duplicados.

---

## Resultado Esperado

- `01_core/src/entities/label_registry.rs` — struct + 5 métodos + tests.
- Re-export em `01_core/src/entities/mod.rs`.

---

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-04-30 | P165 sub-passo .B: sub-store Label→Location para Introspector M3 | `label_registry.rs`, `label_registry.md` |
