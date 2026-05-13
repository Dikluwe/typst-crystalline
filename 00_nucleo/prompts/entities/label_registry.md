# Prompt L0 — `entities/label_registry`
Hash do Código: 7bd1f25f

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/label_registry.rs`
**Criado em**: 2026-04-30 (P165 sub-passo .B — sub-store de M3)
**ADRs relevantes**: ADR-0033 (paridade vanilla), ADR-0066 (Introspection runtime)

---

## Contexto

`LabelRegistry` é o sub-store que mapeia `Label → [Location]` (multi-label). Construído em `from_tags` (P165 .E) a partir de `Tag::Start(loc, info)` quando `info.label.is_some()`.

Vanilla agrega no `ElementIntrospector` field `labels: MultiMap<Label, usize>`. Cristalino mantém isolamento num tipo próprio mas adopta semântica multi-label em P207C (M9c): refactor `HashMap<Label, Location>` → `HashMap<Label, Vec<Location>>`. Decisão fundamentada na cláusula C2 do diagnóstico P207A (item 7 da auditoria) — desbloqueio de `query_labelled` + `label_count` exige semântica que distingue 0/1/N locations por label. Ver `00_nucleo/diagnosticos/auditoria-isolamento-vs-vanilla.md` (referência histórica anterior a P207C).

---

## Restrições Estruturais

- Camada **L1**: struct puro, sem I/O.
- Read-only após construção (mutação só via `pub(crate) fn add` durante fase de construção em `from_tags`).
- `Clone` derivado para satisfazer contrato `comemo::Track` (M8/ADR-0073).
- **P207C** — Semântica multi-label: cada `add(label, location)` acumula em `Vec<Location>` interno. `lookup` mantém comportamento single-Location (retorna **primeira** inserção) para compatibilidade com call-sites pre-P207C; `lookup_all` e `count` expõem semântica multi-label completa.
- Sub-store `figure_label_numbers` em `TagIntrospector` é **separado**: usa `HashMap<Label, usize>` para mapear label → figure number e assume label única por figura. P207C não toca esse sub-store.

---

## Interface pública

```rust
use crate::entities::label::Label;
use crate::entities::location::Location;

#[derive(Debug, Clone, Default)]
pub struct LabelRegistry { /* HashMap<Label, Vec<Location>> interno (P207C multi-label) */ }

impl LabelRegistry {
    pub fn empty() -> Self;

    /// Primeira `Location` inserida para `label` (compat single-Location).
    pub fn lookup(&self, label: &Label) -> Option<Location>;

    /// **P207C (M9c)** — Todas as `Location`s associadas a `label`,
    /// em ordem de inserção. Slice vazio se label desconhecido.
    pub fn lookup_all(&self, label: &Label) -> &[Location];

    /// **P207C (M9c)** — Número de Locations associadas a `label`.
    /// 0 para label desconhecido.
    pub fn count(&self, label: &Label) -> usize;

    /// Número de **labels únicas** registadas (chaves do mapa interno),
    /// não o total de pares (Label, Location). Para o total ver `iter().count()`.
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;

    /// **P207B (M9c)** — iterador determinístico sobre `(label, location)`
    /// ordenado por `Label`. **P207C**: emite **um par por Location**
    /// (entradas multi-label aparecem agrupadas e consecutivas, na
    /// ordem de inserção dentro do grupo). Custo O(n log n) por
    /// invocação onde n = nº de labels únicas.
    pub fn iter(&self) -> impl Iterator<Item = (&Label, &Location)>;

