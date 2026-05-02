# Passo 181B — Relatório (`entities/bib_store.rs` + field em `TagIntrospector`)

**Data**: 2026-05-01
**Natureza**: passo **infra-pura** — sub-store paralelo aos 4
existentes; sem população em produção; sem alteração de output
observable.
**Pré-condição**: P181A concluído. Decisões cláusula 1, 2, 3
fixadas (Vec+HashMap; concat via `extend`; `or_insert`).

---

## 1. Sumário

`BibStore` materializado como sub-store de `TagIntrospector`. Replica
literalmente o shape de `CounterStateLegacy.bib_entries` /
`bib_numbers` (cláusula 1 P181A). API mínima:

- `pub fn empty/entries/entry_for_key/number_for_key/len/is_empty`.
- `pub(crate) fn add_bibliography(Vec<BibEntry>)` (`extend`; cláusula 2).
- `pub(crate) fn assign_number(String, u32)` (`or_insert`; cláusula 3).

Field `pub bib_store: BibStore` adicionado a `TagIntrospector` em
composição visível (sem getter — convenção L0
`entities/introspector.md` justifica). População começa em P181E;
até lá permanece vazio.

**Outputs**:

- `00_nucleo/prompts/entities/bib_store.md` (L0 novo; hash final
  `ecaed36e`).
- `01_core/src/entities/bib_store.rs` (struct + 8 métodos + 7 tests;
  hash de linhagem `1173cc1b`).
- `01_core/src/entities/mod.rs`: `+pub mod bib_store;` (1 linha).
- `01_core/src/entities/introspector.rs`: field + use + 1 test
  (`empty_inicializa_bib_store_vazio`); hash de linhagem actualizado
  `85bbbbfb` → reflectido em L0 (`de097d16`).
- `00_nucleo/prompts/entities/introspector.md` (L0 actualizado: field
  novo + entrada Histórico de Revisões 2026-05-01).

**Sem ADR nova**. **Sem DEBT novo**.

---

## 2. Verificações `.H` (todas confirmadas)

| Item | Estado |
|------|--------|
| 1. `cargo check --workspace` passa | ✅ |
| 2. `cargo test --workspace --lib`: 1440 → **1448** (+8) | ✅ |
| 3. `crystalline-lint .` zero violations | ✅ |
| 4. L0 `entities/bib_store.md` existe com hash preenchido | ✅ (`ecaed36e`) |
| 5. L1 `entities/bib_store.rs` existe com linhagem (`@prompt-hash 1173cc1b`) | ✅ |
| 6. `entities/mod.rs` declara `pub mod bib_store;` | ✅ (sem `pub use` — convenção cristalina) |
| 7. `TagIntrospector.bib_store: BibStore` field público | ✅ (composição visível conforme L0 introspector.md) |
| 8. Getter público dedicado | ❌ omitido por convenção (sub-stores são `pub` directo; L0 introspector.md justifica) |
| 9. Walk **NÃO modificado** | ✅ (`introspect.rs:567-573` inalterado) |
| 10. `is_locatable`, `extract_payload`, `from_tags` **NÃO modificados** | ✅ |
| 11. Layouter **NÃO modificado** | ✅ (`mod.rs:584-597` inalterado; copy-sites inalterados) |
| 12. `Content::Bibliography` walk arm inalterado (continua a popular `state.bib_*` legacy) | ✅ |
| 13. Snapshot tests ADR-0033 verdes | ✅ (215 infra integration tests + 6 ignored) |
| 14. Linter passa final | ✅ |

**Decisão registada vs instrução .G**: a instrução P181B sugeriu
"field privado + getter `pub fn bib_store(&self) -> &BibStore`". A
convenção estabelecida em cristalino (e justificada em
`entities/introspector.md` linhas 28-30 + 87-97) é sub-store `pub`
em composição visível, sem getter dedicado — replicando o padrão de
`labels`/`counters`/`metadata`/`state`. A instrução P181B notou
explicitamente "ou padrão equivalente conforme convenção cristalina —
verificar `mod.rs` actual". Convenção foi seguida.

