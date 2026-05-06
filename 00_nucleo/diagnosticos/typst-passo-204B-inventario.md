# Inventário interno P204B — Aplicação de `#[comemo::track]`

**Data**: 2026-05-06.
**Executor**: Claude Code (Opus 4.7).
**Spec**: `00_nucleo/materialization/typst-passo-204B.md`.
**Natureza**: diagnóstico interno (factos empíricos +
decisões + alterações aplicadas).

---

## §1 C1 — Inventário empírico

### Localização

- **Trait `Introspector`**: `01_core/src/entities/introspector.rs:37-164`
  (não `01_core/src/contracts/` como spec indicou; caminho
  real confirmado).
- **Impl `TagIntrospector`**: mesmo ficheiro, linhas
  173-219 (struct) + 228-327 (impl).

### Bounds actuais (pré-P204B)

- Trait: `pub trait Introspector` — sem bounds.
- TagIntrospector: `#[derive(Debug, Clone, Default)]`.

### Tabela das 20 assinaturas

Cada método com argumentos (`ToOwned` por arg) e retorno (`Hash`).

| # | Método | Args | Args ToOwned? | Retorno | Retorno Hash? | Etiqueta |
|---|--------|------|---------------|---------|---------------|----------|
| 1 | `query_by_kind` | `ElementKind` | ✅ (Clone+Hash) | `Vec<Location>` | ✅ | CONFIRMADO |
| 2 | `query_by_label` | `&Label` | ✅ | `Option<Location>` | ✅ | CONFIRMADO |
| 3 | `query_first` | `ElementKind` | ✅ | `Option<Location>` | ✅ | CONFIRMADO |
| 4 | `query_unique` | `ElementKind` | ✅ | `Option<Location>` | ✅ | CONFIRMADO |
| 5 | `position_of` | `Location` | ✅ (Copy) | `Option<()>` | ✅ | CONFIRMADO |
| 6 | `figure_number_for_label` | `&Label` | ✅ | `Option<usize>` | ✅ | CONFIRMADO |
| 7 | `query_metadata` | (none) | n/a | `&[Value]` | ❌ → ✅ | **AJUSTE** |
| 8 | `formatted_counter` | `&str` | ✅ | `Option<String>` | ✅ | CONFIRMADO |
| 9 | `state_value` | `&str, Location` | ✅ | `Option<&Value>` | ❌ → ✅ | **AJUSTE** |
| 10 | `state_final_value` | `&str` | ✅ | `Option<&Value>` | ❌ → ✅ | **AJUSTE** |
| 11 | `query` | `&Selector` | ✅ | `Vec<Location>` | ✅ | CONFIRMADO |
| 12 | `formatted_counter_at` | `&str, Location` | ✅ | `Option<String>` | ✅ | CONFIRMADO |
| 13 | `bib_entry_for_key` | `&str` | ✅ | `Option<&BibEntry>` | ❌ → ✅ | **AJUSTE** |
| 14 | `bib_number_for_key` | `&str` | ✅ | `Option<u32>` | ✅ | CONFIRMADO |
| 15 | `is_numbering_active` | `&str` | ✅ | `bool` | ✅ | CONFIRMADO |
| 16 | `figure_number_at_index` | `&str, usize` | ✅ | `Option<usize>` | ✅ | CONFIRMADO |
| 17 | `is_numbering_active_at` | `&str, Location` | ✅ | `bool` | ✅ | CONFIRMADO |
| 18 | `flat_counter_at` | `&str, Location` | ✅ | `Option<usize>` | ✅ | CONFIRMADO |
| 19 | `resolved_label_for` | `&Label` | ✅ | `Option<&str>` | ✅ | CONFIRMADO |
| 20 | `headings_for_toc` | (none) | n/a | `&[(Label, Content, usize)]` | ❌ → ✅ | **AJUSTE** |

**4 métodos com ajuste necessário** — todos por causa de
`Hash` no tipo retornado:
- 7, 9, 10: `Value` não derivava `Hash`.
- 13: `BibEntry` não derivava `Hash`.
- 20: `Content` não tinha impl Hash.

### Verificação dos tipos auxiliares

| Tipo | Hash pré-P204B | Hash pós-P204B |
|------|----------------|------------------|
| `Location` | ✅ derive | ✅ |
| `Label` | ✅ derive | ✅ |
| `ElementKind` | ✅ derive (Hash + Eq) | ✅ |
| `Selector` | ✅ derive | ✅ |
| `Value` | ❌ (f64 bloqueia derive) | ✅ **manual via Debug** |
| `BibEntry` | ❌ (Hash não derivado) | ✅ **derive Hash** |
| `Content` | ❌ (manual PartialEq sem Hash) | ✅ **manual via hash_content u128** |

---

## §2 Divergência registada — `P204B.div-1`

**Detectada em**: C1.

**Estado anterior**: spec P204B §2 indicava trait em
`01_core/src/contracts/introspector.rs`. Realidade:
`01_core/src/entities/introspector.rs`. Origem: erro
factual da spec; sem impacto material.

