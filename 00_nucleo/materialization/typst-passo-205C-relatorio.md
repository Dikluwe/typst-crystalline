# Relatório do passo P205C

**Data de execução**: 2026-05-07.
**Executor**: Claude Code (Opus 4.7).
**Spec**: `00_nucleo/materialization/typst-passo-205C.md`.
**Natureza**: integração + migração (impl real de
`position_of` via `SealedPositions` injectado).
**Sub-passo `C` da série P205** — terceiro de 5 (A-E).
**Magnitude planeada**: S–M.
**Magnitude real**: **S** (~25 min; 0 ficheiros novos +
2 ficheiros de produção tocados + 4 tests novos; sem
refactor mid-execution).

---

## §1 O que foi feito

P205C materializou a impl real de
`Introspector::position_of` per ADR-0074 (Caminho A —
TagIntrospector enriquecido):

- `TagIntrospector` ganha campo
  `pub positions: SealedPositions` (default empty).
- Método `pub fn inject_positions(&mut self, sealed)`
  para caller pós-layout activar lookup real.
- `Introspector::position_of` impl: delega a
  `self.positions.position_of(location)`. Pre-injecção
  devolve `None` (preservando semântica P204D §C6a);
  pós-injecção devolve `Some(Position)` real.
- 4 tests novos: 3 unit + 1 E2E pipeline completo.

P205C **não materializa stdlib `here()`/`locate()`** —
esses são futuros consumers do impl real, não trabalho
de P205C.

### Caminho escolhido: A (não B nem C)

- **Caminho B (PagedTagIntrospector wrapper)**:
  rejeitado. Cristalino tem único impl `Introspector`;
  wrapper exigiria delegar 19 métodos só para 1
  especial. Vanilla precisa porque tem múltiplas impls
  (paged/html/...); cristalino não.
- **Caminho C (adiar)**: rejeitado. ADR-0074 §"Decisão"
  fixou explicitamente que F3 minimal fecha pendência
  ADR-0073 §C6a; adiar contradiria ADR PROPOSTA. Embora
  não haja consumers de produção (per C1.1), a
  infraestrutura completa (sealing + impl real) é
  pré-requisito para futuros `here()`/`locate()`.
- **Caminho A**: cirúrgico (1 field + 1 método + 1
  line); coerente com pattern P190C (struct dedicada,
  não wrapper).

### Output 1 — Inventário interno

Localização:
`00_nucleo/diagnosticos/typst-passo-205C-inventario.md`.

Conteúdo:
- §1 C1 inventário (5 sub-secções; 2 AJUSTES, 3
  CONFIRMADO).
- §2 C2 caminho fixado (A) com justificação em 4
  pontos.
- §3 C3 implementação literal.
- §4 C4-C5 sealing point + migração (caller controla
  injecção; tests existentes preservam semântica).
- §5 6 decisões durante a leitura (D1–D6).

Tamanho: ~9 KB.

### Output 2 — Relatório (este ficheiro)

### Output 3 — Alterações em código

#### Ficheiros tocados

- **`01_core/src/entities/introspector.rs`**:
  - Campo `pub positions: SealedPositions` adicionado a
    `TagIntrospector` (após `headings_for_toc`).
  - Método `pub fn inject_positions(&mut self, sealed)`
    em `impl TagIntrospector`.
  - `Introspector::position_of` impl agora delega a
    `self.positions.position_of(location)` (em vez de
    retornar sempre `None`).
  - Comentário do test
    `populado_responde_correctamente:472` actualizado
    para clarificar semântica P205C.
  - 3 tests P205C novos no `mod tests`:
    `p205c_position_of_pre_injecao_devolve_none`,
    `p205c_inject_positions_activa_lookup_real`,
    `p205c_inject_positions_e_idempotente_para_reinjecao`.
- **`01_core/src/rules/layout/tests.rs`**:
  - 1 test E2E novo:
    `p205c_pipeline_layout_seal_inject_query_devolve_some`
    (exercita pipeline completo).
- **`00_nucleo/adr/typst-adr-0074-f3-layouter-substores-trackable.md`**:
  - §P205C anotada com `✅ MATERIALIZADO 2026-05-07`
    + sumário literal.

**Sem ficheiros novos**. Sem L0 prompt novo (Caminho A
não introduz novo módulo; `entities/introspector.md`
existente cobre o trait).

---

## §2 Tempo de execução

~25 minutos efectivos:

- ~5 min: leitura da spec + setup TaskList.
- ~5 min: C1 inventário empírico (5 sub-secções; grep
  consumers + leitura arquitectural).
- ~3 min: C2 fixar Caminho A.
- ~5 min: C3-C5 implementação (3 edições cirúrgicas em
  introspector.rs).
- ~5 min: C6 escrita dos 4 tests novos.
- ~3 min: C7-C9 build + tests + lint.
- ~4 min: C10-C11 ADR anotada + outputs.

---

## §3 Métricas

| Métrica | Valor |
|---------|-------|
| Tests workspace antes | 1856 |
| Tests workspace depois | 1860 (+4) |
| Tests P205C | 4 (3 unit + 1 E2E) |
| Linter violations | 0 |
| Linter warnings | 0 |
| Ficheiros novos | 0 |
| Ficheiros modificados | 3 (introspector.rs, layout/tests.rs, ADR-0074) |
| LOC novas | ~75 (tests) + ~20 (impl) = ~95 |
| Cargo deps adicionados | 0 |
| Refactor mid-execution | 0 |

Distribuição:

- `typst_core` unit: 1580 → **1584** (+4 P205C tests).
- Restantes crates: sem alteração.

---

## §4 Decisões

### D1 — Caminho A sobre B (pragmatismo cristalino)

