# Passo 181C — Relatório (`ElementKind::Bibliography` + `ElementPayload::Bibliography`)

**Data**: 2026-05-01
**Natureza**: passo **infra-pura** — variants adicionados aos enums
discriminadores; nenhum produtor / consumer real ainda. Replica
padrão P169/P171/P178.
**Pré-condição**: P181B concluído. `BibStore` em `entities/`, field
público `pub bib_store: BibStore` em `TagIntrospector` (vazio em
produção).

---

## 1. Sumário

`ElementKind::Bibliography` (variant 8 → 9) e
`ElementPayload::Bibliography { entries: Vec<BibEntry> }` adicionados
aos enums discriminadores. `as_str() == "bibliography"`,
`from_name("bibliography") == Some(Bibliography)` — round-trip
completo.

Hash via Debug existente cobre o variant novo (impl manual em
`ElementPayload::hash` usa `format!("{:?}", self).hash(state)`;
`BibEntry` deriva `Debug` desde P159A — sem alteração ao hash impl).

**Outputs**:

- `00_nucleo/prompts/entities/element_kind.md` (L0 actualizado;
  hash final `b07af9bf`).
- `00_nucleo/prompts/entities/element_payload.md` (L0 actualizado;
  hash final `af47c732`).
- `01_core/src/entities/element_kind.rs` (variant + 2 arms +
  3 tests; linhagem `00427273`).
- `01_core/src/entities/element_payload.rs` (variant + 3 tests;
  linhagem `f7121de5`).
- `01_core/src/rules/introspect/from_tags.rs` (arm defensivo
  no-op `ElementPayload::Bibliography { .. } => {}`).

**Sem ADR nova**. **Sem DEBT novo**.

---

## 2. Verificações `.G` (todas confirmadas)

| Item | Estado |
|------|--------|
| 1. `cargo check --workspace` passa | ✅ |
| 2. `cargo test --workspace --lib`: 1448 → **1454** (+6) | ✅ |
| 3. `crystalline-lint .` zero violations | ✅ |
| 4. L0 `element_kind.md` actualizado com hash `b07af9bf` | ✅ |
| 5. L0 `element_payload.md` actualizado com hash `af47c732` | ✅ |
| 6. L1 `element_kind.rs` linhagem `@prompt-hash 00427273` | ✅ |
| 7. L1 `element_payload.rs` linhagem `@prompt-hash f7121de5` | ✅ |
| 8. `ElementKind::Bibliography` existe | ✅ |
| 9. `ElementPayload::Bibliography { entries: Vec<BibEntry> }` existe | ✅ |
| 10. `ElementKind::from_name("bibliography") == Some(Bibliography)` | ✅ |
| 11. Walk **NÃO modificado** (`introspect.rs:567-573` inalterado) | ✅ |
| 12. `is_locatable` / `extract_payload` **NÃO modificados** — `Content::Bibliography` continua não-locatable | ✅ |
| 13. `BibStore` (P181B) **NÃO populado** — arm em `from_tags` é defensivo no-op | ✅ |
| 14. Layouter **NÃO modificado** | ✅ |
| 15. Snapshot tests ADR-0033 verdes (215 infra integration, 6 ignored) | ✅ |
| 16. Linter passa final | ✅ |

**Desvio mínimo registado**: `from_tags.rs` recebeu **arm defensivo**
`ElementPayload::Bibliography { .. } => {}` (4 linhas + comentário 3
linhas). A instrução .G item 12 lista `from_tags` como "NÃO
modificados", mas a instrução §"O que pode sair errado" antecipa
explicitamente:

> Variant novo em ElementKind/ElementPayload força exhaustive matches:
> [...] Compilador guia. Cada arm novo retorna defensive ou reuso.
> **Aceitável.**

`from_tags` tem match exhaustivo sobre `ElementPayload` (sem `_ =>
...`); adicionar variant sem arm causaria `error[E0004]`. O arm é
no-op até P181E ligar a população. Nenhuma lógica de
`bib_store.add_bibliography` foi adicionada — esse é o trabalho
substantivo de P181E.

---

## 3. Decisões registadas em `.A`

