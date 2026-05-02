# Passo 181F — Relatório (trait `Introspector` métodos `bib_entry_for_key` + `bib_number_for_key`)

**Data**: 2026-05-01
**Natureza**: passo **API estendida** — trait `Introspector` ganha
2 métodos delegantes; impl em `TagIntrospector` chama
`BibStore::entry_for_key` / `number_for_key`. Sem cascade. Layouter
ainda não consome — migração em P181G.
**Pré-condição**: P181E concluído. `BibStore` populado em
produção; field `pub bib_store: BibStore` em `TagIntrospector`.

---

## 1. Sumário

Trait `Introspector` ganha 2 métodos read-only:

```rust
fn bib_entry_for_key(&self, key: &str) -> Option<&BibEntry>;
fn bib_number_for_key(&self, key: &str) -> Option<u32>;
```

Impl em `TagIntrospector` delega directamente:

```rust
fn bib_entry_for_key(&self, key: &str) -> Option<&BibEntry> {
    self.bib_store.entry_for_key(key)
}

fn bib_number_for_key(&self, key: &str) -> Option<u32> {
    self.bib_store.number_for_key(key)
}
```

Field `pub bib_store: BibStore` permanece público (composição
visível P181B preservada) — Layouter pode escolher trait method ou
acesso directo em P181G.

**Outputs**:

- `00_nucleo/prompts/entities/introspector.md` (L0 actualizado;
  hash final `9c591aff`).
- `01_core/src/entities/introspector.rs` (use BibEntry + 2 trait
  methods + 2 impls + 3 tests; linhagem `c91f6d5b`).

**Sem ADR nova**. **Sem DEBT novo**.

---

## 2. Verificações `.F` (todas confirmadas)

| Item | Estado |
|------|--------|
| 1. `cargo check --workspace` passa | ✅ |
| 2. `cargo test --workspace --lib`: 1462 → **1465** (+3) | ✅ |
| 3. `crystalline-lint .` zero violations | ✅ |
| 4. L0 `introspector.md` actualizado com hash `9c591aff` | ✅ |
| 5. L1 `introspector.rs` linhagem `@prompt-hash c91f6d5b` | ✅ |
| 6. Trait `Introspector` tem 2 métodos novos (`bib_entry_for_key`, `bib_number_for_key`) | ✅ |
| 7. `TagIntrospector` impl delega para `bib_store` | ✅ |
| 8. `bib_entry_for_key` em vazio → `None` | ✅ (test `bib_entry_for_key_em_introspector_vazio_devolve_none`) |
| 9. `bib_number_for_key` em vazio → `None` | ✅ (test `bib_number_for_key_em_introspector_vazio_devolve_none`) |
| 10. `bib_entry_for_key` em populado resolve | ✅ (test `bib_methods_resolvem_apos_populacao_directa_do_sub_store`) |
| 11. Walk **NÃO modificado** | ✅ |
| 12. `from_tags` **NÃO modificado estruturalmente** (P181E impl preservada) | ✅ |
| 13. Layouter **NÃO modificado** (P181G) | ✅ |
| 14. Walk arm `Content::Bibliography` em `walk()` (linha 567-573) inalterado — continua a popular state legacy | ✅ |
| 15. Field `bib_store` em `TagIntrospector` continua público | ✅ |
| 16. Snapshot tests ADR-0033 verdes (215 infra integration; 6 ignored) | ✅ |
| 17. Linter passa final | ✅ |

---

## 3. Decisões registadas em `.A`

- **Localização exacta dos métodos no trait**: após
  `formatted_counter_at` (linha 90-91), antes do `}` que fecha o
  trait (linha 91 → linha 104 após adição). Mantém ordem cronológica
  P165→P175→P177→P181F.
- **Localização dos impls**: após `formatted_counter_at` impl
  (linha 184-191), antes do `}` que fecha `impl Introspector for
  TagIntrospector`. Espelha ordem do trait.
- **Re-export de `BibEntry`**: `entities/mod.rs` usa apenas `pub mod
  bib_entry;` (convenção cristalina sem `pub use` — confirmado em
  P181B). Trait usa `use crate::entities::bib_entry::BibEntry;`
  directo no topo de `introspector.rs` (consistente com `use
  crate::entities::bib_store::BibStore;` adicionado em P181B).
