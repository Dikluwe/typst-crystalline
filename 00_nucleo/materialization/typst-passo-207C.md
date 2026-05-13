# Passo 207C — `LabelRegistry` → `MultiMap` + `label_count`

**Série**: 207 (sub-passo `C`).
**Marco**: M9c (Bloco III sub-store refactor + Bloco II
parcial item 7).
**Tipo**: refactor estrutural + trait extension.
**Magnitude**: M (~2-3h).
**Pré-condição**: P207B concluído; trait 21 métodos;
tests 1878 verdes; 0 violations; ADR-0076 PROPOSTO
anotado §P207B ✅ MATERIALIZADO.
**Output**: 1 ficheiro (relatório curto).

---

## §1 Trabalho

Refactor `LabelRegistry` para suportar multi-label
+ materializar `label_count(label) -> usize` no trait
`Introspector` (item 7 P207A).

**Bloqueio resolvido**: `P207A.div-1` revisto aprovado;
Q1=β / Q2=γ / Q3=α / Q4=β fixados em histórico desta
conversa.

Reuso de dados P207A + P207B:

- `LabelRegistry` em
  `01_core/src/entities/label_registry.rs` (115L + iter
  P207B; hash actual `358133ac`).
- Trait com 21 métodos (`introspector.rs`; hash
  `22bcb907`).
- L0 prompts `label_registry.md` + `introspector.md`
  (hashes propagados em P207B).
- `CountingIntrospector` wrapper L3 — **regra empírica
  P207B §5**: cada novo trait method exige update em
  `03_infra/src/measurements.rs` (INTROSPECTOR_METHODS,
  CALL_COUNTERS, impl, sentinel).

---

## §2 Cláusulas (5)

### C1 — Verificação curta de pré-condições

Antes de tocar código:

1. `LabelRegistry` API actual (4 + 1 P207B):
   `empty`, `lookup -> Option<Location>`, `len`,
   `is_empty`, `iter`. Confirmar.
2. Call-sites empíricos de `lookup` (consumers da
   semântica single-label):
   ```
   grep -rn "labels.lookup\|label_registry.*lookup" 01_core/ 02_shell/ 03_infra/ 04_wiring/
   ```
   Listar literalmente (esperado ≤ 10 call-sites; per
   P207A A4 grep).
3. Sub-store `figure_label_numbers` em
   `TagIntrospector` (per P207A A4 #4) — verificar
   se assume label única.

Se algum call-site existente quebrar com semântica
multi-label, registar `P207C.div-N`.

### C2 — Decisão de tipo `MultiMap`

Decidir entre 2 opções (fixar antes de C3):

- **Opção a — `HashMap<Label, Vec<Location>>`** com
  helpers — sem dep nova.
- **Opção b — `multimap` crate** (allowlist L1
  verificada).

Critério: opção a é preferida (sem dep nova; trivial).
Cristalino tem precedente `HashMap<K, Vec<V>>` (per
P207A A4 #3 `kind_index`).

C2 fixa **a** salvo descoberta empírica que justifique
b.

### C3 — Materializar refactor

**L0 primeiro**:

Edição `00_nucleo/prompts/entities/label_registry.md`:
- Documentar semântica multi-label: `lookup` retorna
  `Option<Location>` (primeira inserção); novo
  `lookup_all` retorna `&[Location]`; `count(label)`
  retorna `usize`.
- API esperada: `empty`, `lookup`, `lookup_all`,
  `count`, `len`, `is_empty`, `iter` (preservado
  P207B).
- Invariante: `iter` continua ordenado por
  `Label.0`; entradas multi-label aparecem
  agrupadas.

Edição `00_nucleo/prompts/entities/introspector.md`:
- Trait ganha método 22: `fn label_count(&self, label:
  &Label) -> usize`.
- Semântica: retorna count de Locations associadas;
  0 se label desconhecido.
- Paralelo vanilla: `Introspector::label_count(Label)
  -> usize`.

**L1 depois**:

`label_registry.rs`:
- Field interno `HashMap<Label, Vec<Location>>`.
- `lookup(label) -> Option<Location>` mantém
  retornando primeira entrada (compatibilidade
  call-sites P207B).
- `lookup_all(label) -> &[Location]` novo.
- `count(label) -> usize` novo.
- `iter()` preservado — ajustar para emitir
  `(Label, Location)` pares para cada entrada
  multi-label (semântica: 1 par por Location).
- `add(label, location)` em `pub(crate)` — push em
  Vec.

`introspector.rs`:
- Trait method `label_count` + impl em
  `TagIntrospector` delegando a `self.labels.count`.

### C4 — Propagação a `CountingIntrospector` (per
regra empírica P207B §5)

`03_infra/src/measurements.rs`:
- `INTROSPECTOR_METHODS`: array passa 21 → **22**;
  entry `label_count`.
- `CALL_COUNTERS`: array passa 21 → **22**.
- impl `Introspector for CountingIntrospector` ganha
  `fn label_count` com `record_call(21)`.
- Sentinel `p204g_introspector_call_counts_existe`:
  assertion 21 → 22.
- L0 `measurements.md`: descrição "22 métodos".

### C5 — Tests + verificação final

Tests dedicados (~6-8):

- `p207c_label_registry_multilabel_lookup_primeira` —
  `lookup` retorna primeira Location inserida.
- `p207c_label_registry_lookup_all_retorna_todas` —
  ordem de inserção preservada em Vec.
- `p207c_label_registry_count_zero_para_desconhecido`.
- `p207c_label_registry_count_um_para_label_unica`.
- `p207c_label_registry_count_multiplo_para_label_repetida`.
- `p207c_label_registry_iter_agrupa_multilabel` —
  iter retorna todos os pares; ordem alfabética por
  Label preservada.
- `p207c_introspector_label_count_via_trait` — invoca
  via trait method.

```
cargo test --workspace 2>&1 | tail -10
crystalline-lint .
crystalline-lint --fix-hashes .
```

Critério: 1884+ verdes (1878 + 6+); 0 violations.

Anotar ADR-0076 §P207C: `✅ MATERIALIZADO {data}`.

---

## §3 Output

1 ficheiro: `00_nucleo/materialization/typst-passo-207C-relatorio.md`.

Estrutura paralela a P207B (~3-5 KB).

---

## §4 Não-objectivos

- `query_count_before(selector, end)` (Q4 deferred).
- Page-aware methods (P207D-E).
- `here()` / `locate()` (P208).
- Selector enum extensions (P209).
- Refactor `figure_label_numbers` para multi-label —
  out-of-scope; sub-store separado em
  `TagIntrospector` mantém single-label por design (per
  P207A A4 #4).

---

## §5 Riscos a evitar

1. **Quebrar call-sites de `lookup`** com semântica
   single-label. Mitigação: `lookup` mantém comportamento
   ("retorna primeira"); call-sites não precisam de
   alteração.
2. **`figure_label_numbers` confusion** — este sub-store
   é separado de `LabelRegistry` e usa `HashMap<Label,
   usize>` para mapear label → figure number. P207C
   **não toca** neste sub-store. Documentar distinção
   no L0 `label_registry.md`.
3. **Iter ordem** — P207B fixou alfabética por Label;
   multi-label preserva isso (entradas com mesma label
   ficam consecutivas; ordem dentro do grupo é
   inserção via Vec).