**Estado real**: 3 tipos retornados por métodos do trait
(`Value`, `BibEntry`, `Content`) **não** implementavam
`Hash`, requisito de comemo (per A10 da auditoria P204A).

**Decisão (per spec §3 C1 critério)**: **resolver dentro
de P204B (preferido)** — adicionar `Hash` impls aos 3
tipos. Magnitude permanece dentro de S-M (3 impls
simples, ~30 LOC).

**Justificação**:

- `BibEntry`: derive trivial — campos String/Option<String>/u32.
- `Value`: enum com `f64` bloqueia derive; manual via
  Debug formatting (mesmo padrão de `hash_content` P162).
- `Content`: enum com 60 variantes; já tem manual
  PartialEq; manual Hash delega ao `hash_content` u128
  existente (P162).

Recuar para P204A não traria ganho — alternativas C2 (B/C/D)
todas requereriam trabalho similar ou maior.

---

## §3 C3+C4 — `Send + Sync` de `TagIntrospector`

### Análise por field

| # | Field | Tipo | Send? | Sync? |
|---|-------|------|-------|-------|
| 1 | `labels` | `LabelRegistry` (HashMap<Label, Location>) | ✅ | ✅ |
| 2 | `counters` | `CounterRegistry` | ✅ | ✅ |
| 3 | `kind_index` | `HashMap<ElementKind, Vec<Location>>` | ✅ | ✅ |
| 4 | `figure_label_numbers` | `HashMap<Label, usize>` | ✅ | ✅ |
| 5 | `metadata` | `MetadataStore` (Vec<Value>) | ✅ | ✅ |
| 6 | `state` | `StateRegistry` | ✅ | ✅ |
| 7 | `bib_store` | `BibStore` | ✅ | ✅ |
| 8 | `resolved_labels` | `ResolvedLabelStore` | ✅ | ✅ |
| 9 | `headings_for_toc` | `Vec<(Label, Content, usize)>` | ✅ | ✅ |

**Conclusão C3**: todos os 9 sub-stores são `Send + Sync`
automáticos (auto-trait via `Send + Sync` derivados de
todos os fields). Sem fields obstrutivos.

**C4 — sem trabalho necessário**. Compilador confirmou
que `TagIntrospector: Send + Sync` automaticamente quando
trait foi declarado com `: Send + Sync`.

---

## §4 C5 — Alterações literais aplicadas

### 4.1 `01_core/src/entities/introspector.rs`

```text
+ /// **P204B (M8)** — `#[comemo::track]` aplicado per ADR-0073
+ /// (paridade vanilla literal). Trait fica `Send + Sync`.
+ #[comemo::track]
- pub trait Introspector {
+ pub trait Introspector: Send + Sync {
      // 20 métodos inalterados em assinatura.
  }
```

### 4.2 `01_core/src/entities/bib_entry.rs`

```text
- #[derive(Debug, Clone, PartialEq, Eq)]
+ #[derive(Debug, Clone, PartialEq, Eq, Hash)]
  pub struct BibEntry { ... }
```

Todos os fields (String, Option<String>, u32) já eram
Hash; derive trivial.

### 4.3 `01_core/src/entities/value.rs`

```text
+ // P204B (M8): impl Hash via Debug formatting.
+ impl std::hash::Hash for Value {
+     fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
+         format!("{:?}", self).hash(state);
+     }
+ }
```

Estratégia Debug-based mesmo padrão de `hash_content`
(P162). f64 e tipos compostos ficam bem cobertos.
Comentário documenta razão (NaN bloqueia derive).

### 4.4 `01_core/src/entities/content.rs`

```text
+ // P204B (M8): impl Hash via hash_content (existing
+ // Debug-based hash function from P162).
+ impl std::hash::Hash for Content {
+     fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
+         crate::entities::content_hash::hash_content(self).hash(state);
+     }
+ }
```

Reusa `hash_content -> u128` que já era usado em
`extract_payload` para hash de body. Sem duplicação de
lógica.

### 4.5 Edições não necessárias

- Sem ajustes em `TagIntrospector` (Send+Sync automático).
- Sem ajustes em consumers (Layouter migra em P204C).
- Sem ajustes em outros traits.

---

## §5 C6+C7+C8 — Verificações

### C6 — Compilação

`cargo build -p typst-core`: **verde**, 2 warnings
pré-existentes (em foundations.rs sobre matches
unreachable; não relacionados com P204B).

`cargo build --workspace`: **verde**.

### C7 — Tests workspace

```
cargo test --workspace
Total tests: 1827
```

**1824 → 1827** (+3 P204B sentinel tests; ver §6 C9).
**Sem regressões**.

### C8 — Linter

`crystalline-lint .`: **0 violations**.

---

## §6 C9 — Tests sentinela adicionados

3 tests em
`01_core/src/entities/introspector.rs` módulo `tests`:

### Test 1 — `p204b_trait_e_send_sync`

Sentinel para bounds. Falha compilação se `Send + Sync`
removidos do trait.

```rust
fn assert_send_sync<T: Send + Sync + ?Sized>() {}
assert_send_sync::<dyn Introspector>();
```

### Test 2 — `p204b_dyn_trait_implementa_track`

Sentinel para `#[comemo::track]`. Falha compilação se
atributo removido (porque `comemo::Track` impl é gerado
pelo macro).

