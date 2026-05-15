# Diagnóstico Introspection actualizado — Passo 254A

**Data**: 2026-05-15
**Tipo**: passo arquitectural de diagnóstico (não materializa código)
**Análogo estrutural**: P160 (diagnóstico Introspection original) com
estado factual actualizado pós série M3-M9 + P204A-H.
**Motivação**: o resumo cumulativo pós-P254 (mensagem final da
conversa anterior) cita "Introspection ~17%". Esse número vem do
P160 (2026-04-25) e foi superado por materialização massiva entre
P164 (M2) e P204H (M8 measurements internos). Este diagnóstico
substitui o número antigo por inventário factual de 2026-05-15.

---

## §1 — ADRs e DEBTs relevantes

### ADRs activos para Introspection

| ADR | Status | Relevância |
|-----|--------|------------|
| ADR-0029 | EM VIGOR | Pureza física L1 — Introspection runtime preserva |
| ADR-0033 | EM VIGOR | Paridade observable — preservada; pipeline interno diverge single-pass → 2-pass + fixpoint |
| ADR-0034 | EM VIGOR | Diagnóstico canónico — este ficheiro segue |
| ADR-0054 | EM VIGOR | Perfil graded — subset minimal aceite |
| ADR-0065 | EM VIGOR | Inventariar primeiro — este passo aplica critério #5 |
| ADR-0066 | ACEITE (com qualificação intermédia) | Introspection runtime — promovida P192B |
| ADR-0072 | ACEITE | M7 fixpoint runtime fechado (P192B) |
| ADR-0073 | ACEITE | Ponte hash-based convergence (cumprida P204H) |

### Histórico de promoção ADR-0066

- **2026-04-27** (P160A): criada PROPOSTO.
- **2026-05-05** (P192B): PROPOSTO → ACEITE com qualificação
  "intermédio até M8".
- **2026-05-07** (P204): ADR-0073 ACEITE; ponte hash-based
  convergence cumprida.

### DEBTs

- **DEBT-12** (forward refs page numbers): coberto por TOC fixpoint
  em `layout/mod.rs:1515` desde P192B.
- Sem DEBT formal específico para Introspection runtime — ADR-0066
  capturou tudo.

---

## §2 — Inventário factual de código (2026-05-15)

### Ficheiros centrais

| Ficheiro | Linhas (aprox) | Função |
|----------|----------------|--------|
| `01_core/src/rules/introspect.rs` | 1108 (P160) → estendido | Walk DFS; populador de stores |
| `01_core/src/rules/introspect/fixpoint.rs` | 626 | M7 fixpoint runtime (P192B) |
| `01_core/src/rules/introspect/from_tags.rs` | — | M3+ construtor de TagIntrospector |
| `01_core/src/rules/introspect/locatable.rs` | — | `is_locatable` pura (P164) |
| `01_core/src/entities/introspector.rs` | — | TagIntrospector struct (P165) |
| `01_core/src/entities/metadata_store.rs` | — | MetadataStore sub-store (P169) |
| `01_core/src/entities/counter_state.rs` | 333 (P160) | Legacy single-pass; cleanup orgânico M6 |
| `01_core/src/entities/resolved_label_store.rs` | — | ResolvedLabelStore (P193B) |

### Variants Content locatable (10 de 56 — per P164 + extensões)

1. `Heading` (P164 baseline)
2. `Figure` (P164 baseline)
3. `Cite` (P164 baseline)
4. `Metadata` (P169 — feature `metadata()` M9)
5. `State` (P171 — feature `state()` M9)
6. `StateUpdate` (P171 — feature `state()` M9)
7. `Outline` (P178)
8. `Bibliography` (P181D)
9. `SetHeadingNumbering` (P182C)
10. `Equation` (P186D)

### Marcos arquitecturais fechados

| Marco | Passo | Significado |
|-------|-------|-------------|
| M2 | P164 | `is_locatable` pura |
| M3 | P165 | `TagIntrospector` struct |
| M5 universal | P200B | Todos walk arms cristalinos fechados estruturalmente |
| M6 | P190I | `CounterStateLegacy` eliminado (reescrita do zero) |
| M7 | P192B | Fixpoint runtime fechado (2 loops complementares) |
| M8 | P204H | Measurements internos materializados |
| M9 | 11/11 | Features stdlib runtime |

---

## §3 — Features Introspection vanilla vs cristalino actual

Vanilla typst tem ~13 features observable + ~6 features arquitecturais
no módulo introspection.

### Features observable (user-facing)

| Feature | Estado P160 (2026-04-25) | Estado actual (2026-05-15) | Passo |
|---------|--------------------------|----------------------------|-------|
| `counter()` | implementado | implementado | P60-62 |
| `state(key, init)` | ausente | **implementado** | P171 |
| `metadata(value)` | ausente | **implementado** | P169 |
| `here()` / `locate()` | ausente | **implementado** | série M9 |
| `query(target)` | ausente | **implementado** | série M9 |
| `position(target)` | ausente | parcial / implementado | série M9 |
| `measure()` | parcial (helper) | **implementado** stdlib | P204G measurements |
| `convergence` (fixpoint) | ausente | **implementado** | P192B M7 |
| `introspector` engine | ausente | **implementado** | P165 M3 |
| `location` type | ausente | **implementado** | série M5 |
| `locator` | ausente | implementado parcial | série M5/M6 |
| `tag` | ausente | implementado | série M5 |
| cross-document refs | ausente | ausente | bloqueia pipeline multi-document |

**Contagem**: 12/13 implementado ou parcial; 1/13 ausente
(cross-document refs).

### Cobertura observable A.9 (vanilla user-facing tracking)