    pub(crate) fn add(&mut self, label: Label, location: Location);
}
```

---

## Semântica

- `empty()`: cria registry vazio. Equivalente a `Default::default()`.
- `lookup(&label)` (compat): `Some(loc)` com **primeira** Location inserida para `label`; `None` se label nunca foi adicionado. Não distingue entre "label inexistente" e "label com 0 Locations" (este último não é representável).
- `lookup_all(&label)` (P207C): slice de **todas** as Locations associadas a `label`, em ordem de inserção. `&[]` se label desconhecido.
- `count(&label)` (P207C): nº de Locations associadas a `label`. `0` se label desconhecido. Equivalente a `lookup_all(label).len()`.
- `add(label, location)` (pub(crate)): faz `push` no `Vec<Location>` interno. Duplicados de pares `(label, location)` são preservados; em multi-label, múltiplas inserções da mesma `label` acumulam.
- `len()`/`is_empty()`: contagem de **labels únicas** (chaves do mapa). Não confundir com total de pares — para isso usar `iter().count()`.
- `iter()` (P207B + P207C): iterador ordenado por `Label` (alfabético sobre `Label.0: String`) emitindo **um par `(label, location)` por Location**. Entradas multi-label aparecem agrupadas e consecutivas, com ordem dentro do grupo = ordem de inserção. Ordenação determinística é requisito para consumers tipo `Introspector::query_labelled`. Custo O(n log n) na invocação (collect+sort sobre `HashMap::iter` interno; n = nº de labels únicas).

---

## Invariantes

- Após construção, registry é read-only para callers externos (`lookup`, `lookup_all`, `count`, `len`, `is_empty`, `iter`).
- Igualdade entre instances é por mapa interno (`HashMap` Eq sobre `Label → Vec<Location>`); ordem dos Vecs é parte da igualdade.
- Ordem de chaves (Labels) **não** é preservada no mapa interno; ordem é imposta por `iter()` (alfabética por `Label.0`).
- Ordem **dentro** de um Vec multi-label é preservada (ordem de `add`).

---

## Tests obrigatórios (sub-passo .B P165)

- `LabelRegistry::empty().lookup(&label)` retorna `None`.
- Após `add(label, location)`, `lookup(&label)` retorna `Some(location)`.
- 5 labels distintos, todos resolvem correctamente.
- Adicionar label duplicada preserva ordem de inserção em `Vec`; `lookup` continua a retornar **primeira** (compat); `lookup_all` retorna todas.
- **P207B**: `LabelRegistry::empty().iter()` produz iterador vazio. Após inserir 3+ labels em ordem arbitrária, `iter()` devolve pares em ordem alfabética de `Label` (determinística independente da ordem de `add`).
- **P207C** multi-label:
  - `lookup` retorna primeira inserção (mesmo com 2+ adds para o mesmo label).
  - `lookup_all` retorna `&[]` para label desconhecido; slice de N elementos para label com N inserções.
  - `count` retorna 0 para desconhecido; N para N inserções.
  - `iter` agrupa entradas multi-label consecutivamente; pares (Label, Location) preservam ordem de inserção dentro do grupo.

---

## Consumers actuais

Nenhum no momento da criação.

## Consumers planeados

- `rules/introspect/from_tags.rs` (P165 .E) — populador.
- `entities/introspector.rs` `TagIntrospector::query_by_label` (P165 .D) — leitor.

---

## Sobre paridade

Vanilla `ElementIntrospector.labels: MultiMap<Label, usize>` (índice de label para posições em `elems` array). Cristalino isola num tipo dedicado e em P207C adopta semântica multi-label paralela. Diferenças:

- Cristalino: tipo dedicado (~140 linhas) vs vanilla (campo num struct de 695 linhas).
- Cristalino: `HashMap<Label, Vec<Location>>` simples vs vanilla `MultiMap` crate.
- Cristalino: API pública com 7 métodos (`empty`, `lookup`, `lookup_all`, `count`, `len`, `is_empty`, `iter`) vs vanilla múltiplos métodos via Introspector trait.
- `lookup` cristalino retorna **primeira** (compat single-Location); vanilla expõe directamente o MultiMap.

---

## Resultado Esperado

- `01_core/src/entities/label_registry.rs` — struct + 7 métodos públicos + tests.
- Re-export em `01_core/src/entities/mod.rs`.

---

## Histórico de Revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-04-30 | P165 sub-passo .B: sub-store Label→Location para Introspector M3 | `label_registry.rs`, `label_registry.md` |
| 2026-05-12 | P207B (M9c — primeiro item C1 do roadmap ADR-0076): método público `pub fn iter(&self) -> impl Iterator<Item = (&Label, &Location)>` com ordenação determinística por `Label`. Desbloqueia `Introspector::query_labelled` (P207B); fundamento para futura migração `HashMap → MultiMap` (P207C planeado per ADR-0076 C2). | `label_registry.rs`, `label_registry.md` |
| 2026-05-12 | P207C (M9c — Bloco III sub-store refactor + Bloco II item 7): refactor interno `HashMap<Label, Location>` → `HashMap<Label, Vec<Location>>` (multi-label semântica). API ganha `lookup_all(&Label) -> &[Location]` e `count(&Label) -> usize`. `lookup` mantém comportamento single-Location (primeira inserção) por compatibilidade. `iter` adaptado para emitir um par por Location, preservando ordem alfabética por Label (P207B). Desbloqueia trait method `Introspector::label_count` (P207C). | `label_registry.rs`, `label_registry.md` |
