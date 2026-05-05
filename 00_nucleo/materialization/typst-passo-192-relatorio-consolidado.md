# Relatório Consolidado P192 — M7 estruturalmente fechado

**Data**: 2026-05-05
**Magnitude consolidada**: S-M agregada (2 sub-passos declarativos).
**Estado**: P192 série completa — **M7 estruturalmente fechado**.
**ADR-0072**: ACEITE em P192B.
**ADR-0066**: PROPOSTO → ACEITE em P192B com nota "intermediário até M8".

---

## §1 Resumo executivo

P192 série fecha após 2 sub-passos declarativos (A diagnóstico + B
declaração formal). Cristalino atinge **consolidação arquitectural
intermédia**: M5 universal completo + M6 fechado + M7
estruturalmente fechado + M9 11/11.

P192A revelou empiricamente que M7 estava estruturalmente fechado
sem ADR explícita, por agregação incremental P174 → P175-P179 → M9
→ P190 série → P191 série. P192B formaliza:
- ADR-0072 NOVA ACEITE — fechamento estrutural M7.
- ADR-0066 PROPOSTO → ACEITE com nota "intermediário até M8".

**Correcção interpretativa**: divergência sem comemo é **intermédia,
não permanente**. Hash-based convergence funciona como mecanismo
intermédio viável; **paridade vanilla via comemo é objectivo final
(M8)** — próximo passo arquitectural natural.

---

## §2 Sub-passos materializados

| Passo | Magnitude planeada | Magnitude real | Δ tests | Outputs |
|-------|--------------------|--------------------|---------|---------|
| **P192A** | S-M | S-M | 0 | Diagnóstico (10 secções) + relatório (14 secções) |
| **P192B** | S | S | 0 | ADR-0072 NOVA ACEITE + ADR-0066 ACEITE (com nota) + relatório consolidado (10 secções) |
| **Total** | S-M agregado | S-M | **0** | 2 ADRs + 3 documentos |

---

## §3 Decisões arquitecturais

### 7 cláusulas P192A fechadas

| # | Cláusula | Decisão final |
|---|----------|---------------|
| 1 | `fixpoint.rs` | 626 LOC; `run_fixpoint`, `introspect_to_fixpoint` públicos |
| 2 | Loops fixpoint | 2 loops complementares: TOC (mod.rs:1515) + run_fixpoint (introspect/fixpoint.rs:65); MAX = 5 paridade nominal |
| 3 | Queries runtime | 4 queries activas em Layouter; `current_location` (P185C) |
| 4 | Validação | 13+ tests fixpoint.rs + TOC tests; 1802 workspace verdes |
| 5 | Comparação vanilla | Paralelo conceptual; divergência intermédia (sem comemo) |
| 6 | Estado A/B/C | **Estado A** — completo |
| 7 | Critério fecho M7 | Loop fixpoint + queries runtime + tests + L0 + ADRs ACEITES |

### Decisões empíricas P192B

- **ADR-0072**: criar como NOVA ACEITE imediato (justificável pela
  validação empírica P192A).
- **ADR-0066**: PROPOSTO → ACEITE com nota "intermediário até M8".
- **Correcção interpretativa**: P192A §4.3 inicial — "divergência
  arquitectural intencional sem comemo" — interpretação prematura.
  **Correcção**: divergência é intencional para fase intermédia,
  não permanente. Hash-based convergence é mecanismo intermédio;
  comemo virá em M8.

---

## §4 Achados não-triviais

### 4.1 Dois loops fixpoint complementares

Cristalino tem **dois loops fixpoint distintos** que resolvem
**categorias diferentes** de dependências reverse:

| Loop | Dependência reverse | Activado | Convergência |
|------|---------------------|----------|--------------|
| TOC fixpoint | Page numbers (layout-time) | doc com Outline | page map |
| `run_fixpoint` | Stdlib queries (eval-time) | opt-in via M9 | tag hash |

Não redundantes; cobertura disjunta.

### 4.2 `run_fixpoint` é mecanismo opt-in estruturalmente pronto

Per docs internas em `fixpoint.rs:10-13`: "Mecanismo sem clientes
em P174. Adopção planeada para P175+".

P175-P179 (M9) materializaram features stdlib que dependem do
mecanismo. Em produção real (não-tests), features são exercitadas
quando documentos invocam `query()`, `counter.at()`, `here()`. Em
testes apenas, o mecanismo é validado por 13+ tests E2E.

Implicação: estrutura pronta; tracção em produção depende de
expansão de features stdlib em uso real.

### 4.3 (CORRIGIDO) Divergência sem comemo é intermédia, não permanente

**Narrativa P192A §4.3 inicial** afirmou que cristalino "não usa
comemo" como decisão arquitectural permanente. **Correcção em
P192B**:

- Cristalino actual: hash-based convergence — **mecanismo
  intermédio viável** (per ADR-0066 ACEITE com nota).
