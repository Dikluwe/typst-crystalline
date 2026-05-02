# Passo 181G — Relatório (Layouter cite-arm migra para `Introspector`)

**Data**: 2026-05-01
**Natureza**: passo **migração de consumer** — único M no plano
P181. Cite-arm passa a consumir via `Introspector` com fallback
defensivo a state legacy. Padrão **substitution-with-fallback**
(P168 figure-ref) replicado pela 2ª vez.
**Pré-condição**: P181F concluído. Trait `Introspector` expõe
`bib_entry_for_key` + `bib_number_for_key`. `BibStore` populado em
produção (P181E). Paridade `BibStore` ↔ `state.bib_*` garantida por
construção.

---

## 1. Sumário

Cite-arm de `Content::Cite { key, supplement, form }` em
`layout/mod.rs:584-602` migra de leitura directa de
`self.counter.bib_*` para consulta via `Introspector` com fallback:

```rust
let entry = self.introspector
    .bib_entry_for_key(key)
    .or_else(|| self.counter.bib_entries.iter().find(|e| e.key == *key));

let number = self.introspector
    .bib_number_for_key(key)
    .or_else(|| self.counter.bib_numbers.get(key).copied());
```

Comportamento:

- **`layout()` legacy** invoca `layout_with_introspector(_, _,
  TagIntrospector::empty())` — Introspector vazio retorna `None` →
  fallback a state legacy serve as 4 cite forms. Backward compat
  preservado.
- **`layout_with_introspector(content, state, intr)`** usa
  Introspector populado por `from_tags` (P181E). State legacy
  preservado paralelamente.

Paridade `BibStore` ↔ `state.bib_*` confirmada por test
`paridade_legacy_vs_introspector_para_cite` (4 forms × ambos paths
→ output idêntico).

**Outputs**:

- `00_nucleo/prompts/rules/layout.md` (L0 actualizado: secção
  "Cite-arm consome Introspector (P181G)" adicionada; hash final
  `95e8429b`).
- `01_core/src/rules/layout/mod.rs` (cite-arm reescrito + use
  `Introspector` trait + use `CitationForm` movido para o topo do
  scope; linhagem `ad89eb8e`).
- `01_core/src/rules/layout/tests.rs` (módulo `p181g_cite_arm_migration`
  com 6 tests E2E).

**Sem ADR nova**. **Sem DEBT novo**.

---

## 2. Verificações `.F` (todas confirmadas)

| Item | Estado |
|------|--------|
| 1. `cargo check --workspace` passa | ✅ |
| 2. `cargo test --workspace --lib`: 1465 → **1471** (+6; estimativa era +5) | ✅ |
| 3. `crystalline-lint .` zero violations | ✅ |
| 4. L0 `rules/layout.md` actualizado com hash `95e8429b` | ✅ |
| 5. L1 `mod.rs` linhagem `@prompt-hash ad89eb8e` | ✅ |
| 6. Cite-arm consulta `Introspector` primeiro | ✅ (test `cite_consulta_introspector_quando_state_legacy_vazio`) |
| 7. Fallback a state legacy preservado | ✅ (legacy `layout()` continua a funcionar) |
| 8. 4 cite forms (Normal/Prose/Author/Year) renderizam correctamente | ✅ (4 tests dedicados) |
| 9. Paridade pre/post-migração confirmada via teste dedicado | ✅ (test `paridade_legacy_vs_introspector_para_cite` cobre 4 forms) |
| 10. Walk **NÃO modificado** | ✅ |
| 11. Walk arm `Content::Bibliography` (linha 567-573) inalterado | ✅ |
| 12. Copy-sites `state→Layouter` (1385-1388, 1413-1416) preservados | ✅ |
| 13. `BibStore` (`from_tags`) populado paralelamente | ✅ |
| 14. State legacy (`walk arm`) populado paralelamente | ✅ |
| 15. Snapshot tests ADR-0033 verdes — output observável inalterado | ✅ |
| 16. Linter passa final | ✅ |

---

## 3. Decisões registadas em `.A`

- **Fallback escolhido**: **Opção F1 — substitution-with-fallback**
  (padrão P168). Justificação: defensiva durante janela compat;
  `layout()` legacy continua a funcionar (Introspector vazio →
  fallback a state); paridade BibStore ↔ state garantida por
  construção mas o fallback evita regressão silenciosa em caso de
  bug futuro de reconstrução do Introspector.

- **4 cite forms identificados** (em `entities/citation_form.rs` via
  `CitationForm::{Normal, Prose, Author, Year}`):
  - **Normal/None** — usa `bib_number_for_key(key)` para `[N]`.
  - **Prose** — usa `bib_entry_for_key(key)` para `Author (Year)`.
  - **Author** — usa `bib_entry_for_key(key)` para `e.author`.
  - **Year** — usa `bib_entry_for_key(key)` para `e.year`.

- **Localização exacta**:
  - `layout/mod.rs:584` — linha de leitura `bib_entries` substituída.
  - `layout/mod.rs:590` — linha de leitura `bib_numbers` substituída.
  - `use Introspector` adicionado dentro do arm (escopo local;
    consistente com `use CitationForm` pré-existente).

- **`bib_number_for_key` retorna `Option<u32>`, `bib_numbers.get`
  retorna `Option<&u32>`**: `.copied()` adicionado no fallback para
  alinhar tipos. Cláusula gate trivial.

- **Acesso a `self.introspector`**: confirmado em P168 (Layouter
  field `introspector: TagIntrospector` linha 101). Cite-arm dentro
  de `layout_content` tem acesso directo.

---

## 4. Δ tests vs baseline P181F