P160 reportou **1/6 = ~17%** (counter único user-facing observable).
Estado actual aproximado: **5-6/6 = ~83-100%** observable
user-facing — counter, state, metadata, here/locate, query
materializados; measure() exposto via stdlib P204G.

**Estimativa conservadora**: **~85%** (subset minimal P160 §6
plus measurements; cross-document refs single-doc é o gap real
restante).

### Refinos qualitativos pós-implementação

- **comemo::Track** (ADR-0073 ponte hash-based → adopção real):
  ainda não materializado; M8 dedicado.
- **Re-walks parciais**: actual é full re-walk por iteração;
  comemo permitiria invalidação granular.
- **Cross-document cite refs**: requer pipeline multi-document;
  fora de Introspection puro.

---

## §4 — Análise de tecto realista

### Tecto cristalino actual (single-doc, hash-based convergence)

**Estimativa**: ~85% observable A.9. Saturado pela ausência de:

1. **Cross-document cite refs** — requer pipeline multi-document
   (não Introspection puro).
2. **`comemo::Track` granular** — performance, não cobertura
   observable.

### Tecto pós-M8 (`comemo::Track`)

**Estimativa**: ~90-95%. Ganho é principalmente arquitectural
(performance + paridade interna) não observable.

### Tecto pós-multi-document

**Estimativa**: ~95-100%. Cross-document refs último gap.

### Comparação com diagnóstico anterior

| Métrica | P160 (2026-04-25) | P254A (2026-05-15) |
|---------|-------------------|---------------------|
| Cobertura observable | ~17% | ~85% (estimado) |
| Features implementadas (user-facing) | 1/13 | 12/13 |
| Features arquitecturais (M-marcos) | 0 fechados | M2/M3/M5/M6/M7/M8/M9 fechados |
| Tecto realista próximo | ~50% pós-Bloco B | ~95% pós-comemo + multi-doc |

**Conclusão factual**: o número "~17%" do resumo cumulativo
pós-P254 está **desactualizado por ~50 sub-passos de
materialização**. Introspection deixou de ser "módulo mais fraco" —
passou a estar entre os mais cobertos.

---

## §5 — Sequência candidata pós-P254

### Bloco A (Introspection puro, fechado)

**Vazio** — features observable single-doc essencialmente
saturadas. Refinos qualitativos restantes:

- R1: refino de `measure()` (já materializado P204G — possíveis
  edge cases).
- R2: refino de fixpoint convergence (paridade exacta vanilla
  comemo).
- R3: refino de `query()` predicates avançados.

Cada um S+/M-; ganho de cobertura observable agregada baixo
(<5pp); valor arquitectural sim.

### Bloco B (M8 comemo adopção — ADR futura)

5-8 sub-passos estimados:

1. ADR-create M8 adopção `comemo::Track` (XS administrativo).
2. `#[comemo::track]` no trait `Introspector` (M).
3. Migração queries location-aware para comemo cache (M).
4. Sub-stores granulares com `#[comemo::track]` (M).
5. Eliminação fixpoint hash-based em favor de invalidação
   comemo (M+).
6. Benchmarks de performance pré/pós (S).

Cobertura observable: ~85% → ~92%. Ganho arquitectural alto.

### Bloco C (multi-document — fora de Introspection puro)

Cross-document cite refs; cross-document state. Requer pipeline
multi-document — domínio cross-módulo. Não materializável em
Introspection puro.

### Bloco D (módulo distinto — pivot real)

Se prioridade for cobertura agregada cross-módulo, considerar:

- **Visualize** ~54% (shape primitives, paths, curves) — pode
  reusar `Stroke` P252.
- **Text** ~52% — StyleChain refino + shaping rustybuzz real
  (DEBT-53).
- **Math** — confirmar estado actual.

---

## §6 — Recomendação concreta

### Recomendação primária

**P254B Bloco D — pivot para módulo distinto.** Introspection
atingiu tecto pragmático (~85%) sem trabalho arquitectural
significativo (M8 comemo). Continuar Introspection produz baixo
retorno em cobertura agregada vs alternativas.

**Submódulo recomendado**: **Visualize** (~54% pré-pivot;
arquitecturalmente distinto de Layout; reusa `Stroke` refactor
P252; ganho potencial +20-30pp cumulativo).

### Recomendação secundária

**ADR-create M8 adopção comemo** (XS administrativo) seguido de
materialização incremental. Padrão `ADR-0062-create`/`P160A`.
Ganho principal: paridade interna vanilla; cobertura observable
modesta.

### Recomendação terciária

**Refinos qualitativos R1/R2/R3** em sub-passos S+. Valor
incremental baixo; útil para preservar momentum granular se
pivot Visualize for adiado.

### Não recomendado

- **Continuar Introspection sem M8** — saturação real; cada
  refino adicional rende <2pp.
- **Refactor pipeline multi-document agora** — magnitude L
  cross-modular; melhor depois de M8 comemo estar fechado.

---

## Referências

- ADR-0033, ADR-0034, ADR-0054, ADR-0065, ADR-0066, ADR-0072,
  ADR-0073.
- P160 — diagnóstico Introspection original (2026-04-25).
- P164 — `is_locatable` M2.
- P165 — `TagIntrospector` M3.
- P169 — `metadata()` M9.
- P171 — `state()` + `StateUpdate` M9.
- P178 — `Outline` locatable.
- P181D — `Bibliography` locatable.
- P182C — `SetHeadingNumbering` locatable.
- P186D — `Equation` locatable.
- P189B → P200B — série M5 universal completo.
- P190 série → P190I — M6 fechado.
- P192A/B — M7 fixpoint fechado.
- P204A-H — M8 measurements + ADR-0073 ponte.
- Resumo cumulativo pós-P254 — fonte do número desactualizado
  "~17%".
