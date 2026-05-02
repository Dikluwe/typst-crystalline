# Passo 181E — Relatório (`from_tags` arm popula `BibStore`)

**Data**: 2026-05-01
**Natureza**: passo **populacional** — substitui o no-op defensivo
P181C pela lógica que liga tags → `BibStore`. `state.bib_*` legacy
continua a ser populado paralelamente (walk arm inalterado);
Layouter continua a ler de state legacy. `BibStore` é shadow data
até P181G migrar consumer.

---

## 1. Sumário

`from_tags.rs` arm `ElementPayload::Bibliography { entries }` activo:

1. `kind_index[Bibliography]` recebe `loc`.
2. Loop sobre entries: `assign_number(entry.key, numbers_len() + 1)`
   — replica `state.bib_numbers.entry(key).or_insert(state.bib_numbers.len() + 1)`
   do walk arm (introspect.rs:569-571). Numeração 1-based contínua
   sobre keys novas; duplicates preservam primeiro número (cláusula 3
   P181A).
3. `add_bibliography(entries)` faz `extend` (cláusula 2 P181A).

**Bug capturado e corrigido em `.E`**: a versão sugerida na instrução
usava `bib_store.len() as u32 + 1` para `next_num`. Como
`add_bibliography` é chamado depois do loop, `len()` (= `entries.len()`)
permanece 0 durante toda a iteração — todas as entries recebiam
número 1. Solução: novo método `BibStore::numbers_len()` (paralelo
ao `state.bib_numbers.len()` do walk arm) que cresce **só em keys
novas** via `or_insert`. P181E adiciona-o à API pública de
`BibStore`.

**Outputs**:

- `00_nucleo/prompts/rules/introspect/from_tags.md` (L0;
  hash final `2f6b31cd`).
- `00_nucleo/prompts/entities/bib_store.md` (L0 actualizado:
  `numbers_len()` documentado; hash final `3ea366ac`).
- `01_core/src/rules/introspect/from_tags.rs` (arm Bibliography
  substitui no-op + 4 tests; linhagem `75237ba7`).
- `01_core/src/entities/bib_store.rs` (`numbers_len()` adicionado;
  linhagem `4051b23d`).

**Sem ADR nova**. **Sem DEBT novo**.

---

## 2. Verificações `.F` (todas confirmadas)

| Item | Estado |
|------|--------|
| 1. `cargo check --workspace` passa | ✅ |
| 2. `cargo test --workspace --lib`: 1458 → **1462** (+4) | ✅ |
| 3. `crystalline-lint .` zero violations | ✅ |
| 4. L0 `from_tags.md` actualizado com hash `2f6b31cd` | ✅ |
| 5. L1 `from_tags.rs` linhagem `@prompt-hash 75237ba7` | ✅ |
| 6. Arm Bibliography activo (não mais no-op) | ✅ |
| 7. `BibStore` populado quando `from_tags` processa tags Bibliography | ✅ (test `bibliography_arm_popula_bib_store`) |
| 8. `kind_index[Bibliography]` populado paralelamente | ✅ (test `bibliography_arm_popula_kind_index`) |
| 9. Walk **NÃO modificado** estruturalmente | ✅ |
| 10. Walk arm `Content::Bibliography` (linha 567-573) inalterado — continua a popular state legacy | ✅ |
| 11. `Introspector` trait **NÃO modificado** (P181F) | ✅ |
| 12. Layouter **NÃO modificado** (P181G) | ✅ |
| 13. Snapshot tests ADR-0033 verdes (215 infra integration; 6 ignored) | ✅ |
| 14. Linter passa final | ✅ |

---

## 3. Decisões registadas em `.A`

- **`BibStore::len()` existente**: confirmado em P181B (linha 61 de
  `bib_store.rs`); retorna `entries.len()`.
- **`BibStore::numbers_len()` novo**: criado em P181E para corrigir
  bug de numeração. Retorna `numbers.len()`. L0 actualizado.
- **Variável local em `from_tags`**: `intr` (linha 42).
- **Deref de `Tag::Start(loc, info)`**: `*loc` (Location é Copy).
- **`ElementInfo::new(payload)`** vs `ElementInfo::with_label(...)`:
  P181E tests usam apenas `ElementInfo::new` (sem label — não testado
  em P181E; cobertura genérica de label fica em P181I E2E).

---

## 4. Δ tests vs baseline P181D

