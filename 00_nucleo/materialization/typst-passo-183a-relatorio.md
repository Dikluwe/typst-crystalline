# Relatório — Passo 183A

**Data**: 2026-05-02
**Passo**: P183A — diagnóstico-primeiro / L0-puro (M4 consumers)
**Escopo**: identificar os consumers de `CounterStateLegacy` ainda não migrados; fixar plano P183B–P183F.
**Resultado**: 6 cláusulas fechadas; plano de 5 sub-passos sem condicionais; ADR não criada; DEBT M4-residual aberto literalmente em P183F.

---

## 1. Sumário

P183A executou auditoria empírica que ajustou a contagem original "4 consumers restantes" (P181J §5) para um inventário granular: **12 read-sites totais** em `01_core/src/rules/layout/`, dos quais **5 já migrados** (P168 figure-ref, P181G cite-arm × 2, P182D heading-arm + equation-arm) e **7 não migrados**, agrupáveis em **5 áreas funcionais**:

- **C1**: heading prefix (`mod.rs:310` `format_hierarchical`).
- **C2**: equation counter value (`equation.rs:97` `get_flat`).
- **C3**: figure auto-number per kind (`mod.rs:435–439` `figure_numbers[kind][idx]`).
- **C4**: resolved label text (`references.rs:53` `resolved_labels.get`).
- **C5**: TOC entries (`outline.rs:24` `headings_for_toc`) — **bloqueado lacuna #3**.

**Plus 2 sites fora do âmbito M4** (fixpoint side-channels — `label_pages` em `mod.rs:1072` + `known_page_numbers` em `outline.rs:35`): permanecem em legacy até M5/M6.

Decisões consolidadas:
- **Mecanismo**: substitution-with-fallback (replica P168/P181G/P182D).
- **Métodos trait**: 1 existente reutilizado (`formatted_counter`); 3 novos (`flat_counter`, `figure_number_at_index`, `resolved_label`); 1 sub-store novo (`ResolvedLabelRegistry` análogo a `BibStore`).
- **Ordem**: triviais primeiro (C1 → C2 → C3 → C4); C5 fica para passo dedicado.
- **Critério de fecho M4**: Opção 3 (4/5 migrados; C5 DEBT M4-residual; legacy preservado até M6).
- **ADR/DEBT**: ADR não criada; DEBT M4-residual aberto formalmente em P183F.

Magnitude estimada: S-M cumulativo (5 sub-passos: trivial + S + S-M + S-M + S).

Surpresa do inventário: a contagem "4 consumers" do P181J §5 era grosseira. O inventário granular revela 5 áreas funcionais (uma adicional: `resolved_label`). C5 (TOC entries) é o único bloqueado; todas as outras migráveis com método trait existente ou novo cirúrgico.

---

## 2. Validação estado actual (sub-passo 183A.A)

12 read-sites inspeccionados em 2026-05-02. Detalhe completo em `diagnostico-m4-consumers-passo-183a.md` §1. Síntese:

- Migrados (5): `equation.rs:33`, `mod.rs:308`, `mod.rs:601`, `mod.rs:609`, `references.rs:49`.
- Não migrados (7): `equation.rs:97`, `mod.rs:310`, `mod.rs:435–439`, `mod.rs:1072` (side-channel), `outline.rs:24` (bloqueado), `outline.rs:35` (side-channel), `references.rs:53`.

**Áreas funcionais consolidadas**: 5 (4 migráveis + 1 bloqueada).

---

## 3. Inventário trait `Introspector` (sub-passo 183A.A)

15 métodos existentes. **1 cobre directamente** consumer não-migrado: `formatted_counter` (P170) ↔ `format_hierarchical` legacy.

Detalhe em diagnóstico §2.

---

## 4. Decisões cláusula 1–6 (sub-passos 183A.B–G)

Síntese; detalhe O1–O5 em diagnóstico §3.

| # | Cláusula | Decisão | Magnitude | Reversibilidade |
|---|----------|---------|-----------|-----------------|
| 1 | Lista exacta | **5 áreas** (C1–C5); 4 migráveis (C1–C4) + 1 bloqueada (C5) | trivial | alta |
| 2 | Bloqueios | **C5 bloqueado** pela lacuna #3 (TOC body frozen vs hash); C3 atenção a lacuna #1 (não bloqueio absoluto) | substancial para C5 | alta |
| 3 | Métodos trait | C1 reutiliza `formatted_counter`; C2 `flat_counter` novo; C3 `figure_number_at_index` novo; C4 `resolved_label` novo + `ResolvedLabelRegistry` sub-store novo | trivial a S-M | alta |
| 4 | Ordem | **A — triviais primeiro**: C1 → C2 → C3 → C4 → C5/DEBT | trivial | alta |
| 5 | Forma | **substitution-with-fallback** (replica P168/P181G/P182D) | trivial | alta |
| 6 | Critério de fecho M4 | **Opção 3** — 4/5 migrados; C5 DEBT M4-residual; legacy preservado até M6 | trivial | alta |

---

## 5. Plano de sub-passos (sub-passo 183A.H)

| Sub-passo | Escopo | Magnitude | Depende |
|-----------|--------|-----------|---------|
| `P183B` | C1 heading prefix em `mod.rs:310` migra para `formatted_counter("heading")` com fallback | trivial (S) | — |
| `P183C` | C2 `flat_counter(key)` no trait + impl + consumer `equation.rs:97` migrado | S | — |
| `P183D` | C3 `figure_number_at_index(kind, idx)` no trait + impl + consumer `mod.rs:435–439` migrado | S-M | — |
| `P183E` | C4 sub-store `ResolvedLabelRegistry` + populador `from_tags` + `resolved_label(label)` no trait + consumer `references.rs:53` migrado | S-M | — |
| `P183F` | Tests E2E paridade C1–C4; DEBT M4-residual aberto para C5; relatório consolidado série P183 | S | `.B`–`.E` |