```rust
fn assert_track<T: comemo::Track + ?Sized>() {}
assert_track::<dyn Introspector>();
```

### Test 3 — `p204b_tagintrospector_pode_ser_tracked_via_dyn`

Sentinel runtime. Confirma coerção `&TagIntrospector → &dyn
Introspector` e que `.track()` está disponível via macro.

```rust
use comemo::Track;
let intr = TagIntrospector::empty();
let dyn_ref: &dyn Introspector = &intr;
let _tracked = dyn_ref.track();
```

Todos verdes em `cargo test -p typst-core --lib p204b`.

---

## §7 Decisões tomadas durante a leitura

### 7.1 Probe-then-resolve em vez de inventário ex ante

C1 inicial detectou que 4 métodos retornam tipos sem
`Hash`. Em vez de listar cada erro especulativamente,
**apliquei o `#[comemo::track]` como probe** e li os
erros do compilador para classificação precisa.

Resultado: 3 erros distintos (`Value`, `BibEntry`,
`Content`). Pragmático.

### 7.2 Manual Hash via Debug — pragmático sobre puro

Para `Value`:
- Derive directo bloqueia (f64 NaN).
- Manual estrutural seria 17 variantes de match.
- Manual via `format!("{:?}", self)` é 1 linha; mesma
  estratégia que `hash_content` do P162 já usa para
  Content.

Trade-off aceito: colisões teóricas de hash entre Values
com Debug idêntico mas estrutura diferente — improvável
porque Debug é estrutural recursivo. Comemo trata
colisões como cache miss (sem prejuízo correção).

### 7.3 Reuso de `hash_content` para Content

`hash_content -> u128` já existia (P162). Em vez de
duplicar lógica, `impl Hash for Content` delega.
Idiomático e sem duplicação.

### 7.4 Sentinel tests como verificação multi-nível

3 tests em vez de 1 ou 2 porque cobrem aspectos
distintos:
- Test 1: bounds estruturais (Send+Sync).
- Test 2: macro `#[comemo::track]` aplicado (gera Track
  impl).
- Test 3: pipeline runtime end-to-end (.track() funciona).

Falhas distintas indicam problemas distintos.

### 7.5 Sem alterações em consumers

Layouter consumers ainda usam `&dyn Introspector`
directamente (não Tracked). Isso continua a compilar
porque `Tracked` é opt-in. P204C migra para
`Tracked<dyn Introspector>`.

P204B é foundational sem invadir P204C.

---

## §8 Métricas

| Métrica | Pré-P204B | Pós-P204B | Δ |
|---------|-----------|-----------|---|
| Tests workspace | 1824 | **1827** | +3 |
| Crystalline-lint violations | 0 | 0 | = |
| LOC produção | baseline | +~30 | +30 |
| LOC tests | baseline | +~30 | +30 |
| ADRs alterados | — | 0 | = |
| Ficheiros modificados | — | 4 | (introspector.rs, bib_entry.rs, value.rs, content.rs) |

---

## §9 Critério de fecho — C11

Per spec §3 C11:

- [x] C1 inventário completo (tabela 20 métodos +
  tipos auxiliares).
- [x] C2 alteração aplicada (`#[comemo::track]` +
  `: Send + Sync`).
- [x] C3+C4 `Send + Sync` confirmado (todos 9 sub-stores
  trivialmente Send+Sync).
- [x] C5 edições literais aplicadas (4 ficheiros).
- [x] C6 compilação verde.
- [x] C7 tests workspace verdes (1827).
- [x] C8 linter 0 violations.
- [x] C9 sentinel tests (3 adicionados).
- [x] Inventário registado (este ficheiro).
- [ ] Relatório escrito (próximo output).

**Divergência `P204B.div-1`** registada e resolvida
internamente (3 Hash impls).

---

## §10 Referências

### Modificados em P204B

- `01_core/src/entities/introspector.rs` (trait
  declaration + 3 sentinel tests).
- `01_core/src/entities/bib_entry.rs` (derive Hash).
- `01_core/src/entities/value.rs` (manual Hash).
- `01_core/src/entities/content.rs` (manual Hash via
  hash_content).

### Inalterados (intencional)

- `01_core/src/entities/content_hash.rs` (P162 hash_content
  reusado).
- Layouter consumers (P204C migra para Tracked).
- ADR-0073 (transita ACEITE em P204H).
- ADR-0066 (sem alteração).

### Auditoria fonte

- `00_nucleo/diagnosticos/typst-passo-204A-auditoria-comemo.md`
  (A1-A16).
- `00_nucleo/diagnosticos/typst-passo-204A-diagnostico.md`
  (C1-C14).
- `00_nucleo/adr/typst-adr-0073-comemo-introspector.md`
  (PROPOSTO).
- `00_nucleo/materialization/typst-passo-204B.md` (spec).