| Suite | Baseline P181D | Pós-P181E | Δ |
|-------|----------------|-----------|---|
| core lib | 1458 | **1462** | +4 |
| infra integration | 215 (+6 ignored) | 215 (+6 ignored) | 0 |
| shell | 24 | 24 | 0 |
| **Total lib/integration verdes** | 1697 | **1701** | **+4** |
| CLI integration (separado) | 21 | 21 | 0 |
| **Total auditoria fresh** | 1718 | **1722** | **+4** |

Estimativa P181E: **+4** (4 tests `.B`). **Actual +4** —
exactamente conforme estimado.

Tests novos:

- `bibliography_arm_popula_bib_store` — `BibStore::len() == 2` +
  `entry_for_key` resolve.
- `bibliography_arm_atribui_numeros_em_ordem` — numeração 1, 2, 3.
- `bibliography_multi_extend_replica_legacy` — multi-Bib concat:
  `len() == 4`, números 1-4.
- `bibliography_arm_popula_kind_index` — `kind_index[Bibliography]`
  populado em paralelo.

---

## 5. Estado de M9 e P181

**M9 features**: 9/11 (sem alteração — feature bib não conta até
P181I fechar lacuna #6). `BibStore` populado em produção mas ainda
sem consumer real (Layouter continua em state legacy).

**P181**:

- `.A` (P181A): ✅ concluído (decisões + plano).
- `.B` (P181B): ✅ concluído (`BibStore` + field).
- `.C` (P181C): ✅ concluído (`ElementKind::Bibliography` +
  `ElementPayload::Bibliography`).
- `.D` (P181D): ✅ concluído (`is_locatable` + `extract_payload`
  arms).
- `.E` (P181E): ✅ concluído (este relatório). `BibStore`
  populado em produção; `kind_index[Bibliography]` populado.
- `.F` (P181F): pendente — `Introspector::bib_entry_for_key` +
  `bib_number_for_key` no trait + impl. **Pré-condição**:
  `BibStore` populado em produção (✓).
- `.G`–`.J`: pendentes.

---

## 6. Paridade `BibStore` vs `state.bib_*`

Por construção, P181E garante paridade entre `BibStore` populado
via `from_tags` e `state.bib_*` populado pelo walk arm legacy:

- **Mesma origem**: ambos derivam de `Content::Bibliography.entries`.
- **Mesma ordem**: walk arm faz `entries.extend(entries.iter().cloned())`;
  `BibStore::add_bibliography` faz `entries.extend(entries)`.
- **Mesma semântica de numeração**: ambos usam
  `next_num = current_count + 1` seguido de `or_insert(next_num)`.

Validação E2E formal (`introspect_with_introspector` produz
`BibStore` idêntico ao state legacy preenchido) fica para **P181I**.

---

## 7. Pendências cumulativas

P181E activa população do `BibStore` mas mantém path duplo. Sem
novas pendências; pré-existentes inalteradas:

- F1 — `CounterStateLegacy` 18 fields (M6).
- F2 — `Content` 59 variants em 3 560 linhas (M6/M9).
- F3 — `Layouter` 19 fields (M6).
- F10 — `format!("{:?}", x)` como hash determinístico.
- DEBT-55 — Bibliography + Cite XL (ADR-0062 PROPOSTO).

**Path duplo P181E-G-H reconhecido**: walk arm continua a popular
`state.bib_*` legacy; `from_tags` arm popula `BibStore`; Layouter
consome `state.bib_*` legacy (até P181G migrar). Mutação directa
em walk arm é removida em P181H, depois de Layouter migrar. Path
duplo é janela controlada de transição — output observable
garantido inalterado (Layouter ignora `BibStore`).

---

## 8. Estado pós-passo

- **P181E concluído**.
- **P181F desbloqueado**: adicionar ao trait `Introspector`:

  ```rust
  fn bib_entry_for_key(&self, key: &str) -> Option<&BibEntry>;
  fn bib_number_for_key(&self, key: &str) -> Option<u32>;
  ```

  Impl em `TagIntrospector` delega para `self.bib_store.entry_for_key`
  / `self.bib_store.number_for_key`. Magnitude S (~10 linhas L1 + ~3
  linhas L0 + ~2 tests).

- **Output observable**: inalterado. PDF e diagnostics idênticos
  a P181D.
- **Linhagem**: hashes consistentes via `crystalline-lint
  --fix-hashes`:
  - `from_tags.md ↔ from_tags.rs` = `2f6b31cd ↔ 75237ba7`
  - `bib_store.md ↔ bib_store.rs` = `3ea366ac ↔ 4051b23d`

P181E liga tags → `BibStore`. Próximo: **P181F** expõe `BibStore`
via trait `Introspector` para Layouter consumir em P181G.