- Vanilla typst: comemo + invalidação granular — **paridade que
  cristalino procura**.
- M8 introduzirá comemo — **objectivo final**.

A divergência é **intencional para a fase intermédia M5-M7**, não
arquitectural definitiva. ADR-0066 ACEITE em P192B agora explicita
esta nuance.

### 4.4 Layouter location-aware queries são pré-condição satisfeita

ADR-0068 (location-aware Layouter) — ACEITE — fornece o mecanismo
`current_location` em Layouter, sincronizado-por-construção com
walk Locator. M7 sub-passo "queries runtime location-aware durante
layout" depende deste mecanismo. Confirmação pós-P190I: 4 queries
activas em produção.

### 4.5 Pattern "auditoria sobre estado existente" (1ª aplicação)

P192A é **primeira instância documentada** do padrão "auditoria
sobre estado existente vs planeamento de trabalho futuro" no
projecto. Distinção:

- Auditorias planeadoras (P190A, P191A, etc.) produzem ADR
  PROPOSTO + plano implementação.
- **P192A**: audita estado já materializado; produz declaração
  formal + ADR ACEITE retrospectiva.

Pattern reaproveitável quando trabalho cumulativo incremental
atinge fechamento estrutural sem ADR explícita. Documentado em
ADR-0072.

---

## §5 Estado activo vs preservado

### Activado (pré-existente confirmado em P192A; declarado em P192B)

- ✅ TOC fixpoint loop em Layouter (mod.rs:1515).
- ✅ `run_fixpoint` mecanismo opt-in (introspect/fixpoint.rs:65).
- ✅ `introspect_to_fixpoint` wrapper (P175).
- ✅ 4 queries location-aware activas em Layouter.
- ✅ `current_location` field populated por
  `advance_locator_if_locatable`.
- ✅ Pattern `apply_state_funcs` slim post-pass (P191B).
- ✅ MAX_FIXPOINT_ITERATIONS = 5; MAX_ITERATIONS = 5 (paridade
  nominal vanilla).

### Preservado

- Trait `Introspector` 20 métodos.
- `TagIntrospector` 9 sub-stores.
- `LayouterRuntimeState` 3 fields.
- ADRs ACEITES anteriores: 0067, 0068, 0069, 0070, 0071.
- M5 universal completo, M6 fechado, M9 11/11.

### Pendente para M8

- **Adopção `comemo::Track`** em:
  - Trait `Introspector` (`#[comemo::track]` impl).
  - Queries location-aware.
  - Sub-stores `TagIntrospector` (se aplicável).
- **Objectivo**: paridade vanilla typst — saída igual + performance
  comparável.

---

## §6 Estado final M5, M6, M7, M9

| Marco | Estado | Sub-passo final |
|-------|--------|-----------------|
| **M5** universal completo | ✅ | P200B |
| **M6** fechado completo | ✅ | P190I |
| **M7** estruturalmente fechado | ✅ | **P192B** |
| **M9** | ✅ 11/11 | (snapshot pré-P190) |
| M8 (comemo) | pendente | próximo passo natural |

**Marco arquitectural**: cristalino atinge **consolidação
arquitectural intermédia** com 4 marcos fechados estruturalmente.

---

## §7 Estado final lacunas

| Lacuna | Estado |
|--------|--------|
| #1 (Position) | residual (inalterado) |
| #1b (Position-related) | residual (inalterado) |
| #2 (Counter at locations) | residual (inalterado) |
| #3 (headings_for_toc) | fechada P200B |

P192 não impactou lacunas residuais.

---

## §8 Pendências cumulativas

### M7 fechado por declaração formal estrutural

- ADR-0072 ACEITE: M7 fixpoint runtime estruturalmente fechado.
- ADR-0066 ACEITE com nota: introspection runtime adiada
  validada como decisão intermédia.

### M8 reconhecido como próximo passo natural

- **Magnitude esperada**: L cross-modular (similar a M6).
- **Pré-condição cumprida**: M5+M6+M7 estruturalmente fechados.
- **Objectivos**:
  - Paridade vanilla typst (saída igual + performance).
  - Adopção `comemo::Track` em Introspector + queries.
  - Re-walks parciais via invalidação granular.

### Ortogonais (fora de escopo P192)

