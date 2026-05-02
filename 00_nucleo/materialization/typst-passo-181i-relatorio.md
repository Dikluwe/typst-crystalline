# Passo 181I — Relatório (Tests E2E + lacuna #6 fechada)

**Data**: 2026-05-01
**Natureza**: passo **validação + documentação** — pipeline E2E
codificado em tests (protege contra regressão futura); lacuna #6
formalmente fechada em `m1-lacunas-captura.md`.
**Pré-condição**: P181H concluído. Pipeline completo (P181B → P181H).

---

## 1. Sumário

5 tests E2E componente (módulo `p181i_e2e_bib` em
`layout/tests.rs`) codificam invariantes do pipeline bib pós-P181:

1. **`pipeline_completo_bib_state_via_layout_legacy`** — path
   `introspect() + layout()` legacy renderiza cite Normal `[1]`/`[2]`
   correctamente após P181H (Introspector populado internamente).
2. **`walk_puro_state_legacy_vazio_em_producao`** — confirma walk
   puro restaurado: `state.bib_*` vazio; BibStore populado.
3. **`multi_bibliography_concat_replica_clausula_2_p181a`** —
   cláusula 2 P181A: 2+2 entries → `len() == 4`.
4. **`or_insert_preserva_primeiro_numero_clausula_3_p181a`** —
   cláusula 3 P181A: keys duplicadas preservam primeiro número.
5. **`cite_4_forms_via_layout_with_introspector`** — 4 cite forms
   (Normal/Prose/Author/Year) renderizam correctamente via path
   Introspector.

**Lacuna #6 formalmente fechada**: entrada em
`m1-lacunas-captura.md` linha 89 actualizada para "✅ **Resolvida
em P181**" com mecanismo detalhado e 3 critérios verificados.
Tabela §"Resumo" linha 112 também marcada ✅.

**Outputs**:

- `01_core/src/rules/layout/tests.rs` (módulo `p181i_e2e_bib` com
  5 tests).
- `00_nucleo/diagnosticos/m1-lacunas-captura.md` (lacuna #6
  marcada ✅; mecanismo + critérios documentados).

**Sem ADR nova**. **Sem DEBT novo**. **Nenhum código de produção
modificado** (P181B–H já materializaram tudo).

---

## 2. Verificações `.D` (todas confirmadas)

| Item | Estado |
|------|--------|
| 1. `cargo check --workspace` passa | ✅ |
| 2. `cargo test --workspace --lib`: 1473 → **1478** (+5; estimativa era +4) | ✅ |
| 3. `crystalline-lint .` zero violations | ✅ |
| 4. Tests E2E P181I cobrem pipeline completo + walk puro + multi-Bib + or_insert + 4 forms | ✅ |
| 5. `m1-lacunas-captura.md`: lacuna #6 ✅ resolvida; 3 critérios documentados | ✅ |
| 6. Walk **NÃO modificado** | ✅ |
| 7. `from_tags` **NÃO modificado** | ✅ |
| 8. `Introspector` trait **NÃO modificado** | ✅ |
| 9. Layouter **NÃO modificado** | ✅ |
| 10. Sub-store `BibStore` **NÃO modificado** | ✅ |
| 11. Snapshot tests ADR-0033 verdes (215 infra integration; 6 ignored) | ✅ |
| 12. Linter passa final | ✅ |

---

## 3. Decisões registadas em `.A`

- **3 critérios P181A §2.6 verificados literalmente**:
  - Critério 1: `ls 01_core/src/entities/bib_store.rs` → existe.
  - Critério 2: `grep "bib_entry_for_key\|bib_number_for_key"
    introspector.rs` → 6 ocorrências (2 trait + 2 impl + 2 tests);
    `grep "ElementPayload::Bibliography { entries }" from_tags.rs`
    → arm activa em linha 137.
  - Critério 3: `grep "self.introspector" layout/mod.rs` → linhas
    591 e 599 (cite-arm consultando Introspector).

- **Opção B (E2E componente)** confirmada: tests constroem
  `Content` directamente (sem parser/eval). Replica padrão
  estabelecido P162–P181G. Sem dependência em parser.