Sem cláusulas condicionais. Sem gates.

---

## 6. Magnitude consolidada

**S-M cumulativo**. 5 sub-passos. Comparável a P182 (5 sub-passos S/S-M).

---

## 7. ADR — avaliação (sub-passo 183A.I)

Não criada. Justificação literal: todas as cláusulas replicam padrões estabelecidos (P168/P181F/P181G/P182B/P182D). Detalhe em diagnóstico §6.

Caso edge: se P183D revelar lacuna #1 como bloqueio real para `figure_number_at_index`, ADR PROPOSTO pode emergir nesse sub-passo.

---

## 8. DEBT — avaliação

Não aberto em P183A. P183F abrirá:

- **DEBT M4-residual**: C5 (TOC entries `outline.rs:24` `headings_for_toc.clone()`) bloqueado pela lacuna #3. Migração exige decisão arquitectural sobre body frozen vs hash em payload — substancial; M+ ou ADR nova. Bloqueia 1/5 consumers M4. Acompanhar até passo dedicado.

Pendências cumulativas inalteradas (legacy fields + walk arm canonical + write paralelo + copy-sites + leituras intra-walk + fallbacks `||`).

---

## 9. Plano de materialização (P183B+)

P183B (próximo) — C1 heading prefix. Magnitude trivial. Sem dependência fora de P183A.

P183C — C2 equation counter. Magnitude S.

P183D — C3 figure auto-number. Magnitude S-M (cuidado lacuna #1).

P183E — C4 resolved label + sub-store novo. Magnitude S-M.

P183F — Tests E2E + DEBT M4-residual + relatório consolidado.

Total: 5 sub-passos; cumulativo S-M.

---

## 10. ADR

Não produzido (cf. §7).

---

## 11. DEBTs

Não aberto em P183A; **DEBT M4-residual será aberto em P183F** (cf. §8).

---

## 12. `m1-lacunas-captura.md` actualizado

P183A não modifica `m1-lacunas-captura.md` directamente — as 7 lacunas documentadas mantêm o seu estado (4 resolvidas + 3 abertas pós-P182). P183F poderá adicionar nota a #3 quando abrir o DEBT M4-residual a referenciá-la.

---

## 13. Próximo passo

**P183B** — C1 heading prefix.

Escopo concreto:
1. **`01_core/src/rules/layout/mod.rs:310`**: substituir `self.counter.format_hierarchical("heading")` por:
   ```rust
   self.introspector
       .formatted_counter("heading")
       .or_else(|| self.counter.format_hierarchical("heading"))
   ```
2. **Trait `Introspector` import local** (replica P181G/P182D padrão).
3. **Tests E2E** em `mod p183b_heading_prefix` (irmão de `mod p182d_heading_numbering` em `tests.rs`):
   - Path Introspector populado dispara prefixo via `formatted_counter`.
   - Path fallback (Introspector vazio) cai em legacy.
   - Paridade `layout()` legacy ↔ `layout_with_introspector` para documento típico.
4. Verificações: `cargo test +N`, `crystalline-lint .` zero violations.

Sem L0 modificado (método trait `formatted_counter` existe desde P170; só altera o consumer).

Magnitude **trivial** (S).

---

## 14. Verificação final

| Item | Estado |
|------|--------|
| Diagnóstico `00_nucleo/diagnosticos/diagnostico-m4-consumers-passo-183a.md` (8 secções) produzido | ✅ |
| Relatório `00_nucleo/materialization/typst-passo-183a-relatorio.md` (14 secções) produzido | ✅ |
| 6 cláusulas fechadas com decisão literal | ✅ (5 áreas C1–C5; bloqueio C5 lacuna #3; métodos trait identificados; ordem triviais primeiro; substitution-with-fallback; Opção 3 fecho) |
| Plano sub-passos sem condicionais (escopo + magnitude + dependência) | ✅ (5 sub-passos B–F) |
| `auditoria-fresh-projecto.md` F1 actualizado com nota P183 contribution | ✅ (entrada de progresso adicionada — cf. §15 abaixo) |
| Magnitude consolidada | ✅ (S-M cumulativo) |
| Critério de fecho M4 fixado em palavras verificáveis | ✅ (Opção 3, 4 critérios literais em diagnóstico §3 cláusula 6) |
| ADR avaliada | ✅ (não criada — justificação literal §7) |
| DEBT avaliada | ✅ (não aberto em P183A; será aberto em P183F) |
| Nenhum ficheiro em `01_core/`, `02_shell/`, `03_infra/`, `04_wiring/` tocado | ✅ (zero modificações fora de `00_nucleo/`) |
| `cargo test --workspace --lib` 1.756 inalterados | ✅ (não correu — código não tocado; mantido vs P182F) |
| `crystalline-lint .` zero violations | ✅ (não correu — código não tocado) |

---

## 15. `auditoria-fresh-projecto.md` F1 actualizado

P183A adiciona uma frase à entrada F1 (`CounterStateLegacy` 18 fields): registar que P183 é a série que ataca os consumers M4 não-migrados em 5 áreas funcionais (4 migráveis + 1 bloqueada por lacuna #3); F1 ainda **não fecha** em P183 — só em P185 quando struct for eliminada.