---

## 3. Decisões registadas em `.A`

- **`BibEntry` deriva `Debug`, `Clone`, `PartialEq`, `Eq`**
  (`bib_entry.rs:80`). Suficiente para `BibStore` (Clone para
  `add_bibliography(Vec<BibEntry>)` move semantics; PartialEq usado
  por `entry_for_key` para comparação de keys).
- **Field exacto para lookup**: `BibEntry.key: String`
  (`bib_entry.rs:82`). `entry_for_key` faz `e.key == key` directamente.
- **`TagIntrospector` é `pub struct` com `Default`**: construção
  via `TagIntrospector::empty()` (delega a `Default::default()`);
  field novo `pub bib_store: BibStore` inicializa via `Default`
  derivado.

---

## 4. Δ tests vs baseline P180

| Suite | Baseline P180 | Pós-P181B | Δ |
|-------|---------------|-----------|---|
| core lib | 1440 | **1448** | +8 |
| infra integration | 215 (+6 ignored) | 215 (+6 ignored) | 0 |
| shell | 24 | 24 | 0 |
| **Total lib/integration verdes** | 1679 | **1687** | **+8** |
| CLI integration (separado) | 21 | 21 | 0 |
| **Total auditoria fresh** | 1700 | **1708** | **+8** |

Estimativa P181B era +6 (5 BibStore + 1 TagIntrospector). Actual
**+8** — 2 tests adicionais em `bib_store.rs`:
`number_for_key_inexistente_devolve_none` (simétrico a
`entry_for_key_inexistente`) e `entries_preserva_ordem_em_multi_bib`
(cobertura explícita da cláusula 2 multi-Bibliography concat). Sem
custo arquitectural — apenas cobertura mais densa.

---

## 5. Estado de M9 e P181

**M9 features**: 9/11 (sem alteração). P181B é infra do sub-store —
feature bib não conta até P181I fechar lacuna #6. M9 atinge 10/11
quando lacuna #6 fechar; 11/11 quando lacuna #4 (`numbering_active`)
fechar.

**P181**:

- `.A` (P181A): ✅ concluído (decisões + plano).
- `.B` (P181B): ✅ concluído (este relatório).
- `.C` (P181C): pendente — `ElementKind::Bibliography` +
  `ElementPayload::Bibliography { entries: Vec<BibEntry> }`.
- `.D`–`.J`: pendentes.

---

## 6. Pendências cumulativas

P181B é puro infra; sem novas pendências. Pendências pré-existentes
inalteradas:

- F1 — `CounterStateLegacy` 18 fields (M6).
- F2 — `Content` 59 variants em 3 560 linhas (M6/M9).
- F3 — `Layouter` 19 fields + match 101 arms (M6).
- F10 — `format!("{:?}", x)` como hash determinístico.
- DEBT-55 — Bibliography + Cite XL (hayagriva ADR-0062 PROPOSTO).

---

## 7. Estado pós-passo

- **P181B concluído**.
- **P181C desbloqueado**: adicionar `ElementKind::Bibliography` +
  `ElementPayload::Bibliography { entries: Vec<BibEntry> }`. Magnitude
  S. Toca `entities/element_kind.rs` + `entities/element_payload.rs`
  + L0s correspondentes + ~3 tests.
- **Output observable**: inalterado. PDF e diagnostics idênticos a
  P181A.
- **Linhagem**: hashes consistentes via `crystalline-lint
  --fix-hashes` (`bib_store.md ↔ bib_store.rs` =
  `ecaed36e ↔ 1173cc1b`; `introspector.md ↔ introspector.rs` =
  `de097d16 ↔ 85bbbbfb`).

P181B é fundação. Próximo: **P181C** alarga ElementKind +
ElementPayload para suportar Bibliography como locatable kind
(decisão cláusula 4 P181A — Opção β walk puro).