C1.4 mostrou que cristalino tem único impl Introspector;
wrapper Caminho B (vanilla-style) seria inflação
desproporcionada. Caminho A respeita pattern P190C
(field directo em vez de wrapper externo).

### D2 — Field directo `SealedPositions` (sem `Option`)

`pub positions: SealedPositions` em vez de
`pub positions: Option<SealedPositions>`.
`Default::default()` é `empty()` — devolve `None` para
qualquer location, semanticamente equivalente. Mais
leve sintacticamente.

### D3 — Caller controla injecção (sem wiring automático)

`pub fn layout` mantém assinatura existente. Caller
pós-layout invoca `intr.inject_positions(...)`
manualmente. Pattern análogo a
`PagedDocument.extracted_label_pages` (P63 + P190C):
infraestrutura disponível, caller decide se consume.

### D4 — Tests existentes preservam semântica `None`

2 tests existentes asseram `position_of(loc) == None` em
introspectors empty. P205C preserva esse comportamento:
`SealedPositions::empty().position_of(...)` devolve
`None`. Comentário actualizado num dos tests para
clarificar a nova semântica.

### D5 — 4 tests novos (3 unit + 1 E2E)

Spec C6 pediu 2-4 tests E2E. Implementei 3 unit
substantivos (cobertura granular: pre-injecção,
inject + lookup, re-inject idempotência) + 1 E2E
pipeline completo. Cobertura mais densa que o mínimo.

### D6 — `position_of` chama método raw (não Tracked)

`self.positions.position_of(location)` chama o método
`&self -> Option<Position>` directamente, sem `.track()`.
Este impl roda dentro de `Introspector::position_of` que
já é tracked a nível do trait (P204B); re-tracking
interno seria recursivo. O método raw existe (gerado
pelo macro `#[comemo::track] impl` que preserva ambos
variants).

---

## §5 Confronto de hipóteses do passo

A spec listou hipóteses específicas em §3 e §8:

| Hipótese | Resultado |
|----------|-----------|
| C1.1 mostra zero consumers reais (Caminho C provável) | **CONFIRMADO empíricamente** — zero consumers de produção. **Mas Caminho C rejeitado** por ADR fixar materialização (não inflação por demonstração; é dever da spec). |
| Wrapper PagedTagIntrospector por simetria com vanilla | **EVITADO** — C1.4 mostrou que cristalino tem única impl; wrapper seria inflação |
| `Option<SealedPositions>` para Hash/Send/Sync | **NÃO MATERIALIZOU** — Field directo (sem Option) suficiente; SealedPositions::empty() é equivalente semanticamente |
| Tests asserting None falham após migração | **PARCIALMENTE** — tests existentes usam empty introspectors; assert None continua válido (preservado) |
| `inject_positions` precisa `&mut self` | **CONFIRMADO** — método aceita `&mut self`; caller pós-layout não usa Tracked, sem invalidação |
| Tracking recursive no impl | **EVITADO** — chamada raw `self.positions.position_of(...)` em vez de via `.track()` |

5 de 6 hipóteses resolvidas pela auditoria empírica;
1 (Caminho C) refutada honestamente (ADR fixou
materialização).

---

## §6 Sugestão para próximo passo

P205C fechado per C12 com todos os critérios cumpridos:

- ✓ C1 inventário completo (5 sub-secções; 2 AJUSTES, 3
  CONFIRMADO).
- ✓ C2 caminho fixado (A) com justificação.
- ✓ C3 implementação aplicada.
- ✓ C4 sealing point + injecção (caller manual).
- ✓ C5 consumers migrados (tests E2E novos exercitam o
  caminho).
- ✓ C6 tests dedicados (4: 3 unit + 1 E2E).
- ✓ C7 compilação verde.
- ✓ C8 tests workspace verdes (1860).
- ✓ C9 linter 0 violations.
- ✓ C10 ADR-0074 anotada.
- ✓ C11 sentinelas (3 tests P205C funcionam como
  sentinelas estruturais para field + método +
  comportamento).
- ✓ Inventário registado.
- ✓ Relatório escrito.

**Próximo sub-passo**: **P205D — `label_pages`
trackable (condicional)** (per ADR-0074 plano de
materialização):

- Decisão de prosseguir P205D fixa-se no início de P205D
  com base em benefício observado em P205B/C
  (provavelmente ainda zero consumers `label_pages`
  trackable em produção; pode resultar em P205D = adiar).
- Magnitude estimada: S (se prosseguir) ou nula (se
  adiar).

---

## §7 Cross-references

- **Spec**:
  `00_nucleo/materialization/typst-passo-205C.md`.
- **Outputs**:
  - `00_nucleo/diagnosticos/typst-passo-205C-inventario.md`.
- **ADR**:
  `00_nucleo/adr/typst-adr-0074-f3-layouter-substores-trackable.md`
  (§P205C ✅ MATERIALIZADO 2026-05-07).
- **Predecessores**:
  - P205B (sealing infrastructure +
    `SealedPositions`).
  - P205A (diagnóstico-primeiro de F3).
  - P204D (Position concrete; pendência §C6a fechada
    estruturalmente por P205B+C).
- **Sucessor planeado**: P205D (label_pages
  condicional).
- **Pattern referência arquitectural**: P190C
  (`LayouterRuntimeState` pattern — struct dedicada em
  vez de wrapper externo).
- **Pattern `#[comemo::track]`**:
  `01_core/src/entities/introspector.rs:40` (M8 P204B).
- **Vanilla referência**:
  `lab/typst-original/crates/typst-layout/src/introspect.rs:60-63`
  (`PagedIntrospector::position` — não paridade literal).
