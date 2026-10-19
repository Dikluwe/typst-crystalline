# Passo 181A — Relatório (decisões Bib Store, plano P181B+)

**Data**: 2026-05-01
**Natureza**: passo **L0-puro / diagnóstico-primeiro**.
**Zero código tocado**. **Zero testes**. **0 ADR criada**. **0 DEBT
aberto**.
**Precondição**: Passo 180 encerrado (`inventario-bib-state.md`
produzido). Auditoria fresh 2026-04-29 baseline: 1700 tests workspace
verdes; `crystalline-lint .` zero violations; refactor Introspection
P161–P180 fechado; lacuna #6 inventariada com magnitude S-M.

---

## 1. Sumário

P180 inventariou a lacuna #6 (`bib_entries` / `bib_numbers` em
`CounterStateLegacy` populados directamente por walk arm
`Content::Bibliography`) e listou 6 cláusulas a fechar. P181A é
diagnóstico-primeiro — fixa as 6 decisões, valida o plano de
sub-passos `.A`–`.J` proposto, e produz critério de fecho da lacuna
#6 verificável.

**Outputs**:

- `00_nucleo/diagnosticos/diagnostico-bib-store-passo-181a.md`
  (~410 linhas; 8 secções).
- `00_nucleo/materialization/typst-passo-181a-relatorio.md`
  (este ficheiro; 14 secções).
- `m1-lacunas-captura.md` actualizado: linha lacuna #6 evolui de
  "Inventário P180" para "Inventário P180 + decisões P181A —
  magnitude S-M re-confirmada; critério de fecho fixado".
- **Sem ADR nova** (decisões replicam invariantes/padrões P162–P178).
- **Sem DEBT novo** (trabalho residual coberto por F1/M6/DEBT-55).

**Tests cristalino**: 1440 lib core + 215 infra (+6 ignored) + 24
shell = 1679 lib/integration verdes (+ 21 CLI integration → 1700
declarado em auditoria fresh). Inalterados em P181A (zero código
tocado).

`crystalline-lint .` zero violations.

---

## 2. Validação inventário P180 (sub-passo 181A.A)

Tabela 9 itens contra estado actual da `Tekt`:

| Item | Localização P180 | Estado actual | Confirmação |
|------|------------------|---------------|-------------|
| `bib_entries` | `counter_state_legacy.rs:84` | linha 84 | ✓ |
| `bib_numbers` | `counter_state_legacy.rs:92` | linha 92 | ✓ |
| `BibEntry` 16 fields | `bib_entry.rs:82-100` | 16 `pub` fields | ✓ |
| Walk arm Bibliography | `introspect.rs:567` | linha 567 | ✓ |
| Walk arm corpo | `introspect.rs:567-573` | idêntico | ✓ |
| Layouter cite-arm | `layout/mod.rs:584-597` | linhas 584-597 | ✓ |
| Layouter copy-site #1 | `layout/mod.rs:1386-1388` | linhas 1385-1388 | ⚠ shift 1 linha |
| Layouter copy-site #2 | `layout/mod.rs:1414-1416` | linhas 1413-1416 | ⚠ shift 1 linha |
| `extract_bib_entries` | `structural.rs:516` | linha 516 | ✓ |

**Conclusão**: inventário P180 factualmente correcto. Shifts de 1
linha em copy-sites são cosméticos (sem alteração de comportamento)
e não afectam decisões.

Detalhe completo no diagnóstico §1.

---

## 3. Decisões cláusula 1–6 (sub-passos 181A.B–.G)

Síntese das 6 decisões fixadas no diagnóstico §2. Cada uma com
formato O1–O5 + opção literal lá.

| # | Cláusula | Opção fixada | Magnitude |
|---|----------|--------------|-----------|
| 1 | Forma de `BibStore` | `Vec<BibEntry>` + `HashMap<String, u32>` (replica shape actual; sem `IndexMap`) | trivial |
| 2 | Multi-Bibliography concat | `add_bibliography` faz `extend` (replica `state.bib_entries.extend`) | trivial |
| 3 | `bib_numbers` order preservation | `or_insert` mantido (não sobrescreve duplicate) | trivial |
| 4 | Walk arm modificação | **Opção β (walk puro)** — Tag emitida; mutação directa removida; locatable kind adicionado | **substancial** |
| 5 | Layouter cite-arm migração | `Introspector::bib_entry_for_key` + `bib_number_for_key` adicionados ao trait; cite-arm consulta via Introspector (caminho P168) | substancial |
| 6 | Critério de fecho lacuna #6 | **Opção 3** — fecha em "infraestrutura pronta + Layouter migrado"; fields legacy permanecem até M6 | trivial |

A única decisão substancial (cláusula 4) **não introduz padrão novo**
— replica P162/P165/P169/P171/P177/P178.

Detalhes O1–O5 no diagnóstico §2.

---

## 4. Plano de sub-passos revisto (sub-passo 181A.H)

