# Passo 181D — Relatório (`is_locatable` + `extract_payload` arms para Bibliography)

**Data**: 2026-05-01
**Natureza**: passo **infra-pura** — `Content::Bibliography` activado
como kind locatable; walk emite Tag::Start automaticamente via
`extract_payload`. Walk arm legacy (`introspect.rs:567-573`) e
`from_tags` arm Bibliography (no-op) **inalterados** —
redundância intencional documentada na instrução §"O que pode sair
errado".
**Pré-condição**: P181C concluído. `ElementKind::Bibliography` e
`ElementPayload::Bibliography { entries: Vec<BibEntry> }` existem.

---

## 1. Sumário

`Content::Bibliography { .. }` move-se de não-locatable para
locatable em `is_locatable.rs` (P164 invariante exhaustivo
preservado). `extract_payload.rs` ganha arm
`Bibliography { entries, .. } => Some(ElementPayload::Bibliography
{ entries: entries.clone() })` antes do `_ => None` fall-through.

**Consequência directa (sem mudança em walk)**: o topo de `walk()`
em `introspect.rs:329-335` chama `extract_payload(content)` antes
de qualquer mutação; agora retorna `Some(Bibliography {...})` e
emite `Tag::Start(loc, ElementInfo { payload: Bibliography {...},
label: ... })` automaticamente para cada `Content::Bibliography`.
A tag flui pelo `Vec<Tag>` mas `from_tags.rs::Bibliography arm`
continua **no-op** (P181C); `BibStore` permanece vazio em
produção até P181E.

Walk arm `Content::Bibliography` (linha 567-573) **não foi
tocado** — continua a mutar `state.bib_*` legacy. Redundância
durante a transição. P181H removerá a mutação directa após
P181G migrar Layouter.

**Outputs**:

- `00_nucleo/prompts/rules/introspect/locatable.md` (L0;
  hash final `bdae0a1f`).
- `00_nucleo/prompts/rules/introspect/extract_payload.md` (L0;
  hash final `1da1c130`).
- `01_core/src/rules/introspect/locatable.rs` (arm Bibliography
  movido + 1 test; linhagem `d26cf6ff`).
- `01_core/src/rules/introspect/extract_payload.rs` (arm novo +
  3 tests + helper `bib_entry`; linhagem `a30fd785`).

**Sem ADR nova**. **Sem DEBT novo**.

---

## 2. Verificações `.H` (todas confirmadas)

| Item | Estado |
|------|--------|
| 1. `cargo check --workspace` passa | ✅ |
| 2. `cargo test --workspace --lib`: 1454 → **1458** (+4) | ✅ |
| 3. `crystalline-lint .` zero violations | ✅ |
| 4. L0 `locatable.md` actualizado com hash `bdae0a1f` | ✅ |
| 5. L0 `extract_payload.md` actualizado com hash `1da1c130` | ✅ |
| 6. L1 `locatable.rs` linhagem `@prompt-hash d26cf6ff` | ✅ |
| 7. L1 `extract_payload.rs` linhagem `@prompt-hash a30fd785` | ✅ |
| 8. `is_locatable(&Content::Bibliography {..}) == true` | ✅ (test `bibliography_e_locatable`) |
| 9. `extract_payload(&Content::Bibliography {..})` retorna `Some(Bibliography { entries })` | ✅ (3 tests P181D) |
| 10. Invariante `is_locatable == extract_payload.is_some()` para Bibliography | ✅ (assert no test `bibliography_e_locatable`) |
| 11. Walk **NÃO modificado estruturalmente**; consequência directa: tag emitida automaticamente via `extract_payload` no topo (`introspect.rs:329`) | ✅ |
| 12. `from_tags` arm Bibliography continua **no-op** (P181C) — `BibStore` vazio em produção | ✅ |
| 13. Walk arm `Content::Bibliography` em `walk()` (linha 567-573) inalterado — continua a popular `state.bib_*` legacy | ✅ |
| 14. Layouter **NÃO modificado** | ✅ |
| 15. Snapshot tests ADR-0033 verdes (215 infra integration; 6 ignored) | ✅ |
| 16. Linter passa final | ✅ |

---

## 3. Decisões registadas em `.A`

- **`Content::Bibliography` campos**: `entries: Vec<crate::entities::bib_entry::BibEntry>`
  + `title: Option<Box<Content>>` (`content.rs:538-541`). `extract_payload`
  arm captura `entries` e ignora `title` via `..` — `title` não é
  relevante para introspecção (Layouter consome via path separado quando
  renderiza).
- **Padrão P178 (Outline) replicado literalmente**: arm dedicado em
  bloco "Locatable" do `is_locatable.rs` match exhaustivo; arm em
  `extract_payload.rs` antes do `_ => None`.
