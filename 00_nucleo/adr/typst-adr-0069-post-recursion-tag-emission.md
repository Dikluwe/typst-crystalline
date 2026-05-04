# ⚖️ ADR-0069: Post-recursion tag emission for state-dependent payload

**Status**: `ACEITE`
**Validado**: 2026-05-04 — P195D 4 tests E2E (`mod
p195d_walk_labelled`) passam; paridade observable
preservada via mutação legacy paralela; sincronização
ADR-0068 mantida via reuso de Location do target. P195E
ratificou em relatório consolidado.
**Data**: 2026-05-04 (PROPOSTO) → 2026-05-04 (ACEITE)
**Diagnóstico prévio**:
- `00_nucleo/diagnosticos/diagnostico-walk-labelled-passo-195a.md` (P195A).
- `00_nucleo/materialization/typst-passo-189-relatorio-consolidado.md` §5 E4 (excepção identificada).
- `00_nucleo/materialization/typst-passo-186-relatorio-consolidado.md` §4 (descoberta empírica P186C/D — gating walk em `extract_payload`).
**Materialização**:
- `00_nucleo/materialization/typst-passo-195b-relatorio.md` (variant + stub no-op + ADR PROPOSTO).
- `00_nucleo/materialization/typst-passo-195c-relatorio.md` (`from_tags` arm completo — pendente).
- `00_nucleo/materialization/typst-passo-195d-relatorio.md` (walk arm post-recursion emit — pendente).
- `00_nucleo/materialization/typst-passo-195-relatorio-consolidado.md` (encerramento P195E — pendente).

---

## Contexto

Padrão existente em cristalino para extracção de payload a
partir de walk de introspect: walk top-level chama
`extract_payload(content: &Content) -> Option<ElementPayload>`
(função pura) **antes** do match arm. Se retorna `Some`,
walk emite `Tag::Start(loc, ElementInfo { payload, label })`
automaticamente via gating uniforme (`introspect.rs:329`).

Casos cobertos por este padrão:
- **P181D Bibliography**: `extract_payload` clona
  `entries: Vec<BibEntry>`. Sem dependência de state.
- **P182C SetHeadingNumbering**: `extract_payload` produz
  `StateUpdate { key: "numbering_active:heading", update:
  Set(Bool(active)) }`. Sem dependência de state.
- **P186C Equation**: `extract_payload` carrega `block` +
  `counter_update`. Sem dependência de state.
- **P184B Figure / P181F Citation / P169 Metadata / etc.**:
  payload puro a partir de campos do `Content`.

P195A identificou o **primeiro caso onde payload depende
de state mutado durante walk recursivo**:

`Content::Labelled { target, label }` — walk arm computa
`resolved_text` baseado em:
- `state.format_hierarchical("heading")` (counter mutado
  em walk arm Heading do target).
- `state.get_flat("equation")` (counter mutado em walk arm
  Equation do target).
- `state.figure_numbers.get(kind_key)` (counter mutado em
  walk arm Figure do target).
- `state.lang: Option<Lang>` (state field externo).

`extract_payload` é **função pura** sem parâmetro `state`.
Não pode replicar lógica state-dependent. Refactorizá-lo
para receber `state` mudaria contrato fundamental + 8
implementações.

---

## Decisão

**Pattern alternativo: post-recursion tag emission for
state-dependent payload.**

Forma:
1. Walk arm Labelled processa target normalmente
   (`walk(target, state, locator, tags, Some(label))`).
2. **Após recursão**, walk arm computa payload usando
   state actual (counters acumulados pelos walks
   recursivos do target).
3. Walk arm emite Tag manualmente em `tags`:
   ```rust
   let loc = locator.next();
   tags.push(Tag::Start(loc, ElementInfo::new(
       ElementPayload::Labelled { label, resolved_text, figure_number }
   )));
   tags.push(Tag::End(loc, 0));  // hash não usado
   ```
4. **Sem** alteração em `is_locatable` (continua `false`).
5. **Sem** arm em `extract_payload` (catch-all `_ => None`
   intacto).
6. **Sem** `ElementKind::Labelled` (não conta para
   `kind_index`).
7. `from_tags` arm Labelled processa payload via match
   uniforme; popula `intr.resolved_labels` +
   `intr.figure_label_numbers`.

Implementação detalhada em P195D (walk arm) + P195C
(`from_tags` arm).

---

## Justificação

### Why not Opção 1 padrão (locatable + extract_payload)

`extract_payload(content: &Content) -> Option<ElementPayload>`
não tem `state` parameter. Replicar lógica de
`state.format_hierarchical("heading")` exige conhecer
counters acumulados — impossível em função pura.

Refactorizar `extract_payload` para
`extract_payload(content, state)` mudaria contrato + 8
implementações + L0s. Magnitude L+ vs M para post-recursion
emit.

### Why not Opção 2 (StateUpdate)

Walk arm poderia emitir Tag `ElementPayload::StateUpdate
{ key: "resolved_label:{label}", update: Set(Str(text)) }`
após recursão. `from_tags` arm StateUpdate (existente)
popula `StateRegistry`.