P180 §5 propôs 10 sub-passos `.A`–`.J`. P181A absorve `.A` (validação
+ decisões); restam **9 sub-passos** para P181B+:

| Sub-passo | Escopo | Magnitude | Depende |
|-----------|--------|-----------|---------|
| `.B` | criar `entities/bib_store.rs` (Vec+HashMap+métodos+tests); field em `TagIntrospector` | S | — |
| `.C` | `ElementKind::Bibliography` + `ElementPayload::Bibliography { entries }` | S | `.B` |
| `.D` | `is_locatable(Bibliography) = true` + `extract_payload` arm | S | `.C` |
| `.E` | `from_tags` arm Bibliography popula `BibStore` | S | `.C`, `.B` |
| `.F` | trait `Introspector::bib_entry_for_key` + `bib_number_for_key`; impl em `TagIntrospector` | S | `.B` |
| `.G` | Layouter cite-arm consulta via Introspector | **M** | `.F` |
| `.H` | walk arm Bibliography puro (remove mutação `state.bib_*`); descida em `title` | S | `.E`, `.G` |
| `.I` | tests E2E + lacuna #6 marcada fechada em `m1-lacunas-captura.md` | S | `.H` |
| `.J` | relatório P181 completo | S | `.I` |

**Total**: 8 S + 1 M. Ordem condicionada por β (`.G` precede `.H`
para evitar Layouter ler legacy vazio).

Detalhe completo no diagnóstico §3.

---

## 5. Magnitude consolidada

P180 §4 declarou **S-M (~10 sub-passos; +15-25 tests)**. Pós-P181A:

- 9 sub-passos restantes: 8 S + 1 M.
- Distribuição consistente com P171 (StateRegistry) e P177
  (CounterRegistry hierarchical).
- Estimativa +15-25 tests inalterada.

**S-M re-confirmada**. Sem revisão.

---

## 6. ADR avaliação (sub-passo 181A.I)

P181A **não cria ADR**. Decisões §3 são consequência directa de:

- Pattern sub-store estabelecido por P165/P169/P171/P177 (cláusulas
  1, 2, 3 — replicação literal).
- Invariante walk puro P163 + materialização em P162/P165/P169/P171/
  P177/P178 (cláusula 4 — Opção β).
- Padrão cascade Layouter→Introspector estabelecido por P168 figure-ref
  (cláusula 5).
- Critério de fecho de lacuna #7 (P178 Outline) (cláusula 6 —
  Opção 3).

ADR-0023 (`indexmap`) considerada para cláusula 1 mas **não invocada**
— Opção (a) `Vec`+`HashMap` foi escolhida por simetria com sub-stores
existentes (4/4 usam `HashMap` ou `Vec`; nenhum `IndexMap`).

ADR-0062 (`hayagriva` PROPOSTO) **independente** de P181 — P181 trabalha
sobre o subset minimal cristalino (`Vec<BibEntry>` literal). Promoção
para `IMPLEMENTADO` continua a depender da decisão futura de adoptar
`hayagriva` para CSL parsing.

Detalhes no diagnóstico §5.

---

## 7. DEBT avaliação

P181A **não abre DEBT novo**.

| Item residual | Cobertura existente |
|---------------|---------------------|
| Eliminação de `bib_entries`/`bib_numbers` de `CounterStateLegacy` | F1 (auditoria fresh) + M6 roadmap |
| Layouter copy-sites (1385-1388, 1413-1416) durante janela compat | M6 (desaparecem com fields legacy) |
| Paridade hayagriva 100% | DEBT-55 + ADR-0062 |

Detalhes no diagnóstico §6.

---

## 8. Plano de materialização (sub-passos P181B+)

Sequência crítica:

| Passo | Escopo | Magnitude | ADR? |
|-------|--------|-----------|------|
| **P181B** | `entities/bib_store.rs` + field em `TagIntrospector` + L0 | S | — |
| **P181C** | `ElementKind::Bibliography` + `ElementPayload::Bibliography` | S | — |
| **P181D** | `is_locatable` + `extract_payload` arm Bibliography | S | — |
| **P181E** | `from_tags` arm Bibliography → `bib_store.add_bibliography` | S | — |
| **P181F** | trait `Introspector::bib_entry_for_key` + `bib_number_for_key` | S | — |
| **P181G** | Layouter cite-arm via Introspector | M | — |
| **P181H** | walk arm Bibliography puro | S | — |
| **P181I** | tests E2E + lacuna #6 marcada fechada | S | — |
| **P181J** | relatório consolidado | S | — |

**Sem ADR adicional** prevista. Cada sub-passo segue Protocolo de
Nucleação (auditoria L0 → redacção L0 → humano calcula hash → testes
falham → implementação → linhagem → validação).

---

## 9. ADR

P181A não produz ADR. Esperado e justificado em §6 e no diagnóstico §5.

---

## 10. DEBTs

P181A não abre DEBT. Esperado e justificado em §7 e no diagnóstico §6.

---

## 11. `m1-lacunas-captura.md` actualizado