- **L0 `locatable.md` §"Cobertura M1" desactualizada**: cabeçalho
  declarava "Locatable (4)" mas estado real era 7 (incluindo Outline
  P178). Actualizado para 8 com Bibliography (renomeado para
  "Cobertura (P164 baseline; expandido em P169/P171/P178/P181D)") com
  enumeração explícita.

---

## 4. Δ tests vs baseline P181C

| Suite | Baseline P181C | Pós-P181D | Δ |
|-------|----------------|-----------|---|
| core lib | 1454 | **1458** | +4 |
| infra integration | 215 (+6 ignored) | 215 (+6 ignored) | 0 |
| shell | 24 | 24 | 0 |
| **Total lib/integration verdes** | 1693 | **1697** | **+4** |
| CLI integration (separado) | 21 | 21 | 0 |
| **Total auditoria fresh** | 1714 | **1718** | **+4** |

Estimativa P181D: **+4** (1 locatable + 3 extract_payload). **Actual
+4** — exactamente conforme estimado.

Tests novos:

- `locatable::bibliography_e_locatable` (cobre is_locatable + assert
  invariante extract_payload.is_some()).
- `extract_payload::bibliography_produz_some_payload_com_entries`.
- `extract_payload::bibliography_clona_entries_para_payload`.
- `extract_payload::bibliography_com_title_continua_a_extrair_apenas_entries`.

---

## 5. Estado de M9 e P181

**M9 features**: 9/11 (sem alteração — feature bib não conta até
P181I fechar lacuna #6).

**P181**:

- `.A` (P181A): ✅ concluído (decisões + plano).
- `.B` (P181B): ✅ concluído (`BibStore` + field).
- `.C` (P181C): ✅ concluído (`ElementKind::Bibliography` +
  `ElementPayload::Bibliography`).
- `.D` (P181D): ✅ concluído (este relatório).
- `.E` (P181E): pendente — `from_tags` arm Bibliography popula
  `BibStore` via `add_bibliography(entries) + assign_number(key, n)`
  em loop. **Pré-condições satisfeitas**: arm defensivo no-op existe
  desde P181C; tag fluindo desde P181D.
- `.F`–`.J`: pendentes.

---

## 6. Pendências cumulativas

P181D activa pipeline tag para Bibliography mas mantém path legacy.
Sem novas pendências; pré-existentes inalteradas:

- F1 — `CounterStateLegacy` 18 fields (M6).
- F2 — `Content` 59 variants em 3 560 linhas (M6/M9).
- F3 — `Layouter` 19 fields (M6).
- F10 — `format!("{:?}", x)` como hash determinístico — agora cobre
  payloads Bibliography também. Risco F10 inalterado em magnitude.
- DEBT-55 — Bibliography + Cite XL (ADR-0062 PROPOSTO).

**Redundância P181D-E-G-H reconhecida**: walk arm continua a popular
`state.bib_*` legacy; tag emitida via `extract_payload`; `from_tags`
arm é no-op (até P181E); Layouter consome `state.bib_*` legacy
(até P181G migrar). Mutação directa em walk arm é removida em
P181H, depois de Layouter migrar. Path duplo é janela controlada de
transição — output observable garantido inalterado.

---

## 7. Estado pós-passo

- **P181D concluído**.
- **P181E desbloqueado**: substituir o no-op
  `ElementPayload::Bibliography { .. } => {}` em
  `from_tags.rs:128` (arm defensivo P181C) por:

  ```rust
  ElementPayload::Bibliography { entries } => {
      intr.kind_index
          .entry(ElementKind::Bibliography)
          .or_default()
          .push(*loc);
      let entries_owned = entries.clone();
      for entry in &entries_owned {
          let next_num = intr.bib_store.len() as u32 + 1;
          intr.bib_store.assign_number(entry.key.clone(), next_num);
      }
      intr.bib_store.add_bibliography(entries_owned);
  }
  ```

  Magnitude S (~10 linhas + ~3 tests E2E que validam paridade
  `BibStore` vs `state.bib_*`).

- **Output observable**: inalterado. PDF e diagnostics idênticos a
  P181C.
- **Linhagem**: hashes consistentes via `crystalline-lint
  --fix-hashes`:
  - `locatable.md ↔ locatable.rs` = `bdae0a1f ↔ d26cf6ff`
  - `extract_payload.md ↔ extract_payload.rs` = `1da1c130 ↔ a30fd785`

P181D liga Bibliography ao pipeline de tags. Próximo: **P181E**
populará `BibStore` via `from_tags` (a partir das tags que P181D
começou a emitir).