- **Naming**: `bib_entry_for_key` / `bib_number_for_key` segue
  literalmente o padrão de `BibStore::entry_for_key` / `number_for_key`
  (P181B). Consistente com convenção `BibStore` API.

---

## 4. Δ tests vs baseline P181E

| Suite | Baseline P181E | Pós-P181F | Δ |
|-------|----------------|-----------|---|
| core lib | 1462 | **1465** | +3 |
| infra integration | 215 (+6 ignored) | 215 (+6 ignored) | 0 |
| shell | 24 | 24 | 0 |
| **Total lib/integration verdes** | 1701 | **1704** | **+3** |
| CLI integration (separado) | 21 | 21 | 0 |
| **Total auditoria fresh** | 1722 | **1725** | **+3** |

Estimativa P181F: **+3** (3 tests `.B`). **Actual +3** —
exactamente conforme estimado.

Tests novos:

- `bib_entry_for_key_em_introspector_vazio_devolve_none`.
- `bib_number_for_key_em_introspector_vazio_devolve_none`.
- `bib_methods_resolvem_apos_populacao_directa_do_sub_store` —
  popula via sub-store directo (não via `from_tags`; aquele caminho
  é coberto em `from_tags::tests` P181E) e verifica que trait methods
  delegam correctamente.

---

## 5. Estado de M9 e P181

**M9 features**: 9/11 (sem alteração — feature bib não conta até
P181I fechar lacuna #6). Trait expõe API mas Layouter ainda não
consome.

**P181**:

- `.A` (P181A): ✅ concluído (decisões + plano).
- `.B` (P181B): ✅ concluído (`BibStore` + field).
- `.C` (P181C): ✅ concluído (`ElementKind`/`ElementPayload` Bib).
- `.D` (P181D): ✅ concluído (`is_locatable` + `extract_payload`).
- `.E` (P181E): ✅ concluído (`from_tags` arm popula `BibStore`).
- `.F` (P181F): ✅ concluído (este relatório).
- `.G` (P181G): pendente — **Layouter cite-arm migra** para usar
  trait methods. **Pré-condição**: trait expõe `bib_entry_for_key`
  + `bib_number_for_key` (✓). Magnitude **M** (única M no plano —
  toca cite-arm em `layout/mod.rs:584-597` + cópia state→Layouter
  em 1385-1388 e 1413-1416 mantidas durante compat).
- `.H`–`.J`: pendentes.

---

## 6. Pendências cumulativas

P181F estende API mas mantém path duplo. Sem novas pendências;
pré-existentes inalteradas:

- F1 — `CounterStateLegacy` 18 fields (M6).
- F2 — `Content` 59 variants em 3 560 linhas (M6/M9).
- F3 — `Layouter` 19 fields (M6).
- F10 — `format!("{:?}", x)` como hash determinístico.
- DEBT-55 — Bibliography + Cite XL (ADR-0062 PROPOSTO).

---

## 7. Estado pós-passo

- **P181F concluído**.
- **P181G desbloqueado**: Layouter cite-arm (`mod.rs:584-597`)
  consulta via Introspector em vez de `self.counter.bib_entries`/
  `self.counter.bib_numbers`. Toca:

  - `mod.rs:584` — `self.counter.bib_entries.iter().find(...)` →
    `self.introspector.bib_entry_for_key(key)`.
  - `mod.rs:590` — `self.counter.bib_numbers.get(key)` →
    `self.introspector.bib_number_for_key(key)`.
  - Copy-sites em `pub fn layout` (1385-1388) e `pub fn
    layout_with_introspector` (1413-1416) **mantidas** (compat M6;
    eliminadas só quando fields legacy forem removidos).

  Magnitude **M**. Toca 1 ficheiro L1 + ~1 L0 + ~5 tests E2E
  (cite Normal/Prose/Author/Year + path legacy paridade).

- **Output observable**: inalterado. PDF e diagnostics idênticos
  a P181E.
- **Linhagem**: hashes consistentes via `crystalline-lint
  --fix-hashes`:
  - `introspector.md ↔ introspector.rs` = `9c591aff ↔ c91f6d5b`

P181F expõe `BibStore` via trait. Próximo: **P181G** liga Layouter
ao trait — primeira migração real de consumer (até aqui, walk arm
legacy era source-of-truth).