- **Localização tests**: `01_core/src/rules/layout/tests.rs`
  módulo `p181i_e2e_bib` (paralelo a `p168_figure_ref_migration`,
  `p181g_cite_arm_migration`, `p169_metadata_feature`, etc.).

- **Helpers reutilizados**:
  - `BibEntry::new(key, author, title, year)` (constructor existente).
  - `Content::cite(key, supplement, form)` + `Content::bibliography(entries, title)`
    (constructors existentes).
  - `introspect()`, `introspect_with_introspector()`, `layout()`,
    `layout_with_introspector()` (entry points).
  - `plain_text()` em `PagedDocument` (helper existente).

  Helper local `bib(key)` adicionado para construir `BibEntry` mínimo
  com author/title/year padrão.

---

## 4. Δ tests vs baseline P181H

| Suite | Baseline P181H | Pós-P181I | Δ |
|-------|----------------|-----------|---|
| core lib | 1473 | **1478** | +5 |
| infra integration | 215 (+6 ignored) | 215 (+6 ignored) | 0 |
| shell | 24 | 24 | 0 |
| **Total lib/integration verdes** | 1712 | **1717** | **+5** |
| CLI integration (separado) | 21 | 21 | 0 |
| **Total auditoria fresh** | 1733 | **1738** | **+5** |

Estimativa P181I: **+4**. **Actual +5** — adicionei 1 teste extra
(`cite_4_forms_via_layout_with_introspector`) que cobre os 4 cite
forms num único loop. Cobertura mais densa sem custo arquitectural.

---

## 5. Estado de M9 e P181

**M9 features**: **10/11** (Bibliography conta agora — lacuna #6
fecha = feature bib materializada). Restante: lacuna #4
(`numbering_active`) — infraestrutura pronta P171, consumer aguarda.

**Lacuna #6**: ✅ **Resolvida em P181**.
- `m1-lacunas-captura.md` linha 89: "✅ Resolvida em P181"
  com mecanismo detalhado.
- Tabela §"Resumo" linha 112: ✅ marcada.
- 3 critérios P181A §2.6 (Opção 3) documentados como verificados.

**P181**:

- `.A`–`.I`: ✅ concluídos.
- `.J` (P181J): pendente — relatório consolidado de todos os
  sub-passos, métricas finais, lições aprendidas.

---

## 6. Pendências cumulativas

P181I encerra a fase de implementação P181. Pendências
pré-existentes inalteradas:

- **F1** — `CounterStateLegacy` 18 fields (M6).
- **F2** — `Content` 59 variants em 3 560 linhas (M6/M9).
- **F3** — `Layouter` 19 fields (M6).
- **F10** — `format!("{:?}", x)` como hash determinístico.
- **DEBT-55** — Bibliography + Cite XL (ADR-0062 PROPOSTO).

**Lacuna #4** (`numbering_active`) continua aberta — próxima
candidata para fechar M9 = 11/11.

**M6 elimina** (quando F1 retomar):
- `CounterStateLegacy.bib_entries` e `bib_numbers` (vazios em
  produção pós-P181H).
- Copy-sites em `pub fn layout`/`pub fn layout_with_introspector`
  (linhas 1397, 1399, 1425, 1427).
- Cite-arm fallback a state legacy.
- Re-walk em `layout()` legacy quando callers adoptarem
  `introspect_with_introspector + layout_with_introspector`
  directamente.

---

## 7. Estado pós-passo

- **P181I concluído**.
- **Lacuna #6**: ✅ **Resolvida em P181**.
- **M9**: **10/11 features** (saltou de 9/11 para 10/11 em P181I).
- **P181J desbloqueado**: relatório consolidado documentando todos
  os 9 sub-passos materializados, métricas agregadas, ADRs
  envolvidas (nenhuma nova; ADR-0062 PROPOSTO referenciada),
  lições aprendidas (especialmente: instrução vs realidade do
  código em P181E e P181H; padrão substitution-with-fallback P168
  replicado com sucesso pela 2ª vez em P181G).

- **Output observable**: inalterado. PDF e diagnostics idênticos
  a P181H.

P181I valida E2E o pipeline completo bib state e fecha
formalmente a lacuna #6. Próximo: **P181J** consolida
materialização completa P181 num relatório único.