- F3 completo (Layouter restantes 19 fields).
- Lacunas residuais (#1 Position, #1b, #2).

---

## §9 Próximos passos sugeridos

### Imediato

- **Decisão estratégica**: avançar para M8 (comemo) **ou** pausa
  estratégica para consolidar M5+M6+M7 fechados.

### M8 (comemo) — próximo passo arquitectural natural

Trabalho concreto esperado:
1. ADR dedicada (futura ADR-0073 ou similar).
2. Adopção `#[comemo::track]` em trait `Introspector`.
3. Queries location-aware re-emitidas com tracking.
4. Validação saída cristalino == vanilla (snapshot tests).
5. Performance comparável.

### Outras opções ortogonais

- F3 completo: refactor Layouter 19 fields ortogonais.
- Lacunas residuais: passos dedicados para #1, #1b, #2.
- Pausa estratégica: consolidar marcos fechados; foco em features
  novas em vez de refactor arquitectural.

---

## §10 Marco arquitectural — Consolidação intermédia M5+M6+M7+M9

### Pattern "auditoria sobre estado existente"

P192A é **1ª aplicação completa** do padrão. Documentado em
ADR-0072 como pattern reaproveitável.

### 7 ADRs ACEITES no ciclo M5/M6/M7

- **ADR-0066** — Introspection runtime adiada (ACEITE com nota
  "intermediário até M8").
- **ADR-0067** — Attribute grammar scoping (ACEITE).
- **ADR-0068** — Location-aware Layouter (ACEITE).
- **ADR-0069** — Post-recursion tag emission (ACEITE P195E).
- **ADR-0070** — Eliminação `CounterStateLegacy` (ACEITE P190I).
- **ADR-0071** — Walk pipeline redesign (ACEITE P191C).
- **ADR-0072** — M7 fixpoint runtime estruturalmente fechado
  (ACEITE P192B).

### 33ª aplicação diagnóstico-primeiro consecutiva

Padrão consolidado: P181 → P200 + P190A-I + P191A-C + P192A-B.

### Histórico fechamento M7

```
P174 (run_fixpoint mecanismo)
   ↓
P175-P179 (features stdlib via fixpoint)
   ↓
M9 11/11 (snapshot pré-P190)
   ↓
P190 série A-I (M6 fechado completo)
   ↓
P191 série A-C (ramo paralelo ADR-0071)
   ↓
P192A diagnóstico (Estado A confirmado)
   ↓
P192B declaração formal (ADRs ACEITES)
   ↓
M7 ESTRUTURALMENTE FECHADO
```

### Significado arquitectural

**M5+M6+M7+M9 fechados estruturalmente**: cristalino tem mecanismo
funcional para todas as principais features typst — pre-pass walk +
loops fixpoint + queries location-aware + features stdlib via
fixpoint.

**M8 elevará para paridade arquitectural com vanilla** — saída
igual + performance comparável via adopção comemo. Pré-condição
cumprida; M8 enfocará exclusivamente em comemo.

### Métricas P192 série

- **2 sub-passos**.
- **0 LOC produção** (declarativo puro).
- **0 LOC tests** (inalterados).
- **2 ADRs**: ADR-0072 (NOVA ACEITE), ADR-0066 (PROPOSTO → ACEITE
  com nota).
- **3 documentos**: P192A diagnóstico + P192A relatório +
  P192-relatório-consolidado.
- **Tests workspace**: 1.802 verdes (inalterados).
- **Linter**: 0 violations.

---

## §11 Restrições mantidas

- ✅ Zero código produção tocado.
- ✅ Zero testes modificados.
- ✅ `fixpoint.rs` NÃO modificado.
- ✅ Trait `Introspector` NÃO modificado.
- ✅ `TagIntrospector` NÃO modificado.
- ✅ `LayouterRuntimeState` NÃO modificado.
- ✅ Layouter NÃO modificado.
- ✅ Lacunas residuais NÃO materializadas.
- ✅ Sem reservas de identificadores.
- ✅ Sem inflação retórica (palavras vetadas evitadas).
- ✅ Comparação vanilla feita.
- ✅ Correcção interpretativa P192A §4.3 sem inflação.

---

## §12 Linhagem

- **Pattern arquitectural**: M7 fechado estruturalmente por
  agregação incremental; ADR retrospectiva (ADR-0072 ACEITE
  P192B).
- **ADR-0066** (ACEITE com nota P192B) — autoriza adiamento
  intermédio comemo.
- **ADR-0072** (ACEITE P192B) — fechamento estrutural M7.
- **5 ADRs ACEITES anteriores** ciclo M5/M6: 0067, 0068, 0069,
  0070, 0071.
- **Pattern stylesheet "diagnóstico-primeiro"**: 33ª aplicação.
- **Pattern stylesheet "auditoria sobre estado existente"**: 1ª
  aplicação completa (documentada em ADR-0072).
- **M5+M6+M7+M9** estruturalmente fechados.
- **M8** reconhecido como próximo passo natural.

---

## §13 Achado arquitectural significativo

P192 série **não acrescenta funcionalidade**. **Documenta** que
funcionalidade já estava materializada por agregação incremental
P174 → P175-P179 → M9 → P190 série → P191 série.

**Significado**: cristalino atinge fase de **consolidação
arquitectural** — marcos fechados; pode-se prosseguir para
trabalho elevado (M8 comemo, paridade vanilla) ou ortogonal (F3
completo, lacunas residuais) ou pausa estratégica.

**99 passos executados** (P192A=98 + P192B=99 vs P191C=94 + P190I=97).

A próxima decisão é estratégica do utilizador.
