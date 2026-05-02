# Diagnóstico — M4 consumers restantes (Passo 183A)

**Data**: 2026-05-02
**Passo**: P183A — diagnóstico-primeiro / L0-puro
**Escopo**: migração dos consumers restantes de `CounterStateLegacy` → `Introspector` (M4 série).
**Postura**: zero código tocado em L1–L4; zero testes modificados; produzir decisões + plano executável.

---

## §1 Validação do estado actual

Inspecção empírica em 2026-05-02. Comando: `grep -rn "self\.counter\.\|layouter\.counter\." 01_core/src/rules/layout/ | grep -v tests.rs`.

**12 read-sites totais em `01_core/src/rules/layout/`** (excluindo `tests.rs`, mutations, copy-sites em `mod.rs:1414–1458`, e helpers em `counters.rs` que recebem `counter` como parâmetro):

| # | Site | Read | Estado migração |
|---|------|------|------------------|
| 1 | `equation.rs:33` | `self.counter.is_numbering_active("equation")` | ✅ P182D (fallback `||`) |
| 2 | `equation.rs:97` | `self.counter.get_flat("equation")` | ❌ não migrado |
| 3 | `mod.rs:308` | `self.counter.is_numbering_active("heading")` | ✅ P182D (fallback `||`) |
| 4 | `mod.rs:310` | `self.counter.format_hierarchical("heading")` | ❌ não migrado |
| 5 | `mod.rs:435–439` | `self.counter.figure_numbers.get(kind).get(idx)` | ❌ não migrado |
| 6 | `mod.rs:601` | `self.counter.bib_entries.iter().find(...)` | ✅ P181G (fallback) |
| 7 | `mod.rs:609` | `self.counter.bib_numbers.get(key)` | ✅ P181G (fallback) |
| 8 | `mod.rs:1072` | `doc.extracted_label_pages = self.counter.label_pages` (move) | ❌ não migrado (fixpoint side-channel) |
| 9 | `outline.rs:24` | `layouter.counter.headings_for_toc.clone()` | ❌ não migrado (bloqueado lacuna #3) |
| 10 | `outline.rs:35` | `layouter.counter.known_page_numbers.get(label)` | ❌ não migrado (fixpoint side-channel) |
| 11 | `references.rs:49` | `layouter.counter.figure_label_numbers.get(target)` | ✅ P168 (fallback) |
| 12 | `references.rs:53` | `layouter.counter.resolved_labels.get(target)` | ❌ não migrado |

**Status agregado**: 5 sites já migrados (P168 + P181G + P182D × 2); **7 sites não migrados**.

---

## §2 Inventário trait `Introspector` actual

Métodos existentes no trait (15 total):

| Método | Sub-passo | Suporta consumer não-migrado? |
|--------|-----------|-------------------------------|
| `query_by_kind` | P165 | indirecto |
| `query_by_label` | P165 | indirecto |
| `query_first` / `query_unique` | P165 | indirecto |
| `position_of` | P165 (stub) | — |
| `figure_number_for_label` | P168 | ✅ resolve refs (já em uso) |
| `query_metadata` | P169 | — |
| `formatted_counter(key)` | P170 | **✅ resolve `mod.rs:310` (sítio 4)** |
| `state_value`, `state_final_value` | P171 | indirecto |
| `query` (Selector) | P175 | indirecto |
| `formatted_counter_at(key, location)` | P177 | indirecto |
| `bib_entry_for_key`, `bib_number_for_key` | P181F | ✅ em uso |
| `is_numbering_active` | P182B | ✅ em uso |

**Conclusão**: o trait já cobre **um** dos sítios não migrados (sítio 4: `format_hierarchical` ↔ `formatted_counter`). Os restantes precisam métodos novos OU repensam a arquitectura.

---

## §3 Decisões cláusula 1–6

### Cláusula 1 — Lista exacta de consumers não-migrados

**O1 — Inputs verificáveis**: §1 inventário (12 sites; 7 não migrados).

**O2 — Alternativas consideradas**: classificação grosseira ("4 consumers" P181J §5) vs classificação granular por área funcional.

**O3 — Critério de escolha**: granularidade alinhada com sub-passos S magnitude (cada consumer = 1 sub-passo).

**Decisão**: **5 áreas funcionais não-migradas**, agrupando os 7 sítios:

| Área | Sítios | Método trait | Bloqueio |
|------|--------|--------------|----------|
| **C1**: Heading prefix counter | `mod.rs:310` (`format_hierarchical("heading")`) | **existe** — `formatted_counter("heading")` | nenhum |
| **C2**: Equation counter value | `equation.rs:97` (`get_flat("equation")`) | **novo** — ex. `counter_flat(key) -> usize` ou reusar `formatted_counter` parsed | nenhum |
| **C3**: Figure auto-number per kind | `mod.rs:435–439` (`figure_numbers[kind][idx]`) | **novo** — ex. `figure_number_at_index(kind, idx) -> Option<usize>` | possível: lacuna #1 (kind None vs "image") |
| **C4**: Resolved label text | `references.rs:53` (`resolved_labels.get(target)`) | **novo** — ex. `resolved_label(label) -> Option<&str>` | nenhum (populado em walk arm Heading/Labelled em introspect.rs) |
| **C5**: TOC entries | `outline.rs:24` (`headings_for_toc`) | **novo** ou bloqueado | **bloqueado** — lacuna #3 (body frozen vs hash em tags) |

**Sítios fora do âmbito M4** (fixpoint side-channels):
- `mod.rs:1072` (`label_pages`) — populado pelo Layouter durante layout (`references.rs:28`); não pelo walk; não migrável para Introspector trivialmente porque o write-side é fixpoint, não introspecção.
- `outline.rs:35` (`known_page_numbers`) — análogo; lido pelo Outline a partir de `label_pages` da iteração anterior, injectado em `mod.rs:1458`.

Estes 2 sites permanecem em `CounterStateLegacy` até M5 (walk puro) ou M6 (eliminação) — fora P183.

**O4 — Magnitude**: trivial (classificação).

**O5 — Reversibilidade**: alta — agrupamento pode evoluir.

---

### Cláusula 2 — Bloqueios por consumer

**O1 — Inputs**: `m1-lacunas-captura.md` lacunas abertas (#1, #2, #3) + análise por consumer.

**O2 — Alternativas**: bloqueio "sim" ou "não" para cada consumer.

**O3 — Critério**: presença literal da lacuna no consumer.

**Decisão**:

| Consumer | Bloqueio | Detalhe |
|----------|----------|---------|
| **C1** Heading prefix | ❌ não bloqueado | `formatted_counter` já existe e produz output equivalente |
| **C2** Equation counter | ❌ não bloqueado | método trait novo é trivial |
| **C3** Figure auto-number | ⚠ parcial — lacuna #1 (kind None vs "image" colapso em state) | walk arm canonical em `introspect.rs:391–399` colapsa kind=None para "image"; tags preservam literal. Método trait deve respeitar a convenção do consumer. Aceitável — não bloqueio absoluto, apenas atenção na assinatura |
| **C4** Resolved label | ❌ não bloqueado | método trait `resolved_label(label)` reproduz `state.resolved_labels.get(label).cloned()` |
| **C5** TOC entries | ✅ **BLOQUEADO** — lacuna #3 | tags só guardam hash u128 do body; `headings_for_toc` carrega `Content` body completo para preservar formatação rica em TOC. Migração exige decisão arquitectural sobre body em payload (substancial; M+ ou ADR nova) |

**O4 — Magnitude**: classificação trivial; **bloqueio C5 é substancial** se M4 quiser fechar 5/5.

**O5 — Reversibilidade**: alta — bloqueio pode ser reavaliado.

---

### Cláusula 3 — Métodos trait necessários

**O1 — Inputs**: §2 inventário trait + cláusula 1 inventário consumers.

**O2 — Alternativas**:
- A1: usar métodos existentes onde possível; novos apenas onde necessário.
- A2: criar métodos novos para todos para uniformidade.

**O3 — Critério**: minimalismo (replica P181F que adicionou apenas o necessário).

**Decisão**: **A1**. Métodos por consumer:

| Consumer | Método consumido |
|----------|------------------|
| **C1** Heading prefix | **existente** — `Introspector::formatted_counter(key) -> Option<String>` (P170) |
| **C2** Equation counter | **novo** — `Introspector::flat_counter(key) -> usize` (paralelo a `CounterStateLegacy::get_flat`); default 0; delega a `counters.flat_value(key).unwrap_or(0)` ou similar; OU reusar `formatted_counter` e o caller fazer parse — preferir helper dedicado por simetria com `bib_*_for_key` que encapsula tipo |
| **C3** Figure auto-number per kind | **novo** — `Introspector::figure_number_at_index(kind: &str, index: usize) -> Option<usize>` (replica `state.figure_numbers.get(kind).and_then(\|v\| v.get(idx)).copied()`); fallback `idx + 1` é responsabilidade do caller |
| **C4** Resolved label text | **novo** — `Introspector::resolved_label(label: &Label) -> Option<&str>` (replica `state.resolved_labels.get(label).map(\|s\| s.as_str())`); requer sub-store novo `ResolvedLabelRegistry` em `TagIntrospector` populado por `from_tags` arm Heading/Labelled (paralelo ao walk arm em `introspect.rs:367`) |
| **C5** TOC entries | **bloqueado** — não decidir método trait até lacuna #3 ser resolvida (passo dedicado fora P183) |

**Detalhes operacionais**:

- **C1**: assinatura compatível; chamada inline directa. Migração mais barata.
- **C2**: trabalho ~ trait method (P182B análogo) + impl + 5 tests unit. **Método novo único**.
- **C3**: trabalho ~ trait method + sub-store leitura via `state.figure_numbers` legacy ou via `kind_index[Figure]` indirecto. Sub-store novo provavelmente desnecessário se reusar `kind_index` + payload Figure (já tem `is_counted`); **mais cuidado** — pode revelar dependência circular ou repensar approach.
- **C4**: trabalho ~ trait method + **sub-store novo** `ResolvedLabelRegistry` + populador em `from_tags` (paralelo a P171 StateRegistry). Magnitude **S-M** (mais que C1/C2 individualmente).

**O4 — Magnitude**:
- C1: trivial (sem método novo).
- C2: S (1 método trait + impl + tests).
- C3: S-M (1 método trait + impl + tests, mas semântica per-kind+per-index é não-trivial; pode revelar lacuna #1).
- C4: **S-M** (sub-store novo + populador + método trait + tests).
- C5: bloqueado.

**O5 — Reversibilidade**: alta para C1/C2; média para C3/C4 (sub-store novo).

---

### Cláusula 4 — Ordem de migração

**O1 — Inputs**: dificuldade por consumer (cláusula 3).

**O2 — Alternativas**:
- A: triviais primeiro (C1 → C2 → C3 → C4 → C5/DEBT).
- B: bloqueados primeiro (C5/lacuna #3 antes; arrisca L+).

**O3 — Critério**: P168/P181G estabeleceram que consumers triviais migram fácil; lacuna #3 pode ser L+; bloqueio é localizado e não impede outros.

**Decisão**: **A — triviais primeiro**. Sequência:

1. **P183B** — C1 (heading prefix `formatted_counter`). Magnitude trivial; valida o padrão substitution-with-fallback no contexto M4 e desbloqueia confiança nos próximos.
2. **P183C** — C2 (equation counter `flat_counter`). Magnitude S; método trait novo + impl + tests.
3. **P183D** — C3 (figure auto-number `figure_number_at_index`). Magnitude S-M; método trait novo + impl + tests; cuidado com lacuna #1.
4. **P183E** — C4 (resolved label `resolved_label`). Magnitude S-M; sub-store novo `ResolvedLabelRegistry` + populador `from_tags` + método trait + tests.
5. **P183F** — tests E2E paridade (4 áreas) + relatório de fecho M4 + DEBT M4-residual para C5 (lacuna #3).

**O4 — Magnitude**: trivial (decisão de ordenação).

**O5 — Reversibilidade**: alta.

---

### Cláusula 5 — Forma de migração

**O1 — Inputs**: P168 + P181G + P182D estabeleceram substitution-with-fallback.

**O2 — Alternativas**:
- A: substitution-with-fallback (`introspector.X().or_else(|| counter.X())`).
- B: substituição directa (sem fallback).

**O3 — Critério**: durante M4 (consumer migrado, walk ainda muta state em paralelo), substitution-with-fallback dá segurança contra casos onde o caller passa Introspector vazio (path `layout()` legacy pré-P181H). Substituição directa só faz sentido após M5 (walk puro) ou em call-sites onde o caller é conhecidamente populado.

**Decisão**: **A — substitution-with-fallback**, replica P168/P181G/P182D literalmente.

Nota: para C4 (`resolved_label`), o sub-store novo precisa ser populado por `from_tags` arm; se ainda não for em P183E inicial, fallback a `state.resolved_labels.get(target)` legacy preserva paridade.

**O4 — Magnitude**: trivial (decisão).

**O5 — Reversibilidade**: alta.

---

### Cláusula 6 — Critério de fecho de M4

**O1 — Inputs**: P181/P182 fecharam Opção 3 análoga (infra pronta + consumer migrado; legacy preservado até M6).

**O2 — Alternativas**:
- Opção 1 — 5/5 migrados (exige resolver lacuna #3 dentro de M4).
- Opção 2 — 4/5 migrados; C5 bloqueado-com-DEBT (M4 fecha apesar de lacuna #3).
- Opção 3 — análogo simétrico a P181/P182 — "consumers não-bloqueados migrados; bloqueados ficam como DEBT M4-residual; legacy permanece até M6".

**O3 — Critério**: simetria com P181/P182. Lacuna #3 é trabalho substancial (decisão arquitectural sobre body em payload) — bloquear M4 inteiro até resolver é desproporcional. Opção 3 fecha M4 com 4/5 + DEBT explicitando o residual.

**Decisão**: **Opção 3**. M4 é considerado fechado quando, literalmente:

1. C1, C2, C3, C4 migrados via substitution-with-fallback (consumers consultam Introspector primeiro).
2. C5 (TOC entries) registado como DEBT M4-residual em `auditoria-fresh-projecto.md` F1 — bloqueado pela lacuna #3, requer passo dedicado (não-P183).
3. Tests E2E paridade `layout()` legacy ↔ `layout_with_introspector` directo passam (4 consumers).
4. Pendências M6 herdadas (legacy field + fallbacks + write-paralelo + copy-sites + leituras intra-walk) preservadas.

**O4 — Magnitude**: trivial (definição).

**O5 — Reversibilidade**: alta — Opção 3 mantém legacy paralelo.

---

## §4 Plano de sub-passos sem condicionais (P183B+)

| Sub-passo | Escopo | Magnitude | Depende |
|-----------|--------|-----------|---------|
| **P183B** | C1: Layouter heading prefix em `mod.rs:310` migra para `self.introspector.formatted_counter("heading").or_else(\|\| self.counter.format_hierarchical("heading"))` (substitution-with-fallback). Sem método trait novo. Tests: 2 unit cobrindo Introspector path + fallback path | S | — |
| **P183C** | C2: `Introspector::flat_counter(key) -> usize` adicionado ao trait + impl `TagIntrospector` delega a `counters.flat_value(key).unwrap_or(0)` ou similar; consumer `equation.rs:97` migra para `self.introspector.flat_counter("equation")` (sem fallback necessário se semântica idêntica — confirmar) | S | — |
| **P183D** | C3: `Introspector::figure_number_at_index(kind: &str, index: usize) -> Option<usize>` adicionado; consumer `mod.rs:435–439` migra para `self.introspector.figure_number_at_index(kind_key, idx).or_else(\|\| self.counter.figure_numbers.get(kind_key).and_then(\|v\| v.get(idx)).copied())` | S-M (cuidado lacuna #1) | — |
| **P183E** | C4: sub-store novo `ResolvedLabelRegistry` em `TagIntrospector`; populador em `from_tags` arm Heading/Labelled (paralelo a `introspect.rs:367`); `Introspector::resolved_label(label) -> Option<&str>` adicionado; consumer `references.rs:53` migra com fallback | S-M | — |
| **P183F** | Tests E2E paridade legacy ↔ Introspector para C1–C4; DEBT M4-residual aberto para C5 (TOC entries / lacuna #3); relatório consolidado série P183 (estilo P181J/P182F-consolidado) | S | `.B`–`.E` |

Sem cláusulas condicionais. Sem gates "FULL vs INVENTORY_ONLY". Direcção fixada: A1 + Opção 3 + substitution-with-fallback.

---

## §5 Magnitude consolidada

**S-M cumulativo** (5 sub-passos: P183B trivial + P183C S + P183D S-M + P183E S-M + P183F S).

Estimativa: 5 sub-passos S/S-M; total comparável a P182 (5 sub-passos) ou metade de P181 (9 sub-passos).

Se P183D ou P183E revelarem trabalho L+ inesperado (ex.: lacuna #1 dispara em P183D, ou ResolvedLabelRegistry exige refactor de from_tags), magnitude pode crescer. Auditoria empírica em cada sub-passo `.A` revisa.

---

## §6 ADR — avaliação

P183A não cria ADR. Justificação literal:

- Cláusula 1 (lista) é inventário, não decisão arquitectural.
- Cláusula 3 (métodos trait) reusa padrão P181F (`bib_*_for_key`) + P182B (`is_numbering_active`). Sub-store novo C4 (`ResolvedLabelRegistry`) replica P181B (`BibStore`).
- Cláusula 4 (ordem) e Cláusula 5 (forma) replicam P168/P181G/P182D.
- Cláusula 6 (Opção 3) replica P181/P182.

Caso edge: se P183D dispara lacuna #1 (kind None vs "image") como bloqueio real para `figure_number_at_index`, **ADR PROPOSTO** pode emergir nesse sub-passo. Não é o caso de P183A.

---

## §7 DEBT — avaliação

P183A não abre DEBT. P183F abrirá **DEBT M4-residual** quando consolidar:

- **DEBT-Mx (proposto P183F)**: C5 TOC entries (`outline.rs:24` `headings_for_toc.clone()`) bloqueado pela lacuna #3 (`m1-lacunas-captura.md` linha 43+). Migração para `Introspector::toc_entries()` ou similar exige decisão arquitectural sobre body frozen vs hash em payload — substancial; M+ ou ADR nova. Bloqueia 1/5 consumers M4. Acompanhar até passo dedicado.

Pendências cumulativas inalteradas face a P181J/P182 (legacy fields, walk arm canonical, write paralelo, copy-sites, leituras intra-walk). M6 elimina.

---

## §8 Próximo sub-passo

**P183B** — C1 heading prefix migração via `formatted_counter`.

Escopo concreto:
1. Em `01_core/src/rules/layout/mod.rs:310`:
   ```rust
   if let Some(num_str) = self.introspector
       .formatted_counter("heading")
       .or_else(|| self.counter.format_hierarchical("heading"))
   {
       let prefix = Content::text(format!("{}. ", num_str));
       self.layout_content(&prefix);
   }
   ```
2. Trait `Introspector` import local (replica padrão P181G/P182D).
3. Tests E2E em `mod p183b_*` (irmão do P182D heading-numbering tests):
   - Path Introspector populado: `formatted_counter` retorna `Some("1.1")`; consumer usa.
   - Path fallback: Introspector vazio; consumer cai em legacy.
   - Paridade `layout()` legacy ↔ `layout_with_introspector` para documento típico.
4. Verificações: `cargo test +N`, `crystalline-lint .` zero violations, nenhum L0 modificado (método trait já existe).

Magnitude **trivial** (S). Sem dependência fora de P183A.