**Inconsistência crítica**: consumer C4 (P194B) lê de
`intr.resolved_labels` (sub-store P193B). Para Opção 2
funcionar, ou:
- Bridge no `from_tags` copia de `StateRegistry` para
  `ResolvedLabelStore` (duplicação de estado).
- Consumer C4 muda para ler `StateRegistry` (regressão
  P194B).

Ambos desfazem trabalho recente. Padrão não-natural.

### Why this pattern (Opção 3 / Opção 1-modificada)

- **Preserva semântica de sub-stores**: `intr.resolved_labels`
  é o destino correcto.
- **Mínima superfície de mudança**: walk arm + from_tags
  arm + variant nova. Sem alteração em `is_locatable`,
  `extract_payload`, ou trait `Introspector`.
- **Sem janela invariante quebrada**: `is_locatable
  ↔ extract_payload.is_some()` mantido (ambos `false`/`None`
  para Content::Labelled).
- **Aplicabilidade futura**: pattern reutilizável para
  outros walk arms state-dependent (P196 Heading
  auto-toc, P197 Figure, P198).

### Trade-offs reconhecidos

**Positivos**:
- Permite migração de walk arms state-dependent.
- Sem refactor major de contratos existentes.
- Pattern aplicável a P196/P197/P198.

**Negativos**:
- Walk arm tem responsabilidade dupla (computar +
  emitir Tag) vs uniformidade `extract_payload`.
- Sem benefícios de `kind_index` (query por kind não
  funciona para Labelled).
- Pattern menos canônico — leitores precisam de
  documentação adicional.
- Helper privado (e.g. `compute_labelled` per P195A
  §11.6) duplica lógica entre mutação legacy
  (preservada durante janela compat) e populate Tag.

---

## Alternativas rejeitadas

### Opção 1 padrão — locatable + extract_payload puro

Impossível. `extract_payload` puro não pode ler state.

### Opção 2 — StateUpdate via `extract_payload` ou walk arm

Inconsistência sub-store: bridge ou regressão consumer.

### Opção 4 — Refactor `extract_payload` para `extract_payload(content, state)`

Mudança de contrato fundamental + 8 implementações + L0s.
Magnitude L+. Trade-off pior que post-recursion emit.

### Opção 5 — `extract_payload` retorna `Option<Box<dyn Fn(&State) -> ElementPayload>>`

Closure retornada por extract_payload, avaliada em walk
top com state acessível. Idiomático em algumas linguagens
mas overhead de Box+dyn em Rust + complexidade
arquitectural alta. Magnitude M+. Não vale o custo.

---

## Aplicabilidade futura (sequência §9 P189)

Pattern post-recursion emit é candidato natural para:

- **P196 Heading walk arm** (auto-toc): `state.resolved_labels`
  populated com chave `auto-toc-N` baseado em
  `state.format_hierarchical("heading")` durante walk.
  Idêntico padrão.

- **P197 Figure walk arm**: `state.figure_numbers` +
  `state.local_figure_counters` mutados durante walk;
  payload depende.

- **P198 SetHeadingNumbering + CounterUpdate**: walks
  state-dependent menores. Podem usar pattern ou continuar
  com StateUpdate via extract_payload (já cobertos por
  P182C).

ADR-0069 abre porta para estes passos sem decisão
arquitectural nova em cada um.

---

## Critério de validação

ADR transita para `ACEITE` quando:

1. P195D materializa walk arm Labelled com pattern
   post-recursion emit.
2. P195E tests E2E confirmam:
   - Paridade observable preservada (output Layouter
     idêntico legacy vs Introspector path).
   - Activação Introspector path para explicit labels
     (consumer C4 P194B começa a receber `Some(text)`).
3. P195 série fecha em P195E sem regressões.

ADR transita para `REJEITADO` se:
- P195C/D revelarem bloqueador estrutural não previsto.
- Tests E2E mostrarem regressão observable não-trivial.
- Custo real escalar para L+ inesperadamente.

---

## Histórico

| Data | Estado | Motivo |
|------|--------|--------|
| 2026-05-04 | `PROPOSTO` | P195A diagnóstico identificou pattern arquitectural novo necessário para migrar walk arm Labelled (state-dependent payload). 7 cláusulas decididas; Opção 1-modificada fixada. |
| 2026-05-04 | `ACEITE` | P195E §1 confirmou validação empírica: P195B variant + stub no-op; P195C `from_tags` arm funcional (4 tests passam); P195D walk arm emite Tag pós-recursão com helper `compute_labelled` + reuso de Location do target via snapshot+find_map (preserva sincronização ADR-0068); P195D 4 tests E2E `mod p195d_walk_labelled` passam (1838 verdes total). Pattern aplicado com sucesso; mutação legacy preservada como write paralelo durante janela compat M5; output observable em produção inalterado. Critério §6 cumprido. Pattern disponível para P196 Heading auto-toc, P197 Figure, P198 walks state-dependent. |
