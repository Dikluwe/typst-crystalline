# Inventário Bib State — Lacuna #6

Documento P180 (2026-04-29). Inventário factual do estado actual de bibliografia em cristalino, comparação com vanilla, magnitude estimada para migração para `Introspector`, recomendação para P181+.

Padrão estabelecido por P167 (`inventario-consumers-counter-state-legacy.md`).

---

## 1. Componentes em cristalino

### 1.1 Fields legacy em `CounterStateLegacy`

| Field | Tipo | Origem | Documentação |
|-------|------|--------|--------------|
| `bib_entries` | `Vec<BibEntry>` | P159C (passo 159, sub-passo C) | "Entries bibliográficos coletadas durante introspect walk. Multi-Bibliography concatena na ordem de aparecimento." |
| `bib_numbers` | `HashMap<String, u32>` | P159F (subpadrão #15 N=3) | "Numeração 1-based de bib entries para style numeric. Multi-Bibliography preserva primeiro número via `or_insert`." |

Ambos populados durante `walk` em `01_core/src/rules/introspect.rs:567-573` (arm `Content::Bibliography`).

### 1.2 Tipo `BibEntry` (`01_core/src/entities/bib_entry.rs`)

Struct minimal extendido, **16 fields totais** sem parsing externo:

- **4 obrigatórios** (P159A): `key`, `author`, `title`, `year`.
- **4 comuns** (P159D): `volume`, `pages`, `journal`, `publisher`.
- **2 identificadores** (P159E): `url`, `doi`.
- **6 restantes** (P159G): `editor`, `series`, `note`, `isbn`, `location`, `organization`.

Cobertura ~70-75% hayagriva universais. Sem parsing `.bib`/`.yaml`. Input cristalino é literal (caller constrói `BibEntry` à mão via builder pattern).

### 1.3 `Content::Bibliography` (content.rs:538)

```rust
Bibliography {
    entries: Vec<BibEntry>,
    title:   Option<Box<Content>>,
}
```

Subset minimal — vanilla `BibliographyElem` tem 6+ fields adicionais (`sources`, `full`, `style`, `lang`, `region`) **scope-out** em cristalino.

### 1.4 `Content::Cite` (content.rs:566)

```rust
Cite {
    key:        String,
    supplement: Option<Box<Content>>,
    form:       Option<CitationForm>,
}
```

`CitationForm`: `Normal | Prose | Author | Year`.

Já é payload-yielder via `extract_payload` arm `Cite` (P162) — `ElementPayload::Citation { key }` indexada em `kind_index[Citation]`.

### 1.5 Walk arm `Content::Bibliography` (introspect.rs:567)

```rust
Content::Bibliography { entries, title } => {
    for entry in entries {
        let next_num = state.bib_numbers.len() as u32 + 1;
        state.bib_numbers.entry(entry.key.clone()).or_insert(next_num);
    }
    state.bib_entries.extend(entries.iter().cloned());
    if let Some(t) = title { walk(t, state, locator, tags, None); }
}
```

Walk **muta state** directamente — sem emissão de Tag. Não respeita padrão "walk puro" (P163 invariante) face a `bib_*` fields.

### 1.6 Stdlib `bibliography(...)` (`structural.rs`)

`extract_bib_entries` em `01_core/src/rules/stdlib/structural.rs:516` — parser de literal `Vec<BibEntry>` a partir de `Value::Array(Vec<Value::Dict>)`.

---

## 2. Consumers

| Consumer | Field lido | Localização | Comportamento |
|----------|------------|-------------|---------------|
| `Layouter::layout_content` arm `Cite` | `bib_entries`, `bib_numbers` | `01_core/src/rules/layout/mod.rs:584-597` | Resolve form (Normal/Prose/Author/Year): lookup por key → format conforme |
| `Layouter` (legacy entry) | `bib_entries`, `bib_numbers` | `01_core/src/rules/layout/mod.rs:1386-1388` | Copia state legacy para Layouter |
| `Layouter::layout_with_introspector` | `bib_entries`, `bib_numbers` | `01_core/src/rules/layout/mod.rs:1414-1416` | Idem (clone) |

**Total**: 1 consumer real (Layouter cite-arm); 2 sites de cópia state→Layouter.

Test consumers (não migram):
- `counter_state_legacy::tests` (3 tests): default empty + insertion + lookup.

---

## 3. Comparação com vanilla

| Componente | Vanilla | Cristalino | Diferença |
|------------|---------|------------|-----------|
| Parser BibLaTeX | `hayagriva` crate (3rd party) | Ausente — input literal `Vec<BibEntry>` | Cristalino divergiu completamente |
| CSL formatting | `citationberg` crate (3rd party) | Ausente — formatting hardcoded por `CitationForm` enum | Cristalino reduzido para 4 forms |
| `BibliographyElem` fields | 6+ fields (sources, full, style, lang, region, hash) | 2 (entries, title) | Subset minimal |
| `BibEntry` cobertura | 100% hayagriva | ~70-75% (16 fields) | ADR-0054 graded |
| Storage | `Bibliography(Arc<...>)` com `IndexMap<PicoStr, hayagriva::Entry>` + memoization | `Vec<BibEntry>` linear scan | Cristalino simplifica O(1) → O(n) lookup |
| Indexação numeração | Integrado via comemo + Introspector | `HashMap<String, u32>` populado em walk | Sem fixpoint memoization |

**Domínio próprio?** Vanilla bibliografia é um sub-sistema com:
- Parser externo (hayagriva).
- Formatter externo (citationberg, 100+ styles CSL).
- Fixpoint via comemo para resolução tardia.
- ~1226 linhas só em `bibliography.rs`.

Cristalino divergiu **deliberadamente** para subset minimal (~150 linhas em `bib_entry.rs` + walk arm + cite-arm em layout). **Bib state cristalino é Introspection-style**, não domínio próprio externo.

---

## 4. Magnitude estimada

**S-M (Pequena a Média)** com justificação:

✅ **A favor de S-M**:
- Bib state cristalino é apenas 2 fields legacy (`bib_entries: Vec`, `bib_numbers: HashMap`).
- Walk arm já popula ambos linearmente.
- 1 consumer real (Layouter cite-arm).
- Sem dependências externas (sem hayagriva/CSL).
- Storage simples — `Vec<BibEntry>` + lookup linear é o que existe.
- **Padrão sub-store** já estabelecido em P165 (LabelRegistry, CounterRegistry), P169 (MetadataStore), P171 (StateRegistry). `BibStore` paralelo é replicação directa.

⚠️ **Subtilezas**:
- `bib_numbers` requer indexação **incremental** (1, 2, 3, ...) — paralelo a `figure_label_numbers` (P168).
- Walk arm muta state **fora** do mecanismo de tags. Migração para sub-store via `from_tags` requer **promoção** de `Bibliography` a payload kind (`ElementKind::Bibliography` + `ElementPayload::Bibliography { entries, ... }`).
- 1 consumer (Layouter) — migração análoga a P168 figure-ref (caminho C estabelecido).

**Sem gates substanciais identificados**. Trabalho replica padrões existentes (sub-store + locatable kind + cascade cite-arm).

---

## 5. Recomendação para implementação

### Caminho A (recomendado): Implementação directa em **P181**

Magnitude S-M confirma viabilidade.

**Sub-passos sugeridos para P181**:

1. **`.A`** — confirmar inventário (este P180), decidir forma de `BibStore`.
2. **`.B`** — criar `entities/bib_store.rs` (sub-store paralelo a `MetadataStore`):
   - Fields: `entries: Vec<BibEntry>`, `numbers: HashMap<String, u32>`.
   - Métodos: `empty()`, `add_bibliography(entries)`, `lookup_entry(key) -> Option<&BibEntry>`, `lookup_number(key) -> Option<u32>`.
3. **`.C`** — adicionar `ElementKind::Bibliography` + `ElementPayload::Bibliography { entries: Vec<BibEntry> }`.
4. **`.D`** — `is_locatable(Content::Bibliography) == true`; `extract_payload` arm.
5. **`.E`** — `from_tags` arm popula `bib_store.add_bibliography(entries)`.
6. **`.F`** — adicionar `Introspector::bib_entry_for_key(key)` + `bib_number_for_key(key)` ao trait + impl.
7. **`.G`** — modificar walk arm `Content::Bibliography` para preservar mutação `state.bib_*` (compat) **E** emitir Tag — **ou** aceitar dual-state durante transição M5+M6.
8. **`.H`** — Layouter cite-arm migra para usar `Introspector` (similar a figure-ref P168).
9. **`.I`** — tests E2E + lacuna #6 fechada.
10. **`.J`** — relatório.

**Magnitude**: 10 sub-passos S-M, comparável a P171 (state feature). Estimativa **+15 a +25 tests**.

### Caminhos rejeitados

- **Caminho B (M-L)** — inventário ainda mais detalhado em P181 + implementação P182. Rejeitado: P180 já produziu inventário suficiente; informação adicional não destrava decisões.
- **Caminho C (L-XL)** — decomposição em N passos. Rejeitado: cristalino bib é minimal; não há justificação para fragmentar.

---

## 6. Decisões a tomar antes de P181

Cláusulas gate trivial em P181 `.A` resolvem:

1. **Forma de `BibStore`**:
   - `Vec<BibEntry>` simples vs `IndexMap<key, BibEntry>` (lookup O(1) vs O(n)).
   - Sugestão: `IndexMap` (simetria com vanilla; lookup O(1)).
2. **Multi-Bibliography concat semantics**:
   - `state.bib_entries.extend(...)` actual concatena. `BibStore::add_bibliography` deve replicar esse comportamento.
   - Sugestão: replicar com `add_bibliography` aceitando múltiplas chamadas.
3. **`bib_numbers` order preservation**:
   - `or_insert` actual preserva primeiro número se key duplicada. Manter.
4. **Walk arm modificação**:
   - **Opção α**: walk continua a mutar `state.bib_*` directamente; `from_tags` adicionalmente popula `BibStore`. Dual-state durante transição.
   - **Opção β**: walk emite só Tag; `from_tags` popula `BibStore`; Layouter usa `BibStore`. Walk arm puro.
   - Sugestão: **β** (preserva P163 invariante; alinha com padrão pós-P162).
5. **Layouter cite-arm migração**:
   - Caminho similar a P168 figure-ref (introduzir `layout_with_introspector` already exists). Consumer único.
6. **Lacuna #6 fechamento**:
   - Resolvido se Layouter migrar **e** `bib_entries`/`bib_numbers` tornarem-se redundantes (M6 cleanup).
   - Decisão: P181 fecha lacuna em "infraestrutura pronta + consumer migrado". M6 elimina fields legacy quando todos os call-sites tiverem migrado.

---

## 7. Risco e mitigação

| Risco | Severidade | Mitigação |
|-------|------------|-----------|
| Walk arm já tem mutação significativa (`state.bib_numbers.entry().or_insert(...)`) | Médio | Replicar lógica em `BibStore::add_bibliography` literalmente |
| Layouter cite-arm tem 4 forms (Normal/Prose/Author/Year) | Médio | Padrão P168 — adaptar arm para chamar Introspector |
| Tests existentes em counter_state_legacy esperam `bib_numbers` | Baixo | Manter fields legacy durante M6 transição; tests passam |
| `BibEntry` 16 fields — Hash/Eq/etc | Baixo | Tipo já existe; deriva ou usa `format!("{:?}", ...)` se necessário |
| Multi-Bibliography reorder | Baixo | Documentar semântica em L0 |

---

## 8. Resumo numérico

- **Fields legacy**: 2 (`bib_entries`, `bib_numbers`).
- **Tipos relacionados**: 3 (`BibEntry`, `Content::Bibliography`, `Content::Cite`).
- **Walk arms**: 1 com mutação (`Content::Bibliography`).
- **Stdlib funcs**: 1 (`bibliography(...)`).
- **Consumers**: 1 (Layouter cite-arm).
- **Test sites**: 3 (counter_state_legacy::tests).
- **Magnitude**: **S-M** (+15 a +25 tests; ~10 sub-passos).
- **Caminho recomendado**: **A — implementação directa em P181**.

---

## 9. Estado de lacuna #6

Pré-P180: "Adiar — feature dedicada (não Introspection canonical)".

Pós-P180: **"Inventário concluído P180; magnitude S-M; recomendação: implementação directa P181 via padrão sub-store + locatable kind"**.

`m1-lacunas-captura.md` será actualizado em P180 `.G` para reflectir.