| Suite | Baseline P181F | Pós-P181G | Δ |
|-------|----------------|-----------|---|
| core lib | 1465 | **1471** | +6 |
| infra integration | 215 (+6 ignored) | 215 (+6 ignored) | 0 |
| shell | 24 | 24 | 0 |
| **Total lib/integration verdes** | 1704 | **1710** | **+6** |
| CLI integration (separado) | 21 | 21 | 0 |
| **Total auditoria fresh** | 1725 | **1731** | **+6** |

Estimativa P181G: **+5** (5 tests `.B`). **Actual +6** — adicionei
1 teste diferencial (`cite_consulta_introspector_quando_state_legacy_vazio`)
que **prova explicitamente** que cite-arm consulta Introspector
(state legacy vazio + introspector populado → renderiza `[1]`).
Sem este teste, os 5 tests originais passariam mesmo se cite-arm
não fosse migrado, porque path legacy via state.bib_* produziria
o mesmo resultado. Cobertura mais robusta.

Tests adicionados em módulo `p181g_cite_arm_migration`:

1. `cite_normal_via_introspector_renderiza_numero` — `[1]` via path novo.
2. `cite_prose_via_introspector_renderiza_author_year` — `Smith, J. (2024)`.
3. `cite_author_via_introspector_renderiza_apenas_author` — autor.
4. `cite_year_via_introspector_renderiza_apenas_ano` — ano.
5. `paridade_legacy_vs_introspector_para_cite` — 4 forms × 2 paths
   → `plain_text` idêntico.
6. `cite_consulta_introspector_quando_state_legacy_vazio` — diferencial.

---

## 5. Estado de M5 e P181

**M5 progresso**: 2/6 consumers migrados (figure-ref P168 + cite-arm
P181G). Padrão substitution-with-fallback replicado com sucesso pela
2ª vez — confirma reusabilidade para outros consumers M5 futuros.

**M9 features**: 9/11 (sem alteração — feature bib não conta até
P181I fechar lacuna #6).

**P181**:

- `.A`–`.G`: ✅ concluídos.
- `.H` (P181H): pendente — **walk arm puro** (remove mutação directa
  `state.bib_*` em `introspect.rs:567-573`). **Pré-condição
  satisfeita**: cite-arm consome Introspector como source primário
  (✓), fallback a state legacy preservado mas só toca quando
  Introspector vazio.

  Magnitude **S**. Walk arm passa a apenas descer no `title` quando
  presente. State legacy `bib_entries`/`bib_numbers` ficam vazios em
  paths que usam `layout_with_introspector` (Introspector preenche o
  papel). Backward compat: `layout()` legacy continua a chamar
  `layout_with_introspector(_, _, empty)` — agora o introspector é
  populado pelo próprio walk via `extract_payload` (P181D); cite-arm
  lê do introspector.

  **Atenção**: P181H tem subtileza. Se walk arm não popular
  `state.bib_*` mas `layout()` legacy não tiver introspector
  populado, cite-arm fallback retorna None → fallback `[key]`. Path
  `layout()` legacy precisa migrar para `introspect_with_introspector`
  internamente, OU `layout()` deve aceitar que `layout()` simples
  perde funcionalidade bib (gate substancial). A decisão fica para
  P181H sub-passo `.A`.

- `.I`–`.J`: pendentes.

---

## 6. Pendências cumulativas

P181G migra primeiro consumer real bib. Sem novas pendências;
pré-existentes inalteradas:

- F1 — `CounterStateLegacy` 18 fields (M6).
- F2 — `Content` 59 variants em 3 560 linhas (M6/M9).
- F3 — `Layouter` 19 fields (M6).
- F10 — `format!("{:?}", x)` como hash determinístico.
- DEBT-55 — Bibliography + Cite XL (ADR-0062 PROPOSTO).

**Janela compat reconhecida**: walk arm legacy + state legacy +
copy-sites + cite-arm fallback são todos componentes da janela.
Eliminados gradualmente em **M6** quando F1 retomar — após lacuna
#6 fechar (P181I) e M5 ficar saturado.

---

## 7. Estado pós-passo

- **P181G concluído**.
- **P181H desbloqueado**: walk arm `Content::Bibliography`
  (`introspect.rs:567-573`) torna-se puro:

  ```rust
  // ANTES (P159C/F):
  Content::Bibliography { entries, title } => {
      for entry in entries {
          let next_num = state.bib_numbers.len() as u32 + 1;
          state.bib_numbers.entry(entry.key.clone()).or_insert(next_num);
      }
      state.bib_entries.extend(entries.iter().cloned());
      if let Some(t) = title { walk(t, state, locator, tags, None); }
  }

  // DEPOIS (P181H):
  Content::Bibliography { title, .. } => {
      // Tag emitida pelo topo via extract_payload (P181D);
      // BibStore populado por from_tags arm (P181E).
      // Sem mutação directa de state.bib_* — invariante walk
      // puro P163 restaurada.
      if let Some(t) = title { walk(t, state, locator, tags, None); }
  }
  ```

  **Pré-condição P181H não totalmente satisfeita ainda**: `layout()`
  legacy chama `layout_with_introspector(_, _, empty_introspector)`.
  Sem walk arm a popular state legacy, esse path perde funcionalidade
  bib. Decisão P181H `.A`: ou migrar `layout()` para usar
  `introspect_with_introspector` internamente, ou aceitar regressão
  silenciosa em path legacy (gate substancial).

  Magnitude **S**.

- **Output observable**: inalterado. PDF e diagnostics idênticos
  a P181F.
- **Linhagem**: hashes consistentes via `crystalline-lint
  --fix-hashes`:
  - `rules/layout.md ↔ layout/mod.rs` = `95e8429b ↔ ad89eb8e`

P181G migra primeiro consumer real bib. Próximo: **P181H** torna
walk arm puro — restaura invariante walk puro P163 violada por
P159C/F.