- **`BibEntry` Hash**: não deriva `Hash` (16 fields, sem necessidade
  identificada). Mas `ElementPayload` tem `impl Hash` manual via
  `format!("{:?}", self).hash(state)` desde P169 — `BibEntry` deriva
  `Debug` (linha 80 `bib_entry.rs`), logo o hash impl cobre
  `ElementPayload::Bibliography { entries }` automaticamente. **Sem
  workaround adicional necessário** — confirmado por test
  `bibliography_hash_diferente_para_entries_distintos`.
- **Variant payload exacto**:
  `Bibliography { entries: Vec<crate::entities::bib_entry::BibEntry> }`
  (path qualificado — convenção `Metadata`/`State` em
  `element_payload.rs` que usam `Box<crate::entities::value::Value>`).
- **Localização do helper `from_name`**: confirmado em
  `element_kind.rs:50-67` (criado por P175; arm `outline` adicionado
  em P178; agora arm `bibliography` adicionado).

---

## 4. Δ tests vs baseline P181B

| Suite | Baseline P181B | Pós-P181C | Δ |
|-------|----------------|-----------|---|
| core lib | 1448 | **1454** | +6 |
| infra integration | 215 (+6 ignored) | 215 (+6 ignored) | 0 |
| shell | 24 | 24 | 0 |
| **Total lib/integration verdes** | 1687 | **1693** | **+6** |
| CLI integration (separado) | 21 | 21 | 0 |
| **Total auditoria fresh** | 1708 | **1714** | **+6** |

Estimativa P181C era +5 (3 element_kind + 2 element_payload). Actual
**+6** — adicionei 3+3 (cobertura simétrica entre os dois enums):

- `element_kind`: `bibliography_existe_e_distinto`,
  `bibliography_as_str`, `from_name_bibliography`.
- `element_payload`: `bibliography_constroi_e_compara`,
  `bibliography_distinto_de_outras_variants`,
  `bibliography_hash_diferente_para_entries_distintos` (cobertura
  explícita do impl Hash manual via Debug).

Sem custo arquitectural — apenas cobertura mais densa.

---

## 5. Estado de M9 e P181

**M9 features**: 9/11 (sem alteração). Variants existem mas ninguém
os produz; lacuna #6 ainda aberta.

**P181**:

- `.A` (P181A): ✅ concluído (decisões + plano).
- `.B` (P181B): ✅ concluído (`BibStore` + field).
- `.C` (P181C): ✅ concluído (este relatório).
- `.D` (P181D): pendente — `is_locatable(Content::Bibliography) =
  true` + `extract_payload` arm Bibliography. **Pré-condição
  satisfeita**: `ElementPayload::Bibliography` existe.
- `.E`–`.J`: pendentes.

---

## 6. Pendências cumulativas

P181C é puro infra; sem novas pendências. Pendências pré-existentes
inalteradas:

- F1 — `CounterStateLegacy` 18 fields (M6).
- F2 — `Content` 59 variants em 3 560 linhas (M6/M9).
- F3 — `Layouter` 19 fields + match 101 arms (M6).
- F10 — `format!("{:?}", x)` como hash determinístico — confirmado
  como mecanismo activo para o variant novo. Risco F10 inalterado
  (Bibliography apenas adiciona um path mais ao hash já existente).
- DEBT-55 — Bibliography + Cite XL (hayagriva ADR-0062 PROPOSTO).

---

## 7. Estado pós-passo

- **P181C concluído**.
- **P181D desbloqueado**: actualizar `is_locatable.rs` (mover
  `Content::Bibliography` do bloco non-locatable para locatable) e
  `extract_payload.rs` (adicionar arm Bibliography que retorna
  `Some(ElementPayload::Bibliography { entries: entries.clone() })`).
  Magnitude S. Toca 2 ficheiros L1 + 2 L0s + ~3 tests.
- **Output observable**: inalterado. PDF e diagnostics idênticos.
- **Linhagem**: hashes consistentes via `crystalline-lint
  --fix-hashes`:
  - `element_kind.md ↔ element_kind.rs` = `b07af9bf ↔ 00427273`
  - `element_payload.md ↔ element_payload.rs` = `af47c732 ↔ f7121de5`

P181C é fundação. Próximo: **P181D** liga `Content::Bibliography` ao
mecanismo de extracção de payload (cascade Opção β walk puro,
decisão P181A cláusula 4).