Linha 89 (entrada lacuna #6, secção "Lacunas adicionais — detectadas
em P167") evolui:

**Antes (pós-P180)**:

```
**Inventário P180** (`inventario-bib-state.md`): magnitude **S-M**
confirmada. Recomendação: implementação directa em P181 via padrão
sub-store + locatable kind (replicação de P165/P169/P171/P178
estabelecidos). 10 sub-passos planeados; ~+15-25 tests.
```

**Depois (pós-P181A)**:

```
**Inventário P180 + decisões P181A** (`diagnostico-bib-store-passo-181a.md`):
magnitude **S-M** re-confirmada. 6 cláusulas fixadas: BibStore =
Vec<BibEntry>+HashMap<String,u32>; concat replica `extend`; or_insert
mantido; walk Opção β (Tag + locatable kind); Layouter cite-arm via
Introspector (caminho P168); critério de fecho Opção 3
(infraestrutura+consumer migrado, fields legacy preservados até M6).
9 sub-passos `.B`–`.J` validados (~+15-25 tests). Próximo: **P181B**
materializa `entities/bib_store.rs`.
```

Linha 112 (tabela "Resumo", coluna "Decisão" da lacuna #6) evolui:

**Antes**:

```
**Inventário P180**: magnitude S-M; recomendação implementação directa
P181 (sub-store + locatable kind)
```

**Depois**:

```
**P181A decisões fixadas**: 6 cláusulas resolvidas; 9 sub-passos
.B–.J validados; critério de fecho fixado (Opção 3); próximo P181B
```

---

## 12. README dos ADRs actualizado

P181A **não altera** `00_nucleo/adr/README.md` — sem ADR criada nem
estado de ADR existente alterado. README permanece com 60 ADRs (1
omitido por revogação) e 11 `PROPOSTO` (incluindo ADR-0062 hayagriva
inalterado).

---

## 13. Próximo passo

**P181B** — primeira materialização do plano:

1. Auditoria L0: confirmar que não existe prompt prévio para
   `bib_store`.
2. Redigir `00_nucleo/prompts/entities/bib_store.md` com spec literal
   (campos, métodos, invariantes, tests obrigatórios).
3. Humano confirma e calcula `@prompt-hash`.
4. Tests primeiro (`#[cfg(test)]` em `bib_store.rs`) — confirmar que
   falham.
5. Implementação `01_core/src/entities/bib_store.rs` (~150-200 linhas
   + 5-7 tests).
6. Adicionar field `bib_store: BibStore` a `TagIntrospector` +
   método `pub fn bib_store(&self) -> &BibStore`.
7. Header de linhagem.
8. `cargo build && cargo test --workspace --lib` verde.
9. `crystalline-lint .` zero violations.

Magnitude P181B: **S**. Sem dependências externas.

Sequência cumulativa: P181B → P181C → P181D → P181E → P181F →
P181G → P181H → P181I → P181J.

---

## 14. Verificação final

| Item | Estado |
|------|--------|
| Diagnóstico em `00_nucleo/diagnosticos/diagnostico-bib-store-passo-181a.md` (8 secções) | ✅ |
| Relatório em `00_nucleo/materialization/typst-passo-181a-relatorio.md` (14 secções) | ✅ |
| 6 cláusulas P180 §6 fechadas com decisão literal | ✅ |
| Plano de 9 sub-passos validado (tabela com escopo + magnitude + dependência) | ✅ |
| `m1-lacunas-captura.md` actualizado (linha 89 + tabela §Resumo) | ✅ |
| Magnitude S-M re-confirmada | ✅ |
| Critério de fecho lacuna #6 fixado em palavras verificáveis | ✅ |
| ADR avaliada (não necessária; justificação literal §6) | ✅ |
| DEBT avaliado (não necessário; justificação literal §7) | ✅ |
| Nenhum ficheiro em `01_core/`, `02_shell/`, `03_infra/`, `04_wiring/` tocado | ✅ |
| `cargo test --workspace --lib`: 1440 + 215 + 24 = 1679 (+ 21 CLI integration) inalterados | ✅ |
| `crystalline-lint .`: zero violations | ✅ |
| README dos ADRs não alterado (sem ADR nova) | ✅ |
| Relatório do passo escrito | ✅ |

**Pós-181A**:

- 0 ADRs novas; 0 DEBTs novos.
- `m1-lacunas-captura.md` linha lacuna #6 refinada com decisões
  fixadas.
- 9 sub-passos `.B`–`.J` planeados com magnitude individual S/M e
  ordem de dependência explicitada.
- Critério de fecho lacuna #6 verificável sem julgamento subjectivo
  (3 itens enumerados em diagnóstico §2.6 + §7).
- **Próximo substantivo**: P181B (materializa `bib_store.rs`).

Padrão diagnóstico-primeiro continua a aplicar-se (6ª aplicação:
131A/132A/140A/148/154A/181A).

P181A é instrumento. Implementação concreta da lacuna #6 começa em
P181B.
